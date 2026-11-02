//! `fallout shared/bgsbipedmodellist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
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
/// `00500940` (`ECX` = a `BGSListForm`): the address of its list head, the
/// form plus `0x18`.
const LIST_HEAD: u32 = 0x0050_0940;
/// `008256d0` (`ECX` = list node): true when the node has neither an item nor
/// a next node (the words at +0 and +4 are both 0).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`ECX` = list node): the node's own address, the place its item
/// is read from.
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// `00726070` (`ECX` = list node): the word at +4, the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `TESBipedModelForm::GetFormAsBipedModel` (Xbox PDB name of `00480db0`;
/// `cdecl`, a form): the form's biped model component, or 0.
const GET_FORM_AS_BIPED_MODEL: u32 = 0x0048_0db0;
/// `00480d50` (`ECX` = biped model component, a byte): sets bit 0x10 of the
/// byte at +8 when the argument is non-zero, clears it otherwise.
const SET_FLAG_10: u32 = 0x0048_0d50;

/// The `TESForm` type descriptor (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// The `BGSListForm` type descriptor (`.?AVBGSListForm@@`).
const TYPE_BGS_LIST_FORM: u32 = 0x0118_63d4;

/// `"MASTERFILE: Unable to find biped model list (%08X) for unknown owner."`
const MESSAGE_UNKNOWN_OWNER: u32 = 0x0101_a558;
/// `"MASTERFILE: Unable to find biped model list (%08X) for owner object (%08X)."`
const MESSAGE_OWNER_ID: u32 = 0x0101_a5a0;
/// `"MASTERFILE: Unable to find biped model list (%08X) for owner object \"%s\"."`
const MESSAGE_OWNER_NAME: u32 = 0x0101_a5f0;
/// `"MASTERFILE: Form '%s' (%08X) in form list '%s' (%08X) is not a biped
/// model. BipedModelList on '%s' (%08X) will be cleared."`
const MESSAGE_NOT_A_BIPED_MODEL: u32 = 0x0101_a4d8;

layout! {
    /// `BGSBipedModelList` (Xbox PDB), 8 bytes: a `BaseFormComponent` and the list.
    pub struct BGSBipedModelList: 0x8 {
        /// `pListForm` (Xbox PDB): the `BGSListForm`; before `InitItem` it
        /// holds the form id read from the file.
        0x04 pListForm: u32,
    }
}

