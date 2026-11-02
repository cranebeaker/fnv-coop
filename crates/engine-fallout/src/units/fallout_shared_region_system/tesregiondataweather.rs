//! `fallout shared/region system/tesregiondataweather.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f50b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather` scalar deleting destructor (engine map name): runs the destructor body `004f50e0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_weather_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_50e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtable of `TESRegionDataWeather` (RTTI `.?AVTESRegionDataWeather@@`, read at the word before it).
const TES_REGION_DATA_WEATHER_VTABLE: u32 = 0x0102_3e18;
/// `00582320` (unnamed in the engine map): the destructor of the weather list at +8.
const WEATHER_LIST_DTOR_00582320: u32 = 0x0058_2320;
/// The `TESRegionData` destructor body (`004f14a0`).
const TES_REGION_DATA_DTOR: u32 = 0x004f_14a0;
/// `0046e8a0` (unnamed in the engine map; no arguments): a count the
/// loader compares before and after reading the chunk (the warnings so far).
const WARNING_COUNT_0046E8A0: u32 = 0x0046_e8a0;
/// `MessageHandler::IncDisableWarningCount` (`0043b2b0`, engine map; cdecl, one word).
const INC_DISABLE_WARNING_COUNT: u32 = 0x0043_b2b0;
/// `TESWeatherList::LoadWeatherListChunk` (`00582940`, engine map; cdecl,
/// the list, the file and the form).
const LOAD_WEATHER_LIST_CHUNK: u32 = 0x0058_2940;
/// The virtual slot (byte offset) of the form the warning names (its
/// identifying text, passed to the format).
const SLOT_FORM_TEXT: u32 = 0x130;
/// `005b5e40` (unnamed in the engine map; cdecl, a format and its
/// arguments): prints the warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// `"Warnings were encountered while loading weather list chunk from ..."`
/// (the format, with the form's text).
const WEATHER_WARNING_FORMAT: u32 = 0x0102_3e40;

// Translated from 004f50e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f50e0 - its FS:[0] exception frame (the unwind state that runs the base destructor when the list's destructor throws) is not translated
/// `TESRegionDataWeather` destructor body (unnamed in the engine map):
/// stores the class's vtable, destroys the weather list at +8 (`00582320`),
/// then runs the `TESRegionData` destructor body.
pub fn fn_004f50e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_REGION_DATA_WEATHER_VTABLE);
    e.call(WEATHER_LIST_DTOR_00582320, &args![this.addr() + 8]);
    e.call(TES_REGION_DATA_DTOR, &args![this]);
}

// Translated from 004f51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather::LoadRegionDataWeather` (Xbox PDB; `thiscall`,
/// `RET 8`): reads the weather list chunk of `file` into the list at +8
/// (`TESWeatherList::LoadWeatherListChunk(list, file, form)`) between
/// `IncDisableWarningCount(1)` and `(0)`. When the count `0046e8a0` grew
/// (signed comparison) and `form` is set, it prints the weather-list warning
/// with the text of the form's virtual slot +0x130.
pub fn tes_region_data_weather_load_region_data_weather(
    e: &mut Engine,
    this: Ptr,
    file: Ptr,
    form: Ptr,
) {
    let before = e.call(WARNING_COUNT_0046E8A0, &args![]).i32();
    e.call(INC_DISABLE_WARNING_COUNT, &args![1u32]);
    e.call(LOAD_WEATHER_LIST_CHUNK, &args![this.addr() + 8, file, form]);
    e.call(INC_DISABLE_WARNING_COUNT, &args![0u32]);
    let after = e.call(WARNING_COUNT_0046E8A0, &args![]).i32();
    if after > before && !form.is_null() {
        let text = e.vcall(form.addr(), SLOT_FORM_TEXT, &args![]).u32();
        e.call(PRINT_WARNING_005B5E40, &args![WEATHER_WARNING_FORMAT, text]);
    }
}

