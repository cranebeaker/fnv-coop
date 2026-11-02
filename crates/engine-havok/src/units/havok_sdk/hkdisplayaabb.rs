//! `Havok SDK/hkdisplayaabb.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the arrays and the
/// geometry object come from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The exe's float constant (0.0) that the fourth lane of every vertex is read from.
const FOURTH_LANE: u32 = 0x0101_1d78;
/// The vtable of `hkDisplayAABB`.
const VTABLE: u32 = 0x010d_80d0;

/// The exe moves 16-byte vectors with `MOVAPS`, which faults on an address that is not 16-byte
/// aligned.
fn assert_aligned(address: u32) {
    assert_eq!(
        address & 0xf,
        0,
        "MOVAPS address {address:#010x} is not 16-byte aligned"
    );
}

/// A `MOVAPS` store of four words.
fn store_vector(e: &mut Engine, address: u32, lanes: [u32; 4]) {
    assert_aligned(address);
    for (index, lane) in lanes.iter().enumerate() {
        e.mem.set_u32(address + 4 * index as u32, *lane);
    }
}

/// The part of both constructors that comes before the extents: the word `0` at `+8`, 3 at
/// `+0x50`, 1 at `+6` (a word), the 4x4 transform at `+0x10` as the identity (zero vectors
/// with 1.0 stored at `+0x10`, `+0x24` and `+0x38`), a zero vector at `+0x40` and the vtable.
fn initialise(e: &mut Engine, this: u32) {
    assert_aligned(this);
    e.mem.set_u32(this + 8, 0);
    e.mem.set_u32(this + 0x50, 3);
    e.mem.set_u16(this + 6, 1);
    for offset in [0x10, 0x20, 0x30] {
        store_vector(e, this + offset, [0; 4]);
    }
    for offset in [0x10, 0x24, 0x38] {
        e.mem.set_u32(this + offset, 1.0f32.to_bits());
    }
    store_vector(e, this + 0x40, [0; 4]);
    e.mem.set_u32(this, VTABLE);
}

// Translated from 00de1170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayAABB::setExtents` (Xbox PDB; `this` = the display box, then `(minimum, maximum)`,
/// `RET 8`): copies the two 16-byte vectors to `+0x60` and `+0x70`. All four addresses must be
/// 16-byte aligned (`MOVAPS`).
pub fn hk_display_aabb_set_extents(e: &mut Engine, this: Ptr, minimum: Ptr, maximum: Ptr) {
    let extents = [(minimum.addr(), 0x60), (maximum.addr(), 0x70)];
    for (source, offset) in extents {
        assert_aligned(source);
        assert_aligned(this.addr() + offset);
        for word in 0..4 {
            let value = e.mem.u32(source + 4 * word);
            e.mem.set_u32(this.addr() + offset + 4 * word, value);
        }
    }
}

// Translated from 00de1190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor with extents (`this` = the display box, then `(minimum, maximum)`, `RET 8`;
/// not named in the map): the common start (word `0` at `+8`, 3 at `+0x50`, 1 at `+6`, the
/// identity transform at `+0x10`, a zero vector at `+0x40`, the vtable `0x010D80D0`), then
/// the four words of the minimum to `+0x60` and of the maximum to `+0x70`, one word at a
/// time. Answers `this`.
pub fn fn_00de1190(e: &mut Engine, this: Ptr, minimum: Ptr, maximum: Ptr) -> Ptr {
    initialise(e, this.addr());
    for word in 0..4 {
        let value = e.mem.u32(minimum.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 0x60 + 4 * word, value);
    }
    for word in 0..4 {
        let value = e.mem.u32(maximum.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 0x70 + 4 * word, value);
    }
    this
}

// Translated from 00de1210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayAABB::hkDisplayAABB` (Xbox PDB): the common start of `fn_00de1190` followed by
/// zero vectors for both extents (`+0x60`, `+0x70`). Answers `this`.
pub fn hk_display_aabb_construct(e: &mut Engine, this: Ptr) -> Ptr {
    initialise(e, this.addr());
    store_vector(e, this.addr() + 0x60, [0; 4]);
    store_vector(e, this.addr() + 0x70, [0; 4]);
    this
}

