//! `fallout shared/bgsrepairitemlist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (Xbox PDB name of `00484e60`; `ECX` = form, the index).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB name of `00485d50`; `cdecl`: address
/// of a form id, file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, a form id): the loaded form with that id, or 0.
const FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vfptr offset, source type,
/// target type, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `00474cb0` (`ECX` = form): the length of the name from the form's
/// virtual slot `0x130`, or 0.
const FORM_NAME_LENGTH: u32 = 0x0047_4cb0;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (`cdecl` format, ...): the log print.
const PRINT: u32 = 0x005b_5e40;
/// The form's name virtual slot, as `00474cb0` reads it.
const NAME_SLOT: u32 = 0x130;

/// The `TESForm` type descriptor (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The `BGSListForm` type descriptor (`.?AVBGSListForm@@`).
const TYPE_BGS_LIST_FORM: u32 = 0x0118_63d4;

/// `"MASTERFILE: Unable to find repair item list (%08X) for unknown owner."`
const MESSAGE_UNKNOWN_OWNER: u32 = 0x0101_adf8;
/// `"MASTERFILE: Unable to find repair item list (%08X) for owner object (%08X)."`
const MESSAGE_OWNER_ID: u32 = 0x0101_ae40;
/// `"MASTERFILE: Unable to find repair item list (%08X) for owner object \"%s\"."`
const MESSAGE_OWNER_NAME: u32 = 0x0101_ae90;

layout! {
    /// `BGSRepairItemList` (Xbox PDB), 8 bytes: a `BaseFormComponent` and the list.
    pub struct BGSRepairItemList: 0x8 {
        /// `pListForm` (Xbox PDB): the `BGSListForm`; before `InitItem` it
        /// holds the form id read from the file.
        0x04 pListForm: u32,
    }
}

// Translated from 0047b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRepairItemList::InitItem` (Xbox PDB; `RET 4`, `owner` the form being
/// loaded, possibly null): turns the form id in `pListForm` into the form
/// list (the id adjusted by the owner's file, 0 when there is no owner, with
/// `TESForm::AddCompileIndex`, looked up and cast to `BGSListForm`). When
/// nothing is left, a `MASTERFILE` message names the owner: unknown (null
/// owner), by name (when `00474cb0` finds one) or by form id.
pub fn bgs_repair_item_list_init_item(e: &mut Engine, this: Ptr<BGSRepairItemList>, owner: Ptr) {
    let loaded = e.get(this, BGSRepairItemList::pListForm);
    if loaded == 0 {
        return;
    }
    let file = if owner.is_null() {
        0
    } else {
        e.call(TES_FORM_GET_FILE, &args![owner, 0xffff_ffffu32])
            .u32()
    };
    let adjusted = e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), loaded);
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id, file]);
        e.mem.u32(id.addr())
    });
    let form = e.call(FORM_BY_ID, &args![adjusted]).u32();
    let list = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_BGS_LIST_FORM, 0u32],
        )
        .u32();
    e.set(this, BGSRepairItemList::pListForm, list);
    if e.get(this, BGSRepairItemList::pListForm) == 0 {
        if owner.is_null() {
            e.call(PRINT, &args![MESSAGE_UNKNOWN_OWNER, adjusted]);
        } else if e.call(FORM_NAME_LENGTH, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), NAME_SLOT, &[]).u32();
            e.call(PRINT, &args![MESSAGE_OWNER_NAME, adjusted, name]);
        } else {
            let owner_id = e.call(FORM_ID, &args![owner]).u32();
            e.call(PRINT, &args![MESSAGE_OWNER_ID, adjusted, owner_id]);
        }
    }
}

