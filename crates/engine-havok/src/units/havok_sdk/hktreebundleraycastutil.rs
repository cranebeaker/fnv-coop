//! `Havok SDK/hktreebundleraycastutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::x87::rsqrt_estimate;

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

/// The memory router (`00c85750`): `+0` is the address the temporary stack allocator hands out
/// next, `+0x10` the allocator whose slot `+0xc` is `bufAlloc(size in and out)` and slot `+0x10`
/// is `bufFree(block, size)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve(allocator, array, capacity, element size)` (`00c908c0`, cdecl).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `_CIcos` (`00ec9f70`, argument in `ST0`).
const CI_COS: u32 = 0x00ec_9f70;
/// `pow` (`00ec9b00`): the exe passes the base in `ST1` and the exponent in `ST0`.
const POW: u32 = 0x00ec_9b00;
/// The `double` the exe loads into `ST0` for `pow`: the exponent.
const POW_EXPONENT: u32 = 0x0101_1590;
/// `hkJobQueue::addJob` (`00d04b90`, thiscall: the job, a flag word of 1).
const ADD_JOB: u32 = 0x00d0_4b90;
/// `hkSemaphore::acquire` (`00d04380`, thiscall).
const SEMAPHORE_ACQUIRE: u32 = 0x00d0_4380;
/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"LtAutoJob"` and its second label `"StAddJobs"`, `"StWaitThreads"`, the
/// label before the semaphore wait (`01076f38`), `"TtCPUSplit"`, `"TtAutoBundleJobs"` and the end
/// record (`010c41f8`).
const LABEL_AUTO_JOB: u32 = 0x010e_b178;
const LABEL_ADD_JOBS: u32 = 0x010e_b16c;
const LABEL_WAIT_THREADS: u32 = 0x010e_b15c;
const LABEL_BEFORE_WAIT: u32 = 0x0107_6f38;
const LABEL_CPU_SPLIT: u32 = 0x010e_b150;
const LABEL_AUTO_BUNDLE_JOBS: u32 = 0x010e_b13c;
const LABEL_END: u32 = 0x010c_41f8;
/// The flag every capacity word of an array that is not to be freed carries.
const DONT_DEALLOCATE: u32 = 0x8000_0000;
/// The capacity bits of an `hkArray`'s capacity-and-flags word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

type Lanes = [f32; 4];

/// A `MOVAPS` load of four lanes.
fn load(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| f32::from_bits(e.mem.u32(addr.wrapping_add(4 * lane as u32))))
}

/// A `MOVAPS` store of four lanes.
fn store(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem
            .set_u32(addr.wrapping_add(4 * lane as u32), value.to_bits());
    }
}

/// A 16-byte copy (four dword moves or a `MOVAPS` pair), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from.wrapping_add(4 * lane));
        e.mem.set_u32(to.wrapping_add(4 * lane), word);
    }
}

fn sub(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

/// The sum the code builds from the lanes of a `MULPS` product: `SHUFPS 0x55` / `ADDSS` adds
/// lane 1 to lane 0, `SHUFPS 0xaa` / `ADDSS` adds lane 2 to that: `p2 + (p1 + p0)`.
fn lane_sum(product: Lanes) -> f32 {
    product[2] + (product[1] + product[0])
}

/// The word `index` of the array at `base`.
fn word_at(base: u32, index: u32) -> u32 {
    base.wrapping_add(index.wrapping_mul(4))
}

/// The low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a monitor record to the thread's stream while its cursor (`+4`) is below its limit
/// (`+0xc`): the label, the time stamp, and `size` bytes in all (the record's other words are
/// left as they are). `second_label` goes to `+0xc` of the record when it is given.
fn profile_record(e: &mut Engine, label: u32, size: u32, second_label: Option<u32>) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        if let Some(second) = second_label {
            e.mem.set_u32(cursor + 0xc, second);
        }
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + size);
    }
}

/// The base-pointer the memory router keeps: `*router`.
fn router(e: &mut Engine) -> u32 {
    e.call(MEMORY_ROUTER, &args![]).u32()
}

/// The allocator behind the memory router (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = router(e);
    e.mem.u32(router + 0x10)
}

/// `bufAlloc(&size)` on the allocator: `size` goes in and comes back as the size granted.
fn buf_alloc(e: &mut Engine, allocator: u32, size: &mut u32) -> u32 {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), *size);
        let block = e.vcall(allocator, 0xc, &args![cell]).u32();
        *size = e.mem.u32(cell.addr());
        block
    })
}

/// `bufFree(block, size)` on the allocator.
fn buf_free(e: &mut Engine, allocator: u32, block: u32, size: u32) {
    e.vcall(allocator, 0x10, &args![block, size]);
}

/// `ST0 = float`, `pow(ST1 = length, ST0 = exponent)`, then the result through two
/// `FSTP`/`FLD` float round trips: the `float` the code keeps (as its bit pattern).
fn power_of_length(e: &mut Engine, length_bits: u32) -> f32 {
    let exponent: f64 = e.global(POW_EXPONENT);
    let result = e.call(POW, &args![x87_load32(length_bits), exponent]).f64();
    f32::from_bits(x87_store32(x87_load32(x87_store32(result))))
}

/// `|_CIcos(value)|` kept as a `float`, the way the group builders compute their limit: the
/// argument through `FLD`, the cosine through `FSTP`/`FLD`, `FABS`, `FSTP`.
fn absolute_cosine(e: &mut Engine, argument: f32) -> f32 {
    let cosine = e.call(CI_COS, &args![x87_load32(argument.to_bits())]).f64();
    let kept = x87_load32(x87_store32(cosine));
    f32::from_bits(x87_store32(kept.abs()))
}

// Translated from 00e29920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, eight stack words, the answer in `AL`): whether two rays
/// are alike. Each ray is a start, an end and a direction (`hkVector4`s). With the products of
/// the two directions summed as `z + (y + x)`: false when `cos_limit` is above that dot
/// product (an unordered compare goes on). Then false when `max_distance` is below the squared
/// distance (summed the same way) of the two starts, and again of the two ends; otherwise true.
/// The six pointers are in the order start, end, direction of the first ray, then of the
/// second.
pub fn fn_00e29920(
    e: &mut Engine,
    start_a: Ptr,
    end_a: Ptr,
    direction_a: Ptr,
    start_b: Ptr,
    end_b: Ptr,
    direction_b: Ptr,
    cos_limit: f32,
    max_distance: f32,
) -> bool {
    let product = mul(load(e, direction_a.addr()), load(e, direction_b.addr()));
    let dot = lane_sum(product);
    if cos_limit > dot {
        return false;
    }
    let start_gap = sub(load(e, start_b.addr()), load(e, start_a.addr()));
    let start_distance = lane_sum(mul(start_gap, start_gap));
    if max_distance < start_distance {
        return false;
    }
    let end_gap = sub(load(e, end_b.addr()), load(e, end_a.addr()));
    let end_distance = lane_sum(mul(end_gap, end_gap));
    // `TEST AH,5` / `JNP`: an unordered compare (a NaN) does not fail the test either.
    if max_distance < end_distance {
        return false;
    }
    true
}

// Translated from 00e29a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, eight stack words): writes out the bundles of rays the
/// group builders made. `bundles` points at an array (`+0` the 20-byte entries, `+4` their
/// count); an entry is up to four ray indices and, at `+0x10`, how many it holds. For a bundle
/// of exactly one ray a 0x40-byte record goes to `singles` (the ray's start and end vectors
/// from `starts` and `ends`, a byte at `+0x20`, zero words, `records + index * 0x60` at
/// `+0x30` and a one at `+0x34`); any other bundle is a 0x100-byte record in `groups`: four
/// 0x30-byte ray slots (start, end, a zero byte at `+0x20`, a zero word at `+0x24`), the
/// bundle's count at `+0xc0`, a zero byte at `+0xc4`, per ray the pointer `records + index *
/// 0x60` at `+0xc8` and a zero word at `+0xe0`, each as the ray's position in the bundle
/// (bundles with a count of zero or less write only the count and the byte). The two numbers
/// of records written are stored through `single_count` and `group_count`. An empty array
/// stores two zeros.
pub fn fn_00e29a10(
    e: &mut Engine,
    bundles: Ptr,
    starts: Ptr,
    ends: Ptr,
    records: Ptr,
    singles: Ptr,
    single_count: Ptr,
    groups: Ptr,
    group_count: Ptr,
) {
    let data = e.mem.u32(bundles.addr());
    let total = e.mem.i32(bundles.addr() + 4);
    if total <= 0 {
        e.mem.set_u32(single_count.addr(), 0);
        e.mem.set_u32(group_count.addr(), 0);
        return;
    }
    let mut singles_written = 0u32;
    let mut groups_written = 0u32;
    let mut single_slot = singles.addr().wrapping_add(0x24);
    let mut group_slot = groups.addr().wrapping_add(0x24);
    let mut bundle = data;
    for _ in 0..total {
        let ray_count = e.mem.i32(bundle + 0x10);
        if ray_count == 1 {
            let ray = e.mem.u32(bundle);
            let offset = ray << 4;
            copy_vec(
                e,
                starts.addr().wrapping_add(offset),
                single_slot.wrapping_sub(0x24),
            );
            copy_vec(
                e,
                ends.addr().wrapping_add(offset),
                single_slot.wrapping_sub(0x14),
            );
            let record = ray
                .wrapping_mul(3)
                .wrapping_shl(5)
                .wrapping_add(records.addr());
            e.mem.set_u32(single_slot, 0);
            e.mem.set_u8(single_slot.wrapping_sub(4), 0);
            e.mem.set_u32(single_slot.wrapping_add(0xc), record);
            e.mem.set_u32(single_slot.wrapping_add(0x10), 1);
            e.mem.set_u32(single_slot.wrapping_add(0x14), 0);
            e.mem.set_u8(single_slot.wrapping_add(0x19), 0);
            single_slot = single_slot.wrapping_add(0x40);
            singles_written += 1;
        } else {
            for position in 0..ray_count {
                let ray = e.mem.u32(word_at(bundle, position as u32));
                let offset = ray << 4;
                let slot = group_slot.wrapping_add(0x30u32.wrapping_mul(position as u32));
                copy_vec(
                    e,
                    starts.addr().wrapping_add(offset),
                    slot.wrapping_sub(0x24),
                );
                copy_vec(e, ends.addr().wrapping_add(offset), slot.wrapping_sub(0x14));
                let record = ray
                    .wrapping_mul(3)
                    .wrapping_shl(5)
                    .wrapping_add(records.addr());
                e.mem.set_u32(slot, 0);
                e.mem.set_u8(slot.wrapping_sub(4), 0);
                let pointers = group_slot.wrapping_add(0xa4);
                e.mem.set_u32(word_at(pointers, position as u32), record);
                e.mem
                    .set_u32(word_at(group_slot.wrapping_add(0xbc), position as u32), 0);
            }
            e.mem
                .set_u32(group_slot.wrapping_add(0x9c), ray_count as u32);
            e.mem.set_u8(group_slot.wrapping_add(0xa0), 0);
            group_slot = group_slot.wrapping_add(0x100);
            groups_written += 1;
        }
        bundle = bundle.wrapping_add(0x14);
    }
    e.mem.set_u32(single_count.addr(), singles_written);
    e.mem.set_u32(group_count.addr(), groups_written);
}

