//! `fallout shared/region system/tesregiondatamanager.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_shared_region_system::run_004f3610::TESRegionDataManager;

/// `00401000` (`cdecl`): allocate.
const ALLOCATE: u32 = 0x0040_1000;
/// The region data constructors `ConstructRegionData` picks by type (with
/// the allocation size): 3 (`004f5040`, 0x10 bytes), 4
/// (`TESRegionDataMap::TESRegionDataMap`, `004f3ca0`, 0x10), 5
/// (`TESRegionDataLandscape::TESRegionDataLandscape`, `004f2790`, 0xc), 6
/// (`004f15f0`, 0xc), 7 (`004f42a0`, 0x20) and 8 (`004f1ed0`, 0x10).
const WEATHER_CONSTRUCT: u32 = 0x004f_5040;
const MAP_CONSTRUCT: u32 = 0x004f_3ca0;
const LANDSCAPE_CONSTRUCT: u32 = 0x004f_2790;
const TYPE_6_CONSTRUCT: u32 = 0x004f_15f0;
const SOUND_CONSTRUCT: u32 = 0x004f_42a0;
const IMPOSTER_CONSTRUCT: u32 = 0x004f_1ed0;
/// The region data's vtable slot (byte offset) that answers its type.
const REGION_DATA_TYPE_SLOT: u32 = 0x10;
/// The region data's vtable slots `LoadRegionData` calls: the header read
/// (`0x8`, the `RDAT` data's address) and the chunk load (`0x24`).
const REGION_DATA_INIT_SLOT: u32 = 0x8;
const REGION_DATA_LOAD_SLOT: u32 = 0x24;
/// The manager's vtable slots: `ConstructRegionData` (`0x4`) and the
/// `IsRegionData...` casts for weather (`0xc`), map (`0x10`), landscape
/// (`0x14`), the type-6 data (`0x18`), sound (`0x1c`) and imposter (`0x20`).
const CONSTRUCT_SLOT: u32 = 0x4;
const AS_WEATHER_SLOT: u32 = 0xc;
const AS_MAP_SLOT: u32 = 0x10;
const AS_LANDSCAPE_SLOT: u32 = 0x14;
const AS_TYPE_6_SLOT: u32 = 0x18;
const AS_SOUND_SLOT: u32 = 0x1c;
const AS_IMPOSTER_SLOT: u32 = 0x20;
/// The region data the last `RDAT` chunk made (`011c7f10`), which the
/// following chunks load into.
const CURRENT_REGION_DATA: u32 = 0x011c_7f10;
/// `TESFile::GetTESChunk` (`004726b0`) and the sized `GetChunkData`
/// (`00472890`, `RET 8`); `00401680` (whether ids need fixing up) and
/// `00503210` (fixes the `RDAT` header up).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const FILE_FIXES_IDS: u32 = 0x0040_1680;
const FIX_UP_HEADER: u32 = 0x0050_3210;
/// `00403d30` (`cdecl`: address, byte, count): fills memory.
const FILL_MEMORY: u32 = 0x0040_3d30;
/// `009611e0` (`thiscall`): the region's data list (its +0x18).
const REGION_DATA_LIST: u32 = 0x0096_11e0;
/// `005ae3d0` (`thiscall`, `RET 4`, the item's address): inserts into a
/// list.
const LIST_INSERT: u32 = 0x005a_e3d0;
/// Chunk tags.
const CHUNK_RDAT: u32 = 0x5441_4452;
const CHUNK_RDWT: u32 = 0x5457_4452;
const CHUNK_RDMP: u32 = 0x504d_4452;
const CHUNK_RDGS: u32 = 0x5347_4452;
const CHUNK_RDMD: u32 = 0x444d_4452;
const CHUNK_RDSB: u32 = 0x4253_4452;
const CHUNK_RDSD: u32 = 0x4453_4452;
const CHUNK_RDSI: u32 = 0x4953_4452;
const CHUNK_RDLN: u32 = 0x4e4c_4452;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_RDID: u32 = 0x4449_4452;

