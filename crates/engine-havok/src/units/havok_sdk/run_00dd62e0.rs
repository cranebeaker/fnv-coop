//! `Havok SDK/run_00dd62e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These are the helpers of the plane-equation to convex-vertices computation (the caller is
//! `00dd6270`): planes are 16-byte vectors `(nx, ny, nz, w)` in an `hkArray` (`{ data, size,
//! capacity word }`, the capacity in the low 30 bits of the word and the top bit set for storage
//! the array does not own) and a vertex is where three planes meet. The functions have no
//! names in the engine map; what each does is described at the function. Where a function
//! reaches the container allocator it asks the memory router (`00c85750`): its word at `+0x10`
//! is the allocator the arrays grow and free through (`00c90940`, `00c908c0`, virtual function
//! `+0x10`), and its word at `+0x14` the allocator the temporary arrays of `00dd6ec0` and
//! `00dd6f20` come from (virtual function `+4` allocates a number of bytes, `+8` frees a block
//! of a given size).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, compare_flags, load_bits, load_vec4, mul4, rotate_c9, rotate_d2, shufps, store_vec4,
    sub4, sum3, test_parity_even, Vec4,
};

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

/// The memory router getter (`00c85750`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl: allocator, array, capacity, element
/// size) and the function that makes room for one more element (`00c90940`, cdecl: allocator,
/// array, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The sort `00dd6f20` runs (`00dd7d00`, cdecl: the array of pointers, 0, the last index, the
/// comparison function) and the comparison function it passes (`00dd6ea0`).
const QUICK_SORT: u32 = 0x00dd_7d00;
const KEY_COMPARE: u32 = 0x00dd_6ea0;
/// The low 30 bits of an array's capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// `float`s and vectors read from the exe image: the sign mask (`010c44d0`, four words), the
/// zero (`01011d78`), the negative zero (`010c4990`), the tolerance of the determinant
/// (`0104da4c`), the largest and smallest `float` (`0104da48`, `0104e148`), the negative
/// tolerance of the side test (`010c8440`), the far distance of the first test point
/// (`010b9be8`, -100000.0) and the zero vector (`010c7160`).
const SIGN_MASK: u32 = 0x010c_44d0;
const ZERO_FLOAT: u32 = 0x0101_1d78;
const NEGATIVE_ZERO: u32 = 0x010c_4990;
const DETERMINANT_TOLERANCE: u32 = 0x0104_da4c;
const LARGEST_FLOAT: u32 = 0x0104_da48;
const SMALLEST_FLOAT: u32 = 0x0104_e148;
const SIDE_TOLERANCE: u32 = 0x010c_8440;
const FAR_DISTANCE: u32 = 0x010b_9be8;
const ZERO_VECTOR: u32 = 0x010c_7160;
/// `float` below which a plane is not facing the direction (`0109afc4`), and the `double`
/// (`01032980`) two distances may differ by to count as the same.
const FACING_LIMIT: u32 = 0x0109_afc4;
const TIE_TOLERANCE: u32 = 0x0103_2980;
/// The `double` of the pair search's upper limit (`0106b9e8`) and the `double` the vertex
/// deduplication keeps its window by (`01016408`).
const PAIR_DOT_LIMIT: u32 = 0x0106_b9e8;
const WINDOW: u32 = 0x0101_6408;

/// The allocator behind the memory router's word at `+0x10` (the router is asked, then the word
/// read).
fn container_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The allocator behind the router's word at `+0x14`.
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x14)
}

/// Asks the router, then makes room for one more element of an `hkArray` when its size equals
/// its capacity (`00c90940`).
fn make_room(e: &mut Engine, array: u32, element: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element]);
    }
}

