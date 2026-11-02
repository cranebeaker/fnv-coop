//! `NiXenonRenderer/run_00e7cd50` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00e7ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; no arguments): allocates 0x18 bytes from the memory
/// manager, zeroes the first four words, sets the words at `+0x10` and `+0x14` to 8 and
/// keeps the block in `0126f6cc` (null when the allocation fails).
pub fn fn_00e7ec90(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(0x0126_f6cc, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(0x0126_f6cc, block);
}

use crate::types::{NiTListItem, NiTPointerList};

/// The list's remove-head and remove-tail helpers `00b602d0` and `00e7d5b0` (unnamed,
/// `thiscall` on the list, no stack words; they answer the removed element) and the item
/// release `00e6e8a0` (unnamed, `thiscall` on the list, the item).
const REMOVE_HEAD_00B602D0: u32 = 0x00b6_02d0;
const REMOVE_TAIL_00E7D5B0: u32 = 0x00e7_d5b0;
const FREE_ITEM_00E6E8A0: u32 = 0x00e6_e8a0;

// Translated from 00e7d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): an `NiTPointerList` remove at a position,
/// `thiscall` with one stack word (the address of the position, an item pointer). At the head
/// the position moves to the next item and `00b602d0` removes the head; at the tail the
/// position becomes null and `00e7d5b0` removes the tail; otherwise the position moves to the
/// next item, the neighbours are linked to each other, the item is released (`00e6e8a0`) and
/// the count at +8 goes down by one. Answers the removed element (what the helpers answer, or
/// the item's element at +8).
pub fn fn_00e7d7f0(e: &mut Engine, this: Ptr<NiTPointerList>, position: Ptr) -> u32 {
    let item: Ptr<NiTListItem> = Ptr::new(e.mem.u32(position.addr()));
    if item.addr() == e.get(this, NiTPointerList::m_pkHead) {
        let next = e.get(item, NiTListItem::m_pkNext);
        e.mem.set_u32(position.addr(), next);
        return e.call(REMOVE_HEAD_00B602D0, &args![this]).u32();
    }
    if item.addr() == e.get(this, NiTPointerList::m_pkTail) {
        e.mem.set_u32(position.addr(), 0);
        return e.call(REMOVE_TAIL_00E7D5B0, &args![this]).u32();
    }
    let previous = e.get(item, NiTListItem::m_pkPrev);
    let next = e.get(item, NiTListItem::m_pkNext);
    e.mem.set_u32(position.addr(), next);
    if previous != 0 {
        e.mem.set_u32(previous, next);
    }
    if next != 0 {
        e.mem.set_u32(next + 4, previous);
    }
    let element = e.get(item, NiTListItem::m_element);
    e.call(FREE_ITEM_00E6E8A0, &args![this, item]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    element
}

/// The list-item pool's lock (`011c5f80`: the `this` of the lock enter `0040fbf0`, `thiscall`
/// with one word, and the lock's owner word), the lock's entry count (`011c5f84`) that the
/// inlined leave decrements, and the head of the pool's free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;

// Translated from 00e7d5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): an `NiTPointerList` remove-tail, `thiscall`, no
/// stack words. The tail item's previous item becomes the tail (its next cleared; the head
/// cleared instead when there is none); the item's element (+8) is taken and cleared, the item
/// goes back to the item pool inline (the pool's lock entered through `0040fbf0` on `011c5f80`
/// with 0, the item's +4 cleared, the item pushed on the free list at `011c5f58`, the lock
/// left: its count at `011c5f84` down by one and the owner word cleared at zero) and the
/// list's count goes down by one. Answers the element.
pub fn fn_00e7d5b0(e: &mut Engine, this: Ptr<NiTPointerList>) -> u32 {
    let item: Ptr<NiTListItem> = Ptr::new(e.get(this, NiTPointerList::m_pkTail));
    let previous = e.get(item, NiTListItem::m_pkPrev);
    e.set(this, NiTPointerList::m_pkTail, previous);
    if previous != 0 {
        e.mem.set_u32(previous, 0);
    } else {
        e.set(this, NiTPointerList::m_pkHead, 0);
    }
    let element = e.get(item, NiTListItem::m_element);
    e.set(item, NiTListItem::m_element, 0);
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.set(item, NiTListItem::m_pkPrev, 0);
    let head = e.mem.u32(NODE_POOL_FREE_LIST);
    e.set(item, NiTListItem::m_pkNext, head);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, item.addr());
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
    let items = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, items);
    element
}

/// The empty fixed string (`0109b220`), the string release `00aa04e0` (engine map:
/// `NiShaderFactory::ReleaseGlobalShaderConstant`; `cdecl`, the address of a string word) and
/// `00e89670` (unnamed; `cdecl`, one word).
const FIXED_STRING_EMPTY_0109B220: u32 = 0x0109_b220;
const RELEASE_STRING_00AA04E0: u32 = 0x00aa_04e0;
const RELEASE_00E89670: u32 = 0x00e8_9670;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00e7de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): an element's clean-up, `thiscall`, no stack
/// words. When the entry at +0x18 is set, its string at +8 is held (count at string - 8 up
/// through `InterlockedIncrement` unless it is the empty fixed string `0109b220`), copied into
/// a stack word (the one the function saved `ECX` in) handed to `00aa04e0`, and the word's
/// string afterwards let go the same way (`InterlockedDecrement`). Then `00e89670` gets the
/// word at +0x10.
pub fn fn_00e7de30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let entry = e.mem.u32(base + 0x18);
    if entry != 0 {
        let text = e.mem.u32(entry + 8);
        if text != e.global::<u32>(FIXED_STRING_EMPTY_0109B220) {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![text.wrapping_sub(8)]);
        }
        e.with_stack(4, |e, slot| {
            let slot = slot.addr();
            let text = e.mem.u32(entry + 8);
            e.mem.set_u32(slot, text);
            e.call(RELEASE_STRING_00AA04E0, &args![slot]);
            let left = e.mem.u32(slot);
            if left != e.global::<u32>(FIXED_STRING_EMPTY_0109B220) {
                e.call(IMPORT_INTERLOCKED_DECREMENT, &args![left.wrapping_sub(8)]);
            }
        });
    }
    let word = e.mem.u32(base + 0x10);
    e.call(RELEASE_00E89670, &args![word]);
}

/// What `00e7d6b0` uses: the allocation `00aa13e0`, the `NiRefObject` table and global object
/// count (`011f4400`), the class's table (RTTI `NiDX9ImplicitBufferData`), `memcpy`
/// (`00ec44d0`), the format conversion `00e7c500` and name `00e7a9a0` (unnamed; `cdecl`, a
/// format), the report `00a29680` and its message (`"NiDX9Renderer::Create> Using %s
/// backbuffer format\n"`), the object at `011f4748` and the two globals filled from its slots
/// 0xd8 and 0xdc (`0126f594`, the byte `0126f598`), and the render target group's maker
/// `00a8f2c0` (unnamed; `cdecl`: width, height, buffer) and attacher `00a8f140` (unnamed;
/// `thiscall`, the buffer).
const NEW_BUFFER_OBJECT: u32 = 0x00aa_13e0;
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const NI_DX9_IMPLICIT_BUFFER_DATA_VTABLE: u32 = 0x010e_f06c;
const MEMCPY: u32 = 0x00ec_44d0;
const CONVERT_FORMAT_00E7C500: u32 = 0x00e7_c500;
const FORMAT_NAME_00E7A9A0: u32 = 0x00e7_a9a0;
const REPORT_00A29680: u32 = 0x00a2_9680;
const USING_FORMAT_010EF0D8: u32 = 0x010e_f0d8;
const SOURCE_011F4748: u32 = 0x011f_4748;
const SLOT_D8_0126F594: u32 = 0x0126_f594;
const SLOT_DC_0126F598: u32 = 0x0126_f598;
const MAKE_GROUP_00A8F2C0: u32 = 0x00a8_f2c0;
const ATTACH_00A8F140: u32 = 0x00a8_f140;
/// `IDirect3DDevice9::GetRenderTarget` (vtable slot 0x98) and `IDirect3DSurface9::GetDesc`
/// (0x30), named by the interfaces' slot order; the arguments match.
const GET_RENDER_TARGET: u32 = 0x98;
const GET_DESC: u32 = 0x30;

// Translated from 00e7d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): makes the renderer's implicit back buffer data
/// (an `NiDX9ImplicitBufferData`, 0x5c bytes; the RTTI of the table it stores), `cdecl`: the
/// device, the present parameters and the word for the render target group. A made object
/// gets the `NiRefObject` start, +8, +0xc, +0x14 and +0x58 cleared and the class's table; null
/// when the allocation fails (the rest still uses it). The 0x38 bytes of the present
/// parameters are copied to +0x20, the device kept at +0x58 (counted: its slot 4). The device's
/// render target 0 goes to +0x14; when that or the surface's description fails the object is
/// deleted (its slot 0 with 1) and null answered. Otherwise +0xc gets the converted format
/// (`00e7c500`), the format's name is reported, the globals at `0126f594`/`0126f598` are
/// filled once from the object at `011f4748`, and the group word gets a new group
/// (`00a8f2c0` with the buffer, the description's width and height) or, when it holds one,
/// the buffer attached to it (`00a8f140`); the group goes to +8. Answers the object.
pub fn fn_00e7d6b0(e: &mut Engine, device: u32, present: Ptr, group_out: Ptr) -> u32 {
    let block = e.call(NEW_BUFFER_OBJECT, &args![0x5cu32]).u32();
    let data = if block != 0 {
        e.mem.set_u32(block, NI_REF_OBJECT_VTABLE);
        e.mem.set_u32(block + 4, 0);
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
        e.mem.set_u32(block + 8, 0);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x14, 0);
        e.mem.set_u32(block, NI_DX9_IMPLICIT_BUFFER_DATA_VTABLE);
        e.mem.set_u32(block + 0x58, 0);
        block
    } else {
        0
    };
    e.call(MEMCPY, &args![data.wrapping_add(0x20), present, 0x38u32]);
    e.mem.set_u32(data.wrapping_add(0x58), device);
    e.vcall(device, 4, &args![]);
    // The surface description (0x20 bytes: format first, width and height last).
    e.with_stack(0x20, |e, desc| {
        let fail = |e: &mut Engine| {
            e.vcall(data, 0, &args![1u32]);
            0
        };
        let target = data.wrapping_add(0x14);
        if e.vcall(device, GET_RENDER_TARGET, &args![0u32, target])
            .i32()
            < 0
        {
            return fail(e);
        }
        let surface = e.mem.u32(target);
        if e.vcall(surface, GET_DESC, &args![desc]).i32() < 0 {
            return fail(e);
        }
        let format = e.mem.u32(desc.addr());
        let converted = e.call(CONVERT_FORMAT_00E7C500, &args![format]).u32();
        e.mem.set_u32(data.wrapping_add(0xc), converted);
        let format = e.mem.u32(desc.addr());
        let name = e.call(FORMAT_NAME_00E7A9A0, &args![format]).u32();
        e.call(REPORT_00A29680, &args![USING_FORMAT_010EF0D8, name]);
        if e.mem.u32(SLOT_D8_0126F594) == 0 {
            let source = e.mem.u32(SOURCE_011F4748);
            let value = e.vcall(source, 0xd8, &args![]).u32();
            e.mem.set_u32(SLOT_D8_0126F594, value);
            let flag = e.vcall(source, 0xdc, &args![]).u8();
            e.mem.set_u8(SLOT_DC_0126F598, flag);
        }
        let held = e.mem.u32(group_out.addr());
        if held == 0 {
            let width = e.mem.u32(desc.addr() + 0x18);
            let height = e.mem.u32(desc.addr() + 0x1c);
            let group = e
                .call(MAKE_GROUP_00A8F2C0, &args![width, height, data])
                .u32();
            e.mem.set_u32(group_out.addr(), group);
            e.mem.set_u32(data.wrapping_add(8), group);
        } else {
            e.call(ATTACH_00A8F140, &args![held, data]);
            let group = e.mem.u32(group_out.addr());
            e.mem.set_u32(data.wrapping_add(8), group);
        }
        data
    })
}

/// The surfaces last set per render target index (`0126f580`, one word each) and the depth
/// surface last set (`0126f590`); the number of render targets is the word at `0126f594`.
const RENDER_TARGETS_0126F580: u32 = 0x0126_f580;
const DEPTH_SURFACE_0126F590: u32 = 0x0126_f590;
/// `IDirect3DDevice9::SetRenderTarget` (slot 0x94) and `SetDepthStencilSurface` (0x9c), named
/// by the interface's slot order; the arguments match.
const SET_RENDER_TARGET: u32 = 0x94;
const SET_DEPTH_STENCIL_SURFACE: u32 = 0x9c;

// Translated from 00e7cd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`; a virtual slot of the buffer data classes):
/// sets the buffer as render target `index`, `thiscall` with the device and the index. False
/// without a surface (+0x14) or when `index` is above the count at `0126f594`. The surface is
/// the one at +0x18 when set, else +0x14; unless it is already the one recorded for the index
/// (`0126f580`), the device's `SetRenderTarget` gets it: a failure answers false, success
/// records it. Answers true otherwise.
pub fn fn_00e7cd50(e: &mut Engine, this: Ptr, device: u32, index: u32) -> bool {
    let base = this.addr();
    let mut surface = e.mem.u32(base.wrapping_add(0x14));
    if surface == 0 {
        return false;
    }
    if index > e.mem.u32(SLOT_D8_0126F594) {
        return false;
    }
    let other = e.mem.u32(base.wrapping_add(0x18));
    if other != 0 {
        surface = other;
    }
    let slot = RENDER_TARGETS_0126F580.wrapping_add(index.wrapping_mul(4));
    if surface != e.mem.u32(slot) {
        if e.vcall(device, SET_RENDER_TARGET, &args![index, surface])
            .i32()
            < 0
        {
            return false;
        }
        e.mem.set_u32(slot, surface);
    }
    true
}

// Translated from 00e7cdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`; a virtual slot of the depth buffer data
/// classes): sets the buffer's surface (+0x14) as the depth stencil surface, `thiscall` with
/// the device. False without a surface; unless it is already the one recorded at `0126f590`,
/// the device's `SetDepthStencilSurface` gets it: a failure answers false, success records it
/// (read again from +0x14). Answers true otherwise.
pub fn fn_00e7cdb0(e: &mut Engine, this: Ptr, device: u32) -> bool {
    let base = this.addr();
    let surface = e.mem.u32(base.wrapping_add(0x14));
    if surface == 0 {
        return false;
    }
    if surface != e.mem.u32(DEPTH_SURFACE_0126F590) {
        if e.vcall(device, SET_DEPTH_STENCIL_SURFACE, &args![surface])
            .i32()
            < 0
        {
            return false;
        }
        let surface = e.mem.u32(base.wrapping_add(0x14));
        e.mem.set_u32(DEPTH_SURFACE_0126F590, surface);
    }
    true
}

