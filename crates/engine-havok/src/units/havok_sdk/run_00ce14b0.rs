//! `Havok SDK/run_00ce14b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` epsilon (`0104da4c`, `1.1920929e-07`) a vector's length must exceed in `00ce1500`.
const LENGTH_EPSILON: u32 = 0x0104_da4c;
/// The global vector (`01267e30`) the bodies' velocities are compared with and set to.
const VELOCITY_VECTOR: u32 = 0x0126_7e30;
/// The `float` tolerance (`01017d00`) of the velocity comparisons and the lane mask (`01017d10`)
/// that takes the magnitude (`ANDPS`).
const TOLERANCE: u32 = 0x0101_7d00;
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
/// `hkpRigidBody::setPositionAndRotation` (`00ca8860`, `this` = the body, stack `(position,
/// rotation)`), named by Ghidra from the Xbox PDB.
const SET_POSITION_AND_ROTATION: u32 = 0x00ca_8860;
/// `hkpEntity::activate` (`00c9c1d0`, `this` only), named by Ghidra from the Xbox PDB.
const ACTIVATE: u32 = 0x00c9_c1d0;

// Translated from 00ce14b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor-like initialiser (the class is unconfirmed): the 16-bit words at `+0x20`, `+0x22`
/// and `+0x24` take `first`, `second` and `third`; an empty array is written at `+0` (data 0,
/// size 0, capacity word `0x80000000`); the 16-byte vector at `vector` is copied to `+0x10`.
/// Answers `this` (`MOV EAX,ECX`; the caller stores it, 00c7b387).
pub fn fn_00ce14b0(
    e: &mut Engine,
    this: Ptr,
    first: u16,
    second: u16,
    third: u16,
    vector: Ptr,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 0x20, first);
    e.mem.set_u16(base + 0x22, second);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0x8000_0000);
    e.mem.set_u16(base + 0x24, third);
    for lane in 0..4 {
        let word = e.mem.u32(vector.addr() + 4 * lane);
        e.mem.set_u32(base + 0x10 + 4 * lane, word);
    }
    this
}

// Translated from 00ce1500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes 1 to the byte at `out` when the length of the vector at `this` (the first three
/// lanes, `SQRTSS` of the sum `z + (y + x)` of squares) is greater than the float at
/// `0104da4c`, 0 otherwise (a NaN length gives 0). Returns `out` (left in `EAX`).
pub fn fn_00ce1500(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let lanes: [f32; 4] = std::array::from_fn(|lane| e.mem.f32(this.addr() + 4 * lane as u32));
    let squares: [f32; 4] = std::array::from_fn(|lane| lanes[lane] * lanes[lane]);
    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
    let epsilon = x87_load32(e.mem.u32(LENGTH_EPSILON));
    // FCOMP: C0 is set when the constant is below the length, C2 when unordered.
    let longer = epsilon < f64::from(length);
    e.mem.set_u8(out.addr(), u8::from(longer));
    out
}

/// The four lanes `a - b` of two vectors in memory taken in magnitude (`SUBPS`, `ANDPS` with
/// the mask at `01017d10`) are all `<=` the splatted tolerance (`CMPLEPS`, false for NaN).
fn within_tolerance(e: &Engine, a: [f32; 4], b_at: u32, tolerance: f32) -> bool {
    (0..4u32).all(|lane| {
        let b = e.mem.f32(b_at + 4 * lane);
        let difference = a[lane as usize] - b;
        let magnitude = f32::from_bits(difference.to_bits() & e.mem.u32(ABSOLUTE_MASK + 4 * lane));
        magnitude <= tolerance
    })
}

fn velocity_vector(e: &Engine) -> [f32; 4] {
    std::array::from_fn(|lane| e.mem.f32(VELOCITY_VECTOR + 4 * lane as u32))
}

