//! `(unplaced)/run_005a8080` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a8080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the embedded struct at `+0x3c` of `this` (`this + 0x3c`).
/// `TESQuest` calls it to reach its `QUEST_DATA`; the engine map has no name
/// for the address, and other classes may share the code.
pub fn fn_005a8080(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x3c)
}

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// `TESForm::GetFile` (`00484e60`) and `TESForm::AddCompileIndex`
/// (`00485d50`, cdecl).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The two lists the record's entries go to (`005d43c0`: the `RCOD`
/// entries; `005a8080`: the others) and the condition holder
/// (`006a9540`), each answering an address in `this`.
const RCOD_LIST: u32 = 0x005d_43c0;
const OTHER_LIST: u32 = 0x005d_43e0;
const CONDITIONS: u32 = 0x006a_9540;
/// Empties a list (`00470470`, on the list).
const LIST_CLEAR: u32 = 0x0047_0470;
/// Adds an item to a list (`00905820`, given the item's address).
const LIST_ADD: u32 = 0x0090_5820;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_RCIL: u32 = 0x4c49_4352;
const CHUNK_RCOD: u32 = 0x444f_4352;
const CHUNK_RCQY: u32 = 0x5951_4352;

// Translated from 005a80a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the record (the engine map has no name): empties the condition
/// list (`00680fe0` on `006a9540`'s answer), the lists at `005a8080`,
/// `005d43c0` and `005d43e0` (`00470470`), then `007037c0(-1)`,
/// `0061cc40(0)`, `005a8040(0)` and `005a8060(0)`.
pub fn fn_005a80a0(e: &mut Engine, this: Ptr) {
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    e.call(0x0068_0fe0, &args![conditions]);
    let list = fn_005a8080(e, this);
    e.call(LIST_CLEAR, &args![list]);
    let list = e.call(RCOD_LIST, &args![this]).u32();
    e.call(LIST_CLEAR, &args![list]);
    let list = e.call(OTHER_LIST, &args![this]).u32();
    e.call(LIST_CLEAR, &args![list]);
    e.call(0x0070_37c0, &args![this, 0xffff_ffffu32]);
    e.call(0x0061_cc40, &args![this, 0u32]);
    e.call(0x005a_8040, &args![this, 0u32]);
    e.call(0x005a_8060, &args![this, 0u32]);
}

