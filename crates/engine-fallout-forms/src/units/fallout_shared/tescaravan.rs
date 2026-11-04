//! `fallout shared/tescaravan.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0059ae60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059ae60 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x74 (0xcc bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_49b4`) is `TESCaravanMoney`, so this is the constructor of `TESCaravanMoney`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0059ae60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x78]);
    e.call(0x0047_b4f0, &args![base + 0x80]);
    e.mem.set_u32(base, 0x0103_49b4);
    e.mem.set_u32(base + 0x30, 0x0103_499c);
    e.mem.set_u32(base + 0x3c, 0x0103_4978);
    e.mem.set_u32(base + 0x5c, 0x0103_4958);
    e.mem.set_u32(base + 0x68, 0x0103_4944);
    e.mem.set_u32(base + 0x78, 0x0103_4924);
    e.mem.set_u32(base + 0x80, 0x0103_4910);
    e.call(
        0x00ec_782f,
        &args![base + 0x8c, 0x20u32, 0x2u32, 0x0048_a3d0u32, 0x0048_a490u32],
    );
    e.call(0x004f_15a0, &args![this, 0x74u32]);
    e.call(0x0059_b120, &args![this]);
    this
}

/// `00484d60` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_00484D60: u32 = 0x0048_4d60;

// Translated from 0059add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 010347e0 and 010368a8): forwards `this` and one
/// pointer word to `00484d60`, which, after a check on the word through `00428110` and
/// `004280f0`, calls `00865e50` on the word with `this+8` and 4. Returns nothing: no path of
/// `00484d60` produces a value (it ends after that call).
pub fn fn_0059add0(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_00484D60, &args![this, argument]);
}

/// `00484da0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_00484DA0: u32 = 0x0048_4da0;

// Translated from 0059adf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 010347e8 and 010368b0): forwards `this` and one
/// pointer word to `00484da0`, which reads a word through `00864980` and merges it into the
/// flags word at `this+8`. Returns nothing: every path of `00484da0` ends after that store
/// with no value.
pub fn fn_0059adf0(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_00484DA0, &args![this, argument]);
}

/// `TESFile::GetTESForm` (`00472660`): the current record's form type, in `AL`.
pub(crate) const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (`004726b0`): the current chunk's type, 0 at the end.
pub(crate) const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::NextChunk` (`004726f0`): false when the record has no more chunks.
pub(crate) const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (`004727f0`, `thiscall`, one word: the destination).
pub(crate) const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData` with a size (`00472890`, `thiscall`: destination, size).
pub(crate) const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// The current chunk's size (`00401660`, `thiscall` on the file).
pub(crate) const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESForm::LoadForm` (`00485110`, `thiscall`, one word: the file).
pub(crate) const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`thiscall`, one word): called with 0 before the chunks are read and with 1
/// by `InitItem`.
pub(crate) const SET_FLAG_00484AB0: u32 = 0x0048_4ab0;
/// `004013e0` (`thiscall`, no stack word): the check every `InitItem` here starts with;
/// nonzero `AL` skips the rest.
const INIT_ITEM_CHECK: u32 = 0x0040_13e0;

/// Chunk types (the four characters, little-endian).
pub(crate) const CHUNK_EDID: u32 = 0x4449_4445;
pub(crate) const CHUNK_OBND: u32 = 0x444e_424f;
pub(crate) const CHUNK_FULL: u32 = 0x4c4c_5546;
pub(crate) const CHUNK_MODL: u32 = 0x4c44_4f4d;
pub(crate) const CHUNK_MODT: u32 = 0x5444_4f4d;
pub(crate) const CHUNK_MODD: u32 = 0x4444_4f4d;
pub(crate) const CHUNK_MOSD: u32 = 0x4453_4f4d;
pub(crate) const CHUNK_MODS: u32 = 0x5344_4f4d;
pub(crate) const CHUNK_ICON: u32 = 0x4e4f_4349;
pub(crate) const CHUNK_MICO: u32 = 0x4f43_494d;
pub(crate) const CHUNK_SCRI: u32 = 0x4952_4353;
pub(crate) const CHUNK_YNAM: u32 = 0x4d41_4e59;
pub(crate) const CHUNK_ZNAM: u32 = 0x4d41_4e5a;
pub(crate) const CHUNK_DATA: u32 = 0x4154_4144;
pub(crate) const CHUNK_TX00: u32 = 0x3030_5854;
pub(crate) const CHUNK_TX01: u32 = 0x3130_5854;
pub(crate) const CHUNK_INTV: u32 = 0x5654_4e49;
pub(crate) const CHUNK_CARD: u32 = 0x4452_4143;

/// `TESCaravanMoney`'s vtables, stored at the start of its destructor (the words its
/// constructor `0059ae60` stores), by field offset.
const MONEY_VTABLES: [(u32, u32); 7] = [
    (0x00, 0x0103_49b4),
    (0x30, 0x0103_499c),
    (0x3c, 0x0103_4978),
    (0x5c, 0x0103_4958),
    (0x68, 0x0103_4944),
    (0x78, 0x0103_4924),
    (0x80, 0x0103_4910),
];

/// `p + offset`, or 0 when `p` is null (the compiler's null-checked base-class cast).
pub(crate) fn cast_or_null(p: Ptr, offset: u32) -> u32 {
    if p.is_null() {
        0
    } else {
        p.addr() + offset
    }
}

// Translated from 0059a6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCaravanCard::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, runs the components' `InitItem`s with the form (`0048aa80` on the
/// `TESModelTextureSwap` at +0x3c, `0048cd90` on the `TESScriptableForm` at +0x80,
/// `0047b6a0` on the `BGSPickupPutdownSounds` at +0x8c), then `00484ab0` with 1.
pub fn tes_caravan_card_init_item(e: &mut Engine, this: Ptr) {
    if e.call(INIT_ITEM_CHECK, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_aa80, &args![base + 0x3c, this]);
    e.call(0x0048_cd90, &args![base + 0x80, this]);
    e.call(0x0047_b6a0, &args![base + 0x8c, this]);
    e.call(SET_FLAG_00484AB0, &args![this, 1u32]);
}

// Translated from 0059b160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCaravanMoney::InitItem` (Xbox PDB; `ECX` only, plain `RET`): unless `004013e0` says
/// otherwise, runs `0048aa80` on the `TESModelTextureSwap` at +0x3c and `0047b6a0` on the
/// `BGSPickupPutdownSounds` at +0x80 with the form, then `00484ab0` with 1.
pub fn tes_caravan_money_init_item(e: &mut Engine, this: Ptr) {
    if e.call(INIT_ITEM_CHECK, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0048_aa80, &args![base + 0x3c, this]);
    e.call(0x0047_b6a0, &args![base + 0x80, this]);
    e.call(SET_FLAG_00484AB0, &args![this, 1u32]);
}