// Translated from 00ce1560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves a set of bodies to the transforms at `transforms` (`0x30` bytes each: the position
/// vector at `+0`, the rotation at `+0x10`). `this` lists the bodies: the count is the word
/// at `+0x1c` of the object at `this + 0x2c`, entry `i` of the index table (`this + 0x20`) is
/// the position of the body in the table whose data pointer is at `this + 8` (negative or
/// null entries are skipped). For each body: `setPositionAndRotation(position, rotation)`;
/// then when the global vector (`01267e30`) differs from the body's word vector at `+0x1b0` by
/// more than the tolerance (`01017d00`) in any lane (`ANDPS`/`CMPLEPS` and `MOVMSKPS`), the
/// body is activated (`00c9c1d0`) and virtual method `+0x40` of its member at `+0xe0` is called
/// with the global vector's address (the vector is then read again); the same for the vector
/// at `+0x1c0` with virtual method `+0x44`. The class is unconfirmed.
pub fn fn_00ce1560(e: &mut Engine, this: Ptr, transforms: Ptr) {
    let base = this.addr();
    let owner = e.mem.u32(base + 0x2c);
    let count = e.mem.i32(owner + 0x1c);
    let mut cursor = transforms.addr();
    let mut index = 0u32;
    if count > 0 {
        loop {
            let slot = e.mem.i32(e.mem.u32(base + 0x20).wrapping_add(index * 4));
            if slot >= 0 {
                let body = e.mem.u32(e.mem.u32(base + 8).wrapping_add(slot as u32 * 4));
                if body != 0 {
                    e.call(
                        SET_POSITION_AND_ROTATION,
                        &args![body, cursor, cursor + 0x10],
                    );
                    let tolerance = e.mem.f32(TOLERANCE);
                    let mut global = velocity_vector(e);
                    if !within_tolerance(e, global, body + 0x1b0, tolerance) {
                        e.call(ACTIVATE, &args![body]);
                        e.vcall(body + 0xe0, 0x40, &args![VELOCITY_VECTOR]);
                        global = velocity_vector(e);
                    }
                    if !within_tolerance(e, global, body + 0x1c0, tolerance) {
                        e.call(ACTIVATE, &args![body]);
                        e.vcall(body + 0xe0, 0x44, &args![VELOCITY_VECTOR]);
                    }
                }
            }
            index += 1;
            cursor = cursor.wrapping_add(0x30);
            if index as i32 >= count {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_14b0, fn_00ce14b0(Ptr, u16, u16, u16, Ptr) -> Ptr),
        entry!(0x00ce_1500, fn_00ce1500(Ptr, Ptr) -> Ptr),
        entry!(0x00ce_1560, fn_00ce1560(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// The three words, the empty array and the vector are written; the rest is untouched.
    #[test]
    fn initialiser_writes_words_array_and_vector() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.mem.write(this, &[0xaa; 0x30]);
        let vector = e.mem.alloc(0x10);
        for (i, word) in [0x11u32, 0x22, 0x33, 0x44].into_iter().enumerate() {
            e.mem.set_u32(vector + 4 * i as u32, word);
        }
        e.call(
            0x00ce_14b0,
            &args![this, 0x1_0001u32, 0x2_0002u32, 0x3_0003u32, vector],
        );
        let mut image = [0xaau8; 0x30];
        image[0x20..0x22].copy_from_slice(&1u16.to_le_bytes());
        image[0x22..0x24].copy_from_slice(&2u16.to_le_bytes());
        image[0x24..0x26].copy_from_slice(&3u16.to_le_bytes());
        image[0..8].copy_from_slice(&[0; 8]);
        image[8..12].copy_from_slice(&0x8000_0000u32.to_le_bytes());
        for (i, word) in [0x11u32, 0x22, 0x33, 0x44].into_iter().enumerate() {
            image[0x10 + 4 * i..0x14 + 4 * i].copy_from_slice(&word.to_le_bytes());
        }
        assert_eq!(e.mem.bytes(this, 0x30), image.to_vec());
    }

    #[test]
    fn initialiser_answers_this() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        let vector = e.mem.alloc(0x10);
        // `MOV EAX,ECX`: the caller stores the answer (00c7b387).
        let answered = e
            .call(0x00ce_14b0, &args![this, 1u32, 2u32, 3u32, vector])
            .u32();
        assert_eq!(answered, this);
    }

    /// The flag is 1 only when the length exceeds the epsilon; equal, shorter and NaN lengths
    /// give 0, and the fourth lane does not count.
    #[test]
    fn length_flag_compares_with_the_epsilon() {
        let mut e = engine();
        e.mem.set_u32(LENGTH_EPSILON, 0x3400_0000);
        let vector = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        for (lanes, expected) in [
            ([1.0f32, 0.0, 0.0, 9.0], 1u8),
            ([1e-8, 0.0, 0.0, 9.0], 0),
            ([0.0, 0.0, 1.2e-7, 0.0], 1),
            ([0.0, 0.0, 0.0, 5.0], 0),
            ([f32::NAN, 0.0, 0.0, 0.0], 0),
            ([f32::INFINITY, 0.0, 0.0, 0.0], 1),
        ] {
            for (i, value) in lanes.into_iter().enumerate() {
                e.mem.set_f32(vector + 4 * i as u32, value);
            }
            e.mem.set_u8(out, 0xee);
            e.call(0x00ce_1500, &args![vector, out]);
            assert_eq!(e.mem.u8(out), expected, "lanes {lanes:?}");
        }
        // The epsilon itself: a length equal to it is not above it.
        e.mem.set_f32(vector, f32::from_bits(0x3400_0000));
        for lane in 1..4 {
            e.mem.set_f32(vector + 4 * lane, 0.0);
        }
        e.mem.set_u32(LENGTH_EPSILON, 0x3400_0000);
        e.call(0x00ce_1500, &args![vector, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    const MOTION_SLOT_40: u32 = 0x0900_0401;
    const MOTION_SLOT_44: u32 = 0x0900_0441;

    /// Two table entries (a body and a skipped negative index) plus a null body entry; the
    /// bodies' stored vectors are `stored_a` at `+0x1b0` and `stored_b` at `+0x1c0`.
    fn rig(stored_a: [f32; 4], stored_b: [f32; 4]) -> (Engine, Log, u32, u32, u32) {
        let mut e = engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for (address, name) in [
            (SET_POSITION_AND_ROTATION, "set"),
            (ACTIVATE, "activate"),
            (MOTION_SLOT_40, "slot40"),
            (MOTION_SLOT_44, "slot44"),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((name, a.to_vec()));
                Ret::default()
            });
        }
        e.mem.set_f32(TOLERANCE, 0.001);
        for lane in 0..4 {
            e.mem.set_u32(ABSOLUTE_MASK + 4 * lane, 0x7fff_ffff);
        }
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(VELOCITY_VECTOR + 4 * lane as u32, value);
        }
        let motion_vtable = e.mem.alloc(0x80);
        e.mem.set_u32(motion_vtable + 0x40, MOTION_SLOT_40);
        e.mem.set_u32(motion_vtable + 0x44, MOTION_SLOT_44);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(body + 0xe0, motion_vtable);
        for lane in 0..4 {
            e.mem
                .set_f32(body + 0x1b0 + 4 * lane, stored_a[lane as usize]);
            e.mem
                .set_f32(body + 0x1c0 + 4 * lane, stored_b[lane as usize]);
        }
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x1c, 3);
        let bodies = e.mem.alloc(0x10);
        e.mem.set_u32(bodies + 4, body);
        let indices = e.mem.alloc(0x10);
        for (i, value) in [1i32, -1, 0].into_iter().enumerate() {
            e.mem.set_i32(indices + 4 * i as u32, value);
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, owner);
        e.mem.set_u32(this + 0x20, indices);
        e.mem.set_u32(this + 8, bodies);
        (e, log, this, body, bodies)
    }

    fn names(log: &Log) -> Vec<&'static str> {
        log.borrow().iter().map(|(name, _)| *name).collect()
    }

    /// A body whose two vectors already equal the global one only gets its transform; entries
    /// that are negative or point at a null body are skipped; the transforms advance by `0x30`
    /// for every entry, skipped or not.
    #[test]
    fn bodies_at_the_global_velocity_are_only_moved() {
        let (mut e, log, this, body, _) = rig([1.0, 2.0, 3.0, 0.0], [1.0005, 2.0, 3.0, 0.0]);
        let transforms = e.mem.alloc(0x120);
        e.call(0x00ce_1560, &args![this, transforms]);
        assert_eq!(names(&log), vec!["set"]);
        assert_eq!(log.borrow()[0].1, vec![body, transforms, transforms + 0x10]);
    }

    /// A differing first vector activates and calls slot `0x40` with the global's address,
    /// a differing second vector activates and calls slot `0x44`; each lane counts, the
    /// fourth too, and a NaN lane counts as different.
    #[test]
    fn differing_vectors_activate_and_call_the_motion() {
        let (mut e, log, this, body, _) = rig([1.0, 2.0, 3.5, 0.0], [1.0, 2.0, 3.0, 0.0]);
        let transforms = e.mem.alloc(0x120);
        e.call(0x00ce_1560, &args![this, transforms]);
        assert_eq!(names(&log), vec!["set", "activate", "slot40"]);
        assert_eq!(log.borrow()[2].1, vec![body + 0xe0, VELOCITY_VECTOR]);
        let (mut e, log, this, body, _) = rig([1.0, 2.0, 3.0, 0.0], [1.0, 2.0, 3.0, 0.5]);
        let transforms = e.mem.alloc(0x120);
        e.call(0x00ce_1560, &args![this, transforms]);
        assert_eq!(names(&log), vec!["set", "activate", "slot44"]);
        assert_eq!(log.borrow()[2].1, vec![body + 0xe0, VELOCITY_VECTOR]);
        let (mut e, log, this, _, _) = rig([f32::NAN, 2.0, 3.0, 0.0], [1.0, 2.0, 3.0, 0.0]);
        let transforms = e.mem.alloc(0x120);
        e.call(0x00ce_1560, &args![this, transforms]);
        assert_eq!(names(&log), vec!["set", "activate", "slot40"]);
        // Both differ: both calls, in that order.
        let (mut e, log, this, _, _) = rig([9.0; 4], [9.0; 4]);
        let transforms = e.mem.alloc(0x120);
        e.call(0x00ce_1560, &args![this, transforms]);
        assert_eq!(
            names(&log),
            vec!["set", "activate", "slot40", "activate", "slot44"]
        );
    }

    /// A count of zero or less does nothing.
    #[test]
    fn a_count_of_zero_moves_nothing() {
        let (mut e, log, this, _, _) = rig([9.0; 4], [9.0; 4]);
        let owner = e.mem.u32(this + 0x2c);
        e.mem.set_u32(owner + 0x1c, 0);
        let transforms = e.mem.alloc(0x120);
        e.call(0x00ce_1560, &args![this, transforms]);
        assert!(log.borrow().is_empty());
    }
}
