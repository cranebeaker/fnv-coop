//! `fallout shared/run_005fc220` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fc680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc680 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 8 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005fc680(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_ba00, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0104_98dc);
    e.mem.set_u32(base + 0x18, 0x0104_98c8);
    e.mem.set_u32(base + 0x24, 0x0104_98b4);
    e.call(0x0096_a2d0, &args![base + 0x3c]);
    e.call(0x005f_c800, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x8]);
    this
}

// Translated from 005fc840 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `005fc860`, which walks a list and deletes its
/// items, and returns nothing. Parameters: one word, `this`; the disassembly of the function
/// ends in a plain `RET` and passes only `ECX` on. `005fc860`'s disassembly is a loop that
/// ends on its exit test without loading a result (Ghidra types it void).
pub fn fn_005fc840(e: &mut Engine, this: Ptr) {
    e.call(0x005f_c860, &args![this]);
}

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`): the type of the current
/// record.
const FILE_GET_FORM_TYPE: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`).
const FILE_GET_CHUNK: u32 = 0x0047_26b0;
/// `004726f0` (`ECX` = file): moves to the next chunk, false at the end.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00401660` (`ECX` = file): the current chunk's size.
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`ECX` = file): the flag tested before a byte-order swap.
const FILE_SWAPS_BYTES: u32 = 0x0040_1680;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`).
const FILE_GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`): the destination and
/// the most bytes to read.
const FILE_GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `TESForm::LoadData` (Xbox PDB, `004861f0`): the file, the destination,
/// its size.
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `00403570` (`ECX` = file): the 16-bit value `TESFaction`'s `Load`
/// compares with 9 before reading `DATA`.
const FILE_FORM_VERSION: u32 = 0x0040_3570;
/// `00487050` (`cdecl`: the `TESFullName`, the file).
const FULL_NAME_LOAD: u32 = 0x0048_7050;
/// `TESTexture::LoadTextureChunk` (Xbox PDB, `0048e3d0`, `cdecl`).
const TEXTURE_LOAD_CHUNK: u32 = 0x0048_e3d0;
/// `004037f0` (`ECX` = `BSStringT`, the text, a word): sets the string.
const STRING_SET: u32 = 0x0040_37f0;
/// `00401000` / `00401030` (`cdecl`): the game's allocation and free.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `005a8080` (`ECX` = faction): the address of the rank list (`this +
/// 0x3C`, `m_rankDataList`, Xbox PDB).
const RANK_LIST: u32 = 0x005a_8080;
/// `006815c0` (`ECX` = node): the address of the node's item.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `0063f7b0` (`ECX` = node): removes the node's item.
const NODE_REMOVE: u32 = 0x0063_f7b0;
/// `00905820` (`ECX` = list, the address of an item): adds the item.
const LIST_ADD: u32 = 0x0090_5820;
/// `008256d0` (`ECX` = list): true when the list is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;

const SIG_EDID: u32 = 0x4449_4445;
const SIG_OBND: u32 = 0x444e_424f;
const SIG_FULL: u32 = 0x4c4c_5546;
const SIG_DATA: u32 = 0x4154_4144;
const SIG_ICON: u32 = 0x4e4f_4349;
const SIG_FNAM: u32 = 0x4d41_4e46;
const SIG_INAM: u32 = 0x4d41_4e49;
const SIG_MNAM: u32 = 0x4d41_4e4d;
const SIG_RNAM: u32 = 0x4d41_4e52;
const SIG_XNAM: u32 = 0x4d41_4e58;
const SIG_WMI1: u32 = 0x3149_4d57;

/// An `EDID`-style chunk: a stack buffer of the chunk's size (the
/// `_alloca`), read with `limit`, handed to `then`.
fn with_chunk_buffer(e: &mut Engine, file: Ptr, limit: u32, then: impl FnOnce(&mut Engine, Ptr)) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    e.with_stack(size, |e, buffer| {
        e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, buffer, limit]);
        then(e, buffer);
    });
}

// Translated from 005fc220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc220 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from _RtlCheckStack12 (00ec5ec0, the _alloca) is a with_stack block of the chunk size (its alignment, and the stack growing with each EDID chunk, are not modelled)
/// `TESEyes`' `Load` (vtable slot 0x20): false unless the record is type
/// 0xB. Otherwise `TESForm::LoadForm`, then every chunk: `EDID` (a stack
/// buffer of the chunk's size, at most 0x200 bytes, to vtable slot 0x134),
/// `OBND` (vtable slot 0xE0), `FULL` (the `TESFullName` at +0x18, null for
/// a null `this`), `ICON` (the `TESTexture` at +0x24, likewise) and `DATA`
/// (one byte into the flags at +0x30). Returns true.
pub fn fn_005fc220(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(FILE_GET_FORM_TYPE, &args![file]).u32() != 0xb {
        return false;
    }
    let base = this.addr();
    e.call(FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_EDID => with_chunk_buffer(e, file, 0x200, |e, buffer| {
                e.vcall(base, 0x134, &args![buffer]);
            }),
            SIG_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SIG_FULL => {
                let full_name = if this.is_null() { 0 } else { base + 0x18 };
                e.call(FULL_NAME_LOAD, &args![full_name, file]);
            }
            SIG_ICON => {
                let texture = if this.is_null() { 0 } else { base + 0x24 };
                e.call(TEXTURE_LOAD_CHUNK, &args![texture, file]);
            }
            SIG_DATA => {
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, base + 0x30, 1u32]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005fc530 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x2 of `TESEyes`' flags byte (+0x30) is clear (the
/// variant `005fc5f0` asks for sex 0).
pub fn fn_005fc530(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr() + 0x30)) & 2 == 0
}

