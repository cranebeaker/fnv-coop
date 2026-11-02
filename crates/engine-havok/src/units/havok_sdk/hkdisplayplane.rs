//! `Havok SDK/hkdisplayplane.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplayPlane` (Xbox PDB) keeps four 16-byte vectors at `+0x60`, `+0x70`, `+0x80` and
//! `+0x90` and the shape kind word `+0x50` is 7. Its vtable (`0x010d828c`) holds this unit's
//! `getWireframeGeometry` (slot 3) and `buildGeometry` (slot 4). From the vectors the code
//! forms four corner points: `createPlanePoints` writes them from the vector at `+0x70` plus
//! and minus `D * (A x C)` and plus and minus `D * C`, with `A`, `C` and `D` the vectors at
//! `+0x60`, `+0x80` and `+0x90`. The shared base layout is described in `hkdisplaygeometry`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdisplaygeometry::{allocate_geometry_block, push_back_slot, reset_transform};
use super::hkpsphereshape::{copy_vec, load_vec, shufps, store_vec, Vec4};

/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The vtable the constructors store at `+0`.
const PLANE_VTABLE: u32 = 0x010d_828c;

// Translated from 00de4e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayPlane::createPlanePoints` (Xbox PDB), `this` in `ECX`, one stack word: writes four
/// 16-byte corner points at `out`. With `A`, `B`, `C` and `D` the vectors at `+0x60`, `+0x70`,
/// `+0x80` and `+0x90`, `X = C_yzx * A_zxy - C_zxy * A_yzx` (the cross product `A x C` on the
/// first three lanes), `U = D * X` and `V = D * C` (lane by lane), the points are
/// `(B + U) + V`, `(B + U) - V`, `(B - U) + V` and `(B - U) - V`. Returns `out`.
pub fn hk_display_plane_create_plane_points(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let me = this.addr();
    let a = load_vec(e, me + 0x60);
    let b = load_vec(e, me + 0x70);
    let c = load_vec(e, me + 0x80);
    let d = load_vec(e, me + 0x90);
    use crate::sse::{addps, mulps, subps};
    // `MULPS`/`SUBPS`/`ADDPS` with the SSE NaN rules, the destination first.
    let cross = subps(
        mulps(shufps(c, c, 0xd2), shufps(a, a, 0xc9)),
        mulps(shufps(c, c, 0xc9), shufps(a, a, 0xd2)),
    );
    let across = mulps(d, cross);
    let along = mulps(d, c);
    let corners: [Vec4; 4] = [
        addps(addps(b, across), along),
        subps(addps(b, across), along),
        addps(subps(b, across), along),
        subps(subps(b, across), along),
    ];
    for (index, corner) in corners.into_iter().enumerate() {
        store_vec(e, out.addr() + 0x10 * index as u32, corner);
    }
    out
}

// Translated from 00de4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map. Copies the four vectors the stack words point to into the
/// shape: the first to `+0x60`, the third to `+0x70`, the second to `+0x80` and the fourth to
/// `+0x90` (16-byte copies). The exe leaves the pointer of the second word in `EAX`; that is
/// not a result.
pub fn fn_00de4ee0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr, fourth: Ptr) {
    let me = this.addr();
    copy_vec(e, first.addr(), me + 0x60);
    copy_vec(e, third.addr(), me + 0x70);
    copy_vec(e, second.addr(), me + 0x80);
    copy_vec(e, fourth.addr(), me + 0x90);
}

// Translated from 00de4f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map: a constructor of the class whose vtable (`0x010d828c`) holds
/// the map's `hkDisplayPlane::getWireframeGeometry` and `buildGeometry`. Clears the geometry
/// pointer `+8`, sets the kind word `+0x50` to 7 and the 16-bit count `+6` to 1, writes the
/// identity transform (`reset_transform`), the vtable, and copies the four vectors the stack
/// words point to like `00de4ee0` (first to `+0x60`, third to `+0x70`, second to `+0x80`,
/// fourth to `+0x90`). Returns `this`.
pub fn fn_00de4f20(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    fourth: Ptr,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0x50, 7);
    e.mem.set_u16(me + 6, 1);
    reset_transform(e, me);
    e.mem.set_u32(me, PLANE_VTABLE);
    copy_vec(e, first.addr(), me + 0x60);
    copy_vec(e, third.addr(), me + 0x70);
    copy_vec(e, second.addr(), me + 0x80);
    copy_vec(e, fourth.addr(), me + 0x90);
    this
}