/// `00a77980` (unnamed; `thiscall` on a pixel format, one word: a channel kind): the bits of
/// that channel (a byte).
const CHANNEL_BITS_00A77980: u32 = 0x00a7_7980;

/// The shared shape of `00e7cdf0` and `00e7ce10`: with a pixel format (+0xc) and a surface
/// (+0x14), whether the format's channel `kind` has bits (`TEST AL,AL; SETA AL`).
fn has_channel_bits(e: &mut Engine, this: Ptr, kind: u32) -> bool {
    let base = this.addr();
    let format = e.mem.u32(base.wrapping_add(0xc));
    if format != 0 && e.mem.u32(base.wrapping_add(0x14)) != 0 {
        return e.call(CHANNEL_BITS_00A77980, &args![format, kind]).u8() != 0;
    }
    false
}

// Translated from 00e7cdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`), `thiscall`: with a pixel format (+0xc) and a
/// surface (+0x14), whether the format's channel 0x11 has bits (`00a77980`); else false.
pub fn fn_00e7cdf0(e: &mut Engine, this: Ptr) -> bool {
    has_channel_bits(e, this, 0x11)
}

// Translated from 00e7ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`), `thiscall`: as `00e7cdf0` for channel 0x12.
pub fn fn_00e7ce10(e: &mut Engine, this: Ptr) -> bool {
    has_channel_bits(e, this, 0x12)
}

/// The lock of the additional depth buffer list (`0126f600`, a `CRITICAL_SECTION`) with the
/// imported `EnterCriticalSection` / `LeaveCriticalSection` (import slots), and the list
/// itself (`0126f5cc` head, `0126f5d0` tail, `0126f5d4` count; items: next, previous,
/// element).
const BUFFER_LIST_LOCK_0126F600: u32 = 0x0126_f600;
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const BUFFER_LIST_0126F5CC: u32 = 0x0126_f5cc;
const BUFFER_LIST_TAIL_0126F5D0: u32 = 0x0126_f5d0;
const BUFFER_LIST_COUNT_0126F5D4: u32 = 0x0126_f5d4;

// Translated from 00e7cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`; no arguments): under the list's lock
/// (`0126f600`), calls virtual slot 0x54 of every element of the additional depth buffer
/// list (`0126f5cc`), the next item read before each call.
pub fn fn_00e7cec0(e: &mut Engine) {
    e.call(
        IMPORT_ENTER_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
    let mut item = e.mem.u32(BUFFER_LIST_0126F5CC);
    while item != 0 {
        let element = e.mem.u32(item.wrapping_add(8));
        item = e.mem.u32(item);
        e.vcall(element, 0x54, &args![]);
    }
    e.call(
        IMPORT_LEAVE_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
}

/// The memory manager's deallocate (`00aa4060`, `thiscall` on `011f6238`, the block) and the
/// sized free `00aa1460` (`cdecl`: block, size).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const FREE_00AA1460: u32 = 0x00aa_1460;
/// The tables the buffer data destructors store on the way down (RTTI
/// `NiDX9TextureBufferData`, `NiDX92DBufferData`, `NiDX9AdditionalDepthStencilBufferData`).
const NI_DX9_TEXTURE_BUFFER_DATA_VTABLE: u32 = 0x010e_efbc;
const NI_DX9_2D_BUFFER_DATA_VTABLE: u32 = 0x010e_eed4;
const NI_DX9_ADDITIONAL_DEPTH_STENCIL_BUFFER_DATA_VTABLE: u32 = 0x010e_f184;

/// A COM pointer at `at` released (its slot 8) and cleared when set.
fn release_and_clear(e: &mut Engine, at: u32) {
    let held = e.mem.u32(at);
    if held != 0 {
        e.vcall(held, 8, &args![]);
        e.mem.set_u32(at, 0);
    }
}

/// The `NiDX92DBufferData` part of the destructors: its table, the surface (+0x14) released
/// and cleared, the block at +0xc given back to the memory manager and cleared, then the
/// `NiRefObject` table and the global object count down.
fn buffer_data_down(e: &mut Engine, base: u32) {
    e.mem.set_u32(base, NI_DX9_2D_BUFFER_DATA_VTABLE);
    release_and_clear(e, base.wrapping_add(0x14));
    let block = e.mem.u32(base.wrapping_add(0xc));
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, block]);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e7d180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`; a virtual slot of the buffer data classes),
/// `thiscall`: releases the surface (+0x14, its slot 8) and clears it, gives the block at +0xc
/// back to the memory manager (`00aa4060`) and clears it.
pub fn fn_00e7d180(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_and_clear(e, base.wrapping_add(0x14));
    let block = e.mem.u32(base.wrapping_add(0xc));
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, block]);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
}

// Translated from 00e7d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): the `NiDX9TextureBufferData` destructor (the
/// RTTI of the table it stores), `thiscall`. The texture (+0x20) released and cleared, then
/// the `NiDX92DBufferData` part: its surface (+0x14) released and cleared, the block at +0xc
/// freed (`00aa4060`) and cleared, the `NiRefObject` table and the global object count
/// (`011f4400`) down.
pub fn fn_00e7d1b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_DX9_TEXTURE_BUFFER_DATA_VTABLE);
    release_and_clear(e, base.wrapping_add(0x20));
    buffer_data_down(e, base);
}

// Translated from 00e7d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`; virtual slot 2 of `NiDX9TextureBufferData`):
/// answers the address `0126f5a4` (the class's RTTI record).
pub fn fn_00e7d220(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_f5a4
}

// Translated from 00e7d230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): the `NiDX9TextureBufferData` deleting
/// destructor, `thiscall` with the flags: the destructor (`00e7d1b0`), then with bit 0 the
/// 0x24 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e7d230(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e7d1b0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x24u32]);
    }
    this
}

/// `NiXenonErrorString` (`00e89d50`, `cdecl`, a result code) and the messages
/// `"NiDX9TextureBufferData::Create> FAILED- GetSurfaceLevel - %s"` (`010ef028`) and
/// `"NiDX9AdditionalDepthStencilBufferData::Create> FAILED %s"` (`010ef1f0`).
const ERROR_STRING_00E89D50: u32 = 0x00e8_9d50;
const TEXTURE_BUFFER_FAILED_010EF028: u32 = 0x010e_f028;
const DEPTH_BUFFER_FAILED_010EF1F0: u32 = 0x010e_f1f0;
/// `IDirect3DTexture9::GetSurfaceLevel` (slot 0x48), by the interface's slot order.
const GET_SURFACE_LEVEL: u32 = 0x48;

/// The multisample type stored for a buffer's sample kind (the `switch` on 1, 2, 3).
fn multisample_type(kind: u32) -> u32 {
    match kind {
        1 => 2,
        2 => 4,
        3 => 8,
        _ => 0,
    }
}

// Translated from 00e7d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): makes an `NiDX9TextureBufferData` (0x24 bytes;
/// the RTTI of the table it stores), `cdecl`: the Direct3D texture, the render target group
/// word, the sample kind and a word kept at +0x18. A made object gets the `NiRefObject` start,
/// +8, +0xc, +0x14 and +0x20 cleared and the class's table (null when the allocation fails; the
/// stores still go to it). The texture is kept at +0x20 (counted: its slot 4 when not null),
/// the kind at +0x10 and its multisample type (1, 2, 3 to 2, 4, 8, else 0) at +0x1c. The
/// texture's level 0 surface goes to +0x14 (`GetSurfaceLevel`); a failure is reported
/// (`00a29680` with `NiXenonErrorString`), the object deleted (its slot 0 with 1) and null
/// answered; a failed surface description deletes it too, without a report. Otherwise the
/// converted format (`00e7c500`) goes to +0xc and the group word gets a new group (`00a8f2c0`
/// with the buffer and the description's width and height) or, when it holds one, the buffer
/// attached to it (`00a8f140`); the group goes to +8. Answers the object.
pub fn fn_00e7d250(e: &mut Engine, texture: u32, group_out: Ptr, kind: u32, word: u32) -> u32 {
    let block = e.call(NEW_BUFFER_OBJECT, &args![0x24u32]).u32();
    let data = if block != 0 {
        e.mem.set_u32(block, NI_REF_OBJECT_VTABLE);
        e.mem.set_u32(block + 4, 0);
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
        e.mem.set_u32(block + 8, 0);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x14, 0);
        e.mem.set_u32(block, NI_DX9_TEXTURE_BUFFER_DATA_VTABLE);
        e.mem.set_u32(block + 0x20, 0);
        block
    } else {
        0
    };
    e.mem.set_u32(data.wrapping_add(0x20), texture);
    if texture != 0 {
        e.vcall(texture, 4, &args![]);
    }
    e.mem.set_u32(data.wrapping_add(0x10), kind);
    e.mem
        .set_u32(data.wrapping_add(0x1c), multisample_type(kind));
    let result = e
        .vcall(
            texture,
            GET_SURFACE_LEVEL,
            &args![0u32, data.wrapping_add(0x14)],
        )
        .i32();
    e.mem.set_u32(data.wrapping_add(0x18), word);
    if result < 0 {
        let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
        e.call(
            REPORT_00A29680,
            &args![TEXTURE_BUFFER_FAILED_010EF028, name],
        );
        e.vcall(data, 0, &args![1u32]);
        return 0;
    }
    e.with_stack(0x20, |e, desc| {
        let surface = e.mem.u32(data.wrapping_add(0x14));
        if e.vcall(surface, GET_DESC, &args![desc]).i32() < 0 {
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        let format = e.mem.u32(desc.addr());
        let converted = e.call(CONVERT_FORMAT_00E7C500, &args![format]).u32();
        e.mem.set_u32(data.wrapping_add(0xc), converted);
        let held = e.mem.u32(group_out.addr());
        if held == 0 {
            let width = e.mem.u32(desc.addr() + 0x18);
            let height = e.mem.u32(desc.addr() + 0x1c);
            let group = e
                .call(MAKE_GROUP_00A8F2C0, &args![width, height, data])
                .u32();
            e.mem.set_u32(group_out.addr(), group);
            e.mem.set_u32(data.wrapping_add(8), group);
        } else {
            e.call(ATTACH_00A8F140, &args![held, data]);
            let group = e.mem.u32(group_out.addr());
            e.mem.set_u32(data.wrapping_add(8), group);
        }
        data
    })
}

// Translated from 00e7d4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): the `NiDX9ImplicitBufferData` destructor (the
/// RTTI of the table it stores), `thiscall`. The device (+0x58) released and cleared, then the
/// `NiDX92DBufferData` part (the surface at +0x14 released and cleared, the block at +0xc
/// freed and cleared, the `NiRefObject` table and the global object count down).
pub fn fn_00e7d4e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_DX9_IMPLICIT_BUFFER_DATA_VTABLE);
    release_and_clear(e, base.wrapping_add(0x58));
    buffer_data_down(e, base);
}

// Translated from 00e7d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`; virtual slot 2 of `NiDX9ImplicitBufferData`):
/// answers the address `0126f5e0` (the class's RTTI record).
pub fn fn_00e7d550(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_f5e0
}

// Translated from 00e7d690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): the `NiDX9ImplicitBufferData` deleting
/// destructor, `thiscall` with the flags: the destructor (`00e7d4e0`), then with bit 0 the
/// 0x5c bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e7d690(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e7d4e0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x5cu32]);
    }
    this
}

/// `0043a010`, named `NiTPointerListBase<...>::AddHead` in the engine map: called (`thiscall`,
/// no stack words) on the list's word at `0126f5d4` it answers a new item.
const NEW_ITEM_0043A010: u32 = 0x0043_a010;

// Translated from 00e7d940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): the `NiDX9AdditionalDepthStencilBufferData`
/// constructor (the RTTI of the table it stores), `thiscall`. The `NiRefObject` start, +8,
/// +0xc, +0x14 and +0x20 cleared and the class's table; then under the list's lock
/// (`0126f600`) a new item (`0043a010`) holding `this` (previous cleared) goes to the head of
/// the additional depth buffer list (`0126f5cc`; the tail at `0126f5d0` when the list was
/// empty) and the count at `0126f5d4` goes up. Answers `this`.
pub fn fn_00e7d940(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    e.mem
        .set_u32(base, NI_DX9_ADDITIONAL_DEPTH_STENCIL_BUFFER_DATA_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x20), 0);
    e.call(
        IMPORT_ENTER_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
    let item = e
        .call(NEW_ITEM_0043A010, &args![BUFFER_LIST_COUNT_0126F5D4])
        .u32();
    e.mem.set_u32(item.wrapping_add(8), base);
    e.mem.set_u32(item.wrapping_add(4), 0);
    let head = e.mem.u32(BUFFER_LIST_0126F5CC);
    e.mem.set_u32(item, head);
    let head = e.mem.u32(BUFFER_LIST_0126F5CC);
    if head != 0 {
        e.mem.set_u32(head.wrapping_add(4), item);
    } else {
        e.mem.set_u32(BUFFER_LIST_TAIL_0126F5D0, item);
    }
    let count = e.mem.u32(BUFFER_LIST_COUNT_0126F5D4).wrapping_add(1);
    e.mem.set_u32(BUFFER_LIST_COUNT_0126F5D4, count);
    e.mem.set_u32(BUFFER_LIST_0126F5CC, item);
    e.call(
        IMPORT_LEAVE_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
    this
}

// Translated from 00e7d9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`; virtual slot 2 of
/// `NiDX9AdditionalDepthStencilBufferData`): answers the address `0126f5c4` (the class's RTTI
/// record).
pub fn fn_00e7d9c0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_f5c4
}

/// What `00e7d9d0` uses besides the shared ones: the format of a pixel format (`00e7bd20`,
/// unnamed; `cdecl`), the renderer (`011f4748`) with its virtual slot 0xc8 and its adapter,
/// device type and adapter format (+0x5c4, +0x5c8, +0x5d0), the Direct3D object (`0126f0d8`)
/// with `IDirect3D9::CheckDeviceFormat` (slot 0x28), the word kept at +0x24 (`011bf400`) and
/// `IDirect3DDevice9::CreateDepthStencilSurface` (slot 0x74; slot names by the interfaces'
/// order, the arguments match).
const FORMAT_OF_00E7BD20: u32 = 0x00e7_bd20;
const DIRECT3D_0126F0D8: u32 = 0x0126_f0d8;
const CHECK_DEVICE_FORMAT: u32 = 0x28;
const DISCARD_011BF400: u32 = 0x011b_f400;
const CREATE_DEPTH_STENCIL_SURFACE: u32 = 0x74;