/// `00485910` (cdecl: signature, word): writes a one-word chunk.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The `REPL` chunk signature, as the word the exe pushes.
const SIG_REPL: u32 = 0x4c50_4552;
/// The list getter (`00726070`, the word at +4).
const GET_LIST: u32 = 0x0072_6070;
/// The list setter (`006ecd40`, the word at +4; the engine map's name
/// `CombatProcedureAttackMelee::Initialize` is a folded body).
const SET_LIST: u32 = 0x006e_cd40;
/// The `BaseFormComponent` type descriptor (`.?AVBaseFormComponent@@`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// The `BGSRepairItemList` type descriptor (`.?AVBGSRepairItemList@@`).
const TYPE_REPAIR_ITEM_LIST: u32 = 0x0118_6a64;

/// `other` cast from `BaseFormComponent` to `BGSRepairItemList`.
fn cast_component(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_REPAIR_ITEM_LIST,
            0u32
        ],
    )
    .u32()
}

// Translated from 0047b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRepairItemList::Save` (Xbox PDB): with a list, writes its form id as a
/// one-word `REPL` chunk (`00485910`).
pub fn bgs_repair_item_list_save(e: &mut Engine, this: Ptr<BGSRepairItemList>) {
    let list = e.get(this, BGSRepairItemList::pListForm);
    if list != 0 {
        let id = e.call(FORM_ID, &args![list]).u32();
        e.call(ADD_CHUNK_WORD, &args![SIG_REPL, id]);
    }
}

// Translated from 0047b9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRepairItemList::CopyComponent` (Xbox PDB; `RET 4`): when `other`
/// casts to a `BGSRepairItemList`, its list is copied in (getter, setter).
pub fn bgs_repair_item_list_copy_component(
    e: &mut Engine,
    this: Ptr<BGSRepairItemList>,
    other: Ptr,
) {
    let source = cast_component(e, other);
    if source != 0 {
        let list = e.call(GET_LIST, &args![source]).u32();
        e.call(SET_LIST, &args![this, list]);
    }
}

// Translated from 0047ba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRepairItemList::CompareComponent` (Xbox PDB; `RET 4`): true
/// (different) when `other` is not a `BGSRepairItemList` or holds another
/// list, false when the lists match (the other's getter is read first).
pub fn bgs_repair_item_list_compare_component(
    e: &mut Engine,
    this: Ptr<BGSRepairItemList>,
    other: Ptr,
) -> bool {
    let source = cast_component(e, other);
    if source == 0 {
        return true;
    }
    let theirs = e.call(GET_LIST, &args![source]).u32();
    theirs != e.call(GET_LIST, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0047b8e0,
            bgs_repair_item_list_init_item(Ptr<BGSRepairItemList>, Ptr)
        ),
        entry!(
            0x0047_b8a0,
            bgs_repair_item_list_save(Ptr<BGSRepairItemList>)
        ),
        entry!(
            0x0047_b9d0,
            bgs_repair_item_list_copy_component(Ptr<BGSRepairItemList>, Ptr)
        ),
        entry!(
            0x0047_ba20,
            bgs_repair_item_list_compare_component(Ptr<BGSRepairItemList>, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    /// An engine where the cast answers `cast` and the owner has a name of
    /// `name_length` characters.
    fn engine(cast: u32, name_length: u32) -> Engine {
        let mut e = Engine::new();
        e.register(TES_FORM_GET_FILE, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(FORM_BY_ID, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register_double(RT_DYNAMIC_CAST, move |_, _| Ret {
            eax: cast,
            ..Ret::default()
        });
        e.register_double(FORM_NAME_LENGTH, move |_, _| Ret {
            eax: name_length,
            ..Ret::default()
        });
        e.register(FORM_ID, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(PRINT, |_, _| Ret::default());
        e.register(0x0050_0000, |_, _| Ret {
            eax: 0x0400_0000,
            ..Ret::default()
        });
        e
    }

    fn component(e: &mut Engine, id: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, id);
        object
    }

    #[test]
    fn nothing_is_done_without_a_loaded_id() {
        let mut e = engine(0x9000, 0);
        let object = component(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0047b8e0, &args![object, 0x8000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn the_list_is_looked_up_with_the_owners_file() {
        let mut e = engine(0x9000, 0);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0047b8e0, &args![object, owner]);
        assert_eq!(e.mem.u32(object + 4), 0x9000);
        assert_eq!(
            calls_to(&e, TES_FORM_GET_FILE),
            vec![vec![owner, 0xffff_ffff]]
        );
        assert_eq!(calls_to(&e, TES_FORM_ADD_COMPILE_INDEX)[0][1], 0x77);
        assert_eq!(calls_to(&e, FORM_BY_ID), vec![vec![0x0100_0042]]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x0100_0052, 0, TYPE_TES_FORM, TYPE_BGS_LIST_FORM, 0]]
        );
        assert!(calls_to(&e, PRINT).is_empty());
    }

    #[test]
    fn a_missing_list_is_reported_for_an_unknown_owner_with_file_zero() {
        let mut e = engine(0, 0);
        let object = component(&mut e, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x0047b8e0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert!(calls_to(&e, TES_FORM_GET_FILE).is_empty());
        assert_eq!(calls_to(&e, TES_FORM_ADD_COMPILE_INDEX)[0][1], 0);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_UNKNOWN_OWNER, 0x0100_0042]]
        );
    }

    #[test]
    fn a_missing_list_is_reported_by_the_owner_id_without_a_name() {
        let mut e = engine(0, 0);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0047b8e0, &args![object, owner]);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_OWNER_ID, 0x0100_0042, 0x1234]]
        );
    }

    #[test]
    fn a_missing_list_is_reported_by_the_owner_name_when_it_has_one() {
        let mut e = engine(0, 5);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.put_vtable(0x0700_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0700_0130, 0x0050_0000);
        e.mem.set_u32(owner, 0x0700_0000);
        e.call_log = Some(vec![]);
        e.call(0x0047b8e0, &args![object, owner]);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_OWNER_NAME, 0x0100_0042, 0x0400_0000]]
        );
    }
}

