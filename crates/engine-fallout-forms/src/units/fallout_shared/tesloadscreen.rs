//! `fallout shared/tesloadscreen.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a66e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a66e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESLoadScreen::TESLoadScreen` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x4b (0x40 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tesloadscreen_tesloadscreen(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_e270, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_66cc);
    e.mem.set_u32(base + 0x18, 0x0103_66ac);
    e.mem.set_u32(base + 0x24, 0x0103_6694);
    e.call(0x0096_a2d0, &args![base + 0x2c]);
    e.call(0x0040_37b0, &args![base + 0x38]);
    e.call(0x004f_15a0, &args![this, 0x4b]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    e.mem.set_u32(base + 0x34, 0x0);
    this
}

/// Byte-swaps the `u32` at the address in place (`00401080`, `cdecl`: the
/// address and a byte the callers push as 0).
const SWAP_WORD_IN_PLACE: u32 = 0x0040_1080;

// Translated from 005a6b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the three words at `+0`, `+4` and `+8` in place, each through
/// `00401080` with 0 as its second word (unnamed in the map; `ECX`, no
/// stack words). The file header reader (`00471400`) calls it on the
/// 12-byte `HEDR` data it read into its `TESFile` (`+0x3dc`) when the file
/// needs its byte order converted.
pub fn fn_005a6b10(e: &mut Engine, this: Ptr) {
    for offset in [0u32, 4, 8] {
        e.call(SWAP_WORD_IN_PLACE, &args![this.byte_add(offset), 0u32]);
    }
}

// Translated from 005a67b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a67e0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a67b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_67e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESForm::LoadForm` (`00485110`, engine map): `ECX` = the form, the file.
const TES_FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`tesform.cpp`): `ECX` = the form, one word (0 before
/// loading, 1 after the ids are resolved).
const TES_FORM_MARK_00484AB0: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (`004726b0`, engine map): the next chunk's tag, 0
/// at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0`: moves to the next chunk; false when there is none.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (`004727f0`, engine map): reads one word.
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (`00472890`, engine map): buffer and size
/// (0: the whole chunk).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `00401660`: the current chunk's size (`ECX` = the file).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680`: whether the file's data needs its byte order converted.
const NEEDS_BYTE_SWAP: u32 = 0x0040_1680;
/// `00401000`: `operator new` (cdecl, the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030`: `operator delete` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `TESForm::GetFile` (`00484e60`, engine map): `ECX` = the form, the index.
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, engine map): cdecl, the address
/// of a form id and the file.
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `0084e3a0`: the form's id (`ECX` = the form).
const FORM_ID: u32 = 0x0084_e3a0;
/// `006815c0`: returns its `ECX` (the address of a list node's item).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070`: the next list node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `008256d0`: whether a list node is empty (`ECX` = the node).
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `00470470`: empties the list (`ECX` = the list head).
const LIST_CLEAR: u32 = 0x0047_0470;
/// `"MASTERFILE: File %s contains old data for loadscreen %s (%08X)."`.
const OLD_DATA_WARNING: u32 = 0x0103_6808;
/// The empty string `fn_005a6e10` returns when there is no text.
const EMPTY_STRING: u32 = 0x0101_1584;
/// The data handler (`TESDataHandler*`) `fn_005a6b70` looks forms up in.
const DATA_HANDLER: u32 = 0x011c_3f2c;

const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_WMI1: u32 = 0x3149_4d57;
const CHUNK_DESC: u32 = 0x4353_4544;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_FNAM: u32 = 0x4d41_4e46;
const CHUNK_LNAM: u32 = 0x4d41_4e4c;
const CHUNK_ICON: u32 = 0x4e4f_4349;

