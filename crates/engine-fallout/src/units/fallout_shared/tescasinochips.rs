//! `fallout shared/tescasinochips.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00504480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00504480 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6d (0x220 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_6574`) is `TESCasino`, so this is the constructor of `TESCasino`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00504480(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0102_6574);
    e.mem.set_u32(base + 0x18, 0x0102_6560);
    e.call(
        0x00ec_782f,
        &args![base + 0x24, 0x20u32, 0xau32, 0x0048_a3d0u32, 0x0048_a490u32],
    );
    e.call(
        0x00ec_782f,
        &args![base + 0x164, 0xcu32, 0xbu32, 0x0048_e270u32, 0x0048_e2e0u32],
    );
    e.call(0x004f_15a0, &args![this, 0x6du32]);
    this
}

// Translated from 00504540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00504570` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00504540(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_4570, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `004013e0`: true makes `InitItem` do nothing; `00484ab0`: the form
/// flag setter.
const FORM_INIT_SKIP: u32 = 0x0040_13e0;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_FORM_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_SWAPS_ENDIAN: u32 = 0x0040_1680;
const FILE_VERSION: u32 = 0x0040_3570;
/// Component readers, cdecl `(component, file)`: `TESModel::LoadModelChunk`
/// (`004892d0`), `TESModel::LoadModelTextureChunk` (`004893e0`),
/// `TESTexture::LoadTextureChunk` (`0048e3d0`), the full name
/// (`00487050`), the message icon (`0047a290`) and
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`).
const LOAD_MODEL: u32 = 0x0048_92d0;
const LOAD_MODEL_TEXTURE: u32 = 0x0048_93e0;
const LOAD_TEXTURE: u32 = 0x0048_e3d0;
const LOAD_FULL_NAME: u32 = 0x0048_7050;
const LOAD_MESSAGE_ICON: u32 = 0x0047_a290;
const LOAD_DESTRUCTIBLE: u32 = 0x0047_81e0;
/// `memset` (`00403d30`, cdecl, three words).
const MEMSET: u32 = 0x0040_3d30;

const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_MOD2: u32 = 0x3244_4f4d;
const CHUNK_MOD3: u32 = 0x3344_4f4d;
const CHUNK_MOD4: u32 = 0x3444_4f4d;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_ICO2: u32 = 0x324f_4349;
const CHUNK_MICO: u32 = 0x4f43_494d;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_YNAM: u32 = 0x4d41_4e59;
const CHUNK_ZNAM: u32 = 0x4d41_4e5a;
const CHUNK_ONAM: u32 = 0x4d41_4e4f;
const CHUNK_DATA: u32 = 0x4154_4144;

/// A chunk whose text (on the stack, `__alloca_probe_16` `00ec5ec0`, read
/// with size word `limit`) goes to `f`.
fn with_chunk_text(e: &mut Engine, file: Ptr, limit: u32, f: impl FnOnce(&mut Engine, Ptr)) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
        e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, limit]);
        f(e, text);
    });
}

/// A form id chunk read into a zeroed local (`004727f0`).
fn read_form_id(e: &mut Engine, file: Ptr) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(FILE_CHUNK_DATA, &args![file, slot]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 00503e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCasinoChips::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// `BGSDestructibleObjectForm::InitItem` (+0x80),
/// `TESModelTextureSwap::InitItem` (+0x3C), `BGSPickupPutdownSounds::
/// InitItem` (+0x88), each with the form, then `00484ab0(1)`.
pub fn tes_casino_chips_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_INIT_SKIP, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(0x0047_8be0, &args![base + 0x80, this]);
    e.call(0x0048_aa80, &args![base + 0x3c, this]);
    e.call(0x0047_b6a0, &args![base + 0x88, this]);
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
}

