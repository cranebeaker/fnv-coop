//! `fallout shared/teseffectshader.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00505c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505c40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4f (0x170 bytes); `ECX` is the object.
/// The engine map names it `TESEffectShader::TESEffectShader` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_685c`) is `TESEffectShader`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00505c40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_685c);
    e.call(0x0050_64d0, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x14c]);
    e.call(0x0048_e270, &args![base + 0x158]);
    e.call(0x0048_e270, &args![base + 0x164]);
    e.call(0x004f_15a0, &args![this, 0x4fu32]);
    e.call(0x0050_5db0, &args![this]);
    this
}

/// `006815c0` (`ECX`, answers `this`).
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `00500940` (`ECX`): answers `this + 0x18` (here the shader data).
const DATA_00500940: u32 = 0x0050_0940;
/// The `NiPoint3` constructor (`00416870`, `ECX`, three float words), answers `this`.
const POINT3_CTOR: u32 = 0x0041_6870;

/// One store of `00506510`, in the exe's order.
enum Store {
    /// A byte constant.
    Byte(u32, u8),
    /// A word constant.
    Word(u32, u32),
    /// `FLD1`/`FSTP float`.
    One(u32),
    /// `FLDZ`/`FSTP float`.
    Zero(u32),
    /// `FLD float [global]`/`FSTP float` (a signalling NaN comes out quiet).
    Global(u32, u32),
    /// A stack `NiPoint3` made by `00416870` from three zeros, its three words copied.
    Point(u32),
}

/// The stores of `00506510`.
const EFFECT_SHADER_DEFAULTS: [Store; 72] = [
    Store::Byte(0x0, 0),
    Store::Word(0x4, 5),
    Store::Word(0x8, 1),
    Store::Word(0xc, 3),
    Store::Word(0x10, 0),
    Store::Zero(0x14),
    Store::Zero(0x18),
    Store::Zero(0x1c),
    Store::Zero(0x20),
    Store::Zero(0x24),
    Store::One(0x28),
    Store::Zero(0x2c),
    Store::Zero(0x30),
    Store::One(0x34),
    Store::Word(0x38, 0),
    Store::Zero(0x3c),
    Store::Zero(0x40),
    Store::Zero(0x44),
    Store::Zero(0x48),
    Store::Zero(0x4c),
    Store::One(0x50),
    Store::Word(0x5c, 6),
    Store::One(0x54),
    Store::One(0x58),
    Store::Word(0x60, 5),
    Store::Word(0x6c, 6),
    Store::Word(0x64, 1),
    Store::Word(0x68, 4),
    Store::Zero(0x70),
    Store::Zero(0x74),
    Store::Zero(0x78),
    Store::One(0x7c),
    Store::One(0x80),
    Store::One(0x84),
    Store::Zero(0x88),
    Store::Zero(0x8c),
    Store::Zero(0x90),
    Store::Point(0x94),
    Store::Point(0xa0),
    Store::One(0xac),
    Store::One(0xb0),
    Store::Zero(0xb4),
    Store::One(0xb8),
    Store::Word(0xbc, 0x00ff_ffff),
    Store::Word(0xc0, 0x00ff_ffff),
    Store::Word(0xc4, 0x00ff_ffff),
    Store::One(0xc8),
    Store::One(0xcc),
    Store::One(0xd0),
    Store::Zero(0xd4),
    Store::Global(0xd8, 0x0101_6248),
    Store::One(0xdc),
    Store::Zero(0xe0),
    Store::Zero(0xe4),
    Store::Zero(0xe8),
    Store::Zero(0xec),
    Store::Zero(0xf0),
    Store::Zero(0xf8),
    Store::Global(0xfc, 0x0101_7b78),
    Store::Global(0x100, 0x0102_3cd8),
    Store::Zero(0x104),
    Store::Zero(0x108),
    Store::Word(0x10c, 0x00ff_ffff),
    Store::Zero(0x110),
    Store::Word(0x114, 1),
    Store::Word(0x118, 1),
    Store::One(0x11c),
    Store::One(0x120),
    Store::One(0x124),
    Store::One(0x128),
    Store::One(0x12c),
    Store::One(0x130),
];

// Translated from 00506510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `ECX` = the effect shader data, the
/// `TESEffectShader`'s +0x18): its default values, written in the exe's order: the flag byte
/// cleared, the blend and test modes (5, 1, 3, ...), the fill, edge and particle floats
/// (0.0, 1.0, or a copy of a float global), the two `NiPoint3`s at +0x94 and +0xa0 (zeros,
/// through a stack `NiPoint3` made by `00416870`), the white colour keys and the counts.
pub fn fn_00506510(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for store in EFFECT_SHADER_DEFAULTS.iter() {
        match store {
            Store::Byte(offset, value) => e.mem.set_u8(base + offset, *value),
            Store::Word(offset, value) => e.mem.set_u32(base + offset, *value),
            Store::One(offset) => e.mem.set_u32(base + offset, x87_store32(1.0)),
            Store::Zero(offset) => e.mem.set_u32(base + offset, x87_store32(0.0)),
            Store::Global(offset, global) => {
                let bits = x87_quiet_bits(e.global::<u32>(*global));
                e.mem.set_u32(base + offset, bits);
            }
            Store::Point(offset) => {
                let zero = x87_store32(0.0);
                e.with_stack(0xc, |e, point| {
                    let made = e.call(POINT3_CTOR, &args![point, zero, zero, zero]).u32();
                    for word in 0..3u32 {
                        let value = e.mem.u32(made + word * 4);
                        e.mem.set_u32(base + offset + word * 4, value);
                    }
                });
            }
        }
    }
}

// Translated from 005064d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map: the effect shader data constructor (`ECX`, no stack words; the
/// `TESEffectShader` constructor runs it on its +0x18): `006815c0` on the two `NiPoint3`s
/// (+0x94, +0xa0), then the defaults ([`fn_00506510`]); answers `this`.
pub fn fn_005064d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(IDENTITY_006815C0, &args![this.addr() + 0x94]);
    e.call(IDENTITY_006815C0, &args![this.addr() + 0xa0]);
    fn_00506510(e, this);
    this
}

// Translated from 00505db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEffectShader`'s vtable (`0102685c`) slot +0x14, the `InitializeData` slot of the Xbox
/// PDB's `TESForm` (`ECX`): the defaults ([`fn_00506510`]) on the shader data `00500940`
/// answers.
pub fn fn_00505db0(e: &mut Engine, this: Ptr) {
    let data = e.call(DATA_00500940, &args![this]).u32();
    fn_00506510(e, Ptr::new(data));
}

// Translated from 00506390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`ECX`): answers `this + 0x158` (the `TESEffectShader`'s second texture,
/// the one `Load` fills from `ICO2`).
pub fn fn_00506390(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x158)
}

// Translated from 00507b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many classes (a folded getter): the `float` at +0x4c of
/// `this` (`ECX`), returned in `ST0` (`FLD dword`, so a signalling NaN comes out quiet).
pub fn fn_00507b00(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x4c)))
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
/// `TESTexture::LoadTextureChunk` (Xbox PDB, `0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `00506850` (`ECX` = the shader data): swaps its byte order.
const SWAP_DATA_00506850: u32 = 0x0050_6850;
/// `TESForm::LoadData` (Xbox PDB, `004861f0`, `ECX` = form, three stack words: file,
/// destination, start).
const LOAD_DATA: u32 = 0x0048_61f0;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;
/// `00ba52d0` (no words): a count kept at `012003ec` (0x1a, or 0x4e below a setting of 2).
const COUNT_00BA52D0: u32 = 0x00ba_52d0;
/// The doubles 1.0 (`01012070`) and 0.0 (`01012060`) the two scales are compared with.
const DOUBLE_ONE: u32 = 0x0101_2070;
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// Chunk codes `TESEffectShader::Load` reads (the four characters, first byte lowest).
const CHUNK_EDID: i32 = 0x4449_4445;
const CHUNK_ICO2: i32 = 0x324f_4349;
const CHUNK_NAM7: i32 = 0x374d_414e;
const CHUNK_DATA: i32 = 0x4154_4144;
const CHUNK_OBND: i32 = 0x444e_424f;
const CHUNK_ICON: i32 = 0x4e4f_4349;
/// The effect shader record's form type.
const FORM_TYPE_EFFECT_SHADER: u8 = 0x4f;

/// A scale float of the data at `at`: when it is in (0, 1] (`FCOMP` against the doubles,
/// NaN failing both tests) it becomes `00ba52d0`'s count (`FILD qword` of the zero-extended
/// word) times itself, stored as a float.
fn scale_by_count(e: &mut Engine, at: u32) {
    let value = x87_m32(e.mem.u32(at));
    if !(value <= e.global::<f64>(DOUBLE_ONE)) {
        return;
    }
    let value = x87_m32(e.mem.u32(at));
    if !(value > e.global::<f64>(DOUBLE_ZERO)) {
        return;
    }
    let count = e.call(COUNT_00BA52D0, &args![]).u32();
    let product = x87_mul(f64::from(count), x87_m32(e.mem.u32(at)));
    e.mem.set_u32(at, x87_store32(product));
}

