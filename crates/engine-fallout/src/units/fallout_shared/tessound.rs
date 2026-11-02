//! `fallout shared/tessound.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e2b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e2b70 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESSound::TESSound` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0xd (0x6c bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tessound_tessound(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0059_12d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_4ffc);
    e.mem.set_u32(base + 0x30, 0x0104_4fe4);
    e.call(0x0040_37b0, &args![base + 0x3c]);
    e.call(0x004f_15a0, &args![this, 0xd]);
    e.mem.set_u8(base + 0x68, 0x0);
    e.call(0x005e_2ef0, &args![this]);
    this
}

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `005e3160` tells apart.
const FNAM: u32 = u32::from_le_bytes(*b"FNAM");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const SNDD: u32 = u32::from_le_bytes(*b"SNDD");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const ANAM: u32 = u32::from_le_bytes(*b"ANAM");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
const GNAM: u32 = u32::from_le_bytes(*b"GNAM");
const HNAM: u32 = u32::from_le_bytes(*b"HNAM");
const SNDX: u32 = u32::from_le_bytes(*b"SNDX");

// Translated from 005e2ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the form `005e2b70` (`TESSound`, form type 0xd) builds: the sound data bytes
/// at `+0x44`..`+0x47` are cleared, the word at `+0x48` and the words at `+0x4c` and `+0x4e` are
/// cleared, the words at `+0x50`..`+0x5a` are set to 100, 0x32, 0x14, 5, 0 and 0x50, the word
/// at `+0x5c` is set to 0x80 and the words at `+0x60` and `+0x64` are cleared.
pub fn fn_005e2ef0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u8(base + 0x45, 0);
    e.mem.set_u8(base + 0x44, 0);
    e.mem.set_u8(base + 0x46, 0);
    e.mem.set_u16(base + 0x4c, 0);
    e.mem.set_u8(base + 0x47, 0);
    e.mem.set_u16(base + 0x4e, 0);
    e.mem.set_u16(base + 0x50, 100);
    e.mem.set_u16(base + 0x52, 0x32);
    e.mem.set_u16(base + 0x54, 0x14);
    e.mem.set_u16(base + 0x56, 5);
    e.mem.set_u16(base + 0x58, 0);
    e.mem.set_u16(base + 0x5a, 0x50);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u32(base + 0x5c, 0x80);
}