// Translated from 00503ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00503ef0 - the /GS stack cookie check (00ec408c) is not translated.
/// The casino chips' record reader (`RET 4`: the `TESFile`; the engine map
/// has no name): false unless the form type byte is 0x6C; otherwise
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID`, `OBND`,
/// `FULL` (+0x30), `MODL`/`MODT`/`MODD`/`MOSD` (the model at +0x3C), `MODS`
/// (its texture swaps), `ICON` (+0x5C), `MICO` (+0x68), `DEST`/`DSTD`
/// (+0x80), `YNAM`/`ZNAM` (the sounds at +0x88, `006ecd40` and `00403550`)
/// and `ONAM` (the text into the string at +0x94, `004037f0`). Returns
/// true.
pub fn fn_00503ef0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    if form_type != 0x6c {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    let or_null = |offset: u32| if this.is_null() { 0 } else { base + offset };
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_ONAM => with_chunk_text(e, file, 0, |e, text| {
                e.call(0x0040_37f0, &args![base + 0x94, text, 0u32]);
            }),
            CHUNK_MOSD | CHUNK_MODD | CHUNK_MODL | CHUNK_MODT => {
                e.call(LOAD_MODEL, &args![or_null(0x3c), file]);
            }
            CHUNK_EDID => with_chunk_text(e, file, 0x200, |e, text| {
                e.vcall(base, 0x134, &args![text]);
            }),
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_DEST | CHUNK_DSTD => {
                e.call(LOAD_DESTRUCTIBLE, &args![or_null(0x80), file]);
            }
            CHUNK_FULL => {
                e.call(LOAD_FULL_NAME, &args![or_null(0x30), file]);
            }
            CHUNK_MICO => {
                e.call(LOAD_MESSAGE_ICON, &args![or_null(0x68), file]);
            }
            CHUNK_YNAM => {
                let id = read_form_id(e, file);
                e.call(0x006e_cd40, &args![base + 0x88, id]);
            }
            CHUNK_ZNAM => {
                let id = read_form_id(e, file);
                e.call(0x0040_3550, &args![base + 0x88, id]);
            }
            CHUNK_ICON => {
                e.call(LOAD_TEXTURE, &args![or_null(0x5c), file]);
            }
            CHUNK_MODS => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                e.with_stack((size.wrapping_add(15) & !15).max(16), |e, data| {
                    e.call(FILE_CHUNK_DATA_SIZED, &args![file, data, size]);
                    let version = e.call(FILE_VERSION, &args![file]).u16();
                    e.call(
                        0x0048_a7b0,
                        &args![base + 0x3c, data, size, u32::from(version)],
                    );
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

// Translated from 00504570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00504570 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESCasino` (the vtables `00504480` stores): puts
/// back the vtables, clears the data (`00504620`), destroys the eleven
/// 0xC-byte textures at +0x164 (destructor `0048e2e0`) and the ten
/// 0x20-byte models at +0x24 (destructor `0048a490`) through
/// `` `eh vector destructor iterator' `` (`00ec5fce`), the full name at
/// +0x18 (`00591300`) and the `TESForm` base (`00483630`).
pub fn fn_00504570(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_6574);
    e.mem.set_u32(base + 0x18, 0x0102_6560);
    fn_00504620(e, this);
    e.call(
        0x00ec_5fce,
        &args![base + 0x164, 0xcu32, 0xbu32, 0x0048_e2e0u32],
    );
    e.call(
        0x00ec_5fce,
        &args![base + 0x24, 0x20u32, 0xau32, 0x0048_a490u32],
    );
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00504620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the casino's 0x38-byte data at +0x1E8 (`memset`).
pub fn fn_00504620(e: &mut Engine, this: Ptr) {
    e.call(MEMSET, &args![this.addr() + 0x1e8, 0u32, 0x38u32]);
}

// Translated from 005046e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adjusts the form id at +0x214 of the casino's data to its file
/// (`TESForm::AddCompileIndex` with `TESForm::GetFile(-1)`).
pub fn fn_005046e0(e: &mut Engine, this: Ptr) {
    let file = e.call(0x0048_4e60, &args![this, 0xffff_ffffu32]).u32();
    e.call(0x0048_5d50, &args![this.addr() + 0x214, file]);
}

