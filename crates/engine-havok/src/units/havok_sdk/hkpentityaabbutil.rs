//! `Havok SDK/hkpentityaabbutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::hkpsphereshape::{load_vec, Vec4};

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// `0.5` as a `double` (`01011588`): the tolerance factor.
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `0.5` as a `float` (`01016248`) and the four-lane `1.0` vector (`010ec7f0`): the centre blend.
const HALF: u32 = 0x0101_6248;
const ONES: u32 = 0x010e_c7f0;
/// The four-lane upper clamp of the quantised coordinates (`011b6d70`, `0x4efffc00` in the exe
/// image) and the four-lane lower clamp (`01267e30`, a vector in `.data`, zero in the image).
const UPPER_CLAMP: u32 = 0x011b_6d70;
const LOWER_CLAMP: u32 = 0x0126_7e30;

/// `00c85750` (`Havok SDK/hkmemoryrouter.cpp`, no arguments): answers the thread's TLS value;
/// this function bumps the word it points at by aligned block sizes and later puts it back,
/// using the old values as scratch arrays.
const THREAD_ROUTER: u32 = 0x00c8_5750;
/// `hkAabbUtil::initOffsetAabbInput` (Xbox PDB, cdecl `(motion state, output)`).
const INIT_OFFSET_AABB_INPUT: u32 = 0x00cb_ec90;
/// `00d1b1d0` (no map name, cdecl four words): quicksort of 8-byte pairs by their first word.
const SORT_PAIRS: u32 = 0x00d1_b1d0;

/// `hkpCollisionInput` (Xbox PDB): `m_tolerance` +0x08 and `m_aabb32Info` +0x20
/// (`m_bitOffsetLow` +0x20, `m_bitOffsetHigh` +0x30, `m_bitScale` +0x40).
const INPUT_TOLERANCE: u32 = 0x08;
const INPUT_OFFSET_LOW: u32 = 0x20;
const INPUT_OFFSET_HIGH: u32 = 0x30;
const INPUT_SCALE: u32 = 0x40;

/// Entity fields the code uses (PC offsets): the collidable's shape (`m_collidable` +0x10,
/// `hkpCdBody::m_shape`), its `BoundingVolumeData` from +0x30 (`m_min` +0x30, `m_max` +0x40,
/// `m_numChildShapeAabbs` +0x50, `m_capacityChildShapeAabbs` +0x52, `m_childShapeAabbs` +0x54,
/// `m_childShapeKeys` +0x58, Xbox PDB names of `hkpCollidable::BoundingVolumeData`) and the
/// motion state at +0xf0, used with the `hkMotionState` layout (Xbox PDB): centres of mass
/// at +0x40 and +0x50, `m_deltaAngle` +0x90 (last lane +0x9c), `m_objectRadius` +0xa0.
const ENTITY_SHAPE: u32 = 0x10;
const ENTITY_VOLUME: u32 = 0x30;
const ENTITY_NUM_CHILD_AABBS: u32 = 0x50;
const ENTITY_CAPACITY_CHILD_AABBS: u32 = 0x52;
const ENTITY_CHILD_AABBS: u32 = 0x54;
const ENTITY_CHILD_KEYS: u32 = 0x58;
const ENTITY_MOTION: u32 = 0xf0;
const MOTION_CENTRE_0: u32 = 0x40;
const MOTION_CENTRE_1: u32 = 0x50;
const MOTION_DELTA_ANGLE_W: u32 = 0x9c;
const MOTION_OBJECT_RADIUS: u32 = 0xa0;
/// The motion state's local centre of mass (`m_sweptTransform` +0x40 of it, at +0x80).
const MOTION_LOCAL_CENTRE: u32 = 0x80;

/// Virtual slots: `hkpShape::getContainer` +0x10 and `getAabbImpl` +0x1c; `hkpShapeContainer`
/// (Xbox PDB) `getNumChildShapes` +0x04, `getFirstKey` +0x08, `getNextKey` +0x0c,
/// `getChildShape` +0x14.
const SHAPE_GET_CONTAINER: u32 = 0x10;
const SHAPE_GET_AABB: u32 = 0x1c;
const CONTAINER_NUM_CHILDREN: u32 = 0x04;
const CONTAINER_FIRST_KEY: u32 = 0x08;
const CONTAINER_NEXT_KEY: u32 = 0x0c;
const CONTAINER_CHILD_SHAPE: u32 = 0x14;