// Translated from 005e3160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e3160 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of `TESSound` (form type 0xd; the engine map gives the function no name).
/// `this` is the form and `file` the `TESFile` positioned on its record. Answers false without
/// touching the form when the file's record is not of type 0xd, otherwise true.
///
/// It runs `TESForm::LoadForm` (`00485110`) and walks the record's chunks:
///
/// - `FNAM`: reads the chunk into a buffer and runs virtual slot `+0x10` of the component at
///   `+0x30` on it;
/// - `OBND`: virtual slot `+0xe0` with the file;
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it;
/// - `SNDD`: `TESForm::LoadData` (`004861f0`) of 0x24 bytes into `+0x44`, then `005e3020` on it
///   when `00401680` answers true;
/// - `SNDX`: loads 0xc bytes into a local, runs `005e3510` on it when `00401680` answers true,
///   and stores its bytes at `+0x44`, `+0x45`, `+0x46`, its word at local offset 4 at `+0x48`
///   and its words at local offsets 8 and 10 at `+0x4c` and `+0x4e`;
/// - `ANAM`: loads 10 bytes into `+0x50`; when `00401680` answers true, runs `00407a90` with 0 on
///   each of the five words; when the words are then 100, 0x4b, 0x32, 0x19 and 0 they are
///   replaced by 100, 0x32, 0x14, 5 and 0;
/// - `RNAM`: loads 1 byte into `+0x68`; `GNAM`: 2 bytes into `+0x5a`; `HNAM`: 4 bytes into `+0x5c`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_005e3160(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(0x0047_2660, &args![file]).u32() != 0xd {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            FNAM => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0u32]);
                e.vcall(base + 0x30, 0x10, &args![buffer]);
                e.mem.free(buffer);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SNDD => {
                e.call(0x0048_61f0, &args![this, file, base + 0x44, 0x24u32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x005e_3020, &args![base + 0x44]);
                }
            }
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            ANAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x50, 10u32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    for offset in [0x50, 0x52, 0x54, 0x56, 0x58] {
                        e.call(0x0040_7a90, &args![base + offset, 0u32]);
                    }
                }
                let words = [0x50, 0x52, 0x54, 0x56, 0x58].map(|offset| e.mem.u16(base + offset));
                if words == [100, 0x4b, 0x32, 0x19, 0] {
                    for (offset, value) in [
                        (0x50, 100),
                        (0x52, 0x32),
                        (0x54, 0x14),
                        (0x56, 5),
                        (0x58, 0),
                    ] {
                        e.mem.set_u16(base + offset, value);
                    }
                }
            }
            RNAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x68, 1u32]);
            }
            GNAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x5a, 2u32]);
            }
            HNAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x5c, 4u32]);
            }
            SNDX => {
                let data = e.mem.alloc(12);
                e.call(0x0048_61f0, &args![this, file, data, 0xcu32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x005e_3510, &args![data]);
                }
                let first = e.mem.u8(data);
                let second = e.mem.u8(data + 1);
                let third = e.mem.u8(data + 2);
                let long = e.mem.u32(data + 4);
                let low_word = e.mem.u16(data + 8);
                let high_word = e.mem.u16(data + 10);
                e.mem.set_u8(base + 0x44, first);
                e.mem.set_u8(base + 0x45, second);
                e.mem.set_u8(base + 0x46, third);
                e.mem.set_u32(base + 0x48, long);
                e.mem.set_u16(base + 0x4c, low_word);
                e.mem.set_u16(base + 0x4e, high_word);
                e.mem.free(data);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005e2c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e2ca0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e2c70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_2ca0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `BSStringT<char>` text assignment (`004037f0`, `ECX` = string, two stack words: text,
/// minimum capacity).
const STRING_SET_004037F0: u32 = 0x0040_37f0;
/// `00559450` (`ECX`): the word at +0 (a `BSStringT`'s text pointer).
const STRING_CHARACTERS_00559450: u32 = 0x0055_9450;
/// The empty C string (`01011584`).
const EMPTY_TEXT: u32 = 0x0101_1584;
/// The TLS setting guard (`00404eb0`, `ECX` = the guard, four stack words) and its restore
/// (`00404ee0`, `ECX` = the guard).
const SETTING_GUARD: u32 = 0x0040_4eb0;
const SETTING_GUARD_RESTORE: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESSound.c"`, the guard's source file.
const SOURCE_FILE_NAME: u32 = 0x0104_517c;
/// The player (`011dea3c`), or null.
const PLAYER: u32 = 0x011d_ea3c;
/// `004997b0` (`ECX` = actor): sneaking (flag 0x400 without 0x800), `AL`.
const IS_SNEAKING_004997B0: u32 = 0x0049_97b0;
/// `Actor::IsRunning` (Xbox PDB, `00884730`, `ECX` = actor), `AL`.
const IS_RUNNING: u32 = 0x0088_4730;
/// `"Sneak"`, `"Run"`, `"Walk"`.
const TEXT_SNEAK: u32 = 0x0104_4fd8;
const TEXT_RUN: u32 = 0x0104_4fd4;
const TEXT_WALK: u32 = 0x0104_4fcc;
/// `"BRUSH%s%s"` and `"BRUSH%sWalk"`.
const FORMAT_BRUSH_MOVE: u32 = 0x0104_4fc0;
const FORMAT_BRUSH_WALK: u32 = 0x0104_4fb4;
/// `sprintf_s` (`00406d00`, `cdecl`: buffer, size, format, variable words).
const SPRINTF_S: u32 = 0x0040_6d00;
/// The data handler (`011c3f2c`) and `TESDataHandler::GetSound` by editor id (Xbox PDB,
/// `004616c0`, `ECX` = data handler, one stack word: the name).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const GET_SOUND_BY_NAME: u32 = 0x0046_16c0;
/// The table of ten name texts `005e2b50` reads (`01196d40`).
const NAME_TABLE: u32 = 0x0119_6d40;

// Translated from 005e2c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `ECX` and one stack word, `RET 4`): copies
/// `text` into the editor id string at +0x3c (`004037f0` with a minimum capacity of 0) and
/// answers true.
pub fn fn_005e2c10(e: &mut Engine, this: Ptr, text: Ptr) -> bool {
    e.call(STRING_SET_004037F0, &args![this.addr() + 0x3c, text, 0u32]);
    true
}

// Translated from 005e2c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `ECX`): the text of the editor id string at
/// +0x3c (`00559450`, read a second time when nonzero), or the empty string `01011584`.
pub fn fn_005e2c30(e: &mut Engine, this: Ptr) -> u32 {
    let string = this.addr() + 0x3c;
    if e.call(STRING_CHARACTERS_00559450, &args![string]).u32() != 0 {
        e.call(STRING_CHARACTERS_00559450, &args![string]).u32()
    } else {
        EMPTY_TEXT
    }
}

// Translated from 005e2b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `cdecl`, one word): for `index` 0 to 9 (signed)
/// the word of the table at `01196d40`, otherwise 0.
pub fn fn_005e2b50(e: &mut Engine, index: i32) -> u32 {
    if (0..10).contains(&index) {
        e.global::<u32>(NAME_TABLE + index as u32 * 4)
    } else {
        0
    }
}

// Translated from 005e2a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e2a70 - the /GS cookie check (00ec408c) is not translated
/// `TESSound::GetPassThroughSound` (Xbox PDB), `cdecl` with one word, a material index: the
/// sound `TESDataHandler::GetSound` finds by name. With a player the name is
/// `"BRUSH%s%s"` of the index's name (`005e2b50`) and `"Sneak"` (sneaking, `004997b0`), else
/// `"Run"` (`Actor::IsRunning`) or `"Walk"`; without one `"BRUSH%sWalk"`. Each name is made
/// by `sprintf_s` in its own 0x28-byte stack buffer.
pub fn tes_sound_get_pass_through_sound(e: &mut Engine, index: i32) -> u32 {
    let player = e.global::<u32>(PLAYER);
    e.with_stack(0x28, |e, name| {
        if player != 0 {
            let player = e.global::<u32>(PLAYER);
            let movement = if e.call(IS_SNEAKING_004997B0, &args![player]).u8() != 0 {
                TEXT_SNEAK
            } else {
                let player = e.global::<u32>(PLAYER);
                if e.call(IS_RUNNING, &args![player]).u8() != 0 {
                    TEXT_RUN
                } else {
                    TEXT_WALK
                }
            };
            let material = fn_005e2b50(e, index);
            e.call(
                SPRINTF_S,
                &args![name, 0x28u32, FORMAT_BRUSH_MOVE, material, movement],
            );
        } else {
            let material = fn_005e2b50(e, index);
            e.call(
                SPRINTF_S,
                &args![name, 0x28u32, FORMAT_BRUSH_WALK, material],
            );
        }
        let data_handler = e.global::<u32>(DATA_HANDLER);
        e.call(GET_SOUND_BY_NAME, &args![data_handler, name]).u32()
    })
}

// Translated from 005e3600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSound::GenerateDefaultObjects` (Xbox PDB), no words: a TLS setting guard on the stack
/// (`00404eb0` with 0x2f, 1, this unit's source file name and line 0x559), restored at once
/// (`00404ee0`).
pub fn tes_sound_generate_default_objects(e: &mut Engine) {
    e.with_stack(4, |e, guard| {
        e.call(
            SETTING_GUARD,
            &args![guard, 0x2fu32, 1u32, SOURCE_FILE_NAME, 0x559u32],
        );
        e.call(SETTING_GUARD_RESTORE, &args![guard]);
    });
}

// Translated from 005e2ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e2ca0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef9dd3, state variable) is not translated
/// The `TESSound` destructor body (unnamed in the map; `ECX`, no stack words; the scalar
/// deleting destructor `005e2c70` runs it): stores the two vtables (`01044ffc` at +0,
/// `01044fe4` at +0x30, RTTI `.?AVTESSound@@`), then destroys the editor id string at +0x3c
/// (`004037d0`), the component at +0x30 (`00591300`) and the `TESBoundAnimObject` base
/// (`0050f740`).
pub fn fn_005e2ca0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_4ffc);
    e.mem.set_u32(base + 0x30, 0x0104_4fe4);
    e.call(0x0040_37d0, &args![base + 0x3c]);
    e.call(0x0059_1300, &args![base + 0x30]);
    e.call(0x0050_f740, &args![this]);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) and the RTTI type descriptors of
