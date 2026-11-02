//! `fallout shared/run_00504cc0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00505010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505010 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4a (0x108 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00505010(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_66e4);
    e.call(0x004f_15a0, &args![this, 0x4a]);
    e.call(0x0050_50d0, &args![this]);
    this
}

// Translated from 00504fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x78 (`SUB ECX,0x78`) and jumps to `00504420`. `00504420` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_00504fe0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0050_4420,
        &args![this.addr().wrapping_sub(0x78), first, second],
    );
}

// Translated from 00504ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `00503d10`. `00503d10` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00504ff0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_3d10, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00505080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005050b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00505080(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_50b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The default combat style's form id.
const DEFAULT_COMBAT_STYLE_ID: u32 = 0x3d;
/// `memcpy` wrapper (`00401460`, `cdecl`: destination, source, size) and the `memset`
/// wrapper (`00403d30`, `cdecl`: destination, byte, size).
const MEMCPY: u32 = 0x0040_1460;
const MEMSET: u32 = 0x0040_3d30;
/// The address getters of the source style's three data blocks: `0043c490` (+0x58),
/// `006ab360` (+0xb4) and `00500940` (+0x18), each `ECX`.
const DATA_58_0043C490: u32 = 0x0043_c490;
const DATA_B4_006AB360: u32 = 0x006a_b360;
const DATA_18_00500940: u32 = 0x0050_0940;

// Translated from 00505000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, no words): the form 0x3d (`004839c0`), the default combat
/// style the `TESCombatStyle` defaults and several `InitItem`s read.
pub fn fn_00505000(e: &mut Engine) -> u32 {
    e.call(LOOKUP_FORM, &args![DEFAULT_COMBAT_STYLE_ID]).u32()
}

// Translated from 005050d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCombatStyle`'s vtable (`010266e4`) slot +0x14, the `InitializeData` slot of the Xbox
/// PDB's `TESForm` (`ECX`; the constructor runs it): with a default combat style
/// ([`fn_00505000`]) its three data blocks are copied (`memcpy`: 0x5c bytes to +0x58, 0x54 to
/// +0xb4, 0x40 to +0x18, each source address from its getter); without one the three blocks
/// are cleared (`memset`).
pub fn fn_005050d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let source = fn_00505000(e);
    if source != 0 {
        let data = e.call(DATA_58_0043C490, &args![source]).u32();
        e.call(MEMCPY, &args![base + 0x58, data, 0x5cu32]);
        let data = e.call(DATA_B4_006AB360, &args![source]).u32();
        e.call(MEMCPY, &args![base + 0xb4, data, 0x54u32]);
        let data = e.call(DATA_18_00500940, &args![source]).u32();
        e.call(MEMCPY, &args![base + 0x18, data, 0x40u32]);
    } else {
        e.call(MEMSET, &args![base + 0x58, 0u32, 0x5cu32]);
        e.call(MEMSET, &args![base + 0xb4, 0u32, 0x54u32]);
        e.call(MEMSET, &args![base + 0x18, 0u32, 0x40u32]);
    }
}

// Translated from 005050b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESCombatStyle` destructor body (`ECX`; the scalar deleting destructor `00505080`
/// runs it): the vtable `010266e4` (RTTI `.?AVTESCombatStyle@@`), then `TESForm::~TESForm`
/// (`00483630`).
pub fn fn_005050b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_66e4);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00505c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many places (a folded getter): the byte at +0x99 of
/// `this` (`ECX`), in `AL`.
pub fn fn_00505c00(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x99))
}

