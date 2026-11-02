//! `NiXenonRenderer/run_00e75a70` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiXenonRenderer::CreateDirect3D9` (engine map, `00e69460`; `cdecl`, one word): -1 when
/// Direct3D cannot be created.
const CREATE_DIRECT3D9: u32 = 0x00e6_9460;
/// `_strncpy_s` (`00ec8d5f`, `cdecl`: destination, size, source, count), the error text buffer
/// it fills (`011f4648`, 0x100 bytes) and the message copied into it
/// (`"Creation failed: Could not initialize DirectX9"`, `010ee6c8`).
const STRNCPY_S: u32 = 0x00ec_8d5f;
const ERROR_MESSAGE_011F4648: u32 = 0x011f_4648;
const CREATION_FAILED_010EE6C8: u32 = 0x010e_e6c8;
/// `00a29680` (unnamed; `cdecl`, a message) and the message passed to it
/// (`"NiDX9Renderer::Create> Invalid device ID specified - FAILING\n"`, `010ee688`).
const REPORT_00A29680: u32 = 0x00a2_9680;
const INVALID_DEVICE_010EE688: u32 = 0x010e_e688;
/// The allocation `00aa13e0` (`cdecl`, size), the renderer's size and its constructor
/// (`00e754a0`, unnamed; `thiscall`, no stack words).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const RENDERER_SIZE: u32 = 0xb80;
const RENDERER_CONSTRUCT: u32 = 0x00e7_54a0;
/// The renderer's initialisation (`00e72e60`, `nixenonrenderer.cpp`; `thiscall` with fourteen
/// words): a nonzero byte on success.
const RENDERER_INITIALIZE: u32 = 0x00e7_2e60;
/// `00a63520` (`nistream.cpp`; `cdecl`, a class name), `NiStream::RegisterLoader` (engine map,
/// `00a64900`; `cdecl`: class name, loader) and what they are given: the class name
/// `"NiPersistentSrcTextureRendererData"` (`0109f0a0`) and the loader at `00e916d0`.
const STREAM_00A63520: u32 = 0x00a6_3520;
const NI_STREAM_REGISTER_LOADER: u32 = 0x00a6_4900;
const PERSISTENT_SRC_TEXTURE_RENDERER_DATA: u32 = 0x0109_f0a0;
const PERSISTENT_SRC_TEXTURE_LOADER: u32 = 0x00e9_16d0;

