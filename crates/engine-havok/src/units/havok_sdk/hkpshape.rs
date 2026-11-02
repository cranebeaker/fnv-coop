//! `Havok SDK/hkpshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Four `u32` bit masks (`1, 2, 4, 8`): the ray lane `n` is live when its mask intersects the
/// sign bits of the lane mask vector.
const RAY_LANE_MASKS: u32 = 0x010c_92e0;
/// Sixteen 16-byte vectors indexed by a 4-bit lane mask (filled in at start-up by the game):
/// the result of the bundle cast is the vector for the set of lanes that hit.
const LANE_MASK_VECTORS: u32 = 0x0126_8460;
/// The vector `(0.5, 0.5, 0.5, 0.5)`.
const HALF_VECTOR: u32 = 0x010c_71d0;
/// The vector of four sign bits (`0x80000000`).
const SIGN_MASK_VECTOR: u32 = 0x010c_44d0;

// Translated from 00c9e400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShape::castRayBundleImpl` (Xbox PDB): casts four rays against the shape with the single
/// ray cast (virtual slot 0x20 of the shape: `(&hit, &input, &output)`, where `input` is the
/// 0x28-byte ray `from`, `to`, filter info and user data and `output` is the 0x50-byte
/// element of the array at `outputs`).
///
/// `bundle` holds the four rays in structure-of-arrays form: the `x`, `y`, `z` vectors of the
/// start points at +0x00, +0x10, +0x20, those of the end points at +0x30, +0x40, +0x50, the
/// filter info word at +0x60 and the user data word at +0x64. `lanes` is a vector whose sign
/// bits (`MOVMSKPS`) say which rays are live; ray `n` uses `outputs + 0x50 * n`. The sign bits
/// are read again for every ray. The result written to `result` is the 16-byte vector
/// `LANE_MASK_VECTORS[hit mask]`, where the hit mask has bit `n` set for each live ray that
/// hit.
/// Answers `result` (the `hkVector4Comparison` returned through the hidden pointer, Xbox PDB
/// `castRayBundleImpl` returns `?AVhkVector4Comparison`; `EAX` is `result`, loaded at 00c9e547,
/// at the RET 00c9e553). No exe reader of the slot +0x28 was found.
pub fn hkp_shape_cast_ray_bundle_impl(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    bundle: Ptr,
    outputs: u32,
    lanes: Ptr,
) -> Ptr {
    let base = bundle.addr();
    // The transposed rays: lane `n` of each coordinate vector, with 0 as the fourth word.
    let ray_point = |e: &Engine, group: u32, n: u32| -> [u32; 4] {
        [
            e.mem.u32(base + group + 4 * n),
            e.mem.u32(base + group + 0x10 + 4 * n),
            e.mem.u32(base + group + 0x20 + 4 * n),
            0,
        ]
    };
    let rays: Vec<([u32; 4], [u32; 4])> = (0..4)
        .map(|n| (ray_point(e, 0, n), ray_point(e, 0x30, n)))
        .collect();
    let filter_info = e.mem.u32(base + 0x60);
    let user_data = e.mem.u32(base + 0x64);

    // The input (0x28 bytes) and the hit flag the cast returns through its first argument.
    let hits = e.with_stack(0x40, |e, frame| {
        let input = frame.addr();
        let flag = frame.addr() + 0x30;
        let mut hits = 0u32;
        for (n, (from, to)) in rays.iter().enumerate() {
            let lane_mask = e.mem.u32(RAY_LANE_MASKS + 4 * n as u32);
            let signs = (0..4).fold(0u32, |signs, lane| {
                signs | ((e.mem.u32(lanes.addr() + 4 * lane) >> 31) << lane)
            });
            if lane_mask & signs == 0 {
                continue;
            }
            for (word, value) in from.iter().chain(to.iter()).enumerate() {
                e.mem.set_u32(input + 4 * word as u32, *value);
            }
            e.mem.set_u32(input + 0x20, filter_info);
            e.mem.set_u32(input + 0x24, user_data);
            let output = outputs.wrapping_add(0x50 * n as u32);
            e.vcall(this.addr(), 0x20, &args![flag, input, output]);
            if e.mem.u8(flag) != 0 {
                hits |= lane_mask;
            }
        }
        hits
    });
    let vector = e.mem.bytes(LANE_MASK_VECTORS + (hits << 4), 16);
    e.mem.write(result.addr(), &vector);
    result
}

