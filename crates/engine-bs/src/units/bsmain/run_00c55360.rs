//! `BSMain/run_00c55360` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, `RET 4`);
/// `NiNew` (`00aa13e0`, `cdecl`: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiTriShape::NiTriShape` (`00a74410`, engine map; `thiscall`, `RET 0x24`: vertex count,
/// vertices, normals, colours, texture coordinates, two words, triangle count, indices).
const TRI_SHAPE_CONSTRUCTOR: u32 = 0x00a7_4410;
/// The global `NiPoint3` used as the centre when none is given (`011f426c`) and the normal
/// every vertex gets (`011a9484`, three words).
const ZERO_POINT: u32 = 0x011f_426c;
const QUAD_NORMAL: u32 = 0x011a_9484;
/// The shape's virtual slot `+0xE8` (no arguments) run on the new shape.
const SLOT_AFTER_CONSTRUCTION: u32 = 0xe8;

/// Allocates `size` bytes from the memory manager.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32()
}

// Translated from 00c55360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c55360 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f655, state variable) is not translated
/// Makes a flat square shape (unnamed in the engine map; `cdecl`: half the side as a
/// `float`, the centre or null for `011f426c`, a colour or null). Four vertices (from the
/// memory manager: 0x30 bytes of vertices, 0x30 of normals, 0x40 of colours, zeroed, only
/// with a colour, 0x20 of texture coordinates, 0xC of indices): (cx - h, cy + h, 0) with
/// texture coordinate (0, 0), (h + cx, h + cy, 0) with (1, 0), (cx - h, cy - h, 0) with (0, 1)
/// and (h + cx, cy - h, 0) with (1, 1) (x87 sums stored as `float`s); every normal is the
/// three words at `011a9484` and every colour the four words given; the triangles are
/// (0, 1, 2) and (2, 1, 3). A 0xC4-byte `NiNew` block becomes the `NiTriShape` (`00a74410`
/// with 4, the arrays, 0, 0, 2 and the indices; null when the allocation fails), whose
/// virtual `+0xE8` runs (also on null, which faults). Answers the shape.
pub fn fn_00c55360(e: &mut Engine, half: f32, center: Ptr, color: Ptr) -> u32 {
    let center = if center.is_null() {
        ZERO_POINT
    } else {
        center.addr()
    };
    let center_x = e.mem.u32(center);
    let center_y = e.mem.u32(center + 4);
    let vertices = allocate(e, 0x30);
    let normals = allocate(e, 0x30);
    let colors = if color.is_null() {
        0
    } else {
        let block = allocate(e, 0x40);
        if block != 0 {
            for k in 0..0x10u32 {
                e.mem.set_f32(block + 4 * k, 0.0);
            }
        }
        block
    };
    let coordinates = allocate(e, 0x20);
    let indices = allocate(e, 0xc);
    let h = x87_load32(half.to_bits());
    let minus = |coordinate: u32| x87_store32(x87_sub(x87_m32(coordinate), h));
    let plus = |coordinate: u32| x87_store32(x87_add(h, x87_m32(coordinate)));
    let corners = [
        (minus(center_x), plus(center_y), 0.0f32, 0.0f32),
        (plus(center_x), plus(center_y), 1.0, 0.0),
        (minus(center_x), minus(center_y), 0.0, 1.0),
        (plus(center_x), minus(center_y), 1.0, 1.0),
    ];
    for (i, (x, y, u, v)) in corners.iter().enumerate() {
        let i = i as u32;
        let vertex = vertices + 0xc * i;
        e.mem.set_u32(vertex, *x);
        e.mem.set_u32(vertex + 4, *y);
        e.mem.set_f32(vertex + 8, 0.0);
        for k in 0..3u32 {
            let word = e.mem.u32(QUAD_NORMAL + 4 * k);
            e.mem.set_u32(normals + 0xc * i + 4 * k, word);
        }
        if !color.is_null() {
            for k in 0..4u32 {
                let word = e.mem.u32(color.addr() + 4 * k);
                e.mem.set_u32(colors + 0x10 * i + 4 * k, word);
            }
        }
        e.mem.set_f32(coordinates + 8 * i, *u);
        e.mem.set_f32(coordinates + 8 * i + 4, *v);
    }
    for (k, index) in [0u16, 1, 2, 2, 1, 3].iter().enumerate() {
        e.mem.set_u16(indices + 2 * k as u32, *index);
    }
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    let shape = if block == 0 {
        0
    } else {
        e.call(
            TRI_SHAPE_CONSTRUCTOR,
            &args![
                block,
                4u32,
                vertices,
                normals,
                colors,
                coordinates,
                0u32,
                0u32,
                2u32,
                indices
            ],
        )
        .u32()
    };
    e.vcall(shape, SLOT_AFTER_CONSTRUCTION, &args![]);
    shape
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c5_5360, fn_00c55360(f32, Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn the_square_has_four_corners_and_two_triangles() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            e.mem.write(block, &vec![0xcc; a[1] as usize]);
            block.into_ret()
        });
        e.map(QUAD_NORMAL, 12);
        e.mem.set_f32(QUAD_NORMAL + 8, 1.0);
        e.map(ZERO_POINT, 12);
        let shape = e.mem.alloc(8);
        e.register_double(NI_NEW, move |_, _| 0x4000u32.into_ret());
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = seen.clone();
        e.register_double(TRI_SHAPE_CONSTRUCTOR, move |_, a| {
            inner.borrow_mut().push(a.to_vec());
            shape.into_ret()
        });
        e.put_vtable(0x0300_b000, &[0; 0x3b]);
        e.mem.set_u32(0x0300_b000 + 0xe8, 0x00ee_e000);
        e.mem.set_u32(shape, 0x0300_b000);
        e.register(0x00ee_e000, |_, _| Ret::default());
        let center = e.mem.alloc(12);
        e.mem.set_f32(center, 10.0);
        e.mem.set_f32(center + 4, 20.0);
        let color = e.mem.alloc(16);
        e.mem.set_f32(color + 12, 0.5);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c5_5360, &args![2.0f32, center, color]).u32(),
            shape
        );
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x00ee_e000, vec![shape])));
        let a = seen.borrow()[0].clone();
        assert_eq!((a[0], a[1], a[6], a[7], a[8]), (0x4000, 4, 0, 0, 2));
        let f = |e: &Engine, base: u32, k: u32| e.mem.f32(base + 4 * k);
        let vertices: Vec<f32> = (0..12).map(|k| f(&e, a[2], k)).collect();
        assert_eq!(
            vertices,
            vec![8.0, 22.0, 0.0, 12.0, 22.0, 0.0, 8.0, 18.0, 0.0, 12.0, 18.0, 0.0]
        );
        assert_eq!(f(&e, a[3], 11), 1.0);
        assert_eq!(f(&e, a[4], 15), 0.5);
        let coordinates: Vec<f32> = (0..8).map(|k| f(&e, a[5], k)).collect();
        assert_eq!(coordinates, vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        let indices: Vec<u16> = (0..6).map(|k| e.mem.u16(a[9] + 2 * k)).collect();
        assert_eq!(indices, vec![0, 1, 2, 2, 1, 3]);
        // Without a centre or colour: the zero point, no colours.
        seen.borrow_mut().clear();
        e.call(0x00c5_5360, &args![1.0f32, 0u32, 0u32]);
        let a = seen.borrow()[0].clone();
        assert_eq!(a[4], 0);
        assert_eq!(e.mem.f32(a[2]), -1.0);
    }
}
