//! `NiMain/nitrishapedynamicdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiTriShapeDynamicData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_SHAPE_DYNAMIC_DATA_RTTI: u32 = 0x011f_5af4;

// Translated from 00a81800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_shape_dynamic_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_SHAPE_DYNAMIC_DATA_RTTI)
}

// Translated from 00a81830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7b8c0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x5c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_tri_shape_dynamic_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_b8c0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x5cu32]);
    }
    this
}

/// `NiTriShapeDynamicData` (Xbox PDB, offsets checked against the PC code):
/// `NiGeometryData::m_usVertices` +0x08, `NiTriBasedGeomData::m_usTriangles` +0x40,
/// `NiTriShapeData::m_uiTriListLength` +0x44 and `m_pusTriList` +0x48, then
/// `m_usActiveVertices` +0x58 and `m_usActiveTriangles` +0x5A.
const VERTICES: u32 = 0x08;
const TRIANGLES: u32 = 0x40;
const TRI_LIST_LENGTH: u32 = 0x44;
const TRI_LIST: u32 = 0x48;
const ACTIVE_VERTICES: u32 = 0x58;
const ACTIVE_TRIANGLES: u32 = 0x5a;
/// The class's vtable (`0109e32c`).
const NI_TRI_SHAPE_DYNAMIC_DATA_VTABLE: u32 = 0x0109_e32c;
/// `NiTriShapeData::NiTriShapeData` (`00a7b630`, thiscall, nine words, `RET 0x24`).
const NI_TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b630;
/// `NiTriShapeData::IsEqual`, `LoadBinary`, `SaveBinary` and `GetViewerStrings`
/// (thiscall, one word each).
const NI_TRI_SHAPE_DATA_IS_EQUAL: u32 = 0x00a7_b7e0;
const NI_TRI_SHAPE_DATA_LOAD_BINARY: u32 = 0x00a7_b940;
const NI_TRI_SHAPE_DATA_SAVE_BINARY: u32 = 0x00a7_bbe0;
const NI_TRI_SHAPE_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_bd60;
/// `NiGetViewerString` (cdecl, the class name) and `NiGetViewerString_ov6` (cdecl, a name
/// and a 16-bit value, `"%s = %hu"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_U16: u32 = 0x00a7_d490;
/// The strings array's `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_usActiveVertices"` and `"m_usActiveTriangles"`.
const TEXT_ACTIVE_VERTICES: u32 = 0x0109_e3f4;
const TEXT_ACTIVE_TRIANGLES: u32 = 0x0109_e3e0;

/// Adds the string word at `slot` at the end of the strings array (`SetSize` to the size
/// plus the grow-by step when the array is full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

/// One 2-byte transfer (one component of 2 bytes) of the field at `field` through the
/// binary stream `binary` and its function `function` (`m_pfnRead` or `m_pfnWrite`); the
/// component size is kept in the stream argument's stack word, and the stream's position
/// moves on by the result.
fn transfer_u16(e: &mut Engine, binary: Ptr<NiBinaryStream>, function: u32, field: u32) {
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 2);
        let moved = e
            .call(function, &args![binary, field, 2u32, sizes, 1u32])
            .u32();
        let position = e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            binary,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(moved),
        );
    });
}

// Translated from 00a816a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::NiTriShapeDynamicData` (Xbox PDB; thiscall, `RET 0x2c`): the
/// eleven words are, in the PDB's order, the vertex count, the vertex, normal, colour and
/// texture arrays, the texture-set count, the normal/binormal/tangent method, the triangle
/// count, the triangle list, the active vertex count and the active triangle count.
/// `NiTriShapeData::NiTriShapeData` is given the active vertex count, the four arrays, the
/// texture-set count, the method, the active triangle count and the triangle list; then
/// the code stores the triangle count (+0x40), the active triangle count (+0x5A), the
/// active vertex count (+0x58), the vtable (`0109e32c`), the vertex count (+0x08) and three
/// times the triangle count as the list length (+0x44). Returns `this`.
pub fn ni_tri_shape_dynamic_data_ni_tri_shape_dynamic_data(
    e: &mut Engine,
    this: Ptr,
    vertices: u32,
    vertex: u32,
    normal: u32,
    colour: u32,
    texture: u32,
    texture_sets: u32,
    method: u32,
    triangles: u32,
    tri_list: u32,
    active_vertices: u32,
    active_triangles: u32,
) -> Ptr {
    e.call(
        NI_TRI_SHAPE_DATA_CONSTRUCT,
        &args![
            this,
            active_vertices,
            vertex,
            normal,
            colour,
            texture,
            texture_sets,
            method,
            active_triangles,
            tri_list
        ],
    );
    let base = this.addr();
    let triangles = triangles as u16;
    e.mem.set_u16(base + TRIANGLES, triangles);
    e.mem
        .set_u16(base + ACTIVE_TRIANGLES, active_triangles as u16);
    e.mem
        .set_u16(base + ACTIVE_VERTICES, active_vertices as u16);
    e.mem.set_u32(base, NI_TRI_SHAPE_DYNAMIC_DATA_VTABLE);
    e.mem.set_u16(base + VERTICES, vertices as u16);
    e.mem
        .set_u32(base + TRI_LIST_LENGTH, u32::from(triangles) * 3);
    this
}

