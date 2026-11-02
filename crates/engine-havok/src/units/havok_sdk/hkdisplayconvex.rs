//! `Havok SDK/hkdisplayconvex.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplayConvex` (Xbox PDB) does not build its geometry: the constructor is handed the
//! geometry block (the layout `hkdisplaygeometry` describes: vertices at `+0`, index records
//! at `+0xc`, each record three vertex indices and a fourth word) and keeps it at `+8`. The
//! shape kind word at `+0x50` is 6.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdisplaygeometry::reset_transform;
use super::hkpsphereshape::copy_vec;

/// The vtable the constructor stores at `+0`.
const CONVEX_VTABLE: u32 = 0x010d_80e8;
/// Reserves room for one more element (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW_BY_ONE: u32 = 0x00c9_0940;

// Translated from 00de1d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayConvex::buildGeometry` (Xbox PDB): the exe's body is a bare `RET`; the geometry
/// block is supplied to the constructor, so there is nothing to build.
pub fn hk_display_convex_build_geometry(_e: &mut Engine, _this: Ptr) {}

// Translated from 00de1d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayConvex::hkDisplayConvex` (Xbox PDB): sets the kind word `+0x50` to 6, the 16-bit
/// count `+6` to 1, writes the identity transform (`reset_transform`), the vtable, and keeps
/// the geometry block pointer argument at `+8`. Returns `this`.
pub fn hk_display_convex_constructor(e: &mut Engine, this: Ptr, geometry: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 0x50, 6);
    e.mem.set_u16(me + 6, 1);
    reset_transform(e, me);
    e.mem.set_u32(me, CONVEX_VTABLE);
    e.mem.set_u32(me + 8, geometry.addr());
    this
}

/// Appends the 16-byte vertex at `source` to the vector array `out`: when the size has
/// reached the capacity (masked with `0x3fffffff`) the array is first grown by one element
/// (`00c90940`, `(allocator, out, 0x10)`).
fn append_vertex(e: &mut Engine, out: u32, allocator: u32, source: u32) {
    if e.mem.u32(out + 4) == e.mem.u32(out + 8) & 0x3fff_ffff {
        e.call(ARRAY_GROW_BY_ONE, &args![allocator, out, 0x10u32]);
    }
    let slot = e
        .mem
        .u32(out + 4)
        .wrapping_mul(16)
        .wrapping_add(e.mem.u32(out));
    copy_vec(e, source, slot);
    let counted = e.mem.u32(out + 4).wrapping_add(1);
    e.mem.set_u32(out + 4, counted);
}