/// A new 8-byte entry (`00401000`, constructed by `005a7be0`, null when
/// the allocation fails).
fn new_entry(e: &mut Engine) -> u32 {
    let block = e.call(0x0040_1000, &args![8u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(0x005a_7be0, &args![block]).u32()
    }
}

/// A form id chunk read into a zeroed local and adjusted to `this`'s file.
fn read_form_id(e: &mut Engine, this: Ptr, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(FILE_CHUNK_DATA, &args![file, slot]);
        let owner = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, owner]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 005a8110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a8110 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// The record reader for form type 0x6A (`RET 4`: the `TESFile`; the
/// engine map has no name): vtable +0x18 first, then false unless the form
/// type byte is 0x6A; otherwise `TESForm::LoadForm`, `00484ab0(0)` and each
/// chunk: `EDID`, `OBND`, `FULL` (+0x18), `DATA` (0x10 bytes at +0x24,
/// swapped by `00414220`), `CTDA` (`TESCondition::Load` on `006a9540`'s
/// answer), `RCIL`/`RCOD` (a new entry, `005a7be0` on 8 allocated bytes,
/// and its form id adjusted to the record's file) and `RCQY` (`00537e90`
/// on the current entry with the chunk's word). After every chunk, while
/// there is a current entry whose `00559450` is not 0 and a form id, the
/// entry gets the id (`006ecd40`) and is added to the `RCOD` list
/// (`005d43c0`) after an `RCOD` chunk, to the list at `005a8080` after an
/// `RCIL` one. Returns true.
pub fn fn_005a8110(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    e.vcall(base, 0x18, &args![]);
    if form_type != 0x6a {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    let entry = e.mem.alloc(4);
    let mut id = 0u32;
    let mut rcod = false;
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_RCOD => {
                rcod = true;
                let new = new_entry(e);
                e.mem.set_u32(entry, new);
                id = read_form_id(e, this, file);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_CTDA => {
                let conditions = e.call(CONDITIONS, &args![this]).u32();
                e.call(0x0068_0960, &args![conditions, file]);
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x24, 0x10u32]);
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(0x0041_4220, &args![base + 0x24]);
                }
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_RCIL => {
                rcod = false;
                let new = new_entry(e);
                e.mem.set_u32(entry, new);
                id = read_form_id(e, this, file);
            }
            CHUNK_FULL => {
                let full = if this.is_null() { 0 } else { base + 0x18 };
                e.call(0x0048_7050, &args![full, file]);
            }
            CHUNK_RCQY => {
                let word = e.with_stack(4, |e, slot| {
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                let current = e.mem.u32(entry);
                e.call(0x0053_7e90, &args![current, word]);
            }
            _ => {}
        }
        let current = e.mem.u32(entry);
        if current != 0 && e.call(0x0055_9450, &args![current]).u32() != 0 && id != 0 {
            e.call(0x006e_cd40, &args![current, id]);
            let list = if rcod {
                e.call(RCOD_LIST, &args![this]).u32()
            } else {
                fn_005a8080(e, this).addr()
            };
            e.call(LIST_ADD, &args![list, entry]);
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.mem.free(entry);
    true
}

/// List helpers: whether a node is empty (`008256d0`, `AL`), its item word's address
/// (`006815c0`), the next node (`00726070`, also the entry's second word), removing the
/// first item (`0063f7b0`), and the first word of an entry (`00559450`).
const NODE_EMPTY: u32 = 0x0082_56d0;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_REMOVE_FIRST: u32 = 0x0063_f7b0;
const ENTRY_FIRST: u32 = 0x0055_9450;
/// `0084e3a0` (`ECX` = a form): the word saved for it (its form id, by the chunks it goes
/// into), and `00485910` (`cdecl`: chunk code, word) that writes one chunk.
const FORM_WORD: u32 = 0x0084_e3a0;
const WRITE_CHUNK: u32 = 0x0048_5910;
/// The recipe's getter/setter pairs (`ECX` only; setters one stack word).
const GET_006A1B60: u32 = 0x006a_1b60;
const GET_006286D0: u32 = 0x0062_86d0;
const GET_0059BB30: u32 = 0x0059_bb30;
const GET_0045CD60: u32 = 0x0045_cd60;

/// Writes `code` with the word of the entry's form (its second word) and `RCQY` with the
/// entry's first word, for each entry of `list` up to the first null one.
fn save_entries(e: &mut Engine, list: u32, code: u32) {
    let mut node = list;
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        node = e.call(NODE_NEXT, &args![node]).u32();
        let form = e.call(NODE_NEXT, &args![entry]).u32();
        let word = e.call(FORM_WORD, &args![form]).u32();
        e.call(WRITE_CHUNK, &args![code, word]);
        let count = e.call(ENTRY_FIRST, &args![entry]).u32();
        e.call(WRITE_CHUNK, &args![CHUNK_RCQY, count]);
    }
}

/// The word of the form `getter` answers for the recipe, or 0 when it answers null.
fn form_word_or_zero(e: &mut Engine, this: Ptr, getter: u32) -> u32 {
    if e.call(getter, &args![this]).u32() == 0 {
        return 0;
    }
    let form = e.call(getter, &args![this]).u32();
    e.call(FORM_WORD, &args![form]).u32()
}

// Translated from 005a8490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRecipe`'s form-save slot (the vtable slot at `01036b58`; unnamed in the engine map,
/// whose names for the getters come from folded code; `ECX`, plain `RET`):
/// `TESForm::StartForm`, `TESFullName::Save` on +0x18, the conditions saved (`00680910`
/// on `006a9540`'s answer), the words of the forms `006a1b60` and `006286d0` answer (0 for
/// none) into +8 and +0xc of `00430830`'s data, the 0x10 bytes at +0x24
/// (`TESForm::__SaveData`), byte-swapped before and back after when `00401500` says so
/// (`00414220`), the entries of [`fn_005a8080`]'s list as `RCIL`/`RCQY` chunks and those
/// of `005d43c0`'s as `RCOD`/`RCQY`, then `TESForm::CloseForm`.
pub fn fn_005a8490(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_7010, &args![base + 0x18]);
    let conditions = e.call(CONDITIONS, &args![this]).u32();
    e.call(0x0068_0910, &args![conditions]);
    let first = form_word_or_zero(e, this, GET_006A1B60);
    let data = e.call(0x0043_0830, &args![this]).u32();
    e.mem.set_u32(data + 8, first);
    let second = form_word_or_zero(e, this, GET_006286D0);
    let data = e.call(0x0043_0830, &args![this]).u32();
    e.mem.set_u32(data + 0xc, second);
    if e.call(0x0040_1500, &args![]).u8() != 0 {
        e.call(0x0041_4220, &args![base + 0x24]);
    }
    e.call(0x0048_5e20, &args![this, base + 0x24, 0x10u32]);
    if e.call(0x0040_1500, &args![]).u8() != 0 {
        e.call(0x0041_4220, &args![base + 0x24]);
    }
    let list = fn_005a8080(e, this).addr();
    save_entries(e, list, CHUNK_RCIL);
    let list = e.call(RCOD_LIST, &args![this]).u32();
    save_entries(e, list, CHUNK_RCOD);
    e.call(0x0048_5680, &args![this]);
}

/// Empties the list `list_of` answers for `this`, then appends a copy of each entry of the
/// source's (a new entry, `005a7be0`, given the source entry's first word through
/// `00537e90` and its second through `006ecd40`; `00905820` with the slot holding it).
fn copy_entries(e: &mut Engine, this: Ptr, source: u32, list_of: ListOf) {
    loop {
        let list = list_of(e, this);
        if e.call(NODE_EMPTY, &args![list]).u8() != 0 {
            break;
        }
        let list = list_of(e, this);
        e.call(LIST_REMOVE_FIRST, &args![list]);
    }
    let mut node = list_of(e, Ptr::new(source));
    e.with_stack(4, |e, slot| {
        while node != 0 && e.call(NODE_EMPTY, &args![node]).u8() == 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            let entry = e.mem.u32(item);
            let copy = new_entry(e);
            e.mem.set_u32(slot.addr(), copy);
            let copy = e.mem.u32(slot.addr());
            let first = e.call(ENTRY_FIRST, &args![entry]).u32();
            e.call(0x0053_7e90, &args![copy, first]);
            let second = e.call(NODE_NEXT, &args![entry]).u32();
            let copy = e.mem.u32(slot.addr());
            e.call(0x006e_cd40, &args![copy, second]);
            let list = list_of(e, this);
            e.call(LIST_ADD, &args![list, slot]);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
    });
}

/// A list getter of the recipe (answers the list's first node).
type ListOf = fn(&mut Engine, Ptr) -> u32;

fn other_list(e: &mut Engine, this: Ptr) -> u32 {
    fn_005a8080(e, this).addr()
}

fn rcod_list(e: &mut Engine, this: Ptr) -> u32 {
    e.call(RCOD_LIST, &args![this]).u32()
}

/// `__RTDynamicCast` (`00ec43fb`) and the RTTI type descriptors `.?AVTESForm@@` and
/// `.?AVTESRecipe@@`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_RECIPE: u32 = 0x0118_64b0;

fn as_recipe(e: &mut Engine, form: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_RECIPE, 0u32],
    )
    .u32()
}

