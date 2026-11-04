//! `fallout shared/run_0050ad90` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050adf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050adf0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x2c (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050adf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.mem.set_u32(base, 0x0102_886c);
    e.mem.set_u32(base + 0x30, 0x0102_884c);
    e.mem.set_u32(base + 0x4c, 0x0102_8828);
    e.call(0x004f_15a0, &args![this, 0x2c]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050b400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b400 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x34 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b400(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8a64);
    e.mem.set_u32(base + 0x30, 0x0102_8a44);
    e.call(0x004f_15a0, &args![this, 0x34]);
    e.call(0x0048_3710, &args![this]);
    this
}

// Translated from 0050b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b8f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4c (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0050b8f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_7650, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_8c5c);
    e.mem.set_u32(base + 0x30, 0x0102_8c38);
    e.call(0x004f_15a0, &args![this, 0x4c]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0050ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `0050a820`. `0050a820` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050ade0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_a820, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

// Translated from 0050b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4c (`SUB ECX,0x4c`) and jumps to `0050aea0`. `0050aea0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0050b3f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0050_aea0, &args![this.addr().wrapping_sub(0x4c), flags])
        .u32()
}

// Translated from 0050aea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050aed0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050aea0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_aed0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050b490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050b4c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050b490(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_b4c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0050b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050b9b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050b980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_b9b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0043d4d0` (`thiscall`): answers the address of a static's value.
const STATIC_VALUE_ADDRESS: u32 = 0x0043_d4d0;
/// The statics `0050ad90`, `0050b370` and `0050b8d0` read through `0043d4d0`.
const LEV_CREATURE_STATIC: u32 = 0x011c_8e1c;
const LEV_ITEM_STATIC: u32 = 0x011c_8ed4;
const LEV_SPELL_STATIC: u32 = 0x011c_8f00;
/// The `LEVELED_OBJECT` the last `LVLO` chunk added, which a following `COED`
/// chunk gives its extra (one per class: `TESLevCreature`, `TESLevItem`).
const LAST_LEV_CREATURE_OBJECT: u32 = 0x011c_8eec;
const LAST_LEV_ITEM_OBJECT: u32 = 0x011c_8f1c;
/// `TESFile::GetTESForm` (`00472660`): the record's form type.
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESFile::GetTESChunk` (`004726b0`): the current chunk's tag.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `004726f0`: moves to the next chunk; false at the end of the record.
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `TESFile::GetChunkData` (`004727f0`, `RET 4`).
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData` overload (`00472890`, `RET 8`: address, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401660` (`thiscall` on the file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `00401680` (`thiscall` on the file): whether form ids read need fixing up
/// (`00488510`); the meaning is not checked here.
const FILE_FIXES_IDS: u32 = 0x0040_1680;
/// `00488510` (`thiscall` on the 12-byte `LVLO` data): fixes it up.
const FIX_UP_LVLO: u32 = 0x0048_8510;
/// `TESForm::LoadForm` (`00485110`, `RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (`thiscall`, `RET 4`).
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `00403d30` (`cdecl`, three words: address, byte, count): fills memory
/// (the engine map's name for it is a library one).
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `00487a60` (`thiscall`, `RET 0x10`): adds a `LEVELED_OBJECT`.
const LEVELED_LIST_ADD: u32 = 0x0048_7a60;
/// `0042e680` (`thiscall`, `RET 4`): sets the leveled list's chance of none.
const SET_CHANCE_NONE: u32 = 0x0042_e680;
/// `00401000`: allocate (`cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
/// `ContainerItemExtra::ContainerItemExtra` (`0040e690`).
const CONTAINER_ITEM_EXTRA_CONSTRUCT: u32 = 0x0040_e690;
/// `0040e780` (`thiscall`, `RET 4`, the file): loads a `ContainerItemExtra`.
const CONTAINER_ITEM_EXTRA_LOAD: u32 = 0x0040_e780;
/// `TESModel::LoadModelChunk` (`004892d0`, `cdecl`: the model, the file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `00403570` (`thiscall` on the file): a 16-bit word passed on to the
/// texture-swap unpacking.
const FILE_WORD: u32 = 0x0040_3570;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (`0048a7b0`, `RET 0xc`).
const UNPACK_TEXTURE_SWAP: u32 = 0x0048_a7b0;
/// The form vtable slots (byte offsets) the loads call: `0x134` with the
/// editor id read, `0xe0` with the file for `OBND`.
const SET_EDITOR_ID_SLOT: u32 = 0x134;
const LOAD_BOUNDS_SLOT: u32 = 0xe0;
/// `TESForm::TESForm`-side helpers the destructors and `InitItem` call:
/// `0050e9f0` (the form base destructor), `0048a490` (the texture-swap model
/// destructor), `004013e0` (a `bool`), `00488570` (the leveled list's
/// `InitItem`, the form passed), `00484d60` and `00488c50` (`RET 4`).
const FORM_BASE_DESTRUCT: u32 = 0x0050_e9f0;
const MODEL_DESTRUCT: u32 = 0x0048_a490;
const FORM_SKIPS_INIT: u32 = 0x0040_13e0;
const LEVELED_LIST_INIT_ITEM: u32 = 0x0048_8570;
const FORM_COPY: u32 = 0x0048_4d60;
const LEVELED_LIST_COPY: u32 = 0x0048_8c50;
/// `004876d0`: the `TESLeveledList` destructor body.
const LEVELED_LIST_DESTRUCT: u32 = 0x0048_76d0;
/// `TESLevCreature` and `TESLevItem` vtables (RTTI checked) and their bases'.
const LEV_CREATURE_VTABLE: u32 = 0x0102_886c;
const LEV_CREATURE_LIST_VTABLE: u32 = 0x0102_884c;
const LEV_CREATURE_MODEL_VTABLE: u32 = 0x0102_8828;
const LEV_ITEM_VTABLE: u32 = 0x0102_8a64;
const LEV_ITEM_LIST_VTABLE: u32 = 0x0102_8a44;
/// Chunk tags.
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_LVLD: u32 = 0x444c_564c;
const CHUNK_LVLF: u32 = 0x464c_564c;
const CHUNK_LVLG: u32 = 0x474c_564c;
const CHUNK_LVLO: u32 = 0x4f4c_564c;
const CHUNK_COED: u32 = 0x4445_4f43;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;

/// `__alloca_probe_16` (`00ec5ec0`) of the chunk's size: the stack block,
/// rounded up to 16 bytes as the probe does, for the duration of `body`.
fn with_chunk_stack<R>(e: &mut Engine, size: u32, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(size.wrapping_add(15) & !15, |e, block| {
        body(e, block.addr())
    })
}

/// The `EDID` case of both loads: the chunk (at most 0x200 bytes) read into
/// a stack block of the chunk's size, handed to vtable `+0x134`.
fn load_editor_id(e: &mut Engine, this: Ptr, file: Ptr) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    with_chunk_stack(e, size, |e, block| {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, block, 0x200u32]);
        e.vcall(this.addr(), SET_EDITOR_ID_SLOT, &args![block]);
    });
}

