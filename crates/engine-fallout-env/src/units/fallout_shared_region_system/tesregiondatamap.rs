//! `fallout shared/region system/tesregiondatamap.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f3d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap` scalar deleting destructor (engine map name): runs the destructor body `004f3de0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_3de0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESRegionData::TESRegionData` (`004f1400`).
const TES_REGION_DATA_CTOR: u32 = 0x004f_1400;
/// The `TESRegionData` copy constructor (`004f1460`, `thiscall`, the source).
const TES_REGION_DATA_COPY_CTOR: u32 = 0x004f_1460;
/// The `TESRegionData` destructor body (`004f14a0`).
const TES_REGION_DATA_DTOR: u32 = 0x004f_14a0;
/// The vtable of `TESRegionDataMap` (RTTI `.?AVTESRegionDataMap@@`, read at the word before it).
const TES_REGION_DATA_MAP_VTABLE: u32 = 0x0102_3d28;
/// `"Default Region Name"`.
const DEFAULT_REGION_NAME: u32 = 0x0102_3d10;
/// `004037b0` (unnamed in the engine map): the `BSStringT<char>` constructor.
const STRING_CTOR: u32 = 0x0040_37b0;
/// `004037d0` (unnamed in the engine map): the `BSStringT<char>` destructor.
const STRING_DTOR: u32 = 0x0040_37d0;
/// `004037f0` (unnamed in the engine map; `thiscall`, `RET 8`): sets the
/// string from a C string and a length (0 here).
const STRING_SET: u32 = 0x0040_37f0;
/// `00559450` (unnamed in the engine map; no stack words): the string's
/// characters.
const STRING_CHARS: u32 = 0x0055_9450;
/// `BSStringT<char>::StrCmp` (`00408a80`, engine map; `thiscall`, `RET 8`).
const STRING_COMPARE: u32 = 0x0040_8a80;
/// The virtual slot (byte offset) `GetMapName` (Xbox PDB slot +0x2C) the
/// copy constructor reads the source's name through.
const SLOT_GET_MAP_NAME: u32 = 0x2c;
/// `TESFile::GetTESChunk` (`004726b0`, Xbox PDB).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (unnamed in the engine map): the file's word at +0x25C (the
/// current chunk's size).
const CHUNK_SIZE_00401660: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, Xbox PDB; `thiscall`, buffer and size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `RDMP` (as a little-endian word), the chunk type of the map name.
const CHUNK_RDMP: u32 = 0x504d_4452;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;

// Translated from 004f3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3ca0 - its FS:[0] exception frame (the unwind states that destroy the base and the name) is not translated
/// `TESRegionDataMap::TESRegionDataMap` (Xbox PDB): runs the `TESRegionData`
/// constructor, stores the class's vtable, constructs the name string at +8
/// (`MapName`, Xbox PDB) and sets it to `"Default Region Name"`. Returns
/// `this`.
pub fn tes_region_data_map_tes_region_data_map(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_MAP_VTABLE);
    e.call(STRING_CTOR, &args![this.addr() + 8]);
    e.call(
        STRING_SET,
        &args![this.addr() + 8, DEFAULT_REGION_NAME, 0u32],
    );
    this
}

// Translated from 004f3d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3d50 - its FS:[0] exception frame (the unwind states that destroy the base and the name) is not translated
/// `TESRegionDataMap` copy constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): runs the `TESRegionData` copy constructor with
/// `source`, stores the class's vtable, constructs the name string at +8,
/// then sets it (`SetMapName`, `004f3f40`) to the name the source's virtual
/// slot +0x2C returns. Returns `this`.
pub fn fn_004f3d50(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_COPY_CTOR, &args![this, source]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_MAP_VTABLE);
    e.call(STRING_CTOR, &args![this.addr() + 8]);
    let name = e.vcall(source.addr(), SLOT_GET_MAP_NAME, &args![]).u32();
    tes_region_data_map_set_map_name(e, this, name);
    this
}

// Translated from 004f3de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3de0 - its FS:[0] exception frame (the unwind state that runs the base destructor when the name's destructor throws) is not translated
/// `TESRegionDataMap` destructor body (unnamed in the engine map): stores
/// the class's vtable, destroys the name string at +8, then runs the
/// `TESRegionData` destructor body.
pub fn fn_004f3de0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_REGION_DATA_MAP_VTABLE);
    e.call(STRING_DTOR, &args![this.addr() + 8]);
    e.call(TES_REGION_DATA_DTOR, &args![this]);
}