// Translated from 005048d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005048d0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESCasino::Load` (Xbox PDB): false unless the form type byte is 0x6D;
/// otherwise `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID`,
/// `OBND`, `FULL` (+0x18), `MODL` (the next of the models at +0x24, 0x20
/// bytes each: its path through vtable +0x18, then its textures),
/// `MOD2`/`MOD3`/`MOD4` (the models at +0x104, +0x124, +0x144 the same
/// way), `ICON` (the next texture at +0x164, 0xC bytes each, from the
/// first), `ICO2` (the next texture from the eighth) and `DATA` (0x38
/// bytes at +0x1E8, swapped by `CASINO_DATA::Endian`). Returns true.
pub fn tes_casino_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(FILE_FORM_TYPE, &args![file]).u8();
    if form_type != 0x6d {
        return false;
    }
    let base = this.addr();
    e.call(0x0048_5110, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    let mut model = 0u32;
    let mut second_icon = 7u32;
    let mut icon = 0u32;
    let model_from_text = |e: &mut Engine, component: u32| {
        with_chunk_text(e, file, 0, |e, text| {
            e.vcall(component, 0x18, &args![text]);
        });
        e.call(LOAD_MODEL_TEXTURE, &args![component, file]);
    };
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => with_chunk_text(e, file, 0x200, |e, text| {
                e.vcall(base, 0x134, &args![text]);
            }),
            CHUNK_MOD3 => model_from_text(e, base + 0x124),
            CHUNK_MOD2 => model_from_text(e, base + 0x104),
            CHUNK_ICO2 => {
                let texture = base
                    .wrapping_add(second_icon.wrapping_mul(0xc))
                    .wrapping_add(0x164);
                e.call(LOAD_TEXTURE, &args![texture, file]);
                second_icon = second_icon.wrapping_add(1);
            }
            CHUNK_MOD4 => model_from_text(e, base + 0x144),
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x1e8, 0x38u32]);
                if e.call(FILE_SWAPS_ENDIAN, &args![file]).bool() {
                    e.call(0x004f_aaa0, &args![base + 0x1e8]);
                }
            }
            CHUNK_FULL => {
                let full = if this.is_null() { 0 } else { base + 0x18 };
                e.call(LOAD_FULL_NAME, &args![full, file]);
            }
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_MODL => {
                let component = base.wrapping_add(model << 5).wrapping_add(0x24);
                model_from_text(e, component);
                model = model.wrapping_add(1);
            }
            CHUNK_ICON => {
                let texture = base
                    .wrapping_add(icon.wrapping_mul(0xc))
                    .wrapping_add(0x164);
                e.call(LOAD_TEXTURE, &args![texture, file]);
                icon = icon.wrapping_add(1);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// The swap helper `CASINO_DATA::Endian` calls on each word (`00401080`,
/// `cdecl (word address, flag)`; given 0 here).
const SWAP_WORD: u32 = 0x0040_1080;
/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`),
/// `TESForm::__SaveData` (`00485e20`, `thiscall (form, data, size)`) and
/// `TESForm::AddChunkArray` (`004856f0`, `cdecl (chunk, data, size)`).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
const FORM_ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// The byte `00401500` answers (`011c54ba`): whether saving swaps bytes.
const SAVE_SWAPS: u32 = 0x0040_1500;
/// Component savers: `TESFullName::Save` (`00487010`),
/// `TESModelTextureSwap::Save` (`0048a520`, three chunk words),
/// `TESTexture::Save` (`0048e370`, one chunk word), `BGSMessageIcon::Save`
/// (`0047a240`, one chunk word), `BGSDestructibleObjectForm::Save`
/// (`00477f20`) and `BGSPickupPutdownSounds::Save` (`0047b560`).
const SAVE_FULL_NAME: u32 = 0x0048_7010;
const SAVE_MODEL_TEXTURE_SWAP: u32 = 0x0048_a520;
const SAVE_TEXTURE: u32 = 0x0048_e370;
const SAVE_MESSAGE_ICON: u32 = 0x0047_a240;
const SAVE_DESTRUCTIBLE: u32 = 0x0047_7f20;
const SAVE_PICKUP_PUTDOWN_SOUNDS: u32 = 0x0047_b560;
/// A string's length (`004048e0`) and text (`00559450`), `BSStringT::Set`
/// (`004037f0`, `thiscall (string, text, length)`) and the length of the
/// string at +4 of a component (`0048cee0`).
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_TEXT: u32 = 0x0055_9450;
const STRING_SET: u32 = 0x0040_37f0;
const COMPONENT_STRING_LENGTH: u32 = 0x0048_cee0;
/// `__RTDynamicCast` (`00ec43fb`) from `TESForm` (`01183028`) to
/// `TESCasinoChips` (`01189ee4`), the type descriptors' names read at +8.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_CASINO_CHIPS: u32 = 0x0118_9ee4;
/// `TESForm::CopyAllComponents` (`004851b0`), `TESForm::CompareAllComponents`
/// (`00485270`), the form's type (`00401170`), the type setter `004f15a0`
/// (one word) and the string comparison `00501c90` (`thiscall (string,
/// other string, flag)`).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const FORM_TYPE: u32 = 0x0040_1170;
const FORM_SET_TYPE: u32 = 0x004f_15a0;
const STRING_COMPARE: u32 = 0x0050_1c90;
/// `TESForm::LoadGame` (`00484c50`, two words), `0048ea10` (two words, on
/// the component at +0x78), `00484d60` and `0048ea40` (one word each, the
/// same pair).
const FORM_LOAD_GAME: u32 = 0x0048_4c50;
const VALUE_LOAD_GAME: u32 = 0x0048_ea10;
const FORM_00484D60: u32 = 0x0048_4d60;
const VALUE_0048EA40: u32 = 0x0048_ea40;

// Translated from 004faaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CASINO_DATA::Endian` (Xbox PDB): swaps each of the 13 words of the
/// casino data (`00401080` with flag 0 on `this + 4 * n`).
pub fn casino_data_endian(e: &mut Engine, this: Ptr) {
    for index in 0..13u32 {
        e.call(SWAP_WORD, &args![this.addr() + 4 * index, 0u32]);
    }
}

// Translated from 00504280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCasinoChips::Save` (Xbox PDB): starts the form, saves the full name
/// (+0x30), the model (+0x3C: `MODL`, `MODT`, `MODS`), the icon (+0x5C,
/// `ICON`), the message icon (+0x68, `MICO`), the destructible data (+0x80)
/// and the pick-up/put-down sounds (+0x88); with a non-empty string at
/// +0x94 adds `ONAM` holding it and its terminating zero; closes the form.
pub fn tes_casino_chips_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![base + 0x30]);
    e.call(
        SAVE_MODEL_TEXTURE_SWAP,
        &args![base + 0x3c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    e.call(SAVE_TEXTURE, &args![base + 0x5c, CHUNK_ICON]);
    e.call(SAVE_MESSAGE_ICON, &args![base + 0x68, CHUNK_MICO]);
    e.call(SAVE_DESTRUCTIBLE, &args![base + 0x80]);
    e.call(SAVE_PICKUP_PUTDOWN_SOUNDS, &args![base + 0x88]);
    if e.call(STRING_LENGTH, &args![base + 0x94]).u32() != 0 {
        let size = e
            .call(STRING_LENGTH, &args![base + 0x94])
            .u32()
            .wrapping_add(1);
        let text = e.call(STRING_TEXT, &args![base + 0x94]).u32();
        e.call(FORM_ADD_CHUNK_ARRAY, &args![CHUNK_ONAM, text, size]);
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// `other` cast from `TESForm` to `TESCasinoChips`.
fn cast_casino_chips(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_FORM, TYPE_CASINO_CHIPS, 0u32],
    )
    .u32()
}

// Translated from 00504340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the copy of a
/// `TESCasinoChips`): when `other` is one, copies all components, takes its
/// form type (`00401170` into `004f15a0`) and sets the string at +0x94 to
/// its text (`BSStringT::Set(text, 0)`).
pub fn fn_00504340(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = cast_casino_chips(e, other);
    if source == 0 {
        return;
    }
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    let form_type = e.call(FORM_TYPE, &args![source]).u32();
    e.call(FORM_SET_TYPE, &args![this, form_type]);
    let text = e.call(STRING_TEXT, &args![source + 0x94]).u32();
    e.call(STRING_SET, &args![this.addr() + 0x94, text, 0u32]);
}

// Translated from 005043b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the comparison of a
/// `TESCasinoChips`): true (different) when `other` is none, when
/// `CompareAllComponents` says so, or when `00501c90` (with flag 1) finds
/// the strings at +0x94 differ.
pub fn fn_005043b0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = cast_casino_chips(e, other);
    if source == 0 {
        return true;
    }
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, source])
        .u8()
        != 0
    {
        return true;
    }
    e.call(
        STRING_COMPARE,
        &args![this.addr() + 0x94, source + 0x94, 1u32],
    )
    .u32()
        != 0
}

