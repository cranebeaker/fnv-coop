//! `NiMain/nitristripsdynamicdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiTriStripsDynamicData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_STRIPS_DYNAMIC_DATA_RTTI: u32 = 0x011f_5e8c;

// Translated from 00a92470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_strips_dynamic_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_STRIPS_DYNAMIC_DATA_RTTI)
}

// Translated from 00a924b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a76040`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x54 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_tri_strips_dynamic_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_6040;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x54u32]);
    }
    this
}

// Translated from 00a923b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::GetActiveTriangleCount` (Xbox PDB): the 16-bit active count at
/// +0x52.
pub fn ni_tri_strips_dynamic_data_get_active_triangle_count(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr().wrapping_add(0x52))
}

// Translated from 00a92490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::SetActiveTriangleCount` (Xbox PDB): the active count at +0x52,
/// and, when the strip count (`NiTriStripsData::m_usStrips`, +0x44) is not 0, the first
/// strip's length (the first word of `m_pusStripLengths`, +0x48) too.
pub fn ni_tri_strips_dynamic_data_set_active_triangle_count(e: &mut Engine, this: Ptr, count: u16) {
    let strips = e.mem.u16(this.addr().wrapping_add(0x44));
    e.mem.set_u16(this.addr().wrapping_add(0x52), count);
    if strips != 0 {
        let lengths = e.mem.u32(this.addr().wrapping_add(0x48));
        e.mem.set_u16(lengths, count);
    }
}

/// `NiTriStripsDynamicData` (Xbox PDB, offsets checked against the PC code):
/// `NiTriStripsData::m_pusStripLengths` +0x48 and `m_pusStripLists` +0x4C, then
/// `m_usActiveVertices` +0x50 and `m_usActiveTriangles` +0x52.
const STRIP_LENGTHS: u32 = 0x48;
const STRIP_LISTS: u32 = 0x4c;
const ACTIVE_VERTICES: u32 = 0x50;
const ACTIVE_TRIANGLES: u32 = 0x52;
/// The class's vtable (`0109fc54`).
const NI_TRI_STRIPS_DYNAMIC_DATA_VTABLE: u32 = 0x0109_fc54;
/// `NiNew`'s allocator (`00aa13e0`, cdecl: size) and the `NiTriStripsData` default
/// constructor body `00a75e20` (unnamed in the engine map; thiscall; it sets the vtable
/// `0109d8ac`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_TRI_STRIPS_DATA_CONSTRUCT_00A75E20: u32 = 0x00a7_5e20;
/// `NiTriStripsData::IsEqual`, `LoadBinary`, `SaveBinary` and `GetViewerStrings`
/// (thiscall, one word each).
const NI_TRI_STRIPS_DATA_IS_EQUAL: u32 = 0x00a7_5f80;
const NI_TRI_STRIPS_DATA_LOAD_BINARY: u32 = 0x00a7_61c0;
const NI_TRI_STRIPS_DATA_SAVE_BINARY: u32 = 0x00a7_6340;
const NI_TRI_STRIPS_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_6430;
/// `NiGetViewerString` (cdecl, the class name) and `NiGetViewerString_ov6` (cdecl, a name
/// and a 16-bit value, `"%s = %hu"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_U16: u32 = 0x00a7_d490;
/// The strings array's `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_usActiveVertices"` and `"m_usActiveTriangles"` (the strings
/// `NiTriShapeDynamicData` uses too).
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

// Translated from 00a923f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class's stream loader (Gamebryo's `CreateObject`; Ghidra made no function of it):
/// registered with `NiStream::RegisterLoader` under `"NiTriStripsDynamicData"` at
/// `00a8a15d`. A new 0x54-byte object (`00aa13e0`) with the `NiTriStripsData` default
/// constructor body, both active counts cleared and the class's vtable; null when the
/// allocation fails.
pub fn ni_tri_strips_dynamic_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x54u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_TRI_STRIPS_DATA_CONSTRUCT_00A75E20, &args![object]);
    e.mem.set_u16(object + ACTIVE_VERTICES, 0);
    e.mem.set_u32(object, NI_TRI_STRIPS_DYNAMIC_DATA_VTABLE);
    e.mem.set_u16(object + ACTIVE_TRIANGLES, 0);
    Ptr::new(object)
}

// Translated from 00a923c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::GetStripData` (Xbox PDB; thiscall, `RET 0x10`, four out
/// pointers): one strip (16 bits), the strip lengths (+0x48), the strip lists (+0x4C) and
/// the active triangle count plus 2 as the list length.
pub fn ni_tri_strips_dynamic_data_get_strip_data(
    e: &mut Engine,
    this: Ptr,
    strips: u32,
    strip_lengths: u32,
    strip_lists: u32,
    list_length: u32,
) {
    let base = this.addr();
    e.mem.set_u16(strips, 1);
    let lengths = e.mem.u32(base + STRIP_LENGTHS);
    e.mem.set_u32(strip_lengths, lengths);
    let lists = e.mem.u32(base + STRIP_LISTS);
    e.mem.set_u32(strip_lists, lists);
    let active = u32::from(e.mem.u16(base + ACTIVE_TRIANGLES));
    e.mem.set_u32(list_length, active + 2);
}

