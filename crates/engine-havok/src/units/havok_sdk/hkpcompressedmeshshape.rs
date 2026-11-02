//! `Havok SDK/hkpcompressedmeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (cdecl: allocator, array, capacity, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkVector4::setTransformedPos` (`RET 8`: transform, position; `this` = the result).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// The float at `01011d78` (0.0) the vertex's fourth lane is made from.
const LANE_W: u32 = 0x0101_1d78;

// Translated from 00e3d9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCompressedMeshShape::ConvexPiece::getVertices` (Xbox PDB): decodes the piece's
/// 16-bit vertex coordinates into `out`, an `hkArray` of 16-byte vectors (data `+0`, size
/// `+4`, capacity and flags `+8`). The piece holds an offset vector at `+0`, the `u16`
/// coordinate array at `+0x10` and the coordinate count (three per vertex) at `+0x14`.
/// The array size becomes `count / 3` (signed, truncated); when its capacity (low 30 bits)
/// is below that, `hkArrayUtil::_reserve` (`00c908c0`) is asked, through the memory router's
/// allocator, for the larger of twice the capacity and the size, with 16-byte elements. Then
/// for each vertex `(x, y, z)` the vector `(x, y, z, 0.0) * scale + offset` (single
/// precision, lane by lane) is moved by the transform with `hkVector4::setTransformedPos`
/// (`00c7f640`, in place) and stored at index `vertex`.
pub fn hkp_compressed_mesh_shape_convex_piece_get_vertices(
    e: &mut Engine,
    this: Ptr,
    scale: f32,
    transform: Ptr,
    out: Ptr,
) {
    let piece = this.addr();
    let array = out.addr();
    let size = (e.mem.u32(piece + 0x14) as i32) / 3;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if size < doubled { doubled } else { size };
        e.call(ARRAY_RESERVE, &args![allocator, out, wanted, 0x10u32]);
    }
    e.mem.set_u32(array + 4, size as u32);
    if (e.mem.u32(piece + 0x14) as i32) <= 0 {
        return;
    }
    e.with_stack(0x10, |e, vertex| {
        let mut index = 0i32;
        loop {
            let coordinates = e.mem.u32(piece + 0x10);
            let at = coordinates.wrapping_add((index as u32).wrapping_mul(2));
            let x = f32::from(e.mem.u16(at));
            let y = f32::from(e.mem.u16(at.wrapping_add(2)));
            let z = f32::from(e.mem.u16(at.wrapping_add(4)));
            let w = e.mem.f32(LANE_W);
            for (lane, value) in [x, y, z, w].into_iter().enumerate() {
                let offset = e.mem.f32(piece + 4 * lane as u32);
                e.mem
                    .set_f32(vertex.addr() + 4 * lane as u32, value * scale + offset);
            }
            e.call(SET_TRANSFORMED_POS, &args![vertex, transform, vertex]);
            let data = e.mem.u32(array);
            let target = data.wrapping_add(((index / 3) as u32).wrapping_mul(16));
            for lane in 0..4 {
                let word = e.mem.u32(vertex.addr() + 4 * lane);
                e.mem.set_u32(target + 4 * lane, word);
            }
            index += 3;
            if index >= (e.mem.u32(piece + 0x14) as i32) {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e3_d9d0,
        hkp_compressed_mesh_shape_convex_piece_get_vertices(Ptr, f32, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };

    const ADD: f32 = 100.0;

    /// A piece with offset `(1, 2, 3, 4)` and the given 16-bit coordinates, an empty output
    /// array with capacity word `capacity`, a memory router whose allocator is `0x5151`, a
    /// reserve double that grows the array to the asked capacity and a transform double that
    /// adds 100 to the three first lanes of the vector it is given. Returns
    /// `(engine, log, piece, array, transform)`.
    fn setup(coordinates: &[u16], capacity: u32) -> (Engine, Log, u32, u32, u32) {
        let mut e = engine();
        e.mem.set_f32(LANE_W, 0.0);
        let log = new_log();
        let piece = e.mem.alloc(0x20);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(piece + 4 * lane as u32, value);
        }
        let data = e.mem.alloc(0x40);
        for (i, value) in coordinates.iter().enumerate() {
            e.mem.set_u16(data + 2 * i as u32, *value);
        }
        e.mem.set_u32(piece + 0x10, data);
        e.mem.set_u32(piece + 0x14, coordinates.len() as u32);
        let array = e.mem.alloc(0x10);
        let storage = e.mem.alloc(0x200);
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 8, capacity);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x5151);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        let seen = log.clone();
        e.register_double(ARRAY_RESERVE, move |e, a| {
            seen.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let storage = e.mem.alloc(0x200);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let seen = log.clone();
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            seen.borrow_mut().push((SET_TRANSFORMED_POS, a.to_vec()));
            for lane in 0..3 {
                let value = e.mem.f32(a[2] + 4 * lane) + ADD;
                e.mem.set_f32(a[0] + 4 * lane, value);
            }
            Ret::default()
        });
        let transform = e.mem.alloc(0x40);
        (e, log, piece, array, transform)
    }

    fn vector(e: &Engine, array: u32, index: u32) -> Vec<f32> {
        let data = e.mem.u32(array);
        (0..4)
            .map(|i| e.mem.f32(data + 16 * index + 4 * i))
            .collect()
    }

    #[test]
    fn decodes_scales_offsets_and_transforms_each_vertex_00e3d9d0() {
        let (mut e, log, piece, array, transform) = setup(&[2, 4, 6, 8, 10, 12], 0);
        e.call(0x00e3_d9d0, &args![piece, 0.5f32, transform, array]);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(vector(&e, array, 0), vec![102.0, 104.0, 106.0, 4.0]);
        assert_eq!(vector(&e, array, 1), vec![105.0, 107.0, 109.0, 4.0]);
        let moves = calls(&log, SET_TRANSFORMED_POS);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0][1], transform);
        assert_eq!(moves[0][0], moves[0][2]);
    }

    #[test]
    fn reserves_the_larger_of_twice_the_capacity_and_the_size_00e3d9d0() {
        // (capacity word, coordinate count) -> the capacity asked for, or none.
        let cases: [(u32, usize, Option<u32>); 6] = [
            (0, 6, Some(2)),
            (0x8000_0002, 15, Some(5)),
            (3, 12, Some(6)),
            (4, 12, None),
            (0xc000_0005, 12, None),
            (0, 0, None),
        ];
        for (capacity, count, asked) in cases {
            let coordinates = vec![0u16; count];
            let (mut e, log, piece, array, transform) = setup(&coordinates, capacity);
            e.call(0x00e3_d9d0, &args![piece, 1.0f32, transform, array]);
            let reserve = calls(&log, ARRAY_RESERVE);
            match asked {
                Some(wanted) => assert_eq!(reserve, vec![vec![0x5151, array, wanted, 0x10]]),
                None => assert!(reserve.is_empty()),
            }
            assert_eq!(e.mem.u32(array + 4), (count / 3) as u32);
        }
    }
}
