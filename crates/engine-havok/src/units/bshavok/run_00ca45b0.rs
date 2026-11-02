//! `BSHavok/run_00ca45b0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions belong to the shape built from `NiTriStripsData` (the neighbouring
//! functions of the engine map come from `hknitristripsshape.obj`); the class itself is not
//! named by the executable and is left unconfirmed here.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriStripsData::GetStripLists` (name from the engine map's neighbourhood, unconfirmed):
/// returns the field at +0x4c (`this` only).
const GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// Constructor of the child triangle shape: `this` is the block to fill, the stack word is its
/// `float` radius (bit pattern). Returns `this`.
const CONSTRUCT_TRIANGLE_SHAPE: u32 = 0x00ca_39a0;
/// The `float` every vertex coordinate is multiplied with (read from `.data`).
const VERTEX_SCALE: u32 = 0x0126_82c8;

layout! {
    /// The shape of the strips (class unconfirmed), PC offsets.
    pub struct TriStripsShape: 0x90 {
        /// A 16-byte vector multiplied lane by lane into every vertex (+0x70..+0x7c).
        0x70 m_scale_x: f32,
        0x74 m_scale_y: f32,
        0x78 m_scale_z: f32,
        0x7c m_scale_w: f32,
        /// The `float` handed to the triangle's constructor.
        0x80 m_radius: f32,
        /// Table of 8-byte records, indexed by the top 12 bits of a key: the first word points
        /// to the strips data, the second word is what `fn_00ca4780` returns.
        0x88 m_entries: Ptr,
    }
}

/// Fields of the strips data the entries point to (`NiTriStripsData`-like, PC offsets).
const DATA_VERTICES: u32 = 0x20;
const DATA_STRIP_COUNT: u32 = 0x44;
const DATA_STRIP_LENGTHS: u32 = 0x48;

// Translated from 00ca4780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second word of the table record picked by the top 12 bits of `key`.
pub fn fn_00ca4780(e: &mut Engine, this: Ptr<TriStripsShape>, key: u32) -> u32 {
    let table = e.get(this, TriStripsShape::m_entries).addr();
    e.mem.u32(
        table
            .wrapping_add(4)
            .wrapping_add((key >> 20).wrapping_mul(8)),
    )
}