// Translated from 00505e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505e70 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `TESEffectShader::Load` (Xbox PDB), `ECX` and one stack word, the file (`RET 4`). A
/// record that is not an effect shader (`TESFile::GetTESForm` not 0x4f) answers false.
/// Otherwise vtable slot +0x18, `TESForm::LoadForm`, form flag 8 cleared (`00484ab0`), then
/// each chunk (codes compared signed) until `GetTESChunk` answers 0 or `004726f0` false:
/// `EDID` (a stack buffer of the chunk's size, read with at most 0x200 bytes, given to vtable
/// slot +0x134), `OBND` (vtable slot +0xe0 with the file), `ICON`, `ICO2`, `NAM7` (the
/// textures at +0x14c, +0x158, +0x164), `DATA` (the shader data at +0x18 through
/// `TESForm::LoadData` with 0x134 bytes, byte-swapped by `00506850` before and after when the
/// file needs it). Then the two floats at +0x94 and +0x98 are scaled by `00ba52d0`'s count
/// when they are in (0, 1], and true.
pub fn tes_effect_shader_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let form_type = e.call(GET_TES_FORM, &args![file]).u32() as u8;
    if form_type != FORM_TYPE_EFFECT_SHADER {
        return false;
    }
    let base = this.addr();
    e.vcall(base, 0x18, &args![]);
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let code = chunk as i32;
        if code > CHUNK_EDID {
            if code == CHUNK_OBND {
                e.vcall(base, 0xe0, &args![file]);
            } else if code == CHUNK_ICON {
                e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x14c, file]);
            }
        } else if code == CHUNK_EDID {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, name| {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                e.vcall(base, 0x134, &args![name]);
            });
        } else if code == CHUNK_ICO2 {
            e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x158, file]);
        } else if code == CHUNK_NAM7 {
            e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x164, file]);
        } else if code == CHUNK_DATA {
            if e.call(NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_DATA_00506850, &args![base + 0x18]);
            }
            e.call(LOAD_DATA, &args![this, file, base + 0x18, 0x134u32]);
            if e.call(NEEDS_SWAP, &args![file]).bool() {
                e.call(SWAP_DATA_00506850, &args![base + 0x18]);
            }
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    scale_by_count(e, base + 0x94);
    scale_by_count(e, base + 0x98);
    true
}

// Translated from 00507b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map, called from many classes (a folded getter): the `float` at +0x118
/// of `this` (`ECX`), returned in `ST0` (`FLD dword`, so a signalling NaN comes out quiet).
pub fn fn_00507b20(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x118)))
}

/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`),
/// `TESForm::__SaveData` (`00485e20`, `thiscall (form, data, size)`) and
/// `TESForm::AddChunkArray` (`004856f0`, `cdecl (chunk, data, size)`).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const FORM_SAVE_DATA: u32 = 0x0048_5e20;
const FORM_ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// A texture's path length (`0048cee0`) and text (`00408da0`), and
/// `TESTexture::Save` (`0048e370`, one chunk word).
const TEXTURE_PATH_LENGTH: u32 = 0x0048_cee0;
const TEXTURE_PATH_TEXT: u32 = 0x0040_8da0;
const TEXTURE_SAVE: u32 = 0x0048_e370;
/// The getter of the form id at +0xC (`0084e3a0`, `thiscall`).
const FORM_ID: u32 = 0x0084_e3a0;
/// The byte `00401500` answers (`011c54ba`): whether saving swaps bytes.
const SAVE_SWAPS: u32 = 0x0040_1500;
/// The word swap helper (`00401080`, `cdecl (word address, flag)`).
const SWAP_WORD: u32 = 0x0040_1080;
/// `__RTDynamicCast` (`00ec43fb`) from `TESForm` (`01183028`) to
/// `TESEffectShader` (`01183918`), the type descriptors' names read at +8.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_FORM: u32 = 0x0118_3028;
const TYPE_EFFECT_SHADER: u32 = 0x0118_3918;
/// `TESForm::CopyAllComponents` (`004851b0`), `TESForm::CompareAllComponents`
/// (`00485270`), the copy `00401460` (`cdecl (to, from, size)`) and
/// `memcmp` (`00ec4835`, `cdecl`).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// The chunks of the three textures.
const CHUNK_ICON_WORD: u32 = 0x4e4f_4349;
const CHUNK_ICO2_WORD: u32 = 0x324f_4349;
const CHUNK_NAM7_WORD: u32 = 0x374d_414e;
/// The size of the shader data at +0x18.
const SHADER_DATA_SIZE: u32 = 0x134;
/// The float 255.0 (`01023cd8`) the colour bytes are divided by.
const FLOAT_255: u32 = 0x0102_3cd8;
/// The double -1.0 (`0101a6b0`).
const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;
/// The `NiColorA` constructor (`00414430`, `thiscall (colour, r, g, b,
/// a)`, answers the colour).
const COLOR_CTOR: u32 = 0x0041_4430;
/// The setter of a reference-counted pointer (`0066b0d0`, `thiscall`, one
/// word).
const SET_POINTER: u32 = 0x0066_b0d0;
/// The particle shader property setters `SetupParticleShader` calls that
/// are not in this unit (`thiscall`, one word each), by address.
const TARGET_00584150: u32 = 0x0058_4150;
const TARGET_0051B8C0: u32 = 0x0051_b8c0;
const TARGET_00810550: u32 = 0x0081_0550;
const TARGET_006E29D0: u32 = 0x006e_29d0;
const TARGET_005075F0: u32 = 0x0050_75f0;
const TARGET_009314D0: u32 = 0x0093_14d0;
const TARGET_00507610: u32 = 0x0050_7610;
const TARGET_006E5EE0: u32 = 0x006e_5ee0;
const TARGET_00507780: u32 = 0x0050_7780;
const TARGET_005F9B10: u32 = 0x005f_9b10;
const TARGET_005077A0: u32 = 0x0050_77a0;
const TARGET_006E4EF0: u32 = 0x006e_4ef0;
const TARGET_006E4320: u32 = 0x006e_4320;
const TARGET_005076C0: u32 = 0x0050_76c0;
const TARGET_00733FD0: u32 = 0x0073_3fd0;
const TARGET_00507700: u32 = 0x0050_7700;
const TARGET_006D30E0: u32 = 0x006d_30e0;
const TARGET_00505E50: u32 = 0x0050_5e50;
/// `ParticleShader::Start` (`00ba68d0`, `cdecl`, two words),
/// `ParticleShader::Stop` (`00ba6290`, `cdecl`, two words) and
/// `ParticleShaderProperty::InitParticleData` (`00ba6e00`, `thiscall`, one
/// float word).
const PARTICLE_SHADER_START: u32 = 0x00ba_68d0;
const PARTICLE_SHADER_STOP: u32 = 0x00ba_6290;
const PARTICLE_INIT_DATA: u32 = 0x00ba_6e00;
/// The allocation `00507830` makes (`00aa13e0`, `cdecl (size)`) and the
/// constructor it runs on it (`00b668b0`, `thiscall`).
const ALLOCATE: u32 = 0x00aa_13e0;
const EFFECT_DATA_CTOR: u32 = 0x00b6_68b0;
/// The getters `00507830` calls on the shader (`thiscall`, no words): two
/// colours (`0045cd60`, `0068a830`), then four words (`00441110`,
/// `004fd3a0`, `007af430`, `0059bb30`).
const SHADER_COLOR_A: u32 = 0x0045_cd60;
const SHADER_COLOR_B: u32 = 0x0068_a830;
const SHADER_GETTER_00441110: u32 = 0x0044_1110;
const SHADER_GETTER_004FD3A0: u32 = 0x004f_d3a0;
const SHADER_GETTER_007AF430: u32 = 0x007a_f430;
const SHADER_GETTER_0059BB30: u32 = 0x0059_bb30;
/// `NiAVObject::GetWorldBound` (`0043d450`) and the float `0084d030`
/// answers of it (`ST0`).
const NODE_WORLD_BOUND: u32 = 0x0043_d450;
const BOUND_RADIUS_0084D030: u32 = 0x0084_d030;
/// `NiAVObject::GetProperty` (`00a59d30`, one word: the property type),
/// the property's type word (`00441110`), `00413f40` and `0043b1b0` (the
/// name the shader property's texture check compares), `00408b20` (`cdecl
/// (text, other)`), `BSShaderPPLightingProperty::SetTextureEffectData`
/// (`00b675c0`, one word), the getter `00507d80`, and the node's children
/// (`0043b480` count, `0043b4a0` one word: the child at an index).
const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_TYPE: u32 = 0x0044_1110;
const PROPERTY_00413F40: u32 = 0x0041_3f40;
const PROPERTY_NAME_0043B1B0: u32 = 0x0043_b1b0;
const TEXT_COMPARE: u32 = 0x0040_8b20;
const SET_TEXTURE_EFFECT_DATA: u32 = 0x00b6_75c0;
const TEXTURE_EFFECT_DATA: u32 = 0x0050_7d80;
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// The text `"skin"` (`01020490`).
const TEXT_SKIN: u32 = 0x0102_0490;

/// The three textures of the shader: offset and chunk.
const TEXTURES: [(u32, u32); 3] = [
    (0x14c, CHUNK_ICON_WORD),
    (0x158, CHUNK_ICO2_WORD),
    (0x164, CHUNK_NAM7_WORD),
];

/// A float word as `FLD float`/`FSTP float` copy it (a signalling NaN comes
/// out quiet).
fn float_copy(e: &Engine, at: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(at))
}