// Translated from 00a92430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::IsEqual` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriStripsData::IsEqual` and both active counts (+0x50, +0x52) equal.
pub fn ni_tri_strips_dynamic_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if e.call(NI_TRI_STRIPS_DATA_IS_EQUAL, &args![this, other])
        .u32() as u8
        == 0
    {
        return false;
    }
    let (mine, theirs) = (this.addr(), other.addr());
    e.mem.u16(mine + ACTIVE_VERTICES) == e.mem.u16(theirs + ACTIVE_VERTICES)
        && e.mem.u16(mine + ACTIVE_TRIANGLES) == e.mem.u16(theirs + ACTIVE_TRIANGLES)
}

// Translated from 00a92480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::SetActiveVertexCount` (Xbox PDB; thiscall, `RET 4`): stores the
/// low 16 bits of the word at +0x50.
pub fn ni_tri_strips_dynamic_data_set_active_vertex_count(e: &mut Engine, this: Ptr, count: u32) {
    e.mem.set_u16(this.addr() + ACTIVE_VERTICES, count as u16);
}

// Translated from 00a924d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::LoadBinary` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriStripsData::LoadBinary`, then the active vertex count (+0x50) and the active
/// triangle count (+0x52), two bytes each, through the stream being read (`m_pkIstr`,
/// `m_pfnRead`; the stream field is read again for each).
pub fn ni_tri_strips_dynamic_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(NI_TRI_STRIPS_DATA_LOAD_BINARY, &args![this, stream]);
    for field in [ACTIVE_VERTICES, ACTIVE_TRIANGLES] {
        let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
        let read = e.get(binary, NiBinaryStream::m_pfnRead);
        transfer_u16(e, binary, read, this.addr() + field);
    }
}

// Translated from 00a92530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::SaveBinary` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriStripsData::SaveBinary`, then the active vertex count (+0x50) and the active
/// triangle count (+0x52), two bytes each, through the stream being written (`m_pkOstr`,
/// `m_pfnWrite`; the stream field is read again for each).
pub fn ni_tri_strips_dynamic_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(NI_TRI_STRIPS_DATA_SAVE_BINARY, &args![this, stream]);
    for field in [ACTIVE_VERTICES, ACTIVE_TRIANGLES] {
        let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
        let write = e.get(binary, NiBinaryStream::m_pfnWrite);
        transfer_u16(e, binary, write, this.addr() + field);
    }
}

