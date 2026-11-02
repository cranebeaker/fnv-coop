//! `fallout shared/bgsdefaultobjectmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0058d820` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0058d7f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_d820, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `BGSDefaultObjectManager::Instance` (Xbox PDB, `0058d680`): the singleton, built on first use.
const DEFAULT_OBJECT_MANAGER_INSTANCE: u32 = 0x0058_d680;
/// `004013e0` (unnamed in the engine map): `thiscall`; whether bit 3 (mask 8) of the dword at
/// `this + 8` is set, in `AL`.
const MANAGER_IS_LOADED: u32 = 0x0040_13e0;

// Translated from 0058db10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDefaultObjectManager::GetDefaultObject` (Xbox PDB): `cdecl`, one stack word (the index).
/// Zero when the flag test (`004013e0`) on the singleton is false; otherwise the dword at
/// `instance + 0x18 + index * 4` (the singleton is fetched a second time for it, as in the exe).
pub fn bgs_default_object_manager_get_default_object(e: &mut Engine, index: u32) -> u32 {
    let manager = e.call(DEFAULT_OBJECT_MANAGER_INSTANCE, &args![]).u32();
    if !e.call(MANAGER_IS_LOADED, &args![manager]).bool() {
        return 0;
    }
    let manager = e.call(DEFAULT_OBJECT_MANAGER_INSTANCE, &args![]).u32();
    e.mem.u32(
        manager
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_add(0x18),
    )
}

/// The singleton (`011ca80c`).
const INSTANCE: u32 = 0x011c_a80c;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The vtable `01032dc4` (RTTI `.?AVBGSDefaultObjectManager@@`).
const VTABLE: u32 = 0x0103_2dc4;
/// `TESForm::TESForm` (Xbox PDB, `00483370`, `ECX`), `TESForm::~TESForm` (`00483630`,
/// `ECX`), `004f15a0` (`ECX`, one stack word: the form type), `TESForm::SetFormID` (Xbox
/// PDB, `00485c10`, `ECX`, two stack words) and `00401290` (`ECX`, one stack word: a shared
/// `return true` body some vtables use as their editor id setter).
const TES_FORM_CTOR: u32 = 0x0048_3370;
const TES_FORM_DTOR: u32 = 0x0048_3630;
const SET_FORM_TYPE: u32 = 0x004f_15a0;
const SET_FORM_ID: u32 = 0x0048_5c10;
const RETURN_TRUE_00401290: u32 = 0x0040_1290;
/// `"DefaultObjectManager"`.
const EDITOR_ID: u32 = 0x0103_2da4;
/// The game's `memset` wrapper (`00403d30`, `cdecl`: destination, byte, size).
const MEMSET: u32 = 0x0040_3d30;
/// The number of default objects (34 words from +0x18).
const OBJECT_COUNT: u32 = 0x22;
/// The default object table (`0118c360`): per object a name pointer and, at +4, the form type
/// byte it must have.
const OBJECT_TABLE: u32 = 0x0118_c360;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `ECX`, one stack word).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, `cdecl`: address of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `00401170` (`ECX`): the form type byte at +4, zero-extended.
const FORM_TYPE_00401170: u32 = 0x0040_1170;
/// The release build's logging stub (`005b5e40`, `cdecl`, variable words).
const LOG: u32 = 0x005b_5e40;
/// "Unable to find valid '%s' default object for ID (%08X)."
const MESSAGE_NO_DEFAULT_OBJECT: u32 = 0x0103_2efc;
/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type (`AL`).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `00401080` (`cdecl`: address of a word, a byte flag): swaps the word's byte order.
const SWAP_WORD: u32 = 0x0040_1080;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The `DATA` chunk code.
const CHUNK_DATA: u32 = 0x4154_4144;

