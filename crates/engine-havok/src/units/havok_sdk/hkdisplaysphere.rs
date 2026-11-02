//! `Havok SDK/hkdisplaysphere.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdisplaygeometry::{allocate_geometry_block, reset_transform};
use super::hkpprimitivedrawer::normalised;
use super::hkpsphereshape::{load_vec, store_vec, zip_with};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d58ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00da_2560`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d58ce0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00da_2560, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// `hkDisplaySphere` (the Xbox PDB has no record of the display classes; names and offsets are
// read from the code): the geometry block pointer at `+8`, the 16-bit count at `+6` and the
// transform at `+0x10` are the shared base (`hkdisplaygeometry`); the shape kind word at
// `+0x50` is 1; the sphere's own vector (centre in lanes 0 to 2, radius in lane 3) is at
// `+0x60`, followed by two words the constructor is given at `+0x70` and `+0x74`.

/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The function the exe uses to make the rotation rows at `this` from the quaternion argument
/// (`00cb2d90`, thiscall, one word; unnamed in the engine map).
const ROTATION_FROM_QUATERNION: u32 = 0x00cb_2d90;
/// The identity quaternion (`012683b0`) the constructor makes the first rotation from.
const IDENTITY_QUATERNION: u32 = 0x0126_83b0;
/// The vtable of `hkDisplaySphere`.
const SPHERE_VTABLE: u32 = 0x010e_a630;
/// The `float` constant `-0.0` that `-a` is made from (`-0.0 - a`).
const MINUS_ZERO: u32 = 0x010c_4990;
/// The double `1.0` the depth is counted down with.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The `float` `2.0`: the depth each of the twenty faces is subdivided to.
const START_DEPTH: u32 = 0x0101_62c0;
/// The doubles (about `0.52573` and `0.85065`, both with a zero low word) the icosahedron's
/// two coordinates are the radius times.
const ICOSAHEDRON_SMALL: u32 = 0x010e_a650;
const ICOSAHEDRON_LARGE: u32 = 0x010e_a648;

/// The faces of the icosahedron: three vertex indices each (the fourth word of every index
/// record is `-1`).
const FACES: [[u32; 3]; 20] = [
    [1, 4, 0],
    [4, 9, 0],
    [4, 5, 9],
    [8, 5, 4],
    [1, 8, 4],
    [1, 10, 8],
    [10, 3, 8],
    [8, 3, 5],
    [3, 2, 5],
    [3, 7, 2],
    [3, 10, 7],
    [10, 6, 7],
    [6, 11, 7],
    [6, 0, 11],
    [6, 1, 0],
    [10, 1, 6],
    [11, 0, 9],
    [2, 11, 9],
    [5, 2, 9],
    [11, 2, 7],
];

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes room in the `hkArray` at `array` for `wanted` elements of 16 bytes: when the capacity
/// (without its flag bits) is below `wanted`, `_reserve` gets the larger of `wanted` and twice
/// the capacity (signed compares).
fn reserve_at_least(e: &mut Engine, array: u32, wanted: i32) {
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let new_capacity = wanted.max(capacity.wrapping_add(capacity));
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity as u32, 0x10u32],
        );
    }
}

// Translated from 00e0af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySphere::getWireframeGeometry` (Xbox PDB), thiscall, `RET 8`: does nothing (a bare
/// `RET 8`).
pub fn hk_display_sphere_get_wireframe_geometry(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) {
}

// Translated from 00e0af50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySphere::hkDisplaySphere` (Xbox PDB), thiscall, `RET 0xc` (the vector holding
/// centre and radius, and two words): clears the geometry block pointer `+8`, sets the 16-bit
/// count `+6` and the shape kind `+0x50` to 1, writes the identity transform
/// (`reset_transform`), the vtable, copies the four words of the vector to `+0x60` and keeps the
/// two words at `+0x70` and `+0x74`; the rotation rows are then made from the identity
/// quaternion (`00cb2d90` on `this + 0x10`) and the vector is also the translation row (`+0x40`).
/// Returns `this`.
pub fn hk_display_sphere_constructor(
    e: &mut Engine,
    this: Ptr,
    sphere: Ptr,
    first: u32,
    second: u32,
) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 0x50, 1);
    reset_transform(e, me);
    e.mem.set_u32(me, SPHERE_VTABLE);
    for lane in 0..4 {
        let word = e.mem.u32(sphere.addr() + 4 * lane);
        e.mem.set_u32(me + 0x60 + 4 * lane, word);
    }
    e.mem.set_u32(me + 0x70, first);
    e.mem.set_u32(me + 0x74, second);
    e.call(
        ROTATION_FROM_QUATERNION,
        &args![me + 0x10, IDENTITY_QUATERNION],
    );
    let vector = load_vec(e, sphere.addr());
    store_vec(e, me + 0x40, vector);
    this
}

