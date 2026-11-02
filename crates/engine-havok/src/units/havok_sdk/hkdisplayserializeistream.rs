//! `Havok SDK/hkdisplayserializeistream.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkDisplaySerializeIStream` is an `hkIArchive`: every read is `hkIArchive::readArrayGeneric`
//! (`00dd8fe0`, `(this, buffer, element size, count)`), which swaps the bytes of a value when
//! the archive was written the other way round. The exe reads each value into a local of its
//! frame; the frames are rebuilt here at the offsets the exe uses, so a read that fails to write
//! leaves what the frame held (zeros here) and the scratch words the exe reuses keep the values
//! the exe pushed into them.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{load_vec, store_vec};

/// The memory router (`00c85750`): its field `+0x10` is the allocator the arrays and objects
/// come from; `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element
/// size)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkIArchive::readArrayGeneric` (`00dd8fe0`, `(this, buffer, element size, count)`).
const READ_ARRAY_GENERIC: u32 = 0x00dd_8fe0;
/// Makes the three rotation rows at `this` from the quaternion argument (`00cb2d90`).
const ROTATION_FROM_QUATERNION: u32 = 0x00cb_2d90;
/// Copies a transform (`00c6cd60`, `this` = the target, one word: the source).
const COPY_TRANSFORM: u32 = 0x00c6_cd60;
/// `hkGeometry`'s constructor (`00cce5e0`, `this` only; returns `this`).
const GEOMETRY_CONSTRUCT: u32 = 0x00cc_e5e0;
/// The constructors of the display shapes the stream describes: `hkDisplayConvex`, `hkDisplayBox`,
/// `hkDisplayAABB` (unit `hkdisplayaabb.cpp`), the cone and semicircle constructors (unnamed in
/// the engine map), `hkDisplayPlane`'s, `hkDisplayCapsule`, `hkDisplayCylinder` and
/// `hkDisplaySphere`.
const CONVEX_CONSTRUCT: u32 = 0x00de_1d20;
const BOX_CONSTRUCT: u32 = 0x00de_1f50;
const AABB_CONSTRUCT: u32 = 0x00de_1190;
const CONE_CONSTRUCT: u32 = 0x00de_53b0;
const SEMICIRCLE_CONSTRUCT: u32 = 0x00de_49b0;
const PLANE_CONSTRUCT: u32 = 0x00de_4f20;
const CAPSULE_CONSTRUCT: u32 = 0x00d2_2dc0;
const CYLINDER_CONSTRUCT: u32 = 0x00d5_8510;
const SPHERE_CONSTRUCT: u32 = 0x00e0_af50;

