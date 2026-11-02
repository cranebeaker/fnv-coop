//! `fallout shared/magic/run_0040c0e0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` (Xbox PDB) base constructor for a word value (`00404920`).
const SETTING_CONSTRUCT_WORD: u32 = 0x0040_4920;
/// The game-setting vtable.
const GAME_SETTING_VTABLE: u32 = 0x0101_2114;
/// The game-setting collection (`00404a70`: created on first use).
const GAME_SETTINGS: u32 = 0x0040_4a70;
/// Slot of the collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 0040c150 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a game setting with a word value (an integer, a
/// string pointer; `thiscall`, `RET 8`): the base constructor (`00404920`)
/// with the name and the value, the game-setting vtable (`01012114`), then
/// adds itself to the game-setting collection (`00404a70`, vtable slot +4).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn fn_0040c150(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_WORD, &args![this, name, value]);
    e.mem.set_u32(this.addr(), GAME_SETTING_VTABLE);
    let collection = e.call(GAME_SETTINGS, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// `TESForm::LoadForm` (Xbox PDB name of `00485110`; `ECX` = form, the file):
/// reads the record header into the form.
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB name of `004726b0`; `ECX` = file): the id
/// of the current chunk, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk; true when there is one.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// The current chunk size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): the byte at `+0x299`.
const FILE_FLAG_299: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall`
/// (file; buffer, size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `memset` through its wrapper `00403d30` (`cdecl`: buffer, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// The component loader `00487050` (`cdecl`: component, file).
const LOAD_FULL_NAME: u32 = 0x0048_7050;
/// `00408da0` (`ECX` = the component at `+0x30`): its name text, or the empty
/// string `01011584`.
const FULL_NAME_TEXT: u32 = 0x0040_8da0;
/// `EffectItemList::Load` (Xbox PDB name of `00405760`; `ECX` = the list
/// component at `+0x3c`, the file, the owner's name text).
const EFFECT_ITEM_LIST_LOAD: u32 = 0x0040_5760;
/// `VATS::GetCount` (the engine map name of `005ae380`; `ECX` = a list
/// head): how many nodes of the list hold a non-null item.
const LIST_COUNT: u32 = 0x005a_e380;
/// `0084e3a0` (`ECX` = form): the word at +0xC (`TESForm::iFormID`).
const FORM_ID: u32 = 0x0084_e3a0;
/// `005b5e40` (`cdecl` format, ...): the log print.
const PRINT: u32 = 0x005b_5e40;

/// `"GAME EFFECTS: Effect Item (%08X) %s has no effects defined."`
const MESSAGE_NO_EFFECTS: u32 = 0x0101_361c;

/// Chunk ids, as the little-endian words of their four letters.
const CHUNK_EFID: u32 = 0x4449_4645;
const CHUNK_CTDA: u32 = 0x4144_5443;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;

/// The object's virtual slot called with the file for an `OBND` chunk.
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
/// The object's virtual slot called with the text of an `EDID` chunk.
const SLOT_SET_EDITOR_ID: u32 = 0x134;
/// The object's virtual slot called when the file's flag at `+0x299` is set
/// after the sub-object read its own chunk.
const SLOT_AFTER_LOAD: u32 = 0x180;
/// The slots of the object at `+0x30` (the one the name text comes from):
/// the chunk offered to it (file, chunk id), the chunk id it reads itself,
/// the address and the size of the block it reads that chunk into.
const SLOT_OFFER_CHUNK: u32 = 0x40;
const SLOT_OWN_CHUNK_ID: u32 = 0x24;
const SLOT_OWN_BLOCK: u32 = 0x28;
const SLOT_OWN_BLOCK_SIZE: u32 = 0x2c;

// Translated from 0040c650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040c650 - the /GS stack cookie check (00ec408c) is not translated.
/// The engine map gives it no name. `thiscall` (`RET 4`, `file`), answers
/// true: `TESForm::LoadForm`, then for every chunk of the record: an `EFID` or
/// `CTDA` chunk goes to `EffectItemList::Load` on the list at `+0x3c` with the
/// name text of the `+0x30` component; `EDID` is read into a stack buffer
/// (at most 0x200 bytes) and passed to virtual slot `0x134`; `OBND` goes to
/// slot `0xe0`; `FULL` to `00487050` with the `+0x30` component (null for a
/// null object). Every chunk is then offered to the `+0x30` component's slot
/// `0x40`; when it equals the chunk id slot `0x24` of that component names,
/// its block (slots `0x28`, `0x2c`) is zeroed and read from the file, and if
/// the file's flag at `+0x299` is set the object's slot `0x180` runs. Last, a
/// record whose list at `+0x40` holds no item prints the `GAME EFFECTS`
/// message with the form id and name. The stack cookie check is not
/// translated.
pub fn fn_0040c650(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    let component = object + 0x30;
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EFID | CHUNK_CTDA => {
                let name = e.call(FULL_NAME_TEXT, &args![component]).u32();
                e.call(EFFECT_ITEM_LIST_LOAD, &args![object + 0x3c, file, name]);
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_FULL => {
                let target = if object == 0 { 0 } else { component };
                e.call(LOAD_FULL_NAME, &args![target, file]);
            }
            _ => {}
        }
        e.vcall(component, SLOT_OFFER_CHUNK, &args![file, chunk]);
        if chunk == e.vcall(component, SLOT_OWN_CHUNK_ID, &[]).u32() {
            let size = e.vcall(component, SLOT_OWN_BLOCK_SIZE, &[]).u32();
            let block = e.vcall(component, SLOT_OWN_BLOCK, &[]).u32();
            e.call(MEMSET, &args![block, 0u32, size]);
            let size = e.vcall(component, SLOT_OWN_BLOCK_SIZE, &[]).u32();
            let block = e.vcall(component, SLOT_OWN_BLOCK, &[]).u32();
            e.call(GET_CHUNK_DATA, &args![file, block, size]);
            if e.call(FILE_FLAG_299, &args![file]).bool() {
                e.vcall(object, SLOT_AFTER_LOAD, &[]);
            }
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if e.call(LIST_COUNT, &args![object + 0x40]).u32() == 0 {
        let name = e.call(FULL_NAME_TEXT, &args![component]).u32();
        let form_id = e.call(FORM_ID, &args![this]).u32();
        e.call(PRINT, &args![MESSAGE_NO_EFFECTS, form_id, name]);
    }
    true
}

/// The string constructor `00403970` (`thiscall`, no stack words) and the assignment
/// `004037f0` (`thiscall`, two words: the text and 0).
const STRING_CONSTRUCT_00403970: u32 = 0x0040_3970;
const STRING_SET_004037F0: u32 = 0x0040_37f0;
/// `WaitForSingleObjectEx` (engine map name of `004019a0`, `cdecl`, one word).
const WAIT_004019A0: u32 = 0x0040_19a0;
/// `004013e0` (`thiscall`, no stack words; answers a byte).
const QUERY_004013E0: u32 = 0x0040_13e0;
/// `MagicItem::MagicItemInitItem` (engine map, `0040c030`, `thiscall` on the component at
/// `+0x18`, one word: the form).
const MAGIC_ITEM_INIT_ITEM: u32 = 0x0040_c030;
/// `00484ab0` (`fallout shared/tesform.cpp`, `thiscall`, one word).
const FORM_SET_00484AB0: u32 = 0x0048_4ab0;
/// The object's virtual slot called when the file's flag at `+0x299` is set after the
/// `+0x18` component read its own chunk (`fn_0040c230`).
const SLOT_AFTER_LOAD_0040C230: u32 = 0x138;

// Translated from 0040c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a string constructor from text (`thiscall`, one word):
/// `00403970` on `this`, then `004037f0(text, 0)`. Answers `this`.
pub fn fn_0040c0e0(e: &mut Engine, this: Ptr, text: u32) -> Ptr {
    e.call(STRING_CONSTRUCT_00403970, &args![this]);
    e.call(STRING_SET_004037F0, &args![this, text, 0u32]);
    this
}

// Translated from 0040c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the word at `this` is non-null, [`fn_0040c130`] on it.
pub fn fn_0040c110(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr());
    if object != 0 {
        fn_0040c130(e, Ptr::new(object));
    }
}

// Translated from 0040c130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `WaitForSingleObjectEx` (`004019a0`) on `this + 4`.
pub fn fn_0040c130(e: &mut Engine, this: Ptr) {
    e.call(WAIT_004019A0, &args![this.addr().wrapping_add(4)]);
}

// Translated from 0040c230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040c230 - the /GS stack cookie check (00ec408c) is not translated.
/// The engine map gives it no name. The same load as [`fn_0040c650`] for an object whose
/// name component is at `+0x18`, effect list at `+0x24` and item list at `+0x28`
/// (`thiscall`, `RET 4`, `file`; answers true): `TESForm::LoadForm`, then for every chunk:
/// `EFID`/`CTDA` to `EffectItemList::Load` on `+0x24` with the `+0x18` component's name
/// text; `EDID` read into a stack buffer (the chunk size rounded up to 16 bytes, at most
/// 0x200 bytes read) and passed to slot `0x134`; `OBND` to slot `0xe0`; `FULL` to `00487050`
/// with the `+0x18` component (null for a null object). Every chunk is offered to the
/// component's slot `0x40`; its own chunk (slot `0x24`) is zeroed and read into its block
/// (slots `0x28`, `0x2c`), and with the file's flag at `+0x299` set the object's slot
/// `0x138` runs. A record whose `+0x28` list holds no item prints the `GAME EFFECTS`
/// message with the form id and name.
pub fn fn_0040c230(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let object = this.addr();
    let component = object.wrapping_add(0x18);
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EFID | CHUNK_CTDA => {
                let name = e.call(FULL_NAME_TEXT, &args![component]).u32();
                e.call(
                    EFFECT_ITEM_LIST_LOAD,
                    &args![object.wrapping_add(0x24), file, name],
                );
            }
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(object, SLOT_SET_EDITOR_ID, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(object, SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_FULL => {
                let target = if object == 0 { 0 } else { component };
                e.call(LOAD_FULL_NAME, &args![target, file]);
            }
            _ => {}
        }
        e.vcall(component, SLOT_OFFER_CHUNK, &args![file, chunk]);
        if chunk == e.vcall(component, SLOT_OWN_CHUNK_ID, &[]).u32() {
            let size = e.vcall(component, SLOT_OWN_BLOCK_SIZE, &[]).u32();
            let block = e.vcall(component, SLOT_OWN_BLOCK, &[]).u32();
            e.call(MEMSET, &args![block, 0u32, size]);
            let size = e.vcall(component, SLOT_OWN_BLOCK_SIZE, &[]).u32();
            let block = e.vcall(component, SLOT_OWN_BLOCK, &[]).u32();
            e.call(GET_CHUNK_DATA, &args![file, block, size]);
            if e.call(FILE_FLAG_299, &args![file]).bool() {
                e.vcall(object, SLOT_AFTER_LOAD_0040C230, &[]);
            }
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if e.call(LIST_COUNT, &args![object.wrapping_add(0x28)]).u32() == 0 {
        let name = e.call(FULL_NAME_TEXT, &args![component]).u32();
        let form_id = e.call(FORM_ID, &args![this]).u32();
        e.call(PRINT, &args![MESSAGE_NO_EFFECTS, form_id, name]);
    }
    true
}

// Translated from 0040c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless `004013e0` answers non-zero (its low byte), runs
/// `MagicItem::MagicItemInitItem` on the component at `+0x18` with the form and then
/// `00484ab0(1)` on the form.
pub fn fn_0040c5b0(e: &mut Engine, this: Ptr) {
    if e.call(QUERY_004013E0, &args![this]).bool() {
        return;
    }
    e.call(
        MAGIC_ITEM_INIT_ITEM,
        &args![this.addr().wrapping_add(0x18), this],
    );
    e.call(FORM_SET_00484AB0, &args![this, 1u32]);
}

/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`) and
/// `TESFullName::Save` (`00487010`) (engine map), thiscall.
const TES_FORM_START_FORM: u32 = 0x0048_55a0;
const TES_FORM_CLOSE_FORM: u32 = 0x0048_5680;
const TES_FULL_NAME_SAVE: u32 = 0x0048_7010;
/// Thiscall on an `EffectItemList` (`004056d0`, `effectitemlist.cpp` range):
/// saves it.
const EFFECT_LIST_SAVE_004056D0: u32 = 0x0040_56d0;
/// `TESForm::CompareAllComponents` (`00485270`, `AL`) and
/// `TESForm::CopyAllComponents` (`004851b0`) (engine map), thiscall (other
/// form), `RET 4`.
const TES_FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const TES_FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// Thiscall on an `EffectItemList` (other list), `RET 4`: whether they
/// differ (`00405a00`) and the copy (`00405b10`) (`effectitemlist.rs`).
const EFFECT_LIST_DIFFERS: u32 = 0x0040_5a00;
const EFFECT_LIST_COPY: u32 = 0x0040_5b10;
/// `__RTDynamicCast` (CRT), cdecl (object, offset, source type, target type,
/// is-reference); the type descriptors of `TESForm`, `MagicItemForm` and
/// `MagicItemObject`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_MAGIC_ITEM_FORM: u32 = 0x0118_395c;
const TYPE_MAGIC_ITEM_OBJECT: u32 = 0x0118_30e8;
/// The `MagicItem` base's virtual slots the save, compare and copy use:
/// two saves (+0x38, +0x3c), the compare (+0x30, `AL`) and the copy (+0x34).
const SLOT_MAGIC_ITEM_SAVE_A: u32 = 0x38;
const SLOT_MAGIC_ITEM_SAVE_B: u32 = 0x3c;
const SLOT_MAGIC_ITEM_COMPARE: u32 = 0x30;
const SLOT_MAGIC_ITEM_COPY: u32 = 0x34;