// Translated from 00474da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBipedModelList::InitItem` (Xbox PDB; `RET 4`, `owner` the form being
/// loaded, possibly null): turns the form id in `pListForm` into the form
/// list (the id adjusted by the owner's file, 0 without an owner, with
/// `TESForm::AddCompileIndex`, looked up and cast to `BGSListForm`). When
/// nothing is left, a `MASTERFILE` message names the owner: unknown (null
/// owner), by name (when `00474cb0` finds one) or by form id. When the list
/// was found, every item on it is turned into its biped model component and
/// that component's `0x10` flag is set; the first item that has no biped
/// model prints a message (owner, list and item, by id and name) and clears
/// the component's list, ending the walk. The walk also ends at the first
/// empty node. The owner is used without a null check on that path.
pub fn bgs_biped_model_list_init_item(e: &mut Engine, this: Ptr<BGSBipedModelList>, owner: Ptr) {
    let loaded = e.get(this, BGSBipedModelList::pListForm);
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
    e.set(this, BGSBipedModelList::pListForm, list);
    if list == 0 {
        if owner.is_null() {
            e.call(PRINT, &args![MESSAGE_UNKNOWN_OWNER, adjusted]);
        } else if e.call(FORM_NAME_LENGTH, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), NAME_SLOT, &[]).u32();
            e.call(PRINT, &args![MESSAGE_OWNER_NAME, adjusted, name]);
        } else {
            let owner_id = e.call(FORM_ID, &args![owner]).u32();
            e.call(PRINT, &args![MESSAGE_OWNER_ID, adjusted, owner_id]);
        }
        return;
    }
    let mut node = e.call(LIST_HEAD, &args![list]).u32();
    while node != 0 {
        if e.call(NODE_IS_EMPTY, &args![node]).bool() {
            return;
        }
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        let biped_model = e.call(GET_FORM_AS_BIPED_MODEL, &args![item]).u32();
        if biped_model == 0 {
            let owner_id = e.call(FORM_ID, &args![owner]).u32();
            let owner_name = e.vcall(owner.addr(), NAME_SLOT, &[]).u32();
            let list_id = e.call(FORM_ID, &args![list]).u32();
            let list_name = e.vcall(list, NAME_SLOT, &[]).u32();
            let item_id = e.call(FORM_ID, &args![item]).u32();
            let item_name = e.vcall(item, NAME_SLOT, &[]).u32();
            e.call(
                PRINT,
                &args![
                    MESSAGE_NOT_A_BIPED_MODEL,
                    item_name,
                    item_id,
                    list_name,
                    list_id,
                    owner_name,
                    owner_id
                ],
            );
            e.set(this, BGSBipedModelList::pListForm, 0);
            return;
        }
        e.call(SET_FLAG_10, &args![biped_model, 1u32]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// `00401170` (unnamed in the engine map): `thiscall` getter, the form type byte at `this + 4`.
const FORM_TYPE_GETTER_00401170: u32 = 0x0040_1170;

// Translated from 00475020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBipedModelList::GetFormAsBipedModelList` (Xbox PDB): `cdecl`, one stack word. For a
/// non-null form, answers its biped model list part by the form's type byte (`00401170`): type
/// 0x18 at `form + 0x164`, type 0x28 at `form + 0xe0`; null for any other type or a null form.
/// (The exe tests the form for null again inside each case; those tests cannot fail and are
/// kept.)
pub fn bgs_biped_model_list_get_form_as_biped_model_list(e: &mut Engine, form: Ptr) -> Ptr {
    let mut result = 0;
    if !form.is_null() {
        let part_at = |offset: u32| {
            if form.is_null() {
                0
            } else {
                form.addr().wrapping_add(offset)
            }
        };
        match e.call(FORM_TYPE_GETTER_00401170, &args![form]).u32() {
            0x18 => result = part_at(0x164),
            0x28 => result = part_at(0xe0),
            _ => {}
        }
    }
    Ptr::new(result)
}

/// `00485910` (`cdecl`: chunk signature, word): saves a chunk holding one word.
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The chunk signature `BIPL` (`0x4c504942`).
const CHUNK_BIPL: u32 = 0x4c50_4942;
/// The `BaseFormComponent` and `BGSBipedModelList` type descriptors (`01183040`,
/// `011868b0`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_BGS_BIPED_MODEL_LIST: u32 = 0x0118_68b0;
/// `006ecd40` (`ECX` = object, `RET 4`): stores its argument at +4 (the engine map names it
/// `CombatProcedureAttackMelee::Initialize`, a folded body).
const SET_WORD_4: u32 = 0x006e_cd40;

/// `__RTDynamicCast` of `other` from `BaseFormComponent` to `BGSBipedModelList`.
fn cast_to_biped_model_list(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_BGS_BIPED_MODEL_LIST,
            0u32
        ],
    )
    .u32()
}

// Translated from 00474d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBipedModelList::Save` (Xbox PDB; `thiscall`, no stack word): when a list is set,
/// saves its form id (`0084e3a0`) in a `BIPL` chunk (`00485910`).
pub fn bgs_biped_model_list_save(e: &mut Engine, this: Ptr<BGSBipedModelList>) {
    let list = e.get(this, BGSBipedModelList::pListForm);
    if list == 0 {
        return;
    }
    let id = e.call(FORM_ID, &args![list]).u32();
    e.call(ADD_CHUNK_WORD, &args![CHUNK_BIPL, id]);
}

// Translated from 00474f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBipedModelList::CopyComponent` (Xbox PDB; `RET 4`): casts `other` to a
/// `BGSBipedModelList` and, when it is one, copies its list: the word at +4 read through
/// `00726070` and stored through `006ecd40`.
pub fn bgs_biped_model_list_copy_component(
    e: &mut Engine,
    this: Ptr<BGSBipedModelList>,
    other: Ptr,
) {
    let cast = cast_to_biped_model_list(e, other);
    if cast == 0 {
        return;
    }
    let list = e.call(NODE_NEXT, &args![cast]).u32();
    e.call(SET_WORD_4, &args![this, list]);
}