/// One `readArrayGeneric` of `count` values of `size` bytes into `buffer`.
fn read(e: &mut Engine, this: Ptr, buffer: u32, size: u32, count: u32) {
    e.call(READ_ARRAY_GENERIC, &args![this, buffer, size, count]);
}

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// A block of `size` bytes from the router's allocator (slot `4`).
fn allocate(e: &mut Engine, size: u32) -> u32 {
    let allocator = router_allocator(e);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// An object block as the shapes are made: `size` bytes with the 16-bit size kept at `+4`.
fn allocate_object(e: &mut Engine, size: u16) -> u32 {
    let object = allocate(e, u32::from(size));
    e.mem.set_u16(object + 4, size);
    object
}

/// Appends one 16-byte element to the `hkArray` at `array` the way the exe inlines it: the
/// router's allocator is asked for first; when the capacity (without its flag bits) is below the
/// size plus one, `_reserve` gets the larger of that and twice the capacity (signed); the size
/// goes up by one. Returns the address of the new element.
fn append_slot(e: &mut Engine, array: u32) -> u32 {
    let allocator = router_allocator(e);
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
    let counted = e.mem.u32(array + 4).wrapping_add(1);
    e.mem.set_u32(array + 4, counted);
    size.wrapping_mul(16).wrapping_add(e.mem.u32(array))
}

// Translated from 00ddac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`): reads three 4-byte values, one at a time,
/// into the words `out[0]`, `out[1]` and `out[2]`. The first goes through a scratch word that
/// holds `this` until it is read (the exe's `PUSH ECX`), the other two through the argument's
/// own stack slot, which holds the `out` pointer until it is read.
pub fn fn_00ddac00(e: &mut Engine, this: Ptr, out: Ptr) {
    let target = out.addr();
    e.with_stack(8, |e, scratch| {
        let first = scratch.addr();
        let second = first + 4;
        e.mem.set_u32(first, this.addr());
        e.mem.set_u32(second, target);
        read(e, this, first, 4, 1);
        let word = e.mem.u32(first);
        e.mem.set_u32(target, word);
        read(e, this, second, 4, 1);
        let word = e.mem.u32(second);
        e.mem.set_u32(target + 4, word);
        read(e, this, second, 4, 1);
        let word = e.mem.u32(second);
        e.mem.set_u32(target + 8, word);
    });
}

// Translated from 00ddac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySerializeIStream::readQuadVector4` (Xbox PDB), thiscall, `RET 4`: reads three
/// 4-byte values into `out` and sets its fourth lane to 0.0.
pub fn hk_display_serialize_i_stream_read_quad_vector4(e: &mut Engine, this: Ptr, out: Ptr) {
    read(e, this, out.addr(), 4, 3);
    e.mem.set_u32(out.addr() + 0xc, 0);
}

// Translated from 00ddac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySerializeIStream::readTransform` (Xbox PDB), thiscall, `RET 4`: reads the
/// translation (three values, the fourth lane 1.0) and a quaternion (four values) into the
/// frame, stores the translation as the transform's fourth row (`out + 0x30`) and makes the
/// rotation rows at `out` from the quaternion (`00cb2d90`).
pub fn hk_display_serialize_i_stream_read_transform(e: &mut Engine, this: Ptr, out: Ptr) {
    e.with_stack(0x50, |e, block| {
        let b = (block.addr() + 0xf) & !0xf;
        read(e, this, b + 0x10, 4, 3);
        e.mem.set_f32(b + 0x1c, 1.0);
        read(e, this, b + 0x20, 4, 4);
        let translation = load_vec(e, b + 0x10);
        store_vec(e, out.addr() + 0x30, translation);
        e.call(ROTATION_FROM_QUATERNION, &args![out, b + 0x20]);
    });
}

// Translated from 00ddacd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySerializeIStream::readGeometry` (Xbox PDB), thiscall, `RET 4`; `geometry` is the
/// `hkGeometry` block (vertices at `+0`, index records at `+0xc`). Reads a vertex count and,
/// when it is above zero, that many vertices appended to the first array
/// (`readQuadVector4` into each new element); then a record count and that many index records
/// appended to the second array (`00ddac00` reads three words into each). The count words are
/// read through a scratch word that holds `this` (the exe's `PUSH ECX`) and through the
/// argument's own slot, which holds `geometry`.
pub fn hk_display_serialize_i_stream_read_geometry(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let block = geometry.addr();
    e.with_stack(8, |e, scratch| {
        let vertex_count = scratch.addr();
        let record_count = vertex_count + 4;
        e.mem.set_u32(vertex_count, this.addr());
        e.mem.set_u32(record_count, block);
        read(e, this, vertex_count, 4, 1);
        let vertices = e.mem.i32(vertex_count);
        if vertices > 0 {
            for _ in 0..vertices {
                let slot = append_slot(e, block);
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(slot));
            }
        }
        read(e, this, record_count, 4, 1);
        let records = e.mem.i32(record_count);
        if records > 0 {
            for _ in 0..records {
                let slot = append_slot(e, block + 0xc);
                fn_00ddac00(e, this, Ptr::new(slot));
            }
        }
    });
}

/// Stores the new shape in `*out` and gives it the transform that was read (`00c6cd60` on
/// `shape + 0x10`).
fn finish_with_transform(e: &mut Engine, out: Ptr, shape: u32, transform: u32) {
    e.mem.set_u32(out.addr(), shape);
    e.call(COPY_TRANSFORM, &args![shape + 0x10, transform]);
}