/// Where a magic item form keeps its parts: the `MagicItem` base and its
/// `EffectItemList`, and the RTTI type of the class.
struct MagicItemParts {
    magic_item: u32,
    effects: u32,
    class_type: u32,
}

/// `MagicItemForm`: the `MagicItem` at +0x18, the list at +0x24.
const MAGIC_ITEM_FORM_PARTS: MagicItemParts = MagicItemParts {
    magic_item: 0x18,
    effects: 0x24,
    class_type: TYPE_MAGIC_ITEM_FORM,
};
/// `MagicItemObject`: the `MagicItem` at +0x30, the list at +0x3c.
const MAGIC_ITEM_OBJECT_PARTS: MagicItemParts = MagicItemParts {
    magic_item: 0x30,
    effects: 0x3c,
    class_type: TYPE_MAGIC_ITEM_OBJECT,
};

/// `p + offset`, or 0 for a null `p`.
fn part(p: u32, offset: u32) -> u32 {
    if p == 0 {
        0
    } else {
        p.wrapping_add(offset)
    }
}

/// The shared body of `0040c1d0` and `0040c5f0`.
fn save_magic_item_form(e: &mut Engine, this: Ptr, parts: &MagicItemParts) {
    let base = this.addr();
    e.call(TES_FORM_START_FORM, &args![this]);
    let magic_item = base.wrapping_add(parts.magic_item);
    e.call(TES_FULL_NAME_SAVE, &args![magic_item]);
    e.vcall(magic_item, SLOT_MAGIC_ITEM_SAVE_A, &args![]);
    e.vcall(magic_item, SLOT_MAGIC_ITEM_SAVE_B, &args![]);
    e.call(
        EFFECT_LIST_SAVE_004056D0,
        &args![base.wrapping_add(parts.effects)],
    );
    e.call(TES_FORM_CLOSE_FORM, &args![this]);
}