/// Which extent each coordinate of the 24 vertices of the wireframe comes from (0 = the
/// minimum at `+0x60`, 1 = the maximum at `+0x70`), as `(x, y, z)`. Consecutive pairs are the
/// ends of the 12 edges.
const WIREFRAME_CORNERS: [[u32; 3]; 24] = [
    [0, 0, 0],
    [0, 1, 0],
    [0, 0, 0],
    [0, 0, 1],
    [0, 0, 0],
    [1, 0, 0],
    [1, 1, 1],
    [1, 1, 0],
    [1, 1, 1],
    [0, 1, 1],
    [1, 1, 1],
    [1, 0, 1],
    [0, 1, 0],
    [1, 1, 0],
    [0, 1, 0],
    [0, 1, 1],
    [1, 1, 0],
    [1, 0, 0],
    [0, 1, 1],
    [0, 0, 1],
    [0, 0, 1],
    [1, 0, 1],
    [1, 0, 1],
    [1, 0, 0],
];

// Translated from 00de1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayAABB::getWireframeGeometry` (Xbox PDB; `this` = the display box, then `(array,
/// allocator)`, `RET 8`): makes sure the `hkArray` of 16-byte vectors (data, size, capacity
/// word) has room for 24 elements (when its capacity, the low 30 bits of the word, is
/// below 0x18 it is reserved through `hkArrayUtil::_reserve` (`00c908c0`) with the allocator,
/// twice the capacity or 0x18, whichever is larger, and the element size 0x10), sets the size to
/// 0x18 and stores the 24 corner vertices of the 12 edges of the box between the extents at
/// `+0x60` and `+0x70`, each with the fourth lane read from the float at `0101_1d78`. The data
/// must be 16-byte aligned (`MOVAPS`).
pub fn hk_display_aabb_get_wireframe_geometry(
    e: &mut Engine,
    this: Ptr,
    array: Ptr,
    allocator: u32,
) {
    let box_address = this.addr();
    let array = array.addr();
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < 0x18 {
        let wanted = capacity.wrapping_add(capacity).max(0x18);
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, wanted as u32, 0x10u32],
        );
    }
    e.mem.set_u32(array + 4, 0x18);
    let fourth = e.mem.u32(FOURTH_LANE);
    for (index, corner) in WIREFRAME_CORNERS.iter().enumerate() {
        let x = e.mem.u32(box_address + 0x60 + 0x10 * corner[0]);
        let y = e.mem.u32(box_address + 0x64 + 0x10 * corner[1]);
        let z = e.mem.u32(box_address + 0x68 + 0x10 * corner[2]);
        let data = e.mem.u32(array);
        store_vector(e, data + 0x10 * index as u32, [x, y, z, fourth]);
    }
}

/// Appends one element slot to the `hkArray` at `array` the way the exe's inlined push does:
/// asks the memory router for its allocator, and when the capacity (the low 30 bits of the
/// word at `+8`) is below the size plus one reserves twice the capacity or size plus one,
/// whichever is larger, with element size 0x10 (`00c908c0`); raises the size and answers the
/// address of the new element in the data.
fn append_slot(e: &mut Engine, array: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.u32(array + 4);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    let wanted = (size as i32).wrapping_add(1);
    if capacity < wanted {
        let new_capacity = wanted.max(capacity.wrapping_add(capacity));
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity as u32, 0x10u32],
        );
    }
    let current = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, current.wrapping_add(1));
    size.wrapping_shl(4).wrapping_add(e.mem.u32(array))
}

/// The eight corners of the box as `(x, y, z)` extent selectors (0 minimum, 1 maximum), in
/// the order `buildGeometry` appends them.
const BOX_VERTICES: [[u32; 3]; 8] = [
    [0, 0, 0],
    [0, 0, 1],
    [1, 0, 1],
    [1, 0, 0],
    [0, 1, 0],
    [0, 1, 1],
    [1, 1, 1],
    [1, 1, 0],
];