/// `TESForm` (`01183028`) and `TESSound` (`011832fc`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
/// `TESForm::CopyAllComponents` (engine map, `004851b0`, `RET 4`) and
/// `TESForm::CompareAllComponents` (`00485270`, `RET 4`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `004e75d0` (`thiscall` on a sound, `RET 4`: a 0x24-byte buffer): answers the address of the
/// sound data to copy (9 words).
const SOUND_DATA_COPY: u32 = 0x004e_75d0;
/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`), engine map names.
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// `0048cea0` (`thiscall` on the file name component at +0x30): saves it.
const SAVE_FILE_NAME: u32 = 0x0048_cea0;
/// `004858f0` (`cdecl`: chunk, byte): adds a one-byte chunk.
const ADD_BYTE_CHUNK: u32 = 0x0048_58f0;
/// `TESForm::__AddChunkData` (engine map, `00485990`, `cdecl`: chunk, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `00401500` (`cdecl`, no words): whether saved data has its byte order swapped.
const SWAPS_BYTE_ORDER: u32 = 0x0040_1500;
/// The byte-order swaps of a 32-bit word (`00401080`) and a 16-bit word (`00407a90`), `cdecl`
/// (address, a flag byte).
const SWAP_U32: u32 = 0x0040_1080;
const SWAP_U16: u32 = 0x0040_7a90;
/// `0045c650` (`thiscall` on the data handler at `011c3f2c`): the list of sounds.
const SOUND_LIST: u32 = 0x0045_c650;
/// List helpers: `008256d0` (empty), `006815c0` (the item word's address), `00726070` (next).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
/// `00408da0` (`thiscall` on the file name component): its text.
const FILE_NAME_TEXT: u32 = 0x0040_8da0;
/// `00404dc0` (`cdecl`, two texts): `_stricmp`.
const STRICMP: u32 = 0x0040_4dc0;

// Translated from 005e2d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a form; the sound's copy by its body): when the form is a
/// `TESSound`, takes its byte at +0x68, copies all components (`TESForm::CopyAllComponents`
/// with the form as given) and the 9 words of sound data the source's `004e75d0` gives into
/// +0x44.
pub fn fn_005e2d20(e: &mut Engine, this: Ptr, form: u32) {
    let base = this.addr();
    let source = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_SOUND, 0u32],
        )
        .u32();
    if source == 0 {
        return;
    }
    let flags = e.mem.u8(source.wrapping_add(0x68));
    e.mem.set_u8(base.wrapping_add(0x68), flags);
    e.call(COPY_ALL_COMPONENTS, &args![this, form]);
    e.with_stack(0x24, |e, buffer| {
        let data = e.call(SOUND_DATA_COPY, &args![source, buffer]).u32();
        for i in 0..9u32 {
            let word = e.mem.u32(data.wrapping_add(4 * i));
            e.mem.set_u32(base.wrapping_add(0x44 + 4 * i), word);
        }
    });
}

// Translated from 005e2d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a form; the sound's comparison by its body): true
/// ("different") unless the form is a `TESSound` with the same byte at +0x68, the same five
/// 16-bit words at +0x50, word +0x48, bytes +0x45, +0x44 and +0x46, 16-bit words +0x4C, +0x4E
/// and +0x5A, words +0x5C, +0x60 and +0x64, and `TESForm::CompareAllComponents` false.
pub fn fn_005e2d90(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let mine = this.addr();
    let theirs = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_SOUND, 0u32],
        )
        .u32();
    if theirs == 0 {
        return true;
    }
    if e.mem.i8(theirs.wrapping_add(0x68)) != e.mem.i8(mine.wrapping_add(0x68)) {
        return true;
    }
    let mut index = 0u32;
    while index < 5 {
        let at = 0x50 + index * 2;
        if e.mem.u16(theirs.wrapping_add(at)) != e.mem.u16(mine.wrapping_add(at)) {
            return true;
        }
        index += 1;
    }
    let same_u32 = |e: &Engine, at: u32| {
        e.mem.u32(theirs.wrapping_add(at)) == e.mem.u32(mine.wrapping_add(at))
    };
    let same_u16 = |e: &Engine, at: u32| {
        e.mem.u16(theirs.wrapping_add(at)) == e.mem.u16(mine.wrapping_add(at))
    };
    let same_u8 =
        |e: &Engine, at: u32| e.mem.u8(theirs.wrapping_add(at)) == e.mem.u8(mine.wrapping_add(at));
    let same = same_u32(e, 0x48)
        && same_u8(e, 0x45)
        && same_u8(e, 0x44)
        && same_u8(e, 0x46)
        && same_u16(e, 0x4c)
        && same_u16(e, 0x4e)
        && same_u16(e, 0x5a)
        && same_u32(e, 0x5c)
        && same_u32(e, 0x60)
        && same_u32(e, 0x64);
    if !same {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, theirs]).bool()
}

/// When saved data is byte-swapped (`00401500`): `005e3020` on the sound data (+0x44).
fn swap_sound_data(e: &mut Engine, base: u32) {
    if e.call(SWAPS_BYTE_ORDER, &[]).bool() {
        fn_005e3020(e, Ptr::new(base.wrapping_add(0x44)));
    }
}

// Translated from 005e2fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX`, plain `RET`; the sound's save by its body): writes the record:
/// the file name (`0048cea0` on +0x30), `RNAM` (the byte at +0x68), `SNDD` (0x24 bytes at
/// +0x44, byte-swapped by `005e3020` before and after when `00401500` says so).
pub fn fn_005e2fa0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(SAVE_FILE_NAME, &args![base.wrapping_add(0x30)]);
    let flags = u32::from(e.mem.u8(base.wrapping_add(0x68)));
    e.call(ADD_BYTE_CHUNK, &args![RNAM, flags]);
    swap_sound_data(e, base);
    e.call(
        ADD_CHUNK_DATA,
        &args![SNDD, base.wrapping_add(0x44), 0x24u32],
    );
    swap_sound_data(e, base);
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 005e3020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = the 0x24-byte sound data): swaps the byte order of its fields:
/// the word at +4, the 16-bit words at +8 to +0x16, the words at +0x18, +0x1C and +0x20
/// (`00401080`, `00407a90`, each with flag 0).
pub fn fn_005e3020(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(SWAP_U32, &args![base.wrapping_add(4), 0u32]);
    for at in (8..0x18).step_by(2) {
        e.call(SWAP_U16, &args![base.wrapping_add(at), 0u32]);
    }
    for at in [0x18u32, 0x1c, 0x20] {
        e.call(SWAP_U32, &args![base.wrapping_add(at), 0u32]);
    }
}

