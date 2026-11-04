//! `fallout shared/tesclimate.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESClimate` (Xbox PDB), 0x60 bytes on PC (the Xbox `TESForm` base is 0x10 larger; the
    /// PC code reads these offsets).
    pub struct TESClimate: 0x60 {
        /// `mNightSky` (Xbox PDB): a `TESModel`.
        0x18 mNightSky: u32,
        /// `weatherList` (Xbox PDB): a `TESWeatherList` (`BSSimpleList` of `WeatherEntry`).
        0x30 weatherList: u32,
        /// `txSkyObjects` (Xbox PDB): two `TESTexture`s of 0xc bytes (sun, sun glare).
        0x38 txSkyObjects: u32,
        /// `ucData` (Xbox PDB): six bytes; the byte at +0x55 holds flags in its top bits.
        0x50 ucData: u8,
        /// `strName` (Xbox PDB).
        0x58 strName: u32,
    }
}

/// `TESSoundFile::SetSoundFile` (engine map, `00489100`; by its body: copies a C string into
/// the `BSStringT` at `this + 4`), `thiscall`, one stack word.
const SET_STRING_00489100: u32 = 0x0048_9100;
/// `"Sky\Sun.dds"`, `"Sky\SunGlare.dds"`, `"Sky\Stars.nif"`.
const SUN_NAME: u32 = 0x0102_d7d8;
const SUN_GLARE_NAME: u32 = 0x0102_d7c4;
const STARS_NAME: u32 = 0x0102_d7b4;
/// `0052ac50` (`ECX`, one stack word): sets the low six bits of the byte at +0x55.
const SET_LOW_BITS_0052AC50: u32 = 0x0052_ac50;
/// `00470470` (`ECX`): empties a `BSSimpleList`.
const LIST_REMOVE_ALL_00470470: u32 = 0x0047_0470;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVTESWeather@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_WEATHER: u32 = 0x0118_629c;
/// The form id of the weather the default climate gets.
const DEFAULT_WEATHER_ID: u32 = 0x15e;
/// `005ae3d0` (`ECX` = list, one stack word: address of the new head value): pushes a value
/// onto the front of a `BSSimpleList`.
const LIST_ADD_HEAD_005AE3D0: u32 = 0x005a_e3d0;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The release build's logging stub (`005b5e40`, `cdecl`, variable words).
const LOG: u32 = 0x005b_5e40;
/// "DEFAULT: Unable to add default weather to default climate.  ( TESClimate::MakeDefault() )"
const MESSAGE_NO_DEFAULT_WEATHER: u32 = 0x0102_d758;
/// "MASTERFILE: Warnings were encountered while loading weather list chunk from climate %s"
const MESSAGE_WEATHER_LIST_WARNINGS: u32 = 0x0102_d700;

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
/// `TESModel::LoadModelChunk` (Xbox PDB, `004892d0`, `cdecl`: model, file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `0046e8a0` (`cdecl`, no words): the thread's warning count.
const WARNING_COUNT_0046E8A0: u32 = 0x0046_e8a0;
/// `MessageHandler::IncDisableWarningCount` (Xbox PDB, `0043b2b0`, `cdecl`, one byte word).
const INC_DISABLE_WARNING_COUNT: u32 = 0x0043_b2b0;
/// `TESWeatherList::LoadWeatherListChunk` (Xbox PDB, `00582940`, `cdecl`: list, file, form).
const LOAD_WEATHER_LIST_CHUNK: u32 = 0x0058_2940;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;

/// Chunk codes `TESClimate::Load` reads (the four characters, first byte lowest).
const CHUNK_EDID: i32 = 0x4449_4445;
const CHUNK_OBND: i32 = 0x444e_424f;
const CHUNK_MODL: i32 = 0x4c44_4f4d;
const CHUNK_FNAM: i32 = 0x4d41_4e46;
const CHUNK_PNAM: i32 = 0x4d41_4e50;
const CHUNK_TNAM: i32 = 0x4d41_4e54;
const CHUNK_WLST: i32 = 0x5453_4c57;
/// The climate record's form type.
const FORM_TYPE_CLIMATE: u8 = 0x36;