// Translated from 00e7d9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): makes an additional depth stencil buffer,
/// `cdecl`: the device, the render target group word (the group's +8 and +0xc are its width
/// and height), a pixel format and the sample kind. Null without a pixel format or when it has
/// no Direct3D format (`00e7bd20`). The renderer (`011f4748`) gets its slot 0xc8, then
/// Direct3D is asked whether the format works as a depth stencil surface (`CheckDeviceFormat`
/// with the renderer's adapter, device type and adapter format, usage 2, resource type 1); a
/// failure is reported (`NiXenonErrorString`, `00a29680`) and null answered. Otherwise a new
/// `NiDX9AdditionalDepthStencilBufferData` (0x28 bytes, [`fn_00e7d940`]; null when the
/// allocation fails, the stores still go to it) gets the word at `011bf400` at +0x24 and the
/// kind at +0x10; the device creates its surface (+0x14) with the group's width and height,
/// the format, the kind's multisample type (1, 2, 3 to 2, 4, 8, else 0), quality 0 and +0x24.
/// A failure is reported and the object deleted (its slot 0 with 1); a failed surface
/// description deletes it without a report: both answer null. Otherwise the converted format
/// (`00e7c500`) goes to +0xc, the format to +0x20, the buffer is attached to the group
/// (`00a8f140`) and the group kept at +8. Answers the object.
pub fn fn_00e7d9d0(
    e: &mut Engine,
    device: u32,
    group_out: Ptr,
    pixel_format: u32,
    kind: u32,
) -> u32 {
    if pixel_format == 0 {
        return 0;
    }
    let format = e.call(FORMAT_OF_00E7BD20, &args![pixel_format]).u32();
    if format == 0 {
        return 0;
    }
    let renderer = e.mem.u32(SOURCE_011F4748);
    e.vcall(renderer, 0xc8, &args![]);
    let adapter_format = e.mem.u32(renderer.wrapping_add(0x5d0));
    let device_type = e.mem.u32(renderer.wrapping_add(0x5c8));
    let direct3d = e.mem.u32(DIRECT3D_0126F0D8);
    let adapter = e.mem.u32(renderer.wrapping_add(0x5c4));
    let result = e
        .vcall(
            direct3d,
            CHECK_DEVICE_FORMAT,
            &args![adapter, device_type, adapter_format, 2u32, 1u32, format],
        )
        .i32();
    if result < 0 {
        let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
        e.call(REPORT_00A29680, &args![DEPTH_BUFFER_FAILED_010EF1F0, name]);
        return 0;
    }
    let block = e.call(NEW_BUFFER_OBJECT, &args![0x28u32]).u32();
    let data = if block != 0 {
        fn_00e7d940(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let discard = e.mem.u32(DISCARD_011BF400);
    e.mem.set_u32(data.wrapping_add(0x24), discard);
    e.mem.set_u32(data.wrapping_add(0x10), kind);
    let samples = multisample_type(kind);
    let group = e.mem.u32(group_out.addr());
    let height = e.mem.u32(group.wrapping_add(0xc));
    let width = e.mem.u32(group.wrapping_add(8));
    let discard = e.mem.u32(data.wrapping_add(0x24));
    let result = e
        .vcall(
            device,
            CREATE_DEPTH_STENCIL_SURFACE,
            &args![
                width,
                height,
                format,
                samples,
                0u32,
                discard,
                data.wrapping_add(0x14),
                0u32
            ],
        )
        .i32();
    if result < 0 {
        let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
        e.call(REPORT_00A29680, &args![DEPTH_BUFFER_FAILED_010EF1F0, name]);
        e.vcall(data, 0, &args![1u32]);
        return 0;
    }
    e.with_stack(0x20, |e, desc| {
        let surface = e.mem.u32(data.wrapping_add(0x14));
        if e.vcall(surface, GET_DESC, &args![desc]).i32() < 0 {
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        let surface_format = e.mem.u32(desc.addr());
        let converted = e
            .call(CONVERT_FORMAT_00E7C500, &args![surface_format])
            .u32();
        e.mem.set_u32(data.wrapping_add(0xc), converted);
        e.mem.set_u32(data.wrapping_add(0x20), format);
        let group = e.mem.u32(group_out.addr());
        e.call(ATTACH_00A8F140, &args![group, data]);
        let group = e.mem.u32(group_out.addr());
        e.mem.set_u32(data.wrapping_add(8), group);
        data
    })
}

// Translated from 00e7db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): an `NiTPointerList` remove of an element,
/// `thiscall` with the address of the element. Finds the first item holding it; when there is
/// one, removes it through [`fn_00e7d7f0`] (its position kept in the argument's stack slot)
/// and answers what that answers; else answers the element.
pub fn fn_00e7db70(e: &mut Engine, this: Ptr<NiTPointerList>, element: Ptr) -> u32 {
    let wanted = e.mem.u32(element.addr());
    let mut item = e.get(this, NiTPointerList::m_pkHead);
    let found = loop {
        if item == 0 {
            break 0;
        }
        let next = e.mem.u32(item);
        if wanted == e.mem.u32(item.wrapping_add(8)) {
            break item;
        }
        item = next;
    };
    if found != 0 {
        return e.with_stack(4, |e, position| {
            e.mem.set_u32(position.addr(), found);
            fn_00e7d7f0(e, this, position)
        });
    }
    e.mem.u32(element.addr())
}

// Translated from 00e7dbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): the `NiDX9AdditionalDepthStencilBufferData`
/// destructor, `thiscall`. Its table; under the list's lock (`0126f600`) `this` is removed from
/// the additional depth buffer list ([`fn_00e7db70`] with a stack word holding `this`); then
/// the `NiDX92DBufferData` part (the surface at +0x14 released and cleared, the block at +0xc
/// freed and cleared, the `NiRefObject` table and the global object count down).
pub fn fn_00e7dbc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem
        .set_u32(base, NI_DX9_ADDITIONAL_DEPTH_STENCIL_BUFFER_DATA_VTABLE);
    e.call(
        IMPORT_ENTER_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), base);
        fn_00e7db70(e, Ptr::new(BUFFER_LIST_0126F5CC), slot);
    });
    e.call(
        IMPORT_LEAVE_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
    buffer_data_down(e, base);
}

// Translated from 00e7dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`): the `NiDX9AdditionalDepthStencilBufferData`
/// deleting destructor, `thiscall` with the flags: the destructor (`00e7dbc0`), then with
/// bit 0 the 0x28 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e7dc40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e7dbc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x28u32]);
    }
    this
}

/// `00e89630` (unnamed; no arguments): answers a block from a pool (`0126f9a0`).
const POOL_BLOCK_00E89630: u32 = 0x00e8_9630;

// Translated from 00e7dc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map and not a function in the Ghidra project (read from its 0x30
/// bytes of code; `00e7e880` passes it as an element constructor), `thiscall`: clears the
/// words +4, +0xc, +0x14, +0x18 and +0x60, the 16-bit +0x5c and the bytes +0x5e and +0x5f,
/// sets the byte +0x64 to 1 and stores a pool block (`00e89630`) at +0x10. Answers `this`.
pub fn fn_00e7dc60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    e.mem.set_u32(base.wrapping_add(0x14), 0);
    e.mem.set_u32(base.wrapping_add(0x18), 0);
    e.mem.set_u16(base.wrapping_add(0x5c), 0);
    e.mem.set_u8(base.wrapping_add(0x5e), 0);
    e.mem.set_u8(base.wrapping_add(0x5f), 0);
    e.mem.set_u32(base.wrapping_add(0x60), 0);
    e.mem.set_u8(base.wrapping_add(0x64), 1);
    let block = e.call(POOL_BLOCK_00E89630, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0x10), block);
    this
}

/// The two objects `00e7dc90` drives (`0126f6c4`: its slot 0xa0 answers flags; `0126f6c8`:
/// its slots 0xcc and 0x68 take four words each, `thiscall`).
const FLAGS_SOURCE_0126F6C4: u32 = 0x0126_f6c4;
const STATE_TARGET_0126F6C8: u32 = 0x0126_f6c8;

// Translated from 00e7dc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`), `thiscall` with a byte. With the byte set and
/// the flags of the object at `0126f6c4` (its slot 0xa0, asked twice) without bit 3 and with
/// bit 2: the object at `0126f6c8` gets slot 0xcc with (+4 of `this`, 1, 3, 0) and (+4, 2, 3,
/// 0), then for each of 8 steps slot 0x68 with (0x80 + step, 0, 0, 0) and (0xc6 + step, 0, 0,
/// 0).
pub fn fn_00e7dc90(e: &mut Engine, this: Ptr, enable: u8) {
    if enable == 0 {
        return;
    }
    let source = e.mem.u32(FLAGS_SOURCE_0126F6C4);
    if e.vcall(source, 0xa0, &args![]).u32() & 8 != 0 {
        return;
    }
    let source = e.mem.u32(FLAGS_SOURCE_0126F6C4);
    if e.vcall(source, 0xa0, &args![]).u32() & 4 == 0 {
        return;
    }
    let base = this.addr();
    for stage in [1u32, 2] {
        let target = e.mem.u32(STATE_TARGET_0126F6C8);
        let word = e.mem.u32(base.wrapping_add(4));
        e.vcall(target, 0xcc, &args![word, stage, 3u32, 0u32]);
    }
    for step in 0..8u32 {
        let register = 0xc6 + step;
        let target = e.mem.u32(STATE_TARGET_0126F6C8);
        e.vcall(target, 0x68, &args![register - 0x46, 0u32, 0u32, 0u32]);
        let target = e.mem.u32(STATE_TARGET_0126F6C8);
        e.vcall(target, 0x68, &args![register, 0u32, 0u32, 0u32]);
    }
}

/// The pairs of state values `00e7de90` picks from (`0126f0b0`, two words per index) and the
/// setter `00e890c0` (unnamed; `thiscall`, four words).
const STATE_PAIRS_0126F0B0: u32 = 0x0126_f0b0;
const SET_SAMPLER_STATE_00E890C0: u32 = 0x00e8_90c0;

// Translated from 00e7de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7cd50`), `thiscall` with an index: the object at +0x10
/// gets `00e890c0` with (1, first word of pair `index` at `0126f0b0`, 0, 0) and then (2, its
/// second word, 0, 0).
pub fn fn_00e7de90(e: &mut Engine, this: Ptr, index: u32) {
    let base = this.addr();
    let pair = STATE_PAIRS_0126F0B0.wrapping_add(index.wrapping_mul(8));
    let first = e.mem.u32(pair);
    let target = e.mem.u32(base.wrapping_add(0x10));
    e.call(
        SET_SAMPLER_STATE_00E890C0,
        &args![target, 1u32, first, 0u32, 0u32],
    );
    let second = e.mem.u32(pair.wrapping_add(4));
    let target = e.mem.u32(base.wrapping_add(0x10));
    e.call(
        SET_SAMPLER_STATE_00E890C0,
        &args![target, 2u32, second, 0u32, 0u32],
    );
}

/// What the buffer data `Recreate` slots and makers below use besides the shared ones: the
/// render target group's resize `00a8f080` (unnamed; `thiscall` on the group: width,
/// height), the depth group maker `00a9ff60` (unnamed; `cdecl`: width, height, buffer),
/// `IDirect3DDevice9::GetDepthStencilSurface` (slot 0xa0) and
/// `IDirect3DCubeTexture9::GetCubeMapSurface` (slot 0x48; slot names by the interfaces'
/// order, the arguments match), the messages `"NiDX9AdditionalDepthStencilBufferData::
/// Recreate> FAILED %s"` (`010eef40`) and `"NiDX9Renderer::Recreate> Using %s backbuffer
/// format\n"` (`010eef7c`), and the `NiDX9ImplicitDepthStencilBufferData` table (`010ef114`,
/// RTTI).
const GROUP_RESIZE_00A8F080: u32 = 0x00a8_f080;
const MAKE_DEPTH_GROUP_00A9FF60: u32 = 0x00a9_ff60;
const GET_DEPTH_STENCIL_SURFACE: u32 = 0xa0;
const GET_CUBE_MAP_SURFACE: u32 = 0x48;
const RECREATE_FAILED_010EEF40: u32 = 0x010e_ef40;
const RECREATE_USING_FORMAT_010EEF7C: u32 = 0x010e_ef7c;
const NI_DX9_IMPLICIT_DEPTH_STENCIL_BUFFER_DATA_VTABLE: u32 = 0x010e_f114;

/// After a surface (+0x14) was got: its description (`GetDesc` into `desc`); a failure
/// releases and clears the surface and answers `false`.
fn describe_surface(e: &mut Engine, base: u32, desc: u32) -> bool {
    let surface = e.mem.u32(base.wrapping_add(0x14));
    if e.vcall(surface, GET_DESC, &args![desc]).i32() < 0 {
        release_and_clear(e, base.wrapping_add(0x14));
        return false;
    }
    true
}

// Translated from 00e7ce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9ImplicitDepthStencilBufferData`'s slot +0x58, `thiscall`
/// with the device): releases the old surface first (its own slot +0x54, when +0x14 is set),
/// gets the device's depth stencil surface into +0x14 (`false` on failure) and its
/// description (`false`, the surface released, on failure); then +0xc gets the converted
/// format (`00e7c500`), the group (+8) is resized to the surface's width and height
/// (`00a8f080`) and the answer is `true`.
pub fn fn_00e7ce30(e: &mut Engine, this: Ptr, device: u32) -> bool {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x14)) != 0 {
        e.vcall(base, 0x54, &args![]);
    }
    let result = e
        .vcall(
            device,
            GET_DEPTH_STENCIL_SURFACE,
            &args![base.wrapping_add(0x14)],
        )
        .i32();
    if result < 0 {
        return false;
    }
    e.with_stack(0x20, |e, desc| {
        let desc = desc.addr();
        if !describe_surface(e, base, desc) {
            return false;
        }
        let format = e.mem.u32(desc);
        let converted = e.call(CONVERT_FORMAT_00E7C500, &args![format]).u32();
        let (width, height) = (e.mem.u32(desc + 0x18), e.mem.u32(desc + 0x1c));
        let group = e.mem.u32(base.wrapping_add(8));
        e.mem.set_u32(base.wrapping_add(0xc), converted);
        e.call(GROUP_RESIZE_00A8F080, &args![group, width, height]);
        true
    })
}