/// The `LVLO`/`COED` case of both loads (the tag is read again): an `LVLO`
/// (12 bytes, cleared first, fixed up when `00401680` says so) adds an object
/// to the leveled list at +0x30 with its level (+0), count (+8) and form
/// (+4) and remembers it in `last_object`; a `COED` gives the remembered
/// object a new `ContainerItemExtra` (allocation failure leaves null) loaded
/// from the file, and forgets it.
fn load_leveled_object(e: &mut Engine, this: Ptr, file: Ptr, last_object: u32) {
    let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
    if tag == CHUNK_COED {
        if e.global::<u32>(last_object) != 0 {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let extra = if block != 0 {
                e.call(CONTAINER_ITEM_EXTRA_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.call(CONTAINER_ITEM_EXTRA_LOAD, &args![extra, file]);
            let object = e.global::<u32>(last_object);
            e.mem.set_u32(object.wrapping_add(8), extra);
            e.set_global::<u32>(last_object, 0);
        }
    } else if tag == CHUNK_LVLO {
        e.with_stack(0xc, |e, data| {
            let data = data.addr();
            e.call(FILL_MEMORY, &args![data, 0u32, 0xcu32]);
            e.call(GET_CHUNK_DATA_SIZED, &args![file, data, 0xcu32]);
            if e.call(FILE_FIXES_IDS, &args![file]).bool() {
                e.call(FIX_UP_LVLO, &args![data]);
            }
            let form = e.mem.u32(data + 4);
            let count = u32::from(e.mem.u16(data + 8));
            let level = u32::from(e.mem.u16(data));
            let object = e
                .call(
                    LEVELED_LIST_ADD,
                    &args![this.addr().wrapping_add(0x30), level, count, form, 0u32],
                )
                .u32();
            e.set_global::<u32>(last_object, object);
        });
    }
}

/// The `LVLD` case: one byte read (cleared first) as the chance of none.
fn load_chance_none(e: &mut Engine, this: Ptr, file: Ptr) {
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        e.call(GET_CHUNK_DATA_SIZED, &args![file, byte, 0u32]);
        let chance = u32::from(e.mem.u8(byte.addr()));
        e.call(
            SET_CHANCE_NONE,
            &args![this.addr().wrapping_add(0x30), chance],
        );
    });
}

/// The `LVLG` case: the global's form id (cleared first) into the leveled
/// list's `pChanceGlobal` (+0x40).
fn load_chance_global(e: &mut Engine, this: Ptr, file: Ptr) {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.call(GET_CHUNK_DATA, &args![file, word]);
        let value = e.mem.u32(word.addr());
        e.mem.set_u32(this.addr().wrapping_add(0x40), value);
    });
}

// Translated from 0050af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLevCreature`'s load (unnamed in the engine map; the class by the
/// form type 0x2c and the vtables its constructor sets), `thiscall`,
/// `RET 4`: the file. Answers false for another record type. Otherwise
/// `TESForm::LoadForm`, then each chunk: `EDID` (vtable `+0x134`), `OBND`
/// (vtable `+0xe0`), `LVLD` (chance of none), `LVLF` (the flags byte at
/// +0x3d, size 0), `LVLG` (the chance global), `LVLO`/`COED` (objects and
/// their extras), `MODL`/`MODT`/`MODD`/`MOSD` (`TESModel::LoadModelChunk` on
/// the model at +0x4c) and `MODS` (texture swaps: the chunk into a stack
/// block of its size, unpacked with `00403570`'s word). At the end
/// `00484ab0` with 0; answers true.
// Translation incomplete: 0050af60 - the SEH frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_0050af60(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x2c {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        match tag {
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_OBND => {
                e.vcall(this.addr(), LOAD_BOUNDS_SLOT, &args![file]);
            }
            CHUNK_COED | CHUNK_LVLO => load_leveled_object(e, this, file, LAST_LEV_CREATURE_OBJECT),
            CHUNK_LVLD => load_chance_none(e, this, file),
            CHUNK_LVLF => {
                e.call(
                    GET_CHUNK_DATA_SIZED,
                    &args![file, this.addr().wrapping_add(0x3d), 0u32],
                );
            }
            CHUNK_LVLG => load_chance_global(e, this, file),
            CHUNK_MODL | CHUNK_MODT | CHUNK_MODD | CHUNK_MOSD => {
                let model = if this.is_null() {
                    0
                } else {
                    this.addr().wrapping_add(0x4c)
                };
                e.call(LOAD_MODEL_CHUNK, &args![model, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                with_chunk_stack(e, size, |e, block| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, block, size]);
                    let word = u32::from(e.call(FILE_WORD, &args![file]).u16());
                    e.call(
                        UNPACK_TEXTURE_SWAP,
                        &args![this.addr().wrapping_add(0x4c), block, size, word],
                    );
                });
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    true
}

// Translated from 0050b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLevItem`'s load (unnamed in the engine map; the class by the form
/// type 0x34 and the vtables its constructor sets), `thiscall`, `RET 4`: the
/// file. As `0050af60` without the model chunks: answers false for another
/// record type; otherwise `TESForm::LoadForm`, then `EDID`, `OBND`, `LVLD`,
/// `LVLF`, `LVLG` and `LVLO`/`COED` (remembering the last object in its own
/// static), `00484ab0` with 0 at the end; answers true.
// Translation incomplete: 0050b560 - the SEH frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_0050b560(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x34 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        match tag {
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_OBND => {
                e.vcall(this.addr(), LOAD_BOUNDS_SLOT, &args![file]);
            }
            CHUNK_COED | CHUNK_LVLO => load_leveled_object(e, this, file, LAST_LEV_ITEM_OBJECT),
            CHUNK_LVLD => load_chance_none(e, this, file),
            CHUNK_LVLF => {
                e.call(
                    GET_CHUNK_DATA_SIZED,
                    &args![file, this.addr().wrapping_add(0x3d), 0u32],
                );
            }
            CHUNK_LVLG => load_chance_global(e, this, file),
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    true
}

/// The shape of `0050ad90`, `0050b370` and `0050b8d0`: the word at the
/// address `0043d4d0` answers for the static (`this` is unused).
fn static_word(e: &mut Engine, holder: u32) -> u32 {
    let address = e.call(STATIC_VALUE_ADDRESS, &args![holder]).u32();
    e.mem.u32(address)
}

// Translated from 0050ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the word behind
/// the static `011c8e1c` (`0043d4d0`).
pub fn fn_0050ad90(e: &mut Engine, _this: Ptr) -> u32 {
    static_word(e, LEV_CREATURE_STATIC)
}

// Translated from 0050b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the word behind
/// the static `011c8ed4` (`0043d4d0`).
pub fn fn_0050b370(e: &mut Engine, _this: Ptr) -> u32 {
    static_word(e, LEV_ITEM_STATIC)
}

// Translated from 0050b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` unused): the word behind
/// the static `011c8f00` (`0043d4d0`).
pub fn fn_0050b8d0(e: &mut Engine, _this: Ptr) -> u32 {
    static_word(e, LEV_SPELL_STATIC)
}

// Translated from 0050adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): passes the word to
/// `00484d60` on the form and to `00488c50` on its leveled list (+0x30).
pub fn fn_0050adb0(e: &mut Engine, this: Ptr, other: u32) {
    e.call(FORM_COPY, &args![this, other]);
    e.call(
        LEVELED_LIST_COPY,
        &args![this.addr().wrapping_add(0x30), other],
    );
}