/// The `TESRegionData` copy constructor (`004f1460`, `thiscall`, `RET 4`).
const TES_REGION_DATA_COPY_CTOR: u32 = 0x004f_1460;
/// `TESRegionData::Save` (`004f14c0`, engine map; no stack word).
const TES_REGION_DATA_SAVE: u32 = 0x004f_14c0;
/// `005822c0` (unnamed in the engine map; no stack word): constructs the
/// weather list at +8.
const WEATHER_LIST_CTOR_005822C0: u32 = 0x0058_22c0;
/// `005825a0` (unnamed in the engine map; `thiscall`, `RET 8`): merges the
/// weather list given into this one (the second word: 0 replaces, 1 blends).
const WEATHER_LIST_MERGE_005825A0: u32 = 0x0058_25a0;
/// `TESWeatherList::Save` (`00582380`, engine map; `thiscall`, `RET 4`: the chunk type).
const WEATHER_LIST_SAVE: u32 = 0x0058_2380;
/// `008256d0` (unnamed in the engine map; no stack word): a byte answer
/// about the weather list (non-zero makes `Validate` false).
const WEATHER_LIST_TEST_008256D0: u32 = 0x0082_56d0;
/// `004f1540` (unnamed in the engine map; no stack word): `TESRegionData::bOverride` (+4).
const REGION_DATA_GET_OVERRIDE: u32 = 0x004f_1540;
/// `004f15a0` (unnamed in the engine map; `RET 4`): stores its byte in `bOverride`.
const REGION_DATA_SET_OVERRIDE: u32 = 0x004f_15a0;
/// `004f15c0` (unnamed in the engine map; `RET 4`): stores its byte in
/// `cPriority` when it is at most 100.
const REGION_DATA_SET_PRIORITY: u32 = 0x004f_15c0;
/// `005bb4d0` (unnamed in the engine map; no stack word): the byte at +6
/// (`TESRegionData::cPriority`, Xbox PDB).
const REGION_DATA_GET_PRIORITY: u32 = 0x005b_b4d0;
/// `00406cc0` (unnamed in the engine map; cdecl, a `float`): returns a value in ST0.
const FLOAT_HELPER_00406CC0: u32 = 0x0040_6cc0;
/// `0.005` (`double`).
const HALF_PERCENT: u32 = 0x0102_3b88;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `RDWT` (as a little-endian word), the chunk type of the weather list.
const CHUNK_RDWT: u32 = 0x5457_4452;
/// `TESRegionData::GetID` (Xbox PDB slot +0x10).
const SLOT_GET_ID: u32 = 0x10;
/// `TESRegionData::Copy` (Xbox PDB slot +0x14).
const SLOT_COPY: u32 = 0x14;
/// `TESRegionData::BlendInto` (Xbox PDB slot +0x1C).
const SLOT_BLEND_INTO: u32 = 0x1c;
/// The `GetID` of weather region data.
const WEATHER_ID: u32 = 3;

// Translated from 004f5140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f5140 - its FS:[0] exception frame (the unwind states that destroy the list and the base when a call throws) is not translated
/// `TESRegionDataWeather` copy constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the `TESRegionData` copy constructor with
/// `source`, the class's vtable, the weather list at +8 constructed
/// (`005822c0`) and given the source's list (`005825a0` with 0). Returns
/// `this`.
pub fn fn_004f5140(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_COPY_CTOR, &args![this, source]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_WEATHER_VTABLE);
    e.call(WEATHER_LIST_CTOR_005822C0, &args![this.addr() + 8]);
    e.call(
        WEATHER_LIST_MERGE_005825A0,
        &args![this.addr() + 8, source.addr() + 8, 0u32],
    );
    this
}

// Translated from 004f51c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather::Save` (Xbox PDB): runs `TESRegionData::Save`,
/// then saves the weather list at +8 as an `RDWT` chunk.
pub fn tes_region_data_weather_save(e: &mut Engine, this: Ptr) {
    e.call(TES_REGION_DATA_SAVE, &args![this]);
    e.call(WEATHER_LIST_SAVE, &args![this.addr() + 8, CHUNK_RDWT]);
}

// Translated from 004f5260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f5260 - its FS:[0] exception frame (the unwind state that frees the block when the copy constructor throws) is not translated
/// `TESRegionDataWeather::Copy` (Xbox PDB): allocates 0x10 bytes and, when
/// that succeeds, copy-constructs a `TESRegionDataWeather` of `this` in
/// them (`004f5140`). Returns the copy, or null.
pub fn tes_region_data_weather_copy(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_004f5140(e, Ptr::new(block), this)
}

// Translated from 004f52e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather::Blend` (Xbox PDB; `thiscall`, `RET 4`): a copy of
/// this data (`Copy`, slot +0x14) with `other` blended into it (`BlendInto`,
/// slot +0x1C, with 2). Returns the copy.
pub fn tes_region_data_weather_blend(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let copy = e.vcall(this.addr(), SLOT_COPY, &args![]).u32();
    e.vcall(copy, SLOT_BLEND_INTO, &args![other, 2u32]);
    Ptr::new(copy)
}