/// Appends the `words` words at `from` to an `hkArray` of `words * 4` byte elements (the room
/// is made first; the copy is skipped when the slot's address is 0, as in the code).
fn push_record(e: &mut Engine, array: u32, from: u32, words: u32) {
    make_room(e, array, words * 4);
    let slot = e
        .mem
        .u32(array + 4)
        .wrapping_mul(words * 4)
        .wrapping_add(e.mem.u32(array));
    if slot != 0 {
        for word in 0..words {
            let value = e.mem.u32(from + 4 * word);
            e.mem.set_u32(slot + 4 * word, value);
        }
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Releases an array's storage the way the code does at the end of a function: the router is
/// asked, the size is set to 0, and when the capacity word's sign bit is clear the allocator's
/// virtual function `+0x10` frees `data` with `capacity * unit` bytes.
fn release_storage(e: &mut Engine, array: u32, unit_shift: u32) {
    let allocator = container_allocator(e);
    let capacity = e.mem.u32(array + 8);
    let data = e.mem.u32(array);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & CAPACITY_MASK) << unit_shift;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// The plane at index `index` of the plane array (`planes` points at the `hkArray`; the data
/// pointer is read each time).
fn plane_at(e: &Engine, planes: u32, index: u32) -> Vec4 {
    let data = e.mem.u32(planes);
    load_vec4(e, data.wrapping_add(index << 4))
}

/// `rot_c9(p) * rot_d2(q) - rot_d2(p) * rot_c9(q)`: the cross-product form of `00dd6380`.
fn cross_form(p: Vec4, q: Vec4) -> Vec4 {
    sub4(
        mul4(rotate_c9(p), rotate_d2(q)),
        mul4(rotate_d2(p), rotate_c9(q)),
    )
}

// Translated from 00dd62e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `planes` (an `hkArray` of 16-byte vectors), `i`, `j`, `k`,
/// `out`). With `a = plane[j]`, `b = plane[i]` and `c = plane[k]`, writes at `out` the vector
/// `n = rot_d2(a) * rot_c9(b) - rot_c9(a) * rot_d2(b)` and, when `c . n` (lanes 0 to 2 summed
/// in the code's order) is above 0 (x87 compare, a NaN is not), stores `n` with its sign bits
/// flipped (the mask at `010c44d0`) instead.
pub fn fn_00dd62e0(e: &mut Engine, planes: u32, i: i32, j: i32, k: i32, out: u32) {
    let a = plane_at(e, planes, j as u32);
    let b = plane_at(e, planes, i as u32);
    let n = sub4(
        mul4(rotate_d2(a), rotate_c9(b)),
        mul4(rotate_c9(a), rotate_d2(b)),
    );
    store_vec4(e, out, n);
    let c = plane_at(e, planes, k as u32);
    let along = sum3(mul4(c, n));
    let flags = compare_flags(0.0, x87_m32(along.to_bits()));
    if !test_parity_even(flags, 5) {
        let mask = load_bits(e, SIGN_MASK);
        for lane in 0..4 {
            let bits = n[lane].to_bits() ^ mask[lane];
            e.mem.set_u32(out + 4 * lane as u32, bits);
        }
    }
}

// Translated from 00dd6380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `planes`, `ia`, `ib`, `ic`, `out`). Finds the point where
/// the three planes `a = plane[ia]`, `b = plane[ib]` and `c = plane[ic]` meet. With
/// `x1 = cross_form(b, c)`, `x2 = cross_form(c, a)`, `x3 = cross_form(a, b)` and the determinant
/// `det = a . x1` (lanes 0 to 2 summed in the code's order), answers 1 (and writes nothing) when
/// `|det|` (stored as a `float`) is below the `float` at `0104da4c` (x87 compare; a NaN is not
/// below). Otherwise writes at `out` the vector `(1 / det) * ((v1 * x2 + v0 * x1) + v2 * x3)`
/// where `v = (-0 - a.w, -0 - b.w, -0 - c.w, 0)` (the `float` at `010c4990` is the -0, the zero
/// at `01011d78`; each `v` lane is splatted), then the indices at `out + 0x10`, `+0x14` and
/// `+0x18`: `ia` and `ib` in increasing order (signed) and `ic`. Answers 0.
pub fn fn_00dd6380(e: &mut Engine, planes: u32, ia: i32, ib: i32, ic: i32, out: u32) -> u32 {
    let c = plane_at(e, planes, ic as u32);
    let b = plane_at(e, planes, ib as u32);
    let x1 = cross_form(b, c);
    let a = plane_at(e, planes, ia as u32);
    let x2 = cross_form(c, a);
    let x3 = cross_form(a, b);
    let det = sum3(mul4(a, x1));
    let det_loaded = x87_load32(det.to_bits());
    let magnitude = x87_store32(det_loaded.abs());
    let tolerance = x87_m32(e.mem.u32(DETERMINANT_TOLERANCE));
    let flags = compare_flags(x87_load32(magnitude), tolerance);
    if !test_parity_even(flags, 5) {
        return 1;
    }
    let inverse = f32::from_bits(x87_store32(x87_div(1.0, det_loaded)));
    let negative_zero = f32::from_bits(e.mem.u32(NEGATIVE_ZERO));
    let zero = f32::from_bits(e.mem.u32(ZERO_FLOAT));
    let lane = |p: Vec4| negative_zero - p[3];
    let v: Vec4 = [lane(a), lane(b), lane(c), zero];
    let sum = add4(
        add4(mul4([v[1]; 4], x2), mul4([v[0]; 4], x1)),
        mul4([v[2]; 4], x3),
    );
    store_vec4(e, out, sum);
    store_vec4(e, out, mul4([inverse; 4], sum));
    e.mem.set_i32(out + 0x10, ia);
    e.mem.set_i32(out + 0x14, ib);
    e.mem.set_i32(out + 0x18, ic);
    if ia > ib {
        e.mem.set_i32(out + 0x10, ib);
        e.mem.set_i32(out + 0x14, ia);
    }
    0
}

// Translated from 00dd6510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `direction` (a 16-byte vector), `planes`, `i1`, `i2`,
/// `out_b`, `out_a`). Finds the two ends of the edge where planes `i1` and `i2` meet, along
/// `direction`. The words at `+0x10`, `+0x14` and `+0x18` of both outputs start as all ones
/// ("none"). For every other plane `i` it meets the three planes with `00dd6380`; when that
/// answers 0 the first such vertex seeds the outputs (`out_a = -100000 * direction + vertex`,
/// the `float` at `010b9be8`, and `out_b` that vector with its sign bits flipped); then
/// `out_a` is replaced by the vertex (its vector and its three plane indices) when `(out_a -
/// vertex) . plane[i]` (lanes 0 to 2 summed in the code's order) is above the `float` at
/// `010c8440` (-1.19e-7; x87 compare, a NaN is not above), and `out_b` likewise with its own
/// vector. At the end, when one output still has no indices it takes the other's (answering 1
/// when both have none); the answer is 0 when both end with indices.
pub fn fn_00dd6510(
    e: &mut Engine,
    direction: u32,
    planes: u32,
    i1: i32,
    i2: i32,
    out_b: u32,
    out_a: u32,
) -> u32 {
    let none = 0xffff_ffffu32;
    for offset in [0x10u32, 0x14, 0x18] {
        e.mem.set_u32(out_b + offset, none);
    }
    for offset in [0x10u32, 0x14, 0x18] {
        e.mem.set_u32(out_a + offset, none);
    }
    let mut seeded = false;
    if e.mem.i32(planes + 4) > 0 {
        // The exe aligns ESP, then `SUB ESP,0x54` and three pushes: a 0x60-byte frame, the hit at
        // +0x40 (00dd6574).
        with_aligned_frame(e, 0x60, |e, frame| {
            let hit = frame + 0x40;
            let mut i = 0i32;
            loop {
                if i != i1 && i != i2 && fn_00dd6380(e, planes, i1, i2, i, hit) == 0 {
                    let vertex = load_vec4(e, hit);
                    if !seeded {
                        let far = f32::from_bits(e.mem.u32(FAR_DISTANCE));
                        let along = load_vec4(e, direction);
                        let seed = add4(mul4([far; 4], along), vertex);
                        store_vec4(e, out_a, seed);
                        let mask = load_bits(e, SIGN_MASK);
                        for lane in 0..4 {
                            let bits = seed[lane].to_bits() ^ mask[lane];
                            e.mem.set_u32(out_b + 4 * lane as u32, bits);
                        }
                        seeded = true;
                    }
                    let tolerance = x87_load32(e.mem.u32(SIDE_TOLERANCE));
                    let indices = [0x10u32, 0x14, 0x18].map(|o| e.mem.u32(hit + o));
                    for out in [out_a, out_b] {
                        let plane = plane_at(e, planes, i as u32);
                        let current = load_vec4(e, out);
                        let side = sum3(mul4(sub4(current, vertex), plane));
                        let flags = compare_flags(tolerance, x87_m32(side.to_bits()));
                        if !test_parity_even(flags, 5) {
                            store_vec4(e, out, vertex);
                            for (offset, index) in [0x10u32, 0x14, 0x18].into_iter().zip(indices) {
                                e.mem.set_u32(out + offset, index);
                            }
                        }
                    }
                }
                i += 1;
                if i >= e.mem.i32(planes + 4) {
                    break;
                }
            }
        });
    }
    if e.mem.u32(out_b + 0x10) == none {
        let first = e.mem.u32(out_a + 0x10);
        if first == none {
            return 1;
        }
        for offset in [0x10u32, 0x14, 0x18] {
            let word = e.mem.u32(out_a + offset);
            e.mem.set_u32(out_b + offset, word);
        }
    }
    if e.mem.u32(out_a + 0x10) == none {
        let first = e.mem.u32(out_b + 0x10);
        if first == none {
            return 1;
        }
        for offset in [0x10u32, 0x14, 0x18] {
            let word = e.mem.u32(out_b + offset);
            e.mem.set_u32(out_a + offset, word);
        }
    }
    0
}

// Translated from 00dd66d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00dd66d0 - the code copies eight words of its 0x20-byte local result
// (`00dd6380` writes the first seven) into every record it stores; the eighth word is whatever
// the stack held (here the frame word at +0x5c, which starts at 0).
/// Unnamed in the engine map (cdecl: `out_flag`, `direction` (a 16-byte vector), `planes`,
/// `i1`, `i2`, `point` (a 16-byte vector), `results` (an `hkArray` of 0x20-byte records)).
/// Answers `out_flag`, after storing a byte there: 1 when a vertex was recorded.
///
/// For every plane `i` other than `i1` and `i2` whose normal has a dot product with `direction`
/// above the `float` at `0109afc4` (x87 compare), the code sets `point.w` to 1.0 and forms the
/// distance `(0 - (plane . point))` (all four lanes, `(x3 + x1) + (x2 + x0)`, minus the zero
/// vector's lane 0) divided by `|dot|` (x87, each stored as a `float`). When that distance is
/// below `best + 0.0001` (the `double` at `01032980`; `best` starts as the largest `float`,
/// `0104da48`) and `00dd6380` meets planes `i1`, `i2` and `i` (answering 0) the vertex record
/// (`00dd6380`'s output) is stored in `results`: appended when the distance is within 0.0001 of
/// `best` (x87 compare of the absolute difference, a NaN counts as not within), otherwise
/// `results` is emptied first and `best` becomes the distance.
pub fn fn_00dd66d0(
    e: &mut Engine,
    out_flag: u32,
    direction: u32,
    planes: u32,
    i1: i32,
    i2: i32,
    point: u32,
    results: u32,
) -> u32 {
    let mut best = x87_quiet_bits(e.mem.u32(LARGEST_FLOAT));
    let mut found = 0u8;
    if e.mem.i32(planes + 4) > 0 {
        // The exe aligns ESP, then `SUB ESP,0x54` and three pushes: a 0x60-byte frame, the hit at
        // +0x40 (00dd67cb).
        with_aligned_frame(e, 0x60, |e, frame| {
            let hit = frame + 0x40;
            let mut i = 0i32;
            let mut offset = 0u32;
            loop {
                if i != i1 && i != i2 {
                    let data = e.mem.u32(planes);
                    let plane = load_vec4(e, data.wrapping_add(offset));
                    let along = load_vec4(e, direction);
                    let dot = sum3(mul4(plane, along));
                    let limit = x87_load32(e.mem.u32(FACING_LIMIT));
                    let dot_loaded = x87_load32(dot.to_bits());
                    let flags = compare_flags(dot_loaded, limit);
                    if flags & 0x41 == 0 {
                        e.mem.set_u32(point + 0xc, 1.0f32.to_bits());
                        let data = e.mem.u32(planes);
                        let position = load_vec4(e, point);
                        let normal = load_vec4(e, data.wrapping_add(offset));
                        let product = mul4(normal, position);
                        let pairs = add4(shufps(product, product, 0x4e), product);
                        let total = add4(shufps(pairs, pairs, 0xb1), pairs)[0];
                        let zero = f32::from_bits(e.mem.u32(ZERO_VECTOR));
                        let negated = zero - total;
                        let magnitude = x87_store32(dot_loaded.abs());
                        let distance =
                            x87_store32(x87_div(x87_load32(negated.to_bits()), x87_m32(magnitude)));
                        let tolerance = e.mem.f64(TIE_TOLERANCE);
                        let reach = x87_add(x87_load32(best), tolerance);
                        let flags = compare_flags(reach, x87_load32(distance));
                        if flags & 0x41 == 0 && fn_00dd6380(e, planes, i1, i2, i, hit) == 0 {
                            found = 1;
                            let difference =
                                x87_store32(x87_sub(x87_load32(distance), x87_m32(best)));
                            let size = x87_store32(x87_load32(difference).abs());
                            let flags = compare_flags(x87_load32(size), tolerance);
                            if test_parity_even(flags, 5) {
                                e.mem.set_u32(results + 4, 0);
                                push_record(e, results, hit, 8);
                                best = x87_quiet_bits(distance);
                            } else {
                                push_record(e, results, hit, 8);
                            }
                        }
                    }
                }
                i += 1;
                offset = offset.wrapping_add(0x10);
                if i >= e.mem.i32(planes + 4) {
                    break;
                }
            }
        });
    }
    e.mem.set_u8(out_flag, found);
    out_flag
}

// Translated from 00dd68d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `tolerance` (a `float`), `array` (an `hkArray` of 16-byte
/// vectors, sorted by their first lane), `out_count`). Removes the vectors that lie within the
/// squared distance `tolerance` of one kept before them. Each vector in turn is compared with
/// the kept ones, newest first, for as long as the kept vector's first lane is not below the
/// vector's first lane minus 0.01 (the `double` at `01016408`; x87 compares, a NaN keeps
/// scanning): if the squared distance (lanes 0 to 2 summed in the code's order) is below the
/// tolerance the vector is dropped, together with the vectors that follow it and are within
/// the tolerance of the kept one it matched; otherwise it is kept. The kept vectors are packed
/// at the start of the array; the number kept is stored at `out_count` and becomes the array's
/// size, after reserving room (`00c908c0`, element size 0x10; the larger of twice the capacity
/// and the number) when the capacity is smaller.
pub fn fn_00dd68d0(e: &mut Engine, tolerance: f32, array: u32, out_count: u32) {
    let data = e.mem.u32(array);
    let mut remaining = e.mem.i32(array + 4).wrapping_sub(1);
    let mut source = data;
    let mut kept_end = data;
    let mut scan = data.wrapping_sub(0x10);
    if remaining >= 0 {
        let tolerance = x87_load32(tolerance.to_bits());
        let window = e.mem.f64(WINDOW);
        loop {
            let mut keep = true;
            if scan >= e.mem.u32(array) {
                let first_lane = x87_load32(e.mem.u32(source));
                let lowest = x87_sub(first_lane, window);
                let mut at = scan;
                loop {
                    let flags = compare_flags(x87_load32(e.mem.u32(at)), lowest);
                    if !test_parity_even(flags, 5) {
                        break;
                    }
                    let difference = sub4(load_vec4(e, at), load_vec4(e, source));
                    let squared = sum3(mul4(difference, difference));
                    let flags = compare_flags(x87_load32(squared.to_bits()), tolerance);
                    if !test_parity_even(flags, 5) {
                        keep = false;
                        let mut inner = remaining.wrapping_sub(1);
                        if inner >= 0 {
                            let matched = load_vec4(e, at);
                            loop {
                                let next = load_vec4(e, source + 0x10);
                                let difference = sub4(matched, next);
                                let squared = sum3(mul4(difference, difference));
                                let flags = compare_flags(x87_load32(squared.to_bits()), tolerance);
                                if test_parity_even(flags, 5) {
                                    break;
                                }
                                source = source.wrapping_add(0x10);
                                remaining = remaining.wrapping_sub(1);
                                inner = inner.wrapping_sub(1);
                                if inner < 0 {
                                    break;
                                }
                            }
                        }
                        break;
                    }
                    at = at.wrapping_sub(0x10);
                    if at < e.mem.u32(array) {
                        break;
                    }
                }
            }
            if keep {
                let vector = load_vec4(e, source);
                store_vec4(e, kept_end, vector);
                kept_end = kept_end.wrapping_add(0x10);
                scan = scan.wrapping_add(0x10);
            }
            source = source.wrapping_add(0x10);
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
        }
    }
    let count = (kept_end.wrapping_sub(e.mem.u32(array)) as i32) >> 4;
    e.mem.set_i32(out_count, count);
    let allocator = container_allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
    }
    e.mem.set_i32(array + 4, count);
}

// Translated from 00dd6a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00dd6a10 - the two 0x20-byte edge records it builds on its stack for
// the recursive calls set words 0 to 2 and the vector at `+0x10`; the word at `+0xc` is whatever
// the stack held and is copied into the edge array by the recursive call (here the frame
// words at +0x2c / +0x4c, which start at 0).
/// Unnamed in the engine map (cdecl: `edge` (a 0x20-byte record: two plane indices at `+0` and
/// `+4`, a pointer to a 16-byte vertex at `+8`, a direction vector at `+0x10`), `planes`,
/// `edges` (an `hkArray` of such records), `results` (an `hkArray` of 16-byte vectors)). Walks
/// the edges of a convex shape. Does nothing when `edges` already holds an edge with the same
/// two indices in either order (the search ends on a match; the code's `-1` test of the match
/// index cannot hold, and is translated). Otherwise it appends the edge, runs `00dd66d0` on it
/// into a temporary array of 0x20-byte records (starting empty with the capacity word
/// `0x80000000`), and when that records a vertex stores the first record's vector in `results`
/// and, for every record, recurses on two new edges built from the record's plane indices
/// (words `+0x10`, `+0x14`, `+0x18`: the first edge is `(w4, w6)` with the direction `00dd62e0`
/// makes from `(w4, w6, w5)`, the second `(w5, w6)` with the direction from `(w5, w6, w4)`),
/// each pointing at the record's vertex. The temporary array's storage is released at the end.
pub fn fn_00dd6a10(e: &mut Engine, edge: u32, planes: u32, edges: u32, results: u32) {
    let count = e.mem.i32(edges + 4);
    if count > 0 {
        let first = e.mem.u32(edge);
        let mut record = e.mem.u32(edges);
        let mut index = 0i32;
        loop {
            let a = e.mem.u32(record);
            let b = e.mem.u32(record + 4);
            let matched = (a == first && b == e.mem.u32(edge + 4))
                || (b == e.mem.u32(edge) && a == e.mem.u32(edge + 4));
            if matched && index != -1 {
                return;
            }
            if matched {
                break;
            }
            index += 1;
            record = record.wrapping_add(0x20);
            if index >= e.mem.i32(edges + 4) {
                break;
            }
        }
    }
    push_record(e, edges, edge, 8);
    // The exe aligns ESP, then `SUB ESP,0x54` and three pushes: a 0x60-byte frame with the
    // result array at +0x14 (00dd6aac), the flag byte at +0xf (00dd6aba), the edge records at
    // +0x20 and +0x40 (00dd6b3b..00dd6b4d, 00dd6b7b..00dd6b8d).
    with_aligned_frame(e, 0x60, |e, frame| {
        let local = frame + 0x14;
        e.mem.set_u32(local, 0);
        e.mem.set_u32(local + 4, 0);
        e.mem.set_u32(local + 8, 0x8000_0000);
        let flag = frame + 0xf;
        let i1 = e.mem.i32(edge);
        let i2 = e.mem.i32(edge + 4);
        let point = e.mem.u32(edge + 8);
        let status = fn_00dd66d0(e, flag, edge + 0x10, planes, i1, i2, point, local);
        if e.mem.u8(status) != 0 {
            let first_record = e.mem.u32(local);
            make_room(e, results, 0x10);
            let vector = load_vec4(e, first_record);
            let slot = e
                .mem
                .u32(results + 4)
                .wrapping_mul(0x10)
                .wrapping_add(e.mem.u32(results));
            store_vec4(e, slot, vector);
            let size = e.mem.u32(results + 4);
            e.mem.set_u32(results + 4, size.wrapping_add(1));
            if e.mem.i32(local + 4) > 0 {
                let mut index = 0i32;
                let mut offset = 0u32;
                loop {
                    let record = e.mem.u32(local).wrapping_add(offset);
                    let w4 = e.mem.i32(record + 0x10);
                    let w5 = e.mem.i32(record + 0x14);
                    let w6 = e.mem.i32(record + 0x18);
                    let first_edge = frame + 0x20;
                    e.mem.set_i32(first_edge, w4);
                    e.mem.set_i32(first_edge + 4, w6);
                    e.mem.set_u32(first_edge + 8, record);
                    fn_00dd62e0(e, planes, w4, w6, w5, first_edge + 0x10);
                    fn_00dd6a10(e, first_edge, planes, edges, results);
                    let record = e.mem.u32(local).wrapping_add(offset);
                    let w4 = e.mem.i32(record + 0x10);
                    let w5 = e.mem.i32(record + 0x14);
                    let w6 = e.mem.i32(record + 0x18);
                    let second_edge = frame + 0x40;
                    e.mem.set_i32(second_edge, w5);
                    e.mem.set_i32(second_edge + 4, w6);
                    e.mem.set_u32(second_edge + 8, record);
                    fn_00dd62e0(e, planes, w5, w6, w4, second_edge + 0x10);
                    fn_00dd6a10(e, second_edge, planes, edges, results);
                    index += 1;
                    offset = offset.wrapping_add(0x20);
                    if index >= e.mem.i32(local + 4) {
                        break;
                    }
                }
            }
        }
        release_storage(e, local, 5);
    });
}

// Translated from 00dd6c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `planes`, `out_i`, `out_j` (pointers to words), `out_a`,
/// `out_b` (pointers to 16-byte vectors for `00dd6510`)). Looks for an edge of the shape: it
/// keeps an array of the pairs of plane indices it has tried (an `hkInplaceArray` of 32
/// 8-byte pairs on the stack). Each round picks, over all pairs `i < j` of planes, the one with
/// the largest dot product of the two normals (lanes 0 to 2 summed in the code's order) that
/// is above the best so far (starting at the smallest `float`, `0104e148`), below the `double`
/// at `0106b9e8` (0.9) and not in the tried list (x87 compares; a NaN never qualifies); the pair
/// is stored at `out_i` and `out_j`. When there is none the answer is 1. Otherwise the pair is
/// added to the tried list, `cross = rot_d2(plane[j]) * rot_c9(plane[i]) - rot_c9(plane[j]) *
/// rot_d2(plane[i])` is formed and `00dd6510(cross, planes, i, j, out_b, out_a)` run (answering
/// 1 ends with 1); when `(out_b - out_a) . cross` (lanes 0 to 2 summed) is above the `float` at
/// `010c8440` the answer is 0, otherwise the next round starts. The tried list's storage is
/// released at the end (only freed when it grew onto the heap).
pub fn fn_00dd6c10(
    e: &mut Engine,
    planes: u32,
    out_i: u32,
    out_j: u32,
    out_a: u32,
    out_b: u32,
) -> u32 {
    // The exe aligns ESP, then `SUB ESP,0x154` and three pushes: a 0x160-byte frame with the
    // tried-pairs array at +0x50, its 0x20 inline pairs at +0x5c (00dd6c21..00dd6c32), and the
    // cross product at +0x40 (00dd6dc4).
    with_aligned_frame(e, 0x160, |e, frame| {
        let tried = frame + 0x50;
        e.mem.set_u32(tried, tried + 0xc);
        e.mem.set_u32(tried + 4, 0);
        e.mem.set_u32(tried + 8, 0x8000_0020);
        let cross_at = frame + 0x40;
        let result = 'rounds: loop {
            let mut best_i = 0xffff_ffffu32;
            let mut best_j = 0xffff_ffffu32;
            let mut best = x87_quiet_bits(e.mem.u32(SMALLEST_FLOAT));
            let limit = e.mem.f64(PAIR_DOT_LIMIT);
            let mut count = e.mem.i32(planes + 4);
            if count < 1 {
                break 'rounds 1;
            }
            let mut i = 0i32;
            let mut i_offset = 0u32;
            loop {
                let next = i + 1;
                if next < count {
                    let mut j = next;
                    let mut j_offset = i_offset.wrapping_add(0x10);
                    loop {
                        let data = e.mem.u32(planes);
                        let pj = load_vec4(e, data.wrapping_add(j_offset));
                        let pi = load_vec4(e, data.wrapping_add(i_offset));
                        let dot = sum3(mul4(pi, pj));
                        let dot_loaded = x87_load32(dot.to_bits());
                        let above_best = compare_flags(x87_load32(best), dot_loaded);
                        let below_limit = compare_flags(dot_loaded, limit);
                        if !test_parity_even(above_best, 5) && !test_parity_even(below_limit, 5) {
                            let size = e.mem.i32(tried + 4);
                            let pairs = e.mem.u32(tried);
                            let mut seen = false;
                            if size > 0 {
                                let mut k = 0i32;
                                loop {
                                    let at = pairs.wrapping_add((k as u32).wrapping_mul(8));
                                    if e.mem.u32(at) == i as u32 && e.mem.u32(at + 4) == j as u32 {
                                        // Any match counts: the code's `-1` index test cannot hold.
                                        seen = true;
                                        break;
                                    }
                                    k += 1;
                                    if k >= size {
                                        break;
                                    }
                                }
                            }
                            if !seen {
                                best = x87_store32(dot_loaded);
                                e.mem.set_i32(out_i, i);
                                e.mem.set_i32(out_j, j);
                                best_i = i as u32;
                                best_j = j as u32;
                            }
                        }
                        j += 1;
                        j_offset = j_offset.wrapping_add(0x10);
                        if j >= e.mem.i32(planes + 4) {
                            break;
                        }
                    }
                }
                i_offset = i_offset.wrapping_add(0x10);
                count = e.mem.i32(planes + 4);
                i = next;
                if i >= count {
                    break;
                }
            }
            if best_i == 0xffff_ffff {
                break 'rounds 1;
            }
            make_room(e, tried, 8);
            let slot = e
                .mem
                .u32(tried)
                .wrapping_add(e.mem.u32(tried + 4).wrapping_mul(8));
            if slot != 0 {
                e.mem.set_u32(slot, best_i);
                e.mem.set_u32(slot + 4, best_j);
            }
            let j = e.mem.u32(out_j);
            let i = e.mem.u32(out_i);
            let size = e.mem.u32(tried + 4);
            e.mem.set_u32(tried + 4, size.wrapping_add(1));
            let pj = plane_at(e, planes, j);
            let pi = plane_at(e, planes, i);
            let cross = sub4(
                mul4(rotate_d2(pj), rotate_c9(pi)),
                mul4(rotate_c9(pj), rotate_d2(pi)),
            );
            store_vec4(e, cross_at, cross);
            let status = fn_00dd6510(e, cross_at, planes, i as i32, j as i32, out_b, out_a);
            if status == 1 {
                break 'rounds 1;
            }
            let side = {
                let a = load_vec4(e, out_a);
                let b = load_vec4(e, out_b);
                let cross = load_vec4(e, cross_at);
                sum3(mul4(sub4(b, a), cross))
            };
            let tolerance = x87_load32(e.mem.u32(SIDE_TOLERANCE));
            let flags = compare_flags(tolerance, x87_m32(side.to_bits()));
            if !test_parity_even(flags, 5) {
                break 'rounds 0;
            }
        };
        release_storage(e, tried, 3);
        result
    })
}

