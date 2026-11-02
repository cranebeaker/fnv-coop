//! `NiMain/nitribasedgeomdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `NiTriBasedGeomData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRI_BASED_GEOM_DATA_RTTI: u32 = 0x011f_5b90;

// Translated from 00a8a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_tri_based_geom_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRI_BASED_GEOM_DATA_RTTI)
}

/// `NiGeometryData::NiGeometryData` (engine map, `00a67c00`, `thiscall`, no words).
const NI_GEOMETRY_DATA_CONSTRUCT: u32 = 0x00a6_7c00;
/// `NiTriBasedGeomData`'s vtable (`0109f2f4`, named by its RTTI locator).
const NI_TRI_BASED_GEOM_DATA_VTABLE: u32 = 0x0109_f2f4;

// Translated from 00a8a970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::NiTriBasedGeomData` (Xbox PDB): `thiscall`, no words. Runs
/// `NiGeometryData::NiGeometryData` (`00a67c00`), clears the 16-bit word at +0x40 (the
/// triangle count in the Xbox PDB's `NiTriBasedGeomData::m_usTriangles`; offset from the PC
/// code), sets the class's vtable (`0109f2f4`) and answers `this`.
pub fn ni_tri_based_geom_data_ni_tri_based_geom_data(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_GEOMETRY_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u16(this.addr().wrapping_add(0x40), 0);
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_DATA_VTABLE);
    this
}

/// `NiGeometryData::LoadBinary` / `SaveBinary` (engine map, `00a67ea0` / `00a68520`;
/// `thiscall`, `RET 4`).
const NI_GEOMETRY_DATA_LOAD_BINARY: u32 = 0x00a6_7ea0;
const NI_GEOMETRY_DATA_SAVE_BINARY: u32 = 0x00a6_8520;

/// One transfer of the triangle count (+0x40, two bytes, element size 2) through the
/// stream's `NiBinaryStream` at `stream_slot` (`m_pkIstr` +0x24c or `m_pkOstr` +0x250) and its
/// function at `function_slot` (`m_pfnRead` +8 or `m_pfnWrite` +0xc), the element size kept
/// in the stream argument's own slot and the position (+4) advanced by the result.
fn transfer_triangle_count(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
    stream_slot: u32,
    function_slot: u32,
) {
    let binary = e.mem.u32(stream.addr() + stream_slot);
    let function = e.mem.u32(binary.wrapping_add(function_slot));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), 2);
        let moved = e
            .call(
                function,
                &args![binary, this.addr() + 0x40, 2u32, element.addr(), 1u32],
            )
            .u32();
        let position = e.mem.u32(binary.wrapping_add(4)).wrapping_add(moved);
        e.mem.set_u32(binary.wrapping_add(4), position);
    });
}

// Translated from 00a8aa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::LoadBinary` (Xbox PDB), `thiscall`, `RET 4`:
/// `NiGeometryData::LoadBinary`, then the triangle count (`m_usTriangles`, +0x40).
pub fn ni_tri_based_geom_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_GEOMETRY_DATA_LOAD_BINARY, &args![this, stream]);
    transfer_triangle_count(e, this, stream, 0x24c, 8);
}

// Translated from 00a8aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::SaveBinary` (Xbox PDB), `thiscall`, `RET 4`:
/// `NiGeometryData::SaveBinary`, then the triangle count (+0x40).
pub fn ni_tri_based_geom_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_GEOMETRY_DATA_SAVE_BINARY, &args![this, stream]);
    transfer_triangle_count(e, this, stream, 0x250, 0xc);
}

/// `NiGeometryData::IsEqual` (`00a67840`, thiscall, the other object; `AL`).
const NI_GEOMETRY_DATA_IS_EQUAL: u32 = 0x00a6_7840;
/// The destructor body `00a67c70` (unnamed in the engine map; thiscall) and the sized
/// delete (`00aa1460`, cdecl `(pointer, size)`).
const NI_GEOMETRY_DATA_DESTRUCT_00A67C70: u32 = 0x00a6_7c70;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiGeometryData::GetViewerStrings` (`00a688b0`, thiscall, the strings array).
const NI_GEOMETRY_DATA_GET_VIEWER_STRINGS: u32 = 0x00a6_88b0;
/// `NiGetViewerString` (cdecl, the class name) and `NiGetViewerString_ov6` (cdecl, a name
/// and a 16-bit value, `"%s = %hu"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_U16: u32 = 0x00a7_d490;
/// The strings array's `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_usTriangles"`.
const TEXT_TRIANGLES: u32 = 0x0109_f3a8;

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

// Translated from 00a8a9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::IsEqual` (Xbox PDB; thiscall, `RET 4`): false when
/// `NiGeometryData::IsEqual` is; otherwise whether both triangle counts (+0x40) match.
pub fn ni_tri_based_geom_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if e.call(NI_GEOMETRY_DATA_IS_EQUAL, &args![this, other]).u32() as u8 == 0 {
        return false;
    }
    e.mem.u16(this.addr() + 0x40) == e.mem.u16(other.addr() + 0x40)
}

// Translated from 00a8aa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::_scalar_deleting_destructor_` (Xbox PDB; thiscall, `RET 4`): sets
/// the class's vtable (`0109f2f4`), runs the base destructor body (`00a67c70`), then frees
/// the 0x44-byte object (`00aa1460`) when bit 0 of `flags` is set. Returns `this`.
pub fn ni_tri_based_geom_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_TRI_BASED_GEOM_DATA_VTABLE);
    e.call(NI_GEOMETRY_DATA_DESTRUCT_00A67C70, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x44u32]);
    }
    this
}