/// A `float` the exe moves through the x87 stack on its way to a call (a signalling NaN is
/// quieted).
fn quiet_float(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.mem.f32(address))
}

// Translated from 00ddada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplaySerializeIStream::readDisplayGeometry` (Xbox PDB), thiscall, `RET 4`; `out`
/// receives the new shape. Reads one signed byte, the kind, and by it (1 to 9; any other kind
/// makes nothing and leaves `*out` alone) reads the shape's data into the frame and builds it
/// (every object block is `size` bytes from the router's allocator, the 16-bit size kept at `+4`):
///
/// - 1 sphere (0x80): a transform, a value `r`, a vector (its fourth lane becomes `r`, quieted)
///   and two integers `a` then `b`: `00e0af50(vector, a, b)`; the transform is copied to the shape.
/// - 2 box (0x70): a transform, a vector: `00de1f50(vector)`; with the transform.
/// - 3 AABB (0x80): two vectors `p` then `q`: `00de1190(p, q)`; no transform.
/// - 4 cone (0x90): two vectors `p`, `q`, two values `x`, `y` and a word `w`:
///   `00de53b0(x, y, w, q, p)` (the values through the x87 stack); no transform.
/// - 5 semicircle (0xa0): three vectors `p`, `q`, `s`, then four values `a`, `b`, `c`, `d` (the
///   last an integer): `00de49b0(p, q, s, b, c, a, d)` (`a`, `b`, `c` through the x87 stack); no
///   transform.
/// - 6 convex (0x60): a transform and an `hkGeometry` (a 0x18-byte block made by `00cce5e0`)
///   read with `readGeometry`: `00de1d20(geometry)`; with the transform.
/// - 7 plane (0xa0): three vectors `p`, `q`, `s` and a value `v`, spread over a vector `w` in
///   all four lanes: `00de4f20(q, s, p, w)`; no transform.
/// - 8 capsule (0x90): a transform, a value `r`, two vectors `p`, `q` and two words `a`, `b`:
///   `00d22dc0(p, q, r, a, b)`; with the transform.
/// - 9 cylinder (0x90): a transform, a value `r`, two vectors `p`, `q` and two words `a`, `b`:
///   `00d58510(p, q, r, a, b)`; with the transform.
///
/// A failing read leaves the frame word it targets as it was (zero here, stale in the exe).
pub fn hk_display_serialize_i_stream_read_display_geometry(e: &mut Engine, this: Ptr, out: Ptr) {
    e.with_stack(0xb0, |e, block| {
        let b = (block.addr() + 0xf) & !0xf;
        read(e, this, b + 0x17, 1, 1);
        let kind = i32::from(e.mem.i8(b + 0x17)).wrapping_sub(1) as u32;
        let transform = b + 0x50;
        match kind {
            5 => {
                hk_display_serialize_i_stream_read_transform(e, this, Ptr::new(transform));
                let geometry = allocate(e, 0x18);
                let geometry = if geometry != 0 {
                    e.call(GEOMETRY_CONSTRUCT, &args![geometry]).u32()
                } else {
                    0
                };
                hk_display_serialize_i_stream_read_geometry(e, this, Ptr::new(geometry));
                let shape = allocate_object(e, 0x60);
                let shape = e.call(CONVEX_CONSTRUCT, &args![shape, geometry]).u32();
                finish_with_transform(e, out, shape, transform);
            }
            0 => {
                hk_display_serialize_i_stream_read_transform(e, this, Ptr::new(transform));
                read(e, this, b + 0xc, 4, 1);
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                let radius = quiet_float(e, b + 0xc);
                let vector = load_vec(e, b + 0x20);
                store_vec(e, b + 0x30, vector);
                e.mem.set_f32(b + 0x3c, radius);
                read(e, this, b + 0x10, 4, 1);
                read(e, this, b + 0xc, 4, 1);
                let shape = allocate_object(e, 0x80);
                let (second, first) = (e.mem.u32(b + 0xc), e.mem.u32(b + 0x10));
                let shape = e
                    .call(SPHERE_CONSTRUCT, &args![shape, b + 0x30, first, second])
                    .u32();
                finish_with_transform(e, out, shape, transform);
            }
            1 => {
                hk_display_serialize_i_stream_read_transform(e, this, Ptr::new(transform));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                let shape = allocate_object(e, 0x70);
                let shape = e.call(BOX_CONSTRUCT, &args![shape, b + 0x20]).u32();
                finish_with_transform(e, out, shape, transform);
            }
            2 => {
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x30));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                let shape = allocate_object(e, 0x80);
                let shape = e
                    .call(AABB_CONSTRUCT, &args![shape, b + 0x30, b + 0x20])
                    .u32();
                e.mem.set_u32(out.addr(), shape);
            }
            3 => {
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x30));
                read(e, this, b + 0xc, 4, 1);
                read(e, this, b + 0x10, 4, 1);
                read(e, this, b + 0x18, 4, 1);
                let shape = allocate_object(e, 0x90);
                let (angle, scale) = (quiet_float(e, b + 0xc), quiet_float(e, b + 0x10));
                let sides = e.mem.u32(b + 0x18);
                let shape = e
                    .call(
                        CONE_CONSTRUCT,
                        &args![shape, angle, scale, sides, b + 0x30, b + 0x20],
                    )
                    .u32();
                e.mem.set_u32(out.addr(), shape);
            }
            4 => {
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x40));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x30));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                read(e, this, b + 0x10, 4, 1);
                read(e, this, b + 0x1c, 4, 1);
                read(e, this, b + 0xc, 4, 1);
                read(e, this, b + 0x18, 4, 1);
                let shape = allocate_object(e, 0xa0);
                let words = (
                    quiet_float(e, b + 0x1c),
                    quiet_float(e, b + 0xc),
                    quiet_float(e, b + 0x10),
                    e.mem.u32(b + 0x18),
                );
                let shape = e
                    .call(
                        SEMICIRCLE_CONSTRUCT,
                        &args![
                            shape,
                            b + 0x40,
                            b + 0x30,
                            b + 0x20,
                            words.0,
                            words.1,
                            words.2,
                            words.3
                        ],
                    )
                    .u32();
                e.mem.set_u32(out.addr(), shape);
            }
            6 => {
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x90));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x30));
                read(e, this, b + 0x1c, 4, 1);
                let value = quiet_float(e, b + 0x1c);
                e.mem.set_f32(b + 0x1c, value);
                store_vec(e, b + 0x40, [value; 4]);
                let shape = allocate_object(e, 0xa0);
                let shape = e
                    .call(
                        PLANE_CONSTRUCT,
                        &args![shape, b + 0x90, b + 0x30, b + 0x20, b + 0x40],
                    )
                    .u32();
                e.mem.set_u32(out.addr(), shape);
            }
            7 => {
                hk_display_serialize_i_stream_read_transform(e, this, Ptr::new(transform));
                read(e, this, b + 0x18, 4, 1);
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x30));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                read(e, this, b + 0xc, 4, 1);
                read(e, this, b + 0x10, 4, 1);
                let shape = allocate_object(e, 0x90);
                let radius = quiet_float(e, b + 0x18);
                let (first, second) = (e.mem.u32(b + 0xc), e.mem.u32(b + 0x10));
                let shape = e
                    .call(
                        CAPSULE_CONSTRUCT,
                        &args![shape, b + 0x30, b + 0x20, radius, first, second],
                    )
                    .u32();
                finish_with_transform(e, out, shape, transform);
            }
            8 => {
                hk_display_serialize_i_stream_read_transform(e, this, Ptr::new(transform));
                read(e, this, b + 0xc, 4, 1);
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x30));
                hk_display_serialize_i_stream_read_quad_vector4(e, this, Ptr::new(b + 0x20));
                read(e, this, b + 0x10, 4, 1);
                read(e, this, b + 0x18, 4, 1);
                let shape = allocate_object(e, 0x90);
                let radius = quiet_float(e, b + 0xc);
                let (first, second) = (e.mem.u32(b + 0x10), e.mem.u32(b + 0x18));
                let shape = e
                    .call(
                        CYLINDER_CONSTRUCT,
                        &args![shape, b + 0x30, b + 0x20, radius, first, second],
                    )
                    .u32();
                finish_with_transform(e, out, shape, transform);
            }
            _ => {}
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_ac00, fn_00ddac00(Ptr, Ptr)),
        entry!(
            0x00dd_ac60,
            hk_display_serialize_i_stream_read_quad_vector4(Ptr, Ptr)
        ),
        entry!(
            0x00dd_ac80,
            hk_display_serialize_i_stream_read_transform(Ptr, Ptr)
        ),
        entry!(
            0x00dd_acd0,
            hk_display_serialize_i_stream_read_geometry(Ptr, Ptr)
        ),
        entry!(
            0x00dd_ada0,
            hk_display_serialize_i_stream_read_display_geometry(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Stream = Rc<RefCell<VecDeque<u8>>>;
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    const ALLOCATE_DOUBLE: u32 = 0x4a00_0004;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The reader double: `readArrayGeneric` copies `size * count` bytes from the queue
    /// into the buffer (nothing once the queue is empty).
    fn install_reader(e: &mut Engine, bytes: &[u8]) -> Stream {
        let stream: Stream = Rc::new(RefCell::new(bytes.iter().copied().collect()));
        let source = Rc::clone(&stream);
        e.register_double(READ_ARRAY_GENERIC, move |e, a| {
            for i in 0..a[2].wrapping_mul(a[3]) {
                let Some(byte) = source.borrow_mut().pop_front() else {
                    break;
                };
                e.mem.set_u8(a[1] + i, byte);
            }
            Ret::default()
        });
        stream
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn floats(values: &[f32]) -> Vec<u8> {
        values
            .iter()
            .flat_map(|v| v.to_bits().to_le_bytes())
            .collect()
    }

    /// A 16-byte aligned block of `size` bytes.
    fn aligned(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    /// The router (slot 4 hands out blocks) and a real `_reserve` that moves the elements.
    fn install_arrays(e: &mut Engine) {
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
    }

    #[test]
    fn the_word_reader_fills_three_words() {
        let mut e = Engine::new();
        install_reader(&mut e, &words(&[1, 2, 3]));
        let out = e.mem.alloc(0x10);
        e.call(0x00dd_ac00, &args![0x1000u32, out]);
        let got: Vec<u32> = (0..3).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(got, vec![1, 2, 3]);
    }

    #[test]
    fn the_word_reader_keeps_its_scratch_values_when_a_read_writes_nothing() {
        let mut e = Engine::new();
        install_reader(&mut e, &[]);
        let out = e.mem.alloc(0x10);
        e.call(0x00dd_ac00, &args![0x1000u32, out]);
        // The first word is the pushed `this`; the others the argument slot's own value.
        let got: Vec<u32> = (0..3).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(got, vec![0x1000, out, out]);
    }

    #[test]
    fn read_quad_vector4_reads_three_floats_and_clears_the_fourth() {
        let mut e = Engine::new();
        let stream = install_reader(&mut e, &floats(&[1.0, 2.0, 3.0]));
        let out = e.mem.alloc(0x10);
        e.mem.set_f32(out + 12, 9.0);
        e.call_log = Some(vec![]);
        e.call(0x00dd_ac60, &args![0x1000u32, out]);
        assert_eq!(
            calls_to(&e, READ_ARRAY_GENERIC),
            vec![vec![0x1000, out, 4, 3]]
        );
        let got: Vec<f32> = (0..4).map(|i| e.mem.f32(out + 4 * i)).collect();
        assert_eq!(got, vec![1.0, 2.0, 3.0, 0.0]);
        assert!(stream.borrow().is_empty());
    }

    #[test]
    fn read_transform_reads_the_translation_and_the_rotation_quaternion() {
        let mut e = Engine::new();
        install_reader(&mut e, &floats(&[5.0, 6.0, 7.0, 0.1, 0.2, 0.3, 0.4]));
        // The rotation double records the quaternion it is given and the target.
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(ROTATION_FROM_QUATERNION, move |e, a| {
            let words = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push(words);
            log.borrow_mut().push(vec![a[0]]);
            Ret::default()
        });
        let out = aligned(&mut e, 0x40);
        e.call(0x00dd_ac80, &args![0x1000u32, out]);
        let translation: Vec<f32> = (0..4).map(|i| e.mem.f32(out + 0x30 + 4 * i)).collect();
        assert_eq!(translation, vec![5.0, 6.0, 7.0, 1.0]);
        let seen = seen.borrow();
        let quaternion: Vec<f32> = seen[0].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(quaternion, vec![0.1, 0.2, 0.3, 0.4]);
        assert_eq!(seen[1], vec![out]);
    }

    #[test]
    fn read_geometry_appends_the_vertices_and_the_index_records() {
        let mut e = Engine::new();
        install_arrays(&mut e);
        let mut bytes = words(&[2]);
        bytes.extend(floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bytes.extend(words(&[1, 7, 8, 9]));
        install_reader(&mut e, &bytes);
        let geometry = e.mem.alloc(0x18);
        for array in [0, 0xc] {
            e.mem.set_u32(geometry + array + 8, 0x8000_0000);
        }
        e.call(0x00dd_acd0, &args![0x1000u32, geometry]);
        assert_eq!(e.mem.u32(geometry + 4), 2);
        let vertices = e.mem.u32(geometry);
        let got: Vec<f32> = (0..8).map(|i| e.mem.f32(vertices + 4 * i)).collect();
        assert_eq!(got, vec![1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0]);
        assert_eq!(e.mem.u32(geometry + 0x10), 1);
        let records = e.mem.u32(geometry + 0xc);
        let got: Vec<u32> = (0..3).map(|i| e.mem.u32(records + 4 * i)).collect();
        assert_eq!(got, vec![7, 8, 9]);
    }

    #[test]
    fn read_geometry_skips_empty_and_negative_counts() {
        let mut e = Engine::new();
        install_arrays(&mut e);
        let mut bytes = words(&[0]);
        bytes.extend(words(&[0xffff_ffff]));
        install_reader(&mut e, &bytes);
        let geometry = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00dd_acd0, &args![0x1000u32, geometry]);
        assert_eq!(e.mem.u32(geometry + 4), 0);
        assert_eq!(e.mem.u32(geometry + 0x10), 0);
        assert_eq!(calls_to(&e, READ_ARRAY_GENERIC).len(), 2);
    }

    /// The doubles of every constructor the display-shape reader calls: each returns its first
    /// word (`this`) and records its address, the words it was given and, for every argument
    /// index in `vectors`, the four words the argument points to at that moment.
    fn install_constructors(e: &mut Engine, vectors: &'static [usize]) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        e.register(GEOMETRY_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(COPY_TRANSFORM, |_, _| Ret::default());
        e.register(ROTATION_FROM_QUATERNION, |_, _| Ret::default());
        for address in [
            CONVEX_CONSTRUCT,
            BOX_CONSTRUCT,
            AABB_CONSTRUCT,
            CONE_CONSTRUCT,
            SEMICIRCLE_CONSTRUCT,
            PLANE_CONSTRUCT,
            CAPSULE_CONSTRUCT,
            CYLINDER_CONSTRUCT,
            SPHERE_CONSTRUCT,
        ] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |e, a| {
                let mut record = vec![address];
                record.extend_from_slice(a);
                for index in vectors {
                    record.extend((0..4).map(|lane| e.mem.u32(a[*index] + 4 * lane)));
                }
                log.borrow_mut().push(record);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        seen
    }

    /// The seven values of a transform in the stream: translation and quaternion.
    fn transform_bytes() -> Vec<u8> {
        floats(&[1.0, 2.0, 3.0, 0.1, 0.2, 0.3, 0.4])
    }

    /// Runs `readDisplayGeometry` on `bytes`, returning the shape stored (0 when nothing was).
    fn run(e: &mut Engine, bytes: Vec<u8>) -> u32 {
        install_reader(e, &bytes);
        let out = e.mem.alloc(8);
        e.call(0x00dd_ada0, &args![0x1000u32, out]);
        e.mem.u32(out)
    }

    fn display_engine(vectors: &'static [usize]) -> (Engine, Seen) {
        let mut e = Engine::new();
        install_arrays(&mut e);
        let seen = install_constructors(&mut e, vectors);
        e.call_log = Some(vec![]);
        (e, seen)
    }

    fn f(bits: u32) -> f32 {
        f32::from_bits(bits)
    }

    /// The four floats at `record[start..start + 4]`.
    fn lanes(record: &[u32], start: usize) -> Vec<f32> {
        record[start..start + 4].iter().map(|b| f(*b)).collect()
    }

    #[test]
    fn a_sphere_is_a_transform_a_radius_a_vector_and_two_words() {
        let (mut e, seen) = display_engine(&[1]);
        let mut bytes = vec![1u8];
        bytes.extend(transform_bytes());
        bytes.extend(floats(&[2.5]));
        bytes.extend(floats(&[7.0, 8.0, 9.0]));
        bytes.extend(words(&[11, 12]));
        let shape = run(&mut e, bytes);
        assert_ne!(shape, 0);
        assert_eq!(e.mem.u16(shape + 4), 0x80);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], SPHERE_CONSTRUCT);
        // (shape, vector, a, b) then the vector's four floats: the radius is its fourth lane.
        assert_eq!(record[1], shape);
        assert_eq!(record[3..5], [11, 12]);
        assert_eq!(lanes(&record, 5), vec![7.0, 8.0, 9.0, 2.5]);
        let copy = calls_to(&e, COPY_TRANSFORM);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0][0], shape + 0x10);
    }

    #[test]
    fn a_box_is_a_transform_and_a_vector() {
        let (mut e, seen) = display_engine(&[1]);
        let mut bytes = vec![2u8];
        bytes.extend(transform_bytes());
        bytes.extend(floats(&[7.0, 8.0, 9.0]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0x70);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], BOX_CONSTRUCT);
        assert_eq!(lanes(&record, 3), vec![7.0, 8.0, 9.0, 0.0]);
        assert_eq!(calls_to(&e, COPY_TRANSFORM).len(), 1);
    }

    #[test]
    fn an_aabb_is_two_vectors_and_has_no_transform() {
        let (mut e, seen) = display_engine(&[1, 2]);
        let mut bytes = vec![3u8];
        bytes.extend(floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0x80);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], AABB_CONSTRUCT);
        assert_eq!(lanes(&record, 4), vec![1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lanes(&record, 8), vec![4.0, 5.0, 6.0, 0.0]);
        assert!(calls_to(&e, COPY_TRANSFORM).is_empty());
    }

    #[test]
    fn a_cone_is_two_vectors_two_values_and_a_word() {
        let (mut e, seen) = display_engine(&[4, 5]);
        let mut bytes = vec![4u8];
        bytes.extend(floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bytes.extend(floats(&[0.5, 0.25]));
        bytes.extend(words(&[16]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0x90);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], CONE_CONSTRUCT);
        // (shape, x, y, sides, q, p): the first vector read is the last argument.
        assert_eq!((f(record[2]), f(record[3]), record[4]), (0.5, 0.25, 16));
        assert_eq!(lanes(&record, 7), vec![4.0, 5.0, 6.0, 0.0]);
        assert_eq!(lanes(&record, 11), vec![1.0, 2.0, 3.0, 0.0]);
        assert!(calls_to(&e, COPY_TRANSFORM).is_empty());
    }

    #[test]
    fn a_semicircle_is_three_vectors_three_values_and_a_word() {
        let (mut e, seen) = display_engine(&[1, 2, 3]);
        let mut bytes = vec![5u8];
        bytes.extend(floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]));
        // The values a, b, c, then the word d.
        bytes.extend(floats(&[0.1, 0.2, 0.3]));
        bytes.extend(words(&[24]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0xa0);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], SEMICIRCLE_CONSTRUCT);
        // The constructor gets (p, q, s, b, c, a, d).
        assert_eq!(f(record[5]), 0.2);
        assert_eq!(f(record[6]), 0.3);
        assert_eq!(f(record[7]), 0.1);
        assert_eq!(record[8], 24);
        assert_eq!(lanes(&record, 9), vec![1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lanes(&record, 13), vec![4.0, 5.0, 6.0, 0.0]);
        assert_eq!(lanes(&record, 17), vec![7.0, 8.0, 9.0, 0.0]);
    }

    #[test]
    fn a_convex_shape_reads_its_geometry_first() {
        let (mut e, seen) = display_engine(&[]);
        let mut bytes = vec![6u8];
        bytes.extend(transform_bytes());
        bytes.extend(words(&[1]));
        bytes.extend(floats(&[1.0, 2.0, 3.0]));
        bytes.extend(words(&[1, 0, 1, 2]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0x60);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], CONVEX_CONSTRUCT);
        let geometry = record[2];
        assert_eq!(e.mem.u32(geometry + 4), 1);
        assert_eq!(e.mem.u32(geometry + 0x10), 1);
        assert_eq!(calls_to(&e, GEOMETRY_CONSTRUCT), vec![vec![geometry]]);
        assert_eq!(calls_to(&e, COPY_TRANSFORM).len(), 1);
    }

    #[test]
    fn a_plane_spreads_its_value_over_a_vector() {
        let (mut e, seen) = display_engine(&[1, 2, 3, 4]);
        let mut bytes = vec![7u8];
        bytes.extend(floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 0.75]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0xa0);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], PLANE_CONSTRUCT);
        // (q, s, p, w): the vectors read second, third and first, and the spread value.
        assert_eq!(lanes(&record, 6), vec![4.0, 5.0, 6.0, 0.0]);
        assert_eq!(lanes(&record, 10), vec![7.0, 8.0, 9.0, 0.0]);
        assert_eq!(lanes(&record, 14), vec![1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lanes(&record, 18), vec![0.75; 4]);
    }

    #[test]
    fn a_capsule_and_a_cylinder_read_a_transform_a_radius_two_vectors_and_two_words() {
        let (mut e, seen) = display_engine(&[1, 2]);
        let mut bytes = vec![8u8];
        bytes.extend(transform_bytes());
        bytes.extend(floats(&[0.5]));
        bytes.extend(floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bytes.extend(words(&[10, 20]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0x90);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], CAPSULE_CONSTRUCT);
        // (shape, p, q, r, a, b): p is the first vector read.
        assert_eq!(f(record[4]), 0.5);
        assert_eq!(record[5..7], [10, 20]);
        assert_eq!(lanes(&record, 7), vec![1.0, 2.0, 3.0, 0.0]);
        assert_eq!(lanes(&record, 11), vec![4.0, 5.0, 6.0, 0.0]);
        assert_eq!(calls_to(&e, COPY_TRANSFORM).len(), 1);

        let (mut e, seen) = display_engine(&[1, 2]);
        let mut bytes = vec![9u8];
        bytes.extend(transform_bytes());
        bytes.extend(floats(&[0.5]));
        bytes.extend(floats(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        bytes.extend(words(&[10, 20]));
        let shape = run(&mut e, bytes);
        assert_eq!(e.mem.u16(shape + 4), 0x90);
        let record = seen.borrow()[0].clone();
        assert_eq!(record[0], CYLINDER_CONSTRUCT);
        assert_eq!(f(record[4]), 0.5);
        assert_eq!(record[5..7], [10, 20]);
        assert_eq!(calls_to(&e, COPY_TRANSFORM).len(), 1);
    }

    #[test]
    fn an_unknown_kind_makes_nothing() {
        for kind in [0u8, 10, 0xff] {
            let (mut e, _) = display_engine(&[]);
            install_reader(&mut e, &[kind]);
            let out = e.mem.alloc(8);
            e.mem.set_u32(out, 0x1234_5678);
            e.call(0x00dd_ada0, &args![0x1000u32, out]);
            assert_eq!(e.mem.u32(out), 0x1234_5678, "kind {kind}");
            assert!(calls_to(&e, COPY_TRANSFORM).is_empty());
        }
    }
}
