//! `NiXenonRenderer/run_00e7b200` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Direct3D 9 adapter description the renderer builds: per adapter its modes
//! (`NiDX9AdapterDesc::ModeDesc` records in an `NiTPrimitiveArray`, the RTTI names of the
//! virtual tables these functions store), and per device type the display formats with their
//! render-target formats (`NiTPointerMap<_D3DFORMAT, DisplayFormatInfo::RenderTargetInfo *>`
//! of `NiDX9DeviceDesc`). The Direct3D object is called through its vtable in the C style
//! (the object is `this` and the first stack word, so `e.vcall(object, slot, ..)` is the same
//! call); the slots are named here by the order of the `IDirect3D9` interface (0x14
//! `GetAdapterIdentifier`, 0x18 `GetAdapterModeCount`, 0x1c `EnumAdapterModes`, 0x20
//! `GetAdapterDisplayMode`, 0x24 `CheckDeviceType`, 0x28 `CheckDeviceFormat`, 0x2c
//! `CheckDeviceMultiSampleType`, 0x30 `CheckDepthStencilMatch`, 0x38 `GetDeviceCaps`),
//! which the arguments the code passes match.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTArray, NiTListItem, NiTPointerList, NiTPointerMap};

const GET_ADAPTER_IDENTIFIER: u32 = 0x14;
const GET_ADAPTER_MODE_COUNT: u32 = 0x18;
const ENUM_ADAPTER_MODES: u32 = 0x1c;
const GET_ADAPTER_DISPLAY_MODE: u32 = 0x20;
const CHECK_DEVICE_TYPE: u32 = 0x24;
const CHECK_DEVICE_FORMAT: u32 = 0x28;
const CHECK_DEVICE_MULTI_SAMPLE_TYPE: u32 = 0x2c;
const CHECK_DEPTH_STENCIL_MATCH: u32 = 0x30;
const GET_DEVICE_CAPS: u32 = 0x38;

/// The allocations `00aa13e0` (`cdecl`, size) and `00aa1070` (`cdecl`, size; the map's
/// buckets and the array's elements), the sized delete `00aa1460` (`cdecl`: block, size) and
/// `memset` (`00ec61c0`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMSET: u32 = 0x00ec_61c0;
/// `004dcbe0` (`renderer.cpp`; `cdecl`, an index): the render-target format for an index.
const FORMAT_FOR_INDEX: u32 = 0x004d_cbe0;
/// The map's insertion `00b57ea0` (unnamed, `bsshader`; `thiscall` on the map: key, value).
const MAP_SET_AT: u32 = 0x00b5_7ea0;
/// The nine depth formats tried for each render target (`011bf140`, nine words).
const DEPTH_FORMATS_011BF140: u32 = 0x011b_f140;
const DEPTH_FORMAT_COUNT: u32 = 9;
/// The virtual tables the constructors store: the map's base and final ones (RTTI
/// `NiTMapBase<..., _D3DFORMAT, RenderTargetInfo *>`, `NiTPointerMap<_D3DFORMAT,
/// DisplayFormatInfo::RenderTargetInfo *>`) and the mode array's (`NiTArray<ModeDesc *, ...>`,
/// `NiTPrimitiveArray<NiDX9AdapterDesc::ModeDesc *>`).
const MAP_BASE_VTABLE: u32 = 0x010e_ec54;
const MAP_VTABLE: u32 = 0x010e_ec7c;
const ARRAY_BASE_VTABLE: u32 = 0x010e_ec74;
const ARRAY_VTABLE: u32 = 0x010e_ec9c;
/// The list-item pool: its lock (`011c5f80`: the `this` of the lock enter `0040fbf0`,
/// `thiscall` with one word, and the owner word), the lock's entry count (`011c5f84`) that the
/// inlined leave decrements, the free list (`011c5f58`) and its refill `0043a080` (no words).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;
const POOL_REFILL: u32 = 0x0043_a080;
/// The array's resize `00a90180` (`nioldscreenspace.cpp`; `thiscall` on the array, the new
/// capacity) and the bit size of a format `00e7acb0` (`cdecl`, the format).
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
const FORMAT_BIT_SIZE: u32 = 0x00e7_acb0;

/// Sizes the code allocates: a render-target record, a display-format record, a mode record
/// and a device description.
const RENDER_TARGET_INFO_SIZE: u32 = 0x48;
const DISPLAY_FORMAT_INFO_SIZE: u32 = 0x18;
const MODE_DESC_SIZE: u32 = 0x1c;
const DEVICE_DESC_SIZE: u32 = 0x144;
/// Device description: the device type (+0), the caps `GetDeviceCaps` fills (+4), the list of
/// display-format records (+0x134) and the byte for "the display format works windowed"
/// (+0x140).
const DEVICE_CAPS: u32 = 4;
const DEVICE_FORMATS: u32 = 0x134;
const DEVICE_WINDOWED: u32 = 0x140;
/// Adapter description: the adapter (+0), the identifier (+4), the mode array (+0x450) and the
/// two device descriptions (+0x460 for device type 1, +0x464 for 2).
const ADAPTER_IDENTIFIER: u32 = 4;
const ADAPTER_MODES: u32 = 0x450;
const ADAPTER_DEVICE_1: u32 = 0x460;
const ADAPTER_DEVICE_2: u32 = 0x464;

/// Takes a cleared item from the list-item pool, as the code inlines it: the pool's lock
/// entered, the free list refilled when empty, the head taken and cleared, the lock left
/// (count down by one, the owner word cleared at zero).
fn take_list_item(e: &mut Engine) -> Ptr<NiTListItem> {
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    let mut item = e.mem.u32(NODE_POOL_FREE_LIST);
    if item == 0 {
        e.call(POOL_REFILL, &args![]);
        item = e.mem.u32(NODE_POOL_FREE_LIST);
    }
    let next = e.mem.u32(item);
    e.mem.set_u32(NODE_POOL_FREE_LIST, next);
    let item: Ptr<NiTListItem> = Ptr::new(item);
    e.set(item, NiTListItem::m_element, 0);
    e.set(item, NiTListItem::m_pkNext, 0);
    e.set(item, NiTListItem::m_pkPrev, 0);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
    item
}

/// Appends `element` to `list` with a pool item (tail insertion).
fn add_tail(e: &mut Engine, list: Ptr<NiTPointerList>, element: u32) {
    let item = take_list_item(e);
    e.set(item, NiTListItem::m_element, element);
    e.set(item, NiTListItem::m_pkNext, 0);
    let tail = e.get(list, NiTPointerList::m_pkTail);
    e.set(item, NiTListItem::m_pkPrev, tail);
    let tail = e.get(list, NiTPointerList::m_pkTail);
    if tail != 0 {
        e.set(
            Ptr::<NiTListItem>::new(tail),
            NiTListItem::m_pkNext,
            item.addr(),
        );
    } else {
        e.set(list, NiTPointerList::m_pkHead, item.addr());
    }
    let count = e.get(list, NiTPointerList::m_uiCount).wrapping_add(1);
    e.set(list, NiTPointerList::m_uiCount, count);
    e.set(list, NiTPointerList::m_pkTail, item.addr());
}

