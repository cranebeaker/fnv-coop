//! `fallout shared/bgslistform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BGSListForm` keeps its forms in the `BSSimpleList` at `+0x18`. Every
//! walk over it goes through the same small functions by address: the list
//! getter `00500940` (`this + 0x18`), the node's empty test `008256d0`, the
//! node's item address `006815c0` and the next node `00726070`.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSListForm` (Xbox PDB, 0x34 bytes there): 0x24 bytes on PC
    /// (`CreateFormOfType` allocates that much), `TESForm` being 0x18.
    pub struct BGSListForm: 0x24 {
        /// `ListOfForms` (Xbox PDB): `BSSimpleList<TESForm *>`.
        0x18 ListOfForms: Inline<crate::types::BSSimpleList>,
        /// `iScriptAddedFormCount` (Xbox PDB).
        0x20 iScriptAddedFormCount: u32,
    }
}

/// `BGSListForm`'s vtable (RTTI `.?AVBGSListForm@@`).
const LIST_FORM_VTABLE: u32 = 0x0103_34b4;
/// The list at `this + 0x18` (`00500940`, plain `RET`).
const LIST_OF_FORMS: u32 = 0x0050_0940;
/// A `BSSimpleList` node's empty test (`008256d0`, `bool`).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// The address of a node's item (`006815c0`).
const NODE_ITEM: u32 = 0x0068_15c0;
/// A node's next node (`00726070`).
const NODE_NEXT: u32 = 0x0072_6070;
/// Appends the item a pointer points at to a list (`00905820`, `RET 4`).
const LIST_ADD_TAIL: u32 = 0x0090_5820;
/// Empties a list (`00470470`, plain `RET`).
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// Counts a list's non-null items (`005ae380`, named `VATS::GetCount` by
/// the engine map through identical-code folding).
const LIST_COUNT: u32 = 0x005a_e380;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `.?AVTESForm@@` and `.?AVBGSListForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BGS_LIST_FORM: u32 = 0x0118_63d4;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// Tests form flag 0x8 (`004013e0`: `iFormFlags & 8`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// Sets (`true`) or clears form flag 0x8 (`00484ab0`, `RET 4`).
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`).
const LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The size of the current chunk (`00401660`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): buffer and size.
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`): one word.
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// Moves to the next chunk (`004726f0`, `bool`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The `MemoryManager` free wrapper (`00401030`, cdecl).
const FREE: u32 = 0x0040_1030;
/// "MASTERFILE: Unable to find form (%08X) on list form %s (%08X)."
const MISSING_FORM_MESSAGE: u32 = 0x0103_35ec;

/// Chunk signatures, as the words the exe compares.
const SIG_EDID: u32 = 0x4449_4445;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_LNAM: u32 = 0x4d41_4e4c;

/// The list of a list form (`00500940`), for this file's own forms and for
/// forms it got from a cast.
fn list_of_forms(e: &mut Engine, form: u32) -> u32 {
    e.call(LIST_OF_FORMS, &args![form]).u32()
}

fn node_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(NODE_IS_EMPTY, &args![node]).bool()
}

/// The item (a `TESForm *`) of a node, read through its address.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let at = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(at)
}

fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

/// `__RTDynamicCast(form, 0, TESForm, BGSListForm, 0)`.
fn cast_to_list_form(e: &mut Engine, form: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_BGS_LIST_FORM, 0u32],
    )
    .u32()
}

// Translated from 0058f9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058f9d0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x55 (0x24 bytes); `ECX` is the object.
/// The engine map names it `BGSListForm::BGSListForm` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_34b4`) is `BGSListForm`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058f9d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, LIST_FORM_VTABLE);
    e.call(0x0096_a2d0, &args![base + 0x18]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x55u32]);
    e.mem.set_u32(base + 0x20, 0x0);
    this
}