// Translated from 00474fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSBipedModelList::CompareComponent` (Xbox PDB; `RET 4`, result in `AL`): true
/// ("differs") when `other` is not a `BGSBipedModelList` or its list (the word at +4,
/// through `00726070`) is another than this one's.
pub fn bgs_biped_model_list_compare_component(
    e: &mut Engine,
    this: Ptr<BGSBipedModelList>,
    other: Ptr,
) -> bool {
    let cast = cast_to_biped_model_list(e, other);
    if cast == 0 {
        return true;
    }
    let theirs = e.call(NODE_NEXT, &args![cast]).u32();
    let ours = e.call(NODE_NEXT, &args![this]).u32();
    theirs != ours
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_5020, bgs_biped_model_list_get_form_as_biped_model_list(Ptr) -> Ptr),
        entry!(
            0x00474da0,
            bgs_biped_model_list_init_item(Ptr<BGSBipedModelList>, Ptr)
        ),
        entry!(
            0x0047_4d60,
            bgs_biped_model_list_save(Ptr<BGSBipedModelList>)
        ),
        entry!(
            0x0047_4f70,
            bgs_biped_model_list_copy_component(Ptr<BGSBipedModelList>, Ptr)
        ),
        entry!(
            0x0047_4fc0,
            bgs_biped_model_list_compare_component(Ptr<BGSBipedModelList>, Ptr) -> bool
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

    /// A list form at 0x9000 whose list head (the form plus 0x18) is a node
    /// with item 0xb000 and next node 0xa010; that node has item 0xb100 and
    /// next node 0xa020, which is empty. A node holds its item at +0 and the
    /// next node at +4.
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
        e.register(FORM_ID, |_, a| Ret {
            eax: 0x1000 + a[0],
            ..Ret::default()
        });
        e.register(PRINT, |_, _| Ret::default());
        e.register(LIST_HEAD, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        e.register(NODE_IS_EMPTY, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0),
            ..Ret::default()
        });
        e.register(NODE_ITEM_SLOT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(GET_FORM_AS_BIPED_MODEL, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x20),
            ..Ret::default()
        });
        e.register(SET_FLAG_10, |_, _| Ret::default());
        e.register(0x0050_0000, |_, _| Ret {
            eax: 0x0400_0000,
            ..Ret::default()
        });
        e
    }

    /// Maps the list: `first` and `second` are the biped model components of
    /// the two items (0 for none).
    fn list(e: &mut Engine, first: u32, second: u32) {
        e.mem.map(0x9000, 0x100);
        e.mem.map(0xa000, 0x100);
        e.mem.map(0xb000, 0x200);
        e.mem.set_u32(0xa010, 0xb100);
        e.mem.set_u32(0xa014, 0xa020);
        e.mem.set_u32(0xb020, first);
        e.mem.set_u32(0xb120, second);
        e.put_vtable(0x0700_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0700_0130, 0x0050_0000);
        e.mem.set_u32(0xb000, 0x0700_0000);
        // The first node is the list head, the form plus 0x18.
        e.mem.set_u32(0x9018, 0xb000);
        e.mem.set_u32(0x901c, 0xa010);
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
        e.call(0x00474da0, &args![object, 0x8000u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn a_missing_list_is_reported_by_the_owner() {
        let mut e = engine(0, 0);
        let object = component(&mut e, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x00474da0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert!(calls_to(&e, TES_FORM_GET_FILE).is_empty());
        assert_eq!(calls_to(&e, TES_FORM_ADD_COMPILE_INDEX)[0][1], 0);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_UNKNOWN_OWNER, 0x0100_0042]]
        );
        assert!(calls_to(&e, LIST_HEAD).is_empty());

        let mut e = engine(0, 0);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00474da0, &args![object, owner]);
        assert_eq!(
            calls_to(&e, TES_FORM_GET_FILE),
            vec![vec![owner, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_OWNER_ID, 0x0100_0042, 0x1000 + owner]]
        );

        let mut e = engine(0, 5);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner, 0x0700_0000);
        e.put_vtable(0x0700_0000, &[0; 0x4d]);
        e.mem.set_u32(0x0700_0130, 0x0050_0000);
        e.call_log = Some(vec![]);
        e.call(0x00474da0, &args![object, owner]);
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![MESSAGE_OWNER_NAME, 0x0100_0042, 0x0400_0000]]
        );
    }

    #[test]
    fn every_item_with_a_biped_model_is_flagged() {
        let mut e = engine(0x9000, 0);
        list(&mut e, 0xc000, 0xc100);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00474da0, &args![object, owner]);
        assert_eq!(e.mem.u32(object + 4), 0x9000);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            vec![vec![0x0100_0052, 0, TYPE_TES_FORM, TYPE_BGS_LIST_FORM, 0]]
        );
        assert_eq!(calls_to(&e, LIST_HEAD), vec![vec![0x9000]]);
        assert_eq!(
            calls_to(&e, SET_FLAG_10),
            vec![vec![0xc000, 1], vec![0xc100, 1]]
        );
        assert!(calls_to(&e, PRINT).is_empty());
    }

    #[test]
    fn an_item_without_a_biped_model_clears_the_list_and_ends_the_walk() {
        let mut e = engine(0x9000, 0);
        list(&mut e, 0, 0xc100);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner, 0x0700_0000);
        e.mem.set_u32(0x9000, 0x0700_0000);
        e.call_log = Some(vec![]);
        e.call(0x00474da0, &args![object, owner]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert!(calls_to(&e, SET_FLAG_10).is_empty());
        assert_eq!(
            calls_to(&e, PRINT),
            vec![vec![
                MESSAGE_NOT_A_BIPED_MODEL,
                0x0400_0000,
                0x1000 + 0xb000,
                0x0400_0000,
                0x1000 + 0x9000,
                0x0400_0000,
                0x1000 + owner,
            ]]
        );
    }

    #[test]
    fn an_empty_node_ends_the_walk() {
        let mut e = engine(0x9000, 0);
        list(&mut e, 0xc000, 0xc100);
        // Make the first node (the list head) the empty one.
        e.mem.set_u32(0x9018, 0);
        e.mem.set_u32(0x901c, 0);
        let object = component(&mut e, 0x42);
        let owner = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00474da0, &args![object, owner]);
        assert_eq!(e.mem.u32(object + 4), 0x9000);
        assert!(calls_to(&e, GET_FORM_AS_BIPED_MODEL).is_empty());
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00475020_answers_the_part_each_type_selects() {
        let mut e = Engine::new();
        e.register(FORM_TYPE_GETTER_00401170, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        for (kind, offset) in [(0x18u32, 0x164u32), (0x28, 0xe0)] {
            let form = e.mem.alloc(0x180);
            e.mem.set_u32(form + 4, kind);
            assert_eq!(
                e.call(0x0047_5020, &args![form]).u32(),
                form + offset,
                "{kind:#x}"
            );
        }
        for kind in [0u32, 0x17, 0x19, 0x27, 0x29, 0xff] {
            let form = e.mem.alloc(0x180);
            e.mem.set_u32(form + 4, kind);
            assert_eq!(e.call(0x0047_5020, &args![form]).u32(), 0, "{kind:#x}");
        }
        assert_eq!(e.call(0x0047_5020, &args![0u32]).u32(), 0);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_writes_the_list_form_id_only_when_set() {
        let mut e = Engine::new();
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register(ADD_CHUNK_WORD, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0047_4d60, &args![this]);
        assert!(calls(&e, ADD_CHUNK_WORD).is_empty());
        e.mem.set_u32(this + 4, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0047_4d60, &args![this]);
        assert_eq!(calls(&e, FORM_ID), vec![vec![0x5000]]);
        assert_eq!(calls(&e, ADD_CHUNK_WORD), vec![vec![CHUNK_BIPL, 0x5001]]);
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SET_WORD_4, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn copy_component_copies_the_list_of_a_biped_model_list() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, 0x6000);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_4f70, &args![this, other]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BGS_BIPED_MODEL_LIST,
                0
            ]]
        );
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_4f70, &args![this, other]);
        assert_eq!(calls(&e, SET_WORD_4), vec![vec![this, 0x6000]]);
        assert_eq!(e.mem.u32(this + 4), 0x6000);
    }

    #[test]
    fn compare_component_differs_unless_the_same_list() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.mem.set_u32(other + 4, 0x6000);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        assert!(!e.call(0x0047_4fc0, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 0x6100);
        assert!(e.call(0x0047_4fc0, &args![this, other]).bool());
        e.mem.set_u32(other + 4, 0x6000);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0047_4fc0, &args![this, other]).bool());
    }
}