// Translated from 00a81710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::GetActiveVertexCount` (Xbox PDB): the 16-bit count at +0x58.
pub fn ni_tri_shape_dynamic_data_get_active_vertex_count(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + ACTIVE_VERTICES)
}

// Translated from 00a81720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::GetActiveTriangleCount` (Xbox PDB): the 16-bit count at +0x5A.
pub fn ni_tri_shape_dynamic_data_get_active_triangle_count(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + ACTIVE_TRIANGLES)
}

// Translated from 00a81730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::GetStripData` (Xbox PDB; thiscall, `RET 0x10`, four out
/// pointers): the active triangle count as the strip count (16 bits), no strip lengths
/// (0), the triangle list (+0x48) and three times the active triangle count as the list
/// length.
pub fn ni_tri_shape_dynamic_data_get_strip_data(
    e: &mut Engine,
    this: Ptr,
    strips: u32,
    strip_lengths: u32,
    tri_list: u32,
    tri_list_length: u32,
) {
    let base = this.addr();
    let active = e.mem.u16(base + ACTIVE_TRIANGLES);
    e.mem.set_u16(strips, active);
    e.mem.set_u32(strip_lengths, 0);
    let list = e.mem.u32(base + TRI_LIST);
    e.mem.set_u32(tri_list, list);
    let active = u32::from(e.mem.u16(base + ACTIVE_TRIANGLES));
    e.mem.set_u32(tri_list_length, active * 3);
}

// Translated from 00a817c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::IsEqual` (Xbox PDB; thiscall, `RET 4`): `NiTriShapeData::IsEqual`
/// and both active counts (+0x58, +0x5A) equal.
pub fn ni_tri_shape_dynamic_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if e.call(NI_TRI_SHAPE_DATA_IS_EQUAL, &args![this, other])
        .u32() as u8
        == 0
    {
        return false;
    }
    let (mine, theirs) = (this.addr(), other.addr());
    e.mem.u16(mine + ACTIVE_VERTICES) == e.mem.u16(theirs + ACTIVE_VERTICES)
        && e.mem.u16(mine + ACTIVE_TRIANGLES) == e.mem.u16(theirs + ACTIVE_TRIANGLES)
}

// Translated from 00a81810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::SetActiveVertexCount` (Xbox PDB; thiscall, `RET 4`): stores the
/// low 16 bits of the word at +0x58.
pub fn ni_tri_shape_dynamic_data_set_active_vertex_count(e: &mut Engine, this: Ptr, count: u32) {
    e.mem.set_u16(this.addr() + ACTIVE_VERTICES, count as u16);
}

// Translated from 00a81820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::SetActiveTriangleCount` (Xbox PDB; thiscall, `RET 4`): stores
/// the low 16 bits of the word at +0x5A.
pub fn ni_tri_shape_dynamic_data_set_active_triangle_count(e: &mut Engine, this: Ptr, count: u32) {
    e.mem.set_u16(this.addr() + ACTIVE_TRIANGLES, count as u16);
}

// Translated from 00a81850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::LoadBinary` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriShapeData::LoadBinary`, then the active vertex count (+0x58) and the active
/// triangle count (+0x5A), two bytes each, through the stream being read (`m_pkIstr`,
/// `m_pfnRead`; the stream field is read again for each).
pub fn ni_tri_shape_dynamic_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(NI_TRI_SHAPE_DATA_LOAD_BINARY, &args![this, stream]);
    for field in [ACTIVE_VERTICES, ACTIVE_TRIANGLES] {
        let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
        let read = e.get(binary, NiBinaryStream::m_pfnRead);
        transfer_u16(e, binary, read, this.addr() + field);
    }
}