// Translated from 00e7b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7b200`): the display-format record's constructor
/// (`DisplayFormatInfo`, the class its map's RTTI names), `thiscall` with six words: the
/// Direct3D object, adapter, device type, the display format, the full-screen byte and the
/// windowed byte. Makes the empty render-target map at +8 (0x25 buckets of `00aa1070`, cleared
/// by `memset`, the base then the final virtual table), stores the format (+0), the windowed
/// byte (+4) and the full-screen byte (+5). Then for format indexes 1 to 0x12 (`004dcbe0`)
/// usable as a render target (`CheckDeviceFormat` with usage 1, surface): a cleared 0x48-byte
/// record; the multisample counts 1 to 16 that work windowed (when the windowed byte is set)
/// and full screen (when the full-screen byte is set) on the display format, one bit each; for
/// each of the nine depth formats at `011bf140` usable as a depth buffer (usage 2) and
/// matching the render target (`CheckDepthStencilMatch`), its byte and the byte 0x24 later
/// get the windowed and full-screen bytes, and for each multisample count (the test of the
/// count's bit or the mask never fails) that works on the depth format windowed, the 16-bit
/// mask at +2 gets the bit (count 1 also stores its quality level's low byte at +1), full
/// screen likewise at +0x26 and +0x25. A record with a usable depth format goes into the map
/// (`00b57ea0`) under the format; the others are deleted (`00aa1460`, 0x48 bytes). Answers
/// `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e7b260(
    e: &mut Engine,
    this: Ptr,
    direct3d: u32,
    adapter: u32,
    device_type: u32,
    display_format: u32,
    full_screen: u8,
    windowed: u8,
) -> Ptr {
    let map: Ptr<NiTPointerMap> = Ptr::new(this.addr().wrapping_add(8));
    e.mem.set_u32(map.addr(), MAP_BASE_VTABLE);
    e.set(map, NiTPointerMap::m_uiHashSize, 0x25);
    e.set(map, NiTPointerMap::m_uiCount, 0);
    let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
    e.set(map, NiTPointerMap::m_ppkHashTable, buckets);
    let size = e.get(map, NiTPointerMap::m_uiHashSize).wrapping_mul(4);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u32(map.addr(), MAP_VTABLE);
    e.mem.set_u32(this.addr(), display_format);
    e.mem.set_u8(this.addr().wrapping_add(5), full_screen);
    e.mem.set_u8(this.addr().wrapping_add(4), windowed);
    // The quality-level words the multisample checks write (three stack slots).
    e.with_stack(0xc, |e, levels| {
        let display_level = levels.addr();
        let windowed_level = levels.addr() + 4;
        let full_screen_level = levels.addr() + 8;
        for index in 1..0x13u32 {
            let format = e.call(FORMAT_FOR_INDEX, &args![index]).u32();
            let usable = e
                .vcall(
                    direct3d,
                    CHECK_DEVICE_FORMAT,
                    &args![adapter, device_type, display_format, 1u32, 1u32, format],
                )
                .i32();
            if usable < 0 {
                continue;
            }
            let record = e.call(NEW_OBJECT, &args![RENDER_TARGET_INFO_SIZE]).u32();
            if record != 0 {
                for word in 0..RENDER_TARGET_INFO_SIZE / 4 {
                    e.mem.set_u32(record + 4 * word, 0);
                }
            }
            let mut windowed_samples = 0u32;
            let mut full_screen_samples = 0u32;
            let mut bit = 1u32;
            for samples in 1..=16u32 {
                if windowed != 0
                    && e.vcall(
                        direct3d,
                        CHECK_DEVICE_MULTI_SAMPLE_TYPE,
                        &args![
                            adapter,
                            device_type,
                            display_format,
                            1u32,
                            samples,
                            display_level
                        ],
                    )
                    .i32()
                        >= 0
                {
                    windowed_samples |= bit;
                }
                if full_screen != 0
                    && e.vcall(
                        direct3d,
                        CHECK_DEVICE_MULTI_SAMPLE_TYPE,
                        &args![
                            adapter,
                            device_type,
                            display_format,
                            0u32,
                            samples,
                            display_level
                        ],
                    )
                    .i32()
                        >= 0
                {
                    full_screen_samples |= bit;
                }
                bit = bit.rotate_left(1);
            }
            let mut usable_depth = false;
            for depth_index in 0..DEPTH_FORMAT_COUNT {
                let cursor = record.wrapping_add(4 * depth_index);
                let depth = e
                    .mem
                    .u32(DEPTH_FORMATS_011BF140.wrapping_add(4 * depth_index));
                if e.vcall(
                    direct3d,
                    CHECK_DEVICE_FORMAT,
                    &args![adapter, device_type, display_format, 2u32, 1u32, depth],
                )
                .i32()
                    < 0
                {
                    continue;
                }
                if e.vcall(
                    direct3d,
                    CHECK_DEPTH_STENCIL_MATCH,
                    &args![adapter, device_type, display_format, format, depth],
                )
                .i32()
                    < 0
                {
                    continue;
                }
                e.mem.set_u8(cursor, windowed);
                usable_depth = true;
                e.mem.set_u8(cursor.wrapping_add(0x24), full_screen);
                let mut bit = 1u32;
                for samples in 1..=16u32 {
                    // The bit is never zero; the exe tests it with the mask.
                    if (bit | windowed_samples) != 0
                        && e.vcall(
                            direct3d,
                            CHECK_DEVICE_MULTI_SAMPLE_TYPE,
                            &args![adapter, device_type, depth, 1u32, samples, windowed_level],
                        )
                        .i32()
                            >= 0
                    {
                        if samples == 1 {
                            let level = e.mem.u8(windowed_level);
                            e.mem.set_u8(cursor.wrapping_add(1), level);
                        }
                        let mask = e.mem.u16(cursor.wrapping_add(2)) | bit as u16;
                        e.mem.set_u16(cursor.wrapping_add(2), mask);
                    }
                    if (bit | full_screen_samples) != 0
                        && e.vcall(
                            direct3d,
                            CHECK_DEVICE_MULTI_SAMPLE_TYPE,
                            &args![
                                adapter,
                                device_type,
                                depth,
                                0u32,
                                samples,
                                full_screen_level
                            ],
                        )
                        .i32()
                            >= 0
                    {
                        if samples == 1 {
                            let level = e.mem.u8(full_screen_level);
                            e.mem.set_u8(cursor.wrapping_add(0x25), level);
                        }
                        let mask = e.mem.u16(cursor.wrapping_add(0x26)) | bit as u16;
                        e.mem.set_u16(cursor.wrapping_add(0x26), mask);
                    }
                    bit = bit.rotate_left(1);
                }
            }
            if usable_depth {
                e.call(MAP_SET_AT, &args![map, format, record]);
            } else {
                e.call(SIZED_DELETE, &args![record, RENDER_TARGET_INFO_SIZE]);
            }
        }
    });
    this
}