// Translated from 0050b890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: unless `004013e0` answers true, runs the
/// leveled list's `InitItem` (`00488570` on +0x30 with the form) and
/// `00484ab0` with 1.
pub fn fn_0050b890(e: &mut Engine, this: Ptr) {
    if e.call(FORM_SKIPS_INIT, &args![this]).bool() {
        return;
    }
    e.call(
        LEVELED_LIST_INIT_ITEM,
        &args![this.addr().wrapping_add(0x30), this],
    );
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 0050aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESLevCreature` destructor body (unnamed in the engine map; its
/// vtables by RTTI): sets the three vtables, destroys the model (+0x4c,
/// `0048a490`) and the leveled list (+0x30, `004876d0`), then the form base
/// (`0050e9f0`).
// Translation incomplete: 0050aed0 - the SEH frame (FS:[0], unwind of the bases) is not translated.
pub fn fn_0050aed0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LEV_CREATURE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x30), LEV_CREATURE_LIST_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x4c), LEV_CREATURE_MODEL_VTABLE);
    e.call(MODEL_DESTRUCT, &args![base.wrapping_add(0x4c)]);
    e.call(LEVELED_LIST_DESTRUCT, &args![base.wrapping_add(0x30)]);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

// Translated from 0050b4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESLevItem` destructor body (unnamed in the engine map; its vtables
/// by RTTI): sets the two vtables, destroys the leveled list (+0x30,
/// `004876d0`), then the form base (`0050e9f0`).
// Translation incomplete: 0050b4c0 - the SEH frame (FS:[0], unwind of the bases) is not translated.
pub fn fn_0050b4c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LEV_ITEM_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x30), LEV_ITEM_LIST_VTABLE);
    e.call(LEVELED_LIST_DESTRUCT, &args![base.wrapping_add(0x30)]);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

/// The `LEVELED_OBJECT` the last `LVLO` chunk of a type-0x4C record added (`011c8f98`), which
/// a following `COED` chunk gives its extra.
const LAST_TYPE_4C_OBJECT: u32 = 0x011c_8f98;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, 0, from, to, 0).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm::CopyAllComponents` and `TESForm::CompareAllComponents` (Xbox PDB).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `TESForm::StartForm` and `TESForm::CloseForm` (Xbox PDB).
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The list walk: `00717e50` (the leveled list's first node), `006815c0` (the node, whose
/// word +0 is its entry) and `00726070` (the next node).
const LIST_HEAD: u32 = 0x0071_7e50;
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `00487f50` (no arguments): a random word.
const RANDOM_WORD: u32 = 0x0048_7f50;
/// The form list `0050c000` and `0050c1d0` build: `operator new(8)` and the constructor
/// `0096a2d0` (null when the allocation fails), `005ae3d0` adds the word at an address.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_ADD: u32 = 0x005a_e3d0;

/// A cast of `other` from `01183028` to `to`, then `TESForm::CopyAllComponents` with `other`
/// itself (not the cast pointer) when it casts.
fn copy_if_cast(e: &mut Engine, this: Ptr, other: u32, to: u32) {
    let cast = e
        .call(DYNAMIC_CAST, &args![other, 0u32, 0x0118_3028u32, to, 0u32])
        .u32();
    if cast != 0 {
        e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    }
}

/// True unless `other` casts from `01183028` to `to` and `TESForm::CompareAllComponents`
/// with the cast pointer finds no difference.
fn differs_unless_cast(e: &mut Engine, this: Ptr, other: u32, to: u32) -> bool {
    let cast = e
        .call(DYNAMIC_CAST, &args![other, 0u32, 0x0118_3028u32, to, 0u32])
        .u32();
    if cast == 0 {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, cast]).bool()
}

/// A new form list (`operator new(8)`, `0096a2d0`), or null when the allocation fails.
fn new_form_list(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    }
}

// Translated from 0050b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the components (`RET 4`) of `other` itself when it casts to `01189c10`.
pub fn fn_0050b2e0(e: &mut Engine, this: Ptr, other: u32) {
    copy_if_cast(e, this, other, 0x0118_9c10);
}

// Translated from 0050b320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts to `01189c10` and
/// `TESForm::CompareAllComponents` finds no difference.
pub fn fn_0050b320(e: &mut Engine, this: Ptr, other: u32) -> bool {
    differs_unless_cast(e, this, other, 0x0118_9c10)
}

// Translated from 0050b390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a saved game's data (`RET 4`): `00484da0` on the form, then
/// `TESLeveledList::LoadGame` (`00488d70`) on the leveled list at +0x30, both with `buffer`.
pub fn fn_0050b390(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4da0, &args![this, buffer]);
    e.call(0x0048_8d70, &args![this.addr() + 0x30, buffer]);
}

// Translated from 0050b3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004534f0` on the form, then `00488f10` on the leveled list at +0x30, both with `buffer`
/// (`RET 4`).
pub fn fn_0050b3c0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0045_34f0, &args![this, buffer]);
    e.call(0x0048_8f10, &args![this.addr() + 0x30, buffer]);
}

// Translated from 0050b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A form's save (`ECX` only): `TESForm::StartForm`, `00483710`, `00488400` on the leveled
/// list at +0x30 and `TESForm::CloseForm`.
pub fn fn_0050b530(e: &mut Engine, this: Ptr) {
    e.call(START_FORM, &args![this]);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0048_8400, &args![this.addr() + 0x30]);
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 0050b800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the components (`RET 4`) of `other` itself when it casts to `01186f7c`.
pub fn fn_0050b800(e: &mut Engine, this: Ptr, other: u32) {
    copy_if_cast(e, this, other, 0x0118_6f7c);
}

// Translated from 0050b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts to `01186f7c` and
/// `TESForm::CompareAllComponents` finds no difference.
pub fn fn_0050b840(e: &mut Engine, this: Ptr, other: u32) -> bool {
    differs_unless_cast(e, this, other, 0x0118_6f7c)
}

// Translated from 0050b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050b9b0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The type-0x4C leveled form's destructor body (`ECX`): the vtables `01028c5c` and
/// `01028c38` (+0x30), then `00483710`, the leveled list's destructor body (`004876d0` on
/// +0x30) and the base destructor `0050e9f0`.
pub fn fn_0050b9b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_8c5c);
    e.mem.set_u32(base + 0x30, 0x0102_8c38);
    e.call(0x0048_3710, &args![this]);
    e.call(LEVELED_LIST_DESTRUCT, &args![base + 0x30]);
    e.call(FORM_BASE_DESTRUCT, &args![this]);
}