// Translated from 00504420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): `TESForm::LoadGame`
/// on the form, then `0048ea10` on the component at +0x78, both with the
/// two words.
pub fn fn_00504420(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(FORM_LOAD_GAME, &args![this, first, second]);
    e.call(VALUE_LOAD_GAME, &args![this.addr() + 0x78, first, second]);
}

// Translated from 00504450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): `00484d60` on the
/// form, then `0048ea40` on the component at +0x78, both with the word.
pub fn fn_00504450(e: &mut Engine, this: Ptr, value: u32) {
    e.call(FORM_00484D60, &args![this, value]);
    e.call(VALUE_0048EA40, &args![this.addr() + 0x78, value]);
}

// Translated from 00504640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; on a `TESCasino`): clears the
/// 0x38-byte data at +0x1E8 (`memset`), then calls virtual slot 0 of each
/// of the ten 0x20-byte models at +0x24 and of the eleven 0xC-byte textures
/// at +0x164.
pub fn fn_00504640(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(MEMSET, &args![base + 0x1e8, 0u32, 0x38u32]);
    for index in 0..10u32 {
        e.vcall(base + 0x24 + index * 0x20, 0, &args![]);
    }
    for index in 0..11u32 {
        e.vcall(base + 0x164 + index * 0xc, 0, &args![]);
    }
}

