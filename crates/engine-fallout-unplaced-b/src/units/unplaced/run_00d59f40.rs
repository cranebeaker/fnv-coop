//! `(unplaced)/run_00d59f40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d5a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d5_9870`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d5a4e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d5_9870, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkpConvexVerticesShape::getOriginalVertices` (engine map, `00d59200`; thiscall: the
/// address of an `hkArray` of 16-byte vertices it fills).
const GET_ORIGINAL_VERTICES: u32 = 0x00d5_9200;
/// `hkArrayUtil::_reserve` (engine map, `00c908c0`; cdecl: allocator, array, capacity,
/// element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkMemUtil::memMove` (engine map, `00d2cc70`; cdecl: destination, source, bytes).
const MEM_MOVE: u32 = 0x00d2_cc70;
/// The CRT square root `00ec6040` (argument and result in `ST0`).
const CRT_SQRT: u32 = 0x00ec_6040;

/// The shape's arrays and geometry (offsets as the PC code uses them): the plane-candidate
/// vertex `hkArray` at +0x54 (data, size +0x58, capacity +0x5c), the vertex count used for
/// the local copy (+0x4c) and the geometry at +0x60 (index words +8, per-face vertex counts
/// +0x14, face count +0x18).
const SHAPE_ARRAY: u32 = 0x54;
const SHAPE_VERTEX_COUNT: u32 = 0x4c;
const SHAPE_GEOMETRY: u32 = 0x60;

/// Four lanes of the vector at `addr`.
fn load_vector(e: &Engine, addr: u32) -> crate::sse::Xmm {
    std::array::from_fn(|lane| e.mem.f32(addr.wrapping_add(4 * lane as u32)))
}

/// Copies one 16-byte vector (`MOVAPS`, bits kept).
fn copy_vector(e: &mut Engine, to: u32, from: u32) {
    let bytes = e.mem.bytes(from, 0x10);
    e.mem.write(to, &bytes);
}

/// The exe's inlined three-lane sum of a product: `prod[2] + (prod[1] + prod[0])` (`ADDSS`).
fn sum3(v: crate::sse::Xmm) -> f32 {
    use crate::sse::add;
    add(v[2], add(v[1], v[0]))
}