// Translated from 00e76210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (its own message calls it `NiDX9Renderer::Create`), `cdecl` with
/// 15 words: the first 14 go to the renderer's initialisation, the 15th to
/// `NiXenonRenderer::CreateDirect3D9`. When that answers -1 the error text gets "Creation
/// failed: Could not initialize DirectX9" (`_strncpy_s`, at most 0xff characters), `00a29680`
/// reports the invalid device ID, and null is answered. Otherwise a 0xb80-byte renderer is
/// made (`00e754a0`; null when the allocation fails, and `00e72e60` still runs on it) and
/// initialised with the 14 words; a failed initialisation deletes a made renderer (its
/// virtual slot 0 with 1) and answers null. Success registers
/// `NiPersistentSrcTextureRendererData` (`00a63520`, then `NiStream::RegisterLoader` with
/// `00e916d0`) and answers the renderer.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e76210(
    e: &mut Engine,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    eighth: u32,
    ninth: u32,
    tenth: u32,
    eleventh: u32,
    twelfth: u32,
    thirteenth: u32,
    fourteenth: u32,
    device: u32,
) -> u32 {
    if e.call(CREATE_DIRECT3D9, &args![device]).i32() == -1 {
        e.call(
            STRNCPY_S,
            &args![
                ERROR_MESSAGE_011F4648,
                0x100u32,
                CREATION_FAILED_010EE6C8,
                0xffu32
            ],
        );
        e.call(REPORT_00A29680, &args![INVALID_DEVICE_010EE688]);
        return 0;
    }
    let block = e.call(NEW_OBJECT, &args![RENDERER_SIZE]).u32();
    let renderer = if block != 0 {
        e.call(RENDERER_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let initialized = e
        .call(
            RENDERER_INITIALIZE,
            &args![
                renderer, first, second, third, fourth, fifth, sixth, seventh, eighth, ninth,
                tenth, eleventh, twelfth, thirteenth, fourteenth
            ],
        )
        .u8();
    if initialized == 0 {
        if renderer != 0 {
            e.vcall(renderer, 0, &args![1u32]);
        }
        return 0;
    }
    e.call(
        STREAM_00A63520,
        &args![PERSISTENT_SRC_TEXTURE_RENDERER_DATA],
    );
    e.call(
        NI_STREAM_REGISTER_LOADER,
        &args![
            PERSISTENT_SRC_TEXTURE_RENDERER_DATA,
            PERSISTENT_SRC_TEXTURE_LOADER
        ],
    );
    renderer
}

/// The registry entry of [`fn_00e76210`]: its 15 words.
fn fn_00e76210_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let w = |i: usize| words[i];
    fn_00e76210(
        e,
        w(0),
        w(1),
        w(2),
        w(3),
        w(4),
        w(5),
        w(6),
        w(7),
        w(8),
        w(9),
        w(10),
        w(11),
        w(12),
        w(13),
        w(14),
    )
    .into_ret()
}

/// `NiDX9Renderer`'s virtual table (RTTI `NiDX9Renderer`).
const DX9_RENDERER_VTABLE: u32 = 0x010e_e4bc;
/// The texture stage count (`011bf558`) the shader library's slot 0xdc is cleared for.
const STAGE_COUNT_011BF558: u32 = 0x011b_f558;
/// The memory manager (`011f6238`) and its free (`00aa4060`, `thiscall`, one word); the sized
/// free (`00aa1460`, cdecl: block and size); the array free (`00aa10f0`, cdecl, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const FREE_SIZED: u32 = 0x00aa_1460;
const FREE_ARRAY: u32 = 0x00aa_10f0;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The free list of 0x10-byte blocks (`0126f0ec`, next at +0xc) the renderer's list at +0x620
/// goes back to.
const BLOCK_FREE_LIST: u32 = 0x0126_f0ec;
/// The 0x100 `NiPointer`s from `0126f108` the destructor lets go of.
const POINTERS_0126F108: u32 = 0x0126_f108;
/// Callees of the destructor: `00b57ea0` (`thiscall` on the map at +0x610: key, next entry),
/// `00e91fd0` (`thiscall` on the same map: key), `00a8f870` and `00e819f0`, `00e7cec0`,
/// `00a98460` (no arguments), `NiXenonRenderer::PurgeAllD3DShaders` (`00e75380`, `thiscall`),
/// the map clean-up `00b601a0` (`thiscall`), the map iteration `00e6e630` (`thiscall`: the
/// position, key and value slots), `00e6e730` and `00e6e7b0` (`thiscall`), the light manager's
/// destructor body `00e8b630`, the texture data clean-up `00e8a9b0` (cdecl, one word), the
/// class name removal `00a63520` (cdecl) with `"NiPersistentSrcTextureRendererData"`, the two
/// lists' `RemoveAll` (`00b71300`, `00e74d40`, `thiscall`) and the `NiRenderer` base destructor
/// body (`00a62b60`, `thiscall`), which the exe jumps to.
const MAP_DROP_CHAINED_00B57EA0: u32 = 0x00b5_7ea0;
const MAP_DROP_00E91FD0: u32 = 0x00e9_1fd0;
const STEP_00A8F870: u32 = 0x00a8_f870;
const PURGE_ALL_D3D_SHADERS: u32 = 0x00e7_5380;
const STEP_00E819F0: u32 = 0x00e8_19f0;
const STEP_00E7CEC0: u32 = 0x00e7_cec0;
const MAP_CLEAN_UP: u32 = 0x00b6_01a0;
const TEXTURE_DATA_CLEAN_UP_00E8A9B0: u32 = 0x00e8_a9b0;
const MAP_ITERATE_00E6E630: u32 = 0x00e6_e630;
const LIGHT_MANAGER_DESTRUCTOR_00E8B630: u32 = 0x00e8_b630;
const STEP_00E6E730: u32 = 0x00e6_e730;
const STEP_00E6E7B0: u32 = 0x00e6_e7b0;
const STEP_00A98460: u32 = 0x00a9_8460;
const STRING_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
const MODIFIER_LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
const RENDERER_BASE_DESTRUCTOR: u32 = 0x00a6_2b60;

/// `NiPointer<T>` let go: the object's count at +4 goes down (`InterlockedDecrement`),
/// deleting through virtual slot 4 at 0.
fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let left = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
}