// Translated from 005fc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x4 of `TESEyes`' flags byte (+0x30) is clear (the
/// variant `005fc5f0` asks for sex 1).
pub fn fn_005fc590(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr() + 0x30)) & 4 == 0
}

// Translated from 005fc5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether these eyes suit `npc`: false for a null `npc` or when
/// `004ac110(npc)` is null; otherwise `005f65d0` on `00503650` of that
/// (with the address of a stack copy of `this`) must be true, and then
/// `TESActorBase::GetSex` (`005f0cc0`) picks `005fc530` (sex 0) or
/// `005fc590` (sex 1); any other sex gives false.
pub fn fn_005fc5f0(e: &mut Engine, this: Ptr, npc: Ptr) -> bool {
    if npc.is_null() || e.call(0x004a_c110, &args![npc]).u32() == 0 {
        return false;
    }
    let listed = e.with_stack(4, |e, eyes_at| {
        e.mem.set_u32(eyes_at.addr(), this.addr());
        let part = e.call(0x004a_c110, &args![npc]).u32();
        let race = e.call(0x0050_3650, &args![part]).u32();
        e.call(0x005f_65d0, &args![race, eyes_at]).bool()
    });
    if !listed {
        return false;
    }
    match e.call(0x005f_0cc0, &args![npc]).u32() {
        0 => fn_005fc530(e, this),
        1 => fn_005fc590(e, this),
        _ => false,
    }
}

// Translated from 005fc760 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc760 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESFaction`'s destructor body: stores its three vtables, frees the
/// ranks (`005fc840`), then destroys the rank list (`0046ffb0`), the
/// `TESReactionForm` at +0x24 (`0048ba70`), the `TESFullName` at +0x18
/// (`00591300`) and the `TESForm` base (`TESForm::~TESForm`, `00483630`).
pub fn fn_005fc760(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_98dc);
    e.mem.set_u32(base + 0x18, 0x0104_98c8);
    e.mem.set_u32(base + 0x24, 0x0104_98b4);
    e.call(0x005f_c840, &args![this]);
    e.call(0x0046_ffb0, &args![base + 0x3c]);
    e.call(0x0048_ba70, &args![base + 0x24]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005fc730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFaction`'s scalar deleting destructor: the destructor body
/// (`005fc760`), then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_005fc730(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005fc760(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 005fc800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFaction`'s start values: `iMajorCrime` (+0x44), `iMinorCrime`
/// (+0x48) and `pReputation` (+0x38) 0, and the 4-byte `FACTION_DATA` at
/// +0x34 cleared (`00403d30` with 0, 4; Xbox PDB names, PC offsets).
pub fn fn_005fc800(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x38, 0);
    e.call(0x0040_3d30, &args![base + 0x34, 0u32, 4u32]);
}

// Translated from 005fc860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the faction's ranks: until the rank list is empty (`008256d0`),
/// deletes the first rank (`005fc8d0` with 1, when not null) and removes
/// it from the list (`0063f7b0`). The list's address comes from
/// `005a8080` each time.
pub fn fn_005fc860(e: &mut Engine, this: Ptr) {
    loop {
        let list = e.call(RANK_LIST, &args![this]).u32();
        if e.call(LIST_IS_EMPTY, &args![list]).bool() {
            break;
        }
        let list = e.call(RANK_LIST, &args![this]).u32();
        let at = e.call(NODE_ITEM, &args![list]).u32();
        let rank = e.mem.u32(at);
        if rank != 0 {
            fn_005fc8d0(e, Ptr::new(rank), 1);
        }
        let list = e.call(RANK_LIST, &args![this]).u32();
        e.call(NODE_REMOVE, &args![list]);
    }
}

// Translated from 005fc8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A faction rank's scalar deleting destructor: the destructor body
/// (`005fc900`), then frees `this` (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_005fc8d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005fc900(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 005fc900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fc900 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// A faction rank's destructor body (0x1C bytes: two `BSStringT`s, the
/// male and female titles, and a `TESTexture`, the insignia): destroys the
/// texture (`0048e2e0`), then the strings at +8 and +0 (`004037d0`).
pub fn fn_005fc900(e: &mut Engine, this: Ptr) {
    e.call(0x0048_e2e0, &args![this.addr() + 0x10]);
    e.call(0x0040_37d0, &args![this.addr() + 8]);
    e.call(0x0040_37d0, &args![this]);
}

// Translated from 005fcf10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fcf10 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// A faction rank's constructor: the two strings (`004037b0` on +0 and +8)
/// and the texture (`0048e270` on +0x10). Returns `this`.
pub fn fn_005fcf10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_37b0, &args![this]);
    e.call(0x0040_37b0, &args![this.addr() + 8]);
    e.call(0x0048_e270, &args![this.addr() + 0x10]);
    this
}