// Translated from 0058d680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058d680 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f109cb, state variable) is not translated
/// `BGSDefaultObjectManager::Instance` (Xbox PDB), no words: the singleton (`011ca80c`), made
/// on first use (`00401000` of 0xa0 bytes and the constructor `0058d760`; null when the
/// allocation fails).
pub fn bgs_default_object_manager_instance(e: &mut Engine) -> u32 {
    if e.global::<u32>(INSTANCE) == 0 {
        let block = e.call(ALLOCATE, &args![0xa0u32]).u32();
        let manager = if block != 0 {
            bgs_default_object_manager_constructor(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.set_global(INSTANCE, manager);
    }
    e.global::<u32>(INSTANCE)
}

// Translated from 0058d760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058d760 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef8ae8, state variable) is not translated
/// `BGSDefaultObjectManager::BGSDefaultObjectManager` (Xbox PDB), `ECX`: the `TESForm` base,
/// the vtable `01032dc4`, form type 0x64 (`004f15a0`), form id 0x31 (`TESForm::SetFormID`
/// with 1), `00401290` with `"DefaultObjectManager"`, then the data cleared ([`fn_0058d840`]);
/// answers `this`.
pub fn bgs_default_object_manager_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TES_FORM_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(SET_FORM_TYPE, &args![this, 0x64u32]);
    e.call(SET_FORM_ID, &args![this, 0x31u32, 1u32]);
    e.call(RETURN_TRUE_00401290, &args![this, EDITOR_ID]);
    fn_0058d840(e, this);
    this
}

// Translated from 0058d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (the constructor runs it; `ECX`): clears the 34 default objects
/// (`memset` of 0x88 bytes at +0x18).
pub fn fn_0058d840(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this.addr() + 0x18, 0u32, 0x88u32]);
}

// Translated from 0058d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, no words): when there is a singleton, it is deleted (its
/// vtable slot +0x10 with 1) and the singleton cleared.
pub fn fn_0058d710(e: &mut Engine) {
    if e.global::<u32>(INSTANCE) == 0 {
        return;
    }
    let manager = e.global::<u32>(INSTANCE);
    if manager != 0 {
        e.vcall(manager, 0x10, &args![1u32]);
    }
    e.set_global(INSTANCE, 0u32);
}

// Translated from 0058d820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (`ECX`; the scalar deleting destructor `0058d7f0` runs it): the vtable
/// `01032dc4`, then `TESForm::~TESForm`.
pub fn fn_0058d820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(TES_FORM_DTOR, &args![this]);
}

// Translated from 0058d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSDefaultObjectManager` `Load` (unnamed in the map; `ECX` and one stack word, the
/// file, `RET 4`): false unless the record's form type (`TESFile::GetTESForm`) is 0x64.
/// Otherwise `TESForm::LoadForm`, form flag 8 cleared, then each chunk: `DATA` reads the 34
/// default object ids (0x88 bytes at +0x18), each word byte-swapped when the file needs it.
/// After the chunks form flag 8 is cleared again; true.
pub fn fn_0058d920(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() as u8 != 0x64 {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        if chunk == CHUNK_DATA {
            e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x18, 0x88u32]);
            if e.call(NEEDS_SWAP, &args![file]).bool() {
                for index in 0..OBJECT_COUNT {
                    e.call(SWAP_WORD, &args![base + 0x18 + index * 4, 0u32]);
                }
            }
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    true
}