/// The first item of a hash map (virtual table +0, bucket count +4, buckets +8): the first
/// set bucket's head, or 0.
fn map_first(e: &Engine, map: u32) -> u32 {
    let count = e.mem.u32(map.wrapping_add(4));
    let buckets = e.mem.u32(map.wrapping_add(8));
    let mut index = 0u32;
    while index < count {
        let head = e.mem.u32(buckets.wrapping_add(index * 4));
        if head != 0 {
            return head;
        }
        index += 1;
    }
    0
}

/// The item after `item` (next +0, key +4): its next, else the head of the first set bucket
/// after the key's bucket (the map's virtual slot 4 hashes the key), else 0.
fn map_next(e: &mut Engine, map: u32, item: u32) -> u32 {
    let next = e.mem.u32(item);
    if next != 0 {
        return next;
    }
    let key = e.mem.u32(item.wrapping_add(4));
    let mut index = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
    let count = e.mem.u32(map.wrapping_add(4));
    if index < count {
        let buckets = e.mem.u32(map.wrapping_add(8));
        while index < count {
            let head = e.mem.u32(buckets.wrapping_add(index * 4));
            if head != 0 {
                return head;
            }
            index += 1;
        }
    }
    0
}

/// The map's two virtual tables in turn, each followed by the clean-up `00b601a0`, then its
/// bucket array (+8) freed.
fn destroy_map(e: &mut Engine, map: u32, own_vtable: u32, base_vtable: u32) {
    e.mem.set_u32(map, own_vtable);
    e.call(MAP_CLEAN_UP, &args![map]);
    e.mem.set_u32(map, base_vtable);
    e.call(MAP_CLEAN_UP, &args![map]);
    let buckets = e.mem.u32(map.wrapping_add(8));
    e.call(FREE_ARRAY, &args![buckets]);
}