/// The function's stack frame (`AND ESP,-16; SUB ESP,0x5e4` and three saved registers: 0x5f0
/// bytes, 16-byte aligned) and the locals it passes by address: the shape's AABB (+0x20, min
/// then max), the `initOffsetAabbInput` output (+0x240: the motion state pointer at +0, step
/// transforms of 0x40 bytes from +0x10, their count at +0x110, a first transform at +0x120, an
/// inverse transform at +0x160), the child shape buffer (+0x3e0) and the sort's fourth word
/// (+0x124).
const FRAME_SIZE: u32 = 0x5f0;
const FRAME_AABB: u32 = 0x20;
const FRAME_OFFSET_INPUT: u32 = 0x240;
const FRAME_CHILD_BUFFER: u32 = 0x3e0;
const FRAME_SORT_WORD: u32 = 0x124;
const OFFSET_INPUT_STEPS: u32 = 0x10;
const OFFSET_INPUT_STEP_COUNT: u32 = 0x110;
const OFFSET_INPUT_FIRST: u32 = 0x120;
const OFFSET_INPUT_INVERSE: u32 = 0x160;

fn splat(value: f32) -> Vec4 {
    [value; 4]
}

fn add(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] + b[i])
}

fn sub(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] - b[i])
}

fn mul(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] * b[i])
}

/// `MINPS a, b`: each lane `a` when `a < b`, else `b` (NaN and equal lanes give `b`).
fn min4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| if a[i] < b[i] { a[i] } else { b[i] })
}

/// `MAXPS a, b`: each lane `a` when `a > b`, else `b`.
fn max4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| if a[i] > b[i] { a[i] } else { b[i] })
}

/// The `SHUFPS` / `MULPS` / `ADDPS` transform of `p` by the three rotation rows and the
/// translation row at `at`: `(y * row1 + x * row0) + (z * row2 + translation)`.
fn transform(e: &Engine, at: u32, p: Vec4) -> Vec4 {
    let (row0, row1, row2) = (
        load_vec(e, at),
        load_vec(e, at + 0x10),
        load_vec(e, at + 0x20),
    );
    let translation = load_vec(e, at + 0x30);
    let yx = add(mul(splat(p[1]), row1), mul(splat(p[0]), row0));
    add(yx, add(mul(splat(p[2]), row2), translation))
}

/// `MULPS` by the bit scale, `MINPS` by the upper clamp, `MAXPS` by the lower clamp, then each
/// lane through the stack (`FLD dword`, `FSTP dword`, `FLD dword`, `FISTP`).
fn quantise(e: &Engine, input: u32, sum: Vec4) -> [i32; 4] {
    let scaled = mul(sum, load_vec(e, input + INPUT_SCALE));
    let clamped = max4(
        min4(scaled, load_vec(e, UPPER_CLAMP)),
        load_vec(e, LOWER_CLAMP),
    );
    clamped.map(|lane| crate::x87::fistp(x87_load32(lane.to_bits())))
}

/// The expansion shift: the OR of the six differences, `SHR 4` while it is at least 0x800
/// and `SHR 1` while at least 0x100 (signed compares, so a negative OR gives 0).
fn expansion_shift(bits: u32) -> u32 {
    let mut value = bits;
    let mut shift = 0u32;
    if value as i32 >= 0x800 {
        loop {
            value >>= 4;
            shift += 4;
            if (value as i32) < 0x800 {
                break;
            }
        }
    }
    if value as i32 >= 0x100 {
        loop {
            value >>= 1;
            shift += 1;
            if (value as i32) < 0x100 {
                break;
            }
        }
    }
    shift
}

/// Writes a quantised AABB at `at` (`hkAabbUint32` / `BoundingVolumeData` head: the min words
/// at +0, the max words at +0x10) and its expansion: the differences `min - expanded min` and
/// `expanded max - max` shifted right (`SAR`) by the shift, in the bytes +0xc..+0xe and
/// +0x1c..+0x1e, the shift in +0xf.
fn store_quantised(
    e: &mut Engine,
    at: u32,
    min: [i32; 4],
    max: [i32; 4],
    low: [i32; 4],
    high: [i32; 4],
) {
    for lane in 0..4u32 {
        e.mem.set_i32(at + 4 * lane, min[lane as usize]);
    }
    for lane in 0..4u32 {
        e.mem.set_i32(at + 0x10 + 4 * lane, max[lane as usize]);
    }
    let below = [0, 1, 2].map(|i| min[i].wrapping_sub(low[i]));
    let above = [0, 1, 2].map(|i| high[i].wrapping_sub(max[i]));
    let bits = below
        .iter()
        .chain(above.iter())
        .fold(0u32, |acc, d| acc | *d as u32);
    let shift = expansion_shift(bits);
    e.mem.set_u8(at + 0xf, shift as u8);
    for i in 0..3u32 {
        e.mem
            .set_u8(at + 0xc + i, (below[i as usize] >> (shift & 31)) as u8);
        e.mem
            .set_u8(at + 0x1c + i, (above[i as usize] >> (shift & 31)) as u8);
    }
}