// Translated from 00c9e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShape::getMaximumProjection` (Xbox PDB): the extent of the shape along `direction`.
/// Asks the shape for its bounding box with the identity transform and zero tolerance (virtual
/// slot 0x1c: `(&transform, tolerance, &aabb)`), then, per axis, adds the box's centre and the
/// half-size with the sign of the direction component, multiplies by the component, and sums
/// `z + (y + x)`, all in single precision as the SSE code does.
pub fn hkp_shape_get_maximum_projection(e: &mut Engine, this: Ptr, direction: Ptr) -> f32 {
    // The 64-byte identity `hkTransform` (rotation columns, then a zero translation) and the
    // 32-byte box the call fills in.
    e.with_stack(0x60, |e, frame| {
        let transform = frame.addr();
        let aabb = frame.addr() + 0x40;
        for (offset, value) in [(0x00, 1.0f32), (0x14, 1.0), (0x28, 1.0)] {
            e.mem.set_f32(transform + offset, value);
        }
        e.vcall(this.addr(), 0x1c, &args![transform, 0.0f32.to_bits(), aabb]);
        let mut terms = [0.0f32; 3];
        for (lane, term) in terms.iter_mut().enumerate() {
            let lane = lane as u32;
            let low = e.mem.f32(aabb + 4 * lane);
            let high = e.mem.f32(aabb + 0x10 + 4 * lane);
            let half = e.mem.f32(HALF_VECTOR + 4 * lane);
            let mask = e.mem.u32(SIGN_MASK_VECTOR + 4 * lane);
            let component = e.mem.f32(direction.addr() + 4 * lane);
            let signed_half_size =
                f32::from_bits(((high - low) * half).to_bits() ^ (component.to_bits() & mask));
            *term = (signed_half_size + (low + high) * half) * component;
        }
        terms[2] + (terms[1] + terms[0])
    })
}