// Translated from 00e29b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTreeBundleRayCastUtil::splitToOctant` (Xbox PDB), `cdecl`, seven stack words: for each of
/// `count` rays the gap `end - start` (`SUBPS`) is summed as `z + (y + x)` over its squares;
/// `RSQRTSS` (the host CPU's estimate, no refinement) gives the factor `r`; `directions[i]`
/// becomes `gap * r` in all four lanes, and `lengths[i]` first `(sum * r, sz, sz, sz)` (`sz` is
/// the z square) and then, over it, `(pow(sum * r, exponent), 0, 0, 0)` (`pow` of `00ec9b00`
/// with the double at `01011590` as the exponent). Then the eight `hkArray<int>` of `octants`
/// (12 bytes each) are reserved up to the larger of `count` and twice their capacity
/// (`00c908c0`, element size 4; no reserve when the capacity is at least `count`), their
/// entries of `counts` cleared, and every ray's index is appended to the array of its octant:
/// the sign bits of the first three lanes of its direction (`CMPLTPS` against zero,
/// `MOVMSKPS`), x in bit 0, y in bit 1, z in bit 2; `counts[octant]` is counted up.
pub fn hk_tree_bundle_ray_cast_util_split_to_octant(
    e: &mut Engine,
    starts: Ptr,
    ends: Ptr,
    count: i32,
    directions: Ptr,
    lengths: Ptr,
    octants: Ptr,
    counts: Ptr,
) {
    if count > 0 {
        for ray in 0..count as u32 {
            let offset = ray.wrapping_mul(16);
            let start = load(e, starts.addr().wrapping_add(offset));
            let end = load(e, ends.addr().wrapping_add(offset));
            let gap = sub(end, start);
            let squares = mul(gap, gap);
            let sum = lane_sum(squares);
            let factor = rsqrt_estimate(sum);
            let length = sum * factor;
            let direction = mul(gap, [factor; 4]);
            store(e, directions.addr().wrapping_add(offset), direction);
            let first = [length, squares[2], squares[2], squares[2]];
            let length_slot = lengths.addr().wrapping_add(offset);
            store(e, length_slot, first);
            let power = power_of_length(e, length.to_bits());
            store(e, length_slot, [power, 0.0, 0.0, 0.0]);
        }
    }
    for octant in 0..8u32 {
        e.mem.set_u32(word_at(counts.addr(), octant), 0);
        let allocator = router_allocator(e);
        let array = octants.addr().wrapping_add(12 * octant);
        let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 4u32]);
        }
    }
    for ray in 0..count.max(0) as u32 {
        let direction = load(e, directions.addr().wrapping_add(ray.wrapping_mul(16)));
        let octant = u32::from(direction[0] < 0.0)
            + (u32::from(direction[1] < 0.0) << 1)
            + (u32::from(direction[2] < 0.0) << 2);
        let counter = word_at(counts.addr(), octant);
        let counted = e.mem.u32(counter);
        e.mem.set_u32(counter, counted.wrapping_add(1));
        let array = octants.addr().wrapping_add(12 * octant);
        let entries = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        e.mem.set_u32(word_at(entries, size), ray);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
    }
}

/// What the two group builders (`00e29cd0`, `00e2a010`) share, as the exe's `rays` they work
/// on: scratch vectors for the ray being placed, a bundle search over the bundles made so far
/// and the creation of a new bundle. The six scratch vectors (start, end and direction of the
/// new ray, then of the bundle's first ray, the order `00e29920` takes them in) sit in the
/// builder's frame, at offsets of its own for each builder ([`ScratchLayout`]).
type ScratchLayout = [u32; 6];

/// `00e29cd0`'s frame (aligned, `SUB ESP,0xc4` and three pushes under the boundary) and where
/// its scratch vectors are (the pointers it pushes for `00e29920`, 00e29ec9..00e29f1f).
const INDEXED_FRAME_BELOW: u32 = 0xd0;
const INDEXED_SCRATCH: ScratchLayout = [0x60, 0x70, 0xa0, 0x50, 0x80, 0x90];

/// `00e2a010`'s frame (aligned, `SUB ESP,0xd4` and three pushes) and its scratch vectors
/// (00e2a22c..00e2a282).
const OCTANT_FRAME_BELOW: u32 = 0xe0;
const OCTANT_SCRATCH: ScratchLayout = [0x80, 0x50, 0xa0, 0x70, 0x60, 0xb0];
/// A bundle of rays: up to four ray indices and their count at `+0x10`.
const BUNDLE_SIZE: u32 = 0x14;

/// The state of one group build: the temporary buffer of bundles and how many there are.
struct BundleBuild {
    /// The address the memory router's stack allocator gave out (the buffer of bundles).
    buffer: u32,
    /// The bundles made so far.
    made: i32,
}

/// Takes the buffer of `count` bundles from the memory router's stack allocator: the router's
/// next address moves on by `count * 0x14` rounded up to a multiple of 0x80 (the `0x7f` mask).
fn take_bundle_buffer(e: &mut Engine, count: i32) -> BundleBuild {
    let router = router(e);
    let buffer = e.mem.u32(router);
    let size = (count as u32).wrapping_mul(BUNDLE_SIZE).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(buffer));
    BundleBuild { buffer, made: 0 }
}

/// The ray's gap and direction: `end - start` (`SUBPS`), the `RSQRTSS` factor of its sum
/// (`z + (y + x)` of the squares) and the gap times it in all four lanes (the host CPU's
/// estimate, no refinement). Returns the gap, the factor and the sum times the factor.
fn ray_direction(start: Lanes, end: Lanes) -> (Lanes, f32, f32) {
    let gap = sub(end, start);
    let sum = lane_sum(mul(gap, gap));
    let factor = rsqrt_estimate(sum);
    (mul(gap, [factor; 4]), factor, sum * factor)
}

