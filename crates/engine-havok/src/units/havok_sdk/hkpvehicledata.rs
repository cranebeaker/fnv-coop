//! `Havok SDK/hkpvehicledata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpboxmotion::{add, mul};
use super::hkpsphereshape::{copy_vec, load_bits, load_vec, store_bits, store_vec};

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, `cdecl`: allocator, array, count, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkpMotion::getMass` (`00c86430`, `this` = the motion at `+0xe0` of the body, mass in `ST0`).
const GET_MASS: u32 = 0x00c8_6430;
/// `hkMatrix3::setMulInverse` (`00cd5610`, `this` the output, two words).
const SET_MUL_INVERSE: u32 = 0x00cd_5610;
/// `hkVehicleFrictionDescriptionInitValues` (`00e593f0`, `cdecl`: the description, the
/// friction block at `+0xa4` of the vehicle data).
const FRICTION_INIT_VALUES: u32 = 0x00e5_93f0;
/// The absolute-value mask (four lanes of `0x7fffffff`).
const ABS_MASK: u32 = 0x0101_7d10;

/// The locals of the function, relative to the frame block: the matrix of the scaled rows
/// (`+0x20`, 3 vectors) and the description handed to `00e593f0` (`+0x50`, `0xa8` bytes).
/// The frame is `0x100` bytes (stack, 16-byte aligned).
const FRAME_SIZE: u32 = 0x100;
const SCALED_ROWS: u32 = 0x20;
const DESCRIPTION: u32 = 0x50;
/// The description: a vector per axle index from `+0` (indices 0 and 1 are the ones the
/// vehicle uses, 2 and up run over the fields that follow), the chassis vector at `+0x20`, the
/// absolute values of the vehicle data rows `+0x50`, `+0x40` and `+0x30` at `+0x30`, `+0x40`
/// and `+0x50`, the matrix `setMulInverse` makes at `+0x60`, the chassis scale at `+0x90`, the
/// per-axle sums from `+0x94` and the per-axle first wheel values from `+0x9c`, then the
/// float of the vehicle data at `+0x80` in `+0xa4`.
const DESCRIPTION_CHASSIS_VECTOR: u32 = 0x20;
const DESCRIPTION_ABS_ROW_2: u32 = 0x30;
const DESCRIPTION_ABS_ROW_1: u32 = 0x40;
const DESCRIPTION_ABS_ROW_0: u32 = 0x50;
const DESCRIPTION_MATRIX: u32 = 0x60;
const DESCRIPTION_CHASSIS_SCALE: u32 = 0x90;
const DESCRIPTION_AXLE_SUMS: u32 = 0x94;
const DESCRIPTION_AXLE_FIRSTS: u32 = 0x9c;
const DESCRIPTION_VEHICLE_FLOAT: u32 = 0xa4;

/// The vehicle data's wheel records are `0x28` bytes (two floats at `+0` and `+4`, the axle
/// index byte at `+0x24`); the wheel transforms are `0x30` bytes (two vectors and, at
/// `+0x20`, a float).
const WHEEL_SIZE: u32 = 0x28;
const WHEEL_AXLE: u32 = 0x24;
const TRANSFORM_SIZE: u32 = 0x30;

/// The absolute values of the four lanes at `address`, by the mask (`ANDPS`).
fn masked_abs(e: &Engine, address: u32, mask: [u32; 4]) -> [u32; 4] {
    let lanes = load_bits(e, address);
    std::array::from_fn(|lane| lanes[lane] & mask[lane])
}

/// [`masked_abs`] as four `float`s.
fn abs_lanes(e: &Engine, address: u32, mask: [u32; 4]) -> [f32; 4] {
    masked_abs(e, address, mask).map(f32::from_bits)
}

// Translated from 00e39ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e39ab0 - the description is a stack block the exe leaves unwritten where no wheel uses an axle index (the vectors of axles 0 and 1 and the per-axle values), and the friction routine reads it; modelled as the zeroed block.
/// `hkpVehicleData::init` (Xbox PDB; `this` in `ECX`, then the two stack words, popped by the
/// callee): `wheel_transforms` points at the pointer to the wheels' `0x30`-byte transforms and
/// `chassis` is the rigid body. Sets the byte at `+0x190`, counts the wheels of each axle
/// into the byte array at `+0x98` (size `+0x9c`, grown with `00c908c0` to the highest axle
/// index plus one when needed), derives the inertia lanes `+0x180..+0x18c` from the divided
/// pairs `+0x68/+0x74`, `+0x60/+0x78` and `+0x64/+0x7c` (weights of the absolute rows
/// `+0x30`, `+0x40` and `+0x50`) times the inverse mass, builds the description (the chassis
/// vector, the absolute rows, `setMulInverse` of the inertia-scaled chassis rows by the
/// chassis rotation, the per-axle wheel values) on the stack and hands it to the friction
/// routine with the block at `+0xa4`. The wheel loop writes through the axle index, which is
/// not range checked (the exe writes the same stack words).
pub fn hkp_vehicle_data_init(e: &mut Engine, this: Ptr, wheel_transforms: u32, chassis: u32) {
    let data = this.addr();
    e.mem.set_u8(data + 0x190, 1);
    // A wheel count that is zero or negative (a signed byte) runs none of the loops.
    let wheel_count = i32::from(e.mem.u8(data + 0x20) as i8);
    let wheels = e.mem.u32(data + 0x8c);
    let mut axle_count = 0i32;
    for index in 0..wheel_count {
        let axle = i32::from(e.mem.u8(wheels + WHEEL_SIZE * index as u32 + WHEEL_AXLE) as i8);
        axle_count = axle_count.max(axle + 1);
    }
    let size = e.mem.u32(data + 0x9c) as i32;
    if axle_count > size {
        let array = data + 0x98;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < axle_count {
            let wanted = (capacity + capacity).max(axle_count);
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 1u32]);
        }
        e.mem.set_u32(array + 4, axle_count as u32);
    }
    let size = e.mem.u32(data + 0x9c) as i32;
    let storage = e.mem.u32(data + 0x98);
    for index in 0..size {
        e.mem.set_u8(storage.wrapping_add(index as u32), 0);
    }
    for index in 0..wheel_count {
        let axle = i32::from(e.mem.u8(wheels + WHEEL_SIZE * index as u32 + WHEEL_AXLE) as i8);
        let slot = storage.wrapping_add(axle as u32);
        let count = e.mem.u8(slot);
        e.mem.set_u8(slot, count.wrapping_add(1));
    }
    // The divided pairs (x87 divisions rounded to `float`), then the weighted sum of the
    // absolute rows; the fourth lane is replaced by 1.0.
    let first = e.mem.f32(data + 0x68) / e.mem.f32(data + 0x74);
    let second = e.mem.f32(data + 0x60) / e.mem.f32(data + 0x78);
    let third = e.mem.f32(data + 0x64) / e.mem.f32(data + 0x7c);
    let mask = load_bits(e, ABS_MASK);
    let weighted = add(
        add(
            mul([second; 4], abs_lanes(e, data + 0x40, mask)),
            mul([first; 4], abs_lanes(e, data + 0x30, mask)),
        ),
        mul([third; 4], abs_lanes(e, data + 0x50, mask)),
    );
    let inertia = [weighted[0], weighted[1], weighted[2], 1.0];
    let mass = e.call(GET_MASS, &args![chassis + 0xe0]).f64();
    let inverse_mass = (1.0f64 / mass) as f32;
    let scaled = mul([inverse_mass; 4], inertia);
    store_vec(e, data + 0x180, scaled);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let description = frame + DESCRIPTION;
        copy_vec(e, chassis + 0x170, description + DESCRIPTION_CHASSIS_VECTOR);
        for (row, lane) in [scaled[0], scaled[1], scaled[2]].into_iter().enumerate() {
            let offset = 0x10 * row as u32;
            let rotation = load_vec(e, chassis + 0xf0 + offset);
            store_vec(e, frame + SCALED_ROWS + offset, mul([lane; 4], rotation));
        }
        e.call(
            SET_MUL_INVERSE,
            &args![
                description + DESCRIPTION_MATRIX,
                frame + SCALED_ROWS,
                chassis + 0xf0
            ],
        );
        let chassis_scale = x87_quiet_bits(e.mem.u32(chassis + 0x1ac));
        e.mem
            .set_u32(description + DESCRIPTION_CHASSIS_SCALE, chassis_scale);
        let vehicle_float = x87_quiet_bits(e.mem.u32(data + 0x80));
        e.mem
            .set_u32(description + DESCRIPTION_VEHICLE_FLOAT, vehicle_float);
        let rows = [
            (0x30, DESCRIPTION_ABS_ROW_0),
            (0x40, DESCRIPTION_ABS_ROW_1),
            (0x50, DESCRIPTION_ABS_ROW_2),
        ];
        for (source, target) in rows {
            let bits = masked_abs(e, data + source, mask);
            store_bits(e, description + target, bits);
        }
        e.mem.set_u32(description + DESCRIPTION_AXLE_SUMS, 0);
        e.mem.set_u32(description + DESCRIPTION_AXLE_SUMS + 4, 0);
        let transforms = e.mem.u32(wheel_transforms);
        for index in 0..wheel_count {
            let wheel = wheels + WHEEL_SIZE * index as u32;
            let transform = transforms + TRANSFORM_SIZE * index as u32;
            let axle = i32::from(e.mem.u8(wheel + WHEEL_AXLE) as i8) as u32;
            let first_value = x87_quiet_bits(e.mem.u32(wheel));
            e.mem.set_u32(
                description + DESCRIPTION_AXLE_FIRSTS + axle.wrapping_mul(4),
                first_value,
            );
            let scale = e.mem.f32(transform + 0x20);
            let moved = add(
                mul([scale; 4], load_vec(e, transform + 0x10)),
                load_vec(e, transform),
            );
            store_vec(e, description + axle.wrapping_mul(0x10), moved);
            // The product stays in extended precision for the addition and is rounded to
            // `float` when stored; the `double` here holds the 48-bit product exactly.
            let product = f64::from(e.mem.f32(wheel + 4)) * f64::from(e.mem.f32(wheel));
            let sum_address = description + DESCRIPTION_AXLE_SUMS + axle.wrapping_mul(4);
            let sum = product + f64::from(e.mem.f32(sum_address));
            e.mem.set_f32(sum_address, sum as f32);
        }
        e.call(FRICTION_INIT_VALUES, &args![description, data + 0xa4]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_9ab0, hkp_vehicle_data_init(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<u32>>>;

    const ALLOCATOR: u32 = 0x7700;
    const MATRIX_PATTERN: u32 = 0x1000;
    const DESCRIPTION_WORDS: u32 = 0x2a;

    struct Scene {
        e: Engine,
        log: Log,
        this: u32,
        chassis: u32,
        transforms: u32,
        description: Words,
        rows: Words,
    }

    fn words(e: &Engine, base: u32, count: u32) -> Vec<u32> {
        (0..count)
            .map(|index| e.mem.u32(base + 4 * index))
            .collect()
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    /// A vehicle with the given wheels `(axle, first value, second value)`, a byte array of
    /// `size` entries (all `0xee`) with capacity word `capacity`, the mass 4 and the doubles.
    fn scene(wheels: &[(i8, f32, f32)], size: u32, capacity: u32) -> Scene {
        let mut e = engine();
        let log = new_log();
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let storage = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(GET_MASS, move |_, a| {
            sink.borrow_mut().push((GET_MASS, a.to_vec()));
            Ret {
                st0: 4.0,
                ..Ret::default()
            }
        });
        let rows: Words = Rc::new(RefCell::new(Vec::new()));
        let (sink, kept) = (Rc::clone(&log), Rc::clone(&rows));
        e.register_double(SET_MUL_INVERSE, move |e, a| {
            sink.borrow_mut().push((SET_MUL_INVERSE, a.to_vec()));
            *kept.borrow_mut() = words(e, a[1], 12);
            for index in 0..12 {
                e.mem.set_u32(a[0] + 4 * index, MATRIX_PATTERN + index);
            }
            Ret::default()
        });
        let description: Words = Rc::new(RefCell::new(Vec::new()));
        let (sink, kept) = (Rc::clone(&log), Rc::clone(&description));
        e.register_double(FRICTION_INIT_VALUES, move |e, a| {
            sink.borrow_mut().push((FRICTION_INIT_VALUES, a.to_vec()));
            *kept.borrow_mut() = words(e, a[0], DESCRIPTION_WORDS);
            Ret::default()
        });
        let this = e.mem.alloc(0x1a0);
        e.mem.set_u8(this + 0x20, wheels.len() as u8);
        let records = e.mem.alloc(WHEEL_SIZE * wheels.len() as u32 + 4);
        let blocks = e.mem.alloc(TRANSFORM_SIZE * wheels.len() as u32 + 4);
        for (index, (axle, first, second)) in wheels.iter().enumerate() {
            let wheel = records + WHEEL_SIZE * index as u32;
            e.mem.set_f32(wheel, *first);
            e.mem.set_f32(wheel + 4, *second);
            e.mem.set_u8(wheel + WHEEL_AXLE, *axle as u8);
            // Every transform: rows (index + 1, 2, 3, 4) and (10, 20, 30, 40), scale 2.
            let transform = blocks + TRANSFORM_SIZE * index as u32;
            store_vec(&mut e, transform, [index as f32 + 1.0, 2.0, 3.0, 4.0]);
            store_vec(&mut e, transform + 0x10, [10.0, 20.0, 30.0, 40.0]);
            e.mem.set_f32(transform + 0x20, 2.0);
        }
        e.mem.set_u32(this + 0x8c, records);
        let storage = e.mem.alloc(16);
        for index in 0..16 {
            e.mem.set_u8(storage + index, 0xee);
        }
        e.mem.set_u32(this + 0x98, storage);
        e.mem.set_u32(this + 0x9c, size);
        e.mem.set_u32(this + 0xa0, capacity);
        store_vec(&mut e, this + 0x30, [1.0, -2.0, 3.0, 0.5]);
        store_vec(&mut e, this + 0x40, [-1.0, 2.0, -3.0, 1.0]);
        store_vec(&mut e, this + 0x50, [2.0, -1.0, -0.0, 4.0]);
        e.mem.set_f32(this + 0x60, 6.0);
        e.mem.set_f32(this + 0x64, 9.0);
        e.mem.set_f32(this + 0x68, 8.0);
        e.mem.set_f32(this + 0x74, 2.0);
        e.mem.set_f32(this + 0x78, 3.0);
        e.mem.set_f32(this + 0x7c, 3.0);
        e.mem.set_f32(this + 0x80, 0.75);
        let chassis = e.mem.alloc(0x1c0);
        store_vec(&mut e, chassis + 0xf0, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, chassis + 0x100, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, chassis + 0x110, [9.0, 10.0, 11.0, 12.0]);
        store_vec(&mut e, chassis + 0x170, [0.5, 1.5, 2.5, 3.5]);
        e.mem.set_f32(chassis + 0x1ac, 7.0);
        let transforms = e.mem.alloc(4);
        e.mem.set_u32(transforms, blocks);
        Scene {
            e,
            log,
            this,
            chassis,
            transforms,
            description,
            rows,
        }
    }

    fn run(scene: &mut Scene) {
        scene.e.call(
            0x00e3_9ab0,
            &args![scene.this, scene.transforms, scene.chassis],
        );
    }

    /// Sets the `float`s `values` from byte offset `offset` of the description words.
    fn put(words: &mut [u32], offset: usize, values: &[f32]) {
        for (lane, value) in values.iter().enumerate() {
            words[offset / 4 + lane] = value.to_bits();
        }
    }

    /// The description for a vehicle without wheels, as `scene` builds it.
    fn base_description() -> Vec<u32> {
        let mut words = vec![0u32; DESCRIPTION_WORDS as usize];
        let vectors = [
            (0x20, [0.5f32, 1.5, 2.5, 3.5]),
            (0x30, [2.0, 1.0, 0.0, 4.0]),
            (0x40, [1.0, 2.0, 3.0, 1.0]),
            (0x50, [1.0, 2.0, 3.0, 0.5]),
        ];
        for (offset, vector) in vectors {
            put(&mut words, offset, &vector);
        }
        for index in 0..12 {
            words[0x60 / 4 + index] = MATRIX_PATTERN + index as u32;
        }
        words[0x90 / 4] = 7.0f32.to_bits();
        words[0xa4 / 4] = 0.75f32.to_bits();
        words
    }

    #[test]
    fn the_inertia_lanes_are_the_weighted_rows_times_the_inverse_mass() {
        let mut s = scene(&[], 0, 0);
        run(&mut s);
        // Weights 2 (row 0x40), 4 (row 0x30), 3 (row 0x50): (12, 15, 18), lane 3 is 1; the
        // mass is 4.
        assert_eq!(load_vec(&s.e, s.this + 0x180), [3.0, 3.75, 4.5, 0.25]);
        assert_eq!(s.e.mem.u8(s.this + 0x190), 1);
        assert_eq!(calls(&s.log, GET_MASS), vec![vec![s.chassis + 0xe0]]);
    }

    #[test]
    fn set_mul_inverse_gets_the_scaled_chassis_rows_and_the_rotation() {
        let mut s = scene(&[], 0, 0);
        run(&mut s);
        let friction = calls(&s.log, FRICTION_INIT_VALUES);
        assert_eq!(friction.len(), 1);
        let description = friction[0][0];
        assert_eq!(friction[0][1], s.this + 0xa4);
        let made = calls(&s.log, SET_MUL_INVERSE);
        assert_eq!(
            made,
            vec![vec![
                description + 0x60,
                description - 0x30,
                s.chassis + 0xf0
            ]]
        );
        assert_eq!(
            *s.rows.borrow(),
            bits(&[3.0, 6.0, 9.0, 12.0, 18.75, 22.5, 26.25, 30.0, 40.5, 45.0, 49.5, 54.0])
        );
    }

    #[test]
    fn the_description_of_a_vehicle_without_wheels() {
        let mut s = scene(&[], 0, 0);
        run(&mut s);
        assert_eq!(*s.description.borrow(), base_description());
    }

    #[test]
    fn signalling_nans_copied_through_the_x87_stack_are_quieted() {
        let mut s = scene(&[(0, f32::from_bits(0x7f80_0003), 1.0)], 2, 2);
        s.e.mem.set_u32(s.chassis + 0x1ac, 0x7f80_0001);
        s.e.mem.set_u32(s.this + 0x80, 0x7f80_0002);
        run(&mut s);
        let got = s.description.borrow().clone();
        assert_eq!(got[0x90 / 4], 0x7fc0_0001);
        assert_eq!(got[0xa4 / 4], 0x7fc0_0002);
        assert_eq!(got[0x9c / 4], 0x7fc0_0003);
    }

    #[test]
    fn wheels_fill_the_per_axle_vectors_and_sums() {
        let wheels = [(0, 2.0, 0.5), (1, 3.0, 4.0), (1, 5.0, 2.0)];
        let mut s = scene(&wheels, 2, 2);
        run(&mut s);
        let mut expected = base_description();
        // Wheel 0 moves to (21, 42, 63, 84); the last wheel of axle 1 (transform 2) wins its
        // vector and first value, and its sum is 2 * 5 + 4 * 3.
        for (offset, values) in [
            (0x00, [21.0f32, 42.0, 63.0, 84.0]),
            (0x10, [23.0, 42.0, 63.0, 84.0]),
        ] {
            put(&mut expected, offset, &values);
        }
        expected[0x94 / 4] = 1.0f32.to_bits();
        expected[0x98 / 4] = 22.0f32.to_bits();
        expected[0x9c / 4] = 2.0f32.to_bits();
        expected[0xa0 / 4] = 5.0f32.to_bits();
        assert_eq!(*s.description.borrow(), expected);
    }

    #[test]
    fn a_wheel_on_axle_two_runs_over_the_chassis_vector_and_the_vehicle_float() {
        let mut s = scene(&[(2, 6.0, 0.5)], 3, 4);
        run(&mut s);
        let mut expected = base_description();
        put(&mut expected, 0x20, &[21.0, 42.0, 63.0, 84.0]);
        expected[0xa4 / 4] = 6.0f32.to_bits();
        // The sum of axle 2 lands in the first-value word of axle 0.
        expected[0x9c / 4] = 3.0f32.to_bits();
        assert_eq!(*s.description.borrow(), expected);
    }

    #[test]
    fn no_wheels_or_a_negative_count_leave_the_description_alone() {
        for count in [0u8, 0x80, 0xff] {
            let mut s = scene(&[], 3, 8);
            s.e.mem.set_u8(s.this + 0x20, count);
            run(&mut s);
            assert_eq!(*s.description.borrow(), base_description(), "{count}");
            assert!(calls(&s.log, MEMORY_ROUTER).is_empty());
            let storage = s.e.mem.u32(s.this + 0x98);
            let kept: Vec<u8> = (0..4).map(|index| s.e.mem.u8(storage + index)).collect();
            assert_eq!(kept, vec![0, 0, 0, 0xee], "{count}");
        }
    }

    /// `(axle of each wheel, size, capacity, reserved count, final size, counts)`.
    type Growth = (Vec<i8>, u32, u32, Option<u32>, u32, Vec<u8>);

    #[test]
    fn the_axle_array_grows_to_the_highest_axle_plus_one() {
        let cases: [Growth; 7] = [
            (vec![0, 1, 1], 0, 0, Some(2), 2, vec![1, 2]),
            (vec![0, 1, 1], 0, 1, Some(2), 2, vec![1, 2]),
            (vec![0, 4], 1, 1, Some(5), 5, vec![1, 0, 0, 0, 1]),
            (vec![0, 4], 1, 3, Some(6), 5, vec![1, 0, 0, 0, 1]),
            (vec![0, 1], 0, 4, None, 2, vec![1, 1]),
            (vec![0, 1], 2, 0, None, 2, vec![1, 1]),
            (vec![0, 1], 5, 8, None, 5, vec![1, 1, 0, 0, 0]),
        ];
        for (index, (axles, size, capacity, reserved, final_size, counts)) in
            cases.into_iter().enumerate()
        {
            let wheels: Vec<(i8, f32, f32)> = axles.iter().map(|axle| (*axle, 1.0, 1.0)).collect();
            let mut s = scene(&wheels, size, capacity);
            let array = s.this + 0x98;
            run(&mut s);
            let wanted: Vec<Vec<u32>> = reserved
                .into_iter()
                .map(|count| vec![ALLOCATOR, array, count, 1])
                .collect();
            assert_eq!(calls(&s.log, ARRAY_RESERVE), wanted, "case {index}");
            let needs_router = size < axles.iter().map(|axle| *axle as u32 + 1).max().unwrap();
            assert_eq!(
                calls(&s.log, MEMORY_ROUTER).len(),
                usize::from(needs_router),
                "case {index}"
            );
            assert_eq!(s.e.mem.u32(s.this + 0x9c), final_size, "case {index}");
            let storage = s.e.mem.u32(s.this + 0x98);
            let kept: Vec<u8> = (0..final_size)
                .map(|entry| s.e.mem.u8(storage + entry))
                .collect();
            assert_eq!(kept, counts, "case {index}");
        }
    }

    #[test]
    fn entries_past_the_size_of_the_array_are_not_cleared() {
        let mut s = scene(&[(0, 1.0, 1.0)], 5, 8);
        run(&mut s);
        let storage = s.e.mem.u32(s.this + 0x98);
        assert_eq!(s.e.mem.u8(storage + 4), 0);
        assert_eq!(s.e.mem.u8(storage + 5), 0xee);
    }
}