// Translated from 00504e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many places (a folded getter): the address `this + 0x1e8`.
pub fn fn_00504e60(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x1e8)
}

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type (`AL`).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// The byte-order swaps of the three data blocks: `00505610` (+0x18), `005059c0` (+0x58),
/// `005057a0` (+0xb4), each `ECX`.
const SWAP_DATA_00505610: u32 = 0x0050_5610;
const SWAP_DATA_005059C0: u32 = 0x0050_59c0;
const SWAP_DATA_005057A0: u32 = 0x0050_57a0;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00403570` (`ECX` = file): the record's form version (16 bits, in `AX`).
const FORM_VERSION: u32 = 0x0040_3570;
/// Chunk codes `00505180` reads (the four characters, first byte lowest), compared signed.
const CHUNK_OBND: i32 = 0x444e_424f;
const CHUNK_CSAD: i32 = 0x4441_5343;
const CHUNK_EDID: i32 = 0x4449_4445;
const CHUNK_CSSD: i32 = 0x4453_5343;
const CHUNK_CSTD: i32 = 0x4454_5343;

// Translated from 00505180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505180 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// The `TESCombatStyle` `Load` (unnamed in the map; `ECX` and one stack word, the file,
/// `RET 4`): false unless the record's form type is 0x4a. Otherwise vtable slot +0x18
/// (`ClearData`), `TESForm::LoadForm`, form flag 8 cleared, then each chunk: `OBND` (vtable
/// slot +0xe0 with the file), `EDID` (a stack buffer of the chunk's size, read with at most
/// 0x200 bytes, given to vtable slot +0x134), `CSTD` (0x5c bytes at +0x58, swapped by
/// `005059c0` when the file needs it), `CSAD` (0x54 bytes at +0xb4, swapped by `005057a0`),
/// `CSSD` (0x40 bytes at +0x18; when the file needs swapping and the chunk is shorter than
/// 0x40 bytes, the block is saved to a stack buffer first, swapped by `00505610`, and its
/// bytes past the chunk's size restored). After the chunks a record of form version below
/// 0xc gets 1.0 at +0x38 and +0x44. True.
pub fn fn_00505180(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() as u8 != 0x4a {
        return false;
    }
    let base = this.addr();
    e.vcall(base, 0x18, &args![]);
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    e.with_stack(0x40, |e, saved| {
        let saved = saved.addr();
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            let code = chunk as i32;
            if code == CHUNK_OBND {
                e.vcall(base, 0xe0, &args![file]);
            } else if code == CHUNK_CSAD {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0xb4, 0x54u32]);
                if e.call(NEEDS_SWAP, &args![file]).bool() {
                    e.call(SWAP_DATA_005057A0, &args![base + 0xb4]);
                }
            } else if code == CHUNK_EDID {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, name| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                    e.vcall(base, 0x134, &args![name]);
                });
            } else if code == CHUNK_CSSD {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x18, 0x40u32]);
                if e.call(NEEDS_SWAP, &args![file]).bool() {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    if size < 0x40 {
                        e.call(MEMCPY, &args![saved, base + 0x18, 0x40u32]);
                    }
                    e.call(SWAP_DATA_00505610, &args![base + 0x18]);
                    if size < 0x40 {
                        e.call(
                            MEMCPY,
                            &args![base + 0x18 + size, saved + size, 0x40 - size],
                        );
                    }
                }
            } else if code == CHUNK_CSTD {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x58, 0x5cu32]);
                if e.call(NEEDS_SWAP, &args![file]).bool() {
                    e.call(SWAP_DATA_005059C0, &args![base + 0x58]);
                }
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    if e.call(FORM_VERSION, &args![file]).u32() & 0xffff < 0xc {
        // FLD1; FSTP float.
        e.mem.set_u32(base + 0x38, x87_store32(1.0));
        e.mem.set_u32(base + 0x44, x87_store32(1.0));
    }
    true
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, 0, from, to, 0).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `memcmp` (`00ec4835`, cdecl: first, second, size).
const MEMCMP: u32 = 0x00ec_4835;
/// `00408da0` (`thiscall`): the string a name holder holds.
const NAME_TEXT: u32 = 0x0040_8da0;
/// `00408b20` (cdecl): compares two C strings.
const STRING_COMPARE: u32 = 0x0040_8b20;
/// `00401080` (cdecl: address, byte): swaps the byte order of one word.
const SWAP_WORD: u32 = 0x0040_1080;
/// `00401500` (no arguments): whether saved data needs its byte order swapped (`AL`).
const SAVE_NEEDS_SWAP: u32 = 0x0040_1500;

// Translated from 00504cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies from `other` (`RET 4`) when it casts (`__RTDynamicCast` from `01183028` to
/// `0118645c`): the components (`TESForm::CopyAllComponents`), the ten 0x20-byte entries at
/// +0x24 (each this entry's vtable slot 0x18 with the other entry's slot 0x14,
/// `00504e00`), the eleven sound names at +0x164 (`00489100` with `00408da0` of
/// `00504e20`), the form at +0x218 (`0084e3a0` of `00504e40`, or 0 without one), the 0x38
/// bytes at +0x1E8 (`REP MOVSD` from `00504e60`; they reach +0x220, so the form word at +0x218 ends as the copied one) and the form type (`004f15a0` with
/// `00401170`).
pub fn fn_00504cc0(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_645cu32, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, cast]);
    for index in 0..10u32 {
        let theirs = fn_00504e00(e, Ptr::new(cast), index);
        let value = e.vcall(theirs, 0x14, &args![]).u32();
        let ours = base + 0x24 + (index << 5);
        e.vcall(ours, 0x18, &args![value]);
    }
    for index in 0..0xbu32 {
        let theirs = fn_00504e20(e, Ptr::new(cast), index);
        let name = e.call(NAME_TEXT, &args![theirs]).u32();
        e.call(0x0048_9100, &args![base + 0x164 + index * 0xc, name]);
    }
    if fn_00504e40(e, Ptr::new(cast)) != 0 {
        let form = fn_00504e40(e, Ptr::new(cast));
        let value = e.call(0x0084_e3a0, &args![form]).u32();
        e.mem.set_u32(base + 0x218, value);
    } else {
        e.mem.set_u32(base + 0x218, 0);
    }
    let source = fn_00504e60(e, Ptr::new(cast));
    for word in 0..0xe {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(base + 0x1e8 + 4 * word, value);
    }
    let kind = e.call(0x0040_1170, &args![cast]).u32();
    e.call(0x004f_15a0, &args![this, kind]);
}

// Translated from 00504e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of entry `index` of the 0x20-byte entries at +0x24 (`RET 4`).
pub fn fn_00504e00(_e: &mut Engine, this: Ptr, index: u32) -> u32 {
    this.addr().wrapping_add(index << 5).wrapping_add(0x24)
}

// Translated from 00504e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of entry `index` of the 0xC-byte entries at +0x164 (`RET 4`).
pub fn fn_00504e20(_e: &mut Engine, this: Ptr, index: u32) -> u32 {
    this.addr()
        .wrapping_add(index.wrapping_mul(0xc))
        .wrapping_add(0x164)
}

// Translated from 00504e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form of the form id at +0x218 (`004839c0`), and its value.
pub fn fn_00504e40(e: &mut Engine, this: Ptr) -> u32 {
    let id = e.mem.u32(this.addr() + 0x218);
    e.call(LOOKUP_FORM, &args![id]).u32()
}

// Translated from 00504e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts (`__RTDynamicCast` from
/// `01183028` to `0118645c`), `TESForm::CompareAllComponents` finds no difference, the
/// eleven sound names (`00408da0`, `00408b20`) and the ten entries' slot 0x14 strings
/// compare equal, the form of +0x218 (`004839c0`) is the other's (`00504e40`) and the 0x38
/// bytes at +0x1E8 equal the other's (`00504e60`, `memcmp`).
pub fn fn_00504e80(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_645cu32, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, cast]).bool() {
        return true;
    }
    for index in 0..0xbu32 {
        let theirs = fn_00504e20(e, Ptr::new(cast), index);
        let their_name = e.call(NAME_TEXT, &args![theirs]).u32();
        let our_name = e.call(NAME_TEXT, &args![base + 0x164 + index * 0xc]).u32();
        if e.call(STRING_COMPARE, &args![our_name, their_name]).u32() != 0 {
            return true;
        }
    }
    for index in 0..10u32 {
        let theirs = fn_00504e00(e, Ptr::new(cast), index);
        let their_value = e.vcall(theirs, 0x14, &args![]).u32();
        let ours = base + 0x24 + (index << 5);
        let our_value = e.vcall(ours, 0x14, &args![]).u32();
        if e.call(STRING_COMPARE, &args![our_value, their_value]).u32() != 0 {
            return true;
        }
    }
    let id = e.mem.u32(base + 0x218);
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    if form != fn_00504e40(e, Ptr::new(cast)) {
        return true;
    }
    let theirs = fn_00504e60(e, Ptr::new(cast));
    e.call(MEMCMP, &args![base + 0x1e8, theirs, 0x38u32]).u32() != 0
}

/// A byte-order swap of one data block.
type BlockSwap = fn(&mut Engine, Ptr);

// Translated from 005053c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the form (`ECX` only; the chunks `00505180` loads): `TESForm::StartForm`, then the `CSTD` (0x5C bytes
/// at +0x58), `CSAD` (0x54 at +0xB4) and `CSSD` (0x40 at +0x18) chunks through
/// `TESForm::__AddChunkData`, each block swapped (`005059c0`, `005057a0`, `00505610`) before
/// and after when `00401500` says the save needs it, and `TESForm::CloseForm`.
pub fn fn_005053c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0048_55a0, &args![this]);
    let blocks: [(u32, u32, u32, BlockSwap); 3] = [
        (0x4454_5343, 0x58, 0x5c, fn_005059c0),
        (0x4441_5343, 0xb4, 0x54, fn_005057a0),
        (0x4453_5343, 0x18, 0x40, fn_00505610),
    ];
    for (chunk, offset, size, swap) in blocks {
        if e.call(SAVE_NEEDS_SWAP, &args![]).bool() {
            swap(e, Ptr::new(base + offset));
        }
        e.call(0x0048_5990, &args![chunk, base + offset, size]);
        if e.call(SAVE_NEEDS_SWAP, &args![]).bool() {
            swap(e, Ptr::new(base + offset));
        }
    }
    e.call(0x0048_5680, &args![this]);
}

// Translated from 005054b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies from `other` (`RET 4`) when it casts (`__RTDynamicCast` from `01183028` to
/// `0118627c`): the components (`TESForm::CopyAllComponents`), then the three data blocks
/// (`memcpy` of 0x5C bytes between the `0043c490` addresses, 0x54 between the `006ab360`
/// ones, 0x40 between the `00500940` ones).
pub fn fn_005054b0(e: &mut Engine, this: Ptr, other: u32) {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_627cu32, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, cast]);
    for (getter, size) in [
        (DATA_58_0043C490, 0x5cu32),
        (DATA_B4_006AB360, 0x54),
        (DATA_18_00500940, 0x40),
    ] {
        let source = e.call(getter, &args![cast]).u32();
        let destination = e.call(getter, &args![this]).u32();
        e.call(MEMCPY, &args![destination, source, size]);
    }
}

// Translated from 00505550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts (`__RTDynamicCast` from
/// `01183028` to `0118627c`), `TESForm::CompareAllComponents` finds no difference and the
/// three data blocks compare equal (`memcmp`, the `0043c490`, `006ab360` and `00500940`
/// addresses, 0x5C, 0x54 and 0x40 bytes).
pub fn fn_00505550(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_627cu32, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, cast]).bool() {
        return true;
    }
    for (getter, size) in [
        (DATA_58_0043C490, 0x5cu32),
        (DATA_B4_006AB360, 0x54),
        (DATA_18_00500940, 0x40),
    ] {
        let theirs = e.call(getter, &args![cast]).u32();
        let ours = e.call(getter, &args![this]).u32();
        if e.call(MEMCMP, &args![ours, theirs, size]).u32() != 0 {
            return true;
        }
    }
    false
}

/// `00401080` with 0 on each word at `offsets` from `block`, in the exe's order.
fn swap_words(e: &mut Engine, block: Ptr, offsets: &[u32]) {
    for &offset in offsets {
        e.call(SWAP_WORD, &args![block.addr() + offset, 0u32]);
    }
}

// Translated from 00505610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the byte order of the `CSSD` block's words (`00401080` with 0): +0x0 .. +0x20,
/// +0x2C, +0x24, +0x30 .. +0x3C.
pub fn fn_00505610(e: &mut Engine, this: Ptr) {
    swap_words(
        e,
        this,
        &[
            0, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x2c, 0x24, 0x30, 0x34, 0x38, 0x3c,
        ],
    );
}

// Translated from 005057a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the byte order of the `CSAD` block's words (`00401080` with 0): +0x0 .. +0x40,
/// +0x48, +0x44, +0x4C, +0x50.
pub fn fn_005057a0(e: &mut Engine, this: Ptr) {
    swap_words(
        e,
        this,
        &[
            0, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c,
            0x40, 0x48, 0x44, 0x4c, 0x50,
        ],
    );
}

// Translated from 005059c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the byte order of the `CSTD` block's words (`00401080` with 0): +0x4 .. +0x20,
/// +0x28, +0x2C, +0x30, +0x38, +0x3C, +0x48, +0x4C, +0x58, +0x50.
pub fn fn_005059c0(e: &mut Engine, this: Ptr) {
    swap_words(
        e,
        this,
        &[
            4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x28, 0x2c, 0x30, 0x38, 0x3c, 0x48, 0x4c,
            0x58, 0x50,
        ],
    );
}

// Translated from 00505b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The combat style's byte for attack `kind` (`RET 4`, `AL`; the jump table at `00505be4`):
/// 0x5C `0073cb80`, 0x5D `00505c00`, 0x5E `00505c20`, 0x5F `006a9560`, 0x60 `009e8670`,
/// anything else 0.
pub fn fn_00505b80(e: &mut Engine, this: Ptr, kind: u32) -> u8 {
    match kind.wrapping_sub(0x5c) {
        0 => e.call(0x0073_cb80, &args![this]).u8(),
        1 => fn_00505c00(e, this),
        2 => fn_00505c20(e, this),
        3 => e.call(0x006a_9560, &args![this]).u8(),
        4 => e.call(0x009e_8670, &args![this]).u8(),
        _ => 0,
    }
}

// Translated from 00505c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x9A (`AL`).
pub fn fn_00505c20(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x9a)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_5180, fn_00505180(Ptr, Ptr) -> bool),
        entry!(0x0050_5000, fn_00505000() -> u32),
        entry!(0x0050_50d0, fn_005050d0(Ptr)),
        entry!(0x0050_50b0, fn_005050b0(Ptr)),
        entry!(0x0050_5c00, fn_00505c00(Ptr) -> u8),
        entry!(0x0050_4e60, fn_00504e60(Ptr) -> u32),
        entry!(0x0050_5010, fn_00505010(Ptr) -> Ptr),
        entry!(0x0050_4fe0, fn_00504fe0(Ptr, u32, u32)),
        entry!(0x0050_4ff0, fn_00504ff0(Ptr, u32) -> u32),
        entry!(0x0050_5080, fn_00505080(Ptr, u32) -> Ptr),
        entry!(0x0050_4cc0, fn_00504cc0(Ptr, u32)),
        entry!(0x0050_4e00, fn_00504e00(Ptr, u32) -> u32),
        entry!(0x0050_4e20, fn_00504e20(Ptr, u32) -> u32),
        entry!(0x0050_4e40, fn_00504e40(Ptr) -> u32),
        entry!(0x0050_4e80, fn_00504e80(Ptr, u32) -> bool),
        entry!(0x0050_53c0, fn_005053c0(Ptr)),
        entry!(0x0050_54b0, fn_005054b0(Ptr, u32)),
        entry!(0x0050_5550, fn_00505550(Ptr, u32) -> bool),
        entry!(0x0050_5610, fn_00505610(Ptr)),
        entry!(0x0050_57a0, fn_005057a0(Ptr)),
        entry!(0x0050_59c0, fn_005059c0(Ptr)),
        entry!(0x0050_5b80, fn_00505b80(Ptr, u32) -> u8),
        entry!(0x0050_5c20, fn_00505c20(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00505010_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_5010,
            size: 0x108,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4a)]),
                (0x0050_50d0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_66e4)],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00504fe0() {
        check(0x0050_4fe0, 0x0050_4420, 0x78, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_00504ff0() {
        assert_eq!(
            check(0x0050_4ff0, 0x0050_3d10, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_00505080_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_50b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_5080, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_5080)
                .collect();
            let mut expected = vec![(0x0050_50b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod combat_style_tests {
    use super::*;

    fn order(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .cloned()
            .collect()
    }

    #[test]
    fn fn_00505000_looks_up_form_0x3d() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x1000).into_ret());
        assert_eq!(e.call(0x0050_5000, &args![]).u32(), 0x103d);
    }

    #[test]
    fn fn_005050d0_copies_the_default_or_clears() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, _| 0x7000u32.into_ret());
        e.register(DATA_58_0043C490, |_, a| (a[0] + 0x58).into_ret());
        e.register(DATA_B4_006AB360, |_, a| (a[0] + 0xb4).into_ret());
        e.register(DATA_18_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_50d0, &args![0x5000u32]);
        let copies: Vec<(u32, Vec<u32>)> =
            order(&e).into_iter().filter(|c| c.0 == MEMCPY).collect();
        assert_eq!(
            copies,
            vec![
                (MEMCPY, vec![0x5058, 0x7058, 0x5c]),
                (MEMCPY, vec![0x50b4, 0x70b4, 0x54]),
                (MEMCPY, vec![0x5018, 0x7018, 0x40]),
            ]
        );
        e.register(LOOKUP_FORM, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_50d0, &args![0x5000u32]);
        assert_eq!(
            order(&e)
                .into_iter()
                .filter(|c| c.0 != LOOKUP_FORM)
                .collect::<Vec<_>>(),
            vec![
                (MEMSET, vec![0x5058, 0, 0x5c]),
                (MEMSET, vec![0x50b4, 0, 0x54]),
                (MEMSET, vec![0x5018, 0, 0x40]),
            ]
        );
    }

    #[test]
    fn fn_005050b0_stores_the_vtable_then_destroys() {
        let mut e = Engine::new();
        e.register(0x0048_3630, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x108);
        e.call(0x0050_50b0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0x0102_66e4);
    }

    #[test]
    fn fn_00505c00_and_00504e60_read_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        e.mem.set_u8(this + 0x99, 0x5a);
        assert_eq!(e.call(0x0050_5c00, &args![this]).u32() & 0xff, 0x5a);
        assert_eq!(e.call(0x0050_4e60, &args![0x5000u32]).u32(), 0x51e8);
    }
}

#[cfg(test)]
mod combat_style_load_tests {
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

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool, version: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x4au32.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).map_or(0, |c| c.0).into_ret()
        });
        let s = script.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            s.0[s.1].1.into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        // Reads fill their destination with 0x11.
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0x11);
            }
            Ret::default()
        });
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        // The swaps write 0x22 over the whole block.
        e.register(SWAP_DATA_00505610, |e, a| {
            for i in 0..0x40 {
                e.mem.set_u8(a[0] + i, 0x22);
            }
            Ret::default()
        });
        e.register(SWAP_DATA_005059C0, |_, _| Ret::default());
        e.register(SWAP_DATA_005057A0, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register_double(FORM_VERSION, move |_, _| (0xffff_0000 | version).into_ret());
        e.put_vtable(0x0f03_1000, &[0; 0x4e]);
        e.mem.set_u32(0x0f03_1018, 0x0f03_1300);
        e.mem.set_u32(0x0f03_10e0, 0x0f03_1100);
        e.mem.set_u32(0x0f03_1134, 0x0f03_1200);
        for addr in [0x0f03_1100u32, 0x0f03_1200, 0x0f03_1300] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x108);
        e.mem.set_u32(this, 0x0f03_1000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00505180_refuses_other_records() {
        let (mut e, this) = load_rig(vec![], false, 0xc);
        e.register(GET_TES_FORM, |_, _| 0x4bu32.into_ret());
        assert!(!e.call(0x0050_5180, &args![this, FILE]).bool());
        assert!(calls(&e, 0x0f03_1300).is_empty());
    }

    #[test]
    fn fn_00505180_reads_each_chunk() {
        let chunks = vec![
            (CHUNK_EDID as u32, 5),
            (CHUNK_OBND as u32, 0xc),
            (CHUNK_CSTD as u32, 0x5c),
            (CHUNK_CSAD as u32, 0x54),
            (CHUNK_CSSD as u32, 0x40),
        ];
        let (mut e, this) = load_rig(chunks, true, 0xc);
        assert!(e.call(0x0050_5180, &args![this, FILE]).bool());
        assert_eq!(calls(&e, 0x0f03_1300), vec![vec![this]]);
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![this, FILE]]);
        let reads = calls(&e, GET_CHUNK_DATA_SIZED);
        assert_eq!(reads[0][2], 0x200);
        assert_eq!(
            reads[1..],
            [
                vec![FILE, this + 0x58, 0x5c],
                vec![FILE, this + 0xb4, 0x54],
                vec![FILE, this + 0x18, 0x40]
            ]
        );
        assert_eq!(calls(&e, SWAP_DATA_005059C0), vec![vec![this + 0x58]]);
        assert_eq!(calls(&e, SWAP_DATA_005057A0), vec![vec![this + 0xb4]]);
        assert_eq!(calls(&e, SWAP_DATA_00505610), vec![vec![this + 0x18]]);
        // A full CSSD chunk: no save and restore.
        assert!(calls(&e, MEMCPY).is_empty());
        assert_eq!(e.mem.u8(this + 0x18 + 0x3f), 0x22);
        // Version 0xc keeps the floats as read.
        assert_eq!(e.mem.u32(this + 0x38), 0x2222_2222);
    }

    #[test]
    fn fn_00505180_restores_the_bytes_past_a_short_chunk() {
        let (mut e, this) = load_rig(vec![(CHUNK_CSSD as u32, 0x30)], true, 0xb);
        e.mem.write(this + 0x18, &[0x11; 0x40]);
        assert!(e.call(0x0050_5180, &args![this, FILE]).bool());
        let copies = calls(&e, MEMCPY);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][1..], [this + 0x18, 0x40]);
        assert_eq!(copies[1][0], this + 0x18 + 0x30);
        assert_eq!(copies[1][2], 0x10);
        // The swapped part stays, the part past the chunk is restored.
        assert_eq!(e.mem.u8(this + 0x18 + 0x10), 0x22);
        assert_eq!(e.mem.u8(this + 0x18 + 0x30), 0x11);
        // An old record (version 0xb) gets 1.0 at +0x38 and +0x44.
        assert_eq!(e.mem.f32(this + 0x38), 1.0);
        assert_eq!(e.mem.f32(this + 0x44), 1.0);
    }

    #[test]
    fn fn_00505180_without_swapping_leaves_the_data() {
        let (mut e, this) = load_rig(vec![(CHUNK_CSSD as u32, 0x30)], false, 0x20);
        assert!(e.call(0x0050_5180, &args![this, FILE]).bool());
        assert!(calls(&e, SWAP_DATA_00505610).is_empty());
        assert!(calls(&e, CHUNK_SIZE).is_empty());
        assert_eq!(e.mem.u8(this + 0x18), 0x11);
    }
}

#[cfg(test)]
mod n039_tests {
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

    /// Two objects whose ten entries at +0x24 share one vtable: slot 0x14 answers the entry
    /// address, slot 0x18 is recorded.
    fn entry_objects(e: &mut Engine) -> (u32, u32) {
        let mut table = vec![0u32; 8];
        table[0x14 / 4] = 0x00ea_0014;
        table[0x18 / 4] = 0x00ea_0018;
        e.put_vtable(0x00dd_0000, &table);
        e.register(0x00ea_0014, |_, a| a[0].into_ret());
        e.register(0x00ea_0018, |_, _| Ret::default());
        let make = |e: &mut Engine| {
            let o = e.mem.alloc(0x220);
            for i in 0..10 {
                e.mem.set_u32(o + 0x24 + 0x20 * i, 0x00dd_0000);
            }
            o
        };
        (make(e), make(e))
    }

    #[test]
    fn fn_00504cc0_copies_entries_names_form_and_block() {
        let mut e = Engine::new();
        let (this, other) = entry_objects(&mut e);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        for address in [0x0048_51b0, 0x0048_9100, 0x004f_15a0] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(NAME_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(LOOKUP_FORM, |_, a| a[0].into_ret());
        e.register(0x0084_e3a0, |_, a| (a[0] + 2).into_ret());
        e.register(0x0040_1170, |_, _| 0x41u8.into_ret());
        e.mem.set_u32(other + 0x218, 0x700);
        // The block at +0x1E8 reaches +0x220: its word 12 is the form id at +0x218.
        for word in 0..0xc {
            e.mem.set_u32(other + 0x1e8 + 4 * word, 0x100 + word);
        }
        e.call_log = Some(vec![]);
        e.call(0x0050_4cc0, &args![this, other]);
        let set = calls_to(&e, 0x00ea_0018);
        assert_eq!(set.len(), 10);
        assert_eq!(set[3], vec![this + 0x24 + 0x60, other + 0x24 + 0x60]);
        let names = calls_to(&e, 0x0048_9100);
        assert_eq!(
            names[10],
            vec![this + 0x164 + 0x78, other + 0x164 + 0x78 + 1]
        );
        assert_eq!(calls_to(&e, 0x0084_e3a0), vec![vec![0x700]]);
        assert_eq!(e.mem.u32(this + 0x218), 0x700);
        assert_eq!(e.mem.u32(this + 0x1e8 + 0x2c), 0x10b);
        assert_eq!(calls_to(&e, 0x004f_15a0), vec![vec![this, 0x41]]);
        e.mem.set_u32(other + 0x218, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_4cc0, &args![this, other]);
        assert!(calls_to(&e, 0x0084_e3a0).is_empty());
        assert_eq!(e.mem.u32(this + 0x218), 0);
    }

    #[test]
    fn fn_00504e00_addresses_the_entry() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0050_4e00, &args![0x1000u32, 3u32]).u32(), 0x1084);
    }

    #[test]
    fn fn_00504e20_addresses_the_sound() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0050_4e20, &args![0x1000u32, 2u32]).u32(), 0x117c);
    }

    #[test]
    fn fn_00504e40_looks_up_the_form() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, a| (a[0] + 1).into_ret());
        let this = e.mem.alloc(0x220);
        e.mem.set_u32(this + 0x218, 0x700);
        assert_eq!(e.call(0x0050_4e40, &args![this]).u32(), 0x701);
    }

    #[test]
    fn fn_00504e80_compares_everything() {
        let mut e = Engine::new();
        let (this, other) = entry_objects(&mut e);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(0x0048_5270, |_, _| false.into_ret());
        e.register(NAME_TEXT, |_, a| a[0].into_ret());
        e.register(STRING_COMPARE, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, a| a[0].into_ret());
        e.register(MEMCMP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_4e80, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, STRING_COMPARE)[0],
            vec![this + 0x164, other + 0x164]
        );
        assert_eq!(
            calls_to(&e, MEMCMP),
            vec![vec![this + 0x1e8, other + 0x1e8, 0x38]]
        );
        e.mem.set_u32(this + 0x218, 5);
        assert!(e.call(0x0050_4e80, &args![this, other]).bool());
        e.mem.set_u32(this + 0x218, 0);
        e.register(STRING_COMPARE, |_, _| 1u32.into_ret());
        assert!(e.call(0x0050_4e80, &args![this, other]).bool());
        e.register(0x0048_5270, |_, _| true.into_ret());
        assert!(e.call(0x0050_4e80, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0050_4e80, &args![this, other]).bool());
    }

    #[test]
    fn fn_005053c0_saves_three_chunks_swapping_around_them() {
        let mut e = Engine::new();
        for address in [0x0048_55a0, 0x0048_5990, 0x0048_5680, SWAP_WORD] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(SAVE_NEEDS_SWAP, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_53c0, &args![0x1000u32]);
        assert_eq!(
            calls_to(&e, 0x0048_5990),
            vec![
                vec![0x4454_5343, 0x1058, 0x5c],
                vec![0x4441_5343, 0x10b4, 0x54],
                vec![0x4453_5343, 0x1018, 0x40]
            ]
        );
        assert_eq!(calls_to(&e, SWAP_WORD).len(), 2 * (17 + 21 + 15));
        e.register(SAVE_NEEDS_SWAP, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_53c0, &args![0x1000u32]);
        assert!(calls_to(&e, SWAP_WORD).is_empty());
        assert_eq!(calls_to(&e, 0x0048_5680), vec![vec![0x1000]]);
    }

    #[test]
    fn fn_005054b0_copies_the_three_blocks() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(0x0048_51b0, |_, _| Ret::default());
        e.register(DATA_58_0043C490, |_, a| (a[0] + 0x58).into_ret());
        e.register(DATA_B4_006AB360, |_, a| (a[0] + 0xb4).into_ret());
        e.register(DATA_18_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.register(MEMCPY, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_54b0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![
                vec![0x1058, 0x2058, 0x5c],
                vec![0x10b4, 0x20b4, 0x54],
                vec![0x1018, 0x2018, 0x40]
            ]
        );
    }

    #[test]
    fn fn_00505550_compares_the_three_blocks() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(0x0048_5270, |_, _| false.into_ret());
        e.register(DATA_58_0043C490, |_, a| (a[0] + 0x58).into_ret());
        e.register(DATA_B4_006AB360, |_, a| (a[0] + 0xb4).into_ret());
        e.register(DATA_18_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.register(MEMCMP, |_, a| u32::from(a[2] == 0x40).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_5550, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(
            calls_to(&e, MEMCMP).last().unwrap(),
            &vec![0x1018, 0x2018, 0x40]
        );
        e.register(MEMCMP, |_, _| Ret::default());
        assert!(!e.call(0x0050_5550, &args![0x1000u32, 0x2000u32]).bool());
        e.register(0x0048_5270, |_, _| true.into_ret());
        assert!(e.call(0x0050_5550, &args![0x1000u32, 0x2000u32]).bool());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0050_5550, &args![0x1000u32, 0x2000u32]).bool());
    }

    fn swapped(e: &mut Engine, address: u32) -> Vec<u32> {
        e.register(SWAP_WORD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![0x1000u32]);
        calls_to(e, SWAP_WORD)
            .iter()
            .map(|c| {
                assert_eq!(c[1], 0);
                c[0] - 0x1000
            })
            .collect()
    }

    #[test]
    fn fn_00505610_swaps_the_cssd_words() {
        let mut e = Engine::new();
        assert_eq!(
            swapped(&mut e, 0x0050_5610),
            vec![0, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x2c, 0x24, 0x30, 0x34, 0x38, 0x3c]
        );
    }

    #[test]
    fn fn_005057a0_swaps_the_csad_words() {
        let mut e = Engine::new();
        let offsets = swapped(&mut e, 0x0050_57a0);
        assert_eq!(offsets.len(), 21);
        assert_eq!(offsets[16..], [0x40, 0x48, 0x44, 0x4c, 0x50]);
    }

    #[test]
    fn fn_005059c0_swaps_the_cstd_words() {
        let mut e = Engine::new();
        let offsets = swapped(&mut e, 0x0050_59c0);
        assert_eq!(offsets.len(), 17);
        assert_eq!(
            (offsets[0], offsets[8], offsets[15], offsets[16]),
            (4, 0x28, 0x58, 0x50)
        );
    }

    #[test]
    fn fn_00505b80_dispatches_by_kind() {
        let mut e = Engine::new();
        e.register(0x0073_cb80, |_, _| 1u8.into_ret());
        e.register(0x006a_9560, |_, _| 4u8.into_ret());
        e.register(0x009e_8670, |_, _| 5u8.into_ret());
        let this = e.mem.alloc(0xa0);
        e.mem.set_u8(this + 0x99, 2);
        e.mem.set_u8(this + 0x9a, 3);
        let answers: Vec<u8> = (0x5bu32..0x62)
            .map(|kind| e.call(0x0050_5b80, &args![this, kind]).u8())
            .collect();
        assert_eq!(answers, vec![0, 1, 2, 3, 4, 5, 0]);
    }

    #[test]
    fn fn_00505c20_reads_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.set_u8(this + 0x9a, 0x77);
        assert_eq!(e.call(0x0050_5c20, &args![this]).u8(), 0x77);
    }
}