// Translated from 00dd6ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`, no stack words). The object holds an array
/// of keys (`+0`), an array of 0x48-byte records (`+4`) and their common count (`+8`). When the
/// keys pointer is not 0 both blocks are returned to the router's second allocator (virtual
/// function `+8`, with `count * 4` and `count * 0x48` bytes); then the three words are cleared.
pub fn fn_00dd6ec0(e: &mut Engine, this: u32) {
    let keys = e.mem.u32(this);
    if keys != 0 {
        let count = e.mem.u32(this + 8);
        let allocator = heap_allocator(e);
        e.vcall(allocator, 8, &args![keys, count.wrapping_mul(4)]);
        let count = e.mem.u32(this + 8);
        let records = e.mem.u32(this + 4);
        let allocator = heap_allocator(e);
        e.vcall(allocator, 8, &args![records, count.wrapping_mul(0x48)]);
    }
    e.mem.set_u32(this, 0);
    e.mem.set_u32(this + 4, 0);
    e.mem.set_u32(this + 8, 0);
}

// Translated from 00dd6f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall` on `this`, no stack words). Sorts the object of
/// `00dd6ec0` (keys at `+0`, 0x48-byte records at `+4`, count at `+8`) by key, doing nothing for
/// a count of 1 or less. It makes an array of pointers to the keys from the router's second
/// allocator (virtual function `+4` takes a size), sorts the pointers with `00dd7d00` (arguments:
/// the pointers, 0, count - 1 and the comparison function `00dd6ea0`), builds new key and record
/// arrays in that order (the records are copied 0x48 bytes at a time from the index the
/// pointer's offset from the old keys gives), frees the pointer array and the old arrays
/// (virtual function `+8`: block, size) and stores the new arrays in the object.
pub fn fn_00dd6f20(e: &mut Engine, this: u32) {
    let count = e.mem.i32(this + 8);
    if count <= 1 {
        return;
    }
    let allocator = heap_allocator(e);
    let pointers = e
        .vcall(allocator, 4, &args![(count as u32).wrapping_mul(4)])
        .u32();
    if e.mem.i32(this + 8) > 0 {
        let mut i = 0u32;
        loop {
            let keys = e.mem.u32(this);
            e.mem
                .set_u32(pointers + 4 * i, keys.wrapping_add(i.wrapping_mul(4)));
            i += 1;
            if i as i32 >= e.mem.i32(this + 8) {
                break;
            }
        }
    }
    let count = e.mem.i32(this + 8);
    if count > 1 {
        e.call(
            QUICK_SORT,
            &args![pointers, 0u32, (count - 1) as u32, KEY_COMPARE],
        );
    }
    let count_for_keys = e.mem.u32(this + 8);
    let allocator = heap_allocator(e);
    let new_keys = e
        .vcall(allocator, 4, &args![count_for_keys.wrapping_mul(4)])
        .u32();
    let count_for_records = e.mem.u32(this + 8);
    let allocator = heap_allocator(e);
    let new_records = e
        .vcall(allocator, 4, &args![count_for_records.wrapping_mul(0x48)])
        .u32();
    if e.mem.i32(this + 8) > 0 {
        let mut j = 0u32;
        loop {
            let key_at = e.mem.u32(pointers + 4 * j);
            let key = e.mem.u32(key_at);
            e.mem.set_u32(new_keys + 4 * j, key);
            let old_keys = e.mem.u32(this);
            let index = (key_at.wrapping_sub(old_keys) as i32) >> 2;
            let from = e
                .mem
                .u32(this + 4)
                .wrapping_add((index as u32).wrapping_mul(0x48));
            let to = new_records.wrapping_add(j.wrapping_mul(0x48));
            for word in 0..0x12 {
                let value = e.mem.u32(from + 4 * word);
                e.mem.set_u32(to + 4 * word, value);
            }
            j += 1;
            if j as i32 >= e.mem.i32(this + 8) {
                break;
            }
        }
    }
    let count = e.mem.u32(this + 8);
    let allocator = heap_allocator(e);
    e.vcall(allocator, 8, &args![pointers, count.wrapping_mul(4)]);
    let count = e.mem.u32(this + 8);
    let old_keys = e.mem.u32(this);
    let allocator = heap_allocator(e);
    e.vcall(allocator, 8, &args![old_keys, count.wrapping_mul(4)]);
    let count = e.mem.u32(this + 8);
    let old_records = e.mem.u32(this + 4);
    let allocator = heap_allocator(e);
    e.vcall(allocator, 8, &args![old_records, count.wrapping_mul(0x48)]);
    e.mem.set_u32(this, new_keys);
    e.mem.set_u32(this + 4, new_records);
}

