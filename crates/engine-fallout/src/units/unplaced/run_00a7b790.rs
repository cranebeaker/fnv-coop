//! `(unplaced)/run_00a7b790` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNew`'s allocator (`00aa13e0`, cdecl, the size in bytes).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiTriBasedGeomData::NiTriBasedGeomData` (`00a8a970`, engine map).
const NI_TRI_BASED_GEOM_DATA_CONSTRUCT: u32 = 0x00a8_a970;
/// The vtable of `NiTriShapeData` (RTTI `.?AVNiTriShapeData@@`).
const NI_TRI_SHAPE_DATA_VTABLE: u32 = 0x0109_dac4;
/// Size of an `NiTriShapeData` (Xbox PDB 0x58, the allocation's size).
const NI_TRI_SHAPE_DATA_SIZE: u32 = 0x58;
/// `NiTriShapeData` (Xbox PDB, the PC code's offsets): `m_uiTriListLength`
/// +0x44, `m_pusTriList` +0x48, `m_pkSharedNormals` +0x4c,
/// `m_usSharedNormalsArraySize` +0x50 (u16), `m_pkSNAMemoryBlocks` +0x54.
const TRI_LIST_LENGTH: u32 = 0x44;
const TRI_LIST: u32 = 0x48;
const SHARED_NORMALS: u32 = 0x4c;
const SHARED_NORMALS_ARRAY_SIZE: u32 = 0x50;
const SNA_MEMORY_BLOCKS: u32 = 0x54;

// Translated from 00a7b790 (disassembled, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `NiTriShapeData::CreateObject` (by the vtable it stores; the code is not
/// a Ghidra function): allocates 0x58 bytes; null when that fails.
/// Otherwise the `NiTriBasedGeomData` constructor, an empty triangle list
/// and shared normals (the words at +0x44, +0x48, +0x4c, +0x54 and the
/// `u16` at +0x50 cleared; the two bytes at +0x52 are left as they are) and
/// the `NiTriShapeData` vtable.
pub fn ni_tri_shape_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_TRI_SHAPE_DATA_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_TRI_BASED_GEOM_DATA_CONSTRUCT, &args![object]);
    e.mem.set_u32(object + TRI_LIST_LENGTH, 0);
    e.mem.set_u32(object + TRI_LIST, 0);
    e.mem.set_u32(object + SHARED_NORMALS, 0);
    e.mem.set_u32(object + SNA_MEMORY_BLOCKS, 0);
    e.mem.set_u16(object + SHARED_NORMALS_ARRAY_SIZE, 0);
    e.mem.set_u32(object, NI_TRI_SHAPE_DATA_VTABLE);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_b790, ni_tri_shape_data_create_object() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_object_builds_an_empty_tri_shape_data_or_gives_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(NI_TRI_BASED_GEOM_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 0x40, 0x0005_0005);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let object = e.call(0x00a7_b790, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.block_size(object), Some(NI_TRI_SHAPE_DATA_SIZE));
        assert_eq!(e.mem.u32(object), NI_TRI_SHAPE_DATA_VTABLE);
        // The base's fields are the constructor's.
        assert_eq!(e.mem.u32(object + 0x40), 0x0005_0005);
        assert_eq!(e.mem.u32(object + TRI_LIST_LENGTH), 0);
        assert_eq!(e.mem.u32(object + TRI_LIST), 0);
        assert_eq!(e.mem.u32(object + SHARED_NORMALS), 0);
        assert_eq!(e.mem.u32(object + SNA_MEMORY_BLOCKS), 0);
        // Only the low half of the word at +0x50 is written.
        assert_eq!(e.mem.u32(object + SHARED_NORMALS_ARRAY_SIZE), 0xdead_0000);
        let constructed: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == NI_TRI_BASED_GEOM_DATA_CONSTRUCT)
            .collect();
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0].1, vec![object]);

        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_b790, &args![]).u32(), 0);
        assert!(!e
            .call_log
            .iter()
            .flatten()
            .any(|(a, _)| *a == NI_TRI_BASED_GEOM_DATA_CONSTRUCT));
    }
}
