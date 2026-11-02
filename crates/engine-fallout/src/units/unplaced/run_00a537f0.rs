//! `(unplaced)/run_00a537f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNew`'s allocator (`00aa13e0`, cdecl, the size in bytes).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiObject` constructor (`00a5d3a0`: `NiRefObject`'s vtable and count,
/// the live-object counter, then `NiObject`'s vtable).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The vtable of `NiBoolData` (RTTI `.?AVNiBoolData@@`).
const NI_BOOL_DATA_VTABLE: u32 = 0x0109_a1d4;
/// Size of an `NiBoolData` (Xbox PDB 0x18, the allocation's size).
const NI_BOOL_DATA_SIZE: u32 = 0x18;
/// `NiBoolData` (Xbox PDB, offsets as the PC code writes them):
/// `m_uiNumKeys` +0x08, `m_pkKeys` +0x0c, `m_eType` +0x10, `m_ucKeySize`
/// +0x14 (a byte).
const NUM_KEYS: u32 = 0x08;
const KEYS: u32 = 0x0c;
const KEY_TYPE: u32 = 0x10;
const KEY_SIZE: u32 = 0x14;

// Translated from 00a537f0 (disassembled, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `NiBoolData::CreateObject` (by the vtable it stores; the code is not a
/// Ghidra function): allocates 0x18 bytes; null when that fails. Otherwise
/// the `NiObject` constructor, no keys (count, key pointer and key type 0,
/// key size byte 0) and the `NiBoolData` vtable.
pub fn ni_bool_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_BOOL_DATA_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object + NUM_KEYS, 0);
    e.mem.set_u32(object + KEYS, 0);
    e.mem.set_u32(object + KEY_TYPE, 0);
    e.mem.set_u8(object + KEY_SIZE, 0);
    e.mem.set_u32(object, NI_BOOL_DATA_VTABLE);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_37f0, ni_bool_data_create_object() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_object_builds_an_empty_bool_data_or_gives_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            // Garbage the constructor must overwrite.
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(NI_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let object = e.call(0x00a5_37f0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.block_size(object), Some(NI_BOOL_DATA_SIZE));
        assert_eq!(e.mem.u32(object), NI_BOOL_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + NUM_KEYS), 0);
        assert_eq!(e.mem.u32(object + KEYS), 0);
        assert_eq!(e.mem.u32(object + KEY_TYPE), 0);
        // Only the low byte of the last word is written.
        assert_eq!(e.mem.u32(object + KEY_SIZE), 0xdead_be00);
        let constructed: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == NI_OBJECT_CONSTRUCT)
            .collect();
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0].1, vec![object]);

        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_37f0, &args![]).u32(), 0);
        assert!(!e
            .call_log
            .iter()
            .flatten()
            .any(|(a, _)| *a == NI_OBJECT_CONSTRUCT));
    }
}