// Translated from 004f3690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::ConstructRegionData` (Xbox PDB), `RET 4`: the
/// type. Types 3 to 8 (the exe's jump table on `type - 3`) allocate and
/// construct their region data (null when the allocation failed); other
/// types answer null.
// Translation incomplete: 004f3690 - the SEH frame (FS:[0], unwind of each allocation) is not translated.
pub fn tes_region_data_manager_construct_region_data(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data_type: u32,
) -> u32 {
    let (size, construct) = match data_type.wrapping_sub(3) {
        0 => (0x10u32, WEATHER_CONSTRUCT),
        1 => (0x10, MAP_CONSTRUCT),
        2 => (0xc, LANDSCAPE_CONSTRUCT),
        3 => (0xc, TYPE_6_CONSTRUCT),
        4 => (0x20, SOUND_CONSTRUCT),
        5 => (0x10, IMPOSTER_CONSTRUCT),
        _ => return 0,
    };
    let block = e.call(ALLOCATE, &args![size]).u32();
    if block != 0 {
        e.call(construct, &args![block]).u32()
    } else {
        0
    }
}

/// The shape of the `IsRegionData...` casts: the data when its type (vtable
/// `+0x10`) is `data_type`, else null (and null for null).
fn data_of_type(e: &mut Engine, data: Ptr, data_type: u32) -> u32 {
    if !data.is_null() && e.vcall(data.addr(), REGION_DATA_TYPE_SLOT, &args![]).u32() == data_type {
        data.addr()
    } else {
        0
    }
}

// Translated from 004f38e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::IsRegionDataWeather` (Xbox PDB), `RET 4`: the
/// data when its type is 3, else null.
pub fn tes_region_data_manager_is_region_data_weather(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data: Ptr,
) -> u32 {
    data_of_type(e, data, 3)
}

// Translated from 004f3910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::IsRegionDataMap` (Xbox PDB), `RET 4`: the data
/// when its type is 4, else null.
pub fn tes_region_data_manager_is_region_data_map(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data: Ptr,
) -> u32 {
    data_of_type(e, data, 4)
}

// Translated from 004f3940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::IsRegionDataLandscape` (Xbox PDB), `RET 4`: the
/// data when its type is 5, else null.
pub fn tes_region_data_manager_is_region_data_landscape(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data: Ptr,
) -> u32 {
    data_of_type(e, data, 5)
}

// Translated from 004f38b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::IsRegionDataObjects` (Xbox PDB), `RET 4`: the
/// data when its type is 2, else null.
pub fn tes_region_data_manager_is_region_data_objects(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data: Ptr,
) -> u32 {
    data_of_type(e, data, 2)
}

// Translated from 004f3970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::IsRegionDataGrass` (Xbox PDB), `RET 4`: the data
/// when its type is 6, else null.
pub fn tes_region_data_manager_is_region_data_grass(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data: Ptr,
) -> u32 {
    data_of_type(e, data, 6)
}

// Translated from 004f39a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::IsRegionDataSound` (Xbox PDB), `RET 4`: the data
/// when its type is 7, else null.
pub fn tes_region_data_manager_is_region_data_sound(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data: Ptr,
) -> u32 {
    data_of_type(e, data, 7)
}

// Translated from 004f39d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::IsRegionDataImposter` (Xbox PDB), `RET 4`: the
/// data when its type is 8, else null.
pub fn tes_region_data_manager_is_region_data_imposter(
    e: &mut Engine,
    _this: Ptr<TESRegionDataManager>,
    data: Ptr,
) -> u32 {
    data_of_type(e, data, 8)
}