// Translated from 00e7b590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7b200`): the device description's constructor
/// (`NiDX9DeviceDesc`, the class the record list's map names), `thiscall` with five words: the
/// Direct3D object, adapter, device type, the adapter's display format and the list of formats
/// to enumerate. Stores the device type, clears the record list (+0x134) and asks for the
/// device's caps into +4; when that fails the first caps word is cleared. Otherwise, for each
/// nonzero format of the list and each of its modes (`GetAdapterModeCount`,
/// `EnumAdapterModes`): an enumerated mode sets the full-screen byte (its `CheckDeviceType`
/// for full screen, display format = the list's format, back buffer = the mode's format, is
/// made and its result not used), and `CheckDeviceType` windowed on the adapter's display
/// format and the mode's format sets the windowed byte (both checks use the mode buffer, which
/// keeps the previous mode when an enumeration fails). With either byte set and no record yet
/// for the mode's format, a record is made ([`fn_00e7b260`] on 0x18 bytes, null when the
/// allocation fails) and appended. Finally +0x140 is set when the record of the adapter's display
/// format has its windowed byte (+4) set. Answers `this`.
pub fn fn_00e7b590(
    e: &mut Engine,
    this: Ptr,
    direct3d: u32,
    adapter: u32,
    device_type: u32,
    display_format: u32,
    formats: Ptr<NiTPointerList>,
) -> Ptr {
    let this_addr = this.addr();
    e.mem.set_u32(this_addr, device_type);
    let records: Ptr<NiTPointerList> = Ptr::new(this_addr.wrapping_add(DEVICE_FORMATS));
    e.set(records, NiTPointerList::m_uiCount, 0);
    e.set(records, NiTPointerList::m_pkHead, 0);
    e.set(records, NiTPointerList::m_pkTail, 0);
    let caps = this_addr.wrapping_add(DEVICE_CAPS);
    let device = e.mem.u32(this_addr);
    let status = e
        .vcall(direct3d, GET_DEVICE_CAPS, &args![adapter, device, caps])
        .i32();
    if status < 0 {
        e.mem.set_u32(caps, 0);
    } else {
        // The mode `EnumAdapterModes` fills (width, height, refresh rate, format).
        e.with_stack(0x10, |e, mode| {
            let mode_format = |e: &Engine| e.mem.u32(mode.addr() + 0xc);
            let mut node = e.mem.u32(formats.addr());
            while node != 0 {
                let item: Ptr<NiTListItem> = Ptr::new(node);
                let next = e.get(item, NiTListItem::m_pkNext);
                let format = e.get(item, NiTListItem::m_element);
                node = next;
                if format == 0 {
                    continue;
                }
                let count = e
                    .vcall(direct3d, GET_ADAPTER_MODE_COUNT, &args![adapter, format])
                    .u32();
                let mut index = 0u32;
                while index < count {
                    let mut full_screen = 0u8;
                    let mut windowed = 0u8;
                    let enumerated = e
                        .vcall(
                            direct3d,
                            ENUM_ADAPTER_MODES,
                            &args![adapter, format, index, mode],
                        )
                        .i32();
                    if enumerated >= 0 {
                        let device = e.mem.u32(this_addr);
                        let back_buffer = mode_format(e);
                        e.vcall(
                            direct3d,
                            CHECK_DEVICE_TYPE,
                            &args![adapter, device, format, back_buffer, 0u32],
                        );
                        full_screen = 1;
                    }
                    let device = e.mem.u32(this_addr);
                    let back_buffer = mode_format(e);
                    if e.vcall(
                        direct3d,
                        CHECK_DEVICE_TYPE,
                        &args![adapter, device, display_format, back_buffer, 1u32],
                    )
                    .i32()
                        >= 0
                    {
                        windowed = 1;
                    }
                    if full_screen != 0 || windowed != 0 {
                        let wanted = mode_format(e);
                        let mut present = false;
                        let mut walk = e.get(records, NiTPointerList::m_pkHead);
                        while walk != 0 {
                            let walk_item: Ptr<NiTListItem> = Ptr::new(walk);
                            let record = e.get(walk_item, NiTListItem::m_element);
                            walk = e.get(walk_item, NiTListItem::m_pkNext);
                            if e.mem.u32(record) == wanted {
                                present = true;
                                break;
                            }
                        }
                        if !present {
                            let block = e.call(NEW_OBJECT, &args![DISPLAY_FORMAT_INFO_SIZE]).u32();
                            let record = if block != 0 {
                                // The exe passes the whole stack words (their upper bytes
                                // those of the list and device-type arguments); the callee
                                // reads only the low byte.
                                let device = e.mem.u32(this_addr);
                                let back_buffer = mode_format(e);
                                fn_00e7b260(
                                    e,
                                    Ptr::new(block),
                                    direct3d,
                                    adapter,
                                    device,
                                    back_buffer,
                                    full_screen,
                                    windowed,
                                )
                                .addr()
                            } else {
                                0
                            };
                            add_tail(e, records, record);
                        }
                    }
                    index += 1;
                }
            }
        });
    }
    let mut windowed = 0u8;
    let mut walk = e.get(records, NiTPointerList::m_pkHead);
    while walk != 0 {
        let walk_item: Ptr<NiTListItem> = Ptr::new(walk);
        let record = e.get(walk_item, NiTListItem::m_element);
        walk = e.get(walk_item, NiTListItem::m_pkNext);
        if e.mem.u32(record) == display_format {
            if e.mem.u8(record.wrapping_add(4)) != 0 {
                windowed = 1;
            }
            break;
        }
    }
    e.mem
        .set_u8(this_addr.wrapping_add(DEVICE_WINDOWED), windowed);
    this
}