// Translated from 00e7cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the device; called by the renderer's reset code):
/// under the list's lock (`0126f600`), gives the device to virtual slot 0x58 (`Recreate`) of
/// every element of the additional depth buffer list (`0126f5cc`), the next item read before
/// each call.
pub fn fn_00e7cf00(e: &mut Engine, device: u32) {
    e.call(
        IMPORT_ENTER_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
    let mut item = e.mem.u32(BUFFER_LIST_0126F5CC);
    while item != 0 {
        let element = e.mem.u32(item.wrapping_add(8));
        item = e.mem.u32(item);
        e.vcall(element, 0x58, &args![device]);
    }
    e.call(
        IMPORT_LEAVE_CRITICAL_SECTION,
        &args![BUFFER_LIST_LOCK_0126F600],
    );
}

// Translated from 00e7cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9AdditionalDepthStencilBufferData`'s slot +0x58, the
/// `Recreate` its message names; `thiscall` with the device): releases the old surface first
/// (its own slot +0x54, when +0x14 is set); `false` without a format (+0x20); otherwise the
/// rest is [`fn_00e7cf72`] (the jump target the exe continues at).
pub fn fn_00e7cf50(e: &mut Engine, this: Ptr, device: u32) -> bool {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x14)) != 0 {
        e.vcall(base, 0x54, &args![]);
    }
    if e.mem.u32(base.wrapping_add(0x20)) == 0 {
        return false;
    }
    fn_00e7cf72(e, this, device)
}

// Translated from 00e7cf72 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a function of its own: the rest of `fn_00e7cf50` (its jump target, reached with
/// `this` in EDI and `fn_00e7cf50`'s frame), registered with the words that function has
/// (`this`, the device). The renderer (`011f4748`) gets its slot 0xc8; Direct3D
/// (`0126f0d8`) is asked whether the format (+0x20) works as a depth stencil surface
/// (`CheckDeviceFormat` with the renderer's adapter, device type and adapter format, usage 2,
/// resource type 1); then the device creates the surface (+0x14) with the group's (+8) width
/// and height (+8, +0xc), the format, the kind's (+0x10) multisample type, quality 0 and
/// +0x24. Either failure is reported (`00a29680` with `"...Recreate> FAILED %s"` and
/// `NiXenonErrorString`) and answers `false`; a failed surface description releases and
/// clears the surface and answers `false`. Otherwise +0xc gets the converted format
/// (`00e7c500`) and the answer is `true`.
pub fn fn_00e7cf72(e: &mut Engine, this: Ptr, device: u32) -> bool {
    let base = this.addr();
    let renderer = e.mem.u32(SOURCE_011F4748);
    e.vcall(renderer, 0xc8, &args![]);
    let format = e.mem.u32(base.wrapping_add(0x20));
    let adapter_format = e.mem.u32(renderer.wrapping_add(0x5d0));
    let device_type = e.mem.u32(renderer.wrapping_add(0x5c8));
    let direct3d = e.mem.u32(DIRECT3D_0126F0D8);
    let adapter = e.mem.u32(renderer.wrapping_add(0x5c4));
    let mut result = e
        .vcall(
            direct3d,
            CHECK_DEVICE_FORMAT,
            &args![adapter, device_type, adapter_format, 2u32, 1u32, format],
        )
        .i32();
    if result >= 0 {
        let samples = multisample_type(e.mem.u32(base.wrapping_add(0x10)));
        let discard = e.mem.u32(base.wrapping_add(0x24));
        let group = e.mem.u32(base.wrapping_add(8));
        let height = e.mem.u32(group.wrapping_add(0xc));
        let width = e.mem.u32(group.wrapping_add(8));
        let format = e.mem.u32(base.wrapping_add(0x20));
        result = e
            .vcall(
                device,
                CREATE_DEPTH_STENCIL_SURFACE,
                &args![
                    width,
                    height,
                    format,
                    samples,
                    0u32,
                    discard,
                    base.wrapping_add(0x14),
                    0u32
                ],
            )
            .i32();
        if result >= 0 {
            return e.with_stack(0x20, |e, desc| {
                let desc = desc.addr();
                if !describe_surface(e, base, desc) {
                    return false;
                }
                let format = e.mem.u32(desc);
                let converted = e.call(CONVERT_FORMAT_00E7C500, &args![format]).u32();
                e.mem.set_u32(base.wrapping_add(0xc), converted);
                true
            });
        }
    }
    let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
    e.call(REPORT_00A29680, &args![RECREATE_FAILED_010EEF40, name]);
    false
}

// Translated from 00e7d0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9ImplicitBufferData`'s slot +0x58, `thiscall` with the
/// device): releases the old surface first (its own slot +0x54, when +0x14 is set), clears
/// the globals `0126f580` and `0126f590`, releases the held device (+0x58, its slot 8) and
/// keeps the new one there (counted: its slot 4). The device's render target 0 goes to +0x14
/// (`false` on failure); a failed surface description releases and clears it and answers
/// `false`. Otherwise the group (+8) is resized to the surface's width and height
/// (`00a8f080`), +0xc gets the converted format (`00e7c500`), the format's name is reported
/// (`"NiDX9Renderer::Recreate> Using %s backbuffer format\n"`) and the answer is `true`.
pub fn fn_00e7d0a0(e: &mut Engine, this: Ptr, device: u32) -> bool {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x14)) != 0 {
        e.vcall(base, 0x54, &args![]);
    }
    e.mem.set_u32(RENDER_TARGETS_0126F580, 0);
    e.mem.set_u32(DEPTH_SURFACE_0126F590, 0);
    release_and_clear(e, base.wrapping_add(0x58));
    e.mem.set_u32(base.wrapping_add(0x58), device);
    e.vcall(device, 4, &args![]);
    let held = e.mem.u32(base.wrapping_add(0x58));
    let result = e
        .vcall(
            held,
            GET_RENDER_TARGET,
            &args![0u32, base.wrapping_add(0x14)],
        )
        .i32();
    if result < 0 {
        return false;
    }
    e.with_stack(0x20, |e, desc| {
        let desc = desc.addr();
        if !describe_surface(e, base, desc) {
            return false;
        }
        let (width, height) = (e.mem.u32(desc + 0x18), e.mem.u32(desc + 0x1c));
        let group = e.mem.u32(base.wrapping_add(8));
        e.call(GROUP_RESIZE_00A8F080, &args![group, width, height]);
        let format = e.mem.u32(desc);
        let converted = e.call(CONVERT_FORMAT_00E7C500, &args![format]).u32();
        e.mem.set_u32(base.wrapping_add(0xc), converted);
        let format = e.mem.u32(desc);
        let name = e.call(FORMAT_NAME_00E7A9A0, &args![format]).u32();
        e.call(
            REPORT_00A29680,
            &args![RECREATE_USING_FORMAT_010EEF7C, name],
        );
        true
    })
}

/// A new `NiDX9TextureBufferData`-style object of `size` bytes as the makers below build it:
/// the `NiRefObject` start (count 0, the global count up), +8, +0xc and +0x14 cleared and
/// `vtable`; null when the allocation fails.
fn new_buffer_data(e: &mut Engine, size: u32, vtable: u32) -> u32 {
    let block = e.call(NEW_BUFFER_OBJECT, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(block + 4, 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x14, 0);
    e.mem.set_u32(block, vtable);
    block
}

// Translated from 00e7d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words; called by `00e901a0` for each of the six
/// faces of a cube map, the third and fourth words never read): makes an
/// `NiDX9TextureBufferData` (0x24 bytes, the table `010eefbc`; null when the allocation
/// fails, the stores still go to it) for one face of `texture`: the texture kept at +0x20
/// (counted: its slot 4) and the face's level 0 surface got into +0x14 (`GetCubeMapSurface`
/// with the face and 0). A failure is reported (`"NiDX9TextureBufferData::Create> FAILED-
/// GetSurfaceLevel - %s"`), +0x14 cleared, the object deleted (its slot 0 with 1) and null
/// answered; a failed surface description deletes it too, without a report. Otherwise +0xc
/// gets the converted format (`00e7c500`) and the group word a new group (`00a8f2c0` with the
/// surface's width and height and the buffer) or, when it holds one, the buffer attached to
/// it (`00a8f140`); the group goes to +8. Answers the object.
pub fn fn_00e7d390(
    e: &mut Engine,
    texture: u32,
    face: u32,
    _unused_3: u32,
    _unused_4: u32,
    group_out: Ptr,
) -> u32 {
    let data = new_buffer_data(e, 0x24, NI_DX9_TEXTURE_BUFFER_DATA_VTABLE);
    if data != 0 {
        e.mem.set_u32(data + 0x20, 0);
    }
    e.mem.set_u32(data.wrapping_add(0x20), texture);
    e.vcall(texture, 4, &args![]);
    let held = e.mem.u32(data.wrapping_add(0x20));
    let result = e
        .vcall(
            held,
            GET_CUBE_MAP_SURFACE,
            &args![face, 0u32, data.wrapping_add(0x14)],
        )
        .i32();
    if result < 0 {
        let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
        e.call(
            REPORT_00A29680,
            &args![TEXTURE_BUFFER_FAILED_010EF028, name],
        );
        e.mem.set_u32(data.wrapping_add(0x14), 0);
        e.vcall(data, 0, &args![1u32]);
        return 0;
    }
    e.with_stack(0x20, |e, desc| {
        let desc = desc.addr();
        let surface = e.mem.u32(data.wrapping_add(0x14));
        if e.vcall(surface, GET_DESC, &args![desc]).i32() < 0 {
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        let format = e.mem.u32(desc);
        let converted = e.call(CONVERT_FORMAT_00E7C500, &args![format]).u32();
        e.mem.set_u32(data.wrapping_add(0xc), converted);
        let held = e.mem.u32(group_out.addr());
        if held == 0 {
            let (width, height) = (e.mem.u32(desc + 0x18), e.mem.u32(desc + 0x1c));
            let group = e
                .call(MAKE_GROUP_00A8F2C0, &args![width, height, data])
                .u32();
            e.mem.set_u32(group_out.addr(), group);
            e.mem.set_u32(data.wrapping_add(8), group);
        } else {
            e.call(ATTACH_00A8F140, &args![held, data]);
            let group = e.mem.u32(group_out.addr());
            e.mem.set_u32(data.wrapping_add(8), group);
        }
        data
    })
}

// Translated from 00e7d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9TextureBufferData`'s slot +0x68, `thiscall`): releases
/// and clears the surface (+0x14), gives the block at +0xc back to the memory manager and
/// clears it, then releases and clears the texture (+0x20).
pub fn fn_00e7d490(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_and_clear(e, base.wrapping_add(0x14));
    let block = e.mem.u32(base.wrapping_add(0xc));
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, block]);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    release_and_clear(e, base.wrapping_add(0x20));
}

// Translated from 00e7d560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9ImplicitBufferData`'s slot +0x68, `thiscall`): as
/// `fn_00e7d490` with the device (+0x58) in place of the texture.
pub fn fn_00e7d560(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_and_clear(e, base.wrapping_add(0x14));
    let block = e.mem.u32(base.wrapping_add(0xc));
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, block]);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    release_and_clear(e, base.wrapping_add(0x58));
}

