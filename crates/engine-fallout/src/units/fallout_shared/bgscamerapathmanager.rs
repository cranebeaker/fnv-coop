//! `fallout shared/bgscamerapathmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The manager (`this`) is a form array of root camera paths (`NiFormArray`
//! at +0) with a second array of every path at +0x18.

#[allow(unused_imports)]
use crate::prelude::*;

/// The default camera path (`BGSCameraPath*`, `011ca704`).
const DEFAULT_PATH: u32 = 0x011c_a704;
/// `004839c0` (`cdecl`): the form with a form ID.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (engine map), `00ec43fb` (`cdecl`: object, 0, source
/// type, target type, 0).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptor of `TESForm` (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The RTTI type descriptor of `BGSCameraPath` (`.?AVBGSCameraPath@@`).
const TYPE_CAMERA_PATH: u32 = 0x0118_c268;
/// `00401000` (`cdecl`, one word): allocates a block.
const ALLOCATE: u32 = 0x0040_1000;
/// `BGSCameraPath::BGSCameraPath` (engine map), `0058a380`.
const CAMERA_PATH_CONSTRUCT: u32 = 0x0058_a380;
/// `"DefaultCameraPath"`.
const DEFAULT_PATH_NAME: u32 = 0x0103_2730;
/// `0084e3a0` (`thiscall`): the number of entries of a form array.
const ARRAY_COUNT: u32 = 0x0084_e3a0;
/// `0061bc10` (`thiscall`, `RET 8`): inserts a form into a form array at an
/// index.
const ARRAY_INSERT: u32 = 0x0061_bc10;
/// `NiFormArray::GetFormAt` (engine map), `004d6170`.
const ARRAY_FORM_AT: u32 = 0x004d_6170;
/// `00658930` (`thiscall`): the number of entries of the all-paths array.
const PATHS_COUNT: u32 = 0x0065_8930;
/// `00877a30` (`thiscall`, `RET 4`): the address of entry `i` (data +
/// 4 i).
const PATHS_ENTRY: u32 = 0x0087_7a30;
/// `0096ae90` (`thiscall`, `RET 8`): stores the word at an address into
/// entry `i`.
const PATHS_SET: u32 = 0x0096_ae90;
/// `BGSCameraPath::InitParents` (engine map), `0058aef0`.
const CAMERA_PATH_INIT_PARENTS: u32 = 0x0058_aef0;

/// `__RTDynamicCast(form, 0, TESForm, BGSCameraPath, 0)`.
fn as_camera_path(e: &mut Engine, form: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_CAMERA_PATH, 0u32],
    )
    .u32()
}

// Translated from 0058b850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058b850 - the C++ exception unwinding states (FS:[0] frame, handler 00eed64b) are not translated
/// `BGSCameraPathManager::CreateDefaultPath` (Xbox PDB, matched by the
/// engine map): without a default path, the form 0x16 cast to a camera
/// path becomes it; when there is none, a new 0x38-byte path
/// (`BGSCameraPath::BGSCameraPath`; null when the allocation fails) gets
/// the form ID 0x16 (virtual slot 0x128 with 1), the editor ID
/// `"DefaultCameraPath"` (slot 0x134) and is appended to the root array.
pub fn bgs_camera_path_manager_create_default_path(e: &mut Engine, this: Ptr) {
    if e.mem.u32(DEFAULT_PATH) != 0 {
        return;
    }
    let form = e.call(LOOKUP_FORM, &args![0x16u32]).u32();
    let path = as_camera_path(e, form);
    e.mem.set_u32(DEFAULT_PATH, path);
    if e.mem.u32(DEFAULT_PATH) != 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0x38u32]).u32();
    let path = if block == 0 {
        0
    } else {
        e.call(CAMERA_PATH_CONSTRUCT, &args![block]).u32()
    };
    e.mem.set_u32(DEFAULT_PATH, path);
    let path = e.mem.u32(DEFAULT_PATH);
    e.vcall(path, 0x128, &args![0x16u32, 1u32]);
    let path = e.mem.u32(DEFAULT_PATH);
    e.vcall(path, 0x134, &args![DEFAULT_PATH_NAME]);
    let path = e.mem.u32(DEFAULT_PATH);
    let count = e.call(ARRAY_COUNT, &args![this]).u32();
    e.call(ARRAY_INSERT, &args![this, count, path]);
}