// Translated from 00e7b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7b200`): the adapter description's constructor
/// (`NiDX9AdapterDesc`, the class its mode array's RTTI names), `thiscall` with three words: the
/// Direct3D object, the adapter and the list of formats to enumerate. Stores the adapter, makes
/// the mode array at +0x450 (capacity 0x20, grow step 0x10, elements from `00aa1070`, the base
/// then the final virtual table) and fills the identifier at +4 (`GetAdapterIdentifier`, flags
/// 0, result not used). For each nonzero format of the list and each of its modes that
/// enumerates: every mode record with that format, width and height gets the mode's refresh
/// rate appended to its list (+0x10; the search does not stop at a match); with none, a new
/// 0x1c-byte record (width, height, the format at +0xc, a one-item refresh list, the format's
/// bit size from `00e7acb0` at +8; null when the allocation fails) is stored at the array's
/// used size (grown by `00a90180` to size plus grow step when full; the non-null count kept as
/// the exe's set-at does). Then the adapter's display mode is read and the two device
/// descriptions are made ([`fn_00e7b590`] on 0x144 bytes each, device types 1 and 2, null when
/// the allocation fails) into +0x460 and +0x464. Answers `this`.
pub fn fn_00e7b7d0(
    e: &mut Engine,
    this: Ptr,
    direct3d: u32,
    adapter: u32,
    formats: Ptr<NiTPointerList>,
) -> Ptr {
    let this_addr = this.addr();
    e.mem.set_u32(this_addr, adapter);
    let modes: Ptr<NiTArray> = Ptr::new(this_addr.wrapping_add(ADAPTER_MODES));
    e.set(modes, NiTArray::m_usMaxSize, 0x20);
    e.mem.set_u32(modes.addr(), ARRAY_BASE_VTABLE);
    e.set(modes, NiTArray::m_usGrowBy, 0x10);
    e.set(modes, NiTArray::m_usSize, 0);
    e.set(modes, NiTArray::m_usESize, 0);
    let elements = e.call(ALLOCATE_00AA1070, &args![0x80u32]).u32();
    e.set(modes, NiTArray::m_pBase, elements);
    e.mem.set_u32(modes.addr(), ARRAY_VTABLE);
    let identifier = this_addr.wrapping_add(ADAPTER_IDENTIFIER);
    let stored_adapter = e.mem.u32(this_addr);
    e.vcall(
        direct3d,
        GET_ADAPTER_IDENTIFIER,
        &args![stored_adapter, 0u32, identifier],
    );
    // The mode `EnumAdapterModes` fills and the one `GetAdapterDisplayMode` fills (width,
    // height, refresh rate, format each).
    e.with_stack(0x20, |e, frame| {
        let mode = frame.addr();
        let display_mode = frame.addr() + 0x10;
        let mut node = e.mem.u32(formats.addr());
        while node != 0 {
            let item: Ptr<NiTListItem> = Ptr::new(node);
            let format = e.get(item, NiTListItem::m_element);
            node = e.get(item, NiTListItem::m_pkNext);
            if format == 0 {
                continue;
            }
            let stored_adapter = e.mem.u32(this_addr);
            let count = e
                .vcall(
                    direct3d,
                    GET_ADAPTER_MODE_COUNT,
                    &args![stored_adapter, format],
                )
                .u32();
            let mut index = 0u32;
            while index < count {
                let stored_adapter = e.mem.u32(this_addr);
                let enumerated = e
                    .vcall(
                        direct3d,
                        ENUM_ADAPTER_MODES,
                        &args![stored_adapter, format, index, mode],
                    )
                    .i32();
                index += 1;
                if enumerated < 0 {
                    continue;
                }
                let width = e.mem.u32(mode);
                let height = e.mem.u32(mode + 4);
                let refresh = e.mem.u32(mode + 8);
                let mode_format = e.mem.u32(mode + 0xc);
                let mut found = 0u32;
                let mut slot = 0u32;
                while slot < u32::from(e.get(modes, NiTArray::m_usSize)) {
                    let base = e.get(modes, NiTArray::m_pBase);
                    let record = e.mem.u32(base.wrapping_add(4 * slot));
                    slot += 1;
                    if record == 0
                        || e.mem.u32(record.wrapping_add(0xc)) != format
                        || e.mem.u32(record) != width
                        || e.mem.u32(record.wrapping_add(4)) != height
                    {
                        continue;
                    }
                    found = record;
                    let rates: Ptr<NiTPointerList> = Ptr::new(record.wrapping_add(0x10));
                    add_tail(e, rates, refresh);
                }
                if found != 0 {
                    continue;
                }
                let record = e.call(NEW_OBJECT, &args![MODE_DESC_SIZE]).u32();
                if record != 0 {
                    e.mem.set_u32(record, width);
                    e.mem.set_u32(record.wrapping_add(4), height);
                    e.mem.set_u32(record.wrapping_add(0xc), mode_format);
                    let rates: Ptr<NiTPointerList> = Ptr::new(record.wrapping_add(0x10));
                    e.set(rates, NiTPointerList::m_uiCount, 0);
                    e.set(rates, NiTPointerList::m_pkHead, 0);
                    e.set(rates, NiTPointerList::m_pkTail, 0);
                    // Head insertion.
                    let rate = take_list_item(e);
                    e.set(rate, NiTListItem::m_element, refresh);
                    e.set(rate, NiTListItem::m_pkPrev, 0);
                    let head = e.get(rates, NiTPointerList::m_pkHead);
                    e.set(rate, NiTListItem::m_pkNext, head);
                    let head = e.get(rates, NiTPointerList::m_pkHead);
                    if head != 0 {
                        e.set(
                            Ptr::<NiTListItem>::new(head),
                            NiTListItem::m_pkPrev,
                            rate.addr(),
                        );
                    } else {
                        e.set(rates, NiTPointerList::m_pkTail, rate.addr());
                    }
                    let count = e.get(rates, NiTPointerList::m_uiCount).wrapping_add(1);
                    e.set(rates, NiTPointerList::m_uiCount, count);
                    e.set(rates, NiTPointerList::m_pkHead, rate.addr());
                    let bits = e.call(FORMAT_BIT_SIZE, &args![mode_format]).u32();
                    e.mem.set_u32(record.wrapping_add(8), bits);
                }
                // Stored at the used size, as the array's set-at does.
                let at = u32::from(e.get(modes, NiTArray::m_usSize));
                if at >= u32::from(e.get(modes, NiTArray::m_usMaxSize)) {
                    let capacity = u32::from(e.get(modes, NiTArray::m_usGrowBy)).wrapping_add(at);
                    e.call(ARRAY_SET_SIZE, &args![modes, capacity]);
                }
                let slot_address = |e: &Engine| {
                    e.get(modes, NiTArray::m_pBase)
                        .wrapping_add(at.wrapping_mul(4))
                };
                if at >= u32::from(e.get(modes, NiTArray::m_usSize)) {
                    e.set(modes, NiTArray::m_usSize, at.wrapping_add(1) as u16);
                    if record != 0 {
                        let used = e.get(modes, NiTArray::m_usESize).wrapping_add(1);
                        e.set(modes, NiTArray::m_usESize, used);
                    }
                } else if record != 0 {
                    if e.mem.u32(slot_address(e)) == 0 {
                        let used = e.get(modes, NiTArray::m_usESize).wrapping_add(1);
                        e.set(modes, NiTArray::m_usESize, used);
                    }
                } else if e.mem.u32(slot_address(e)) != 0 {
                    let used = e.get(modes, NiTArray::m_usESize).wrapping_add(0xffff);
                    e.set(modes, NiTArray::m_usESize, used);
                }
                let address = slot_address(e);
                e.mem.set_u32(address, record);
            }
        }
        let stored_adapter = e.mem.u32(this_addr);
        e.vcall(
            direct3d,
            GET_ADAPTER_DISPLAY_MODE,
            &args![stored_adapter, display_mode],
        );
        let display_format = e.mem.u32(display_mode + 0xc);
        for (device_type, field) in [(1u32, ADAPTER_DEVICE_1), (2u32, ADAPTER_DEVICE_2)] {
            let block = e.call(NEW_OBJECT, &args![DEVICE_DESC_SIZE]).u32();
            let device = if block != 0 {
                fn_00e7b590(
                    e,
                    Ptr::new(block),
                    direct3d,
                    adapter,
                    device_type,
                    display_format,
                    formats,
                )
                .addr()
            } else {
                0
            };
            e.mem.set_u32(this_addr.wrapping_add(field), device);
        }
    });
    this
}