// Translated from 00c9e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShape::calcSizeForSpu` (Xbox PDB), thiscall with two stack words (`RET 8`): none of
/// them is read; answers `0xffffffff` (the shape has no size for the SPU).
pub fn hkp_shape_calc_size_for_spu(
    _e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> u32 {
    u32::MAX
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9e3f0, hkp_shape_calc_size_for_spu(u32, u32, u32) -> u32),
        entry!(
            0x00c9e400,
            hkp_shape_cast_ray_bundle_impl(Ptr, Ptr, Ptr, u32, Ptr) -> Ptr
        ),
        entry!(0x00c9e560, hkp_shape_get_maximum_projection(Ptr, Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CAST_RAY_SLOT: u32 = 0x0900_0020;
    const GET_AABB_SLOT: u32 = 0x0900_001c;

    /// An engine with the constants mapped and a shape whose vtable has the two slots.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [0x010c_9000, 0x010c_7000, 0x010c_4000, 0x0126_8000] {
            e.map(page, 0x1000);
        }
        for (i, mask) in [1u32, 2, 4, 8].into_iter().enumerate() {
            e.mem.set_u32(RAY_LANE_MASKS + 4 * i as u32, mask);
        }
        for lane in 0..4 {
            e.mem.set_f32(HALF_VECTOR + 4 * lane, 0.5);
            e.mem.set_u32(SIGN_MASK_VECTOR + 4 * lane, 0x8000_0000);
        }
        // Vector for hit mask `m` holds `m` in every word.
        for mask in 0..16u32 {
            for lane in 0..4 {
                e.mem
                    .set_u32(LANE_MASK_VECTORS + 16 * mask + 4 * lane, mask);
            }
        }
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x1c, GET_AABB_SLOT);
        e.mem.set_u32(table + 0x20, CAST_RAY_SLOT);
        let shape = e.mem.alloc(8);
        e.mem.set_u32(shape, table);
        (e, shape)
    }

    type Casts = Rc<RefCell<Vec<(Vec<u32>, u32)>>>;

    /// A cast double that hits the rays whose index is in `hitting` (found by the output
    /// pointer's offset from `outputs`), logging `(input words, output pointer)`.
    fn cast_double(e: &mut Engine, outputs: u32, hitting: &'static [u32]) -> Casts {
        let casts: Casts = Rc::new(RefCell::new(vec![]));
        let sink = casts.clone();
        e.register_double(CAST_RAY_SLOT, move |e, a| {
            let words = (0..10).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            sink.borrow_mut().push((words, a[3]));
            let ray = (a[3] - outputs) / 0x50;
            e.mem.set_u8(a[1], u8::from(hitting.contains(&ray)));
            Ret::default()
        });
        casts
    }

    /// A bundle with ray `n` going from `(n, 10 + n, 20 + n)` to `(30 + n, 40 + n, 50 + n)`,
    /// filter info 0x11 and user data 0x22.
    fn bundle(e: &mut Engine) -> u32 {
        let bundle = e.mem.alloc(0x70);
        for n in 0..4u32 {
            for (group, base) in [(0u32, 0.0f32), (0x30, 30.0)] {
                for axis in 0..3u32 {
                    let value = base + 10.0 * axis as f32 + n as f32;
                    e.mem.set_f32(bundle + group + 0x10 * axis + 4 * n, value);
                }
            }
        }
        e.mem.set_u32(bundle + 0x60, 0x11);
        e.mem.set_u32(bundle + 0x64, 0x22);
        bundle
    }

    fn lanes(e: &mut Engine, signs: [bool; 4]) -> u32 {
        let lanes = e.mem.alloc(16);
        for (i, negative) in signs.into_iter().enumerate() {
            e.mem
                .set_u32(lanes + 4 * i as u32, if negative { 0x8000_0000 } else { 0 });
        }
        lanes
    }

    #[test]
    fn cast_ray_bundle_casts_the_live_rays_and_reports_the_hits() {
        let (mut e, shape) = engine();
        let bundle = bundle(&mut e);
        let outputs = e.mem.alloc(0x50 * 4);
        let lanes = lanes(&mut e, [true, false, true, true]);
        let result = e.mem.alloc(16);
        let casts = cast_double(&mut e, outputs, &[2]);
        e.call(0x00c9e400, &args![shape, result, bundle, outputs, lanes]);
        let casts = casts.borrow();
        // Rays 0, 2 and 3 are live; only ray 2 hits.
        assert_eq!(casts.len(), 3);
        let ray = |n: u32| -> Vec<u32> {
            let f = |v: f32| v.to_bits();
            vec![
                f(n as f32),
                f(10.0 + n as f32),
                f(20.0 + n as f32),
                0,
                f(30.0 + n as f32),
                f(40.0 + n as f32),
                f(50.0 + n as f32),
                0,
                0x11,
                0x22,
            ]
        };
        assert_eq!(casts[0], (ray(0), outputs));
        assert_eq!(casts[1], (ray(2), outputs + 0x50 * 2));
        assert_eq!(casts[2], (ray(3), outputs + 0x50 * 3));
        for lane in 0..4 {
            assert_eq!(e.mem.u32(result + 4 * lane), 4);
        }
    }

    #[test]
    fn cast_ray_bundle_without_live_rays_casts_nothing() {
        let (mut e, shape) = engine();
        let bundle = bundle(&mut e);
        let outputs = e.mem.alloc(0x50 * 4);
        let lanes = lanes(&mut e, [false; 4]);
        let result = e.mem.alloc(16);
        let casts = cast_double(&mut e, outputs, &[0, 1, 2, 3]);
        e.call(0x00c9e400, &args![shape, result, bundle, outputs, lanes]);
        assert!(casts.borrow().is_empty());
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn cast_ray_bundle_answers_the_result_pointer() {
        let (mut e, shape) = engine();
        let bundle = bundle(&mut e);
        let outputs = e.mem.alloc(0x50 * 4);
        let lanes = lanes(&mut e, [false; 4]);
        let result = e.mem.alloc(16);
        cast_double(&mut e, outputs, &[]);
        // EAX is `result` (00c9e547) at the RET 00c9e553.
        let answer = e.call(0x00c9e400, &args![shape, result, bundle, outputs, lanes]);
        assert_eq!(answer.u32(), result);
    }

    #[test]
    fn cast_ray_bundle_ors_the_masks_of_all_hits() {
        let (mut e, shape) = engine();
        let bundle = bundle(&mut e);
        let outputs = e.mem.alloc(0x50 * 4);
        let lanes = lanes(&mut e, [true; 4]);
        let result = e.mem.alloc(16);
        cast_double(&mut e, outputs, &[0, 1, 3]);
        e.call(0x00c9e400, &args![shape, result, bundle, outputs, lanes]);
        assert_eq!(e.mem.u32(result + 12), 1 | 2 | 8);
    }

    /// A shape whose bounding box is `min`..`max`; the double records the arguments of the
    /// box call.
    fn projection_scene(e: &mut Engine, min: [f32; 3], max: [f32; 3]) -> Rc<RefCell<Vec<u32>>> {
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(GET_AABB_SLOT, move |e, a| {
            // The transform must be the identity with a zero translation.
            let identity: Vec<u32> = (0..16).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            sink.borrow_mut().extend(identity);
            sink.borrow_mut().push(a[2]);
            for lane in 0..3u32 {
                e.mem.set_f32(a[3] + 4 * lane, min[lane as usize]);
                e.mem.set_f32(a[3] + 0x10 + 4 * lane, max[lane as usize]);
            }
            Ret::default()
        });
        seen
    }

    fn direction(e: &mut Engine, values: [f32; 3]) -> u32 {
        let direction = e.mem.alloc(16);
        for (i, value) in values.into_iter().enumerate() {
            e.mem.set_f32(direction + 4 * i as u32, value);
        }
        direction
    }

    #[test]
    fn maximum_projection_is_the_far_corner_along_the_direction() {
        let (mut e, shape) = engine();
        let seen = projection_scene(&mut e, [-1.0, -2.0, -3.0], [3.0, 2.0, 1.0]);
        let direction = direction(&mut e, [1.0, -1.0, 2.0]);
        let projection = e.call(0x00c9e560, &args![shape, direction]).f32();
        assert_eq!(projection, 7.0);
        let seen = seen.borrow();
        let one = 1.0f32.to_bits();
        assert_eq!(
            seen[..16],
            [one, 0, 0, 0, 0, one, 0, 0, 0, 0, one, 0, 0, 0, 0, 0]
        );
        // The tolerance word is 0.0.
        assert_eq!(seen[16], 0);
    }

    #[test]
    fn maximum_projection_of_a_negative_direction_uses_the_near_corner() {
        let (mut e, shape) = engine();
        projection_scene(&mut e, [-1.0, -2.0, -3.0], [3.0, 2.0, 1.0]);
        let direction = direction(&mut e, [-1.0, -1.0, -1.0]);
        let projection = e.call(0x00c9e560, &args![shape, direction]).f32();
        // 1 + 2 + 3.
        assert_eq!(projection, 6.0);
    }

    #[test]
    fn calc_size_for_spu_00c9e3f0_answers_all_ones() {
        let (mut e, _) = engine();
        let got = e.call(0x00c9e3f0, &args![0x1234u32, 1u32, 2u32]);
        assert_eq!(got.u32(), 0xffff_ffff);
    }
}