/// The shared body of `0040c450` and `0040c870`.
fn compare_magic_item_form(e: &mut Engine, this: Ptr, other: u32, parts: &MagicItemParts) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, parts.class_type, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    if e.call(TES_FORM_COMPARE_ALL_COMPONENTS, &args![this, other])
        .bool()
    {
        return true;
    }
    let base = this.addr();
    let magic_item = base.wrapping_add(parts.magic_item);
    if e.vcall(
        magic_item,
        SLOT_MAGIC_ITEM_COMPARE,
        &args![part(cast, parts.magic_item)],
    )
    .bool()
    {
        return true;
    }
    e.call(
        EFFECT_LIST_DIFFERS,
        &args![base.wrapping_add(parts.effects), part(cast, parts.effects)],
    )
    .bool()
}

/// The shared body of `0040c510` and `0040c930`.
fn copy_magic_item_form(e: &mut Engine, this: Ptr, other: u32, parts: &MagicItemParts) {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, TYPE_TES_FORM, parts.class_type, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    e.call(TES_FORM_COPY_ALL_COMPONENTS, &args![this, other]);
    let base = this.addr();
    e.vcall(
        base.wrapping_add(parts.magic_item),
        SLOT_MAGIC_ITEM_COPY,
        &args![part(cast, parts.magic_item)],
    );
    e.call(
        EFFECT_LIST_COPY,
        &args![base.wrapping_add(parts.effects), part(cast, parts.effects)],
    );
}