// Translated from 00ca45b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca45b0 - the /GS stack cookie check (00ec408c) at both returns is not modelled; the body is otherwise complete.
/// Builds the triangle `key` names into `shape`. The top 12 bits of `key` pick the strips data
/// from the table, the low 20 bits are the position in the data's concatenated strips (each
/// strip of `n` indices holds `n - 2` triangles; the walk subtracts `n - 2` per strip passed).
/// Three consecutive indices that are not pairwise different make no triangle: returns 0.
/// Otherwise the child shape is constructed in `shape` (with the radius at +0x80) and its three
/// vertices at +0x20, +0x30 and +0x40 are the strips data's vertices times the scale
/// coordinate by coordinate, with 0 as the fourth lane, times the vector at +0x70. Returns the
/// block as the constructor returned it (the register the exe leaves it in); a null `shape`
/// makes the writes fault as in the game.
pub fn fn_00ca45b0(e: &mut Engine, this: Ptr<TriStripsShape>, key: u32, shape: Ptr) -> Ptr {
    let table = e.get(this, TriStripsShape::m_entries).addr();
    let data = e.mem.u32(table.wrapping_add((key >> 20).wrapping_mul(8)));
    let strip_count = e.mem.u16(data + DATA_STRIP_COUNT);
    let mut lists = e.call(GET_STRIP_LISTS, &args![data]).u32();
    let lengths = e.mem.u32(data + DATA_STRIP_LENGTHS);
    let vertices = e.mem.u32(data + DATA_VERTICES);

    let mut offset = key & 0xfffff;
    let mut strip: u16 = 0;
    if strip_count != 0 {
        loop {
            let length = u32::from(e.mem.u16(lengths.wrapping_add(u32::from(strip) * 2)));
            if offset < length.wrapping_sub(2) {
                break;
            }
            strip = strip.wrapping_add(1);
            offset = offset.wrapping_add(2u32.wrapping_sub(length));
            lists = lists.wrapping_add(length * 2);
            if strip >= strip_count {
                break;
            }
        }
    }

    let indices = lists.wrapping_add(offset.wrapping_mul(2));
    let first = e.mem.u16(indices);
    let second = e.mem.u16(indices.wrapping_add(2));
    let third = e.mem.u16(indices.wrapping_add(4));
    if first == second || first == third || second == third {
        return Ptr::NULL;
    }
    let corners = [first, second, third].map(|index| vertices.wrapping_add(u32::from(index) * 12));

    let triangle: Ptr = if shape.is_null() {
        Ptr::NULL
    } else {
        let radius = x87_quiet(e.get(this, TriStripsShape::m_radius));
        e.call(CONSTRUCT_TRIANGLE_SHAPE, &args![shape, radius.to_bits()])
            .ptr()
    };

    // The `float` scale is on the x87 stack: each product is formed in extended precision and
    // rounded once on the store, then the SSE multiply by the +0x70 vector rounds again.
    let scale: f32 = e.global(VERTEX_SCALE);
    let factors = [
        e.get(this, TriStripsShape::m_scale_x),
        e.get(this, TriStripsShape::m_scale_y),
        e.get(this, TriStripsShape::m_scale_z),
        e.get(this, TriStripsShape::m_scale_w),
    ];
    for (n, corner) in corners.into_iter().enumerate() {
        let target = triangle.addr().wrapping_add(0x20 + 0x10 * n as u32);
        for (lane, factor) in factors.iter().take(3).enumerate() {
            let coordinate = e.mem.f32(corner + 4 * lane as u32);
            let scaled = (coordinate as f64 * scale as f64) as f32;
            e.mem.set_f32(target + 4 * lane as u32, scaled * factor);
        }
        e.mem.set_f32(target + 12, 0.0f32 * factors[3]);
    }
    triangle
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca4780, fn_00ca4780(Ptr<TriStripsShape>, u32) -> u32),
        entry!(
            0x00ca45b0,
            fn_00ca45b0(Ptr<TriStripsShape>, u32, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A shape with two table records. Record 1 points to strips data holding two strips of
    /// lengths 5 and 4, whose index lists are `lists` and whose vertices are `(k, k + 0.5, -k)`.
    fn setup(e: &mut Engine, lists: &[u16]) -> Ptr<TriStripsShape> {
        e.map(0x0126_8000, 0x1000);
        e.set_global(VERTEX_SCALE, 2.0f32);
        let this: Ptr<TriStripsShape> = e.new_object();
        let table = e.mem.alloc(16);
        e.mem.set_u32(table + 12, 0xabcd);
        let data = e.mem.alloc(0x60);
        e.mem.set_u32(table + 8, data);
        e.set(this, TriStripsShape::m_entries, Ptr::new(table));
        e.set(this, TriStripsShape::m_scale_x, 1.0);
        e.set(this, TriStripsShape::m_scale_y, 10.0);
        e.set(this, TriStripsShape::m_scale_z, 100.0);
        e.set(this, TriStripsShape::m_scale_w, 5.0);
        e.set(this, TriStripsShape::m_radius, 0.25);
        let vertices = e.mem.alloc(12 * 8);
        for k in 0..8u32 {
            e.mem.set_f32(vertices + 12 * k, k as f32);
            e.mem.set_f32(vertices + 12 * k + 4, k as f32 + 0.5);
            e.mem.set_f32(vertices + 12 * k + 8, -(k as f32));
        }
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        e.mem.set_u16(data + DATA_STRIP_COUNT, 2);
        let lengths = e.mem.alloc(4);
        e.mem.set_u16(lengths, 5);
        e.mem.set_u16(lengths + 2, 4);
        e.mem.set_u32(data + DATA_STRIP_LENGTHS, lengths);
        let strips = e.mem.alloc(2 * lists.len() as u32);
        for (i, index) in lists.iter().enumerate() {
            e.mem.set_u16(strips + 2 * i as u32, *index);
        }
        e.mem.set_u32(data + 0x4c, strips);
        e.register(GET_STRIP_LISTS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x4c),
            ..Ret::default()
        });
        this
    }

    fn constructor_double(e: &mut Engine) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(vec![]));
        let sink = calls.clone();
        e.register_double(CONSTRUCT_TRIANGLE_SHAPE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        calls
    }

    #[test]
    fn the_second_word_of_the_record_is_picked_by_the_top_bits() {
        let mut e = Engine::new();
        let this = setup(&mut e, &[0; 9]);
        let key = 0x0010_0005u32;
        assert_eq!(e.call(0x00ca4780, &args![this, key]).u32(), 0xabcd);
    }

    #[test]
    fn a_triangle_of_the_first_strip_is_scaled_into_the_shape() {
        let mut e = Engine::new();
        let this = setup(&mut e, &[0, 1, 2, 3, 4, 5, 6, 7, 1]);
        let calls = constructor_double(&mut e);
        let shape = e.mem.alloc(0x60);
        // Entry 1, triangle 1 of the first strip: indices 1, 2, 3.
        let key = 0x0010_0001u32;
        let result: Ptr = e.call(0x00ca45b0, &args![this, key, shape]).ptr();
        assert_eq!(result.addr(), shape);
        assert_eq!(*calls.borrow(), vec![vec![shape, 0.25f32.to_bits()]]);
        // Vertex 1 = (1, 1.5, -1): times 2 times (1, 10, 100); lane 3 is 0 * 5.
        assert_eq!(e.mem.f32(shape + 0x20), 2.0);
        assert_eq!(e.mem.f32(shape + 0x24), 30.0);
        assert_eq!(e.mem.f32(shape + 0x28), -200.0);
        assert_eq!(e.mem.f32(shape + 0x2c), 0.0);
        // Vertex 3 = (3, 3.5, -3).
        assert_eq!(e.mem.f32(shape + 0x40), 6.0);
        assert_eq!(e.mem.f32(shape + 0x44), 70.0);
        assert_eq!(e.mem.f32(shape + 0x48), -600.0);
    }

    #[test]
    fn a_position_past_the_first_strip_continues_in_the_second() {
        let mut e = Engine::new();
        let this = setup(&mut e, &[0, 1, 2, 3, 4, 5, 6, 7, 1]);
        let _ = constructor_double(&mut e);
        let shape = e.mem.alloc(0x60);
        // Position 3 is past the 3 triangles of the strip of 5: triangle 0 of the next one
        // (indices 5, 6, 7).
        let key = 0x0010_0003u32;
        e.call(0x00ca45b0, &args![this, key, shape]);
        assert_eq!(e.mem.f32(shape + 0x20), 10.0);
        assert_eq!(e.mem.f32(shape + 0x30), 12.0);
        assert_eq!(e.mem.f32(shape + 0x40), 14.0);
    }

    #[test]
    fn repeated_indices_make_no_triangle() {
        let mut e = Engine::new();
        let this = setup(&mut e, &[0, 1, 1, 3, 4, 5, 6, 7, 1]);
        let calls = constructor_double(&mut e);
        let shape = e.mem.alloc(0x60);
        let key = 0x0010_0001u32;
        let result: Ptr = e.call(0x00ca45b0, &args![this, key, shape]).ptr();
        assert_eq!(result, Ptr::NULL);
        assert!(calls.borrow().is_empty());
        assert_eq!(e.mem.f32(shape + 0x20), 0.0);
    }
}