/// Places the ray `ray` (its start and end, `ray_index` its number) in a bundle. The ray's direction and its
/// `pow` of the length (`power`) are computed first; then each bundle made so far with fewer
/// than four rays is tried through its first ray (`00e29920` with `abs_cos` and `power *
/// threshold`; the exe's inner loop over the bundle's rays runs once), the ray joining the
/// first that takes it. When none does (or there are none) a new bundle holding the ray is
/// started at the next entry of the buffer, unless that address is null. Returns whether a new
/// bundle was started (the `made` counter then went up). `frame` is the builder's frame and
/// `layout` where its scratch vectors are.
#[allow(clippy::too_many_arguments)]
fn place_ray(
    e: &mut Engine,
    frame: u32,
    layout: &ScratchLayout,
    build: &mut BundleBuild,
    starts: u32,
    ends: u32,
    ray: (Lanes, Lanes),
    ray_index: u32,
    abs_cos: f32,
    threshold: f32,
) {
    let (start_a, end_a) = ray;
    let (direction_a, _, length) = ray_direction(start_a, end_a);
    let [start_at_a, end_at_a, direction_at_a, start_at_b, end_at_b, direction_at_b] =
        layout.map(|offset| frame + offset);
    store(e, start_at_a, start_a);
    store(e, end_at_a, end_a);
    store(e, direction_at_a, direction_a);
    let power = power_of_length(e, length.to_bits());
    let mut bundle = build.buffer;
    let mut index = 0;
    while index < build.made {
        if e.mem.i32(bundle + 0x10) < 4 {
            let limit = power * threshold;
            let mut position = 0u32;
            while position < 1 {
                let other = e.mem.u32(word_at(bundle, position));
                let offset = other << 4;
                let start_b = load(e, starts.wrapping_add(offset));
                let end_b = load(e, ends.wrapping_add(offset));
                let (direction_b, _, _) = ray_direction(start_b, end_b);
                store(e, start_at_b, start_b);
                store(e, end_at_b, end_b);
                store(e, direction_at_b, direction_b);
                let alike = fn_00e29920(
                    e,
                    Ptr::new(start_at_a),
                    Ptr::new(end_at_a),
                    Ptr::new(direction_at_a),
                    Ptr::new(start_at_b),
                    Ptr::new(end_at_b),
                    Ptr::new(direction_at_b),
                    x87_quiet(abs_cos),
                    x87_quiet(limit),
                );
                if alike {
                    let held = e.mem.u32(bundle + 0x10);
                    e.mem.set_u32(word_at(bundle, held), ray_index);
                    e.mem.set_u32(bundle + 0x10, held.wrapping_add(1));
                    return;
                }
                position += 1;
            }
        }
        index += 1;
        bundle = bundle.wrapping_add(BUNDLE_SIZE);
    }
    // A new bundle at the next entry (the buffer pointer is not checked past its start).
    let fresh = build
        .buffer
        .wrapping_add((build.made as u32).wrapping_mul(BUNDLE_SIZE));
    if fresh != 0 {
        e.mem.set_u32(fresh, ray_index);
        e.mem.set_u32(fresh + 4, 0);
        e.mem.set_u32(fresh + 8, 0);
        e.mem.set_u32(fresh + 0xc, 0);
        e.mem.set_u32(fresh + 0x10, 1);
    }
    build.made += 1;
}

/// Hands the made bundles to `00e29a10` through a stack array (`+0` buffer, `+4` how many, `+8`
/// capacity word `count | 0x80000000`, `+0xc` the buffer again), then puts the router's next
/// address back and, when the capacity word has no flag, gives the buffer back (`bufFree`,
/// 20 bytes per entry). The flag is always set, so the release is never reached.
#[allow(clippy::too_many_arguments)]
fn finish_bundles(
    e: &mut Engine,
    build: &BundleBuild,
    count: i32,
    starts: Ptr,
    ends: Ptr,
    outputs: [u32; 5],
) {
    let capacity_word = (count as u32) | DONT_DEALLOCATE;
    e.with_stack(0x10, |e, array| {
        e.mem.set_u32(array.addr(), build.buffer);
        e.mem.set_u32(array.addr() + 4, build.made as u32);
        e.mem.set_u32(array.addr() + 8, capacity_word);
        e.mem.set_u32(array.addr() + 0xc, build.buffer);
        fn_00e29a10(
            e,
            array,
            starts,
            ends,
            Ptr::new(outputs[0]),
            Ptr::new(outputs[1]),
            Ptr::new(outputs[2]),
            Ptr::new(outputs[3]),
            Ptr::new(outputs[4]),
        );
    });
    let router_now = router(e);
    e.mem.set_u32(router_now, build.buffer);
    let allocator = router_allocator(e);
    if (capacity_word as i32) >= 0 {
        let size = (capacity_word & CAPACITY_MASK).wrapping_mul(BUNDLE_SIZE);
        buf_free(e, allocator, build.buffer, size);
    }
}

// Translated from 00e29cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, twelve stack words; the fifth is never read): groups
/// the rays `indices[0..count]` (indices into `starts` and `ends`, 16 bytes a ray) into
/// bundles of at most four alike rays. The limit is `|cos|` of `cos_argument` (`00ec9f70`,
/// kept as a `float`) and the distance limit of a ray is `pow(length, exponent)` (see
/// `splitToOctant`) times `threshold`. The buffer of bundles comes from the memory router's
/// stack allocator (`count * 0x14` rounded up to a multiple of 0x80). Each ray is placed
/// (`place_ray`); the bundles are then written out by `00e29a10` with the five last words
/// and the buffer is given back. The exe's store of zero to a stack slot nothing reads is
/// left out.
#[allow(clippy::explicit_counter_loop)] // the exe keeps the ray index in its own register beside the loop count
pub fn fn_00e29cd0(
    e: &mut Engine,
    indices: Ptr,
    count: i32,
    starts: Ptr,
    ends: Ptr,
    _unused_5: u32,
    cos_argument: f32,
    threshold: f32,
    record_base: u32,
    single_out: u32,
    single_count: u32,
    group_out: u32,
    group_count: u32,
) {
    let abs_cos = absolute_cosine(e, cos_argument);
    let mut build = take_bundle_buffer(e, count);
    with_aligned_frame(e, INDEXED_FRAME_BELOW, |e, frame| {
        let mut position = 0u32;
        for _ in 0..count.max(0) {
            let ray = e.mem.u32(word_at(indices.addr(), position));
            position += 1;
            let offset = ray << 4;
            let start = load(e, starts.addr().wrapping_add(offset));
            let end = load(e, ends.addr().wrapping_add(offset));
            place_ray(
                e,
                frame,
                &INDEXED_SCRATCH,
                &mut build,
                starts.addr(),
                ends.addr(),
                (start, end),
                ray,
                abs_cos,
                threshold,
            );
        }
    });
    finish_bundles(
        e,
        &build,
        count,
        starts,
        ends,
        [
            record_base,
            single_out,
            single_count,
            group_out,
            group_count,
        ],
    );
}

// Translated from 00e2a010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, eleven stack words): `00e29cd0` over the rays `0` to
/// `count - 1` in order, but only those whose direction falls in the octant `octant`: the sign
/// bits of the first three lanes of `end - start` (`CMPLTPS` against zero, `MOVMSKPS`, x in bit
/// 0) as in `splitToOctant`, compared with `octant`. A bundle's ray is told by its position in
/// `starts` and `ends` (16 bytes a ray). The limit and the buffer are as there; the five last
/// words go to `00e29a10`.
pub fn fn_00e2a010(
    e: &mut Engine,
    octant: i32,
    starts: Ptr,
    ends: Ptr,
    count: i32,
    cos_argument: f32,
    threshold: f32,
    record_base: u32,
    single_out: u32,
    single_count: u32,
    group_out: u32,
    group_count: u32,
) {
    let abs_cos = absolute_cosine(e, cos_argument);
    let mut build = take_bundle_buffer(e, count);
    with_aligned_frame(e, OCTANT_FRAME_BELOW, |e, frame| {
        for ray in 0..count.max(0) as u32 {
            let offset = ray << 4;
            let start = load(e, starts.addr().wrapping_add(offset));
            let end = load(e, ends.addr().wrapping_add(offset));
            let gap = sub(end, start);
            let sign_bits = u32::from(gap[0] < 0.0)
                + (u32::from(gap[1] < 0.0) << 1)
                + (u32::from(gap[2] < 0.0) << 2);
            if sign_bits as i32 != octant {
                continue;
            }
            place_ray(
                e,
                frame,
                &OCTANT_SCRATCH,
                &mut build,
                starts.addr(),
                ends.addr(),
                (start, end),
                ray,
                abs_cos,
                threshold,
            );
        }
    });
    finish_bundles(
        e,
        &build,
        count,
        starts,
        ends,
        [
            record_base,
            single_out,
            single_count,
            group_out,
            group_count,
        ],
    );
}

/// The group job `startWorldRayCastWithAutoBundleJobs` queues (`hkJobQueue::addJob`) is 0x70
/// bytes on the exe's stack, at `+0x60` of its frame (aligned: `SUB ESP,0xe4` and three pushes
/// under the boundary, [`AUTO_BUNDLE_FRAME_BELOW`]): the exe writes the words `fill_job` lists
/// and leaves the other bytes as it finds them (here zero). The eight octant counts
/// `splitToOctant` fills are at `+0xd0` (00e2a6bc).
const AUTO_BUNDLE_FRAME_BELOW: u32 = 0xf0;
const JOB_AT: u32 = 0x60;
const COUNTS_AT: u32 = 0xd0;

/// The words both paths of `startWorldRayCastWithAutoBundleJobs` put into a job: the header
/// bytes 7, 3, 1 (+0, +1, +2), the word 0x70 at +4 and 0xffff at +6, zeros at +0x10, +0x14
/// and +0x18, then (`words`: the first argument, the second, the third argument's count, the
/// two floats, the ninth argument) and the words copied from the parameter block `params` and
/// the two running output addresses `first_output` (+0x44) and `second_output` (+0x4c).
struct JobWords {
    first: u32,
    second: u32,
    count: u32,
    near: f32,
    far: f32,
    ninth: u32,
    first_output: u32,
    second_output: u32,
}