// Translated from 005fcb70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fcb70 - the exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated; the chunk buffers from _RtlCheckStack12 (00ec5ec0, the _alloca) are with_stack blocks of the chunk size (their alignment, and the stack growing with each such chunk, are not modelled)
/// `TESFaction`'s `Load` (vtable slot 0x20): `TESForm::LoadForm` (no
/// record type check), then every chunk (the first one read without a
/// test for 0):
/// - `EDID`, `OBND`, `FULL` (+0x18) as every form.
/// - `XNAM`: a new 0xC-byte reaction (`00401000`, not checked for null),
///   0xC bytes read into it, swapped (`005a6b10`) when the file swaps, and
///   added (`00905820`) to the reaction list `00717e50` gives for the
///   `TESReactionForm` at +0x24.
/// - `DATA`: when `00403570(file)` is below 9, one signed byte (`LoadData`)
///   sign-extended into +0x34; otherwise 4 bytes into +0x34, swapped
///   (`00503210`) when the file swaps.
/// - `RNAM`: a new rank (0x1C bytes, constructed by `005fcf10`, null when
///   the allocation fails) becomes the current rank and is added to the
///   rank list (`005a8080`, `00905820`); the chunk's data is not read.
/// - `MNAM` / `FNAM`: the current rank's male (+0) / female (+8) title
///   (a stack buffer of the chunk's size, read with limit 0); `INAM`: its
///   insignia (`00489100` on +0x10 with the buffer). Each only with a
///   current rank.
/// - `WMI1`: a form id; a nonzero one is stored as `pReputation` (+0x38).
///
/// Returns true. The compiler's switch on `chunk - 'INAM'` (a byte table
/// over 0..0xF and four cases) is the match below.
pub fn fn_005fcb70(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let mut rank = 0u32;
    e.call(FORM_LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(FILE_GET_CHUNK, &args![file]).u32();
        match chunk {
            SIG_EDID => with_chunk_buffer(e, file, 0x200, |e, buffer| {
                e.vcall(base, 0x134, &args![buffer]);
            }),
            SIG_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SIG_FULL => {
                let full_name = if this.is_null() { 0 } else { base + 0x18 };
                e.call(FULL_NAME_LOAD, &args![full_name, file]);
            }
            SIG_XNAM => {
                let reaction = e.call(ALLOCATE, &args![0xcu32]).u32();
                e.call(FILE_GET_CHUNK_DATA_SIZED, &args![file, reaction, 0xcu32]);
                if e.call(FILE_SWAPS_BYTES, &args![file]).bool() {
                    e.call(0x005a_6b10, &args![reaction]);
                }
                e.with_stack(4, |e, item_at| {
                    e.mem.set_u32(item_at.addr(), reaction);
                    let list = e.call(0x0071_7e50, &args![base + 0x24]).u32();
                    e.call(LIST_ADD, &args![list, item_at]);
                });
            }
            SIG_DATA => {
                let version = e.call(FILE_FORM_VERSION, &args![file]).u16();
                if version < 9 {
                    let flags = e.with_stack(4, |e, byte_at| {
                        e.call(FORM_LOAD_DATA, &args![this, file, byte_at, 1u32]);
                        e.mem.i8(byte_at.addr())
                    });
                    e.mem.set_i32(base + 0x34, i32::from(flags));
                } else {
                    e.call(FORM_LOAD_DATA, &args![this, file, base + 0x34, 4u32]);
                    if e.call(FILE_SWAPS_BYTES, &args![file]).bool() {
                        e.call(0x0050_3210, &args![base + 0x34]);
                    }
                }
            }
            SIG_RNAM => {
                let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
                rank = if block != 0 {
                    fn_005fcf10(e, Ptr::new(block)).addr()
                } else {
                    0
                };
                let current = rank;
                e.with_stack(4, |e, rank_at| {
                    e.mem.set_u32(rank_at.addr(), current);
                    let list = e.call(RANK_LIST, &args![this]).u32();
                    e.call(LIST_ADD, &args![list, rank_at]);
                });
            }
            SIG_MNAM => {
                if rank != 0 {
                    with_chunk_buffer(e, file, 0, |e, buffer| {
                        e.call(STRING_SET, &args![rank, buffer, 0u32]);
                    });
                }
            }
            SIG_FNAM => {
                if rank != 0 {
                    with_chunk_buffer(e, file, 0, |e, buffer| {
                        e.call(STRING_SET, &args![rank + 8, buffer, 0u32]);
                    });
                }
            }
            SIG_INAM => {
                if rank != 0 {
                    with_chunk_buffer(e, file, 0, |e, buffer| {
                        e.call(0x0048_9100, &args![rank + 0x10, buffer]);
                    });
                }
            }
            SIG_WMI1 => {
                let id = e.with_stack(4, |e, id_at| {
                    e.call(FILE_GET_CHUNK_DATA, &args![file, id_at]);
                    e.mem.u32(id_at.addr())
                });
                if id != 0 {
                    e.mem.set_u32(base + 0x38, id);
                }
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl), the source type descriptor and
/// the target `005fc3a0` and `005fc420` pass (`01186388`, the type
/// descriptor named `.?AVTESEyes@@`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE: u32 = 0x0118_3028;
const EYES_TYPE: u32 = 0x0118_6388;
/// The chunk writers (cdecl): a word (`00485910`) and an array
/// (`TESForm::AddChunkArray`, `004856f0`: chunk, data, size).
const WRITE_WORD_CHUNK: u32 = 0x0048_5910;
const WRITE_ARRAY_CHUNK: u32 = 0x0048_56f0;
/// One of `TESEyes`' flag getters.
type FlagGetter = fn(&mut Engine, Ptr) -> bool;

// Translated from 005fc3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEyes`' copy (`RET 4`: the source form; the engine map has no
/// name): the source through `__RTDynamicCast(source, 0, 01183028,
/// 01186388, 0)`; nothing without it. Otherwise
/// `TESForm::CopyAllComponents` given the uncast source, then the three
/// flags through their getters and setters ([`fn_005fc4d0`] /
/// [`fn_005fc4f0`], [`fn_005fc530`] / [`fn_005fc550`], [`fn_005fc590`] /
/// [`fn_005fc5b0`]).
pub fn fn_005fc3a0(e: &mut Engine, this: Ptr, source: u32) {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![source, 0u32, CAST_SOURCE, EYES_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, source]);
    let flag = fn_005fc4d0(e, Ptr::new(other));
    fn_005fc4f0(e, this, u8::from(flag));
    let flag = fn_005fc530(e, Ptr::new(other));
    fn_005fc550(e, this, u8::from(flag));
    let flag = fn_005fc590(e, Ptr::new(other));
    fn_005fc5b0(e, this, u8::from(flag));
}

// Translated from 005fc420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEyes`' compare (`RET 4`: the other form; true when they differ;
/// the engine map has no name): true when the cast as in `005fc3a0` fails,
/// `TESForm::CompareAllComponents` answers true, or [`fn_005fc4d0`],
/// [`fn_005fc530`] or [`fn_005fc590`] answer differently for the two.
pub fn fn_005fc420(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE, EYES_TYPE, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, other]).bool() {
        return true;
    }
    let getters: [FlagGetter; 3] = [fn_005fc4d0, fn_005fc530, fn_005fc590];
    for getter in getters {
        let theirs = getter(e, Ptr::new(other));
        if theirs != getter(e, this) {
            return true;
        }
    }
    false
}