/// An object deleted through its virtual slot 0 with 1, when there is one.
fn delete_object(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

// Translated from 00e75a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e75a70`): the `NiDX9Renderer` destructor body
/// (`thiscall`, no stack words; the virtual table it stores has that RTTI).
///
/// With a device (+0x288): the shader library (+0x8b8), when set, gets slot 0xdc with each
/// texture stage below `011bf558` and 0; the device's stream sources below +0x6e8 are cleared
/// (slot 0x190: stream, 0, 0, 0) and its indices (slot 0x1a0 with 0). The 4 x 0x16 blocks
/// from +0x700 go back to the memory manager and the four `NiPointer`s from +0x870 are let go
/// and cleared. The block list at +0x620 moves onto the free list `0126f0ec` (+0x620, +0x624
/// cleared), whose blocks are then all freed. Each entry of the map at +0x610 has its chain
/// (next at +0x20) dropped (`00b57ea0` with the next one, `00e91fd0` for the last) and freed.
/// Then `00a8f870`, `PurgeAllD3DShaders`, `00e819f0`, `00e7cec0`, the clean-up of the maps
/// at +0x8cc, +0x8dc, +0x8ec, `00e8a9b0`; each value of the map at +0x890 has its parts (slots
/// 0xa4 count, 0xb0 part, 0xb4 last part) let go of their +0x10 and is let go itself; the
/// map's clean-up; the values of the map at +0xb14 are freed. +0x8a8 cleared; the objects at
/// +0x8bc, +0x8c0, +0x8ac (after its slot 8 with +0x8b0 and +0x8b4), +0x8b8, +0x8c4 deleted
/// and cleared, the light manager (+0x8c8) destroyed and freed; the 0x100 `NiPointer`s from
/// `0126f108` let go and cleared; the device released and cleared; the blocks at +0xa80,
/// +0xa84, +0xa88 freed and the array +0xa90; `00e6e730`, `00e6e7b0`, the class name removed
/// (`00a63520`), `00a98460`; then the members' destructors: the map at +0xb14, the arrays at
/// +0xb04, +0xaf4, +0xae4, +0xad4, the lists at +0x90c and +0x900, +0x8fc let go, the maps at
/// +0x8ec, +0x8dc, +0x8cc, +0x890, the pointers +0x87c down to +0x870 and +0x630 let go, the
/// map at +0x610, and the `NiRenderer` base destructor `00a62b60`.
pub fn fn_00e75a70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let device = e.mem.u32(base + 0x288);
    e.mem.set_u32(base, DX9_RENDERER_VTABLE);
    if device != 0 {
        if e.mem.u32(base + 0x8b8) != 0 {
            let mut stage = 0u32;
            while stage < e.mem.u32(STAGE_COUNT_011BF558) {
                let library = e.mem.u32(base + 0x8b8);
                e.vcall(library, 0xdc, &args![stage, 0u32]);
                stage += 1;
            }
        }
        let mut stream = 0u32;
        while stream < e.mem.u32(base + 0x6e8) {
            let device = e.mem.u32(base + 0x288);
            e.vcall(device, 0x190, &args![stream, 0u32, 0u32, 0u32]);
            stream += 1;
        }
        let device = e.mem.u32(base + 0x288);
        e.vcall(device, 0x1a0, &args![0u32]);
    }

    let mut block = base + 0x700;
    for group in 0..4u32 {
        for _ in 0..0x16 {
            let held = e.mem.u32(block);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, held]);
            block += 4;
        }
        let slot = base + 0x870 + group * 4;
        let pointer = e.mem.u32(slot);
        if pointer != 0 {
            release(e, pointer);
            e.mem.set_u32(slot, 0);
        }
    }

    let mut item = e.mem.u32(base + 0x620);
    while item != 0 {
        let next = e.mem.u32(item + 0xc);
        let free = e.mem.u32(BLOCK_FREE_LIST);
        e.mem.set_u32(BLOCK_FREE_LIST, item);
        e.mem.set_u32(item + 0xc, free);
        item = next;
    }
    e.mem.set_u32(base + 0x620, 0);
    e.mem.set_u32(base + 0x624, 0);
    let mut free = e.mem.u32(BLOCK_FREE_LIST);
    while free != 0 {
        let next = e.mem.u32(free + 0xc);
        e.call(FREE_SIZED, &args![free, 0x10u32]);
        free = next;
    }
    e.mem.set_u32(BLOCK_FREE_LIST, 0);

    let map = base + 0x610;
    let mut item = map_first(e, map);
    while item != 0 {
        let key = e.mem.u32(item + 4);
        let mut chain = e.mem.u32(item + 8);
        item = map_next(e, map, item);
        while chain != 0 {
            let following = e.mem.u32(chain + 0x20);
            if following != 0 {
                e.call(MAP_DROP_CHAINED_00B57EA0, &args![map, key, following]);
            } else {
                e.call(MAP_DROP_00E91FD0, &args![map, key]);
            }
            e.call(FREE_SIZED, &args![chain, 0x24u32]);
            chain = following;
        }
    }

    e.call(STEP_00A8F870, &args![]);
    e.call(PURGE_ALL_D3D_SHADERS, &args![this]);
    e.call(STEP_00E819F0, &args![]);
    e.call(STEP_00E7CEC0, &args![]);
    for offset in [0x8ccu32, 0x8dc, 0x8ec] {
        e.call(MAP_CLEAN_UP, &args![base + offset]);
    }
    e.call(TEXTURE_DATA_CLEAN_UP_00E8A9B0, &args![this]);

    // The map at +0x890, walked by `00e6e630` through the position, key and value slots.
    let start = map_first(e, base + 0x890);
    e.with_stack(12, |e, slots| {
        let value_slot = slots.addr();
        let position = value_slot + 4;
        let key_slot = value_slot + 8;
        e.mem.set_u32(position, start);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(value_slot, 0);
            e.call(
                MAP_ITERATE_00E6E630,
                &args![base + 0x890, position, key_slot, value_slot],
            );
            let value = e.mem.u32(value_slot);
            if value == 0 {
                continue;
            }
            // The value slot doubles as the part counter.
            e.mem.set_u32(value_slot, 0);
            let mut index = 0u32;
            if e.vcall(value, 0xa4, &args![]).u32() != 0 {
                loop {
                    let part = e.vcall(value, 0xb0, &args![index]).u32();
                    if part != 0 {
                        let held = e.mem.u32(part + 0x10);
                        if held != 0 {
                            release(e, held);
                            e.mem.set_u32(part + 0x10, 0);
                        }
                    }
                    index = e.mem.u32(value_slot).wrapping_add(1);
                    e.mem.set_u32(value_slot, index);
                    if index >= e.vcall(value, 0xa4, &args![]).u32() {
                        break;
                    }
                }
            }
            let last = e.vcall(value, 0xb4, &args![]).u32();
            if last != 0 {
                let held = e.mem.u32(last + 0x10);
                if held != 0 {
                    release(e, held);
                    e.mem.set_u32(last + 0x10, 0);
                }
            }
            release(e, value);
        }
    });
    e.call(MAP_CLEAN_UP, &args![base + 0x890]);

    let map = base + 0xb14;
    let mut item = map_first(e, map);
    while item != 0 {
        let value = e.mem.u32(item + 8);
        item = map_next(e, map, item);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, value]);
    }

    e.mem.set_u32(base + 0x8a8, 0);
    let object = e.mem.u32(base + 0x8bc);
    delete_object(e, object);
    e.mem.set_u32(base + 0x8bc, 0);
    let object = e.mem.u32(base + 0x8c0);
    if object != 0 {
        delete_object(e, object);
        e.mem.set_u32(base + 0x8c0, 0);
    }
    let object = e.mem.u32(base + 0x8ac);
    if object != 0 {
        let first = e.mem.u32(base + 0x8b0);
        e.vcall(object, 8, &args![first]);
        let object = e.mem.u32(base + 0x8ac);
        let second = e.mem.u32(base + 0x8b4);
        e.vcall(object, 8, &args![second]);
        let object = e.mem.u32(base + 0x8ac);
        delete_object(e, object);
        e.mem.set_u32(base + 0x8ac, 0);
    }
    let object = e.mem.u32(base + 0x8b8);
    delete_object(e, object);
    e.mem.set_u32(base + 0x8b8, 0);
    let object = e.mem.u32(base + 0x8c4);
    delete_object(e, object);
    e.mem.set_u32(base + 0x8c4, 0);
    let lights = e.mem.u32(base + 0x8c8);
    if lights != 0 {
        e.call(LIGHT_MANAGER_DESTRUCTOR_00E8B630, &args![lights]);
        e.call(FREE_SIZED, &args![lights, 0x23cu32]);
    }
    e.mem.set_u32(base + 0x8c8, 0);
    for index in 0..0x100u32 {
        let slot = POINTERS_0126F108 + index * 4;
        let pointer = e.mem.u32(slot);
        if pointer != 0 {
            release(e, pointer);
            e.mem.set_u32(slot, 0);
        }
    }
    let device = e.mem.u32(base + 0x288);
    if device != 0 {
        e.vcall(device, 8, &args![]);
        e.mem.set_u32(base + 0x288, 0);
    }
    for offset in [0xa80u32, 0xa84, 0xa88] {
        let held = e.mem.u32(base + offset);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, held]);
    }
    let array = e.mem.u32(base + 0xa90);
    e.call(FREE_ARRAY, &args![array]);
    e.call(STEP_00E6E730, &args![this]);
    e.call(STEP_00E6E7B0, &args![this]);
    e.call(
        STREAM_00A63520,
        &args![PERSISTENT_SRC_TEXTURE_RENDERER_DATA],
    );
    e.call(STEP_00A98460, &args![]);

    // The members' destructors.
    destroy_map(e, base + 0xb14, 0x010e_db9c, 0x010e_dac4);
    for (offset, vtable) in [
        (0xb04u32, 0x010e_dae4u32),
        (0xaf4, 0x010e_daf4),
        (0xae4, 0x010e_dae4),
        (0xad4, 0x010e_daec),
    ] {
        let array = e.mem.u32(base + offset + 4);
        e.mem.set_u32(base + offset, vtable);
        e.call(FREE_ARRAY, &args![array]);
    }
    e.call(STRING_LIST_REMOVE_ALL, &args![base + 0x90c]);
    e.call(MODIFIER_LIST_REMOVE_ALL, &args![base + 0x900]);
    let pointer = e.mem.u32(base + 0x8fc);
    release(e, pointer);
    destroy_map(e, base + 0x8ec, 0x010e_db7c, 0x010e_daa4);
    destroy_map(e, base + 0x8dc, 0x010e_db5c, 0x010e_da84);
    destroy_map(e, base + 0x8cc, 0x010e_db3c, 0x010e_da64);
    destroy_map(e, base + 0x890, 0x010e_db1c, 0x010e_da44);
    for offset in [0x87cu32, 0x878, 0x874, 0x870] {
        let pointer = e.mem.u32(base + offset);
        release(e, pointer);
    }
    let pointer = e.mem.u32(base + 0x630);
    release(e, pointer);
    destroy_map(e, base + 0x610, 0x010e_dafc, 0x010e_da24);
    e.call(RENDERER_BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00e7_6210, fn_00e76210_entry as AbiFn),
        entry!(0x00e7_5a70, fn_00e75a70(Ptr)),
    ]
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    const WORDS: [u32; 15] = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
    const RENDERER_VTABLE: u32 = 0x6000;
    const RENDERER_DELETE: u32 = 0x6100;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(direct3d: u32, allocation: bool, initialized: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(CREATE_DIRECT3D9, move |_, _| direct3d.into_ret());
        e.register(STRNCPY_S, |_, _| Ret::default());
        e.register(REPORT_00A29680, |_, _| Ret::default());
        e.register_double(NEW_OBJECT, move |e, a| {
            if allocation {
                e.mem.alloc(a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(RENDERER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], RENDERER_VTABLE);
            a[0].into_ret()
        });
        e.put_vtable(RENDERER_VTABLE, &[RENDERER_DELETE]);
        e.register(RENDERER_DELETE, |_, _| Ret::default());
        e.register_double(RENDERER_INITIALIZE, move |_, _| initialized.into_ret());
        e.register(STREAM_00A63520, |_, _| Ret::default());
        e.register(NI_STREAM_REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e76210_fails_without_direct3d() {
        let mut e = rig(u32::MAX, true, 1);
        assert_eq!(e.call(0x00e7_6210, &WORDS).u32(), 0);
        assert_eq!(calls(&e, CREATE_DIRECT3D9), vec![vec![25]]);
        assert_eq!(
            calls(&e, STRNCPY_S),
            vec![vec![
                ERROR_MESSAGE_011F4648,
                0x100,
                CREATION_FAILED_010EE6C8,
                0xff
            ]]
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![INVALID_DEVICE_010EE688]]
        );
        assert!(calls(&e, NEW_OBJECT).is_empty());
    }

    #[test]
    fn fn_00e76210_initialises_and_registers_the_loader() {
        let mut e = rig(0, true, 0x101);
        let renderer = e.call(0x00e7_6210, &WORDS).u32();
        assert_ne!(renderer, 0);
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![RENDERER_SIZE]]);
        let mut expected = vec![renderer];
        expected.extend_from_slice(&WORDS[..14]);
        assert_eq!(calls(&e, RENDERER_INITIALIZE), vec![expected]);
        assert_eq!(
            calls(&e, STREAM_00A63520),
            vec![vec![PERSISTENT_SRC_TEXTURE_RENDERER_DATA]]
        );
        assert_eq!(
            calls(&e, NI_STREAM_REGISTER_LOADER),
            vec![vec![
                PERSISTENT_SRC_TEXTURE_RENDERER_DATA,
                PERSISTENT_SRC_TEXTURE_LOADER
            ]]
        );
        assert!(calls(&e, RENDERER_DELETE).is_empty());
    }

    #[test]
    fn fn_00e76210_deletes_a_renderer_that_fails_to_initialise() {
        let mut e = rig(0, true, 0x100);
        assert_eq!(e.call(0x00e7_6210, &WORDS).u32(), 0);
        let renderer = calls(&e, RENDERER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, RENDERER_DELETE), vec![vec![renderer, 1]]);
        assert!(calls(&e, STREAM_00A63520).is_empty());
        // No renderer: the initialisation still runs on null; nothing is deleted.
        let mut e = rig(0, false, 0);
        assert_eq!(e.call(0x00e7_6210, &WORDS).u32(), 0);
        assert_eq!(calls(&e, RENDERER_INITIALIZE)[0][0], 0);
        assert!(calls(&e, RENDERER_DELETE).is_empty());
    }
}

