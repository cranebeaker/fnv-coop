//! `NiMain/nitristripsdata_xenon.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriStripsData` fields (Xbox PDB, PC offsets): `NiGeometryData::m_usVertices` +0x08,
/// `m_pkVertex` +0x20 and `m_pkNormal` +0x24 (twelve bytes per entry), then
/// `m_usStrips` +0x44, `m_pusStripLengths` +0x48 and `m_pusStripLists` +0x4C.
const VERTEX_COUNT: u32 = 0x08;
const VERTICES: u32 = 0x20;
const NORMALS: u32 = 0x24;
const STRIPS: u32 = 0x44;
const STRIP_LENGTHS: u32 = 0x48;
const STRIP_LISTS: u32 = 0x4c;
/// `NiGeometryData::CreateNormals` (`00a669f0`, thiscall, one word) and `MarkAsChanged`
/// (`00a67090`, thiscall, one word); `NiPoint3::UnitizeVectors` (`00a7e960`, cdecl: the
/// array, the count, the stride).
const CREATE_NORMALS: u32 = 0x00a6_69f0;
const MARK_AS_CHANGED: u32 = 0x00a6_7090;
const UNITIZE_VECTORS: u32 = 0x00a7_e960;
/// `00525340` (unnamed in the engine map; cdecl, the address of three floats): scales the
/// point by a table-based inverse of its length.
const SCALE_TO_UNIT_00525340: u32 = 0x0052_5340;

/// The bits of `minuend - subtrahend` for two `float` words, as `FLD`, `FSUB dword`,
/// `FSTP dword` compute it.
fn float_difference(minuend: u32, subtrahend: u32) -> u32 {
    x87_store32(x87_sub(x87_load32(minuend), x87_m32(subtrahend)))
}

// Translated from 00a8ab40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsData::CalculateNormals` (Xbox PDB; thiscall, no arguments): makes sure there
/// are normals (`CreateNormals(1)`), then for each triangle of each strip (indices `j - 2`,
/// `j - 1`, `j` of the strip, the last two swapped on odd `j`) whose three indices differ,
/// takes the cross product of the edges `v1 - v0` and `v2 - v1` (in a local, each float
/// stored), scales it with `00525340` and adds it to the three normals; finally
/// `NiPoint3::UnitizeVectors` on all the normals and `MarkAsChanged(2)`. x87 arithmetic in
/// the code order and operand order.
pub fn ni_tri_strips_data_calculate_normals(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(CREATE_NORMALS, &args![this, 1u32]);
    let mut cursor = e.mem.u32(base + STRIP_LISTS);
    let mut strip = 0u32;
    while (strip as u16) < e.mem.u16(base + STRIPS) {
        let length_at = u32::from(strip as u16) * 2;
        let mut j = 2u32;
        while (j as u16)
            < e.mem
                .u16(e.mem.u32(base + STRIP_LENGTHS).wrapping_add(length_at))
        {
            let index = |e: &Engine, back: u32| {
                let at = cursor
                    .wrapping_add(u32::from(j as u16) * 2)
                    .wrapping_sub(back * 2);
                e.mem.u16(at)
            };
            let first = index(e, 2);
            let (second, third) = if j & 1 == 0 {
                (index(e, 1), index(e, 0))
            } else {
                (index(e, 0), index(e, 1))
            };
            if first != second && second != third && third != first {
                add_triangle_normal(e, base, first, second, third);
            }
            j = j.wrapping_add(1);
        }
        let length = u32::from(
            e.mem
                .u16(e.mem.u32(base + STRIP_LENGTHS).wrapping_add(length_at)),
        );
        strip = strip.wrapping_add(1);
        cursor = cursor.wrapping_add(length * 2);
    }
    let normals = e.mem.u32(base + NORMALS);
    let count = u32::from(e.mem.u16(base + VERTEX_COUNT));
    e.call(UNITIZE_VECTORS, &args![normals, count, 0xcu32]);
    e.call(MARK_AS_CHANGED, &args![this, 2u32]);
}