// Translated from 00de4ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named by the engine map: the constructor of the same class as `00de4f20` without
/// vectors. Same header fields (`+8` cleared, kind 7, count 1, identity transform, vtable);
/// the four vectors at `+0x60`..`+0x9f` are zeroed. Returns `this`.
pub fn fn_00de4ff0(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0x50, 7);
    e.mem.set_u16(me + 6, 1);
    reset_transform(e, me);
    e.mem.set_u32(me, PLANE_VTABLE);
    for vector in [0x90, 0x60, 0x70, 0x80] {
        store_vec(e, me + vector, [0.0; 4]);
    }
    this
}

// Translated from 00de5050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayPlane::getWireframeGeometry` (Xbox PDB), `this` in `ECX`, two stack words: the
/// output vertex array and the allocator it is reserved with. When the array's capacity
/// (masked with `0x3fffffff`) is below 12 it is reserved to the larger of twice the capacity
/// and 12 (`00c908c0`, element size 0x10); the size becomes 12 and the vertices are the four
/// corner points `P0`..`P3` of `createPlanePoints` as six segments:
/// `P0 P1  P1 P2  P2 P3  P3 P0  P0 P2  P1 P3`. The exe leaves the array's data pointer in
/// `EAX`; that is not a result.
pub fn hk_display_plane_get_wireframe_geometry(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    allocator: u32,
) {
    let array = out.addr();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < 0xc {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = (doubled as i32).max(0xc) as u32;
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, 0x10u32],
        );
    }
    e.with_stack(0x40, |e, corners| {
        e.mem.set_u32(array + 4, 0xc);
        hk_display_plane_create_plane_points(e, this, corners);
        let points: [Vec4; 4] =
            std::array::from_fn(|i| load_vec(e, corners.addr() + 0x10 * i as u32));
        // Segment end points by index into the four corners, in the order they are stored.
        const SEGMENTS: [usize; 12] = [0, 1, 1, 2, 2, 3, 3, 0, 0, 2, 1, 3];
        for (slot, corner) in SEGMENTS.into_iter().enumerate() {
            let data = e.mem.u32(array);
            store_vec(e, data + 0x10 * slot as u32, points[corner]);
        }
    });
}

