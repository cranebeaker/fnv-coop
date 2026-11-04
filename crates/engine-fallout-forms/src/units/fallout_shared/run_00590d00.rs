//! `fallout shared/run_00590d00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the flag word at `+0x38` of `this` is set. `ShowMessage`
/// calls it on a message form to tell a message box from a plain message;
/// the engine map has no name for it.
pub fn fn_00590d00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x38) & 1 != 0
}

// Translated from 00591200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591200 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x66 (0x34 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_397c`) is `BGSMusicType`, so this is the constructor of `BGSMusicType`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It copies the float at `0103395c` into `+0x24`.
/// The exception-unwinding frame is not translated.
pub fn fn_00591200(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0059_12d0, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_397c);
    e.mem.set_u32(base + 0x18, 0x0103_3964);
    e.call(0x0096_a2d0, &args![base + 0x28]);
    e.call(0x004f_15a0, &args![this, 0x66u32]);
    let f1 = e.global::<f32>(0x0103_395c);
    e.mem.set_f32(base + 0x24, f1);
    this
}

// Translated from 00591b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591b70 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5a (0x20 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_3b34`) is `BGSRadiationStage`, so this is the constructor of `BGSRadiationStage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00591b70(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0103_3b34);
    e.call(0x0040_19f0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x5au32]);
    e.call(0x0040_1b20, &args![this]);
    this
}

// Translated from 005912d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x18` of `BGSMusicType` (`00591200` calls it): runs
/// `00402d30` on the object, stores the vtable `0x0103_3ab8` and runs `004037b0` on the string
/// at `+0x4`. Returns `this`. The engine map gives it no name.
pub fn fn_005912d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(base, 0x0103_3ab8);
    e.call(0x0040_37b0, &args![base + 0x4]);
    this
}

// Translated from 00591140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00591170` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00591140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_1170, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005912a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00591320` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005912a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_1320, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00591bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00591c20` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00591bf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_1c20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The node list helpers the music type and the other lists here use
/// (`BSSimpleList`-shaped, called on the list): whether it is empty
/// (`008256d0`), the head item's slot (`006815c0`, a pointer to the item
/// word), removing the head (`0063f7b0`), the item count (`005ae380`), the
/// next node (`00726070`) and adding an item by address (`005ae3d0`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_HEAD_ITEM: u32 = 0x0068_15c0;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_ADD: u32 = 0x005a_e3d0;
/// Frees a block (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// Allocates a block (`00401000`, cdecl, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// A random integer in `[low, high]` (`00944460`, cdecl, two words).
const RANDOM_INT: u32 = 0x0094_4460;
/// `BSStringT` text and length (`00408da0`, `0048cee0`).
const STRING_TEXT: u32 = 0x0040_8da0;
const STRING_LENGTH: u32 = 0x0048_cee0;
/// `sprintf_s` (`00406d00`, cdecl, variable arguments).
const SPRINTF_S: u32 = 0x0040_6d00;
/// `"%s\%s"` and `"Data\Music"`.
const PATH_FORMAT: u32 = 0x0101_dcc4;
const MUSIC_FOLDER: u32 = 0x0103_3acc;
/// `"%s%s"`.
const CONCAT_FORMAT: u32 = 0x0101_996c;
/// `".mp3"`.
const MP3_EXTENSION: u32 = 0x0103_3ad8;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\BGSMusicType.cpp"`.
const MUSIC_TYPE_SOURCE: u32 = 0x0103_3ae0;

/// `TESFile` chunk reading: the record's form type byte (`00472660`), the
/// current chunk's type (`004726b0`, 0 at the end), the step to the next
/// chunk (`004726f0`, false at the end), the chunk's size (`00401660`), the
/// chunk's data into a buffer (`004727f0`, one word) or with a size
/// (`00472890`, two words), and whether the file needs its words swapped
/// (`00401680`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
/// `TESForm::LoadForm` (`00485110`), the flag setter `00484ab0` and
/// `TESForm::LoadData` (`004861f0`: file, buffer, size).
const FORM_LOAD_FORM: u32 = 0x0048_5110;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
const FORM_LOAD_DATA: u32 = 0x0048_61f0;
/// `004013e0`: true makes `InitItem` do nothing.
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
/// `TESForm::GetFile` (`00484e60`), `TESForm::AddCompileIndex` (`00485d50`,
/// cdecl), the form lookup by id (`004839c0`, cdecl) and `__RTDynamicCast`
/// (`00ec43fb`, cdecl, five words) from `.?AVTESForm@@` (`01183028`) to
/// `.?AVSpellItem@@` (`01183060`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_SPELL_ITEM: u32 = 0x0118_3060;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FNAM: u32 = 0x4d41_4e46;
const CHUNK_ANAM: u32 = 0x4d41_4e41;
const CHUNK_DATA: u32 = 0x4154_4144;

/// The radiation stages sorted by threshold: the count (a word, rebuilt
/// from a byte when a stage is added) and the array of stage pointers.
const RADIATION_STAGE_COUNT: u32 = 0x011c_a87c;
const RADIATION_STAGES: u32 = 0x011c_a880;
/// Allocates an array of `n` pointers (`004024e0`, cdecl, one word).
const ALLOCATE_POINTERS: u32 = 0x0040_24e0;
/// The radiation stage's `DATA` (`00500940`, on the stage): a pointer to its
/// first word, the threshold the list is sorted by.
const RADIATION_STAGE_DATA: u32 = 0x0050_0940;

// Translated from 00590ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of the flag word at `+0x38` of `this` is set (the message
/// form's flags, as `00590d00` reads bit 0). The engine map has no name.
pub fn fn_00590ed0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x38) & 2 != 0
}

// Translated from 005910d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list at `+0x30`: takes each head item off and deletes it
/// (`00591140` with flag 1) when it is not null.
pub fn fn_005910d0(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 0x30;
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let slot = e.call(LIST_HEAD_ITEM, &args![list]).u32();
        let item = e.mem.u32(slot);
        e.call(LIST_REMOVE_HEAD, &args![list]);
        if item != 0 {
            fn_00591140(e, Ptr::new(item), 1);
        }
    }
}

// Translated from 00591170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591170 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body that `00591140` runs: `006808b0` on `+0x8`, then the
/// string destructor `004037d0` on `this`.
pub fn fn_00591170(e: &mut Engine, this: Ptr) {
    e.call(0x0068_08b0, &args![this.addr() + 8]);
    e.call(0x0040_37d0, &args![this]);
}

// Translated from 005911d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `item` to the list at `+0x30` (`00905820`, given the item's
/// address) when it is not null.
pub fn fn_005911d0(e: &mut Engine, this: Ptr, item: u32) {
    if item != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(0x0090_5820, &args![this.addr() + 0x30, slot]);
        });
    }
}

// Translated from 00591300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the `BGSMusicType` component at `+0x18` (built by
/// `005912d0`): the string destructor `004037d0` on its `+0x4`.
pub fn fn_00591300(e: &mut Engine, this: Ptr) {
    e.call(0x0040_37d0, &args![this.addr() + 4]);
}

// Translated from 00591320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591320 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `BGSMusicType` (the vtables `00591200` stores): puts
/// back the vtables, deletes the file names (`005913b0`), destroys the list
/// at `+0x28` (`0046ffb0`) and the component at `+0x18` (`00591300`), then
/// the `TESForm` destructor (`00483630`).
pub fn fn_00591320(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_397c);
    e.mem.set_u32(base + 0x18, 0x0103_3964);
    fn_005913b0(e, this);
    e.call(0x0046_ffb0, &args![base + 0x28]);
    fn_00591300(e, Ptr::new(base + 0x18));
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005913b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the music type's file list at `+0x28`, freeing each head item
/// (`00401030`) as it is taken off.
pub fn fn_005913b0(e: &mut Engine, this: Ptr) {
    let list = this.addr() + 0x28;
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let slot = e.call(LIST_HEAD_ITEM, &args![list]).u32();
        let item = e.mem.u32(slot);
        e.call(LIST_REMOVE_HEAD, &args![list]);
        e.call(FREE, &args![item]);
    }
}