/// The 12 triangles over those vertices, each with the word -1 after the three indices.
const BOX_TRIANGLES: [[u32; 3]; 12] = [
    [0, 3, 1],
    [1, 3, 2],
    [2, 6, 5],
    [5, 1, 2],
    [5, 6, 4],
    [4, 6, 7],
    [7, 3, 0],
    [0, 4, 7],
    [0, 1, 4],
    [4, 1, 5],
    [2, 3, 6],
    [6, 3, 7],
];

// Translated from 00de15c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayAABB::buildGeometry` (Xbox PDB; `this` = the display box, nothing else): gets a
/// 0x18-byte geometry object from the memory router's allocator (virtual call at `+4` with
/// 0x18; when it answers null the exe goes on with a null object and faults reading its size),
/// gives it two empty arrays (vertices at `+0`, triangles at `+0xC`: data 0, size 0, capacity
/// word `0x80000000`), stores it at `this + 8`, then appends the eight corner vertices of the
/// box between the extents at `+0x60` and `+0x70` (fourth lane read from the float at
/// `0101_1d78`) and the twelve triangles (three vertex indices and -1). The vertex stores
/// are `MOVAPS`: the array data must be 16-byte aligned.
pub fn hk_display_aabb_build_geometry(e: &mut Engine, this: Ptr) {
    let box_address = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x18u32]).u32();
    let geometry = if block != 0 {
        e.mem.set_u32(block, 0);
        e.mem.set_u32(block + 4, 0);
        e.mem.set_u32(block + 8, 0x8000_0000);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x10, 0);
        e.mem.set_u32(block + 0x14, 0x8000_0000);
        block
    } else {
        0
    };
    e.mem.set_u32(box_address + 8, geometry);
    for corner in BOX_VERTICES {
        let geometry = e.mem.u32(box_address + 8);
        let slot = append_slot(e, geometry);
        let x = e.mem.u32(box_address + 0x60 + 0x10 * corner[0]);
        let y = e.mem.u32(box_address + 0x64 + 0x10 * corner[1]);
        let z = e.mem.u32(box_address + 0x68 + 0x10 * corner[2]);
        let fourth = e.mem.u32(FOURTH_LANE);
        store_vector(e, slot, [x, y, z, fourth]);
    }
    for triangle in BOX_TRIANGLES {
        let geometry = e.mem.u32(box_address + 8);
        let slot = append_slot(e, geometry + 0xc);
        e.mem.set_u32(slot, triangle[0]);
        e.mem.set_u32(slot + 4, triangle[1]);
        e.mem.set_u32(slot + 8, triangle[2]);
        e.mem.set_u32(slot + 0xc, 0xffff_ffff);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_1170, hk_display_aabb_set_extents(Ptr, Ptr, Ptr)),
        entry!(0x00de_1190, fn_00de1190(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00de_1210, hk_display_aabb_construct(Ptr) -> Ptr),
        entry!(
            0x00de_1260,
            hk_display_aabb_get_wireframe_geometry(Ptr, Ptr, u32)
        ),
        entry!(0x00de_15c0, hk_display_aabb_build_geometry(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 16-byte aligned block of `size` zeroed bytes.
    fn aligned_block(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    /// An engine whose float constant page is mapped and holds `fourth`.
    fn engine(fourth: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u32(FOURTH_LANE, fourth);
        e
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + 4 * i)).collect()
    }

    fn floats(values: [f32; 4]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    /// A box at an aligned address, its words dirtied, so the constructors' stores show.
    fn dirty_box(e: &mut Engine) -> u32 {
        let this = aligned_block(e, 0x80);
        for offset in (0..0x80).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        this
    }

    #[test]
    fn construct_sets_the_identity_and_zero_extents() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        assert_eq!(e.call(0x00de_1210, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_80d0);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 3);
        assert_eq!(words(&e, this + 0x10, 4), floats([1.0, 0.0, 0.0, 0.0]));
        assert_eq!(words(&e, this + 0x20, 4), floats([0.0, 1.0, 0.0, 0.0]));
        assert_eq!(words(&e, this + 0x30, 4), floats([0.0, 0.0, 1.0, 0.0]));
        assert_eq!(words(&e, this + 0x40, 4), vec![0; 4]);
        assert_eq!(words(&e, this + 0x60, 8), vec![0; 8]);
        // The words the constructor does not touch are left alone.
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn construct_faults_on_an_unaligned_box() {
        let mut e = Engine::new();
        let this = aligned_block(&mut e, 0x90) + 8;
        e.call(0x00de_1210, &args![this]);
    }

    #[test]
    fn construct_with_extents_copies_them_word_by_word() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        // The extents need no alignment for this constructor.
        let source = e.mem.alloc(0x20) + 4;
        for i in 0..8 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        assert_eq!(
            e.call(0x00de_1190, &args![this, source, source + 16]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 0x010d_80d0);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x50), 3);
        assert_eq!(words(&e, this + 0x10, 4), floats([1.0, 0.0, 0.0, 0.0]));
        assert_eq!(words(&e, this + 0x20, 4), floats([0.0, 1.0, 0.0, 0.0]));
        assert_eq!(words(&e, this + 0x30, 4), floats([0.0, 0.0, 1.0, 0.0]));
        assert_eq!(words(&e, this + 0x40, 4), vec![0; 4]);
        assert_eq!(words(&e, this + 0x60, 4), vec![0x100, 0x101, 0x102, 0x103]);
        assert_eq!(words(&e, this + 0x70, 4), vec![0x104, 0x105, 0x106, 0x107]);
    }

    #[test]
    fn set_extents_copies_both_vectors() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        let minimum = aligned_block(&mut e, 16);
        let maximum = aligned_block(&mut e, 16);
        for i in 0..4 {
            e.mem.set_u32(minimum + 4 * i, 0x10 + i);
            e.mem.set_u32(maximum + 4 * i, 0x20 + i);
        }
        e.call(0x00de_1170, &args![this, minimum, maximum]);
        assert_eq!(words(&e, this + 0x60, 4), vec![0x10, 0x11, 0x12, 0x13]);
        assert_eq!(words(&e, this + 0x70, 4), vec![0x20, 0x21, 0x22, 0x23]);
        assert_eq!(e.mem.u32(this + 0x5c), 0xdead_beef);
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn set_extents_faults_on_an_unaligned_vector() {
        let mut e = Engine::new();
        let this = dirty_box(&mut e);
        let minimum = aligned_block(&mut e, 32) + 4;
        let maximum = aligned_block(&mut e, 16);
        e.call(0x00de_1170, &args![this, minimum, maximum]);
    }

    /// A box at an aligned address with the minimum (1, 2, 3, 9) and the maximum
    /// (10, 20, 30, 9).
    fn extents_box(e: &mut Engine) -> u32 {
        let this = dirty_box(e);
        for (offset, value) in [(0x60, 1.0f32), (0x64, 2.0), (0x68, 3.0), (0x6c, 9.0)] {
            e.mem.set_f32(this + offset, value);
        }
        for (offset, value) in [(0x70, 10.0f32), (0x74, 20.0), (0x78, 30.0), (0x7c, 9.0)] {
            e.mem.set_f32(this + offset, value);
        }
        this
    }

    /// An array header (data, size, capacity word) with `capacity` and the data an aligned
    /// block of 0x200 bytes.
    fn array_with(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = aligned_block(e, 0x200);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    /// The reserve double that logs and swaps in a larger aligned block with the new
    /// capacity (element size 0x10), keeping the old elements.
    fn install_reserve(e: &mut Engine) {
        e.register(ARRAY_RESERVE, |e, a| {
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let grown = (e.mem.alloc(a[2] * a[3] + 16) + 15) & !15;
            for word in 0..size * 4 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(grown + 4 * word, value);
            }
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
    }

    fn vertex(e: &Engine, address: u32) -> [f32; 4] {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
            e.mem.f32(address + 12),
        ]
    }

    fn reserved(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == ARRAY_RESERVE)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn wireframe_stores_24_vertices_that_form_the_12_edges() {
        let mut e = engine(0x7fc0_0001);
        install_reserve(&mut e);
        e.call_log = Some(Vec::new());
        let this = extents_box(&mut e);
        let array = array_with(&mut e, 0x8000_0000);
        e.call(0x00de_1260, &args![this, array, 0x5000u32]);
        assert_eq!(reserved(&e), vec![vec![0x5000, array, 0x18, 0x10]]);
        assert_eq!(e.mem.u32(array + 4), 0x18);
        let data = e.mem.u32(array);
        let vertices: Vec<[f32; 4]> = (0..24).map(|i| vertex(&e, data + 0x10 * i)).collect();
        for v in &vertices {
            assert!([1.0, 10.0].contains(&v[0]));
            assert!([2.0, 20.0].contains(&v[1]));
            assert!([3.0, 30.0].contains(&v[2]));
            assert!(v[3].is_nan());
        }
        // The fourth lane keeps the constant's bits, quiet or not.
        assert_eq!(e.mem.u32(data + 12), 0x7fc0_0001);
        // Each pair differs in exactly one coordinate; the 12 edges are all different and
        // every corner is the end of three edges.
        let mut edges = Vec::new();
        let mut corner_uses = std::collections::BTreeMap::new();
        for pair in vertices.chunks(2) {
            let differing = (0..3).filter(|&i| pair[0][i] != pair[1][i]).count();
            assert_eq!(differing, 1);
            let key = |v: &[f32; 4]| [v[0] as u32, v[1] as u32, v[2] as u32];
            let mut edge = [key(&pair[0]), key(&pair[1])];
            for end in edge {
                *corner_uses.entry(end).or_insert(0) += 1;
            }
            edge.sort_unstable();
            edges.push(edge);
        }
        edges.sort_unstable();
        edges.dedup();
        assert_eq!(edges.len(), 12);
        assert_eq!(corner_uses.len(), 8);
        assert!(corner_uses.values().all(|&n| n == 3));
        // The first edge runs along y from the minimum corner, the last along z at the
        // maximum x with the minimum y.
        assert_eq!(vertices[0][..3], [1.0, 2.0, 3.0]);
        assert_eq!(vertices[1][..3], [1.0, 20.0, 3.0]);
        assert_eq!(vertices[22][..3], [10.0, 2.0, 30.0]);
        assert_eq!(vertices[23][..3], [10.0, 2.0, 3.0]);
    }

    #[test]
    fn wireframe_reserves_only_when_the_capacity_is_below_24() {
        for (capacity, wanted) in [
            (0x8000_0000u32, Some(0x18u32)),
            (0x0c, Some(0x18)),
            (0x0d, Some(0x1a)),
            (0x17, Some(0x2e)),
            (0x18, None),
            (0xc000_0018, None),
        ] {
            let mut e = engine(0);
            install_reserve(&mut e);
            e.call_log = Some(Vec::new());
            let this = extents_box(&mut e);
            let array = array_with(&mut e, capacity);
            e.call(0x00de_1260, &args![this, array, 0x5000u32]);
            let expected: Vec<Vec<u32>> = wanted
                .map(|w| vec![vec![0x5000, array, w, 0x10]])
                .unwrap_or_default();
            assert_eq!(reserved(&e), expected, "capacity word {capacity:#x}");
            assert_eq!(e.mem.u32(array + 4), 0x18);
        }
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn wireframe_faults_on_unaligned_data() {
        let mut e = engine(0);
        let this = extents_box(&mut e);
        let array = array_with(&mut e, 0x18);
        let data = e.mem.u32(array);
        e.mem.set_u32(array, data + 4);
        e.call(0x00de_1260, &args![this, array, 0x5000u32]);
    }

    /// An engine for `buildGeometry`: a memory router whose allocator's virtual `+4` hands out
    /// blocks (logging the requested size) and the reserve double above.
    fn geometry_engine(allocation: Option<u32>) -> Engine {
        let mut e = engine(0);
        install_reserve(&mut e);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0044);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0044, move |e, _| Ret {
            eax: allocation.unwrap_or_else(|| e.mem.alloc(0x18)),
            ..Ret::default()
        });
        e.call_log = Some(Vec::new());
        e
    }

    #[test]
    fn build_geometry_makes_the_box_vertices_and_triangles() {
        let mut e = geometry_engine(None);
        let this = extents_box(&mut e);
        e.call(0x00de_15c0, &args![this]);
        let geometry = e.mem.u32(this + 8);
        assert_eq!(e.mem.block_size(geometry), Some(0x18));
        let allocations: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x0900_0044)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0][1..], [0x18]);
        assert_eq!(e.mem.u32(geometry + 4), 8);
        assert_eq!(e.mem.u32(geometry + 0x10), 12);
        let vertices = e.mem.u32(geometry);
        let expected: [[f32; 3]; 8] = [
            [1.0, 2.0, 3.0],
            [1.0, 2.0, 30.0],
            [10.0, 2.0, 30.0],
            [10.0, 2.0, 3.0],
            [1.0, 20.0, 3.0],
            [1.0, 20.0, 30.0],
            [10.0, 20.0, 30.0],
            [10.0, 20.0, 3.0],
        ];
        for (index, corner) in expected.iter().enumerate() {
            let got = vertex(&e, vertices + 0x10 * index as u32);
            assert_eq!(got, [corner[0], corner[1], corner[2], 0.0]);
        }
        let triangles = e.mem.u32(geometry + 0xc);
        let all: Vec<Vec<u32>> = (0..12)
            .map(|i| words(&e, triangles + 0x10 * i, 4))
            .collect();
        let expected_triangles: [[u32; 3]; 12] = [
            [0, 3, 1],
            [1, 3, 2],
            [2, 6, 5],
            [5, 1, 2],
            [5, 6, 4],
            [4, 6, 7],
            [7, 3, 0],
            [0, 4, 7],
            [0, 1, 4],
            [4, 1, 5],
            [2, 3, 6],
            [6, 3, 7],
        ];
        for (got, want) in all.iter().zip(expected_triangles) {
            assert_eq!(got[..3], want);
            assert_eq!(got[3], 0xffff_ffff);
        }
        // The two empty arrays grew one element at a time, doubling: capacities 1, 2, 4, 8
        // for the vertices and one more step for the twelve triangles.
        let grown: Vec<(u32, u32)> = reserved(&e).iter().map(|w| (w[1], w[2])).collect();
        let mut want = Vec::new();
        for capacity in [1, 2, 4, 8] {
            want.push((geometry, capacity));
        }
        for capacity in [1, 2, 4, 8, 16] {
            want.push((geometry + 0xc, capacity));
        }
        assert_eq!(grown, want);
        assert!(reserved(&e).iter().all(|w| w[3] == 0x10));
    }

    #[test]
    fn build_geometry_stores_the_fourth_lane_bits_of_the_constant() {
        let mut e = geometry_engine(None);
        e.mem.set_u32(FOURTH_LANE, 0x7f80_0001);
        let this = extents_box(&mut e);
        e.call(0x00de_15c0, &args![this]);
        let geometry = e.mem.u32(this + 8);
        let vertices = e.mem.u32(geometry);
        for index in 0..8 {
            assert_eq!(e.mem.u32(vertices + 0x10 * index + 12), 0x7f80_0001);
        }
    }

    #[test]
    #[should_panic]
    fn build_geometry_with_a_null_allocation_faults_like_the_exe() {
        let mut e = geometry_engine(Some(0));
        let this = extents_box(&mut e);
        e.call(0x00de_15c0, &args![this]);
    }
}