fn fill_job(e: &mut Engine, job: u32, words: &JobWords, params: u32) {
    e.mem.set_u8(job, 7);
    e.mem.set_u8(job + 1, 3);
    e.mem.set_u8(job + 2, 1);
    e.mem.set_u16(job + 4, 0x70);
    e.mem.set_u16(job + 6, 0xffff);
    for offset in [0x10, 0x14, 0x18] {
        e.mem.set_u32(job + offset, 0);
    }
    e.mem.set_u32(job + 0x2c, words.first);
    e.mem.set_u32(job + 0x30, words.second);
    e.mem.set_u32(job + 0x34, words.count);
    // The floats go through FLD / FSTP.
    e.mem.set_f32(job + 0x38, x87_quiet(words.near));
    e.mem.set_f32(job + 0x3c, x87_quiet(words.far));
    e.mem.set_u32(job + 0x40, words.ninth);
    e.mem.set_u32(job + 0x44, words.first_output);
    e.mem.set_u32(job + 0x4c, words.second_output);
    for (offset, field) in [
        (0x54u32, 0x1cu32),
        (0x58, 0x14),
        (0x5c, 0x10),
        (0x60, 0x34),
        (0x64, 0x38),
        (0x68, 0x3c),
        (0x6c, 0x40),
    ] {
        let value = e.mem.u32(params + field);
        e.mem.set_u32(job + offset, value);
    }
}

/// `hkJobQueue::addJob(job, 1)` on the job queue `queue`.
fn add_job(e: &mut Engine, queue: u32, job: u32) {
    e.call(ADD_JOB, &args![queue, job, 1u32]);
}

/// The exe's `(size + 0xf) >> 4` style division of a granted size: `size / divisor` truncated
/// towards zero, or `0x80000000` (the don't-deallocate flag alone) for zero.
fn capacity_of(size: u32, divisor: i32) -> u32 {
    let capacity = (size as i32) / divisor;
    if capacity == 0 {
        DONT_DEALLOCATE
    } else {
        capacity as u32
    }
}

/// Gives back a buffer of `capacity` entries of `entry_size` bytes when its capacity word
/// does not carry the don't-deallocate flag.
fn release_buffer(e: &mut Engine, allocator: u32, block: u32, capacity: u32, entry_size: u32) {
    if (capacity as i32) >= 0 {
        buf_free(
            e,
            allocator,
            block,
            (capacity & CAPACITY_MASK).wrapping_mul(entry_size),
        );
    }
}