/// The display-format record's destructor (`00e7b040`, `thiscall`), the pointer lists'
/// `RemoveAll` (`00b71300` for the device's format list, `004ed900` for a mode's refresh
/// list; `thiscall`) and the free (`00aa10f0`, `cdecl`, a block).
const DISPLAY_FORMAT_INFO_DESTRUCT: u32 = 0x00e7_b040;
const FORMAT_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
const REFRESH_LIST_REMOVE_ALL: u32 = 0x004e_d900;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `IDirect3D9::GetAdapterCount` (slot 4).
const GET_ADAPTER_COUNT: u32 = 0x10;
/// The size of an adapter description (`fn_00e7b7d0`'s object).
const ADAPTER_DESC_SIZE: u32 = 0x468;

// Translated from 00e7b550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7b200`): the device description's destructor
/// (`thiscall`, no stack words). Destroys and frees (0x18 bytes) each display-format record
/// of the list at +0x134, then jumps to the list's `RemoveAll` (`00b71300`).
pub fn fn_00e7b550(e: &mut Engine, this: Ptr) {
    let list = this.addr().wrapping_add(DEVICE_FORMATS);
    let mut node = e.mem.u32(list);
    while node != 0 {
        let record = e.mem.u32(node.wrapping_add(8));
        node = e.mem.u32(node);
        if record != 0 {
            e.call(DISPLAY_FORMAT_INFO_DESTRUCT, &args![record]);
            e.call(SIZED_DELETE, &args![record, DISPLAY_FORMAT_INFO_SIZE]);
        }
    }
    e.call(FORMAT_LIST_REMOVE_ALL, &args![list]);
}

// Translated from 00e7bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7b200`): the adapter description's destructor
/// (`thiscall`, no stack words). Each mode record of the array at +0x450 (count `m_usSize`,
/// read again each step) has its refresh list (+0x10) emptied (`004ed900`) and is freed
/// (0x1C bytes); both device descriptions are destroyed ([`fn_00e7b550`]) and freed (0x144
/// bytes); the array gets its base vtable back and its elements are freed (`00aa10f0`).
pub fn fn_00e7bb50(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let modes: Ptr<NiTArray> = Ptr::new(t.wrapping_add(ADAPTER_MODES));
    let mut index = 0u32;
    if e.get(modes, NiTArray::m_usSize) != 0 {
        loop {
            let elements = e.get(modes, NiTArray::m_pBase);
            let record = e.mem.u32(elements.wrapping_add(index * 4));
            if record != 0 {
                e.call(REFRESH_LIST_REMOVE_ALL, &args![record.wrapping_add(0x10)]);
                e.call(SIZED_DELETE, &args![record, MODE_DESC_SIZE]);
            }
            index += 1;
            if index >= u32::from(e.get(modes, NiTArray::m_usSize)) {
                break;
            }
        }
    }
    for field in [ADAPTER_DEVICE_1, ADAPTER_DEVICE_2] {
        let device = e.mem.u32(t.wrapping_add(field));
        if device != 0 {
            fn_00e7b550(e, Ptr::new(device));
            e.call(SIZED_DELETE, &args![device, DEVICE_DESC_SIZE]);
        }
    }
    let elements = e.get(modes, NiTArray::m_pBase);
    e.mem.set_u32(modes.addr(), ARRAY_BASE_VTABLE);
    e.call(FREE_00AA10F0, &args![elements]);
}

// Translated from 00e7bbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7b200`): fills the adapter list (`thiscall`, two words:
/// the `IDirect3D9` and the format list passed on). Stores the adapter count at +0, sizes the
/// array at +4 to it (`00a90180`) and, for each adapter, stores a new adapter description
/// (0x468 bytes, [`fn_00e7b7d0`] with the Direct3D object, the adapter and the list; 0 when
/// the allocation failed) at its index, keeping the array's used size (grown to index + 1)
/// and non-null count as the exe's inlined set-at does.
pub fn fn_00e7bbf0(e: &mut Engine, this: Ptr, direct3d: u32, formats: Ptr<NiTPointerList>) {
    let t = this.addr();
    let count = e.vcall(direct3d, GET_ADAPTER_COUNT, &args![]).u32();
    e.mem.set_u32(t, count);
    let array: Ptr<NiTArray> = Ptr::new(t.wrapping_add(4));
    e.call(ARRAY_SET_SIZE, &args![array, count]);
    let mut index = 0u32;
    while index < e.mem.u32(t) {
        let block = e.call(NEW_OBJECT, &args![ADAPTER_DESC_SIZE]).u32();
        let desc = if block != 0 {
            fn_00e7b7d0(e, Ptr::new(block), direct3d, index, formats).addr()
        } else {
            0
        };
        let size = u32::from(e.get(array, NiTArray::m_usSize));
        let elements = e.get(array, NiTArray::m_pBase);
        let slot = elements.wrapping_add(index * 4);
        if index >= size {
            e.set(array, NiTArray::m_usSize, (index + 1) as u16);
            if desc != 0 {
                let used = e.get(array, NiTArray::m_usESize).wrapping_add(1);
                e.set(array, NiTArray::m_usESize, used);
            }
        } else if desc != 0 {
            if e.mem.u32(slot) == 0 {
                let used = e.get(array, NiTArray::m_usESize).wrapping_add(1);
                e.set(array, NiTArray::m_usESize, used);
            }
        } else if e.mem.u32(slot) != 0 {
            let used = e.get(array, NiTArray::m_usESize).wrapping_add(0xffff);
            e.set(array, NiTArray::m_usESize, used);
        }
        let elements = e.get(array, NiTArray::m_pBase);
        e.mem.set_u32(elements.wrapping_add(index * 4), desc);
        index += 1;
    }
}

/// `NiTArray<NiDX9AdapterDesc*, NiTMallocInterface<NiDX9AdapterDesc*>>`'s vtable
/// (`010eec4c`, RTTI).
const ADAPTER_ARRAY_VTABLE: u32 = 0x010e_ec4c;
/// The memory manager (`011f6238`) and its free (`00aa4060`, `thiscall`, the block).
const MEMORY_MANAGER_011F6238: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE_00AA4060: u32 = 0x00aa_4060;

/// The two array destructors' shared body: `vtable` stored, the elements (`m_pBase`, read
/// first) freed (`00aa10f0`) and, when `flags & 1`, the array given to the memory manager.
fn destroy_array(e: &mut Engine, this: Ptr<NiTArray>, vtable: u32, flags: u32) -> Ptr<NiTArray> {
    let elements = e.get(this, NiTArray::m_pBase);
    e.mem.set_u32(this.addr(), vtable);
    e.call(FREE_00AA10F0, &args![elements]);
    if flags & 1 != 0 {
        e.call(
            MEMORY_MANAGER_FREE_00AA4060,
            &args![MEMORY_MANAGER_011F6238, this],
        );
    }
    this
}