/// The tail the "take the source" branches of `BlendInto` share: the
/// source's `bOverride` and priority, then its weather list (replacing).
fn weather_take_source(e: &mut Engine, this: Ptr, source: Ptr) {
    let flag = e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8();
    e.call(REGION_DATA_SET_OVERRIDE, &args![this, flag as u32]);
    let priority = e.call(REGION_DATA_GET_PRIORITY, &args![source]).u8();
    e.call(REGION_DATA_SET_PRIORITY, &args![this, priority as u32]);
    e.call(
        WEATHER_LIST_MERGE_005825A0,
        &args![this.addr() + 8, source.addr() + 8, 0u32],
    );
}

/// `005bb4d0` on an object, as the `MOVZX` the exe does.
fn weather_priority(e: &mut Engine, data: Ptr) -> i32 {
    e.call(REGION_DATA_GET_PRIORITY, &args![data]).u8() as i32
}

// Translated from 004f5320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather::BlendInto` (Xbox PDB; `thiscall`, `RET 8`): merges
/// weather data `source` (its `GetID` must be 3, and `enabled` nonzero)
/// into this one. When this data's `bIgnore` (+5) is set, the source is
/// taken (its `bOverride`, priority and weather list, `005825a0` with 0).
/// Otherwise nothing happens when the source's `bIgnore` is set. With this
/// data's `bOverride` (+4) set, the source is taken only when its
/// `bOverride` is set and its priority is higher. Otherwise the source is
/// taken when its `bOverride` is set; else the lists are blended
/// (`005825a0` with 1) and the priority becomes
/// `trunc(00406cc0((this*this + (100-this)*src + (100-src)*this + src*src) * 0.005))`
/// (each priority read anew, in the exe's order).
pub fn tes_region_data_weather_blend_into(e: &mut Engine, this: Ptr, source: Ptr, enabled: u32) {
    if source.is_null()
        || e.vcall(source.addr(), SLOT_GET_ID, &args![]).u32() != WEATHER_ID
        || enabled == 0
    {
        return;
    }
    // `TESRegionData::bIgnore` (Xbox PDB) +0x05.
    if e.mem.u8(this.addr() + 5) != 0 {
        weather_take_source(e, this, source);
        return;
    }
    if e.mem.u8(source.addr() + 5) != 0 {
        return;
    }
    // `TESRegionData::bOverride` (Xbox PDB) +0x04.
    if e.mem.u8(this.addr() + 4) != 0 {
        if e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8() != 0
            && weather_priority(e, source) > weather_priority(e, this)
        {
            weather_take_source(e, this, source);
        }
        return;
    }
    if e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8() != 0 {
        weather_take_source(e, this, source);
        return;
    }
    e.call(
        WEATHER_LIST_MERGE_005825A0,
        &args![this.addr() + 8, source.addr() + 8, 1u32],
    );
    let a = weather_priority(e, this);
    let b = weather_priority(e, this);
    let c = weather_priority(e, this);
    let d = weather_priority(e, source);
    let first = a * b + (100 - c) * d;
    let mut total = x87_store32(f64::from(first));
    let f = weather_priority(e, source);
    let g = weather_priority(e, this);
    let h = weather_priority(e, source);
    let i = weather_priority(e, source);
    let second = (100 - f) * g + h * i;
    total = x87_store32(x87_add(f64::from(second), x87_m32(total)));
    let half_percent: f64 = e.global(HALF_PERCENT);
    total = x87_store32(x87_mul(x87_load32(total), half_percent));
    let rounded = e.call(FLOAT_HELPER_00406CC0, &args![total]).f64();
    let value = crate::x87::fistp_chop(rounded) as u8;
    e.call(REGION_DATA_SET_PRIORITY, &args![this, value as u32]);
}