// Translated from 00d1a330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (Xbox PDB), cdecl `(collision input, entities,
/// count)`: for each of the `count` entity pointers at `entities`, recomputes the quantised
/// AABB of its collidable (`m_min`, `m_max` and the expansion bytes toward the swept volume)
/// with half the input's tolerance.
///
/// Without child AABB capacity (+0x52 zero) the shape's `getAabbImpl` gives the AABB; the
/// expansion takes the AABB grown by `deltaAngle.w * objectRadius`, limited to the second
/// centre of mass grown by `objectRadius + tolerance / 2`, and extended by the move from the
/// second centre of mass to the first.
///
/// With capacity the shape's container is walked key by key from `getFirstKey` to the key -1.
/// Each child's AABB (`getAabbImpl` on the `getChildShape` result, with the child buffer)
/// goes into a scratch array of 32-byte quantised AABBs (the key's low byte in the last byte)
/// and its key into a key array, both carved from the word the thread's router value points
/// at; its centre goes through the inverse, the first and each step transform of
/// `initOffsetAabbInput`, and the swept box, grown per lane by
/// `min(length(half extent), deltaAngle.w * length + half extent)`, is the expansion. The
/// entity's AABB and expansion are the union of the children's, starting from the second
/// centre of mass. The number of keys goes into +0x50; when the container has children the
/// scratch AABBs are sorted by their first min word (`00d1b1d0`, on (word, index) pairs) and
/// copied in that order into `m_childShapeAabbs`, the keys into `m_childShapeKeys`. Every
/// scratch array is given back in reverse order. The keys are not bounded by
/// `getNumChildShapes`, as in the code.
pub fn hkp_entity_aabb_util_entity_batch_recalc_aabb(
    e: &mut Engine,
    input: Ptr,
    entities: Ptr,
    count: i32,
) {
    let input = input.addr();
    let tolerance = x87_load32(e.mem.u32(input + INPUT_TOLERANCE));
    let half_tolerance = x87_store32(x87_mul(tolerance, e.mem.f64(HALF_DOUBLE)));
    if count <= 0 {
        return;
    }
    // The exe aligns ESP, then `SUB ESP,0x5e4` and three pushes: the frame is the 0x5f0 bytes
    // under the 16-byte boundary (00d1a333..00d1a348), so its base is aligned too.
    with_aligned_frame(e, FRAME_SIZE, |e, frame| {
        let mut index = 0i32;
        while index < count {
            let entity = e
                .mem
                .u32(entities.addr().wrapping_add((index as u32).wrapping_mul(4)));
            let shape = e.mem.u32(entity + ENTITY_SHAPE);
            if e.mem.u16(entity + ENTITY_CAPACITY_CHILD_AABBS) == 0 {
                recalc_single(e, input, entity, shape, half_tolerance, frame);
            } else {
                recalc_children(e, input, entity, shape, half_tolerance, frame);
            }
            index += 1;
        }
    });
}

/// The branch at 00d1ae20: an entity without child AABB capacity.
fn recalc_single(
    e: &mut Engine,
    input: u32,
    entity: u32,
    shape: u32,
    half_tolerance: u32,
    frame: u32,
) {
    let motion = entity + ENTITY_MOTION;
    let out = frame + FRAME_AABB;
    e.vcall(
        shape,
        SHAPE_GET_AABB,
        &args![motion, x87_quiet_bits(half_tolerance), out],
    );
    // FLD [motion + 0x9c]; FMUL [motion + 0xa0]; FSTP.
    let angle_radius = f32::from_bits(x87_store32(x87_mul(
        x87_load32(e.mem.u32(motion + MOTION_DELTA_ANGLE_W)),
        x87_m32(e.mem.u32(motion + MOTION_OBJECT_RADIUS)),
    )));
    let min = load_vec(e, out);
    let centre_1 = load_vec(e, motion + MOTION_CENTRE_1);
    let max = load_vec(e, out + 0x10);
    // FLD [motion + 0xa0]; FADD half tolerance; FSTP.
    let reach = f32::from_bits(x87_store32(x87_add(
        x87_load32(e.mem.u32(motion + MOTION_OBJECT_RADIUS)),
        x87_m32(half_tolerance),
    )));
    let grown_min = sub(min, splat(angle_radius));
    let grown_max = add(splat(angle_radius), max);
    let reach_min = sub(centre_1, splat(reach));
    let reach_max = add(splat(reach), centre_1);
    let upper = max4(min4(grown_max, reach_max), max);
    let lower = min4(max4(grown_min, reach_min), min);
    let moved = sub(load_vec(e, motion + MOTION_CENTRE_0), centre_1);
    let back = min4(splat(0.0), moved);
    let ahead = max4(splat(0.0), moved);
    let low = load_vec(e, input + INPUT_OFFSET_LOW);
    let min_words = quantise(e, input, add(min, low));
    let expanded_min = add(back, lower);
    let expanded_max = add(ahead, upper);
    let high = load_vec(e, input + INPUT_OFFSET_HIGH);
    let max_words = quantise(e, input, add(high, max));
    let low_words = quantise(e, input, add(expanded_min, low));
    let high_words = quantise(e, input, add(high, expanded_max));
    store_quantised(
        e,
        entity + ENTITY_VOLUME,
        min_words,
        max_words,
        low_words,
        high_words,
    );
}