// Translated from 00dd7090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `other`; both objects as for
/// `00dd6f20`). Sorts both (`00dd6f20`), then walks the two key arrays together and sets to 0
/// every key of `this` that equals a key of `other` (unsigned order: the walk advances `this`
/// when its key is below, `other` when it is above or equal after clearing).
pub fn fn_00dd7090(e: &mut Engine, this: u32, other: u32) {
    fn_00dd6f20(e, this);
    fn_00dd6f20(e, other);
    let mut a = e.mem.u32(this);
    let a_end = a.wrapping_add(e.mem.u32(this + 8).wrapping_mul(4));
    let mut b = e.mem.u32(other);
    let b_end = b.wrapping_add(e.mem.u32(other + 8).wrapping_mul(4));
    if a >= a_end {
        return;
    }
    while b < b_end {
        let x = e.mem.u32(a);
        let y = e.mem.u32(b);
        if x == y {
            e.mem.set_u32(a, 0);
            a = a.wrapping_add(4);
            b = b.wrapping_add(4);
        } else if x < y {
            a = a.wrapping_add(4);
        } else {
            b = b.wrapping_add(4);
        }
        if a >= a_end {
            return;
        }
    }
}

// Translated from 00dd7140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `out` (a pointer to a byte)). Checks
/// the chain of free elements that starts at the word at `this + 0`: each element must be
/// aligned to the word at `this + 0x18` (a power of two), must lie in one of the blocks of the
/// list at `this + 0xc` (each block: next block at `+0`, start at `+8`, element count at `+0xc`;
/// the element size is the word at `this + 4`; the object at `this + 0x30` is asked through its
/// virtual function `+0xc` (a pointer to a local byte, the block) and must answer a pointer to
/// a non-zero byte for each block looked at) at a multiple of the element size from the block's
/// start, and its first word is the next element. `out` receives 1 when the chain ends (a
/// null word) and 0 when a test fails. A zero element size faults in the code's `DIV`; it
/// panics here.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00dd7140(e: &mut Engine, this: u32, out: u32) -> u32 {
    let mut element = e.mem.u32(this);
    loop {
        if element == 0 {
            e.mem.set_u8(out, 1);
            return out;
        }
        let alignment = e.mem.u32(this + 0x18).wrapping_sub(1);
        if element & alignment != 0 {
            break;
        }
        let mut block = e.mem.u32(this + 0xc);
        let start = loop {
            if block == 0 {
                e.mem.set_u8(out, 0);
                return out;
            }
            let checker = e.mem.u32(this + 0x30);
            // The exe's frame (`PUSH ECX` and three pushes): 0x10 bytes, the answer byte at +0xf,
            // the high byte of the pushed `ECX` (00dd7169).
            let valid = e.with_stack(0x10, |e, frame| {
                let answer = e
                    .vcall(checker, 0xc, &args![frame.addr() + 0xf, block])
                    .u32();
                e.mem.u8(answer) != 0
            });
            if !valid {
                e.mem.set_u8(out, 0);
                return out;
            }
            let start = e.mem.u32(block + 8);
            if element >= start {
                let end = e
                    .mem
                    .u32(block + 0xc)
                    .wrapping_mul(e.mem.u32(this + 4))
                    .wrapping_add(start);
                if element < end {
                    break start;
                }
            }
            block = e.mem.u32(block);
        };
        let size = e.mem.u32(this + 4);
        assert!(size != 0, "divide by zero at 00dd71a3 (element size 0)");
        if element.wrapping_sub(start) % size != 0 {
            break;
        }
        element = e.mem.u32(element);
    }
    e.mem.set_u8(out, 0);
    out
}