// Translated from 00e0afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_subdivideRecursive` (engine map name), cdecl, four words the caller removes: the geometry
/// block, a triangle index, the radius (`float`) and the depth (`float`). While the depth is not
/// zero (an ordered compare; a NaN depth never ends) the triangle is split in four: the block's
/// vertex array and triangle array each grow by three; three new vertices are the radius times
/// the normalised sums of the corners `b + a`, `c + b` and `c + a` (the refined `RSQRTSS` and its
/// zero guard); three new triangles `(a, N, N + 2)`, `(N, b, N + 1)` and `(N + 2, N + 1, c)`
/// (`N` the old vertex count) are appended and the triangle itself becomes `(N, N + 1, N + 2)`.
/// The depth minus one (the x87 `FSUB` of the double `1.0`, stored as a `float`) is passed on
/// when the three new triangles are subdivided in turn (their indices are the old triangle
/// count, plus one and plus two) and also becomes the depth of this loop, which then runs on
/// the middle triangle again. (The exe stores each normalised sum twice, the unnormalised
/// one first; the first store is dead.)
pub fn subdivide_recursive(e: &mut Engine, block: u32, triangle: u32, radius: f32, depth: f32) {
    if depth == 0.0 {
        return;
    }
    let offset = triangle << 4;
    let mut depth = depth;
    loop {
        let new_base = e.mem.u32(block + 4);
        let old_triangles = e.mem.u32(block + 0x10);
        reserve_at_least(e, block, (new_base as i32).wrapping_add(3));
        e.mem.set_u32(block + 4, new_base.wrapping_add(3));
        let triangle_count = e.mem.u32(block + 0x10);
        reserve_at_least(e, block + 0xc, (triangle_count as i32).wrapping_add(3));
        let counted = e.mem.u32(block + 0x10).wrapping_add(3);
        e.mem.set_u32(block + 0x10, counted);
        let triangles = e.mem.u32(block + 0xc);
        let vertices = e.mem.u32(block);
        let record = triangles.wrapping_add(offset);
        let (a, b, c) = (
            e.mem.u32(record),
            e.mem.u32(record + 4),
            e.mem.u32(record + 8),
        );
        let corner = |e: &Engine, index: u32| load_vec(e, vertices.wrapping_add(index << 4));
        let (corner_a, corner_b, corner_c) = (corner(e, a), corner(e, b), corner(e, c));
        let midpoint = |e: &Engine, first, second| {
            normalised(e, zip_with(first, second, |x, y| x + y)).map(|lane| radius * lane)
        };
        let vertex = |index: u32| vertices.wrapping_add(new_base.wrapping_add(index) << 4);
        let middle = midpoint(e, corner_b, corner_a);
        store_vec(e, vertex(0), middle);
        let middle = midpoint(e, corner_c, corner_b);
        store_vec(e, vertex(1), middle);
        let next_depth = (f64::from(x87_quiet(depth)) - e.global::<f64>(ONE_DOUBLE)) as f32;
        let middle = midpoint(e, corner_c, corner_a);
        store_vec(e, vertex(2), middle);
        let fresh = triangles.wrapping_add(old_triangles << 4);
        let middle_index = new_base.wrapping_add(2);
        for (slot, words) in [
            (0u32, [a, new_base, middle_index]),
            (0x10, [new_base, b, new_base.wrapping_add(1)]),
            (0x20, [middle_index, new_base.wrapping_add(1), c]),
        ] {
            for (word, value) in words.into_iter().enumerate() {
                e.mem.set_u32(fresh + slot + 4 * word as u32, value);
            }
        }
        e.mem.set_u32(record, new_base);
        e.mem.set_u32(record + 4, new_base.wrapping_add(1));
        e.mem.set_u32(record + 8, middle_index);
        for step in 0..3u32 {
            subdivide_recursive(
                e,
                block,
                old_triangles.wrapping_add(step),
                x87_quiet(radius),
                next_depth,
            );
        }
        depth = next_depth;
        if depth == 0.0 {
            break;
        }
    }
}