// Translated from 0058b960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The default camera path (`011ca704`; no Xbox PDB name, no arguments).
pub fn fn_0058b960(e: &mut Engine) -> u32 {
    e.mem.u32(DEFAULT_PATH)
}

// Translated from 0058b970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes every camera path of the array at +0x18 (no Xbox PDB name):
/// a null entry or one `004013e0` marks is cleared (`0096ae90` with a
/// zero local); any other runs `InitItem` (virtual slot 0x88) and
/// `BGSCameraPath::InitParents` (`0058aef0`), and its entry is cleared
/// when that answers true. Then `00600bc0` on the array.
pub fn fn_0058b970(e: &mut Engine, this: Ptr) {
    let paths = this.addr() + 0x18;
    let count = e.call(PATHS_COUNT, &args![paths]).u32();
    let mut i = 0u32;
    while i < count {
        let entry = e.call(PATHS_ENTRY, &args![paths, i]).u32();
        let path = e.mem.u32(entry);
        let clear = if path == 0 || e.call(0x0040_13e0, &args![path]).bool() {
            true
        } else {
            e.vcall(path, 0x88, &args![]);
            e.call(CAMERA_PATH_INIT_PARENTS, &args![path]).bool()
        };
        if clear {
            e.with_stack(4, |e, zero| {
                e.mem.set_u32(zero.addr(), 0);
                e.call(PATHS_SET, &args![paths, i, zero]);
            });
        }
        i += 1;
    }
    e.call(0x0060_0bc0, &args![paths]);
}