// Translated from 0040c1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `MagicItemForm`, plain `RET`):
/// saves the form: `TESForm::StartForm`, the `MagicItem` at +0x18
/// (`TESFullName::Save`, then its virtual slots +0x38 and +0x3c), the
/// effect list at +0x24 (`004056d0`), `TESForm::CloseForm`.
pub fn fn_0040c1d0(e: &mut Engine, this: Ptr) {
    save_magic_item_form(e, this, &MAGIC_ITEM_FORM_PARTS);
}

// Translated from 0040c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `MagicItemForm`, `RET 4`):
/// whether `other` differs: true when it is not a `MagicItemForm`
/// (`__RTDynamicCast` from `TESForm`), when `TESForm::CompareAllComponents`
/// says so, when the `MagicItem` (+0x18) virtual slot +0x30 says so for the
/// other's `MagicItem`, or when the effect lists (+0x24) differ
/// (`00405a00`).
pub fn fn_0040c450(e: &mut Engine, this: Ptr, other: u32) -> bool {
    compare_magic_item_form(e, this, other, &MAGIC_ITEM_FORM_PARTS)
}

// Translated from 0040c510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `MagicItemForm`, `RET 4`):
/// when `other` is a `MagicItemForm`, copies it: `TESForm::CopyAllComponents`,
/// the `MagicItem` (+0x18) virtual slot +0x34 with the other's `MagicItem`,
/// then the effect list (+0x24, `00405b10`); otherwise nothing.
pub fn fn_0040c510(e: &mut Engine, this: Ptr, other: u32) {
    copy_magic_item_form(e, this, other, &MAGIC_ITEM_FORM_PARTS);
}

// Translated from 0040c5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `MagicItemObject`, plain
/// `RET`): `0040c1d0`'s save with the `MagicItem` at +0x30 and the effect
/// list at +0x3c.
pub fn fn_0040c5f0(e: &mut Engine, this: Ptr) {
    save_magic_item_form(e, this, &MAGIC_ITEM_OBJECT_PARTS);
}

// Translated from 0040c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `MagicItemObject`, `RET 4`):
/// `0040c450`'s comparison for a `MagicItemObject` (the `MagicItem` at
/// +0x30, the effect list at +0x3c).
pub fn fn_0040c870(e: &mut Engine, this: Ptr, other: u32) -> bool {
    compare_magic_item_form(e, this, other, &MAGIC_ITEM_OBJECT_PARTS)
}