// Translated from 005060f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`; the save of a `TESEffectShader`,
/// the form whose data `tes_effect_shader_load` reads): starts the form;
/// for each texture (+0x14C `ICON`, +0x158 `ICO2`, +0x164 `NAM7`) adds the
/// chunk with its text and terminating zero when its path is empty
/// (`0048cee0` 0), else `TESTexture::Save` with the chunk; turns the form
/// pointer at +0x10C into its form id (0 for none) for the save, saves the
/// 0x134-byte data at +0x18 (`__SaveData`, swapped by `00506850` before and
/// after when `00401500` answers non-zero), puts the pointer back and
/// closes the form.
pub fn fn_005060f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(FORM_START, &args![this]);
    for (offset, chunk) in TEXTURES {
        let texture = base + offset;
        if e.call(TEXTURE_PATH_LENGTH, &args![texture]).u32() == 0 {
            let size = e
                .call(TEXTURE_PATH_LENGTH, &args![texture])
                .u32()
                .wrapping_add(1);
            let text = e.call(TEXTURE_PATH_TEXT, &args![texture]).u32();
            e.call(FORM_ADD_CHUNK_ARRAY, &args![chunk, text, size]);
        } else {
            e.call(TEXTURE_SAVE, &args![texture, chunk]);
        }
    }
    let saved = e.mem.u32(base + 0x10c);
    let id = if e.mem.u32(base + 0x10c) != 0 {
        let form = e.mem.u32(base + 0x10c);
        e.call(FORM_ID, &args![form]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x10c, id);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_00506850(e, Ptr::new(base + 0x18));
    }
    e.call(FORM_SAVE_DATA, &args![this, base + 0x18, SHADER_DATA_SIZE]);
    if e.call(SAVE_SWAPS, &args![]).u8() != 0 {
        fn_00506850(e, Ptr::new(base + 0x18));
    }
    e.mem.set_u32(base + 0x10c, saved);
    e.call(FORM_CLOSE, &args![this]);
}

/// `other` cast from `TESForm` to `TESEffectShader`.
fn cast_effect_shader(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_FORM, TYPE_EFFECT_SHADER, 0u32],
    )
    .u32()
}

// Translated from 005062a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the copy of a
/// `TESEffectShader`): when `other` is one, copies all components, the
/// 0x134-byte shader data (`00401460` between the two `00500940` answers)
/// and each texture through the texture's virtual slot `+8` with `other`'s
/// texture (`00506370`, `fn_00506390`, `005063b0`).
pub fn fn_005062a0(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = cast_effect_shader(e, other);
    if source == 0 {
        return;
    }
    let base = this.addr();
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    let from = e.call(DATA_00500940, &args![source]).u32();
    let to = e.call(DATA_00500940, &args![this]).u32();
    e.call(COPY_BYTES, &args![to, from, SHADER_DATA_SIZE]);
    let texture = fn_00506370(e, Ptr::new(source));
    e.vcall(base + 0x14c, 8, &args![texture]);
    let texture = fn_00506390(e, Ptr::new(source));
    e.vcall(base + 0x158, 8, &args![texture]);
    let texture = fn_005063b0(e, Ptr::new(source));
    e.vcall(base + 0x164, 8, &args![texture]);
}

// Translated from 00506370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): answers `this + 0x14C` (the
/// shader's first texture, `ICON`).
pub fn fn_00506370(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x14c)
}

// Translated from 005063b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): answers `this + 0x164` (the
/// shader's third texture, `NAM7`).
pub fn fn_005063b0(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x164)
}

// Translated from 005063d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the comparison of a
/// `TESEffectShader`): true (different) when `other` is none, when
/// `CompareAllComponents` says so, when a texture's virtual slot `+0xC`
/// (with `other`'s texture) answers non-zero, or when `memcmp` of the
/// 0x134-byte data at +0x18 with `other`'s (`00500940`) is non-zero.
pub fn fn_005063d0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = cast_effect_shader(e, other);
    if source == 0 {
        return true;
    }
    let base = this.addr();
    if e.call(FORM_COMPARE_ALL_COMPONENTS, &args![this, source])
        .u8()
        != 0
    {
        return true;
    }
    let texture = fn_00506370(e, Ptr::new(source));
    if e.vcall(base + 0x14c, 0xc, &args![texture]).u8() != 0 {
        return true;
    }
    let texture = fn_00506390(e, Ptr::new(source));
    if e.vcall(base + 0x158, 0xc, &args![texture]).u8() != 0 {
        return true;
    }
    let texture = fn_005063b0(e, Ptr::new(source));
    if e.vcall(base + 0x164, 0xc, &args![texture]).u8() != 0 {
        return true;
    }
    let data = e.call(DATA_00500940, &args![source]).u32();
    e.call(MEMCMP, &args![base + 0x18, data, SHADER_DATA_SIZE])
        .u32()
        != 0
}

// Translated from 00506850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the shader data): swaps the
/// byte order of the words at +4 to +0x130 (`00401080` with flag 0 on
/// each), all but the word at +0xF4 (the first byte's word at +0 is not
/// swapped either).
pub fn fn_00506850(e: &mut Engine, this: Ptr) {
    let mut offset = 4u32;
    while offset <= 0x130 {
        if offset != 0xf4 {
            e.call(SWAP_WORD, &args![this.addr() + offset, 0u32]);
        }
        offset += 4;
    }
}

// Translated from 00507040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): whether the flag
/// byte at the start of the shader data (`00500940`) has any bit of `mask`.
pub fn fn_00507040(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    let data = e.call(DATA_00500940, &args![this]).u32();
    u32::from(e.mem.u8(data)) & mask != 0
}

/// One colour of `SetupParticleShader` and `00507830`: the `NiColorA`
/// constructor (`00414430`) on `scratch` with the colour word's low,
/// second and third bytes as floats and alpha 0, then [`fn_005074e0`] into
/// `out` with the float at `01023cd8` (255).
fn color_from_bytes(e: &mut Engine, color: u32, scratch: u32, out: u32) -> u32 {
    let divisor = x87_quiet_bits(e.mem.u32(FLOAT_255));
    let red = ((color & 0xff) as f32).to_bits();
    let green = ((((color & 0xffff) >> 8) & 0xff) as f32).to_bits();
    let blue = (((color >> 0x10) & 0xff) as f32).to_bits();
    let made = e
        .call(COLOR_CTOR, &args![scratch, red, green, blue, 0u32])
        .u32();
    fn_005074e0(e, Ptr::new(made), out, divisor)
}

// Translated from 00507060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEffectShader::SetupParticleShader` (Xbox PDB): hands the shader's
/// particle settings to `target`, in the exe's order: `flag`
/// ([`fn_00507550`]); the floats at +0x9C, +0xA0, +0xA4, +0xA8, +0xF8; the
/// two triples at +0xAC and +0xB8; the floats at +0xFC, +0x100, +0x104,
/// +0x108; the words at +0x12C and +0x130 as floats; the float at +0x128;
/// three colours made from the colour words at +0xD4, +0xD8, +0xDC (bytes
/// over 255) with the alphas at +0xE0, +0xE4, +0xE8; the floats at +0xEC,
/// +0xF0, +0xF4, +0xC4, +0xC8, +0xCC, +0xD0; the words at +0x78, +0x84,
/// +0x7C, +0x80. Setters outside this unit are called by address.
pub fn tes_effect_shader_setup_particle_shader(e: &mut Engine, this: Ptr, target: Ptr, flag: u32) {
    let base = this.addr();
    let target = target.addr();
    fn_00507550(e, Ptr::new(target), flag);
    let value = float_copy(e, base + 0x9c);
    e.call(TARGET_00584150, &args![target, value]);
    let value = float_copy(e, base + 0xa0);
    fn_00507570(e, Ptr::new(target), value);
    let value = float_copy(e, base + 0xa4);
    e.call(TARGET_0051B8C0, &args![target, value]);
    let value = float_copy(e, base + 0xa8);
    e.call(TARGET_00810550, &args![target, value]);
    let value = float_copy(e, base + 0xf8);
    e.call(TARGET_006E29D0, &args![target, value]);
    let (x, y, z) = (
        e.mem.u32(base + 0xac),
        e.mem.u32(base + 0xb0),
        e.mem.u32(base + 0xb4),
    );
    fn_00507590(e, Ptr::new(target), x, y, z);
    let (x, y, z) = (
        e.mem.u32(base + 0xb8),
        e.mem.u32(base + 0xbc),
        e.mem.u32(base + 0xc0),
    );
    fn_005075c0(e, Ptr::new(target), x, y, z);
    let value = float_copy(e, base + 0xfc);
    e.call(TARGET_005075F0, &args![target, value]);
    let value = float_copy(e, base + 0x100);
    e.call(TARGET_009314D0, &args![target, value]);
    let value = float_copy(e, base + 0x104);
    e.call(TARGET_00507610, &args![target, value]);
    let value = float_copy(e, base + 0x108);
    e.call(TARGET_006E5EE0, &args![target, value]);
    // FILD qword of the zero-extended word, FSTP float.
    let value = x87_store32(f64::from(e.mem.u32(base + 0x12c)));
    e.call(TARGET_00507780, &args![target, value]);
    let value = x87_store32(f64::from(e.mem.u32(base + 0x130)));
    e.call(TARGET_005F9B10, &args![target, value]);
    let value = float_copy(e, base + 0x128);
    e.call(TARGET_005077A0, &args![target, value]);
    e.with_stack(0x60, |e, locals| {
        // Three NiColorA scratch colours, then the three results.
        let scratch = locals.addr();
        let out = locals.addr() + 0x30;
        for index in 0..3u32 {
            let color = e.mem.u32(base + 0xd4 + 4 * index);
            color_from_bytes(e, color, scratch + 0x10 * index, out + 0x10 * index);
        }
        for index in 0..3u32 {
            let alpha = float_copy(e, base + 0xe0 + 4 * index);
            e.mem.set_u32(out + 0x10 * index + 0xc, alpha);
        }
        let color = |e: &Engine, index: u32| -> [u32; 4] {
            let at = out + 0x10 * index;
            [
                e.mem.u32(at),
                e.mem.u32(at + 4),
                e.mem.u32(at + 8),
                e.mem.u32(at + 0xc),
            ]
        };
        let [r, g, b, a] = color(e, 0);
        fn_00507630(e, Ptr::new(target), r, g, b, a);
        let [r, g, b, a] = color(e, 1);
        fn_00507660(e, Ptr::new(target), r, g, b, a);
        let [r, g, b, a] = color(e, 2);
        fn_00507690(e, Ptr::new(target), r, g, b, a);
    });
    let value = float_copy(e, base + 0xec);
    e.call(TARGET_006E4EF0, &args![target, value]);
    let value = float_copy(e, base + 0xf0);
    e.call(TARGET_006E4320, &args![target, value]);
    let value = float_copy(e, base + 0xf4);
    e.call(TARGET_005076C0, &args![target, value]);
    let value = float_copy(e, base + 0xc4);
    fn_005076e0(e, Ptr::new(target), value);
    let value = float_copy(e, base + 0xc8);
    e.call(TARGET_00733FD0, &args![target, value]);
    let value = float_copy(e, base + 0xcc);
    e.call(TARGET_00507700, &args![target, value]);
    let value = float_copy(e, base + 0xd0);
    fn_00507720(e, Ptr::new(target), value);
    let value = e.mem.u32(base + 0x78);
    e.call(TARGET_006D30E0, &args![target, value]);
    let value = e.mem.u32(base + 0x84);
    e.call(TARGET_00505E50, &args![target, value]);
    let value = e.mem.u32(base + 0x7c);
    fn_00507740(e, Ptr::new(target), value);
    let value = e.mem.u32(base + 0x80);
    fn_00507760(e, Ptr::new(target), value);
}