// Translated from 00de1d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayConvex::getWireframeGeometry` (Xbox PDB), `this` in `ECX`, two stack words: the
/// output vertex array and the allocator it grows with. Walks the index records of the
/// geometry block from the last to the first and, for each record with vertex indices
/// `a, b, c`, appends the six vertices `a, b, a, c, b, c`: the three edge segments of the
/// triangle. Does nothing without a geometry block or with no records.
pub fn hk_display_convex_get_wireframe_geometry(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    allocator: u32,
) {
    let block = e.mem.u32(this.addr() + 8);
    if block == 0 {
        return;
    }
    let mut record = e.mem.i32(block + 0x10).wrapping_sub(1);
    if record < 0 {
        return;
    }
    loop {
        let block = e.mem.u32(this.addr() + 8);
        let records = e.mem.u32(block + 0xc);
        let at = records.wrapping_add((record as u32).wrapping_mul(16));
        let first = e.mem.u32(at).wrapping_mul(16);
        let second = e.mem.u32(at + 4).wrapping_mul(16);
        let third = e.mem.u32(at + 8).wrapping_mul(16);
        for index in [first, second, first, third, second, third] {
            let vertices = e.mem.u32(e.mem.u32(this.addr() + 8));
            append_vertex(e, out.addr(), allocator, vertices.wrapping_add(index));
        }
        record = record.wrapping_sub(1);
        if record < 0 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_1d10, hk_display_convex_build_geometry(Ptr)),
        entry!(0x00de_1d20, hk_display_convex_constructor(Ptr, Ptr) -> Ptr),
        entry!(
            0x00de_1d70,
            hk_display_convex_get_wireframe_geometry(Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, Vec4};

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

    /// A grow-by-one double: moves the array to a buffer of double the capacity (at least
    /// four elements), keeping its contents and clearing the flag bits.
    fn install_grow(e: &mut Engine) {
        e.register_double(ARRAY_GROW_BY_ONE, |e, a| {
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff).max(2) * 2;
            let fresh = e.mem.alloc(capacity * a[2]);
            if size > 0 {
                let bytes = e.mem.bytes(old, size * a[2]);
                e.mem.write(fresh, &bytes);
            }
            e.mem.set_u32(a[1], fresh);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
    }

    /// A geometry block with four vertices and the two triangles `(0, 1, 2)` and `(1, 2, 3)`.
    fn shape(e: &mut Engine) -> (u32, Vec<Vec4>) {
        let this = e.mem.alloc(0xa0);
        let block = e.mem.alloc(0x18);
        let vertices: Vec<Vec4> = (0..4u8)
            .map(|i| {
                let i = f32::from(i);
                [i, 10.0 + i, 20.0 + i, 30.0 + i]
            })
            .collect();
        let data = e.mem.alloc(0x40);
        for (i, v) in vertices.iter().enumerate() {
            store_vec(e, data + 0x10 * i as u32, *v);
        }
        e.mem.set_u32(block, data);
        e.mem.set_u32(block + 4, 4);
        e.mem.set_u32(block + 8, 4);
        let records = e.mem.alloc(0x20);
        for (i, record) in [[0u32, 1, 2, 0xffff_ffff], [1, 2, 3, 0xffff_ffff]]
            .iter()
            .enumerate()
        {
            for (lane, word) in record.iter().enumerate() {
                e.mem
                    .set_u32(records + 0x10 * i as u32 + 4 * lane as u32, *word);
            }
        }
        e.mem.set_u32(block + 0xc, records);
        e.mem.set_u32(block + 0x10, 2);
        e.mem.set_u32(block + 0x14, 2);
        e.mem.set_u32(this + 8, block);
        (this, vertices)
    }

    fn output(e: &Engine, out: u32) -> Vec<Vec4> {
        let data = e.mem.u32(out);
        (0..e.mem.u32(out + 4))
            .map(|i| load_vec(e, data + 0x10 * i))
            .collect()
    }

    #[test]
    fn hk_display_convex_build_geometry_does_nothing() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x1234);
        e.call(0x00de_1d10, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0x1234);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|c| c.0 == 0x00de_1d10));
    }

    #[test]
    fn hk_display_convex_constructor_stores_the_geometry_and_the_identity_transform() {
        let mut e = engine();
        let this = e.mem.alloc(0xa0);
        for offset in (0..0x60).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let got = e.call(0x00de_1d20, &args![this, 0x4455_6677u32]);
        assert_eq!(got.u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_80e8);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        assert_eq!(e.mem.u32(this + 8), 0x4455_6677);
        assert_eq!(e.mem.u32(this + 0x50), 6);
        for lane in 0..16u32 {
            let expected = if [0, 5, 10].contains(&lane) { 1.0 } else { 0.0 };
            assert_eq!(e.mem.f32(this + 0x10 + 4 * lane), expected);
        }
        // Nothing past the transform and the kind word is written.
        assert_eq!(e.mem.u32(this + 0x54), 0xdead_beef);
    }

    #[test]
    fn hk_display_convex_get_wireframe_geometry_appends_three_edges_per_record_last_first() {
        let mut e = engine();
        install_grow(&mut e);
        let (this, v) = shape(&mut e);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call(0x00de_1d70, &args![this, out, 0x7777u32]);
        assert_eq!(
            output(&e, out),
            vec![
                v[1], v[2], v[1], v[3], v[2], v[3], // the record (1, 2, 3)
                v[0], v[1], v[0], v[2], v[1], v[2], // the record (0, 1, 2)
            ]
        );
        // Grown when size met capacity: at 0 (capacity 0), 4 and 8, each time by one element
        // of 0x10 bytes for the allocator the caller passed.
        let grows = calls_to(&e, ARRAY_GROW_BY_ONE);
        assert_eq!(grows, vec![vec![0x7777, out, 0x10]; 3]);
    }

    #[test]
    fn hk_display_convex_get_wireframe_geometry_does_not_grow_an_array_with_room() {
        let mut e = engine();
        install_grow(&mut e);
        let (this, v) = shape(&mut e);
        let out = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10 * 12);
        e.mem.set_u32(out, data);
        // The capacity word carries a flag bit above the capacity: only the low 30 bits count.
        e.mem.set_u32(out + 8, 0x8000_0000 | 12);
        e.call(0x00de_1d70, &args![this, out, 0u32]);
        assert!(calls_to(&e, ARRAY_GROW_BY_ONE).is_empty());
        assert_eq!(e.mem.u32(out), data);
        assert_eq!(output(&e, out).len(), 12);
        assert_eq!(output(&e, out)[0], v[1]);
    }

    #[test]
    fn hk_display_convex_get_wireframe_geometry_appends_after_existing_vertices() {
        let mut e = engine();
        install_grow(&mut e);
        let (this, v) = shape(&mut e);
        let out = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10 * 20);
        store_vec(&mut e, data, [9.0, 9.0, 9.0, 9.0]);
        e.mem.set_u32(out, data);
        e.mem.set_u32(out + 4, 1);
        e.mem.set_u32(out + 8, 20);
        e.call(0x00de_1d70, &args![this, out, 0u32]);
        let got = output(&e, out);
        assert_eq!(got.len(), 13);
        assert_eq!(got[0], [9.0, 9.0, 9.0, 9.0]);
        assert_eq!(got[1], v[1]);
    }

    #[test]
    fn hk_display_convex_get_wireframe_geometry_ignores_a_missing_block_or_no_records() {
        let mut e = engine();
        install_grow(&mut e);
        let this = e.mem.alloc(0xa0);
        let out = e.mem.alloc(0x10);
        e.call(0x00de_1d70, &args![this, out, 0u32]);
        assert_eq!(e.mem.u32(out + 4), 0);
        let block = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, block);
        e.call(0x00de_1d70, &args![this, out, 0u32]);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert!(calls_to(&e, ARRAY_GROW_BY_ONE).is_empty());
    }
}