// Translated from 0040c930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `MagicItemObject`, `RET 4`):
/// `0040c510`'s copy for a `MagicItemObject` (the `MagicItem` at +0x30, the
/// effect list at +0x3c).
pub fn fn_0040c930(e: &mut Engine, this: Ptr, other: u32) {
    copy_magic_item_form(e, this, other, &MAGIC_ITEM_OBJECT_PARTS);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040c150, fn_0040c150(Ptr, u32, u32) -> Ptr),
        entry!(0x0040c650, fn_0040c650(Ptr, Ptr) -> bool),
        entry!(0x0040_c0e0, fn_0040c0e0(Ptr, u32) -> Ptr),
        entry!(0x0040_c110, fn_0040c110(Ptr)),
        entry!(0x0040_c130, fn_0040c130(Ptr)),
        entry!(0x0040_c230, fn_0040c230(Ptr, Ptr) -> bool),
        entry!(0x0040_c5b0, fn_0040c5b0(Ptr)),
        entry!(0x0040_c1d0, fn_0040c1d0(Ptr)),
        entry!(0x0040_c450, fn_0040c450(Ptr, u32) -> bool),
        entry!(0x0040_c510, fn_0040c510(Ptr, u32)),
        entry!(0x0040_c5f0, fn_0040c5f0(Ptr)),
        entry!(0x0040_c870, fn_0040c870(Ptr, u32) -> bool),
        entry!(0x0040_c930, fn_0040c930(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_game_setting_adds_itself_to_the_collection() {
        let mut e = Engine::new();
        let collection = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(collection, 0x0090_0000);
        e.register_double(GAME_SETTINGS, move |_, _| collection.into_ret());
        e.register_double(0x0090_1000, |e, a| {
            assert_eq!(e.mem.u32(a[1]), GAME_SETTING_VTABLE);
            Ret::default()
        });
        let s = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040c150, &[s, 0x66, 7]).u32(), s);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SETTING_CONSTRUCT_WORD, vec![s, 0x66, 7]));
        assert_eq!(log[2].0, GAME_SETTINGS);
        assert_eq!(log[3], (0x0090_1000, vec![collection, s]));
        assert_eq!(e.mem.u32(s + 4), 7);
        assert_eq!(e.mem.u32(s + 8), 0x66);
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const FILE: u32 = 0x6000;
    const OBJECT_TABLE: u32 = 0x0700_0000;
    const COMPONENT_TABLE: u32 = 0x0710_0000;
    const OWN_CHUNK: u32 = 0x1234;
    const OWN_BLOCK: u32 = 0x9000;
    const OWN_BLOCK_SIZE: u32 = 20;
    const TARGET_LOAD_BOUNDS: u32 = 0x0050_0001;
    const TARGET_SET_EDITOR_ID: u32 = 0x0050_0002;
    const TARGET_AFTER_LOAD: u32 = 0x0050_0003;
    const TARGET_OFFER_CHUNK: u32 = 0x0050_0004;
    const TARGET_OWN_CHUNK_ID: u32 = 0x0050_0005;
    const TARGET_OWN_BLOCK: u32 = 0x0050_0006;
    const TARGET_OWN_BLOCK_SIZE: u32 = 0x0050_0007;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Runs the loader on a record of `chunks`; `count` is what the list
    /// count answers, `flag` the file flag at `+0x299`. Returns the call log
    /// (without the top-level call), the object and the result.
    fn run(chunks: Vec<u32>, count: u32, flag: u32) -> (CallLog, u32, bool) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            ret(list.get(at.get()).copied().unwrap_or(0))
        });
        let total = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            ret(u32::from(position.get() < total))
        });
        for addr in [
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            MEMSET,
            LOAD_FULL_NAME,
            EFFECT_ITEM_LIST_LOAD,
            PRINT,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
            TARGET_AFTER_LOAD,
            TARGET_OFFER_CHUNK,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(CHUNK_SIZE, |_, _| ret(5));
        e.register_double(FILE_FLAG_299, move |_, _| ret(flag));
        e.register(FULL_NAME_TEXT, |_, _| ret(0x5000));
        e.register(FORM_ID, |_, _| ret(0x77));
        e.register_double(LIST_COUNT, move |_, _| ret(count));
        e.register(TARGET_OWN_CHUNK_ID, |_, _| ret(OWN_CHUNK));
        e.register(TARGET_OWN_BLOCK, |_, _| ret(OWN_BLOCK));
        e.register(TARGET_OWN_BLOCK_SIZE, |_, _| ret(OWN_BLOCK_SIZE));
        let mut object_table = vec![0u32; 0x61];
        object_table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        object_table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        object_table[0x180 / 4] = TARGET_AFTER_LOAD;
        e.put_vtable(OBJECT_TABLE, &object_table);
        let mut component_table = vec![0u32; 0x11];
        component_table[0x24 / 4] = TARGET_OWN_CHUNK_ID;
        component_table[0x28 / 4] = TARGET_OWN_BLOCK;
        component_table[0x2c / 4] = TARGET_OWN_BLOCK_SIZE;
        component_table[0x40 / 4] = TARGET_OFFER_CHUNK;
        e.put_vtable(COMPONENT_TABLE, &component_table);
        let object = e.mem.alloc(0x60);
        e.mem.set_u32(object, OBJECT_TABLE);
        e.mem.set_u32(object + 0x30, COMPONENT_TABLE);
        e.call_log = Some(vec![]);
        let result = e.call(0x0040c650, &args![object, FILE]).bool();
        let log = e.call_log.take().unwrap();
        (log.into_iter().skip(1).collect(), object, result)
    }

    #[test]
    fn every_chunk_kind_goes_to_its_reader() {
        let chunks = vec![
            CHUNK_EDID,
            CHUNK_FULL,
            CHUNK_OBND,
            CHUNK_EFID,
            CHUNK_CTDA,
            0x5555_5555,
        ];
        let (log, object, result) = run(chunks, 1, 0);
        assert!(result);
        let component = object + 0x30;
        assert_eq!(log[0], (TES_FORM_LOAD_FORM, vec![object, FILE]));
        let data = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(data.len(), 1);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls_to(&log, TARGET_SET_EDITOR_ID),
            vec![vec![object, data[0][1]]]
        );
        assert_eq!(calls_to(&log, LOAD_FULL_NAME), vec![vec![component, FILE]]);
        assert_eq!(calls_to(&log, TARGET_LOAD_BOUNDS), vec![vec![object, FILE]]);
        assert_eq!(calls_to(&log, FULL_NAME_TEXT).len(), 2);
        assert_eq!(
            calls_to(&log, EFFECT_ITEM_LIST_LOAD),
            vec![
                vec![object + 0x3c, FILE, 0x5000],
                vec![object + 0x3c, FILE, 0x5000]
            ]
        );
        // Every chunk, known or not, is offered to the component.
        let offered: Vec<u32> = calls_to(&log, TARGET_OFFER_CHUNK)
            .iter()
            .map(|c| {
                assert_eq!(c[..2], [component, FILE]);
                c[2]
            })
            .collect();
        assert_eq!(
            offered,
            vec![
                CHUNK_EDID,
                CHUNK_FULL,
                CHUNK_OBND,
                CHUNK_EFID,
                CHUNK_CTDA,
                0x5555_5555
            ]
        );
        assert!(calls_to(&log, MEMSET).is_empty());
        assert!(calls_to(&log, PRINT).is_empty());
    }

    #[test]
    fn the_components_own_chunk_is_zeroed_and_read_into_its_block() {
        let (log, object, _) = run(vec![0x5555_5555, OWN_CHUNK], 1, 1);
        assert_eq!(
            calls_to(&log, MEMSET),
            vec![vec![OWN_BLOCK, 0, OWN_BLOCK_SIZE]]
        );
        assert_eq!(
            calls_to(&log, GET_CHUNK_DATA),
            vec![vec![FILE, OWN_BLOCK, OWN_BLOCK_SIZE]]
        );
        assert_eq!(calls_to(&log, FILE_FLAG_299), vec![vec![FILE]]);
        assert_eq!(calls_to(&log, TARGET_AFTER_LOAD), vec![vec![object]]);
    }

    #[test]
    fn the_after_load_slot_needs_the_file_flag() {
        let (log, _, _) = run(vec![OWN_CHUNK], 1, 0);
        assert_eq!(calls_to(&log, FILE_FLAG_299).len(), 1);
        assert!(calls_to(&log, TARGET_AFTER_LOAD).is_empty());
    }

    #[test]
    fn a_record_without_chunks_reports_that_it_has_no_effects() {
        let (log, object, result) = run(vec![], 0, 0);
        assert!(result);
        assert!(calls_to(&log, TARGET_OFFER_CHUNK).is_empty());
        assert_eq!(calls_to(&log, LIST_COUNT), vec![vec![object + 0x40]]);
        assert_eq!(
            calls_to(&log, PRINT),
            vec![vec![MESSAGE_NO_EFFECTS, 0x77, 0x5000]]
        );
    }
}