// Translated from 0050ba30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050ba30 - the SEH frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block.
/// The type-0x4C leveled form's load (`thiscall`, `RET 4`: the file): false for another
/// record type; otherwise `TESForm::LoadForm`, then each chunk: `EDID` (vtable +0x134),
/// `OBND` (vtable +0xE0), `LVLD` (chance of none), `LVLF` (the flags byte at +0x3D, size 0),
/// `LVLG` (the chance global), `LVLO`/`COED` (objects and their extras, the last object
/// kept in `011c8f98`); at the end `00484ab0` with 0; true.
pub fn fn_0050ba30(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x4c {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
        match tag {
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_OBND => {
                e.vcall(this.addr(), LOAD_BOUNDS_SLOT, &args![file]);
            }
            CHUNK_COED | CHUNK_LVLO => load_leveled_object(e, this, file, LAST_TYPE_4C_OBJECT),
            CHUNK_LVLD => load_chance_none(e, this, file),
            CHUNK_LVLF => {
                e.call(
                    GET_CHUNK_DATA_SIZED,
                    &args![file, this.addr().wrapping_add(0x3d), 0u32],
                );
            }
            CHUNK_LVLG => load_chance_global(e, this, file),
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    true
}

// Translated from 0050bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the components (`RET 4`) of `other` itself when it casts to `01187818`.
pub fn fn_0050bcd0(e: &mut Engine, this: Ptr, other: u32) {
    copy_if_cast(e, this, other, 0x0118_7818);
}

// Translated from 0050bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts to `01187818` and
/// `TESForm::CompareAllComponents` finds no difference.
pub fn fn_0050bd10(e: &mut Engine, this: Ptr, other: u32) -> bool {
    differs_unless_cast(e, this, other, 0x0118_7818)
}

// Translated from 0050bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks an entry of the leveled list at +0x30 (`RET 0x14`): clears `*form` and the 16-bit
/// `*count`; when the chance of none (vtable slot 0x10 of the list, a byte) is above a
/// random number modulo 100 (`00487f50`), nothing. Otherwise walks the entries
/// (`00717e50`, each entry's level the 16-bit word at +6) while their level is at most
/// `level` and, for entries whose form casts to `01183140`, the skill `0040ca30` gives for
/// `004065e0` of its +0xC part is `owner`; the highest level seen (a level above the best
/// one, kept unless `004877e0` says to use all) starts a new run, an equal or lower one
/// lengthens it. Among the run a random one (`00487f50` modulo its length) is taken: its
/// form (+0) and count (+4), and when the form casts to `01187818` and `recurse` is set,
/// the pick of that list instead (recursively, the count multiplied).
#[allow(clippy::too_many_arguments)]
pub fn fn_0050bd60(
    e: &mut Engine,
    this: Ptr,
    owner: u32,
    level: u16,
    form: u32,
    count: u32,
    recurse: u8,
) {
    let list = this.addr() + 0x30;
    e.mem.set_u32(form, 0);
    e.mem.set_u16(count, 0);
    let chance = e.vcall(list, 0x10, &args![]).u8();
    if i32::from(chance) > 0 {
        let roll = e.call(RANDOM_WORD, &args![]).u32() % 100;
        if (roll as i32) < i32::from(chance) {
            return;
        }
    }
    let mut run = 1u32;
    let mut best_level = 0i32;
    let mut best_node = 0u32;
    let mut node = e.call(LIST_HEAD, &args![list]).u32();
    while node != 0 {
        let item = e.call(NODE_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(NODE_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item);
        let entry_form = e.mem.u32(entry);
        let cast = e
            .call(
                DYNAMIC_CAST,
                &args![entry_form, 0u32, 0x0118_3028u32, 0x0118_3140u32, 0u32],
            )
            .u32();
        let entry_level = i32::from(e.mem.u16(entry + 6));
        if entry_level > i32::from(level) {
            break;
        }
        if cast != 0 {
            let skill = e.call(0x0040_65e0, &args![cast + 0xc]).u32();
            if e.call(0x0040_ca30, &args![skill]).u32() != owner {
                break;
            }
        }
        if entry_level > best_level
            && (best_level == 0 || !e.call(0x0048_77e0, &args![list]).bool())
        {
            run = 1;
            best_node = node;
            best_level = entry_level;
        } else {
            run = run.wrapping_add(1);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if best_node == 0 {
        return;
    }
    let mut steps = e.call(RANDOM_WORD, &args![]).u32() % run;
    let mut node = best_node;
    while steps != 0 {
        node = e.call(NODE_NEXT, &args![node]).u32();
        steps -= 1;
        if node == 0 {
            return;
        }
    }
    let item = e.call(NODE_ITEM, &args![node]).u32();
    let entry = e.mem.u32(item);
    let mut picked = e.mem.u32(entry);
    let mut number = e.mem.u16(entry + 4);
    let entry_form = e.mem.u32(entry);
    let nested = e
        .call(
            DYNAMIC_CAST,
            &args![entry_form, 0u32, 0x0118_3028u32, 0x0118_7818u32, 0u32],
        )
        .u32();
    if nested != 0 && recurse != 0 {
        e.with_stack(8, |e, locals| {
            let inner_form = locals.addr();
            let inner_count = inner_form + 4;
            e.mem.set_u32(inner_form, 0);
            e.mem.set_u16(inner_count, 0);
            fn_0050bd60(
                e,
                Ptr::new(nested),
                owner,
                level,
                inner_form,
                inner_count,
                recurse,
            );
            picked = e.mem.u32(inner_form);
            number = (u32::from(number).wrapping_mul(u32::from(e.mem.u16(inner_count)))) as u16;
        });
    }
    e.mem.set_u32(form, picked);
    e.mem.set_u16(count, number);
}

// Translated from 0050bf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pick of `0050bd60` (`RET 8`, recursing) cast to `01183060`, or null.
pub fn fn_0050bf90(e: &mut Engine, this: Ptr, owner: u32, level: u16) -> u32 {
    e.with_stack(8, |e, locals| {
        let form = locals.addr();
        let count = form + 4;
        e.mem.set_u32(form, 0);
        e.mem.set_u16(count, 0);
        fn_0050bd60(e, this, owner, level, form, count, 1);
        let picked = e.mem.u32(form);
        if picked == 0 {
            return 0;
        }
        e.call(
            DYNAMIC_CAST,
            &args![picked, 0u32, 0x0118_3028u32, 0x0118_3060u32, 0u32],
        )
        .u32()
    })
}

// Translated from 0050c000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050c000 - the exception-unwinding frame (FS:[0]) is not translated.
/// The forms the leveled list at +0x30 gives (`RET 8`), as a new form list (`005ae3d0`
/// adds): when `004878a0` says to use all, every entry's form that casts to `01183060`, and
/// for forms that cast to `01187818` their pick (`0050bf90`); otherwise this list's pick
/// (`0050bf90`). Null when nothing was added to a list that was never made.
pub fn fn_0050c000(e: &mut Engine, this: Ptr, owner: u32, level: u16) -> u32 {
    let list = this.addr() + 0x30;
    let mut result = 0u32;
    e.with_stack(8, |e, locals| {
        let added = locals.addr();
        let single = added + 4;
        if e.call(0x0048_78a0, &args![list]).bool() {
            let mut node = e.call(LIST_HEAD, &args![list]).u32();
            while node != 0 {
                let item = e.call(NODE_ITEM, &args![node]).u32();
                let entry = e.mem.u32(item);
                if entry != 0 {
                    if result == 0 {
                        result = new_form_list(e);
                    }
                    let entry_form = e.mem.u32(entry);
                    let form = e
                        .call(
                            DYNAMIC_CAST,
                            &args![entry_form, 0u32, 0x0118_3028u32, 0x0118_3060u32, 0u32],
                        )
                        .u32();
                    e.mem.set_u32(added, form);
                    if form != 0 {
                        e.call(LIST_ADD, &args![result, added]);
                    } else {
                        let entry_form = e.mem.u32(entry);
                        let nested = e
                            .call(
                                DYNAMIC_CAST,
                                &args![entry_form, 0u32, 0x0118_3028u32, 0x0118_7818u32, 0u32],
                            )
                            .u32();
                        if nested != 0 {
                            let picked = fn_0050bf90(e, Ptr::new(nested), owner, level);
                            e.mem.set_u32(added, picked);
                            if picked != 0 {
                                e.call(LIST_ADD, &args![result, added]);
                            }
                        }
                    }
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
        } else {
            let picked = fn_0050bf90(e, this, owner, level);
            e.mem.set_u32(single, picked);
            if picked != 0 {
                result = new_form_list(e);
                e.call(LIST_ADD, &args![result, single]);
            }
        }
    });
    result
}

// Translated from 0050c1d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050c1d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The forms of all six owners (`RET 4`): for each index 0..6, the owner `0040ca10` gives,
/// `actor`'s +0xA4 part's vtable slot 8 for it (the level), the forms `0050c000` gives,
/// each new one (`005f65d0` says whether it is in) added to one form list made on the first
/// (`operator new(8)`, `0096a2d0`); each per-index list is emptied (`00470470`) and deleted
/// (`004702f0` with 1). Answers the form list, or null.
pub fn fn_0050c1d0(e: &mut Engine, this: Ptr, actor: u32) -> u32 {
    let mut result = 0u32;
    e.with_stack(4, |e, local| {
        let entry_word = local.addr();
        let mut index = 0i32;
        while index < 6 {
            let owner = e.call(0x0040_ca10, &args![index as u32]).u32();
            let value = e.vcall(actor + 0xa4, 8, &args![owner]).u32();
            let forms = fn_0050c000(e, this, index as u32, value as u16);
            let mut node = forms;
            while node != 0 {
                let item = e.call(NODE_ITEM, &args![node]).u32();
                let entry = e.mem.u32(item);
                e.mem.set_u32(entry_word, entry);
                if entry != 0 {
                    let mut add = true;
                    if result == 0 {
                        result = new_form_list(e);
                    } else {
                        add = !e.call(0x005f_65d0, &args![result, entry_word]).bool();
                    }
                    if add {
                        e.call(LIST_ADD, &args![result, entry_word]);
                    }
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            if forms != 0 {
                e.call(0x0047_0470, &args![forms]);
            }
            if forms != 0 {
                e.call(0x0047_02f0, &args![forms, 1u32]);
            }
            index += 1;
        }
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_adf0, fn_0050adf0(Ptr) -> Ptr),
        entry!(0x0050_b400, fn_0050b400(Ptr) -> Ptr),
        entry!(0x0050_b8f0, fn_0050b8f0(Ptr) -> Ptr),
        entry!(0x0050_ade0, fn_0050ade0(Ptr, u32) -> u32),
        entry!(0x0050_b3f0, fn_0050b3f0(Ptr, u32) -> u32),
        entry!(0x0050_aea0, fn_0050aea0(Ptr, u32) -> Ptr),
        entry!(0x0050_b490, fn_0050b490(Ptr, u32) -> Ptr),
        entry!(0x0050_b980, fn_0050b980(Ptr, u32) -> Ptr),
        entry!(0x0050_af60, fn_0050af60(Ptr, Ptr) -> bool),
        entry!(0x0050_b560, fn_0050b560(Ptr, Ptr) -> bool),
        entry!(0x0050_ad90, fn_0050ad90(Ptr) -> u32),
        entry!(0x0050_b370, fn_0050b370(Ptr) -> u32),
        entry!(0x0050_b8d0, fn_0050b8d0(Ptr) -> u32),
        entry!(0x0050_adb0, fn_0050adb0(Ptr, u32)),
        entry!(0x0050_b890, fn_0050b890(Ptr)),
        entry!(0x0050_aed0, fn_0050aed0(Ptr)),
        entry!(0x0050_b4c0, fn_0050b4c0(Ptr)),
        entry!(0x0050_b2e0, fn_0050b2e0(Ptr, u32)),
        entry!(0x0050_b320, fn_0050b320(Ptr, u32) -> bool),
        entry!(0x0050_b390, fn_0050b390(Ptr, u32)),
        entry!(0x0050_b3c0, fn_0050b3c0(Ptr, u32)),
        entry!(0x0050_b530, fn_0050b530(Ptr)),
        entry!(0x0050_b800, fn_0050b800(Ptr, u32)),
        entry!(0x0050_b840, fn_0050b840(Ptr, u32) -> bool),
        entry!(0x0050_b9b0, fn_0050b9b0(Ptr)),
        entry!(0x0050_ba30, fn_0050ba30(Ptr, Ptr) -> bool),
        entry!(0x0050_bcd0, fn_0050bcd0(Ptr, u32)),
        entry!(0x0050_bd10, fn_0050bd10(Ptr, u32) -> bool),
        entry!(0x0050_bd60, fn_0050bd60(Ptr, u32, u16, u32, u32, u8)),
        entry!(0x0050_bf90, fn_0050bf90(Ptr, u32, u16) -> u32),
        entry!(0x0050_c000, fn_0050c000(Ptr, u32, u16) -> u32),
        entry!(0x0050_c1d0, fn_0050c1d0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0050adf0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_adf0,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2c)]),
                (0x0048_3710, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_886c),
                (0x30, 4, 0x0102_884c),
                (0x4c, 4, 0x0102_8828),
            ],
        });
    }

    #[test]
    fn fn_0050b400_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b400,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x34)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_8a64), (0x30, 4, 0x0102_8a44)],
        });
    }

    #[test]
    fn fn_0050b8f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_b8f0,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_7650, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4c)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[(0x0, 4, 0x0102_8c5c), (0x30, 4, 0x0102_8c38)],
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
    fn le81_thunk_0050ade0() {
        assert_eq!(
            check(0x0050_ade0, 0x0050_a820, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0050b3f0() {
        assert_eq!(
            check(0x0050_b3f0, 0x0050_aea0, 0x4c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_0050aea0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_aed0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_aea0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_aea0)
                .collect();
            let mut expected = vec![(0x0050_aed0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050b490_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_b4c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_b490, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_b490)
                .collect();
            let mut expected = vec![(0x0050_b4c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0050b980_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_b9b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_b980, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_b980)
                .collect();
            let mut expected = vec![(0x0050_b9b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FORM_VTABLE: u32 = 0x00ab_0000;
    const EDITOR_ID_FN: u32 = 0x00ab_1000;
    const BOUNDS_FN: u32 = 0x00ab_1004;

    type Chunks = Rc<RefCell<(usize, Vec<(u32, Vec<u8>)>)>>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A file of the given record type and chunks (tag, data): the chunk
    /// readers copy the current chunk's data (at most the size asked for, all
    /// of it for size 0).
    fn engine(record_type: u8, chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_8000, 0x1000);
        let state: Chunks = Rc::new(RefCell::new((0, chunks)));
        e.register_double(GET_TES_FORM, move |_, _| u32::from(record_type).into_ret());
        let s = state.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.1.get(s.0).map_or(0, |c| c.0).into_ret()
        });
        let s = state.clone();
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.0 += 1;
            u32::from(s.0 < s.1.len()).into_ret()
        });
        let s = state.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            (s.1[s.0].1.len() as u32).into_ret()
        });
        let s = state.clone();
        e.register_double(GET_CHUNK_DATA_SIZED, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            let n = if a[2] == 0 {
                data.len()
            } else {
                data.len().min(a[2] as usize)
            };
            e.mem.write(a[1], &data[..n]);
            Ret::default()
        });
        let s = state;
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            e.mem.write(a[1], &data);
            Ret::default()
        });
        e.register(FILL_MEMORY, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONTAINER_ITEM_EXTRA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FILE_FIXES_IDS, |_, _| 1u32.into_ret());
        e.register(FILE_WORD, |_, _| 0x1_0007u32.into_ret());
        e.register(EDITOR_ID_FN, |_, _| Ret::default());
        e.register(BOUNDS_FN, |_, _| Ret::default());
        for addr in [
            LOAD_FORM,
            FORM_SET_FLAG,
            FIX_UP_LVLO,
            SET_CHANCE_NONE,
            CONTAINER_ITEM_EXTRA_LOAD,
            LOAD_MODEL_CHUNK,
            UNPACK_TEXTURE_SWAP,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(LEVELED_LIST_ADD, |e, _| e.mem.alloc(0xc).into_ret());
        let mut slots = vec![0u32; 0x4e];
        slots[0x134 / 4] = EDITOR_ID_FN;
        slots[0xe0 / 4] = BOUNDS_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        e
    }

    fn lvlo(level: u16, form: u32, count: u16) -> Vec<u8> {
        let mut v = level.to_le_bytes().to_vec();
        v.extend([0, 0]);
        v.extend(form.to_le_bytes());
        v.extend(count.to_le_bytes());
        v.extend([0, 0]);
        v
    }

    fn chunks() -> Vec<(u32, Vec<u8>)> {
        vec![
            (CHUNK_EDID, b"LL\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_LVLD, vec![25]),
            (CHUNK_LVLF, vec![3]),
            (CHUNK_LVLG, 0x1234u32.to_le_bytes().to_vec()),
            (CHUNK_LVLO, lvlo(5, 0xabcd, 2)),
            (CHUNK_COED, vec![0; 12]),
            (CHUNK_MODL, b"a.nif\0".to_vec()),
            (CHUNK_MODS, vec![1, 2, 3]),
        ]
    }

    fn form(e: &mut Engine) -> u32 {
        let f = e.mem.alloc(0x70);
        e.mem.set_u32(f, FORM_VTABLE);
        f
    }

    #[test]
    fn fn_0050af60_loads_every_chunk_of_a_leveled_creature() {
        let mut e = engine(0x2c, chunks());
        let this = form(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_af60, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, LOAD_FORM), vec![vec![this, 0x9000]]);
        assert_eq!(calls_to(&e, EDITOR_ID_FN).len(), 1);
        assert_eq!(calls_to(&e, BOUNDS_FN), vec![vec![this, 0x9000]]);
        assert_eq!(calls_to(&e, SET_CHANCE_NONE), vec![vec![this + 0x30, 25]]);
        assert_eq!(e.mem.u8(this + 0x3d), 3);
        assert_eq!(e.mem.u32(this + 0x40), 0x1234);
        assert_eq!(
            calls_to(&e, LEVELED_LIST_ADD),
            vec![vec![this + 0x30, 5, 2, 0xabcd, 0]]
        );
        assert_eq!(calls_to(&e, FIX_UP_LVLO).len(), 1);
        // The COED gave the object an extra and cleared the static.
        let extra = calls_to(&e, CONTAINER_ITEM_EXTRA_LOAD)[0][0];
        assert_ne!(extra, 0);
        assert_eq!(e.global::<u32>(LAST_LEV_CREATURE_OBJECT), 0);
        assert_eq!(
            calls_to(&e, LOAD_MODEL_CHUNK),
            vec![vec![this + 0x4c, 0x9000]]
        );
        let swap = calls_to(&e, UNPACK_TEXTURE_SWAP);
        assert_eq!(swap.len(), 1);
        assert_eq!((swap[0][0], swap[0][2], swap[0][3]), (this + 0x4c, 3, 7));
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        // Another record type: false, nothing read.
        let mut e = engine(0x34, chunks());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_af60, &args![this, 0x9000u32]).bool());
        assert!(calls_to(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_0050b560_loads_a_leveled_item_without_models() {
        let mut e = engine(0x34, chunks());
        let this = form(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_b560, &args![this, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, LEVELED_LIST_ADD).len(), 1);
        assert_eq!(calls_to(&e, CONTAINER_ITEM_EXTRA_LOAD).len(), 1);
        assert_eq!(e.global::<u32>(LAST_LEV_ITEM_OBJECT), 0);
        assert!(calls_to(&e, LOAD_MODEL_CHUNK).is_empty());
        assert!(calls_to(&e, UNPACK_TEXTURE_SWAP).is_empty());
        assert_eq!(e.mem.u32(this + 0x40), 0x1234);
        // An LVLO without COED leaves the object remembered; a COED without
        // one does nothing.
        let mut e = engine(
            0x34,
            vec![(CHUNK_COED, vec![]), (CHUNK_LVLO, lvlo(1, 2, 3))],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_b560, &args![this, 0x9000u32]).bool());
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_ne!(e.global::<u32>(LAST_LEV_ITEM_OBJECT), 0);
        let mut e = engine(0x2c, chunks());
        assert!(!e.call(0x0050_b560, &args![this, 0x9000u32]).bool());
    }

    #[test]
    fn static_words_read_through_0043d4d0() {
        let mut e = Engine::new();
        e.register(STATIC_VALUE_ADDRESS, |_, a| (a[0] + 0x100).into_ret());
        e.map(0x011c_8000, 0x2000);
        e.mem.set_u32(LEV_CREATURE_STATIC + 0x100, 11);
        e.mem.set_u32(LEV_ITEM_STATIC + 0x100, 22);
        e.mem.set_u32(LEV_SPELL_STATIC + 0x100, 33);
        assert_eq!(e.call(0x0050_ad90, &args![0u32]).u32(), 11);
        assert_eq!(e.call(0x0050_b370, &args![0u32]).u32(), 22);
        assert_eq!(e.call(0x0050_b8d0, &args![0u32]).u32(), 33);
    }

    #[test]
    fn fn_0050adb0_and_0050b890_pass_on_to_the_form_and_its_list() {
        let mut e = Engine::new();
        for addr in [
            FORM_COPY,
            LEVELED_LIST_COPY,
            LEVELED_LIST_INIT_ITEM,
            FORM_SET_FLAG,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(FORM_SKIPS_INIT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_adb0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls_to(&e, FORM_COPY), vec![vec![0x5000, 0x6000]]);
        assert_eq!(calls_to(&e, LEVELED_LIST_COPY), vec![vec![0x5030, 0x6000]]);
        e.call(0x0050_b890, &args![0x5000u32]);
        assert_eq!(
            calls_to(&e, LEVELED_LIST_INIT_ITEM),
            vec![vec![0x5030, 0x5000]]
        );
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![0x5000, 1]]);
        e.register(FORM_SKIPS_INIT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_b890, &args![0x5000u32]);
        assert!(calls_to(&e, LEVELED_LIST_INIT_ITEM).is_empty());
    }

    #[test]
    fn destructors_set_vtables_and_destroy_the_bases() {
        let mut e = Engine::new();
        for addr in [MODEL_DESTRUCT, LEVELED_LIST_DESTRUCT, FORM_BASE_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x70);
        e.call_log = Some(vec![]);
        e.call(0x0050_aed0, &args![this]);
        assert_eq!(
            [
                e.mem.u32(this),
                e.mem.u32(this + 0x30),
                e.mem.u32(this + 0x4c)
            ],
            [
                LEV_CREATURE_VTABLE,
                LEV_CREATURE_LIST_VTABLE,
                LEV_CREATURE_MODEL_VTABLE
            ]
        );
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (MODEL_DESTRUCT, vec![this + 0x4c]),
                (LEVELED_LIST_DESTRUCT, vec![this + 0x30]),
                (FORM_BASE_DESTRUCT, vec![this]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0050_b4c0, &args![this]);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 0x30)],
            [LEV_ITEM_VTABLE, LEV_ITEM_LIST_VTABLE]
        );
        assert_eq!(calls_to(&e, MODEL_DESTRUCT).len(), 0);
        assert_eq!(calls_to(&e, LEVELED_LIST_DESTRUCT), vec![vec![this + 0x30]]);
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

    fn cast_doubles(e: &mut Engine, casts: bool, differs: bool) {
        if casts {
            e.register(DYNAMIC_CAST, |_, a| (a[0] + 8).into_ret());
        } else {
            e.register(DYNAMIC_CAST, |_, _| Ret::default());
        }
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register_double(COMPARE_ALL_COMPONENTS, move |_, _| differs.into_ret());
    }

    fn check_copy(address: u32, to: u32) {
        let mut e = Engine::new();
        cast_doubles(&mut e, true, false);
        e.call_log = Some(vec![]);
        e.call(address, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![0x2000, 0, 0x0118_3028, to, 0]]
        );
        assert_eq!(
            calls_to(&e, COPY_ALL_COMPONENTS),
            vec![vec![0x1000, 0x2000]]
        );
        cast_doubles(&mut e, false, false);
        e.call_log = Some(vec![]);
        e.call(address, &args![0x1000u32, 0x2000u32]);
        assert!(calls_to(&e, COPY_ALL_COMPONENTS).is_empty());
    }

    fn check_compare(address: u32, to: u32) {
        let mut e = Engine::new();
        cast_doubles(&mut e, true, false);
        e.call_log = Some(vec![]);
        assert!(!e.call(address, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(calls_to(&e, DYNAMIC_CAST)[0][3], to);
        assert_eq!(
            calls_to(&e, COMPARE_ALL_COMPONENTS),
            vec![vec![0x1000, 0x2008]]
        );
        cast_doubles(&mut e, true, true);
        assert!(e.call(address, &args![0x1000u32, 0x2000u32]).bool());
        cast_doubles(&mut e, false, false);
        assert!(e.call(address, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn fn_0050b2e0_copies_the_uncast_form() {
        check_copy(0x0050_b2e0, 0x0118_9c10);
    }

    #[test]
    fn fn_0050b320_compares_a_cast_form() {
        check_compare(0x0050_b320, 0x0118_9c10);
    }

    #[test]
    fn fn_0050b390_loads_form_and_list() {
        let mut e = Engine::new();
        e.register(0x0048_4da0, |_, _| Ret::default());
        e.register(0x0048_8d70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_b390, &args![0x1000u32, 0x5000u32]);
        assert_eq!(calls_to(&e, 0x0048_4da0), vec![vec![0x1000, 0x5000]]);
        assert_eq!(calls_to(&e, 0x0048_8d70), vec![vec![0x1030, 0x5000]]);
    }

    #[test]
    fn fn_0050b3c0_passes_the_buffer_on() {
        let mut e = Engine::new();
        e.register(0x0045_34f0, |_, _| Ret::default());
        e.register(0x0048_8f10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_b3c0, &args![0x1000u32, 0x5000u32]);
        assert_eq!(calls_to(&e, 0x0045_34f0), vec![vec![0x1000, 0x5000]]);
        assert_eq!(calls_to(&e, 0x0048_8f10), vec![vec![0x1030, 0x5000]]);
    }

    #[test]
    fn fn_0050b530_saves_in_order() {
        let mut e = Engine::new();
        for address in [START_FORM, 0x0048_3710, 0x0048_8400, CLOSE_FORM] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0050_b530, &args![0x1000u32]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (START_FORM, vec![0x1000]),
                (0x0048_3710, vec![0x1000]),
                (0x0048_8400, vec![0x1030]),
                (CLOSE_FORM, vec![0x1000])
            ]
        );
    }

    #[test]
    fn fn_0050b800_copies_the_uncast_form() {
        check_copy(0x0050_b800, 0x0118_6f7c);
    }

    #[test]
    fn fn_0050b840_compares_a_cast_form() {
        check_compare(0x0050_b840, 0x0118_6f7c);
    }

    #[test]
    fn fn_0050b9b0_destroys_in_order() {
        let mut e = Engine::new();
        for address in [0x0048_3710, LEVELED_LIST_DESTRUCT, FORM_BASE_DESTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        e.call(0x0050_b9b0, &args![this]);
        assert_eq!(
            (e.mem.u32(this), e.mem.u32(this + 0x30)),
            (0x0102_8c5c, 0x0102_8c38)
        );
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (0x0048_3710, vec![this]),
                (LEVELED_LIST_DESTRUCT, vec![this + 0x30]),
                (FORM_BASE_DESTRUCT, vec![this])
            ]
        );
    }

    #[test]
    fn fn_0050ba30_loads_chunks_of_type_0x4c() {
        let mut e = Engine::new();
        e.register(GET_TES_FORM, |_, _| 0x34u8.into_ret());
        let this = e.mem.alloc(0x50);
        assert!(!e.call(0x0050_ba30, &args![this, 0x9000u32]).bool());
        e.register(GET_TES_FORM, |_, _| 0x4cu8.into_ret());
        let mut tags = vec![CHUNK_LVLG, CHUNK_LVLF];
        e.register_double(GET_TES_CHUNK, move |_, _| tags.remove(0).into_ret());
        let mut more = vec![true, false];
        e.register_double(NEXT_CHUNK, move |_, _| more.remove(0).into_ret());
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        for address in [LOAD_FORM, FORM_SET_FLAG, GET_CHUNK_DATA_SIZED] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_ba30, &args![this, 0x9000u32]).bool());
        assert_eq!(e.mem.u32(this + 0x40), 0x77);
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, LOAD_FORM), vec![vec![this, 0x9000]]);
    }

    #[test]
    fn fn_0050ba30_keeps_the_last_object() {
        let mut e = Engine::new();
        e.register(GET_TES_FORM, |_, _| 0x4cu8.into_ret());
        e.register(GET_TES_CHUNK, |_, _| CHUNK_LVLO.into_ret());
        e.register(NEXT_CHUNK, |_, _| false.into_ret());
        for address in [LOAD_FORM, FORM_SET_FLAG, GET_CHUNK_DATA_SIZED, FILL_MEMORY] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(FILE_FIXES_IDS, |_, _| false.into_ret());
        e.register(LEVELED_LIST_ADD, |_, _| 0x6666u32.into_ret());
        e.map(LAST_TYPE_4C_OBJECT, 4);
        let this = e.mem.alloc(0x50);
        assert!(e.call(0x0050_ba30, &args![this, 0x9000u32]).bool());
        assert_eq!(e.global::<u32>(LAST_TYPE_4C_OBJECT), 0x6666);
    }

    #[test]
    fn fn_0050bcd0_copies_the_uncast_form() {
        check_copy(0x0050_bcd0, 0x0118_7818);
    }

    #[test]
    fn fn_0050bd10_compares_a_cast_form() {
        check_compare(0x0050_bd10, 0x0118_7818);
    }

    /// A leveled list at `this` + 0x30 over `entries` (form, count, level): nodes in a
    /// chain, `00717e50` the first, `006815c0` the node, `00726070` the next. The chance of
    /// none is `chance` (vtable slot 0x10), `00487f50` answers `random`, no form casts.
    fn leveled_list(e: &mut Engine, entries: &[(u32, u16, u16)], chance: u8, random: u32) -> u32 {
        let this = e.mem.alloc(0x50);
        let mut table = vec![0u32; 8];
        table[0x10 / 4] = 0x00ec_0010;
        e.put_vtable(0x00df_0000, &table);
        e.register_double(0x00ec_0010, move |_, _| chance.into_ret());
        e.mem.set_u32(this + 0x30, 0x00df_0000);
        let nodes: Vec<u32> = entries.iter().map(|_| e.mem.alloc(8)).collect();
        for (i, &(form, count, level)) in entries.iter().enumerate() {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, form);
            e.mem.set_u16(entry + 4, count);
            e.mem.set_u16(entry + 6, level);
            e.mem.set_u32(nodes[i], entry);
            e.mem
                .set_u32(nodes[i] + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        let first = nodes.first().copied().unwrap_or(0);
        e.register_double(LIST_HEAD, move |_, _| first.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register_double(RANDOM_WORD, move |_, _| random.into_ret());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        e.register(0x0048_77e0, |_, _| false.into_ret());
        this
    }

    #[test]
    fn fn_0050bd60_picks_from_the_highest_level_run() {
        let mut e = Engine::new();
        // Levels 1, 5, 5, 9: with level 6 the run is the two level-5 entries; 7 % 2 picks
        // the second.
        let this = leveled_list(
            &mut e,
            &[(0xa1, 1, 1), (0xa2, 2, 5), (0xa3, 3, 5), (0xa4, 4, 9)],
            0,
            7,
        );
        let out = e.mem.alloc(8);
        e.call(0x0050_bd60, &args![this, 0u32, 6u32, out, out + 4, 1u32]);
        assert_eq!((e.mem.u32(out), e.mem.u16(out + 4)), (0xa3, 3));
        // The chance of none above the roll gives nothing.
        let this = leveled_list(&mut e, &[(0xa1, 1, 1)], 50, 149);
        e.mem.set_u32(out, 0x55);
        e.call(0x0050_bd60, &args![this, 0u32, 6u32, out, out + 4, 1u32]);
        assert_eq!(e.mem.u32(out), 0);
        // Nothing at or below the level gives nothing.
        let this = leveled_list(&mut e, &[(0xa4, 4, 9)], 0, 0);
        e.call(0x0050_bd60, &args![this, 0u32, 6u32, out, out + 4, 1u32]);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn fn_0050bf90_casts_the_pick() {
        let mut e = Engine::new();
        let this = leveled_list(&mut e, &[(0xa1, 1, 1)], 0, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_bf90, &args![this, 0u32, 3u32]).u32(), 0);
        let casts = calls_to(&e, DYNAMIC_CAST);
        assert_eq!(
            casts.last().unwrap(),
            &vec![0xa1, 0, 0x0118_3028, 0x0118_3060, 0]
        );
        let this = leveled_list(&mut e, &[(0xa4, 1, 9)], 0, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_bf90, &args![this, 0u32, 3u32]).u32(), 0);
        assert!(calls_to(&e, DYNAMIC_CAST)
            .iter()
            .all(|c| c[3] != 0x0118_3060));
    }

    #[test]
    fn fn_0050c000_makes_a_list_of_the_pick() {
        let mut e = Engine::new();
        let this = leveled_list(&mut e, &[(0xa1, 1, 1)], 0, 0);
        e.register(0x0048_78a0, |_, _| false.into_ret());
        e.register(DYNAMIC_CAST, |_, a| {
            if a[3] == 0x0118_3060 { a[0] } else { 0 }.into_ret()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIST_ADD, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let list = e.call(0x0050_c000, &args![this, 0u32, 3u32]).u32();
        assert_ne!(list, 0);
        assert_eq!(e.mem.u32(list), 0xa1);
        // Use-all: every entry's form that casts.
        e.register(0x0048_78a0, |_, _| true.into_ret());
        let this = leveled_list(&mut e, &[(0xb1, 1, 1), (0xb2, 1, 2)], 0, 0);
        e.register(DYNAMIC_CAST, |_, a| {
            if a[3] == 0x0118_3060 { a[0] } else { 0 }.into_ret()
        });
        e.call_log = Some(vec![]);
        let list = e.call(0x0050_c000, &args![this, 0u32, 3u32]).u32();
        assert_eq!(calls_to(&e, LIST_ADD).len(), 2);
        assert_eq!(e.mem.u32(list), 0xb2);
    }

    #[test]
    fn fn_0050c1d0_merges_six_lists_without_repeats() {
        let mut e = Engine::new();
        let this = leveled_list(&mut e, &[(0xa1, 1, 1)], 0, 0);
        e.register(0x0048_78a0, |_, _| false.into_ret());
        e.register(DYNAMIC_CAST, |_, a| {
            if a[3] == 0x0118_3060 { a[0] } else { 0 }.into_ret()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // The form list: word 0 the value, word 1 the next node (none).
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LIST_ADD, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x005f_65d0, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        for address in [0x0047_0470, 0x0047_02f0, 0x0040_ca10] {
            e.register(address, |_, _| Ret::default());
        }
        let actor = e.mem.alloc(0xb0);
        e.put_vtable(0x00df_1000, &[0, 0, 0x00ec_1008]);
        e.register(0x00ec_1008, |_, _| 3u32.into_ret());
        e.mem.set_u32(actor + 0xa4, 0x00df_1000);
        e.call_log = Some(vec![]);
        let merged = e.call(0x0050_c1d0, &args![this, actor]).u32();
        assert_eq!(e.mem.u32(merged), 0xa1);
        // Seven adds: six per-index lists and the merged list's first.
        assert_eq!(calls_to(&e, LIST_ADD).len(), 7);
        assert_eq!(calls_to(&e, 0x005f_65d0).len(), 5);
        assert_eq!(calls_to(&e, 0x0047_02f0).len(), 6);
    }
}