// Translated from 00de5110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayPlane::buildGeometry` (Xbox PDB), `this` in `ECX`, no stack words. Builds the
/// geometry block (`+8`): the vertex array is reserved to the larger of twice its capacity
/// and 5 when the capacity is below 5 (`00c908c0`, element size 0x10) and its size set to 5;
/// vertex 4 is the vector at `+0x70` and vertices 0 to 3 the corner points
/// (`createPlanePoints`, written straight into the array). Four index records join the apex 4
/// with the corner pairs: `(2, 4, 3)`, `(0, 4, 2)`, `(1, 4, 0)` and `(3, 4, 1)`, each with
/// `-1` as the fourth word. The exe leaves a stray value in `EAX`; that is not a result.
pub fn hk_display_plane_build_geometry(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let block = allocate_geometry_block(e);
    e.mem.set_u32(me + 8, block);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(block + 8) & 0x3fff_ffff;
    if (capacity as i32) < 5 {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = (doubled as i32).max(5) as u32;
        e.call(
            ARRAY_RESERVE,
            &args![allocator, block, new_capacity, 0x10u32],
        );
    }
    e.mem.set_u32(block + 4, 5);
    let vertices = e.mem.u32(e.mem.u32(me + 8));
    copy_vec(e, me + 0x70, vertices + 0x40);
    let vertices = e.mem.u32(e.mem.u32(me + 8));
    hk_display_plane_create_plane_points(e, this, Ptr::new(vertices));
    for record in [[2, 4, 3], [0, 4, 2], [1, 4, 0], [3, 4, 1]] {
        let index_array = e.mem.u32(me + 8).wrapping_add(0xc);
        let slot = push_back_slot(e, index_array);
        for (lane, word) in record.into_iter().enumerate() {
            e.mem.set_u32(slot + 4 * lane as u32, word);
        }
        e.mem.set_u32(slot + 0xc, 0xffff_ffff);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_4e20, hk_display_plane_create_plane_points(Ptr, Ptr) -> Ptr),
        entry!(0x00de_4ee0, fn_00de4ee0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00de_4f20, fn_00de4f20(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00de_4ff0, fn_00de4ff0(Ptr) -> Ptr),
        entry!(
            0x00de_5050,
            hk_display_plane_get_wireframe_geometry(Ptr, Ptr, u32)
        ),
        entry!(0x00de_5110, hk_display_plane_build_geometry(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0041;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The router with an allocator handing out zeroed blocks, and a reserve that moves the
    /// array to a fresh buffer of the requested capacity.
    fn install(e: &mut Engine) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_SLOT_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(ARRAY_RESERVE, |e, a| {
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let fresh = e.mem.alloc(a[2] * a[3]);
            if size > 0 {
                let bytes = e.mem.bytes(old, size * a[3]);
                e.mem.write(fresh, &bytes);
            }
            e.mem.set_u32(a[1], fresh);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    const A: Vec4 = [1.0, 2.0, 3.0, 4.0];
    const B: Vec4 = [10.0, 20.0, 30.0, 40.0];
    const C: Vec4 = [-1.0, 4.0, 2.0, 7.0];
    const D: Vec4 = [2.0, 3.0, 5.0, 6.0];

    /// A plane object with the four vectors stored.
    fn plane(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xa0);
        for (offset, v) in [(0x60, A), (0x70, B), (0x80, C), (0x90, D)] {
            store_vec(e, this + offset, v);
        }
        this
    }

    /// The four corner points worked out with plain arithmetic: `U = D * (A x C)` per lane
    /// (lane 3 is `D3 * (C3 * A3 - C3 * A3)`) and `V = D * C`.
    fn corners() -> [Vec4; 4] {
        let cross = [
            A[1] * C[2] - A[2] * C[1],
            A[2] * C[0] - A[0] * C[2],
            A[0] * C[1] - A[1] * C[0],
            C[3] * A[3] - C[3] * A[3],
        ];
        let u: Vec4 = std::array::from_fn(|i| D[i] * cross[i]);
        let v: Vec4 = std::array::from_fn(|i| D[i] * C[i]);
        [
            std::array::from_fn(|i| B[i] + u[i] + v[i]),
            std::array::from_fn(|i| B[i] + u[i] - v[i]),
            std::array::from_fn(|i| B[i] - u[i] + v[i]),
            std::array::from_fn(|i| B[i] - u[i] - v[i]),
        ]
    }

    #[test]
    fn hk_display_plane_create_plane_points_writes_the_four_corners() {
        let mut e = engine();
        let this = plane(&mut e);
        let out = e.mem.alloc(0x40);
        let got = e.call(0x00de_4e20, &args![this, out]);
        assert_eq!(got.u32(), out);
        for (index, expected) in corners().into_iter().enumerate() {
            assert_eq!(load_vec(&e, out + 0x10 * index as u32), expected);
        }
        // The vectors of the shape are not changed.
        assert_eq!(load_vec(&e, this + 0x70), B);
    }

    #[test]
    fn fn_00de4ee0_copies_the_vectors_to_the_crossed_slots() {
        let mut e = engine();
        let this = e.mem.alloc(0xa0);
        let vectors = [A, B, C, D];
        let pointers: Vec<u32> = vectors
            .iter()
            .map(|v| {
                let p = e.mem.alloc(0x10);
                store_vec(&mut e, p, *v);
                p
            })
            .collect();
        e.call(
            0x00de_4ee0,
            &args![this, pointers[0], pointers[1], pointers[2], pointers[3]],
        );
        assert_eq!(load_vec(&e, this + 0x60), A);
        assert_eq!(load_vec(&e, this + 0x70), C);
        assert_eq!(load_vec(&e, this + 0x80), B);
        assert_eq!(load_vec(&e, this + 0x90), D);
    }

    #[test]
    fn fn_00de4f20_builds_the_header_and_copies_the_vectors() {
        let mut e = engine();
        let this = e.mem.alloc(0xa0);
        for offset in (0..0xa0).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let pointers: Vec<u32> = [A, B, C, D]
            .iter()
            .map(|v| {
                let p = e.mem.alloc(0x10);
                store_vec(&mut e, p, *v);
                p
            })
            .collect();
        let got = e.call(
            0x00de_4f20,
            &args![this, pointers[0], pointers[1], pointers[2], pointers[3]],
        );
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_828c);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 7);
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
        for lane in 0..16u32 {
            let expected = if [0, 5, 10].contains(&lane) { 1.0 } else { 0.0 };
            assert_eq!(e.mem.f32(this + 0x10 + 4 * lane), expected);
        }
        assert_eq!(load_vec(&e, this + 0x60), A);
        assert_eq!(load_vec(&e, this + 0x70), C);
        assert_eq!(load_vec(&e, this + 0x80), B);
        assert_eq!(load_vec(&e, this + 0x90), D);
    }

    #[test]
    fn fn_00de4ff0_builds_the_header_and_zeroes_the_vectors() {
        let mut e = engine();
        let this = e.mem.alloc(0xa0);
        for offset in (0..0xa0).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let got = e.call(0x00de_4ff0, &args![this]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_828c);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 7);
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
        for offset in (0x60..0xa0).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for lane in 0..16u32 {
            let expected = if [0, 5, 10].contains(&lane) { 1.0 } else { 0.0 };
            assert_eq!(e.mem.f32(this + 0x10 + 4 * lane), expected);
        }
    }

    /// The twelve vertices of the wireframe, from the four corners.
    fn wireframe() -> Vec<Vec4> {
        let c = corners();
        [0, 1, 1, 2, 2, 3, 3, 0, 0, 2, 1, 3]
            .into_iter()
            .map(|i| c[i])
            .collect()
    }

    fn vertices(e: &Engine, array: u32) -> Vec<Vec4> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| load_vec(e, data + 0x10 * i))
            .collect()
    }

    #[test]
    fn hk_display_plane_get_wireframe_geometry_reserves_twelve_and_stores_six_segments() {
        let mut e = engine();
        install(&mut e);
        let this = plane(&mut e);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 8, 0x8000_0003);
        e.call(0x00de_5050, &args![this, out, 0x5555u32]);
        // Twice 3 is 6, below 12: the reserve is for 12 elements of 0x10 bytes.
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![0x5555, out, 12, 0x10]]
        );
        assert_eq!(vertices(&e, out), wireframe());
    }

    #[test]
    fn hk_display_plane_get_wireframe_geometry_reserves_double_when_that_is_larger() {
        let mut e = engine();
        install(&mut e);
        let this = plane(&mut e);
        let out = e.mem.alloc(0x10);
        // Capacity 7 < 12: twice it is 14, the larger.
        e.mem.set_u32(out + 8, 7);
        e.call(0x00de_5050, &args![this, out, 0x5555u32]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![0x5555, out, 14, 0x10]]
        );
        assert_eq!(vertices(&e, out), wireframe());
    }

    #[test]
    fn hk_display_plane_get_wireframe_geometry_does_not_reserve_with_room_for_twelve() {
        let mut e = engine();
        install(&mut e);
        let this = plane(&mut e);
        let out = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10 * 12);
        e.mem.set_u32(out, data);
        e.mem.set_u32(out + 4, 99);
        e.mem.set_u32(out + 8, 0x8000_0000 | 12);
        e.call(0x00de_5050, &args![this, out, 0x5555u32]);
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        assert_eq!(e.mem.u32(out), data);
        assert_eq!(vertices(&e, out), wireframe());
    }

    #[test]
    fn hk_display_plane_build_geometry_makes_five_vertices_and_four_records() {
        let mut e = engine();
        install(&mut e);
        let this = plane(&mut e);
        e.call(0x00de_5110, &args![this]);
        let block = e.mem.u32(this + 8);
        let allocations = calls_to(&e, ALLOCATE_SLOT_DOUBLE);
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0][1], 0x18);
        let mut expected: Vec<Vec4> = corners().to_vec();
        expected.push(B);
        assert_eq!(vertices(&e, block), expected);
        let data = e.mem.u32(block + 0xc);
        assert_eq!(e.mem.u32(block + 0x10), 4);
        let records: Vec<[u32; 4]> = (0..4)
            .map(|i| std::array::from_fn(|lane| e.mem.u32(data + 0x10 * i + 4 * lane as u32)))
            .collect();
        assert_eq!(
            records,
            vec![
                [2, 4, 3, 0xffff_ffff],
                [0, 4, 2, 0xffff_ffff],
                [1, 4, 0, 0xffff_ffff],
                [3, 4, 1, 0xffff_ffff],
            ]
        );
        // The vertex array is reserved for five elements of 0x10 bytes; the record array is
        // reserved as it fills: 1, 2 and 4.
        let reserves: Vec<(u32, u32)> = calls_to(&e, ARRAY_RESERVE)
            .iter()
            .map(|c| (c[2], c[3]))
            .collect();
        assert_eq!(reserves, vec![(5, 0x10), (1, 0x10), (2, 0x10), (4, 0x10)]);
        // One router call for the block, one for the vertex reserve, one per record.
        assert_eq!(calls_to(&e, MEMORY_ROUTER).len(), 6);
    }
}