// Translated from 00a92590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriStripsDynamicData::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiTriStripsData::GetViewerStrings`, then the class name (the first word of `ms_RTTI`),
/// `"m_usActiveVertices"` and `"m_usActiveTriangles"` with their counts, each held in the
/// argument's stack word and added at the array's end.
pub fn ni_tri_strips_dynamic_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr<NiTArray>,
) {
    e.call(NI_TRI_STRIPS_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_TRI_STRIPS_DYNAMIC_DATA_RTTI);
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
        entry!(0x00a92470, ni_tri_strips_dynamic_data_get_rtti() -> Ptr),
        entry!(0x00a924b0, ni_tri_strips_dynamic_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a923b0, ni_tri_strips_dynamic_data_get_active_triangle_count(Ptr) -> u16),
        entry!(
            0x00a92490,
            ni_tri_strips_dynamic_data_set_active_triangle_count(Ptr, u16)
        ),
        entry!(0x00a9_23f0, ni_tri_strips_dynamic_data_create_object() -> Ptr),
        entry!(
            0x00a9_23c0,
            ni_tri_strips_dynamic_data_get_strip_data(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00a9_2430, ni_tri_strips_dynamic_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a9_2480,
            ni_tri_strips_dynamic_data_set_active_vertex_count(Ptr, u32)
        ),
        entry!(
            0x00a9_24d0,
            ni_tri_strips_dynamic_data_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a9_2530,
            ni_tri_strips_dynamic_data_save_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a9_2590,
            ni_tri_strips_dynamic_data_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_strips_dynamic_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a92470, &args![]).u32(),
            NI_TRI_STRIPS_DYNAMIC_DATA_RTTI
        );
    }

    #[test]
    fn ni_tri_strips_dynamic_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_6040, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a924b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a924b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x54]]);
        assert_eq!(calls_to(&e, 0x00a7_6040), vec![vec![this]; 2]);
    }

    #[test]
    fn active_triangle_count_get_and_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x54);
        let lengths = e.mem.alloc(4);
        e.mem.set_u32(this + 0x48, lengths);
        e.mem.set_u16(lengths, 9);
        e.call(0x00a92490, &args![this, 5u32]);
        assert_eq!(e.call(0x00a923b0, &args![this]).u32() & 0xffff, 5);
        assert_eq!(e.mem.u16(lengths), 9);
        e.mem.set_u16(this + 0x44, 1);
        e.call(0x00a92490, &args![this, 6u32]);
        assert_eq!(e.call(0x00a923b0, &args![this]).u32() & 0xffff, 6);
        assert_eq!(e.mem.u16(lengths), 6);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    /// A stream double: the read and write functions at this address fill a read field
    /// with 0x0102 and move 2 bytes.
    const TRANSFER: u32 = 0x0070_2000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn create_object_builds_a_cleared_object_or_gives_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_23f0, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(2) {
                e.mem.set_u16(block + offset, 0xffff);
            }
            block.into_ret()
        });
        e.register(NI_TRI_STRIPS_DATA_CONSTRUCT_00A75E20, |_, a| {
            a[0].into_ret()
        });
        let object = e.call(0x00a9_23f0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_TRI_STRIPS_DYNAMIC_DATA_VTABLE);
        assert_eq!(e.mem.u16(object + ACTIVE_VERTICES), 0);
        assert_eq!(e.mem.u16(object + ACTIVE_TRIANGLES), 0);
        assert_eq!(e.mem.u16(object + 0x4e), 0xffff);
    }

    #[test]
    fn strip_data_is_one_strip_of_the_active_triangles_plus_two() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x54);
        e.mem.set_u32(this + STRIP_LENGTHS, 0x0aaa_0000);
        e.mem.set_u32(this + STRIP_LISTS, 0x0bbb_0000);
        e.mem.set_u16(this + ACTIVE_TRIANGLES, 5);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out, 0xffff_ffff);
        e.call(0x00a9_23c0, &args![this, out, out + 4, out + 8, out + 0xc]);
        assert_eq!(e.mem.u32(out), 0xffff_0001);
        assert_eq!(e.mem.u32(out + 4), 0x0aaa_0000);
        assert_eq!(e.mem.u32(out + 8), 0x0bbb_0000);
        assert_eq!(e.mem.u32(out + 0xc), 7);
    }

    #[test]
    fn is_equal_needs_the_base_and_both_active_counts() {
        let mut e = Engine::new();
        e.register(NI_TRI_STRIPS_DATA_IS_EQUAL, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        let a = e.mem.alloc(0x54);
        let b = e.mem.alloc(0x54);
        assert_eq!(e.call(0x00a9_2430, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(a + 4, 1);
        assert_eq!(e.call(0x00a9_2430, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u16(b + ACTIVE_VERTICES, 1);
        assert_eq!(e.call(0x00a9_2430, &args![a, b]).u32() & 0xff, 0);
        e.call(0x00a9_2480, &args![a, 0x7_0001u32]);
        assert_eq!(e.mem.u16(a + ACTIVE_VERTICES), 1);
        assert_eq!(e.call(0x00a9_2430, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u16(b + ACTIVE_TRIANGLES, 1);
        assert_eq!(e.call(0x00a9_2430, &args![a, b]).u32() & 0xff, 0);
    }

    #[test]
    fn load_and_save_binary_move_both_active_counts() {
        let mut e = Engine::new();
        e.register(NI_TRI_STRIPS_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(NI_TRI_STRIPS_DATA_SAVE_BINARY, |_, _| Ret::default());
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
        let this = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        e.call(0x00a9_24d0, &args![this, stream]);
        assert_eq!(e.mem.u16(this + ACTIVE_VERTICES), 0x0102);
        assert_eq!(e.mem.u16(this + ACTIVE_TRIANGLES), 0x0102);
        e.call(0x00a9_2530, &args![this, stream]);
        assert_eq!(e.mem.u32(binary + 4), 8);
        let fields: Vec<u32> = calls_to(&e, TRANSFER).iter().map(|w| w[1]).collect();
        assert_eq!(
            fields,
            vec![this + 0x50, this + 0x52, this + 0x50, this + 0x52]
        );
        assert_eq!(
            calls_to(&e, NI_TRI_STRIPS_DATA_LOAD_BINARY),
            vec![vec![this, stream]]
        );
    }

    #[test]
    fn viewer_strings_add_the_class_and_both_active_counts() {
        let mut e = Engine::new();
        e.map(NI_TRI_STRIPS_DYNAMIC_DATA_RTTI, 4);
        e.set_global(NI_TRI_STRIPS_DYNAMIC_DATA_RTTI, 0x0104_0000u32);
        e.register(NI_TRI_STRIPS_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
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
        let this = e.mem.alloc(0x54);
        e.mem.set_u16(this + ACTIVE_VERTICES, 3);
        e.mem.set_u16(this + ACTIVE_TRIANGLES, 4);
        e.call(0x00a9_2590, &args![this, strings]);
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        assert_eq!(e.mem.u32(base), 0x0104_0001);
        assert_eq!(e.mem.u32(base + 4), TEXT_ACTIVE_VERTICES + (3 << 24));
        assert_eq!(e.mem.u32(base + 8), TEXT_ACTIVE_TRIANGLES + (4 << 24));
    }
}