/// The body of [`ni_tri_strips_data_calculate_normals`] for one triangle of distinct
/// indices: the cross product, its scaling and the three normal sums.
fn add_triangle_normal(e: &mut Engine, base: u32, first: u16, second: u16, third: u16) {
    let vertices = e.mem.u32(base + VERTICES);
    let v = |e: &Engine, index: u16, axis: u32| {
        e.mem
            .u32(vertices.wrapping_add(u32::from(index) * 12 + axis * 4))
    };
    // a = v1 - v0, b = v2 - v1, each component stored as a float.
    let a: Vec<u32> = (0..3)
        .map(|axis| float_difference(v(e, second, axis), v(e, first, axis)))
        .collect();
    let b: Vec<u32> = (0..3)
        .map(|axis| float_difference(v(e, third, axis), v(e, second, axis)))
        .collect();
    let (ax, ay, az) = (x87_load32(a[0]), x87_load32(a[1]), x87_load32(a[2]));
    let (bx, by, bz) = (x87_load32(b[0]), x87_load32(b[1]), x87_load32(b[2]));
    e.with_stack(12, |e, normal| {
        let n = normal.addr();
        let bz_ay = x87_mul(bz, ay);
        let by_az = x87_mul(by, az);
        e.mem.set_u32(n, x87_store32(x87_sub(bz_ay, by_az)));
        let az_bx = x87_mul(az, bx);
        let bz_ax = x87_mul(bz, ax);
        e.mem.set_u32(n + 4, x87_store32(x87_sub(az_bx, bz_ax)));
        let ax_by = x87_mul(ax, by);
        let bx_ay = x87_mul(bx, ay);
        e.mem.set_u32(n + 8, x87_store32(x87_sub(ax_by, bx_ay)));
        e.call(SCALE_TO_UNIT_00525340, &args![n]);
        let normals = e.mem.u32(base + NORMALS);
        let entry = |index: u16| normals.wrapping_add(u32::from(index) * 12);
        let nx = x87_load32(e.mem.u32(n));
        let ny = x87_load32(e.mem.u32(n + 4));
        let nz = x87_load32(e.mem.u32(n + 8));
        // The first vertex: `FLD` its component, `FADDP` with the local's.
        let at = entry(first);
        for (axis, value) in [nx, ny, nz].into_iter().enumerate() {
            let at = at + axis as u32 * 4;
            let sum = x87_add(x87_load32(e.mem.u32(at)), value);
            e.mem.set_u32(at, x87_store32(sum));
        }
        // The second: `FADD ST0,STi` on its loaded component.
        let at = entry(second);
        for (axis, value) in [nx, ny, nz].into_iter().enumerate() {
            let at = at + axis as u32 * 4;
            let sum = x87_add(x87_load32(e.mem.u32(at)), value);
            e.mem.set_u32(at, x87_store32(sum));
        }
        // The third: `FADDP ST3,ST0` for x (the local's value first), then `FADD dword`
        // for y and z.
        let at = entry(third);
        let sum = x87_add(nx, x87_load32(e.mem.u32(at)));
        e.mem.set_u32(at, x87_store32(sum));
        for (axis, value) in [(1u32, ny), (2, nz)] {
            let at = at + axis * 4;
            let sum = x87_add(value, x87_m32(e.mem.u32(at)));
            e.mem.set_u32(at, x87_store32(sum));
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a8_ab40,
        ni_tri_strips_data_calculate_normals(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    #[test]
    fn normals_sum_each_strip_triangle_cross_product() {
        let mut e = Engine::new();
        e.register(CREATE_NORMALS, |_, _| Ret::default());
        e.register(MARK_AS_CHANGED, |_, _| Ret::default());
        e.register(UNITIZE_VECTORS, |_, _| Ret::default());
        // The scaling double halves the normal.
        e.register(SCALE_TO_UNIT_00525340, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v * 0.5);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        let vertices = e.mem.alloc(48);
        // Four vertices: (0,0,0), (1,0,0), (0,1,0), (1,1,0).
        set_floats(
            &mut e,
            vertices,
            &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0],
        );
        let normals = e.mem.alloc(48);
        let lengths = e.mem.alloc(4);
        // Strip 0: 0 1 2 3 (two triangles); strip 1: 1 1 2 (degenerate).
        e.mem.set_u16(lengths, 4);
        e.mem.set_u16(lengths + 2, 3);
        let lists = e.mem.alloc(14);
        for (i, v) in [0u16, 1, 2, 3, 1, 1, 2].into_iter().enumerate() {
            e.mem.set_u16(lists + 2 * i as u32, v);
        }
        e.mem.set_u32(this + VERTICES, vertices);
        e.mem.set_u32(this + NORMALS, normals);
        e.mem.set_u16(this + STRIPS, 2);
        e.mem.set_u32(this + STRIP_LENGTHS, lengths);
        e.mem.set_u32(this + STRIP_LISTS, lists);
        e.mem.set_u16(this + VERTEX_COUNT, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a8_ab40, &args![this]);
        // Triangle (0,1,2): (1,0,0) x (-1,1,0) = (0,0,1), halved; triangle j = 3 is
        // (1,3,2): (0,1,0) x (-1,0,0) = (0,0,1), halved.
        let z = |e: &Engine, i: u32| e.mem.f32(normals + 12 * i + 8);
        assert_eq!(
            [z(&e, 0), z(&e, 1), z(&e, 2), z(&e, 3)],
            [0.5, 1.0, 1.0, 0.5]
        );
        let scaled = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == SCALE_TO_UNIT_00525340)
            .count();
        assert_eq!(scaled, 2);
        let unitize: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == UNITIZE_VECTORS)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(unitize, vec![vec![normals, 4, 12]]);
    }
}