// Translated from 00a818b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::SaveBinary` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriShapeData::SaveBinary`, then the active vertex count (+0x58) and the active
/// triangle count (+0x5A), two bytes each, through the stream being written (`m_pkOstr`,
/// `m_pfnWrite`; the stream field is read again for each).
pub fn ni_tri_shape_dynamic_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(NI_TRI_SHAPE_DATA_SAVE_BINARY, &args![this, stream]);
    for field in [ACTIVE_VERTICES, ACTIVE_TRIANGLES] {
        let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
        let write = e.get(binary, NiBinaryStream::m_pfnWrite);
        transfer_u16(e, binary, write, this.addr() + field);
    }
}

// Translated from 00a81910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShapeDynamicData::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriShapeData::GetViewerStrings`, then the class name (the first word of `ms_RTTI`),
/// `"m_usActiveVertices"` and `"m_usActiveTriangles"` with their counts, each held in the
/// argument's stack word and added at the array's end.
pub fn ni_tri_shape_dynamic_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr<NiTArray>,
) {
    e.call(NI_TRI_SHAPE_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_TRI_SHAPE_DYNAMIC_DATA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        for (name, field) in [
            (TEXT_ACTIVE_VERTICES, ACTIVE_VERTICES),
            (TEXT_ACTIVE_TRIANGLES, ACTIVE_TRIANGLES),
        ] {
            let count = u32::from(e.mem.u16(this.addr() + field));
            let text = e.call(VIEWER_STRING_U16, &args![name, count]).u32();
            e.mem.set_u32(slot.addr(), text);
            add_viewer_string(e, strings, slot.addr());
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a81800, ni_tri_shape_dynamic_data_get_rtti() -> Ptr),
        entry!(0x00a81830, ni_tri_shape_dynamic_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a8_16a0,
            ni_tri_shape_dynamic_data_ni_tri_shape_dynamic_data(
                Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
        entry!(0x00a8_1710, ni_tri_shape_dynamic_data_get_active_vertex_count(Ptr) -> u16),
        entry!(0x00a8_1720, ni_tri_shape_dynamic_data_get_active_triangle_count(Ptr) -> u16),
        entry!(
            0x00a8_1730,
            ni_tri_shape_dynamic_data_get_strip_data(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00a8_17c0, ni_tri_shape_dynamic_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a8_1810,
            ni_tri_shape_dynamic_data_set_active_vertex_count(Ptr, u32)
        ),
        entry!(
            0x00a8_1820,
            ni_tri_shape_dynamic_data_set_active_triangle_count(Ptr, u32)
        ),
        entry!(
            0x00a8_1850,
            ni_tri_shape_dynamic_data_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a8_18b0,
            ni_tri_shape_dynamic_data_save_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a8_1910,
            ni_tri_shape_dynamic_data_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_shape_dynamic_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a81800, &args![]).u32(),
            NI_TRI_SHAPE_DYNAMIC_DATA_RTTI
        );
    }

    #[test]
    fn ni_tri_shape_dynamic_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_b8c0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a81830, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a81830, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x5c]]);
        assert_eq!(calls_to(&e, 0x00a7_b8c0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    /// A stream double: the read and write functions at this address log their words,
    /// fill a read field with 0x0102 and move 2 bytes.
    const TRANSFER: u32 = 0x0070_2000;

    fn stream(e: &mut Engine) -> u32 {
        e.register(TRANSFER, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            if e.mem.u16(a[1]) == 0 {
                e.mem.set_u16(a[1], 0x0102);
            }
            2u32.into_ret()
        });
        let stream = e.mem.alloc(0x5c4);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, TRANSFER);
        e.mem.set_u32(binary + 0xc, TRANSFER);
        e.mem.set_u32(stream + 0x24c, binary);
        e.mem.set_u32(stream + 0x250, binary);
        stream
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_passes_the_active_counts_to_the_base() {
        let mut e = Engine::new();
        e.register(NI_TRI_SHAPE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        let words = [
            this, 0x1_0010, 2, 3, 4, 5, 6, 7, 0x2_0009, 10, 0x3_000b, 0x4_000c,
        ];
        assert_eq!(e.call(0x00a8_16a0, &words).u32(), this);
        assert_eq!(
            calls_to(&e, NI_TRI_SHAPE_DATA_CONSTRUCT),
            vec![vec![this, 0x3_000b, 2, 3, 4, 5, 6, 7, 0x4_000c, 10]]
        );
        assert_eq!(e.mem.u32(this), NI_TRI_SHAPE_DYNAMIC_DATA_VTABLE);
        assert_eq!(e.mem.u16(this + VERTICES), 0x10);
        assert_eq!(e.mem.u16(this + TRIANGLES), 9);
        assert_eq!(e.mem.u32(this + TRI_LIST_LENGTH), 27);
        assert_eq!(e.mem.u16(this + ACTIVE_VERTICES), 0xb);
        assert_eq!(e.mem.u16(this + ACTIVE_TRIANGLES), 0xc);
    }

    #[test]
    fn active_counts_get_and_set_sixteen_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        e.call(0x00a8_1810, &args![this, 0x1_0005u32]);
        e.call(0x00a8_1820, &args![this, 0x2_0006u32]);
        assert_eq!(e.call(0x00a8_1710, &args![this]).u32() & 0xffff, 5);
        assert_eq!(e.call(0x00a8_1720, &args![this]).u32() & 0xffff, 6);
    }

    #[test]
    fn strip_data_is_one_list_of_the_active_triangles() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        e.mem.set_u16(this + ACTIVE_TRIANGLES, 7);
        e.mem.set_u32(this + TRI_LIST, 0x0abc_0000);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out, 0xffff_ffff);
        e.mem.set_u32(out + 4, 0xffff_ffff);
        e.call(0x00a8_1730, &args![this, out, out + 4, out + 8, out + 0xc]);
        assert_eq!(e.mem.u32(out), 0xffff_0007);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.mem.u32(out + 8), 0x0abc_0000);
        assert_eq!(e.mem.u32(out + 0xc), 21);
    }

    #[test]
    fn is_equal_needs_the_base_and_both_active_counts() {
        let mut e = Engine::new();
        e.register(NI_TRI_SHAPE_DATA_IS_EQUAL, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        let a = e.mem.alloc(0x5c);
        let b = e.mem.alloc(0x5c);
        assert_eq!(e.call(0x00a8_17c0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(a + 4, 1);
        assert_eq!(e.call(0x00a8_17c0, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u16(b + ACTIVE_TRIANGLES, 1);
        assert_eq!(e.call(0x00a8_17c0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u16(a + ACTIVE_TRIANGLES, 1);
        e.mem.set_u16(a + ACTIVE_VERTICES, 1);
        assert_eq!(e.call(0x00a8_17c0, &args![a, b]).u32() & 0xff, 0);
    }

    #[test]
    fn load_and_save_binary_move_both_active_counts() {
        let mut e = Engine::new();
        e.register(NI_TRI_SHAPE_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(NI_TRI_SHAPE_DATA_SAVE_BINARY, |_, _| Ret::default());
        let stream = stream(&mut e);
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        e.call(0x00a8_1850, &args![this, stream]);
        assert_eq!(e.mem.u16(this + ACTIVE_VERTICES), 0x0102);
        assert_eq!(e.mem.u16(this + ACTIVE_TRIANGLES), 0x0102);
        let binary = e.mem.u32(stream + 0x24c);
        assert_eq!(e.mem.u32(binary + 4), 4);
        e.call(0x00a8_18b0, &args![this, stream]);
        assert_eq!(e.mem.u32(binary + 4), 8);
        let fields: Vec<u32> = calls_to(&e, TRANSFER).iter().map(|w| w[1]).collect();
        assert_eq!(
            fields,
            vec![this + 0x58, this + 0x5a, this + 0x58, this + 0x5a]
        );
        assert_eq!(
            calls_to(&e, NI_TRI_SHAPE_DATA_SAVE_BINARY),
            vec![vec![this, stream]]
        );
    }

    #[test]
    fn viewer_strings_add_the_class_and_both_active_counts() {
        let mut e = Engine::new();
        e.map(NI_TRI_SHAPE_DYNAMIC_DATA_RTTI, 4);
        e.set_global(NI_TRI_SHAPE_DYNAMIC_DATA_RTTI, 0x0104_0000u32);
        e.register(NI_TRI_SHAPE_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_U16, |_, a| (a[0] + (a[1] << 24)).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 0xe, 2);
        let this = e.mem.alloc(0x5c);
        e.mem.set_u16(this + ACTIVE_VERTICES, 3);
        e.mem.set_u16(this + ACTIVE_TRIANGLES, 4);
        e.call(0x00a8_1910, &args![this, strings]);
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        assert_eq!(e.mem.u32(base), 0x0104_0001);
        assert_eq!(e.mem.u32(base + 4), TEXT_ACTIVE_VERTICES + (3 << 24));
        assert_eq!(e.mem.u32(base + 8), TEXT_ACTIVE_TRIANGLES + (4 << 24));
    }
}