// Translated from 005fc4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x1 of `TESEyes`' flags byte (+0x30) is set.
pub fn fn_005fc4d0(e: &mut Engine, this: Ptr) -> bool {
    i32::from(e.mem.i8(this.addr() + 0x30)) & 1 != 0
}

/// Sets (`set`) or clears `bit` in the flags byte at +0x30.
fn set_eyes_bit(e: &mut Engine, this: Ptr, bit: u8, set: bool) {
    let at = this.addr() + 0x30;
    let flags = e.mem.u8(at);
    e.mem
        .set_u8(at, if set { flags | bit } else { flags & !bit });
}

// Translated from 005fc4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bit 0x1 of `TESEyes`' flags byte (+0x30) when the byte `value` is
/// non-zero, clears it otherwise (`RET 4`).
pub fn fn_005fc4f0(e: &mut Engine, this: Ptr, value: u8) {
    set_eyes_bit(e, this, 1, value != 0);
}

// Translated from 005fc550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears bit 0x2 of `TESEyes`' flags byte (+0x30) when the byte `value`
/// is non-zero, sets it otherwise (`RET 4`; the counterpart of
/// [`fn_005fc530`]).
pub fn fn_005fc550(e: &mut Engine, this: Ptr, value: u8) {
    set_eyes_bit(e, this, 2, value == 0);
}

// Translated from 005fc5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears bit 0x4 of `TESEyes`' flags byte (+0x30) when the byte `value`
/// is non-zero, sets it otherwise (`RET 4`; the counterpart of
/// [`fn_005fc590`]).
pub fn fn_005fc5b0(e: &mut Engine, this: Ptr, value: u8) {
    set_eyes_bit(e, this, 4, value == 0);
}

// Translated from 005fc970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (the byte `set` non-zero) or clears the bits `bits` in the
/// faction's flags word at +0x34 (`RET 8`; the engine map has no name),
/// then calls vtable slot 0x48 with 2.
pub fn fn_005fc970(e: &mut Engine, this: Ptr, bits: u32, set: u8) {
    let at = this.addr() + 0x34;
    let flags = e.mem.u32(at);
    if set != 0 {
        e.mem.set_u32(at, flags | bits);
    } else {
        e.mem.set_u32(at, !bits & flags);
    }
    e.vcall(this.addr(), 0x48, &args![2u32]);
}

