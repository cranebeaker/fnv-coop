//! `fallout shared/region system/tesregion.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESRegion` fields (Xbox PDB names; PC offsets 0x10 below the PDB's, as
/// for every `TESForm`): `pDataList` (+0x18), `pPointLists` (+0x1c),
/// `pWorldSpace` (+0x20), `pCurrentWeather` (+0x24).
const DATA_LIST: u32 = 0x18;
const POINT_LISTS: u32 = 0x1c;
const WORLD_SPACE: u32 = 0x20;
const CURRENT_WEATHER: u32 = 0x24;
/// `TESFile` readers: `GetTESForm` (`00472660`), `GetTESChunk`
/// (`004726b0`), next chunk (`004726f0`), `GetChunkData` (`004727f0`,
/// `RET 4`), the sized overload (`00472890`, `RET 8`) and the chunk size
/// (`00401660`).
const GET_TES_FORM: u32 = 0x0047_2660;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const NEXT_CHUNK: u32 = 0x0047_26f0;
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESForm::LoadForm` (`00485110`, `RET 4`) and `00484ab0` (`RET 4`).
const LOAD_FORM: u32 = 0x0048_5110;
const FORM_SET_FLAG: u32 = 0x0048_4ab0;
/// The form vtable slots (byte offsets): `0x134` takes the editor id read,
/// `0xe0` the file for `OBND`.
const SET_EDITOR_ID_SLOT: u32 = 0x134;
const LOAD_BOUNDS_SLOT: u32 = 0xe0;
/// `00401000` (`cdecl`): allocate.
const ALLOCATE: u32 = 0x0040_1000;
/// `004f7680` (`RET 4`): the point-list object's constructor; `0050f9c0`
/// (`RET 4`) stores a word at its +0x20; `TESRegionPointList::LoadPoints`
/// (`004f79f0`, `RET 4`).
const POINT_LIST_CONSTRUCT: u32 = 0x004f_7680;
const POINT_LIST_SET_20: u32 = 0x0050_f9c0;
const POINT_LIST_LOAD_POINTS: u32 = 0x004f_79f0;
/// `00441110` (`thiscall`): the word at +0x1c (`pPointLists`; folded body,
/// the engine map's name is another class's).
const POINT_LISTS_OF: u32 = 0x0044_1110;
/// List helpers: the item's address (`006815c0`), next (`00726070`), append
/// (`00905820`, `RET 4`, the item's address), remove an item (`00905330`,
/// `RET 4`, its address) and remove the head (`0063f7b0`).
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_APPEND: u32 = 0x0090_5820;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `004f81e0` (`RET 4`, a byte): whether a point list is kept after the load
/// (with 1); `004f0a10` (`RET 4`): its deleting destructor.
const POINT_LIST_KEEP: u32 = 0x004f_81e0;
const POINT_LIST_DELETE: u32 = 0x004f_0a10;
/// The `TES` singleton (`011c3f2c`) and `00740940` on it: its region data
/// manager (+0x624), whose vtable slot `0x24` is
/// `TESRegionDataManager::LoadRegionData` (file, region).
const TES_SINGLETON: u32 = 0x011c_3f2c;
const REGION_DATA_MANAGER_OF: u32 = 0x0074_0940;
const LOAD_REGION_DATA_SLOT: u32 = 0x24;
/// `004f3400` (`thiscall`): empties the region data list.
const DATA_LIST_CLEAR: u32 = 0x004f_3400;
/// `00549580` (`thiscall` on the region) and `00482f20` (on the `TES`):
/// the two `bool`s `004f1260` asks; `00586280` (`RET 4`) on the world
/// space.
const REGION_TEST: u32 = 0x0054_9580;
const TES_TEST: u32 = 0x0048_2f20;
const WORLD_SPACE_SET: u32 = 0x0058_6280;
/// `004013e0` (`thiscall`): a `bool` that makes `InitItem` do nothing;
/// `007af430`: the word at +0x20.
const FORM_SKIPS_INIT: u32 = 0x0040_13e0;
const WORD_AT_20: u32 = 0x007a_f430;
/// `TESForm::GetFile` (`00484e60`, `RET 4`), `TESForm::AddCompileIndex`
/// (`00485d50`, `cdecl`), the form lookup (`004839c0`, `cdecl`) and
/// `__RTDynamicCast` (`00ec43fb`) with its source type and the world space
/// target type.
const GET_FILE: u32 = 0x0048_4e60;
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_WORLD_SPACE_TYPE: u32 = 0x0118_3fd0;
/// `009611e0` (`thiscall`): the region's data list (+0x18); each data's
/// vtable slot `0xc` is its `InitItem` (the region).
const DATA_LIST_OF: u32 = 0x0096_11e0;
const DATA_INIT_ITEM_SLOT: u32 = 0xc;
/// `TESRegionDataList::Find` (`004f35b0`, `RET 4`, the type), the weather
/// data cast's type descriptors (`011899cc` to `011899e8`) and
/// `TESWeatherList::GetRandomWeather` (`004f10b0`).
const DATA_LIST_FIND: u32 = 0x004f_35b0;
const CAST_DATA_TYPE: u32 = 0x0118_99cc;
const CAST_WEATHER_DATA_TYPE: u32 = 0x0118_99e8;
const GET_RANDOM_WEATHER: u32 = 0x004f_10b0;
/// Chunk tags.
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_WNAM: u32 = 0x4d41_4e57;
const CHUNK_RPLI: u32 = 0x494c_5052;
const CHUNK_RPLD: u32 = 0x444c_5052;
const CHUNK_RDAT: u32 = 0x5441_4452;
const CHUNK_RDOB: u32 = 0x424f_4452;
const CHUNK_RDSB: u32 = 0x4253_4452;
const CHUNK_RDID: u32 = 0x4449_4452;
const CHUNK_RDSD: u32 = 0x4453_4452;
const CHUNK_RDMD: u32 = 0x444d_4452;
const CHUNK_RDSI: u32 = 0x4953_4452;
const CHUNK_RDLN: u32 = 0x4e4c_4452;
const CHUNK_RDOJ: u32 = 0x4a4f_4452;
const CHUNK_ICON: u32 = 0x4e4f_4349;
const CHUNK_RDMP: u32 = 0x504d_4452;
const CHUNK_RDOT: u32 = 0x544f_4452;
const CHUNK_RDGS: u32 = 0x5347_4452;
const CHUNK_RDWT: u32 = 0x5457_4452;

// Translated from 004f1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): sets `pWorldSpace`; for a world
/// space, when `00549580` answers true and `00482f20` on the `TES`
/// (`011c3f2c`) false, `00586280` with 1 on it.
pub fn fn_004f1260(e: &mut Engine, this: Ptr, world_space: u32) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(WORLD_SPACE), world_space);
    if e.mem.u32(base.wrapping_add(WORLD_SPACE)) == 0 {
        return;
    }
    if !e.call(REGION_TEST, &args![this]).bool() {
        return;
    }
    let tes = e.global::<u32>(TES_SINGLETON);
    if e.call(TES_TEST, &args![tes]).bool() {
        return;
    }
    let world = e.mem.u32(base.wrapping_add(WORLD_SPACE));
    e.call(WORLD_SPACE_SET, &args![world, 1u32]);
}

// Translated from 004f0ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the region: its data list
/// (`004f3400`) when there is one, and each point list (deleted with
/// `004f0a10`) out of `pPointLists`, head by head.
pub fn fn_004f0ef0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let data = e.mem.u32(base.wrapping_add(DATA_LIST));
    if data != 0 {
        e.call(DATA_LIST_CLEAR, &args![data]);
    }
    if e.mem.u32(base.wrapping_add(POINT_LISTS)) == 0 {
        return;
    }
    let lists = e.mem.u32(base.wrapping_add(POINT_LISTS));
    let item_address = e.call(NODE_ITEM_ADDRESS, &args![lists]).u32();
    let mut item = e.mem.u32(item_address);
    while item != 0 {
        let lists = e.mem.u32(base.wrapping_add(POINT_LISTS));
        e.call(LIST_REMOVE_HEAD, &args![lists]);
        if item != 0 {
            e.call(POINT_LIST_DELETE, &args![item, 1u32]);
        }
        let lists = e.mem.u32(base.wrapping_add(POINT_LISTS));
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![lists]).u32();
        item = e.mem.u32(item_address);
    }
}

// Translated from 004f0f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegion`'s `InitItem` (unnamed in the engine map): unless `004013e0`
/// answers true, the world space id (+0x20, `007af430`) gets the file's
/// compile index, is looked up and cast; a world space is set (`004f1260`);
/// `00484ab0` with 1; then each region data of the list (until an empty
/// item) runs its `InitItem` (vtable `+0xc`) with the region.
pub fn fn_004f0f80(e: &mut Engine, this: Ptr) {
    if e.call(FORM_SKIPS_INIT, &args![this]).bool() {
        return;
    }
    let world = e.with_stack(4, |e, id| {
        let value = e.call(WORD_AT_20, &args![this]).u32();
        e.mem.set_u32(id.addr(), value);
        let file = e.call(GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![id, file]);
        let id = e.mem.u32(id.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        e.call(
            DYNAMIC_CAST,
            &args![form, 0u32, CAST_SOURCE_TYPE, CAST_WORLD_SPACE_TYPE, 0u32],
        )
        .u32()
    });
    if world != 0 {
        fn_004f1260(e, this, world);
    }
    e.call(FORM_SET_FLAG, &args![this, 1u32]);
    let mut node = e.call(DATA_LIST_OF, &args![this]).u32();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let data = e.mem.u32(item_address);
        e.vcall(data, DATA_INIT_ITEM_SLOT, &args![this]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 004f1050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegion::UpdateWeather` (engine map name): clears `pCurrentWeather`;
/// with a data list, its type-3 data (`TESRegionDataList::Find`) cast to
/// the weather data (`00ec43fb`) picks the current weather
/// (`TESWeatherList::GetRandomWeather`).
pub fn tes_region_update_weather(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(CURRENT_WEATHER), 0);
    let list = e.mem.u32(base.wrapping_add(DATA_LIST));
    if list == 0 {
        return;
    }
    let found = e.call(DATA_LIST_FIND, &args![list, 3u32]).u32();
    let weather_data = e
        .call(
            DYNAMIC_CAST,
            &args![found, 0u32, CAST_DATA_TYPE, CAST_WEATHER_DATA_TYPE, 0u32],
        )
        .u32();
    if weather_data != 0 {
        let weather = e.call(GET_RANDOM_WEATHER, &args![weather_data]).u32();
        e.mem.set_u32(base.wrapping_add(CURRENT_WEATHER), weather);
    }
}

// Translated from 004f0b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegion::Load` (Xbox PDB), `RET 4`: the file. False for a record that
/// is not form type 0x37. Otherwise `TESForm::LoadForm`, `00484ab0` with 0,
/// then each chunk until the tag is 0 or the record ends: `EDID` (at most
/// 0x200 bytes into a stack block of the chunk's size, vtable `+0x134`),
/// `OBND` (vtable `+0xe0`), `WNAM` (the world space id, `004f1260`), `RPLI`
/// (a new point-list object, 0x28 bytes, `004f7680` with 1 unless the
/// allocation failed, given the word read at +0x20 and appended to
/// `pPointLists`), `RPLD` (that object's `LoadPoints` when there is one),
/// and the region data chunks (`RDAT`, `RDOB`, `RDSB`, `RDID`, `RDSD`,
/// `RDMD`, `RDSI`, `RDLN`, `RDOJ`, `ICON`, `RDMP`, `RDOT`, `RDGS`, `RDWT`)
/// through the `TES`'s region data manager. Afterwards each point list that
/// `004f81e0(1)` does not keep is removed and deleted, the walk restarting
/// at the head. Answers true.
// Translation incomplete: 004f0b10 - the SEH frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not translated.
pub fn tes_region_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x37 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    e.call(FORM_SET_FLAG, &args![this, 0u32]);
    e.with_stack(4, |e, current| {
        let current = current.addr();
        loop {
            let tag = e.call(GET_TES_CHUNK, &args![file]).u32();
            if tag == 0 {
                break;
            }
            match tag {
                CHUNK_EDID => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size.wrapping_add(15) & !15, |e, block| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, block, 0x200u32]);
                        e.vcall(this.addr(), SET_EDITOR_ID_SLOT, &args![block]);
                    });
                }
                CHUNK_OBND => {
                    e.vcall(this.addr(), LOAD_BOUNDS_SLOT, &args![file]);
                }
                CHUNK_WNAM => {
                    let id = e.with_stack(4, |e, word| {
                        e.call(GET_CHUNK_DATA, &args![file, word]);
                        e.mem.u32(word.addr())
                    });
                    fn_004f1260(e, this, id);
                }
                CHUNK_RPLI => {
                    let value = e.with_stack(4, |e, word| {
                        e.call(GET_CHUNK_DATA, &args![file, word]);
                        e.mem.u32(word.addr())
                    });
                    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
                    let list = if block != 0 {
                        e.call(POINT_LIST_CONSTRUCT, &args![block, 1u32]).u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(current, list);
                    e.call(POINT_LIST_SET_20, &args![list, value]);
                    let lists = e.call(POINT_LISTS_OF, &args![this]).u32();
                    e.call(LIST_APPEND, &args![lists, current]);
                }
                CHUNK_RPLD => {
                    let list = e.mem.u32(current);
                    if list != 0 {
                        e.call(POINT_LIST_LOAD_POINTS, &args![list, file]);
                    }
                }
                CHUNK_RDAT | CHUNK_RDOB | CHUNK_RDSB | CHUNK_RDID | CHUNK_RDSD | CHUNK_RDMD
                | CHUNK_RDSI | CHUNK_RDLN | CHUNK_RDOJ | CHUNK_ICON | CHUNK_RDMP | CHUNK_RDOT
                | CHUNK_RDGS | CHUNK_RDWT => {
                    let tes = e.global::<u32>(TES_SINGLETON);
                    let manager = e.call(REGION_DATA_MANAGER_OF, &args![tes]).u32();
                    e.vcall(manager, LOAD_REGION_DATA_SLOT, &args![file, this]);
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    let mut node = e.call(POINT_LISTS_OF, &args![this]).u32();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let list = e.mem.u32(item_address);
        if e.call(POINT_LIST_KEEP, &args![list, 1u32]).bool() {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        e.with_stack(4, |e, item| {
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let list = e.mem.u32(item_address);
            e.mem.set_u32(item.addr(), list);
            let lists = e.call(POINT_LISTS_OF, &args![this]).u32();
            e.call(LIST_REMOVE, &args![lists, item]);
            let list = e.mem.u32(item.addr());
            if list != 0 {
                e.call(POINT_LIST_DELETE, &args![list, 1u32]);
            }
        });
        node = e.call(POINT_LISTS_OF, &args![this]).u32();
    }
    true
}

/// The save/load game singleton pointer (`011de45c`) `TESRegion::SaveGame`
/// writes through.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `00825c00` (`thiscall` on the save/load game, no stack word): answers the
/// address the region count is written to at the end.
const SAVE_POSITION: u32 = 0x0082_5c00;
/// `008579b0` (`thiscall`, `RET 8`: address, size): writes bytes.
const SAVE_WRITE: u32 = 0x0085_79b0;
/// `TESSaveLoadGame::SaveNumericID` (`00857a10`, `thiscall`, `RET 8`).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// `004169d0` (`thiscall` on the `TES`): the region list's holder (the
/// list's first node is at +4).
const REGION_LIST_OWNER: u32 = 0x0041_69d0;
/// `008256d0` (`thiscall` on a node): the end-of-list test.
const NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `0084e3a0` (`thiscall`): the form's id.
const FORM_ID: u32 = 0x0084_e3a0;
/// `00526ae0` (`thiscall` on the region): a float in `ST0`.
const REGION_VALUE_00526AE0: u32 = 0x0052_6ae0;

// Translated from 004f1300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegion::SaveGame` (Xbox PDB; no argument): writes a 16-bit count
/// placeholder (0), then for each node of the `TES`'s region list (from
/// `004169d0` + 4, until a null node or `008256d0` answers true) the
/// region's form id (`SaveNumericID`, 0 for an empty item) and its float
/// from `00526ae0` (0.0 for an empty item), 4 bytes each; at the end the
/// count goes to the address `00825c00` answered first.
pub fn tes_region_save_game(e: &mut Engine) {
    e.with_stack(0x10, |e, frame| {
        let count_at = frame.addr();
        let id_at = count_at + 4;
        let value_at = count_at + 8;
        e.mem.set_u16(count_at, 0);
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        let position = e.call(SAVE_POSITION, &args![save]).u32();
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_WRITE, &args![save, count_at, 2u32]);
        let tes = e.global::<u32>(TES_SINGLETON);
        let owner = e.call(REGION_LIST_OWNER, &args![tes]).u32();
        let mut node = if owner == 0 { 0 } else { owner.wrapping_add(4) };
        while node != 0 {
            if e.call(NODE_IS_EMPTY, &args![node]).bool() {
                break;
            }
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let region = e.mem.u32(item_address);
            e.mem.set_u32(id_at, 0);
            e.mem.set_u32(value_at, 0);
            if region != 0 {
                let id = e.call(FORM_ID, &args![region]).u32();
                e.mem.set_u32(id_at, id);
                let value = e.call(REGION_VALUE_00526AE0, &args![region]).f64();
                e.mem.set_u32(value_at, x87_store32(value));
            }
            let save = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(SAVE_NUMERIC_ID, &args![save, id_at, 4u32]);
            let save = e.global::<u32>(SAVE_LOAD_GAME);
            e.call(SAVE_WRITE, &args![save, value_at, 4u32]);
            let count = e.mem.u16(count_at).wrapping_add(1);
            e.mem.set_u16(count_at, count);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let count = e.mem.u16(count_at);
        e.mem.set_u16(position, count);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_1300, tes_region_save_game()),
        entry!(0x004f_1260, fn_004f1260(Ptr, u32)),
        entry!(0x004f_0ef0, fn_004f0ef0(Ptr)),
        entry!(0x004f_0f80, fn_004f0f80(Ptr)),
        entry!(0x004f_1050, tes_region_update_weather(Ptr)),
        entry!(0x004f_0b10, tes_region_load(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FORM_VTABLE: u32 = 0x00ab_0000;
    const EDITOR_ID_FN: u32 = 0x00ab_1000;
    const BOUNDS_FN: u32 = 0x00ab_1004;
    const MANAGER_VTABLE: u32 = 0x00ac_0000;
    const LOAD_REGION_DATA_FN: u32 = 0x00ac_1000;
    const DATA_VTABLE: u32 = 0x00ad_0000;
    const DATA_INIT_FN: u32 = 0x00ad_1000;

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

    /// List doubles acting on two-word nodes.
    fn lists(e: &mut Engine) {
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
    }

    fn region(e: &mut Engine) -> u32 {
        let r = e.mem.alloc(0x40);
        e.mem.set_u32(r, FORM_VTABLE);
        r
    }

    fn list_of(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        head
    }

    #[test]
    fn fn_004f1260_sets_the_world_space() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global::<u32>(TES_SINGLETON, 0x7777);
        e.register(REGION_TEST, |_, _| 1u32.into_ret());
        e.register(TES_TEST, |_, _| 0u32.into_ret());
        e.register(WORLD_SPACE_SET, |_, _| Ret::default());
        let r = region(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x004f_1260, &args![r, 0x5000u32]);
        assert_eq!(e.mem.u32(r + 0x20), 0x5000);
        assert_eq!(calls_to(&e, WORLD_SPACE_SET), vec![vec![0x5000, 1]]);
        assert_eq!(calls_to(&e, TES_TEST), vec![vec![0x7777]]);
        e.register(TES_TEST, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004f_1260, &args![r, 0x5000u32]);
        assert!(calls_to(&e, WORLD_SPACE_SET).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x004f_1260, &args![r, 0u32]);
        assert!(calls_to(&e, REGION_TEST).is_empty());
    }

    #[test]
    fn fn_004f0ef0_empties_data_and_point_lists() {
        let mut e = Engine::new();
        lists(&mut e);
        e.register(DATA_LIST_CLEAR, |_, _| Ret::default());
        e.register(POINT_LIST_DELETE, |_, _| Ret::default());
        let r = region(&mut e);
        let point_lists = list_of(&mut e, &[0x111, 0x222]);
        e.mem.set_u32(r + 0x18, 0x1818);
        e.mem.set_u32(r + 0x1c, point_lists);
        e.call_log = Some(vec![]);
        e.call(0x004f_0ef0, &args![r]);
        assert_eq!(calls_to(&e, DATA_LIST_CLEAR), vec![vec![0x1818]]);
        assert_eq!(
            calls_to(&e, POINT_LIST_DELETE),
            vec![vec![0x111, 1], vec![0x222, 1]]
        );
        // Without lists nothing runs.
        let r = region(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x004f_0ef0, &args![r]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_004f0f80_resolves_the_world_space_and_inits_the_data() {
        let mut e = Engine::new();
        lists(&mut e);
        e.map(0x011c_3000, 0x1000);
        e.register(FORM_SKIPS_INIT, |_, _| 0u32.into_ret());
        e.register(WORD_AT_20, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(GET_FILE, |_, _| 0x7000u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, a| (a[0] + 0x10).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(REGION_TEST, |_, _| 0u32.into_ret());
        e.register(FORM_SET_FLAG, |_, _| Ret::default());
        e.register(DATA_LIST_OF, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(DATA_INIT_FN, |_, _| Ret::default());
        e.put_vtable(DATA_VTABLE, &[0, 0, 0, DATA_INIT_FN]);
        let r = region(&mut e);
        e.mem.set_u32(r + 0x20, 0x40);
        let d1 = e.mem.alloc(4);
        let d2 = e.mem.alloc(4);
        e.mem.set_u32(d1, DATA_VTABLE);
        e.mem.set_u32(d2, DATA_VTABLE);
        let data = list_of(&mut e, &[d1, d2]);
        e.mem.set_u32(r + 0x18, data);
        e.call_log = Some(vec![]);
        e.call(0x004f_0f80, &args![r]);
        assert_eq!(e.mem.u32(r + 0x20), 0x50);
        assert_eq!(calls_to(&e, DATA_INIT_FN), vec![vec![d1, r], vec![d2, r]]);
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![r, 1]]);
        e.register(FORM_SKIPS_INIT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004f_0f80, &args![r]);
        assert!(calls_to(&e, FORM_SET_FLAG).is_empty());
    }

    #[test]
    fn update_weather_picks_from_the_weather_data() {
        let mut e = Engine::new();
        e.register(DATA_LIST_FIND, |_, a| (a[0] + a[1]).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(GET_RANDOM_WEATHER, |_, a| (a[0] + 0x100).into_ret());
        let r = region(&mut e);
        e.mem.set_u32(r + 0x24, 0x9999);
        e.call(0x004f_1050, &args![r]);
        assert_eq!(e.mem.u32(r + 0x24), 0);
        e.mem.set_u32(r + 0x18, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x004f_1050, &args![r]);
        assert_eq!(e.mem.u32(r + 0x24), 0x1103);
        let cast = calls_to(&e, DYNAMIC_CAST);
        assert_eq!(
            cast,
            vec![vec![0x1003, 0, CAST_DATA_TYPE, CAST_WEATHER_DATA_TYPE, 0]]
        );
    }

    fn loader(chunks: Vec<(u32, Vec<u8>)>) -> Engine {
        let mut e = Engine::new();
        lists(&mut e);
        e.map(0x011c_3000, 0x1000);
        e.set_global::<u32>(TES_SINGLETON, 0x7777);
        let state: Chunks = Rc::new(RefCell::new((0, chunks)));
        e.register(GET_TES_FORM, |_, _| 0x37u32.into_ret());
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
            e.mem.write(a[1], &data[..data.len().min(a[2] as usize)]);
            Ret::default()
        });
        let s = state;
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            let data = s.borrow().1[s.borrow().0].1.clone();
            e.mem.write(a[1], &data);
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(POINT_LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(POINT_LISTS_OF, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(REGION_DATA_MANAGER_OF, |_, _| 0x00ae_0000u32.into_ret());
        e.map(0x00ae_0000, 4);
        e.mem.set_u32(0x00ae_0000, MANAGER_VTABLE);
        let mut slots = vec![0u32; 10];
        slots[9] = LOAD_REGION_DATA_FN;
        e.put_vtable(MANAGER_VTABLE, &slots);
        let mut slots = vec![0u32; 0x4e];
        slots[0x134 / 4] = EDITOR_ID_FN;
        slots[0xe0 / 4] = BOUNDS_FN;
        e.put_vtable(FORM_VTABLE, &slots);
        for addr in [
            LOAD_FORM,
            FORM_SET_FLAG,
            EDITOR_ID_FN,
            BOUNDS_FN,
            LOAD_REGION_DATA_FN,
            POINT_LIST_SET_20,
            POINT_LIST_LOAD_POINTS,
            LIST_APPEND,
            REGION_TEST,
            POINT_LIST_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(POINT_LIST_KEEP, |_, _| 1u32.into_ret());
        e
    }

    #[test]
    fn load_reads_the_region_chunks() {
        let mut e = loader(vec![
            (CHUNK_EDID, b"R\0".to_vec()),
            (CHUNK_OBND, vec![0; 12]),
            (CHUNK_WNAM, 0x60u32.to_le_bytes().to_vec()),
            (CHUNK_RPLD, vec![]),
            (CHUNK_RPLI, 0x33u32.to_le_bytes().to_vec()),
            (CHUNK_RPLD, vec![]),
            (CHUNK_RDAT, vec![]),
            (CHUNK_RDWT, vec![]),
            (0x1234_5678, vec![]),
        ]);
        let r = region(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_0b10, &args![r, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, FORM_SET_FLAG), vec![vec![r, 0]]);
        assert_eq!(calls_to(&e, EDITOR_ID_FN).len(), 1);
        assert_eq!(e.mem.u32(r + 0x20), 0x60);
        let constructed = calls_to(&e, POINT_LIST_CONSTRUCT);
        assert_eq!(constructed[0][1], 1);
        let list = constructed[0][0];
        assert_eq!(calls_to(&e, POINT_LIST_SET_20), vec![vec![list, 0x33]]);
        // Only the RPLD after the RPLI loads.
        assert_eq!(
            calls_to(&e, POINT_LIST_LOAD_POINTS),
            vec![vec![list, 0x9000]]
        );
        assert_eq!(
            calls_to(&e, LOAD_REGION_DATA_FN),
            vec![vec![0x00ae_0000, 0x9000, r]; 2]
        );
        // Another record type: false.
        e.register(GET_TES_FORM, |_, _| 0x36u32.into_ret());
        assert!(!e.call(0x004f_0b10, &args![r, 0x9000u32]).bool());
    }

    #[test]
    fn load_drops_the_point_lists_not_kept() {
        let mut e = loader(vec![(0, vec![])]);
        let r = region(&mut e);
        let point_lists = list_of(&mut e, &[0x111, 0x222, 0x333]);
        e.mem.set_u32(r + 0x1c, point_lists);
        // 0x222 is not kept.
        e.register(POINT_LIST_KEEP, |_, a| u32::from(a[0] != 0x222).into_ret());
        e.register(LIST_REMOVE, |e, a| {
            // Unlink the node holding the item.
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            while e.mem.u32(node + 4) != 0 {
                let next = e.mem.u32(node + 4);
                if e.mem.u32(next) == item {
                    let after = e.mem.u32(next + 4);
                    e.mem.set_u32(node + 4, after);
                    break;
                }
                node = next;
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_0b10, &args![r, 0x9000u32]).bool());
        assert_eq!(calls_to(&e, POINT_LIST_DELETE), vec![vec![0x222, 1]]);
        assert_eq!(calls_to(&e, LIST_REMOVE).len(), 1);
        // The walk restarted at the head: 0x111 asked twice.
        let asked: Vec<u32> = calls_to(&e, POINT_LIST_KEEP).iter().map(|c| c[0]).collect();
        assert_eq!(asked, [0x111, 0x222, 0x111, 0x333]);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn save_game_writes_each_region_and_the_count() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<u32>(TES_SINGLETON, 0x7000);
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x8000);
        let position = e.mem.alloc(4);
        let holder = e.mem.alloc(12);
        let second = e.mem.alloc(8);
        let region = e.mem.alloc(0x30);
        e.mem.set_u32(holder + 4, region);
        e.mem.set_u32(holder + 8, second);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, 0);
        e.register_double(SAVE_POSITION, move |_, _| position.into_ret());
        e.register_double(REGION_LIST_OWNER, move |_, _| holder.into_ret());
        e.register(NODE_IS_EMPTY, |_, _| 0u32.into_ret());
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FORM_ID, |_, _| 0x1234u32.into_ret());
        e.register(REGION_VALUE_00526AE0, |_, _| 2.5f32.into_ret());
        let written: Rc<RefCell<Vec<(u32, Vec<u8>)>>> = Rc::default();
        let log = written.clone();
        e.register_double(SAVE_WRITE, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.bytes(a[1], a[2])));
            Ret::default()
        });
        let log = written.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            log.borrow_mut().push((1, e.mem.bytes(a[1], a[2])));
            Ret::default()
        });
        e.call(0x004f_1300, &args![]);
        let written = written.borrow();
        assert_eq!(written.len(), 5);
        assert_eq!(written[0], (0x8000, vec![0, 0]));
        assert_eq!(written[1], (1, 0x1234u32.to_le_bytes().to_vec()));
        assert_eq!(written[2], (0x8000, 2.5f32.to_le_bytes().to_vec()));
        assert_eq!(written[3], (1, vec![0; 4]));
        assert_eq!(written[4], (0x8000, vec![0; 4]));
        assert_eq!(e.mem.u16(position), 2);
    }

    #[test]
    fn save_game_without_regions_writes_a_zero_count() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let position = e.mem.alloc(4);
        e.mem.set_u16(position, 0xffff);
        e.register_double(SAVE_POSITION, move |_, _| position.into_ret());
        e.register(REGION_LIST_OWNER, |_, _| 0u32.into_ret());
        e.register(SAVE_WRITE, |_, _| Ret::default());
        e.call(0x004f_1300, &args![]);
        assert_eq!(e.mem.u16(position), 0);
    }
}