// Translated from 00e7b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTArray<NiDX9AdapterDesc::ModeDesc*, NiTMallocInterface<...>>` (the vtable `010eec74`
/// it stores, RTTI): frees the elements and, when `flags & 1`, the array. Returns `this`.
pub fn fn_00e7b200(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    destroy_array(e, this, ARRAY_BASE_VTABLE, flags)
}

// Translated from 00e7b230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTArray<NiDX9AdapterDesc*, NiTMallocInterface<NiDX9AdapterDesc*>>` (vtable `010eec4c`,
/// RTTI), as [`fn_00e7b200`].
pub fn fn_00e7b230(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    destroy_array(e, this, ADAPTER_ARRAY_VTABLE, flags)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e7_b200,
            fn_00e7b200(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(
            0x00e7_b230,
            fn_00e7b230(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(0x00e7_b550, fn_00e7b550(Ptr)),
        entry!(0x00e7_bb50, fn_00e7bb50(Ptr)),
        entry!(0x00e7_bbf0, fn_00e7bbf0(Ptr, u32, Ptr<NiTPointerList>)),
        entry!(
            0x00e7_b260,
            fn_00e7b260(Ptr, u32, u32, u32, u32, u8, u8) -> Ptr
        ),
        entry!(
            0x00e7_b590,
            fn_00e7b590(Ptr, u32, u32, u32, u32, Ptr<NiTPointerList>) -> Ptr
        ),
        entry!(
            0x00e7_b7d0,
            fn_00e7b7d0(Ptr, u32, u32, Ptr<NiTPointerList>) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    const DIRECT3D_VTABLE: u32 = 0x6000;
    /// Fake slot targets, one per vtable slot (slot offset added).
    const SLOT_BASE: u32 = 0x6100_0000;
    const DISPLAY: u32 = 0x16;
    const DEPTH: u32 = 0x4b;
    const FAILED: u32 = 0x8876_086c;

    #[derive(Clone, Copy)]
    struct Setup {
        /// Modes 640+i by 480 when set, else the three fixed modes.
        distinct_modes: u32,
        depth_usable: bool,
    }

    const FIXED: Setup = Setup {
        distinct_modes: 0,
        depth_usable: true,
    };

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn slot(offset: u32) -> u32 {
        SLOT_BASE + offset
    }

    /// The mode `EnumAdapterModes` gives: (width, height, refresh).
    fn mode_of(setup: Setup, index: u32) -> (u32, u32, u32) {
        if setup.distinct_modes != 0 {
            (640 + index, 480, 60)
        } else {
            [(800, 600, 60), (800, 600, 75), (1024, 768, 60)][index as usize]
        }
    }

    fn ok_if(ok: bool) -> Ret {
        if ok {
            0u32.into_ret()
        } else {
            FAILED.into_ret()
        }
    }

    /// The Direct3D double, the pool and the callees outside the file. Answers the engine and
    /// the Direct3D object.
    fn rig(setup: Setup) -> (Engine, u32) {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..16).map(|i| slot(4 * i)).collect();
        e.put_vtable(DIRECT3D_VTABLE, &slots);
        let direct3d = e.mem.alloc(4);
        e.mem.set_u32(direct3d, DIRECT3D_VTABLE);
        e.register(slot(GET_ADAPTER_IDENTIFIER), |_, _| Ret::default());
        e.register_double(slot(GET_ADAPTER_MODE_COUNT), move |_, a| {
            let count = if a[2] != DISPLAY {
                0
            } else if setup.distinct_modes != 0 {
                setup.distinct_modes
            } else {
                3
            };
            count.into_ret()
        });
        e.register_double(slot(ENUM_ADAPTER_MODES), move |e, a| {
            let (width, height, refresh) = mode_of(setup, a[3]);
            for (i, word) in [width, height, refresh, DISPLAY].into_iter().enumerate() {
                e.mem.set_u32(a[4] + 4 * i as u32, word);
            }
            0u32.into_ret()
        });
        e.register(slot(GET_ADAPTER_DISPLAY_MODE), |e, a| {
            e.mem.set_u32(a[2] + 0xc, DISPLAY);
            0u32.into_ret()
        });
        e.register(slot(CHECK_DEVICE_TYPE), |_, _| 0u32.into_ret());
        // Render targets: only the display format; depth buffers: only DEPTH.
        e.register_double(slot(CHECK_DEVICE_FORMAT), move |_, a| {
            ok_if(match a[4] {
                1 => a[6] == DISPLAY,
                2 => setup.depth_usable && a[6] == DEPTH,
                _ => false,
            })
        });
        // One and four samples work; the quality level is 10 + samples.
        e.register(slot(CHECK_DEVICE_MULTI_SAMPLE_TYPE), |e, a| {
            e.mem.set_u32(a[6], 10 + a[5]);
            ok_if(a[5] == 1 || a[5] == 4)
        });
        e.register(slot(CHECK_DEPTH_STENCIL_MATCH), |_, _| 0u32.into_ret());
        // Device type 2 has no caps.
        e.register(slot(GET_DEVICE_CAPS), |_, a| ok_if(a[2] == 1));

        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(FORMAT_FOR_INDEX, |_, a| {
            (if a[0] == 1 { DISPLAY } else { 0x99 }).into_ret()
        });
        e.register(MAP_SET_AT, |_, _| Ret::default());
        e.register(FORMAT_BIT_SIZE, |_, _| 32u32.into_ret());
        e.register(ARRAY_SET_SIZE, |e, a| {
            let array: Ptr<NiTArray> = Ptr::new(a[0]);
            let old = e.get(array, NiTArray::m_pBase);
            let size = u32::from(e.get(array, NiTArray::m_usSize));
            let new = e.mem.alloc(4 * a[1]);
            for i in 0..size {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(new + 4 * i, word);
            }
            e.set(array, NiTArray::m_pBase, new);
            e.set(array, NiTArray::m_usMaxSize, a[1] as u16);
            Ret::default()
        });
        e.map(NODE_POOL_FREE_LIST, 4);
        e.map(NODE_POOL_LOCK, 8);
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            Ret::default()
        });
        e.register(POOL_REFILL, |e, _| {
            let item = e.mem.alloc(0x0c);
            e.mem.set_u32(item, 0);
            e.mem.set_u32(NODE_POOL_FREE_LIST, item);
            Ret::default()
        });
        e.map(DEPTH_FORMATS_011BF140, 4 * DEPTH_FORMAT_COUNT);
        e.mem.set_u32(DEPTH_FORMATS_011BF140, DEPTH);
        e.call_log = Some(vec![]);
        (e, direct3d)
    }

    /// A format list holding `formats`.
    fn format_list(e: &mut Engine, formats: &[u32]) -> u32 {
        let list = e.mem.alloc(0x0c);
        let mut previous = 0;
        for format in formats {
            let item = e.mem.alloc(0x0c);
            e.mem.set_u32(item + 8, *format);
            e.mem.set_u32(item + 4, previous);
            if previous == 0 {
                e.mem.set_u32(list, item);
            } else {
                e.mem.set_u32(previous, item);
            }
            previous = item;
        }
        e.mem.set_u32(list + 4, previous);
        e.mem.set_u32(list + 8, formats.len() as u32);
        list
    }

    /// The elements of a pointer list, head to tail.
    fn elements(e: &Engine, list: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut item = e.mem.u32(list);
        while item != 0 {
            out.push(e.mem.u32(item + 8));
            item = e.mem.u32(item);
        }
        out
    }

    #[test]
    fn fn_00e7b260_records_the_usable_render_target() {
        let (mut e, direct3d) = rig(FIXED);
        let this = e.mem.alloc(DISPLAY_FORMAT_INFO_SIZE);
        let args = args![this, direct3d, 0u32, 1u32, DISPLAY, 0x101u32, 0x201u32];
        assert_eq!(e.call(0x00e7_b260, &args).u32(), this);
        assert_eq!(e.mem.u32(this), DISPLAY);
        assert_eq!((e.mem.u8(this + 4), e.mem.u8(this + 5)), (1, 1));
        assert_eq!(e.mem.u32(this + 8), MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x25);
        let buckets = e.mem.u32(this + 0x10);
        assert_eq!(calls(&e, MEMSET), vec![vec![buckets, 0, 0x94]]);
        assert_eq!(calls(&e, FORMAT_FOR_INDEX).len(), 0x12);
        let records: Vec<Vec<u32>> = calls(&e, NEW_OBJECT);
        assert_eq!(records, vec![vec![RENDER_TARGET_INFO_SIZE]]);
        let set = calls(&e, MAP_SET_AT);
        assert_eq!(set.len(), 1);
        assert_eq!(&set[0][..2], &[this + 8, DISPLAY]);
        let record = set[0][2];
        // The first depth format: both bytes, quality 11 for one sample, samples 1 and 4.
        assert_eq!(e.mem.u8(record), 1);
        assert_eq!(e.mem.u8(record + 1), 11);
        assert_eq!(e.mem.u16(record + 2), 0b1001);
        assert_eq!(e.mem.u8(record + 0x24), 1);
        assert_eq!(e.mem.u8(record + 0x25), 11);
        assert_eq!(e.mem.u16(record + 0x26), 0b1001);
        // The other depth formats are not usable.
        assert_eq!(e.mem.u32(record + 4), 0);
        assert!(calls(&e, SIZED_DELETE).is_empty());
    }

    #[test]
    fn fn_00e7b260_deletes_a_target_without_depth() {
        let (mut e, direct3d) = rig(Setup {
            depth_usable: false,
            ..FIXED
        });
        let this = e.mem.alloc(DISPLAY_FORMAT_INFO_SIZE);
        // Neither byte: no multisample checks on the display format.
        let args = args![this, direct3d, 0u32, 1u32, DISPLAY, 0u32, 0u32];
        e.call(0x00e7_b260, &args);
        let record = calls(&e, SIZED_DELETE);
        assert_eq!(record.len(), 1);
        assert_eq!(record[0][1], RENDER_TARGET_INFO_SIZE);
        assert!(calls(&e, MAP_SET_AT).is_empty());
        assert!(calls(&e, slot(CHECK_DEVICE_MULTI_SAMPLE_TYPE)).is_empty());
    }

    #[test]
    fn fn_00e7b590_lists_one_record_per_mode_format() {
        let (mut e, direct3d) = rig(FIXED);
        let list = format_list(&mut e, &[DISPLAY, 0]);
        let this = e.mem.alloc(DEVICE_DESC_SIZE);
        let args = args![this, direct3d, 0u32, 1u32, DISPLAY, list];
        assert_eq!(e.call(0x00e7_b590, &args).u32(), this);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(
            calls(&e, slot(GET_DEVICE_CAPS)),
            vec![vec![direct3d, 0, 1, this + DEVICE_CAPS]]
        );
        let records = elements(&e, this + DEVICE_FORMATS);
        assert_eq!(records.len(), 1);
        assert_eq!(e.mem.u32(this + DEVICE_FORMATS + 8), 1);
        // The record: the mode's format, windowed and full screen.
        assert_eq!(e.mem.u32(records[0]), DISPLAY);
        assert_eq!((e.mem.u8(records[0] + 4), e.mem.u8(records[0] + 5)), (1, 1));
        assert_eq!(e.mem.u8(this + DEVICE_WINDOWED), 1);
        // Three modes, two device-type checks each.
        assert_eq!(calls(&e, slot(CHECK_DEVICE_TYPE)).len(), 6);
        assert_eq!(
            calls(&e, slot(CHECK_DEVICE_TYPE))[1],
            vec![direct3d, 0, 1, DISPLAY, DISPLAY, 1]
        );
    }

    #[test]
    fn fn_00e7b590_without_caps_lists_nothing() {
        let (mut e, direct3d) = rig(FIXED);
        let list = format_list(&mut e, &[DISPLAY]);
        let this = e.mem.alloc(DEVICE_DESC_SIZE);
        e.mem.set_u32(this + DEVICE_CAPS, 0x55);
        e.mem.set_u8(this + DEVICE_WINDOWED, 1);
        e.call(
            0x00e7_b590,
            &args![this, direct3d, 0u32, 2u32, DISPLAY, list],
        );
        assert_eq!(e.mem.u32(this + DEVICE_CAPS), 0);
        assert!(elements(&e, this + DEVICE_FORMATS).is_empty());
        assert!(calls(&e, slot(GET_ADAPTER_MODE_COUNT)).is_empty());
        assert_eq!(e.mem.u8(this + DEVICE_WINDOWED), 0);
    }

    #[test]
    fn fn_00e7b7d0_collects_modes_and_device_descriptions() {
        let (mut e, direct3d) = rig(FIXED);
        let list = format_list(&mut e, &[DISPLAY, 0]);
        let this = e.mem.alloc(0x468);
        assert_eq!(
            e.call(0x00e7_b7d0, &args![this, direct3d, 0u32, list])
                .u32(),
            this
        );
        assert_eq!(
            calls(&e, slot(GET_ADAPTER_IDENTIFIER)),
            vec![vec![direct3d, 0, 0, this + ADAPTER_IDENTIFIER]]
        );
        let modes = this + ADAPTER_MODES;
        assert_eq!(e.mem.u32(modes), ARRAY_VTABLE);
        assert_eq!(e.mem.u16(modes + 8), 0x20);
        assert_eq!(e.mem.u16(modes + 0xa), 2);
        assert_eq!(e.mem.u16(modes + 0xc), 2);
        assert_eq!(e.mem.u16(modes + 0xe), 0x10);
        let base = e.mem.u32(modes + 4);
        let first = e.mem.u32(base);
        let second = e.mem.u32(base + 4);
        assert_eq!(
            [e.mem.u32(first), e.mem.u32(first + 4), e.mem.u32(first + 8)],
            [800, 600, 32]
        );
        assert_eq!(e.mem.u32(first + 0xc), DISPLAY);
        assert_eq!(elements(&e, first + 0x10), vec![60, 75]);
        assert_eq!(e.mem.u32(first + 0x18), 2);
        assert_eq!(elements(&e, second + 0x10), vec![60]);
        assert_eq!(e.mem.u32(second), 1024);
        // The two device descriptions, device types 1 and 2.
        let hal = e.mem.u32(this + ADAPTER_DEVICE_1);
        let reference = e.mem.u32(this + ADAPTER_DEVICE_2);
        assert_eq!((e.mem.u32(hal), e.mem.u32(reference)), (1, 2));
        assert_eq!(e.mem.u8(hal + DEVICE_WINDOWED), 1);
        assert_eq!(e.mem.u8(reference + DEVICE_WINDOWED), 0);
        assert!(calls(&e, ARRAY_SET_SIZE).is_empty());
    }

    #[test]
    fn fn_00e7b7d0_grows_the_mode_array_when_full() {
        let (mut e, direct3d) = rig(Setup {
            distinct_modes: 0x21,
            ..FIXED
        });
        let list = format_list(&mut e, &[DISPLAY]);
        let this = e.mem.alloc(0x468);
        e.call(0x00e7_b7d0, &args![this, direct3d, 0u32, list]);
        let modes = this + ADAPTER_MODES;
        assert_eq!(calls(&e, ARRAY_SET_SIZE), vec![vec![modes, 0x30]]);
        assert_eq!(e.mem.u16(modes + 0xa), 0x21);
        assert_eq!(e.mem.u16(modes + 0xc), 0x21);
        let last = e.mem.u32(e.mem.u32(modes + 4) + 4 * 0x20);
        assert_eq!(e.mem.u32(last), 640 + 0x20);
    }
}

#[cfg(test)]
mod b048_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn device_and_adapter_destructors_free_their_records() {
        let mut e = Engine::new();
        for a in [
            DISPLAY_FORMAT_INFO_DESTRUCT,
            FORMAT_LIST_REMOVE_ALL,
            REFRESH_LIST_REMOVE_ALL,
            SIZED_DELETE,
            FREE_00AA10F0,
        ] {
            e.register(a, nop);
        }
        let device = e.mem.alloc(DEVICE_DESC_SIZE);
        let n2 = e.mem.alloc(12);
        let n1 = e.mem.alloc(12);
        e.mem.set_u32(n1, n2);
        e.mem.set_u32(n1 + 8, 0xf1);
        e.mem.set_u32(n2 + 8, 0);
        e.mem.set_u32(device + DEVICE_FORMATS, n1);
        let adapter = e.mem.alloc(ADAPTER_DESC_SIZE);
        let elements = e.mem.alloc(12);
        let mode = e.mem.alloc(MODE_DESC_SIZE);
        e.mem.set_u32(elements, mode);
        e.mem.set_u32(elements + 8, 0);
        e.mem.set_u32(adapter + ADAPTER_MODES + 4, elements);
        e.mem.set_u16(adapter + ADAPTER_MODES + 0xa, 2);
        e.mem.set_u32(adapter + ADAPTER_DEVICE_2, device);
        e.call_log = Some(vec![]);
        e.call(0x00e7_bb50, &args![adapter]);
        assert_eq!(calls(&e, REFRESH_LIST_REMOVE_ALL), vec![vec![mode + 0x10]]);
        assert_eq!(calls(&e, DISPLAY_FORMAT_INFO_DESTRUCT), vec![vec![0xf1]]);
        assert_eq!(
            calls(&e, FORMAT_LIST_REMOVE_ALL),
            vec![vec![device + DEVICE_FORMATS]]
        );
        assert_eq!(
            calls(&e, SIZED_DELETE),
            vec![
                vec![mode, MODE_DESC_SIZE],
                vec![0xf1, DISPLAY_FORMAT_INFO_SIZE],
                vec![device, DEVICE_DESC_SIZE]
            ]
        );
        assert_eq!(e.mem.u32(adapter + ADAPTER_MODES), ARRAY_BASE_VTABLE);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![elements]]);
    }

    #[test]
    fn adapter_list_holds_one_description_per_adapter() {
        let mut e = Engine::new();
        let table: Vec<u32> = (0..0x10).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &table);
        let direct3d = e.mem.alloc(8);
        e.mem.set_u32(direct3d, 0x6000);
        e.register(0x6100_0000 + GET_ADAPTER_COUNT, |_, _| 2u32.into_ret());
        e.register(ARRAY_SET_SIZE, |e, a| {
            let elements = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, elements);
            Ret::default()
        });
        // The first description's allocation fails, the second's works; the device
        // descriptions inside it are not allocated.
        let mut first = true;
        e.register_double(NEW_OBJECT, move |e, a| {
            let p = if a[0] != ADAPTER_DESC_SIZE || first {
                0
            } else {
                e.mem.alloc(a[0])
            };
            if a[0] == ADAPTER_DESC_SIZE {
                first = false;
            }
            p.into_ret()
        });
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x6100_0000 + GET_ADAPTER_IDENTIFIER, |_, _| 0u32.into_ret());
        e.register(0x6100_0000 + GET_ADAPTER_DISPLAY_MODE, |_, _| {
            0x8000_0000u32.into_ret()
        });
        e.register(0x6100_0000 + GET_ADAPTER_MODE_COUNT, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x14);
        let formats = e.mem.alloc(0x0c);
        e.call(0x00e7_bbf0, &args![this, direct3d, formats]);
        assert_eq!(e.mem.u32(this), 2);
        let elements = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(elements), 0);
        let desc = e.mem.u32(elements + 4);
        assert_ne!(desc, 0);
        assert_eq!(e.mem.u32(desc), 1);
        assert_eq!(e.mem.u16(this + 0xe), 2);
        assert_eq!(e.mem.u16(this + 0x10), 1);
    }
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    fn check(address: u32, vtable: u32) {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(FREE_00AA10F0, |_, _| Ret::default());
            e.register(MEMORY_MANAGER_FREE_00AA4060, |_, _| Ret::default());
            let array = e.new_object::<NiTArray>();
            e.set(array, NiTArray::m_pBase, 0x0e40_0000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![array, flags]).u32(), array.addr());
            assert_eq!(e.mem.u32(array.addr()), vtable);
            let log = e.call_log.take().unwrap();
            assert!(log.contains(&(FREE_00AA10F0, vec![0x0e40_0000])));
            let deletes: Vec<_> = log
                .iter()
                .filter(|c| c.0 == MEMORY_MANAGER_FREE_00AA4060)
                .collect();
            if flags == 1 {
                assert_eq!(
                    deletes,
                    vec![&(
                        MEMORY_MANAGER_FREE_00AA4060,
                        vec![MEMORY_MANAGER_011F6238, array.addr()]
                    )]
                );
            } else {
                assert!(deletes.is_empty());
            }
        }
    }

    #[test]
    fn fn_00e7b200_destroys_the_mode_array() {
        check(0x00e7_b200, ARRAY_BASE_VTABLE);
    }

    #[test]
    fn fn_00e7b230_destroys_the_adapter_array() {
        check(0x00e7_b230, ADAPTER_ARRAY_VTABLE);
    }
}