// Translated from 004f3e90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3e90 - the /GS cookie check (00ec408c) of its stack frame is not translated
/// `TESRegionDataMap::LoadRegionDataMap` (Xbox PDB; `thiscall`, `RET 4`):
/// false for a null `file` or when the current chunk is not `RDMP`.
/// Otherwise the chunk is read (`GetChunkData_ov3` with size 0) into a
/// buffer of the chunk's size (`00401660`) that the exe makes on its stack
/// (`_alloca`, the probe `00ec5ec0`; here the engine's stack block), the
/// name is set from it (length 0) and true is returned.
pub fn tes_region_data_map_load_region_data_map(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if file.is_null() || e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_RDMP {
        return false;
    }
    let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
    e.with_stack(size, |e, buffer| {
        e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
        e.call(STRING_SET, &args![this.addr() + 8, buffer, 0u32]);
    });
    true
}

// Translated from 004f3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap::GetMapName` (Xbox PDB; `thiscall`, `RET 4`): sets the
/// string `out` to the characters of the name at +8 (`00559450`), length 0.
/// The pushed 0 belongs to the setter: `00559450` takes no stack words.
pub fn tes_region_data_map_get_map_name(e: &mut Engine, this: Ptr, out: Ptr) {
    let chars = e.call(STRING_CHARS, &args![this.addr() + 8]).u32();
    e.call(STRING_SET, &args![out, chars, 0u32]);
}

// Translated from 004f3f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap::SetMapName` (Xbox PDB; `thiscall`, `RET 4`): when
/// `name` differs from the current name (`StrCmp` ignoring case, non-zero),
/// sets the name to it (length 0) and returns true; otherwise false.
pub fn tes_region_data_map_set_map_name(e: &mut Engine, this: Ptr, name: u32) -> bool {
    if e.call(STRING_COMPARE, &args![this.addr() + 8, name, 1u32])
        .u32()
        == 0
    {
        return false;
    }
    e.call(STRING_SET, &args![this.addr() + 8, name, 0u32]);
    true
}

// Translated from 004f3f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3f80 - its FS:[0] exception frame (the unwind state that frees the block when the copy constructor throws) is not translated
/// `TESRegionDataMap::Copy` (Xbox PDB): allocates 0x10 bytes and, when that
/// succeeds, copy-constructs a `TESRegionDataMap` of `this` in them
/// (`004f3d50`). Returns the copy, or null.
pub fn tes_region_data_map_copy(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_004f3d50(e, Ptr::new(block), this)
}

/// `TESRegionData::Save` (`004f14c0`, engine map; no stack word).
const TES_REGION_DATA_SAVE: u32 = 0x004f_14c0;
/// `004048e0` (unnamed in the engine map; no stack word): the string's
/// length (its stored length, or the C string's length when that is 0xFFFF).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// `004f4280` (unnamed in the engine map; no stack word): the string's
/// length plus one.
const STRING_SIZE_004F4280: u32 = 0x004f_4280;
/// `TESForm::AddChunkArray` (`004856f0`, engine map; cdecl: type, data, size).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `TESRegionData::GetID` (Xbox PDB slot +0x10).
const SLOT_GET_ID: u32 = 0x10;
/// `TESRegionDataMap::SetMapName` (Xbox PDB slot +0x30).
const SLOT_SET_MAP_NAME: u32 = 0x30;
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
/// The `GetID` of map region data.
const MAP_ID: u32 = 4;

// Translated from 004f3e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap::Save` (Xbox PDB): runs `TESRegionData::Save`, then,
/// when the name at +8 is not empty (`004048e0`), writes it as an `RDMP`
/// chunk of its characters (`00559450`) and its length plus one
/// (`004f4280`; `TESForm::AddChunkArray`).
pub fn tes_region_data_map_save(e: &mut Engine, this: Ptr) {
    e.call(TES_REGION_DATA_SAVE, &args![this]);
    if e.call(STRING_LENGTH, &args![this.addr() + 8]).u32() == 0 {
        return;
    }
    let size = e.call(STRING_SIZE_004F4280, &args![this.addr() + 8]).u32();
    let chars = e.call(STRING_CHARS, &args![this.addr() + 8]).u32();
    e.call(ADD_CHUNK_ARRAY, &args![CHUNK_RDMP, chars, size]);
}