// Translated from 0059b0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): calls vtable slot +4 (no arguments)
/// of the sub-objects at +0x8c and +0xac. The `TESCaravanMoney` destructor `0059afc0` runs
/// it first.
pub fn fn_0059b0e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base + 0x8c, 4, &args![]);
    e.vcall(base + 0xac, 4, &args![]);
}

// Translated from 0059b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): calls vtable slot +0 (no arguments)
/// of the sub-objects at +0x8c and +0xac. The `TESCaravanMoney` constructor `0059ae60` runs
/// it last.
pub fn fn_0059b120(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base + 0x8c, 0, &args![]);
    e.vcall(base + 0xac, 0, &args![]);
}

// Translated from 0059afc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059afc0 - the exception-unwinding frame (FS:[0] chain, handler 00ef89d5, state variable) is not translated.
/// Destructor of `TESCaravanMoney` (the class whose vtable `0x0103_49b4` the constructor
/// `0059ae60` stores; unnamed in the engine map; `ECX` only, plain `RET`): stores the
/// vtables again, runs `0059b0e0`, destroys the array of two 0x20-byte elements at +0x8c
/// through `00ec5fce` (the array destructor iterator, `stdcall`, with the element
/// destructor `0048a490`), then the member and base destructors: `0047b520` on +0x80,
/// `0048e7d0` on +0x78, `0047a1a0` on +0x68, `00402c40` on +0x5c, `0048a490` on +0x3c,
/// `00591300` on +0x30 and `0050e9f0` on the object.
pub fn fn_0059afc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in MONEY_VTABLES {
        e.mem.set_u32(base + offset, vtable);
    }
    fn_0059b0e0(e, this);
    e.call(
        0x00ec_5fce,
        &args![base + 0x8c, 0x20u32, 2u32, 0x0048_a490u32],
    );
    e.call(0x0047_b520, &args![base + 0x80]);
    e.call(0x0048_e7d0, &args![base + 0x78]);
    e.call(0x0047_a1a0, &args![base + 0x68]);
    e.call(0x0040_2c40, &args![base + 0x5c]);
    e.call(0x0048_a490, &args![base + 0x3c]);
    e.call(0x0059_1300, &args![base + 0x30]);
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 0059af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `TESCaravanMoney` (unnamed in the engine map; `ECX` and
/// one stack word, `RET 4`): runs the destructor `0059afc0`, frees the object through
/// `00401030` (cdecl) when bit 0 of `flags` is set, and returns the object.
pub fn fn_0059af90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0059afc0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The `EDID` chunk of the loaders here: a stack buffer of the chunk's size (the
/// `__alloca_probe_16` block), read with at most 0x200 bytes and given to vtable slot
/// +0x134.
pub(crate) fn load_editor_id(e: &mut Engine, this: Ptr, file: Ptr) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    e.with_stack(size, |e, name| {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
        e.vcall(this.addr(), 0x134, &args![name]);
    });
}

/// A word chunk read into a zeroed stack word, which is returned.
pub(crate) fn chunk_word(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_DATA, &args![file, word]);
        e.mem.u32(word.addr())
    })
}