// Translated from 00e2a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTreeBundleRayCastUtil::startWorldRayCastWithAutoBundleJobs` (Xbox PDB), `cdecl`, ten
/// stack words: `first`, `second`, `count`, `near`, `far`, `params` (a block whose words at
/// +0x10 (a semaphore), +0x14, +0x1c, +0x24, +0x2c, +0x34, +0x38, +0x3c and +0x40 go into the
/// jobs), `queue` (the argument of `addJob`), `runner` (an object whose slot `+0xc` takes
/// `(queue, 0xb)` and slot `+0x10` starts the work), `ninth` and `auto_jobs` (a byte). Both
/// paths write profiling records (`RDTSC` is the host clock) to the thread's monitor stream.
///
/// With `auto_jobs` set: eight 0x70-byte jobs are queued (`00d04b90`), job `i` with index `i`
/// at `+0x24`, `count` at `+0x34` and the two output addresses `params + 0x24` and
/// `params + 0x2c` moved on by `count * 0x40` and `count * 0x100` after each; then the runner
/// is started and the semaphore at `params + 0x10` acquired (`00d04380`).
///
/// Otherwise, the work is split first: buffers for `count` 16-byte directions and as many
/// lengths (`bufAlloc` of the memory router's allocator, sizes in and out; none when
/// `count` is zero), an array of eight `hkArray<int>` (96 bytes, their count the granted size
/// over 12, each cleared to data 0, size 0, capacity `0x80000000`) and two scratch buffers of
/// 0x20 bytes that nothing reads, `splitToOctant` over them, and one job (index word not
/// written, `+0x20` the octant array's data, `+0x28` its size) for every octant array that is
/// not empty. After the runner is started and the semaphore acquired every buffer is
/// given back (the octant arrays' data first, then the arrays) unless its capacity word has
/// the flag.
///
/// The job blocks' bytes the exe does not write are stack it leaves as found; here they are
/// zero.
pub fn hk_tree_bundle_ray_cast_util_start_world_ray_cast_with_auto_bundle_jobs(
    e: &mut Engine,
    first: u32,
    second: u32,
    count: u32,
    near: f32,
    far: f32,
    params: Ptr,
    queue: u32,
    runner: Ptr,
    ninth: u32,
    auto_jobs: bool,
) {
    let params = params.addr();
    with_aligned_frame(e, AUTO_BUNDLE_FRAME_BELOW, |e, frame| {
        let job = frame + JOB_AT;
        if auto_jobs {
            profile_record(e, LABEL_AUTO_JOB, 0x10, Some(LABEL_ADD_JOBS));
            let mut first_output = e.mem.u32(params + 0x24);
            let mut second_output = e.mem.u32(params + 0x2c);
            for index in 0..8u32 {
                let words = JobWords {
                    first,
                    second,
                    count,
                    near,
                    far,
                    ninth,
                    first_output,
                    second_output,
                };
                fill_job(e, job, &words, params);
                e.mem.set_u32(job + 0x20, 0);
                e.mem.set_u32(job + 0x24, index);
                e.mem.set_u32(job + 0x28, 0);
                add_job(e, queue, job);
                first_output = first_output.wrapping_add(count << 6);
                second_output = second_output.wrapping_add(count << 8);
            }
            e.vcall(runner.addr(), 0xc, &args![queue, 0xbu32]);
            profile_record(e, LABEL_WAIT_THREADS, 0xc, None);
            e.vcall(runner.addr(), 0x10, &args![]);
            profile_record(e, LABEL_BEFORE_WAIT, 0xc, None);
            let semaphore = e.mem.u32(params + 0x10);
            e.call(SEMAPHORE_ACQUIRE, &args![semaphore]);
            return;
        }
        profile_record(e, LABEL_CPU_SPLIT, 0xc, None);

        // The directions buffer.
        let allocator = router_allocator(e);
        let mut directions = 0u32;
        let mut directions_capacity = DONT_DEALLOCATE;
        if count != 0 {
            let mut size = count << 4;
            directions = buf_alloc(e, allocator, &mut size);
            directions_capacity = capacity_of(size, 16);
        }
        // The lengths buffer.
        let allocator = router_allocator(e);
        let mut lengths = 0u32;
        let mut lengths_capacity = DONT_DEALLOCATE;
        if count != 0 {
            let mut size = count << 4;
            lengths = buf_alloc(e, allocator, &mut size);
            lengths_capacity = capacity_of(size, 16);
        }
        // The array of eight hkArray<int>.
        let allocator = router_allocator(e);
        let mut size = 0x60u32;
        let octants = buf_alloc(e, allocator, &mut size);
        let octant_arrays = (size as i32) / 12;
        let octants_capacity = capacity_of(size, 12);
        let mut array = octants;
        for _ in 0..octant_arrays.max(0) {
            if array != 0 {
                e.mem.set_u32(array, 0);
                e.mem.set_u32(array + 4, 0);
                e.mem.set_u32(array + 8, DONT_DEALLOCATE);
            }
            array = array.wrapping_add(12);
        }
        let counts = frame + COUNTS_AT;
        hk_tree_bundle_ray_cast_util_split_to_octant(
            e,
            Ptr::new(first),
            Ptr::new(second),
            count as i32,
            Ptr::new(directions),
            Ptr::new(lengths),
            Ptr::new(octants),
            Ptr::new(counts),
        );
        // Two scratch buffers of eight words the exe never reads.
        let allocator = router_allocator(e);
        let mut size = 0x20u32;
        let scratch_a = buf_alloc(e, allocator, &mut size);
        let scratch_a_capacity = capacity_of(size, 4);
        let allocator = router_allocator(e);
        let mut size = 0x20u32;
        let scratch_b = buf_alloc(e, allocator, &mut size);
        let scratch_b_capacity = capacity_of(size, 4);

        let mut first_output = e.mem.u32(params + 0x24);
        let mut second_output = e.mem.u32(params + 0x2c);
        let mut array = octants.wrapping_add(4);
        for _ in 0..8 {
            let entries = e.mem.u32(array);
            if entries != 0 {
                let data = e.mem.u32(array.wrapping_sub(4));
                let words = JobWords {
                    first,
                    second,
                    count,
                    near,
                    far,
                    ninth,
                    first_output,
                    second_output,
                };
                fill_job(e, job, &words, params);
                e.mem.set_u32(job + 0x20, data);
                e.mem.set_u32(job + 0x28, entries);
                add_job(e, queue, job);
                first_output = first_output.wrapping_add(count << 6);
                second_output = second_output.wrapping_add(count << 8);
            }
            array = array.wrapping_add(12);
        }
        profile_record(e, LABEL_END, 0xc, None);
        profile_record(e, LABEL_AUTO_BUNDLE_JOBS, 0xc, None);
        e.vcall(runner.addr(), 0xc, &args![queue, 0xbu32]);
        e.vcall(runner.addr(), 0x10, &args![]);
        let semaphore = e.mem.u32(params + 0x10);
        e.call(SEMAPHORE_ACQUIRE, &args![semaphore]);
        profile_record(e, LABEL_END, 0xc, None);

        let allocator = router_allocator(e);
        release_buffer(e, allocator, scratch_b, scratch_b_capacity, 4);
        let allocator = router_allocator(e);
        release_buffer(e, allocator, scratch_a, scratch_a_capacity, 4);
        let allocator = router_allocator(e);
        let mut array = octants.wrapping_add(8);
        for _ in 0..octant_arrays.max(0) {
            let allocator_now = router_allocator(e);
            let capacity = e.mem.u32(array);
            e.mem.set_u32(array.wrapping_sub(4), 0);
            let data = e.mem.u32(array.wrapping_sub(8));
            release_buffer(e, allocator_now, data, capacity, 4);
            e.mem.set_u32(array.wrapping_sub(8), 0);
            e.mem.set_u32(array, DONT_DEALLOCATE);
            array = array.wrapping_add(12);
        }
        release_buffer(e, allocator, octants, octants_capacity, 12);
        let allocator = router_allocator(e);
        release_buffer(e, allocator, lengths, lengths_capacity, 16);
        let allocator = router_allocator(e);
        release_buffer(e, allocator, directions, directions_capacity, 16);
    });
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e29920, fn_00e29920(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, f32, f32) -> bool),
        entry!(
            0x00e29a10,
            fn_00e29a10(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e29b80,
            hk_tree_bundle_ray_cast_util_split_to_octant(Ptr, Ptr, i32, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e29cd0,
            fn_00e29cd0(Ptr, i32, Ptr, Ptr, u32, f32, f32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00e2a010,
            fn_00e2a010(i32, Ptr, Ptr, i32, f32, f32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00e2a390,
            hk_tree_bundle_ray_cast_util_start_world_ray_cast_with_auto_bundle_jobs(
                u32, u32, u32, f32, f32, Ptr, u32, Ptr, u32, bool
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const ROUTER_OBJECT: u32 = 0x0070_0000;
    const ALLOCATOR_OBJECT: u32 = 0x0070_1000;
    const ALLOCATOR_TABLE: u32 = 0x0070_2000;
    const ALLOC_DOUBLE: u32 = 0x0070_3000;
    const FREE_DOUBLE: u32 = 0x0070_3010;
    const TEMP_STACK: u32 = 0x0072_0000;
    const MONITOR_STREAM: u32 = 0x0070_4000;
    const TLS_INDEX: u32 = 7;

    /// A memory router (`00c85750`) whose temporary stack starts at `TEMP_STACK` and whose
    /// allocator's `bufAlloc` hands out fresh blocks (granting what is asked, logging `[size]`)
    /// and whose `bufFree` logs `[block, size]`. Returns the two logs.
    fn memory_fixture(e: &mut Engine) -> (Log, Log) {
        e.map(ROUTER_OBJECT, 0x100);
        e.map(ALLOCATOR_OBJECT, 0x100);
        e.map(TEMP_STACK, 0x1_0000);
        e.mem.set_u32(ROUTER_OBJECT, TEMP_STACK);
        e.mem.set_u32(ROUTER_OBJECT + 0x10, ALLOCATOR_OBJECT);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_TABLE);
        let mut table = vec![0u32; 5];
        table[3] = ALLOC_DOUBLE;
        table[4] = FREE_DOUBLE;
        e.put_vtable(ALLOCATOR_TABLE, &table);
        e.register(0x00c8_5750, |_, _| Ret {
            eax: ROUTER_OBJECT,
            ..Ret::default()
        });
        let allocs: Log = Rc::new(RefCell::new(Vec::new()));
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let log = allocs.clone();
        e.register_double(ALLOC_DOUBLE, move |e, words| {
            let size = e.mem.u32(words[1]);
            log.borrow_mut().push(vec![size]);
            Ret {
                eax: e.mem.alloc(size.max(4)),
                ..Ret::default()
            }
        });
        let log = frees.clone();
        e.register_double(FREE_DOUBLE, move |_, words| {
            log.borrow_mut().push(vec![words[1], words[2]]);
            Ret::default()
        });
        (allocs, frees)
    }

    fn put_vec(e: &mut Engine, addr: u32, v: Lanes) {
        store(e, addr, v);
    }

    fn get_vec(e: &Engine, addr: u32) -> Lanes {
        load(e, addr)
    }

    /// Rays 0 and 1 are alike (along +x, 0.1 apart), ray 2 is another way (+y, -z).
    fn put_rays(e: &mut Engine) -> (u32, u32) {
        let starts = e.mem.alloc(0x40);
        let ends = e.mem.alloc(0x40);
        let rays = [
            ([0.0, 0.0, 0.0, 0.0], [10.0, 0.0, 0.0, 0.0]),
            ([0.0, 0.1, 0.0, 0.0], [10.0, 0.1, 0.0, 0.0]),
            ([0.0, 5.0, 0.0, 0.0], [0.0, 15.0, -10.0, 0.0]),
        ];
        for (index, (start, end)) in rays.iter().enumerate() {
            put_vec(e, starts + 16 * index as u32, *start);
            put_vec(e, ends + 16 * index as u32, *end);
        }
        (starts, ends)
    }

    #[test]
    fn the_ray_likeness_test_checks_the_dot_and_both_distances() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x60);
        let b = e.mem.alloc(0x60);
        let reset = |e: &mut Engine| {
            for (base, shift) in [(a, 0.0f32), (b, 0.0)] {
                put_vec(e, base, [shift, 0.0, 0.0, 0.0]);
                put_vec(e, base + 0x10, [shift + 1.0, 0.0, 0.0, 0.0]);
                put_vec(e, base + 0x20, [1.0, 0.0, 0.0, 0.0]);
            }
        };
        let run = |e: &mut Engine, limit: f32, distance: f32| {
            e.call(
                0x00e2_9920,
                &args![
                    a,
                    a + 0x10,
                    a + 0x20,
                    b,
                    b + 0x10,
                    b + 0x20,
                    limit,
                    distance
                ],
            )
            .bool()
        };
        reset(&mut e);
        assert!(run(&mut e, 0.9, 0.5));
        // The dot product is 1: a limit above it fails, one equal to it does not.
        assert!(!run(&mut e, 1.5, 0.5));
        assert!(run(&mut e, 1.0, 0.5));
        // The starts 3 apart (squared 3 with y and z): distance 0.5 fails on them.
        put_vec(&mut e, b, [1.0, 1.0, 1.0, 0.0]);
        assert!(!run(&mut e, 0.9, 0.5));
        assert!(run(&mut e, 0.9, 3.0));
        // The ends far apart but the starts together.
        reset(&mut e);
        put_vec(&mut e, b + 0x10, [3.0, 1.0, 0.0, 0.0]);
        assert!(!run(&mut e, 0.9, 0.5));
        assert!(run(&mut e, 0.9, 5.0));
        // z + (y + x) of the squares: lane 3 is not part of the sum.
        reset(&mut e);
        put_vec(&mut e, b + 0x10, [1.0, 0.0, 0.0, 9.0]);
        assert!(run(&mut e, 0.9, 0.0));
        // A NaN in either limit does not fail the tests.
        assert!(run(&mut e, f32::NAN, 0.5));
        assert!(run(&mut e, 0.9, f32::NAN));
    }

    #[test]
    fn the_bundle_writer_makes_single_and_group_records() {
        let mut e = Engine::new();
        let (starts, ends) = put_rays(&mut e);
        // Two bundles: [2] and [0, 1], and a bundle with no rays.
        let bundles = e.mem.alloc(0x40);
        e.mem.set_u32(bundles, 2);
        e.mem.set_u32(bundles + 0x10, 1);
        e.mem.set_u32(bundles + 0x14, 0);
        e.mem.set_u32(bundles + 0x18, 1);
        e.mem.set_u32(bundles + 0x24, 2);
        e.mem.set_u32(bundles + 0x28, 0);
        e.mem.set_u32(bundles + 0x2c, 0);
        e.mem.set_u32(bundles + 0x30, 0);
        e.mem.set_u32(bundles + 0x34, 0);
        let header = e.mem.alloc(0x10);
        e.mem.set_u32(header, bundles);
        e.mem.set_u32(header + 4, 3);
        let records = 0x5000_0000u32;
        let singles = e.mem.alloc(0x80);
        let groups = e.mem.alloc(0x300);
        e.mem.write(groups, &[0xee; 0x300]);
        let counts = e.mem.alloc(8);
        e.call(
            0x00e2_9a10,
            &args![
                header,
                starts,
                ends,
                records,
                singles,
                counts,
                groups,
                counts + 4
            ],
        );
        assert_eq!(e.mem.u32(counts), 1);
        assert_eq!(e.mem.u32(counts + 4), 2);
        // The single: ray 2's vectors, zero words and the record pointer 2 * 0x60 on.
        assert_eq!(get_vec(&e, singles), [0.0, 5.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, singles + 0x10), [0.0, 15.0, -10.0, 0.0]);
        assert_eq!(e.mem.u8(singles + 0x20), 0);
        assert_eq!(e.mem.u32(singles + 0x24), 0);
        assert_eq!(e.mem.u32(singles + 0x30), records + 0xc0);
        assert_eq!(e.mem.u32(singles + 0x34), 1);
        assert_eq!(e.mem.u32(singles + 0x38), 0);
        assert_eq!(e.mem.u8(singles + 0x3d), 0);
        // The group of two: slots for rays 0 and 1, the count, the pointers and the zeros.
        assert_eq!(get_vec(&e, groups), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, groups + 0x10), [10.0, 0.0, 0.0, 0.0]);
        assert_eq!(get_vec(&e, groups + 0x30), [0.0, 0.1, 0.0, 0.0]);
        assert_eq!(get_vec(&e, groups + 0x40), [10.0, 0.1, 0.0, 0.0]);
        assert_eq!(e.mem.u8(groups + 0x20), 0);
        assert_eq!(e.mem.u32(groups + 0x24), 0);
        assert_eq!(e.mem.u8(groups + 0x50), 0);
        assert_eq!(e.mem.u32(groups + 0xc0), 2);
        assert_eq!(e.mem.u8(groups + 0xc4), 0);
        assert_eq!(e.mem.u32(groups + 0xc8), records);
        assert_eq!(e.mem.u32(groups + 0xcc), records + 0x60);
        assert_eq!(e.mem.u32(groups + 0xe0), 0);
        assert_eq!(e.mem.u32(groups + 0xe4), 0);
        // A third slot is not written for a group of two.
        assert_eq!(e.mem.u32(groups + 0xd0), 0xeeee_eeee);
        // The empty bundle: only its count and the byte (the next 0x100-byte record).
        assert_eq!(e.mem.u32(groups + 0x1c0), 0);
        assert_eq!(e.mem.u8(groups + 0x1c4), 0);
        // An empty array stores two zeros.
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(counts, 9);
        e.mem.set_u32(counts + 4, 9);
        e.call(
            0x00e2_9a10,
            &args![
                header,
                starts,
                ends,
                records,
                singles,
                counts,
                groups,
                counts + 4
            ],
        );
        assert_eq!(e.mem.u32(counts), 0);
        assert_eq!(e.mem.u32(counts + 4), 0);
    }

    #[test]
    fn split_to_octant_normalizes_reserves_and_sorts_the_rays() {
        let mut e = Engine::new();
        let (_, _) = memory_fixture(&mut e);
        e.map(0x0101_1590, 8);
        e.mem.set_f64(0x0101_1590, 0.5);
        e.register(0x00ec_9b00, |_, a| {
            let base = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let exponent = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: base * 10.0 + exponent,
                ..Ret::default()
            }
        });
        let reserves: Log = Rc::new(RefCell::new(Vec::new()));
        let log = reserves.clone();
        e.register_double(0x00c9_08c0, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let data = e.mem.alloc(4 * a[2].max(1));
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let starts = e.mem.alloc(0x30);
        let ends = e.mem.alloc(0x30);
        let gaps = [
            [3.0f32, 4.0, 0.0, 1.0],
            [-3.0, 4.0, 0.0, 0.0],
            [3.0, -4.0, -12.0, 0.0],
        ];
        for (index, gap) in gaps.iter().enumerate() {
            put_vec(&mut e, starts + 16 * index as u32, [1.0, 1.0, 1.0, 1.0]);
            let end: Lanes = std::array::from_fn(|lane| 1.0 + gap[lane]);
            put_vec(&mut e, ends + 16 * index as u32, end);
        }
        let directions = e.mem.alloc(0x30);
        let lengths = e.mem.alloc(0x30);
        let octants = e.mem.alloc(96);
        for octant in 0..8u32 {
            e.mem.set_u32(octants + 12 * octant + 8, 0);
        }
        // Octant 1 has room (capacity 5 >= 3); octant 2 has capacity 2.
        let roomy = e.mem.alloc(20);
        e.mem.set_u32(octants + 12, roomy);
        e.mem.set_u32(octants + 12 + 8, 5);
        let tight = e.mem.alloc(8);
        e.mem.set_u32(octants + 24, tight);
        e.mem.set_u32(octants + 24 + 8, 0x4000_0002);
        let counts = e.mem.alloc(0x20);
        e.mem.write(counts, &[0x77; 0x20]);
        e.call(
            0x00e2_9b80,
            &args![starts, ends, 3i32, directions, lengths, octants, counts],
        );
        for (index, gap) in gaps.iter().enumerate() {
            let squares = [gap[0] * gap[0], gap[1] * gap[1], gap[2] * gap[2]];
            let sum = squares[2] + (squares[1] + squares[0]);
            let factor = rsqrt_estimate(sum);
            let direction = get_vec(&e, directions + 16 * index as u32);
            let expected: Lanes = std::array::from_fn(|lane| gap[lane] * factor);
            assert_eq!(direction, expected);
            // (pow(length, 0.5)): the double returned base * 10 + exponent.
            let length = get_vec(&e, lengths + 16 * index as u32);
            let power = f64::from(sum * factor) * 10.0 + 0.5;
            assert_eq!(length, [power as f32, 0.0, 0.0, 0.0]);
        }
        // Reserves: capacity 0 -> 3 (seven times); capacity 2 -> max(3, 4) = 4; capacity 5 none.
        let wanted: Vec<u32> = reserves.borrow().iter().map(|a| a[2]).collect();
        assert_eq!(wanted.len(), 7);
        assert_eq!(
            reserves.borrow()[1][2..],
            [4, 4],
            "octant 2 (capacity 2) grows to twice its capacity"
        );
        assert_eq!(reserves.borrow()[0][1], octants);
        assert_eq!(wanted.iter().filter(|w| **w == 3).count(), 6);
        // Octant 0: ray 0; octant 1: ray 1 (x negative); octant 6 (y, z negative): ray 2.
        let size = |octant: u32| e.mem.u32(octants + 12 * octant + 4);
        assert_eq!(size(0), 1);
        assert_eq!(size(1), 1);
        assert_eq!(size(2), 0);
        assert_eq!(size(6), 1);
        assert_eq!(e.mem.u32(e.mem.u32(octants)), 0);
        assert_eq!(e.mem.u32(roomy), 1);
        let counted: Vec<u32> = (0..8).map(|i| e.mem.u32(counts + 4 * i)).collect();
        assert_eq!(counted, vec![1, 1, 0, 0, 0, 0, 1, 0]);
    }

    #[test]
    fn split_to_octant_with_no_rays_only_clears_and_reserves() {
        let mut e = Engine::new();
        memory_fixture(&mut e);
        let reserves: Log = Rc::new(RefCell::new(Vec::new()));
        let log = reserves.clone();
        e.register_double(0x00c9_08c0, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let octants = e.mem.alloc(96);
        for octant in 0..8u32 {
            e.mem.set_u32(octants + 12 * octant + 8, 0x8000_0000);
        }
        let counts = e.mem.alloc(0x20);
        e.mem.write(counts, &[0x77; 0x20]);
        let anywhere = e.mem.alloc(0x10);
        e.call(
            0x00e2_9b80,
            &args![anywhere, anywhere, 0i32, anywhere, anywhere, octants, counts],
        );
        // A capacity of zero (the flag bit is masked off) is not below a count of zero.
        assert!(reserves.borrow().is_empty());
        assert_eq!(e.mem.bytes(counts, 0x20), vec![0u8; 0x20]);
    }

    /// Registers the doubles the group builders call: `_CIcos` (returns -0.9 and logs its
    /// argument) and `pow` (returns 1.0).
    fn group_doubles(e: &mut Engine) -> Log {
        let cosines: Log = Rc::new(RefCell::new(Vec::new()));
        let log = cosines.clone();
        e.register_double(0x00ec_9f70, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                st0: -0.9,
                ..Ret::default()
            }
        });
        e.register(0x00ec_9b00, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.map(0x0101_1590, 8);
        e.mem.set_f64(0x0101_1590, 0.5);
        cosines
    }

    /// The group outputs: records base, singles, single count, groups, group count.
    fn group_outputs(e: &mut Engine) -> [u32; 5] {
        [
            0x5000_0000,
            e.mem.alloc(0x80),
            e.mem.alloc(8),
            e.mem.alloc(0x300),
            e.mem.alloc(8),
        ]
    }

    #[test]
    fn the_indexed_builder_bundles_alike_rays() {
        let mut e = Engine::new();
        let (_, frees) = memory_fixture(&mut e);
        let cosines = group_doubles(&mut e);
        let (starts, ends) = put_rays(&mut e);
        let indices = e.mem.alloc(16);
        for (slot, ray) in [0u32, 1, 2].iter().enumerate() {
            e.mem.set_u32(indices + 4 * slot as u32, *ray);
        }
        let out = group_outputs(&mut e);
        e.call(
            0x00e2_9cd0,
            &args![
                indices, 3i32, starts, ends, 0u32, 0.25f32, 2.0f32, out[0], out[1], out[2], out[3],
                out[4]
            ],
        );
        // The cosine argument went through FLD (a float widened).
        assert_eq!(cosines.borrow().len(), 1);
        assert_eq!(
            f64::from_bits(
                u64::from(cosines.borrow()[0][0]) | u64::from(cosines.borrow()[0][1]) << 32
            ),
            0.25
        );
        // Rays 0 and 1 share a bundle (a group of two), ray 2 has one to itself.
        assert_eq!(e.mem.u32(out[2]), 1);
        assert_eq!(e.mem.u32(out[4]), 1);
        assert_eq!(e.mem.u32(out[1] + 0x30), out[0] + 2 * 0x60);
        assert_eq!(e.mem.u32(out[3] + 0xc0), 2);
        assert_eq!(e.mem.u32(out[3] + 0xc8), out[0]);
        assert_eq!(e.mem.u32(out[3] + 0xcc), out[0] + 0x60);
        // The router's next address is back where it was and nothing was freed (the flag).
        assert_eq!(e.mem.u32(ROUTER_OBJECT), TEMP_STACK);
        assert!(frees.borrow().is_empty());
    }

    #[test]
    fn the_indexed_builder_starts_a_bundle_for_each_ray_when_none_is_alike() {
        let mut e = Engine::new();
        memory_fixture(&mut e);
        group_doubles(&mut e);
        let (starts, ends) = put_rays(&mut e);
        let indices = e.mem.alloc(16);
        // Rays 2 and 0, and a distance limit of zero: no two rays are close enough.
        e.mem.set_u32(indices, 2);
        e.mem.set_u32(indices + 4, 0);
        let out = group_outputs(&mut e);
        e.call(
            0x00e2_9cd0,
            &args![
                indices, 2i32, starts, ends, 0u32, 0.25f32, 0.0f32, out[0], out[1], out[2], out[3],
                out[4]
            ],
        );
        assert_eq!(e.mem.u32(out[2]), 2);
        assert_eq!(e.mem.u32(out[4]), 0);
        assert_eq!(e.mem.u32(out[1] + 0x30), out[0] + 2 * 0x60);
        assert_eq!(e.mem.u32(out[1] + 0x40 + 0x30), out[0]);
        // No rays at all: nothing is written and both counts are zero.
        e.mem.set_u32(out[2], 5);
        e.mem.set_u32(out[4], 5);
        e.call(
            0x00e2_9cd0,
            &args![
                indices, 0i32, starts, ends, 0u32, 0.25f32, 0.0f32, out[0], out[1], out[2], out[3],
                out[4]
            ],
        );
        assert_eq!(e.mem.u32(out[2]), 0);
        assert_eq!(e.mem.u32(out[4]), 0);
    }

    #[test]
    fn the_octant_builder_takes_only_the_rays_of_its_octant() {
        let mut e = Engine::new();
        memory_fixture(&mut e);
        group_doubles(&mut e);
        let (starts, ends) = put_rays(&mut e);
        // Octant 0 (no negative gap): rays 0 and 1 form one bundle; ray 2 (z gap -10) is octant 4.
        let out = group_outputs(&mut e);
        e.call(
            0x00e2_a010,
            &args![
                0i32, starts, ends, 3i32, 0.25f32, 2.0f32, out[0], out[1], out[2], out[3], out[4]
            ],
        );
        assert_eq!(e.mem.u32(out[2]), 0);
        assert_eq!(e.mem.u32(out[4]), 1);
        assert_eq!(e.mem.u32(out[3] + 0xc0), 2);
        assert_eq!(e.mem.u32(out[3] + 0xc8), out[0]);
        assert_eq!(e.mem.u32(out[3] + 0xcc), out[0] + 0x60);
        // Octant 4: only ray 2, alone.
        e.call(
            0x00e2_a010,
            &args![
                4i32, starts, ends, 3i32, 0.25f32, 2.0f32, out[0], out[1], out[2], out[3], out[4]
            ],
        );
        assert_eq!(e.mem.u32(out[2]), 1);
        assert_eq!(e.mem.u32(out[4]), 0);
        assert_eq!(e.mem.u32(out[1] + 0x30), out[0] + 2 * 0x60);
        // An octant nobody is in.
        e.call(
            0x00e2_a010,
            &args![
                7i32, starts, ends, 3i32, 0.25f32, 2.0f32, out[0], out[1], out[2], out[3], out[4]
            ],
        );
        assert_eq!(e.mem.u32(out[2]), 0);
        assert_eq!(e.mem.u32(out[4]), 0);
        assert_eq!(e.mem.u32(ROUTER_OBJECT), TEMP_STACK);
    }

    #[test]
    fn the_builders_keep_their_scratch_vectors_at_the_exe_frame_offsets() {
        let mut e = Engine::new();
        memory_fixture(&mut e);
        group_doubles(&mut e);
        let (starts, ends) = put_rays(&mut e);
        let indices = e.mem.alloc(16);
        e.mem.set_u32(indices, 0);
        e.mem.set_u32(indices + 4, 1);
        let out = group_outputs(&mut e);
        let x_of = |e: &Engine, at: u32| (e.mem.f32(at), e.mem.f32(at + 4));
        // The frame block is the next stack local. Placing ray 1 leaves it (the new ray) and ray
        // 0 (the bundle's first ray) in the frame: in the indexed builder the new ray's start and
        // end at +0x60 and +0x70, the other's at +0x50 and +0x80.
        let raw = e.next_stack_local(INDEXED_FRAME_BELOW + 0x10);
        let frame = aligned_frame_at(raw, INDEXED_FRAME_BELOW);
        e.call(
            0x00e2_9cd0,
            &args![
                indices, 2i32, starts, ends, 0u32, 0.25f32, 2.0f32, out[0], out[1], out[2], out[3],
                out[4]
            ],
        );
        assert_eq!(x_of(&e, frame + 0x60), (0.0, 0.1));
        assert_eq!(x_of(&e, frame + 0x70), (10.0, 0.1));
        assert_eq!(x_of(&e, frame + 0x50), (0.0, 0.0));
        assert_eq!(x_of(&e, frame + 0x80), (10.0, 0.0));
        // The octant builder: the new ray's start and end at +0x80 and +0x50, the other's at
        // +0x70 and +0x60.
        let raw = e.next_stack_local(OCTANT_FRAME_BELOW + 0x10);
        let frame = aligned_frame_at(raw, OCTANT_FRAME_BELOW);
        e.call(
            0x00e2_a010,
            &args![
                0i32, starts, ends, 2i32, 0.25f32, 2.0f32, out[0], out[1], out[2], out[3], out[4]
            ],
        );
        assert_eq!(x_of(&e, frame + 0x80), (0.0, 0.1));
        assert_eq!(x_of(&e, frame + 0x50), (10.0, 0.1));
        assert_eq!(x_of(&e, frame + 0x70), (0.0, 0.0));
        assert_eq!(x_of(&e, frame + 0x60), (10.0, 0.0));
    }

    /// The start-up function's collaborators: the job queue's `addJob` (logging the job's
    /// 0x70 bytes as words), the runner object (slots `+0xc` and `+0x10`) and the semaphore.
    struct JobFixture {
        jobs: Log,
        events: Rc<RefCell<Vec<String>>>,
        params: u32,
        runner: u32,
    }

    fn job_fixture(e: &mut Engine) -> JobFixture {
        e.map(MONITOR_STREAM, 0x100);
        e.map(0x0126_835c, 4);
        e.mem.set_u32(0x0126_835c, TLS_INDEX);
        e.register(0x00fd_f044, |_, _| Ret {
            eax: MONITOR_STREAM,
            ..Ret::default()
        });
        e.mem.set_u32(MONITOR_STREAM + 4, MONITOR_STREAM + 0x20);
        e.mem.set_u32(MONITOR_STREAM + 0xc, MONITOR_STREAM + 0x80);
        let jobs: Log = Rc::new(RefCell::new(Vec::new()));
        let events: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let log = jobs.clone();
        let order = events.clone();
        e.register_double(0x00d0_4b90, move |e, a| {
            let words: Vec<u32> = (0..0x1c).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            assert_eq!(a[2], 1);
            order.borrow_mut().push("job".to_string());
            log.borrow_mut().push([vec![a[0]], words].concat());
            Ret::default()
        });
        let order = events.clone();
        e.register_double(0x00d0_4380, move |_, a| {
            order.borrow_mut().push(format!("acquire {:x}", a[0]));
            Ret::default()
        });
        let runner = e.mem.alloc(8);
        e.mem.set_u32(runner, 0x0073_0000);
        e.put_vtable(0x0073_0000, &[0, 0, 0, 0x0073_1000, 0x0073_1010]);
        let order = events.clone();
        e.register_double(0x0073_1000, move |_, a| {
            order
                .borrow_mut()
                .push(format!("run {:x} {:x}", a[1], a[2]));
            Ret::default()
        });
        let order = events.clone();
        e.register_double(0x0073_1010, move |_, _| {
            order.borrow_mut().push("wait".to_string());
            Ret::default()
        });
        let params = e.mem.alloc(0x50);
        for word in 0..0x14u32 {
            e.mem.set_u32(params + 4 * word, 0x1000 + word);
        }
        e.mem.set_u32(params + 0x10, 0x5e5a);
        e.mem.set_u32(params + 0x24, 0x0100_0000);
        e.mem.set_u32(params + 0x2c, 0x0200_0000);
        JobFixture {
            jobs,
            events,
            params,
            runner,
        }
    }

    #[test]
    fn the_auto_job_path_queues_eight_jobs_and_waits() {
        let mut e = Engine::new();
        let fixture = job_fixture(&mut e);
        e.call(
            0x00e2_a390,
            &args![
                0x10u32,
                0x20u32,
                3u32,
                1.5f32,
                2.5f32,
                fixture.params,
                0x77u32,
                fixture.runner,
                0x99u32,
                true
            ],
        );
        let jobs = fixture.jobs.borrow();
        assert_eq!(jobs.len(), 8);
        for (index, job) in jobs.iter().enumerate() {
            let index = index as u32;
            assert_eq!(job[0], 0x77, "the queue");
            let words = &job[1..];
            assert_eq!(words[0] & 0xff_ffff, 0x01_0307);
            assert_eq!(words[1], 0xffff_0070);
            assert_eq!(words[4..7], [0, 0, 0]);
            assert_eq!(words[8], 0);
            assert_eq!(words[9], index);
            assert_eq!(words[10], 0);
            assert_eq!(words[11], 0x10);
            assert_eq!(words[12], 0x20);
            assert_eq!(words[13], 3);
            assert_eq!(f32::from_bits(words[14]), 1.5);
            assert_eq!(f32::from_bits(words[15]), 2.5);
            assert_eq!(words[16], 0x99);
            assert_eq!(words[17], 0x0100_0000 + index * 3 * 0x40);
            assert_eq!(words[19], 0x0200_0000 + index * 3 * 0x100);
            // The words copied from the parameter block.
            assert_eq!(words[21], 0x1000 + 7);
            assert_eq!(words[22], 0x1000 + 5);
            assert_eq!(words[23], 0x5e5a);
            assert_eq!(words[24], 0x1000 + 0xd);
            assert_eq!(words[25], 0x1000 + 0xe);
            assert_eq!(words[26], 0x1000 + 0xf);
            assert_eq!(words[27], 0x1000 + 0x10);
        }
        let events = fixture.events.borrow();
        assert_eq!(events.len(), 11);
        assert_eq!(events[8], "run 77 b");
        assert_eq!(events[9], "wait");
        assert_eq!(events[10], "acquire 5e5a");
        // Records: "LtAutoJob" (16 bytes, second label at +0xc), then two of 12 bytes.
        let first = MONITOR_STREAM + 0x20;
        assert_eq!(e.mem.u32(first), LABEL_AUTO_JOB);
        assert_eq!(e.mem.u32(first + 0xc), LABEL_ADD_JOBS);
        assert_eq!(e.mem.u32(first + 0x10), LABEL_WAIT_THREADS);
        assert_eq!(e.mem.u32(first + 0x1c), LABEL_BEFORE_WAIT);
        assert_eq!(e.mem.u32(MONITOR_STREAM + 4), first + 0x28);
    }

    #[test]
    fn the_cpu_path_splits_the_rays_and_queues_a_job_per_octant() {
        let mut e = Engine::new();
        let fixture = job_fixture(&mut e);
        let (allocs, frees) = memory_fixture(&mut e);
        e.map(0x0101_1590, 8);
        e.register(0x00ec_9b00, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register_double(0x00c9_08c0, |e, a| {
            let data = e.mem.alloc(4 * a[2].max(1));
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let (starts, ends) = put_rays(&mut e);
        e.call(
            0x00e2_a390,
            &args![
                starts,
                ends,
                3u32,
                1.5f32,
                2.5f32,
                fixture.params,
                0x77u32,
                fixture.runner,
                0x99u32,
                false
            ],
        );
        // Allocations: directions, lengths (0x30 each), the octant arrays (0x60) and two of 0x20.
        let sizes: Vec<u32> = allocs.borrow().iter().map(|a| a[0]).collect();
        assert_eq!(sizes, vec![0x30, 0x30, 0x60, 0x20, 0x20]);
        // Rays 0 and 1 are in octant 0 (both along +x) and ray 2 in octant 4: two jobs, each
        // with the size of its octant array.
        let jobs = fixture.jobs.borrow();
        assert_eq!(jobs.len(), 2);
        let sizes_in_jobs: Vec<u32> = jobs.iter().map(|j| j[1 + 10]).collect();
        assert_eq!(sizes_in_jobs, vec![2, 1]);
        for (index, job) in jobs.iter().enumerate() {
            let words = &job[1..];
            assert_eq!(words[0] & 0xff_ffff, 0x01_0307);
            // The octant array's data pointer and its size.
            assert_ne!(words[8], 0);
            assert_eq!(words[13], 3);
            assert_eq!(words[17], 0x0100_0000 + index as u32 * 3 * 0x40);
            assert_eq!(words[19], 0x0200_0000 + index as u32 * 3 * 0x100);
        }
        let events = fixture.events.borrow();
        assert_eq!(events[2..], ["run 77 b", "wait", "acquire 5e5a"]);
        // Frees: both scratch buffers (0x20 bytes), the data of the eight octant arrays (each
        // reserved for three ints, 12 bytes), the block of arrays (0x60), lengths, directions.
        let freed: Vec<u32> = frees.borrow().iter().map(|f| f[1]).collect();
        let mut expected = vec![0x20, 0x20];
        expected.extend([12; 8]);
        expected.extend([0x60, 0x30, 0x30]);
        assert_eq!(freed, expected);
        // The first record is "TtCPUSplit" (12 bytes).
        assert_eq!(e.mem.u32(MONITOR_STREAM + 0x20), LABEL_CPU_SPLIT);
    }

    #[test]
    fn the_job_and_the_octant_counts_sit_at_the_exe_frame_offsets() {
        let mut e = Engine::new();
        let fixture = job_fixture(&mut e);
        memory_fixture(&mut e);
        e.map(0x0101_1590, 8);
        e.register(0x00ec_9b00, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register_double(0x00c9_08c0, |e, a| {
            let data = e.mem.alloc(4 * a[2].max(1));
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let (starts, ends) = put_rays(&mut e);
        // The frame block is the next stack local: the job at +0x60, the counts at +0xd0.
        let raw = e.next_stack_local(AUTO_BUNDLE_FRAME_BELOW + 0x10);
        let frame = aligned_frame_at(raw, AUTO_BUNDLE_FRAME_BELOW);
        e.call(
            0x00e2_a390,
            &args![
                starts,
                ends,
                3u32,
                1.5f32,
                2.5f32,
                fixture.params,
                0x77u32,
                fixture.runner,
                0x99u32,
                false
            ],
        );
        // Two rays in octant 0, one in octant 4; the last job queued (octant 4) is still there.
        let counts: Vec<u32> = (0..8).map(|i| e.mem.u32(frame + 0xd0 + 4 * i)).collect();
        assert_eq!(counts, vec![2, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(e.mem.u32(frame + 0x60 + 4), 0xffff_0070);
        assert_eq!(e.mem.u32(frame + 0x60 + 0x28), 1);
        // The auto-job path: the eighth job's index word.
        let raw = e.next_stack_local(AUTO_BUNDLE_FRAME_BELOW + 0x10);
        let frame = aligned_frame_at(raw, AUTO_BUNDLE_FRAME_BELOW);
        e.call(
            0x00e2_a390,
            &args![
                0x10u32,
                0x20u32,
                3u32,
                1.5f32,
                2.5f32,
                fixture.params,
                0x77u32,
                fixture.runner,
                0x99u32,
                true
            ],
        );
        assert_eq!(e.mem.u32(frame + 0x60 + 0x24), 7);
    }

    #[test]
    fn the_cpu_path_with_no_rays_allocates_no_ray_buffers() {
        let mut e = Engine::new();
        let fixture = job_fixture(&mut e);
        let (allocs, frees) = memory_fixture(&mut e);
        e.register_double(0x00c9_08c0, |_, _| Ret::default());
        e.register(0x00ec_9b00, |_, _| Ret::default());
        e.map(0x0101_1590, 8);
        let anywhere = e.mem.alloc(0x20);
        e.call(
            0x00e2_a390,
            &args![
                anywhere,
                anywhere,
                0u32,
                1.5f32,
                2.5f32,
                fixture.params,
                0x77u32,
                fixture.runner,
                0x99u32,
                false
            ],
        );
        let sizes: Vec<u32> = allocs.borrow().iter().map(|a| a[0]).collect();
        assert_eq!(sizes, vec![0x60, 0x20, 0x20]);
        assert!(fixture.jobs.borrow().is_empty());
        // The ray buffers carry the flag alone: only the arrays' own data (none, they stayed
        // empty), the block of arrays and the two scratch buffers are given back.
        let freed: Vec<u32> = frees.borrow().iter().map(|f| f[1]).collect();
        assert_eq!(freed, vec![0x20, 0x20, 0x60]);
    }
}