// Translated from 00dd6ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words `out`, `first`, `second`; descriptions, not
/// confirmed; the code at `00dd6f7c` stores its address): a "less than" test on two words in
/// memory. Stores 1 in the byte at `out` when the word `first` points to is below the word
/// `second` points to (unsigned), else 0, and answers `out`.
pub fn fn_00dd6ea0(e: &mut Engine, out: Ptr, first: Ptr, second: Ptr) -> u32 {
    let first_word = e.mem.u32(first.addr());
    let second_word = e.mem.u32(second.addr());
    e.mem.set_u8(out.addr(), u8::from(first_word < second_word));
    out.addr()
}

// Translated from 00dd70f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words `out`, `first`, `second`; descriptions, not
/// confirmed; code in `walkMemory`, `findGarbage` and others stores its address): a "less than"
/// test on two words. Stores 1 in the byte at `out` when `first` is below `second` (unsigned),
/// else 0, and answers `out`.
pub fn fn_00dd70f0(e: &mut Engine, out: Ptr, first: u32, second: u32) -> u32 {
    e.mem.set_u8(out.addr(), u8::from(first < second));
    out.addr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd6ea0, fn_00dd6ea0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00dd70f0, fn_00dd70f0(Ptr, u32, u32) -> u32),
        entry!(0x00dd62e0, fn_00dd62e0(u32, i32, i32, i32, u32)),
        entry!(0x00dd6380, fn_00dd6380(u32, i32, i32, i32, u32) -> u32),
        entry!(0x00dd6510, fn_00dd6510(u32, u32, i32, i32, u32, u32) -> u32),
        entry!(0x00dd66d0, fn_00dd66d0(u32, u32, u32, i32, i32, u32, u32) -> u32),
        entry!(0x00dd68d0, fn_00dd68d0(f32, u32, u32)),
        entry!(0x00dd6a10, fn_00dd6a10(u32, u32, u32, u32)),
        entry!(0x00dd6c10, fn_00dd6c10(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00dd6ec0, fn_00dd6ec0(u32)),
        entry!(0x00dd6f20, fn_00dd6f20(u32)),
        entry!(0x00dd7090, fn_00dd7090(u32, u32)),
        entry!(0x00dd7140, fn_00dd7140(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkgeometryprocessing::store_vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00dd6ea0_compares_the_words_the_pointers_lead_to() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let low = e.mem.alloc(8);
        let high = e.mem.alloc(8);
        e.mem.set_u32(low, 5);
        e.mem.set_u32(high, 0x8000_0000);
        assert_eq!(e.call(0x00dd_6ea0, &args![out, low, high]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00dd_6ea0, &args![out, high, low]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00dd_6ea0, &args![out, low, low]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00dd70f0_compares_the_words_themselves() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00dd_70f0, &args![out, 5u32, 0x8000_0000u32]).u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00dd_70f0, &args![out, 0x8000_0000u32, 5u32]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00dd_70f0, &args![out, 5u32, 5u32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    const CONTAINER_FREE: u32 = 0x0076_0100;
    const HEAP_ALLOCATE: u32 = 0x0076_1100;
    const HEAP_FREE: u32 = 0x0076_1200;

    /// The engine with the exe constants the functions read and a memory router whose two
    /// allocators and the array and sort functions are doubles that log their calls.
    fn env() -> (Engine, Log) {
        let mut e = Engine::new();
        for page in [
            0x010c_4000,
            0x0101_1000,
            0x0104_d000,
            0x0104_e000,
            0x010c_8000,
            0x010b_9000,
            0x010c_7000,
            0x0109_a000,
            0x0103_2000,
            0x0106_b000,
            0x0101_6000,
        ] {
            e.map(page, 0x1000);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.mem.set_u32(NEGATIVE_ZERO, 0x8000_0000);
        e.mem.set_f32(ZERO_FLOAT, 0.0);
        e.mem.set_f32(LARGEST_FLOAT, f32::MAX);
        e.mem.set_f32(DETERMINANT_TOLERANCE, f32::EPSILON);
        e.mem.set_f32(SMALLEST_FLOAT, -f32::MAX);
        e.mem.set_f32(SIDE_TOLERANCE, -f32::EPSILON);
        e.mem.set_f32(FAR_DISTANCE, -100000.0);
        e.mem.set_f32(FACING_LIMIT, 1e-8);
        e.mem.set_f64(TIE_TOLERANCE, 0.0001);
        e.mem.set_f64(PAIR_DOT_LIMIT, 0.9);
        e.mem.set_f64(WINDOW, 0.01);
        let log: Log = Rc::default();
        e.put_vtable(0x0076_0000, &[0, 0, 0, 0, CONTAINER_FREE]);
        let container = e.mem.alloc(8);
        e.mem.set_u32(container, 0x0076_0000);
        e.put_vtable(0x0076_2000, &[0, HEAP_ALLOCATE, HEAP_FREE]);
        let heap = e.mem.alloc(8);
        e.mem.set_u32(heap, 0x0076_2000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, container);
        e.mem.set_u32(router + 0x14, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(CONTAINER_FREE, move |_, a| {
            seen.borrow_mut().push(("free", vec![a[1], a[2]]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(HEAP_ALLOCATE, move |e, a| {
            seen.borrow_mut().push(("allocate", vec![a[1]]));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(HEAP_FREE, move |_, a| {
            seen.borrow_mut().push(("release", vec![a[1], a[2]]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            // Gives the array room for at least four more elements of `a[2]` bytes.
            seen.borrow_mut().push(("more", vec![a[2]]));
            let (array, element) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let capacity = (size * 2).max(4);
            let data = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(data + byte, value);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |_, a| {
            seen.borrow_mut().push(("reserve", vec![a[2], a[3]]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(QUICK_SORT, move |e, a| {
            seen.borrow_mut().push(("sort", vec![a[1], a[2], a[3]]));
            let mut pointers: Vec<(u32, u32)> = (0..=a[2])
                .map(|i| {
                    let p = e.mem.u32(a[0] + 4 * i);
                    (e.mem.u32(p), p)
                })
                .collect();
            pointers.sort_by_key(|p| p.0);
            for (i, (_, p)) in pointers.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, p);
            }
            Ret::default()
        });
        (e, log)
    }

    /// An `hkArray` header over `data`: size `count`, capacity word `capacity`.
    fn array(e: &mut Engine, data: u32, count: u32, capacity: u32) -> u32 {
        let at = e.mem.alloc(16);
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, count);
        e.mem.set_u32(at + 8, capacity);
        at
    }

    fn plane_array(e: &mut Engine, planes: &[Vec4]) -> u32 {
        let data = e.mem.alloc(16 * planes.len() as u32 + 16);
        for (i, p) in planes.iter().enumerate() {
            store_vec4(e, data + 16 * i as u32, *p);
        }
        array(e, data, planes.len() as u32, planes.len() as u32)
    }

    /// The six planes of the unit cube: `x <= 1`, `x >= 0`, `y <= 1`, `y >= 0`, `z <= 1`,
    /// `z >= 0`, each `(n, w)` with `n . p + w <= 0` inside.
    const CUBE: [Vec4; 6] = [
        [1.0, 0.0, 0.0, -1.0],
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, -1.0],
        [0.0, -1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -1.0],
        [0.0, 0.0, -1.0, 0.0],
    ];

    fn words(e: &Engine, at: u32, n: u32) -> Vec<u32> {
        (0..n).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    #[test]
    fn the_plane_cross_product_is_flipped_when_the_third_plane_is_on_its_side() {
        let (mut e, _) = env();
        let planes = plane_array(
            &mut e,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
            ],
        );
        let out = e.mem.alloc(16);
        // n = (0, 0, 1, 0): the third plane (0, 0, 1) has a positive dot with it.
        e.call(0x00dd_62e0, &args![planes, 0i32, 1i32, 2i32, out]);
        assert_eq!(
            words(&e, out, 4),
            vec![0x8000_0000, 0x8000_0000, 0xbf80_0000, 0x8000_0000]
        );
        // The fourth plane (0, 0, -1) is on the other side: n is kept.
        e.call(0x00dd_62e0, &args![planes, 0i32, 1i32, 3i32, out]);
        assert_eq!(words(&e, out, 4), vec![0, 0, 0x3f80_0000, 0]);
    }

    #[test]
    fn three_planes_meet_in_a_point_and_parallel_ones_do_not() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &CUBE);
        let out = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00dd_6380, &args![planes, 0i32, 2i32, 4i32, out])
                .u32(),
            0
        );
        assert_eq!(load_vec4(&e, out), [1.0, 1.0, 1.0, 0.0]);
        assert_eq!(words(&e, out + 0x10, 3), vec![0, 2, 4]);
        // The first two indices are put in increasing order.
        assert_eq!(
            e.call(0x00dd_6380, &args![planes, 2i32, 0i32, 5i32, out])
                .u32(),
            0
        );
        assert_eq!(load_vec4(&e, out), [1.0, 1.0, 0.0, 0.0]);
        assert_eq!(words(&e, out + 0x10, 3), vec![0, 2, 5]);
        // Planes 0 and 1 are parallel: nothing is written.
        for i in 0..8 {
            e.mem.set_u32(out + 4 * i, 0x55);
        }
        assert_eq!(
            e.call(0x00dd_6380, &args![planes, 0i32, 1i32, 2i32, out])
                .u32(),
            1
        );
        assert_eq!(words(&e, out, 8), vec![0x55; 8]);
    }

    #[test]
    fn the_two_ends_of_an_edge_are_found_along_the_direction() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &CUBE);
        let direction = e.mem.alloc(16);
        store_vec4(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        let out_a = e.mem.alloc(0x20);
        let out_b = e.mem.alloc(0x20);
        let status = e
            .call(
                0x00dd_6510,
                &args![direction, planes, 0i32, 2i32, out_b, out_a],
            )
            .u32();
        assert_eq!(status, 0);
        // The edge x = 1, y = 1: out_a is the end with the smaller z, out_b the other.
        assert_eq!(load_vec4(&e, out_a), [1.0, 1.0, 0.0, 0.0]);
        assert_eq!(words(&e, out_a + 0x10, 3), vec![0, 2, 5]);
        assert_eq!(load_vec4(&e, out_b), [1.0, 1.0, 1.0, 0.0]);
        assert_eq!(words(&e, out_b + 0x10, 3), vec![0, 2, 4]);
    }

    #[test]
    fn an_edge_with_one_end_gives_both_outputs_its_indices_and_no_end_answers_one() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &[CUBE[0], CUBE[2], CUBE[4]]);
        let direction = e.mem.alloc(16);
        store_vec4(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        let out_a = e.mem.alloc(0x20);
        let out_b = e.mem.alloc(0x20);
        let status = e
            .call(
                0x00dd_6510,
                &args![direction, planes, 0i32, 1i32, out_b, out_a],
            )
            .u32();
        assert_eq!(status, 0);
        assert_eq!(load_vec4(&e, out_a), [1.0, 1.0, -99999.0, 0.0]);
        assert_eq!(load_vec4(&e, out_b), [1.0, 1.0, 1.0, 0.0]);
        assert_eq!(words(&e, out_a + 0x10, 3), vec![0, 1, 2]);
        assert_eq!(words(&e, out_b + 0x10, 3), vec![0, 1, 2]);
        // With only the two planes of the edge there is no third plane to end it.
        let planes = plane_array(&mut e, &[CUBE[0], CUBE[2]]);
        let status = e
            .call(
                0x00dd_6510,
                &args![direction, planes, 0i32, 1i32, out_b, out_a],
            )
            .u32();
        assert_eq!(status, 1);
        assert_eq!(words(&e, out_a + 0x10, 3), vec![0xffff_ffff; 3]);
        assert_eq!(words(&e, out_b + 0x10, 3), vec![0xffff_ffff; 3]);
    }

    fn record_array(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(8 * 0x20);
        array(e, data, 0, 8)
    }

    #[test]
    fn the_nearest_plane_along_the_direction_gives_the_vertex_record() {
        let (mut e, log) = env();
        let planes = plane_array(&mut e, &CUBE);
        let direction = e.mem.alloc(16);
        store_vec4(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(16);
        store_vec4(&mut e, point, [0.5, 0.5, 0.5, 7.0]);
        let flag = e.mem.alloc(8);
        let results = record_array(&mut e);
        let answer = e
            .call(
                0x00dd_66d0,
                &args![flag, direction, planes, 0i32, 2i32, point, results],
            )
            .u32();
        assert_eq!(answer, flag);
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(e.mem.u32(results + 4), 1);
        let record = e.mem.u32(results);
        assert_eq!(load_vec4(&e, record), [1.0, 1.0, 1.0, 0.0]);
        assert_eq!(words(&e, record + 0x10, 3), vec![0, 2, 4]);
        // The point's fourth lane was set to 1.0.
        assert_eq!(e.mem.f32(point + 0xc), 1.0);
        assert!(log.borrow().iter().all(|c| c.0 != "more"));
    }

    #[test]
    fn a_second_plane_at_the_same_distance_adds_a_record_and_a_nearer_one_replaces() {
        let (mut e, _) = env();
        let mut planes = CUBE.to_vec();
        planes.push([0.0, 0.0, 1.0, -1.0]);
        let planes = plane_array(&mut e, &planes);
        let direction = e.mem.alloc(16);
        store_vec4(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(16);
        store_vec4(&mut e, point, [0.5, 0.5, 0.5, 0.0]);
        let flag = e.mem.alloc(8);
        let results = record_array(&mut e);
        e.call(
            0x00dd_66d0,
            &args![flag, direction, planes, 0i32, 2i32, point, results],
        );
        assert_eq!(e.mem.u32(results + 4), 2);
        let first = e.mem.u32(results);
        assert_eq!(words(&e, first + 0x10, 3), vec![0, 2, 4]);
        assert_eq!(words(&e, first + 0x30, 3), vec![0, 2, 6]);
        // A plane at z = 0.75 is nearer (0.25 away): it replaces the records.
        let mut nearer = CUBE.to_vec();
        nearer.push([0.0, 0.0, 1.0, -0.75]);
        let planes = plane_array(&mut e, &nearer);
        e.call(
            0x00dd_66d0,
            &args![flag, direction, planes, 0i32, 2i32, point, results],
        );
        assert_eq!(e.mem.u32(results + 4), 1);
        let record = e.mem.u32(results);
        assert_eq!(load_vec4(&e, record), [1.0, 1.0, 0.75, 0.0]);
        assert_eq!(words(&e, record + 0x10, 3), vec![0, 2, 6]);
    }

    #[test]
    fn the_direction_picks_the_facing_plane_and_without_planes_nothing_is_recorded() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &CUBE);
        let direction = e.mem.alloc(16);
        store_vec4(&mut e, direction, [0.0, 0.0, -1.0, 0.0]);
        let point = e.mem.alloc(16);
        let flag = e.mem.alloc(8);
        e.mem.set_u8(flag, 9);
        let results = record_array(&mut e);
        e.call(
            0x00dd_66d0,
            &args![flag, direction, planes, 0i32, 2i32, point, results],
        );
        // Only the plane z >= 0 faces the direction (0, 0, -1): one record.
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(e.mem.u32(results + 4), 1);
        let planes = plane_array(&mut e, &[CUBE[0], CUBE[2]]);
        let results = record_array(&mut e);
        e.call(
            0x00dd_66d0,
            &args![flag, direction, planes, 0i32, 2i32, point, results],
        );
        assert_eq!(e.mem.u8(flag), 0);
        assert_eq!(e.mem.u32(results + 4), 0);
    }

    fn vectors_array(e: &mut Engine, list: &[Vec4], capacity: u32) -> u32 {
        let data = e
            .mem
            .alloc(16 * list.len().max(capacity as usize).max(1) as u32);
        for (i, v) in list.iter().enumerate() {
            store_vec4(e, data + 16 * i as u32, *v);
        }
        array(e, data, list.len() as u32, capacity)
    }

    #[test]
    fn vertices_within_the_tolerance_of_a_kept_one_are_dropped() {
        let (mut e, log) = env();
        let list = vectors_array(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [0.001, 0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [1.002, 0.0, 0.0, 0.0],
                [5.0, 5.0, 5.0, 0.0],
            ],
            8,
        );
        let count = e.mem.alloc(4);
        e.call(0x00dd_68d0, &args![1e-4f32, list, count]);
        assert_eq!(e.mem.u32(count), 3);
        assert_eq!(e.mem.u32(list + 4), 3);
        let data = e.mem.u32(list);
        assert_eq!(load_vec4(&e, data), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, data + 16), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, data + 32), [5.0, 5.0, 5.0, 0.0]);
        assert!(log.borrow().iter().all(|c| c.0 != "reserve"));
    }

    #[test]
    fn vertices_that_share_a_distant_neighbour_are_all_dropped_with_it() {
        let (mut e, _) = env();
        // 0.001 and 0.002 are both within the tolerance of the kept vertex at 0.
        let list = vectors_array(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [0.001, 0.0, 0.0, 0.0],
                [0.002, 0.0, 0.0, 0.0],
                [9.0, 0.0, 0.0, 0.0],
            ],
            4,
        );
        let count = e.mem.alloc(4);
        e.call(0x00dd_68d0, &args![1e-4f32, list, count]);
        assert_eq!(e.mem.u32(count), 2);
    }

    #[test]
    fn the_array_is_reserved_when_the_kept_count_exceeds_its_capacity() {
        let (mut e, log) = env();
        let list = vectors_array(&mut e, &[[1.0, 2.0, 3.0, 0.0]], 0);
        let count = e.mem.alloc(4);
        e.call(0x00dd_68d0, &args![1e-4f32, list, count]);
        assert_eq!(e.mem.u32(count), 1);
        assert_eq!(e.mem.u32(list + 4), 1);
        assert_eq!(*log.borrow(), vec![("reserve", vec![1, 0x10])]);
        let empty = vectors_array(&mut e, &[], 0);
        e.call(0x00dd_68d0, &args![1e-4f32, empty, count]);
        assert_eq!(e.mem.u32(count), 0);
        assert_eq!(e.mem.u32(empty + 4), 0);
    }

    /// The edge record `(i1, i2, point, 0, direction)`.
    fn edge(e: &mut Engine, i1: u32, i2: u32, point: Vec4, direction: Vec4) -> u32 {
        let at = e.mem.alloc(0x20);
        let vertex = e.mem.alloc(0x20);
        store_vec4(e, vertex, point);
        e.mem.set_u32(at, i1);
        e.mem.set_u32(at + 4, i2);
        e.mem.set_u32(at + 8, vertex);
        store_vec4(e, at + 0x10, direction);
        at
    }

    #[test]
    fn a_known_edge_ends_the_walk_at_once() {
        let (mut e, log) = env();
        let planes = plane_array(&mut e, &CUBE);
        let known = edge(&mut e, 5, 7, [0.0; 4], [0.0; 4]);
        let data = e.mem.alloc(8 * 0x20);
        let edges = array(&mut e, data, 1, 8);
        for word in 0..8 {
            let value = e.mem.u32(known + 4 * word);
            e.mem.set_u32(data + 4 * word, value);
        }
        let results = vectors_array(&mut e, &[], 8);
        let same_reversed = edge(&mut e, 7, 5, [0.0; 4], [0.0; 4]);
        e.call(0x00dd_6a10, &args![same_reversed, planes, edges, results]);
        assert_eq!(e.mem.u32(edges + 4), 1);
        assert_eq!(e.mem.u32(results + 4), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_new_edge_without_a_vertex_is_only_added() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &[CUBE[0], CUBE[2]]);
        let data = e.mem.alloc(8 * 0x20);
        let edges = array(&mut e, data, 0, 8);
        let results = vectors_array(&mut e, &[], 8);
        let new = edge(&mut e, 0, 1, [0.5; 4], [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00dd_6a10, &args![new, planes, edges, results]);
        assert_eq!(e.mem.u32(edges + 4), 1);
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(data + 8), e.mem.u32(new + 8));
        assert_eq!(e.mem.u32(results + 4), 0);
    }

    #[test]
    fn walking_three_planes_records_their_vertex_and_visits_the_other_two_edges() {
        let (mut e, log) = env();
        let planes = plane_array(&mut e, &[CUBE[0], CUBE[2], CUBE[4]]);
        let data = e.mem.alloc(8 * 0x20);
        let edges = array(&mut e, data, 0, 8);
        let results = vectors_array(&mut e, &[], 8);
        let first = edge(&mut e, 0, 1, [0.5, 0.5, 0.5, 0.0], [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00dd_6a10, &args![first, planes, edges, results]);
        // The vertex (1, 1, 1) is stored once.
        assert_eq!(e.mem.u32(results + 4), 1);
        assert_eq!(load_vec4(&e, e.mem.u32(results)), [1.0, 1.0, 1.0, 0.0]);
        // The edges (0, 1), (0, 2) and (1, 2) were visited, in that order.
        assert_eq!(e.mem.u32(edges + 4), 3);
        let pairs: Vec<(u32, u32)> = (0..3)
            .map(|i| (e.mem.u32(data + 0x20 * i), e.mem.u32(data + 0x20 * i + 4)))
            .collect();
        assert_eq!(pairs, vec![(0, 1), (0, 2), (1, 2)]);
        // The temporary record array grew once and was released (4 records of 0x20 bytes).
        let calls = log.borrow().clone();
        let more = calls.iter().filter(|c| c.0 == "more").count();
        let frees: Vec<_> = calls.iter().filter(|c| c.0 == "free").collect();
        assert_eq!(more, 1);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1[1], 0x80);
    }

    #[test]
    fn the_first_edge_of_the_cube_is_found_from_the_planes() {
        let (mut e, log) = env();
        let planes = plane_array(&mut e, &CUBE);
        let out_i = e.mem.alloc(4);
        let out_j = e.mem.alloc(4);
        let out_a = e.mem.alloc(0x20);
        let out_b = e.mem.alloc(0x20);
        let answer = e
            .call(0x00dd_6c10, &args![planes, out_i, out_j, out_a, out_b])
            .u32();
        assert_eq!(answer, 0);
        assert_eq!((e.mem.u32(out_i), e.mem.u32(out_j)), (0, 2));
        // The same edge as `00dd6510` finds for the direction (0, 0, 1): out_a is the lower end.
        assert_eq!(load_vec4(&e, out_a), [1.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, out_b), [1.0, 1.0, 1.0, 0.0]);
        // The stack array never left the stack, so nothing was freed.
        assert!(log.borrow().iter().all(|c| c.0 != "free"));
    }

    #[test]
    fn without_planes_there_is_no_edge() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &[]);
        let out = e.mem.alloc(0x80);
        assert_eq!(
            e.call(
                0x00dd_6c10,
                &args![planes, out, out + 4, out + 0x10, out + 0x30]
            )
            .u32(),
            1
        );
        // Two parallel planes: their pair is tried, but no third plane ends the edge.
        let planes = plane_array(&mut e, &[CUBE[0], CUBE[1]]);
        let status = e
            .call(
                0x00dd_6c10,
                &args![planes, out, out + 4, out + 0x10, out + 0x30],
            )
            .u32();
        assert_eq!(status, 1);
    }

    fn keyed_object(e: &mut Engine, keys: &[u32]) -> u32 {
        let this = e.mem.alloc(0x10);
        let key_data = e.mem.alloc(4 * keys.len() as u32 + 4);
        let records = e.mem.alloc(0x48 * keys.len() as u32 + 8);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(key_data + 4 * i as u32, *key);
            e.mem.set_u32(records + 0x48 * i as u32, 0x1000 + *key);
            e.mem
                .set_u32(records + 0x48 * i as u32 + 0x44, 0x2000 + *key);
        }
        e.mem.set_u32(this, key_data);
        e.mem.set_u32(this + 4, records);
        e.mem.set_u32(this + 8, keys.len() as u32);
        this
    }

    #[test]
    fn the_keyed_object_is_released_and_cleared() {
        let (mut e, log) = env();
        let this = keyed_object(&mut e, &[1, 2, 3]);
        let keys = e.mem.u32(this);
        let records = e.mem.u32(this + 4);
        e.call(0x00dd_6ec0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                ("release", vec![keys, 12]),
                ("release", vec![records, 0xd8])
            ]
        );
        assert_eq!(words(&e, this, 3), vec![0, 0, 0]);
        log.borrow_mut().clear();
        e.call(0x00dd_6ec0, &args![this]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn keys_and_records_are_sorted_together() {
        let (mut e, log) = env();
        let this = keyed_object(&mut e, &[30, 10, 20]);
        let old_keys = e.mem.u32(this);
        let old_records = e.mem.u32(this + 4);
        e.call(0x00dd_6f20, &args![this]);
        let keys = e.mem.u32(this);
        let records = e.mem.u32(this + 4);
        assert_eq!(words(&e, keys, 3), vec![10, 20, 30]);
        // Each record moved with its key (word 0 and the last word of the 0x48 bytes).
        let firsts: Vec<u32> = (0..3).map(|i| e.mem.u32(records + 0x48 * i)).collect();
        assert_eq!(firsts, vec![0x100a, 0x1014, 0x101e]);
        let lasts: Vec<u32> = (0..3)
            .map(|i| e.mem.u32(records + 0x48 * i + 0x44))
            .collect();
        assert_eq!(lasts, vec![0x200a, 0x2014, 0x201e]);
        let calls = log.borrow().clone();
        let names: Vec<&str> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            names,
            vec!["allocate", "sort", "allocate", "allocate", "release", "release", "release"]
        );
        // The sort gets 0, the last index and the comparison function.
        assert_eq!(calls[1].1, vec![0, 2, KEY_COMPARE]);
        assert_eq!(calls[4].1[1], 12);
        assert_eq!(calls[5], ("release", vec![old_keys, 12]));
        assert_eq!(calls[6], ("release", vec![old_records, 0xd8]));
    }

    #[test]
    fn one_key_or_none_is_left_alone() {
        let (mut e, log) = env();
        let this = keyed_object(&mut e, &[5]);
        let keys = e.mem.u32(this);
        e.call(0x00dd_6f20, &args![this]);
        assert_eq!(e.mem.u32(this), keys);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn keys_found_in_the_other_object_are_cleared_after_sorting_both() {
        let (mut e, _) = env();
        let this = keyed_object(&mut e, &[30, 10, 20, 40]);
        let other = keyed_object(&mut e, &[20, 40, 50]);
        e.call(0x00dd_7090, &args![this, other]);
        let keys = e.mem.u32(this);
        assert_eq!(words(&e, keys, 4), vec![10, 0, 30, 0]);
        let other_keys = e.mem.u32(other);
        assert_eq!(words(&e, other_keys, 3), vec![20, 40, 50]);
    }

    /// A free list object: head at `+0`, element size `+4`, block list `+0xc`, alignment `+0x18`,
    /// block checker at `+0x30`; one block of `count` elements of `size` bytes at `start`.
    fn free_list(e: &mut Engine, valid: bool) -> (u32, u32) {
        let this = e.mem.alloc(0x40);
        let block = e.mem.alloc(0x10);
        let start = e.mem.alloc(0x100);
        e.mem.set_u32(block + 8, start);
        e.mem.set_u32(block + 0xc, 8);
        e.mem.set_u32(this + 4, 0x10);
        e.mem.set_u32(this + 0xc, block);
        e.mem.set_u32(this + 0x18, 8);
        e.put_vtable(0x0077_0000, &[0, 0, 0, 0x0077_0100]);
        let checker = e.mem.alloc(8);
        e.mem.set_u32(checker, 0x0077_0000);
        e.mem.set_u32(this + 0x30, checker);
        e.register_double(0x0077_0100, move |e, a| {
            e.mem.set_u8(a[1], u8::from(valid));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        (this, start)
    }

    #[test]
    fn a_chain_of_aligned_elements_in_a_block_is_accepted() {
        let (mut e, _) = env();
        let (this, start) = free_list(&mut e, true);
        // Elements 3, 1 and 5 chained, the last ends the chain.
        e.mem.set_u32(this, start + 0x30);
        e.mem.set_u32(start + 0x30, start + 0x10);
        e.mem.set_u32(start + 0x10, start + 0x50);
        e.mem.set_u32(start + 0x50, 0);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 7);
        e.call(0x00dd_7140, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
        // An empty chain is fine as well.
        e.mem.set_u32(this, 0);
        e.call(0x00dd_7140, &args![this, out]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn fn_00dd7140_answers_its_result_pointer() {
        let (mut e, _) = env();
        let (this, start) = free_list(&mut e, true);
        // Elements 3, 1 and 5 chained, the last ends the chain.
        e.mem.set_u32(this, start + 0x30);
        e.mem.set_u32(start + 0x30, start + 0x10);
        e.mem.set_u32(start + 0x10, start + 0x50);
        e.mem.set_u32(start + 0x50, 0);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 7);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(e.call(0x00dd_7140, &args![this, out]).u32(), args![out][0]);
    }

    #[test]
    fn a_misaligned_unowned_or_rejected_element_fails_the_check() {
        let (mut e, _) = env();
        let (this, start) = free_list(&mut e, true);
        let out = e.mem.alloc(4);
        // Not aligned to the 8 of `+0x18`.
        e.mem.set_u32(this, start + 4);
        e.call(0x00dd_7140, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
        // Outside every block.
        e.mem.set_u32(this, start + 0x200);
        e.mem.set_u8(out, 1);
        e.call(0x00dd_7140, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
        // Inside the block but not a multiple of the element size from its start.
        e.mem.set_u32(this, start + 8);
        e.mem.set_u8(out, 1);
        e.call(0x00dd_7140, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
        // A block the checker rejects.
        let (this, start) = free_list(&mut e, false);
        e.mem.set_u32(this, start + 0x10);
        e.mem.set_u8(out, 1);
        e.call(0x00dd_7140, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// Where the next stack local of `size` bytes goes (the next frame of that size takes it).
    fn marked_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    #[test]
    fn the_edge_ends_use_the_hit_at_0x40_of_the_aligned_frame() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &CUBE);
        let direction = e.mem.alloc(16);
        store_vec4(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        let out_a = e.mem.alloc(0x20);
        let out_b = e.mem.alloc(0x20);
        let raw = marked_frame(&mut e, 0x70);
        e.call(
            0x00dd_6510,
            &args![direction, planes, 0i32, 2i32, out_b, out_a],
        );
        let frame = aligned_frame_at(raw, 0x60);
        // `00dd6380` wrote the last hit's plane indices at +0x50.
        assert_eq!(words(&e, frame + 0x50, 3), vec![0, 2, 5]);
    }

    #[test]
    fn the_nearest_plane_search_uses_the_hit_at_0x40_of_the_aligned_frame() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &CUBE);
        let direction = e.mem.alloc(16);
        store_vec4(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(16);
        store_vec4(&mut e, point, [0.5, 0.5, 0.5, 7.0]);
        let flag = e.mem.alloc(8);
        let results = record_array(&mut e);
        let raw = marked_frame(&mut e, 0x70);
        e.call(
            0x00dd_66d0,
            &args![flag, direction, planes, 0i32, 2i32, point, results],
        );
        let frame = aligned_frame_at(raw, 0x60);
        let record = e.mem.u32(results);
        // The stored record is the frame's hit, its eighth word included.
        assert_eq!(words(&e, record, 8), words(&e, frame + 0x40, 8));
    }

    #[test]
    fn the_edge_walk_keeps_its_array_and_flag_in_the_aligned_frame() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &[CUBE[0], CUBE[2]]);
        let data = e.mem.alloc(8 * 0x20);
        let edges = array(&mut e, data, 0, 8);
        let results = vectors_array(&mut e, &[], 8);
        let new = edge(&mut e, 0, 1, [0.5; 4], [0.0, 0.0, 1.0, 0.0]);
        let raw = marked_frame(&mut e, 0x70);
        e.call(0x00dd_6a10, &args![new, planes, edges, results]);
        let frame = aligned_frame_at(raw, 0x60);
        // The result array at +0x14 (data, size, capacity flag) and the flag byte at +0xf.
        assert_eq!(words(&e, frame + 0x14, 3), vec![0, 0, 0x8000_0000]);
        assert_eq!(e.mem.u8(frame + 0xf), 0);
    }

    #[test]
    fn the_first_edge_search_keeps_its_inline_array_in_the_aligned_frame() {
        let (mut e, _) = env();
        let planes = plane_array(&mut e, &CUBE);
        let out_i = e.mem.alloc(4);
        let out_j = e.mem.alloc(4);
        let out_a = e.mem.alloc(0x20);
        let out_b = e.mem.alloc(0x20);
        let raw = marked_frame(&mut e, 0x170);
        e.call(0x00dd_6c10, &args![planes, out_i, out_j, out_a, out_b]);
        let frame = aligned_frame_at(raw, 0x160);
        // The tried pairs: data pointing at the inline storage at +0x5c, capacity 0x20 not owned.
        assert_eq!(e.mem.u32(frame + 0x50), frame + 0x5c);
        assert_eq!(e.mem.u32(frame + 0x58), 0x8000_0020);
    }

    #[test]
    fn the_free_list_check_passes_the_answer_byte_at_0xf_of_its_frame() {
        let (mut e, _) = env();
        let (this, start) = free_list(&mut e, true);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(0x0077_0100, move |e, a| {
            sink.borrow_mut().push(a[1]);
            e.mem.set_u8(a[1], 1);
            a[1].into_ret()
        });
        e.mem.set_u32(this, start + 0x10);
        e.mem.set_u32(start + 0x10, 0);
        let out = e.mem.alloc(4);
        let raw = e.next_stack_local(0x10);
        e.call(0x00dd_7140, &args![this, out]);
        assert_eq!(*seen.borrow(), vec![raw + 0xf]);
        assert_eq!(e.mem.u8(out), 1);
    }
}