// Translated from 005074e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall (colour, out, divisor)`): writes
/// the four floats of the colour, each divided by the float `divisor`
/// (`FDIV float`, one store each), to `out`; answers `out`.
pub fn fn_005074e0(e: &mut Engine, this: Ptr, out: u32, divisor: u32) -> u32 {
    let mut parts = [0u32; 4];
    for (index, part) in parts.iter_mut().enumerate() {
        let bits = e.mem.u32(this.addr() + 4 * index as u32);
        *part = x87_store32(x87_div(x87_load32(bits), x87_m32(divisor)));
    }
    for (index, part) in parts.iter().enumerate() {
        e.mem.set_u32(out + 4 * index as u32, *part);
    }
    out
}

// Translated from 00507550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): sets the pointer at
/// +0x118 (`0066b0d0`).
pub fn fn_00507550(e: &mut Engine, this: Ptr, value: u32) {
    e.call(SET_POINTER, &args![this.addr() + 0x118, value]);
}

// Translated from 00507570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float word): stores it at
/// +0x80 (`FLD`/`FSTP float`).
pub fn fn_00507570(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x80, x87_quiet_bits(value));
}

// Translated from 00507590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words, copied as words):
/// stores them at +0x90.
pub fn fn_00507590(e: &mut Engine, this: Ptr, x: u32, y: u32, z: u32) {
    let at = this.addr() + 0x90;
    e.mem.set_u32(at, x);
    e.mem.set_u32(at + 4, y);
    e.mem.set_u32(at + 8, z);
}

// Translated from 005075c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words): stores them at
/// +0x9C.
pub fn fn_005075c0(e: &mut Engine, this: Ptr, x: u32, y: u32, z: u32) {
    let at = this.addr() + 0x9c;
    e.mem.set_u32(at, x);
    e.mem.set_u32(at + 4, y);
    e.mem.set_u32(at + 8, z);
}

/// Four words stored at `at`.
fn store_four(e: &mut Engine, at: u32, words: [u32; 4]) {
    for (index, word) in words.iter().enumerate() {
        e.mem.set_u32(at + 4 * index as u32, *word);
    }
}

// Translated from 00507630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words): stores them at
/// +0xC4.
pub fn fn_00507630(e: &mut Engine, this: Ptr, r: u32, g: u32, b: u32, a: u32) {
    store_four(e, this.addr() + 0xc4, [r, g, b, a]);
}

// Translated from 00507660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words): stores them at
/// +0xD4.
pub fn fn_00507660(e: &mut Engine, this: Ptr, r: u32, g: u32, b: u32, a: u32) {
    store_four(e, this.addr() + 0xd4, [r, g, b, a]);
}

// Translated from 00507690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words): stores them at
/// +0xE4.
pub fn fn_00507690(e: &mut Engine, this: Ptr, r: u32, g: u32, b: u32, a: u32) {
    store_four(e, this.addr() + 0xe4, [r, g, b, a]);
}

// Translated from 005076e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float word): stores it at
/// +0xF4 (`FLD`/`FSTP float`).
pub fn fn_005076e0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xf4, x87_quiet_bits(value));
}

// Translated from 00507720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float word): stores it at
/// +0x100 (`FLD`/`FSTP float`).
pub fn fn_00507720(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x100, x87_quiet_bits(value));
}

// Translated from 00507740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): stores it at +0x110.
pub fn fn_00507740(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x110, value);
}

// Translated from 00507760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): stores it at +0x114.
pub fn fn_00507760(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x114, value);
}

// Translated from 005077c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEffectShader::StartParticleShader` (Xbox PDB, `thiscall`, four
/// words): with a non-zero `flag`, starts a particle shader
/// (`ParticleShader::Start(first, second)`), sets it up
/// ([`tes_effect_shader_setup_particle_shader`] with `flag`), gives it the
/// float `time` (`InitParticleData`) and answers it; else 0.
pub fn tes_effect_shader_start_particle_shader(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    flag: u32,
    time: u32,
) -> u32 {
    if flag == 0 {
        return 0;
    }
    let shader = e.call(PARTICLE_SHADER_START, &args![first, second]).u32();
    tes_effect_shader_setup_particle_shader(e, this, Ptr::new(shader), flag);
    e.call(PARTICLE_INIT_DATA, &args![shader, x87_quiet_bits(time)]);
    shader
}

// Translated from 00507810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEffectShader::StopParticleShader` (Xbox PDB, `thiscall`, two
/// words): `ParticleShader::Stop(first, second)`.
pub fn tes_effect_shader_stop_particle_shader(e: &mut Engine, _this: Ptr, first: u32, second: u32) {
    e.call(PARTICLE_SHADER_STOP, &args![first, second]);
}

// Translated from 00507830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00507830 - the C++ exception frame (FS:[0], state for the 0x74-byte allocation) is not translated
/// Unnamed in the engine map (`thiscall`, three words): makes the texture
/// effect data of the shader: a 0x74-byte block (`00aa13e0`, constructed by
/// `00b668b0`, or null), its pointer at +8 cleared (`0066b0d0` with 0), the
/// shader's two colours (`0045cd60`, `0068a830`, bytes over 255) at +0x2C
/// and +0x3C, 0.0 at +0x4C and +0x50, the float [`fn_00507b00`] at +0x54,
/// twice `node`'s bound float (`0043d450`, `0084d030`) at +0x58, the four
/// words of `00441110`, `004fd3a0`, `007af430`, `0059bb30` at +0x5C..+0x68,
/// the pointers at +8 and +0x6C set to `first` and `second`, the float
/// [`fn_00507b20`] truncated to an integer at +0x70, and, when flag 0x10 is
/// set ([`fn_00507040`]), 1 taken from the second colour's red, green and
/// blue (`FADD` of the double -1.0). Answers the block.
pub fn fn_00507830(e: &mut Engine, this: Ptr, node: u32, first: u32, second: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x74u32]).u32();
    let data = if block != 0 {
        e.call(EFFECT_DATA_CTOR, &args![block]).u32()
    } else {
        0
    };
    let color_a = e.call(SHADER_COLOR_A, &args![this]).u32();
    let color_b = e.call(SHADER_COLOR_B, &args![this]).u32();
    e.call(SET_POINTER, &args![data.wrapping_add(8), 0u32]);
    e.with_stack(0x40, |e, locals| {
        let scratch = locals.addr();
        let out = locals.addr() + 0x20;
        let made = color_from_bytes(e, color_a, scratch, out);
        let words = [
            e.mem.u32(made),
            e.mem.u32(made + 4),
            e.mem.u32(made + 8),
            e.mem.u32(made + 0xc),
        ];
        store_four(e, data.wrapping_add(0x2c), words);
        let made = color_from_bytes(e, color_b, scratch + 0x10, out + 0x10);
        let words = [
            e.mem.u32(made),
            e.mem.u32(made + 4),
            e.mem.u32(made + 8),
            e.mem.u32(made + 0xc),
        ];
        store_four(e, data.wrapping_add(0x3c), words);
    });
    e.mem.set_u32(data.wrapping_add(0x4c), 0);
    e.mem.set_u32(data.wrapping_add(0x50), 0);
    let value = fn_00507b00(e, this).to_bits();
    e.mem.set_u32(data.wrapping_add(0x54), value);
    let bound = e.call(NODE_WORLD_BOUND, &args![node]).u32();
    let radius = e.call(BOUND_RADIUS_0084D030, &args![bound]).f64();
    e.mem.set_u32(
        data.wrapping_add(0x58),
        x87_store32(x87_add(radius, radius)),
    );
    let value = e.call(SHADER_GETTER_00441110, &args![this]).u32();
    e.mem.set_u32(data.wrapping_add(0x5c), value);
    let value = e.call(SHADER_GETTER_004FD3A0, &args![this]).u32();
    e.mem.set_u32(data.wrapping_add(0x60), value);
    let value = e.call(SHADER_GETTER_007AF430, &args![this]).u32();
    e.mem.set_u32(data.wrapping_add(0x64), value);
    let value = e.call(SHADER_GETTER_0059BB30, &args![this]).u32();
    e.mem.set_u32(data.wrapping_add(0x68), value);
    e.call(SET_POINTER, &args![data.wrapping_add(8), first]);
    e.call(SET_POINTER, &args![data.wrapping_add(0x6c), second]);
    let value = fn_00507b20(e, this);
    let whole = x87_fistp64_chop(X87Ext::from(f64::from(value))) as u32;
    e.mem.set_u32(data.wrapping_add(0x70), whole);
    if fn_00507040(e, this, 0x10) {
        let minus_one = e.global::<f64>(DOUBLE_MINUS_ONE);
        for offset in [0x3cu32, 0x40, 0x44] {
            let at = data.wrapping_add(offset);
            let sum = x87_add(x87_load32(e.mem.u32(at)), minus_one);
            e.mem.set_u32(at, x87_store32(sum));
        }
    }
    data
}