// Translated from 005a8660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a8660 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef3d26, state variable around the two entry constructors) and the frame's cookie word are not translated.
/// `TESRecipe`'s `Copy` slot (the vtable slot at `01036c34`; unnamed in the engine map;
/// `ECX` and one stack word, `RET 4`): when `source` casts to a `TESRecipe`, copies the
/// conditions (`00680a70(theirs, 0)` on this one's), the 0x10 bytes at +0x24
/// (`00401460`), `TESForm::CopyAllComponents` from `source`, the entries of
/// [`fn_005a8080`]'s list and of `005d43c0`'s (each emptied first), then the four values
/// `0059bb30`→`007037c0`, `0045cd60`→`0061cc40`, `006a1b60`→`005a8040` and
/// `006286d0`→`005a8060`.
pub fn fn_005a8660(e: &mut Engine, this: Ptr, source: Ptr) {
    let recipe = as_recipe(e, source);
    if recipe == 0 {
        return;
    }
    let theirs = e.call(CONDITIONS, &args![recipe]).u32();
    let mine = e.call(CONDITIONS, &args![this]).u32();
    e.call(0x0068_0a70, &args![mine, theirs, 0u32]);
    e.call(
        0x0040_1460,
        &args![this.addr() + 0x24, recipe + 0x24, 0x10u32],
    );
    e.call(0x0048_51b0, &args![this, source]);
    copy_entries(e, this, recipe, other_list);
    copy_entries(e, this, recipe, rcod_list);
    for (getter, setter) in [
        (GET_0059BB30, 0x0070_37c0u32),
        (GET_0045CD60, 0x0061_cc40),
        (GET_006A1B60, 0x005a_8040),
        (GET_006286D0, 0x005a_8060),
    ] {
        let value = e.call(getter, &args![recipe]).u32();
        e.call(setter, &args![this, value]);
    }
}

