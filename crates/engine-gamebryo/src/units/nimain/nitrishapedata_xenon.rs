//! `NiMain/nitrishapedata_xenon.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTriShapeData` fields (Xbox PDB, PC offsets): `NiGeometryData::m_usVertices` +0x08,
/// `m_pkVertex` +0x20 and `m_pkNormal` +0x24 (twelve bytes per entry), `m_pusTriList`
/// +0x48, `m_pkSharedNormals` +0x4C (one entry of eight bytes per vertex: a 16-bit count
/// and, at +4, its list of 16-bit vertex indices) and `m_usSharedNormalsArraySize` +0x50.
const VERTEX_COUNT: u32 = 0x08;
const VERTICES: u32 = 0x20;
const NORMALS: u32 = 0x24;
const TRI_LIST: u32 = 0x48;
const SHARED_NORMALS: u32 = 0x4c;
const SHARED_NORMALS_SIZE: u32 = 0x50;
/// The vtable slots the code calls on the data: +0xA8 (the triangle count to walk, 16
/// bits) and +0x90 (the vertex count given to `UnitizeVectors`, 16 bits); both thiscall
/// without arguments (`GetActiveTriangleCount` and `GetActiveVertexCount` in the Xbox PDB
/// slot order).
const SLOT_ACTIVE_TRIANGLES: u32 = 0xa8;
const SLOT_ACTIVE_VERTICES: u32 = 0x90;
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

/// The form of one x87 addition of a normal component to a stored float: `FLD` of the
/// stored float then `FADD ST0,STi` or `FADDP` (the stored value first), or `FLD STi` of
/// the component then `FADD dword` (the component first, the stored value a memory
/// operand).
#[derive(Clone, Copy)]
enum Add {
    StoredFirst,
    ComponentFirst,
}

/// Adds the normal to the float triple at `at`, each component in its form.
fn add_normal(e: &mut Engine, at: u32, normal: [f64; 3], forms: [Add; 3]) {
    for (axis, (value, form)) in normal.into_iter().zip(forms).enumerate() {
        let at = at.wrapping_add(axis as u32 * 4);
        let sum = match form {
            Add::StoredFirst => x87_add(x87_load32(e.mem.u32(at)), value),
            Add::ComponentFirst => x87_add(value, x87_m32(e.mem.u32(at))),
        };
        e.mem.set_u32(at, x87_store32(sum));
    }
}

/// Adds the normal to every vertex listed in the shared-normal entry of vertex `vertex`,
/// from the last index of the list to the first.
fn add_to_shared(e: &mut Engine, base: u32, vertex: u32, normal: [f64; 3], forms: [Add; 3]) {
    let entry = e.mem.u32(base + SHARED_NORMALS).wrapping_add(vertex * 8);
    let mut count = e.mem.u16(entry);
    let mut at = e.mem.u32(entry + 4);
    if count == 0 {
        return;
    }
    at = at.wrapping_add(u32::from(count) * 2);
    loop {
        let index = u32::from(e.mem.u16(at.wrapping_sub(2)));
        let normals = e.mem.u32(base + NORMALS);
        at = at.wrapping_sub(2);
        count = count.wrapping_sub(1);
        add_normal(e, normals.wrapping_add(index * 12), normal, forms);
        if count == 0 {
            break;
        }
    }
}

