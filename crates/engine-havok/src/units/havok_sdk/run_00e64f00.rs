//! `Havok SDK/run_00e64f00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::x87::fistp;

/// `hkpMoppSphereVirtualMachine::querySphereOnTree` (Xbox PDB name, `00e64870`): thiscall on the
/// query object, `(query block, code)`, `RET 8`.
const QUERY_SPHERE_ON_TREE: u32 = 0x00e6_4870;

/// `FLD float [vector + 4 * lane]; FSUB float [origin]; FMUL float [scale]; FSTP float; FLD float;
/// FISTP dword` with the default rounding (nearest, ties to even).
fn quantise(e: &Engine, vector: u32, lane: u32, source: u32, origin_lane: Option<u32>) -> i32 {
    let value = x87_load32(e.mem.u32(vector.wrapping_add(4 * lane)));
    let shifted = match origin_lane {
        Some(origin) => x87_sub(
            value,
            x87_m32(e.mem.u32(source.wrapping_add(0x10 + 4 * origin))),
        ),
        None => value,
    };
    let scaled = x87_mul(shifted, x87_m32(e.mem.u32(source.wrapping_add(0x1c))));
    fistp(x87_load32(x87_store32(scaled)))
}

// Translated from 00e64f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: prepares a sphere query on a packed tree. `source` holds the tree's
/// origin (`float`s at `+0x10`, `+0x14`, `+0x18`), a scale (`+0x1c`) and the code pointer
/// (`+0x20`); `point` is a vector `(x, y, z, radius)`. `this + 0` gets `value`; `this + 0x10`,
/// `+0x14` and `+0x18` get `round((point - origin) * scale) - 1` per axis (the exe's default
/// rounding, nearest even) and `this + 0x1c` gets `round(radius * scale) + 2`. A block of ten words is
/// then built (the high 16 bits of `this + 0x10`, `+0x14` and `+0x18` sign-extended, `(this[+0x1c]
/// >> 16) + 1`, five zero words and `0x10`) and `00e64870` runs on `this` with it and the code
/// pointer.
pub fn fn_00e64f00(e: &mut Engine, this: Ptr, source: u32, point: u32, value: u32) {
    let this = this.addr();
    let code = e.mem.u32(source.wrapping_add(0x20));
    e.mem.set_u32(this, value);
    for axis in 0..3u32 {
        let quantised = quantise(e, point, axis, source, Some(axis)).wrapping_sub(1);
        e.mem.set_i32(this.wrapping_add(0x10 + 4 * axis), quantised);
    }
    let radius = quantise(e, point, 3, source, None).wrapping_add(2);
    e.mem.set_i32(this.wrapping_add(0x1c), radius);
    e.with_stack(0x28, |e, block| {
        let block = block.addr();
        let high = |e: &Engine, offset: u32| i32::from(e.mem.i16(this.wrapping_add(offset)));
        let words = [
            high(e, 0x12),
            high(e, 0x16),
            high(e, 0x1a),
            (radius >> 16).wrapping_add(1),
            0,
            0,
            0,
            0,
            0x10,
            0,
        ];
        for (index, word) in words.iter().enumerate() {
            e.mem.set_i32(block.wrapping_add(4 * index as u32), *word);
        }
        e.call(QUERY_SPHERE_ON_TREE, &args![this, block, code]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e6_4f00, fn_00e64f00(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the query double saw: the argument words and the ten words of the block.
    type Seen = Rc<RefCell<Vec<(Vec<u32>, Vec<i32>)>>>;

    fn query_double(e: &mut Engine) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(QUERY_SPHERE_ON_TREE, move |e, a| {
            let block = (0..10).map(|index| e.mem.i32(a[1] + 4 * index)).collect();
            log.borrow_mut().push((a.to_vec(), block));
            Ret::default()
        });
        seen
    }

    fn source(e: &mut Engine, origin: [f32; 3], scale: f32) -> u32 {
        let source = e.mem.alloc(0x30);
        for (index, value) in origin.iter().enumerate() {
            e.mem.set_f32(source + 0x10 + 4 * index as u32, *value);
        }
        e.mem.set_f32(source + 0x1c, scale);
        e.mem.set_u32(source + 0x20, 0x7777);
        source
    }

    fn point(e: &mut Engine, v: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        for (index, value) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
        at
    }

    #[test]
    fn axes_are_quantised_relative_to_the_origin_and_the_radius_is_scaled() {
        let mut e = Engine::new();
        let seen = query_double(&mut e);
        let source = source(&mut e, [1.0, 2.0, 3.0], 4.0);
        // (2.5 - 1) * 4 = 6, (2.125 - 2) * 4 = 0.5 (rounds to 0), (-1 - 3) * 4 = -16; radius
        // 1.5 * 4 = 6 + 2 = 8.
        let at = point(&mut e, [2.5, 2.125, -1.0, 1.5]);
        let this = e.mem.alloc(0x40);
        e.call(0x00e6_4f00, &args![this, source, at, 0x1234u32]);
        assert_eq!(e.mem.u32(this), 0x1234);
        assert_eq!(e.mem.i32(this + 0x10), 5);
        assert_eq!(e.mem.i32(this + 0x14), -1);
        assert_eq!(e.mem.i32(this + 0x18), -17);
        assert_eq!(e.mem.i32(this + 0x1c), 8);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(&seen[0].0[..1], &[this]);
        assert_eq!(seen[0].0[2], 0x7777);
        // The high words of -> 5, -1, -17 are 0, -1, -1; the radius 8 >> 16 is 0, plus 1.
        assert_eq!(seen[0].1, vec![0, -1, -1, 1, 0, 0, 0, 0, 0x10, 0]);
    }

    #[test]
    fn halves_round_to_even_and_large_radius_shifts() {
        let mut e = Engine::new();
        let seen = query_double(&mut e);
        let source = source(&mut e, [0.0; 3], 1.0);
        // 2.5 rounds to 2 and 3.5 to 4; the radius 70000 + 2 >> 16 is 1, plus 1.
        let at = point(&mut e, [2.5, 3.5, 0.5, 70000.0]);
        let this = e.mem.alloc(0x40);
        e.call(0x00e6_4f00, &args![this, source, at, 0u32]);
        assert_eq!(e.mem.i32(this + 0x10), 1);
        assert_eq!(e.mem.i32(this + 0x14), 3);
        assert_eq!(e.mem.i32(this + 0x18), -1);
        assert_eq!(e.mem.i32(this + 0x1c), 70002);
        assert_eq!(seen.borrow()[0].1[3], 2);
    }
}