// Translated from 005a67e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a67e0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `TESLoadScreen` destructor body: stores the vtables back (`010366cc`,
/// `010366ac` at +0x18, `01036694` at +0x24), clears the location list
/// ([`fn_005a6880`]), then runs the member destructors `004037d0` (+0x38),
/// `0046ffb0` (+0x2C), `0048e2e0` (+0x18) and `TESForm::~TESForm`
/// (`00483630`, engine map).
pub fn fn_005a67e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_66cc);
    e.mem.set_u32(base + 0x18, 0x0103_66ac);
    e.mem.set_u32(base + 0x24, 0x0103_6694);
    fn_005a6880(e, this);
    e.call(0x0040_37d0, &args![base + 0x38]);
    e.call(0x0046_ffb0, &args![base + 0x2c]);
    e.call(0x0048_e2e0, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005a6880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the location entries ([`fn_005a6e50`]), clears the word at +0x34
/// and runs `00483710` on `this`.
pub fn fn_005a6880(e: &mut Engine, this: Ptr) {
    fn_005a6e50(e, this);
    e.mem.set_u32(this.addr() + 0x34, 0);
    e.call(0x0048_3710, &args![this]);
}

// Translated from 005a68d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a68d0 - the /GS security-cookie check (00ec408c) is not translated; the __alloca_probe_16 (00ec5ec0) buffers are with_stack blocks of the chunk size rounded up to 16 bytes.
/// `TESLoadScreen::Load` (Xbox PDB): `TESForm::LoadForm`, `00484ab0(0)`,
/// then each chunk of `file` (one stack word) until none is left: `OBND`
/// to vtable slot 0xE0 with the file; `WMI1`, a word, into +0x34 when not
/// 0; `DESC` read whole into a stack buffer of the chunk's size and set on
/// the string at +0x38 (`004037f0`, with 0); `EDID` (at most 0x200 bytes)
/// to vtable slot 0x134; `FNAM` only logs the old-data warning (`005b5e40`
/// with the file's name `00891170`, the editor id from slot 0x130 and the
/// form id); `LNAM` reads 12 bytes into a new block (byte-swapped by
/// [`fn_005a6b10`] when the file needs it) and adds the block to the list at
/// +0x2C (`005ae3d0`); `ICON` loads the texture at +0x18
/// (`TESTexture::LoadTextureChunk`, `0048e3d0`). Returns true.
pub fn tes_load_screen_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(TES_FORM_LOAD_FORM, &args![this, file]);
    e.call(TES_FORM_MARK_00484AB0, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                    e.vcall(base, 0x134, &args![buffer]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_ICON => {
                let texture = if base == 0 { 0 } else { base + 0x18 };
                e.call(0x0048_e3d0, &args![texture, file]);
            }
            CHUNK_DESC => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                let reserved = size.wrapping_add(15) & !15;
                e.with_stack(reserved, |e, buffer| {
                    e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
                    e.call(0x0040_37f0, &args![base + 0x38, buffer, 0u32]);
                });
            }
            CHUNK_LNAM => {
                let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
                e.with_stack(4, |e, local| {
                    e.mem.set_u32(local.addr(), block);
                    e.call(GET_CHUNK_DATA, &args![file, block, 0xcu32]);
                    if e.call(NEEDS_BYTE_SWAP, &args![file]).bool() {
                        fn_005a6b10(e, Ptr::new(block));
                    }
                    e.call(0x005a_e3d0, &args![base + 0x2c, local]);
                });
            }
            CHUNK_FNAM => {
                let form_id = e.call(FORM_ID, &args![this]).u32();
                let editor_id = e.vcall(base, 0x130, &args![]).u32();
                let file_name = e.call(0x0089_1170, &args![file]).u32();
                e.call(
                    0x005b_5e40,
                    &args![OLD_DATA_WARNING, file_name, editor_id, form_id],
                );
            }
            CHUNK_WMI1 => {
                let value = e.with_stack(4, |e, local| {
                    e.call(GET_CHUNK_WORD, &args![file, local]);
                    e.mem.u32(local.addr())
                });
                if value != 0 {
                    e.mem.set_u32(base + 0x34, value);
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

// Translated from 005a6b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the loaded form ids (nothing when `004013e0` is true): each
/// location entry's two ids (+0 and +4, when not 0) get the form's file's
/// compile index (`TESForm::GetFile(-1)`, `TESForm::AddCompileIndex`), the
/// id at +0x34 likewise and is then looked up through the data handler
/// (`00461660` on `[011c3f2c]`) into +0x34; then `00484ab0(1)`.
pub fn fn_005a6b70(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    let mut node = this.addr() + 0x2c;
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item_slot = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        if e.mem.u32(item) != 0 {
            let compile_file = e
                .call(TES_FORM_GET_FILE, &args![this, 0xffff_ffffu32])
                .u32();
            e.call(TES_FORM_ADD_COMPILE_INDEX, &args![item, compile_file]);
        }
        if e.mem.u32(item + 4) != 0 {
            let compile_file = e
                .call(TES_FORM_GET_FILE, &args![this, 0xffff_ffffu32])
                .u32();
            e.call(TES_FORM_ADD_COMPILE_INDEX, &args![item + 4, compile_file]);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if e.mem.u32(this.addr() + 0x34) != 0 {
        let id = e.with_stack(4, |e, local| {
            let id = e.mem.u32(this.addr() + 0x34);
            e.mem.set_u32(local.addr(), id);
            let compile_file = e
                .call(TES_FORM_GET_FILE, &args![this, 0xffff_ffffu32])
                .u32();
            e.call(TES_FORM_ADD_COMPILE_INDEX, &args![local, compile_file]);
            e.mem.u32(local.addr())
        });
        let handler = e.mem.u32(DATA_HANDLER);
        let form = e.call(0x0046_1660, &args![handler, id]).u32();
        e.mem.set_u32(this.addr() + 0x34, form);
    }
    e.call(TES_FORM_MARK_00484AB0, &args![this, 1u32]);
}

// Translated from 005a6e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The text of the string at `this + 0x14` (`00559450`), or the empty
/// string `01011584` when that is null. Two stack words it never reads
/// (`RET 8`).
pub fn fn_005a6e10(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    let text = e.call(0x0055_9450, &args![this.byte_add(0x14)]).u32();
    if text == 0 {
        EMPTY_STRING
    } else {
        e.call(0x0055_9450, &args![this.byte_add(0x14)]).u32()
    }
}

// Translated from 005a6e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every location entry of the list at +0x2C (`00401030`) up to the
/// first empty node, then empties the list (`00470470`).
pub fn fn_005a6e50(e: &mut Engine, this: Ptr) {
    let mut node = this.addr() + 0x2c;
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item_slot = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        e.call(OPERATOR_DELETE, &args![item]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(LIST_CLEAR, &args![this.addr() + 0x2c]);
}

// Translated from 005a6ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLoadScreen::CanDisplayForForm` (Xbox PDB): whether one of the
/// location entries (+0x2C) names `form` (one stack word). By the form's
/// type (`00401170`; the compiler's byte table at `005a70d8` maps 0x39 to
/// the cell case, 0x3A..0x40 and 0x69 to the reference case, 0x41 to the
/// world case, all else to no match):
/// - 0x39: an interior cell (`00425fd0` true) matches an entry whose +0
///   is its id; otherwise its world space (`TESObjectCELL::GetWorldSpace`)
///   and the cell key `00587410(x, y)` of its grid coordinates
///   (`GetDataX`, `GetDataY`) match an entry's +4 and +8;
/// - references: the parent cell (`008d6f30`) when it is interior,
///   otherwise the world space (`TESObjectREFR::GetWorldSpace`) and the key
///   of the position (vtable slot 0x1F4, `TESWorldSpace::GetKeyForWorldCoord`);
/// - 0x41: the form's own id against +0.
pub fn tes_load_screen_can_display_for_form(e: &mut Engine, this: Ptr, form: Ptr) -> bool {
    let mut form_id = 0u32;
    let mut world_id = 0u32;
    let mut key = 0u32;
    let mut found = false;
    let kind = e.call(0x0040_1170, &args![form]).u32();
    match kind.wrapping_sub(0x39) {
        0 => {
            if e.call(0x0042_5fd0, &args![form]).bool() {
                form_id = e.call(FORM_ID, &args![form]).u32();
                found = true;
            } else {
                let world = e.call(0x0054_ddd0, &args![form]).u32();
                if world != 0 {
                    world_id = e.call(FORM_ID, &args![world]).u32();
                    let y = e.call(0x0054_4c60, &args![form]).u32();
                    let x = e.call(0x0054_4c30, &args![form]).u32();
                    key = e.call(0x0058_7410, &args![x, y]).u32();
                    found = true;
                }
            }
        }
        1..=7 | 0x30 => {
            let cell = e.call(0x008d_6f30, &args![form]).u32();
            if cell != 0 && e.call(0x0042_5fd0, &args![cell]).bool() {
                form_id = e.call(FORM_ID, &args![cell]).u32();
                found = true;
            } else {
                let world = e.call(0x0057_5d70, &args![form]).u32();
                if world != 0 {
                    world_id = e.call(FORM_ID, &args![world]).u32();
                    let position = e.vcall(form.addr(), 0x1f4, &args![]).u32();
                    key = e.call(0x0058_7440, &args![position]).u32();
                    found = true;
                }
            }
        }
        8 => {
            form_id = e.call(FORM_ID, &args![form]).u32();
            found = true;
        }
        _ => {}
    }
    if !found {
        return false;
    }
    let mut node = this.addr() + 0x2c;
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item_slot = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_slot);
        if form_id != 0 {
            if e.mem.u32(item) == form_id {
                return true;
            }
        } else if world_id != 0 && e.mem.u32(item + 4) == world_id && e.mem.u32(item + 8) == key {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`),
/// `thiscall` on the form, no stack words.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the type
/// descriptors `TESForm` (`01183028`) and `TESLoadScreen` (`01186260`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_LOAD_SCREEN: u32 = 0x0118_6260;
/// `TESForm::CopyAllComponents` (`004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `00905820` (`thiscall` on a list node, `RET 4`: the address of an item):
/// appends the item; `005ae380` counts a list's items; `005f65d0` (`RET 4`:
/// the address of an item) whether a list holds it.
const LIST_APPEND: u32 = 0x0090_5820;
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_HOLDS: u32 = 0x005f_65d0;
/// The word at +0x34 (`005f36f0`) and its setter (`008d7dc0`, `RET 4`).
const WORD_34: u32 = 0x005f_36f0;
const SET_WORD_34: u32 = 0x008d_7dc0;
/// The load screen's location list.
const LOCATIONS_OFFSET: u32 = 0x2c;

// Translated from 005a68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The load screen's save (engine map: no name; no stack words):
/// `StartForm`, then `CloseForm`.
pub fn fn_005a68b0(e: &mut Engine, this: Ptr) {
    e.call(START_FORM, &args![this]);
    e.call(CLOSE_FORM, &args![this]);
}

/// `other` as a `TESLoadScreen` (`__RTDynamicCast` from `TESForm`), or 0.
fn as_load_screen(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_LOAD_SCREEN, 0u32],
    )
    .u32()
}

// Translated from 005a6c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The load screen's copy (engine map: no name; `RET 4`): when `other` is a
/// `TESLoadScreen`, empties this location list (`fn_005a6e50`), appends each
/// of the other's items (+0x2c) until a node holds none (`00905820` on the
/// current destination node, which moves on to its next node when there is
/// one), copies the word at +0x34, then all components
/// (`TESForm::CopyAllComponents` with the cast form).
pub fn fn_005a6c70(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_load_screen(e, other);
    if source == 0 {
        return;
    }
    fn_005a6e50(e, this);
    let mut node = source + LOCATIONS_OFFSET;
    let mut target = this.addr() + LOCATIONS_OFFSET;
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        e.call(LIST_APPEND, &args![target, item]);
        if e.call(NODE_NEXT, &args![target]).u32() != 0 {
            target = e.call(NODE_NEXT, &args![target]).u32();
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let word = e.call(WORD_34, &args![source]).u32();
    e.call(SET_WORD_34, &args![this, word]);
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
}

// Translated from 005a6d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The load screen's compare (engine map: no name; `RET 4`): true when
/// `other` is no `TESLoadScreen`, when the location lists hold different
/// numbers of items (`005ae380`, this one's first), when an item of the
/// other's list is not in this one's (`005f65d0`), when
/// `TESForm::CompareAllComponents` (with the cast form) says so, or when
/// the words at +0x34 differ.
pub fn fn_005a6d30(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_load_screen(e, other);
    if source == 0 {
        return true;
    }
    let mut node = source + LOCATIONS_OFFSET;
    let mine = e
        .call(LIST_COUNT, &args![this.addr() + LOCATIONS_OFFSET])
        .u32();
    if mine != e.call(LIST_COUNT, &args![node]).u32() {
        return true;
    }
    while node != 0 && !e.call(NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.call(LIST_HOLDS, &args![this.addr() + LOCATIONS_OFFSET, item])
            .u32() as u8
            == 0
        {
            return true;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).u32() as u8 != 0 {
        return true;
    }
    let mine = e.call(WORD_34, &args![this]).u32();
    mine != e.call(WORD_34, &args![source]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_66e0, tesloadscreen_tesloadscreen(Ptr) -> Ptr),
        entry!(0x005a6b10, fn_005a6b10(Ptr)),
        entry!(0x005a_67b0, fn_005a67b0(Ptr, u32) -> Ptr),
        entry!(0x005a_67e0, fn_005a67e0(Ptr)),
        entry!(0x005a_6880, fn_005a6880(Ptr)),
        entry!(0x005a_68d0, tes_load_screen_load(Ptr, Ptr) -> bool),
        entry!(0x005a_6b70, fn_005a6b70(Ptr)),
        entry!(0x005a_6e10, fn_005a6e10(Ptr, u32, u32) -> u32),
        entry!(0x005a_6e50, fn_005a6e50(Ptr)),
        entry!(
            0x005a_6ec0,
            tes_load_screen_can_display_for_form(Ptr, Ptr) -> bool
        ),
        entry!(0x005a_68b0, fn_005a68b0(Ptr)),
        entry!(0x005a_6c70, fn_005a6c70(Ptr, u32)),
        entry!(0x005a_6d30, fn_005a6d30(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesloadscreen_tesloadscreen_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_66e0,
            size: 0x40,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_e270, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x2c)]),
                (0x0040_37b0, &[A::T(0x38)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4b)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_66cc),
                (0x18, 4, 0x0103_66ac),
                (0x24, 4, 0x0103_6694),
                (0x34, 4, 0x0),
            ],
        });
    }

    #[test]
    fn swaps_the_three_words_in_order() {
        use super::*;
        let mut e = Engine::new();
        e.register(SWAP_WORD_IN_PLACE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005a_6b10, &args![Ptr::<()>::new(0x0a00_0000)]);
        let swaps: Vec<Vec<u32>> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == SWAP_WORD_IN_PLACE)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            swaps,
            vec![
                vec![0x0a00_0000, 0],
                vec![0x0a00_0004, 0],
                vec![0x0a00_0008, 0],
            ]
        );
    }
    #[test]
    fn at_005a67b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_67e0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_67b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_67b0)
                .collect();
            let mut expected = vec![(0x005a_67e0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    /// The location list doubles: a node is {item, next}; a node whose item
    /// is 0 is empty.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        for callee in [OPERATOR_DELETE, LIST_CLEAR, 0x0048_3710] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    /// A load screen whose list at +0x2C holds `entries` ({id, world, key}).
    fn screen_with(e: &mut Engine, entries: &[[u32; 3]]) -> (u32, Vec<u32>) {
        let this = e.mem.alloc(0x40);
        let mut items = vec![];
        let mut node = this + 0x2c;
        for (i, entry) in entries.iter().enumerate() {
            let item = e.mem.alloc(12);
            for (k, word) in entry.iter().enumerate() {
                e.mem.set_u32(item + 4 * k as u32, *word);
            }
            items.push(item);
            e.mem.set_u32(node, item);
            if i + 1 < entries.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        (this, items)
    }

    #[test]
    fn fn_005a6e50_frees_every_entry_and_clears_the_list() {
        let mut e = list_engine();
        let (this, items) = screen_with(&mut e, &[[1, 0, 0], [2, 0, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x005a_6e50, &args![this]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == OPERATOR_DELETE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(freed, items);
        assert_eq!(log.last().unwrap(), &(LIST_CLEAR, vec![this + 0x2c]));
    }

    #[test]
    fn fn_005a6880_frees_the_list_and_clears_0x34() {
        let mut e = list_engine();
        let (this, _) = screen_with(&mut e, &[]);
        e.mem.set_u32(this + 0x34, 77);
        e.call_log = Some(vec![]);
        e.call(0x005a_6880, &args![this]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(0x0048_3710, vec![this]));
    }

    #[test]
    fn fn_005a67e0_restores_the_vtables_and_runs_the_member_destructors() {
        let mut e = list_engine();
        for callee in [0x0040_37d0, 0x0046_ffb0, 0x0048_e2e0, 0x0048_3630] {
            e.register(callee, |_, _| Ret::default());
        }
        let (this, _) = screen_with(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x005a_67e0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_66cc);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_66ac);
        assert_eq!(e.mem.u32(this + 0x24), 0x0103_6694);
        let log = e.call_log.take().unwrap();
        let tail: Vec<_> = log[log.len() - 4..].to_vec();
        assert_eq!(
            tail,
            vec![
                (0x0040_37d0, vec![this + 0x38]),
                (0x0046_ffb0, vec![this + 0x2c]),
                (0x0048_e2e0, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_005a6e10_returns_the_text_or_the_empty_string() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x005a_6e10, &args![this, 1u32, 2u32]).u32(),
            EMPTY_STRING
        );
        e.mem.set_u32(this + 0x14, 0x5000);
        assert_eq!(e.call(0x005a_6e10, &args![this, 1u32, 2u32]).u32(), 0x5000);
    }

    #[test]
    fn fn_005a6b70_resolves_the_entries_and_the_word_at_0x34() {
        let mut e = list_engine();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(DATA_HANDLER, 0x0300_0000);
        e.register(0x0040_13e0, |_, _| Ret::default());
        e.register(TES_FORM_GET_FILE, |_, _| 0x0900_0000u32.into_ret());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0200_0000);
            Ret::default()
        });
        e.register(0x0046_1660, |_, a| (a[1] + 1).into_ret());
        e.register(TES_FORM_MARK_00484AB0, |_, _| Ret::default());
        let (this, items) = screen_with(&mut e, &[[0x11, 0, 0], [0, 0x22, 5]]);
        e.mem.set_u32(this + 0x34, 0x33);
        e.call_log = Some(vec![]);
        e.call(0x005a_6b70, &args![this]);
        assert_eq!(e.mem.u32(items[0]), 0x0200_0011);
        assert_eq!(e.mem.u32(items[0] + 4), 0);
        assert_eq!(e.mem.u32(items[1]), 0);
        assert_eq!(e.mem.u32(items[1] + 4), 0x0200_0022);
        assert_eq!(e.mem.u32(items[1] + 8), 5);
        assert_eq!(e.mem.u32(this + 0x34), 0x0200_0034);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0046_1660, vec![0x0300_0000, 0x0200_0033])));
        assert_eq!(
            log.last().unwrap(),
            &(TES_FORM_MARK_00484AB0, vec![this, 1])
        );

        // A form for which 004013e0 is true is left alone.
        e.register(0x0040_13e0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x005a_6b70, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    /// Doubles for the chunk reader: the file holds a list of
    /// (tag, data) chunks at +0 (count) and +4 (index).
    fn load_engine(chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = list_engine();
        let list = std::rc::Rc::new(chunks);
        let current = std::rc::Rc::new(std::cell::Cell::new(0usize));
        e.register(TES_FORM_LOAD_FORM, |_, _| Ret::default());
        e.register(TES_FORM_MARK_00484AB0, |_, _| Ret::default());
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_TES_CHUNK, move |_, _| {
            l.get(c.get()).map_or(0, |chunk| chunk.0).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(NEXT_CHUNK, move |_, _| {
            c.set(c.get() + 1);
            u32::from(c.get() < l.len()).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(CHUNK_SIZE, move |_, _| {
            (l[c.get()].1.len() as u32).into_ret()
        });
        let (l, c) = (list.clone(), current.clone());
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = &l[c.get()].1;
            let n = if a[2] == 0 {
                data.len()
            } else {
                (a[2] as usize).min(data.len())
            };
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        let (l, c) = (list, current);
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            let data = &l[c.get()].1;
            e.mem.write(a[1], &data[..4]);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NEEDS_BYTE_SWAP, |_, _| Ret::default());
        for callee in [
            0x0048_e3d0,
            0x0040_37f0,
            0x005a_e3d0,
            0x005b_5e40,
            0x0089_1170,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn load_reads_every_chunk() {
        let chunks = vec![
            (CHUNK_EDID, b"Screen\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_ICON, b"a.dds\0".to_vec()),
            (CHUNK_DESC, b"text\0".to_vec()),
            (CHUNK_LNAM, vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]),
            (CHUNK_WMI1, vec![0x44, 0, 0, 0]),
            (CHUNK_WMI1, vec![0, 0, 0, 0]),
            (CHUNK_FNAM, vec![0; 4]),
            (0x1234_5678, vec![]),
        ];
        let mut e = load_engine(chunks);
        e.put_vtable(0x0200_0000, &[0; 0x80]);
        e.mem.set_u32(0x0200_0000 + 0x134, 0x0090_0000);
        e.mem.set_u32(0x0200_0000 + 0xe0, 0x0090_0010);
        e.mem.set_u32(0x0200_0000 + 0x130, 0x0090_0020);
        e.register(0x0090_0000, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"Screen".to_vec());
            Ret::default()
        });
        e.register(0x0090_0010, |_, _| Ret::default());
        e.register(0x0090_0020, |_, _| 0x6000u32.into_ret());
        e.register(0x0089_1170, |_, _| 0x7000u32.into_ret());
        e.register(0x0040_37f0, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"text".to_vec());
            Ret::default()
        });
        e.register(0x005a_e3d0, |e, a| {
            let block = e.mem.u32(a[1]);
            assert_eq!(e.mem.u32(block + 8), 3);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 0x0200_0000);
        e.mem.set_u32(this + 0xc, 0x0001_2345);
        let file = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_68d0, &args![this, file]).bool());
        assert_eq!(e.mem.u32(this + 0x34), 0x44);
        let log = e.call_log.take().unwrap();
        let of = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(TES_FORM_LOAD_FORM), vec![vec![this, file]]);
        assert_eq!(of(TES_FORM_MARK_00484AB0), vec![vec![this, 0]]);
        assert_eq!(of(0x0090_0010), vec![vec![this, file]]);
        assert_eq!(of(0x0048_e3d0), vec![vec![this + 0x18, file]]);
        assert_eq!(of(0x0040_37f0)[0][0], this + 0x38);
        assert_eq!(of(0x0040_37f0)[0][2], 0);
        assert_eq!(of(0x005a_e3d0)[0][0], this + 0x2c);
        assert_eq!(
            of(0x005b_5e40),
            vec![vec![OLD_DATA_WARNING, 0x7000, 0x6000, 0x0001_2345]]
        );
        // The EDID read is capped at 0x200 bytes.
        let reads = of(GET_CHUNK_DATA);
        assert_eq!(reads[0][2], 0x200);
    }

    #[test]
    fn load_swaps_the_location_when_the_file_needs_it() {
        let chunks = vec![(CHUNK_LNAM, vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0])];
        let mut e = load_engine(chunks);
        e.register(NEEDS_BYTE_SWAP, |_, _| 1u32.into_ret());
        e.register(SWAP_WORD_IN_PLACE, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let file = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_68d0, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == SWAP_WORD_IN_PLACE).count(), 3);
    }

    /// A form: +0 vtable (slot 0x1F4 returns +0x1C), +4 interior flag, +8
    /// parent cell, +0xC id, +0x10 world space, +0x14/+0x18 grid x/y,
    /// +0x1C position, +0x20 type.
    fn form(e: &mut Engine, words: [u32; 8]) -> u32 {
        let p = e.mem.alloc(0x24);
        e.mem.set_u32(p, 0x0200_0000);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(p + 4 + 4 * i as u32, *w);
        }
        p
    }

    #[test]
    fn can_display_for_form_matches_the_entries() {
        let mut e = list_engine();
        e.register(0x0040_1170, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(0x0042_5fd0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x008d_6f30, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0054_ddd0, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0057_5d70, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(0x0054_4c30, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(0x0054_4c60, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(0x0058_7410, |_, a| (a[0] * 100 + a[1]).into_ret());
        e.register(0x0058_7440, |e, a| e.mem.u32(a[0]).into_ret());
        e.put_vtable(0x0200_0000, &[0; 0x80]);
        e.mem.set_u32(0x0200_0000 + 0x1f4, 0x0090_0000);
        e.register(0x0090_0000, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        let (this, _) = screen_with(&mut e, &[[0x0a, 0, 0], [0, 0x0b, 304]]);
        let run = |e: &mut Engine, f: u32| e.call(0x005a_6ec0, &args![this, f]).bool();
        // An interior cell with id 0x0A matches the first entry.
        let interior = form(&mut e, [1, 0, 0x0a, 0, 0, 0, 0, 0x39]);
        assert!(run(&mut e, interior));
        let other = form(&mut e, [1, 0, 0x0c, 0, 0, 0, 0, 0x39]);
        assert!(!run(&mut e, other));
        // An exterior cell of world 0x0B at (3, 4): key 304.
        let world = form(&mut e, [0, 0, 0x0b, 0, 0, 0, 0, 0]);
        let exterior = form(&mut e, [0, 0, 0x99, world, 3, 4, 0, 0x39]);
        assert!(run(&mut e, exterior));
        let elsewhere = form(&mut e, [0, 0, 0x99, world, 3, 5, 0, 0x39]);
        assert!(!run(&mut e, elsewhere));
        let nowhere = form(&mut e, [0, 0, 0x99, 0, 3, 4, 0, 0x39]);
        assert!(!run(&mut e, nowhere));
        // References: inside the interior cell, or outside at a position
        // whose key is 304 (or 305: no match).
        for kind in [0x3a, 0x40, 0x69] {
            let inside = form(&mut e, [0, interior, 0, 0, 0, 0, 0, kind]);
            assert!(run(&mut e, inside));
        }
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, 304);
        let outside = form(&mut e, [0, exterior, 0, world, 0, 0, position, 0x3b]);
        assert!(run(&mut e, outside));
        let unplaced = form(&mut e, [0, 0, 0, world, 0, 0, position, 0x3c]);
        assert!(run(&mut e, unplaced));
        e.mem.set_u32(position, 305);
        assert!(!run(&mut e, outside));
        let lost = form(&mut e, [0, 0, 0, 0, 0, 0, position, 0x3c]);
        assert!(!run(&mut e, lost));
        // Type 0x41: the form's own id.
        let world_form = form(&mut e, [0, 0, 0x0a, 0, 0, 0, 0, 0x41]);
        assert!(run(&mut e, world_form));
        // Any other type never matches.
        for kind in [0x38, 0x42, 0x68, 0x6a] {
            let unknown = form(&mut e, [1, 0, 0x0a, 0, 0, 0, 0, kind]);
            assert!(!run(&mut e, unknown));
        }
    }
}

#[cfg(test)]
mod save_copy_compare_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NODE_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(LIST_COUNT, |e, a| {
            let (mut node, mut n) = (a[0], 0);
            while node != 0 {
                n += u32::from(e.mem.u32(node) != 0);
                node = e.mem.u32(node + 4);
            }
            ret(n)
        });
        e.register(LIST_HOLDS, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == item {
                    return ret(1);
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                node = e.mem.u32(node + 4);
            }
            let new = e.mem.alloc(8);
            e.mem.set_u32(new, item);
            e.mem.set_u32(node + 4, new);
            Ret::default()
        });
        e.register(LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(WORD_34, |e, a| ret(e.mem.u32(a[0] + 0x34)));
        e.register(SET_WORD_34, |e, a| {
            e.mem.set_u32(a[0] + 0x34, a[1]);
            Ret::default()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        e
    }

    fn screen(e: &mut Engine, items: &[u32], word: u32) -> u32 {
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x34, word);
        let mut node = this + 0x2c;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        this
    }

    #[test]
    fn save_starts_and_closes_the_form() {
        let mut e = Engine::new();
        e.register(START_FORM, |_, _| Ret::default());
        e.register(CLOSE_FORM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005a_68b0, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(START_FORM, vec![0x4000]), (CLOSE_FORM, vec![0x4000])]
        );
    }

    #[test]
    fn copy_then_compare() {
        let mut e = engine();
        let other = screen(&mut e, &[0x61, 0x62], 0x9);
        let this = screen(&mut e, &[0x70], 0);
        assert!(e.call(0x005a_6d30, &args![this, other]).bool());
        e.call_log = Some(vec![]);
        e.call(0x005a_6c70, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, OPERATOR_DELETE), vec![vec![0x70]]);
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(e.mem.u32(this + 0x34), 9);
        assert!(!e.call(0x005a_6d30, &args![this, other]).bool());
        e.mem.set_u32(this + 0x34, 8);
        assert!(e.call(0x005a_6d30, &args![this, other]).bool());
        e.mem.set_u32(this + 0x34, 9);
        let reordered = screen(&mut e, &[0x62, 0x61], 9);
        assert!(!e.call(0x005a_6d30, &args![reordered, other]).bool());
        let different = screen(&mut e, &[0x62, 0x63], 9);
        assert!(e.call(0x005a_6d30, &args![different, other]).bool());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x005a_6d30, &args![this, other]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x005a_6d30, &args![this, other]).bool());
        let fresh = screen(&mut e, &[0x70], 1);
        e.call(0x005a_6c70, &args![fresh, other]);
        assert_eq!(e.mem.u32(fresh + 0x34), 1);
    }
}
