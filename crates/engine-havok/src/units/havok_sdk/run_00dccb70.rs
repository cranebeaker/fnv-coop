//! `Havok SDK/run_00dccb70` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These are members of the mesh shape whose vtable `010d7754` has the RTTI name `hkpMeshShape`
//! (its constructor and destructor are in `run_00dcd1b0`). The Xbox PDB has no entry for that
//! class, so the fields are described by what the code does with them, as offsets from the
//! object the functions work on (for `00dccb70` that is the object at `+0x30` of `this`
//! below, for the others `this` itself, and for `00dccd00` the interface at `+0x10`):
//! - a triangle key holds the subpart index in its top bits and the triangle index inside the
//!   subpart in the rest; the number of top bits is the word at `+0x20` (`+0x30` in the
//!   `00dccb70` view);
//! - the subparts are 0x38-byte records at the pointer at `+0x24` (count at `+0x28`); a record
//!   holds the vertex array (`+0`), the vertex stride (`+4`), the index array (`+0xc`) and its
//!   stride (`+0x14`), a byte at `+0x10` (1 when the indices are `u16`), a mask at `+0x18`
//!   (applied to the triangle index to pick the first vertex in the triangle's winding), the
//!   triangle count (`+0x1c`), a material array (`+0x20`) with its stride (`+0x24`), a table
//!   at `+0x28` with its stride at `+0x2c`, and an offset into the welding array (`+0x34`);
//! - the welding array is `u16`s at the pointer at `+0x30`, with its count at `+0x34`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{add4, load_vec4, store_vec4, sub4, Vec4};

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

/// `hkpTriangleUtil::isDegenerate` (Xbox PDB, `00ceb8d0`, cdecl: a pointer to a result byte, the
/// three vertices, the tolerance as a `float`'s bits); answers the pointer to the result byte.
const IS_DEGENERATE: u32 = 0x00ce_b8d0;
/// The tolerance `isDegenerate` is called with (`011b2e90`).
const DEGENERATE_TOLERANCE: u32 = 0x011b_2e90;
/// `hkVector4::setTransformedPos` (Xbox PDB, `00c7f640`, `thiscall` on the result, `RET 8`: the
/// transform and the vector).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// The exe's `float` zero (`01011d78`), and the largest and smallest `float` (`0104da48`,
/// `0104e148`) a bounding box search starts from.
const ZERO_FLOAT: u32 = 0x0101_1d78;
const LARGEST_FLOAT: u32 = 0x0104_da48;
const SMALLEST_FLOAT: u32 = 0x0104_e148;
/// The vtable of `hkpTriangleShape` (RTTI `.?AVhkpTriangleShape@@`).
const TRIANGLE_SHAPE_VTABLE: u32 = 0x010c_749c;
/// The mesh shape's virtual functions the code calls: `+0x14` takes a triangle key and a
/// 0x200-byte buffer and answers the triangle shape built for it (its vertices at `+0x20`,
/// `+0x30` and `+0x40`); `+0xc` takes a triangle key and answers a key.
const SLOT_GET_TRIANGLE: u32 = 0x14;
const SLOT_NEXT_KEY: u32 = 0x0c;
/// Size of the subpart records.
const SUBPART_SIZE: u32 = 0x38;

/// `key >> (32 - bits)` as the code computes it (`SHR` by `CL`, which masks the count to five
/// bits): the subpart index of a triangle key.
fn subpart_of(key: u32, bits: u32) -> u32 {
    key >> (0x20u32.wrapping_sub(bits) & 31)
}

/// `0xffffffff >> bits` (count masked to five bits) `& key`: the triangle index in its subpart.
fn triangle_in_subpart(key: u32, bits: u32) -> u32 {
    (0xffff_ffffu32 >> (bits & 31)) & key
}

// Translated from 00dccb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `key`, `weld`). Stores the `u16`
/// `weld` (the low half of the word) in the welding array at
/// `[+0x40] + ((triangle index of key) + (the subpart's welding offset at +0x34)) * 2`, with the
/// subpart picked by the top `[+0x30]` bits of `key`, the subparts at the pointer at `+0x34`.
pub fn fn_00dccb70(e: &mut Engine, this: u32, key: u32, weld: u32) {
    let bits = e.mem.u32(this + 0x30);
    let welding = e.mem.u32(this + 0x40);
    let subparts = e.mem.u32(this + 0x34);
    let subpart = subpart_of(key, bits);
    let first = e.mem.u32(
        subparts
            .wrapping_add(subpart.wrapping_mul(SUBPART_SIZE))
            .wrapping_add(0x34),
    );
    let index = triangle_in_subpart(key, bits).wrapping_add(first);
    e.mem
        .set_u16(welding.wrapping_add(index.wrapping_mul(2)), weld as u16);
}

/// The tolerance as the code passes it on the stack: loaded and stored through the x87 stack
/// (`FLD`/`FSTP`), so a signalling NaN comes out quiet.
fn tolerance_bits(e: &Engine) -> u32 {
    x87_quiet_bits(e.mem.u32(DEGENERATE_TOLERANCE))
}