/// Walks this one's and the other's list side by side: true (different) when the other's
/// has an entry where this one's has ended (null or empty), or when `005a7c00` (on this
/// one's entry, with the other's) answers true; false at the end of the other's.
fn entries_differ(e: &mut Engine, this: Ptr, other: u32, list_of: ListOf) -> bool {
    let mut mine = list_of(e, this);
    let mut theirs = list_of(e, Ptr::new(other));
    while theirs != 0 && e.call(NODE_EMPTY, &args![theirs]).u8() == 0 {
        if mine == 0 || e.call(NODE_EMPTY, &args![mine]).u8() != 0 {
            return true;
        }
        let item = e.call(NODE_ITEM, &args![mine]).u32();
        let my_entry = e.mem.u32(item);
        let item = e.call(NODE_ITEM, &args![theirs]).u32();
        let their_entry = e.mem.u32(item);
        if e.call(0x005a_7c00, &args![my_entry, their_entry]).u8() != 0 {
            return true;
        }
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
        mine = e.call(NODE_NEXT, &args![mine]).u32();
    }
    false
}

// Translated from 005a8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRecipe`'s `Compare` slot (the vtable slot at `01036c38`; unnamed in the engine map;
/// `ECX` and one stack word, `RET 4`): true (different) when `other` is not a `TESRecipe`,
/// when `006a1b60`, `006286d0`, `0059bb30` or `0045cd60` answer differently,
/// `TESForm::CompareAllComponents` or the conditions' compare (`00680b70`) answer true, the
/// two lists' counts (`005ae380`) differ, or an entry differs ([`fn_005a8080`]'s list,
/// then `005d43c0`'s); false otherwise.
pub fn fn_005a8910(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let recipe = as_recipe(e, other);
    if recipe == 0 {
        return true;
    }
    for getter in [GET_006A1B60, GET_006286D0, GET_0059BB30, GET_0045CD60] {
        let mine = e.call(getter, &args![this]).u32();
        if mine != e.call(getter, &args![recipe]).u32() {
            return true;
        }
    }
    if e.call(0x0048_5270, &args![this, recipe]).u8() != 0 {
        return true;
    }
    let theirs = e.call(CONDITIONS, &args![recipe]).u32();
    let mine = e.call(CONDITIONS, &args![this]).u32();
    if e.call(0x0068_0b70, &args![mine, theirs]).u8() != 0 {
        return true;
    }
    for list_of in [other_list as ListOf, rcod_list] {
        let list = list_of(e, this);
        let mine = e.call(0x005a_e380, &args![list]).u32();
        let list = list_of(e, Ptr::new(recipe));
        if mine != e.call(0x005a_e380, &args![list]).u32() {
            return true;
        }
    }
    if entries_differ(e, this, recipe, other_list) {
        return true;
    }
    entries_differ(e, this, recipe, rcod_list)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_8490, fn_005a8490(Ptr)),
        entry!(0x005a_8660, fn_005a8660(Ptr, Ptr)),
        entry!(0x005a_8910, fn_005a8910(Ptr, Ptr) -> bool),
        entry!(0x005a8080, fn_005a8080(Ptr) -> Ptr),
        entry!(0x005a_80a0, fn_005a80a0(Ptr)),
        entry!(0x005a_8110, fn_005a8110(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_0x3c_to_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x005a_8080, &args![0x1000u32]).u32(), 0x103c);
    }
}