// Translated from 00e0b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySphere::buildGeometry` (Xbox PDB), thiscall: makes the geometry block (`+8`, two
/// empty arrays; a null block is not checked) and, from the radius (the float at `+0x6c`, loaded
/// through the x87 stack) times the two icosahedron factors (`a = 0.52573 * r`, `b = 0.85065 *
/// r`, as `float`s), fills the vertex array with the twelve vertices `(-a, 0, b)`, `(a, 0, b)`,
/// `(-a, 0, -b)`, `(a, 0, -b)`, `(0, b, a)`, `(0, b, -a)`, `(0, -b, a)`, `(0, -b, -a)`, `(b, a, 0)`,
/// `(-b, a, 0)`, `(b, -a, 0)` and `(-b, -a, 0)` (fourth lane 0) and the index array with the
/// twenty faces of the icosahedron (fourth word `-1`), then subdivides each face with depth `2.0`
/// (`_subdivideRecursive`). The arrays are reserved to 12 and 20 elements first when smaller.
pub fn hk_display_sphere_build_geometry(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let block = allocate_geometry_block(e);
    e.mem.set_u32(me + 8, block);
    let radius = x87_quiet(e.mem.f32(me + 0x6c));
    let small: f64 = e.global(ICOSAHEDRON_SMALL);
    let large: f64 = e.global(ICOSAHEDRON_LARGE);
    let a = (small * f64::from(radius)) as f32;
    let b = (f64::from(radius) * large) as f32;
    reserve_at_least(e, block, 12);
    let minus_zero: f32 = e.global(MINUS_ZERO);
    let (minus_a, minus_b) = (minus_zero - a, minus_zero - b);
    let vertices = e.mem.u32(block);
    e.mem.set_u32(block + 4, 12);
    let positions: [[f32; 4]; 12] = [
        [minus_a, 0.0, b, 0.0],
        [a, 0.0, b, 0.0],
        [minus_a, 0.0, minus_b, 0.0],
        [a, 0.0, minus_b, 0.0],
        [0.0, b, a, 0.0],
        [0.0, b, minus_a, 0.0],
        [0.0, minus_b, a, 0.0],
        [0.0, minus_b, minus_a, 0.0],
        [b, a, 0.0, 0.0],
        [minus_b, a, 0.0, 0.0],
        [b, minus_a, 0.0, 0.0],
        [minus_b, minus_a, 0.0, 0.0],
    ];
    for (index, position) in positions.into_iter().enumerate() {
        store_vec(e, vertices + 16 * index as u32, position);
    }
    let triangles = e.mem.u32(me + 8) + 0xc;
    reserve_at_least(e, triangles, 20);
    let records = e.mem.u32(triangles);
    e.mem.set_u32(triangles + 4, 20);
    for (index, face) in FACES.into_iter().enumerate() {
        let record = records + 16 * index as u32;
        for (word, value) in face.into_iter().enumerate() {
            e.mem.set_u32(record + 4 * word as u32, value);
        }
        e.mem.set_u32(record + 0xc, 0xffff_ffff);
    }
    for face in 0..20u32 {
        let block = e.mem.u32(me + 8);
        let depth: f32 = x87_quiet(e.global(START_DEPTH));
        subdivide_recursive(e, block, face, radius, depth);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5_8ce0, deleting_destructor_00d58ce0(Ptr, u8) -> Ptr),
        entry!(
            0x00e0_af40,
            hk_display_sphere_get_wireframe_geometry(Ptr, u32, u32)
        ),
        entry!(0x00e0_af50, hk_display_sphere_constructor(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00e0_afe0, subdivide_recursive(u32, u32, f32, f32)),
        entry!(0x00e0_b360, hk_display_sphere_build_geometry(Ptr)),
    ]
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
    fn deleting_destructor_00d58ce0_frees_only_when_bit_zero_is_set() {
        check(0x00d5_8ce0, 0x00da_2560);
    }

    const ALLOCATE_DOUBLE: u32 = 0x0900_0004;

    /// A 16-byte aligned block of `size` bytes.
    fn aligned(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The router (allocator slot 4 hands out aligned blocks), a real `_reserve` that moves
    /// the elements to a new buffer, and the constants of the exe the code reads.
    fn sphere_engine() -> Engine {
        let mut e = engine();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ALLOCATE_DOUBLE, |e, a| Ret {
            eax: aligned(e, a[1]),
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let (array, capacity) = (a[1], a[2]);
            let data = aligned(e, capacity * 16);
            let old = e.mem.u32(array);
            for word in 0..e.mem.u32(array + 4) * 4 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(data + 4 * word, value);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        e.set_global(0x0101_1d78, 0.0f32);
        e.set_global(0x0101_7718, 3.0f32);
        e.set_global(0x0101_6248, 0.5f32);
        e.set_global(MINUS_ZERO, -0.0f32);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(START_DEPTH, 2.0f32);
        e.set_global(ICOSAHEDRON_SMALL, f64::from_bits(0x3fe0_d2ca_0000_0000));
        e.set_global(ICOSAHEDRON_LARGE, f64::from_bits(0x3feb_3888_0000_0000));
        e
    }

    fn vertex(e: &Engine, block: u32, index: u32) -> [f32; 4] {
        let data = e.mem.u32(block);
        std::array::from_fn(|lane| e.mem.f32(data + 16 * index + 4 * lane as u32))
    }

    fn face(e: &Engine, block: u32, index: u32) -> [u32; 3] {
        let data = e.mem.u32(block + 0xc);
        std::array::from_fn(|word| e.mem.u32(data + 16 * index + 4 * word as u32))
    }

    #[test]
    fn get_wireframe_geometry_does_nothing() {
        let mut e = engine();
        e.call(0x00e0_af40, &args![0x1000u32, 1u32, 2u32]);
    }

    #[test]
    fn the_constructor_sets_up_the_sphere() {
        let mut e = engine();
        e.register(ROTATION_FROM_QUATERNION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = aligned(&mut e, 0x80);
        e.mem.write(this, &[0xcd; 0x80]);
        let vector = aligned(&mut e, 0x10);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(vector + 4 * lane as u32, value);
        }
        assert_eq!(
            e.call(0x00e0_af50, &args![this, vector, 7u32, 9u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), SPHERE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0x50), 1);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            assert_eq!(e.mem.f32(this + 0x60 + 4 * lane as u32), value);
            assert_eq!(e.mem.f32(this + 0x40 + 4 * lane as u32), value);
        }
        assert_eq!(e.mem.u32(this + 0x70), 7);
        assert_eq!(e.mem.u32(this + 0x74), 9);
        // The identity transform: diagonal 1.0, other lanes zero (the rotation call is a double).
        assert_eq!(e.mem.f32(this + 0x10), 1.0);
        assert_eq!(e.mem.f32(this + 0x24), 1.0);
        assert_eq!(e.mem.f32(this + 0x38), 1.0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(
            calls_to(&e, ROTATION_FROM_QUATERNION),
            vec![vec![this + 0x10, IDENTITY_QUATERNION]]
        );
    }

    fn close(a: [f32; 4], b: [f32; 4]) {
        for lane in 0..4 {
            assert!((a[lane] - b[lane]).abs() < 1e-5, "{a:?} vs {b:?}");
        }
    }

    /// A block with the three corners of a triangle `(0, 1, 2)` on the axes.
    fn corner_block(e: &mut Engine) -> u32 {
        let block = aligned(e, 0x18);
        let vertices = aligned(e, 0x30);
        let triangles = aligned(e, 0x10);
        for (index, lanes) in [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            for (lane, value) in lanes.into_iter().enumerate() {
                e.mem
                    .set_f32(vertices + 16 * index as u32 + 4 * lane as u32, value);
            }
        }
        for (word, value) in [0u32, 1, 2].into_iter().enumerate() {
            e.mem.set_u32(triangles + 4 * word as u32, value);
        }
        e.mem.set_u32(block, vertices);
        e.mem.set_u32(block + 4, 3);
        e.mem.set_u32(block + 8, 3);
        e.mem.set_u32(block + 0xc, triangles);
        e.mem.set_u32(block + 0x10, 1);
        e.mem.set_u32(block + 0x14, 1);
        block
    }

    #[test]
    fn subdividing_one_level_splits_the_triangle_in_four() {
        let mut e = sphere_engine();
        let block = corner_block(&mut e);
        e.call(0x00e0_afe0, &args![block, 0u32, 2.0f32, 1.0f32]);
        assert_eq!(e.mem.u32(block + 4), 6);
        assert_eq!(e.mem.u32(block + 0x10), 4);
        // The new vertices are the radius times the normalised edge midpoints (b + a, c + b,
        // c + a); the vector parts have length 2.
        let half = 2.0 * std::f32::consts::FRAC_1_SQRT_2;
        close(vertex(&e, block, 3), [half, half, 0.0, 0.0]);
        close(vertex(&e, block, 4), [0.0, half, half, 0.0]);
        close(vertex(&e, block, 5), [half, 0.0, half, 0.0]);
        assert_eq!(face(&e, block, 0), [3, 4, 5]);
        assert_eq!(face(&e, block, 1), [0, 3, 5]);
        assert_eq!(face(&e, block, 2), [3, 1, 4]);
        assert_eq!(face(&e, block, 3), [5, 4, 2]);
        // The original corners are untouched.
        close(vertex(&e, block, 0), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn subdividing_two_levels_makes_sixteen_triangles() {
        let mut e = sphere_engine();
        let block = corner_block(&mut e);
        e.call(0x00e0_afe0, &args![block, 0u32, 1.0f32, 2.0f32]);
        // Five splits (the face, its three corners, the face's middle again).
        assert_eq!(e.mem.u32(block + 4), 3 + 5 * 3);
        assert_eq!(e.mem.u32(block + 0x10), 1 + 5 * 3);
    }

    #[test]
    fn a_zero_depth_does_nothing() {
        let mut e = sphere_engine();
        let block = corner_block(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00e0_afe0, &args![block, 0u32, 1.0f32, 0.0f32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(block + 4), 3);
        assert_eq!(e.mem.u32(block + 0x10), 1);
    }

    #[test]
    fn build_geometry_makes_and_subdivides_the_icosahedron() {
        let mut e = sphere_engine();
        let this = aligned(&mut e, 0x80);
        e.mem.set_f32(this + 0x6c, 1.0);
        e.call(0x00e0_b360, &args![this]);
        let block = e.mem.u32(this + 8);
        // Twenty faces, each split to depth two: sixteen triangles from each.
        assert_eq!(e.mem.u32(block + 4), 12 + 20 * 15);
        assert_eq!(e.mem.u32(block + 0x10), 20 * 16);
        let a = (f64::from_bits(0x3fe0_d2ca_0000_0000) * 1.0) as f32;
        let b = (1.0 * f64::from_bits(0x3feb_3888_0000_0000)) as f32;
        for (index, expected) in [
            [-a, 0.0, b, 0.0],
            [a, 0.0, b, 0.0],
            [-a, 0.0, -b, 0.0],
            [a, 0.0, -b, 0.0],
            [0.0, b, a, 0.0],
            [0.0, b, -a, 0.0],
            [0.0, -b, a, 0.0],
            [0.0, -b, -a, 0.0],
            [b, a, 0.0, 0.0],
            [-b, a, 0.0, 0.0],
            [b, -a, 0.0, 0.0],
            [-b, -a, 0.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(vertex(&e, block, index as u32), expected);
        }
        // The first face of the icosahedron was split: its record now names the middle triangle
        // (the first three new vertices).
        assert_eq!(face(&e, block, 0).map(|v| v >= 12), [true; 3]);
        // The unsplit corner of the last face keeps its index record fourth word (-1 for those the
        // icosahedron wrote, untouched by the splits).
        let data = e.mem.u32(block + 0xc);
        assert_eq!(e.mem.u32(data + 0xc), 0xffff_ffff);
    }
}