#[cfg(test)]
mod b060_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    const FILE: u32 = 0x6000;
    const OBJECT_TABLE: u32 = 0x0700_0000;
    const COMPONENT_TABLE: u32 = 0x0710_0000;
    const OWN_CHUNK: u32 = 0x1234;
    const TARGET_LOAD_BOUNDS: u32 = 0x0050_0001;
    const TARGET_SET_EDITOR_ID: u32 = 0x0050_0002;
    const TARGET_AFTER_LOAD: u32 = 0x0050_0003;
    const TARGET_OFFER_CHUNK: u32 = 0x0050_0004;
    const TARGET_OWN_CHUNK_ID: u32 = 0x0050_0005;
    const TARGET_OWN_BLOCK: u32 = 0x0050_0006;
    const TARGET_OWN_BLOCK_SIZE: u32 = 0x0050_0007;

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
    fn fn_0040c0e0_constructs_and_sets_the_text() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCT_00403970, |_, _| Ret::default());
        e.register(STRING_SET_004037F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0040_c0e0, &args![0x5000u32, 0x7000u32]).u32(),
            0x5000
        );
        assert_eq!(calls(&e, STRING_CONSTRUCT_00403970), vec![vec![0x5000]]);
        assert_eq!(
            calls(&e, STRING_SET_004037F0),
            vec![vec![0x5000, 0x7000, 0]]
        );
    }

    #[test]
    fn fn_0040c110_waits_on_the_held_object_only() {
        let mut e = Engine::new();
        e.register(WAIT_004019A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let holder = e.mem.alloc(4);
        e.call(0x0040_c110, &args![holder]);
        assert!(calls(&e, WAIT_004019A0).is_empty());
        e.mem.set_u32(holder, 0x8000);
        e.call(0x0040_c110, &args![holder]);
        e.call(0x0040_c130, &args![0x9000u32]);
        assert_eq!(calls(&e, WAIT_004019A0), vec![vec![0x8004], vec![0x9004]]);
    }

    #[test]
    fn fn_0040c5b0_inits_unless_the_query_answers() {
        let mut e = Engine::new();
        e.map(0x0f00_0000, 0x1000);
        e.register(QUERY_004013E0, |e, _| e.mem.u32(0x0f00_0000).into_ret());
        e.register(MAGIC_ITEM_INIT_ITEM, |_, _| Ret::default());
        e.register(FORM_SET_00484AB0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.mem.set_u32(0x0f00_0000, 0x100);
        e.call(0x0040_c5b0, &args![0x5000u32]);
        assert_eq!(calls(&e, MAGIC_ITEM_INIT_ITEM), vec![vec![0x5018, 0x5000]]);
        assert_eq!(calls(&e, FORM_SET_00484AB0), vec![vec![0x5000, 1]]);
        e.mem.set_u32(0x0f00_0000, 1);
        e.call(0x0040_c5b0, &args![0x5000u32]);
        assert_eq!(calls(&e, MAGIC_ITEM_INIT_ITEM).len(), 1);
    }

    /// Runs `0040c230` on a record of `chunks`; `count` is what the list count answers,
    /// `flag` the file flag at `+0x299`. Returns the engine and the object.
    fn run(chunks: Vec<u32>, count: u32, flag: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let (list, at) = (chunks.clone(), position.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            list.get(at.get()).copied().unwrap_or(0).into_ret()
        });
        let total = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            u32::from(position.get() < total).into_ret()
        });
        for addr in [
            TES_FORM_LOAD_FORM,
            GET_CHUNK_DATA,
            MEMSET,
            LOAD_FULL_NAME,
            EFFECT_ITEM_LIST_LOAD,
            PRINT,
            TARGET_LOAD_BOUNDS,
            TARGET_SET_EDITOR_ID,
            TARGET_AFTER_LOAD,
            TARGET_OFFER_CHUNK,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(CHUNK_SIZE, |_, _| 5u32.into_ret());
        e.register_double(FILE_FLAG_299, move |_, _| flag.into_ret());
        e.register(FULL_NAME_TEXT, |_, _| 0x5000u32.into_ret());
        e.register(FORM_ID, |_, _| 0x77u32.into_ret());
        e.register_double(LIST_COUNT, move |_, _| count.into_ret());
        e.register(TARGET_OWN_CHUNK_ID, |_, _| OWN_CHUNK.into_ret());
        e.register(TARGET_OWN_BLOCK, |_, _| 0x9000u32.into_ret());
        e.register(TARGET_OWN_BLOCK_SIZE, |_, _| 20u32.into_ret());
        let mut object_table = vec![0u32; 0x61];
        object_table[0xe0 / 4] = TARGET_LOAD_BOUNDS;
        object_table[0x134 / 4] = TARGET_SET_EDITOR_ID;
        object_table[0x138 / 4] = TARGET_AFTER_LOAD;
        e.put_vtable(OBJECT_TABLE, &object_table);
        let mut component_table = vec![0u32; 0x11];
        component_table[0x24 / 4] = TARGET_OWN_CHUNK_ID;
        component_table[0x28 / 4] = TARGET_OWN_BLOCK;
        component_table[0x2c / 4] = TARGET_OWN_BLOCK_SIZE;
        component_table[0x40 / 4] = TARGET_OFFER_CHUNK;
        e.put_vtable(COMPONENT_TABLE, &component_table);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, OBJECT_TABLE);
        e.mem.set_u32(object + 0x18, COMPONENT_TABLE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0040_c230, &args![object, FILE]).bool());
        (e, object)
    }

    #[test]
    fn fn_0040c230_reads_each_chunk_kind() {
        let chunks = vec![
            CHUNK_EDID, CHUNK_FULL, CHUNK_OBND, CHUNK_EFID, CHUNK_CTDA, 7,
        ];
        let (e, object) = run(chunks.clone(), 1, 0);
        let component = object + 0x18;
        assert_eq!(calls(&e, TES_FORM_LOAD_FORM), vec![vec![object, FILE]]);
        let data = calls(&e, GET_CHUNK_DATA);
        assert_eq!((data[0][0], data[0][2]), (FILE, 0x200));
        assert_eq!(
            calls(&e, TARGET_SET_EDITOR_ID),
            vec![vec![object, data[0][1]]]
        );
        assert_eq!(calls(&e, LOAD_FULL_NAME), vec![vec![component, FILE]]);
        assert_eq!(calls(&e, TARGET_LOAD_BOUNDS), vec![vec![object, FILE]]);
        assert_eq!(
            calls(&e, EFFECT_ITEM_LIST_LOAD),
            vec![vec![object + 0x24, FILE, 0x5000]; 2]
        );
        let offered: Vec<u32> = calls(&e, TARGET_OFFER_CHUNK).iter().map(|c| c[2]).collect();
        assert_eq!(offered, chunks);
        assert!(calls(&e, PRINT).is_empty());
    }

    #[test]
    fn fn_0040c230_reads_the_components_own_chunk() {
        let (e, object) = run(vec![OWN_CHUNK], 1, 1);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x9000, 0, 20]]);
        assert_eq!(calls(&e, GET_CHUNK_DATA), vec![vec![FILE, 0x9000, 20]]);
        assert_eq!(calls(&e, TARGET_AFTER_LOAD), vec![vec![object]]);
        let (e, _) = run(vec![OWN_CHUNK], 1, 0);
        assert!(calls(&e, TARGET_AFTER_LOAD).is_empty());
    }

    #[test]
    fn fn_0040c230_reports_a_record_without_effects() {
        let (e, object) = run(vec![], 0, 0);
        assert_eq!(calls(&e, LIST_COUNT), vec![vec![object + 0x28]]);
        assert_eq!(
            calls(&e, PRINT),
            vec![vec![MESSAGE_NO_EFFECTS, 0x77, 0x5000]]
        );
    }
}

