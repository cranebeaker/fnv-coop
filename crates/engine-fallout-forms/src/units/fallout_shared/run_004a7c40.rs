//! `fallout shared/run_004a7c40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::bgssceneinfo::BGSSceneInfo;

/// `BGSSceneInfo`'s vtable (RTTI `.?AVBGSSceneInfo@@`).
const BGS_SCENE_INFO_VTABLE: u32 = 0x0101_ec68;
/// `004a7c80` (`thiscall`, no stack words): clears the 0x40-byte `SCENE_INFO_DATA_STRUCT`
/// (`00403d30`, the `memset` wrapper) and answers it.
const SCENE_DATA_CLEAR: u32 = 0x004a_7c80;
/// `00403d30` (`cdecl`: destination, byte, count; the `memset` wrapper).
const MEMSET: u32 = 0x0040_3d30;

// Translated from 004a7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a `BGSSceneInfo` constructor (`thiscall`, no stack words):
/// stores the vtable, clears `SceneData` (+0x04) twice (`004a7c80`, then `00403d30` with
/// 0 and 0x40) and clears `pAssociatedForm`. Returns `this`.
pub fn fn_004a7c40(e: &mut Engine, this: Ptr<BGSSceneInfo>) -> Ptr<BGSSceneInfo> {
    let base = this.addr();
    e.mem.set_u32(base, BGS_SCENE_INFO_VTABLE);
    e.call(SCENE_DATA_CLEAR, &args![base + 4]);
    e.call(MEMSET, &args![base + 4, 0u32, 0x40u32]);
    e.set(this, BGSSceneInfo::pAssociatedForm, Ptr::NULL);
    this
}

// Translated from 004a7c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): clears the
/// 0x40-byte `SCENE_INFO_DATA_STRUCT` at `ECX` (`00403d30` with 0 and 0x40)
/// and returns it.
pub fn fn_004a7c80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMSET, &args![this, 0u32, 0x40u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004a_7c40,
            fn_004a7c40(Ptr<BGSSceneInfo>) -> Ptr<BGSSceneInfo>
        ),
        entry!(0x004a_7c80, fn_004a7c80(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    #[test]
    fn scene_data_clear_zeroes_the_struct_and_returns_it() {
        let mut e = Engine::new();
        e.register(MEMSET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004a_7c80, &args![0x5000u32]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMSET, vec![0x5000, 0, 0x40])));
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_004a7c40_sets_the_vtable_and_clears_the_data() {
        let mut e = Engine::new();
        e.register(SCENE_DATA_CLEAR, |_, a| a[0].into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        let this: Ptr<BGSSceneInfo> = e.new_object();
        e.mem.set_u32(this.addr() + 0x44, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004a_7c40, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BGS_SCENE_INFO_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x44), 0);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (SCENE_DATA_CLEAR, vec![this.addr() + 4]),
                (MEMSET, vec![this.addr() + 4, 0, 0x40]),
            ]
        );
    }
}
