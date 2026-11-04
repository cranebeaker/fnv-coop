//! `fallout/misc/memorylevelmanager.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `GarbageCollector::ClearAll` (`00868d70`, engine map; `cdecl`, one word).
const GARBAGE_COLLECTOR_CLEAR_ALL: u32 = 0x0086_8d70;
/// `00450b80` (`cdecl`, one word; `fallout shared/tes.cpp`): the object
/// `00b5fd60` (`thiscall`, `BSShader`) then runs on.
const TES_OBJECT_00450B80: u32 = 0x0045_0b80;
const SHADER_FREE_00B5FD60: u32 = 0x00b5_fd60;
/// `00c459d0` (`cdecl`, one word; `BSMain`).
const MAIN_FREE_00C459D0: u32 = 0x00c4_59d0;
/// `00651e30` and `00651f40` (no arguments; `bsfacegenmanager.cpp`, both
/// work on the face generation manager at `011d59e8`).
const FACEGEN_FREE_00651E30: u32 = 0x0065_1e30;
const FACEGEN_FREE_00651F40: u32 = 0x0065_1f40;
/// The model loader pointer (`011c3b3c`) and `00448620` (`thiscall`, one
/// word; `modelloader.cpp`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_FREE_00448620: u32 = 0x0044_8620;
/// The `TES` pointer (`011dea10`) and `TES::CleanUpUnusedTextures`
/// (`00452490`, engine map; `thiscall`, one word).
const TES: u32 = 0x011d_ea10;
const TES_CLEAN_UP_UNUSED_TEXTURES: u32 = 0x0045_2490;

// Translated from 00878250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryLevelManager::FreeReleasedObjects` (Xbox PDB; `cdecl`, one byte):
/// `GarbageCollector::ClearAll(1)`, `00b5fd60` on `00450b80(0)`'s object,
/// `00c459d0(0)`; with `face_gen` the two face generation frees (`00651e30`,
/// `00651f40`); then `00448620(1)` on the model loader and
/// `TES::CleanUpUnusedTextures(0)`.
pub fn memory_level_manager_free_released_objects(e: &mut Engine, face_gen: u8) {
    e.call(GARBAGE_COLLECTOR_CLEAR_ALL, &args![1u32]);
    let object = e.call(TES_OBJECT_00450B80, &args![0u32]).u32();
    e.call(SHADER_FREE_00B5FD60, &args![object]);
    e.call(MAIN_FREE_00C459D0, &args![0u32]);
    if face_gen != 0 {
        e.call(FACEGEN_FREE_00651E30, &[]);
        e.call(FACEGEN_FREE_00651F40, &[]);
    }
    let loader = e.mem.u32(MODEL_LOADER);
    e.call(MODEL_LOADER_FREE_00448620, &args![loader, 1u32]);
    let tes = e.mem.u32(TES);
    e.call(TES_CLEAN_UP_UNUSED_TEXTURES, &args![tes, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00878250,
        memory_level_manager_free_released_objects(u8)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frees_in_order_with_face_gen_optional() {
        for face_gen in [0u32, 1] {
            let mut e = Engine::new();
            for address in [
                GARBAGE_COLLECTOR_CLEAR_ALL,
                SHADER_FREE_00B5FD60,
                MAIN_FREE_00C459D0,
                FACEGEN_FREE_00651E30,
                FACEGEN_FREE_00651F40,
                MODEL_LOADER_FREE_00448620,
                TES_CLEAN_UP_UNUSED_TEXTURES,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            e.register(TES_OBJECT_00450B80, |_, _| 0x7700u32.into_ret());
            e.map(0x011c_3000, 0x1000);
            e.map(0x011d_e000, 0x1000);
            e.set_global(MODEL_LOADER, 0x1111u32);
            e.set_global(TES, 0x2222u32);
            e.call_log = Some(vec![]);
            e.call(0x0087_8250, &args![face_gen | 0x100]);
            let log: Vec<(u32, Vec<u32>)> =
                e.call_log.take().unwrap().into_iter().skip(1).collect();
            let mut expected = vec![
                (GARBAGE_COLLECTOR_CLEAR_ALL, vec![1]),
                (TES_OBJECT_00450B80, vec![0]),
                (SHADER_FREE_00B5FD60, vec![0x7700]),
                (MAIN_FREE_00C459D0, vec![0]),
            ];
            if face_gen != 0 {
                expected.push((FACEGEN_FREE_00651E30, vec![]));
                expected.push((FACEGEN_FREE_00651F40, vec![]));
            }
            expected.push((MODEL_LOADER_FREE_00448620, vec![0x1111, 1]));
            expected.push((TES_CLEAN_UP_UNUSED_TEXTURES, vec![0x2222, 0]));
            assert_eq!(log, expected);
        }
    }
}