// Translated from 0059a800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059a800 - the /GS cookie check (00ec408c) is not translated; the EDID and MODS buffers from __alloca_probe_16 (00ec5ec0) are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// The `TESCaravanCard` `Load` (form type 0x73, the type its constructor `0059a370` gives;
/// unnamed in the engine map; `ECX` and one stack word, the file, `RET 4`): false unless
/// `TESFile::GetTESForm`'s `AL` is 0x73. Otherwise `TESForm::LoadForm`, `00484ab0` with 0,
/// then each chunk:
/// - `EDID`: the editor ID (see `load_editor_id`); `OBND`: vtable slot +0xe0 with the file;
/// - `FULL`: `00487050` (cdecl) on the `TESFullName` at +0x30; `MODL`, `MODT`, `MODD`,
///   `MOSD`: `004892d0` (cdecl) on the model at +0x3c; `MODS`: the chunk into a stack buffer
///   of its size, then `0048a7b0` on +0x3c with the buffer, the size and `00403570`'s
///   16-bit count; `ICON`: `0048e3d0` (cdecl) on the texture at +0x5c; `MICO`: `0047a290`
///   (cdecl) on +0x68;
/// - `SCRI`: a word given to `006ecd40` on +0x80, then `0048cd90` on +0x80 with the form;
///   `YNAM`: a word given to `006ecd40` on +0x8c; `ZNAM`: a word given to `00403550` on
///   +0x8c;
/// - `DATA`: `004861f0` (`TESForm::LoadData`) with the file, 0, 0; `TX00` and `TX01`:
///   `0048e3d0` on the textures at +0x9c and +0xa8;
/// - `INTV`: the first one into +0xb8, every later one into +0xb4.
///
/// The casts to +0x30, +0x3c, +0x5c and +0x68 are null-checked (0 for a null form) as the
/// code does. Then true.
pub fn fn_0059a800(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x73 {
        return false;
    }
    let base = this.addr();
    let mut first_intv_read = false;
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FLAG_00484AB0, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = cast_or_null(this, 0x3c);
                e.call(0x0048_92d0, &args![model, file]);
            }
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, 0u32, 0u32]);
            }
            CHUNK_TX00 => {
                e.call(0x0048_e3d0, &args![base + 0x9c, file]);
            }
            CHUNK_TX01 => {
                e.call(0x0048_e3d0, &args![base + 0xa8, file]);
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_SCRI => {
                let script = chunk_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x80, script]);
                e.call(0x0048_cd90, &args![base + 0x80, this]);
            }
            CHUNK_MICO => {
                let icon = cast_or_null(this, 0x68);
                e.call(0x0047_a290, &args![icon, file]);
            }
            CHUNK_ZNAM => {
                let word = chunk_word(e, file);
                e.call(0x0040_3550, &args![base + 0x8c, word]);
            }
            CHUNK_FULL => {
                let name = cast_or_null(this, 0x30);
                e.call(0x0048_7050, &args![name, file]);
            }
            CHUNK_YNAM => {
                let word = chunk_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x8c, word]);
            }
            CHUNK_ICON => {
                let texture = cast_or_null(this, 0x5c);
                e.call(0x0048_e3d0, &args![texture, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    let count = u32::from(e.call(0x0040_3570, &args![file]).u16());
                    e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, count]);
                });
            }
            CHUNK_INTV => {
                if first_intv_read {
                    e.call(GET_CHUNK_DATA, &args![file, base + 0xb4]);
                } else {
                    e.call(GET_CHUNK_DATA, &args![file, base + 0xb8]);
                    first_intv_read = true;
                }
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0059bb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059bb50 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `TESCaravanDeck::Load` (Xbox PDB; `ECX` and one stack word, the file, `RET 4`): false
/// unless `TESFile::GetTESForm`'s `AL` is 0x75. Otherwise `TESForm::LoadForm`, `00484ab0`
/// with 0, then each chunk:
/// - `EDID`: the editor ID (see `load_editor_id`); `OBND`: vtable slot +0xe0 with the file;
/// - `FULL`: `00487050` (cdecl) on the `TESFullName` at +0x18 (null-checked cast);
/// - `DATA`: `004861f0` (`TESForm::LoadData`) with the file, `this+0x28` and 4;
/// - `CARD`: a word read into a stack slot the code does not clear first, resolved in place
///   by `00485d50` (cdecl, the slot and the file); `004839c0` (cdecl) on the resolved word
///   gives the card. With no list at +0x24, or when `008256d0` on the list says so, the card
///   goes to `00726c60` on the list (the list pointer as `this` even when it is null);
///   otherwise to `005ae3d0` on the list. Both take the card's stack slot.
///
/// After each chunk, a `DATA` chunk is followed by `00503210` on +0x28 when `00401680` on
/// the file says so. Then true.
pub fn tes_caravan_deck_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x75 {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FLAG_00484AB0, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x28, 4u32]);
            }
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_CARD => {
                e.with_stack(8, |e, slots| {
                    let read = slots.addr();
                    let card_slot = read + 4;
                    e.call(GET_CHUNK_DATA, &args![file, read]);
                    e.call(0x0048_5d50, &args![read, file]);
                    let resolved = e.mem.u32(read);
                    let card = e.call(0x0048_39c0, &args![resolved]).u32();
                    e.mem.set_u32(card_slot, card);
                    let list = e.mem.u32(base + 0x24);
                    if list == 0 || e.call(0x0082_56d0, &args![list]).bool() {
                        e.call(0x0072_6c60, &args![list, card_slot]);
                    } else {
                        e.call(0x005a_e3d0, &args![list, card_slot]);
                    }
                });
            }
            CHUNK_FULL => {
                let name = cast_or_null(this, 0x18);
                e.call(0x0048_7050, &args![name, file]);
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(0x0040_1680, &args![file]).bool() {
            e.call(0x0050_3210, &args![base + 0x28]);
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// The savers the caravan forms call: `TESForm::StartForm` (`004855a0`),
/// `TESForm::SaveData` (`00485e00`), `TESForm::CloseForm` (`00485680`),
/// `TESFullName::Save` (`00487010`), `TESModelTextureSwap::Save`
/// (`0048a520`, three chunk words), `TESTexture::Save` (`0048e370`, one
/// chunk word), `BGSMessageIcon::Save` (`0047a240`, one chunk word),
/// `TESScriptableForm::Save` (`0048cca0`), `BGSPickupPutdownSounds::Save`
/// (`0047b560`) and `TESForm::AddChunk` with a word (`00485910`, `cdecl`).
const FORM_START: u32 = 0x0048_55a0;
const FORM_SAVE_DATA: u32 = 0x0048_5e00;
const FORM_CLOSE: u32 = 0x0048_5680;
const SAVE_FULL_NAME: u32 = 0x0048_7010;
const SAVE_MODEL_TEXTURE_SWAP: u32 = 0x0048_a520;
const SAVE_TEXTURE: u32 = 0x0048_e370;
const SAVE_MESSAGE_ICON: u32 = 0x0047_a240;
const SAVE_SCRIPTABLE: u32 = 0x0048_cca0;
const SAVE_PICKUP_PUTDOWN_SOUNDS: u32 = 0x0047_b560;
const ADD_CHUNK_WORD: u32 = 0x0048_5910;
/// The card's two textures (`0050d100` answers the first, `00503650` the
/// second; `thiscall`, no words), the card's value pair (`006ab360`: the
/// address of two words), and the money's model getter (`0059b270`,
/// `thiscall`, one word: the index).
const CARD_TEXTURE_FIRST: u32 = 0x0050_d100;
const CARD_TEXTURE_SECOND: u32 = 0x0050_3650;
const CARD_VALUE: u32 = 0x006a_b360;
const MONEY_MODEL: u32 = 0x0059_b270;
/// The chunk codes the saves write.
const CHUNK_MOD2: u32 = 0x3244_4f4d;
const CHUNK_MOD3: u32 = 0x3344_4f4d;
/// `__RTDynamicCast` (`00ec43fb`) from `TESForm` (`01183028`) to
/// `TESCaravanCard` (`0118c5c4`), the type descriptors' names read at +8;
/// `TESForm::CopyAllComponents` (`004851b0`), `TESForm::CompareAllComponents`
/// (`00485270`), a texture's path text (`00408da0`), the path setter
/// (`00489100`, `thiscall`, one word), the form's type (`00401170`), the
/// type setter `004f15a0` (one word), the text comparison `00408b20`
/// (`cdecl`) and `memcmp` (`00ec4835`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_CARAVAN_CARD: u32 = 0x0118_c5c4;
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const TEXTURE_PATH_TEXT: u32 = 0x0040_8da0;
const TEXTURE_SET_PATH: u32 = 0x0048_9100;
const FORM_TYPE: u32 = 0x0040_1170;
const FORM_SET_TYPE: u32 = 0x004f_15a0;
const TEXT_COMPARE: u32 = 0x0040_8b20;
const MEMCMP: u32 = 0x00ec_4835;
/// `00484c20` (`thiscall`, one word): the base body `0059adb0` forwards to.
const FORWARD_TARGET_00484C20: u32 = 0x0048_4c20;

// Translated from 0059a720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCaravanCard::Save` (Xbox PDB): starts the form, saves the full name
/// (+0x30), the model (+0x3C: `MODL`, `MODT`, `MODS`), the icon (+0x5C,
/// `ICON`), the message icon (+0x68, `MICO`), the script (+0x80), the
/// pick-up/put-down sounds (+0x8C), the two card textures (`TX00`,
/// `TX01`), the words at +0xB8 and +0xB4 (both `INTV`, in that order) and
/// the form data, and closes the form.
pub fn tes_caravan_card_save(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![at + 0x30]);
    e.call(
        SAVE_MODEL_TEXTURE_SWAP,
        &args![at + 0x3c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    e.call(SAVE_TEXTURE, &args![at + 0x5c, CHUNK_ICON]);
    e.call(SAVE_MESSAGE_ICON, &args![at + 0x68, CHUNK_MICO]);
    e.call(SAVE_SCRIPTABLE, &args![at + 0x80]);
    e.call(SAVE_PICKUP_PUTDOWN_SOUNDS, &args![at + 0x8c]);
    let texture = e.call(CARD_TEXTURE_FIRST, &args![this]).u32();
    e.call(SAVE_TEXTURE, &args![texture, CHUNK_TX00]);
    let texture = e.call(CARD_TEXTURE_SECOND, &args![this]).u32();
    e.call(SAVE_TEXTURE, &args![texture, CHUNK_TX01]);
    let value = e.mem.u32(at + 0xb8);
    e.call(ADD_CHUNK_WORD, &args![CHUNK_INTV, value]);
    let value = e.mem.u32(at + 0xb4);
    e.call(ADD_CHUNK_WORD, &args![CHUNK_INTV, value]);
    e.call(FORM_SAVE_DATA, &args![this]);
    e.call(FORM_CLOSE, &args![this]);
}

/// `other` cast from `TESForm` to `TESCaravanCard`.
fn cast_caravan_card(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_FORM, TYPE_CARAVAN_CARD, 0u32],
    )
    .u32()
}

// Translated from 0059ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the copy of a
/// `TESCaravanCard`): when `other` is one, copies all components, the two
/// card texture paths (into the textures at +0x9C and +0xA8), its value
/// pair (`006ab360`, into +0xB4 and +0xB8) and its form type (`00401170`
/// of `other` as given into `004f15a0`).
pub fn fn_0059ac20(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = cast_caravan_card(e, other);
    if source == 0 {
        return;
    }
    let at = this.addr();
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    let texture = e.call(CARD_TEXTURE_FIRST, &args![source]).u32();
    let path = e.call(TEXTURE_PATH_TEXT, &args![texture]).u32();
    e.call(TEXTURE_SET_PATH, &args![at + 0x9c, path]);
    let texture = e.call(CARD_TEXTURE_SECOND, &args![source]).u32();
    let path = e.call(TEXTURE_PATH_TEXT, &args![texture]).u32();
    e.call(TEXTURE_SET_PATH, &args![at + 0xa8, path]);
    let pair = e.call(CARD_VALUE, &args![source]).u32();
    let first = e.mem.u32(pair);
    let second = e.mem.u32(pair + 4);
    e.mem.set_u32(at + 0xb4, first);
    e.mem.set_u32(at + 0xb8, second);
    let form_type = e.call(FORM_TYPE, &args![other]).u32();
    e.call(FORM_SET_TYPE, &args![this, form_type]);
}

// Translated from 0059acd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the comparison of a
/// `TESCaravanCard`): true (different) when `other` is none, when
/// `CompareAllComponents` says so, when either texture path differs
/// (`00408b20` of this card's path and `other`'s), or when the eight bytes
/// at +0xB4 differ from `other`'s value pair (`memcmp`).
pub fn fn_0059acd0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = cast_caravan_card(e, other);
    if source == 0 {
        return true;
    }
    let at = this.addr();
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, source])
        .u8()
        != 0
    {
        return true;
    }
    for (getter, offset) in [(CARD_TEXTURE_FIRST, 0x9c), (CARD_TEXTURE_SECOND, 0xa8)] {
        let texture = e.call(getter, &args![source]).u32();
        let theirs = e.call(TEXTURE_PATH_TEXT, &args![texture]).u32();
        let mine = e.call(TEXTURE_PATH_TEXT, &args![at + offset]).u32();
        if e.call(TEXT_COMPARE, &args![mine, theirs]).u32() != 0 {
            return true;
        }
    }
    let pair = e.call(CARD_VALUE, &args![source]).u32();
    e.call(MEMCMP, &args![at + 0xb4, pair, 8u32]).u32() != 0
}