// Translated from 00504710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESCasino::Save` (Xbox PDB): starts the form, saves the full name
/// (+0x18), the 0x38-byte data at +0x1E8 (`__SaveData`; swapped by
/// `CASINO_DATA::Endian` before and after when `00401500` answers non-zero),
/// the first eight models at +0x24 that have a path (`0048cee0`; `MODL`,
/// `MODT`, `MODS`), the models at +0x104, +0x124 and +0x144 the same way
/// as `MOD2`, `MOD3` and `MOD4`, the textures 0..7 at +0x164 as `ICON` and
/// 7..11 as `ICO2`; closes the form.
pub fn tes_casino_save(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(SAVE_FULL_NAME, &args![base + 0x18]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        casino_data_endian(e, Ptr::new(base + 0x1e8));
    }
    e.call(FORM_SAVE_DATA, &args![this, base + 0x1e8, 0x38u32]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        casino_data_endian(e, Ptr::new(base + 0x1e8));
    }
    for index in 0..8u32 {
        let model = base + 0x24 + index * 0x20;
        if e.call(COMPONENT_STRING_LENGTH, &args![model]).u32() != 0 {
            e.call(
                SAVE_MODEL_TEXTURE_SWAP,
                &args![model, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
            );
        }
    }
    for (offset, chunk) in [
        (0x104u32, CHUNK_MOD2),
        (0x124, CHUNK_MOD3),
        (0x144, CHUNK_MOD4),
    ] {
        if e.call(COMPONENT_STRING_LENGTH, &args![base + offset]).u32() != 0 {
            e.call(
                SAVE_MODEL_TEXTURE_SWAP,
                &args![base + offset, chunk, CHUNK_MODT, CHUNK_MODS],
            );
        }
    }
    for index in 0..7u32 {
        e.call(SAVE_TEXTURE, &args![base + 0x164 + index * 0xc, CHUNK_ICON]);
    }
    for index in 7..11u32 {
        e.call(SAVE_TEXTURE, &args![base + 0x164 + index * 0xc, CHUNK_ICO2]);
    }
    e.call(FORM_CLOSE, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_4480, fn_00504480(Ptr) -> Ptr),
        entry!(0x0050_4540, fn_00504540(Ptr, u32) -> Ptr),
        entry!(0x0050_3e90, tes_casino_chips_init_item(Ptr)),
        entry!(0x0050_3ef0, fn_00503ef0(Ptr, Ptr) -> bool),
        entry!(0x0050_4570, fn_00504570(Ptr)),
        entry!(0x0050_4620, fn_00504620(Ptr)),
        entry!(0x0050_46e0, fn_005046e0(Ptr)),
        entry!(0x0050_48d0, tes_casino_load(Ptr, Ptr) -> bool),
        entry!(0x004f_aaa0, casino_data_endian(Ptr)),
        entry!(0x0050_4280, tes_casino_chips_save(Ptr)),
        entry!(0x0050_4340, fn_00504340(Ptr, Ptr)),
        entry!(0x0050_43b0, fn_005043b0(Ptr, Ptr) -> bool),
        entry!(0x0050_4420, fn_00504420(Ptr, u32, u32)),
        entry!(0x0050_4450, fn_00504450(Ptr, u32)),
        entry!(0x0050_4640, fn_00504640(Ptr)),
        entry!(0x0050_4710, tes_casino_save(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00504480_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_4480,
            size: 0x220,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x24),
                        A::V(0x20),
                        A::V(0xa),
                        A::V(0x0048_a3d0),
                        A::V(0x0048_a490),
                    ],
                ),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x164),
                        A::V(0xc),
                        A::V(0xb),
                        A::V(0x0048_e270),
                        A::V(0x0048_e2e0),
                    ],
                ),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6d)]),
            ],
            stores: &[(0x0, 4, 0x0102_6574), (0x18, 4, 0x0102_6560)],
        });
    }
    #[test]
    fn at_00504540_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_4570, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_4540, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_4540)
                .collect();
            let mut expected = vec![(0x0050_4570, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod chips_and_casino_tests {
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
    fn chips_init_item_initializes_the_components() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.register(FORM_INIT_SKIP, |_, _| ret(0));
        quiet(
            &mut e,
            &[0x0047_8be0, 0x0048_aa80, 0x0047_b6a0, FORM_SET_FLAG],
        );
        e.call_log = Some(vec![]);
        e.call(0x0050_3e90, &args![this]);
        assert_eq!(calls(&e, 0x0047_8be0), vec![vec![this + 0x80, this]]);
        assert_eq!(calls(&e, 0x0048_aa80), vec![vec![this + 0x3c, this]]);
        assert_eq!(calls(&e, 0x0047_b6a0), vec![vec![this + 0x88, this]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 1]]);
        e.register(FORM_INIT_SKIP, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0050_3e90, &args![this]);
        assert!(calls(&e, FORM_SET_FLAG).is_empty());
    }

    /// A form whose own vtable (+0xE0, +0x134) and a shared component
    /// vtable (+0x18) are doubles, read from `chunks`.
    fn load(
        address: u32,
        form_type: u32,
        chunks: Vec<(u32, u32)>,
        swap: bool,
    ) -> (Engine, u32, u32, bool) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x230);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let component = e.mem.alloc(0x20);
        e.mem.set_u32(component + 0x18, 0x7100_0018);
        for model in 0..10 {
            e.mem.set_u32(this + 0x24 + 0x20 * model, component);
        }
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
        e.register(FILE_VERSION, |_, _| ret(3));
        if swap {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(1));
        } else {
            e.register(FILE_SWAPS_ENDIAN, |_, _| ret(0));
        }
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                0x7100_0018,
                FILE_CHUNK_DATA_SIZED,
                0x0048_5110,
                FORM_SET_FLAG,
                LOAD_MODEL,
                LOAD_MODEL_TEXTURE,
                LOAD_TEXTURE,
                LOAD_FULL_NAME,
                LOAD_MESSAGE_ICON,
                LOAD_DESTRUCTIBLE,
                0x0048_a7b0,
                0x006e_cd40,
                0x0040_3550,
                0x0040_37f0,
                0x0048_61f0,
                0x004f_aaa0,
            ],
        );
        e.call_log = Some(vec![]);
        let result = e.call(address, &args![this, file]).bool();
        (e, this, file, result)
    }

    #[test]
    fn chips_load_sends_each_chunk_to_its_component() {
        let chunks = vec![
            (CHUNK_EDID, 0),
            (CHUNK_OBND, 0),
            (CHUNK_FULL, 0),
            (CHUNK_MODL, 0),
            (CHUNK_MODT, 0),
            (CHUNK_MODD, 0),
            (CHUNK_MOSD, 0),
            (CHUNK_MODS, 0),
            (CHUNK_ICON, 0),
            (CHUNK_MICO, 0),
            (CHUNK_DEST, 0),
            (CHUNK_DSTD, 0),
            (CHUNK_YNAM, 0x22),
            (CHUNK_ZNAM, 0x33),
            (CHUNK_ONAM, 0),
        ];
        let (e, this, file, result) = load(0x0050_3ef0, 0x6c, chunks, false);
        assert!(result);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, FORM_SET_FLAG), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, LOAD_FULL_NAME), vec![vec![this + 0x30, file]]);
        assert_eq!(calls(&e, LOAD_MODEL), vec![vec![this + 0x3c, file]; 4]);
        let swaps = calls(&e, 0x0048_a7b0);
        assert_eq!((swaps[0][0], swaps[0][2], swaps[0][3]), (this + 0x3c, 8, 3));
        assert_eq!(calls(&e, LOAD_TEXTURE), vec![vec![this + 0x5c, file]]);
        assert_eq!(calls(&e, LOAD_MESSAGE_ICON), vec![vec![this + 0x68, file]]);
        assert_eq!(
            calls(&e, LOAD_DESTRUCTIBLE),
            vec![vec![this + 0x80, file]; 2]
        );
        assert_eq!(calls(&e, 0x006e_cd40), vec![vec![this + 0x88, 0x22]]);
        assert_eq!(calls(&e, 0x0040_3550), vec![vec![this + 0x88, 0x33]]);
        let text = calls(&e, 0x0040_37f0);
        assert_eq!((text[0][0], text[0][2]), (this + 0x94, 0));
        let sized = calls(&e, FILE_CHUNK_DATA_SIZED);
        assert_eq!(sized[0][2], 0x200);
        assert_eq!(sized[2][2], 0);

        let (e, _, _, result) = load(0x0050_3ef0, 0x6d, vec![(CHUNK_FULL, 0)], false);
        assert!(!result);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }

    #[test]
    fn casino_load_fills_the_models_and_textures_in_turn() {
        let chunks = vec![
            (CHUNK_EDID, 0),
            (CHUNK_OBND, 0),
            (CHUNK_FULL, 0),
            (CHUNK_MODL, 0),
            (CHUNK_MODL, 0),
            (CHUNK_MOD2, 0),
            (CHUNK_MOD3, 0),
            (CHUNK_MOD4, 0),
            (CHUNK_ICON, 0),
            (CHUNK_ICON, 0),
            (CHUNK_ICO2, 0),
            (CHUNK_DATA, 0),
        ];
        let (e, this, file, result) = load(0x0050_48d0, 0x6d, chunks, true);
        assert!(result);
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, LOAD_FULL_NAME), vec![vec![this + 0x18, file]]);
        let paths: Vec<u32> = calls(&e, 0x7100_0018).iter().map(|c| c[0]).collect();
        let models = vec![
            this + 0x24,
            this + 0x44,
            this + 0x104,
            this + 0x124,
            this + 0x144,
        ];
        assert_eq!(paths, models);
        let textures: Vec<u32> = calls(&e, LOAD_MODEL_TEXTURE).iter().map(|c| c[0]).collect();
        assert_eq!(textures, models);
        let icons: Vec<u32> = calls(&e, LOAD_TEXTURE).iter().map(|c| c[0]).collect();
        assert_eq!(
            icons,
            vec![this + 0x164, this + 0x170, this + 0x164 + 7 * 0xc]
        );
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0x1e8, 0x38]]
        );
        assert_eq!(calls(&e, 0x004f_aaa0), vec![vec![this + 0x1e8]]);

        let (e, _, _, result) = load(0x0050_48d0, 0x6c, vec![(CHUNK_DATA, 0)], false);
        assert!(!result);
        assert!(calls(&e, 0x0048_5110).is_empty());
    }

    #[test]
    fn casino_destructor_and_helpers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x230);
        quiet(
            &mut e,
            &[MEMSET, 0x00ec_5fce, 0x0059_1300, 0x0048_3630, 0x0048_5d50],
        );
        e.register(0x0048_4e60, |_, _| ret(0x4400));
        e.call_log = Some(vec![]);
        e.call(0x0050_4570, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_6574);
        assert_eq!(e.mem.u32(this + 0x18), 0x0102_6560);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x1e8, 0, 0x38]]);
        assert_eq!(
            calls(&e, 0x00ec_5fce),
            vec![
                vec![this + 0x164, 0xc, 0xb, 0x0048_e2e0],
                vec![this + 0x24, 0x20, 0xa, 0x0048_a490]
            ]
        );
        assert_eq!(calls(&e, 0x0059_1300), vec![vec![this + 0x18]]);
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![this]]);

        e.call_log = Some(vec![]);
        e.call(0x0050_4620, &args![this]);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x1e8, 0, 0x38]]);

        e.call_log = Some(vec![]);
        e.call(0x0050_46e0, &args![this]);
        assert_eq!(calls(&e, 0x0048_4e60), vec![vec![this, 0xffff_ffff]]);
        assert_eq!(calls(&e, 0x0048_5d50), vec![vec![this + 0x214, 0x4400]]);
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const SLOT_0: u32 = 0x7100_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn endian_swaps_the_13_words() {
        let mut e = Engine::new();
        quiet(&mut e, &[SWAP_WORD]);
        e.call_log = Some(vec![]);
        e.call(0x004f_aaa0, &args![0x1000u32]);
        let swaps = calls(&e, SWAP_WORD);
        assert_eq!(swaps.len(), 13);
        assert_eq!(swaps[0], vec![0x1000, 0]);
        assert_eq!(swaps[12], vec![0x1030, 0]);
    }

    #[test]
    fn chips_save_writes_every_component_and_onam() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FORM_START,
                FORM_CLOSE,
                SAVE_FULL_NAME,
                SAVE_MODEL_TEXTURE_SWAP,
                SAVE_TEXTURE,
                SAVE_MESSAGE_ICON,
                SAVE_DESTRUCTIBLE,
                SAVE_PICKUP_PUTDOWN_SOUNDS,
                FORM_ADD_CHUNK_ARRAY,
            ],
        );
        e.map(0x7300_0000, 4);
        e.register(STRING_LENGTH, |e, _| e.mem.u32(0x7300_0000).into_ret());
        e.register(STRING_TEXT, |_, a| (a[0] + 1).into_ret());
        let this = 0x1000u32;
        e.mem.set_u32(0x7300_0000, 3);
        e.call_log = Some(vec![]);
        e.call(0x0050_4280, &args![this]);
        let log: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(log[1], FORM_START);
        assert_eq!(*log.last().unwrap(), FORM_CLOSE);
        assert_eq!(
            calls(&e, SAVE_MODEL_TEXTURE_SWAP),
            vec![vec![0x103c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]]
        );
        assert_eq!(calls(&e, SAVE_TEXTURE), vec![vec![0x105c, CHUNK_ICON]]);
        assert_eq!(calls(&e, SAVE_MESSAGE_ICON), vec![vec![0x1068, CHUNK_MICO]]);
        assert_eq!(calls(&e, SAVE_DESTRUCTIBLE), vec![vec![0x1080]]);
        assert_eq!(calls(&e, SAVE_PICKUP_PUTDOWN_SOUNDS), vec![vec![0x1088]]);
        assert_eq!(
            calls(&e, FORM_ADD_CHUNK_ARRAY),
            vec![vec![CHUNK_ONAM, 0x1095, 4]]
        );
        e.mem.set_u32(0x7300_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_4280, &args![this]);
        assert!(calls(&e, FORM_ADD_CHUNK_ARRAY).is_empty());
    }

    #[test]
    fn copy_takes_components_type_and_string() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[FORM_COPY_ALL_COMPONENTS, FORM_SET_TYPE, STRING_SET],
        );
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_TYPE, |_, _| 0x6cu32.into_ret());
        e.register(STRING_TEXT, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_4340, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x2000, 0, TYPE_FORM, TYPE_CASINO_CHIPS, 0]]
        );
        assert_eq!(
            calls(&e, FORM_COPY_ALL_COMPONENTS),
            vec![vec![0x1000, 0x2000]]
        );
        assert_eq!(calls(&e, FORM_SET_TYPE), vec![vec![0x1000, 0x6c]]);
        assert_eq!(calls(&e, STRING_SET), vec![vec![0x1094, 0x2095, 0]]);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_4340, &args![0x1000u32, 0x2000u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn compare_branches() {
        let mut e = Engine::new();
        e.map(0x7300_0000, 8);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(0x7300_0000).into_ret()
        });
        e.register(STRING_COMPARE, |e, _| e.mem.u32(0x7300_0004).into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_43b0, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(calls(&e, STRING_COMPARE), vec![vec![0x1094, 0x2094, 1]]);
        e.mem.set_u32(0x7300_0004, 2);
        assert!(e.call(0x0050_43b0, &args![0x1000u32, 0x2000u32]).bool());
        e.mem.set_u32(0x7300_0004, 0);
        e.mem.set_u32(0x7300_0000, 0x100);
        // Only the low byte of CompareAllComponents counts.
        assert!(!e.call(0x0050_43b0, &args![0x1000u32, 0x2000u32]).bool());
        e.mem.set_u32(0x7300_0000, 1);
        assert!(e.call(0x0050_43b0, &args![0x1000u32, 0x2000u32]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.mem.set_u32(0x7300_0000, 0);
        assert!(e.call(0x0050_43b0, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn load_game_pair_and_one_word_pair() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FORM_LOAD_GAME,
                VALUE_LOAD_GAME,
                FORM_00484D60,
                VALUE_0048EA40,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0050_4420, &args![0x1000u32, 5u32, 6u32]);
        e.call(0x0050_4450, &args![0x1000u32, 7u32]);
        assert_eq!(calls(&e, FORM_LOAD_GAME), vec![vec![0x1000, 5, 6]]);
        assert_eq!(calls(&e, VALUE_LOAD_GAME), vec![vec![0x1078, 5, 6]]);
        assert_eq!(calls(&e, FORM_00484D60), vec![vec![0x1000, 7]]);
        assert_eq!(calls(&e, VALUE_0048EA40), vec![vec![0x1078, 7]]);
    }

    #[test]
    fn clear_resets_the_data_and_every_component() {
        let mut e = Engine::new();
        quiet(&mut e, &[MEMSET, SLOT_0]);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, SLOT_0);
        let this = e.mem.alloc(0x220);
        for index in 0..10 {
            e.mem.set_u32(this + 0x24 + index * 0x20, vtable);
        }
        for index in 0..11 {
            e.mem.set_u32(this + 0x164 + index * 0xc, vtable);
        }
        e.call_log = Some(vec![]);
        e.call(0x0050_4640, &args![this]);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x1e8, 0, 0x38]]);
        let slots = calls(&e, SLOT_0);
        assert_eq!(slots.len(), 21);
        assert_eq!(slots[9], vec![this + 0x24 + 9 * 0x20]);
        assert_eq!(slots[20], vec![this + 0x164 + 10 * 0xc]);
    }

    #[test]
    fn casino_save_writes_models_textures_and_swapped_data() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FORM_START,
                FORM_CLOSE,
                SAVE_FULL_NAME,
                SAVE_MODEL_TEXTURE_SWAP,
                SAVE_TEXTURE,
                FORM_SAVE_DATA,
                SWAP_WORD,
            ],
        );
        e.map(0x7300_0000, 4);
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(0x7300_0000).into_ret());
        // Models with a path: index 1 and the one at +0x124.
        e.register(COMPONENT_STRING_LENGTH, |_, a| {
            u32::from(a[0] == 0x1044 || a[0] == 0x1124).into_ret()
        });
        e.mem.set_u32(0x7300_0000, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_4710, &args![0x1000u32]);
        assert_eq!(calls(&e, SAVE_FULL_NAME), vec![vec![0x1018]]);
        assert_eq!(calls(&e, FORM_SAVE_DATA), vec![vec![0x1000, 0x11e8, 0x38]]);
        assert_eq!(calls(&e, SWAP_WORD).len(), 26);
        assert_eq!(calls(&e, COMPONENT_STRING_LENGTH).len(), 11);
        assert_eq!(
            calls(&e, SAVE_MODEL_TEXTURE_SWAP),
            vec![
                vec![0x1044, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
                vec![0x1124, CHUNK_MOD3, CHUNK_MODT, CHUNK_MODS]
            ]
        );
        let textures = calls(&e, SAVE_TEXTURE);
        assert_eq!(textures.len(), 11);
        assert_eq!(textures[6], vec![0x1164 + 6 * 0xc, CHUNK_ICON]);
        assert_eq!(textures[7], vec![0x1164 + 7 * 0xc, CHUNK_ICO2]);
        assert_eq!(calls(&e, FORM_CLOSE), vec![vec![0x1000]]);
        // No swapping.
        e.mem.set_u32(0x7300_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_4710, &args![0x1000u32]);
        assert!(calls(&e, SWAP_WORD).is_empty());
    }
}