// Translated from 00a8aab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriBasedGeomData::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiGeometryData::GetViewerStrings`, then the class name (the first word of `ms_RTTI`) and
/// `"m_usTriangles"` with the triangle count (+0x40), each held in the argument's stack word
/// and added at the array's end.
pub fn ni_tri_based_geom_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr<NiTArray>,
) {
    e.call(NI_GEOMETRY_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_TRI_BASED_GEOM_DATA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let triangles = u32::from(e.mem.u16(this.addr() + 0x40));
        let text = e
            .call(VIEWER_STRING_U16, &args![TEXT_TRIANGLES, triangles])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_aa30, ni_tri_based_geom_data_load_binary(Ptr, Ptr)),
        entry!(0x00a8_aa70, ni_tri_based_geom_data_save_binary(Ptr, Ptr)),
        entry!(0x00a8a9f0, ni_tri_based_geom_data_get_rtti() -> Ptr),
        entry!(
            0x00a8_a970,
            ni_tri_based_geom_data_ni_tri_based_geom_data(Ptr) -> Ptr
        ),
        entry!(0x00a8_a9c0, ni_tri_based_geom_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a8_aa00,
            ni_tri_based_geom_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a8_aab0,
            ni_tri_based_geom_data_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_tri_based_geom_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a8a9f0, &args![]).u32(),
            NI_TRI_BASED_GEOM_DATA_RTTI
        );
    }

    #[test]
    fn ni_tri_based_geom_data_ni_tri_based_geom_data_clears_the_count_and_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(NI_GEOMETRY_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x40, 0xffff_ffff);
            Ret::default()
        });
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_a970, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_GEOMETRY_DATA_CONSTRUCT)
            .collect();
        assert_eq!(base.len(), 1);
        assert_eq!(base[0].1, [this]);
        assert_eq!(e.mem.u32(this), NI_TRI_BASED_GEOM_DATA_VTABLE);
        // Only the 16-bit word is cleared.
        assert_eq!(e.mem.u32(this + 0x40), 0xffff_0000);
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;

    const TRANSFER: u32 = 0x0e72_0000;

    fn run(address: u32, base: u32, slot: u32, function_slot: u32) {
        let mut e = Engine::new();
        e.register(base, |_, _| Ret::default());
        e.register(TRANSFER, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            e.mem.set_u16(a[1], 0x1234);
            2u32.into_ret()
        });
        let stream = e.mem.alloc(0x5c4);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 4, 10);
        e.mem.set_u32(binary + function_slot, TRANSFER);
        e.mem.set_u32(stream + slot, binary);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(address, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(order, vec![address, base, TRANSFER]);
        assert_eq!(log[1].1, vec![this, stream]);
        assert_eq!(log[2].1[..3], [binary, this + 0x40, 2]);
        assert_eq!(e.mem.u16(this + 0x40), 0x1234);
        assert_eq!(e.mem.u32(binary + 4), 12);
    }

    #[test]
    fn load_binary_reads_the_triangle_count() {
        run(0x00a8_aa30, NI_GEOMETRY_DATA_LOAD_BINARY, 0x24c, 8);
    }

    #[test]
    fn save_binary_writes_the_triangle_count() {
        run(0x00a8_aa70, NI_GEOMETRY_DATA_SAVE_BINARY, 0x250, 0xc);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    /// A strings array of capacity 1 (grow-by 4) whose `SetSize`/`SetAt` doubles keep the
    /// elements in a 16-word block.
    fn strings_array(e: &mut Engine) -> u32 {
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
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 4);
        strings
    }

    fn strings_of(e: &Engine, strings: u32) -> Vec<u32> {
        let base = e.mem.u32(strings + 4);
        (0..u32::from(e.mem.u16(strings + 0xa)))
            .map(|i| e.mem.u32(base + i * 4))
            .collect()
    }

    #[test]
    fn is_equal_needs_the_base_and_the_triangle_count() {
        let mut e = Engine::new();
        e.register(NI_GEOMETRY_DATA_IS_EQUAL, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0] + 4)).into_ret()
        });
        let a = e.mem.alloc(0x44);
        let b = e.mem.alloc(0x44);
        e.mem.set_u16(a + 0x40, 3);
        e.mem.set_u16(b + 0x40, 3);
        assert_eq!(e.call(0x00a8_a9c0, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(a + 4, 1);
        assert_eq!(e.call(0x00a8_a9c0, &args![a, b]).u32() & 0xff, 1);
        e.mem.set_u16(b + 0x40, 4);
        assert_eq!(e.call(0x00a8_a9c0, &args![a, b]).u32() & 0xff, 0);
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(NI_GEOMETRY_DATA_DESTRUCT_00A67C70, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x44);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_aa00, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TRI_BASED_GEOM_DATA_VTABLE);
        e.call(0x00a8_aa00, &args![this, 1u32]);
        let deletes: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == SIZED_DELETE)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(deletes, vec![vec![this, 0x44]]);
    }

    #[test]
    fn viewer_strings_add_the_class_and_the_triangle_count() {
        let mut e = Engine::new();
        e.map(NI_TRI_BASED_GEOM_DATA_RTTI, 4);
        e.set_global(NI_TRI_BASED_GEOM_DATA_RTTI, 0x0104_0000u32);
        e.register(NI_GEOMETRY_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_U16, |_, a| {
            assert_eq!(a[0], TEXT_TRIANGLES);
            (0x0700_0000 + a[1]).into_ret()
        });
        let strings = strings_array(&mut e);
        let this = e.mem.alloc(0x44);
        e.mem.set_u16(this + 0x40, 0x1234);
        e.call(0x00a8_aab0, &args![this, strings]);
        assert_eq!(strings_of(&e, strings), vec![0x0104_0001, 0x0700_1234]);
        assert_eq!(e.mem.u16(strings + 8), 5);
    }
}