// Translated from 004f5570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataWeather::Validate` (Xbox PDB): 1 when `008256d0` on the
/// weather list at +8 answers 0, else 0 (the whole word).
pub fn tes_region_data_weather_validate(e: &mut Engine, this: Ptr) -> u32 {
    let answer = e
        .call(WEATHER_LIST_TEST_008256D0, &args![this.addr() + 8])
        .u8();
    u32::from(answer == 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_5140, fn_004f5140(Ptr, Ptr) -> Ptr),
        entry!(0x004f_51c0, tes_region_data_weather_save(Ptr)),
        entry!(0x004f_5260, tes_region_data_weather_copy(Ptr) -> Ptr),
        entry!(0x004f_52e0, tes_region_data_weather_blend(Ptr, Ptr) -> Ptr),
        entry!(
            0x004f_5320,
            tes_region_data_weather_blend_into(Ptr, Ptr, u32)
        ),
        entry!(0x004f_5570, tes_region_data_weather_validate(Ptr) -> u32),
        entry!(0x004f_50b0, tes_region_data_weather_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_50e0, fn_004f50e0(Ptr)),
        entry!(
            0x004f_51f0,
            tes_region_data_weather_load_region_data_weather(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod weather_tests {
    use super::*;

    #[test]
    fn at_004f50e0_destroys_the_list_then_the_base() {
        let mut e = Engine::new();
        e.register(WEATHER_LIST_DTOR_00582320, |_, _| Ret::default());
        e.register(TES_REGION_DATA_DTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_50e0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_WEATHER_VTABLE);
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x004f_50e0).collect();
        assert_eq!(
            calls,
            vec![
                (WEATHER_LIST_DTOR_00582320, vec![this + 8]),
                (TES_REGION_DATA_DTOR, vec![this]),
            ]
        );
    }

    fn run(grows: bool, with_form: bool) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        let mut count = 0i32;
        e.register_double(WARNING_COUNT_0046E8A0, move |_, _| {
            let value = count;
            if grows {
                count += 1;
            }
            value.into_ret()
        });
        e.register(INC_DISABLE_WARNING_COUNT, |_, _| Ret::default());
        e.register(LOAD_WEATHER_LIST_CHUNK, |_, _| Ret::default());
        e.register(PRINT_WARNING_005B5E40, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x4d];
        slots[0x4c] = 0x0f02_1000;
        e.put_vtable(0x0f02_0000, &slots);
        e.register(0x0f02_1000, |_, _| 0x0bad_f00du32.into_ret());
        let form = if with_form {
            let form = e.mem.alloc(0x20);
            e.mem.set_u32(form, 0x0f02_0000);
            form
        } else {
            0
        };
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_51f0, &args![this, 0x0123_4567u32, form]);
        let log = e.call_log.take().unwrap();
        let load = log.iter().find(|c| c.0 == LOAD_WEATHER_LIST_CHUNK).unwrap();
        assert_eq!(load.1, vec![this + 8, 0x0123_4567, form]);
        let disables: Vec<_> = log
            .iter()
            .filter(|c| c.0 == INC_DISABLE_WARNING_COUNT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(disables, vec![vec![1], vec![0]]);
        log
    }

    #[test]
    fn at_004f51f0_warns_when_warnings_grew_for_a_form() {
        let log = run(true, true);
        let print = log.iter().find(|c| c.0 == PRINT_WARNING_005B5E40).unwrap();
        assert_eq!(print.1, vec![WEATHER_WARNING_FORMAT, 0x0bad_f00d]);
    }

    #[test]
    fn at_004f51f0_stays_quiet_otherwise() {
        for (grows, with_form) in [(false, true), (true, false), (false, false)] {
            let log = run(grows, with_form);
            assert!(log.iter().all(|c| c.0 != PRINT_WARNING_005B5E40));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f50b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_50e0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_50b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_50b0)
                .collect();
            let mut expected = vec![(0x004f_50e0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_004f5140_copies_the_base_and_the_list() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_COPY_CTOR, |_, _| Ret::default());
        e.register(WEATHER_LIST_CTOR_005822C0, |_, _| Ret::default());
        e.register(WEATHER_LIST_MERGE_005825A0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_5140, &args![this, source]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_WEATHER_VTABLE);
        assert_eq!(
            calls_to(&log, TES_REGION_DATA_COPY_CTOR),
            vec![vec![this, source]]
        );
        assert_eq!(
            calls_to(&log, WEATHER_LIST_CTOR_005822C0),
            vec![vec![this + 8]]
        );
        assert_eq!(
            calls_to(&log, WEATHER_LIST_MERGE_005825A0),
            vec![vec![this + 8, source + 8, 0]]
        );
    }

    #[test]
    fn save_saves_the_base_then_the_list() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_SAVE, |_, _| Ret::default());
        e.register(WEATHER_LIST_SAVE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_51c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (TES_REGION_DATA_SAVE, vec![this]),
                (WEATHER_LIST_SAVE, vec![this + 8, CHUNK_RDWT]),
            ]
        );
    }

    #[test]
    fn copy_allocates_and_copy_constructs() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| ret(0));
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x004f_5260, &args![this]).u32(), 0);
        let block = e.mem.alloc(0x10);
        e.register_double(ALLOCATE, move |_, _| ret(block));
        e.register(TES_REGION_DATA_COPY_CTOR, |_, _| Ret::default());
        e.register(WEATHER_LIST_CTOR_005822C0, |_, _| Ret::default());
        e.register(WEATHER_LIST_MERGE_005825A0, |_, _| Ret::default());
        assert_eq!(e.call(0x004f_5260, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block), TES_REGION_DATA_WEATHER_VTABLE);
    }

    #[test]
    fn blend_copies_then_blends_the_other_in() {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..8).map(|i| 0x0150_1000 + 4 * i).collect();
        e.put_vtable(0x0150_0000, &slots);
        let copy = e.mem.alloc(0x10);
        e.mem.set_u32(copy, 0x0150_0000);
        e.register_double(0x0150_1014, move |_, _| ret(copy));
        e.register(0x0150_101c, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0150_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_52e0, &args![this, 0x7777u32]).u32(), copy);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0150_1014), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x0150_101c), vec![vec![copy, 0x7777, 2]]);
    }

    /// A target and a source of GetID 3.
    fn blend_setup(e: &mut Engine) -> (u32, u32) {
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1010]);
        e.register(0x0150_1010, |_, _| ret(WEATHER_ID));
        e.register(REGION_DATA_GET_OVERRIDE, |e, a| {
            ret(e.mem.u8(a[0] + 4) as u32)
        });
        e.register(REGION_DATA_GET_PRIORITY, |e, a| {
            ret(e.mem.u8(a[0] + 6) as u32)
        });
        e.register(REGION_DATA_SET_OVERRIDE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e.register(REGION_DATA_SET_PRIORITY, |e, a| {
            e.mem.set_u8(a[0] + 6, a[1] as u8);
            Ret::default()
        });
        e.register(WEATHER_LIST_MERGE_005825A0, |_, _| Ret::default());
        e.map(HALF_PERCENT & !0xfff, 0x1000);
        e.set_global(HALF_PERCENT, 0.005f64);
        e.register(FLOAT_HELPER_00406CC0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Default::default()
        });
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0150_0000);
        e.mem.set_u32(source, 0x0150_0000);
        (this, source)
    }

    #[test]
    fn blend_into_takes_or_skips_the_source() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x004f_5320, &args![this, source, 0u32]);
        e.call(0x004f_5320, &args![this, 0u32, 1u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), WEATHER_LIST_MERGE_005825A0).is_empty());
        // This ignored: taken.
        e.mem.set_u8(this + 5, 1);
        e.mem.set_u8(source + 4, 1);
        e.mem.set_u8(source + 6, 70);
        e.call_log = Some(vec![]);
        e.call(0x004f_5320, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), WEATHER_LIST_MERGE_005825A0),
            vec![vec![this + 8, source + 8, 0]]
        );
        assert_eq!(e.mem.u8(this + 6), 70);
        // Source ignored: nothing.
        e.mem.set_u8(this + 5, 0);
        e.mem.set_u8(source + 5, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_5320, &args![this, source, 1u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), WEATHER_LIST_MERGE_005825A0).is_empty());
        // This overrides (from the take above): needs a higher overriding source.
        e.mem.set_u8(source + 5, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_5320, &args![this, source, 1u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), WEATHER_LIST_MERGE_005825A0).is_empty());
        e.mem.set_u8(source + 6, 71);
        e.call_log = Some(vec![]);
        e.call(0x004f_5320, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), WEATHER_LIST_MERGE_005825A0).len(),
            1
        );
        // Neither overrides but the source does: taken.
        e.mem.set_u8(this + 4, 0);
        e.mem.set_u8(source + 6, 10);
        e.call(0x004f_5320, &args![this, source, 1u32]);
        assert_eq!(e.mem.u8(this + 6), 10);
    }

    #[test]
    fn blend_into_blends_the_lists_and_the_priority() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        e.mem.set_u8(this + 6, 40);
        e.mem.set_u8(source + 6, 80);
        e.call_log = Some(vec![]);
        e.call(0x004f_5320, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), WEATHER_LIST_MERGE_005825A0),
            vec![vec![this + 8, source + 8, 1]]
        );
        assert_eq!(e.mem.u8(this + 6), 68);
    }

    #[test]
    fn validate_is_one_when_the_list_answers_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(WEATHER_LIST_TEST_008256D0, |_, _| ret(0xff00));
        assert_eq!(e.call(0x004f_5570, &args![this]).u32(), 1);
        e.register(WEATHER_LIST_TEST_008256D0, |_, _| ret(2));
        assert_eq!(e.call(0x004f_5570, &args![this]).u32(), 0);
    }
}