#[cfg(test)]
mod n037_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The getter and setter as the exe's (+4), form ids the form plus 1, and
    /// a cast that keeps even addresses.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(GET_LIST, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SET_LIST, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(FORM_ID, |_, a| ret(a[0] + 1));
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] & 1 == 0 { a[0] } else { 0 })
        });
        e.call_log = Some(vec![]);
        e
    }

    fn component(e: &mut Engine, list: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, list);
        object
    }

    #[test]
    fn save_writes_the_list_when_there_is_one() {
        let mut e = engine();
        let with = component(&mut e, 0x500);
        let without = component(&mut e, 0);
        e.call(0x0047_b8a0, &[with]);
        e.call(0x0047_b8a0, &[without]);
        assert_eq!(calls_to(&e, ADD_CHUNK_WORD), vec![vec![SIG_REPL, 0x501]]);
    }

    #[test]
    fn copy_component_takes_the_list_of_a_cast_component() {
        let mut e = engine();
        let this = component(&mut e, 1);
        let other = component(&mut e, 0x500);
        e.call(0x0047_b9d0, &[this, other + 1]);
        assert_eq!(e.mem.u32(this + 4), 1);
        e.call(0x0047_b9d0, &[this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x500);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST)[1],
            vec![other, 0, TYPE_BASE_FORM_COMPONENT, TYPE_REPAIR_ITEM_LIST, 0]
        );
    }

    #[test]
    fn compare_component_says_whether_they_differ() {
        let mut e = engine();
        let this = component(&mut e, 0x500);
        let same = component(&mut e, 0x500);
        let other = component(&mut e, 0x600);
        assert!(!e.call(0x0047_ba20, &[this, same]).bool());
        assert!(e.call(0x0047_ba20, &[this, other]).bool());
        assert!(e.call(0x0047_ba20, &[this, same + 1]).bool());
    }
}