#[cfg(test)]
mod record_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_005a80a0_clears_the_lists_and_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x10));
        e.register(RCOD_LIST, |_, a| ret(a[0] + 0x50));
        e.register(OTHER_LIST, |_, a| ret(a[0] + 0x58));
        quiet(
            &mut e,
            &[
                0x0068_0fe0,
                LIST_CLEAR,
                0x0070_37c0,
                0x0061_cc40,
                0x005a_8040,
                0x005a_8060,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x005a_80a0, &args![this]);
        assert_eq!(calls(&e, 0x0068_0fe0), vec![vec![this + 0x10]]);
        assert_eq!(
            calls(&e, LIST_CLEAR),
            vec![vec![this + 0x3c], vec![this + 0x50], vec![this + 0x58]]
        );
        assert_eq!(calls(&e, 0x0070_37c0), vec![vec![this, 0xffff_ffff]]);
        assert_eq!(calls(&e, 0x0061_cc40), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x005a_8040), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x005a_8060), vec![vec![this, 0]]);
    }

    fn record(form_type: u32, chunks: Vec<(u32, u32)>) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x140);
        for slot in [0x18u32, 0xe0, 0x134] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        e.register_double(FILE_FORM_TYPE, move |_, _| ret(form_type));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        e.register(FORM_GET_FILE, |_, _| ret(0x4400));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // New entries are numbered 0x6000, 0x6100, ...; `00559450` on an
        // entry is non-zero.
        let next = Rc::new(RefCell::new(0x6000u32));
        e.register(0x0040_1000, |_, _| ret(0x5000));
        e.register_double(0x005a_7be0, move |_, _| {
            let entry = *next.borrow();
            *next.borrow_mut() += 0x100;
            ret(entry)
        });
        e.register(0x0055_9450, |_, _| ret(1));
        e.register(CONDITIONS, |_, a| ret(a[0] + 0x10));
        e.register(RCOD_LIST, |_, a| ret(a[0] + 0x50));
        let added = Rc::new(RefCell::new(Vec::new()));
        let a2 = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            a2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x7000_0018,
                0x7000_00e0,
                0x7000_0134,
                FILE_CHUNK_DATA_SIZED,
                0x0048_5110,
                0x0048_4ab0,
                0x0068_0960,
                0x0048_61f0,
                0x0041_4220,
                0x0048_7050,
                0x0053_7e90,
                0x006e_cd40,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x005a_8110, &args![this, file]).bool();
        assert_eq!(result, form_type == 0x6a);
        (e, this, file)
    }

    fn added(e: &Engine) -> Vec<Vec<u32>> {
        calls(e, LIST_ADD)
    }

    #[test]
    fn fn_005a8110_reads_the_chunks() {
        let (e, this, file) = record(
            0x6a,
            vec![
                (CHUNK_EDID, 0),
                (CHUNK_OBND, 0),
                (CHUNK_FULL, 0),
                (CHUNK_DATA, 0),
                (CHUNK_CTDA, 0),
            ],
        );
        assert_eq!(calls(&e, 0x7000_0018), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x24, 0x10]]
        );
        assert_eq!(calls(&e, 0x0041_4220), vec![vec![this + 0x24]]);
        assert_eq!(calls(&e, 0x0068_0960), vec![vec![this + 0x10, file]]);
        assert!(added(&e).is_empty());
    }

    #[test]
    fn fn_005a8110_adds_the_entries_after_each_chunk() {
        let (e, this, _) = record(
            0x6a,
            vec![(CHUNK_RCIL, 0x12), (CHUNK_RCQY, 7), (CHUNK_RCOD, 0x34)],
        );
        assert_eq!(calls(&e, 0x005a_7be0).len(), 2);
        // RCIL: the entry is added to the list at +0x3c after its own chunk
        // and again after the RCQY chunk; RCOD's to the other list.
        let adds: Vec<u32> = added(&e).iter().map(|c| c[0]).collect();
        assert_eq!(adds, vec![this + 0x3c, this + 0x3c, this + 0x50]);
        assert_eq!(
            calls(&e, 0x006e_cd40),
            vec![
                vec![0x6000, 0x0100_0012],
                vec![0x6000, 0x0100_0012],
                vec![0x6100, 0x0100_0034]
            ]
        );
        assert_eq!(calls(&e, 0x0053_7e90), vec![vec![0x6000, 7]]);
    }

    #[test]
    fn fn_005a8110_refuses_other_records() {
        let (e, _, _) = record(0x69, vec![(CHUNK_DATA, 0)]);
        assert_eq!(calls(&e, 0x7000_0018).len(), 1);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }
}