// Translated from 00824340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `BSSimpleList<WeatherEntry_P>::AddHead`; by its body (`ECX`, no
/// stack words) it is the `WeatherEntry` constructor: clears the words at +0 (weather), +4
/// (chance) and +8 (global) and answers `this`.
pub fn fn_00824340(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

// Translated from 0052b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range), `ECX` and one byte word (`RET 4`): sets bit
/// 0x80 of the byte at +0x55 when `on` is nonzero, clears it otherwise.
pub fn fn_0052b260(e: &mut Engine, this: Ptr<TESClimate>, on: u8) {
    let at = this.addr() + 0x55;
    let flags = e.mem.u8(at);
    e.mem
        .set_u8(at, if on != 0 { flags | 0x80 } else { flags & 0x7f });
}

// Translated from 0052b2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range), `ECX` and one byte word (`RET 4`): sets bit
/// 0x40 of the byte at +0x55 when `on` is nonzero, clears it otherwise.
pub fn fn_0052b2c0(e: &mut Engine, this: Ptr<TESClimate>, on: u8) {
    let at = this.addr() + 0x55;
    let flags = e.mem.u8(at);
    e.mem
        .set_u8(at, if on != 0 { flags | 0x40 } else { flags & 0xbf });
}

// Translated from 0052b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052b0f0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef781b, state variable) is not translated
/// `TESClimate::MakeDefault` (Xbox PDB), `ECX`: the sun and sun glare textures
/// (`Sky\Sun.dds`, `Sky\SunGlare.dds` through `00489100` on +0x38 and +0x44), the night sky
/// model (`Sky\Stars.nif`, vtable slot +0x18 of the model at +0x18), the timing bytes
/// 0x24, 0x2a, 0x6c, 0x72, 0 at +0x50..+0x54, both flag bits of +0x55 set (`0052b260`,
/// `0052b2c0`), the low bits set to 3 (`0052ac50`), the weather list emptied (`00470470`),
/// then one entry (a new 0xc-byte block through `00824340`, null when the allocation fails):
/// weather form 0x15e cast to `TESWeather`, chance 100. With a weather it is pushed onto the
/// list (`005ae3d0`); without one the entry is deleted and the failure reported.
pub fn tes_climate_make_default(e: &mut Engine, this: Ptr<TESClimate>) {
    let base = this.addr();
    e.call(SET_STRING_00489100, &args![base + 0x38, SUN_NAME]);
    e.call(SET_STRING_00489100, &args![base + 0x44, SUN_GLARE_NAME]);
    e.vcall(base + 0x18, 0x18, &args![STARS_NAME]);
    for (offset, value) in [
        (0x50u32, 0x24u8),
        (0x51, 0x2a),
        (0x52, 0x6c),
        (0x53, 0x72),
        (0x54, 0),
    ] {
        e.mem.set_u8(base + offset, value);
    }
    fn_0052b260(e, this, 1);
    fn_0052b2c0(e, this, 1);
    e.call(SET_LOW_BITS_0052AC50, &args![this, 3u32]);
    e.call(LIST_REMOVE_ALL_00470470, &args![base + 0x30]);
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    let entry = if block != 0 {
        fn_00824340(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.with_stack(4, |e, entry_at| {
        e.mem.set_u32(entry_at.addr(), entry);
        let found = e.call(LOOKUP_FORM, &args![DEFAULT_WEATHER_ID]).u32();
        let weather = e
            .call(
                DYNAMIC_CAST,
                &args![found, 0u32, TYPE_TES_FORM, TYPE_TES_WEATHER, 0u32],
            )
            .u32();
        e.mem.set_u32(entry, weather);
        e.mem.set_u32(entry + 4, 100);
        if e.mem.u32(entry) != 0 {
            e.call(LIST_ADD_HEAD_005AE3D0, &args![base + 0x30, entry_at]);
        } else {
            e.call(OPERATOR_DELETE, &args![entry]);
            e.call(LOG, &args![MESSAGE_NO_DEFAULT_WEATHER]);
        }
    });
}

// Translated from 0052ac90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052ac90 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `TESClimate::Load` (Xbox PDB), `ECX` and one stack word, the file (`RET 4`). A record that
/// is not a climate (`TESFile::GetTESForm` not 0x36) answers false. Otherwise
/// `TESForm::LoadForm`, form flag 8 cleared (`00484ab0`), then each chunk (codes compared
/// signed) until `GetTESChunk` answers 0 or `004726f0` false: `EDID` (a stack buffer of the
/// chunk's size, read with at most 0x200 bytes, given to vtable slot +0x134), `OBND` (vtable
/// slot +0xe0 with the file), `MODL` (the night sky, `TESModel::LoadModelChunk` on +0x18),
/// `FNAM`/`GNAM` (the sky textures at +0x38/+0x44), `TNAM` (six bytes into +0x50), `WLST`
/// (`TESWeatherList::LoadWeatherListChunk` on +0x30 with the file and the climate, warnings
/// disabled around it; when the thread's warning count grew, a report with the climate's
/// name from vtable slot +0x130). Then true.
pub fn tes_climate_load(e: &mut Engine, this: Ptr<TESClimate>, file: Ptr) -> bool {
    let form_type = e.call(GET_TES_FORM, &args![file]).u32() as u8;
    if form_type != FORM_TYPE_CLIMATE {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        let code = chunk as i32;
        if code > CHUNK_PNAM {
            if code == CHUNK_TNAM {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x50, 6u32]);
            } else if code == CHUNK_WLST {
                let before = e.call(WARNING_COUNT_0046E8A0, &args![]).i32();
                e.call(INC_DISABLE_WARNING_COUNT, &args![1u32]);
                e.call(LOAD_WEATHER_LIST_CHUNK, &args![base + 0x30, file, this]);
                e.call(INC_DISABLE_WARNING_COUNT, &args![0u32]);
                if e.call(WARNING_COUNT_0046E8A0, &args![]).i32() > before {
                    let name = e.vcall(base, 0x130, &args![]).u32();
                    e.call(LOG, &args![MESSAGE_WEATHER_LIST_WARNINGS, name]);
                }
            }
        } else if code >= CHUNK_FNAM {
            let index = code.wrapping_sub(CHUNK_FNAM);
            if index < 2 {
                let texture = base + 0x38 + index as u32 * 0xc;
                e.call(LOAD_TEXTURE_CHUNK, &args![texture, file]);
            }
        } else if code == CHUNK_EDID {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            e.with_stack(size, |e, name| {
                e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                e.vcall(base, 0x134, &args![name]);
            });
        } else if code == CHUNK_OBND {
            e.vcall(base, 0xe0, &args![file]);
        } else if code == CHUNK_MODL {
            e.call(LOAD_MODEL_CHUNK, &args![base + 0x18, file]);
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `TESForm::StartForm` (`004855a0`), `TESForm::CloseForm` (`00485680`),
/// `TESForm::AddChunkArray` (`004856f0`, `cdecl (chunk, data, size)`), the
/// weather list's save (`00582380`, `thiscall`, one chunk word),
/// `TESTexture::Save` (`0048e370`, one chunk word) and `TESModel::Save`
/// (`004891d0`, two chunk words).
const FORM_START: u32 = 0x0048_55a0;
const FORM_CLOSE: u32 = 0x0048_5680;
const FORM_ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
const WEATHER_LIST_SAVE: u32 = 0x0058_2380;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MODEL_SAVE: u32 = 0x0048_91d0;
/// `TESForm::CopyAllComponents` (`004851b0`), `TESForm::CompareAllComponents`
/// (`00485270`), the weather list's copy (`005825a0`, two words) and
/// comparison (`00582690`, one word), the copy `00401460` (`cdecl (to,
/// from, size)`) and `memcmp` (`00ec4835`).
const FORM_COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const FORM_COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
const WEATHER_LIST_COPY: u32 = 0x0058_25a0;
const WEATHER_LIST_COMPARE: u32 = 0x0058_2690;
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;
/// The `TESClimate` type descriptor (`01184170`, its name read at +8).
const TYPE_TES_CLIMATE: u32 = 0x0118_4170;

// Translated from 0052ae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESClimate::Save` (Xbox PDB): starts the form, saves the weather list
/// (+0x30, `WLST`), the two sky textures (+0x38, +0x44, as `FNAM` and
/// `GNAM`: `FNAM` plus the index), the night sky model (+0x18, `MODL`,
/// `MODT`), the six data bytes at +0x50 (`TNAM`), and closes the form.
pub fn tes_climate_save(e: &mut Engine, this: Ptr<TESClimate>) {
    let base = this.addr();
    e.call(FORM_START, &args![this]);
    e.call(WEATHER_LIST_SAVE, &args![base + 0x30, CHUNK_WLST as u32]);
    for index in 0..2u32 {
        e.call(
            TEXTURE_SAVE,
            &args![base + 0x38 + index * 0xc, (CHUNK_FNAM as u32) + index],
        );
    }
    e.call(
        MODEL_SAVE,
        &args![base + 0x18, CHUNK_MODL as u32, 0x5444_4f4du32],
    );
    e.call(
        FORM_ADD_CHUNK_ARRAY,
        &args![CHUNK_TNAM as u32, base + 0x50, 6u32],
    );
    e.call(FORM_CLOSE, &args![this]);
}

/// `other` cast from `TESForm` to `TESClimate`.
fn cast_climate(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, TYPE_TES_FORM, TYPE_TES_CLIMATE, 0u32],
    )
    .u32()
}

// Translated from 0052af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the copy of a
/// `TESClimate`): when `other` is one, copies all components, the weather
/// list (`005825a0` with flag 0), each sky texture and the night sky model
/// (virtual slot `+8` of each with `other`'s) and the six data bytes.
pub fn fn_0052af10(e: &mut Engine, this: Ptr<TESClimate>, other: Ptr) {
    let source = cast_climate(e, other);
    if source == 0 {
        return;
    }
    let base = this.addr();
    e.call(FORM_COPY_ALL_COMPONENTS, &args![this, source]);
    e.call(WEATHER_LIST_COPY, &args![base + 0x30, source + 0x30, 0u32]);
    for index in 0..2u32 {
        let offset = 0x38 + index * 0xc;
        e.vcall(base + offset, 8, &args![source + offset]);
    }
    e.vcall(base + 0x18, 8, &args![source + 0x18]);
    e.call(COPY_BYTES, &args![base + 0x50, source + 0x50, 6u32]);
}

// Translated from 0052afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; the comparison of a
/// `TESClimate`): true (different) when `other` is none, when
/// `CompareAllComponents` says so, when a sky texture's or the night sky
/// model's virtual slot `+0xC` answers non-zero, when the weather lists
/// differ (`00582690`) or when the six data bytes differ (`memcmp`).
pub fn fn_0052afe0(e: &mut Engine, this: Ptr<TESClimate>, other: Ptr) -> bool {
    let source = cast_climate(e, other);
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
    for index in 0..2u32 {
        let offset = 0x38 + index * 0xc;
        if e.vcall(base + offset, 0xc, &args![source + offset]).u8() != 0 {
            return true;
        }
    }
    if e.vcall(base + 0x18, 0xc, &args![source + 0x18]).u8() != 0 {
        return true;
    }
    if e.call(WEATHER_LIST_COMPARE, &args![base + 0x30, source + 0x30])
        .u8()
        != 0
    {
        return true;
    }
    e.call(MEMCMP, &args![base + 0x50, source + 0x50, 6u32])
        .u32()
        != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_4340, fn_00824340(Ptr) -> Ptr),
        entry!(0x0052_b260, fn_0052b260(Ptr<TESClimate>, u8)),
        entry!(0x0052_b2c0, fn_0052b2c0(Ptr<TESClimate>, u8)),
        entry!(0x0052_b0f0, tes_climate_make_default(Ptr<TESClimate>)),
        entry!(0x0052_ac90, tes_climate_load(Ptr<TESClimate>, Ptr) -> bool),
        entry!(0x0052_ae80, tes_climate_save(Ptr<TESClimate>)),
        entry!(0x0052_af10, fn_0052af10(Ptr<TESClimate>, Ptr)),
        entry!(0x0052_afe0, fn_0052afe0(Ptr<TESClimate>, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn fn_00824340_clears_the_entry() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        assert_eq!(e.call(0x0082_4340, &args![this]).u32(), this);
        assert_eq!(e.mem.bytes(this, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(this + 0xc), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_0052b260_and_0052b2c0_set_and_clear_their_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.mem.set_u8(this + 0x55, 0x15);
        e.call(0x0052_b260, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x55), 0x95);
        e.call(0x0052_b2c0, &args![this, 0x100u32 + 2]);
        assert_eq!(e.mem.u8(this + 0x55), 0xd5);
        e.call(0x0052_b260, &args![this, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x55), 0x55);
        e.call(0x0052_b2c0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x55), 0x15);
    }

    fn default_rig(found: u32, block_ok: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(SET_STRING_00489100, |_, _| Ret::default());
        e.put_vtable(0x0f00_a000, &[0; 7]);
        e.mem.set_u32(0x0f00_a018, 0x0f00_a100);
        e.register(0x0f00_a100, |_, _| Ret::default());
        e.register(SET_LOW_BITS_0052AC50, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL_00470470, |_, _| Ret::default());
        let block = e.mem.alloc(0xc);
        e.mem.write(block, &[0xaa; 0xc]);
        e.register_double(ALLOCATE, move |_, _| {
            if block_ok { block } else { 0 }.into_ret()
        });
        e.register_double(LOOKUP_FORM, move |_, _| found.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(LIST_ADD_HEAD_005AE3D0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(LOG, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x18, 0x0f00_a000);
        e.call_log = Some(vec![]);
        (e, this, block)
    }

    #[test]
    fn fn_0052b0f0_makes_the_default_climate() {
        let (mut e, this, block) = default_rig(0x0d00_0000, true);
        e.call(0x0052_b0f0, &args![this]);
        assert_eq!(
            calls(&e, SET_STRING_00489100),
            vec![
                vec![this + 0x38, SUN_NAME],
                vec![this + 0x44, SUN_GLARE_NAME]
            ]
        );
        assert_eq!(calls(&e, 0x0f00_a100), vec![vec![this + 0x18, STARS_NAME]]);
        assert_eq!(
            e.mem.bytes(this + 0x50, 6),
            vec![0x24, 0x2a, 0x6c, 0x72, 0, 0xc0]
        );
        assert_eq!(calls(&e, SET_LOW_BITS_0052AC50), vec![vec![this, 3]]);
        assert_eq!(calls(&e, LIST_REMOVE_ALL_00470470), vec![vec![this + 0x30]]);
        assert_eq!(calls(&e, LOOKUP_FORM), vec![vec![DEFAULT_WEATHER_ID]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![0x0d00_0000, 0, TYPE_TES_FORM, TYPE_TES_WEATHER, 0]]
        );
        assert_eq!(e.mem.u32(block), 0x0d00_0000);
        assert_eq!(e.mem.u32(block + 4), 100);
        assert_eq!(e.mem.u32(block + 8), 0);
        let added = calls(&e, LIST_ADD_HEAD_005AE3D0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], this + 0x30);
        assert!(calls(&e, LOG).is_empty());
    }

    #[test]
    fn fn_0052b0f0_without_the_weather_deletes_the_entry() {
        let (mut e, this, block) = default_rig(0, true);
        e.call(0x0052_b0f0, &args![this]);
        assert!(calls(&e, LIST_ADD_HEAD_005AE3D0).is_empty());
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![block]]);
        assert_eq!(calls(&e, LOG), vec![vec![MESSAGE_NO_DEFAULT_WEATHER]]);
    }

    #[test]
    fn fn_0052b0f0_with_a_failed_allocation_writes_at_null() {
        let (mut e, this, _) = default_rig(0x0d00_0000, false);
        e.map(0, 0x10);
        e.call(0x0052_b0f0, &args![this]);
        assert_eq!(e.mem.u32(0), 0x0d00_0000);
        assert_eq!(e.mem.u32(4), 100);
        assert_eq!(calls(&e, LIST_ADD_HEAD_005AE3D0).len(), 1);
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    const FILE: u32 = 0x0e00_0000;

    fn load_rig(chunks: Vec<(u32, u32)>, warnings: [u32; 2]) -> (Engine, u32) {
        let mut e = Engine::new();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x36u32.into_ret());
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
        let mut counts = warnings.to_vec();
        e.register_double(WARNING_COUNT_0046E8A0, move |_, _| {
            counts.remove(0).into_ret()
        });
        for addr in [
            GET_CHUNK_DATA_SIZED,
            LOAD_TEXTURE_CHUNK,
            LOAD_MODEL_CHUNK,
            INC_DISABLE_WARNING_COUNT,
            LOAD_WEATHER_LIST_CHUNK,
            LOG,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.put_vtable(0x0f00_b000, &[0; 0x4e]);
        e.mem.set_u32(0x0f00_b0e0, 0x0f00_b100);
        e.mem.set_u32(0x0f00_b130, 0x0f00_b200);
        e.mem.set_u32(0x0f00_b134, 0x0f00_b300);
        e.register(0x0f00_b100, |_, _| Ret::default());
        e.register(0x0f00_b200, |_, _| 0x0abc_0000u32.into_ret());
        e.register(0x0f00_b300, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this, 0x0f00_b000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_0052ac90_refuses_other_records() {
        let (mut e, this) = load_rig(vec![], [0, 0]);
        e.register(GET_TES_FORM, |_, _| 0x35u32.into_ret());
        assert!(!e.call(0x0052_ac90, &args![this, FILE]).bool());
        assert!(calls(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_0052ac90_reads_each_chunk() {
        let chunks = vec![
            (CHUNK_EDID as u32, 7),
            (CHUNK_OBND as u32, 0),
            (CHUNK_MODL as u32, 0),
            (CHUNK_FNAM as u32, 0),
            (0x4d41_4e47, 0),
            (0x4d41_4e48, 0),
            (CHUNK_PNAM as u32, 0),
            (CHUNK_TNAM as u32, 6),
            (CHUNK_WLST as u32, 0x18),
            (0x8000_0001, 0),
        ];
        let (mut e, this) = load_rig(chunks, [4, 4]);
        assert!(e.call(0x0052_ac90, &args![this, FILE]).bool());
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 0]]);
        let edid = calls(&e, 0x0f00_b300);
        assert_eq!(edid.len(), 1);
        let reads = calls(&e, GET_CHUNK_DATA_SIZED);
        assert_eq!(
            reads,
            vec![vec![FILE, edid[0][1], 0x200], vec![FILE, this + 0x50, 6]]
        );
        assert_eq!(calls(&e, 0x0f00_b100), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, LOAD_MODEL_CHUNK), vec![vec![this + 0x18, FILE]]);
        assert_eq!(
            calls(&e, LOAD_TEXTURE_CHUNK),
            vec![vec![this + 0x38, FILE], vec![this + 0x44, FILE]]
        );
        assert_eq!(
            calls(&e, LOAD_WEATHER_LIST_CHUNK),
            vec![vec![this + 0x30, FILE, this]]
        );
        assert_eq!(calls(&e, INC_DISABLE_WARNING_COUNT), vec![vec![1], vec![0]]);
        assert!(calls(&e, LOG).is_empty());
        assert_eq!(calls(&e, NEXT_CHUNK).len(), 10);
    }

    #[test]
    fn fn_0052ac90_reports_new_weather_list_warnings() {
        let (mut e, this) = load_rig(vec![(CHUNK_WLST as u32, 0xc)], [2, 3]);
        assert!(e.call(0x0052_ac90, &args![this, FILE]).bool());
        assert_eq!(
            calls(&e, LOG),
            vec![vec![MESSAGE_WEATHER_LIST_WARNINGS, 0x0abc_0000]]
        );
        // Counts compared signed: -1 before, 0 after is growth too.
        let (mut e, this) = load_rig(vec![(CHUNK_WLST as u32, 0xc)], [u32::MAX, 0]);
        assert!(e.call(0x0052_ac90, &args![this, FILE]).bool());
        assert_eq!(calls(&e, LOG).len(), 1);
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    const STATE: u32 = 0x7300_0000;
    const SLOT_COPY: u32 = 0x7100_0008;
    const SLOT_COMPARE: u32 = 0x7100_000c;

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
            FORM_CLOSE,
            FORM_ADD_CHUNK_ARRAY,
            WEATHER_LIST_SAVE,
            TEXTURE_SAVE,
            MODEL_SAVE,
            FORM_COPY_ALL_COMPONENTS,
            WEATHER_LIST_COPY,
            COPY_BYTES,
            SLOT_COPY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e
    }

    /// A climate whose components' vtables answer through the two slots.
    fn climate(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, SLOT_COPY);
        e.mem.set_u32(vtable + 0xc, SLOT_COMPARE);
        let this = e.mem.alloc(0x60);
        for offset in [0x18, 0x38, 0x44] {
            e.mem.set_u32(this + offset, vtable);
        }
        this
    }

    #[test]
    fn save_writes_every_part() {
        let mut e = engine();
        let this = 0x1000u32;
        e.call_log = Some(vec![]);
        e.call(0x0052_ae80, &args![this]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (FORM_START, vec![this]),
                (WEATHER_LIST_SAVE, vec![0x1030, 0x5453_4c57]),
                (TEXTURE_SAVE, vec![0x1038, 0x4d41_4e46]),
                (TEXTURE_SAVE, vec![0x1044, 0x4d41_4e47]),
                (MODEL_SAVE, vec![0x1018, 0x4c44_4f4d, 0x5444_4f4d]),
                (FORM_ADD_CHUNK_ARRAY, vec![0x4d41_4e54, 0x1050, 6]),
                (FORM_CLOSE, vec![this]),
            ]
        );
    }

    #[test]
    fn copy_takes_every_part() {
        let mut e = engine();
        let this = climate(&mut e);
        let other = 0x9000u32;
        e.call_log = Some(vec![]);
        e.call(0x0052_af10, &args![this, other]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![other, 0, TYPE_TES_FORM, TYPE_TES_CLIMATE, 0]]
        );
        assert_eq!(calls(&e, FORM_COPY_ALL_COMPONENTS), vec![vec![this, other]]);
        assert_eq!(
            calls(&e, WEATHER_LIST_COPY),
            vec![vec![this + 0x30, other + 0x30, 0]]
        );
        assert_eq!(
            calls(&e, SLOT_COPY),
            vec![
                vec![this + 0x38, other + 0x38],
                vec![this + 0x44, other + 0x44],
                vec![this + 0x18, other + 0x18]
            ]
        );
        assert_eq!(
            calls(&e, COPY_BYTES),
            vec![vec![this + 0x50, other + 0x50, 6]]
        );
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0052_af10, &args![this, other]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn compare_branches() {
        let mut e = engine();
        // STATE: CompareAllComponents; +4: the offset whose slot answers 1;
        // +8: the weather lists; +0xC: memcmp.
        e.register(FORM_COMPARE_ALL_COMPONENTS, |e, _| {
            e.mem.u32(STATE).into_ret()
        });
        e.register_double(SLOT_COMPARE, |e, a| {
            u32::from(a[1] & 0xff == e.mem.u32(STATE + 4)).into_ret()
        });
        e.register(WEATHER_LIST_COMPARE, |e, _| e.mem.u32(STATE + 8).into_ret());
        e.register(MEMCMP, |e, _| e.mem.u32(STATE + 0xc).into_ret());
        let this = climate(&mut e);
        let other = 0x9000u32;
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0052_afe0, &args![this, other]).bool());
        assert_eq!(calls(&e, MEMCMP), vec![vec![this + 0x50, other + 0x50, 6]]);
        for offset in [0x38, 0x44, 0x18] {
            e.mem.set_u32(STATE + 4, offset);
            assert!(e.call(0x0052_afe0, &args![this, other]).bool());
        }
        e.mem.set_u32(STATE + 4, 0);
        for at in [STATE, STATE + 8, STATE + 0xc] {
            e.mem.set_u32(at, 1);
            assert!(e.call(0x0052_afe0, &args![this, other]).bool());
            e.mem.set_u32(at, 0);
        }
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0052_afe0, &args![this, other]).bool());
    }
}