// Translated from 005fc9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The faction's record writer (the layout `005fc680` builds; no stack
/// words; the engine map has no name): `TESForm::StartForm`,
/// `TESFullName::Save` (+0x18), `0048bb70` (+0x24), the flags word (+0x34)
/// as `TESForm::__SaveData(+0x34, 4)` with `00503210` on it before and
/// after when `00401500` answers true; then for each rank of the list
/// (`005a8080`, up to an empty node) its index as `RNAM`, the texts at +0
/// and +8 (`004048e0` the length, `00559450` the text; each with its
/// terminator) as `MNAM` and `FNAM` and the one at +0x10 (`0048cee0`,
/// `00408da0`) as `INAM` when not empty; the form id of +0x38 as `WMI1`
/// when set; then `TESForm::CloseForm`.
pub fn fn_005fc9d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0048_7010, &args![base + 0x18]);
    e.call(0x0048_bb70, &args![base + 0x24]);
    if e.call(0x0040_1500, &args![]).bool() {
        e.call(0x0050_3210, &args![base + 0x34]);
    }
    e.call(0x0048_5e20, &args![this, base + 0x34, 4u32]);
    if e.call(0x0040_1500, &args![]).bool() {
        e.call(0x0050_3210, &args![base + 0x34]);
    }
    let mut index = 0u32;
    let mut node = e.call(RANK_LIST, &args![this]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(NODE_ITEM, &args![node]).u32();
        let rank = e.mem.u32(slot);
        e.call(WRITE_WORD_CHUNK, &args![SIG_RNAM, index]);
        for (offset, chunk) in [(0u32, SIG_MNAM), (8, SIG_FNAM)] {
            if e.call(0x0040_48e0, &args![rank + offset]).u32() != 0 {
                let size = e
                    .call(0x0040_48e0, &args![rank + offset])
                    .u32()
                    .wrapping_add(1);
                let text = e.call(0x0055_9450, &args![rank + offset]).u32();
                e.call(WRITE_ARRAY_CHUNK, &args![chunk, text, size]);
            }
        }
        if e.call(0x0048_cee0, &args![rank + 0x10]).u32() != 0 {
            let size = e
                .call(0x0048_cee0, &args![rank + 0x10])
                .u32()
                .wrapping_add(1);
            let text = e.call(0x0040_8da0, &args![rank + 0x10]).u32();
            e.call(WRITE_ARRAY_CHUNK, &args![SIG_INAM, text, size]);
        }
        index = index.wrapping_add(1);
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    let reputation = e.mem.u32(base + 0x38);
    if reputation != 0 {
        let id = e.call(0x0084_e3a0, &args![reputation]).u32();
        e.call(WRITE_WORD_CHUNK, &args![SIG_WMI1, id]);
    }
    e.call(0x0048_5680, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_c3a0, fn_005fc3a0(Ptr, u32)),
        entry!(0x005f_c420, fn_005fc420(Ptr, u32) -> bool),
        entry!(0x005f_c4d0, fn_005fc4d0(Ptr) -> bool),
        entry!(0x005f_c4f0, fn_005fc4f0(Ptr, u8)),
        entry!(0x005f_c550, fn_005fc550(Ptr, u8)),
        entry!(0x005f_c5b0, fn_005fc5b0(Ptr, u8)),
        entry!(0x005f_c970, fn_005fc970(Ptr, u32, u8)),
        entry!(0x005f_c9d0, fn_005fc9d0(Ptr)),
        entry!(0x005f_c840, fn_005fc840(Ptr)),
        entry!(0x005f_c680, fn_005fc680(Ptr) -> Ptr),
        entry!(0x005f_c220, fn_005fc220(Ptr, Ptr) -> bool),
        entry!(0x005f_c530, fn_005fc530(Ptr) -> bool),
        entry!(0x005f_c590, fn_005fc590(Ptr) -> bool),
        entry!(0x005f_c5f0, fn_005fc5f0(Ptr, Ptr) -> bool),
        entry!(0x005f_c760, fn_005fc760(Ptr)),
        entry!(0x005f_c730, fn_005fc730(Ptr, u32) -> Ptr),
        entry!(0x005f_c800, fn_005fc800(Ptr)),
        entry!(0x005f_c860, fn_005fc860(Ptr)),
        entry!(0x005f_c8d0, fn_005fc8d0(Ptr, u32) -> Ptr),
        entry!(0x005f_c900, fn_005fc900(Ptr)),
        entry!(0x005f_cf10, fn_005fcf10(Ptr) -> Ptr),
        entry!(0x005f_cb70, fn_005fcb70(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005fc680_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_c680,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_ba00, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x3c)]),
                (0x005f_c800, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x8)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_98dc),
                (0x18, 4, 0x0104_98c8),
                (0x24, 4, 0x0104_98b4),
            ],
        });
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_005fc840_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x005f_c860, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_c840, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x005f_c860), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
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

    fn nop(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    /// A file that hands out `chunks` in order (next-chunk false after the
    /// last), every chunk 0x10 bytes; a form at a fresh block whose vtable
    /// sends slots 0x134 and 0xE0 to `00001134` / `000010e0`.
    fn loading(chunks: Vec<u32>) -> (Engine, Ptr) {
        let mut e = Engine::new();
        let queue = Rc::new(RefCell::new(chunks));
        let q = queue.clone();
        e.register_double(FILE_GET_CHUNK, move |_, _| ret(q.borrow()[0]));
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut v = queue.borrow_mut();
            v.remove(0);
            ret(u32::from(!v.is_empty()))
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(0x10));
        nop(
            &mut e,
            &[
                FORM_LOAD_FORM,
                FILE_GET_CHUNK_DATA_SIZED,
                FULL_NAME_LOAD,
                TEXTURE_LOAD_CHUNK,
                STRING_SET,
                LIST_ADD,
                FORM_LOAD_DATA,
                0x0000_1134,
                0x0000_10e0,
            ],
        );
        e.put_vtable(0x0150_0000, &[0; 0x50]);
        e.mem.set_u32(0x0150_0134, 0x0000_1134);
        e.mem.set_u32(0x0150_00e0, 0x0000_10e0);
        let p = e.mem.alloc(0x4c);
        e.mem.set_u32(p, 0x0150_0000);
        (e, Ptr::new(p))
    }

    #[test]
    fn b020_005fc220_loads_the_eyes() {
        let (mut e, p) = loading(vec![SIG_EDID, SIG_OBND, SIG_FULL, SIG_ICON, SIG_DATA, 7]);
        let b = p.addr();
        e.register(FILE_GET_FORM_TYPE, |_, _| ret(0xb));
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_c220, &args![p, 0x9000u32]).bool());
        let sized = calls_to(&e, FILE_GET_CHUNK_DATA_SIZED);
        assert_eq!(sized.len(), 2);
        assert_eq!(sized[0][2], 0x200);
        assert_eq!(sized[1], vec![0x9000, b + 0x30, 1]);
        assert_eq!(calls_to(&e, 0x0000_1134), vec![vec![b, sized[0][1]]]);
        assert_eq!(calls_to(&e, 0x0000_10e0), vec![vec![b, 0x9000]]);
        assert_eq!(calls_to(&e, FULL_NAME_LOAD), vec![vec![b + 0x18, 0x9000]]);
        assert_eq!(
            calls_to(&e, TEXTURE_LOAD_CHUNK),
            vec![vec![b + 0x24, 0x9000]]
        );
        e.register(FILE_GET_FORM_TYPE, |_, _| ret(0x10b));
        assert!(!e.call(0x005f_c220, &args![p, 0x9000u32]).bool());
    }

    #[test]
    fn b020_005fc530_and_005fc590_test_the_flags() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x34);
        for (flags, first, second) in [
            (0u8, true, true),
            (2, false, true),
            (4, true, false),
            (0xfe, false, false),
        ] {
            e.mem.set_u8(p + 0x30, flags);
            assert_eq!(e.call(0x005f_c530, &args![p]).bool(), first);
            assert_eq!(e.call(0x005f_c590, &args![p]).bool(), second);
        }
    }

    #[test]
    fn b020_005fc5f0_checks_race_and_sex() {
        let mut e = Engine::new();
        e.register(0x004a_c110, |_, a| {
            ret(if a[0] == 0x6666 { 0 } else { a[0] + 1 })
        });
        e.register(0x0050_3650, |_, a| ret(a[0] + 2));
        e.register(0x005f_65d0, |e, a| {
            let eyes = e.mem.u32(a[1]);
            ret(u32::from(a[0] == 0x7003 && eyes != 0))
        });
        e.register(0x005f_0cc0, |e, a| ret(e.mem.u32(a[0])));
        let eyes = e.mem.alloc(0x34);
        e.mem.map(0x7000, 0x10);
        assert!(!e.call(0x005f_c5f0, &args![eyes, 0u32]).bool());
        e.mem.map(0x6666, 4);
        assert!(!e.call(0x005f_c5f0, &args![eyes, 0x6666u32]).bool());
        e.mem.set_u32(0x7000, 0);
        assert!(e.call(0x005f_c5f0, &args![eyes, 0x7000u32]).bool());
        e.mem.set_u8(eyes + 0x30, 2);
        assert!(!e.call(0x005f_c5f0, &args![eyes, 0x7000u32]).bool());
        e.mem.set_u32(0x7000, 1);
        assert!(e.call(0x005f_c5f0, &args![eyes, 0x7000u32]).bool());
        e.mem.set_u32(0x7000, 2);
        assert!(!e.call(0x005f_c5f0, &args![eyes, 0x7000u32]).bool());
        e.mem.set_u32(0x7000, 0);
        e.mem.set_u8(eyes + 0x30, 0);
        e.register(0x005f_65d0, |_, _| ret(0));
        assert!(!e.call(0x005f_c5f0, &args![eyes, 0x7000u32]).bool());
    }

    #[test]
    fn b020_005fc760_and_005fc730_destroy_the_faction() {
        let mut e = Engine::new();
        let callees = [
            0x005f_c840,
            0x0046_ffb0,
            0x0048_ba70,
            0x0059_1300,
            0x0048_3630,
            FREE,
        ];
        nop(&mut e, &callees);
        let p = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x005f_c760, &args![p]);
        assert_eq!(e.mem.u32(p), 0x0104_98dc);
        assert_eq!(e.mem.u32(p + 0x18), 0x0104_98c8);
        assert_eq!(e.mem.u32(p + 0x24), 0x0104_98b4);
        let body = vec![
            (0x005f_c840, vec![p]),
            (0x0046_ffb0, vec![p + 0x3c]),
            (0x0048_ba70, vec![p + 0x24]),
            (0x0059_1300, vec![p + 0x18]),
            (0x0048_3630, vec![p]),
        ];
        assert_eq!(calls(&mut e, 0x005f_c760), body);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_c730, &args![p, 1u32]).u32(), p);
        let mut freed = body.clone();
        freed.push((FREE, vec![p]));
        assert_eq!(calls(&mut e, 0x005f_c730), freed);
        e.call_log = Some(vec![]);
        e.call(0x005f_c730, &args![p, 0u32]);
        assert_eq!(calls(&mut e, 0x005f_c730), body);
    }

    #[test]
    fn b020_005fc800_sets_the_start_values() {
        let mut e = Engine::new();
        nop(&mut e, &[0x0040_3d30]);
        let p = e.mem.alloc(0x4c);
        for o in [0x38, 0x44, 0x48] {
            e.mem.set_u32(p + o, 9);
        }
        e.call_log = Some(vec![]);
        e.call(0x005f_c800, &args![p]);
        for o in [0x38, 0x44, 0x48] {
            assert_eq!(e.mem.u32(p + o), 0);
        }
        assert_eq!(
            calls(&mut e, 0x005f_c800),
            vec![(0x0040_3d30, vec![p + 0x34, 0, 4])]
        );
    }

    #[test]
    fn b020_005fc860_frees_the_ranks() {
        let mut e = Engine::new();
        e.register(RANK_LIST, |_, a| ret(a[0] + 0x3c));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        // Removing a node's item moves the next node into it (as
        // `0063f7b0` does).
        e.register(NODE_REMOVE, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        nop(&mut e, &[0x0048_e2e0, 0x0040_37d0, FREE]);
        let p = e.mem.alloc(0x4c);
        let rank = e.mem.alloc(0x1c);
        // The list: the rank, then a null rank, then an empty node.
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(p + 0x3c, rank);
        e.mem.set_u32(p + 0x40, second);
        e.mem.set_u32(second + 4, third);
        e.call_log = Some(vec![]);
        e.call(0x005f_c860, &args![p]);
        assert_eq!(calls_to(&e, FREE), vec![vec![rank]]);
        assert_eq!(calls_to(&e, NODE_REMOVE).len(), 2);
        assert_eq!(calls_to(&e, 0x0048_e2e0), vec![vec![rank + 0x10]]);
    }

    #[test]
    fn b020_rank_construct_destroy_delete() {
        let mut e = Engine::new();
        nop(
            &mut e,
            &[0x0040_37b0, 0x0048_e270, 0x0048_e2e0, 0x0040_37d0, FREE],
        );
        let r = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_cf10, &args![r]).u32(), r);
        assert_eq!(
            calls(&mut e, 0x005f_cf10),
            vec![
                (0x0040_37b0, vec![r]),
                (0x0040_37b0, vec![r + 8]),
                (0x0048_e270, vec![r + 0x10]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x005f_c900, &args![r]);
        let body = vec![
            (0x0048_e2e0, vec![r + 0x10]),
            (0x0040_37d0, vec![r + 8]),
            (0x0040_37d0, vec![r]),
        ];
        assert_eq!(calls(&mut e, 0x005f_c900), body);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_c8d0, &args![r, 3u32]).u32(), r);
        let mut freed = body.clone();
        freed.push((FREE, vec![r]));
        assert_eq!(calls(&mut e, 0x005f_c8d0), freed);
        e.call_log = Some(vec![]);
        e.call(0x005f_c8d0, &args![r, 2u32]);
        assert_eq!(calls(&mut e, 0x005f_c8d0), body);
    }

    #[test]
    fn b020_005fcb70_loads_the_faction() {
        let (mut e, p) = loading(vec![
            SIG_MNAM,
            SIG_EDID,
            SIG_XNAM,
            SIG_DATA,
            SIG_RNAM,
            SIG_MNAM,
            SIG_FNAM,
            SIG_INAM,
            SIG_WMI1,
            SIG_DATA,
            SIG_OBND,
            SIG_FULL,
            0x1234_5678,
        ]);
        let b = p.addr();
        let blocks = Rc::new(RefCell::new(vec![0x0160_0000u32, 0x0160_0100]));
        e.register_double(ALLOCATE, move |_, _| ret(blocks.borrow_mut().remove(0)));
        e.map(0x0160_0000, 0x200);
        e.register(FILE_SWAPS_BYTES, |_, _| ret(1));
        let versions = Rc::new(RefCell::new(vec![8u32, 9]));
        e.register_double(FILE_FORM_VERSION, move |_, _| {
            ret(versions.borrow_mut().remove(0))
        });
        e.register(FORM_LOAD_DATA, |e, a| {
            if a[3] == 1 {
                e.mem.set_u8(a[2], 0xf0);
            }
            Ret::default()
        });
        e.register(FILE_GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x0000_0abc);
            Ret::default()
        });
        e.register(0x0071_7e50, |_, a| ret(a[0] + 4));
        e.register(RANK_LIST, |_, a| ret(a[0] + 0x3c));
        nop(
            &mut e,
            &[
                0x005a_6b10,
                0x0050_3210,
                0x0040_37b0,
                0x0048_e270,
                0x0048_9100,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x005f_cb70, &args![p, 0x9000u32]).bool());
        // The MNAM before any RNAM is skipped.
        let rank = 0x0160_0100u32;
        let set = calls_to(&e, STRING_SET);
        assert_eq!(set.len(), 2);
        assert_eq!((set[0][0], set[0][2]), (rank, 0));
        assert_eq!(set[1][0], rank + 8);
        assert_eq!(calls_to(&e, 0x0048_9100)[0][0], rank + 0x10);
        assert_eq!(calls_to(&e, 0x005a_6b10), vec![vec![0x0160_0000]]);
        let added = calls_to(&e, LIST_ADD);
        assert_eq!(added[0][0], b + 0x28);
        assert_eq!(added[1][0], b + 0x3c);
        let loads = calls_to(&e, FORM_LOAD_DATA);
        assert_eq!(loads[0][3], 1);
        assert_eq!(loads[1], vec![b, 0x9000, b + 0x34, 4]);
        assert_eq!(calls_to(&e, 0x0050_3210), vec![vec![b + 0x34]]);
        assert_eq!(e.mem.u32(b + 0x38), 0xabc);
        assert_eq!(calls_to(&e, FULL_NAME_LOAD), vec![vec![b + 0x18, 0x9000]]);
        assert_eq!(calls_to(&e, 0x0000_10e0), vec![vec![b, 0x9000]]);
        assert_eq!(calls_to(&e, FILE_NEXT_CHUNK).len(), 13);
    }

    #[test]
    fn b020_005fcb70_old_data_is_one_signed_byte() {
        let (mut e, p) = loading(vec![SIG_DATA]);
        e.register(FILE_FORM_VERSION, |_, _| ret(0x0001_0008));
        e.register(FORM_LOAD_DATA, |e, a| {
            e.mem.set_u8(a[2], 0xf0);
            Ret::default()
        });
        e.call(0x005f_cb70, &args![p, 0x9000u32]);
        assert_eq!(e.mem.u32(p.addr() + 0x34), 0xffff_fff0);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
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
    fn eyes_flag_getters_and_setters() {
        let mut e = Engine::new();
        let eyes = e.mem.alloc(0x34);
        e.mem.set_u8(eyes + 0x30, 0xf8);
        assert!(!e.call(0x005f_c4d0, &args![eyes]).bool());
        e.call(0x005f_c4f0, &args![eyes, 0x100u32]);
        assert_eq!(e.mem.u8(eyes + 0x30), 0xf8);
        e.call(0x005f_c4f0, &args![eyes, 1u32]);
        assert_eq!(e.mem.u8(eyes + 0x30), 0xf9);
        assert!(e.call(0x005f_c4d0, &args![eyes]).bool());
        // 005fc550 and 005fc5b0 store the opposite of their byte.
        e.call(0x005f_c550, &args![eyes, 0u32]);
        e.call(0x005f_c5b0, &args![eyes, 0u32]);
        assert_eq!(e.mem.u8(eyes + 0x30), 0xff);
        e.call(0x005f_c550, &args![eyes, 1u32]);
        assert_eq!(e.mem.u8(eyes + 0x30), 0xfd);
        e.call(0x005f_c5b0, &args![eyes, 7u32]);
        assert_eq!(e.mem.u8(eyes + 0x30), 0xf9);
    }

    #[test]
    fn eyes_copy_and_compare() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| ret(if a[0] == 1 { 0 } else { a[0] }));
        e.register(0x0048_51b0, |_, _| Ret::default());
        e.register(0x0048_5270, |_, _| ret(0));
        let this = e.mem.alloc(0x34);
        let other = e.mem.alloc(0x34);
        // Playable, not for males (bit 2 set), for females.
        e.mem.set_u8(other + 0x30, 0x03);
        e.mem.set_u8(this + 0x30, 0x04);
        assert!(e.call(0x005f_c420, &args![this, other]).bool());
        e.call_log = Some(vec![]);
        e.call(0x005f_c3a0, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, CAST_SOURCE, EYES_TYPE, 0]]
        );
        assert_eq!(calls(&e, 0x0048_51b0), vec![vec![this, other]]);
        assert_eq!(e.mem.u8(this + 0x30), 0x03);
        assert!(!e.call(0x005f_c420, &args![this, other]).bool());
        assert!(e.call(0x005f_c420, &args![this, 1u32]).bool());
        e.register(0x0048_5270, |_, _| ret(1));
        assert!(e.call(0x005f_c420, &args![this, other]).bool());
        e.call_log = Some(vec![]);
        e.call(0x005f_c3a0, &args![this, 1u32]);
        assert!(calls(&e, 0x0048_51b0).is_empty());
    }

    #[test]
    fn fn_005fc970_sets_or_clears_the_faction_flags() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x48, 0x7000_0048);
        e.register(0x7000_0048, |_, _| Ret::default());
        let faction = e.mem.alloc(0x40);
        e.mem.set_u32(faction, vtable);
        e.mem.set_u32(faction + 0x34, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x005f_c970, &args![faction, 0x3u32, 1u32]);
        assert_eq!(e.mem.u32(faction + 0x34), 0x13);
        e.call(0x005f_c970, &args![faction, 0x11u32, 0x200u32]);
        assert_eq!(e.mem.u32(faction + 0x34), 0x2);
        assert_eq!(calls(&e, 0x7000_0048), vec![vec![faction, 2]; 2]);
    }

    #[test]
    fn fn_005fc9d0_writes_the_faction_and_its_ranks() {
        let mut e = Engine::new();
        for a in [
            0x0048_55a0u32,
            0x0048_7010,
            0x0048_bb70,
            0x0050_3210,
            0x0048_5e20,
            0x0048_5680,
            WRITE_WORD_CHUNK,
            WRITE_ARRAY_CHUNK,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(0x0040_1500, |_, _| ret(1));
        // The rank list at +0x3C: nodes {item, next}.
        e.register(RANK_LIST, |_, a| ret(a[0] + 0x3c));
        e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        // Texts: the length is the word at +4, the text the word at +0.
        e.register(0x0040_48e0, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0048_cee0, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0040_8da0, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0084_e3a0, |_, a| ret(a[0] + 1));
        let faction = e.mem.alloc(0x50);
        let first = e.mem.alloc(0x18);
        e.mem.set_u32(first, 0x111);
        e.mem.set_u32(first + 4, 5);
        e.mem.set_u32(first + 0x10, 0x333);
        e.mem.set_u32(first + 0x14, 2);
        let second = e.mem.alloc(0x18);
        e.mem.set_u32(second + 8, 0x222);
        e.mem.set_u32(second + 0xc, 3);
        let node = e.mem.alloc(8);
        e.mem.set_u32(faction + 0x3c, first);
        e.mem.set_u32(faction + 0x40, node);
        e.mem.set_u32(node, second);
        e.mem.set_u32(faction + 0x38, 0x900);
        e.call_log = Some(vec![]);
        e.call(0x005f_c9d0, &args![faction]);
        assert_eq!(calls(&e, 0x0048_bb70), vec![vec![faction + 0x24]]);
        assert_eq!(calls(&e, 0x0050_3210).len(), 2);
        assert_eq!(
            calls(&e, 0x0048_5e20),
            vec![vec![faction, faction + 0x34, 4]]
        );
        assert_eq!(
            calls(&e, WRITE_WORD_CHUNK),
            vec![vec![SIG_RNAM, 0], vec![SIG_RNAM, 1], vec![SIG_WMI1, 0x901]]
        );
        assert_eq!(
            calls(&e, WRITE_ARRAY_CHUNK),
            vec![
                vec![SIG_MNAM, 0x111, 6],
                vec![SIG_INAM, 0x333, 3],
                vec![SIG_FNAM, 0x222, 4]
            ]
        );
        assert_eq!(calls(&e, 0x0048_5680), vec![vec![faction]]);
    }
}