/// Takes `size` bytes from the word the router value points at: answers the old value and
/// advances it by `(size + 0x7f) & !0x7f`.
fn carve(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(THREAD_ROUTER, &[]).u32();
    let top = e.mem.u32(router);
    e.mem
        .set_u32(router, (size.wrapping_add(0x7f) & !0x7f).wrapping_add(top));
    top
}

/// The branch at 00d1a379: an entity with child AABB capacity.
fn recalc_children(
    e: &mut Engine,
    input: u32,
    entity: u32,
    shape: u32,
    half_tolerance: u32,
    frame: u32,
) {
    let motion = entity + ENTITY_MOTION;
    let container = e.vcall(shape, SHAPE_GET_CONTAINER, &[]).u32();
    let children = e.vcall(container, CONTAINER_NUM_CHILDREN, &[]).u32();
    let aabbs = carve(e, children << 5);
    let keys = carve(e, children.wrapping_mul(4));
    let offset_input = frame + FRAME_OFFSET_INPUT;
    e.call(INIT_OFFSET_AABB_INPUT, &args![motion, offset_input]);
    let start = load_vec(e, motion + MOTION_CENTRE_1);
    // The four accumulators (stack +0x200 .. +0x230), all the second centre of mass.
    let mut union_expanded_min = start;
    let mut union_expanded_max = start;
    let mut union_min = start;
    let mut union_max = start;
    let mut key_at = keys;
    let mut key = e.vcall(container, CONTAINER_FIRST_KEY, &[]).u32();
    let mut slot = aabbs;
    while key != u32::MAX {
        let child = e
            .vcall(
                container,
                CONTAINER_CHILD_SHAPE,
                &args![key, frame + FRAME_CHILD_BUFFER],
            )
            .u32();
        let out = frame + FRAME_AABB;
        e.vcall(
            child,
            SHAPE_GET_AABB,
            &args![motion, x87_quiet_bits(half_tolerance), out],
        );
        let max = load_vec(e, out + 0x10);
        let min = load_vec(e, out);
        let half = splat(e.mem.f32(HALF));
        let rest = sub(load_vec(e, ONES), half);
        let state = e.mem.u32(offset_input);
        let half_extent = mul(sub(max, min), half);
        let centre = add(mul(rest, max), mul(half, min));
        let local = transform(e, offset_input + OFFSET_INPUT_INVERSE, centre);
        let first = transform(e, offset_input + OFFSET_INPUT_FIRST, local);
        let body_centre = load_vec(e, state + MOTION_LOCAL_CENTRE);
        let mut swept_max = max4(centre, first);
        let mut swept_min = min4(centre, first);
        let steps = e.mem.i32(offset_input + OFFSET_INPUT_STEP_COUNT);
        if steps > 0 {
            let along = sub(local, body_centre);
            let mut step = offset_input + OFFSET_INPUT_STEPS;
            for _ in 0..steps {
                // The step's fraction: the last lane of its translation row.
                let fraction = e.mem.f32(step + 0x3c);
                let point = add(mul(splat(fraction), along), body_centre);
                let moved = transform(e, step, point);
                step += 0x40;
                swept_min = min4(swept_min, moved);
                swept_max = max4(swept_max, moved);
            }
        }
        let squares = mul(half_extent, half_extent);
        let length = (squares[2] + (squares[1] + squares[0])).sqrt();
        let angle = e.mem.f32(state + MOTION_DELTA_ANGLE_W);
        let growth = min4(
            splat(length),
            add(mul(splat(angle), splat(length)), half_extent),
        );
        union_min = min4(union_min, min);
        union_max = max4(union_max, max);
        swept_min = sub(swept_min, growth);
        swept_max = add(swept_max, growth);
        let expanded_min = min4(min, swept_min);
        let low = load_vec(e, input + INPUT_OFFSET_LOW);
        union_expanded_min = min4(union_expanded_min, expanded_min);
        let expanded_max = max4(max, swept_max);
        union_expanded_max = max4(union_expanded_max, expanded_max);
        let min_words = quantise(e, input, add(min, low));
        let high = load_vec(e, input + INPUT_OFFSET_HIGH);
        let max_words = quantise(e, input, add(max, high));
        let low_words = quantise(e, input, add(expanded_min, low));
        let high_words = quantise(e, input, add(expanded_max, high));
        store_quantised(e, slot, min_words, max_words, low_words, high_words);
        e.mem.set_u8(slot + 0x1f, key as u8);
        slot = slot.wrapping_add(0x20);
        e.mem.set_u32(key_at, key);
        key_at = key_at.wrapping_add(4);
        key = e.vcall(container, CONTAINER_NEXT_KEY, &args![key]).u32();
    }
    // 00d1a9dd: SUB, SAR 2, the low word into m_numChildShapeAabbs.
    let found = (key_at.wrapping_sub(keys) as i32) >> 2;
    e.mem.set_u16(entity + ENTITY_NUM_CHILD_AABBS, found as u16);
    let low = load_vec(e, input + INPUT_OFFSET_LOW);
    let min_words = quantise(e, input, add(union_min, low));
    let high = load_vec(e, input + INPUT_OFFSET_HIGH);
    let max_words = quantise(e, input, add(high, union_max));
    let low_words = quantise(e, input, add(union_expanded_min, low));
    let high_words = quantise(e, input, add(union_expanded_max, high));
    store_quantised(
        e,
        entity + ENTITY_VOLUME,
        min_words,
        max_words,
        low_words,
        high_words,
    );
    let n = children as i32;
    if n != 0 {
        let pairs = carve(e, children.wrapping_mul(8));
        if n > 1 {
            for i in 0..n {
                let pair = pairs.wrapping_add((i as u32).wrapping_mul(8));
                e.mem.set_i32(pair + 4, i);
                let first_word = e.mem.u32(aabbs.wrapping_add((i as u32).wrapping_mul(0x20)));
                e.mem.set_u32(pair, first_word);
            }
            // MOV byte [ESP+0x124],0, then the whole word is pushed: its upper three bytes are
            // whatever the frame holds there (this function never writes them).
            e.mem.set_u8(frame + FRAME_SORT_WORD, 0);
            let word = e.mem.u32(frame + FRAME_SORT_WORD);
            e.call(SORT_PAIRS, &args![pairs, 0u32, (n - 1) as u32, word]);
        } else if n == 1 {
            e.mem.set_u32(pairs + 4, 0);
        }
        let mut keys_out = e.mem.u32(entity + ENTITY_CHILD_KEYS);
        let mut aabbs_out = e.mem.u32(entity + ENTITY_CHILD_AABBS);
        if n > 0 {
            for i in 0..n {
                let which = e
                    .mem
                    .u32(pairs.wrapping_add((i as u32).wrapping_mul(8)) + 4);
                let from = (which << 5).wrapping_add(aabbs);
                for word in 0..8u32 {
                    let value = e.mem.u32(from + 4 * word);
                    e.mem.set_u32(aabbs_out + 4 * word, value);
                }
                aabbs_out = aabbs_out.wrapping_add(0x20);
                let key_value = e.mem.u32(keys.wrapping_add(which.wrapping_mul(4)));
                e.mem.set_u32(keys_out, key_value);
                keys_out = keys_out.wrapping_add(4);
            }
        }
        let router = e.call(THREAD_ROUTER, &[]).u32();
        e.mem.set_u32(router, pairs);
    }
    let router = e.call(THREAD_ROUTER, &[]).u32();
    e.mem.set_u32(router, keys);
    let router = e.call(THREAD_ROUTER, &[]).u32();
    e.mem.set_u32(router, aabbs);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d1a330,
        hkp_entity_aabb_util_entity_batch_recalc_aabb(Ptr, Ptr, i32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test double addresses (not exe functions).
    const GET_AABB_SINGLE: u32 = 0x0000_a010;
    const GET_CONTAINER: u32 = 0x0000_a020;
    const NUM_CHILDREN: u32 = 0x0000_a030;
    const FIRST_KEY: u32 = 0x0000_a040;
    const NEXT_KEY: u32 = 0x0000_a050;
    const CHILD_SHAPE: u32 = 0x0000_a060;
    const GET_AABB_CHILD: u32 = 0x0000_a070;

    type SortLog = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    fn map_constants(e: &mut Engine) {
        for page in [
            0x0101_1000,
            0x0101_6000,
            0x010e_c000,
            0x011b_6000,
            0x0126_7000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(HALF, 0.5f32);
        for lane in 0..4u32 {
            e.set_global(ONES + 4 * lane, 1.0f32);
            e.set_global(UPPER_CLAMP + 4 * lane, 0x4eff_fc00u32);
            e.set_global(LOWER_CLAMP + 4 * lane, 0u32);
        }
    }

    fn set_vec(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (lane, value) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, value);
        }
    }

    fn make_input(e: &mut Engine, low: f32, scale: f32) -> u32 {
        let input = e.mem.alloc(0x50);
        e.mem.set_f32(input + INPUT_TOLERANCE, 2.0);
        set_vec(e, input + INPUT_OFFSET_LOW, [low; 4]);
        set_vec(e, input + INPUT_OFFSET_HIGH, [0.0; 4]);
        set_vec(e, input + INPUT_SCALE, [scale; 4]);
        input
    }

    /// An object whose first word is a vtable with `slots` (byte offset, target).
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x20);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let this = e.mem.alloc(size);
        e.mem.set_u32(this, vtable);
        this
    }

    /// `getAabbImpl` double: copies the min and max kept in the shape at +0x10 / +0x20.
    fn get_aabb_from_shape(e: &mut Engine, a: &[u32]) -> Ret {
        for word in 0..8 {
            let value = e.mem.u32(a[0] + 0x10 + 4 * word);
            e.mem.set_u32(a[3] + 4 * word, value);
        }
        Ret::default()
    }

    fn word_ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn num_children(e: &mut Engine, a: &[u32]) -> Ret {
        word_ret(e.mem.u32(a[0] + 0xc))
    }

    fn first_key(_: &mut Engine, _: &[u32]) -> Ret {
        word_ret(7)
    }

    fn next_key(_: &mut Engine, a: &[u32]) -> Ret {
        word_ret(if a[1] == 7 { 3 } else { u32::MAX })
    }

    fn child_shape(e: &mut Engine, a: &[u32]) -> Ret {
        let at = if a[1] == 7 { 4 } else { 8 };
        word_ret(e.mem.u32(a[0] + at))
    }

    fn get_container(e: &mut Engine, a: &[u32]) -> Ret {
        word_ret(e.mem.u32(a[0] + 4))
    }

    /// `initOffsetAabbInput` double: identity first and inverse transforms, one identity step
    /// whose translation's last lane (the step fraction) is 0.5.
    fn init_offset_input(e: &mut Engine, a: &[u32]) -> Ret {
        let out = a[1];
        e.mem.set_u32(out, a[0]);
        for base in [
            out + OFFSET_INPUT_STEPS,
            out + OFFSET_INPUT_FIRST,
            out + OFFSET_INPUT_INVERSE,
        ] {
            set_vec(e, base, [1.0, 0.0, 0.0, 0.0]);
            set_vec(e, base + 0x10, [0.0, 1.0, 0.0, 0.0]);
            set_vec(e, base + 0x20, [0.0, 0.0, 1.0, 0.0]);
            set_vec(e, base + 0x30, [0.0; 4]);
        }
        e.mem.set_f32(out + OFFSET_INPUT_STEPS + 0x3c, 0.5);
        e.mem.set_i32(out + OFFSET_INPUT_STEP_COUNT, 1);
        Ret::default()
    }

    fn ints(e: &Engine, at: u32) -> [i32; 3] {
        [0, 1, 2].map(|i| e.mem.i32(at + 4 * i))
    }

    #[test]
    fn recalc_without_children_quantises_the_aabb_and_its_swept_expansion() {
        let mut e = Engine::new();
        map_constants(&mut e);
        e.register(GET_AABB_SINGLE, get_aabb_from_shape);
        let input = make_input(&mut e, 0.0, 1000.0);
        let shape = object(&mut e, 0x30, &[(SHAPE_GET_AABB, GET_AABB_SINGLE)]);
        set_vec(&mut e, shape + 0x10, [1.0, 2.0, 3.0, 0.0]);
        set_vec(&mut e, shape + 0x20, [5.0, 6.0, 7.0, 0.0]);
        let entity = e.mem.alloc(0x200);
        e.mem.set_u32(entity + ENTITY_SHAPE, shape);
        let motion = entity + ENTITY_MOTION;
        set_vec(&mut e, motion + MOTION_CENTRE_0, [1.0, 4.0, 9.0, 0.0]);
        set_vec(&mut e, motion + MOTION_CENTRE_1, [3.0, 4.0, 5.0, 0.0]);
        e.mem.set_f32(motion + MOTION_DELTA_ANGLE_W, 2.0);
        e.mem.set_f32(motion + MOTION_OBJECT_RADIUS, 0.5);
        let entities = e.mem.alloc(4);
        e.mem.set_u32(entities, entity);

        e.call_log = Some(vec![]);
        e.call(0x00d1a330, &args![input, entities, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != GET_AABB_SINGLE));
        assert_eq!(e.mem.u32(entity + 0x30), 0);

        e.call_log = Some(vec![]);
        e.call(0x00d1a330, &args![input, entities, 1u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == GET_AABB_SINGLE)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], shape);
        assert_eq!(calls[0].1[1], motion);
        assert_eq!(calls[0].1[2], 1.0f32.to_bits());

        assert_eq!(ints(&e, entity + 0x30), [1000, 2000, 3000]);
        assert_eq!(ints(&e, entity + 0x40), [5000, 6000, 7000]);
        // Below: 1000, 0, 0; above: 0, 0, 4000; their OR 0xfe8 gives the shift 4.
        assert_eq!(e.mem.bytes(entity + 0x3c, 4), vec![62, 0, 0, 4]);
        assert_eq!(e.mem.bytes(entity + 0x4c, 4), vec![0, 0, 250, 0]);
    }

    #[test]
    fn the_batch_frame_is_0x5f0_bytes_under_a_16_byte_boundary() {
        let mut e = Engine::new();
        map_constants(&mut e);
        e.register(GET_AABB_SINGLE, get_aabb_from_shape);
        let input = make_input(&mut e, 0.0, 1000.0);
        let shape = object(&mut e, 0x30, &[(SHAPE_GET_AABB, GET_AABB_SINGLE)]);
        let entity = e.mem.alloc(0x200);
        e.mem.set_u32(entity + ENTITY_SHAPE, shape);
        let entities = e.mem.alloc(4);
        e.mem.set_u32(entities, entity);
        let raw = e.next_stack_local(FRAME_SIZE + 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00d1a330, &args![input, entities, 1u32]);
        let log = e.call_log.take().unwrap();
        let call = log
            .iter()
            .find(|(addr, _)| *addr == GET_AABB_SINGLE)
            .unwrap();
        assert_eq!(call.1[3], aligned_frame_at(raw, FRAME_SIZE) + FRAME_AABB);
    }

    #[test]
    fn recalc_with_children_builds_sorted_child_aabbs_and_their_union() {
        let mut e = Engine::new();
        map_constants(&mut e);
        let router = e.mem.alloc(4);
        let scratch = e.mem.alloc(0x400);
        e.mem.set_u32(router, scratch);
        e.register_double(THREAD_ROUTER, move |_, _| word_ret(router));
        let sorts: SortLog = Default::default();
        let seen = sorts.clone();
        e.register_double(SORT_PAIRS, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            let mut pairs: Vec<(u32, u32)> = (a[1]..=a[2])
                .map(|i| (e.mem.u32(a[0] + 8 * i), e.mem.u32(a[0] + 8 * i + 4)))
                .collect();
            pairs.sort_by_key(|pair| pair.0);
            for (i, (word, index)) in pairs.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 8 * i as u32, word);
                e.mem.set_u32(a[0] + 8 * i as u32 + 4, index);
            }
            Ret::default()
        });
        e.register(INIT_OFFSET_AABB_INPUT, init_offset_input);
        e.register(GET_CONTAINER, get_container);
        e.register(NUM_CHILDREN, num_children);
        e.register(FIRST_KEY, first_key);
        e.register(NEXT_KEY, next_key);
        e.register(CHILD_SHAPE, child_shape);
        e.register(GET_AABB_CHILD, get_aabb_from_shape);

        let input = make_input(&mut e, 1.0, 1.0);
        let child_seven = object(&mut e, 0x30, &[(SHAPE_GET_AABB, GET_AABB_CHILD)]);
        set_vec(&mut e, child_seven + 0x10, [10.0, 10.0, 10.0, 0.0]);
        set_vec(&mut e, child_seven + 0x20, [12.0, 14.0, 16.0, 0.0]);
        let child_three = object(&mut e, 0x30, &[(SHAPE_GET_AABB, GET_AABB_CHILD)]);
        set_vec(&mut e, child_three + 0x20, [2.0, 2.0, 2.0, 0.0]);
        let container = object(
            &mut e,
            0x10,
            &[
                (CONTAINER_NUM_CHILDREN, NUM_CHILDREN),
                (CONTAINER_FIRST_KEY, FIRST_KEY),
                (CONTAINER_NEXT_KEY, NEXT_KEY),
                (CONTAINER_CHILD_SHAPE, CHILD_SHAPE),
            ],
        );
        e.mem.set_u32(container + 4, child_seven);
        e.mem.set_u32(container + 8, child_three);
        e.mem.set_u32(container + 0xc, 2);
        let shape = object(&mut e, 8, &[(SHAPE_GET_CONTAINER, GET_CONTAINER)]);
        e.mem.set_u32(shape + 4, container);

        let entity = e.mem.alloc(0x200);
        e.mem.set_u32(entity + ENTITY_SHAPE, shape);
        e.mem.set_u16(entity + ENTITY_CAPACITY_CHILD_AABBS, 2);
        let child_aabbs = e.mem.alloc(0x40);
        let child_keys = e.mem.alloc(8);
        e.mem.set_u32(entity + ENTITY_CHILD_AABBS, child_aabbs);
        e.mem.set_u32(entity + ENTITY_CHILD_KEYS, child_keys);
        set_vec(
            &mut e,
            entity + ENTITY_MOTION + MOTION_CENTRE_1,
            [11.0, 12.0, 13.0, 0.0],
        );
        let entities = e.mem.alloc(4);
        e.mem.set_u32(entities, entity);

        e.call_log = Some(vec![]);
        e.call(0x00d1a330, &args![input, entities, 1u32]);
        let log = e.call_log.take().unwrap();

        let inits: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == INIT_OFFSET_AABB_INPUT)
            .collect();
        assert_eq!(inits.len(), 1);
        assert_eq!(inits[0].1[0], entity + ENTITY_MOTION);
        let aabb_calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == GET_AABB_CHILD)
            .collect();
        assert_eq!(aabb_calls.len(), 2);
        assert_eq!(aabb_calls[0].1[0], child_seven);
        assert_eq!(aabb_calls[1].1[0], child_three);
        assert_eq!(aabb_calls[0].1[2], 1.0f32.to_bits());
        // Scratch: the AABBs at the old top, the keys 0x80 further, the sort pairs after them;
        // the top is given back at the end.
        assert_eq!(sorts.borrow().as_slice(), &[vec![scratch + 0x100, 0, 1, 0]]);
        assert_eq!(e.mem.u32(router), scratch);

        assert_eq!(e.mem.u16(entity + ENTITY_NUM_CHILD_AABBS), 2);
        // Sorted by the first min word: key 3 (min 1) before key 7 (min 11).
        assert_eq!(e.mem.u32(child_keys), 3);
        assert_eq!(e.mem.u32(child_keys + 4), 7);
        assert_eq!(ints(&e, child_aabbs), [1, 1, 1]);
        assert_eq!(e.mem.bytes(child_aabbs + 0xc, 4), vec![1, 1, 1, 0]);
        assert_eq!(ints(&e, child_aabbs + 0x10), [2, 2, 2]);
        assert_eq!(e.mem.bytes(child_aabbs + 0x1c, 4), vec![0, 0, 0, 3]);
        let second = child_aabbs + 0x20;
        assert_eq!(ints(&e, second), [11, 11, 11]);
        // Expanded min 5.5, 5, 4.5 rounds (ties to even) to 6, 5, 4.
        assert_eq!(e.mem.bytes(second + 0xc, 4), vec![5, 6, 7, 0]);
        assert_eq!(ints(&e, second + 0x10), [12, 14, 16]);
        assert_eq!(e.mem.bytes(second + 0x1c, 4), vec![0, 0, 0, 7]);

        assert_eq!(ints(&e, entity + 0x30), [1, 1, 1]);
        assert_eq!(e.mem.bytes(entity + 0x3c, 4), vec![1, 1, 1, 0]);
        assert_eq!(ints(&e, entity + 0x40), [12, 14, 16]);
        assert_eq!(e.mem.bytes(entity + 0x4c, 4), vec![0, 0, 0, 0]);
    }

    #[test]
    fn expansion_shift_steps_by_four_then_one() {
        assert_eq!(expansion_shift(0xff), 0);
        assert_eq!(expansion_shift(0x100), 1);
        assert_eq!(expansion_shift(0xfe8), 4);
        assert_eq!(expansion_shift(0x7fff_ffff), 23);
        assert_eq!(expansion_shift(0x8000_0000), 0);
    }
}