// Translated from 00a8ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeData::CalculateNormals` (Xbox PDB; thiscall, no arguments): makes sure there
/// are normals (`CreateNormals(1)`), then for each of the triangles its slot +0xA8 counts
/// (three list entries each) takes the cross product of the edges `v1 - v0` and `v2 - v1`
/// (in a local, each float stored), scales it with `00525340` and adds it to the three
/// normals; when there are shared normals whose array size is the vertex count, it is also
/// added to every vertex in each corner's shared list. Finally `NiPoint3::UnitizeVectors` on
/// the normals (the count from slot +0x90) and `MarkAsChanged(2)`. x87 arithmetic in the
/// code order and operand order.
pub fn ni_tri_shape_data_calculate_normals(e: &mut Engine, this: Ptr) {
    use Add::{ComponentFirst, StoredFirst};
    let base = this.addr();
    e.call(CREATE_NORMALS, &args![this, 1u32]);
    let triangles = e.vcall(base, SLOT_ACTIVE_TRIANGLES, &args![]).u32() as u16;
    let mut cursor = e.mem.u32(base + TRI_LIST);
    let mut remaining = u32::from(triangles);
    while remaining != 0 {
        let first = e.mem.u16(cursor);
        let second = e.mem.u16(cursor.wrapping_add(2));
        let third = e.mem.u16(cursor.wrapping_add(4));
        cursor = cursor.wrapping_add(6);
        let vertices = e.mem.u32(base + VERTICES);
        let v = |e: &Engine, index: u16, axis: u32| {
            e.mem
                .u32(vertices.wrapping_add(u32::from(index) * 12 + axis * 4))
        };
        let a: Vec<u32> = (0..3)
            .map(|axis| float_difference(v(e, second, axis), v(e, first, axis)))
            .collect();
        let b: Vec<u32> = (0..3)
            .map(|axis| float_difference(v(e, third, axis), v(e, second, axis)))
            .collect();
        let (ax, ay, az) = (x87_load32(a[0]), x87_load32(a[1]), x87_load32(a[2]));
        let (bx, by, bz) = (x87_load32(b[0]), x87_load32(b[1]), x87_load32(b[2]));
        let normal = e.with_stack(12, |e, local| {
            let n = local.addr();
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
            [0u32, 4, 8].map(|offset| x87_load32(e.mem.u32(n + offset)))
        });
        let entry = |e: &Engine, index: u16| {
            e.mem
                .u32(base + NORMALS)
                .wrapping_add(u32::from(index) * 12)
        };
        let at = entry(e, first);
        add_normal(e, at, normal, [StoredFirst, ComponentFirst, StoredFirst]);
        let at = entry(e, second);
        add_normal(e, at, normal, [StoredFirst, ComponentFirst, StoredFirst]);
        let at = entry(e, third);
        add_normal(e, at, normal, [StoredFirst, StoredFirst, ComponentFirst]);
        if e.mem.u32(base + SHARED_NORMALS) != 0
            && e.mem.u16(base + SHARED_NORMALS_SIZE) == e.mem.u16(base + VERTEX_COUNT)
        {
            add_to_shared(
                e,
                base,
                u32::from(first),
                normal,
                [StoredFirst, StoredFirst, StoredFirst],
            );
            add_to_shared(
                e,
                base,
                u32::from(second),
                normal,
                [ComponentFirst, StoredFirst, ComponentFirst],
            );
            add_to_shared(
                e,
                base,
                u32::from(third),
                normal,
                [StoredFirst, ComponentFirst, StoredFirst],
            );
        }
        remaining -= 1;
    }
    let count = u32::from(e.vcall(base, SLOT_ACTIVE_VERTICES, &args![]).u32() as u16);
    let normals = e.mem.u32(base + NORMALS);
    e.call(UNITIZE_VECTORS, &args![normals, count, 0xcu32]);
    e.call(MARK_AS_CHANGED, &args![this, 2u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a8_ed40,
        ni_tri_shape_data_calculate_normals(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0710_0000;
    const TRIANGLES_DOUBLE: u32 = 0x0720_0000;
    const VERTICES_DOUBLE: u32 = 0x0720_0010;

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    #[test]
    fn normals_sum_triangle_cross_products_and_shared_lists() {
        let mut e = Engine::new();
        e.register(CREATE_NORMALS, |_, _| Ret::default());
        e.register(MARK_AS_CHANGED, |_, _| Ret::default());
        e.register(UNITIZE_VECTORS, |_, _| Ret::default());
        e.register(SCALE_TO_UNIT_00525340, |_, _| Ret::default());
        e.register(TRIANGLES_DOUBLE, |_, _| 0xffff_0001u32.into_ret());
        e.register(VERTICES_DOUBLE, |_, _| 0x0001_0004u32.into_ret());
        let mut slots = vec![0u32; 0xac / 4];
        slots[SLOT_ACTIVE_TRIANGLES as usize / 4] = TRIANGLES_DOUBLE;
        slots[SLOT_ACTIVE_VERTICES as usize / 4] = VERTICES_DOUBLE;
        e.put_vtable(VTABLE, &slots);
        let this = e.mem.alloc(0x58);
        e.mem.set_u32(this, VTABLE);
        let vertices = e.mem.alloc(48);
        // (0,0,0), (2,0,0), (0,3,0): the normal (2,0,0) x (-2,3,0) = (0,0,6).
        set_floats(
            &mut e,
            vertices,
            &[0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0],
        );
        let normals = e.mem.alloc(48);
        let list = e.mem.alloc(6);
        for (i, v) in [0u16, 1, 2].into_iter().enumerate() {
            e.mem.set_u16(list + 2 * i as u32, v);
        }
        e.mem.set_u32(this + VERTICES, vertices);
        e.mem.set_u32(this + NORMALS, normals);
        e.mem.set_u32(this + TRI_LIST, list);
        e.mem.set_u16(this + VERTEX_COUNT, 4);
        e.call(0x00a8_ed40, &args![this]);
        let z = |e: &Engine, i: u32| e.mem.f32(normals + 12 * i + 8);
        assert_eq!(
            [z(&e, 0), z(&e, 1), z(&e, 2), z(&e, 3)],
            [6.0, 6.0, 6.0, 0.0]
        );
        // With shared normals: vertex 0 shares with 3, vertex 2 with 3 and 1.
        let shared = e.mem.alloc(32);
        let zero_list = e.mem.alloc(2);
        e.mem.set_u16(zero_list, 3);
        let two_list = e.mem.alloc(4);
        e.mem.set_u16(two_list, 3);
        e.mem.set_u16(two_list + 2, 1);
        e.mem.set_u16(shared, 1);
        e.mem.set_u32(shared + 4, zero_list);
        e.mem.set_u16(shared + 16, 2);
        e.mem.set_u32(shared + 20, two_list);
        e.mem.set_u32(this + SHARED_NORMALS, shared);
        e.mem.set_u16(this + SHARED_NORMALS_SIZE, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a8_ed40, &args![this]);
        assert_eq!(
            [z(&e, 0), z(&e, 1), z(&e, 2), z(&e, 3)],
            [12.0, 18.0, 12.0, 12.0]
        );
        let unitize: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == UNITIZE_VECTORS)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(unitize, vec![vec![normals, 4, 12]]);
        // A size different from the vertex count skips the shared lists.
        e.mem.set_u16(this + SHARED_NORMALS_SIZE, 3);
        e.call(0x00a8_ed40, &args![this]);
        assert_eq!(
            [z(&e, 0), z(&e, 1), z(&e, 2), z(&e, 3)],
            [18.0, 24.0, 18.0, 12.0]
        );
    }
}