// Translated from 0058ba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSCameraPathManager::DestroyRootPathArrayForms` (Xbox PDB, matched by
/// the engine map): deletes (virtual slot 0x10 with 1) every root entry
/// that is a camera path and every non-null entry of the array at +0x18,
/// then `005e03d0` on that array, clears the default path and runs
/// `00863db0` on the root array.
pub fn bgs_camera_path_manager_destroy_root_path_array_forms(e: &mut Engine, this: Ptr) {
    let count = e.call(ARRAY_COUNT, &args![this]).u32();
    let mut i = 0u32;
    while i < count {
        let form = e.call(ARRAY_FORM_AT, &args![this, i]).u32();
        let path = as_camera_path(e, form);
        if path != 0 {
            e.vcall(path, 0x10, &args![1u32]);
        }
        i += 1;
    }
    let paths = this.addr() + 0x18;
    let count = e.call(PATHS_COUNT, &args![paths]).u32();
    let mut i = 0u32;
    while i < count {
        let entry = e.call(PATHS_ENTRY, &args![paths, i]).u32();
        let path = e.mem.u32(entry);
        if path != 0 {
            e.vcall(path, 0x10, &args![1u32]);
        }
        i += 1;
    }
    e.call(0x005e_03d0, &args![paths]);
    e.mem.set_u32(DEFAULT_PATH, 0);
    e.call(0x0086_3db0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058b850, bgs_camera_path_manager_create_default_path(Ptr)),
        entry!(0x0058b960, fn_0058b960() -> u32),
        entry!(0x0058b970, fn_0058b970(Ptr)),
        entry!(
            0x0058ba40,
            bgs_camera_path_manager_destroy_root_path_array_forms(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose vtable sends `slots` to doubles at `0x00ab_0000 +
    /// slot`.
    fn object(e: &mut Engine, slots: &[u32]) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(object, vtable);
        for slot in slots {
            e.mem.set_u32(vtable + slot, 0x00ab_0000 + slot);
            e.register(0x00ab_0000 + slot, |_, _| Ret::default());
        }
        object
    }

    #[test]
    fn default_path_is_found_or_made() {
        let mut e = engine();
        e.register(LOOKUP_FORM, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        e.register(DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call(0x0058_b850, &args![0x9000u32]);
        assert_eq!(e.mem.u32(DEFAULT_PATH), 0x5000);
        assert!(calls(&e, ALLOCATE).is_empty());
        // Present already: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0058_b850, &args![0x9000u32]);
        assert!(calls(&e, LOOKUP_FORM).is_empty());
        // Not found: a new path is made and appended.
        e.mem.set_u32(DEFAULT_PATH, 0);
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        let path = object(&mut e, &[0x128, 0x134]);
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        e.register_double(CAMERA_PATH_CONSTRUCT, move |_, _| Ret {
            eax: path,
            ..Ret::default()
        });
        e.register(ARRAY_COUNT, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(ARRAY_INSERT, |_, _| Ret::default());
        e.call(0x0058_b850, &args![0x9000u32]);
        assert_eq!(e.mem.u32(DEFAULT_PATH), path);
        assert_eq!(calls(&e, CAMERA_PATH_CONSTRUCT), vec![vec![0x7000]]);
        assert_eq!(calls(&e, 0x00ab_0128), vec![vec![path, 0x16, 1]]);
        assert_eq!(calls(&e, 0x00ab_0134), vec![vec![path, DEFAULT_PATH_NAME]]);
        assert_eq!(calls(&e, ARRAY_INSERT), vec![vec![0x9000, 3, path]]);
        assert_eq!(e.call(0x0058_b960, &args![]).u32(), path);
    }

    /// The all-paths array: `entries` in a data block, its count and entry
    /// addresses through doubles; `0096ae90` must get a zero word.
    fn paths(e: &mut Engine, entries: &[u32]) {
        let data = e.mem.alloc(4 * entries.len() as u32 + 4);
        for (i, p) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *p);
        }
        let n = entries.len() as u32;
        e.register_double(PATHS_COUNT, move |_, _| Ret {
            eax: n,
            ..Ret::default()
        });
        e.register_double(PATHS_ENTRY, move |_, a| Ret {
            eax: data + 4 * a[1],
            ..Ret::default()
        });
        e.register(PATHS_SET, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
    }

    #[test]
    fn initialization_clears_missing_and_reparented_paths() {
        let mut e = engine();
        let kept = object(&mut e, &[0x88]);
        let moved = object(&mut e, &[0x88]);
        let marked = object(&mut e, &[0x88]);
        paths(&mut e, &[0, kept, moved, marked]);
        e.register_double(0x0040_13e0, move |_, a| Ret {
            eax: (a[0] == marked) as u32,
            ..Ret::default()
        });
        e.register_double(CAMERA_PATH_INIT_PARENTS, move |_, a| Ret {
            eax: (a[0] == moved) as u32,
            ..Ret::default()
        });
        e.register(0x0060_0bc0, |_, _| Ret::default());
        let manager = 0x9000u32;
        e.call(0x0058_b970, &args![manager]);
        let cleared: Vec<u32> = calls(&e, PATHS_SET).iter().map(|w| w[1]).collect();
        assert_eq!(cleared, vec![0, 2, 3]);
        assert_eq!(calls(&e, 0x00ab_0088).len(), 2);
        assert_eq!(calls(&e, 0x0060_0bc0), vec![vec![manager + 0x18]]);
    }

    #[test]
    fn destroy_deletes_the_root_paths_and_every_path() {
        let mut e = engine();
        e.mem.set_u32(DEFAULT_PATH, 0x1234);
        let root = object(&mut e, &[0x10]);
        let other = object(&mut e, &[0x10]);
        e.register(ARRAY_COUNT, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register_double(ARRAY_FORM_AT, move |_, a| Ret {
            eax: if a[1] == 0 { root } else { 0x4444 },
            ..Ret::default()
        });
        e.register_double(DYNAMIC_CAST, move |_, a| Ret {
            eax: if a[0] == root { root } else { 0 },
            ..Ret::default()
        });
        paths(&mut e, &[other, 0]);
        e.register(0x005e_03d0, |_, _| Ret::default());
        e.register(0x0086_3db0, |_, _| Ret::default());
        e.call(0x0058_ba40, &args![0x9000u32]);
        assert_eq!(calls(&e, 0x00ab_0010), vec![vec![root, 1], vec![other, 1]]);
        assert_eq!(e.mem.u32(DEFAULT_PATH), 0);
        assert_eq!(calls(&e, 0x005e_03d0), vec![vec![0x9018]]);
        assert_eq!(calls(&e, 0x0086_3db0), vec![vec![0x9000]]);
    }
}