/// The shader property `node` has when it is a geometry (virtual slot
/// `+0x18` non-zero): its property of type 3 when that property's type word
/// (`00441110`) is 8 to 12 (signed), else 0.
fn shader_property(e: &mut Engine, node: u32) -> u32 {
    let property = e.call(NODE_GET_PROPERTY, &args![node, 3u32]).u32();
    let matches = property != 0
        && e.call(PROPERTY_TYPE, &args![property]).i32() >= 8
        && e.call(PROPERTY_TYPE, &args![property]).i32() <= 0xc;
    if matches {
        property
    } else {
        0
    }
}

// Translated from 00507b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEffectShader::StartTextureShader` (Xbox PDB, `thiscall`, two
/// words): on a geometry `node` (virtual slot `+0x18`) with a lighting
/// shader property, sets its texture effect data to `data`
/// (`SetTextureEffectData`) unless shader flag 0x20 is set
/// ([`fn_00507040`]) and the node's property of type 2 has no name
/// (`00413f40`, `0043b1b0`) or one other than `"skin"` (`00408b20`); on any
/// other node, walks its children (virtual slot `+0xC`, `0043b480`,
/// `0043b4a0`) the same way.
pub fn tes_effect_shader_start_texture_shader(e: &mut Engine, this: Ptr, node: u32, data: u32) {
    if node == 0 {
        return;
    }
    if e.vcall(node, 0x18, &args![]).u32() != 0 {
        let property = shader_property(e, node);
        if property == 0 {
            return;
        }
        let mut apply = true;
        if fn_00507040(e, this, 0x20) {
            let other = e.call(NODE_GET_PROPERTY, &args![node, 2u32]).u32();
            let holder = e.call(PROPERTY_00413F40, &args![other]).u32();
            if e.call(PROPERTY_NAME_0043B1B0, &args![holder]).u32() == 0 {
                apply = false;
            } else {
                let holder = e.call(PROPERTY_00413F40, &args![other]).u32();
                let name = e.call(PROPERTY_NAME_0043B1B0, &args![holder]).u32();
                if e.call(TEXT_COMPARE, &args![name, TEXT_SKIN]).u32() != 0 {
                    apply = false;
                }
            }
        }
        if apply {
            e.call(SET_TEXTURE_EFFECT_DATA, &args![property, data]);
        }
    } else {
        let children = e.vcall(node, 0xc, &args![]).u32();
        if children == 0 {
            return;
        }
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            tes_effect_shader_start_texture_shader(e, this, child, data);
            index += 1;
        }
    }
}

// Translated from 00507c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEffectShader::StopTextureShader` (Xbox PDB, `thiscall`, two words):
/// on a geometry `node` with a lighting shader property whose texture
/// effect data (`00507d80`) is `data`, clears it (`SetTextureEffectData`
/// with 0); on any other node, walks its children the same way.
pub fn tes_effect_shader_stop_texture_shader(e: &mut Engine, this: Ptr, node: u32, data: u32) {
    if node == 0 {
        return;
    }
    if e.vcall(node, 0x18, &args![]).u32() != 0 {
        let property = shader_property(e, node);
        if property != 0 && e.call(TEXTURE_EFFECT_DATA, &args![property]).u32() == data {
            e.call(SET_TEXTURE_EFFECT_DATA, &args![property, 0u32]);
        }
    } else {
        let children = e.vcall(node, 0xc, &args![]).u32();
        if children == 0 {
            return;
        }
        let mut index = 0u32;
        while index < e.call(CHILD_COUNT, &args![children]).u32() {
            let child = e.call(CHILD_AT, &args![children, index]).u32();
            tes_effect_shader_stop_texture_shader(e, this, child, data);
            index += 1;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_5c40, fn_00505c40(Ptr) -> Ptr),
        entry!(0x0050_6510, fn_00506510(Ptr)),
        entry!(0x0050_64d0, fn_005064d0(Ptr) -> Ptr),
        entry!(0x0050_5db0, fn_00505db0(Ptr)),
        entry!(0x0050_6390, fn_00506390(Ptr) -> u32),
        entry!(0x0050_7b00, fn_00507b00(Ptr) -> f32),
        entry!(0x0050_7b20, fn_00507b20(Ptr) -> f32),
        entry!(0x0050_5e70, tes_effect_shader_load(Ptr, Ptr) -> bool),
        entry!(0x0050_60f0, fn_005060f0(Ptr)),
        entry!(0x0050_62a0, fn_005062a0(Ptr, Ptr)),
        entry!(0x0050_6370, fn_00506370(Ptr) -> u32),
        entry!(0x0050_63b0, fn_005063b0(Ptr) -> u32),
        entry!(0x0050_63d0, fn_005063d0(Ptr, Ptr) -> bool),
        entry!(0x0050_6850, fn_00506850(Ptr)),
        entry!(0x0050_7040, fn_00507040(Ptr, u32) -> bool),
        entry!(
            0x0050_7060,
            tes_effect_shader_setup_particle_shader(Ptr, Ptr, u32)
        ),
        entry!(0x0050_74e0, fn_005074e0(Ptr, u32, u32) -> u32),
        entry!(0x0050_7550, fn_00507550(Ptr, u32)),
        entry!(0x0050_7570, fn_00507570(Ptr, u32)),
        entry!(0x0050_7590, fn_00507590(Ptr, u32, u32, u32)),
        entry!(0x0050_75c0, fn_005075c0(Ptr, u32, u32, u32)),
        entry!(0x0050_7630, fn_00507630(Ptr, u32, u32, u32, u32)),
        entry!(0x0050_7660, fn_00507660(Ptr, u32, u32, u32, u32)),
        entry!(0x0050_7690, fn_00507690(Ptr, u32, u32, u32, u32)),
        entry!(0x0050_76e0, fn_005076e0(Ptr, u32)),
        entry!(0x0050_7720, fn_00507720(Ptr, u32)),
        entry!(0x0050_7740, fn_00507740(Ptr, u32)),
        entry!(0x0050_7760, fn_00507760(Ptr, u32)),
        entry!(
            0x0050_77c0,
            tes_effect_shader_start_particle_shader(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x0050_7810,
            tes_effect_shader_stop_particle_shader(Ptr, u32, u32)
        ),
        entry!(0x0050_7830, fn_00507830(Ptr, u32, u32, u32) -> u32),
        entry!(
            0x0050_7b40,
            tes_effect_shader_start_texture_shader(Ptr, u32, u32)
        ),
        entry!(
            0x0050_7c90,
            tes_effect_shader_stop_texture_shader(Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00505c40_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_5c40,
            size: 0x170,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0050_64d0, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x14c)]),
                (0x0048_e270, &[A::T(0x158)]),
                (0x0048_e270, &[A::T(0x164)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4f)]),
                (0x0050_5db0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_685c)],
        });
    }
}

#[cfg(test)]
mod shader_data_tests {
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