#[cfg(test)]
mod magic_item_form_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const SLOT_FUNCTION: u32 = 0x00f1_0000;

    /// A form whose `MagicItem` part (at `at`) has a vtable whose slots
    /// +0x30 .. +0x3c are doubles at `SLOT_FUNCTION + slot`; slot +0x30
    /// answers the byte at form +0x80.
    fn form(e: &mut Engine, at: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x40);
        for slot in [0x30u32, 0x34, 0x38, 0x3c] {
            e.mem.set_u32(vtable + slot, SLOT_FUNCTION + slot);
        }
        e.mem.set_u32(form + at, vtable);
        form
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SLOT_FUNCTION + 0x30, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x50) != 0).into_ret()
        });
        for slot in [0x34u32, 0x38, 0x3c] {
            e.register(SLOT_FUNCTION + slot, |_, _| Ret::default());
        }
        for addr in [
            TES_FORM_START_FORM,
            TES_FORM_CLOSE_FORM,
            TES_FULL_NAME_SAVE,
            EFFECT_LIST_SAVE_004056D0,
            TES_FORM_COPY_ALL_COMPONENTS,
            EFFECT_LIST_COPY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9000 {
                Ret::default()
            } else {
                a[0].into_ret()
            }
        });
        e.register(TES_FORM_COMPARE_ALL_COMPONENTS, |e, a| {
            u32::from(e.mem.u8(a[0] + 0xf0) != 0).into_ret()
        });
        e.register(EFFECT_LIST_DIFFERS, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x4) != 0).into_ret()
        });
        e
    }

    #[test]
    fn saves_in_order() {
        for (addr, at, list) in [
            (0x0040_c1d0u32, 0x18u32, 0x24u32),
            (0x0040_c5f0, 0x30, 0x3c),
        ] {
            let mut e = engine();
            let this = form(&mut e, at);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this]);
            let log: Vec<(u32, Vec<u32>)> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != addr)
                .collect();
            assert_eq!(
                log,
                vec![
                    (TES_FORM_START_FORM, vec![this]),
                    (TES_FULL_NAME_SAVE, vec![this + at]),
                    (SLOT_FUNCTION + 0x38, vec![this + at]),
                    (SLOT_FUNCTION + 0x3c, vec![this + at]),
                    (EFFECT_LIST_SAVE_004056D0, vec![this + list]),
                    (TES_FORM_CLOSE_FORM, vec![this]),
                ]
            );
        }
    }

    #[test]
    fn compares_by_class_components_item_and_list() {
        for (addr, at, list, class) in [
            (0x0040_c450u32, 0x18u32, 0x24u32, TYPE_MAGIC_ITEM_FORM),
            (0x0040_c870, 0x30, 0x3c, TYPE_MAGIC_ITEM_OBJECT),
        ] {
            let mut e = engine();
            let this = form(&mut e, at);
            let other = form(&mut e, at);
            e.call_log = Some(vec![]);
            assert!(!e.call(addr, &args![this, other]).bool());
            assert_eq!(
                calls(&e, DYNAMIC_CAST),
                vec![vec![other, 0, TYPE_TES_FORM, class, 0]]
            );
            assert_eq!(
                calls(&e, SLOT_FUNCTION + 0x30),
                vec![vec![this + at, other + at]]
            );
            assert_eq!(
                calls(&e, EFFECT_LIST_DIFFERS),
                vec![vec![this + list, other + list]]
            );
            assert!(e.call(addr, &args![this, 0x9000u32]).bool());
            e.mem.set_u8(this + list + 4, 1);
            assert!(e.call(addr, &args![this, other]).bool());
            e.mem.set_u8(this + list + 4, 0);
            e.mem.set_u8(this + at + 0x50, 1);
            assert!(e.call(addr, &args![this, other]).bool());
            e.mem.set_u8(this + at + 0x50, 0);
            e.mem.set_u8(this + 0xf0, 1);
            e.call_log = Some(vec![]);
            assert!(e.call(addr, &args![this, other]).bool());
            assert!(calls(&e, SLOT_FUNCTION + 0x30).is_empty());
        }
    }

    #[test]
    fn copies_components_item_and_list() {
        for (addr, at, list) in [
            (0x0040_c510u32, 0x18u32, 0x24u32),
            (0x0040_c930, 0x30, 0x3c),
        ] {
            let mut e = engine();
            let this = form(&mut e, at);
            let other = form(&mut e, at);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, other]);
            let log: Vec<(u32, Vec<u32>)> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a != addr && *a != DYNAMIC_CAST)
                .collect();
            assert_eq!(
                log,
                vec![
                    (TES_FORM_COPY_ALL_COMPONENTS, vec![this, other]),
                    (SLOT_FUNCTION + 0x34, vec![this + at, other + at]),
                    (EFFECT_LIST_COPY, vec![this + list, other + list]),
                ]
            );
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 0x9000u32]);
            assert!(calls(&e, TES_FORM_COPY_ALL_COMPONENTS).is_empty());
        }
    }
}