// Translated from 0059adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): forwards `this` and
/// the word to `00484c20`.
pub fn fn_0059adb0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(FORWARD_TARGET_00484C20, &args![this, argument]);
}

// Translated from 0059ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl (first, second)`, a sort comparison):
/// the first word of each card's value pair (`006ab360` of `*first` and
/// `*second`), compared signed: 1, -1 or 0.
pub fn fn_0059ae10(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let card = e.mem.u32(first.addr());
    let pair = e.call(CARD_VALUE, &args![card]).u32();
    let left = e.mem.u32(pair) as i32;
    let card = e.mem.u32(second.addr());
    let pair = e.call(CARD_VALUE, &args![card]).u32();
    let right = e.mem.u32(pair) as i32;
    if left > right {
        1
    } else if left < right {
        -1
    } else {
        0
    }
}

// Translated from 0059b1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCaravanMoney::Save` (Xbox PDB): starts the form, saves the full
/// name (+0x30), the model (+0x3C: `MODL`, `MODT`, `MODS`), the icon
/// (+0x5C, `ICON`), the message icon (+0x68, `MICO`), the pick-up/put-down
/// sounds (+0x80) and the form data, then the two extra models
/// (`0059b270` with 0 and 1) as `MOD2` and `MOD3`, and closes the form.
pub fn tes_caravan_money_save(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![at + 0x30]);
    e.call(
        SAVE_MODEL_TEXTURE_SWAP,
        &args![at + 0x3c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    e.call(SAVE_TEXTURE, &args![at + 0x5c, CHUNK_ICON]);
    e.call(SAVE_MESSAGE_ICON, &args![at + 0x68, CHUNK_MICO]);
    e.call(SAVE_PICKUP_PUTDOWN_SOUNDS, &args![at + 0x80]);
    e.call(FORM_SAVE_DATA, &args![this]);
    for (index, chunk) in [(0u32, CHUNK_MOD2), (1, CHUNK_MOD3)] {
        let model = e.call(MONEY_MODEL, &args![this, index]).u32();
        e.call(
            SAVE_MODEL_TEXTURE_SWAP,
            &args![model, chunk, CHUNK_MODT, CHUNK_MODS],
        );
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_ae60, fn_0059ae60(Ptr) -> Ptr),
        entry!(0x0059_add0, fn_0059add0(Ptr, Ptr)),
        entry!(0x0059_adf0, fn_0059adf0(Ptr, Ptr)),
        entry!(0x0059_a6c0, tes_caravan_card_init_item(Ptr)),
        entry!(0x0059_b160, tes_caravan_money_init_item(Ptr)),
        entry!(0x0059_b0e0, fn_0059b0e0(Ptr)),
        entry!(0x0059_b120, fn_0059b120(Ptr)),
        entry!(0x0059_afc0, fn_0059afc0(Ptr)),
        entry!(0x0059_af90, fn_0059af90(Ptr, u32) -> Ptr),
        entry!(0x0059_a800, fn_0059a800(Ptr, Ptr) -> bool),
        entry!(0x0059_bb50, tes_caravan_deck_load(Ptr, Ptr) -> bool),
        entry!(0x0059_a720, tes_caravan_card_save(Ptr)),
        entry!(0x0059_ac20, fn_0059ac20(Ptr, Ptr)),
        entry!(0x0059_acd0, fn_0059acd0(Ptr, Ptr) -> bool),
        entry!(0x0059_adb0, fn_0059adb0(Ptr, u32)),
        entry!(0x0059_ae10, fn_0059ae10(Ptr, Ptr) -> i32),
        entry!(0x0059_b1b0, tes_caravan_money_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0059ae60_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_ae60,
            size: 0xcc,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x78)]),
                (0x0047_b4f0, &[A::T(0x80)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x8c),
                        A::V(0x20),
                        A::V(0x2),
                        A::V(0x0048_a3d0),
                        A::V(0x0048_a490),
                    ],
                ),
                (0x004f_15a0, &[A::T(0x0), A::V(0x74)]),
                (0x0059_b120, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_49b4),
                (0x30, 4, 0x0103_499c),
                (0x3c, 4, 0x0103_4978),
                (0x5c, 4, 0x0103_4958),
                (0x68, 4, 0x0103_4944),
                (0x78, 4, 0x0103_4924),
                (0x80, 4, 0x0103_4910),
            ],
        });
    }

    #[test]
    fn forwarder_fn_0059add0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_00484D60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_add0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_00484D60)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_0059adf0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_00484DA0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_adf0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_00484DA0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    use std::cell::Cell;
    use std::rc::Rc;

    /// The calls a test made, without the top-level call and the file's chunk-walking calls.
    fn calls_after(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(address, _)| {
                ![top, GET_TES_FORM, GET_TES_CHUNK, NEXT_CHUNK].contains(address)
            })
            .collect()
    }

    /// Doubles for a file whose record has `form_type` and the given chunks; every other
    /// callee in `others` returns 0.
    fn file_with_chunks(e: &mut Engine, form_type: u32, chunks: Vec<u32>, others: &[u32]) {
        let position = Rc::new(Cell::new(0usize));
        e.register_double(GET_TES_FORM, move |_, _| Ret {
            eax: 0xffff_ff00 | form_type,
            ..Ret::default()
        });
        let current = position.clone();
        let list = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| Ret {
            eax: list.get(current.get()).copied().unwrap_or(0),
            ..Ret::default()
        });
        let count = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            Ret {
                eax: u32::from(position.get() < count),
                ..Ret::default()
            }
        });
        for address in others {
            e.register(*address, |_, _| Ret::default());
        }
    }

    const LOADER_CALLEES: [u32; 20] = [
        LOAD_FORM,
        SET_FLAG_00484AB0,
        GET_CHUNK_DATA,
        GET_CHUNK_DATA_SIZED,
        0x0048_92d0,
        0x0048_61f0,
        0x0048_e3d0,
        0x006e_cd40,
        0x0048_cd90,
        0x0047_a290,
        0x0040_3550,
        0x0048_7050,
        0x0048_a7b0,
        0x0048_5d50,
        0x0048_39c0,
        0x0082_56d0,
        0x0072_6c60,
        0x005a_e3d0,
        0x0040_1680,
        0x0050_3210,
    ];

    #[test]
    fn caravan_card_init_item_runs_the_components_unless_checked() {
        for (checked, expected) in [(1u32, 1usize), (0, 5)] {
            let mut e = Engine::new();
            let form = e.mem.alloc(0xbc);
            e.register_double(INIT_ITEM_CHECK, move |_, _| Ret {
                eax: 0x100 | checked,
                ..Ret::default()
            });
            for address in [0x0048_aa80u32, 0x0048_cd90, 0x0047_b6a0, SET_FLAG_00484AB0] {
                e.register(address, |_, _| Ret::default());
            }
            e.call_log = Some(vec![]);
            e.call(0x0059_a6c0, &args![form]);
            let calls = calls_after(&mut e, 0x0059_a6c0);
            assert_eq!(calls.len(), expected);
            if checked == 0 {
                assert_eq!(
                    calls[1..],
                    [
                        (0x0048_aa80, vec![form + 0x3c, form]),
                        (0x0048_cd90, vec![form + 0x80, form]),
                        (0x0047_b6a0, vec![form + 0x8c, form]),
                        (SET_FLAG_00484AB0, vec![form, 1]),
                    ]
                );
            }
        }
    }

    #[test]
    fn caravan_money_init_item_runs_the_components_unless_checked() {
        for (checked, expected) in [(1u32, 1usize), (0, 4)] {
            let mut e = Engine::new();
            let form = e.mem.alloc(0xcc);
            e.register_double(INIT_ITEM_CHECK, move |_, _| Ret {
                eax: checked,
                ..Ret::default()
            });
            for address in [0x0048_aa80u32, 0x0047_b6a0, SET_FLAG_00484AB0] {
                e.register(address, |_, _| Ret::default());
            }
            e.call_log = Some(vec![]);
            e.call(0x0059_b160, &args![form]);
            let calls = calls_after(&mut e, 0x0059_b160);
            assert_eq!(calls.len(), expected);
            if checked == 0 {
                assert_eq!(
                    calls[1..],
                    [
                        (0x0048_aa80, vec![form + 0x3c, form]),
                        (0x0047_b6a0, vec![form + 0x80, form]),
                        (SET_FLAG_00484AB0, vec![form, 1]),
                    ]
                );
            }
        }
    }

    /// An object with one-vtable sub-objects at +0x8c and +0xac, and doubles for the
    /// destructor's callees.
    fn money_with_members(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0xcc);
        e.put_vtable(0x0300_0000, &[0x0300_1000, 0x0300_1004]);
        e.put_vtable(0x0300_0100, &[0x0300_1100, 0x0300_1104]);
        e.mem.set_u32(object + 0x8c, 0x0300_0000);
        e.mem.set_u32(object + 0xac, 0x0300_0100);
        for address in [
            0x0300_1000u32,
            0x0300_1004,
            0x0300_1100,
            0x0300_1104,
            0x00ec_5fce,
            0x0047_b520,
            0x0048_e7d0,
            0x0047_a1a0,
            0x0040_2c40,
            0x0048_a490,
            0x0059_1300,
            0x0050_e9f0,
            0x0040_1030,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        object
    }

    #[test]
    fn fn_0059b0e0_and_fn_0059b120_call_slots_four_and_zero() {
        for (address, first, second) in [
            (0x0059_b0e0u32, 0x0300_1004u32, 0x0300_1104u32),
            (0x0059_b120, 0x0300_1000, 0x0300_1100),
        ] {
            let mut e = Engine::new();
            let object = money_with_members(&mut e);
            e.call_log = Some(vec![]);
            e.call(address, &args![object]);
            let calls = calls_after(&mut e, address);
            assert_eq!(
                calls,
                vec![(first, vec![object + 0x8c]), (second, vec![object + 0xac])]
            );
        }
    }

    #[test]
    fn fn_0059afc0_destroys_the_members_in_order() {
        let mut e = Engine::new();
        let object = money_with_members(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0059_afc0, &args![object]);
        let calls = calls_after(&mut e, 0x0059_afc0);
        assert_eq!(
            calls,
            vec![
                (0x0300_1004, vec![object + 0x8c]),
                (0x0300_1104, vec![object + 0xac]),
                (0x00ec_5fce, vec![object + 0x8c, 0x20, 2, 0x0048_a490]),
                (0x0047_b520, vec![object + 0x80]),
                (0x0048_e7d0, vec![object + 0x78]),
                (0x0047_a1a0, vec![object + 0x68]),
                (0x0040_2c40, vec![object + 0x5c]),
                (0x0048_a490, vec![object + 0x3c]),
                (0x0059_1300, vec![object + 0x30]),
                (0x0050_e9f0, vec![object]),
            ]
        );
        for (offset, vtable) in MONEY_VTABLES {
            assert_eq!(e.mem.u32(object + offset), vtable);
        }
    }

    #[test]
    fn fn_0059af90_frees_only_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            let mut e = Engine::new();
            let object = money_with_members(&mut e);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0059_af90, &args![object, flags]).u32(), object);
            let calls = calls_after(&mut e, 0x0059_af90);
            assert_eq!(calls.len(), if frees { 11 } else { 10 });
            assert_eq!(calls[9], (0x0050_e9f0, vec![object]));
            if frees {
                assert_eq!(calls[10], (0x0040_1030, vec![object]));
            }
        }
    }

    #[test]
    fn caravan_card_load_refuses_another_form_type() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xbc);
        file_with_chunks(&mut e, 0x74, vec![CHUNK_DATA], &LOADER_CALLEES);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_a800, &args![form, 0x5000u32]).bool());
        assert!(calls_after(&mut e, 0x0059_a800).is_empty());
    }

    #[test]
    fn caravan_card_load_dispatches_each_chunk() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xbc);
        let file = 0x5000u32;
        e.put_vtable(0x0300_0000, &[0; 0x80]);
        e.mem.set_u32(0x0300_0000 + 0xe0, 0x0300_10e0);
        e.mem.set_u32(0x0300_0000 + 0x134, 0x0300_1134);
        e.mem.set_u32(form, 0x0300_0000);
        let chunks = vec![
            CHUNK_MODL,
            CHUNK_MODT,
            CHUNK_MODD,
            CHUNK_MOSD,
            CHUNK_DATA,
            CHUNK_TX00,
            CHUNK_TX01,
            CHUNK_OBND,
            CHUNK_SCRI,
            CHUNK_MICO,
            CHUNK_ZNAM,
            CHUNK_FULL,
            CHUNK_YNAM,
            CHUNK_ICON,
            CHUNK_INTV,
            CHUNK_INTV,
            CHUNK_INTV,
            0x5858_5858,
        ];
        file_with_chunks(&mut e, 0x73, chunks, &LOADER_CALLEES);
        e.register(0x0300_10e0, |_, _| Ret::default());
        // Each chunk word reads 0x77 into the destination.
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_a800, &args![form, file]).bool());
        let calls = calls_after(&mut e, 0x0059_a800);
        let words: Vec<_> = calls
            .iter()
            .filter(|call| call.0 == GET_CHUNK_DATA)
            .map(|call| call.1[1])
            .collect();
        let calls: Vec<_> = calls
            .into_iter()
            .filter(|call| call.0 != GET_CHUNK_DATA)
            .collect();
        assert_eq!(
            calls,
            vec![
                (LOAD_FORM, vec![form, file]),
                (SET_FLAG_00484AB0, vec![form, 0]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_61f0, vec![form, file, 0, 0]),
                (0x0048_e3d0, vec![form + 0x9c, file]),
                (0x0048_e3d0, vec![form + 0xa8, file]),
                (0x0300_10e0, vec![form, file]),
                (0x006e_cd40, vec![form + 0x80, 0x77]),
                (0x0048_cd90, vec![form + 0x80, form]),
                (0x0047_a290, vec![form + 0x68, file]),
                (0x0040_3550, vec![form + 0x8c, 0x77]),
                (0x0048_7050, vec![form + 0x30, file]),
                (0x006e_cd40, vec![form + 0x8c, 0x77]),
                (0x0048_e3d0, vec![form + 0x5c, file]),
            ]
        );
        // SCRI, ZNAM, YNAM into stack words; the INTVs into +0xb8 first, then +0xb4.
        assert_eq!(words.len(), 6);
        assert_eq!(words[3..], [form + 0xb8, form + 0xb4, form + 0xb4]);
    }

    #[test]
    fn caravan_card_load_reads_editor_id_and_texture_swaps_into_stack_buffers() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xbc);
        let file = 0x5000u32;
        e.put_vtable(0x0300_0000, &[0; 0x80]);
        e.mem.set_u32(0x0300_0000 + 0x134, 0x0300_1134);
        e.mem.set_u32(form, 0x0300_0000);
        file_with_chunks(&mut e, 0x73, vec![CHUNK_EDID, CHUNK_MODS], &LOADER_CALLEES);
        e.register(0x0300_1134, |_, _| Ret::default());
        e.register(CHUNK_SIZE, |_, _| Ret {
            eax: 0x18,
            ..Ret::default()
        });
        e.register(0x0040_3570, |_, _| Ret {
            eax: 0xabcd_0003,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_a800, &args![form, file]).bool());
        let calls = calls_after(&mut e, 0x0059_a800);
        let sized: Vec<_> = calls
            .iter()
            .filter(|call| call.0 == GET_CHUNK_DATA_SIZED)
            .collect();
        assert_eq!(sized.len(), 2);
        assert_eq!(sized[0].1[2], 0x200);
        assert_eq!(sized[1].1[2], 0x18);
        let editor_id: Vec<_> = calls.iter().filter(|call| call.0 == 0x0300_1134).collect();
        assert_eq!(editor_id[0].1, vec![form, sized[0].1[1]]);
        let swaps: Vec<_> = calls.iter().filter(|call| call.0 == 0x0048_a7b0).collect();
        assert_eq!(swaps[0].1, vec![form + 0x3c, sized[1].1[1], 0x18, 3]);
    }

    #[test]
    fn caravan_deck_load_refuses_another_form_type() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x30);
        file_with_chunks(&mut e, 0x73, vec![CHUNK_DATA], &LOADER_CALLEES);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_bb50, &args![form, 0x5000u32]).bool());
        assert!(calls_after(&mut e, 0x0059_bb50).is_empty());
    }

    #[test]
    fn caravan_deck_load_reads_data_name_and_bounds() {
        for swaps in [false, true] {
            let mut e = Engine::new();
            let form = e.mem.alloc(0x30);
            let file = 0x5000u32;
            e.put_vtable(0x0300_0000, &[0; 0x80]);
            e.mem.set_u32(0x0300_0000 + 0xe0, 0x0300_10e0);
            e.mem.set_u32(form, 0x0300_0000);
            file_with_chunks(
                &mut e,
                0x75,
                vec![CHUNK_DATA, CHUNK_FULL, CHUNK_OBND],
                &LOADER_CALLEES,
            );
            e.register(0x0300_10e0, |_, _| Ret::default());
            e.register_double(0x0040_1680, move |_, _| Ret {
                eax: u32::from(swaps),
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            assert!(e.call(0x0059_bb50, &args![form, file]).bool());
            let calls = calls_after(&mut e, 0x0059_bb50);
            let mut expected = vec![
                (LOAD_FORM, vec![form, file]),
                (SET_FLAG_00484AB0, vec![form, 0]),
                (0x0048_61f0, vec![form, file, form + 0x28, 4]),
                (0x0040_1680, vec![file]),
            ];
            if swaps {
                expected.push((0x0050_3210, vec![form + 0x28]));
            }
            expected.push((0x0048_7050, vec![form + 0x18, file]));
            expected.push((0x0300_10e0, vec![form, file]));
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn caravan_deck_load_adds_each_card_to_the_list() {
        // (list at +0x24, what 008256d0 answers, the function that gets the card)
        for (list, answer, adder) in [
            (0u32, 0u32, 0x0072_6c60u32),
            (0x6000, 1, 0x0072_6c60),
            (0x6000, 0, 0x005a_e3d0),
        ] {
            let mut e = Engine::new();
            let form = e.mem.alloc(0x30);
            let file = 0x5000u32;
            e.mem.set_u32(form + 0x24, list);
            file_with_chunks(&mut e, 0x75, vec![CHUNK_CARD], &LOADER_CALLEES);
            e.register(GET_CHUNK_DATA, |e, a| {
                e.mem.set_u32(a[1], 0x1234);
                Ret::default()
            });
            e.register(0x0048_5d50, |e, a| {
                let word = e.mem.u32(a[0]);
                e.mem.set_u32(a[0], word + 1);
                Ret::default()
            });
            e.register(0x0048_39c0, |_, a| Ret {
                eax: a[0] + 0x10_0000,
                ..Ret::default()
            });
            e.register_double(0x0082_56d0, move |_, _| Ret {
                eax: answer,
                ..Ret::default()
            });
            let seen = Rc::new(Cell::new(0u32));
            let card = seen.clone();
            e.register_double(adder, move |e, a| {
                card.set(e.mem.u32(a[1]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            assert!(e.call(0x0059_bb50, &args![form, file]).bool());
            let calls = calls_after(&mut e, 0x0059_bb50);
            assert_eq!(seen.get(), 0x0010_1235);
            let resolve: Vec<_> = calls.iter().filter(|call| call.0 == 0x0048_39c0).collect();
            assert_eq!(resolve[0].1, vec![0x1235]);
            let added: Vec<_> = calls.iter().filter(|call| call.0 == adder).collect();
            assert_eq!(added.len(), 1);
            assert_eq!(added[0].1[0], list);
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0082_56d0).count(),
                usize::from(list != 0)
            );
        }
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        for addr in [
            FORM_START,
            FORM_SAVE_DATA,
            FORM_CLOSE,
            SAVE_FULL_NAME,
            SAVE_MODEL_TEXTURE_SWAP,
            SAVE_TEXTURE,
            SAVE_MESSAGE_ICON,
            SAVE_SCRIPTABLE,
            SAVE_PICKUP_PUTDOWN_SOUNDS,
            ADD_CHUNK_WORD,
            FORM_COPY_ALL_COMPONENTS,
            TEXTURE_SET_PATH,
            FORM_SET_TYPE,
            FORWARD_TARGET_00484C20,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(CARD_TEXTURE_FIRST, |_, a| (a[0] + 0x10).into_ret());
        e.register(CARD_TEXTURE_SECOND, |_, a| (a[0] + 0x20).into_ret());
        e.register(CARD_VALUE, |_, a| (a[0] + 0xb4).into_ret());
        e.register(MONEY_MODEL, |_, a| (a[0] + 0x100 + a[1]).into_ret());
        e.register(TEXTURE_PATH_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_TYPE, |_, _| 0x56u32.into_ret());
        e
    }

    #[test]
    fn card_save_writes_every_part() {
        let mut e = engine();
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0xb4, 7);
        e.mem.set_u32(this + 0xb8, 8);
        e.call_log = Some(vec![]);
        e.call(0x0059_a720, &args![this]);
        assert_eq!(
            calls(&e, SAVE_TEXTURE),
            vec![
                vec![this + 0x5c, CHUNK_ICON],
                vec![this + 0x10, CHUNK_TX00],
                vec![this + 0x20, CHUNK_TX01]
            ]
        );
        assert_eq!(
            calls(&e, ADD_CHUNK_WORD),
            vec![vec![CHUNK_INTV, 8], vec![CHUNK_INTV, 7]]
        );
        assert_eq!(calls(&e, SAVE_SCRIPTABLE), vec![vec![this + 0x80]]);
        assert_eq!(
            calls(&e, SAVE_PICKUP_PUTDOWN_SOUNDS),
            vec![vec![this + 0x8c]]
        );
        let log = e.call_log.take().unwrap();
        let tail: Vec<u32> = log.iter().rev().take(2).map(|c| c.0).collect();
        assert_eq!(tail, vec![FORM_CLOSE, FORM_SAVE_DATA]);
    }

    #[test]
    fn card_copy_takes_paths_values_and_type() {
        let mut e = engine();
        let this = e.mem.alloc(0xc0);
        let other = e.mem.alloc(0xc0);
        e.mem.set_u32(other + 0xb4, 3);
        e.mem.set_u32(other + 0xb8, 4);
        e.call_log = Some(vec![]);
        e.call(0x0059_ac20, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, TYPE_FORM, TYPE_CARAVAN_CARD, 0]]
        );
        assert_eq!(
            calls(&e, TEXTURE_SET_PATH),
            vec![
                vec![this + 0x9c, other + 0x11],
                vec![this + 0xa8, other + 0x21]
            ]
        );
        assert_eq!((e.mem.u32(this + 0xb4), e.mem.u32(this + 0xb8)), (3, 4));
        assert_eq!(calls(&e, FORM_TYPE), vec![vec![other]]);
        assert_eq!(calls(&e, FORM_SET_TYPE), vec![vec![this, 0x56]]);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0059_ac20, &args![this, other]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn card_compare_branches() {
        let mut e = engine();
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(STATE).into_ret()
        });
        // The text comparison differs on the call numbered by STATE + 4.
        e.mem.set_u32(STATE + 4, 0xff);
        e.register(TEXT_COMPARE, |e, _| {
            let n = e.mem.u32(STATE + 8);
            e.mem.set_u32(STATE + 8, n + 1);
            u32::from(n == e.mem.u32(STATE + 4)).into_ret()
        });
        e.register(MEMCMP, |e, _| e.mem.u32(STATE + 0xc).into_ret());
        let this = 0x1000u32;
        let other = 0x2000u32;
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_acd0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, TEXT_COMPARE),
            vec![vec![0x109d, 0x2011], vec![0x10a9, 0x2021]]
        );
        assert_eq!(calls(&e, MEMCMP), vec![vec![0x10b4, 0x20b4, 8]]);
        for stop in [0, 1] {
            e.mem.set_u32(STATE + 4, stop);
            e.mem.set_u32(STATE + 8, 0);
            assert!(e.call(0x0059_acd0, &args![this, other]).bool());
        }
        e.mem.set_u32(STATE + 4, 0xff);
        e.mem.set_u32(STATE + 0xc, 1);
        assert!(e.call(0x0059_acd0, &args![this, other]).bool());
        e.mem.set_u32(STATE + 0xc, 0);
        e.mem.set_u32(STATE, 1);
        assert!(e.call(0x0059_acd0, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0059_acd0, &args![this, other]).bool());
    }

    #[test]
    fn forward_and_sort_comparison() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0059_adb0, &args![0x1000u32, 0x55u32]);
        assert_eq!(calls(&e, FORWARD_TARGET_00484C20), vec![vec![0x1000, 0x55]]);
        let first = e.mem.alloc(0xc0);
        let second = e.mem.alloc(0xc0);
        let a = e.mem.alloc(4);
        let b = e.mem.alloc(4);
        e.mem.set_u32(a, first);
        e.mem.set_u32(b, second);
        e.mem.set_u32(first + 0xb4, 5);
        e.mem.set_u32(second + 0xb4, (-3i32) as u32);
        assert_eq!(e.call(0x0059_ae10, &args![a, b]).i32(), 1);
        assert_eq!(e.call(0x0059_ae10, &args![b, a]).i32(), -1);
        assert_eq!(e.call(0x0059_ae10, &args![a, a]).i32(), 0);
    }

    #[test]
    fn money_save_writes_every_part() {
        let mut e = engine();
        let this = 0x1000u32;
        e.call_log = Some(vec![]);
        e.call(0x0059_b1b0, &args![this]);
        assert_eq!(
            calls(&e, SAVE_MODEL_TEXTURE_SWAP),
            vec![
                vec![0x103c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
                vec![0x1100, CHUNK_MOD2, CHUNK_MODT, CHUNK_MODS],
                vec![0x1101, CHUNK_MOD3, CHUNK_MODT, CHUNK_MODS]
            ]
        );
        assert_eq!(calls(&e, SAVE_PICKUP_PUTDOWN_SOUNDS), vec![vec![0x1080]]);
        assert_eq!(calls(&e, MONEY_MODEL), vec![vec![this, 0], vec![this, 1]]);
        assert_eq!(calls(&e, FORM_CLOSE), vec![vec![this]]);
    }
}