// Translated from 00591410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591410 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSMusicType`'s `InitItem` (vtable slot of the form; the engine map has
/// no name): unless `004013e0` answers true, with a file name longer than
/// two characters (the component at `+0x18`) either collects the folder's
/// files (`005917b0`, when the name ends in `'\'`) or sets the name to
/// `"Data\Music\<name>"` (the component's vtable +0x10); then `+0x30` is a
/// random index into the list at `+0x28` (0 when it is empty), and
/// `00484ab0(1)`.
pub fn fn_00591410(e: &mut Engine, this: Ptr) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    let name = base + 0x18;
    if e.call(STRING_LENGTH, &args![name]).u32() > 2 {
        let text = e.call(STRING_TEXT, &args![name]).u32();
        let length = e.call(STRING_LENGTH, &args![name]).u32();
        if e.mem.u8(text.wrapping_add(length).wrapping_sub(1)) as i8 == 0x5c {
            fn_005917b0(e, this);
        } else {
            let text = e.call(STRING_TEXT, &args![name]).u32();
            e.with_stack(0x104, |e, path| {
                e.call(
                    SPRINTF_S,
                    &args![path, 0x104u32, PATH_FORMAT, MUSIC_FOLDER, text],
                );
                e.vcall(name, 0x10, &args![path]);
            });
        }
    }
    let list = base + 0x28;
    let index = if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        0
    } else {
        let count = e.call(LIST_COUNT, &args![list]).u32();
        e.call(RANDOM_INT, &args![0u32, count.wrapping_sub(1)])
            .u32()
    };
    e.mem.set_u32(base + 0x30, index);
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 00591550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591550 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSMusicType`'s record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): `TESForm::LoadForm`, `00484ab0(0)`, then each chunk:
/// `EDID` (vtable +0x134 with the text), `OBND` (vtable +0xE0 with the
/// file), `FNAM` (the file name, through the component's vtable +0x10) and
/// `ANAM` (the float at `+0x24`, 0.0 before the read). Returns true.
pub fn fn_00591550(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_ANAM => {
                let value = e.with_stack(4, |e, slot| {
                    e.mem.set_f32(slot.addr(), 0.0);
                    e.call(FILE_CHUNK_DATA, &args![file, slot]);
                    e.mem.f32(slot.addr())
                });
                // FLD/FSTP through the x87 stack.
                e.mem.set_f32(base + 0x24, x87_quiet(value));
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_FNAM => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0u32]);
                    e.vcall(base + 0x18, 0x10, &args![text]);
                });
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005917b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005917b0 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `BGSMusicType`'s folder scan (the engine map has no name): under a
/// memory scope (`00404eb0` with 0xB, 1, the source file name and line
/// 0x160), walks `"Data\Music\<name>"` with the directory walker
/// (`00b00820`, next `00b00720` until it finishes or fails) and, for each
/// entry `00591a20` does not reject whose name (`00436aa0`) has the
/// extension `".mp3"` (`_splitpath_s` `00462d40`, `_stricmp` `00404dc0`),
/// adds a new file entry (`00591a00` on 0x104 allocated bytes, with
/// `"<folder><name>"`) to the list at `+0x28`.
pub fn fn_005917b0(e: &mut Engine, this: Ptr) {
    let scope = e.mem.alloc(4);
    let folder = e.mem.alloc(0x104);
    let full = e.mem.alloc(0x104);
    let extension = e.mem.alloc(0x104);
    let walker = e.mem.alloc(0x258);
    let finished = e.mem.alloc(4);
    let new_item = e.mem.alloc(4);
    e.call(
        0x0040_4eb0,
        &args![scope, 0xbu32, 1u32, MUSIC_TYPE_SOURCE, 0x160u32],
    );
    let name = e.call(STRING_TEXT, &args![this.addr() + 0x18]).u32();
    e.call(
        SPRINTF_S,
        &args![folder, 0x104u32, PATH_FORMAT, MUSIC_FOLDER, name],
    );
    let wanted = MP3_EXTENSION;
    e.mem.set_u8(finished, 0);
    e.call(0x00b0_0820, &args![walker, folder]);
    let mut status = e.call(0x00b0_0720, &args![walker, finished]).u32();
    while e.mem.u8(finished) == 0 && status == 0 {
        if !e.call(0x0059_1a20, &args![walker]).bool()
            && e.call(0x0043_6aa0, &args![walker]).u32() != 0
        {
            let file_name = e.call(0x0043_6aa0, &args![walker]).u32();
            e.call(
                0x0046_2d40,
                &args![file_name, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, extension, 0x104u32],
            );
            if e.call(0x0040_4dc0, &args![extension, wanted]).u32() == 0 {
                let file_name = e.call(0x0043_6aa0, &args![walker]).u32();
                e.call(
                    SPRINTF_S,
                    &args![full, 0x104u32, CONCAT_FORMAT, folder, file_name],
                );
                let block = e.call(ALLOCATE, &args![0x104u32]).u32();
                let item = if block == 0 {
                    0
                } else {
                    e.call(0x0059_1a00, &args![block, full]).u32()
                };
                e.mem.set_u32(new_item, item);
                e.call(LIST_ADD, &args![this.addr() + 0x28, new_item]);
            }
        }
        status = e.call(0x00b0_0720, &args![walker, finished]).u32();
    }
    e.call(0x00b0_0860, &args![walker]);
    e.call(0x0040_4ee0, &args![scope]);
    for block in [scope, folder, full, extension, walker, finished, new_item] {
        e.mem.free(block);
    }
}

// Translated from 00591a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMusicType`'s file to play (the engine map has no name): the
/// component's name when the list at `+0x28` is empty with a count of 0;
/// the only entry when there is one; otherwise a random entry
/// (`00944460(0, count)`) that differs from the last one kept at `+0x30`
/// (a repeat of a non-zero index becomes 0, a repeat of 0 becomes 1),
/// falling back to the name when the walk runs off the list or lands on
/// an empty slot.
pub fn fn_00591a40(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let list = base + 0x28;
    let count = e.call(LIST_COUNT, &args![list]).u32();
    if e.call(LIST_IS_EMPTY, &args![list]).bool() && count == 0 {
        return e.call(STRING_TEXT, &args![base + 0x18]).u32();
    }
    if count == 1 {
        let slot = e.call(LIST_HEAD_ITEM, &args![list]).u32();
        return e.mem.u32(slot);
    }
    let mut node = list;
    let count = e.call(LIST_COUNT, &args![list]).u32();
    let mut index = e.call(RANDOM_INT, &args![0u32, count]).u32();
    let last = e.mem.u32(base + 0x30);
    if last != 0 && last == index {
        index = 0;
        e.mem.set_u32(base + 0x30, index);
    } else if last == 0 && last == index {
        index = 1;
        e.mem.set_u32(base + 0x30, index);
    } else {
        e.mem.set_u32(base + 0x30, index);
    }
    while node != 0 {
        let before = index;
        index = index.wrapping_sub(1);
        if before == 0 {
            break;
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    if node != 0 {
        let slot = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        if e.mem.u32(slot) != 0 {
            let slot = e.call(LIST_HEAD_ITEM, &args![node]).u32();
            return e.mem.u32(slot);
        }
    }
    e.call(STRING_TEXT, &args![base + 0x18]).u32()
}

// Translated from 00591c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591c20 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `BGSRadiationStage` (the vtable `00591b70` stores):
/// puts back the vtable, takes the stage out of the sorted stage list
/// (`00591c80`), then the `TESForm` destructor (`00483630`).
pub fn fn_00591c20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0103_3b34);
    fn_00591c80(e, this);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00591c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the radiation stage `this` out of the sorted stage list
/// (`00592110`).
pub fn fn_00591c80(e: &mut Engine, this: Ptr) {
    fn_00592110(e, this.addr());
}

// Translated from 00591ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRadiationStage`'s `InitItem` (the engine map has no name): unless
/// `004013e0` answers true, resolves the effect's form id (`00441110`; the
/// id adjusted to the stage's file, looked up and cast to `SpellItem`) and
/// sets it (`0050f9a0`, given the cast, null when not found), then
/// `00484ab0(1)` and puts the stage into the sorted stage list
/// (`00592010`).
pub fn fn_00591ca0(e: &mut Engine, this: Ptr) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    let id = e.call(0x0044_1110, &args![this]).u32();
    if id != 0 {
        let id = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), id);
            let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            e.mem.u32(slot.addr())
        });
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let spell = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0u32],
            )
            .u32();
        e.call(0x0050_f9a0, &args![this, spell]);
    }
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
    fn_00592010(e, this.addr());
}