#[cfg(test)]
mod b049_tests {
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

    /// A one-bucket map at `map` (virtual table 0x7000, whose slot 4 hashes every key to 0)
    /// holding one item with `key` and `value`.
    fn one_item_map(e: &mut Engine, map: u32, key: u32, value: u32) -> u32 {
        let buckets = e.mem.alloc(4);
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 4, key);
        e.mem.set_u32(item + 8, value);
        e.mem.set_u32(buckets, item);
        e.mem.set_u32(map, 0x7000);
        e.mem.set_u32(map + 4, 1);
        e.mem.set_u32(map + 8, buckets);
        buckets
    }

    #[test]
    fn fn_00e75a70_takes_the_renderer_apart() {
        let mut e = Engine::new();
        e.map(0x011b_f000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        let doubles = [
            MEMORY_MANAGER_FREE,
            FREE_SIZED,
            FREE_ARRAY,
            MAP_DROP_CHAINED_00B57EA0,
            MAP_DROP_00E91FD0,
            STEP_00A8F870,
            PURGE_ALL_D3D_SHADERS,
            STEP_00E819F0,
            STEP_00E7CEC0,
            MAP_CLEAN_UP,
            TEXTURE_DATA_CLEAN_UP_00E8A9B0,
            LIGHT_MANAGER_DESTRUCTOR_00E8B630,
            STEP_00E6E730,
            STEP_00E6E7B0,
            STREAM_00A63520,
            STEP_00A98460,
            STRING_LIST_REMOVE_ALL,
            MODIFIER_LIST_REMOVE_ALL,
            RENDERER_BASE_DESTRUCTOR,
        ];
        for address in doubles {
            e.register(address, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let left = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], left);
            left.into_ret()
        });
        // One virtual table for every object: slot i at 0x8000 + 4 * i; slot 4 (the maps'
        // hash and the counted objects' delete) answers 0, slot 0xa4 (part count) 1, slot
        // 0xb0 and 0xb4 (parts) a part at 0x6000.
        let slots: Vec<u32> = (0..0x70).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        for slot in [0u32, 4, 8, 0xdc, 0x190, 0x1a0] {
            e.register(0x8000 + slot, |_, _| Ret::default());
        }
        e.register(0x8000 + 0xa4, |_, _| 1u32.into_ret());
        e.map(0x6000, 0x20);
        e.register(0x8000 + 0xb0, |_, _| 0x6000u32.into_ret());
        e.register(0x8000 + 0xb4, |_, _| 0u32.into_ret());
        e.set_global(STAGE_COUNT_011BF558, 2u32);

        let this = e.mem.alloc(0xb80);
        let object = |e: &mut Engine, count: u32| {
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, 0x7000);
            e.mem.set_u32(object + 4, count);
            object
        };
        let device = object(&mut e, 1);
        e.mem.set_u32(this + 0x288, device);
        let library = object(&mut e, 1);
        e.mem.set_u32(this + 0x8b8, library);
        e.mem.set_u32(this + 0x6e8, 1);
        // A block on the list at +0x620.
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x620, block);
        // The map at +0x610: one key with a chain of two entries.
        let second = e.mem.alloc(0x24);
        let first = e.mem.alloc(0x24);
        e.mem.set_u32(first + 0x20, second);
        one_item_map(&mut e, this + 0x610, 0x55, first);
        // The map at +0x890: one value with a part holding a counted object.
        let value = object(&mut e, 1);
        let held = object(&mut e, 1);
        e.mem.set_u32(0x6010, held);
        let buckets_890 = one_item_map(&mut e, this + 0x890, 0x66, value);
        e.register_double(MAP_ITERATE_00E6E630, move |e, a| {
            // Hands the one value out and ends the walk.
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        // The map at +0xb14: one value freed to the memory manager.
        one_item_map(&mut e, this + 0xb14, 0x77, 0x4444);
        // Objects and the light manager.
        let deleted = object(&mut e, 1);
        e.mem.set_u32(this + 0x8ac, deleted);
        e.mem.set_u32(this + 0x8b0, 0x8b0);
        e.mem.set_u32(this + 0x8b4, 0x8b4);
        e.mem.set_u32(this + 0x8c8, 0x5550);
        let shared = object(&mut e, 1);
        e.mem.set_u32(POINTERS_0126F108 + 8, shared);

        e.call_log = Some(vec![]);
        e.call(0x00e7_5a70, &args![this]);

        assert_eq!(e.mem.u32(this), DX9_RENDERER_VTABLE);
        assert_eq!(
            calls_to(&e, 0x8000 + 0xdc),
            vec![vec![library, 0, 0], vec![library, 1, 0]]
        );
        assert_eq!(calls_to(&e, 0x8000 + 0x190), vec![vec![device, 0, 0, 0, 0]]);
        assert_eq!(calls_to(&e, 0x8000 + 0x1a0), vec![vec![device, 0]]);
        // 88 blocks from +0x700, then the value of the map at +0xb14, then +0xa80..+0xa88.
        let freed = calls_to(&e, MEMORY_MANAGER_FREE);
        assert_eq!(freed.len(), 88 + 1 + 3);
        assert_eq!(freed[88], vec![MEMORY_MANAGER, 0x4444]);
        // The block list went to the free list and was freed; the chain was dropped.
        let sized = calls_to(&e, FREE_SIZED);
        assert_eq!(
            sized,
            vec![
                vec![block, 0x10],
                vec![first, 0x24],
                vec![second, 0x24],
                vec![0x5550, 0x23c]
            ]
        );
        assert_eq!(e.mem.u32(BLOCK_FREE_LIST), 0);
        assert_eq!(
            calls_to(&e, MAP_DROP_CHAINED_00B57EA0),
            vec![vec![this + 0x610, 0x55, second]]
        );
        assert_eq!(
            calls_to(&e, MAP_DROP_00E91FD0),
            vec![vec![this + 0x610, 0x55]]
        );
        // The +0x890 value's part let go of its object, then the value itself.
        assert_eq!(e.mem.u32(0x6010), 0);
        assert_eq!(e.mem.u32(held + 4), 0);
        assert_eq!(e.mem.u32(value + 4), 0);
        assert_eq!(
            calls_to(&e, MAP_ITERATE_00E6E630)[0][0],
            this + 0x890,
            "the walk starts at the first bucket {buckets_890:x}"
        );
        // +0x8ac got slot 8 twice, then was deleted; the device was released.
        assert_eq!(
            calls_to(&e, 0x8000 + 8),
            vec![vec![deleted, 0x8b0], vec![deleted, 0x8b4], vec![device]]
        );
        assert!(calls_to(&e, 0x8000).contains(&vec![deleted, 1]));
        assert!(calls_to(&e, 0x8000).contains(&vec![library, 1]));
        assert_eq!(e.mem.u32(this + 0x8ac), 0);
        assert_eq!(e.mem.u32(this + 0x8c8), 0);
        assert_eq!(e.mem.u32(this + 0x288), 0);
        assert_eq!(e.mem.u32(POINTERS_0126F108 + 8), 0);
        assert_eq!(
            calls_to(&e, LIGHT_MANAGER_DESTRUCTOR_00E8B630),
            vec![vec![0x5550]]
        );
        assert_eq!(
            calls_to(&e, STREAM_00A63520),
            vec![vec![PERSISTENT_SRC_TEXTURE_RENDERER_DATA]]
        );
        assert_eq!(
            calls_to(&e, TEXTURE_DATA_CLEAN_UP_00E8A9B0),
            vec![vec![this]]
        );
        // The member destructors: the maps' virtual tables end at their bases.
        assert_eq!(e.mem.u32(this + 0x610), 0x010e_da24);
        assert_eq!(e.mem.u32(this + 0xb14), 0x010e_dac4);
        assert_eq!(e.mem.u32(this + 0xad4), 0x010e_daec);
        assert_eq!(
            calls_to(&e, RENDERER_BASE_DESTRUCTOR),
            vec![vec![this]],
            "the base destructor ends it"
        );
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(log.last().unwrap().0, RENDERER_BASE_DESTRUCTOR);
    }
}