/// `isDegenerate(flag, v0, v1, v2, tolerance)` on the triangle shape at `triangle` (vertices at
/// `+0x20`, `+0x30`, `+0x40`); answers the byte it points at in its answer. `flag` is the answer
/// byte in the caller's frame.
fn triangle_is_degenerate(e: &mut Engine, triangle: u32, flag: u32) -> bool {
    let tolerance = tolerance_bits(e);
    let at = e
        .call(
            IS_DEGENERATE,
            &args![
                flag,
                triangle + 0x20,
                triangle + 0x30,
                triangle + 0x40,
                tolerance
            ],
        )
        .u32();
    e.mem.u8(at) != 0
}

// Translated from 00dccbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on `this`, no stack words). Answers `0xffffffff` when
/// the word at `+0x28` (the subpart count) is zero. Otherwise builds the triangle for key 0
/// (virtual function `+0x14`, with a 0x200-byte buffer) and tests it with `isDegenerate`
/// (`00ceb8d0`, tolerance at `011b2e90`): 0 when it is not degenerate, else the answer of the
/// virtual function at `+0xc` for key 0.
pub fn fn_00dccbc0(e: &mut Engine, this: u32) -> u32 {
    if e.mem.u32(this + 0x28) == 0 {
        return 0xffff_ffff;
    }
    // The exe aligns ESP, then `SUB ESP,0x20c` and one push: a 0x210-byte frame, the triangle
    // buffer at +0x10 (00dccbe2), the answer byte at +0xf (00dccc03).
    let degenerate = with_aligned_frame(e, 0x210, |e, frame| {
        let triangle = e
            .vcall(this, SLOT_GET_TRIANGLE, &args![0u32, frame + 0x10])
            .u32();
        triangle_is_degenerate(e, triangle, frame + 0xf)
    });
    if !degenerate {
        return 0;
    }
    e.vcall(this, SLOT_NEXT_KEY, &args![0u32]).u32()
}

// Translated from 00dccc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `key`). Finds the next triangle key
/// after `key` that is not degenerate: it steps the triangle index (taking the next subpart,
/// from index 0, when the index reaches the subpart's triangle count at `+0x1c`, answering
/// `0xffffffff` when the subpart index reaches `[+0x28]`), builds the triangle for the key
/// (virtual function `+0x14`) and tests it with `isDegenerate`; answers the first key whose
/// triangle is not degenerate. The key's split is `[+0x20]` bits for the subpart.
pub fn fn_00dccc30(e: &mut Engine, this: u32, key: u32) -> u32 {
    // The exe aligns ESP, then `SUB ESP,0x214` and three pushes: a 0x220-byte frame, the triangle
    // buffer at +0x20 (00dccca3), the answer byte at +0x1b (00dcccca).
    with_aligned_frame(e, 0x220, |e, frame| {
        let buffer = frame + 0x20;
        let bits = e.mem.u32(this + 0x20);
        let mut subpart = subpart_of(key, bits);
        let mut triangle = triangle_in_subpart(key, bits);
        let mut offset = subpart.wrapping_mul(SUBPART_SIZE);
        loop {
            let subparts = e.mem.u32(this + 0x24);
            triangle = triangle.wrapping_add(1);
            let count = e.mem.i32(offset.wrapping_add(subparts).wrapping_add(0x1c));
            if triangle as i32 >= count {
                subpart = subpart.wrapping_add(1);
                offset = offset.wrapping_add(SUBPART_SIZE);
                if subpart >= e.mem.u32(this + 0x28) {
                    break 0xffff_ffff;
                }
                triangle = 0;
            }
            let bits = e.mem.u32(this + 0x20);
            let candidate = (subpart << (0x20u32.wrapping_sub(bits) & 31)) | triangle;
            let shape = e
                .vcall(this, SLOT_GET_TRIANGLE, &args![candidate, buffer])
                .u32();
            if !triangle_is_degenerate(e, shape, frame + 0x1b) {
                break candidate;
            }
        }
    })
}

// Translated from 00dccd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `key`), on the interface at `+0x10`
/// of the mesh shape (the object is `this - 0x10`). Asks `00dcd410` (on the object) for the
/// triangle's index in its subpart's table; when that is not `0xffffffff` it answers the word
/// at `[subpart + 0x28] + [subpart + 0x2c] * index` (0 when that address is 0), else 0. The
/// subpart is picked by the top `[object + 0x30]` bits of `key`.
pub fn fn_00dccd00(e: &mut Engine, this: u32, key: u32) -> u32 {
    let object = this.wrapping_sub(0x10);
    let slot = e.call(0x00dc_d410, &args![object, key]).u32();
    if slot != 0xffff_ffff {
        let subparts = e.mem.u32(object + 0x34);
        let bits = e.mem.u32(object + 0x30);
        let subpart = subparts.wrapping_add(subpart_of(key, bits).wrapping_mul(SUBPART_SIZE));
        let stride = e.mem.u32(subpart + 0x2c);
        let table = e.mem.u32(subpart + 0x28);
        let at = stride.wrapping_mul(slot).wrapping_add(table);
        if at != 0 {
            return e.mem.u32(at);
        }
    }
    0
}