// Translated from 0058da10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSDefaultObjectManager` `InitItem` (unnamed in the map; `ECX`): unless form flag 8
/// is set (`004013e0`), each nonzero default object id gets the compile index of the
/// manager's file and becomes its form (`004839c0`); a form whose type (`00401170`) is not the
/// table's (`0118c360`, +4 of the object's 8-byte row) is dropped, and a missing or dropped
/// one is reported with the row's name and the id. Then form flag 8 is set.
pub fn fn_0058da10(e: &mut Engine, this: Ptr) {
    if e.call(MANAGER_IS_LOADED, &args![this]).bool() {
        return;
    }
    e.with_stack(4, |e, id_at| {
        for index in 0..OBJECT_COUNT {
            let slot = this.addr() + 0x18 + index * 4;
            let id = e.mem.u32(slot);
            e.mem.set_u32(id_at.addr(), id);
            if id == 0 {
                continue;
            }
            let file = e.call(GET_FILE, &args![this, u32::MAX]).u32();
            e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
            let id = e.mem.u32(id_at.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            e.mem.set_u32(slot, form);
            if e.mem.u32(slot) != 0 {
                let form = e.mem.u32(slot);
                let form_type = e.call(FORM_TYPE_00401170, &args![form]).u32();
                let expected = u32::from(e.mem.u8(OBJECT_TABLE + index * 8 + 4));
                if form_type != expected {
                    e.mem.set_u32(slot, 0);
                }
            }
            if e.mem.u32(slot) == 0 {
                let name = e.global::<u32>(OBJECT_TABLE + index * 8);
                let id = e.mem.u32(id_at.addr());
                e.call(LOG, &args![MESSAGE_NO_DEFAULT_OBJECT, name, id]);
            }
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

/// `TESForm::StartForm` and `TESForm::CloseForm` (`004855a0`, `00485680`, `thiscall`).
const FORM_START_FORM: u32 = 0x0048_55a0;
const FORM_CLOSE_FORM: u32 = 0x0048_5680;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm::AddChunkArray` (`00485710`, engine map `_ov2`; `cdecl`: chunk signature,
/// address of the words, count).
const ADD_CHUNK_ARRAY: u32 = 0x0048_5710;
/// The manager's default objects (`TESForm*`, `OBJECT_COUNT` of them) from +0x18.
const OBJECTS_OFFSET: u32 = 0x18;

// Translated from 0058d870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the default object manager (`thiscall`, no stack word; unnamed in the engine
/// map): starts the form record, gathers the form ids of its 0x22 default objects (0 for an
/// empty slot) in a stack array and saves them as a `DATA` array chunk (`00485710`), then
/// closes the record.
pub fn fn_0058d870(e: &mut Engine, this: Ptr) {
    e.call(FORM_START_FORM, &args![this]);
    e.with_stack(OBJECT_COUNT * 4, |e, ids| {
        let mut index: i32 = 0;
        while index < OBJECT_COUNT as i32 {
            let offset = index as u32 * 4;
            let object = e.mem.u32(this.addr() + OBJECTS_OFFSET + offset);
            let id = if object != 0 {
                e.call(FORM_ID, &args![object]).u32()
            } else {
                0
            };
            e.mem.set_u32(ids.addr() + offset, id);
            index += 1;
        }
        e.call(ADD_CHUNK_ARRAY, &args![CHUNK_DATA, ids, OBJECT_COUNT]);
    });
    e.call(FORM_CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_d680, bgs_default_object_manager_instance() -> u32),
        entry!(0x0058_d760, bgs_default_object_manager_constructor(Ptr) -> Ptr),
        entry!(0x0058_d840, fn_0058d840(Ptr)),
        entry!(0x0058_d710, fn_0058d710()),
        entry!(0x0058_d820, fn_0058d820(Ptr)),
        entry!(0x0058_d920, fn_0058d920(Ptr, Ptr) -> bool),
        entry!(0x0058_da10, fn_0058da10(Ptr)),
        entry!(0x0058_db10, bgs_default_object_manager_get_default_object(u32) -> u32),
        entry!(0x0058_d7f0, fn_0058d7f0(Ptr, u32) -> Ptr),
        entry!(0x0058_d870, fn_0058d870(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0058d7f0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_d820, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_d7f0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_d7f0)
                .collect();
            let mut expected = vec![(0x0058_d820, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0058db10_answers_the_indexed_default_object_when_the_manager_is_ready() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let manager = e.mem.alloc(0x100);
        e.mem.set_u32(manager + 0x18, 0x1000);
        e.mem.set_u32(manager + 0x1c, 0x2000);
        e.mem.set_u32(manager + 0x18 + 4 * 5, 0x6000);
        e.register_double(DEFAULT_OBJECT_MANAGER_INSTANCE, move |_, _| {
            manager.into_ret()
        });
        e.register(MANAGER_IS_LOADED, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x0058_db10, &args![0u32]).u32(), 0x1000);
        assert_eq!(e.call(0x0058_db10, &args![1u32]).u32(), 0x2000);
        assert_eq!(e.call(0x0058_db10, &args![5u32]).u32(), 0x6000);
        let log = e.call_log.take().unwrap();
        // The singleton is fetched twice per lookup, the flag test gets it as `this`.
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == DEFAULT_OBJECT_MANAGER_INSTANCE)
                .count(),
            6
        );
        assert!(log
            .iter()
            .filter(|c| c.0 == MANAGER_IS_LOADED)
            .all(|c| c.1 == vec![manager]));
    }

    #[test]
    fn at_0058db10_answers_zero_when_the_manager_is_not_ready() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let manager = e.mem.alloc(0x100);
        e.mem.set_u32(manager + 0x18, 0x1000);
        e.register_double(DEFAULT_OBJECT_MANAGER_INSTANCE, move |_, _| {
            manager.into_ret()
        });
        e.register(MANAGER_IS_LOADED, |_, _| Ret::default());
        assert_eq!(e.call(0x0058_db10, &args![0u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == DEFAULT_OBJECT_MANAGER_INSTANCE)
                .count(),
            1
        );
    }
}

#[cfg(test)]
mod manager_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        for addr in [
            TES_FORM_CTOR,
            TES_FORM_DTOR,
            SET_FORM_TYPE,
            SET_FORM_ID,
            RETURN_TRUE_00401290,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_0058d680_makes_the_singleton_once() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let manager = e.call(0x0058_d680, &args![]).u32();
        assert_ne!(manager, 0);
        assert_eq!(e.global::<u32>(INSTANCE), manager);
        assert_eq!(e.call(0x0058_d680, &args![]).u32(), manager);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0xa0]]);
        // A failed allocation leaves it null.
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0058_d680, &args![]).u32(), 0);
        assert!(calls(&e, TES_FORM_CTOR).is_empty());
    }

    #[test]
    fn fn_0058d760_constructs_the_manager() {
        let mut e = engine();
        let this = e.mem.alloc(0xa0);
        assert_eq!(e.call(0x0058_d760, &args![this]).u32(), this);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .cloned()
            .collect();
        assert_eq!(
            order,
            vec![
                (TES_FORM_CTOR, vec![this]),
                (SET_FORM_TYPE, vec![this, 0x64]),
                (SET_FORM_ID, vec![this, 0x31, 1]),
                (RETURN_TRUE_00401290, vec![this, EDITOR_ID]),
                (MEMSET, vec![this + 0x18, 0, 0x88]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
    }

    #[test]
    fn fn_0058d710_deletes_the_singleton() {
        let mut e = engine();
        e.put_vtable(0x0f03_0000, &[0, 0, 0, 0, 0x0f03_0010]);
        e.register(0x0f03_0010, |_, _| Ret::default());
        let manager = e.mem.alloc(0xa0);
        e.mem.set_u32(manager, 0x0f03_0000);
        e.set_global(INSTANCE, manager);
        e.call(0x0058_d710, &args![]);
        assert_eq!(calls(&e, 0x0f03_0010), vec![vec![manager, 1]]);
        assert_eq!(e.global::<u32>(INSTANCE), 0);
        e.call(0x0058_d710, &args![]);
        assert_eq!(calls(&e, 0x0f03_0010).len(), 1);
    }

    #[test]
    fn fn_0058d820_stores_the_vtable_then_destroys() {
        let mut e = engine();
        let this = e.mem.alloc(0xa0);
        e.call(0x0058_d820, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(calls(&e, TES_FORM_DTOR), vec![vec![this]]);
    }

    type Script = Rc<RefCell<(Vec<u32>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<u32>, swap: bool) -> (Engine, u32) {
        let mut e = engine();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x164u32.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).copied().unwrap_or(0).into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        e.register(GET_CHUNK_DATA_SIZED, |_, _| Ret::default());
        e.register(SWAP_WORD, |_, _| Ret::default());
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        let this = e.mem.alloc(0xa0);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_0058d920_reads_the_ids() {
        // Only the low byte of the form type counts.
        let (mut e, this) = load_rig(vec![0x1234, CHUNK_DATA], true);
        assert!(e.call(0x0058_d920, &args![this, FILE]).bool());
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![this, FILE]]);
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![FILE, this + 0x18, 0x88]]
        );
        let swapped = calls(&e, SWAP_WORD);
        assert_eq!(swapped.len(), 34);
        assert_eq!(swapped[33], vec![this + 0x18 + 33 * 4, 0]);
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 0]; 2]);
        let (mut e, this) = load_rig(vec![], false);
        e.register(GET_TES_FORM, |_, _| 0x63u32.into_ret());
        assert!(!e.call(0x0058_d920, &args![this, FILE]).bool());
        assert!(calls(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_0058da10_resolves_and_checks_the_types() {
        let mut e = engine();
        e.map(0x0118_c000, 0x1000);
        for index in 0..34u32 {
            e.mem.set_u32(OBJECT_TABLE + index * 8, 0x0a00_0000 + index);
            e.mem
                .set_u8(OBJECT_TABLE + index * 8 + 4, 0x10 + index as u8);
        }
        e.register(MANAGER_IS_LOADED, |_, _| false.into_ret());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        e.register(GET_FILE, |_, _| 0x0777u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Ids ending in 0xf have no form; a form's type byte is at +4.
        e.register(LOOKUP_FORM, |e, a| {
            if a[0] & 0xff == 0xf {
                0u32.into_ret()
            } else {
                let form = e.mem.alloc(8);
                e.mem.set_u8(form + 4, (a[0] >> 8) as u8);
                form.into_ret()
            }
        });
        e.register(FORM_TYPE_00401170, |e, a| {
            u32::from(e.mem.u8(a[0] + 4)).into_ret()
        });
        e.register(LOG, |_, _| Ret::default());
        let this = e.mem.alloc(0xa0);
        // Object 0: type 0x10 (right); object 1: type 0x99 (wrong); object 2: no form.
        e.mem.set_u32(this + 0x18, 0x1001);
        e.mem.set_u32(this + 0x1c, 0x9902);
        e.mem.set_u32(this + 0x20, 0x120f);
        e.call(0x0058_da10, &args![this]);
        assert_ne!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(calls(&e, GET_FILE).len(), 3);
        assert_eq!(
            calls(&e, LOG),
            vec![
                vec![MESSAGE_NO_DEFAULT_OBJECT, 0x0a00_0001, 0x0100_9902],
                vec![MESSAGE_NO_DEFAULT_OBJECT, 0x0a00_0002, 0x0100_120f]
            ]
        );
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        e.register(MANAGER_IS_LOADED, |_, _| true.into_ret());
        e.call(0x0058_da10, &args![this]);
        assert_eq!(calls(&e, SET_FORM_FLAG_8).len(), 1);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    #[test]
    fn save_writes_the_ids_of_every_default_object() {
        let mut e = Engine::new();
        e.register(FORM_START_FORM, |_, _| Ret::default());
        e.register(FORM_CLOSE_FORM, |_, _| Ret::default());
        e.register(FORM_ID, |_, a| (a[0] | 0x0100_0000).into_ret());
        e.mem.map(0x0003_4000, OBJECT_COUNT * 4);
        e.register_double(ADD_CHUNK_ARRAY, |e, a| {
            for i in 0..a[2] {
                let id = e.mem.u32(a[1] + i * 4);
                e.mem.set_u32(0x0003_4000 + i * 4, id);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x18 + OBJECT_COUNT * 4);
        e.mem.set_u32(this + 0x18, 0x500);
        e.mem.set_u32(this + 0x18 + 0x21 * 4, 0x600);
        for i in 0..OBJECT_COUNT {
            e.mem.set_u32(0x0003_4000 + i * 4, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        e.call(0x0058_d870, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FORM_START_FORM, vec![this]));
        assert_eq!(log.iter().filter(|c| c.0 == FORM_ID).count(), 2);
        let array = log.iter().find(|c| c.0 == ADD_CHUNK_ARRAY).unwrap();
        assert_eq!((array.1[0], array.1[2]), (CHUNK_DATA, OBJECT_COUNT));
        assert_eq!(e.mem.u32(0x0003_4000), 0x0100_0500);
        assert_eq!(e.mem.u32(0x0003_4004), 0);
        assert_eq!(e.mem.u32(0x0003_4000 + 0x21 * 4), 0x0100_0600);
        assert_eq!(log.last().unwrap(), &(FORM_CLOSE_FORM, vec![this]));
    }
}