    fn defaults_rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.mem.set_u32(0x0101_6248, 0x3f00_0000);
        e.mem.set_u32(0x0101_7b78, 0x4120_0000);
        // A signalling NaN comes out quiet.
        e.mem.set_u32(0x0102_3cd8, 0x7f80_0001);
        // The point constructor stores its three floats.
        e.register(POINT3_CTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            a[0].into_ret()
        });
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        let data = e.mem.alloc(0x140);
        e.mem.write(data, &[0xaa; 0x140]);
        e.call_log = Some(vec![]);
        (e, data)
    }

    fn check_defaults(e: &Engine, data: u32) {
        let word = |offset: u32| e.mem.u32(data + offset);
        assert_eq!(e.mem.u8(data), 0);
        assert_eq!(e.mem.bytes(data + 1, 3), vec![0xaa; 3]);
        assert_eq!([word(4), word(8), word(0xc), word(0x10)], [5, 1, 3, 0]);
        assert_eq!(word(0x28), 0x3f80_0000);
        assert_eq!(word(0x50), 0x3f80_0000);
        assert_eq!(
            [word(0x5c), word(0x60), word(0x64), word(0x68), word(0x6c)],
            [6, 5, 1, 4, 6]
        );
        assert_eq!(e.mem.bytes(data + 0x88, 0x24), vec![0; 0x24]);
        assert_eq!(word(0xbc), 0x00ff_ffff);
        assert_eq!(word(0xd8), 0x3f00_0000);
        assert_eq!(word(0xf4), 0xaaaa_aaaa);
        assert_eq!(word(0xfc), 0x4120_0000);
        assert_eq!(word(0x100), 0x7fc0_0001);
        assert_eq!(word(0x10c), 0x00ff_ffff);
        assert_eq!([word(0x114), word(0x118)], [1, 1]);
        assert_eq!(word(0x130), 0x3f80_0000);
        assert_eq!(word(0x134), 0xaaaa_aaaa);
        let untouched: Vec<u32> = (0..0x134)
            .step_by(4)
            .filter(|offset| word(*offset) == 0xaaaa_aaaa)
            .collect();
        assert_eq!(untouched, vec![0xf4]);
    }

    #[test]
    fn fn_00506510_writes_the_defaults() {
        let (mut e, data) = defaults_rig();
        e.call(0x0050_6510, &args![data]);
        check_defaults(&e, data);
        let points = calls(&e, POINT3_CTOR);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0][1..], [0, 0, 0]);
    }

    #[test]
    fn fn_005064d0_constructs_the_points_then_the_defaults() {
        let (mut e, data) = defaults_rig();
        assert_eq!(e.call(0x0050_64d0, &args![data]).u32(), data);
        assert_eq!(
            calls(&e, IDENTITY_006815C0),
            vec![vec![data + 0x94], vec![data + 0xa0]]
        );
        check_defaults(&e, data);
    }

    #[test]
    fn fn_00505db0_writes_the_defaults_on_the_data() {
        let (mut e, data) = defaults_rig();
        e.register(DATA_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.call(0x0050_5db0, &args![data - 0x18]);
        check_defaults(&e, data);
    }

    #[test]
    fn fn_00506390_and_00507b00_answer_their_fields() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0050_6390, &args![0x5000u32]).u32(), 0x5158);
        let this = e.mem.alloc(0x50);
        e.mem.set_f32(this + 0x4c, 3.25);
        assert_eq!(e.call(0x0050_7b00, &args![this]).f32(), 3.25);
        e.mem.set_u32(this + 0x4c, 0xff80_0004);
        assert_eq!(
            e.call(0x0050_7b00, &args![this]).f32().to_bits(),
            0xffc0_0004
        );
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(DOUBLE_ONE, 1.0);
        e.mem.set_f64(DOUBLE_ZERO, 0.0);
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x4fu32.into_ret());
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
        for addr in [
            GET_CHUNK_DATA_SIZED,
            LOAD_TEXTURE_CHUNK,
            SWAP_DATA_00506850,
            LOAD_DATA,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        e.register(COUNT_00BA52D0, |_, _| 0x1au32.into_ret());
        e.put_vtable(0x0f00_d000, &[0; 0x4e]);
        e.mem.set_u32(0x0f00_d018, 0x0f00_d300);
        e.mem.set_u32(0x0f00_d0e0, 0x0f00_d100);
        e.mem.set_u32(0x0f00_d134, 0x0f00_d200);
        for addr in [0x0f00_d100u32, 0x0f00_d200, 0x0f00_d300] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x170);
        e.mem.set_u32(this, 0x0f00_d000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00505e70_refuses_other_records() {
        let (mut e, this) = load_rig(vec![], false);
        e.register(GET_TES_FORM, |_, _| 0x50u32.into_ret());
        assert!(!e.call(0x0050_5e70, &args![this, FILE]).bool());
        assert!(calls(&e, 0x0f00_d300).is_empty());
    }

    #[test]
    fn fn_00505e70_reads_each_chunk() {
        let chunks = vec![
            (CHUNK_EDID as u32, 5),
            (CHUNK_OBND as u32, 0xc),
            (CHUNK_ICON as u32, 4),
            (CHUNK_ICO2 as u32, 4),
            (CHUNK_NAM7 as u32, 4),
            (CHUNK_DATA as u32, 0x134),
            (0x8000_0000, 0),
        ];
        let (mut e, this) = load_rig(chunks, true);
        assert!(e.call(0x0050_5e70, &args![this, FILE]).bool());
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .take(3)
            .map(|c| c.0)
            .collect();
        assert_eq!(order, vec![GET_TES_FORM, 0x0f00_d300, LOAD_FORM]);
        let edid = calls(&e, 0x0f00_d200);
        assert_eq!(
            calls(&e, GET_CHUNK_DATA_SIZED),
            vec![vec![FILE, edid[0][1], 0x200]]
        );
        assert_eq!(calls(&e, 0x0f00_d100), vec![vec![this, FILE]]);
        let textures: Vec<u32> = calls(&e, LOAD_TEXTURE_CHUNK)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(textures, vec![0x14c, 0x158, 0x164]);
        assert_eq!(
            calls(&e, LOAD_DATA),
            vec![vec![this, FILE, this + 0x18, 0x134]]
        );
        assert_eq!(calls(&e, SWAP_DATA_00506850), vec![vec![this + 0x18]; 2]);
        assert_eq!(calls(&e, NEXT_CHUNK).len(), 7);
        // Both scales were 0: unchanged, no count asked.
        assert!(calls(&e, COUNT_00BA52D0).is_empty());
    }

    #[test]
    fn fn_00505e70_scales_fractions_by_the_count() {
        for (value, scaled) in [
            (0.5f32, Some(13.0f32)),
            (1.0, Some(26.0)),
            (1.5, None),
            (0.0, None),
            (-0.5, None),
            (f32::NAN, None),
        ] {
            let (mut e, this) = load_rig(vec![], false);
            e.mem.set_f32(this + 0x94, value);
            e.mem.set_f32(this + 0x98, 0.25);
            assert!(e.call(0x0050_5e70, &args![this, FILE]).bool());
            match scaled {
                Some(want) => assert_eq!(e.mem.f32(this + 0x94), want),
                None => assert_eq!(e.mem.u32(this + 0x94), value.to_bits()),
            }
            assert_eq!(e.mem.f32(this + 0x98), 6.5);
        }
    }

    #[test]
    fn fn_00507b20_answers_the_float_at_0x118_quiet() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x120);
        e.mem.set_u32(this + 0x118, 2.5f32.to_bits());
        assert_eq!(e.call(0x0050_7b20, &args![this]).f32(), 2.5);
        e.mem.set_u32(this + 0x118, 0xff80_0003);
        assert_eq!(
            e.call(0x0050_7b20, &args![this]).f32().to_bits(),
            0xffc0_0003
        );
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const SLOT_A: u32 = 0x7100_0000;
    const SLOT_B: u32 = 0x7100_0004;
    const STATE: u32 = 0x7300_0000;
    /// A shader at a fixed address (its texture offsets are tested by the low bits).
    const SHADER: u32 = 0x7600_0000;

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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e.map(0x0101_a000, 0x1000);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        e.map(0x0102_3000, 0x1000);
        e.set_global(FLOAT_255, 255.0f32);
        e.map(SHADER, 0x1000);
        e
    }

    #[test]
    fn save_writes_textures_data_and_restores_the_form() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                FORM_START,
                FORM_CLOSE,
                FORM_ADD_CHUNK_ARRAY,
                TEXTURE_SAVE,
                FORM_SAVE_DATA,
                SWAP_WORD,
            ],
        );
        // Only the texture at +0x158 has a path.
        e.register(TEXTURE_PATH_LENGTH, |_, a| {
            u32::from(a[0] & 0xfff == 0x158).into_ret()
        });
        e.register(TEXTURE_PATH_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(FORM_ID, |_, a| (a[0] + 0x10).into_ret());
        e.register_double(FORM_SAVE_DATA, |e, a| {
            let id = e.mem.u32(a[1] + 0xf4);
            e.mem.set_u32(STATE, id);
            Ret::default()
        });
        e.register(SAVE_SWAPS, |e, _| e.mem.u32(STATE + 4).into_ret());
        let this = SHADER;
        e.mem.set_u32(this + 0x10c, 0x5000);
        e.mem.set_u32(STATE + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_60f0, &args![this]);
        assert_eq!(
            calls(&e, FORM_ADD_CHUNK_ARRAY),
            vec![
                vec![CHUNK_ICON_WORD, this + 0x14d, 1],
                vec![CHUNK_NAM7_WORD, this + 0x165, 1]
            ]
        );
        assert_eq!(
            calls(&e, TEXTURE_SAVE),
            vec![vec![this + 0x158, CHUNK_ICO2_WORD]]
        );
        assert_eq!(e.mem.u32(STATE), 0x5010);
        assert_eq!(e.mem.u32(this + 0x10c), 0x5000);
        assert_eq!(calls(&e, SWAP_WORD).len(), 2 * 75);
        assert_eq!(calls(&e, FORM_CLOSE), vec![vec![this]]);
        // No form, no swapping.
        e.mem.set_u32(this + 0x10c, 0);
        e.mem.set_u32(STATE + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_60f0, &args![this]);
        assert_eq!(e.mem.u32(STATE), 0);
        assert!(calls(&e, FORM_ID).is_empty());
        assert!(calls(&e, SWAP_WORD).is_empty());
    }

    fn shader_with_textures(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, SLOT_A);
        e.mem.set_u32(vtable + 0xc, SLOT_B);
        let this = SHADER;
        for offset in [0x14c, 0x158, 0x164] {
            e.mem.set_u32(this + offset, vtable);
        }
        this
    }

    #[test]
    fn copy_takes_components_data_and_textures() {
        let mut e = engine();
        quiet(&mut e, &[FORM_COPY_ALL_COMPONENTS, COPY_BYTES, SLOT_A]);
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(DATA_00500940, |_, a| (a[0] + 0x18).into_ret());
        let this = shader_with_textures(&mut e);
        let other = 0x9000u32;
        e.call_log = Some(vec![]);
        e.call(0x0050_62a0, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, TYPE_FORM, TYPE_EFFECT_SHADER, 0]]
        );
        assert_eq!(
            calls(&e, COPY_BYTES),
            vec![vec![this + 0x18, other + 0x18, 0x134]]
        );
        assert_eq!(
            calls(&e, SLOT_A),
            vec![
                vec![this + 0x14c, other + 0x14c],
                vec![this + 0x158, other + 0x158],
                vec![this + 0x164, other + 0x164]
            ]
        );
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0050_62a0, &args![this, other]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn texture_getters() {
        let mut e = engine();
        assert_eq!(e.call(0x0050_6370, &args![0x1000u32]).u32(), 0x114c);
        assert_eq!(e.call(0x0050_63b0, &args![0x1000u32]).u32(), 0x1164);
    }

    #[test]
    fn compare_branches() {
        let mut e = engine();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(DATA_00500940, |_, a| (a[0] + 0x18).into_ret());
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(STATE).into_ret()
        });
        // The texture comparison answers the state word for the texture
        // whose offset it holds.
        e.register(SLOT_B, |e, a| {
            u32::from(a[0] & 0xfff == e.mem.u32(STATE + 4)).into_ret()
        });
        e.register(MEMCMP, |e, _| e.mem.u32(STATE + 8).into_ret());
        let this = shader_with_textures(&mut e);
        let other = 0x9000u32;
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_63d0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, MEMCMP),
            vec![vec![this + 0x18, other + 0x18, 0x134]]
        );
        for texture in [0x14c, 0x158, 0x164] {
            e.mem.set_u32(STATE + 4, texture);
            assert!(e.call(0x0050_63d0, &args![this, other]).bool());
        }
        e.mem.set_u32(STATE + 4, 0);
        e.mem.set_u32(STATE + 8, 1);
        assert!(e.call(0x0050_63d0, &args![this, other]).bool());
        e.mem.set_u32(STATE + 8, 0);
        e.mem.set_u32(STATE, 1);
        assert!(e.call(0x0050_63d0, &args![this, other]).bool());
        e.mem.set_u32(STATE, 0);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0050_63d0, &args![this, other]).bool());
    }

    #[test]
    fn swap_skips_the_word_at_f4() {
        let mut e = engine();
        quiet(&mut e, &[SWAP_WORD]);
        e.call_log = Some(vec![]);
        e.call(0x0050_6850, &args![0x1000u32]);
        let swaps: Vec<u32> = calls(&e, SWAP_WORD).iter().map(|c| c[0]).collect();
        assert_eq!(swaps.len(), 75);
        assert_eq!(swaps[0], 0x1004);
        assert_eq!(*swaps.last().unwrap(), 0x1130);
        assert!(!swaps.contains(&0x10f4));
        assert!(!swaps.contains(&0x1000));
        assert!(calls(&e, SWAP_WORD).iter().all(|c| c[1] == 0));
    }

    #[test]
    fn flag_test_reads_the_data_byte() {
        let mut e = engine();
        e.register(DATA_00500940, |_, _| STATE.into_ret());
        e.mem.set_u32(STATE, 0x0000_0110);
        assert!(e.call(0x0050_7040, &args![0x1000u32, 0x10u32]).bool());
        assert!(!e.call(0x0050_7040, &args![0x1000u32, 0x20u32]).bool());
        // Only the byte counts.
        assert!(!e.call(0x0050_7040, &args![0x1000u32, 0x100u32]).bool());
    }

    #[test]
    fn color_divide_writes_out() {
        let mut e = engine();
        let color = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        for (index, value) in [255.0f32, 51.0, 0.0, 1.0].iter().enumerate() {
            e.mem.set_u32(color + 4 * index as u32, value.to_bits());
        }
        let answer = e
            .call(0x0050_74e0, &args![color, out, 255.0f32.to_bits()])
            .u32();
        assert_eq!(answer, out);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 0.2);
        assert_eq!(e.mem.f32(out + 8), 0.0);
        assert_eq!(e.mem.f32(out + 0xc), (1.0f64 / 255.0) as f32);
        // A signalling NaN comes out quiet.
        e.mem.set_u32(color, 0x7f80_0001);
        e.call(0x0050_74e0, &args![color, out, 255.0f32.to_bits()]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
    }

    #[test]
    fn simple_setters() {
        let mut e = engine();
        quiet(&mut e, &[SET_POINTER]);
        let this = e.mem.alloc(0x120);
        e.call_log = Some(vec![]);
        e.call(0x0050_7550, &args![this, 7u32]);
        assert_eq!(calls(&e, SET_POINTER), vec![vec![this + 0x118, 7]]);
        e.call(0x0050_7570, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x80), 0x7fc0_0001);
        e.call(0x0050_7590, &args![this, 1u32, 2u32, 3u32]);
        e.call(0x0050_75c0, &args![this, 4u32, 5u32, 6u32]);
        e.call(0x0050_7630, &args![this, 7u32, 8u32, 9u32, 10u32]);
        e.call(0x0050_7660, &args![this, 11u32, 12u32, 13u32, 14u32]);
        e.call(
            0x0050_7690,
            &args![this, 15u32, 16u32, 17u32, 0x7f80_0001u32],
        );
        let words = |e: &Engine, at: u32, n: u32| -> Vec<u32> {
            (0..n).map(|i| e.mem.u32(this + at + 4 * i)).collect()
        };
        assert_eq!(words(&e, 0x90, 3), vec![1, 2, 3]);
        assert_eq!(words(&e, 0x9c, 3), vec![4, 5, 6]);
        assert_eq!(words(&e, 0xc4, 4), vec![7, 8, 9, 10]);
        assert_eq!(words(&e, 0xd4, 4), vec![11, 12, 13, 14]);
        // Copied as words: the signalling NaN stays.
        assert_eq!(words(&e, 0xe4, 4), vec![15, 16, 17, 0x7f80_0001]);
        e.call(0x0050_76e0, &args![this, 0x7f80_0002u32]);
        e.call(0x0050_7720, &args![this, 2.5f32.to_bits()]);
        e.call(0x0050_7740, &args![this, 0x7f80_0003u32]);
        e.call(0x0050_7760, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0xf4), 0x7fc0_0002);
        assert_eq!(e.mem.f32(this + 0x100), 2.5);
        assert_eq!(e.mem.u32(this + 0x110), 0x7f80_0003);
        assert_eq!(e.mem.u32(this + 0x114), 0x1234);
    }

    const OUTSIDE_SETTERS: [u32; 18] = [
        TARGET_00584150,
        TARGET_0051B8C0,
        TARGET_00810550,
        TARGET_006E29D0,
        TARGET_005075F0,
        TARGET_009314D0,
        TARGET_00507610,
        TARGET_006E5EE0,
        TARGET_00507780,
        TARGET_005F9B10,
        TARGET_005077A0,
        TARGET_006E4EF0,
        TARGET_006E4320,
        TARGET_005076C0,
        TARGET_00733FD0,
        TARGET_00507700,
        TARGET_006D30E0,
        TARGET_00505E50,
    ];

    /// The `NiColorA` constructor double: stores its four words.
    fn color_ctor(e: &mut Engine) {
        e.register(COLOR_CTOR, |e, a| {
            for index in 0..4 {
                e.mem.set_u32(a[0] + 4 * index as u32, a[1 + index]);
            }
            a[0].into_ret()
        });
    }

    #[test]
    fn setup_particle_shader_hands_over_every_setting() {
        let mut e = engine();
        quiet(&mut e, &OUTSIDE_SETTERS);
        quiet(&mut e, &[SET_POINTER]);
        color_ctor(&mut e);
        let this = e.mem.alloc(0x170);
        for offset in (0x78..0x134).step_by(4) {
            e.mem.set_u32(this + offset, 0x3f80_0000 + offset);
        }
        e.mem.set_u32(this + 0x9c, 0x7f80_0001);
        e.mem.set_u32(this + 0x12c, 0xffff_ffff);
        e.mem.set_u32(this + 0x130, 3);
        e.mem.set_u32(this + 0xd4, 0x00ff_33ff);
        e.mem.set_u32(this + 0xd8, 0x0000_0000);
        e.mem.set_u32(this + 0xdc, 0x0012_3456);
        e.mem.set_u32(this + 0xe0, 0x7f80_0005);
        let target = e.mem.alloc(0x120);
        e.call_log = Some(vec![]);
        e.call(0x0050_7060, &args![this, target, 9u32]);
        assert_eq!(calls(&e, SET_POINTER), vec![vec![target + 0x118, 9]]);
        assert_eq!(calls(&e, TARGET_00584150), vec![vec![target, 0x7fc0_0001]]);
        assert_eq!(e.mem.u32(target + 0x80), 0x3f80_00a0);
        assert_eq!(calls(&e, TARGET_006E29D0), vec![vec![target, 0x3f80_00f8]]);
        assert_eq!(e.mem.u32(target + 0x90), 0x3f80_00ac);
        assert_eq!(e.mem.u32(target + 0xa4), 0x3f80_00c0);
        assert_eq!(
            calls(&e, TARGET_00507780),
            vec![vec![target, 4_294_967_296.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, TARGET_005F9B10),
            vec![vec![target, 3.0f32.to_bits()]]
        );
        // Colours: bytes over 255, the alphas from +0xE0..+0xE8.
        assert_eq!(e.mem.f32(target + 0xc4), 1.0);
        assert_eq!(e.mem.f32(target + 0xc8), 0.2);
        assert_eq!(e.mem.f32(target + 0xcc), 1.0);
        assert_eq!(e.mem.u32(target + 0xd0), 0x7fc0_0005);
        assert_eq!(e.mem.f32(target + 0xd4), 0.0);
        assert_eq!(e.mem.u32(target + 0xe0), 0x3f80_00e4);
        assert_eq!(
            e.mem.u32(target + 0xe4),
            x87_store32(x87_div(f64::from(0x56u32), 255.0))
        );
        assert_eq!(e.mem.u32(target + 0xf0), 0x3f80_00e8);
        assert_eq!(e.mem.u32(target + 0xf4), 0x3f80_00c4);
        assert_eq!(e.mem.u32(target + 0x100), 0x3f80_00d0);
        assert_eq!(calls(&e, TARGET_006D30E0), vec![vec![target, 0x3f80_0078]]);
        assert_eq!(calls(&e, TARGET_00505E50), vec![vec![target, 0x3f80_0084]]);
        assert_eq!(e.mem.u32(target + 0x110), 0x3f80_007c);
        assert_eq!(e.mem.u32(target + 0x114), 0x3f80_0080);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| OUTSIDE_SETTERS.contains(a))
            .collect();
        assert_eq!(order, OUTSIDE_SETTERS.to_vec());
    }

    #[test]
    fn start_and_stop_particle_shader() {
        let mut e = engine();
        quiet(&mut e, &OUTSIDE_SETTERS);
        quiet(
            &mut e,
            &[SET_POINTER, PARTICLE_INIT_DATA, PARTICLE_SHADER_STOP],
        );
        color_ctor(&mut e);
        let target = e.mem.alloc(0x120);
        e.register_double(PARTICLE_SHADER_START, move |_, _| target.into_ret());
        let this = e.mem.alloc(0x170);
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x0050_77c0, &args![this, 1u32, 2u32, 3u32, 0x7f80_0001u32])
            .u32();
        assert_eq!(answer, target);
        assert_eq!(calls(&e, PARTICLE_SHADER_START), vec![vec![1, 2]]);
        assert_eq!(calls(&e, SET_POINTER), vec![vec![target + 0x118, 3]]);
        assert_eq!(
            calls(&e, PARTICLE_INIT_DATA),
            vec![vec![target, 0x7fc0_0001]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0050_77c0, &args![this, 1u32, 2u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert!(calls(&e, PARTICLE_SHADER_START).is_empty());
        e.call(0x0050_7810, &args![this, 5u32, 6u32]);
        assert_eq!(calls(&e, PARTICLE_SHADER_STOP), vec![vec![5, 6]]);
    }

    #[test]
    fn texture_effect_data_is_filled() {
        let mut e = engine();
        quiet(&mut e, &[SET_POINTER]);
        color_ctor(&mut e);
        e.register(ALLOCATE, |_, _| 0x7400_0000u32.into_ret());
        e.map(0x7400_0000, 0x80);
        e.register(EFFECT_DATA_CTOR, |_, a| a[0].into_ret());
        e.register(SHADER_COLOR_A, |_, _| 0x00ff_00ffu32.into_ret());
        e.register(SHADER_COLOR_B, |_, _| 0x0033_3333u32.into_ret());
        e.register(NODE_WORLD_BOUND, |_, a| (a[0] + 1).into_ret());
        e.register(BOUND_RADIUS_0084D030, |_, _| Ret {
            st0: 1.25,
            ..Ret::default()
        });
        e.register(SHADER_GETTER_00441110, |_, _| 0x11u32.into_ret());
        e.register(SHADER_GETTER_004FD3A0, |_, _| 0x22u32.into_ret());
        e.register(SHADER_GETTER_007AF430, |_, _| 0x33u32.into_ret());
        e.register(SHADER_GETTER_0059BB30, |_, _| 0x44u32.into_ret());
        e.register(DATA_00500940, |_, a| (a[0] + 0x18).into_ret());
        let this = e.mem.alloc(0x170);
        e.mem.set_f32(this + 0x4c, 0.75);
        e.mem.set_f32(this + 0x118, -2.75);
        e.mem.set_u8(this + 0x18, 0x10);
        e.call_log = Some(vec![]);
        let data = e
            .call(0x0050_7830, &args![this, 0x500u32, 0x66u32, 0x77u32])
            .u32();
        assert_eq!(data, 0x7400_0000);
        assert_eq!(
            calls(&e, SET_POINTER),
            vec![
                vec![data + 8, 0],
                vec![data + 8, 0x66],
                vec![data + 0x6c, 0x77]
            ]
        );
        assert_eq!(e.mem.f32(data + 0x2c), 1.0);
        assert_eq!(e.mem.f32(data + 0x30), 0.0);
        assert_eq!(e.mem.f32(data + 0x34), 1.0);
        assert_eq!(e.mem.f32(data + 0x38), 0.0);
        // The flag 0x10 takes 1 from the second colour's red, green, blue.
        assert_eq!(e.mem.f32(data + 0x3c), 0.2 - 1.0);
        assert_eq!(e.mem.f32(data + 0x44), 0.2 - 1.0);
        assert_eq!(e.mem.f32(data + 0x48), 0.0);
        assert_eq!(e.mem.f32(data + 0x54), 0.75);
        assert_eq!(e.mem.f32(data + 0x58), 2.5);
        assert_eq!(calls(&e, NODE_WORLD_BOUND), vec![vec![0x500]]);
        assert_eq!(calls(&e, BOUND_RADIUS_0084D030), vec![vec![0x501]]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(data + 0x5c + 4 * i)).collect();
        assert_eq!(words, vec![0x11, 0x22, 0x33, 0x44]);
        assert_eq!(e.mem.u32(data + 0x70) as i32, -2);
        // Without the flag the colour stays.
        e.mem.set_u8(this + 0x18, 0);
        e.call(0x0050_7830, &args![this, 0x500u32, 0x66u32, 0x77u32]);
        assert_eq!(e.mem.f32(data + 0x3c), 0.2);
    }

    /// Nodes: `GEOMETRY` (slot +0x18 non-zero) with a property of type 3 at
    /// `PROPERTY` whose type word is the state word; `PARENT` (slot +0x18
    /// zero) with the children list `LIST` holding `GEOMETRY` twice.
    const GEOMETRY: u32 = 0x7500_0000;
    const PARENT: u32 = 0x7500_0100;
    const LIST: u32 = 0x7500_0200;
    const PROPERTY: u32 = 0x7500_0300;
    const IS_GEOMETRY: u32 = 0x7100_0100;

    fn node_engine() -> Engine {
        let mut e = engine();
        e.map(GEOMETRY, 0x400);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x18, IS_GEOMETRY);
        e.mem.set_u32(vtable + 0xc, IS_GEOMETRY + 0xc);
        e.mem.set_u32(GEOMETRY, vtable);
        e.mem.set_u32(PARENT, vtable);
        e.register(IS_GEOMETRY, |_, a| u32::from(a[0] == GEOMETRY).into_ret());
        e.register(IS_GEOMETRY + 0xc, |_, a| {
            let list = if a[0] == PARENT { LIST } else { 0 };
            list.into_ret()
        });
        e.register(CHILD_COUNT, |_, _| 2u32.into_ret());
        e.register(CHILD_AT, |_, _| GEOMETRY.into_ret());
        e.register(NODE_GET_PROPERTY, |_, a| (PROPERTY + a[1]).into_ret());
        e.register(PROPERTY_TYPE, |e, _| e.mem.u32(STATE).into_ret());
        e.register(PROPERTY_00413F40, |_, a| (a[0] + 0x10).into_ret());
        e.register(PROPERTY_NAME_0043B1B0, |e, _| {
            e.mem.u32(STATE + 4).into_ret()
        });
        e.register(TEXT_COMPARE, |e, _| e.mem.u32(STATE + 8).into_ret());
        e.register(SET_TEXTURE_EFFECT_DATA, |_, _| Ret::default());
        e.register(TEXTURE_EFFECT_DATA, |e, _| {
            e.mem.u32(STATE + 0xc).into_ret()
        });
        e.register(DATA_00500940, |_, _| (STATE + 0x10).into_ret());
        e.mem.set_u32(STATE, 8);
        e
    }

    #[test]
    fn start_texture_shader_branches() {
        let mut e = node_engine();
        e.call_log = Some(vec![]);
        // A geometry with a lighting property (type 8..12): set.
        e.call(0x0050_7b40, &args![0x1000u32, GEOMETRY, 0x99u32]);
        assert_eq!(
            calls(&e, SET_TEXTURE_EFFECT_DATA),
            vec![vec![PROPERTY + 3, 0x99]]
        );
        // Type 13: nothing.
        e.mem.set_u32(STATE, 13);
        e.call_log = Some(vec![]);
        e.call(0x0050_7b40, &args![0x1000u32, GEOMETRY, 0x99u32]);
        assert!(calls(&e, SET_TEXTURE_EFFECT_DATA).is_empty());
        // Flag 0x20: no name, a name that is not "skin": nothing; "skin": set.
        e.mem.set_u32(STATE, 12);
        e.mem.set_u8(STATE + 0x10, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0050_7b40, &args![0x1000u32, GEOMETRY, 0x99u32]);
        assert!(calls(&e, SET_TEXTURE_EFFECT_DATA).is_empty());
        e.mem.set_u32(STATE + 4, 0x4444);
        e.mem.set_u32(STATE + 8, 1);
        e.call(0x0050_7b40, &args![0x1000u32, GEOMETRY, 0x99u32]);
        assert!(calls(&e, SET_TEXTURE_EFFECT_DATA).is_empty());
        e.mem.set_u32(STATE + 8, 0);
        e.call(0x0050_7b40, &args![0x1000u32, GEOMETRY, 0x99u32]);
        assert_eq!(calls(&e, TEXT_COMPARE)[1], vec![0x4444, TEXT_SKIN]);
        assert_eq!(calls(&e, NODE_GET_PROPERTY)[1], vec![GEOMETRY, 2]);
        assert_eq!(calls(&e, SET_TEXTURE_EFFECT_DATA).len(), 1);
        // A parent: each child.
        e.mem.set_u8(STATE + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_7b40, &args![0x1000u32, PARENT, 0x98u32]);
        assert_eq!(calls(&e, CHILD_AT), vec![vec![LIST, 0], vec![LIST, 1]]);
        assert_eq!(calls(&e, SET_TEXTURE_EFFECT_DATA).len(), 2);
        // No node.
        e.call_log = Some(vec![]);
        e.call(0x0050_7b40, &args![0x1000u32, 0u32, 0x98u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn stop_texture_shader_clears_only_its_data() {
        let mut e = node_engine();
        e.mem.set_u32(STATE + 0xc, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x0050_7c90, &args![0x1000u32, GEOMETRY, 0x98u32]);
        assert!(calls(&e, SET_TEXTURE_EFFECT_DATA).is_empty());
        e.call(0x0050_7c90, &args![0x1000u32, PARENT, 0x99u32]);
        assert_eq!(
            calls(&e, SET_TEXTURE_EFFECT_DATA),
            vec![vec![PROPERTY + 3, 0], vec![PROPERTY + 3, 0]]
        );
        e.mem.set_u32(STATE, 7);
        e.call_log = Some(vec![]);
        e.call(0x0050_7c90, &args![0x1000u32, GEOMETRY, 0x99u32]);
        assert!(calls(&e, TEXTURE_EFFECT_DATA).is_empty());
    }
}