/// The tail the "take the source" branches of `BlendInto` share: the
/// source's `bOverride` and priority, then its name.
fn map_blend_take_source(e: &mut Engine, this: Ptr, source: Ptr) {
    let flag = e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8();
    e.call(REGION_DATA_SET_OVERRIDE, &args![this, flag as u32]);
    let priority = e.call(REGION_DATA_GET_PRIORITY, &args![source]).u8();
    e.call(REGION_DATA_SET_PRIORITY, &args![this, priority as u32]);
    map_blend_take_name(e, this, source);
}

/// `SetMapName` (slot +0x30) with the source's `GetMapName` (slot +0x2C).
fn map_blend_take_name(e: &mut Engine, this: Ptr, source: Ptr) {
    let name = e.vcall(source.addr(), SLOT_GET_MAP_NAME, &args![]).u32();
    e.vcall(this.addr(), SLOT_SET_MAP_NAME, &args![name]);
}

/// `005bb4d0` on an object, as the `MOVZX` the exe does.
fn map_priority(e: &mut Engine, data: Ptr) -> i32 {
    e.call(REGION_DATA_GET_PRIORITY, &args![data]).u8() as i32
}

// Translated from 004f4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap::BlendInto` (Xbox PDB; `thiscall`, `RET 8`): merges map
/// data `source` (its `GetID` must be 4, and `enabled` nonzero) into this
/// one. When this data's `bIgnore` (+5) is set, the source is taken (its
/// `bOverride`, priority and name). Otherwise nothing happens when the
/// source's `bIgnore` is set. With this data's `bOverride` (+4) set, the
/// source is taken only when its `bOverride` is set and its priority is
/// higher. Otherwise the source is taken when its `bOverride` is set; else
/// its name is taken when its priority is higher, and the priority becomes
/// `trunc(00406cc0((this*this + (100-this)*src + (100-src)*this + src*src) * 0.005))`
/// (each priority read anew, in the exe's order).
pub fn tes_region_data_map_blend_into(e: &mut Engine, this: Ptr, source: Ptr, enabled: u32) {
    if source.is_null()
        || e.vcall(source.addr(), SLOT_GET_ID, &args![]).u32() != MAP_ID
        || enabled == 0
    {
        return;
    }
    // `TESRegionData::bIgnore` (Xbox PDB) +0x05.
    if e.mem.u8(this.addr() + 5) != 0 {
        map_blend_take_source(e, this, source);
        return;
    }
    if e.mem.u8(source.addr() + 5) != 0 {
        return;
    }
    // `TESRegionData::bOverride` (Xbox PDB) +0x04.
    if e.mem.u8(this.addr() + 4) != 0 {
        if e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8() != 0
            && map_priority(e, source) > map_priority(e, this)
        {
            map_blend_take_source(e, this, source);
        }
        return;
    }
    if e.call(REGION_DATA_GET_OVERRIDE, &args![source]).u8() != 0 {
        map_blend_take_source(e, this, source);
        return;
    }
    if map_priority(e, source) > map_priority(e, this) {
        map_blend_take_name(e, this, source);
    }
    let a = map_priority(e, this);
    let b = map_priority(e, this);
    let c = map_priority(e, this);
    let d = map_priority(e, source);
    let first = a * b + (100 - c) * d;
    let mut total = x87_store32(f64::from(first));
    let f = map_priority(e, source);
    let g = map_priority(e, this);
    let h = map_priority(e, source);
    let i = map_priority(e, source);
    let second = (100 - f) * g + h * i;
    total = x87_store32(x87_add(f64::from(second), x87_m32(total)));
    let half_percent: f64 = e.global(HALF_PERCENT);
    total = x87_store32(x87_mul(x87_load32(total), half_percent));
    let rounded = e.call(FLOAT_HELPER_00406CC0, &args![total]).f64();
    let value = crate::x87::fistp_chop(rounded) as u8;
    e.call(REGION_DATA_SET_PRIORITY, &args![this, value as u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_3e40, tes_region_data_map_save(Ptr)),
        entry!(0x004f_4000, tes_region_data_map_blend_into(Ptr, Ptr, u32)),
        entry!(0x004f_3d20, tes_region_data_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_3ca0, tes_region_data_map_tes_region_data_map(Ptr) -> Ptr),
        entry!(0x004f_3d50, fn_004f3d50(Ptr, Ptr) -> Ptr),
        entry!(0x004f_3de0, fn_004f3de0(Ptr)),
        entry!(0x004f_3e90, tes_region_data_map_load_region_data_map(Ptr, Ptr) -> bool),
        entry!(0x004f_3f10, tes_region_data_map_get_map_name(Ptr, Ptr)),
        entry!(0x004f_3f40, tes_region_data_map_set_map_name(Ptr, u32) -> bool),
        entry!(0x004f_3f80, tes_region_data_map_copy(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod map_tests {
    use super::*;

    fn others(log: Vec<(u32, Vec<u32>)>, own: u32) -> Vec<(u32, Vec<u32>)> {
        log.into_iter().filter(|c| c.0 != own).collect()
    }

    #[test]
    fn at_004f3ca0_sets_the_default_name() {
        let mut e = Engine::new();
        for a in [TES_REGION_DATA_CTOR, STRING_CTOR, STRING_SET] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_3ca0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_MAP_VTABLE);
        assert_eq!(
            others(log, 0x004f_3ca0),
            vec![
                (TES_REGION_DATA_CTOR, vec![this]),
                (STRING_CTOR, vec![this + 8]),
                (STRING_SET, vec![this + 8, DEFAULT_REGION_NAME, 0]),
            ]
        );
    }

    fn map_source(e: &mut Engine) -> u32 {
        e.put_vtable(0x0f01_0000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f01_1000]);
        e.register(0x0f01_1000, |_, _| 0x00ab_cdefu32.into_ret());
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x0f01_0000);
        source
    }

    #[test]
    fn at_004f3d50_copies_the_base_and_the_name() {
        let mut e = Engine::new();
        for a in [TES_REGION_DATA_COPY_CTOR, STRING_CTOR, STRING_SET] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(STRING_COMPARE, |_, _| 1u32.into_ret());
        let source = map_source(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_3d50, &args![this, source]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_MAP_VTABLE);
        assert_eq!(
            others(log, 0x004f_3d50),
            vec![
                (TES_REGION_DATA_COPY_CTOR, vec![this, source]),
                (STRING_CTOR, vec![this + 8]),
                (0x0f01_1000, vec![source]),
                (STRING_COMPARE, vec![this + 8, 0x00ab_cdef, 1]),
                (STRING_SET, vec![this + 8, 0x00ab_cdef, 0]),
            ]
        );
    }

    #[test]
    fn at_004f3de0_destroys_the_name_then_the_base() {
        let mut e = Engine::new();
        for a in [STRING_DTOR, TES_REGION_DATA_DTOR] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_3de0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_MAP_VTABLE);
        assert_eq!(
            others(log, 0x004f_3de0),
            vec![
                (STRING_DTOR, vec![this + 8]),
                (TES_REGION_DATA_DTOR, vec![this])
            ]
        );
    }

    #[test]
    fn at_004f3e90_reads_the_name_chunk() {
        let mut e = Engine::new();
        e.register(GET_TES_CHUNK, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CHUNK_SIZE_00401660, |_, _| 0x20u32.into_ret());
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_cstr(a[1], b"Goodsprings");
            Ret::default()
        });
        e.register_double(STRING_SET, |e, a| {
            let text = e.mem.cstr(a[1]);
            assert_eq!(text, b"Goodsprings".to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert!(!e.call(0x004f_3e90, &args![this, 0u32]).bool());
        let file = e.mem.alloc(0x300);
        e.mem.set_u32(file, 0x4449_4452);
        assert!(!e.call(0x004f_3e90, &args![this, file]).bool());
        e.mem.set_u32(file, CHUNK_RDMP);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_3e90, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        let read = log.iter().find(|c| c.0 == GET_CHUNK_DATA).unwrap();
        assert_eq!(read.1[0], file);
        assert_eq!(read.1[2], 0);
        let set = log.iter().find(|c| c.0 == STRING_SET).unwrap();
        assert_eq!(set.1, vec![this + 8, read.1[1], 0]);
    }

    #[test]
    fn at_004f3f10_sets_out_from_the_name_characters() {
        let mut e = Engine::new();
        e.register(STRING_CHARS, |_, _| 0x0055_0000u32.into_ret());
        e.register(STRING_SET, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x004f_3f10, &args![this, out]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            others(log, 0x004f_3f10),
            vec![
                (STRING_CHARS, vec![this + 8]),
                (STRING_SET, vec![out, 0x0055_0000, 0]),
            ]
        );
    }

    #[test]
    fn at_004f3f40_sets_only_a_different_name() {
        let mut e = Engine::new();
        e.register(STRING_COMPARE, |_, a| u32::from(a[1] != 0x10).into_ret());
        e.register(STRING_SET, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_3f40, &args![this, 0x10u32]).bool());
        assert!(e.call(0x004f_3f40, &args![this, 0x20u32]).bool());
        let log = e.call_log.take().unwrap();
        let sets: Vec<_> = log.into_iter().filter(|c| c.0 == STRING_SET).collect();
        assert_eq!(sets, vec![(STRING_SET, vec![this + 8, 0x20, 0])]);
    }

    #[test]
    fn at_004f3f80_copies_into_a_new_block_or_returns_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| Ret::default());
        let this = map_source(&mut e);
        assert_eq!(e.call(0x004f_3f80, &args![this]).u32(), 0);

        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for a in [TES_REGION_DATA_COPY_CTOR, STRING_CTOR, STRING_SET] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(STRING_COMPARE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let copy = e.call(0x004f_3f80, &args![this]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(copy, 0);
        assert_eq!(e.mem.u32(copy), TES_REGION_DATA_MAP_VTABLE);
        let alloc = log.iter().find(|c| c.0 == ALLOCATE).unwrap();
        assert_eq!(alloc.1, vec![0x10]);
        let base = log
            .iter()
            .find(|c| c.0 == TES_REGION_DATA_COPY_CTOR)
            .unwrap();
        assert_eq!(base.1, vec![copy, this]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f3d20_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_3de0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_3d20, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_3d20)
                .collect();
            let mut expected = vec![(0x004f_3de0, vec![this.addr()])];
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
    fn save_writes_a_non_empty_name() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_SAVE, |_, _| Ret::default());
        e.register(STRING_LENGTH, |_, _| ret(5));
        e.register(STRING_SIZE_004F4280, |_, _| ret(6));
        e.register(STRING_CHARS, |_, _| ret(0x4321));
        e.register(ADD_CHUNK_ARRAY, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_3e40, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, TES_REGION_DATA_SAVE), vec![vec![this]]);
        assert_eq!(
            calls_to(&log, ADD_CHUNK_ARRAY),
            vec![vec![CHUNK_RDMP, 0x4321, 6]]
        );
        e.register(STRING_LENGTH, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x004f_3e40, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ADD_CHUNK_ARRAY).is_empty());
    }

    /// A target and a source of GetID 4; slot 0x2C returns the word at +8,
    /// slot 0x30 records the name it is given at +0xC.
    fn blend_setup(e: &mut Engine) -> (u32, u32) {
        let slots: Vec<u32> = (0..13).map(|i| 0x0150_1000 + 4 * i).collect();
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_1010, |_, _| ret(MAP_ID));
        e.register(0x0150_102c, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(0x0150_1030, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
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
        e.mem.set_u32(source + 8, 0xbbbb);
        (this, source)
    }

    #[test]
    fn blend_into_takes_the_source_when_this_is_ignored_or_it_overrides() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        // Bad sources: nothing.
        e.call(0x004f_4000, &args![this, source, 0u32]);
        e.call(0x004f_4000, &args![this, 0u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.mem.set_u8(this + 5, 1);
        e.mem.set_u8(source + 4, 1);
        e.mem.set_u8(source + 6, 70);
        e.call(0x004f_4000, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xbbbb);
        assert_eq!(e.mem.u8(this + 4), 1);
        assert_eq!(e.mem.u8(this + 6), 70);
        // This overrides: the source needs its override and a higher priority.
        e.mem.set_u8(this + 5, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.call(0x004f_4000, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.mem.set_u8(source + 6, 71);
        e.call(0x004f_4000, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xbbbb);
        // An ignored source: nothing.
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u8(source + 5, 1);
        e.call(0x004f_4000, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // Neither overrides, the source does: taken.
        e.mem.set_u8(source + 5, 0);
        e.mem.set_u8(this + 4, 0);
        e.mem.set_u8(source + 6, 20);
        e.call(0x004f_4000, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xbbbb);
        assert_eq!(e.mem.u8(this + 6), 20);
    }

    #[test]
    fn blend_into_blends_the_priority_and_takes_a_higher_name() {
        let mut e = Engine::new();
        let (this, source) = blend_setup(&mut e);
        e.mem.set_u8(this + 6, 40);
        e.mem.set_u8(source + 6, 80);
        e.call(0x004f_4000, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0xbbbb);
        // (40*40 + 60*80 + 20*40 + 80*80) * 0.005 = 68.
        assert_eq!(e.mem.u8(this + 6), 68);
        // A lower source keeps the name: (90*90 + 10*10 + 90*90 + 10*10) * 0.005 = 82.
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u8(this + 6, 90);
        e.mem.set_u8(source + 6, 10);
        e.call(0x004f_4000, &args![this, source, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u8(this + 6), 82);
    }
}