#[cfg(test)]
mod recipe_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Lists are nodes [item, next], empty when the item is 0; the `RCOD` list of a recipe
    /// sits at +0x60 (the other at +0x3c, [`fn_005a8080`]). Entries are [count, form].
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ENTRY_FIRST, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(RCOD_LIST, |_, a| (a[0] + 0x60).into_ret());
        e.register(FORM_WORD, |_, a| (a[0] | 0x0100_0000).into_ret());
        e
    }

    fn recipe(e: &mut Engine, entries: &[(u32, u32)], rcod: &[(u32, u32)]) -> u32 {
        let this = e.mem.alloc(0x80);
        for (offset, list) in [(0x3cu32, entries), (0x60, rcod)] {
            let mut node = this + offset;
            for (i, (count, form)) in list.iter().enumerate() {
                let entry = e.mem.alloc(8);
                e.mem.set_u32(entry, *count);
                e.mem.set_u32(entry + 4, *form);
                e.mem.set_u32(node, entry);
                if i + 1 < list.len() {
                    let next = e.mem.alloc(8);
                    e.mem.set_u32(node + 4, next);
                    node = next;
                }
            }
        }
        this
    }

    #[test]
    fn fn_005a8490_saves_the_recipe() {
        let mut e = engine();
        for callee in [
            0x0048_55a0u32,
            0x0048_7010,
            0x0068_0910,
            0x0048_5e20,
            0x0048_5680,
            WRITE_CHUNK,
            0x0041_4220,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(CONDITIONS, |_, a| (a[0] + 0x70).into_ret());
        e.register(0x0040_1500, |_, _| 1u32.into_ret());
        e.register(GET_006A1B60, |_, _| 0x55u32.into_ret());
        e.register(GET_006286D0, |_, _| 0u32.into_ret());
        let data = e.mem.alloc(0x10);
        e.register_double(0x0043_0830, move |_, _| data.into_ret());
        let this = recipe(&mut e, &[(2, 0x11), (3, 0x22)], &[(1, 0x33)]);
        e.call_log = Some(vec![]);
        e.call(0x005a_8490, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(data + 8), 0x0100_0055);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(calls(&log, 0x0068_0910), vec![vec![this + 0x70]]);
        assert_eq!(calls(&log, 0x0041_4220).len(), 2);
        assert_eq!(
            calls(&log, WRITE_CHUNK),
            vec![
                vec![CHUNK_RCIL, 0x0100_0011],
                vec![CHUNK_RCQY, 2],
                vec![CHUNK_RCIL, 0x0100_0022],
                vec![CHUNK_RCQY, 3],
                vec![CHUNK_RCOD, 0x0100_0033],
                vec![CHUNK_RCQY, 1],
            ]
        );
        assert_eq!(log.last().unwrap(), &(0x0048_5680, vec![this]));
    }

    #[test]
    fn fn_005a8660_copies_a_recipe() {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(CONDITIONS, |_, a| (a[0] + 0x70).into_ret());
        for callee in [
            0x0068_0a70u32,
            0x0040_1460,
            0x0048_51b0,
            0x0053_7e90,
            0x006e_cd40,
            LIST_ADD,
            0x0070_37c0,
            0x0061_cc40,
            0x005a_8040,
            0x005a_8060,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(0x0040_1000, |e, _| e.mem.alloc(8).into_ret());
        e.register(0x005a_7be0, |_, a| a[0].into_ret());
        // Removing the first item empties a one-entry list.
        e.register(LIST_REMOVE_FIRST, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        for getter in [GET_0059BB30, GET_0045CD60, GET_006A1B60, GET_006286D0] {
            e.register_double(getter, move |_, _| (getter & 0xff).into_ret());
        }
        let this = recipe(&mut e, &[(9, 0x99)], &[]);
        let source = recipe(&mut e, &[(2, 0x11), (3, 0x22)], &[(1, 0x33)]);
        e.call_log = Some(vec![]);
        e.call(0x005a_8660, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x0068_0a70),
            vec![vec![this + 0x70, source + 0x70, 0]]
        );
        assert_eq!(
            calls(&log, 0x0040_1460),
            vec![vec![this + 0x24, source + 0x24, 0x10]]
        );
        assert_eq!(calls(&log, LIST_REMOVE_FIRST), vec![vec![this + 0x3c]]);
        let firsts: Vec<u32> = calls(&log, 0x0053_7e90).iter().map(|w| w[1]).collect();
        assert_eq!(firsts, vec![2, 3, 1]);
        let seconds: Vec<u32> = calls(&log, 0x006e_cd40).iter().map(|w| w[1]).collect();
        assert_eq!(seconds, vec![0x11, 0x22, 0x33]);
        let lists: Vec<u32> = calls(&log, LIST_ADD).iter().map(|w| w[0]).collect();
        assert_eq!(lists, vec![this + 0x3c, this + 0x3c, this + 0x60]);
        assert_eq!(calls(&log, 0x0070_37c0), vec![vec![this, 0x30]]);
        assert_eq!(calls(&log, 0x005a_8060), vec![vec![this, 0xd0]]);
    }

    #[test]
    fn fn_005a8910_compares_recipes() {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(CONDITIONS, |_, a| (a[0] + 0x70).into_ret());
        for getter in [GET_006A1B60, GET_006286D0, GET_0059BB30, GET_0045CD60] {
            e.register(getter, |_, _| 7u32.into_ret());
        }
        e.register(0x0048_5270, |_, _| 0u32.into_ret());
        e.register(0x0068_0b70, |_, _| 0u32.into_ret());
        // Counts: the number of nodes with an item.
        e.register(0x005a_e380, |e, a| {
            let mut node = a[0];
            let mut count = 0u32;
            while node != 0 && e.mem.u32(node) != 0 {
                count += 1;
                node = e.mem.u32(node + 4);
            }
            count.into_ret()
        });
        e.register(0x005a_7c00, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        let one = recipe(&mut e, &[(2, 0x11)], &[(1, 0x33)]);
        let same = recipe(&mut e, &[(2, 0x11)], &[(1, 0x33)]);
        let other = recipe(&mut e, &[(2, 0x12)], &[(1, 0x33)]);
        let longer = recipe(&mut e, &[(2, 0x11)], &[(1, 0x33), (1, 0x34)]);
        assert!(!e.call(0x005a_8910, &args![one, same]).bool());
        assert!(e.call(0x005a_8910, &args![one, other]).bool());
        assert!(e.call(0x005a_8910, &args![one, longer]).bool());
        e.register(GET_0045CD60, |_, a| a[0].into_ret());
        assert!(e.call(0x005a_8910, &args![one, same]).bool());
        e.register(GET_0045CD60, |_, _| 7u32.into_ret());
        e.register(0x0068_0b70, |_, _| 1u32.into_ret());
        assert!(e.call(0x005a_8910, &args![one, same]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x005a_8910, &args![one, same]).bool());
    }
}