/// `MINPS dst, src` on one lane: `dst` when it is below `src`, else `src` (also for a NaN or
/// for equal values).
fn min_lane(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` on one lane: `dst` when it is above `src`, else `src`.
fn max_lane(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

// Translated from 00dccd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `vertex` (`ECX`, three `float`s), `scale` (`EAX`, three `float`s) and `bounds` (`ESI`, two
/// 16-byte vectors, minimum then maximum); then the stack word `transform` (a transform for
/// `hkVector4::setTransformedPos`). Scales the vertex lane by lane (each product through the
/// x87 stack, stored as a `float`), makes a vector of the three products and the exe's zero
/// (`01011d78`) in lane 3, transforms it (`00c7f640`) and merges the result into the bounds:
/// the minimum takes `MINPS` of the minimum and the result, the maximum `MAXPS`.
pub fn fn_00dccd50(e: &mut Engine, vertex: u32, scale: u32, bounds: u32, transform: u32) {
    let zero = e.mem.u32(ZERO_FLOAT);
    let mut lanes = [0u32; 4];
    for (lane, slot) in lanes.iter_mut().take(3).enumerate() {
        let a = e.mem.u32(vertex + 4 * lane as u32);
        let b = e.mem.u32(scale + 4 * lane as u32);
        *slot = x87_store32(x87_mul(x87_load32(a), x87_m32(b)));
    }
    lanes[3] = zero;
    // The exe aligns ESP, then `SUB ESP,0x30`: a 0x30-byte frame, the scaled vertex at +0x10, the
    // transformed one at +0x20 (00dccd88..00dccd9b).
    with_aligned_frame(e, 0x30, |e, frame| {
        let local = frame + 0x10;
        let result = frame + 0x20;
        for (i, bits) in lanes.into_iter().enumerate() {
            e.mem.set_u32(local + 4 * i as u32, bits);
        }
        e.call(SET_TRANSFORMED_POS, &args![result, transform, local]);
        let transformed = load_vec4(e, result);
        let minimum = load_vec4(e, bounds);
        let merged: Vec4 = [0, 1, 2, 3].map(|i| min_lane(minimum[i], transformed[i]));
        store_vec4(e, bounds, merged);
        let maximum = load_vec4(e, bounds + 0x10);
        let merged: Vec4 = [0, 1, 2, 3].map(|i| max_lane(maximum[i], transformed[i]));
        store_vec4(e, bounds + 0x10, merged);
    });
}

/// Reads the three vertex indices of triangle `triangle` of the subpart record at `subpart`
/// (the index array at `+0xc` with the stride at `+0x14`, `u16` indices when the byte at
/// `+0x10` is 1, else `u32`) and answers the addresses of the three vertices
/// (`index * [+4] + [+0]`).
fn triangle_vertices(e: &Engine, subpart: u32, triangle: u32) -> [u32; 3] {
    let stride = e.mem.u32(subpart + 0x14);
    let indices = stride
        .wrapping_mul(triangle)
        .wrapping_add(e.mem.u32(subpart + 0xc));
    let scale = e.mem.u32(subpart + 4);
    let wide = e.mem.u8(subpart + 0x10) != 1;
    let base = e.mem.u32(subpart);
    [0u32, 1, 2].map(|i| {
        let index = if wide {
            e.mem.u32(indices + 4 * i)
        } else {
            u32::from(e.mem.u16(indices + 2 * i))
        };
        index.wrapping_mul(scale).wrapping_add(base)
    })
}

// Translated from 00dccdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `this`, `transform`, `margin`, `out`).
/// Computes the mesh's bounding box into the two vectors at `out`. The minimum starts as
/// (largest `float` `0104da48` in lanes 0 to 2, the exe's zero in lane 3) and the maximum as
/// (smallest `float` `0104e148`, zero); then for every triangle of every subpart (the subparts
/// at the pointer at `+0x34`, their count at `+0x38`; each subpart's triangle count at `+0x1c`)
/// its three vertices are merged in with `00dccd50` (scale `this + 0x20`, bounds `out`,
/// `transform`). Last, `[+0x50] + margin` (x87, stored as a `float`) is subtracted from the
/// minimum and added to the maximum in all four lanes.
pub fn fn_00dccdd0(e: &mut Engine, this: u32, transform: u32, margin: f32, out: u32) {
    let largest = e.mem.u32(LARGEST_FLOAT);
    let zero = e.mem.u32(ZERO_FLOAT);
    let smallest = e.mem.u32(SMALLEST_FLOAT);
    for (i, bits) in [largest, largest, largest, zero].into_iter().enumerate() {
        e.mem.set_u32(out + 4 * i as u32, bits);
    }
    for (i, bits) in [smallest, smallest, smallest, zero].into_iter().enumerate() {
        e.mem.set_u32(out + 0x10 + 4 * i as u32, bits);
    }
    if e.mem.i32(this + 0x38) > 0 {
        let mut subpart_index = 0i32;
        loop {
            let subpart = e
                .mem
                .u32(this + 0x34)
                .wrapping_add((subpart_index as u32).wrapping_mul(SUBPART_SIZE));
            if e.mem.i32(subpart + 0x1c) > 0 {
                let mut triangle = 0i32;
                loop {
                    let vertices = triangle_vertices(e, subpart, triangle as u32);
                    for vertex in vertices {
                        fn_00dccd50(e, vertex, this + 0x20, out, transform);
                    }
                    triangle += 1;
                    if triangle >= e.mem.i32(subpart + 0x1c) {
                        break;
                    }
                }
            }
            subpart_index += 1;
            if subpart_index >= e.mem.i32(this + 0x38) {
                break;
            }
        }
    }
    let radius = e.mem.u32(this + 0x50);
    let expanded = f32::from_bits(x87_store32(x87_add(
        x87_load32(radius),
        x87_m32(margin.to_bits()),
    )));
    let splat = [expanded; 4];
    let minimum = load_vec4(e, out);
    store_vec4(e, out, sub4(minimum, splat));
    let maximum = load_vec4(e, out + 0x10);
    store_vec4(e, out + 0x10, add4(splat, maximum));
}

// Translated from 00dccf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `source`). Copies the 16-byte vector
/// at `source` to `this + 0x20` (the scale `00dccdd0` and `00dccf60` use).
pub fn fn_00dccf40(e: &mut Engine, this: u32, source: u32) {
    for i in 0..4 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(this + 0x20 + 4 * i, word);
    }
}

// Translated from 00dccf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `key`, `result`). Builds the
/// `hkpTriangleShape` for triangle `key` of the mesh in the shape at `result`: the subpart is
/// the top `[+0x20]` bits of `key` (subparts at the pointer at `+0x24`), the three vertex
/// addresses come from its index array (the second vertex is the one picked by `mask &
/// triangle index`, the third the one at the index xor 1; vertices `index * stride + base`).
/// Each vertex is scaled by the three `float`s at `this + 0x10` (x87 products stored as
/// `float`s; the first and the second and third vertices differ in which operand is loaded
/// first, which only matters for NaNs) and written, with the exe's zero in lane 3, at
/// `result + 0x20`, `+0x30` and `+0x40`.
///
/// The shape's other fields: reference count 1 (`+6`), `+8` 0, `+0xc` 3, the vtable
/// (`010c749c`), the `float` at `[this + 0x40]` quieted (`+0x10`), the material (`+0x14`:
/// 0 when `[this + 0x34]` is 0, else the `u16` at `[this + 0x30] + ([subpart + 0x34] + triangle
/// index) * 2`), the byte at `[this + 0x3c]` (`+0x16`), a zero byte (`+0x17`) and a zero vector
/// (`+0x50`). When `result` is 0 the code still stores the three vertices (at addresses 0x20,
/// 0x30, 0x40), which faults; that is translated as it is.
/// Answers the shape (`hkpShapeContainer::getChildShape` returns `const hkpShape*` in the Xbox
/// PDB): `EAX` is `result`, or 0 (`XOR EAX,EAX` at 00dcd123) when `result` is 0, at the RET
/// 00dcd137. hkpShapeCollection::getAabbImpl reads the child through it (`MOV EDX,[EAX]` at
/// 00d21657 after the container's slot +0x14 call at 00d21652).
pub fn fn_00dccf60(e: &mut Engine, this: u32, key: u32, result: u32) -> u32 {
    let bits = e.mem.u32(this + 0x20);
    let triangle = triangle_in_subpart(key, bits);
    let subpart_index = subpart_of(key, bits);
    let subpart = e
        .mem
        .u32(this + 0x24)
        .wrapping_add(subpart_index.wrapping_mul(SUBPART_SIZE));
    let scale = e.mem.u32(subpart + 4);
    let pick = e.mem.u32(subpart + 0x18) & triangle;
    let stride = e.mem.u32(subpart + 0x14);
    let indices = stride
        .wrapping_mul(triangle)
        .wrapping_add(e.mem.u32(subpart + 0xc));
    let base = e.mem.u32(subpart);
    let wide = e.mem.u8(subpart + 0x10) != 1;
    let index_at = |e: &Engine, n: u32| {
        if wide {
            e.mem.u32(indices + 4 * n)
        } else {
            u32::from(e.mem.u16(indices + 2 * n))
        }
    };
    let first = index_at(e, 0);
    let second = index_at(e, pick + 1);
    let third = index_at(e, (pick ^ 1) + 1);
    let v0 = first.wrapping_mul(scale).wrapping_add(base);
    let v1 = second.wrapping_mul(scale).wrapping_add(base);
    let v2 = third
        .wrapping_mul(e.mem.u32(subpart + 4))
        .wrapping_add(e.mem.u32(subpart));
    let zero = e.mem.u32(ZERO_FLOAT);
    let factors = [
        e.mem.u32(this + 0x10),
        e.mem.u32(this + 0x14),
        e.mem.u32(this + 0x18),
    ];
    let mut vectors = [[0u32; 4]; 3];
    for lane in 0..3u32 {
        let factor = factors[lane as usize];
        // The first vertex loads its lane first; the second and third load the factor first
        // for lane 0 and the lane first for lanes 1 and 2 (the order of the exe's `FLD`s).
        let product = |vertex: u32, factor_first: bool| {
            let lane_bits = e.mem.u32(vertex + 4 * lane);
            let value = if factor_first {
                x87_mul(x87_load32(factor), x87_m32(lane_bits))
            } else {
                x87_mul(x87_load32(lane_bits), x87_m32(factor))
            };
            x87_store32(value)
        };
        vectors[0][lane as usize] = product(v0, false);
        vectors[1][lane as usize] = product(v1, lane == 0);
        vectors[2][lane as usize] = product(v2, lane == 0);
    }
    for vector in &mut vectors {
        vector[3] = zero;
    }
    let material = if e.mem.u32(this + 0x34) == 0 {
        0
    } else {
        let first = e.mem.u32(subpart + 0x34).wrapping_add(triangle);
        let welding = e.mem.u32(this + 0x30);
        u32::from(e.mem.u16(welding.wrapping_add(first.wrapping_mul(2))))
    };
    let target = if result == 0 {
        0
    } else {
        let radius = x87_quiet_bits(e.mem.u32(this + 0x40));
        let flag = e.mem.u8(this + 0x3c);
        e.mem.set_u32(result + 0x10, radius);
        e.mem.set_u16(result + 6, 1);
        e.mem.set_u32(result + 8, 0);
        e.mem.set_u32(result + 0xc, 3);
        e.mem.set_u32(result, TRIANGLE_SHAPE_VTABLE);
        e.mem.set_u16(result + 0x14, material as u16);
        e.mem.set_u8(result + 0x16, flag);
        for i in 0..4 {
            e.mem.set_u32(result + 0x50 + 4 * i, 0);
        }
        e.mem.set_u8(result + 0x17, 0);
        result
    };
    for (n, vector) in vectors.into_iter().enumerate() {
        for (lane, bits) in vector.into_iter().enumerate() {
            e.mem.set_u32(
                target.wrapping_add(0x20 + 0x10 * n as u32 + 4 * lane as u32),
                bits,
            );
        }
    }
    target
}

// Translated from 00dccae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b7f70): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00cb0a70` (the `CALL` at 00dccaf0) (thiscall, `RET 4`) on `object + 0x4` with the one stack word `finishing`.
pub fn fn_00dccae0(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00cb_0a70, &args![object.addr() + 4, finishing]);
    }
}