// Translated from 00591ec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00591ec0 - the /GS stack cookie check (00ec408c) is not translated.
/// `BGSRadiationStage`'s record reader (`RET 4`: the `TESFile`; the engine
/// map has no name): false unless the record's form type byte is `'Z'`;
/// otherwise vtable +0x18, `TESForm::LoadForm`, `00484ab0(0)` and each
/// chunk: `DATA` (8 bytes into the stage's data, `TESForm::LoadData`, then
/// `00462230` on `+0x18` when the file asks for swapped words), `EDID`
/// (vtable +0x134 with the text) and `OBND` (vtable +0xE0). Returns true.
pub fn fn_00591ec0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    if form_type != 0x5a {
        return false;
    }
    let base = this.addr();
    e.vcall(base, 0x18, &args![]);
    e.call(FORM_LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_DATA => {
                let data = e.call(RADIATION_STAGE_DATA, &args![this]).u32();
                e.call(FORM_LOAD_DATA, &args![this, file, data, 8u32]);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                    e.vcall(base, 0x134, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            _ => {}
        }
        if chunk == CHUNK_DATA && e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
            e.call(0x0046_2230, &args![base + 0x18]);
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 00592010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts the radiation stage `stage` into the sorted stage list: a new
/// array one longer (the count kept as a byte), `stage` placed before the
/// first stage whose threshold (the first `DATA` word) is lower, or at the
/// end; the old array is freed.
pub fn fn_00592010(e: &mut Engine, stage: u32) {
    let new_count = e.global::<u32>(RADIATION_STAGE_COUNT).wrapping_add(1) as u8;
    let array = e
        .call(ALLOCATE_POINTERS, &args![u32::from(new_count)])
        .u32();
    let mut out = 0u32;
    let mut placed = false;
    let mut i = 0u32;
    while i < e.global::<u32>(RADIATION_STAGE_COUNT) {
        let mine = e.call(RADIATION_STAGE_DATA, &args![stage]).u32();
        let other_stage = e.mem.u32(
            e.global::<u32>(RADIATION_STAGES)
                .wrapping_add(i.wrapping_mul(4)),
        );
        let other = e.call(RADIATION_STAGE_DATA, &args![other_stage]).u32();
        if e.mem.u32(mine) > e.mem.u32(other) && !placed {
            e.mem
                .set_u32(array.wrapping_add(out.wrapping_mul(4)), stage);
            out = out.wrapping_add(1);
            placed = true;
        }
        let other_stage = e.mem.u32(
            e.global::<u32>(RADIATION_STAGES)
                .wrapping_add(i.wrapping_mul(4)),
        );
        e.mem
            .set_u32(array.wrapping_add(out.wrapping_mul(4)), other_stage);
        out = out.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    if !placed {
        e.mem
            .set_u32(array.wrapping_add(out.wrapping_mul(4)), stage);
    }
    let old = e.global::<u32>(RADIATION_STAGES);
    e.call(FREE, &args![old]);
    e.set_global(RADIATION_STAGE_COUNT, u32::from(new_count));
    e.set_global(RADIATION_STAGES, array);
}

// Translated from 00592110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the radiation stage `stage` from the sorted stage list: when it
/// is there, a new array one shorter (`004024e0`) gets every other stage in
/// order (nothing is copied when that allocation fails), the old array is
/// freed and the count lowered.
pub fn fn_00592110(e: &mut Engine, stage: u32) {
    if e.global::<u32>(RADIATION_STAGE_COUNT) == 0 {
        return;
    }
    let mut found = false;
    let mut i = 0u32;
    while !found && i < e.global::<u32>(RADIATION_STAGE_COUNT) {
        let entry = e.mem.u32(
            e.global::<u32>(RADIATION_STAGES)
                .wrapping_add(i.wrapping_mul(4)),
        );
        if entry == stage {
            found = true;
        }
        i = i.wrapping_add(1);
    }
    if !found {
        return;
    }
    let new_count = e.global::<u32>(RADIATION_STAGE_COUNT).wrapping_sub(1);
    let array = e.call(ALLOCATE_POINTERS, &args![new_count]).u32();
    if array != 0 {
        let mut out = 0u32;
        let mut i = 0u32;
        while i < e.global::<u32>(RADIATION_STAGE_COUNT) {
            let entry = e.mem.u32(
                e.global::<u32>(RADIATION_STAGES)
                    .wrapping_add(i.wrapping_mul(4)),
            );
            if entry != stage {
                e.mem
                    .set_u32(array.wrapping_add(out.wrapping_mul(4)), entry);
                out = out.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    let old = e.global::<u32>(RADIATION_STAGES);
    e.call(FREE, &args![old]);
    e.set_global(RADIATION_STAGE_COUNT, new_count);
    e.set_global(RADIATION_STAGES, array);
}

/// The message form's getters and setters `00590d20` and `00590f40` use
/// (all `thiscall`; the setters take one word): `0055b980`/`0086cf00`,
/// `00639b40`/`00409480`, and `00436aa0` (the first node of the form's
/// item list).
const MESSAGE_GET_0055B980: u32 = 0x0055_b980;
const MESSAGE_SET_0086CF00: u32 = 0x0086_cf00;
const MESSAGE_GET_00639B40: u32 = 0x0063_9b40;
const MESSAGE_SET_00409480: u32 = 0x0040_9480;
const MESSAGE_FIRST_NODE: u32 = 0x0043_6aa0;
/// The 0x10-byte item constructor `00590b60` (`bgsmessage.cpp`), its text
/// copy `00501c10` and comparison `00501c30` (`thiscall`, one word), and
/// its conditions' copy `00680a70` (`thiscall`, two words) and comparison
/// `00680b70` (`thiscall`, one word) (`tescondition.cpp`).
const ITEM_CONSTRUCT: u32 = 0x0059_0b60;
const ITEM_TEXT_COPY: u32 = 0x0050_1c10;
const ITEM_TEXT_DIFFERS: u32 = 0x0050_1c30;
const CONDITIONS_COPY: u32 = 0x0068_0a70;
const CONDITIONS_DIFFER: u32 = 0x0068_0b70;
/// `TESForm::CopyAllComponents`, `CompareAllComponents`, `StartForm`,
/// `CloseForm` and `__SaveData` (`004851b0`, `00485270`, `004855a0`,
/// `00485680`, `00485e20`; engine map), the chunk writer `00485910`
/// (`cdecl`: name, word), `0048cea0` (`thiscall` on the name member at
/// +0x18), the byte copy `00401460` and `memcmp` (`00ec4835`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const SAVE_DATA: u32 = 0x0048_5e20;
const WRITE_CHUNK_WORD: u32 = 0x0048_5910;
const SAVE_NAME_0048CEA0: u32 = 0x0048_cea0;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// The type descriptors `.?AVBGSMusicType@@` (`011840fc`) and
/// `.?AVBGSRadiationStage@@` (`01186180`).
const TYPE_MUSIC_TYPE: u32 = 0x0118_40fc;
const TYPE_RADIATION_STAGE: u32 = 0x0118_6180;
/// `004046f0` (`cdecl`, two words).
const CDECL_004046F0: u32 = 0x0040_46f0;
/// The pieces of `00591e10`: `00441110` (`thiscall`: the form saved in the
/// record's place; the engine map's name `PathingLocation::GetWorldspace`
/// is a folded body's), `0084e3a0` (`thiscall`: its save value), `0050f9a0`
/// (`thiscall`, one word: stores it back), `00401500` (no argument, a byte:
/// whether the data is byte-swapped around the save) and `00462230`
/// (`thiscall` on the data: the swap).
const RECORD_FORM: u32 = 0x0044_1110;
const FORM_SAVE_VALUE: u32 = 0x0084_e3a0;
const SET_RECORD_FORM: u32 = 0x0050_f9a0;
const SWAPS_DATA: u32 = 0x0040_1500;
const SWAP_DATA: u32 = 0x0046_2230;

/// `form` cast from `TESForm` to `target` (`__RTDynamicCast`), or 0.
fn cast_form(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

// Translated from 00590d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590d20 - the FS:[0] exception frame (freeing a new item when its constructor throws) is not translated
/// The message form's copy (`thiscall`, one word, `RET 4`; `source` is not
/// cast): its own slot +0x18 (clearing it), `0086cf00(0055b980)`, the two
/// flags (`00590d00`, `00590ed0` to `00590e80`, `00590ef0`),
/// `00409480(00639b40)`, then a new item (`00590b60` on 0x10 allocated
/// bytes, null when the allocation fails) for each node of the source's
/// item list up to the first empty one (`008256d0`), its text (`00501c10`)
/// and conditions (`00680a70` with 0) copied from the node's item and added
/// (`005911d0`); last `TESForm::CopyAllComponents(source)`.
pub fn fn_00590d20(e: &mut Engine, this: Ptr, source: u32) {
    e.vcall(this.addr(), 0x18, &args![]);
    let value = e.call(MESSAGE_GET_0055B980, &args![source]).u32();
    e.call(MESSAGE_SET_0086CF00, &args![this, value]);
    let flag = u8::from(fn_00590d00(e, Ptr::new(source)));
    fn_00590e80(e, this, flag);
    let flag = u8::from(fn_00590ed0(e, Ptr::new(source)));
    fn_00590ef0(e, this, flag);
    let value = e.call(MESSAGE_GET_00639B40, &args![source]).u32();
    e.call(MESSAGE_SET_00409480, &args![this, value]);
    let mut node = e.call(MESSAGE_FIRST_NODE, &args![source]).u32();
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        let item = e.mem.u32(slot);
        node = e.call(LIST_NEXT, &args![node]).u32();
        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
        let copy = if block == 0 {
            0
        } else {
            e.call(ITEM_CONSTRUCT, &args![block]).u32()
        };
        e.call(ITEM_TEXT_COPY, &args![copy, item]);
        e.call(
            CONDITIONS_COPY,
            &args![copy.wrapping_add(8), item.wrapping_add(8), 0u32],
        );
        fn_005911d0(e, this, copy);
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
}

// Translated from 00590e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit 0x1 of the flag word at +0x38;
/// `RET 4`.
pub fn fn_00590e80(e: &mut Engine, this: Ptr, set: u8) {
    let at = this.addr() + 0x38;
    let flags = e.mem.u32(at);
    let flags = if set != 0 {
        flags | 1
    } else {
        flags & 0xffff_fffe
    };
    e.mem.set_u32(at, flags);
}

// Translated from 00590ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit 0x2 of the flag word at +0x38;
/// `RET 4`.
pub fn fn_00590ef0(e: &mut Engine, this: Ptr, set: u8) {
    let at = this.addr() + 0x38;
    let flags = e.mem.u32(at);
    let flags = if set != 0 {
        flags | 2
    } else {
        flags & 0xffff_fffd
    };
    e.mem.set_u32(at, flags);
}

// Translated from 00590f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The message form's comparison (`thiscall`, one word, `RET 4`; `form` is
/// not cast): true for a null form, when `0055b980`, the two flags,
/// `00639b40` or the item counts (`005ae380` on both first nodes) differ,
/// when a pair of items (walked together while both nodes hold one) has
/// different texts (`00501c30`) or conditions (`00680b70`); else what
/// `TESForm::CompareAllComponents` answers.
pub fn fn_00590f40(e: &mut Engine, this: Ptr, form: u32) -> bool {
    if form == 0 {
        return true;
    }
    let other = Ptr::new(form);
    let mine = e.call(MESSAGE_GET_0055B980, &args![this]).u32();
    if mine != e.call(MESSAGE_GET_0055B980, &args![other]).u32() {
        return true;
    }
    if fn_00590d00(e, this) != fn_00590d00(e, other) {
        return true;
    }
    if fn_00590ed0(e, this) != fn_00590ed0(e, other) {
        return true;
    }
    let mine = e.call(MESSAGE_GET_00639B40, &args![this]).u32();
    if mine != e.call(MESSAGE_GET_00639B40, &args![other]).u32() {
        return true;
    }
    let mut mine = e.call(MESSAGE_FIRST_NODE, &args![this]).u32();
    let mut theirs = e.call(MESSAGE_FIRST_NODE, &args![other]).u32();
    let count = e.call(LIST_COUNT, &args![mine]).u32();
    if count != e.call(LIST_COUNT, &args![theirs]).u32() {
        return true;
    }
    while theirs != 0
        && !e.call(LIST_IS_EMPTY, &args![theirs]).bool()
        && mine != 0
        && !e.call(LIST_IS_EMPTY, &args![mine]).bool()
    {
        let slot = e.call(LIST_HEAD_ITEM, &args![theirs]).u32();
        let their_item = e.mem.u32(slot);
        let slot = e.call(LIST_HEAD_ITEM, &args![mine]).u32();
        let my_item = e.mem.u32(slot);
        if e.call(ITEM_TEXT_DIFFERS, &args![my_item, their_item])
            .bool()
        {
            return true;
        }
        let differs = e
            .call(
                CONDITIONS_DIFFER,
                &args![my_item.wrapping_add(8), their_item.wrapping_add(8)],
            )
            .bool();
        if differs {
            return true;
        }
        theirs = e.call(LIST_NEXT, &args![theirs]).u32();
        mine = e.call(LIST_NEXT, &args![mine]).u32();
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, form]).bool()
}

// Translated from 005916b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMusicType`'s save (`thiscall`): `StartForm`, `0048cea0` on the
/// member at +0x18, an `ANAM` chunk with the float at +0x24 (copied
/// through the x87 stack), `CloseForm`.
pub fn fn_005916b0(e: &mut Engine, this: Ptr) {
    e.call(START_FORM, &args![this]);
    e.call(SAVE_NAME_0048CEA0, &args![this.addr() + 0x18]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 0x24));
    e.call(WRITE_CHUNK_WORD, &args![CHUNK_ANAM, value]);
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 005916f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMusicType`'s copy (`thiscall`, one word, `RET 4`): when `source`
/// casts to `BGSMusicType`, the float at +0x24 (`FLD`/`FSTP`, a signalling
/// NaN comes out quiet), then `TESForm::CopyAllComponents` with the cast
/// form.
pub fn fn_005916f0(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_form(e, source, TYPE_MUSIC_TYPE);
    if other == 0 {
        return;
    }
    let value = x87_quiet_bits(e.mem.u32(other.wrapping_add(0x24)));
    e.mem.set_u32(this.addr() + 0x24, value);
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
}

// Translated from 00591740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMusicType`'s comparison (`thiscall`, one word, `RET 4`): true when
/// `form` does not cast, when the floats at +0x24 are not equal (`FUCOMPP`,
/// `TEST AH,0x44; JNP`: unordered differs), else what
/// `TESForm::CompareAllComponents` answers.
pub fn fn_00591740(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_form(e, form, TYPE_MUSIC_TYPE);
    if other == 0 {
        return true;
    }
    let mine = x87_load32(e.mem.u32(this.addr() + 0x24));
    let theirs = x87_load32(e.mem.u32(other.wrapping_add(0x24)));
    if theirs != mine {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool()
}

// Translated from 00591a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`): `004046f0(this, value)`
/// (`cdecl`), answers `this`.
pub fn fn_00591a00(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(CDECL_004046F0, &args![this, value]);
    this
}

// Translated from 00591a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x10 of the word at +4 is set (1 or 0 in `EAX`).
pub fn fn_00591a20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 4) & 0x10 != 0
}

// Translated from 00591d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRadiationStage`'s copy (`thiscall`, one word, `RET 4`): when
/// `source` casts, `TESForm::CopyAllComponents` with the cast form, then
/// the 8 data bytes (`00500940` of each) copied with `00401460`.
pub fn fn_00591d30(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_form(e, source, TYPE_RADIATION_STAGE);
    if other == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    let from = e.call(RADIATION_STAGE_DATA, &args![other]).u32();
    let to = e.call(RADIATION_STAGE_DATA, &args![this]).u32();
    e.call(COPY_BYTES, &args![to, from, 8u32]);
}

// Translated from 00591d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRadiationStage`'s comparison (`thiscall`, one word, `RET 4`): true
/// when `form` does not cast or `TESForm::CompareAllComponents` answers
/// true, else whether `memcmp` finds the 8 data bytes (`00500940` of each)
/// different.
pub fn fn_00591d90(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_form(e, form, TYPE_RADIATION_STAGE);
    if other == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    let theirs = e.call(RADIATION_STAGE_DATA, &args![other]).u32();
    let mine = e.call(RADIATION_STAGE_DATA, &args![this]).u32();
    e.call(MEMCMP, &args![mine, theirs, 8u32]).u32() != 0
}

// Translated from 00591e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRadiationStage`'s save (`thiscall`): `StartForm`; the form at
/// `00441110` replaced by its save value (`0084e3a0`, 0 without one) with
/// `0050f9a0` for the save, the 8 bytes at +0x18 saved
/// (`TESForm::__SaveData`) between two swaps (`00462230`, each when
/// `00401500` answers true), the form put back, `CloseForm`.
pub fn fn_00591e10(e: &mut Engine, this: Ptr) {
    let data = this.addr() + 0x18;
    e.call(START_FORM, &args![this]);
    let saved = e.call(RECORD_FORM, &args![this]).u32();
    let value = if e.call(RECORD_FORM, &args![this]).u32() != 0 {
        let form = e.call(RECORD_FORM, &args![this]).u32();
        e.call(FORM_SAVE_VALUE, &args![form]).u32()
    } else {
        0
    };
    e.call(SET_RECORD_FORM, &args![this, value]);
    if e.call(SWAPS_DATA, &args![]).bool() {
        e.call(SWAP_DATA, &args![data]);
    }
    e.call(SAVE_DATA, &args![this, data, 8u32]);
    if e.call(SWAPS_DATA, &args![]).bool() {
        e.call(SWAP_DATA, &args![data]);
    }
    e.call(SET_RECORD_FORM, &args![this, saved]);
    e.call(CLOSE_FORM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_0ed0, fn_00590ed0(Ptr) -> bool),
        entry!(0x0059_10d0, fn_005910d0(Ptr)),
        entry!(0x0059_1170, fn_00591170(Ptr)),
        entry!(0x0059_11d0, fn_005911d0(Ptr, u32)),
        entry!(0x0059_1300, fn_00591300(Ptr)),
        entry!(0x0059_1320, fn_00591320(Ptr)),
        entry!(0x0059_13b0, fn_005913b0(Ptr)),
        entry!(0x0059_1410, fn_00591410(Ptr)),
        entry!(0x0059_1550, fn_00591550(Ptr, Ptr) -> bool),
        entry!(0x0059_17b0, fn_005917b0(Ptr)),
        entry!(0x0059_1a40, fn_00591a40(Ptr) -> u32),
        entry!(0x0059_1c20, fn_00591c20(Ptr)),
        entry!(0x0059_1c80, fn_00591c80(Ptr)),
        entry!(0x0059_1ca0, fn_00591ca0(Ptr)),
        entry!(0x0059_1ec0, fn_00591ec0(Ptr, Ptr) -> bool),
        entry!(0x0059_2010, fn_00592010(u32)),
        entry!(0x0059_2110, fn_00592110(u32)),
        entry!(0x00590d00, fn_00590d00(Ptr) -> bool),
        entry!(0x0059_12d0, fn_005912d0(Ptr) -> Ptr),
        entry!(0x0059_1200, fn_00591200(Ptr) -> Ptr),
        entry!(0x0059_1b70, fn_00591b70(Ptr) -> Ptr),
        entry!(0x0059_1140, fn_00591140(Ptr, u32) -> Ptr),
        entry!(0x0059_12a0, fn_005912a0(Ptr, u32) -> Ptr),
        entry!(0x0059_1bf0, fn_00591bf0(Ptr, u32) -> Ptr),
        entry!(0x0059_0d20, fn_00590d20(Ptr, u32)),
        entry!(0x0059_0e80, fn_00590e80(Ptr, u8)),
        entry!(0x0059_0ef0, fn_00590ef0(Ptr, u8)),
        entry!(0x0059_0f40, fn_00590f40(Ptr, u32) -> bool),
        entry!(0x0059_16b0, fn_005916b0(Ptr)),
        entry!(0x0059_16f0, fn_005916f0(Ptr, u32)),
        entry!(0x0059_1740, fn_00591740(Ptr, u32) -> bool),
        entry!(0x0059_1a00, fn_00591a00(Ptr, u32) -> Ptr),
        entry!(0x0059_1a20, fn_00591a20(Ptr) -> bool),
        entry!(0x0059_1d30, fn_00591d30(Ptr, u32)),
        entry!(0x0059_1d90, fn_00591d90(Ptr, u32) -> bool),
        entry!(0x0059_1e10, fn_00591e10(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_bit_zero_of_the_word_at_0x38() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        assert!(!e.call(0x0059_0d00, &args![object]).bool());
        e.mem.set_u32(object + 0x38, 0b10);
        assert!(!e.call(0x0059_0d00, &args![object]).bool());
        e.mem.set_u32(object + 0x38, 0xffff_ffff);
        assert!(e.call(0x0059_0d00, &args![object]).bool());
    }
}

#[cfg(test)]
mod ctor_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00591200_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_1200,
            size: 0x34,
            globals: &[(0x0103_395c, 0x0100_0001)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0059_12d0, &[A::T(0x18)]),
                (0x0096_a2d0, &[A::T(0x28)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x66)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_397c),
                (0x18, 4, 0x0103_3964),
                (0x24, 4, 0x0100_0001),
            ],
        });
    }

    #[test]
    fn fn_00591b70_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_1b70,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_19f0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5a)]),
                (0x0040_1b20, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_3b34)],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005912d0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_12d0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0040_37b0, &[A::T(0x4)])],
            stores: &[(0x0, 4, 0x0103_3ab8)],
        });
    }
    #[test]
    fn at_00591140_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_1170, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_1140, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_1140)
                .collect();
            let mut expected = vec![(0x0059_1170, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_005912a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_1320, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_12a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_12a0)
                .collect();
            let mut expected = vec![(0x0059_1320, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00591bf0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_1c20, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_1bf0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_1bf0)
                .collect();
            let mut expected = vec![(0x0059_1c20, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod music_and_radiation_tests {
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

    /// A list double: every list holds the given items. The list itself is
    /// the first node (its item word is the array's first word); the later
    /// nodes are the addresses of the array's other words.
    fn list(e: &mut Engine, items: &[u32]) {
        let shared = Rc::new(RefCell::new(items.to_vec()));
        let n = items.len() as u32;
        let words = e.mem.alloc(4 * n.max(1));
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(words + 4 * i as u32, *item);
        }
        let s = shared.clone();
        e.register_double(LIST_IS_EMPTY, move |_, _| {
            ret(u32::from(s.borrow().is_empty()))
        });
        let s = shared.clone();
        e.register_double(LIST_COUNT, move |_, _| ret(s.borrow().len() as u32));
        e.register_double(LIST_HEAD_ITEM, move |_, a| {
            if a[0] >= words && a[0] < words + 4 * n {
                ret(a[0])
            } else {
                ret(words)
            }
        });
        let s = shared;
        e.register_double(LIST_REMOVE_HEAD, move |e, _| {
            let mut s = s.borrow_mut();
            s.remove(0);
            for (i, item) in s.iter().enumerate() {
                e.mem.set_u32(words + 4 * i as u32, *item);
            }
            Ret::default()
        });
        e.register_double(LIST_NEXT, move |_, a| {
            if a[0] >= words && a[0] < words + 4 * n {
                ret(if a[0] + 4 < words + 4 * n {
                    a[0] + 4
                } else {
                    0
                })
            } else {
                // The list itself is the first node.
                ret(if n > 1 { words + 4 } else { 0 })
            }
        });
    }

    #[test]
    fn fn_00590ed0_reads_bit_one() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x38, 1);
        assert!(!e.call(0x0059_0ed0, &args![object]).bool());
        e.mem.set_u32(object + 0x38, 2);
        assert!(e.call(0x0059_0ed0, &args![object]).bool());
    }

    #[test]
    fn fn_005910d0_deletes_each_non_null_item() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        list(&mut e, &[0x5000, 0, 0x6000]);
        quiet(&mut e, &[0x0059_1170, FREE]);
        e.call_log = Some(vec![]);
        e.call(0x0059_10d0, &args![this]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD), vec![vec![this + 0x30]; 3]);
        assert_eq!(calls(&e, 0x0059_1170), vec![vec![0x5000], vec![0x6000]]);
        assert_eq!(calls(&e, FREE), vec![vec![0x5000], vec![0x6000]]);
    }

    #[test]
    fn fn_00591170_destroys_both_members() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0068_08b0, 0x0040_37d0]);
        e.call_log = Some(vec![]);
        e.call(0x0059_1170, &args![0x4000u32]);
        assert_eq!(calls(&e, 0x0068_08b0), vec![vec![0x4008]]);
        assert_eq!(calls(&e, 0x0040_37d0), vec![vec![0x4000]]);
    }

    #[test]
    fn fn_005911d0_adds_only_non_null_items() {
        let mut e = Engine::new();
        e.register_double(0x0090_5820, |e, a| {
            assert_eq!((a[0], e.mem.u32(a[1])), (0x4030, 0x77));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0059_11d0, &args![0x4000u32, 0u32]);
        assert!(calls(&e, 0x0090_5820).is_empty());
        e.call(0x0059_11d0, &args![0x4000u32, 0x77u32]);
        assert_eq!(calls(&e, 0x0090_5820).len(), 1);
    }

    #[test]
    fn fn_00591300_destroys_the_string() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0040_37d0]);
        e.call_log = Some(vec![]);
        e.call(0x0059_1300, &args![0x4018u32]);
        assert_eq!(calls(&e, 0x0040_37d0), vec![vec![0x401c]]);
    }

    #[test]
    fn fn_00591320_destroys_the_music_type() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x34);
        list(&mut e, &[0x5000]);
        quiet(&mut e, &[FREE, 0x0046_ffb0, 0x0040_37d0, 0x0048_3630]);
        e.call_log = Some(vec![]);
        e.call(0x0059_1320, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_397c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_3964);
        assert_eq!(calls(&e, FREE), vec![vec![0x5000]]);
        assert_eq!(calls(&e, 0x0046_ffb0), vec![vec![this + 0x28]]);
        assert_eq!(calls(&e, 0x0040_37d0), vec![vec![this + 0x1c]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);
    }

    #[test]
    fn fn_005913b0_frees_every_item() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x34);
        list(&mut e, &[0x5000, 0x6000]);
        quiet(&mut e, &[FREE]);
        e.call_log = Some(vec![]);
        e.call(0x0059_13b0, &args![this]);
        assert_eq!(calls(&e, FREE), vec![vec![0x5000], vec![0x6000]]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD), vec![vec![this + 0x28]; 2]);
    }

    /// A music type whose name component's text is `name` (the
    /// component's vtable +0x10 is a double).
    fn music_type(e: &mut Engine, name: &[u8]) -> u32 {
        let this = e.mem.alloc(0x34);
        let text = e.mem.alloc(0x40);
        e.mem.set_cstr(text, name);
        e.mem.set_u32(this + 0x1c, text);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x7000_0010);
        e.mem.set_u32(this + 0x18, vtable);
        quiet(e, &[0x7000_0010]);
        e.register(STRING_TEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(STRING_LENGTH, |e, a| {
            let text = e.mem.u32(a[0] + 4);
            ret(e.mem.cstr(text).len() as u32)
        });
        this
    }

    #[test]
    fn fn_00591410_sets_the_path_or_scans_the_folder() {
        let mut e = Engine::new();
        let this = music_type(&mut e, b"Explore\\Song.mp3");
        list(&mut e, &[0x5000, 0x6000, 0x7000]);
        e.register(FORM_INIT_SKIP, |_, _| ret(0));
        quiet(&mut e, &[SPRINTF_S, FORM_SET_FLAG]);
        e.register(RANDOM_INT, |_, a| ret(a[1]));
        e.call_log = Some(vec![]);
        e.call(0x0059_1410, &args![this]);
        let printed = calls(&e, SPRINTF_S);
        assert_eq!(printed.len(), 1);
        assert_eq!(
            &printed[0][1..],
            &[0x104, PATH_FORMAT, MUSIC_FOLDER, e.mem.u32(this + 0x1c)]
        );
        assert_eq!(
            calls(&e, 0x7000_0010),
            vec![vec![this + 0x18, printed[0][0]]]
        );
        assert_eq!(calls(&e, RANDOM_INT), vec![vec![0, 2]]);
        assert_eq!(e.mem.u32(this + 0x30), 2);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 1]]);

        // A folder name: the scan (finished at once), and an empty list: 0.
        let mut e = Engine::new();
        let this = music_type(&mut e, b"Dungeon\\");
        list(&mut e, &[]);
        e.mem.set_u32(this + 0x30, 9);
        e.register(FORM_INIT_SKIP, |_, _| ret(0));
        quiet(
            &mut e,
            &[
                SPRINTF_S,
                FORM_SET_FLAG,
                0x0040_4eb0,
                0x0040_4ee0,
                0x00b0_0820,
                0x00b0_0860,
            ],
        );
        e.register(0x00b0_0720, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0059_1410, &args![this]);
        assert_eq!(calls(&e, 0x00b0_0820).len(), 1);
        assert!(calls(&e, 0x7000_0010).is_empty());
        assert!(calls(&e, RANDOM_INT).is_empty());
        assert_eq!(e.mem.u32(this + 0x30), 0);

        // A short name: neither; `004013e0` true: nothing.
        let mut e = Engine::new();
        let this = music_type(&mut e, b"ab");
        list(&mut e, &[]);
        e.register(FORM_INIT_SKIP, |_, _| ret(0));
        quiet(&mut e, &[SPRINTF_S, FORM_SET_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x0059_1410, &args![this]);
        assert!(calls(&e, SPRINTF_S).is_empty());
        assert_eq!(calls(&e, FORM_SET_FLAG).len(), 1);
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0059_1410, &args![this]);
        assert!(calls(&e, FORM_SET_FLAG).is_empty());
    }

    /// A file whose chunks are `chunks` (type, first data word).
    fn file(e: &mut Engine, form_type: u32, chunks: Vec<(u32, u32)>, swap: bool) -> u32 {
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
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        quiet(
            e,
            &[FORM_LOAD_FORM, FORM_SET_FLAG, FORM_LOAD_DATA, 0x0046_2230],
        );
        file
    }

    /// A form whose vtable slots +0x18, +0xE0 and +0x134 are doubles.
    fn form(e: &mut Engine, size: u32) -> u32 {
        let this = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x140);
        for slot in [0x18u32, 0xe0, 0x134] {
            e.mem.set_u32(vtable + slot, 0x7100_0000 + slot);
        }
        e.mem.set_u32(this, vtable);
        quiet(e, &[0x7100_0018, 0x7100_00e0, 0x7100_0134]);
        this
    }

    #[test]
    fn fn_00591550_reads_the_music_type_chunks() {
        let mut e = Engine::new();
        let this = form(&mut e, 0x34);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x7000_0010);
        e.mem.set_u32(this + 0x18, vtable);
        quiet(&mut e, &[0x7000_0010]);
        let f = file(
            &mut e,
            0,
            vec![
                (CHUNK_EDID, 1),
                (CHUNK_OBND, 0),
                (CHUNK_FNAM, 0x6f6c),
                (CHUNK_ANAM, 0x7f80_0001),
                (CHUNK_DATA, 0),
            ],
            false,
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_1550, &args![this, f]).bool());
        assert_eq!(calls(&e, FORM_LOAD_FORM), vec![vec![this, f]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7100_0134).len(), 1);
        assert_eq!(calls(&e, 0x7100_00e0), vec![vec![this, f]]);
        let names = calls(&e, 0x7000_0010);
        assert_eq!(names.len(), 1);
        assert_eq!(names[0][0], this + 0x18);
        let sized = calls(&e, FILE_CHUNK_DATA_SIZED);
        assert_eq!((sized[0][2], sized[1][2]), (0x200, 0));
        // The signalling NaN comes back quiet through FLD/FSTP.
        assert_eq!(e.mem.u32(this + 0x24), 0x7fc0_0001);
    }

    #[test]
    fn fn_005917b0_adds_the_mp3_files() {
        let mut e = Engine::new();
        let this = music_type(&mut e, b"Dungeon\\");
        let added = Rc::new(RefCell::new(Vec::new()));
        let a2 = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            a2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                SPRINTF_S,
                0x0040_4eb0,
                0x0040_4ee0,
                0x00b0_0820,
                0x00b0_0860,
            ],
        );
        // Entries: one `00591a20` rejects, one without a name, one with
        // another extension and an `.mp3`; the fifth step finishes.
        let step = Rc::new(RefCell::new(0u32));
        let s = step.clone();
        e.register_double(0x00b0_0720, move |e, a| {
            *s.borrow_mut() += 1;
            if *s.borrow() == 5 {
                e.mem.set_u8(a[1], 1);
            }
            Ret::default()
        });
        let s = step.clone();
        e.register_double(0x0059_1a20, move |_, _| ret(u32::from(*s.borrow() == 1)));
        let s = step.clone();
        e.register_double(0x0043_6aa0, move |_, _| {
            let n = *s.borrow();
            ret(if n == 2 { 0 } else { 0x9000 + n })
        });
        quiet(&mut e, &[0x0046_2d40]);
        let s = step;
        e.register_double(0x0040_4dc0, move |_, a| {
            assert_eq!(a[1], MP3_EXTENSION);
            ret(u32::from(*s.borrow() != 4))
        });
        e.register(ALLOCATE, |e, _| ret(e.mem.alloc(0x104)));
        e.register(0x0059_1a00, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x0059_17b0, &args![this]);
        let scope = calls(&e, 0x0040_4eb0);
        assert_eq!(&scope[0][1..], &[0xb, 1, MUSIC_TYPE_SOURCE, 0x160]);
        let printed = calls(&e, SPRINTF_S);
        assert_eq!(printed.len(), 2);
        assert_eq!(&printed[1][1..4], &[0x104, CONCAT_FORMAT, printed[0][0]]);
        assert_eq!(printed[1][4], 0x9004);
        let added = added.borrow();
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].0, this + 0x28);
        assert_eq!(
            calls(&e, 0x0059_1a00),
            vec![vec![added[0].1, printed[1][0]]]
        );
        assert_eq!(calls(&e, 0x00b0_0860).len(), 1);
        assert_eq!(calls(&e, 0x0040_4ee0).len(), 1);
    }

    #[test]
    fn fn_00591a40_picks_a_file() {
        // Empty: the name.
        let mut e = Engine::new();
        let this = music_type(&mut e, b"Song");
        list(&mut e, &[]);
        let name = e.mem.u32(this + 0x1c);
        assert_eq!(e.call(0x0059_1a40, &args![this]).u32(), name);

        // One entry: that entry.
        let mut e = Engine::new();
        let this = music_type(&mut e, b"Song");
        list(&mut e, &[0x5000]);
        assert_eq!(e.call(0x0059_1a40, &args![this]).u32(), 0x5000);

        // Three entries: the random index walks the nodes; a repeat of the
        // last non-zero index becomes 0, a repeat of 0 becomes 1.
        for (last, drawn, kept, picked) in [
            (0u32, 2u32, 2u32, 0x7000u32),
            (2, 2, 0, 0x5000),
            (0, 0, 1, 0x6000),
        ] {
            let mut e = Engine::new();
            let this = music_type(&mut e, b"Song");
            list(&mut e, &[0x5000, 0x6000, 0x7000]);
            e.mem.set_u32(this + 0x30, last);
            e.register_double(RANDOM_INT, move |_, a| {
                assert_eq!(a, &[0, 3]);
                ret(drawn)
            });
            assert_eq!(e.call(0x0059_1a40, &args![this]).u32(), picked);
            assert_eq!(e.mem.u32(this + 0x30), kept);
        }

        // Index 3 runs off the list, and an empty slot: the name.
        for (items, drawn) in [
            ([0x5000u32, 0x6000, 0x7000], 3u32),
            ([0x5000, 0, 0x7000], 1),
        ] {
            let mut e = Engine::new();
            let this = music_type(&mut e, b"Song");
            list(&mut e, &items);
            e.register_double(RANDOM_INT, move |_, _| ret(drawn));
            let name = e.mem.u32(this + 0x1c);
            assert_eq!(e.call(0x0059_1a40, &args![this]).u32(), name);
        }
    }

    fn stages(e: &mut Engine, thresholds: &[u32]) -> Vec<u32> {
        e.map(0x011c_a000, 0x1000);
        let array = e.mem.alloc(4 * thresholds.len().max(1) as u32);
        let mut out = Vec::new();
        for (i, t) in thresholds.iter().enumerate() {
            let stage = e.mem.alloc(0x20);
            e.mem.set_u32(stage + 0x18, *t);
            e.mem.set_u32(array + 4 * i as u32, stage);
            out.push(stage);
        }
        e.set_global(RADIATION_STAGE_COUNT, thresholds.len() as u32);
        e.set_global(RADIATION_STAGES, array);
        e.register(RADIATION_STAGE_DATA, |_, a| ret(a[0] + 0x18));
        e.register(ALLOCATE_POINTERS, |e, a| ret(e.mem.alloc(4 * a[0].max(1))));
        quiet(e, &[FREE]);
        out
    }

    fn stage_list(e: &Engine) -> Vec<u32> {
        let array = e.global::<u32>(RADIATION_STAGES);
        (0..e.global::<u32>(RADIATION_STAGE_COUNT))
            .map(|i| e.mem.u32(array + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00592010_inserts_before_the_first_lower_threshold() {
        let mut e = Engine::new();
        let s = stages(&mut e, &[900, 500, 100]);
        let new = e.mem.alloc(0x20);
        e.mem.set_u32(new + 0x18, 600);
        let old = e.global::<u32>(RADIATION_STAGES);
        e.call_log = Some(vec![]);
        e.call(0x0059_2010, &args![new]);
        assert_eq!(stage_list(&e), vec![s[0], new, s[1], s[2]]);
        assert_eq!(calls(&e, FREE), vec![vec![old]]);
        // The lowest goes last.
        let low = e.mem.alloc(0x20);
        e.mem.set_u32(low + 0x18, 50);
        e.call(0x0059_2010, &args![low]);
        assert_eq!(stage_list(&e), vec![s[0], new, s[1], s[2], low]);
    }

    #[test]
    fn fn_00592110_removes_a_listed_stage_only() {
        let mut e = Engine::new();
        let s = stages(&mut e, &[900, 500, 100]);
        e.call_log = Some(vec![]);
        e.call(0x0059_2110, &args![0x1234u32]);
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(stage_list(&e), s);
        e.call(0x0059_2110, &args![s[1]]);
        assert_eq!(stage_list(&e), vec![s[0], s[2]]);
        assert_eq!(calls(&e, FREE).len(), 1);
        // A failed allocation: the array is null, the count still lowered.
        e.register(ALLOCATE_POINTERS, |_, _| ret(0));
        e.call(0x0059_2110, &args![s[0]]);
        assert_eq!(e.global::<u32>(RADIATION_STAGE_COUNT), 1);
        assert_eq!(e.global::<u32>(RADIATION_STAGES), 0);
        // An empty list: nothing.
        e.set_global(RADIATION_STAGE_COUNT, 0u32);
        e.call(0x0059_2110, &args![s[0]]);
        assert_eq!(calls(&e, FREE).len(), 2);
    }

    #[test]
    fn fn_00591c20_and_00591c80_take_the_stage_out() {
        let mut e = Engine::new();
        let s = stages(&mut e, &[900, 500]);
        quiet(&mut e, &[0x0048_3630]);
        e.call_log = Some(vec![]);
        e.call(0x0059_1c20, &args![s[0]]);
        assert_eq!(e.mem.u32(s[0]), 0x0103_3b34);
        assert_eq!(stage_list(&e), vec![s[1]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![s[0]]]);
        e.call(0x0059_1c80, &args![s[1]]);
        assert!(stage_list(&e).is_empty());
    }

    #[test]
    fn fn_00591ca0_resolves_the_spell_and_sorts_the_stage() {
        let mut e = Engine::new();
        let s = stages(&mut e, &[900]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 1000);
        e.register(FORM_INIT_SKIP, |_, _| ret(0));
        e.register(0x0044_1110, |_, _| ret(0x42));
        e.register(FORM_GET_FILE, |_, _| ret(0x4400));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| ret(a[0] & 0xffff));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        quiet(&mut e, &[0x0050_f9a0, FORM_SET_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x0059_1ca0, &args![this]);
        assert_eq!(calls(&e, LOOKUP_FORM), vec![vec![0x0100_0042]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x42, 0, TYPE_TES_FORM, TYPE_SPELL_ITEM, 0]]
        );
        assert_eq!(calls(&e, 0x0050_f9a0), vec![vec![this, 0x43]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 1]]);
        assert_eq!(stage_list(&e), vec![this, s[0]]);

        // No effect id: no lookup; skipped: nothing.
        e.register(0x0044_1110, |_, _| ret(0));
        let other = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0059_1ca0, &args![other]);
        assert!(calls(&e, LOOKUP_FORM).is_empty());
        assert_eq!(stage_list(&e).len(), 3);
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        e.call(0x0059_1ca0, &args![other]);
        assert_eq!(stage_list(&e).len(), 3);
    }

    #[test]
    fn fn_00591ec0_reads_the_radiation_stage_chunks() {
        let mut e = Engine::new();
        let this = form(&mut e, 0x20);
        e.register(RADIATION_STAGE_DATA, |_, a| ret(a[0] + 0x18));
        let f = file(
            &mut e,
            0x5a,
            vec![(CHUNK_DATA, 0), (CHUNK_EDID, 0), (CHUNK_OBND, 0)],
            true,
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_1ec0, &args![this, f]).bool());
        assert_eq!(calls(&e, 0x7100_0018), vec![vec![this]]);
        assert_eq!(calls(&e, FORM_LOAD_FORM), vec![vec![this, f]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(
            calls(&e, FORM_LOAD_DATA),
            vec![vec![this, f, this + 0x18, 8]]
        );
        assert_eq!(calls(&e, 0x0046_2230), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, FILE_SWAPS_ENDIAN).len(), 1);
        assert_eq!(calls(&e, 0x7100_0134).len(), 1);
        assert_eq!(calls(&e, 0x7100_00e0), vec![vec![this, f]]);

        // Another record type: false at once.
        let mut e = Engine::new();
        let this = form(&mut e, 0x20);
        let f = file(&mut e, 0x58, vec![(CHUNK_DATA, 0)], false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_1ec0, &args![this, f]).bool());
        assert!(calls(&e, FORM_LOAD_FORM).is_empty());
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    /// Lists as nodes of (item, next); a node with a zero item is empty.
    /// Getters answer the word at +4 (`0055b980`) and +8 (`00639b40`); the
    /// first node is the word at +0x30; the count is the number of nodes
    /// with items.
    fn setup(e: &mut Engine) {
        e.register(LIST_IS_EMPTY, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) == 0),
            ..Ret::default()
        });
        e.register(LIST_HEAD_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut count = 0;
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != 0 {
                count += 1;
                node = e.mem.u32(node + 4);
            }
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register(MESSAGE_GET_0055B980, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(MESSAGE_GET_00639B40, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(MESSAGE_FIRST_NODE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        e.register(ITEM_TEXT_DIFFERS, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
            ..Ret::default()
        });
        e.register(CONDITIONS_DIFFER, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
            ..Ret::default()
        });
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
    }

    /// A message with the items (text word, condition word).
    fn message(e: &mut Engine, items: &[(u32, u32)]) -> u32 {
        let mut next = e.mem.alloc(8);
        for &(text, condition) in items.iter().rev() {
            let item = e.mem.alloc(0x10);
            e.mem.set_u32(item, text);
            e.mem.set_u32(item + 8, condition);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0x30, next);
        form
    }

    #[test]
    fn fn_00590d20_copies_a_message() {
        let mut e = Engine::new();
        setup(&mut e);
        e.register(0x0090_0018, |_, _| Ret::default());
        for t in [
            MESSAGE_SET_0086CF00,
            MESSAGE_SET_00409480,
            ITEM_TEXT_COPY,
            CONDITIONS_COPY,
            COPY_ALL_COMPONENTS,
            0x0090_5820,
        ] {
            e.register(t, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(ITEM_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let source = message(&mut e, &[(1, 2), (3, 4)]);
        e.mem.set_u32(source + 4, 0x44);
        e.mem.set_u32(source + 8, 0x88);
        e.mem.set_u32(source + 0x38, 2);
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x18, 0x0090_0018);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x38, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x0059_0d20, &[this, source]);
        assert_eq!(e.mem.u32(this + 0x38), 0x12);
        let log = calls_to(
            &mut e,
            &[
                0x0090_0018,
                MESSAGE_SET_0086CF00,
                MESSAGE_SET_00409480,
                ITEM_TEXT_COPY,
                CONDITIONS_COPY,
                COPY_ALL_COMPONENTS,
            ],
        );
        assert_eq!(log[0], (0x0090_0018, vec![this]));
        assert_eq!(log[1], (MESSAGE_SET_0086CF00, vec![this, 0x44]));
        assert_eq!(log[2], (MESSAGE_SET_00409480, vec![this, 0x88]));
        let first_item = e.mem.u32(e.mem.u32(source + 0x30));
        let copy = log[3].1[0];
        assert_eq!(log[3], (ITEM_TEXT_COPY, vec![copy, first_item]));
        assert_eq!(log[4], (CONDITIONS_COPY, vec![copy + 8, first_item + 8, 0]));
        assert_eq!(log.len(), 8);
        assert_eq!(log[7], (COPY_ALL_COMPONENTS, vec![this, source]));
    }

    #[test]
    fn fn_00590e80_and_00590ef0_set_the_flags() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x38, 0xf0);
        e.call(0x0059_0e80, &[this, 0x100]);
        assert_eq!(e.mem.u32(this + 0x38), 0xf0);
        e.call(0x0059_0e80, &[this, 1]);
        e.call(0x0059_0ef0, &[this, 2]);
        assert_eq!(e.mem.u32(this + 0x38), 0xf3);
        e.call(0x0059_0e80, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x38), 0xf2);
        e.call(0x0059_0ef0, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x38), 0xf0);
    }

    #[test]
    fn fn_00590f40_compares_messages() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = message(&mut e, &[(1, 2), (3, 4)]);
        let other = message(&mut e, &[(1, 2), (3, 4)]);
        assert!(e.call(0x0059_0f40, &[this, 0]).bool());
        assert!(!e.call(0x0059_0f40, &[this, other]).bool());
        for offset in [4, 8, 0x38] {
            e.mem.set_u32(other + offset, 1);
            assert!(e.call(0x0059_0f40, &[this, other]).bool());
            e.mem.set_u32(other + offset, 0);
        }
        e.mem.set_u32(other + 0x38, 2);
        assert!(e.call(0x0059_0f40, &[this, other]).bool());
        e.mem.set_u32(other + 0x38, 0);
        let longer = message(&mut e, &[(1, 2), (3, 4), (5, 6)]);
        assert!(e.call(0x0059_0f40, &[this, longer]).bool());
        let text = message(&mut e, &[(1, 2), (9, 4)]);
        assert!(e.call(0x0059_0f40, &[this, text]).bool());
        let condition = message(&mut e, &[(1, 9), (3, 4)]);
        assert!(e.call(0x0059_0f40, &[this, condition]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0059_0f40, &[this, other]).bool());
    }

    #[test]
    fn music_type_save_copy_and_compare() {
        let mut e = Engine::new();
        for t in [
            START_FORM,
            CLOSE_FORM,
            SAVE_NAME_0048CEA0,
            WRITE_CHUNK_WORD,
            COPY_ALL_COMPONENTS,
        ] {
            e.register(t, |_, _| Ret::default());
        }
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        e.register(DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_MUSIC_TYPE, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x30);
        let other = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x0059_16b0, &[this]);
        assert_eq!(
            calls_to(
                &mut e,
                &[START_FORM, SAVE_NAME_0048CEA0, WRITE_CHUNK_WORD, CLOSE_FORM]
            ),
            vec![
                (START_FORM, vec![this]),
                (SAVE_NAME_0048CEA0, vec![this + 0x18]),
                (WRITE_CHUNK_WORD, vec![CHUNK_ANAM, 0x7fc0_0001]),
                (CLOSE_FORM, vec![this]),
            ]
        );
        // NaN never compares equal.
        e.mem.set_u32(other + 0x24, 0x7fc0_0001);
        assert!(e.call(0x0059_1740, &[this, other]).bool());
        e.mem.set_f32(other + 0x24, 2.5);
        e.mem.set_u32(other + 0xc, 1);
        e.call(0x0059_16f0, &[this, other]);
        assert_eq!(e.mem.u32(this + 0x24), 0x7f80_0001);
        assert!(e.call(0x0059_1740, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        e.call(0x0059_16f0, &[this, other]);
        assert_eq!(e.mem.f32(this + 0x24), 2.5);
        assert!(!e.call(0x0059_1740, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0059_1740, &[this, other]).bool());
    }

    #[test]
    fn fn_00591a00_and_00591a20() {
        let mut e = Engine::new();
        e.register(CDECL_004046F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_1a00, &[0x1000, 7]).u32(), 0x1000);
        assert_eq!(
            calls_to(&mut e, &[CDECL_004046F0]),
            vec![(CDECL_004046F0, vec![0x1000, 7])]
        );
        let this = e.mem.alloc(8);
        assert!(!e.call(0x0059_1a20, &[this]).bool());
        e.mem.set_u32(this + 4, 0x10);
        assert_eq!(e.call(0x0059_1a20, &[this]).u32(), 1);
    }

    #[test]
    fn radiation_stage_copy_compare_and_save() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_RADIATION_STAGE, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        e.register(RADIATION_STAGE_DATA, |_, a| Ret {
            eax: a[0] + 0x18,
            ..Ret::default()
        });
        e.register(MEMCMP, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
            ..Ret::default()
        });
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        for t in [
            COPY_ALL_COMPONENTS,
            COPY_BYTES,
            START_FORM,
            CLOSE_FORM,
            SET_RECORD_FORM,
            SWAP_DATA,
            SAVE_DATA,
        ] {
            e.register(t, |_, _| Ret::default());
        }
        e.register(RECORD_FORM, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x24),
            ..Ret::default()
        });
        e.register(FORM_SAVE_VALUE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(SWAPS_DATA, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x30);
        let other = e.mem.alloc(0x30);
        e.mem.set_u32(other + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0059_1d30, &[this, other]);
        assert!(calls_to(&mut e, &[COPY_BYTES]).is_empty());
        assert!(e.call(0x0059_1d90, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0059_1d30, &[this, other]);
        assert_eq!(
            calls_to(&mut e, &[COPY_ALL_COMPONENTS, COPY_BYTES]),
            vec![
                (COPY_ALL_COMPONENTS, vec![this, other]),
                (COPY_BYTES, vec![this + 0x18, other + 0x18, 8]),
            ]
        );
        assert!(!e.call(0x0059_1d90, &[this, other]).bool());
        e.mem.set_u32(other + 0x18, 1);
        assert!(e.call(0x0059_1d90, &[this, other]).bool());
        e.mem.set_u32(other + 0x18, 0);
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0059_1d90, &[this, other]).bool());
        // The save swaps the record form for its save value around it.
        e.mem.set_u32(this + 0x24, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0059_1e10, &[this]);
        assert_eq!(
            calls_to(&mut e, &[SET_RECORD_FORM, SWAP_DATA, SAVE_DATA]),
            vec![
                (SET_RECORD_FORM, vec![this, 0x5001]),
                (SWAP_DATA, vec![this + 0x18]),
                (SAVE_DATA, vec![this, this + 0x18, 8]),
                (SWAP_DATA, vec![this + 0x18]),
                (SET_RECORD_FORM, vec![this, 0x5000]),
            ]
        );
        e.mem.set_u32(this + 0x24, 0);
        e.call_log = Some(vec![]);
        e.call(0x0059_1e10, &[this]);
        let log = calls_to(&mut e, &[SET_RECORD_FORM, FORM_SAVE_VALUE]);
        assert_eq!(log[0], (SET_RECORD_FORM, vec![this, 0]));
    }
}