// Translated from 00e7d620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiDX92DBufferData` deleting destructor (the table it
/// stores; also slot 0 of `NiDX9ImplicitDepthStencilBufferData`), `thiscall` with the flags:
/// the `NiDX92DBufferData` part (the surface at +0x14 released and cleared, the block at +0xc
/// freed and cleared, the `NiRefObject` table and the global object count down), then with
/// bit 0 the 0x20 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e7d620(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    buffer_data_down(e, this.addr());
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00e7d680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiDX9ImplicitDepthStencilBufferData`'s slot +0xc): answers
/// the address `0126f5b4`.
pub fn fn_00e7d680(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_f5b4
}

// Translated from 00e7d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the device and the render target group word; called
/// by the renderer's start-up): makes the implicit depth stencil buffer data (an
/// `NiDX9ImplicitDepthStencilBufferData`, 0x20 bytes, the table `010ef114`; null when the
/// allocation fails, the stores still go to it). The device's depth stencil surface goes to
/// +0x14; on failure the object (when made) is deleted (its slot 0 with 1) and null answered;
/// a failed surface description deletes it and answers null. Otherwise +0xc gets the
/// converted format (`00e7c500`), the group word a new group (`00a9ff60` with the surface's
/// width and height and the buffer) or, when it holds one, the buffer attached to it
/// (`00a8f140`); the group goes to +8 and the surface to `0126f590`. Answers the object.
pub fn fn_00e7d850(e: &mut Engine, device: u32, group_out: Ptr) -> u32 {
    let data = new_buffer_data(e, 0x20, NI_DX9_IMPLICIT_DEPTH_STENCIL_BUFFER_DATA_VTABLE);
    let surface_at = data.wrapping_add(0x14);
    let result = e
        .vcall(device, GET_DEPTH_STENCIL_SURFACE, &args![surface_at])
        .i32();
    if result < 0 {
        if data != 0 {
            e.vcall(data, 0, &args![1u32]);
        }
        return 0;
    }
    e.with_stack(0x20, |e, desc| {
        let desc = desc.addr();
        let surface = e.mem.u32(surface_at);
        if e.vcall(surface, GET_DESC, &args![desc]).i32() < 0 {
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        let format = e.mem.u32(desc);
        let converted = e.call(CONVERT_FORMAT_00E7C500, &args![format]).u32();
        e.mem.set_u32(data.wrapping_add(0xc), converted);
        let held = e.mem.u32(group_out.addr());
        if held == 0 {
            let (width, height) = (e.mem.u32(desc + 0x18), e.mem.u32(desc + 0x1c));
            let group = e
                .call(MAKE_DEPTH_GROUP_00A9FF60, &args![width, height, data])
                .u32();
            e.mem.set_u32(group_out.addr(), group);
        } else {
            e.call(ATTACH_00A8F140, &args![held, data]);
        }
        let group = e.mem.u32(group_out.addr());
        let surface = e.mem.u32(surface_at);
        e.mem.set_u32(data.wrapping_add(8), group);
        e.mem.set_u32(DEPTH_SURFACE_0126F590, surface);
        data
    })
}

/// The two stage limits `fn_00e7dd50` runs to (`011bf558`, `011bf560`).
const STAGE_LIMIT_011BF558: u32 = 0x011b_f558;
const STAGE_LIMIT_011BF560: u32 = 0x011b_f560;

// Translated from 00e7dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the first stage; called by `SetupRenderingPass`): for
/// each stage from `first` below the limit at `011bf558`, the object at `0126f6c8` gets its
/// slot 0xdc with (stage, 0) and its slot 0xc0 with (stage, 1, 1, 0), (stage, 4, 1, 0) and
/// (stage, 0x18, 0, 0); then for each further stage below the limit at `011bf560` only slot
/// 0xdc with (stage, 0). The object is read again for each call.
pub fn fn_00e7dd50(e: &mut Engine, first: u32) {
    let mut stage = first;
    while stage < e.mem.u32(STAGE_LIMIT_011BF558) {
        let target = e.mem.u32(STATE_TARGET_0126F6C8);
        e.vcall(target, 0xdc, &args![stage, 0u32]);
        for (state, value) in [(1u32, 1u32), (4, 1), (0x18, 0)] {
            let target = e.mem.u32(STATE_TARGET_0126F6C8);
            e.vcall(target, 0xc0, &args![stage, state, value, 0u32]);
        }
        stage = stage.wrapping_add(1);
    }
    while stage < e.mem.u32(STAGE_LIMIT_011BF560) {
        let target = e.mem.u32(STATE_TARGET_0126F6C8);
        e.vcall(target, 0xdc, &args![stage, 0u32]);
        stage = stage.wrapping_add(1);
    }
}

/// `00e89060` (`thiscall`, three words: a state number, the address of the value, the
/// address of a flag byte; `bool`).
const STATE_LOOKUP_00E89060: u32 = 0x00e8_9060;

// Translated from 00e7ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a texture stage, no stack words; called by
/// `00e8d890`): asks the object at the stage's +0x10 for state 0xb (`00e89060`, with the
/// addresses of a value and of a flag byte, both zeroed first) and answers the value when it
/// answers `true`, else the stage's word +4.
pub fn fn_00e7ddf0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let object = e.mem.u32(base.wrapping_add(0x10));
    e.with_stack(8, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 4, 0);
        e.mem.set_u8(frame + 3, 0);
        let found = e
            .call(
                STATE_LOOKUP_00E89060,
                &args![object, 0xbu32, frame + 4, frame + 3],
            )
            .u8();
        if found != 0 {
            e.mem.u32(frame + 4)
        } else {
            e.mem.u32(base.wrapping_add(4))
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_ce30, fn_00e7ce30(Ptr, u32) -> bool),
        entry!(0x00e7_cf00, fn_00e7cf00(u32)),
        entry!(0x00e7_cf50, fn_00e7cf50(Ptr, u32) -> bool),
        entry!(0x00e7_cf72, fn_00e7cf72(Ptr, u32) -> bool),
        entry!(0x00e7_d0a0, fn_00e7d0a0(Ptr, u32) -> bool),
        entry!(0x00e7_d390, fn_00e7d390(u32, u32, u32, u32, Ptr) -> u32),
        entry!(0x00e7_d490, fn_00e7d490(Ptr)),
        entry!(0x00e7_d560, fn_00e7d560(Ptr)),
        entry!(0x00e7_d620, fn_00e7d620(Ptr, u32) -> Ptr),
        entry!(0x00e7_d680, fn_00e7d680(Ptr) -> u32),
        entry!(0x00e7_d850, fn_00e7d850(u32, Ptr) -> u32),
        entry!(0x00e7_dd50, fn_00e7dd50(u32)),
        entry!(0x00e7_ddf0, fn_00e7ddf0(Ptr) -> u32),
        entry!(0x00e7_d7f0, fn_00e7d7f0(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(0x00e7_ec90, fn_00e7ec90()),
        entry!(0x00e7_d5b0, fn_00e7d5b0(Ptr<NiTPointerList>) -> u32),
        entry!(0x00e7_de30, fn_00e7de30(Ptr)),
        entry!(0x00e7_d6b0, fn_00e7d6b0(u32, Ptr, Ptr) -> u32),
        entry!(0x00e7_cd50, fn_00e7cd50(Ptr, u32, u32) -> bool),
        entry!(0x00e7_cdb0, fn_00e7cdb0(Ptr, u32) -> bool),
        entry!(0x00e7_cdf0, fn_00e7cdf0(Ptr) -> bool),
        entry!(0x00e7_ce10, fn_00e7ce10(Ptr) -> bool),
        entry!(0x00e7_cec0, fn_00e7cec0()),
        entry!(0x00e7_d180, fn_00e7d180(Ptr)),
        entry!(0x00e7_d1b0, fn_00e7d1b0(Ptr)),
        entry!(0x00e7_d220, fn_00e7d220(Ptr) -> u32),
        entry!(0x00e7_d230, fn_00e7d230(Ptr, u32) -> Ptr),
        entry!(0x00e7_d250, fn_00e7d250(u32, Ptr, u32, u32) -> u32),
        entry!(0x00e7_d4e0, fn_00e7d4e0(Ptr)),
        entry!(0x00e7_d550, fn_00e7d550(Ptr) -> u32),
        entry!(0x00e7_d690, fn_00e7d690(Ptr, u32) -> Ptr),
        entry!(0x00e7_d940, fn_00e7d940(Ptr) -> Ptr),
        entry!(0x00e7_d9c0, fn_00e7d9c0(Ptr) -> u32),
        entry!(0x00e7_d9d0, fn_00e7d9d0(u32, Ptr, u32, u32) -> u32),
        entry!(0x00e7_db70, fn_00e7db70(Ptr<NiTPointerList>, Ptr) -> u32),
        entry!(0x00e7_dbc0, fn_00e7dbc0(Ptr)),
        entry!(0x00e7_dc40, fn_00e7dc40(Ptr, u32) -> Ptr),
        entry!(0x00e7_dc60, fn_00e7dc60(Ptr) -> Ptr),
        entry!(0x00e7_dc90, fn_00e7dc90(Ptr, u8)),
        entry!(0x00e7_de90, fn_00e7de90(Ptr, u32)),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_or_stores_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.call(0x00e7_ec90, &args![]);
        assert_eq!(e.mem.u32(0x0126_f6cc), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e7_ec90, &args![]);
        assert_eq!(e.mem.u32(0x0126_f6cc), 0);
    }
}

#[cfg(test)]
mod remove_tests {
    use super::*;

    /// A list of three items (elements 10, 20, 30) and a position word.
    fn engine() -> (Engine, u32, [u32; 3], u32) {
        let mut e = Engine::new();
        e.register(REMOVE_HEAD_00B602D0, |_, _| 0x111u32.into_ret());
        e.register(REMOVE_TAIL_00E7D5B0, |_, _| 0x333u32.into_ret());
        e.register(FREE_ITEM_00E6E8A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let list = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        for (i, item) in items.iter().enumerate() {
            let next = if i < 2 { items[i + 1] } else { 0 };
            let previous = if i > 0 { items[i - 1] } else { 0 };
            e.mem.set_u32(*item, next);
            e.mem.set_u32(item + 4, previous);
            e.mem.set_u32(item + 8, 10 * (i as u32 + 1));
        }
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[2]);
        e.mem.set_u32(list + 8, 3);
        let position = e.mem.alloc(4);
        (e, list, items, position)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e7d7f0_middle_item_is_unlinked_and_released() {
        let (mut e, list, items, position) = engine();
        e.mem.set_u32(position, items[1]);
        assert_eq!(e.call(0x00e7_d7f0, &args![list, position]).u32(), 20);
        assert_eq!(e.mem.u32(position), items[2]);
        assert_eq!(e.mem.u32(items[0]), items[2]);
        assert_eq!(e.mem.u32(items[2] + 4), items[0]);
        assert_eq!(e.mem.u32(list + 8), 2);
        assert_eq!(calls(&e, FREE_ITEM_00E6E8A0), vec![vec![list, items[1]]]);
    }

    #[test]
    fn fn_00e7d7f0_head_and_tail_go_to_the_helpers() {
        let (mut e, list, items, position) = engine();
        e.mem.set_u32(position, items[0]);
        assert_eq!(e.call(0x00e7_d7f0, &args![list, position]).u32(), 0x111);
        assert_eq!(e.mem.u32(position), items[1]);
        assert_eq!(calls(&e, REMOVE_HEAD_00B602D0), vec![vec![list]]);
        e.mem.set_u32(position, items[2]);
        assert_eq!(e.call(0x00e7_d7f0, &args![list, position]).u32(), 0x333);
        assert_eq!(e.mem.u32(position), 0);
        assert_eq!(calls(&e, REMOVE_TAIL_00E7D5B0), vec![vec![list]]);
        assert_eq!(e.mem.u32(list + 8), 3);
        assert!(calls(&e, FREE_ITEM_00E6E8A0).is_empty());
    }
}

#[cfg(test)]
mod remove_tail_tests {
    use super::*;

    /// A list of two items (elements 10, 20), the pool's lock taken once by someone else.
    fn engine() -> (Engine, u32, [u32; 2]) {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0e10_0000);
        e.mem.set_u32(NODE_POOL_LOCK, 7);
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let list = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 4, 0);
        e.mem.set_u32(items[0] + 8, 10);
        e.mem.set_u32(items[1], 0);
        e.mem.set_u32(items[1] + 4, items[0]);
        e.mem.set_u32(items[1] + 8, 20);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[1]);
        e.mem.set_u32(list + 8, 2);
        (e, list, items)
    }

    #[test]
    fn fn_00e7d5b0_removes_the_tail_and_frees_its_item() {
        let (mut e, list, items) = engine();
        assert_eq!(e.call(0x00e7_d5b0, &args![list]).u32(), 20);
        assert_eq!(e.mem.u32(list + 4), items[0]);
        assert_eq!(e.mem.u32(items[0]), 0);
        assert_eq!(e.mem.u32(list), items[0]);
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.mem.u32(items[1]), 0x0e10_0000);
        assert_eq!(e.mem.u32(items[1] + 4), 0);
        assert_eq!(e.mem.u32(items[1] + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), items[1]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        // The last item: the head is cleared too.
        assert_eq!(e.call(0x00e7_d5b0, &args![list]).u32(), 10);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), items[0]);
        assert_eq!(e.mem.u32(items[0]), items[1]);
    }

    #[test]
    fn fn_00e7d5b0_keeps_a_lock_still_held() {
        let (mut e, list, _) = engine();
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.call(0x00e7_d5b0, &args![list]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 7);
    }
}

#[cfg(test)]
mod element_clean_up_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_EMPTY_0109B220, 0x0e70_0008u32);
        for target in [
            IMPORT_INTERLOCKED_INCREMENT,
            IMPORT_INTERLOCKED_DECREMENT,
            RELEASE_STRING_00AA04E0,
            RELEASE_00E89670,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e7de30_holds_and_releases_the_entry_string() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry + 8, 0x0e71_0008);
        e.mem.set_u32(this + 0x18, entry);
        e.mem.set_u32(this + 0x10, 0x55);
        e.call(0x00e7_de30, &args![this]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![0x0e71_0000]]
        );
        let slot = calls(&e, RELEASE_STRING_00AA04E0)[0][0];
        assert_ne!(slot, entry + 8);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e71_0000]]
        );
        assert_eq!(calls(&e, RELEASE_00E89670), vec![vec![0x55]]);
    }

    #[test]
    fn fn_00e7de30_skips_the_empty_string_and_a_missing_entry() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry + 8, 0x0e70_0008);
        e.mem.set_u32(this + 0x18, entry);
        e.call(0x00e7_de30, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls(&e, RELEASE_STRING_00AA04E0).len(), 1);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00e7_de30, &args![this]);
        assert_eq!(calls(&e, RELEASE_STRING_00AA04E0).len(), 1);
        assert_eq!(calls(&e, RELEASE_00E89670).len(), 2);
    }
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    const DEVICE_VTABLE: u32 = 0x6000;
    const SURFACE_VTABLE: u32 = 0x6400;
    const DATA_DELETE: u32 = 0x6800;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(target_ok: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(SOURCE_011F4748, 4);
        let mut slots = vec![0u32; 0x40];
        slots[1] = 0x6100;
        slots[0x26] = 0x6198;
        slots[0x36] = 0x61d8;
        slots[0x37] = 0x61dc;
        e.put_vtable(DEVICE_VTABLE, &slots);
        let mut surface_slots = vec![0u32; 0x10];
        surface_slots[0xc] = 0x6430;
        e.put_vtable(SURFACE_VTABLE, &surface_slots);
        let surface = e.mem.alloc(4);
        e.mem.set_u32(surface, SURFACE_VTABLE);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(SOURCE_011F4748, device);
        e.register(0x6100, |_, _| Ret::default());
        e.register_double(0x6198, move |e, a| {
            e.mem.set_u32(a[2], surface);
            (if target_ok { 0u32 } else { 0x8876_086c }).into_ret()
        });
        e.register(0x61d8, |_, _| 0x55u32.into_ret());
        e.register(0x61dc, |_, _| 1u32.into_ret());
        e.register(0x6430, |e, a| {
            e.mem.set_u32(a[1], 0x16);
            e.mem.set_u32(a[1] + 0x18, 640);
            e.mem.set_u32(a[1] + 0x1c, 480);
            0u32.into_ret()
        });
        e.register(NEW_BUFFER_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        for no_op in [
            IMPORT_INTERLOCKED_INCREMENT,
            MEMCPY,
            REPORT_00A29680,
            ATTACH_00A8F140,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(CONVERT_FORMAT_00E7C500, |_, a| (a[0] + 0x100).into_ret());
        e.register(FORMAT_NAME_00E7A9A0, |_, _| 0x9999u32.into_ret());
        e.register(MAKE_GROUP_00A8F2C0, |_, _| 0x4444u32.into_ret());
        e.register(DATA_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, device, surface)
    }

    #[test]
    fn fn_00e7d6b0_makes_the_buffer_data_and_its_group() {
        let (mut e, device, surface) = rig(true);
        let present = e.mem.alloc(0x38);
        let group = e.mem.alloc(4);
        let data = e.call(0x00e7_d6b0, &args![device, present, group]).u32();
        assert_eq!(e.mem.u32(data), NI_DX9_IMPLICIT_BUFFER_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x58), device);
        assert_eq!(e.mem.u32(data + 0x14), surface);
        assert_eq!(e.mem.u32(data + 0xc), 0x116);
        assert_eq!(calls(&e, MEMCPY), vec![vec![data + 0x20, present, 0x38]]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![USING_FORMAT_010EF0D8, 0x9999]]
        );
        assert_eq!(e.mem.u32(SLOT_D8_0126F594), 0x55);
        assert_eq!(e.mem.u8(SLOT_DC_0126F598), 1);
        assert_eq!(calls(&e, MAKE_GROUP_00A8F2C0), vec![vec![640, 480, data]]);
        assert_eq!(e.mem.u32(group), 0x4444);
        assert_eq!(e.mem.u32(data + 8), 0x4444);
        // A second buffer is attached to the group; the globals are not read again.
        let data = e.call(0x00e7_d6b0, &args![device, present, group]).u32();
        assert_eq!(calls(&e, ATTACH_00A8F140), vec![vec![0x4444, data]]);
        assert_eq!(calls(&e, 0x61d8).len(), 1);
    }

    #[test]
    fn fn_00e7d6b0_deletes_the_data_without_a_render_target() {
        let (mut e, device, _) = rig(false);
        let present = e.mem.alloc(0x38);
        let group = e.mem.alloc(4);
        // The class's table must reach the delete: map it with slot 0.
        e.put_vtable(NI_DX9_IMPLICIT_BUFFER_DATA_VTABLE, &[DATA_DELETE]);
        let result = e.call(0x00e7_d6b0, &args![device, present, group]).u32();
        assert_eq!(result, 0);
        assert_eq!(calls(&e, DATA_DELETE).len(), 1);
        assert_eq!(calls(&e, DATA_DELETE)[0][1], 1);
        assert!(calls(&e, MAKE_GROUP_00A8F2C0).is_empty());
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose table at `vtable` has `slots` (byte offset, target).
    fn object(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let mut words = vec![0u32; 0x80];
        for (slot, target) in slots {
            words[(*slot / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &words);
        let object = e.mem.alloc(0x60);
        e.mem.set_u32(object, vtable);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        for no_op in [
            IMPORT_INTERLOCKED_INCREMENT,
            IMPORT_INTERLOCKED_DECREMENT,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            MEMORY_MANAGER_FREE,
            FREE_00AA1460,
            REPORT_00A29680,
            ATTACH_00A8F140,
            SET_SAMPLER_STATE_00E890C0,
            REMOVE_HEAD_00B602D0,
            REMOVE_TAIL_00E7D5B0,
            FREE_ITEM_00E6E8A0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(NEW_BUFFER_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ERROR_STRING_00E89D50, |_, _| 0x5151u32.into_ret());
        e.register(CONVERT_FORMAT_00E7C500, |_, a| (a[0] + 0x100).into_ret());
        e.register(MAKE_GROUP_00A8F2C0, |_, _| 0x4444u32.into_ret());
        for target in [0x7000u32, 0x7004, 0x7008, 0x700c, 0x7010] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e7cd50_sets_a_render_target_once() {
        let mut e = engine();
        e.register(0x7094, |_, a| {
            (if a[2] == 0x99 { 0x8000_0000u32 } else { 0 }).into_ret()
        });
        let device = object(&mut e, 0x6000, &[(SET_RENDER_TARGET, 0x7094)]);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(SLOT_D8_0126F594, 2);
        assert!(!e.call(0x00e7_cd50, &args![this, device, 1u32]).bool());
        e.mem.set_u32(this + 0x14, 0x55);
        assert!(!e.call(0x00e7_cd50, &args![this, device, 3u32]).bool());
        assert!(e.call(0x00e7_cd50, &args![this, device, 1u32]).bool());
        assert_eq!(calls(&e, 0x7094), vec![vec![device, 1, 0x55]]);
        assert_eq!(e.mem.u32(RENDER_TARGETS_0126F580 + 4), 0x55);
        assert!(e.call(0x00e7_cd50, &args![this, device, 1u32]).bool());
        assert_eq!(calls(&e, 0x7094).len(), 1);
        // The surface at +0x18 wins; a failure leaves the record.
        e.mem.set_u32(this + 0x18, 0x99);
        assert!(!e.call(0x00e7_cd50, &args![this, device, 1u32]).bool());
        assert_eq!(e.mem.u32(RENDER_TARGETS_0126F580 + 4), 0x55);
    }

    #[test]
    fn fn_00e7cdb0_sets_the_depth_surface_once() {
        let mut e = engine();
        e.register(0x709c, |_, a| {
            (if a[1] == 0x99 { 0x8000_0000u32 } else { 0 }).into_ret()
        });
        let device = object(&mut e, 0x6000, &[(SET_DEPTH_STENCIL_SURFACE, 0x709c)]);
        let this = e.mem.alloc(0x24);
        assert!(!e.call(0x00e7_cdb0, &args![this, device]).bool());
        e.mem.set_u32(this + 0x14, 0x55);
        assert!(e.call(0x00e7_cdb0, &args![this, device]).bool());
        assert!(e.call(0x00e7_cdb0, &args![this, device]).bool());
        assert_eq!(calls(&e, 0x709c), vec![vec![device, 0x55]]);
        assert_eq!(e.mem.u32(DEPTH_SURFACE_0126F590), 0x55);
        e.mem.set_u32(this + 0x14, 0x99);
        assert!(!e.call(0x00e7_cdb0, &args![this, device]).bool());
        assert_eq!(e.mem.u32(DEPTH_SURFACE_0126F590), 0x55);
    }

    #[test]
    fn fn_00e7cdf0_and_fn_00e7ce10_ask_the_channel_bits() {
        let mut e = engine();
        e.register(CHANNEL_BITS_00A77980, |_, a| {
            (0xffff_ff00 | (a[1] - 0x11)).into_ret()
        });
        let this = e.mem.alloc(0x24);
        assert!(!e.call(0x00e7_cdf0, &args![this]).bool());
        e.mem.set_u32(this + 0xc, 0x77);
        assert!(!e.call(0x00e7_cdf0, &args![this]).bool());
        assert!(calls(&e, CHANNEL_BITS_00A77980).is_empty());
        e.mem.set_u32(this + 0x14, 0x55);
        assert!(!e.call(0x00e7_cdf0, &args![this]).bool());
        assert!(e.call(0x00e7_ce10, &args![this]).bool());
        assert_eq!(
            calls(&e, CHANNEL_BITS_00A77980),
            vec![vec![0x77, 0x11], vec![0x77, 0x12]]
        );
    }

    #[test]
    fn fn_00e7cec0_visits_every_buffer_under_the_lock() {
        let mut e = engine();
        e.register(0x7054, |_, _| Ret::default());
        let first = object(&mut e, 0x6000, &[(0x54, 0x7054)]);
        let second = e.mem.alloc(4);
        e.mem.set_u32(second, 0x6000);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 8, first);
        e.mem.set_u32(items[1] + 8, second);
        e.mem.set_u32(BUFFER_LIST_0126F5CC, items[0]);
        e.call(0x00e7_cec0, &args![]);
        assert_eq!(calls(&e, 0x7054), vec![vec![first], vec![second]]);
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![BUFFER_LIST_LOCK_0126F600]]
        );
        assert_eq!(calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn fn_00e7d180_releases_the_surface_and_frees_the_block() {
        let mut e = engine();
        let surface = object(&mut e, 0x6000, &[(8, 0x7008)]);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x14, surface);
        e.mem.set_u32(this + 0xc, 0x1234);
        e.call(0x00e7_d180, &args![this]);
        assert_eq!(calls(&e, 0x7008), vec![vec![surface]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x1234]]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // Without a surface only the block goes.
        e.call(0x00e7_d180, &args![this]);
        assert_eq!(calls(&e, 0x7008).len(), 1);
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE).len(), 2);
    }

    #[test]
    fn destructors_release_their_members_and_count_down() {
        let mut e = engine();
        let held = object(&mut e, 0x6000, &[(8, 0x7008)]);
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x20, held);
        e.mem.set_u32(this + 0x14, held);
        e.mem.set_u32(this + 0xc, 0x1234);
        e.call(0x00e7_d1b0, &args![this]);
        assert_eq!(calls(&e, 0x7008).len(), 2);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
        e.mem.set_u32(this + 0x58, held);
        e.call(0x00e7_d4e0, &args![this]);
        assert_eq!(calls(&e, 0x7008).len(), 3);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE).len(), 2);
    }

    #[test]
    fn rtti_slots_answer_their_records() {
        let mut e = engine();
        assert_eq!(e.call(0x00e7_d220, &args![0u32]).u32(), 0x0126_f5a4);
        assert_eq!(e.call(0x00e7_d550, &args![0u32]).u32(), 0x0126_f5e0);
        assert_eq!(e.call(0x00e7_d9c0, &args![0u32]).u32(), 0x0126_f5c4);
    }

    #[test]
    fn deleting_destructors_free_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        for (addr, size) in [
            (0x00e7_d230u32, 0x24u32),
            (0x00e7_d690, 0x5c),
            (0x00e7_dc40, 0x28),
        ] {
            assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
            assert!(calls(&e, FREE_00AA1460).is_empty());
            assert_eq!(e.call(addr, &args![this, 0xffff_ff01u32]).u32(), this);
            assert_eq!(calls(&e, FREE_00AA1460), vec![vec![this, size]]);
            e.call_log = Some(vec![]);
        }
    }

    fn make_texture(e: &mut Engine, level_ok: bool, desc_ok: bool) -> u32 {
        let surface_table = 0x6400;
        let surface = object(e, surface_table, &[(GET_DESC, 0x7430)]);
        e.register_double(0x7430, move |e, a| {
            e.mem.set_u32(a[1], 0x16);
            e.mem.set_u32(a[1] + 0x18, 640);
            e.mem.set_u32(a[1] + 0x1c, 480);
            (if desc_ok { 0u32 } else { 0x8000_0000 }).into_ret()
        });
        e.register_double(0x7048, move |e, a| {
            e.mem.set_u32(a[2], surface);
            (if level_ok { 0u32 } else { 0x8876_0001 }).into_ret()
        });
        object(e, 0x6000, &[(4, 0x7004), (GET_SURFACE_LEVEL, 0x7048)])
    }

    #[test]
    fn fn_00e7d250_makes_the_texture_buffer_data() {
        let mut e = engine();
        let texture = make_texture(&mut e, true, true);
        let group = e.mem.alloc(4);
        let data = e
            .call(0x00e7_d250, &args![texture, group, 2u32, 0x77u32])
            .u32();
        assert_eq!(e.mem.u32(data), NI_DX9_TEXTURE_BUFFER_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x20), texture);
        assert_eq!(calls(&e, 0x7004), vec![vec![texture]]);
        assert_eq!(e.mem.u32(data + 0x10), 2);
        assert_eq!(e.mem.u32(data + 0x1c), 4);
        assert_eq!(e.mem.u32(data + 0x18), 0x77);
        assert_eq!(e.mem.u32(data + 0xc), 0x116);
        assert_eq!(calls(&e, 0x7048), vec![vec![texture, 0, data + 0x14]]);
        assert_eq!(calls(&e, MAKE_GROUP_00A8F2C0), vec![vec![640, 480, data]]);
        assert_eq!(e.mem.u32(group), 0x4444);
        assert_eq!(e.mem.u32(data + 8), 0x4444);
        let data = e
            .call(0x00e7_d250, &args![texture, group, 7u32, 0u32])
            .u32();
        assert_eq!(e.mem.u32(data + 0x1c), 0);
        assert_eq!(calls(&e, ATTACH_00A8F140), vec![vec![0x4444, data]]);
        assert_eq!(e.mem.u32(data + 8), 0x4444);
    }

    #[test]
    fn fn_00e7d250_deletes_the_data_when_the_surface_fails() {
        let mut e = engine();
        e.register(0x7100, |_, _| Ret::default());
        e.put_vtable(NI_DX9_TEXTURE_BUFFER_DATA_VTABLE, &[0x7100]);
        let texture = make_texture(&mut e, false, true);
        let group = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00e7_d250, &args![texture, group, 1u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(calls(&e, ERROR_STRING_00E89D50), vec![vec![0x8876_0001]]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![TEXTURE_BUFFER_FAILED_010EF028, 0x5151]]
        );
        assert_eq!(calls(&e, 0x7100)[0][1], 1);
        let texture = make_texture(&mut e, true, false);
        assert_eq!(
            e.call(0x00e7_d250, &args![texture, group, 1u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(calls(&e, 0x7100).len(), 2);
        assert_eq!(calls(&e, REPORT_00A29680).len(), 1);
        assert!(calls(&e, MAKE_GROUP_00A8F2C0).is_empty());
    }

    #[test]
    fn fn_00e7d940_puts_the_buffer_at_the_list_head() {
        let mut e = engine();
        e.register(NEW_ITEM_0043A010, |e, _| e.mem.alloc(0xc).into_ret());
        let first = e.mem.alloc(0x28);
        e.call(0x00e7_d940, &args![first]);
        assert_eq!(
            e.mem.u32(first),
            NI_DX9_ADDITIONAL_DEPTH_STENCIL_BUFFER_DATA_VTABLE
        );
        let item = e.mem.u32(BUFFER_LIST_0126F5CC);
        assert_eq!(e.mem.u32(item + 8), first);
        assert_eq!(e.mem.u32(item), 0);
        assert_eq!(e.mem.u32(BUFFER_LIST_TAIL_0126F5D0), item);
        assert_eq!(e.mem.u32(BUFFER_LIST_COUNT_0126F5D4), 1);
        assert_eq!(
            calls(&e, NEW_ITEM_0043A010),
            vec![vec![BUFFER_LIST_COUNT_0126F5D4]]
        );
        let second = e.mem.alloc(0x28);
        e.call(0x00e7_d940, &args![second]);
        let head = e.mem.u32(BUFFER_LIST_0126F5CC);
        assert_eq!(e.mem.u32(head), item);
        assert_eq!(e.mem.u32(item + 4), head);
        assert_eq!(e.mem.u32(BUFFER_LIST_TAIL_0126F5D0), item);
        assert_eq!(e.mem.u32(BUFFER_LIST_COUNT_0126F5D4), 2);
        assert_eq!(calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len(), 2);
    }

    fn depth_rig(check_ok: bool, create_ok: bool) -> (Engine, u32, u32) {
        let mut e = engine();
        e.register(FORMAT_OF_00E7BD20, |_, a| (a[0] & 0xff).into_ret());
        e.register(NEW_ITEM_0043A010, |e, _| e.mem.alloc(0xc).into_ret());
        e.register_double(0x7228, move |_, _| {
            (if check_ok { 0u32 } else { 0x8000_0001 }).into_ret()
        });
        let direct3d = object(&mut e, 0x6200, &[(CHECK_DEVICE_FORMAT, 0x7228)]);
        e.mem.set_u32(DIRECT3D_0126F0D8, direct3d);
        e.register(0x70c8, |_, _| Ret::default());
        object(&mut e, 0x6300, &[(0xc8, 0x70c8)]);
        let renderer_block = e.mem.alloc(0x600);
        e.mem.set_u32(renderer_block, 0x6300);
        e.mem.set_u32(renderer_block + 0x5c4, 1);
        e.mem.set_u32(renderer_block + 0x5c8, 2);
        e.mem.set_u32(renderer_block + 0x5d0, 3);
        e.mem.set_u32(SOURCE_011F4748, renderer_block);
        e.mem.set_u32(DISCARD_011BF400, 1);
        let surface = object(&mut e, 0x6400, &[(GET_DESC, 0x7430)]);
        e.register(0x7430, |e, a| {
            e.mem.set_u32(a[1], 0x50);
            0u32.into_ret()
        });
        e.register_double(0x7074, move |e, a| {
            e.mem.set_u32(a[7], surface);
            (if create_ok { 0u32 } else { 0x8000_0002 }).into_ret()
        });
        let device = object(&mut e, 0x6000, &[(CREATE_DEPTH_STENCIL_SURFACE, 0x7074)]);
        e.register(0x7100, |_, _| Ret::default());
        e.put_vtable(
            NI_DX9_ADDITIONAL_DEPTH_STENCIL_BUFFER_DATA_VTABLE,
            &[0x7100],
        );
        let group = e.mem.alloc(0x10);
        e.mem.set_u32(group + 8, 800);
        e.mem.set_u32(group + 0xc, 600);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, group);
        (e, device, holder)
    }

    #[test]
    fn fn_00e7d9d0_makes_an_additional_depth_buffer() {
        let (mut e, device, holder) = depth_rig(true, true);
        assert_eq!(
            e.call(0x00e7_d9d0, &args![device, holder, 0u32, 1u32])
                .u32(),
            0
        );
        assert_eq!(
            e.call(0x00e7_d9d0, &args![device, holder, 0x100u32, 1u32])
                .u32(),
            0
        );
        let data = e
            .call(0x00e7_d9d0, &args![device, holder, 0x14du32, 3u32])
            .u32();
        let direct3d = e.mem.u32(DIRECT3D_0126F0D8);
        assert_eq!(calls(&e, 0x7228), vec![vec![direct3d, 1, 2, 3, 2, 1, 0x4d]]);
        assert_eq!(
            calls(&e, 0x7074),
            vec![vec![device, 800, 600, 0x4d, 8, 0, 1, data + 0x14, 0]]
        );
        assert_eq!(e.mem.u32(data + 0x24), 1);
        assert_eq!(e.mem.u32(data + 0x10), 3);
        assert_eq!(e.mem.u32(data + 0xc), 0x150);
        assert_eq!(e.mem.u32(data + 0x20), 0x4d);
        let group = e.mem.u32(holder);
        assert_eq!(calls(&e, ATTACH_00A8F140), vec![vec![group, data]]);
        assert_eq!(e.mem.u32(data + 8), group);
    }

    #[test]
    fn fn_00e7d9d0_reports_failures() {
        let (mut e, device, holder) = depth_rig(false, true);
        assert_eq!(
            e.call(0x00e7_d9d0, &args![device, holder, 0x14du32, 1u32])
                .u32(),
            0
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![DEPTH_BUFFER_FAILED_010EF1F0, 0x5151]]
        );
        assert!(calls(&e, NEW_BUFFER_OBJECT).is_empty());
        let (mut e, device, holder) = depth_rig(true, false);
        assert_eq!(
            e.call(0x00e7_d9d0, &args![device, holder, 0x14du32, 1u32])
                .u32(),
            0
        );
        assert_eq!(calls(&e, ERROR_STRING_00E89D50), vec![vec![0x8000_0002]]);
        assert_eq!(calls(&e, 0x7100)[0][1], 1);
        assert!(calls(&e, ATTACH_00A8F140).is_empty());
    }

    /// A list of three items holding 10, 20, 30.
    fn list(e: &mut Engine, at: u32) -> [u32; 3] {
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        for (i, item) in items.iter().enumerate() {
            let next = if i < 2 { items[i + 1] } else { 0 };
            let previous = if i > 0 { items[i - 1] } else { 0 };
            e.mem.set_u32(*item, next);
            e.mem.set_u32(item + 4, previous);
            e.mem.set_u32(item + 8, 10 * (i as u32 + 1));
        }
        e.mem.set_u32(at, items[0]);
        e.mem.set_u32(at + 4, items[2]);
        e.mem.set_u32(at + 8, 3);
        items
    }

    #[test]
    fn fn_00e7db70_removes_the_item_holding_the_element() {
        let mut e = engine();
        let at = e.mem.alloc(0xc);
        let items = list(&mut e, at);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 20);
        assert_eq!(e.call(0x00e7_db70, &args![at, element]).u32(), 20);
        assert_eq!(e.mem.u32(items[0]), items[2]);
        assert_eq!(e.mem.u32(at + 8), 2);
        assert_eq!(calls(&e, FREE_ITEM_00E6E8A0), vec![vec![at, items[1]]]);
        e.mem.set_u32(element, 40);
        assert_eq!(e.call(0x00e7_db70, &args![at, element]).u32(), 40);
        assert_eq!(e.mem.u32(at + 8), 2);
    }

    #[test]
    fn fn_00e7dbc0_leaves_the_list_and_goes_down() {
        let mut e = engine();
        let this = e.mem.alloc(0x28);
        let items = list(&mut e, BUFFER_LIST_0126F5CC);
        e.mem.set_u32(items[1] + 8, this);
        e.mem.set_u32(this + 0xc, 0x1234);
        e.call(0x00e7_dbc0, &args![this]);
        assert_eq!(
            calls(&e, FREE_ITEM_00E6E8A0),
            vec![vec![BUFFER_LIST_0126F5CC, items[1]]]
        );
        assert_eq!(e.mem.u32(BUFFER_LIST_COUNT_0126F5D4), 2);
        assert_eq!(calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len(), 1);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x1234]]
        );
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
    }

    #[test]
    fn fn_00e7dc60_initializes_the_element() {
        let mut e = engine();
        e.register(POOL_BLOCK_00E89630, |_, _| 0x3333u32.into_ret());
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xaa; 0x68]);
        assert_eq!(e.call(0x00e7_dc60, &args![this]).u32(), this);
        for offset in [4u32, 0xc, 0x14, 0x18, 0x60] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:x}");
        }
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        assert_eq!(e.mem.u8(this + 0x64), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0x3333);
        assert_eq!(e.mem.u32(this + 8), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_00e7dc90_sets_the_states_when_the_flags_allow() {
        let mut e = engine();
        let flags = e.mem.alloc(4);
        e.register_double(0x70a0, move |e, _| e.mem.u32(flags).into_ret());
        let source = object(&mut e, 0x6000, &[(0xa0, 0x70a0)]);
        e.mem.set_u32(FLAGS_SOURCE_0126F6C4, source);
        e.register(0x70cc, |_, _| Ret::default());
        e.register(0x7068, |_, _| Ret::default());
        let target = object(&mut e, 0x6200, &[(0xcc, 0x70cc), (0x68, 0x7068)]);
        e.mem.set_u32(STATE_TARGET_0126F6C8, target);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x42);
        e.mem.set_u32(flags, 4);
        e.call(0x00e7_dc90, &args![this, 0x100u32]);
        assert!(calls(&e, 0x70a0).is_empty());
        e.mem.set_u32(flags, 0xc);
        e.call(0x00e7_dc90, &args![this, 1u32]);
        e.mem.set_u32(flags, 0);
        e.call(0x00e7_dc90, &args![this, 1u32]);
        assert!(calls(&e, 0x70cc).is_empty());
        e.mem.set_u32(flags, 4);
        e.call(0x00e7_dc90, &args![this, 1u32]);
        assert_eq!(
            calls(&e, 0x70cc),
            vec![vec![target, 0x42, 1, 3, 0], vec![target, 0x42, 2, 3, 0]]
        );
        let registers: Vec<u32> = calls(&e, 0x7068).iter().map(|c| c[1]).collect();
        assert_eq!(
            registers,
            vec![
                0x80, 0xc6, 0x81, 0xc7, 0x82, 0xc8, 0x83, 0xc9, 0x84, 0xca, 0x85, 0xcb, 0x86, 0xcc,
                0x87, 0xcd
            ]
        );
    }

    #[test]
    fn fn_00e7de90_sets_both_states_of_the_pair() {
        let mut e = engine();
        e.mem.set_u32(STATE_PAIRS_0126F0B0 + 0x18, 5);
        e.mem.set_u32(STATE_PAIRS_0126F0B0 + 0x1c, 6);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x99);
        e.call(0x00e7_de90, &args![this, 3u32]);
        assert_eq!(
            calls(&e, SET_SAMPLER_STATE_00E890C0),
            vec![vec![0x99, 1, 5, 0, 0], vec![0x99, 2, 6, 0, 0]]
        );
    }
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    /// Slot doubles of the test objects: each object kind has its own range.
    const THIS_SLOTS: u32 = 0x0e51_0000;
    const DEVICE_SLOTS: u32 = 0x0e52_0000;
    const SURFACE_SLOTS: u32 = 0x0e53_0000;
    const RENDERER_SLOTS: u32 = 0x0e54_0000;
    const DIRECT3D_SLOTS: u32 = 0x0e55_0000;
    const TEXTURE_SLOTS: u32 = 0x0e56_0000;
    const OLD_SLOTS: u32 = 0x0e57_0000;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An object of `size` bytes whose vtable slots point at `slots + slot`, every slot
    /// answering 0 unless a test registers it again.
    fn object(e: &mut Engine, slots: u32, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(vtable + slot, slots + slot);
            e.register(slots + slot, |_, _| Ret::default());
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The surface the device and textures hand out, and its description: format 0x15,
    /// width 0x280, height 0x1e0.
    fn surface(e: &mut Engine) -> u32 {
        let surface = object(e, SURFACE_SLOTS, 8);
        e.register(SURFACE_SLOTS + GET_DESC, |e, a| {
            e.mem.set_u32(a[1], 0x15);
            e.mem.set_u32(a[1] + 0x18, 0x280);
            e.mem.set_u32(a[1] + 0x1c, 0x1e0);
            Ret::default()
        });
        surface
    }

    /// A slot double that stores `value` through its argument `index`.
    fn store_through(e: &mut Engine, address: u32, index: usize, value: u32) {
        e.register_double(address, move |e, a| {
            e.mem.set_u32(a[index], value);
            Ret::default()
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0126_f000, 0x011f_4000, 0x011b_f000] {
            e.map(page, 0x1000);
        }
        for callee in [
            CONVERT_FORMAT_00E7C500,
            GROUP_RESIZE_00A8F080,
            REPORT_00A29680,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            IMPORT_INTERLOCKED_INCREMENT,
            IMPORT_INTERLOCKED_DECREMENT,
            MEMORY_MANAGER_FREE,
            FREE_00AA1460,
            ATTACH_00A8F140,
        ] {
            e.register(callee, |_, a| {
                (a.first().copied().unwrap_or(0) + 1).into_ret()
            });
        }
        e.register(ERROR_STRING_00E89D50, |_, _| 0x0e77_0000u32.into_ret());
        e.register(FORMAT_NAME_00E7A9A0, |_, _| 0x0e78_0000u32.into_ret());
        e.register(MAKE_GROUP_00A8F2C0, |_, _| 0x0e79_0000u32.into_ret());
        e.register(MAKE_DEPTH_GROUP_00A9FF60, |_, _| 0x0e7a_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A buffer data object (0x60 bytes) with a group at +8 (width 0x40, height 0x30).
    fn buffer(e: &mut Engine) -> u32 {
        let this = object(e, THIS_SLOTS, 0x60);
        let group = e.mem.alloc(0x10);
        e.mem.set_u32(group + 8, 0x40);
        e.mem.set_u32(group + 0xc, 0x30);
        e.mem.set_u32(this + 8, group);
        this
    }

    #[test]
    fn fn_00e7ce30_gets_the_depth_surface() {
        let mut e = engine();
        let this = buffer(&mut e);
        let device = object(&mut e, DEVICE_SLOTS, 8);
        let surface = surface(&mut e);
        store_through(&mut e, DEVICE_SLOTS + GET_DEPTH_STENCIL_SURFACE, 1, surface);
        e.mem.set_u32(this + 0x14, 0x5555);
        assert!(e.call(0x00e7_ce30, &args![this, device]).bool());
        // The old surface goes through the object's own slot +0x54 first.
        assert_eq!(calls(&e, THIS_SLOTS + 0x54), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0x14), surface);
        assert_eq!(e.mem.u32(this + 0xc), 0x16);
        let group = e.mem.u32(this + 8);
        assert_eq!(
            calls(&e, GROUP_RESIZE_00A8F080),
            vec![vec![group, 0x280, 0x1e0]]
        );
    }

    #[test]
    fn fn_00e7ce30_fails_without_a_surface_or_description() {
        let mut e = engine();
        let this = buffer(&mut e);
        let device = object(&mut e, DEVICE_SLOTS, 8);
        e.register(DEVICE_SLOTS + GET_DEPTH_STENCIL_SURFACE, |_, _| {
            0x8000_0000u32.into_ret()
        });
        assert!(!e.call(0x00e7_ce30, &args![this, device]).bool());
        assert!(calls(&e, THIS_SLOTS + 0x54).is_empty());
        // A surface whose description fails is released and cleared.
        let surface = object(&mut e, SURFACE_SLOTS, 8);
        e.register(SURFACE_SLOTS + GET_DESC, |_, _| 0x8000_0000u32.into_ret());
        store_through(&mut e, DEVICE_SLOTS + GET_DEPTH_STENCIL_SURFACE, 1, surface);
        assert!(!e.call(0x00e7_ce30, &args![this, device]).bool());
        assert_eq!(calls(&e, SURFACE_SLOTS + 8), vec![vec![surface]]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn fn_00e7cf00_recreates_every_listed_buffer() {
        let mut e = engine();
        let first = object(&mut e, THIS_SLOTS, 8);
        let second = object(&mut e, OLD_SLOTS, 8);
        let items = e.mem.alloc(0x18);
        e.mem.set_u32(items, items + 0xc);
        e.mem.set_u32(items + 8, first);
        e.mem.set_u32(items + 0xc, 0);
        e.mem.set_u32(items + 0x14, second);
        e.mem.set_u32(BUFFER_LIST_0126F5CC, items);
        e.call(0x00e7_cf00, &args![0x0e60_0000u32]);
        assert_eq!(calls(&e, THIS_SLOTS + 0x58), vec![vec![first, 0x0e60_0000]]);
        assert_eq!(calls(&e, OLD_SLOTS + 0x58), vec![vec![second, 0x0e60_0000]]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, IMPORT_ENTER_CRITICAL_SECTION);
        assert_eq!(log.last().unwrap().0, IMPORT_LEAVE_CRITICAL_SECTION);
    }

    /// The renderer at `011f4748` (adapter 1, device type 2, adapter format 0x16) and the
    /// Direct3D object at `0126f0d8`, whose format check answers `check`.
    fn depth_rig(e: &mut Engine, check: u32) -> (u32, u32) {
        let renderer = object(e, RENDERER_SLOTS, 0x5d4);
        e.mem.set_u32(renderer + 0x5c4, 1);
        e.mem.set_u32(renderer + 0x5c8, 2);
        e.mem.set_u32(renderer + 0x5d0, 0x16);
        e.mem.set_u32(SOURCE_011F4748, renderer);
        let direct3d = object(e, DIRECT3D_SLOTS, 8);
        e.register_double(DIRECT3D_SLOTS + CHECK_DEVICE_FORMAT, move |_, _| {
            check.into_ret()
        });
        e.mem.set_u32(DIRECT3D_0126F0D8, direct3d);
        (renderer, direct3d)
    }

    #[test]
    fn fn_00e7cf50_needs_a_format() {
        let mut e = engine();
        let this = buffer(&mut e);
        e.mem.set_u32(this + 0x14, 0x5555);
        assert!(!e.call(0x00e7_cf50, &args![this, 0x0e60_0000u32]).bool());
        assert_eq!(calls(&e, THIS_SLOTS + 0x54), vec![vec![this]]);
        assert!(calls(&e, REPORT_00A29680).is_empty());
    }

    #[test]
    fn fn_00e7cf50_recreates_the_depth_surface() {
        let mut e = engine();
        let (renderer, direct3d) = depth_rig(&mut e, 0);
        let this = buffer(&mut e);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x20, 0x4b);
        e.mem.set_u32(this + 0x24, 1);
        let device = object(&mut e, DEVICE_SLOTS, 8);
        let surface = surface(&mut e);
        store_through(
            &mut e,
            DEVICE_SLOTS + CREATE_DEPTH_STENCIL_SURFACE,
            7,
            surface,
        );
        assert!(e.call(0x00e7_cf50, &args![this, device]).bool());
        assert_eq!(calls(&e, RENDERER_SLOTS + 0xc8), vec![vec![renderer]]);
        assert_eq!(
            calls(&e, DIRECT3D_SLOTS + CHECK_DEVICE_FORMAT),
            vec![vec![direct3d, 1, 2, 0x16, 2, 1, 0x4b]]
        );
        assert_eq!(
            calls(&e, DEVICE_SLOTS + CREATE_DEPTH_STENCIL_SURFACE),
            vec![vec![device, 0x40, 0x30, 0x4b, 4, 0, 1, this + 0x14, 0]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0x16);
    }

    #[test]
    fn fn_00e7cf72_reports_failures() {
        let mut e = engine();
        depth_rig(&mut e, 0x8000_0001);
        let this = buffer(&mut e);
        e.mem.set_u32(this + 0x20, 0x4b);
        let device = object(&mut e, DEVICE_SLOTS, 8);
        assert!(!e.call(0x00e7_cf72, &args![this, device]).bool());
        assert_eq!(calls(&e, ERROR_STRING_00E89D50), vec![vec![0x8000_0001]]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![RECREATE_FAILED_010EEF40, 0x0e77_0000]]
        );
        assert!(calls(&e, DEVICE_SLOTS + CREATE_DEPTH_STENCIL_SURFACE).is_empty());
        // The surface creation failing is reported too.
        let mut e = engine();
        depth_rig(&mut e, 0);
        let this = buffer(&mut e);
        e.mem.set_u32(this + 0x20, 0x4b);
        let device = object(&mut e, DEVICE_SLOTS, 8);
        e.register(DEVICE_SLOTS + CREATE_DEPTH_STENCIL_SURFACE, |_, _| {
            0x8000_0002u32.into_ret()
        });
        assert!(!e.call(0x00e7_cf72, &args![this, device]).bool());
        assert_eq!(calls(&e, ERROR_STRING_00E89D50), vec![vec![0x8000_0002]]);
    }

    #[test]
    fn fn_00e7d0a0_takes_the_new_device_and_its_render_target() {
        let mut e = engine();
        let this = buffer(&mut e);
        let old = object(&mut e, OLD_SLOTS, 8);
        e.mem.set_u32(this + 0x58, old);
        e.mem.set_u32(RENDER_TARGETS_0126F580, 9);
        e.mem.set_u32(DEPTH_SURFACE_0126F590, 9);
        let device = object(&mut e, DEVICE_SLOTS, 8);
        let surface = surface(&mut e);
        store_through(&mut e, DEVICE_SLOTS + GET_RENDER_TARGET, 2, surface);
        assert!(e.call(0x00e7_d0a0, &args![this, device]).bool());
        assert_eq!(e.mem.u32(RENDER_TARGETS_0126F580), 0);
        assert_eq!(e.mem.u32(DEPTH_SURFACE_0126F590), 0);
        assert_eq!(calls(&e, OLD_SLOTS + 8), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0x58), device);
        assert_eq!(calls(&e, DEVICE_SLOTS + 4), vec![vec![device]]);
        assert_eq!(
            calls(&e, DEVICE_SLOTS + GET_RENDER_TARGET),
            vec![vec![device, 0, this + 0x14]]
        );
        let group = e.mem.u32(this + 8);
        assert_eq!(
            calls(&e, GROUP_RESIZE_00A8F080),
            vec![vec![group, 0x280, 0x1e0]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0x16);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![RECREATE_USING_FORMAT_010EEF7C, 0x0e78_0000]]
        );
        // A failing render target: false, nothing reported.
        e.register(DEVICE_SLOTS + GET_RENDER_TARGET, |_, _| {
            0x8000_0000u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e7_d0a0, &args![this, device]).bool());
        assert!(calls(&e, REPORT_00A29680).is_empty());
    }

    #[test]
    fn fn_00e7d390_makes_a_cube_face_buffer() {
        let mut e = engine();
        let block = e.mem.alloc(0x24);
        e.register_double(NEW_BUFFER_OBJECT, move |_, _| block.into_ret());
        let texture = object(&mut e, TEXTURE_SLOTS, 8);
        let surface = surface(&mut e);
        store_through(&mut e, TEXTURE_SLOTS + GET_CUBE_MAP_SURFACE, 3, surface);
        let group_out = e.mem.alloc(4);
        let made = e
            .call(0x00e7_d390, &args![texture, 3u32, 7u32, 8u32, group_out])
            .u32();
        assert_eq!(made, block);
        assert_eq!(e.mem.u32(block), NI_DX9_TEXTURE_BUFFER_DATA_VTABLE);
        assert_eq!(e.mem.u32(block + 0x20), texture);
        assert_eq!(calls(&e, TEXTURE_SLOTS + 4), vec![vec![texture]]);
        assert_eq!(
            calls(&e, TEXTURE_SLOTS + GET_CUBE_MAP_SURFACE),
            vec![vec![texture, 3, 0, block + 0x14]]
        );
        assert_eq!(e.mem.u32(block + 0xc), 0x16);
        assert_eq!(
            calls(&e, MAKE_GROUP_00A8F2C0),
            vec![vec![0x280, 0x1e0, block]]
        );
        assert_eq!(e.mem.u32(group_out), 0x0e79_0000);
        assert_eq!(e.mem.u32(block + 8), 0x0e79_0000);
        // With a group already: attached to it.
        e.call_log = Some(vec![]);
        e.mem.set_u32(group_out, 0x0e7b_0000);
        e.call(0x00e7_d390, &args![texture, 0u32, 0u32, 0u32, group_out]);
        assert_eq!(calls(&e, ATTACH_00A8F140), vec![vec![0x0e7b_0000, block]]);
        assert_eq!(e.mem.u32(block + 8), 0x0e7b_0000);
    }

    #[test]
    fn fn_00e7d390_reports_a_failed_face() {
        let mut e = engine();
        let block = object(&mut e, THIS_SLOTS, 0x24);
        e.register_double(NEW_BUFFER_OBJECT, move |_, _| block.into_ret());
        let texture = object(&mut e, TEXTURE_SLOTS, 8);
        e.register(TEXTURE_SLOTS + GET_CUBE_MAP_SURFACE, |_, _| {
            0x8000_0000u32.into_ret()
        });
        let group_out = e.mem.alloc(4);
        // The made object holds the class's table by then: the delete goes through its slot 0.
        e.map(0x010e_e000, 0x1000);
        e.mem.set_u32(NI_DX9_TEXTURE_BUFFER_DATA_VTABLE, THIS_SLOTS);
        let made = e
            .call(0x00e7_d390, &args![texture, 1u32, 0u32, 0u32, group_out])
            .u32();
        assert_eq!(made, 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![TEXTURE_BUFFER_FAILED_010EF028, 0x0e77_0000]]
        );
        assert_eq!(calls(&e, THIS_SLOTS), vec![vec![block, 1]]);
        assert_eq!(e.mem.u32(block + 0x14), 0);
    }

    #[test]
    fn fn_00e7d490_releases_surface_block_and_texture() {
        let mut e = engine();
        let this = e.mem.alloc(0x24);
        let surface = object(&mut e, SURFACE_SLOTS, 8);
        let texture = object(&mut e, TEXTURE_SLOTS, 8);
        e.mem.set_u32(this + 0x14, surface);
        e.mem.set_u32(this + 0xc, 0x0e10_0000);
        e.mem.set_u32(this + 0x20, texture);
        e.call(0x00e7_d490, &args![this]);
        assert_eq!(calls(&e, SURFACE_SLOTS + 8), vec![vec![surface]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x0e10_0000]]
        );
        assert_eq!(calls(&e, TEXTURE_SLOTS + 8), vec![vec![texture]]);
        for offset in [0xcu32, 0x14, 0x20] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn fn_00e7d560_releases_surface_block_and_device() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        let device = object(&mut e, DEVICE_SLOTS, 8);
        e.mem.set_u32(this + 0xc, 0x0e10_0000);
        e.mem.set_u32(this + 0x58, device);
        e.call(0x00e7_d560, &args![this]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, 0x0e10_0000]]
        );
        assert_eq!(calls(&e, DEVICE_SLOTS + 8), vec![vec![device]]);
        assert_eq!(e.mem.u32(this + 0x58), 0);
    }

    #[test]
    fn fn_00e7d620_destroys_and_frees_with_bit_zero() {
        let mut e = engine();
        let this = e.mem.alloc(0x20);
        let surface = object(&mut e, SURFACE_SLOTS, 8);
        e.mem.set_u32(this + 0x14, surface);
        assert_eq!(e.call(0x00e7_d620, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(calls(&e, SURFACE_SLOTS + 8), vec![vec![surface]]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
        assert_eq!(calls(&e, FREE_00AA1460), vec![vec![this, 0x20]]);
        e.call_log = Some(vec![]);
        e.call(0x00e7_d620, &args![this, 0u32]);
        assert!(calls(&e, FREE_00AA1460).is_empty());
    }

    #[test]
    fn fn_00e7d680_answers_its_record() {
        let mut e = engine();
        assert_eq!(e.call(0x00e7_d680, &args![0u32]).u32(), 0x0126_f5b4);
    }

    #[test]
    fn fn_00e7d850_makes_the_implicit_depth_buffer() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        e.register_double(NEW_BUFFER_OBJECT, move |_, _| block.into_ret());
        let device = object(&mut e, DEVICE_SLOTS, 8);
        let surface = surface(&mut e);
        store_through(&mut e, DEVICE_SLOTS + GET_DEPTH_STENCIL_SURFACE, 1, surface);
        let group_out = e.mem.alloc(4);
        assert_eq!(e.call(0x00e7_d850, &args![device, group_out]).u32(), block);
        assert_eq!(
            e.mem.u32(block),
            NI_DX9_IMPLICIT_DEPTH_STENCIL_BUFFER_DATA_VTABLE
        );
        assert_eq!(e.mem.u32(block + 0xc), 0x16);
        assert_eq!(
            calls(&e, MAKE_DEPTH_GROUP_00A9FF60),
            vec![vec![0x280, 0x1e0, block]]
        );
        assert_eq!(e.mem.u32(group_out), 0x0e7a_0000);
        assert_eq!(e.mem.u32(block + 8), 0x0e7a_0000);
        assert_eq!(e.mem.u32(DEPTH_SURFACE_0126F590), surface);
        // A failed surface with a failed allocation: nothing to delete, null.
        let mut e = engine();
        e.register(NEW_BUFFER_OBJECT, |_, _| Ret::default());
        let device = object(&mut e, DEVICE_SLOTS, 8);
        e.register(DEVICE_SLOTS + GET_DEPTH_STENCIL_SURFACE, |_, _| {
            0x8000_0000u32.into_ret()
        });
        assert_eq!(e.call(0x00e7_d850, &args![device, group_out]).u32(), 0);
        assert_eq!(
            calls(&e, DEVICE_SLOTS + GET_DEPTH_STENCIL_SURFACE),
            vec![vec![device, 0x14]]
        );
    }

    #[test]
    fn fn_00e7dd50_resets_the_stages_up_to_both_limits() {
        let mut e = engine();
        let target = object(&mut e, DEVICE_SLOTS, 8);
        e.mem.set_u32(STATE_TARGET_0126F6C8, target);
        e.mem.set_u32(STAGE_LIMIT_011BF558, 3);
        e.mem.set_u32(STAGE_LIMIT_011BF560, 5);
        e.call(0x00e7_dd50, &args![2u32]);
        assert_eq!(
            calls(&e, DEVICE_SLOTS + 0xdc),
            vec![vec![target, 2, 0], vec![target, 3, 0], vec![target, 4, 0]]
        );
        assert_eq!(
            calls(&e, DEVICE_SLOTS + 0xc0),
            vec![
                vec![target, 2, 1, 1, 0],
                vec![target, 2, 4, 1, 0],
                vec![target, 2, 0x18, 0, 0]
            ]
        );
    }

    #[test]
    fn fn_00e7ddf0_answers_the_state_or_the_stage_word() {
        let mut e = engine();
        e.register(STATE_LOOKUP_00E89060, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            assert_eq!(e.mem.u8(a[3]), 0);
            e.mem.set_u32(a[2], 0x77);
            e.mem.u32(a[0]).into_ret()
        });
        let state = e.mem.alloc(4);
        e.mem.set_u32(state, 1);
        let stage = e.mem.alloc(0x14);
        e.mem.set_u32(stage + 4, 0x44);
        e.mem.set_u32(stage + 0x10, state);
        assert_eq!(e.call(0x00e7_ddf0, &args![stage]).u32(), 0x77);
        assert_eq!(calls(&e, STATE_LOOKUP_00E89060)[0][..2], [state, 0xb]);
        e.mem.set_u32(state, 0x100);
        assert_eq!(e.call(0x00e7_ddf0, &args![stage]).u32(), 0x44);
    }
}