// Translated from 00dccb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b7f74): cdecl with one stack word, `object`. Jumps to `00cb0c00` (the `JMP` at 00dccb17) (`hkStringPtr::~hkStringPtr` in Ghidra, fastcall, no stack word) with `object + 0x4` as `this`, so it answers what that answers.
pub fn fn_00dccb10(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00cb_0c00, &args![object.addr() + 4]).u32()
}

// Translated from 00dccb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b7fb0): cdecl with one stack word (never read); only returns.
pub fn fn_00dccb20(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dccb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011b7ff4): cdecl with one stack word (never read); only returns.
pub fn fn_00dccb40(_e: &mut Engine, _unused_0: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dccb70, fn_00dccb70(u32, u32, u32)),
        entry!(0x00dccbc0, fn_00dccbc0(u32) -> u32),
        entry!(0x00dccc30, fn_00dccc30(u32, u32) -> u32),
        entry!(0x00dccd00, fn_00dccd00(u32, u32) -> u32),
        entry!(0x00dccd50, fn_00dccd50(u32, u32, u32, u32)),
        entry!(0x00dccdd0, fn_00dccdd0(u32, u32, f32, u32)),
        entry!(0x00dccf40, fn_00dccf40(u32, u32)),
        entry!(0x00dccf60, fn_00dccf60(u32, u32, u32) -> u32),
        entry!(0x00dccae0, fn_00dccae0(Ptr, u32)),
        entry!(0x00dccb10, fn_00dccb10(Ptr) -> u32),
        entry!(0x00dccb20, fn_00dccb20(u32)),
        entry!(0x00dccb40, fn_00dccb40(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const GET_TRIANGLE_DOUBLE: u32 = 0x0073_0010;
    const NEXT_KEY_DOUBLE: u32 = 0x0073_0020;

    /// The three constants the bounding code starts from, and the tolerance.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011b_2000, 0x0104_d000, 0x0104_e000, 0x0101_1000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(LARGEST_FLOAT, f32::MAX);
        e.mem.set_f32(SMALLEST_FLOAT, -f32::MAX);
        e.mem.set_f32(ZERO_FLOAT, 0.0);
        e.mem.set_f32(DEGENERATE_TOLERANCE, 1e-7);
        e
    }

    fn put_subpart(e: &mut Engine, at: u32, fields: &[(u32, u32)]) {
        for &(offset, value) in fields {
            e.mem.set_u32(at + offset, value);
        }
    }

    #[test]
    fn a_welding_value_is_stored_at_the_triangles_slot() {
        let mut e = engine();
        let this = e.mem.alloc(0x50);
        let subparts = e.mem.alloc(2 * 0x38);
        let welding = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x30, 2);
        e.mem.set_u32(this + 0x34, subparts);
        e.mem.set_u32(this + 0x40, welding);
        e.mem.set_u32(subparts + 0x38 + 0x34, 5);
        let key = (1u32 << 30) | 3;
        e.call(0x00dc_cb70, &args![this, key, 0xabcd_1234u32]);
        assert_eq!(e.mem.u16(welding + 2 * 8), 0x1234);
        assert_eq!(e.mem.u16(welding + 2 * 3), 0);
    }

    /// A mesh object whose triangle builder answers a shape holding the vertices listed for the
    /// key and whose tolerance test answers `degenerate(key)`.
    fn mesh_with_triangles(
        e: &mut Engine,
        degenerate: fn(u32) -> bool,
    ) -> (u32, Rc<RefCell<Vec<u32>>>) {
        e.put_vtable(
            0x0073_1000,
            &[0, 0, 0, NEXT_KEY_DOUBLE, 0, GET_TRIANGLE_DOUBLE],
        );
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this, 0x0073_1000);
        let keys: Rc<RefCell<Vec<u32>>> = Rc::default();
        let seen = Rc::clone(&keys);
        let shape = e.mem.alloc(0x60);
        e.register_double(GET_TRIANGLE_DOUBLE, move |e, a| {
            seen.borrow_mut().push(a[1]);
            e.mem.set_u32(shape, a[1]);
            Ret {
                eax: shape,
                ..Ret::default()
            }
        });
        e.register_double(IS_DEGENERATE, move |e, a| {
            // The shape's first word is the key the triangle was built for.
            let key = e.mem.u32(a[1] - 0x20);
            e.mem.set_u8(a[0], u8::from(degenerate(key)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(NEXT_KEY_DOUBLE, |_, a| Ret {
            eax: 0x7700 + a[1],
            ..Ret::default()
        });
        (this, keys)
    }

    #[test]
    fn the_first_triangle_is_checked_only_when_there_are_subparts() {
        let mut e = engine();
        let (this, _) = mesh_with_triangles(&mut e, |_| false);
        assert_eq!(e.call(0x00dc_cbc0, &args![this]).u32(), 0xffff_ffff);
        e.mem.set_u32(this + 0x28, 1);
        assert_eq!(e.call(0x00dc_cbc0, &args![this]).u32(), 0);
        let (this, _) = mesh_with_triangles(&mut e, |_| true);
        e.mem.set_u32(this + 0x28, 1);
        assert_eq!(e.call(0x00dc_cbc0, &args![this]).u32(), 0x7700);
    }

    /// Two subparts of 3 and 2 triangles, one bit of subpart index.
    fn two_subparts(e: &mut Engine, this: u32) {
        let subparts = e.mem.alloc(2 * 0x38);
        put_subpart(e, subparts, &[(0x1c, 3)]);
        put_subpart(e, subparts + 0x38, &[(0x1c, 2)]);
        e.mem.set_u32(this + 0x20, 1);
        e.mem.set_u32(this + 0x24, subparts);
        e.mem.set_u32(this + 0x28, 2);
    }

    #[test]
    fn the_next_good_triangle_skips_degenerate_ones_and_changes_subpart() {
        let mut e = engine();
        // Keys 1 and 2 (subpart 0) are degenerate; the next is key 0 of subpart 1.
        let (this, keys) = mesh_with_triangles(&mut e, |key| key == 1 || key == 2);
        two_subparts(&mut e, this);
        assert_eq!(e.call(0x00dc_cc30, &args![this, 0u32]).u32(), 0x8000_0000);
        assert_eq!(*keys.borrow(), vec![1, 2, 0x8000_0000]);
        // Starting in the middle of subpart 1 and running off its end answers -1.
        let (this, keys) = mesh_with_triangles(&mut e, |_| true);
        two_subparts(&mut e, this);
        assert_eq!(
            e.call(0x00dc_cc30, &args![this, 0x8000_0000u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(*keys.borrow(), vec![0x8000_0001]);
    }

    #[test]
    fn a_table_word_is_answered_through_the_interface() {
        let mut e = engine();
        let object = e.mem.alloc(0x50);
        let subparts = e.mem.alloc(2 * 0x38);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x30, 1);
        e.mem.set_u32(object + 0x34, subparts);
        put_subpart(&mut e, subparts + 0x38, &[(0x28, table), (0x2c, 8)]);
        e.mem.set_u32(table + 8 * 2, 0xfeed);
        e.register_double(0x00dc_d410, |_, a| Ret {
            eax: if a[1] == 0x8000_0000 { 2 } else { 0xffff_ffff },
            ..Ret::default()
        });
        assert_eq!(
            e.call(0x00dc_cd00, &args![object + 0x10, 0x8000_0000u32])
                .u32(),
            0xfeed
        );
        assert_eq!(e.call(0x00dc_cd00, &args![object + 0x10, 0u32]).u32(), 0);
        // A null table answers 0.
        e.mem.set_u32(subparts + 0x38 + 0x28, 0);
        e.mem.set_u32(subparts + 0x38 + 0x2c, 0);
        assert_eq!(
            e.call(0x00dc_cd00, &args![object + 0x10, 0x8000_0000u32])
                .u32(),
            0
        );
    }

    /// `setTransformedPos` replaced by the vector shifted by (1, 1, 1, 0).
    fn shifting_transform(e: &mut Engine) {
        e.register(SET_TRANSFORMED_POS, |e, a| {
            let v = load_vec4(e, a[2]);
            store_vec4(e, a[0], [v[0] + 1.0, v[1] + 1.0, v[2] + 1.0, v[3]]);
            Ret::default()
        });
    }

    fn vec3(e: &mut Engine, lanes: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x10);
        for (i, v) in lanes.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, v);
        }
        at
    }

    #[test]
    fn a_vertex_is_scaled_transformed_and_merged_into_the_bounds() {
        let mut e = engine();
        shifting_transform(&mut e);
        let vertex = vec3(&mut e, [1.0, 2.0, 3.0]);
        let scale = vec3(&mut e, [2.0, 2.0, 2.0]);
        let bounds = e.mem.alloc(0x20);
        store_vec4(&mut e, bounds, [5.0, 5.0, 5.0, 5.0]);
        store_vec4(&mut e, bounds + 0x10, [0.0, 0.0, 0.0, 0.0]);
        e.call(0x00dc_cd50, &args![vertex, scale, bounds, 0x1111u32]);
        // The transformed vertex is (3, 5, 7, 0).
        assert_eq!(load_vec4(&e, bounds), [3.0, 5.0, 5.0, 0.0]);
        assert_eq!(load_vec4(&e, bounds + 0x10), [3.0, 5.0, 7.0, 0.0]);
    }

    #[test]
    fn the_bounding_box_covers_every_triangle_and_the_margin() {
        let mut e = engine();
        shifting_transform(&mut e);
        let this = e.mem.alloc(0x60);
        let subparts = e.mem.alloc(0x38);
        let vertices = e.mem.alloc(0x30);
        let triangles = e.mem.alloc(0x10);
        // Three vertices (0, 0, 0), (1, 2, 3), (-1, 0, 5) and one `u16` triangle.
        for (i, v) in [0.0f32, 0.0, 0.0, 1.0, 2.0, 3.0, -1.0, 0.0, 5.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(vertices + 4 * i as u32, v);
        }
        for (i, v) in [0u16, 1, 2].into_iter().enumerate() {
            e.mem.set_u16(triangles + 2 * i as u32, v);
        }
        put_subpart(
            &mut e,
            subparts,
            &[
                (0, vertices),
                (4, 12),
                (0xc, triangles),
                (0x14, 6),
                (0x1c, 1),
            ],
        );
        e.mem.set_u8(subparts + 0x10, 1);
        e.mem.set_u32(this + 0x34, subparts);
        e.mem.set_u32(this + 0x38, 1);
        for i in 0..3 {
            e.mem.set_f32(this + 0x20 + 4 * i, 1.0);
        }
        e.mem.set_f32(this + 0x50, 0.5);
        let out = e.mem.alloc(0x20);
        e.call(0x00dc_cdd0, &args![this, 0x2222u32, 0.25f32, out]);
        // The shifted vertices span (0, 1, 1) to (2, 3, 6); the margin is 0.5 + 0.25.
        assert_eq!(load_vec4(&e, out), [-0.75, 0.25, 0.25, -0.75]);
        assert_eq!(load_vec4(&e, out + 0x10), [2.75, 3.75, 6.75, 0.75]);
    }

    #[test]
    fn a_mesh_without_subparts_leaves_a_box_grown_from_the_start_values() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        e.mem.set_f32(this + 0x50, 1.0);
        let out = e.mem.alloc(0x20);
        e.call(0x00dc_cdd0, &args![this, 0u32, 0.0f32, out]);
        assert_eq!(
            load_vec4(&e, out),
            [f32::MAX - 1.0, f32::MAX - 1.0, f32::MAX - 1.0, -1.0]
        );
        assert_eq!(load_vec4(&e, out + 0x10)[3], 1.0);
    }

    #[test]
    fn the_scale_vector_is_copied() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let source = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        e.call(0x00dc_cf40, &args![this, source]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 0x20 + 4 * i)).collect();
        assert_eq!(words, vec![0x100, 0x101, 0x102, 0x103]);
    }

    /// A mesh with one subpart of `u32` or `u16` triangles for `fn_00dccf60`.
    fn triangle_mesh(e: &mut Engine, wide: bool) -> (u32, u32) {
        let this = e.mem.alloc(0x60);
        let subparts = e.mem.alloc(0x38);
        let vertices = e.mem.alloc(0x40);
        let triangles = e.mem.alloc(0x40);
        for (i, v) in [1.0f32, 1.0, 1.0, 2.0, 2.0, 2.0, 4.0, 4.0, 4.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(vertices + 4 * i as u32, v);
        }
        // Triangle 1 is (2, 0, 1) after a first triangle (0, 1, 2).
        let indices = [0u32, 1, 2, 2, 0, 1];
        for (i, v) in indices.into_iter().enumerate() {
            if wide {
                e.mem.set_u32(triangles + 4 * i as u32, v);
            } else {
                e.mem.set_u16(triangles + 2 * i as u32, v as u16);
            }
        }
        put_subpart(
            e,
            subparts,
            &[
                (0, vertices),
                (4, 12),
                (0xc, triangles),
                (0x14, if wide { 12 } else { 6 }),
                (0x18, 0),
                (0x1c, 2),
                (0x34, 1),
            ],
        );
        e.mem.set_u8(subparts + 0x10, u8::from(!wide));
        e.mem.set_u32(this + 0x20, 1);
        e.mem.set_u32(this + 0x24, subparts);
        for (i, v) in [2.0f32, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(this + 0x10 + 4 * i as u32, v);
        }
        e.mem.set_f32(this + 0x40, 0.5);
        e.mem.set_u8(this + 0x3c, 9);
        (this, subparts)
    }

    fn build_triangle(wide: bool, key: u32, with_welding: bool) -> (Engine, u32) {
        let mut e = engine();
        let (this, _) = triangle_mesh(&mut e, wide);
        if with_welding {
            let welding = e.mem.alloc(0x10);
            e.mem.set_u16(welding + 2 * 2, 0x0bad);
            e.mem.set_u32(this + 0x30, welding);
            e.mem.set_u32(this + 0x34, 3);
        }
        let result = e.mem.alloc(0x60);
        for i in 0..0x60 {
            e.mem.set_u8(result + i, 0xee);
        }
        e.call(0x00dc_cf60, &args![this, key, result]);
        (e, result)
    }

    #[test]
    fn the_child_shape_answers_the_buffer() {
        let mut e = engine();
        let (this, _) = triangle_mesh(&mut e, true);
        let result = e.mem.alloc(0x60);
        // EAX is the buffer at the RET (00dcd137); 00d21657 reads the child through it.
        assert_eq!(
            e.call(0x00dc_cf60, &args![this, 1u32, result]).u32(),
            result
        );
    }

    #[test]
    fn a_triangle_shape_is_built_from_the_subparts_indices() {
        for wide in [true, false] {
            // Key 1 (subpart 0, triangle 1): vertices 2, 0, 1 scaled by (2, 3, 4); the mask is 0
            // so the second vertex is index 1 of the triangle (0) and the third index 2 (1).
            let (e, result) = build_triangle(wide, 1, true);
            assert_eq!(e.mem.u32(result), TRIANGLE_SHAPE_VTABLE);
            assert_eq!(e.mem.u16(result + 6), 1);
            assert_eq!(e.mem.u32(result + 8), 0);
            assert_eq!(e.mem.u32(result + 0xc), 3);
            assert_eq!(e.mem.f32(result + 0x10), 0.5);
            assert_eq!(e.mem.u16(result + 0x14), 0x0bad);
            assert_eq!(e.mem.u8(result + 0x16), 9);
            assert_eq!(e.mem.u8(result + 0x17), 0);
            assert_eq!(load_vec4(&e, result + 0x50), [0.0; 4]);
            assert_eq!(load_vec4(&e, result + 0x20), [8.0, 12.0, 16.0, 0.0]);
            assert_eq!(load_vec4(&e, result + 0x30), [2.0, 3.0, 4.0, 0.0]);
            assert_eq!(load_vec4(&e, result + 0x40), [4.0, 6.0, 8.0, 0.0]);
        }
    }

    #[test]
    fn the_material_is_zero_without_a_welding_array() {
        let (e, result) = build_triangle(false, 1, false);
        assert_eq!(e.mem.u16(result + 0x14), 0);
    }

    #[test]
    fn the_mask_picks_which_vertices_are_second_and_third() {
        let mut e = engine();
        let (this, subparts) = triangle_mesh(&mut e, true);
        e.mem.set_u32(subparts + 0x18, 1);
        let result = e.mem.alloc(0x60);
        e.call(0x00dc_cf60, &args![this, 1u32, result]);
        // Triangle 1 is (2, 0, 1) and the mask 1 & 1 picks entries 2 and 1 of the triangle
        // (indices at +1 + 1 and +1 + 0): the second vertex is index 1, the third index 0.
        assert_eq!(load_vec4(&e, result + 0x20), [8.0, 12.0, 16.0, 0.0]);
        assert_eq!(load_vec4(&e, result + 0x30), [4.0, 6.0, 8.0, 0.0]);
        assert_eq!(load_vec4(&e, result + 0x40), [2.0, 3.0, 4.0, 0.0]);
    }

    #[test]
    fn the_triangle_checks_use_the_buffer_and_answer_byte_of_their_aligned_frames() {
        let mut e = engine();
        let (this, _) = mesh_with_triangles(&mut e, |_| false);
        let flags = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&flags);
        e.register_double(IS_DEGENERATE, move |e, a| {
            sink.borrow_mut().push(a[0]);
            e.mem.set_u8(a[0], 0);
            a[0].into_ret()
        });
        e.mem.set_u32(this + 0x28, 1);
        let raw = e.next_stack_local(0x220);
        e.call(0x00dc_cbc0, &args![this]);
        let frame = aligned_frame_at(raw, 0x210);
        assert_eq!(flags.borrow()[0], frame + 0xf);
        two_subparts(&mut e, this);
        let raw = e.next_stack_local(0x230);
        e.call(0x00dc_cc30, &args![this, 0u32]);
        let frame = aligned_frame_at(raw, 0x220);
        assert_eq!(flags.borrow()[1], frame + 0x1b);
    }

    #[test]
    fn the_vertex_and_its_transform_sit_at_0x10_and_0x20_of_the_aligned_frame() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SET_TRANSFORMED_POS, move |_, a| {
            sink.borrow_mut().push((a[0], a[2]));
            Ret::default()
        });
        let vertex = vec3(&mut e, [1.0, 2.0, 3.0]);
        let scale = vec3(&mut e, [2.0, 2.0, 2.0]);
        let bounds = e.mem.alloc(0x20);
        let raw = e.next_stack_local(0x40);
        e.call(0x00dc_cd50, &args![vertex, scale, bounds, 0x1111u32]);
        let frame = aligned_frame_at(raw, 0x30);
        assert_eq!(*seen.borrow(), vec![(frame + 0x20, frame + 0x10)]);
    }
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    /// The argument words of every call to `address` in the engine's call log (which is taken).
    fn calls_to(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn fn_00dccae0_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00dc_cae0, &args![0u32, 7u32]);
        e.call(0x00dc_cae0, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00cb_0a70), vec![vec![object + 4, 7]]);
    }

    #[test]
    fn fn_00dccb10_hands_the_object_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00cb_0c00, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dc_cb10, &args![object]).u32(), 0x55);
        assert_eq!(calls_to(&mut e, 0x00cb_0c00), vec![vec![object + 4]]);
    }

    #[test]
    fn fn_00dccb20_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dc_cb20, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dccb40_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00dc_cb40, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
