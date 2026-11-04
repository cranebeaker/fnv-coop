//! `fallout shared/decaldata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::__AddChunkData` (Xbox PDB, `00485990`, cdecl: signature, data,
/// size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Whether the data written needs byte swapping (`00401500`, no arguments,
/// `bool` in `AL`).
const SAVE_SWAPS: u32 = 0x0040_1500;
/// Byte-swaps the decal data in place (`00593370`, `this` only).
const SWAP_DECAL_DATA: u32 = 0x0059_3370;

// Translated from 00593320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DecalData::Save` (Xbox PDB; `RET 4`): writes the 0x24 bytes of `this`
/// as a chunk of signature `chunk` (`TESForm::__AddChunkData`), swapped in
/// place (`00593370`) before and after when `00401500` says so.
pub fn decal_data_save(e: &mut Engine, this: Ptr, chunk: u32) {
    if e.call(SAVE_SWAPS, &args![]).bool() {
        e.call(SWAP_DECAL_DATA, &args![this]);
    }
    e.call(ADD_CHUNK_DATA, &args![chunk, this, 0x24u32]);
    if e.call(SAVE_SWAPS, &args![]).bool() {
        e.call(SWAP_DECAL_DATA, &args![this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0059_3320, decal_data_save(Ptr, u32))]
}

#[cfg(test)]
mod n037_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_writes_0x24_bytes_swapped_around_the_write_when_asked() {
        let mut e = Engine::new();
        e.register(ADD_CHUNK_DATA, |_, _| Ret::default());
        e.register(SWAP_DECAL_DATA, |_, _| Ret::default());
        e.register(SAVE_SWAPS, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0059_3320, &[0x5000, 0x4154_4144]);
        assert_eq!(
            calls_to(&e, ADD_CHUNK_DATA),
            vec![vec![0x4154_4144, 0x5000, 0x24]]
        );
        assert!(calls_to(&e, SWAP_DECAL_DATA).is_empty());
        e.register(SAVE_SWAPS, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0059_3320, &[0x5000, 0x4154_4144]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x0059_3320,
                SAVE_SWAPS,
                SWAP_DECAL_DATA,
                ADD_CHUNK_DATA,
                SAVE_SWAPS,
                SWAP_DECAL_DATA
            ]
        );
    }
}