/// The exe's inlined `hkArray` free of 16-byte elements at `array` through the thread's
/// memory router's allocator (+0x10, vtable +0x10), unless the capacity word is negative.
fn free_vectors(e: &mut Engine, data: u32, capacity: i32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if capacity >= 0 {
        let bytes = (capacity as u32 & 0x3fff_ffff) << 4;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00d59f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called from `00d5a360`): reorders the vertex array at +0x54
/// by the geometry's faces. The array is copied into a local heap array (16 * size bytes
/// from the router's allocator, slot +0xc with the size in/out) and emptied; the original
/// vertices (`getOriginalVertices`) go into a local array on the router's stack area
/// (capacity +0x4c, 128-byte rounded, flagged not to free). For each face (count +0x18)
/// with at least 3 vertices (the index cursor advances only for those), while candidates
/// remain: the face normal `(v2 - v0) × (v1 - v0)` of its first three vertices (SSE);
/// a zero squared length (x87 `FUCOM`) appends the first candidate (left in place);
/// otherwise the normal is scaled by `1 / sqrt` (CRT square root, x87 divide), the
/// candidate with the largest dot product (first of equals; NaN never larger) is appended
/// and removed by moving the last candidate into its place. The candidates left are
/// appended (`00d5a4c0`); then the stack area is restored and both local arrays freed.
pub fn fn_00d59f40(e: &mut Engine, this: Ptr) {
    use crate::sse::{mulps, subps};
    let this = this.addr();
    let target = this + SHAPE_ARRAY;
    e.with_stack(0x50, |e, frame| {
        // The exe's locals: +0 the candidates hkArray (data, size, capacity), +0xc the
        // original vertices (data, size, capacity), +0x18 the saved stack top, +0x1c the
        // allocation size in/out, +0x20 the normal.
        let frame = frame.addr();
        let candidates = frame;
        let originals = frame + 0xc;
        let saved_top = frame + 0x18;
        let size_slot = frame + 0x1c;
        e.mem.set_u32(candidates, 0);
        e.mem.set_u32(candidates + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let count = e.mem.i32(target + 4);
        if count > 0 {
            e.mem.set_u32(size_slot, (count as u32) << 4);
            let data = e.vcall(allocator, 0xc, &args![size_slot]).u32();
            e.mem.set_u32(candidates, data);
            let bytes = e.mem.i32(size_slot);
            e.mem.set_i32(candidates + 8, bytes / 16);
        }
        let count = e.mem.i32(target + 4);
        let source = e.mem.u32(target);
        e.mem.set_i32(candidates + 4, count);
        let data = e.mem.u32(candidates);
        for i in 0..count.max(0) as u32 {
            copy_vector(e, data + 16 * i, source + 16 * i);
        }
        let capacity = e.mem.u32(this + SHAPE_VERTEX_COUNT);
        e.mem.set_u32(this + 0x58, 0);
        e.mem.set_u32(originals + 4, 0);
        e.mem.set_u32(originals + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        let rounded = ((capacity << 4).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(top);
        e.mem.set_u32(router, rounded);
        e.mem.set_u32(originals, top);
        e.mem.set_u32(saved_top, top);
        e.mem.set_u32(originals + 8, capacity | 0x8000_0000);
        e.call(GET_ORIGINAL_VERTICES, &args![this, originals]);

        let start = e.mem.u32(candidates);
        let mut end = start.wrapping_add((e.mem.u32(candidates + 4)) << 4);
        let mut cursor = 0u32;
        let mut face = 0i32;
        let mut exhausted = false;
        let geometry = e.mem.u32(this + SHAPE_GEOMETRY);
        if e.mem.i32(geometry + 0x18) > 0 {
            loop {
                if e.mem.u32(candidates + 4) == 0 {
                    exhausted = true;
                    break;
                }
                let geometry = e.mem.u32(this + SHAPE_GEOMETRY);
                let sizes = e.mem.u32(geometry + 0x14);
                let corners = u32::from(e.mem.u8(sizes.wrapping_add(face as u32)));
                if corners >= 3 {
                    let indices = e.mem.u32(geometry + 8);
                    let index = |e: &Engine, k: u32| {
                        u32::from(e.mem.u16(indices.wrapping_add((cursor + k) * 2)))
                    };
                    let (i0, i1, i2) = (index(e, 0), index(e, 1), index(e, 2));
                    cursor = cursor.wrapping_add(corners);
                    let vertices = e.mem.u32(originals);
                    let p0 = load_vector(e, vertices.wrapping_add(i0 << 4));
                    let p1 = load_vector(e, vertices.wrapping_add(i1 << 4));
                    let p2 = load_vector(e, vertices.wrapping_add(i2 << 4));
                    let e1 = subps(p1, p0);
                    let e2 = subps(p2, p0);
                    let shuffle = |v: crate::sse::Xmm, imm: u8| -> crate::sse::Xmm {
                        std::array::from_fn(|lane| v[usize::from((imm >> (2 * lane)) & 3)])
                    };
                    let normal = subps(
                        mulps(shuffle(e2, 0xd2), shuffle(e1, 0xc9)),
                        mulps(shuffle(e2, 0xc9), shuffle(e1, 0xd2)),
                    );
                    let length2 = sum3(mulps(normal, normal));
                    let data = e.mem.u32(candidates);
                    if f64::from(length2) == 0.0 {
                        let size = e.mem.u32(target + 4);
                        let slot = e.mem.u32(target).wrapping_add(size << 4);
                        copy_vector(e, slot, data);
                        e.mem.set_u32(target + 4, size.wrapping_add(1));
                    } else {
                        let length = e
                            .call(CRT_SQRT, &args![x87_load32(length2.to_bits())])
                            .f64();
                        let length = x87_load32(x87_store32(length));
                        let inverse = f32::from_bits(x87_store32(x87_div(1.0, length)));
                        let unit = mulps([inverse; 4], normal);
                        let total = e.mem.i32(candidates + 4);
                        let mut best = 0u32;
                        let mut best_dot = x87_quiet(sum3(mulps(unit, load_vector(e, data))));
                        let mut k = 1i32;
                        while k < total {
                            let dot = sum3(mulps(unit, load_vector(e, data + 16 * k as u32)));
                            let dot = x87_quiet(dot);
                            if best_dot < dot {
                                best_dot = dot;
                                best = k as u32;
                            }
                            k += 1;
                        }
                        end = end.wrapping_sub(0x10);
                        let size = e.mem.u32(target + 4);
                        let slot = e.mem.u32(target).wrapping_add(size << 4);
                        let chosen = data.wrapping_add(best << 4);
                        copy_vector(e, slot, chosen);
                        e.mem.set_u32(target + 4, size.wrapping_add(1));
                        let remaining = (total - 1) as u32;
                        e.mem.set_u32(candidates + 4, remaining);
                        if remaining != best {
                            copy_vector(e, chosen, end);
                        }
                    }
                }
                face += 1;
                let geometry = e.mem.u32(this + SHAPE_GEOMETRY);
                if face >= e.mem.i32(geometry + 0x18) {
                    break;
                }
            }
        }
        if !exhausted && e.mem.i32(candidates + 4) > 0 {
            let index = e.mem.u32(this + 0x58);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let data = e.mem.u32(candidates);
            let remaining = e.mem.u32(candidates + 4);
            fn_00d5a4c0(
                e,
                Ptr::new(target),
                Ptr::new(allocator),
                index,
                Ptr::new(data),
                remaining,
            );
        }
        let top = e.mem.u32(saved_top);
        if top == e.mem.u32(originals) {
            e.mem.set_u32(originals + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, top);
        let capacity = e.mem.i32(originals + 8);
        e.mem.set_u32(originals + 4, 0);
        let data = e.mem.u32(originals);
        free_vectors(e, data, capacity);
        e.mem.set_u32(originals, 0);
        e.mem.set_u32(originals + 8, 0x8000_0000);
        let capacity = e.mem.i32(candidates + 8);
        let data = e.mem.u32(candidates);
        free_vectors(e, data, capacity);
    });
}

// Translated from 00d5a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (nothing references it): sets the vertex array at +0x54 from
/// `source` (an `hkArray` of 16-byte vectors): when the capacity is below the source's size
/// the old storage is freed (unless flagged) and `16 * size` bytes allocated (allocator
/// slot +0xc, size in/out; capacity = bytes / 16); the size is set and the vectors copied;
/// with a geometry at +0x60 the array is reordered (`00d59f40`).
pub fn fn_00d5a360(e: &mut Engine, this: Ptr, source: Ptr) {
    let this = this.addr();
    let source = source.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.i32(this + 0x5c);
    let wanted = e.mem.i32(source + 4);
    if (capacity & 0x3fff_ffff) < wanted {
        if capacity >= 0 {
            let bytes = ((capacity & 0x3fff_ffff) as u32) << 4;
            let data = e.mem.u32(this + 0x54);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        let bytes = (e.mem.u32(source + 4)) << 4;
        let data = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), bytes);
            let data = e.vcall(allocator, 0xc, &args![size]).u32();
            let granted = e.mem.i32(size.addr());
            e.mem.set_i32(this + 0x5c, granted / 16);
            data
        });
        e.mem.set_u32(this + 0x54, data);
    }
    let count = e.mem.i32(source + 4);
    let data = e.mem.u32(this + 0x54);
    e.mem.set_i32(this + 0x58, count);
    let from = e.mem.u32(source);
    for i in 0..count.max(0) as u32 {
        copy_vector(e, data + 16 * i, from + 16 * i);
    }
    if e.mem.u32(this + SHAPE_GEOMETRY) != 0 {
        fn_00d59f40(e, Ptr::new(this));
    }
}

// Translated from 00d5a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: replaces `removed` 16-byte elements at `index` of the
/// `hkArray` `this` with `inserted` ones from `source`. The new size is
/// `size - removed + inserted`; below it the capacity grows (`hkArrayUtil::_reserve` with
/// `allocator`, to twice the capacity or the new size if larger, element size 0x10); the
/// tail (`size - index - removed` elements) moves (`hkMemUtil::memMove`), the new elements
/// are copied in and the size stored.
pub fn fn_00d5a410(
    e: &mut Engine,
    this: Ptr,
    allocator: Ptr,
    index: u32,
    removed: u32,
    source: Ptr,
    inserted: u32,
) {
    let this = this.addr();
    let size = e.mem.u32(this + 4);
    let new_size = size.wrapping_sub(removed).wrapping_add(inserted);
    let tail = size.wrapping_sub(index).wrapping_sub(removed);
    let capacity = (e.mem.u32(this + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let grown = if (new_size as i32) < doubled {
            doubled
        } else {
            new_size as i32
        };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, this, grown as u32, 0x10u32],
        );
    }
    let data = e.mem.u32(this);
    let to = (index.wrapping_add(inserted) << 4).wrapping_add(data);
    let from = (removed.wrapping_add(index) << 4).wrapping_add(data);
    e.call(MEM_MOVE, &args![to, from, tail << 4]);
    let start = (index << 4).wrapping_add(e.mem.u32(this));
    if (inserted as i32) > 0 {
        for i in 0..inserted {
            copy_vector(e, start + 16 * i, source.addr() + 16 * i);
        }
    }
    e.mem.set_u32(this + 4, new_size);
}

// Translated from 00d5a4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: inserts `count` elements from `source` at `index` without
/// removing any (`00d5a410` with 0 removed).
pub fn fn_00d5a4c0(e: &mut Engine, this: Ptr, allocator: Ptr, index: u32, source: Ptr, count: u32) {
    fn_00d5a410(e, this, allocator, index, 0, source, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_a4e0, deleting_destructor_00d5a4e0(Ptr, u8) -> Ptr),
        entry!(0x00d5_9f40, fn_00d59f40(Ptr)),
        entry!(0x00d5_a360, fn_00d5a360(Ptr, Ptr)),
        entry!(0x00d5_a410, fn_00d5a410(Ptr, Ptr, u32, u32, Ptr, u32)),
        entry!(0x00d5_a4c0, fn_00d5a4c0(Ptr, Ptr, u32, Ptr, u32)),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(e: &Engine, addr: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .cloned()
            .collect()
    }

    fn put(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (lane, x) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *x);
        }
    }

    /// A router (stack top at +0, allocator at +0x10) whose allocator allocates from the
    /// heap (slot +0xc, size in/out) and frees through a double (slot +0x10).
    fn router(e: &mut Engine) -> (u32, u32) {
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(vtable + 0xc, 0x0079_000c);
        e.mem.set_u32(vtable + 0x10, 0x0079_0010);
        e.register(0x0079_000c, |e, a| {
            let bytes = e.mem.u32(a[1]);
            e.mem.alloc(bytes.max(16)).into_ret()
        });
        e.register(0x0079_0010, |_, _| Ret::default());
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x14);
        let stack = e.mem.alloc(0x1000);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        (router, allocator)
    }

    #[test]
    fn insert_moves_the_tail_and_copies_the_new_elements() {
        let mut e = Engine::new();
        e.register(MEM_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2] * a[3]);
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let bytes = e.mem.bytes(old, size * 16);
            e.mem.write(data, &bytes);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(16 * 3);
        for i in 0..3u32 {
            put(&mut e, data + 16 * i, [i as f32; 4]);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 3);
        let source = e.mem.alloc(32);
        put(&mut e, source, [7.0; 4]);
        put(&mut e, source + 16, [8.0; 4]);
        e.call_log = Some(vec![]);
        e.call(0x00d5_a4c0, &args![array, 0x4000u32, 1u32, source, 2u32]);
        // Capacity 3 < 5: grown to max(6, 5) = 6.
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![(ARRAY_RESERVE, vec![0x4000, array, 6, 0x10])]
        );
        let data = e.mem.u32(array);
        let firsts: Vec<f32> = (0..5).map(|i| e.mem.f32(data + 16 * i)).collect();
        assert_eq!(firsts, vec![0.0, 7.0, 8.0, 1.0, 2.0]);
        assert_eq!(e.mem.u32(array + 4), 5);
        // Removing one in place, no growth.
        e.call_log = Some(vec![]);
        e.call(
            0x00d5_a410,
            &args![array, 0x4000u32, 0u32, 1u32, source, 0u32],
        );
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        let firsts: Vec<f32> = (0..4).map(|i| e.mem.f32(data + 16 * i)).collect();
        assert_eq!(firsts, vec![7.0, 8.0, 1.0, 2.0]);
        assert_eq!(e.mem.u32(array + 4), 4);
    }

    #[test]
    fn set_vertices_reallocates_copies_and_reorders_only_with_geometry() {
        let mut e = Engine::new();
        let (_, allocator) = router(&mut e);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + 0x54, 0x6000);
        e.mem.set_u32(this + 0x5c, 1);
        let source = e.mem.alloc(8);
        let vectors = e.mem.alloc(32);
        put(&mut e, vectors, [1.0, 2.0, 3.0, 4.0]);
        put(&mut e, vectors + 16, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_u32(source, vectors);
        e.mem.set_u32(source + 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x00d5_a360, &args![this, source]);
        assert_eq!(
            calls_to(&e, 0x0079_0010),
            vec![(0x0079_0010, vec![allocator, 0x6000, 16])]
        );
        assert_eq!(e.mem.u32(this + 0x5c), 2);
        assert_eq!(e.mem.u32(this + 0x58), 2);
        let data = e.mem.u32(this + 0x54);
        assert_eq!(e.mem.bytes(data, 32), e.mem.bytes(vectors, 32));
        assert!(calls_to(&e, GET_ORIGINAL_VERTICES).is_empty());
    }

    #[test]
    fn reorder_takes_the_best_candidate_per_face() {
        let mut e = Engine::new();
        let (router, _) = router(&mut e);
        let stack_top = e.mem.u32(router);
        // Original vertices: a triangle in the z = 0 plane, normal (0, 0, -1) for
        // (v2 - v0) x (v1 - v0).
        e.register(GET_ORIGINAL_VERTICES, |e, a| {
            let data = e.mem.u32(a[1]);
            put(e, data, [0.0, 0.0, 0.0, 0.0]);
            put(e, data + 16, [1.0, 0.0, 0.0, 0.0]);
            put(e, data + 32, [0.0, 1.0, 0.0, 0.0]);
            e.mem.set_u32(a[1] + 4, 3);
            Ret::default()
        });
        e.register(CRT_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        let geometry = e.mem.alloc(0x1c);
        let indices = e.mem.alloc(16);
        for (i, v) in [0u16, 1, 2, 0, 0, 0].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *v);
        }
        let sizes = e.mem.alloc(4);
        // Face 0: a triangle; face 1: a degenerate "triangle" (all index 0); face 2: two
        // corners (skipped).
        e.mem.write(sizes, &[3, 3, 2]);
        e.mem.set_u32(geometry + 8, indices);
        e.mem.set_u32(geometry + 0x14, sizes);
        e.mem.set_u32(geometry + 0x18, 3);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + 0x4c, 3);
        e.mem.set_u32(this + 0x60, geometry);
        let data = e.mem.alloc(48);
        put(&mut e, data, [0.0, 0.0, 1.0, 10.0]);
        put(&mut e, data + 16, [0.0, 0.0, -2.0, 20.0]);
        put(&mut e, data + 32, [0.0, 0.0, -5.0, 30.0]);
        e.mem.set_u32(this + 0x54, data);
        e.mem.set_u32(this + 0x58, 3);
        e.mem.set_u32(this + 0x5c, 0x8000_0003);
        e.register(MEM_MOVE, |_, a| a[0].into_ret());
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2] * a[3]);
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let bytes = e.mem.bytes(old, size * 16);
            e.mem.write(data, &bytes);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.call(0x00d5_9f40, &args![this]);
        // Face 0's normal is (0, 0, 1): the largest dot is the first candidate (w 10),
        // appended, and the last (w 30) moved into its place. Face 1 is degenerate: the
        // first candidate (now w 30) is appended and stays. Face 2 has two corners. Then
        // the two left (w 30, 20) are appended.
        let data = e.mem.u32(this + 0x54);
        let ws: Vec<f32> = (0..4).map(|i| e.mem.f32(data + 16 * i + 12)).collect();
        assert_eq!(ws, vec![10.0, 30.0, 30.0, 20.0]);
        assert_eq!(e.mem.u32(this + 0x58), 4);
        assert_eq!(e.mem.u32(router), stack_top);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d5a4e0_frees_only_when_bit_zero_is_set() {
        check(0x00d5_a4e0, 0x00d5_9870);
    }
}