// Translated from 0058fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSListForm`'s scalar deleting destructor: runs the destructor
/// (`0058fa90`) and, when bit 0 of `flags` is set, frees the object
/// (`00401030`). Returns `this`.
pub fn fn_0058fa60(e: &mut Engine, this: Ptr<BGSListForm>, flags: u32) -> Ptr<BGSListForm> {
    fn_0058fa90(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0058fa90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058fa90 - the exception-unwinding frame (FS:[0]) and its security-cookie word are not translated.
/// `BGSListForm`'s destructor (Ghidra's library match
/// `CMFCFilterChunkValueImpl::~CMFCFilterChunkValueImpl` is wrong: the body
/// stores the `BGSListForm` vtable): stores the vtable, empties the list
/// and the script-added count (`0058fb00`), destroys the list (`0046ffb0`
/// on `this + 0x18`) and runs `TESForm::~TESForm` (Xbox PDB, `00483630`).
pub fn fn_0058fa90(e: &mut Engine, this: Ptr<BGSListForm>) {
    e.mem.set_u32(this.addr(), LIST_FORM_VTABLE);
    fn_0058fb00(e, this);
    e.call(0x0046_ffb0, &args![this.byte_add(0x18)]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0058fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties `ListOfForms` (`00470470` on the list `00500940` answers) and
/// sets `iScriptAddedFormCount` to 0.
pub fn fn_0058fb00(e: &mut Engine, this: Ptr<BGSListForm>) {
    let list = list_of_forms(e, this.addr());
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.set(this, BGSListForm::iScriptAddedFormCount, 0);
}

// Translated from 0058fb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `source` into this form. Nothing happens unless the source casts
/// to a `BGSListForm` (`__RTDynamicCast(source, 0, TESForm, BGSListForm,
/// 0)`); then every item of its list is appended to this list (`00905820`
/// with the address of a stack copy of the item, the list fetched again
/// for each item) and `TESForm::CopyAllComponents` (Xbox PDB, `004851b0`)
/// runs with the cast source. This list is not emptied first.
pub fn fn_0058fb30(e: &mut Engine, this: Ptr<BGSListForm>, source: Ptr) {
    let other = cast_to_list_form(e, source);
    if other == 0 {
        return;
    }
    let mut node = list_of_forms(e, other);
    e.with_stack(4, |e, item_at| {
        while node != 0 && !node_is_empty(e, node) {
            let item = node_item(e, node);
            e.mem.set_u32(item_at.addr(), item);
            let list = list_of_forms(e, this.addr());
            e.call(LIST_ADD_TAIL, &args![list, item_at]);
            node = node_next(e, node);
        }
    });
    e.call(0x0048_51b0, &args![this, other]);
}

// Translated from 0058fbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares this form with `other`: true when they differ. A form that is
/// not a `BGSListForm` (the cast gives null) differs; so do lists with
/// different counts of non-null items (`005ae380` on each list) and lists
/// whose items differ at the same position. Otherwise the answer is
/// `TESForm::CompareAllComponents` (Xbox PDB, `00485270`) with the cast
/// form.
pub fn fn_0058fbc0(e: &mut Engine, this: Ptr<BGSListForm>, other: Ptr) -> bool {
    let other = cast_to_list_form(e, other);
    if other == 0 {
        return true;
    }
    let list = list_of_forms(e, this.addr());
    let count = e.call(LIST_COUNT, &args![list]).u32();
    let other_list = list_of_forms(e, other);
    if count != e.call(LIST_COUNT, &args![other_list]).u32() {
        return true;
    }
    let mut node = list_of_forms(e, this.addr());
    let mut other_node = list_of_forms(e, other);
    while node != 0 && !node_is_empty(e, node) && other_node != 0 && !node_is_empty(e, other_node) {
        if node_item(e, node) != node_item(e, other_node) {
            return true;
        }
        node = node_next(e, node);
        other_node = node_next(e, other_node);
    }
    e.call(0x0048_5270, &args![this, other]).bool()
}

// Translated from 0058fcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the form: `TESForm::StartForm` (Xbox PDB, `004855a0`), one `LNAM`
/// chunk per item with the item's form id (`00485910`, cdecl, signature
/// and word), then `TESForm::CloseForm` (Xbox PDB, `00485680`).
pub fn fn_0058fcc0(e: &mut Engine, this: Ptr<BGSListForm>) {
    e.call(0x0048_55a0, &args![this]);
    let mut node = list_of_forms(e, this.addr());
    while node != 0 && !node_is_empty(e, node) {
        let item = node_item(e, node);
        let id = e.call(FORM_ID, &args![item]).u32();
        e.call(0x0048_5910, &args![SIG_LNAM, id]);
        node = node_next(e, node);
    }
    e.call(0x0048_5680, &args![this]);
}

// Translated from 0058fd40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058fd40 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// Loads the form from `file`: `TESForm::LoadForm` (Xbox PDB, `00485110`),
/// clears form flag 0x8 (`00484ab0`), then reads every chunk: `EDID` (into
/// a stack buffer of the chunk's size, read with at most 0x200 bytes,
/// given to vtable slot 0x134), `OBND` (vtable slot 0xE0 with the file)
/// and `LNAM` (a form id, appended to the list as it is; `InitItem`
/// resolves it). Returns true.
pub fn fn_0058fd40(e: &mut Engine, this: Ptr<BGSListForm>, file: Ptr) -> bool {
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, false]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(this.addr(), 0x134, &args![buffer]);
                });
            }
            SIG_OBND => {
                e.vcall(this.addr(), 0xe0, &args![file]);
            }
            SIG_LNAM => {
                e.with_stack(8, |e, locals| {
                    let id_at = locals;
                    let copy_at = locals.byte_add(4);
                    e.mem.set_u32(id_at.addr(), 0);
                    e.call(GET_CHUNK_DATA, &args![file, id_at]);
                    let id = e.mem.u32(id_at.addr());
                    e.mem.set_u32(copy_at.addr(), id);
                    let list = list_of_forms(e, this.addr());
                    e.call(LIST_ADD_TAIL, &args![list, copy_at]);
                });
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0058fe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSListForm::InitItem` (Xbox PDB): unless form flag 0x8 is set
/// (`004013e0`), resolves every item of the list, which `Load` left as a
/// form id: the id gets the compile index of the form's file
/// (`TESForm::AddCompileIndex`, `00485d50`, with `TESForm::GetFile(-1)`,
/// `00484e60`) and is looked up (`004839c0`). A found form replaces the
/// item (`00726c60`, with the address of the found form). A missing one is
/// reported ("MASTERFILE: Unable to find form (%08X) on list form %s
/// (%08X).", `005b5e40`, with the id, vtable slot 0x130's answer and the
/// form id) and its node removed: after an earlier node through `00905330`
/// on that earlier node (with the address of a stack copy of the id), the
/// walk going on from the earlier node's next; for the first node through
/// `0063f7b0` on the node itself, which is then walked again. Finally form
/// flag 0x8 is set (`00484ab0`).
pub fn bgs_list_form_init_item(e: &mut Engine, this: Ptr<BGSListForm>) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    let mut node = list_of_forms(e, this.addr());
    let mut previous = 0u32;
    e.with_stack(12, |e, locals| {
        let id_at = locals;
        let form_at = locals.byte_add(4);
        let copy_at = locals.byte_add(8);
        while node != 0 && !node_is_empty(e, node) {
            let item = node_item(e, node);
            e.mem.set_u32(id_at.addr(), item);
            let file = e.call(0x0048_4e60, &args![this, u32::MAX]).u32();
            e.call(0x0048_5d50, &args![id_at, file]);
            let id = e.mem.u32(id_at.addr());
            let form = e.call(0x0048_39c0, &args![id]).u32();
            e.mem.set_u32(form_at.addr(), form);
            if form != 0 {
                e.call(0x0072_6c60, &args![node, form_at]);
                previous = node;
                node = node_next(e, node);
            } else {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let name = e.vcall(this.addr(), 0x130, &args![]).u32();
                let id = e.mem.u32(id_at.addr());
                e.call(0x005b_5e40, &args![MISSING_FORM_MESSAGE, id, name, form_id]);
                if previous != 0 {
                    let id = e.mem.u32(id_at.addr());
                    e.mem.set_u32(copy_at.addr(), id);
                    e.call(0x0090_5330, &args![previous, copy_at]);
                    node = node_next(e, previous);
                } else {
                    e.call(0x0063_f7b0, &args![node]);
                }
            }
        }
    });
    e.call(SET_FORM_FLAG_8, &args![this, true]);
}

// Translated from 0058ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSListForm::GetItemIndex` (Xbox PDB): the position of `form` in the
/// list, or -1 when the walk reaches a null node. A walk that stops at an
/// empty node answers the number of items before it.
pub fn bgs_list_form_get_item_index(e: &mut Engine, this: Ptr<BGSListForm>, form: u32) -> i32 {
    let mut index = 0i32;
    let mut node = list_of_forms(e, this.addr());
    while node != 0 && !node_is_empty(e, node) {
        if node_item(e, node) == form {
            break;
        }
        index += 1;
        node = node_next(e, node);
    }
    if node == 0 {
        index = -1;
    }
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_f9d0, fn_0058f9d0(Ptr) -> Ptr),
        entry!(
            0x0058_fa60,
            fn_0058fa60(Ptr<BGSListForm>, u32) -> Ptr<BGSListForm>
        ),
        entry!(0x0058_fa90, fn_0058fa90(Ptr<BGSListForm>)),
        entry!(0x0058_fb00, fn_0058fb00(Ptr<BGSListForm>)),
        entry!(0x0058_fb30, fn_0058fb30(Ptr<BGSListForm>, Ptr)),
        entry!(0x0058_fbc0, fn_0058fbc0(Ptr<BGSListForm>, Ptr) -> bool),
        entry!(0x0058_fcc0, fn_0058fcc0(Ptr<BGSListForm>)),
        entry!(0x0058_fd40, fn_0058fd40(Ptr<BGSListForm>, Ptr) -> bool),
        entry!(0x0058_fe50, bgs_list_form_init_item(Ptr<BGSListForm>)),
        entry!(
            0x0058_ff60,
            bgs_list_form_get_item_index(Ptr<BGSListForm>, u32) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles for the list walk: a node is `[item, next]`, empty when its
    /// item is null.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_OF_FORMS, |_, a| ret(a[0] + 0x18));
        e.register(NODE_IS_EMPTY, |e, a| ret((e.mem.u32(a[0]) == 0) as u32));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
    }

    /// A list form whose list holds `items` (the first node inline).
    fn list_form(e: &mut Engine, items: &[u32]) -> u32 {
        let form = e.mem.alloc(0x24);
        let mut node = form + 0x18;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        form
    }

    fn items(e: &Engine, form: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = form + 0x18;
        while node != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    fn recorder(e: &mut Engine, addr: u32, answer: u32) -> Log {
        let log: Log = Rc::default();
        let l = log.clone();
        e.register_double(addr, move |_, a| {
            l.borrow_mut().push(a.to_vec());
            ret(answer)
        });
        log
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_0058f9d0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_f9d0,
            size: 0x24,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0096_a2d0, &[A::T(0x18)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x55)]),
            ],
            stores: &[(0x0, 4, 0x0103_34b4), (0x20, 4, 0x0)],
        });
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        for addr in [LIST_REMOVE_ALL, 0x0046_ffb0, 0x0048_3630] {
            e.register(addr, |_, _| Ret::default());
        }
        let freed = recorder(&mut e, FREE, 0);
        let form = list_form(&mut e, &[]);
        assert_eq!(e.call(0x0058_fa60, &[form, 0]).u32(), form);
        assert!(freed.borrow().is_empty());
        assert_eq!(e.call(0x0058_fa60, &[form, 3]).u32(), form);
        assert_eq!(*freed.borrow(), vec![vec![form]]);
    }

    #[test]
    fn destructor_stores_the_vtable_and_tears_down_in_order() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        for addr in [LIST_REMOVE_ALL, 0x0046_ffb0, 0x0048_3630] {
            e.register(addr, |_, _| Ret::default());
        }
        let form = list_form(&mut e, &[]);
        e.mem.set_u32(form + 0x20, 7);
        e.call_log = Some(vec![]);
        e.call(0x0058_fa90, &[form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(form), LIST_FORM_VTABLE);
        assert_eq!(e.mem.u32(form + 0x20), 0);
        let order: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(a, _)| [LIST_REMOVE_ALL, 0x0046_ffb0, 0x0048_3630].contains(a))
            .collect();
        assert_eq!(
            order,
            vec![
                (LIST_REMOVE_ALL, vec![form + 0x18]),
                (0x0046_ffb0, vec![form + 0x18]),
                (0x0048_3630, vec![form]),
            ]
        );
    }

    #[test]
    fn clear_empties_the_list_and_the_count() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let removed = recorder(&mut e, LIST_REMOVE_ALL, 0);
        let form = list_form(&mut e, &[5]);
        e.mem.set_u32(form + 0x20, 9);
        e.call(0x0058_fb00, &[form]);
        assert_eq!(*removed.borrow(), vec![vec![form + 0x18]]);
        assert_eq!(e.mem.u32(form + 0x20), 0);
    }

    #[test]
    fn copy_appends_the_source_items_when_it_is_a_list_form() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let added: Log = Rc::default();
        let a = added.clone();
        e.register_double(LIST_ADD_TAIL, move |e, w| {
            a.borrow_mut().push(vec![w[0], e.mem.u32(w[1])]);
            Ret::default()
        });
        let copied = recorder(&mut e, 0x0048_51b0, 0);
        let this = list_form(&mut e, &[]);
        let source = list_form(&mut e, &[0x100, 0x200]);
        // Not a list form: nothing.
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.call(0x0058_fb30, &[this, source]);
        assert!(added.borrow().is_empty() && copied.borrow().is_empty());
        let casts = recorder(&mut e, RT_DYNAMIC_CAST, source);
        e.call(0x0058_fb30, &[this, source]);
        assert_eq!(
            *casts.borrow(),
            vec![vec![source, 0, TYPE_TES_FORM, TYPE_BGS_LIST_FORM, 0]]
        );
        assert_eq!(
            *added.borrow(),
            vec![vec![this + 0x18, 0x100], vec![this + 0x18, 0x200]]
        );
        assert_eq!(*copied.borrow(), vec![vec![this, source]]);
    }

    #[test]
    fn compare_answers_true_on_any_difference() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(LIST_COUNT, |e, a| {
            let (mut n, mut node) = (0, a[0]);
            while node != 0 {
                n += (e.mem.u32(node) != 0) as u32;
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let components = recorder(&mut e, 0x0048_5270, 0);
        let this = list_form(&mut e, &[1, 2]);
        let same = list_form(&mut e, &[1, 2]);
        let shorter = list_form(&mut e, &[1]);
        let reordered = list_form(&mut e, &[2, 1]);
        assert!(!e.call(0x0058_fbc0, &[this, same]).bool());
        assert_eq!(*components.borrow(), vec![vec![this, same]]);
        assert!(e.call(0x0058_fbc0, &[this, shorter]).bool());
        assert!(e.call(0x0058_fbc0, &[this, reordered]).bool());
        assert_eq!(components.borrow().len(), 1);
        e.register(0x0048_5270, |_, _| ret(0xffff_ff01));
        assert!(e.call(0x0058_fbc0, &[this, same]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x0058_fbc0, &[this, same]).bool());
    }

    #[test]
    fn save_writes_one_lnam_per_item() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(FORM_ID, |_, a| ret(a[0] + 0x0100_0000));
        for addr in [0x0048_55a0, 0x0048_5910, 0x0048_5680] {
            e.register(addr, |_, _| Ret::default());
        }
        let form = list_form(&mut e, &[0x10, 0x20]);
        e.call_log = Some(vec![]);
        e.call(0x0058_fcc0, &[form]);
        let log = e.call_log.take().unwrap();
        let saved: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(a, _)| [0x0048_55a0, 0x0048_5910, 0x0048_5680].contains(a))
            .collect();
        assert_eq!(
            saved,
            vec![
                (0x0048_55a0, vec![form]),
                (0x0048_5910, vec![SIG_LNAM, 0x0100_0010]),
                (0x0048_5910, vec![SIG_LNAM, 0x0100_0020]),
                (0x0048_5680, vec![form]),
            ]
        );
    }

    #[test]
    fn load_reads_edid_obnd_and_lnam() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let chunks = Rc::new(RefCell::new(vec![
            0x1234_5678,
            SIG_LNAM,
            SIG_OBND,
            SIG_EDID,
        ]));
        let c = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(c.borrow_mut().pop().unwrap_or(0))
        });
        e.register(NEXT_CHUNK, |_, _| ret(1));
        e.register(CHUNK_SIZE, |_, _| ret(6));
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            e.mem.set_cstr(a[1], b"Name");
            Ret::default()
        });
        e.register(GET_CHUNK_DATA, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 0x0001_2345);
            Ret::default()
        });
        for addr in [LOAD_FORM, SET_FORM_FLAG_8] {
            e.register(addr, |_, _| Ret::default());
        }
        let edid: Log = Rc::default();
        let d = edid.clone();
        e.register_double(0x7000_0000, move |e, a| {
            d.borrow_mut()
                .push(e.mem.cstr(a[1]).iter().map(|b| *b as u32).collect());
            Ret::default()
        });
        let bounds = recorder(&mut e, 0x7000_0004, 0);
        let added: Log = Rc::default();
        let a = added.clone();
        e.register_double(LIST_ADD_TAIL, move |e, w| {
            a.borrow_mut().push(vec![w[0], e.mem.u32(w[1])]);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x138 / 4];
        slots[0x134 / 4] = 0x7000_0000;
        slots[0xe0 / 4] = 0x7000_0004;
        e.put_vtable(0x0600_0000, &slots);
        let form = list_form(&mut e, &[]);
        e.mem.set_u32(form, 0x0600_0000);
        let file = 0x0500_0000;
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_fd40, &[form, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOAD_FORM), vec![vec![form, file]]);
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![form, 0]]);
        assert_eq!(calls_to(&log, GET_CHUNK_DATA_SIZED)[0][2], 0x200);
        assert_eq!(
            *edid.borrow(),
            vec![b"Name".iter().map(|b| *b as u32).collect::<Vec<_>>()]
        );
        assert_eq!(*bounds.borrow(), vec![vec![form, file]]);
        assert_eq!(*added.borrow(), vec![vec![form + 0x18, 0x0001_2345]]);
        // The walk stops when the next chunk is missing.
        chunks.borrow_mut().extend([SIG_OBND, SIG_OBND]);
        e.register(NEXT_CHUNK, |_, _| ret(0));
        assert!(e.call(0x0058_fd40, &[form, file]).bool());
        assert_eq!(bounds.borrow().len(), 2);
    }

    /// Doubles for `InitItem`: ids below 0x100 + 0x1000 resolve to
    /// `id + 0x5000` unless listed as missing; the compile index adds
    /// 0x1000; removing after a node unlinks its next node; removing the
    /// first node pulls the second node into it.
    fn init_item_doubles(e: &mut Engine, missing: &'static [u32]) {
        list_doubles(e);
        e.register(FORM_FLAG_8_SET, |_, _| ret(0));
        e.register(0x0048_4e60, |_, _| ret(0x0400_0000));
        e.register(0x0048_5d50, |e, a| {
            assert_eq!(a[1], 0x0400_0000);
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x1000);
            Ret::default()
        });
        e.register_double(0x0048_39c0, move |_, a| {
            ret(if missing.contains(&a[0]) {
                0
            } else {
                a[0] + 0x5000
            })
        });
        e.register(0x0072_6c60, |e, a| {
            let form = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], form);
            Ret::default()
        });
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        e.register(0x7000_0000, |_, _| ret(0x0102_0304));
        e.register(0x005b_5e40, |_, _| Ret::default());
        e.register(0x0090_5330, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x7000_0000;
        e.put_vtable(0x0600_0000, &slots);
    }

    #[test]
    fn init_item_resolves_items_and_drops_missing_ones() {
        let mut e = Engine::new();
        init_item_doubles(&mut e, &[0x1002, 0x1011]);
        let form = list_form(&mut e, &[0x01, 0x02, 0x03]);
        e.mem.set_u32(form, 0x0600_0000);
        e.call_log = Some(vec![]);
        e.call(0x0058_fe50, &[form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, form), vec![0x6001, 0x6003]);
        assert_eq!(
            calls_to(&log, 0x005b_5e40),
            vec![vec![MISSING_FORM_MESSAGE, 0x1002, 0x0102_0304, 0x00ab_cdef]]
        );
        assert_eq!(calls_to(&log, 0x0090_5330).len(), 1);
        assert_eq!(calls_to(&log, 0x0090_5330)[0][0], form + 0x18);
        assert!(calls_to(&log, 0x0063_f7b0).is_empty());
        assert_eq!(calls_to(&log, SET_FORM_FLAG_8), vec![vec![form, 1]]);

        // A missing first item is removed in place and its node walked again.
        let form = list_form(&mut e, &[0x11, 0x12]);
        e.mem.set_u32(form, 0x0600_0000);
        e.call_log = Some(vec![]);
        e.call(0x0058_fe50, &[form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(items(&e, form), vec![0x6012]);
        assert_eq!(calls_to(&log, 0x0063_f7b0), vec![vec![form + 0x18]]);
        assert!(calls_to(&log, 0x0090_5330).is_empty());

        // Already initialized: nothing.
        e.register(FORM_FLAG_8_SET, |_, _| ret(0xffff_ff01));
        e.call_log = Some(vec![]);
        e.call(0x0058_fe50, &[form]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn get_item_index_counts_until_the_form() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let form = list_form(&mut e, &[0xa, 0xb, 0xc]);
        let index = |e: &mut Engine, item: u32| e.call(0x0058_ff60, &[form, item]).i32();
        assert_eq!(index(&mut e, 0xa), 0);
        assert_eq!(index(&mut e, 0xc), 2);
        assert_eq!(index(&mut e, 0xd), -1);
        let empty = list_form(&mut e, &[]);
        assert_eq!(e.call(0x0058_ff60, &[empty, 0xa]).i32(), 0);
    }
}