// Translated from 005e3510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX` = the 0xC-byte `SNDX` data): swaps the byte order of the word at
/// +4 and the 16-bit words at +8 and +0xA (flag 0).
pub fn fn_005e3510(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(SWAP_U32, &args![base.wrapping_add(4), 0u32]);
    e.call(SWAP_U16, &args![base.wrapping_add(8), 0u32]);
    e.call(SWAP_U16, &args![base.wrapping_add(0xa), 0u32]);
}

// Translated from 005e3570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`cdecl`, a file name; `creaturesounds.rs` calls it the sound for a file
/// name): the first sound of the data handler's list (`0045c650`, until the list is empty) whose
/// file name (`00408da0` on +0x30) is set and equals the name (`_stricmp`), else 0; 0 for a
/// null name.
pub fn fn_005e3570(e: &mut Engine, name: u32) -> u32 {
    if name == 0 {
        return 0;
    }
    let handler = e.global::<u32>(DATA_HANDLER);
    let mut node = e.call(SOUND_LIST, &args![handler]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let at = e.call(LIST_ITEM, &args![node]).u32();
        let sound = e.mem.u32(at);
        if e.call(FILE_NAME_TEXT, &args![sound.wrapping_add(0x30)])
            .u32()
            != 0
        {
            let text = e
                .call(FILE_NAME_TEXT, &args![sound.wrapping_add(0x30)])
                .u32();
            if e.call(STRICMP, &args![text, name]).u32() == 0 {
                return sound;
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    0
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_2b70, tessound_tessound(Ptr) -> Ptr),
        entry!(0x005e_2ef0, fn_005e2ef0(Ptr)),
        entry!(0x005e_3160, fn_005e3160(Ptr, Ptr) -> bool),
        entry!(0x005e_2c70, fn_005e2c70(Ptr, u32) -> Ptr),
        entry!(0x005e_2c10, fn_005e2c10(Ptr, Ptr) -> bool),
        entry!(0x005e_2c30, fn_005e2c30(Ptr) -> u32),
        entry!(0x005e_2b50, fn_005e2b50(i32) -> u32),
        entry!(0x005e_2a70, tes_sound_get_pass_through_sound(i32) -> u32),
        entry!(0x005e_3600, tes_sound_generate_default_objects()),
        entry!(0x005e_2ca0, fn_005e2ca0(Ptr)),
        entry!(0x005e_2d20, fn_005e2d20(Ptr, u32)),
        entry!(0x005e_2d90, fn_005e2d90(Ptr, u32) -> bool),
        entry!(0x005e_2fa0, fn_005e2fa0(Ptr)),
        entry!(0x005e_3020, fn_005e3020(Ptr)),
        entry!(0x005e_3510, fn_005e3510(Ptr)),
        entry!(0x005e_3570, fn_005e3570(u32) -> u32),
    ]
}

#[cfg(test)]
mod sound_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_005e2c10_sets_the_editor_id() {
        let mut e = Engine::new();
        e.register(STRING_SET_004037F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_2c10, &args![0x5000u32, 0x7000u32]).bool());
        assert_eq!(
            calls(&e, STRING_SET_004037F0),
            vec![vec![0x503c, 0x7000, 0]]
        );
    }

    #[test]
    fn fn_005e2c30_answers_the_text_or_empty() {
        let mut e = Engine::new();
        e.register(STRING_CHARACTERS_00559450, |e, a| {
            e.mem.u32(a[0]).into_ret()
        });
        let this = e.mem.alloc(0x6c);
        assert_eq!(e.call(0x005e_2c30, &args![this]).u32(), EMPTY_TEXT);
        e.mem.set_u32(this + 0x3c, 0x0abc_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_2c30, &args![this]).u32(), 0x0abc_0000);
        assert_eq!(calls(&e, STRING_CHARACTERS_00559450).len(), 2);
    }

    #[test]
    fn fn_005e2b50_reads_the_table_in_range() {
        let mut e = Engine::new();
        e.map(0x0119_6000, 0x1000);
        for i in 0..11u32 {
            e.mem.set_u32(NAME_TABLE + i * 4, 0x100 + i);
        }
        assert_eq!(e.call(0x005e_2b50, &args![0u32]).u32(), 0x100);
        assert_eq!(e.call(0x005e_2b50, &args![9u32]).u32(), 0x109);
        assert_eq!(e.call(0x005e_2b50, &args![10u32]).u32(), 0);
        assert_eq!(e.call(0x005e_2b50, &args![u32::MAX]).u32(), 0);
    }

    fn pass_through_rig(player: u32, sneaking: u32, running: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0119_6000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(NAME_TABLE + 8, 0x0222_0000);
        e.set_global(PLAYER, player);
        e.set_global(DATA_HANDLER, 0x0444_0000u32);
        e.register_double(IS_SNEAKING_004997B0, move |_, _| sneaking.into_ret());
        e.register_double(IS_RUNNING, move |_, _| running.into_ret());
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.register(GET_SOUND_BY_NAME, |_, _| 0x0555_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_005e2a70_names_the_brush_sound_by_movement() {
        for (sneaking, running, movement) in [
            (1u32, 1u32, TEXT_SNEAK),
            (0x100, 1, TEXT_RUN),
            (0, 0, TEXT_WALK),
        ] {
            let mut e = pass_through_rig(0x0333_0000, sneaking, running);
            assert_eq!(e.call(0x005e_2a70, &args![2u32]).u32(), 0x0555_0000);
            let printed = calls(&e, SPRINTF_S);
            assert_eq!(printed.len(), 1);
            assert_eq!(
                printed[0][1..],
                [0x28, FORMAT_BRUSH_MOVE, 0x0222_0000, movement]
            );
            assert_eq!(
                calls(&e, GET_SOUND_BY_NAME),
                vec![vec![0x0444_0000, printed[0][0]]]
            );
            assert_eq!(calls(&e, IS_SNEAKING_004997B0), vec![vec![0x0333_0000]]);
        }
    }

    #[test]
    fn fn_005e2a70_without_a_player_walks() {
        let mut e = pass_through_rig(0, 1, 1);
        assert_eq!(e.call(0x005e_2a70, &args![2u32]).u32(), 0x0555_0000);
        let printed = calls(&e, SPRINTF_S);
        assert_eq!(printed[0][1..], [0x28, FORMAT_BRUSH_WALK, 0x0222_0000]);
        assert!(calls(&e, IS_SNEAKING_004997B0).is_empty());
    }

    #[test]
    fn fn_005e3600_sets_and_restores_the_guard() {
        let mut e = Engine::new();
        e.register(SETTING_GUARD, |_, _| Ret::default());
        e.register(SETTING_GUARD_RESTORE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005e_3600, &args![]);
        let guard = calls(&e, SETTING_GUARD);
        assert_eq!(guard.len(), 1);
        assert_eq!(guard[0][1..], [0x2f, 1, SOURCE_FILE_NAME, 0x559]);
        assert_eq!(calls(&e, SETTING_GUARD_RESTORE), vec![vec![guard[0][0]]]);
    }

    #[test]
    fn fn_005e2ca0_destroys_last_first() {
        let mut e = Engine::new();
        for addr in [0x0040_37d0, 0x0059_1300, 0x0050_f740] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x6c);
        e.call_log = Some(vec![]);
        e.call(0x005e_2ca0, &args![this]);
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
                (0x0040_37d0, vec![this + 0x3c]),
                (0x0059_1300, vec![this + 0x30]),
                (0x0050_f740, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0104_4ffc);
        assert_eq!(e.mem.u32(this + 0x30), 0x0104_4fe4);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tessound_tessound_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_2b70,
            size: 0x6c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0059_12d0, &[A::T(0x30)]),
                (0x0040_37b0, &[A::T(0x3c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xd)]),
                (0x005e_2ef0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_4ffc),
                (0x30, 4, 0x0104_4fe4),
                (0x68, 1, 0x0),
            ],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{self, calls_to, run, stub, tag, FILE};

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a sound form and a file holding `chunks`. `TESForm::LoadData`
    /// (`004861f0`) copies `data` into the buffer it is given; `version_flag` is what `00401680`
    /// answers. The form's virtual slots `+0xe0`, `+0x134` and (component at `+0x30`) `+0x10`
    /// are the doubles `0x0090_0500`, `0x0090_0600` and `0x0090_0700`.
    fn load(form_type: u32, chunks: &[u32], version_flag: bool, data: &[u8]) -> Loaded {
        let mut e = load_test::engine(form_type, chunks, version_flag);
        stub(
            &mut e,
            &[0x0048_5110, 0x005e_3020, 0x005e_3510, 0x0040_7a90],
        );
        let data = data.to_vec();
        e.register_double(0x0048_61f0, move |e, a| {
            let count = data.len().min(a[3] as usize);
            e.mem.write(a[2], &data[..count]);
            Ret::default()
        });
        let this = load_test::object(
            &mut e,
            0x70,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        e.map(0x0900_1000, 0x200);
        e.mem.set_u32(this + 0x30, 0x0900_1000);
        e.mem.set_u32(0x0900_1010, 0x0090_0700);
        stub(&mut e, &[0x0090_0700]);
        let (ret, calls) = run(&mut e, 0x005e_3160, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    fn sound(chunks: &[u32], version_flag: bool, data: &[u8]) -> Loaded {
        load(0xd, chunks, version_flag, data)
    }

    #[test]
    fn fn_005e2ef0_sets_the_start_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.mem.write(this, &[0xcd; 0x70]);
        e.call(0x005e_2ef0, &args![Ptr::<()>::new(this)]);
        let mut want = vec![0xcd_u8; 0x70];
        let mut put =
            |offset: usize, bytes: &[u8]| want[offset..offset + bytes.len()].copy_from_slice(bytes);
        put(0x44, &[0, 0, 0, 0]);
        put(0x48, &0u32.to_le_bytes());
        put(0x4c, &[0; 4]);
        for (offset, value) in [
            (0x50, 100u16),
            (0x52, 0x32),
            (0x54, 0x14),
            (0x56, 5),
            (0x58, 0),
            (0x5a, 0x50),
        ] {
            put(offset, &value.to_le_bytes());
        }
        put(0x5c, &0x80u32.to_le_bytes());
        put(0x60, &[0; 8]);
        assert_eq!(e.mem.bytes(this, 0x70), want);
    }

    #[test]
    fn another_form_type_is_refused_untouched() {
        let (e, this, answer, calls) = load(0xc, &[tag(b"EDID")], false, &[]);
        assert!(!answer);
        assert!(calls.is_empty());
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn the_buffer_chunks_reach_their_virtual_slots() {
        let (_, this, answer, calls) = sound(&[tag(b"FNAM")], false, &[]);
        assert!(answer);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0));
        assert_eq!(
            calls_to(&calls, 0x0090_0700),
            [vec![this + 0x30, read[0][1]]]
        );

        let (_, this, _, calls) = sound(&[tag(b"EDID")], false, &[]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = sound(&[tag(b"OBND")], false, &[]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);
    }

    #[test]
    fn the_small_data_chunks_load_into_their_fields() {
        for (chunk, offset, size) in [
            (tag(b"RNAM"), 0x68, 1),
            (tag(b"GNAM"), 0x5a, 2),
            (tag(b"HNAM"), 0x5c, 4),
        ] {
            let (_, this, _, calls) = sound(&[chunk], false, &[]);
            assert_eq!(
                calls_to(&calls, 0x0048_61f0),
                [vec![this, FILE, this + offset, size]]
            );
        }
    }

    #[test]
    fn the_sound_data_chunk_runs_its_fix_up_only_for_the_flag() {
        let (_, this, _, calls) = sound(&[tag(b"SNDD")], false, &[]);
        assert_eq!(
            calls_to(&calls, 0x0048_61f0),
            [vec![this, FILE, this + 0x44, 0x24]]
        );
        assert!(calls_to(&calls, 0x005e_3020).is_empty());

        let (_, this, _, calls) = sound(&[tag(b"SNDD")], true, &[]);
        assert_eq!(calls_to(&calls, 0x005e_3020), [vec![this + 0x44]]);
    }

    #[test]
    fn the_extended_sound_data_is_split_over_the_form() {
        let data = [
            1, 2, 3, 0xee, 0x44, 0x33, 0x22, 0x11, 0x66, 0x55, 0x88, 0x77,
        ];
        let (e, this, _, calls) = sound(&[tag(b"SNDX")], false, &data);
        assert_eq!(e.mem.u8(this + 0x44), 1);
        assert_eq!(e.mem.u8(this + 0x45), 2);
        assert_eq!(e.mem.u8(this + 0x46), 3);
        assert_eq!(e.mem.u8(this + 0x47), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0x1122_3344);
        assert_eq!(e.mem.u16(this + 0x4c), 0x5566);
        assert_eq!(e.mem.u16(this + 0x4e), 0x7788);
        assert!(calls_to(&calls, 0x005e_3510).is_empty());
        let load = calls_to(&calls, 0x0048_61f0);
        assert_eq!((load[0][0], load[0][1], load[0][3]), (this, FILE, 0xc));

        let (_, _, _, calls) = sound(&[tag(b"SNDX")], true, &data);
        let load = calls_to(&calls, 0x0048_61f0);
        assert_eq!(calls_to(&calls, 0x005e_3510), [vec![load[0][2]]]);
    }

    fn attenuation(words: [u16; 5]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    #[test]
    fn the_old_default_attenuation_is_replaced() {
        let old = attenuation([100, 0x4b, 0x32, 0x19, 0]);
        let (e, this, _, calls) = sound(&[tag(b"ANAM")], false, &old);
        let words: Vec<u16> = (0..5).map(|i| e.mem.u16(this + 0x50 + 2 * i)).collect();
        assert_eq!(words, [100, 0x32, 0x14, 5, 0]);
        assert!(calls_to(&calls, 0x0040_7a90).is_empty());
        assert_eq!(
            calls_to(&calls, 0x0048_61f0),
            [vec![this, FILE, this + 0x50, 10]]
        );
    }

    #[test]
    fn other_attenuation_values_are_kept_and_the_flag_runs_the_clearing_calls() {
        let other = attenuation([100, 0x4c, 0x32, 0x19, 0]);
        let (e, this, _, _) = sound(&[tag(b"ANAM")], false, &other);
        let words: Vec<u16> = (0..5).map(|i| e.mem.u16(this + 0x50 + 2 * i)).collect();
        assert_eq!(words, [100, 0x4c, 0x32, 0x19, 0]);

        let old = attenuation([100, 0x4b, 0x32, 0x19, 0]);
        let (e, this, _, calls) = sound(&[tag(b"ANAM")], true, &old);
        assert_eq!(
            calls_to(&calls, 0x0040_7a90),
            [
                vec![this + 0x50, 0],
                vec![this + 0x52, 0],
                vec![this + 0x54, 0],
                vec![this + 0x56, 0],
                vec![this + 0x58, 0]
            ]
        );
        // The clearing calls are doubles; the old defaults are then still replaced.
        assert_eq!(e.mem.u16(this + 0x52), 0x32);
    }
    #[test]
    fn at_005e2c70_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_2ca0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_2c70, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_2c70)
                .collect();
            let mut expected = vec![(0x005e_2ca0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn sound(e: &mut Engine) -> u32 {
        let s = e.mem.alloc(0x70);
        for i in 0..0x24 {
            e.mem.set_u8(s + 0x44 + i, i as u8 + 1);
        }
        e.mem.set_u8(s + 0x68, 3);
        s
    }

    #[test]
    fn copy_takes_the_flags_components_and_data() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COPY_ALL_COMPONENTS, ok);
        e.register(SOUND_DATA_COPY, |_, a| ret(a[0] + 0x44));
        let source = sound(&mut e);
        let this = e.mem.alloc(0x70);
        e.call(0x005e_2d20, &[this, 0]);
        assert_eq!(e.mem.u8(this + 0x68), 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_2d20, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, COPY_ALL_COMPONENTS), vec![vec![this, source]]);
        assert_eq!(e.mem.u8(this + 0x68), 3);
        assert_eq!(
            e.mem.bytes(this + 0x44, 0x24),
            e.mem.bytes(source + 0x44, 0x24)
        );
    }

    #[test]
    fn compare_checks_each_field() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        let a = sound(&mut e);
        let b = sound(&mut e);
        assert!(e.call(0x005e_2d90, &[a, 0]).bool());
        assert!(!e.call(0x005e_2d90, &[a, b]).bool());
        for at in [
            0x68u32, 0x52, 0x48, 0x45, 0x44, 0x46, 0x4c, 0x4e, 0x5a, 0x5c, 0x60, 0x64,
        ] {
            let old = e.mem.u8(b + at);
            e.mem.set_u8(b + at, old ^ 0x80);
            assert!(e.call(0x005e_2d90, &[a, b]).bool(), "{at:x}");
            e.mem.set_u8(b + at, old);
        }
        // Byte +0x47 is not compared.
        e.mem.set_u8(b + 0x47, 0);
        assert!(!e.call(0x005e_2d90, &[a, b]).bool());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x005e_2d90, &[a, b]).bool());
    }

    #[test]
    fn save_writes_rnam_and_sndd_swapped_when_asked() {
        let mut e = Engine::new();
        for a in [
            START_FORM,
            CLOSE_FORM,
            SAVE_FILE_NAME,
            ADD_BYTE_CHUNK,
            ADD_CHUNK_DATA,
            SWAP_U32,
            SWAP_U16,
        ] {
            e.register(a, ok);
        }
        e.register(SWAPS_BYTE_ORDER, |_, _| ret(0));
        let s = sound(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005e_2fa0, &[s]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, SAVE_FILE_NAME), vec![vec![s + 0x30]]);
        assert_eq!(only(&log, ADD_BYTE_CHUNK), vec![vec![RNAM, 3]]);
        assert_eq!(only(&log, ADD_CHUNK_DATA), vec![vec![SNDD, s + 0x44, 0x24]]);
        assert!(only(&log, SWAP_U32).is_empty());
        e.register(SWAPS_BYTE_ORDER, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x005e_2fa0, &[s]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, SWAP_U32).len(), 8);
        assert_eq!(only(&log, SWAP_U16).len(), 16);
    }

    #[test]
    fn the_swaps_cover_the_data_fields() {
        let mut e = Engine::new();
        e.register(SWAP_U32, ok);
        e.register(SWAP_U16, ok);
        e.call_log = Some(vec![]);
        e.call(0x005e_3020, &[0x1000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, SWAP_U32),
            vec![
                vec![0x1004, 0],
                vec![0x1018, 0],
                vec![0x101c, 0],
                vec![0x1020, 0]
            ]
        );
        let halves: Vec<u32> = only(&log, SWAP_U16).iter().map(|w| w[0]).collect();
        assert_eq!(halves, (0x1008..0x1018).step_by(2).collect::<Vec<u32>>());
        e.call_log = Some(vec![]);
        e.call(0x005e_3510, &[0x2000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, SWAP_U32), vec![vec![0x2004, 0]]);
        assert_eq!(only(&log, SWAP_U16), vec![vec![0x2008, 0], vec![0x200a, 0]]);
    }

    #[test]
    fn sound_for_a_file_name_walks_the_sound_list() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        let head = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let a = e.mem.alloc(0x40);
        let b = e.mem.alloc(0x40);
        e.mem.set_u32(head, a);
        e.mem.set_u32(head + 4, second);
        e.mem.set_u32(second, b);
        e.mem.set_u32(b + 0x30, 0x0300);
        e.set_global::<u32>(DATA_HANDLER, 0x9000);
        e.register_double(SOUND_LIST, move |_, _| ret(head));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(LIST_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(FILE_NAME_TEXT, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRICMP, |_, a| ret(u32::from(a[0] != a[1])));
        assert_eq!(e.call(0x005e_3570, &[0]).u32(), 0);
        assert_eq!(e.call(0x005e_3570, &[0x0300]).u32(), b);
        assert_eq!(e.call(0x005e_3570, &[0x0400]).u32(), 0);
    }
}