// Translated from 004f3a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataManager::LoadRegionData` (Xbox PDB), `RET 8`: the file and
/// the region. False without either. Remembers the region
/// (`pLastLoadedRegion`) and reads the current chunk: `RDAT` (an 8-byte
/// header, cleared, read and fixed up when `00401680` says so) makes the
/// region data of the header's type (vtable `+0x4`) into the current data
/// (`011c7f10`), which reads the header (its vtable `+0x8`) and goes into
/// the region's data list (`009611e0`, `005ae3d0`); the other chunks load
/// into the current data cast by the manager (vtable slots `0xc` to `0x20`):
/// `RDWT` the weather (with the file and the region), `RDMP` the map,
/// `RDLN`/`ICON` the landscape, `RDGS` the type-6 data, `RDMD`/`RDSB`/
/// `RDSD`/`RDSI` the sound, `RDID` the imposter (each with the file, data
/// vtable `+0x24`). Answers true.
pub fn tes_region_data_manager_load_region_data(
    e: &mut Engine,
    this: Ptr<TESRegionDataManager>,
    file: Ptr,
    region: Ptr,
) -> bool {
    if file.is_null() || region.is_null() {
        return false;
    }
    e.set(this, TESRegionDataManager::pLastLoadedRegion, region.addr());
    let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
    let cast_slot = match tag {
        CHUNK_RDAT => {
            e.with_stack(8, |e, header| {
                e.call(FILL_MEMORY, &args![header, 0u32, 8u32]);
                e.call(GET_CHUNK_DATA_SIZED, &args![file, header, 8u32]);
                if e.call(FILE_FIXES_IDS, &args![file]).bool() {
                    e.call(FIX_UP_HEADER, &args![header]);
                }
                let data_type = e.mem.u32(header.addr());
                let data = e
                    .vcall(this.addr(), CONSTRUCT_SLOT, &args![data_type])
                    .u32();
                e.set_global::<u32>(CURRENT_REGION_DATA, data);
                if e.global::<u32>(CURRENT_REGION_DATA) != 0 {
                    let current = e.global::<u32>(CURRENT_REGION_DATA);
                    e.vcall(current, REGION_DATA_INIT_SLOT, &args![header]);
                    let list = e.call(REGION_DATA_LIST, &args![region]).u32();
                    e.call(LIST_INSERT, &args![list, CURRENT_REGION_DATA]);
                }
            });
            return true;
        }
        CHUNK_RDWT => {
            let current = e.global::<u32>(CURRENT_REGION_DATA);
            let weather = e.vcall(this.addr(), AS_WEATHER_SLOT, &args![current]).u32();
            if weather != 0 {
                e.vcall(weather, REGION_DATA_LOAD_SLOT, &args![file, region]);
            }
            return true;
        }
        CHUNK_RDMP => AS_MAP_SLOT,
        CHUNK_RDGS => AS_TYPE_6_SLOT,
        CHUNK_RDMD | CHUNK_RDSB | CHUNK_RDSD | CHUNK_RDSI => AS_SOUND_SLOT,
        CHUNK_RDLN | CHUNK_ICON => AS_LANDSCAPE_SLOT,
        CHUNK_RDID => AS_IMPOSTER_SLOT,
        _ => return true,
    };
    let current = e.global::<u32>(CURRENT_REGION_DATA);
    let data = e.vcall(this.addr(), cast_slot, &args![current]).u32();
    if data != 0 {
        e.vcall(data, REGION_DATA_LOAD_SLOT, &args![file]);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004f_3690,
            tes_region_data_manager_construct_region_data(Ptr<TESRegionDataManager>, u32) -> u32
        ),
        entry!(
            0x004f_38e0,
            tes_region_data_manager_is_region_data_weather(Ptr<TESRegionDataManager>, Ptr) -> u32
        ),
        entry!(
            0x004f_3910,
            tes_region_data_manager_is_region_data_map(Ptr<TESRegionDataManager>, Ptr) -> u32
        ),
        entry!(
            0x004f_3940,
            tes_region_data_manager_is_region_data_landscape(Ptr<TESRegionDataManager>, Ptr) -> u32
        ),
        entry!(
            0x004f_38b0,
            tes_region_data_manager_is_region_data_objects(Ptr<TESRegionDataManager>, Ptr) -> u32
        ),
        entry!(
            0x004f_3970,
            tes_region_data_manager_is_region_data_grass(Ptr<TESRegionDataManager>, Ptr) -> u32
        ),
        entry!(
            0x004f_39a0,
            tes_region_data_manager_is_region_data_sound(Ptr<TESRegionDataManager>, Ptr) -> u32
        ),
        entry!(
            0x004f_39d0,
            tes_region_data_manager_is_region_data_imposter(Ptr<TESRegionDataManager>, Ptr) -> u32
        ),
        entry!(
            0x004f_3a00,
            tes_region_data_manager_load_region_data(Ptr<TESRegionDataManager>, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    const MANAGER_VTABLE: u32 = 0x00ab_0000;
    const DATA_VTABLE: u32 = 0x00ac_0000;
    const TYPE_FN: u32 = 0x00ac_1000;
    const INIT_FN: u32 = 0x00ac_1004;
    const LOAD_FN: u32 = 0x00ac_1008;
    /// The tag `GetTESChunk` answers.
    const TAG: u32 = 0x00ad_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
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
        e.map(TAG, 0x10);
        e.map(0x011c_7000, 0x1000);
        // The manager's own vtable runs this file's translations.
        let mut slots = vec![0u32; 9];
        slots[1] = 0x004f_3690;
        slots[3] = 0x004f_38e0;
        slots[4] = 0x004f_3910;
        slots[5] = 0x004f_3940;
        slots[6] = 0x004f_3970;
        slots[7] = 0x004f_39a0;
        slots[8] = 0x004f_39d0;
        e.put_vtable(MANAGER_VTABLE, &slots);
        // 004f3970 (the type-6 cast) is not this file's: a double of its shape.
        e.register(0x004f_3970, |e, a| {
            let is = a[1] != 0 && e.vcall(a[1], 0x10, &args![]).u32() == 6;
            (if is { a[1] } else { 0 }).into_ret()
        });
        let mut slots = vec![0u32; 10];
        slots[2] = INIT_FN;
        slots[4] = TYPE_FN;
        slots[9] = LOAD_FN;
        e.put_vtable(DATA_VTABLE, &slots);
        e.register(TYPE_FN, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(INIT_FN, |_, _| Ret::default());
        e.register(LOAD_FN, |_, _| Ret::default());
        e.register(GET_TES_CHUNK, |e, _| e.mem.u32(TAG).into_ret());
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            e.mem.set_u32(a[1], 5);
            e.mem.set_u32(a[1] + 4, 0x99);
            Ret::default()
        });
        e.register(FILL_MEMORY, |_, _| Ret::default());
        e.register(FILE_FIXES_IDS, |_, _| 1u32.into_ret());
        e.register(FIX_UP_HEADER, |_, _| Ret::default());
        e.register(REGION_DATA_LIST, |_, a| (a[0] + 0x18).into_ret());
        e.register(LIST_INSERT, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for c in [
            WEATHER_CONSTRUCT,
            MAP_CONSTRUCT,
            LANDSCAPE_CONSTRUCT,
            TYPE_6_CONSTRUCT,
            SOUND_CONSTRUCT,
            IMPOSTER_CONSTRUCT,
        ] {
            e.register_double(c, move |e, a| {
                e.mem.set_u32(a[0], DATA_VTABLE);
                e.mem.set_u32(a[0] + 4, c);
                a[0].into_ret()
            });
        }
        e
    }

    fn data(e: &mut Engine, data_type: u32) -> u32 {
        let d = e.mem.alloc(8);
        e.mem.set_u32(d, DATA_VTABLE);
        e.mem.set_u32(d + 4, data_type);
        d
    }

    fn manager(e: &mut Engine) -> Ptr<TESRegionDataManager> {
        let m = e.new_object::<TESRegionDataManager>();
        e.mem.set_u32(m.addr(), MANAGER_VTABLE);
        m
    }

    #[test]
    fn construct_region_data_by_type() {
        let mut e = engine();
        let m = manager(&mut e);
        for (t, size, c) in [
            (3u32, 0x10u32, WEATHER_CONSTRUCT),
            (4, 0x10, MAP_CONSTRUCT),
            (5, 0xc, LANDSCAPE_CONSTRUCT),
            (6, 0xc, TYPE_6_CONSTRUCT),
            (7, 0x20, SOUND_CONSTRUCT),
            (8, 0x10, IMPOSTER_CONSTRUCT),
        ] {
            e.call_log = Some(vec![]);
            let d = e.call(0x004f_3690, &args![m, t]).u32();
            assert_eq!(calls_to(&e, ALLOCATE), vec![vec![size]]);
            assert_eq!(e.mem.u32(d + 4), c);
        }
        for t in [0u32, 2, 9, 0xffff_ffff] {
            assert_eq!(e.call(0x004f_3690, &args![m, t]).u32(), 0);
        }
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x004f_3690, &args![m, 3u32]).u32(), 0);
    }

    #[test]
    fn is_region_data_casts() {
        let mut e = engine();
        let m = manager(&mut e);
        for (address, t) in [
            (0x004f_38e0u32, 3u32),
            (0x004f_3910, 4),
            (0x004f_3940, 5),
            (0x004f_39a0, 7),
            (0x004f_39d0, 8),
        ] {
            let right = data(&mut e, t);
            let wrong = data(&mut e, t + 1);
            assert_eq!(e.call(address, &args![m, right]).u32(), right);
            assert_eq!(e.call(address, &args![m, wrong]).u32(), 0);
            assert_eq!(e.call(address, &args![m, 0u32]).u32(), 0);
        }
    }

    #[test]
    fn load_region_data_makes_and_loads_the_current_data() {
        let mut e = engine();
        let m = manager(&mut e);
        let region = e.mem.alloc(0x40);
        // Without a file or region: false.
        assert!(!e.call(0x004f_3a00, &args![m, 0u32, region]).bool());
        assert!(!e.call(0x004f_3a00, &args![m, 0x9000u32, 0u32]).bool());
        // RDAT: type 5 from the header, made current and listed.
        e.mem.set_u32(TAG, CHUNK_RDAT);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_3a00, &args![m, 0x9000u32, region]).bool());
        assert_eq!(e.get(m, TESRegionDataManager::pLastLoadedRegion), region);
        let current = e.global::<u32>(CURRENT_REGION_DATA);
        assert_eq!(e.mem.u32(current + 4), LANDSCAPE_CONSTRUCT);
        assert_eq!(calls_to(&e, FIX_UP_HEADER).len(), 1);
        assert_eq!(calls_to(&e, INIT_FN).len(), 1);
        assert_eq!(
            calls_to(&e, LIST_INSERT),
            vec![vec![region + 0x18, CURRENT_REGION_DATA]]
        );
        // Each chunk loads into the current data when it has the right type.
        let cases = [
            (CHUNK_RDWT, 3u32, true),
            (CHUNK_RDMP, 4, false),
            (CHUNK_RDLN, 5, false),
            (CHUNK_ICON, 5, false),
            (CHUNK_RDGS, 6, false),
            (CHUNK_RDMD, 7, false),
            (CHUNK_RDSB, 7, false),
            (CHUNK_RDSD, 7, false),
            (CHUNK_RDSI, 7, false),
            (CHUNK_RDID, 8, false),
        ];
        for (tag, t, with_region) in cases {
            let d = data(&mut e, t);
            e.set_global::<u32>(CURRENT_REGION_DATA, d);
            e.mem.set_u32(TAG, tag);
            e.call_log = Some(vec![]);
            assert!(e.call(0x004f_3a00, &args![m, 0x9000u32, region]).bool());
            let expected = if with_region {
                vec![d, 0x9000, region]
            } else {
                vec![d, 0x9000]
            };
            assert_eq!(calls_to(&e, LOAD_FN), vec![expected], "tag {tag:08x}");
            // The wrong type: nothing loads.
            e.mem.set_u32(d + 4, t + 10);
            e.call_log = Some(vec![]);
            e.call(0x004f_3a00, &args![m, 0x9000u32, region]);
            assert!(calls_to(&e, LOAD_FN).is_empty());
        }
        // Another tag: true, nothing read.
        e.mem.set_u32(TAG, 0x1234_5678);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_3a00, &args![m, 0x9000u32, region]).bool());
        assert!(calls_to(&e, LOAD_FN).is_empty());
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    #[test]
    fn is_region_data_objects_and_grass_casts() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1010]);
        e.register(0x0150_1010, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Default::default()
        });
        let manager = e.mem.alloc(8);
        for (address, kind) in [(0x004f_38b0u32, 2u32), (0x004f_3970, 6)] {
            let right = e.mem.alloc(8);
            e.mem.set_u32(right, 0x0150_0000);
            e.mem.set_u32(right + 4, kind);
            let wrong = e.mem.alloc(8);
            e.mem.set_u32(wrong, 0x0150_0000);
            e.mem.set_u32(wrong + 4, kind + 1);
            assert_eq!(e.call(address, &args![manager, right]).u32(), right);
            assert_eq!(e.call(address, &args![manager, wrong]).u32(), 0);
            assert_eq!(e.call(address, &args![manager, 0u32]).u32(), 0);
        }
    }
}
