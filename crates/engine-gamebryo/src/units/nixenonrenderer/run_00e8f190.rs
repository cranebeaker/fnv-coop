//! `NiXenonRenderer/run_00e8f190` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiRefObject` virtual table and the global object count it counts up (`011f4400`), the
/// interlocked imports, the bucket allocation `00aa1070`, `memset`, the map set-up `00b601a0`
/// (unnamed; `thiscall`, no stack words) and the allocation `00aa13e0` (`cdecl`, size).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const MAP_00B601A0: u32 = 0x00b6_01a0;
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// The class virtual tables these constructors store (RTTI `NiDX9IndexBufferManager`, its
/// maps' `NiTMapBase<..., unsigned int, NiDX9IndexBufferManager::NiDX9IBInfo *>` and
/// `NiTPointerMap<...>`, `NiDX9TextureManager`, `NiDX9ErrorShader`, `NiDX9RenderState`).
const NI_DX9_INDEX_BUFFER_MANAGER_VTABLE: u32 = 0x010f_02f0;
const IB_MAP_BASE_VTABLE: u32 = 0x010f_02b0;
const IB_MAP_VTABLE: u32 = 0x010f_02d0;
const NI_DX9_TEXTURE_MANAGER_VTABLE: u32 = 0x010f_086c;
const NI_DX9_ERROR_SHADER_VTABLE: u32 = 0x010f_0764;
const NI_DX9_RENDER_STATE_VTABLE: u32 = 0x010f_088c;
/// The renderer's device word (+0x288).
const RENDERER_DEVICE: u32 = 0x288;

/// The `NiRefObject` start every one of these objects makes: its table, count 0, the global
/// object count up.
fn ref_object_start(e: &mut Engine, base: u32) {
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e8fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9IndexBufferManager` constructor (the
/// RTTI of the virtual table it stores), `thiscall` with the device. The `NiRefObject` start,
/// the class's table, three maps at +0x1c, +0x2c, +0x3c (0x25 buckets each, cleared), the
/// device at +8 (counted: its slot 4), +0xc to +0x18 cleared, then `00b601a0` on each map.
/// Answers `this`.
pub fn fn_00e8fb20(e: &mut Engine, this: Ptr, device: u32) -> Ptr {
    let base = this.addr();
    ref_object_start(e, base);
    e.mem.set_u32(base, NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
    for offset in [0x1cu32, 0x2c, 0x3c] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, IB_MAP_BASE_VTABLE);
        e.mem.set_u32(map.wrapping_add(4), 0x25);
        e.mem.set_u32(map.wrapping_add(0xc), 0);
        let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
        e.mem.set_u32(map.wrapping_add(8), buckets);
        let size = e.mem.u32(map.wrapping_add(4)).wrapping_mul(4);
        e.call(MEMSET, &args![buckets, 0u32, size]);
        e.mem.set_u32(map, IB_MAP_VTABLE);
    }
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    for offset in [0xcu32, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    for offset in [0x1cu32, 0x2c, 0x3c] {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    this
}

// Translated from 00e90a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9TextureManager` constructor (the RTTI
/// of the virtual table it stores), `thiscall` with the renderer. The `NiRefObject` start, the
/// class's table, the renderer at +0xc and its device (+0x288) at +8, counted (its slot 4).
/// Answers `this`.
pub fn fn_00e90a80(e: &mut Engine, this: Ptr, renderer: u32) -> Ptr {
    let base = this.addr();
    ref_object_start(e, base);
    e.mem.set_u32(base, NI_DX9_TEXTURE_MANAGER_VTABLE);
    e.mem.set_u32(base.wrapping_add(0xc), renderer);
    let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    this
}

/// The shader base constructor (`00e818a0`, unnamed; `thiscall`, no stack words), the error
/// shader's set-up (`00e90850`, unnamed; `thiscall`, no stack words), the fixed-string maker
/// `00a5b690` (`cdecl`, a C string), the name setter `00a5b950` (unnamed; `thiscall`, the
/// address of a fixed string), the empty fixed string (`0109b220`) and the name
/// `"NiDX9ErrorShader"` (`010f0854`).
const SHADER_CONSTRUCT_00E818A0: u32 = 0x00e8_18a0;
const ERROR_SHADER_SET_UP_00E90850: u32 = 0x00e9_0850;
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
const ERROR_SHADER_NAME_010F0854: u32 = 0x010f_0854;

// Translated from 00e909e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9ErrorShader` constructor (the RTTI of
/// the virtual table it stores), `thiscall`, no stack words. The shader base constructor, the
/// bytes +0x24 and +0x25 and the words +0x28 to +0x38 cleared, the class's table, `00e90850`,
/// the name `"NiDX9ErrorShader"` made as a fixed string (in a stack word), set (`00a5b950`) and
/// released (its count at -8 down unless it is the empty string), then +0x20 set to 1.
/// Answers `this`.
pub fn fn_00e909e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(SHADER_CONSTRUCT_00E818A0, &args![base]);
    e.mem.set_u8(base.wrapping_add(0x24), 0);
    e.mem.set_u8(base.wrapping_add(0x25), 0);
    for offset in [0x28u32, 0x2c, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u32(base, NI_DX9_ERROR_SHADER_VTABLE);
    e.call(ERROR_SHADER_SET_UP_00E90850, &args![base]);
    e.with_stack(4, |e, name| {
        let handle = e
            .call(FIXED_STRING_MAKE, &args![ERROR_SHADER_NAME_010F0854])
            .u32();
        e.mem.set_u32(name.addr(), handle);
        e.call(SET_NAME_00A5B950, &args![base, name]);
        let handle = e.mem.u32(name.addr());
        if handle != e.mem.u32(FIXED_STRING_EMPTY) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    e.mem.set_u8(base.wrapping_add(0x20), 1);
    this
}

/// The render state's base constructor (`00e881a0`, unnamed; `thiscall`, the renderer),
/// `memcpy` (`00ec44d0`), the table of (state, value) word pairs it adjusts (`011bfce8`, ended
/// by state -1, 256 pairs at most), and the object `00e978a0` makes (unnamed; `cdecl`:
/// renderer, caps).
const RENDER_STATE_CONSTRUCT_00E881A0: u32 = 0x00e8_81a0;
const MEMCPY: u32 = 0x00ec_44d0;
const STATE_PAIRS_011BFCE8: u32 = 0x011b_fce8;
const MAKE_00E978A0: u32 = 0x00e9_78a0;

// Translated from 00e91590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): makes the `NiDX9RenderState` (0x1248 bytes; the
/// RTTI of the virtual table it stores), `cdecl`: the renderer, its caps and a byte. A made
/// object gets the base constructor (`00e881a0` with the renderer), its table, the byte
/// +0x1114 cleared and the 0x130 caps bytes copied to +0x1118; null when the allocation fails
/// (its slot 0x104 still runs). Then its virtual slot 0x104; in the pairs at `011bfce8` (until
/// state -1, at most 256), state 7's value becomes 1 when the byte is set, else 0; with the
/// byte set +8 gets bit 1. The object `00e978a0` makes is held at +0x10f0 (`NiPointer`
/// assignment: the old one released, the new one counted). Answers the render state.
pub fn fn_00e91590(e: &mut Engine, renderer: u32, caps: u32, flag: u8) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x1248u32]).u32();
    let state = if block != 0 {
        e.call(RENDER_STATE_CONSTRUCT_00E881A0, &args![block, renderer]);
        e.mem.set_u32(block, NI_DX9_RENDER_STATE_VTABLE);
        e.mem.set_u8(block + 0x1114, 0);
        e.call(MEMCPY, &args![block + 0x1118, caps, 0x130u32]);
        block
    } else {
        0
    };
    e.vcall(state, 0x104, &args![]);
    for pair in 0..0x100u32 {
        let at = STATE_PAIRS_011BFCE8 + 8 * pair;
        let id = e.mem.u32(at);
        if id == 0xffff_ffff {
            break;
        }
        if id == 7 {
            e.mem.set_u32(at + 4, u32::from(flag != 0));
        }
    }
    if flag != 0 {
        let flags = e.mem.u32(state.wrapping_add(8)) | 2;
        e.mem.set_u32(state.wrapping_add(8), flags);
    }
    let made = e.call(MAKE_00E978A0, &args![renderer, caps]).u32();
    let slot = state.wrapping_add(0x10f0);
    let held = e.mem.u32(slot);
    if held != made {
        if held != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(slot, made);
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
        }
    }
    state
}

/// What the error shader's set-up calls: the pass maker `00e804d0` and the render state
/// group maker `00e7edf0` (unnamed; `cdecl`, the address of the word they fill), the state
/// setters `00e88fc0` (three words) and `00e890c0` (four words) (unnamed; `thiscall` on the
/// group's +0x10), the pass attachment `00e802b0` (unnamed; `thiscall` on the pass: its +0x14
/// and the group), the pass releases `00e7ecd0` and `00e80190` (unnamed; `thiscall`) and the
/// global the pass is kept in (`0126f74c`).
const MAKE_PASS_00E804D0: u32 = 0x00e8_04d0;
const MAKE_GROUP_00E7EDF0: u32 = 0x00e7_edf0;
const SET_STATE_00E88FC0: u32 = 0x00e8_8fc0;
const SET_SAMPLER_STATE_00E890C0: u32 = 0x00e8_90c0;
const ATTACH_00E802B0: u32 = 0x00e8_02b0;
const RELEASE_GROUP_00E7ECD0: u32 = 0x00e7_ecd0;
const RELEASE_PASS_00E80190: u32 = 0x00e8_0190;
const ERROR_PASS_0126F74C: u32 = 0x0126_f74c;

// Translated from 00e90850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the error shader's set-up, `thiscall`, no
/// stack words; answers 1. Makes a pass (`00e804d0`, kept at +0x3c) and a state group
/// (`00e7edf0`; +4 and +8 cleared), sets its states (`00e88fc0` with (1, 2, 0), (2, 3, 0), (4,
/// 2, 0), (5, 3, 0), (0xb, 0, 0); `00e890c0` with (1, 3, 0, 0), (2, 3, 0, 0), (5, 2, 0, 0), (6,
/// 2, 0, 0), (7, 0, 0, 0); the byte +0x5e cleared; `00e88fc0` with (0x18, 0, 0)), attaches the
/// group to the pass (`00e802b0` with the pass's +0x14), keeps the pass at `0126f74c`, then
/// drops the local references: the group's count (+0x60) and the pass's count (+0x64) down,
/// each released (`00e7ecd0`, `00e80190`) at zero.
pub fn fn_00e90850(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    e.with_stack(8, |e, locals| {
        let pass_slot = locals.addr();
        let group_slot = locals.addr() + 4;
        e.mem.set_u32(pass_slot, 0);
        e.call(MAKE_PASS_00E804D0, &args![pass_slot]);
        let pass = e.mem.u32(pass_slot);
        e.mem.set_u32(base + 0x3c, pass);
        e.mem.set_u32(group_slot, 0);
        e.call(MAKE_GROUP_00E7EDF0, &args![group_slot]);
        let group = e.mem.u32(group_slot);
        e.mem.set_u32(group + 4, 0);
        e.mem.set_u32(group + 8, 0);
        let states = |e: &mut Engine, words: [u32; 3]| {
            let target = e.mem.u32(group + 0x10);
            e.call(
                SET_STATE_00E88FC0,
                &args![target, words[0], words[1], words[2]],
            );
        };
        let samplers = |e: &mut Engine, words: [u32; 4]| {
            let target = e.mem.u32(group + 0x10);
            e.call(
                SET_SAMPLER_STATE_00E890C0,
                &args![target, words[0], words[1], words[2], words[3]],
            );
        };
        states(e, [1, 2, 0]);
        states(e, [2, 3, 0]);
        states(e, [4, 2, 0]);
        states(e, [5, 3, 0]);
        states(e, [0xb, 0, 0]);
        samplers(e, [1, 3, 0, 0]);
        samplers(e, [2, 3, 0, 0]);
        samplers(e, [5, 2, 0, 0]);
        samplers(e, [6, 2, 0, 0]);
        samplers(e, [7, 0, 0, 0]);
        e.mem.set_u8(group + 0x5e, 0);
        states(e, [0x18, 0, 0]);
        let pass = e.mem.u32(pass_slot);
        let stage = e.mem.u32(pass.wrapping_add(0x14));
        e.call(ATTACH_00E802B0, &args![pass, stage, group]);
        let group = e.mem.u32(group_slot);
        let pass = e.mem.u32(pass_slot);
        e.mem.set_u32(ERROR_PASS_0126F74C, pass);
        if group != 0 {
            let count = e.mem.u32(group + 0x60).wrapping_sub(1);
            e.mem.set_u32(group + 0x60, count);
            if count == 0 {
                e.call(RELEASE_GROUP_00E7ECD0, &args![group]);
            }
        }
        let pass = e.mem.u32(pass_slot);
        if pass != 0 {
            let count = e.mem.u32(pass + 0x64).wrapping_sub(1);
            e.mem.set_u32(pass + 0x64, count);
            if count == 0 {
                e.call(RELEASE_PASS_00E80190, &args![pass]);
            }
        }
    });
    1
}

/// What the index buffer manager uses: the report `00a29680` (`cdecl`) and its messages
/// (`"NiDX9IndexBufferManager::CreateIndexBuffer FAILED"` at `010f0184`,
/// `"NiDX9IndexBufferManager::PackBuffer> Failed to create index buffer\n"` at `010f01b8`,
/// `"...> Failed to lock index buffer\n"` at `010eddd0`), `IDirect3DDevice9::CreateIndexBuffer`
/// (slot 0x6c) and `SetIndices` (0x1a0), `IDirect3DIndexBuffer9::Lock` (0x2c), `Unlock`
/// (0x30) and `GetDesc` (0x34) (slot names by the interfaces' order; the arguments match).
const REPORT_00A29680: u32 = 0x00a2_9680;
const CREATE_INDEX_BUFFER_FAILED_010F0184: u32 = 0x010f_0184;
const PACK_CREATE_FAILED_010F01B8: u32 = 0x010f_01b8;
const PACK_LOCK_FAILED_010EDDD0: u32 = 0x010e_ddd0;
const CREATE_INDEX_BUFFER: u32 = 0x6c;
const SET_INDICES: u32 = 0x1a0;
const INDEX_BUFFER_LOCK: u32 = 0x2c;
const INDEX_BUFFER_UNLOCK: u32 = 0x30;
const INDEX_BUFFER_GET_DESC: u32 = 0x34;
/// `D3DFMT_INDEX16` (0x65) and `D3DRTYPE_INDEXBUFFER` (7), the values the code compares.
const FORMAT_INDEX16: u32 = 0x65;
const RESOURCE_INDEX_BUFFER: u32 = 7;

// Translated from 00e8f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9IndexBufferManager`'s index buffer maker
/// (the name its report gives), `thiscall` with five words: length, usage, format, pool and
/// the address of a description (may be null). The device (+8) creates the buffer into a
/// stack word cleared first (`CreateIndexBuffer`, no shared handle); a failure is reported
/// and null answered. With a description address the buffer's `GetDesc` fills it. Answers the
/// buffer.
pub fn fn_00e8f190(
    e: &mut Engine,
    this: Ptr,
    length: u32,
    usage: u32,
    format: u32,
    pool: u32,
    desc: u32,
) -> u32 {
    let device = e.mem.u32(this.addr().wrapping_add(8));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let result = e
            .vcall(
                device,
                CREATE_INDEX_BUFFER,
                &args![length, usage, format, pool, slot, 0u32],
            )
            .i32();
        if result < 0 {
            e.call(REPORT_00A29680, &args![CREATE_INDEX_BUFFER_FAILED_010F0184]);
            return 0;
        }
        let buffer = e.mem.u32(slot.addr());
        if desc != 0 {
            e.vcall(buffer, INDEX_BUFFER_GET_DESC, &args![desc]);
        }
        e.mem.u32(slot.addr())
    })
}

// Translated from 00e8f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9IndexBufferManager::PackBuffer` (the
/// name its reports give), `thiscall` with seven words: the 16-bit indices, their count, the
/// count the buffer must hold, a buffer that may be reused, the address of the length word,
/// the pool and the usage. Null without a device (+8) or indices. The buffer is reused when
/// its description (`GetDesc` into a cleared stack block) succeeds and says format
/// `D3DFMT_INDEX16`, type index buffer, the usage and pool asked for and a size of at least
/// twice the count to hold; otherwise a new one is created (twice that count, the usage,
/// `D3DFMT_INDEX16`, the pool; into the reuse argument's stack word, cleared first): a failed
/// creation is reported, then a missing buffer is reported and null answered; a new buffer's
/// length goes to the length word. With indices to copy (twice their count in bytes, nonzero),
/// the buffer is locked (offset 0, into the pool argument's stack word, cleared first); the
/// indices are copied in (`memcpy`) and the buffer unlocked; a failed lock is reported and the
/// buffer unlocked and released (its slot 8). Answers the buffer (also a released one).
pub fn fn_00e8f200(
    e: &mut Engine,
    this: Ptr,
    indices: u32,
    count: u32,
    capacity: u32,
    reuse: u32,
    length_out: Ptr,
    pool: u32,
    usage: u32,
) -> u32 {
    let device = e.mem.u32(this.addr().wrapping_add(8));
    if device == 0 || indices == 0 {
        return 0;
    }
    let length = capacity.wrapping_add(capacity);
    let bytes = count.wrapping_add(count);
    let reused = reuse != 0
        && e.with_stack(0x14, |e, desc| {
            let desc = desc.addr();
            if e.vcall(reuse, INDEX_BUFFER_GET_DESC, &args![desc]).i32() < 0 {
                return false;
            }
            e.mem.u32(desc) == FORMAT_INDEX16
                && e.mem.u32(desc + 4) == RESOURCE_INDEX_BUFFER
                && e.mem.u32(desc + 8) == usage
                && e.mem.u32(desc + 0xc) == pool
                && e.mem.u32(desc + 0x10) >= length
        });
    let buffer = if reused {
        reuse
    } else {
        let made = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), 0);
            let device = e.mem.u32(this.addr().wrapping_add(8));
            let result = e
                .vcall(
                    device,
                    CREATE_INDEX_BUFFER,
                    &args![length, usage, FORMAT_INDEX16, pool, slot, 0u32],
                )
                .i32();
            if result < 0 {
                e.call(REPORT_00A29680, &args![CREATE_INDEX_BUFFER_FAILED_010F0184]);
                return 0;
            }
            e.mem.u32(slot.addr())
        });
        if made == 0 {
            e.call(REPORT_00A29680, &args![PACK_CREATE_FAILED_010F01B8]);
            return 0;
        }
        e.mem.set_u32(length_out.addr(), length);
        made
    };
    if bytes != 0 {
        let locked = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), 0);
            let result = e
                .vcall(buffer, INDEX_BUFFER_LOCK, &args![0u32, bytes, slot, 0u32])
                .i32();
            if result < 0 {
                return false;
            }
            let data = e.mem.u32(slot.addr());
            e.call(MEMCPY, &args![data, indices, bytes]);
            true
        });
        if locked {
            e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
            return buffer;
        }
        e.call(REPORT_00A29680, &args![PACK_LOCK_FAILED_010EDDD0]);
        e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
        e.vcall(buffer, 8, &args![]);
    }
    buffer
}

/// The index buffer manager's three maps (+0x1c, +0x2c, +0x3c), the clean-up of an index
/// buffer entry (`00e95b80`, unnamed; `thiscall`, no stack words), the sized free `00aa1460`
/// (`cdecl`: block, size) and the free `00aa10f0` (`cdecl`, a block).
const INDEX_BUFFER_MAPS: [u32; 3] = [0x1c, 0x2c, 0x3c];
const ENTRY_CLEAN_UP_00E95B80: u32 = 0x00e9_5b80;
const FREE_00AA1460: u32 = 0x00aa_1460;
const FREE_00AA10F0: u32 = 0x00aa_10f0;

/// The first item of the first non-empty bucket at or after `bucket` in the map at `map`
/// (the bucket count and array read once), or null.
fn first_item_from(e: &mut Engine, map: u32, bucket: u32) -> u32 {
    let size = e.mem.u32(map.wrapping_add(4));
    if bucket >= size {
        return 0;
    }
    let buckets = e.mem.u32(map.wrapping_add(8));
    for index in bucket..size {
        let item = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if item != 0 {
            return item;
        }
    }
    0
}

// Translated from 00e8f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9IndexBufferManager` destructor (the
/// RTTI of the table it stores), `thiscall`. Its table; for each of its three maps (+0x1c,
/// +0x2c, +0x3c) every item is visited (the next one found first: the item's next, else the
/// first item of a later bucket after the map's hash of the key, virtual slot 4, plus one) and
/// its value, an index buffer information block, freed: each of its five entries (+8 to
/// +0x18) that is set has its buffer (+0x20) released (slot 8) when set, gets `00e95b80` and
/// its 0x38 bytes freed (`00aa1460`), then the block's 0x1c bytes are freed. Then each map is
/// emptied (`00b601a0`), the device's `SetIndices` gets null, the objects at +0xc and +0x14
/// are released (slot 8) when set, the device (+8) released and cleared, and the maps are
/// destroyed from the last (their two tables in turn, `00b601a0` with each, the bucket array
/// freed with `00aa10f0`); last the `NiRefObject` table and the global object count down.
pub fn fn_00e8f840(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
    for offset in INDEX_BUFFER_MAPS {
        let map = base.wrapping_add(offset);
        let mut item = first_item_from(e, map, 0);
        while item != 0 {
            let next = e.mem.u32(item);
            let info = e.mem.u32(item.wrapping_add(8));
            let following = if next != 0 {
                next
            } else {
                let key = e.mem.u32(item.wrapping_add(4));
                let bucket = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
                first_item_from(e, map, bucket)
            };
            if info != 0 {
                for slot in 0..5u32 {
                    let entry = e.mem.u32(info.wrapping_add(8 + 4 * slot));
                    if entry != 0 {
                        let held = e.mem.u32(entry.wrapping_add(0x20));
                        if held != 0 {
                            e.vcall(held, 8, &args![]);
                        }
                        e.call(ENTRY_CLEAN_UP_00E95B80, &args![entry]);
                        e.call(FREE_00AA1460, &args![entry, 0x38u32]);
                    }
                }
                e.call(FREE_00AA1460, &args![info, 0x1cu32]);
            }
            item = following;
        }
    }
    for offset in INDEX_BUFFER_MAPS {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    let device = e.mem.u32(base.wrapping_add(8));
    e.vcall(device, SET_INDICES, &args![0u32]);
    for offset in [0xcu32, 0x14] {
        let held = e.mem.u32(base.wrapping_add(offset));
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
    }
    let device = e.mem.u32(base.wrapping_add(8));
    if device != 0 {
        e.vcall(device, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(8), 0);
    }
    for offset in [0x3cu32, 0x2c, 0x1c] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, IB_MAP_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        e.mem.set_u32(map, IB_MAP_BASE_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        let buckets = e.mem.u32(map.wrapping_add(8));
        e.call(FREE_00AA10F0, &args![buckets]);
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

/// The index buffer maps' lookup (`00a53c30`, unnamed; `thiscall`: the key, the address of the
/// word it fills; answers a byte) and insertion (`00b57ea0`, unnamed; `thiscall`: the key, the
/// value).
const MAP_LOOKUP_00A53C30: u32 = 0x00a5_3c30;
const MAP_SET_00B57EA0: u32 = 0x00b5_7ea0;

// Translated from 00e8fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the index buffer information block for a usage
/// and format, `thiscall` with three words: the usage, the format and the pool (0, 1, 2 pick
/// the map at +0x1c, +0x2c, +0x3c; any other pool answers null). The key is
/// `usage << 16 | format`; a block found in the map is answered; otherwise a new 0x1c-byte one
/// (`00aa13e0`; no null check) gets the key at +0, the byte +4 and the words +8 to +0x18
/// cleared, is put in the map (`00b57ea0`) and answered.
pub fn fn_00e8fa80(e: &mut Engine, this: Ptr, usage: u32, format: u32, pool: u32) -> u32 {
    let key = (usage << 16) | format;
    let offset = match pool {
        0 => 0x1c,
        1 => 0x2c,
        2 => 0x3c,
        _ => return 0,
    };
    let map = this.addr().wrapping_add(offset);
    let found = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if e.call(MAP_LOOKUP_00A53C30, &args![map, key, slot]).u8() != 0 {
            Some(e.mem.u32(slot.addr()))
        } else {
            None
        }
    });
    if let Some(block) = found {
        return block;
    }
    let block = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    e.mem.set_u32(block, 0);
    e.mem.set_u8(block.wrapping_add(4), 0);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(block.wrapping_add(offset), 0);
    }
    e.mem.set_u32(block, key);
    e.call(MAP_SET_00B57EA0, &args![map, key, block]);
    block
}

// Translated from 00e8fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9IndexBufferManager` deleting
/// destructor, `thiscall` with the flags: the destructor ([`fn_00e8f840`]), then with bit 0
/// the 0x4c bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e8fc30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e8f840(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x4cu32]);
    }
    this
}

/// The index buffer entry constructor (`00e95b60`, unnamed; `thiscall`, no stack words).
const ENTRY_CONSTRUCT_00E95B60: u32 = 0x00e9_5b60;

// Translated from 00e8fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): an index buffer from the manager's rotation,
/// `thiscall` with four words: length, usage, format and pool. Null without a device (+8) or
/// an information block ([`fn_00e8fa80`] with usage, format, pool). The block's current entry
/// (the byte +4 indexes the five words from +8): none makes a new 0x38-byte entry (`00e95b60`;
/// null when the allocation fails, the stores still go to it; the entry is not stored in the
/// block) with the pool at +8, the usage at +0xc, 4 at +4, the length at +0x14, the format at
/// +0x18 and +0x1c, +0x20 cleared; an entry whose length (+0x14) is above the length asked
/// has its buffer (+0x20) released (slot 8; no null check) and cleared; otherwise its buffer
/// is the answer and the index moves on. An entry left without a buffer gets a new one ([`fn_00e8f190`]
/// with the entry's +0x24 as the description), its length, format and +0x1c cleared, the
/// index moves on and the buffer is the answer. The index wraps to 0 at 5.
pub fn fn_00e8fc50(
    e: &mut Engine,
    this: Ptr,
    length: u32,
    usage: u32,
    format: u32,
    pool: u32,
) -> u32 {
    if e.mem.u32(this.addr().wrapping_add(8)) == 0 {
        return 0;
    }
    let mut answer = 0;
    let info = fn_00e8fa80(e, this, usage, format, pool);
    if info == 0 {
        return answer;
    }
    let index = e.mem.u8(info.wrapping_add(4));
    let mut entry = e.mem.u32(info.wrapping_add(8 + 4 * u32::from(index)));
    if entry != 0 {
        if e.mem.u32(entry.wrapping_add(0x14)) > length {
            let held = e.mem.u32(entry.wrapping_add(0x20));
            e.vcall(held, 8, &args![]);
            e.mem.set_u32(entry.wrapping_add(0x20), 0);
        } else {
            answer = e.mem.u32(entry.wrapping_add(0x20));
            e.mem.set_u8(info.wrapping_add(4), index.wrapping_add(1));
        }
    } else {
        let block = e.call(NEW_OBJECT, &args![0x38u32]).u32();
        entry = if block != 0 {
            e.call(ENTRY_CONSTRUCT_00E95B60, &args![block]);
            block
        } else {
            0
        };
        e.mem.set_u32(entry.wrapping_add(8), pool);
        e.mem.set_u32(entry.wrapping_add(0xc), usage);
        e.mem.set_u32(entry.wrapping_add(4), 4);
        e.mem.set_u32(entry.wrapping_add(0x14), length);
        e.mem.set_u32(entry.wrapping_add(0x18), format);
        e.mem.set_u32(entry.wrapping_add(0x1c), 0);
        e.mem.set_u32(entry.wrapping_add(0x20), 0);
    }
    if e.mem.u32(entry.wrapping_add(0x20)) == 0 {
        let buffer = fn_00e8f190(
            e,
            this,
            length,
            usage,
            format,
            pool,
            entry.wrapping_add(0x24),
        );
        e.mem.set_u32(entry.wrapping_add(0x20), buffer);
        e.mem.set_u32(entry.wrapping_add(0x14), length);
        e.mem.set_u32(entry.wrapping_add(0x18), format);
        e.mem.set_u32(entry.wrapping_add(0x1c), 0);
        let index = e.mem.u8(info.wrapping_add(4)).wrapping_add(1);
        e.mem.set_u8(info.wrapping_add(4), index);
        answer = buffer;
    }
    if e.mem.u8(info.wrapping_add(4)) == 5 {
        e.mem.set_u8(info.wrapping_add(4), 0);
    }
    answer
}

/// The `NiDX9RenderedTextureData` table (RTTI) and the base destructor `00e8a2b0` (unnamed;
/// `thiscall`, no stack words).
const NI_DX9_RENDERED_TEXTURE_DATA_VTABLE: u32 = 0x010f_02fc;
const TEXTURE_DATA_DOWN_00E8A2B0: u32 = 0x00e8_a2b0;

// Translated from 00e8fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9RenderedTextureData` deleting
/// destructor (the RTTI of the table it stores), `thiscall` with the flags. Its table; the
/// renderer (+0x60) gets its slot 0x15c with the word at +8; the surface (+0x6c) is released
/// (slot 8) and cleared when set; the base destructor `00e8a2b0`; then with bit 0 the 0x70
/// bytes are freed (`00aa1460`). Answers `this`.
pub fn fn_00e8fdc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let renderer = e.mem.u32(base.wrapping_add(0x60));
    let word = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(base, NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
    e.vcall(renderer, 0x15c, &args![word]);
    let surface = e.mem.u32(base.wrapping_add(0x6c));
    if surface != 0 {
        e.vcall(surface, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x6c), 0);
    }
    e.call(TEXTURE_DATA_DOWN_00E8A2B0, &args![base]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x70u32]);
    }
    this
}

/// What `00e8fe10` uses: the pixel format pick `00e89ed0` (unnamed; `cdecl`: the address of
/// three preference words, the renderer's +0x758), the byte at `0126f0d0`, the multisample
/// type of a sample kind (`00e7ccf0`, unnamed; `cdecl`), `NiXenonErrorString` (`00e89d50`,
/// `cdecl`), the message `"NiDX9RenderedTextureData::CreateSurf> Failed CreateImageSurface -
/// %s"` (`010f03b0`) and `IDirect3DDevice9::CreateRenderTarget` (slot 0x70) and
/// `CreateTexture` (0x5c) (by the interface's order; the arguments match).
const PICK_FORMAT_00E89ED0: u32 = 0x00e8_9ed0;
const POOL_FLAG_0126F0D0: u32 = 0x0126_f0d0;
const MULTISAMPLE_TYPE_00E7CCF0: u32 = 0x00e7_ccf0;
const ERROR_STRING_00E89D50: u32 = 0x00e8_9d50;
const CREATE_SURFACE_FAILED_010F03B0: u32 = 0x010f_03b0;
const CREATE_RENDER_TARGET: u32 = 0x70;
const CREATE_TEXTURE: u32 = 0x5c;

// Translated from 00e8fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9RenderedTextureData::CreateSurf` (the
/// name its report gives), `thiscall` with the texture and a sample kind. Null without a
/// texture. Its width (slot 0x94) and height (0x98) go to +0xc and +0x10, +0x68 becomes 1; the
/// pixel format picked for its three preference words (+0x18 to +0x20, copied to the stack) by
/// `00e89ed0` is copied to +0x14 (0x11 words) and gives the Direct3D format (+0xc), replaced
/// by the texture's +0x44 when its byte +0x41 is set. A texture with the byte +0x40 set uses
/// usage 1 and pool 0; else usage 0 and pool 1 when the byte at `0126f0d0` is clear, 0 when
/// set. With a sample kind the device (renderer +0x60, its +0x288) creates a render target
/// surface into +0x6c (width, height, format, `00e7ccf0`'s multisample type, quality 0, not
/// lockable); in any case a texture (width, height, 1 level, usage, format, pool, into a stack
/// word). When either result is negative (their bits or-ed) the failure is reported, +0x64 and
/// +0x6c cleared and null answered; else the texture goes to +0x64 and the pixel format is
/// answered.
pub fn fn_00e8fe10(e: &mut Engine, this: Ptr, texture: u32, kind: u32) -> u32 {
    let base = this.addr();
    if texture == 0 {
        return 0;
    }
    let width = e.vcall(texture, 0x94, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0xc), width);
    let height = e.vcall(texture, 0x98, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0x10), height);
    e.mem.set_u32(base.wrapping_add(0x68), 1);
    e.with_stack(0x10, |e, frame| {
        let made = frame.addr();
        let preferences = made + 4;
        for word in 0..3u32 {
            let value = e.mem.u32(texture.wrapping_add(0x18 + 4 * word));
            e.mem.set_u32(preferences + 4 * word, value);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let pixel_format = e
            .call(
                PICK_FORMAT_00E89ED0,
                &args![preferences, renderer.wrapping_add(0x758)],
            )
            .u32();
        for word in 0..0x11u32 {
            let value = e.mem.u32(pixel_format.wrapping_add(4 * word));
            e.mem.set_u32(base.wrapping_add(0x14 + 4 * word), value);
        }
        let mut format = e.mem.u32(pixel_format.wrapping_add(0xc));
        let (usage, pool) = if e.mem.u8(texture.wrapping_add(0x40)) != 0 {
            (1u32, 0u32)
        } else {
            (0, u32::from(e.mem.u8(POOL_FLAG_0126F0D0) == 0))
        };
        if e.mem.u8(texture.wrapping_add(0x41)) != 0 {
            format = e.mem.u32(texture.wrapping_add(0x44));
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
        e.mem.set_u32(made, 0);
        let mut surface_result = 0u32;
        if kind != 0 {
            let samples = e.call(MULTISAMPLE_TYPE_00E7CCF0, &args![kind]).u32();
            let width = e.mem.u32(base.wrapping_add(0xc));
            let height = e.mem.u32(base.wrapping_add(0x10));
            surface_result = e
                .vcall(
                    device,
                    CREATE_RENDER_TARGET,
                    &args![
                        width,
                        height,
                        format,
                        samples,
                        0u32,
                        0u32,
                        base.wrapping_add(0x6c),
                        0u32
                    ],
                )
                .u32();
        }
        let width = e.mem.u32(base.wrapping_add(0xc));
        let height = e.mem.u32(base.wrapping_add(0x10));
        let texture_result = e
            .vcall(
                device,
                CREATE_TEXTURE,
                &args![width, height, 1u32, usage, format, pool, made, 0u32],
            )
            .u32();
        let result = surface_result | texture_result;
        if (result as i32) < 0 {
            let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(
                REPORT_00A29680,
                &args![CREATE_SURFACE_FAILED_010F03B0, name],
            );
            e.mem.set_u32(base.wrapping_add(0x64), 0);
            e.mem.set_u32(base.wrapping_add(0x6c), 0);
            return 0;
        }
        let made = e.mem.u32(made);
        e.mem.set_u32(base.wrapping_add(0x64), made);
        pixel_format
    })
}

/// What `00e8ff60` uses: the base constructor `00e8a260` (unnamed; `thiscall`, two words), the
/// interface identifier at `01101014` (`IID_IDirect3DTexture9`, by the report's text), the
/// texture buffer data maker `00e7d250` (unnamed; `cdecl`: texture, group word, kind, word)
/// and the messages `"NiDX9RenderedTextureData::Create> Failed QueryInterface
/// IID_IDirect3DTexture9 - %s"` (`010f0440`) and `"...> Failed NiDX92DBufferData::Create -
/// %s"` (`010f03f8`, given the string at `01011584`).
const TEXTURE_DATA_CONSTRUCT_00E8A260: u32 = 0x00e8_a260;
const IID_DIRECT3D_TEXTURE_01101014: u32 = 0x0110_1014;
const MAKE_TEXTURE_BUFFER_DATA_00E7D250: u32 = 0x00e7_d250;
const QUERY_FAILED_010F0440: u32 = 0x010f_0440;
const BUFFER_DATA_FAILED_010F03F8: u32 = 0x010f_03f8;
const BUFFER_DATA_FAILED_ARGUMENT_01011584: u32 = 0x0101_1584;

// Translated from 00e8ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9RenderedTextureData::Create` (the name
/// its reports give), `cdecl`: the texture, a word handed to the base constructor and the
/// sample kind. A new 0x70-byte object (`00aa13e0`; null when that fails) gets the base
/// constructor (`00e8a260` with the texture and the word) and the class's table; when
/// [`fn_00e8fe10`] fails the object (when made) is deleted (slot 0 with 1) and null answered.
/// Otherwise the object goes to +0x24 of its +8; its Direct3D texture (slot 0x9c) is asked for
/// `IID_IDirect3DTexture9` (`QueryInterface` into a cleared stack word) and the texture's
/// slot 0xa4 gives the render target group word: a failed query is reported, the object
/// deleted and null answered. The texture buffer data is made (`00e7d250` with the queried
/// texture, the group word, the kind and +0x6c), the queried texture released (slot 8) when
/// set; without buffer data the failure is reported, the object deleted and null answered.
/// Answers the object.
pub fn fn_00e8ff60(e: &mut Engine, texture: u32, word: u32, kind: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x70u32]).u32();
    let data = if block != 0 {
        e.call(
            TEXTURE_DATA_CONSTRUCT_00E8A260,
            &args![block, texture, word],
        );
        e.mem.set_u32(block, NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
        block
    } else {
        0
    };
    if fn_00e8fe10(e, Ptr::new(data), texture, kind) == 0 {
        if data != 0 {
            e.vcall(data, 0, &args![1u32]);
        }
        return 0;
    }
    let owner = e.mem.u32(data.wrapping_add(8));
    e.mem.set_u32(owner.wrapping_add(0x24), data);
    let direct3d_texture = e.vcall(data, 0x9c, &args![]).u32();
    let group = e.vcall(texture, 0xa4, &args![]).u32();
    e.with_stack(8, |e, frame| {
        let queried = frame.addr();
        let group_word = queried + 4;
        e.mem.set_u32(group_word, group);
        e.mem.set_u32(queried, 0);
        let result = e
            .vcall(
                direct3d_texture,
                0,
                &args![IID_DIRECT3D_TEXTURE_01101014, queried],
            )
            .i32();
        if result < 0 {
            let name = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(REPORT_00A29680, &args![QUERY_FAILED_010F0440, name]);
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        let surface_word = e.mem.u32(data.wrapping_add(0x6c));
        let queried_texture = e.mem.u32(queried);
        let made = e
            .call(
                MAKE_TEXTURE_BUFFER_DATA_00E7D250,
                &args![queried_texture, group_word, kind, surface_word],
            )
            .u32();
        let queried_texture = e.mem.u32(queried);
        if queried_texture != 0 {
            e.vcall(queried_texture, 8, &args![]);
        }
        if made == 0 {
            e.call(
                REPORT_00A29680,
                &args![
                    BUFFER_DATA_FAILED_010F03F8,
                    BUFFER_DATA_FAILED_ARGUMENT_01011584
                ],
            );
            e.vcall(data, 0, &args![1u32]);
            return 0;
        }
        data
    })
}

// Translated from 00e902d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the word at +0x6c.
pub fn fn_00e902d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x6c))
}

/// The shader base destructor `00e806c0` (`nid3dshader.cpp`; `thiscall`, no stack words).
const SHADER_DOWN_00E806C0: u32 = 0x00e8_06c0;

// Translated from 00e90560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9ErrorShader` deleting destructor,
/// `thiscall` with the flags: its table, the error pass global (`0126f74c`) cleared, the
/// shader base destructor (`00e806c0`), then with bit 0 the 0x40 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn fn_00e90560(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_DX9_ERROR_SHADER_VTABLE);
    e.mem.set_u32(ERROR_PASS_0126F74C, 0);
    e.call(SHADER_DOWN_00E806C0, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x40u32]);
    }
    this
}

/// `NiXenonShaderDeclaration::Create` (`00e76700`, `cdecl`: three words).
const SHADER_DECLARATION_CREATE_00E76700: u32 = 0x00e7_6700;

// Translated from 00e90970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall`; answers 1. The shader declaration
/// made by `NiXenonShaderDeclaration::Create` (with +0x18, 1, 1) is held at +0x28 (`NiPointer`
/// assignment: the old one let go, deleted at zero; the new one counted up when not null),
/// then the held declaration gets its slot 0x8c with (0, 0, 0, 2, 0).
pub fn fn_00e90970(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    let word = e.mem.u32(base.wrapping_add(0x18));
    let made = e
        .call(SHADER_DECLARATION_CREATE_00E76700, &args![word, 1u32, 1u32])
        .u32();
    let held = e.mem.u32(base.wrapping_add(0x28));
    if held != made {
        if held != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(held, 4, &args![]);
        }
        e.mem.set_u32(base.wrapping_add(0x28), made);
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
        }
    }
    let held = e.mem.u32(base.wrapping_add(0x28));
    e.vcall(held, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    1
}

/// `NiD3DShaderInterface::Initialize` (`00e80570`, `thiscall`; answers a byte).
const SHADER_INTERFACE_INITIALIZE_00E80570: u32 = 0x00e8_0570;

// Translated from 00e90a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall`: unless the byte +0x10 is set, runs
/// `NiD3DShaderInterface::Initialize` (false when it fails) and then [`fn_00e90970`]. Answers
/// true otherwise.
pub fn fn_00e90a50(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.u8(this.addr().wrapping_add(0x10)) == 0 {
        if e.call(SHADER_INTERFACE_INITIALIZE_00E80570, &args![this])
            .u8()
            == 0
        {
            return false;
        }
        fn_00e90970(e, this);
    }
    true
}

// Translated from 00e90ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9TextureManager` deleting destructor,
/// `thiscall` with the flags: its table, the device (+8) released (slot 8; no null check) and
/// cleared, the `NiRefObject` table and the global object count down, then with bit 0 the 0x10
/// bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e90ac0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let device = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(base, NI_DX9_TEXTURE_MANAGER_VTABLE);
    e.vcall(device, 8, &args![]);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x10u32]);
    }
    this
}

/// What the texture manager's two queries use: the renderer (`011f4748`, whose +0x180 is a
/// `CRITICAL_SECTION`) with the imported `EnterCriticalSection` / `LeaveCriticalSection`
/// (import slots), the source lookup `00653290` (`bsfacegenmanager.cpp` by the engine map;
/// `thiscall` on the texture, the word `011f444c`) and the renderer data maker `00e68ef0`
/// (unnamed; `cdecl`: the source, the manager's renderer).
const RENDERER_011F4748: u32 = 0x011f_4748;
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const SOURCE_OF_00653290: u32 = 0x0065_3290;
const SOURCE_KIND_011F444C: u32 = 0x011f_444c;
const MAKE_RENDERER_DATA_00E68EF0: u32 = 0x00e6_8ef0;

/// The texture's renderer data (+0x24) under the renderer's lock; when there is none it is
/// made from the texture's source (`00653290`, `00e68ef0` with the manager's +0xc) and
/// `made` is called; no source answers null.
fn renderer_data(e: &mut Engine, this: Ptr, texture: u32, made: impl FnOnce(&mut Engine)) -> u32 {
    let lock = e.mem.u32(RENDERER_011F4748).wrapping_add(0x180);
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![lock]);
    let mut data = e.mem.u32(texture.wrapping_add(0x24));
    if data == 0 {
        let source = e
            .call(SOURCE_OF_00653290, &args![texture, SOURCE_KIND_011F444C])
            .u32();
        if source == 0 {
            e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![lock]);
            return 0;
        }
        let renderer = e.mem.u32(this.addr().wrapping_add(0xc));
        data = e
            .call(MAKE_RENDERER_DATA_00E68EF0, &args![source, renderer])
            .u32();
        made(e);
    }
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![lock]);
    data
}

/// Whether `value` is a nonzero power of two (`v != 0 && (v & (v - 1)) == 0`).
fn power_of_two(value: u32) -> bool {
    value != 0 && value & value.wrapping_sub(1) == 0
}

// Translated from 00e90b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the texture manager's query of a texture,
/// `thiscall` with four words: the texture and the addresses of three bytes it clears first
/// (`made`, `mipmapped`, `not_power_of_two`). Null without a texture. The texture's renderer
/// data (+0x24, made under the renderer's lock when missing, which sets `made`; null without a
/// source) gives `not_power_of_two` from its width (+0xc) and height (+0x10). When its slot
/// 0xa4 or else 0xac answers an object, that object's slot 0x9c is answered. Otherwise the
/// object of slot 0xa8, when there is one, gets its slot 0xb4 when the data's slot 0x9c
/// answers null (which sets `made`) or when the texture's byte +0x40 is clear; `mipmapped` is
/// whether the data's slot 0x98 answers more than 1, and the data's slot 0x9c is answered.
pub fn fn_00e90b10(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    made: Ptr,
    mipmapped: Ptr,
    not_power_of_two: Ptr,
) -> u32 {
    e.mem.set_u8(made.addr(), 0);
    e.mem.set_u8(mipmapped.addr(), 0);
    e.mem.set_u8(not_power_of_two.addr(), 0);
    if texture == 0 {
        return 0;
    }
    let data = renderer_data(e, this, texture, |e| e.mem.set_u8(made.addr(), 1));
    if data == 0 {
        return 0;
    }
    let width = e.mem.u32(data.wrapping_add(0xc));
    let height = e.mem.u32(data.wrapping_add(0x10));
    let regular = power_of_two(width) && power_of_two(height);
    e.mem.set_u8(not_power_of_two.addr(), u8::from(!regular));
    let mut target = e.vcall(data, 0xa4, &args![]).u32();
    if target == 0 {
        target = e.vcall(data, 0xac, &args![]).u32();
    }
    if target != 0 {
        return e.vcall(target, 0x9c, &args![]).u32();
    }
    let other = e.vcall(data, 0xa8, &args![]).u32();
    if other != 0 {
        if e.vcall(data, 0x9c, &args![]).u32() == 0 {
            e.vcall(other, 0xb4, &args![]);
            e.mem.set_u8(made.addr(), 1);
        } else if e.mem.u8(texture.wrapping_add(0x40)) == 0 {
            e.vcall(other, 0xb4, &args![]);
        }
    }
    let levels = e.vcall(data, 0x98, &args![]).u32();
    e.mem.set_u8(mipmapped.addr(), u8::from(levels > 1));
    e.vcall(data, 0x9c, &args![]).u32()
}

// Translated from 00e90c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the texture manager's second query, `thiscall`
/// with three words of which it reads the first, the texture. False without a texture or
/// without renderer data (made under the renderer's lock as in [`fn_00e90b10`]). When the
/// data's slot 0xa4 or else 0xac answers an object, whether that object's slot 0x9c answers
/// nonzero; with neither, true.
pub fn fn_00e90c70(
    e: &mut Engine,
    this: Ptr,
    texture: u32,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    if texture == 0 {
        return false;
    }
    let data = renderer_data(e, this, texture, |_| {});
    if data == 0 {
        return false;
    }
    let mut target = e.vcall(data, 0xa4, &args![]).u32();
    if target == 0 {
        target = e.vcall(data, 0xac, &args![]).u32();
        if target == 0 {
            return true;
        }
    }
    e.vcall(target, 0x9c, &args![]).u32() != 0
}

/// The render state's reset of its shader words (`00e87a50`, unnamed; `thiscall`).
const RESET_SHADERS_00E87A50: u32 = 0x00e8_7a50;

// Translated from 00e90d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` on the render state: clears the byte
/// +0x1100 and the words +0x1104 and +0x110c, sets +0x1108 and +0x1110 to 1, then `00e87a50`
/// (a tail jump).
pub fn fn_00e90d80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base.wrapping_add(0x1100), 0);
    e.mem.set_u32(base.wrapping_add(0x1104), 0);
    e.mem.set_u32(base.wrapping_add(0x1108), 1);
    e.mem.set_u32(base.wrapping_add(0x110c), 0);
    e.mem.set_u32(base.wrapping_add(0x1110), 1);
    e.call(RESET_SHADERS_00E87A50, &args![base]);
}

/// The render state's device word (+0x10f8) and the device's `SetVertexShader` (slot 0x170)
/// and `SetVertexDeclaration` (0x15c), by the interface's order; the arguments match.
const STATE_DEVICE: u32 = 0x10f8;
const SET_VERTEX_SHADER: u32 = 0x170;
const SET_VERTEX_DECLARATION: u32 = 0x15c;

// Translated from 00e90e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot of `NiDX9RenderState`), `thiscall`
/// with the vertex shader and a byte: with the byte set the current one (+0x10e0) is kept at
/// +0x10e4; a different shader becomes current and goes to the device's `SetVertexShader` (a
/// tail jump).
pub fn fn_00e90e00(e: &mut Engine, this: Ptr, shader: u32, keep: u8) {
    let base = this.addr();
    if keep != 0 {
        let current = e.mem.u32(base.wrapping_add(0x10e0));
        e.mem.set_u32(base.wrapping_add(0x10e4), current);
    }
    if e.mem.u32(base.wrapping_add(0x10e0)) != shader {
        e.mem.set_u32(base.wrapping_add(0x10e0), shader);
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.vcall(device, SET_VERTEX_SHADER, &args![shader]);
    }
}

// Translated from 00e90e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the current vertex
/// shader (+0x10e0).
pub fn fn_00e90e40(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x10e0))
}

// Translated from 00e90e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall`: sets the kept vertex shader
/// (+0x10e4) again through its own virtual slot 0x8c (with 0 for the byte).
pub fn fn_00e90e50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let kept = e.mem.u32(base.wrapping_add(0x10e4));
    e.vcall(base, 0x8c, &args![kept, 0u32]);
}

// Translated from 00e90e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` with a vertex shader that goes away:
/// when it is current (+0x10e0) that is cleared and the device's `SetVertexShader` gets null;
/// when it is the kept one (+0x10e4) that is cleared.
pub fn fn_00e90e70(e: &mut Engine, this: Ptr, shader: u32) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x10e0)) == shader {
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.mem.set_u32(base.wrapping_add(0x10e0), 0);
        e.vcall(device, SET_VERTEX_SHADER, &args![0u32]);
    }
    if e.mem.u32(base.wrapping_add(0x10e4)) == shader {
        e.mem.set_u32(base.wrapping_add(0x10e4), 0);
    }
}

// Translated from 00e90fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with a vertex
/// declaration and a byte: a null declaration does nothing; nor does the current one (+0x110c)
/// while the byte +0x1100 says one is set. Otherwise the byte +0x1100 becomes 1, with the byte
/// argument set the current one is kept at +0x1110, the declaration becomes current and goes
/// to the device's `SetVertexDeclaration` (a tail jump).
pub fn fn_00e90fc0(e: &mut Engine, this: Ptr, declaration: u32, keep: u8) {
    let base = this.addr();
    if declaration == 0 {
        return;
    }
    if e.mem.u8(base.wrapping_add(0x1100)) != 0
        && e.mem.u32(base.wrapping_add(0x110c)) == declaration
    {
        return;
    }
    e.mem.set_u8(base.wrapping_add(0x1100), 1);
    if keep != 0 {
        let current = e.mem.u32(base.wrapping_add(0x110c));
        e.mem.set_u32(base.wrapping_add(0x1110), current);
    }
    e.mem.set_u32(base.wrapping_add(0x110c), declaration);
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.vcall(device, SET_VERTEX_DECLARATION, &args![declaration]);
}

// Translated from 00e91020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the current vertex
/// declaration (+0x110c) when the byte +0x1100 says one is set, else 0.
pub fn fn_00e91020(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u8(base.wrapping_add(0x1100)) != 0 {
        return e.mem.u32(base.wrapping_add(0x110c));
    }
    0
}

// Translated from 00e91060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` with a vertex declaration that goes
/// away: when it is not null and the byte +0x1100 is set, the current (+0x110c) and kept
/// (+0x1110) words holding it are cleared.
pub fn fn_00e91060(e: &mut Engine, this: Ptr, declaration: u32) {
    let base = this.addr();
    if declaration == 0 || e.mem.u8(base.wrapping_add(0x1100)) == 0 {
        return;
    }
    if e.mem.u32(base.wrapping_add(0x110c)) == declaration {
        e.mem.set_u32(base.wrapping_add(0x110c), 0);
    }
    if e.mem.u32(base.wrapping_add(0x1110)) == declaration {
        e.mem.set_u32(base.wrapping_add(0x1110), 0);
    }
}

/// The sampler state slots by state (`0126f92c`, 16-bit entries; below 5 the state is cached)
/// and the device's `SetSamplerState` (slot 0x114, by the interface's order).
const SAMPLER_STATE_SLOTS_0126F92C: u32 = 0x0126_f92c;
const SET_SAMPLER_STATE: u32 = 0x114;

// Translated from 00e910a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` with four words, the last not read:
/// the sampler, the state and the value. A state whose slot (`0126f92c`) is below 5 is cached
/// at +0xe20 (8 bytes per slot, 5 slots per sampler); a value different from the cached one
/// is cached and goes to the device's `SetSamplerState`.
pub fn fn_00e910a0(
    e: &mut Engine,
    this: Ptr,
    sampler: u32,
    state: u32,
    value: u32,
    _unused_3: u32,
) {
    let slot = u32::from(
        e.mem
            .u16(SAMPLER_STATE_SLOTS_0126F92C.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 5 {
        return;
    }
    let base = this.addr();
    let cached = base
        .wrapping_add(0xe20)
        .wrapping_add(sampler.wrapping_mul(5).wrapping_add(slot).wrapping_mul(8));
    if e.mem.u32(cached) == value {
        return;
    }
    e.mem.set_u32(cached, value);
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.vcall(device, SET_SAMPLER_STATE, &args![sampler, state, value]);
}

/// The render state's base destructor (`00e88b00`, unnamed; `thiscall`).
const RENDER_STATE_DOWN_00E88B00: u32 = 0x00e8_8b00;

// Translated from 00e91160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9RenderState` deleting destructor,
/// `thiscall` with the flags: its table, the base destructor (`00e88b00`), then with bit 0 the
/// 0x1248 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e91160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_DX9_RENDER_STATE_VTABLE);
    e.call(RENDER_STATE_DOWN_00E88B00, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x1248u32]);
    }
    this
}

/// What `00e911e0` uses: the member reset `00e94130` (unnamed; `thiscall` on +0xf8), the
/// default colour words (`011f4998` to `011f49a0`) and the `double` 255.0 (`0101e568`).
const MEMBER_RESET_00E94130: u32 = 0x00e9_4130;
const DEFAULT_COLOR_011F4998: u32 = 0x011f_4998;
const SCALE_0101E568: u32 = 0x0101_e568;

/// The low byte of a colour channel: `FISTP` under the chop rounding the code sets.
fn colour_byte(value: f64) -> u32 {
    (crate::x87::fistp_chop(value) as u32) & 0xff
}

// Translated from 00e911e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot of `NiDX9RenderState`), the
/// render state's reset, `thiscall`. Unless the flags at +0x1138 have bit 0x4000, the (state,
/// value) pairs at `011bfce8` (ended by state -1) are compacted in place: pairs of states
/// 0xbe, 0xbf and 0xc0 are dropped (the loop's read and write positions and its end test, on
/// the word at the write position, kept as the code has them). Then its virtual slots 0x58,
/// 0x70, 0xbc and 0xd8, `00e94130` on +0xf8, and its slot 0x68 (render state, value, 0, 0)
/// for states 0x16 (2), 8 (3), 9 (2), 0x1b (0), 0x13 (+0x38), 0x14 (+0x3c), 0xf (0), 0x19 (8),
/// 0x18 (0), 0x17 (8), 0xe (0), 0x1a (0), 0x1d (0), 0x1c (0), 0x23 (3 when bit 0 of +8 is set,
/// else 0), 0x8c (3), 0x30 (0), 0x22 (0), 0x8f (1), 7 (1) and 0x8d (1). Last the colour words
/// from `011f4998` go to +0x8c and their packed form (alpha 0xff, each channel times 255.0
/// truncated to its low byte) to +0x98.
pub fn fn_00e911e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x1138)) & 0x4000 == 0 {
        let pair = |index: u32| STATE_PAIRS_011BFCE8.wrapping_add(index.wrapping_mul(8));
        let mut written = 0u32;
        let mut dropped = 0u32;
        if e.mem.u32(STATE_PAIRS_011BFCE8) != 0xffff_ffff {
            let mut write = STATE_PAIRS_011BFCE8;
            let mut test = STATE_PAIRS_011BFCE8;
            loop {
                let read = pair(dropped.wrapping_add(written));
                let state = e.mem.u32(read);
                if state == 0xbe || state == 0xbf || state == 0xc0 {
                    dropped = dropped.wrapping_add(1);
                } else {
                    e.mem.set_u32(write, state);
                    let value = e.mem.u32(read.wrapping_add(4));
                    e.mem.set_u32(write.wrapping_add(4), value);
                    if e.mem.u32(read) == 0xffff_ffff {
                        dropped = dropped.wrapping_sub(1);
                    }
                    written = written.wrapping_add(1);
                    write = write.wrapping_add(8);
                    test = write;
                }
                if e.mem.u32(test) == 0xffff_ffff {
                    break;
                }
            }
        }
    }
    for slot in [0x58u32, 0x70, 0xbc, 0xd8] {
        e.vcall(base, slot, &args![]);
    }
    e.call(MEMBER_RESET_00E94130, &args![base.wrapping_add(0xf8)]);
    // States 0x13, 0x14 and 0x23 take values read when their call is made (below).
    let states = [
        (0x16u32, 2u32),
        (8, 3),
        (9, 2),
        (0x1b, 0),
        (0x13, 0),
        (0x14, 0),
        (0xf, 0),
        (0x19, 8),
        (0x18, 0),
        (0x17, 8),
        (0xe, 0),
        (0x1a, 0),
        (0x1d, 0),
        (0x1c, 0),
        (0x23, 0),
        (0x8c, 3),
        (0x30, 0),
        (0x22, 0),
        (0x8f, 1),
        (7, 1),
        (0x8d, 1),
    ];
    for (state, value) in states {
        let value = match state {
            0x13 => e.mem.u32(base.wrapping_add(0x38)),
            0x14 => e.mem.u32(base.wrapping_add(0x3c)),
            0x23 if e.mem.u8(base.wrapping_add(8)) & 1 != 0 => 3,
            _ => value,
        };
        e.vcall(base, 0x68, &args![state, value, 0u32, 0u32]);
    }
    for word in 0..3u32 {
        let value = e.mem.u32(DEFAULT_COLOR_011F4998 + 4 * word);
        e.mem.set_u32(base.wrapping_add(0x8c + 4 * word), value);
    }
    let scale = x87_load64(e.mem.u64(SCALE_0101E568));
    // FMUL ST(1),ST(0) (bytes DC C9), then FMUL ST(0),ST(1) (D8 C9), then FMUL dword.
    let red = x87_mul(x87_load32(e.mem.u32(base.wrapping_add(0x8c))), scale);
    let mut packed = (0xffff_ff00 | colour_byte(red)) << 8;
    let green = x87_mul(x87_load32(e.mem.u32(base.wrapping_add(0x90))), scale);
    packed = (packed | colour_byte(green)) << 8;
    let blue = x87_mul(scale, x87_m32(e.mem.u32(base.wrapping_add(0x94))));
    packed |= colour_byte(blue);
    e.mem.set_u32(base.wrapping_add(0x98), packed);
}

// Translated from 00e914e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`), `thiscall` on the render state: bit 0 of +8 set
/// when the caps word +0x113c has both 0x100 and 0x100000, else cleared; +0x6c and +0x70
/// rebuilt from the eleven words at +0x20 (bit `n` when word `n` shares a bit with the masks at
/// +0x1144 resp. +0x1148, read once); bit 2 of +8 from bit 4 of the byte +0x1164; the byte
/// +0x1114 set when the renderer (+0x10fc) has its byte +0x5d4 set and +0x5d5 clear, else
/// cleared.
pub fn fn_00e914e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let caps = e.mem.u32(base.wrapping_add(0x113c));
    let flags = e.mem.u32(base.wrapping_add(8));
    let flags = if caps & 0x100 != 0 && caps & 0x10_0000 != 0 {
        flags | 1
    } else {
        flags & 0xffff_fffe
    };
    e.mem.set_u32(base.wrapping_add(8), flags);
    let second_mask = e.mem.u32(base.wrapping_add(0x1148));
    let first_mask = e.mem.u32(base.wrapping_add(0x1144));
    e.mem.set_u32(base.wrapping_add(0x70), 0);
    e.mem.set_u32(base.wrapping_add(0x6c), 0);
    let mut bit = 1u32;
    for index in 0..0xbu32 {
        let word = e.mem.u32(base.wrapping_add(0x20 + 4 * index));
        if word & first_mask != 0 {
            let value = e.mem.u32(base.wrapping_add(0x6c)) | bit;
            e.mem.set_u32(base.wrapping_add(0x6c), value);
        }
        if word & second_mask != 0 {
            let value = e.mem.u32(base.wrapping_add(0x70)) | bit;
            e.mem.set_u32(base.wrapping_add(0x70), value);
        }
        bit = bit.rotate_left(1);
    }
    let flags = e.mem.u32(base.wrapping_add(8));
    let flags = if e.mem.u8(base.wrapping_add(0x1164)) & 0x10 != 0 {
        flags | 4
    } else {
        flags & 0xffff_fffb
    };
    e.mem.set_u32(base.wrapping_add(8), flags);
    let renderer = e.mem.u32(base.wrapping_add(0x10fc));
    let fixed =
        e.mem.u8(renderer.wrapping_add(0x5d4)) != 0 && e.mem.u8(renderer.wrapping_add(0x5d5)) == 0;
    e.mem.set_u8(base.wrapping_add(0x1114), u8::from(fixed));
}

/// The two connectivity messages (`"NiDX9IndexBufferManager::ResizeTempPointConnectivity>
/// Failed to create index buffer\n"` and the `ResizeTempLineConnectivity` one).
const POINT_CONNECTIVITY_FAILED_010F0200: u32 = 0x010f_0200;
const LINE_CONNECTIVITY_FAILED_010F0258: u32 = 0x010f_0258;

/// Whether the index buffer's description (`GetDesc`, a five-word buffer) is 16-bit indices
/// (0x65) of an index buffer (7) with the usage, the pool and at least `size` bytes. One that
/// does not fit is released (slot 8); a failed `GetDesc` answers false without releasing it.
fn index_buffer_fits(e: &mut Engine, buffer: u32, usage: u32, pool: u32, size: u32) -> bool {
    e.with_stack(0x14, |e, desc| {
        let desc = desc.addr();
        for word in 0..5u32 {
            e.mem.set_u32(desc + 4 * word, 0);
        }
        if e.vcall(buffer, INDEX_BUFFER_GET_DESC, &args![desc]).i32() < 0 {
            return false;
        }
        let fits = e.mem.u32(desc) == FORMAT_INDEX16
            && e.mem.u32(desc + 4) == RESOURCE_INDEX_BUFFER
            && e.mem.u32(desc + 8) == usage
            && e.mem.u32(desc + 0xc) == pool
            && e.mem.u32(desc + 0x10) >= size;
        if !fits {
            e.vcall(buffer, 8, &args![]);
        }
        fits
    })
}

// Translated from 00e8f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9IndexBufferManager`'s point index buffer
/// (its message names `ResizeTempPointConnectivity`), `thiscall` with five words: the needed
/// count, the quad count, a refill byte, the usage and the pool. Without a device (+8) or a
/// needed count it answers 0. The kept buffer (+0xc) is used when its description fits
/// (`index_buffer_fits` with 12 bytes per needed quad): it is answered as it is unless the byte
/// asks for a refill; one that does not fit is released (a failed `GetDesc` keeps it). Else a
/// new buffer of 12 bytes per quad is created (`CreateIndexBuffer`: the size, the usage, 0x65,
/// the pool, the out word, 0; a failure reports the create message, a null buffer the
/// connectivity message and answers 0). The buffer is locked (`Lock` 0, size, out word, 0;
/// failing, the lock message, `Unlock`, release, +0xc cleared, +0x10 the size and 0 answered)
/// and filled with two triangles (0, 1, 2, 0, 2, 3) per quad, unlocked and kept at +0xc with
/// its size at +0x10. Answers the buffer.
///
/// The exe's out words are the stack words of the refill byte (create) and of the needed count
/// (lock).
pub fn fn_00e8f360(
    e: &mut Engine,
    this: Ptr,
    needed: u32,
    quads: u32,
    refill: u8,
    usage: u32,
    pool: u32,
) -> u32 {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(8)) == 0 || needed == 0 {
        return 0;
    }
    let size = quads.wrapping_mul(12);
    let kept = e.mem.u32(base.wrapping_add(0xc));
    let mut buffer = 0u32;
    if kept != 0 {
        let fits = index_buffer_fits(e, kept, usage, pool, needed.wrapping_mul(12));
        if fits {
            if refill == 0 {
                return kept;
            }
            buffer = kept;
        }
    }
    e.with_stack(4, |e, out| {
        let out = out.addr();
        if buffer == 0 {
            let device = e.mem.u32(base.wrapping_add(8));
            e.mem.set_u32(out, 0);
            let result = e
                .vcall(
                    device,
                    CREATE_INDEX_BUFFER,
                    &args![size, usage, FORMAT_INDEX16, pool, out, 0u32],
                )
                .i32();
            if result < 0 {
                e.call(REPORT_00A29680, &args![CREATE_INDEX_BUFFER_FAILED_010F0184]);
                e.mem.set_u32(out, 0);
                buffer = 0;
            } else {
                buffer = e.mem.u32(out);
            }
            if buffer == 0 {
                e.call(REPORT_00A29680, &args![POINT_CONNECTIVITY_FAILED_010F0200]);
                return 0;
            }
        }
        e.mem.set_u32(out, 0);
        let result = e
            .vcall(buffer, INDEX_BUFFER_LOCK, &args![0u32, size, out, 0u32])
            .i32();
        if result < 0 {
            e.call(REPORT_00A29680, &args![PACK_LOCK_FAILED_010EDDD0]);
            e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
            e.vcall(buffer, 8, &args![]);
            e.mem.set_u32(base.wrapping_add(0xc), 0);
            e.mem.set_u32(base.wrapping_add(0x10), size);
            return 0;
        }
        let mut at = e.mem.u32(out);
        let mut first = 0u32;
        for _ in 0..quads {
            let index = first & 0xffff;
            for value in [index, index + 1, index + 2, index, index + 2, index + 3] {
                e.mem.set_u16(at, value as u16);
                at = at.wrapping_add(2);
            }
            first = first.wrapping_add(4);
        }
        e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
        e.mem.set_u32(base.wrapping_add(0xc), buffer);
        e.mem.set_u32(base.wrapping_add(0x10), size);
        buffer
    })
}

// Translated from 00e8f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9IndexBufferManager`'s line index buffer
/// (its message names `ResizeTempLineConnectivity`), `thiscall` with five words: the vertex
/// count, the connection flags (a byte per vertex), the address of the index count, the usage
/// and the pool. Without a device (+8), a count or flags it answers 0. The size is 4 bytes per
/// vertex plus 4; the kept buffer (+0x14) is used when its description fits, else (released
/// unless `GetDesc` failed) a new one comes from `00e8f190` (the size, the usage, 0x65, the
/// pool, no description; none: the connectivity message and 0). The index count is cleared,
/// the buffer locked (failing: the lock message, `Unlock`, release and a null buffer) and for
/// each vertex but the last whose flag is set the pair (i, i + 1) written and the count
/// raised by 2; a set last flag closes the line with (last, 0). Unlocked; the buffer (or null)
/// is kept at +0x14 with the size at +0x18 and answered.
pub fn fn_00e8f520(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    flags: u32,
    indices: Ptr,
    usage: u32,
    pool: u32,
) -> u32 {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(8)) == 0 || count == 0 || flags == 0 {
        return 0;
    }
    let size = count.wrapping_mul(4).wrapping_add(4);
    let kept = e.mem.u32(base.wrapping_add(0x14));
    let mut buffer = 0u32;
    if kept != 0 {
        let fits = index_buffer_fits(e, kept, usage, pool, size);
        if fits {
            buffer = kept;
        }
    }
    if buffer == 0 {
        buffer = fn_00e8f190(e, this, size, usage, FORMAT_INDEX16, pool, 0);
        if buffer == 0 {
            e.call(REPORT_00A29680, &args![LINE_CONNECTIVITY_FAILED_010F0258]);
            return 0;
        }
    }
    let result = e.with_stack(4, |e, out| {
        let out = out.addr();
        e.mem.set_u32(indices.addr(), 0);
        e.mem.set_u32(out, 0);
        let locked = e
            .vcall(buffer, INDEX_BUFFER_LOCK, &args![0u32, size, out, 0u32])
            .i32();
        if locked < 0 {
            e.call(REPORT_00A29680, &args![PACK_LOCK_FAILED_010EDDD0]);
            e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
            e.vcall(buffer, 8, &args![]);
            return 0;
        }
        let mut at = e.mem.u32(out);
        let last = count.wrapping_sub(1);
        let mut flag = flags;
        let mut vertex = 0u32;
        let raise = |e: &mut Engine| {
            let value = e.mem.u32(indices.addr()).wrapping_add(2);
            e.mem.set_u32(indices.addr(), value);
        };
        while vertex < last {
            if e.mem.u8(flag) != 0 {
                let index = vertex & 0xffff;
                e.mem.set_u16(at, index as u16);
                e.mem.set_u16(at.wrapping_add(2), (index + 1) as u16);
                at = at.wrapping_add(4);
                raise(e);
            }
            flag = flag.wrapping_add(1);
            vertex += 1;
        }
        if e.mem.u8(flag) != 0 {
            e.mem.set_u16(at, last as u16);
            e.mem.set_u16(at.wrapping_add(2), 0);
            raise(e);
        }
        e.vcall(buffer, INDEX_BUFFER_UNLOCK, &args![]);
        buffer
    });
    e.mem.set_u32(base.wrapping_add(0x14), result);
    e.mem.set_u32(base.wrapping_add(0x18), size);
    result
}

/// `00b57f20` (`thiscall` on a pointer map: the iterator, the key's and the value's out
/// words): the map's `GetNext`.
const MAP_ITERATE_00B57F20: u32 = 0x00b5_7f20;

// Translated from 00e8f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): frees the index buffer records of the first
/// map (+0x1c) and the two kept buffers (`thiscall`). From the first non-null bucket each
/// item goes through `GetNext` (`00b57f20`); a record has five entries at +8 to +0x18, each
/// one not null releasing its +0x20 (slot 8) when set, cleaned up (`00e95b80`) and freed
/// (0x38 bytes); then the record is freed (0x1c bytes). Then the map's `00b601a0`, and the
/// kept buffers (+0xc, +0x14) released and cleared when set.
pub fn fn_00e8f6d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let map = base.wrapping_add(0x1c);
    let first = first_item_from(e, map, 0);
    e.with_stack(0xc, |e, words| {
        let iterator = words.addr();
        let key = iterator + 4;
        let value = iterator + 8;
        e.mem.set_u32(iterator, first);
        while e.mem.u32(iterator) != 0 {
            e.call(MAP_ITERATE_00B57F20, &args![map, iterator, key, value]);
            let record = e.mem.u32(value);
            if record == 0 {
                continue;
            }
            for slot in 0..5u32 {
                let entry = e.mem.u32(record.wrapping_add(8 + 4 * slot));
                if entry == 0 {
                    continue;
                }
                let held = e.mem.u32(entry.wrapping_add(0x20));
                if held != 0 {
                    e.vcall(held, 8, &args![]);
                }
                e.call(ENTRY_CLEAN_UP_00E95B80, &args![entry]);
                e.call(FREE_00AA1460, &args![entry, 0x38u32]);
            }
            e.call(FREE_00AA1460, &args![record, 0x1cu32]);
        }
    });
    e.call(MAP_00B601A0, &args![map]);
    for offset in [0xcu32, 0x14] {
        let buffer = e.mem.u32(base.wrapping_add(offset));
        if buffer != 0 {
            e.vcall(buffer, 8, &args![]);
            e.mem.set_u32(base.wrapping_add(offset), 0);
        }
    }
}

/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00e8f7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the scalar deleting destructor of an index
/// buffer map's base (`thiscall`, the flags): its table (`010f02b0`), `00b601a0`, the bucket
/// array (+8) freed (`00aa10f0`), and with bit 0 the block goes back to the memory manager
/// (`00aa4060`). Answers `this`.
pub fn fn_00e8f7c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, IB_MAP_BASE_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    }
    this
}

// Translated from 00e8f800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the scalar deleting destructor of an index
/// buffer map (`thiscall`, the flags): its table (`010f02d0`) and `00b601a0`, the base's table
/// (`010f02b0`) and `00b601a0` again, the bucket array (+8) freed (`00aa10f0`), and with bit 0
/// the block goes back to the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00e8f800(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, IB_MAP_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    e.mem.set_u32(base, IB_MAP_BASE_VTABLE);
    e.call(MAP_00B601A0, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, base]);
    }
    this
}

// Translated from 00e8fd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9RenderedTextureData` constructor (the
/// RTTI of its table), `thiscall` with three words, the last not read: the texture data base
/// constructor (`00e8a260`: the texture and the renderer), then its table (`010f02fc`).
/// Answers `this`.
pub fn fn_00e8fd60(e: &mut Engine, this: Ptr, texture: u32, renderer: u32, _unused_2: u32) -> Ptr {
    e.call(
        TEXTURE_DATA_CONSTRUCT_00E8A260,
        &args![this, texture, renderer],
    );
    e.mem
        .set_u32(this.addr(), NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
    this
}

// Translated from 00e8fd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9RenderedTextureData` destructor body
/// (`thiscall`). Its table; the renderer's (+0x60) virtual slot 0x15c with the texture (+8);
/// the surface at +0x6c released (slot 8) and cleared when set; then (a tail jump) the
/// texture data base destructor (`00e8a2b0`).
pub fn fn_00e8fd80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let renderer = e.mem.u32(base.wrapping_add(0x60));
    let texture = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(base, NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
    e.vcall(renderer, 0x15c, &args![texture]);
    let surface = e.mem.u32(base.wrapping_add(0x6c));
    if surface != 0 {
        e.vcall(surface, 8, &args![]);
        e.mem.set_u32(base.wrapping_add(0x6c), 0);
    }
    e.call(TEXTURE_DATA_DOWN_00E8A2B0, &args![base]);
}

/// The `NiDX9RenderedCubeMapData` (`010f049c`) and `NiDX9DynamicTextureData` (`010f05ec`)
/// tables (by their RTTI).
const NI_DX9_RENDERED_CUBE_MAP_DATA_VTABLE: u32 = 0x010f_049c;
const NI_DX9_DYNAMIC_TEXTURE_DATA_VTABLE: u32 = 0x010f_05ec;

// Translated from 00e90080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9RenderedCubeMapData` scalar deleting
/// destructor (`thiscall`, the flags): its table, the renderer's (+0x60) slot 0x158 with the
/// texture (+8), the rendered texture data destructor (`00e8fd80`), and with bit 0 the 0x70
/// bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00e90080(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let renderer = e.mem.u32(base.wrapping_add(0x60));
    let texture = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(base, NI_DX9_RENDERED_CUBE_MAP_DATA_VTABLE);
    e.vcall(renderer, 0x158, &args![texture]);
    fn_00e8fd80(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![base, 0x70u32]);
    }
    this
}

/// The two messages of the rendered cube map (`"NiDX9RenderedCubeMapData::CreateSurf> Failed
/// CreateImageSurface - %s"` and `"...::Create> NiDX9TextureBufferData::CreateCubeFace -
/// %s"`), the device's `CreateCubeTexture` (slot 0x64, by the interface's order), the cube
/// face maker `00e7d390` (cdecl, five words) and the render-target memory counters
/// (`0126f9e0` bytes, `0126f9e4` page padding).
const CREATE_SURF_FAILED_010F0550: u32 = 0x010f_0550;
const CREATE_CUBE_FACE_FAILED_010F0598: u32 = 0x010f_0598;
const CREATE_CUBE_TEXTURE: u32 = 0x64;
const MAKE_CUBE_FACE_00E7D390: u32 = 0x00e7_d390;
const TARGET_BYTES_0126F9E0: u32 = 0x0126_f9e0;
const TARGET_PADDING_0126F9E4: u32 = 0x0126_f9e4;

// Translated from 00e900c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9RenderedCubeMapData::CreateSurf` (its
/// message's name), `thiscall` with the texture (null answers 0). The texture's size (virtual
/// slot 0x94) goes to +0x10 and +0xc, +0x68 becomes 1; the pixel format is picked
/// (`00e89ed0`) for the texture's request (its words +0x18..+0x20) from the renderer's (+0x60)
/// table at +0x808; the Direct3D format is the format's +0xc, or the texture's +0x44 when its
/// byte +0x41 is set. The device (renderer +0x288) makes a cube texture (`CreateCubeTexture`:
/// size, 1 level, usage 1, the format, pool 0, the out word, 0): failing, the message with
/// `NiXenonErrorString`, +0x64 cleared and 0 answered; else the texture at +0x64 and the
/// pixel format answered.
///
/// The exe's out word is the stack word of the texture argument.
pub fn fn_00e900c0(e: &mut Engine, this: Ptr, texture: u32) -> u32 {
    let base = this.addr();
    if texture == 0 {
        return 0;
    }
    let size = e.vcall(texture, 0x94, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0x10), size);
    e.mem.set_u32(base.wrapping_add(0xc), size);
    e.mem.set_u32(base.wrapping_add(0x68), 1);
    e.with_stack(0x10, |e, words| {
        let request = words.addr();
        let out = request + 0xc;
        for word in 0..3u32 {
            let value = e.mem.u32(texture.wrapping_add(0x18 + 4 * word));
            e.mem.set_u32(request + 4 * word, value);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let format = e
            .call(
                PICK_FORMAT_00E89ED0,
                &args![request, renderer.wrapping_add(0x808)],
            )
            .u32();
        let mut d3d_format = e.mem.u32(format.wrapping_add(0xc));
        if e.mem.u8(texture.wrapping_add(0x41)) != 0 {
            d3d_format = e.mem.u32(texture.wrapping_add(0x44));
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
        let size = e.mem.u32(base.wrapping_add(0xc));
        e.mem.set_u32(out, 0);
        let result = e
            .vcall(
                device,
                CREATE_CUBE_TEXTURE,
                &args![size, 1u32, 1u32, d3d_format, 0u32, out, 0u32],
            )
            .i32();
        if result < 0 {
            let text = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(REPORT_00A29680, &args![CREATE_SURF_FAILED_010F0550, text]);
            e.mem.set_u32(base.wrapping_add(0x64), 0);
            return 0;
        }
        let cube = e.mem.u32(out);
        e.mem.set_u32(base.wrapping_add(0x64), cube);
        format
    })
}

/// Adds `bytes` to the render-target memory counter and the padding up to the next 4 KB page
/// to the padding counter.
fn count_target_memory(e: &mut Engine, bytes: u32) {
    let total = e.mem.u32(TARGET_BYTES_0126F9E0).wrapping_add(bytes);
    e.mem.set_u32(TARGET_BYTES_0126F9E0, total);
    let page = bytes & 0xffff_f000;
    let padding = if page == bytes {
        0
    } else {
        page.wrapping_sub(bytes).wrapping_add(0x1000)
    };
    let total = e.mem.u32(TARGET_PADDING_0126F9E4).wrapping_add(padding);
    e.mem.set_u32(TARGET_PADDING_0126F9E4, total);
}

// Translated from 00e901a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9RenderedCubeMapData::Create` (its
/// message's name), `cdecl` with the texture and the renderer. A 0x70-byte block (`00aa13e0`)
/// is constructed as rendered texture data (`00e8fd60` with 0) with the cube map's table; its
/// surface is created (`00e900c0`; failing, the block's deleting destructor, slot 0 with 1,
/// and 0 answered). The texture's renderer data (+0x24 of the texture at +8) becomes the block.
/// For each of the six faces (the texture's words from +0x4c) the block's slot 0x9c runs
/// twice (no arguments) and the face maker (`00e7d390`: the second's answer, the face, the
/// first face made, the device and the address of the face's word) makes it; failing, the
/// message, the deleting destructor and 0. The six faces' bytes (the pixel format's bits per
/// pixel over 8 times +0x10 times +0xc, six times) are counted as render-target memory.
/// Answers the block.
pub fn fn_00e901a0(e: &mut Engine, texture: u32, renderer: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x70u32]).u32();
    let data = if block != 0 {
        fn_00e8fd60(e, Ptr::new(block), texture, renderer, 0);
        e.mem.set_u32(block, NI_DX9_RENDERED_CUBE_MAP_DATA_VTABLE);
        block
    } else {
        0
    };
    let format = fn_00e900c0(e, Ptr::new(data), texture);
    if format == 0 {
        if data != 0 {
            e.vcall(data, 0, &args![1u32]);
        }
        return 0;
    }
    let owner = e.mem.u32(data.wrapping_add(8));
    e.mem.set_u32(owner.wrapping_add(0x24), data);
    let mut first = 0u32;
    let result = e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        for face in 0..6u32 {
            e.vcall(data, 0x9c, &args![]);
            let word = e.mem.u32(texture.wrapping_add(0x4c + 4 * face));
            let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
            e.mem.set_u32(slot, word);
            let answer = e.vcall(data, 0x9c, &args![]).u32();
            let made = e
                .call(
                    MAKE_CUBE_FACE_00E7D390,
                    &args![answer, face, first, device, slot],
                )
                .u32();
            if made == 0 {
                e.call(
                    REPORT_00A29680,
                    &args![
                        CREATE_CUBE_FACE_FAILED_010F0598,
                        BUFFER_DATA_FAILED_ARGUMENT_01011584
                    ],
                );
                e.vcall(data, 0, &args![1u32]);
                return false;
            }
            if first == 0 {
                first = made;
            }
        }
        true
    });
    if !result {
        return 0;
    }
    let bits = u32::from(e.mem.u8(format.wrapping_add(1)) >> 3);
    let bytes = bits
        .wrapping_mul(e.mem.u32(data.wrapping_add(0x10)))
        .wrapping_mul(e.mem.u32(data.wrapping_add(0xc)))
        .wrapping_mul(6);
    count_target_memory(e, bytes);
    data
}

// Translated from 00e902e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the byte at +0x70
/// (in `AL`).
pub fn fn_00e902e0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x70))
}

// Translated from 00e902f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with one word: its
/// low byte goes to +0x70.
pub fn fn_00e902f0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x70), value);
}

// Translated from 00e90300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9DynamicTextureData` scalar deleting
/// destructor (`thiscall`, the flags): its table, the renderer's (+0x60) slot 0x160 with the
/// texture (+8), the texture data base destructor (`00e8a2b0`), and with bit 0 the 0x74 bytes
/// freed (`00aa1460`). Answers `this`.
pub fn fn_00e90300(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let renderer = e.mem.u32(base.wrapping_add(0x60));
    let texture = e.mem.u32(base.wrapping_add(8));
    e.mem.set_u32(base, NI_DX9_DYNAMIC_TEXTURE_DATA_VTABLE);
    e.vcall(renderer, 0x160, &args![texture]);
    e.call(TEXTURE_DATA_DOWN_00E8A2B0, &args![base]);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![base, 0x74u32]);
    }
    this
}

/// The dynamic texture's message (`"NiDX9DynamicTextureData::CreateSurf> Failed
/// CreateImageSurface - %s"`).
const DYNAMIC_SURF_FAILED_010F06B0: u32 = 0x010f_06b0;

// Translated from 00e90340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9DynamicTextureData::CreateSurf` (its
/// message's name), `thiscall` with the texture (null answers 0) and a byte. The texture's
/// width and height (slots 0x94, 0x98) go to +0xc and +0x10, +0x68 becomes 1; the pixel format
/// is picked (`00e89ed0`) for the texture's request from the renderer's (+0x60) table at
/// +0x700 and its 0x44 bytes copied to +0x14. With the byte set the device (renderer +0x288)
/// makes one texture (`CreateTexture`: width, height, 1 level, usage 0x200, the format, pool 0)
/// kept at +0x64; without it one with usage 0 at +0x64 and a second in pool 2 at +0x6c. A
/// failure prints the message with `NiXenonErrorString`, clears the word it was making and
/// answers 0; else the pixel format is answered.
///
/// The exe's out word is the stack word of the texture argument.
pub fn fn_00e90340(e: &mut Engine, this: Ptr, texture: u32, dynamic: u8) -> u32 {
    let base = this.addr();
    if texture == 0 {
        return 0;
    }
    let width = e.vcall(texture, 0x94, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0xc), width);
    let height = e.vcall(texture, 0x98, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0x10), height);
    e.mem.set_u32(base.wrapping_add(0x68), 1);
    e.with_stack(0x10, |e, words| {
        let request = words.addr();
        let out = request + 0xc;
        for word in 0..3u32 {
            let value = e.mem.u32(texture.wrapping_add(0x18 + 4 * word));
            e.mem.set_u32(request + 4 * word, value);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let format = e
            .call(
                PICK_FORMAT_00E89ED0,
                &args![request, renderer.wrapping_add(0x700)],
            )
            .u32();
        for word in 0..0x11u32 {
            let value = e.mem.u32(format.wrapping_add(4 * word));
            e.mem.set_u32(base.wrapping_add(0x14 + 4 * word), value);
        }
        let renderer = e.mem.u32(base.wrapping_add(0x60));
        let d3d_format = e.mem.u32(format.wrapping_add(0xc));
        let device = e.mem.u32(renderer.wrapping_add(RENDERER_DEVICE));
        let create = |e: &mut Engine, usage: u32, pool: u32| {
            let width = e.mem.u32(base.wrapping_add(0xc));
            let height = e.mem.u32(base.wrapping_add(0x10));
            e.vcall(
                device,
                CREATE_TEXTURE,
                &args![width, height, 1u32, usage, d3d_format, pool, out, 0u32],
            )
            .i32()
        };
        let fail = |e: &mut Engine, result: i32, at: u32| {
            let text = e.call(ERROR_STRING_00E89D50, &args![result]).u32();
            e.call(REPORT_00A29680, &args![DYNAMIC_SURF_FAILED_010F06B0, text]);
            e.mem.set_u32(base.wrapping_add(at), 0);
        };
        e.mem.set_u32(out, 0);
        let result = create(e, if dynamic != 0 { 0x200 } else { 0 }, 0);
        if result < 0 {
            fail(e, result, 0x64);
            return 0;
        }
        let made = e.mem.u32(out);
        e.mem.set_u32(base.wrapping_add(0x64), made);
        if dynamic != 0 {
            return format;
        }
        e.mem.set_u32(out, 0);
        let result = create(e, 0, 2);
        if result < 0 {
            fail(e, result, 0x6c);
            return 0;
        }
        let made = e.mem.u32(out);
        e.mem.set_u32(base.wrapping_add(0x6c), made);
        format
    })
}

/// The tiled-texture message (`"NiDX9DynamicTextureData Failed> Can not create a tiled dynamic
/// texture.\n"`).
const TILED_DYNAMIC_FAILED_010F06F8: u32 = 0x010f_06f8;

// Translated from 00e904c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9DynamicTextureData::Create` (its class by
/// the RTTI and message), `cdecl` with the texture. A tiled texture (byte +0x38) prints the
/// message and answers 0. Otherwise a 0x74-byte block is constructed as texture data
/// (`00e8a260` with the renderer singleton `011f4748`) with the class's table, +0x6c and the
/// byte +0x70 cleared; its surfaces are created (`00e90340`, with the renderer's byte +0x6f5);
/// failing, the block's deleting destructor (slot 0 with 1) and 0. Else the texture's renderer
/// data (+0x24 of the texture at +8) becomes the block, which is answered.
///
/// The exe stores the byte into the low byte of the texture's stack word and pushes that whole
/// word; `00e90340` reads only the byte, which the direct call passes.
pub fn fn_00e904c0(e: &mut Engine, texture: u32) -> u32 {
    if e.mem.u8(texture.wrapping_add(0x38)) != 0 {
        e.call(REPORT_00A29680, &args![TILED_DYNAMIC_FAILED_010F06F8]);
        return 0;
    }
    let renderer = e.mem.u32(RENDERER_011F4748);
    let block = e.call(NEW_OBJECT, &args![0x74u32]).u32();
    let data = if block != 0 {
        e.call(
            TEXTURE_DATA_CONSTRUCT_00E8A260,
            &args![block, texture, renderer],
        );
        e.mem.set_u32(block, NI_DX9_DYNAMIC_TEXTURE_DATA_VTABLE);
        e.mem.set_u32(block.wrapping_add(0x6c), 0);
        e.mem.set_u8(block.wrapping_add(0x70), 0);
        block
    } else {
        0
    };
    let flag = e.mem.u8(renderer.wrapping_add(0x6f5));
    if fn_00e90340(e, Ptr::new(data), texture, flag) == 0 {
        if data != 0 {
            e.vcall(data, 0, &args![1u32]);
        }
        return 0;
    }
    let owner = e.mem.u32(data.wrapping_add(8));
    e.mem.set_u32(owner.wrapping_add(0x24), data);
    data
}

/// `NiDX9ErrorShader::ms_RTTI` (`0126f9f0`; the function sits in slot +8 of the class's table
/// `010f0764`).
const ERROR_SHADER_RTTI_0126F9F0: u32 = 0x0126_f9f0;

// Translated from 00e90550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): `NiDX9ErrorShader`'s `GetRTTI` (slot +8 of its
/// table, the only reference): answers `0126f9f0`.
pub fn fn_00e90550(_e: &mut Engine, _this: Ptr) -> u32 {
    ERROR_SHADER_RTTI_0126F9F0
}

/// `rand` (`00ecadb8`), the `double`s -32767.0 (`0109ed88`) and 127.0 (`0102dfe0`), the
/// render state group maker `00e7f5d0` (no arguments), the group's state set `00e7f430`
/// (`thiscall`, three words) and `NiShader`'s base render `00a8f3e0` (`thiscall`, seven
/// words).
const RAND: u32 = 0x00ec_adb8;
const MINUS_32767_0109ED88: u32 = 0x0109_ed88;
const SCALE_127_0102DFE0: u32 = 0x0102_dfe0;
const MAKE_STATE_GROUP_00E7F5D0: u32 = 0x00e7_f5d0;
const GROUP_SET_STATE_00E7F430: u32 = 0x00e7_f430;
const SHADER_RENDER_00A8F3E0: u32 = 0x00a8_f3e0;

/// One channel of the error shader's colour: `rand()` (`FILD`) over -32767.0, rounded to a
/// `float`, times 127.0, truncated (`FISTP` under chop); 0x80 minus its low byte.
fn random_channel(e: &mut Engine) -> u8 {
    let value = e.call(RAND, &args![]).i32();
    let divisor = f64::from_bits(e.mem.u64(MINUS_32767_0109ED88));
    let scale = f64::from_bits(e.mem.u64(SCALE_127_0102DFE0));
    let ratio = x87_store32(x87_div(f64::from(value), divisor));
    let scaled = x87_mul(x87_load32(ratio), scale);
    0x80u8.wrapping_sub(crate::x87::fistp_chop(scaled) as u8)
}

// Translated from 00e90590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9ErrorShader`'s render (`thiscall`,
/// seven words passed on). A random colour (three `random_channel`s, alpha 0xff) goes to the
/// property state's (+0x3c) render state group (+0x30, made by `00e7f5d0` when missing) as
/// state 0x3c; states 0x1d, 0x1b, 0xf and 0x34 get 0 (each set with 1, the group made again
/// when missing). Then (a tail jump) `NiShader`'s base render (`00a8f3e0`) with the seven
/// words, whose answer it answers.
pub fn fn_00e90590(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
) -> u32 {
    let base = this.addr();
    let red = random_channel(e);
    let green = random_channel(e);
    let blue = random_channel(e);
    let colour = (((u32::from(red) | 0xffff_ff00) << 8) | u32::from(green)) << 8 | u32::from(blue);
    for (state, value) in [(0x3cu32, colour), (0x1d, 0), (0x1b, 0), (0xf, 0), (0x34, 0)] {
        let holder = e.mem.u32(base.wrapping_add(0x3c));
        if e.mem.u32(holder.wrapping_add(0x30)) == 0 {
            let group = e.call(MAKE_STATE_GROUP_00E7F5D0, &args![]).u32();
            e.mem.set_u32(holder.wrapping_add(0x30), group);
        }
        let group = e.mem.u32(holder.wrapping_add(0x30));
        e.call(GROUP_SET_STATE_00E7F430, &args![group, state, value, 1u32]);
    }
    e.call(
        SHADER_RENDER_00A8F3E0,
        &args![base, first, second, third, fourth, fifth, sixth, seventh],
    )
    .u32()
}

/// The `double`s 0.1 (`0101ffa0`), 32767.0 (`01029790`) and 1.0 (`01012070`) and the pass
/// render `00e807d0` (`thiscall`, eight words).
const TENTH_0101FFA0: u32 = 0x0101_ffa0;
const RAND_MAX_01029790: u32 = 0x0102_9790;
const ONE_01012070: u32 = 0x0101_2070;
const RENDER_PASS_00E807D0: u32 = 0x00e8_07d0;

// Translated from 00e90760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`): the `NiDX9ErrorShader`'s jittered draw
/// (`thiscall`, eight words: six passed through, the transform and the bound; the second and
/// third are not read). The transform (13 words) is copied; each of its translation's three
/// floats (+0x24, +0x28, +0x2c) moves by `(2 * rand() / 32767.0 - 1.0)` (rounded to a
/// `float`) times a tenth of the bound's radius (+0xc, rounded to a `float`). Then `00e807d0`
/// with the first word, 0, 0, the fourth to sixth, the copy and the bound, whose `EAX` it
/// answers (the exe ends on that call).
/// The slot is `NiShader`'s `+0x60`, which the Xbox PDB names `SetupTransformations`
/// (`unsigned int`, eight words).
pub fn fn_00e90760(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    _unused_1: u32,
    _unused_2: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    transform: u32,
    bound: u32,
) -> u32 {
    e.with_stack(0x34, |e, copy| {
        let copy = copy.addr();
        for word in 0..0xdu32 {
            let value = e.mem.u32(transform.wrapping_add(4 * word));
            e.mem.set_u32(copy + 4 * word, value);
        }
        let tenth = f64::from_bits(e.mem.u64(TENTH_0101FFA0));
        let radius = x87_load32(e.mem.u32(bound.wrapping_add(0xc)));
        let scale = x87_store32(x87_mul(radius, tenth));
        let rand_max = f64::from_bits(e.mem.u64(RAND_MAX_01029790));
        let one = f64::from_bits(e.mem.u64(ONE_01012070));
        for axis in 0..3u32 {
            let value = f64::from(e.call(RAND, &args![]).i32());
            let ratio = x87_sub(x87_div(x87_add(value, value), rand_max), one);
            let ratio = x87_load32(x87_store32(ratio));
            let at = copy + 0x24 + 4 * axis;
            let moved = x87_add(x87_mul(ratio, x87_m32(scale)), x87_m32(e.mem.u32(at)));
            e.mem.set_u32(at, x87_store32(moved));
        }
        e.call(
            RENDER_PASS_00E807D0,
            &args![this, first, 0u32, 0u32, fourth, fifth, sixth, copy, bound],
        )
        .u32()
    })
}

// Translated from 00e90d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; the texture manager's), `thiscall` with a
/// texture: the format word (+0x24) of its renderer data (+0x24) when it has one. Otherwise,
/// when the texture has a source (`00653290` with the kind at `011f444c`) that the renderer's
/// (+0xc) slot 0x13c accepts, the format of the renderer data the texture has by then; else
/// 0x16.
pub fn fn_00e90d20(e: &mut Engine, this: Ptr, texture: u32) -> u32 {
    let data = e.mem.u32(texture.wrapping_add(0x24));
    if data != 0 {
        return e.mem.u32(data.wrapping_add(0x24));
    }
    let source = e
        .call(SOURCE_OF_00653290, &args![texture, SOURCE_KIND_011F444C])
        .u32();
    if source != 0 {
        let renderer = e.mem.u32(this.addr().wrapping_add(0xc));
        if e.vcall(renderer, 0x13c, &args![source]).u8() != 0 {
            let data = e.mem.u32(texture.wrapping_add(0x24));
            if data != 0 {
                return e.mem.u32(data.wrapping_add(0x24));
            }
        }
    }
    0x16
}

// Translated from 00e90db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot of `NiDX9RenderState`),
/// `thiscall`: the current vertex format (+0x1104) and declaration (+0x110c) become the kept
/// ones (+0x1108, +0x1110).
pub fn fn_00e90db0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let format = e.mem.u32(base.wrapping_add(0x1104));
    let declaration = e.mem.u32(base.wrapping_add(0x110c));
    e.mem.set_u32(base.wrapping_add(0x1108), format);
    e.mem.set_u32(base.wrapping_add(0x1110), declaration);
}

// Translated from 00e90dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: its own slots 0xa4
/// and then 0xb4 (a tail jump), without arguments.
pub fn fn_00e90dd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base, 0xa4, &args![]);
    e.vcall(base, 0xb4, &args![]);
}

// Translated from 00e90df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the byte at +0x1114
/// (in `AL`).
pub fn fn_00e90df0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x1114))
}

/// The device's `SetFVF` (slot 0x164, by the interface's order).
const SET_FVF: u32 = 0x164;

// Translated from 00e90ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with a vertex format
/// and a byte: a zero format does nothing; nor does the current one (+0x1104) while no
/// declaration is set (byte +0x1100 clear). Otherwise the byte +0x1100 is cleared, with the
/// byte argument the current format is kept at +0x1108, the format becomes current and goes to
/// the device's `SetFVF` (a tail jump on the same stack words).
pub fn fn_00e90ec0(e: &mut Engine, this: Ptr, format: u32, keep: u8) {
    let base = this.addr();
    if format == 0 {
        return;
    }
    if e.mem.u8(base.wrapping_add(0x1100)) == 0 && e.mem.u32(base.wrapping_add(0x1104)) == format {
        return;
    }
    e.mem.set_u8(base.wrapping_add(0x1100), 0);
    if keep != 0 {
        let current = e.mem.u32(base.wrapping_add(0x1104));
        e.mem.set_u32(base.wrapping_add(0x1108), current);
    }
    e.mem.set_u32(base.wrapping_add(0x1104), format);
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.vcall(device, SET_FVF, &args![format]);
}

// Translated from 00e90f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: the current vertex
/// format (+0x1104), or 0 while a declaration is set (byte +0x1100).
pub fn fn_00e90f20(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u8(base.wrapping_add(0x1100)) != 0 {
        return 0;
    }
    e.mem.u32(base.wrapping_add(0x1104))
}

// Translated from 00e90f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: its own slot 0x9c
/// with the kept vertex format (+0x1108) and 0.
pub fn fn_00e90f40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let kept = e.mem.u32(base.wrapping_add(0x1108));
    e.vcall(base, 0x9c, &args![kept, 0u32]);
}

// Translated from 00e90f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with a vertex format
/// that goes away: when it is not zero and no declaration is set (byte +0x1100 clear), a
/// current format (+0x1104) equal to it becomes 2 and goes to the device's `SetFVF`, and a kept
/// one (+0x1108) equal to it becomes 2.
pub fn fn_00e90f60(e: &mut Engine, this: Ptr, format: u32) {
    let base = this.addr();
    if format == 0 || e.mem.u8(base.wrapping_add(0x1100)) != 0 {
        return;
    }
    if e.mem.u32(base.wrapping_add(0x1104)) == format {
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.mem.set_u32(base.wrapping_add(0x1104), 2);
        e.vcall(device, SET_FVF, &args![2u32]);
    }
    if e.mem.u32(base.wrapping_add(0x1108)) == format {
        e.mem.set_u32(base.wrapping_add(0x1108), 2);
    }
}

// Translated from 00e91040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall`: its own slot 0xac
/// with the kept vertex declaration (+0x1110) and 0.
pub fn fn_00e91040(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let kept = e.mem.u32(base.wrapping_add(0x1110));
    e.vcall(base, 0xac, &args![kept, 0u32]);
}

// Translated from 00e910f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with a sampler and a
/// state: the cached sampler state (+0xe20, 8 bytes per slot, 5 slots per sampler) when the
/// state's slot (`0126f92c`) is below 5, else 0.
pub fn fn_00e910f0(e: &mut Engine, this: Ptr, sampler: u32, state: u32) -> u32 {
    let slot = u32::from(
        e.mem
            .u16(SAMPLER_STATE_SLOTS_0126F92C.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 5 {
        return 0;
    }
    let index = sampler.wrapping_mul(5).wrapping_add(slot);
    e.mem.u32(
        this.addr()
            .wrapping_add(0xe20)
            .wrapping_add(index.wrapping_mul(8)),
    )
}

// Translated from 00e91120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with a sampler and a
/// state: when the state's slot (`0126f92c`) is below 5, its own slot 0xcc with the sampler,
/// the state, the state's second cached word (+0xe24) and 0.
pub fn fn_00e91120(e: &mut Engine, this: Ptr, sampler: u32, state: u32) {
    let slot = u32::from(
        e.mem
            .u16(SAMPLER_STATE_SLOTS_0126F92C.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 5 {
        return;
    }
    let base = this.addr();
    let index = sampler.wrapping_mul(5).wrapping_add(slot);
    let value = e
        .mem
        .u32(base.wrapping_add(0xe24).wrapping_add(index.wrapping_mul(8)));
    e.vcall(base, 0xcc, &args![sampler, state, value, 0u32]);
}

// Translated from 00e91190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8f190`; a virtual slot), `thiscall` with a byte: when it
/// differs from +0x1114 and the renderer's (+0x10fc) byte +0x5d5 is set, it is stored, goes to
/// the device's slot 0x134 (by the interface's order `SetSoftwareVertexProcessing`) and the
/// current vertex shader (+0x10e0) is set again through its own slot 0x98.
pub fn fn_00e91190(e: &mut Engine, this: Ptr, value: u8) {
    let base = this.addr();
    if value == e.mem.u8(base.wrapping_add(0x1114)) {
        return;
    }
    let renderer = e.mem.u32(base.wrapping_add(0x10fc));
    if e.mem.u8(renderer.wrapping_add(0x5d5)) == 0 {
        return;
    }
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.mem.set_u8(base.wrapping_add(0x1114), value);
    e.vcall(device, 0x134, &args![u32::from(value)]);
    let shader = e.mem.u32(base.wrapping_add(0x10e0));
    e.vcall(base, 0x98, &args![shader]);
}

/// The renderer data's base constructor `00a981d0` (`thiscall`) and the
/// `NiDX9PersistentSrcTextureRendererData` table (`010f099c`, by its RTTI).
const RENDERER_DATA_CONSTRUCT_00A981D0: u32 = 0x00a9_81d0;
const PERSISTENT_SRC_TEXTURE_RENDERER_DATA_VTABLE: u32 = 0x010f_099c;

// Translated from 00e916d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function of it; read from the bytes): the
/// `NiDX9PersistentSrcTextureRendererData` stream loader (registered by `00e76210`), `cdecl`,
/// no arguments: a 0x94-byte block (`00aa13e0`) constructed by `00a981d0` with the class's
/// table, or 0 when the allocation fails.
pub fn fn_00e916d0(e: &mut Engine) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x94u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(RENDERER_DATA_CONSTRUCT_00A981D0, &args![block]);
    e.mem
        .set_u32(block, PERSISTENT_SRC_TEXTURE_RENDERER_DATA_VTABLE);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_f360, fn_00e8f360(Ptr, u32, u32, u8, u32, u32) -> u32),
        entry!(0x00e8_f520, fn_00e8f520(Ptr, u32, u32, Ptr, u32, u32) -> u32),
        entry!(0x00e8_f6d0, fn_00e8f6d0(Ptr)),
        entry!(0x00e8_f7c0, fn_00e8f7c0(Ptr, u32) -> Ptr),
        entry!(0x00e8_f800, fn_00e8f800(Ptr, u32) -> Ptr),
        entry!(0x00e8_fd60, fn_00e8fd60(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00e8_fd80, fn_00e8fd80(Ptr)),
        entry!(0x00e9_0080, fn_00e90080(Ptr, u32) -> Ptr),
        entry!(0x00e9_00c0, fn_00e900c0(Ptr, u32) -> u32),
        entry!(0x00e9_01a0, fn_00e901a0(u32, u32) -> u32),
        entry!(0x00e9_02e0, fn_00e902e0(Ptr) -> u8),
        entry!(0x00e9_02f0, fn_00e902f0(Ptr, u8)),
        entry!(0x00e9_0300, fn_00e90300(Ptr, u32) -> Ptr),
        entry!(0x00e9_0340, fn_00e90340(Ptr, u32, u8) -> u32),
        entry!(0x00e9_04c0, fn_00e904c0(u32) -> u32),
        entry!(0x00e9_0550, fn_00e90550(Ptr) -> u32),
        entry!(
            0x00e9_0590,
            fn_00e90590(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e9_0760,
            fn_00e90760(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e9_0d20, fn_00e90d20(Ptr, u32) -> u32),
        entry!(0x00e9_0db0, fn_00e90db0(Ptr)),
        entry!(0x00e9_0dd0, fn_00e90dd0(Ptr)),
        entry!(0x00e9_0df0, fn_00e90df0(Ptr) -> u8),
        entry!(0x00e9_0ec0, fn_00e90ec0(Ptr, u32, u8)),
        entry!(0x00e9_0f20, fn_00e90f20(Ptr) -> u32),
        entry!(0x00e9_0f40, fn_00e90f40(Ptr)),
        entry!(0x00e9_0f60, fn_00e90f60(Ptr, u32)),
        entry!(0x00e9_1040, fn_00e91040(Ptr)),
        entry!(0x00e9_10f0, fn_00e910f0(Ptr, u32, u32) -> u32),
        entry!(0x00e9_1120, fn_00e91120(Ptr, u32, u32)),
        entry!(0x00e9_1190, fn_00e91190(Ptr, u8)),
        entry!(0x00e9_16d0, fn_00e916d0() -> u32),
        entry!(0x00e9_0850, fn_00e90850(Ptr) -> u8),
        entry!(0x00e8_fb20, fn_00e8fb20(Ptr, u32) -> Ptr),
        entry!(0x00e9_0a80, fn_00e90a80(Ptr, u32) -> Ptr),
        entry!(0x00e9_09e0, fn_00e909e0(Ptr) -> Ptr),
        entry!(0x00e9_1590, fn_00e91590(u32, u32, u8) -> u32),
        entry!(0x00e8_f190, fn_00e8f190(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_f200, fn_00e8f200(Ptr, u32, u32, u32, u32, Ptr, u32, u32) -> u32),
        entry!(0x00e8_f840, fn_00e8f840(Ptr)),
        entry!(0x00e8_fa80, fn_00e8fa80(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e8_fc30, fn_00e8fc30(Ptr, u32) -> Ptr),
        entry!(0x00e8_fc50, fn_00e8fc50(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00e8_fdc0, fn_00e8fdc0(Ptr, u32) -> Ptr),
        entry!(0x00e8_fe10, fn_00e8fe10(Ptr, u32, u32) -> u32),
        entry!(0x00e8_ff60, fn_00e8ff60(u32, u32, u32) -> u32),
        entry!(0x00e9_02d0, fn_00e902d0(Ptr) -> u32),
        entry!(0x00e9_0560, fn_00e90560(Ptr, u32) -> Ptr),
        entry!(0x00e9_0970, fn_00e90970(Ptr) -> u8),
        entry!(0x00e9_0a50, fn_00e90a50(Ptr) -> bool),
        entry!(0x00e9_0ac0, fn_00e90ac0(Ptr, u32) -> Ptr),
        entry!(0x00e9_0b10, fn_00e90b10(Ptr, u32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e9_0c70, fn_00e90c70(Ptr, u32, u32, u32) -> bool),
        entry!(0x00e9_0d80, fn_00e90d80(Ptr)),
        entry!(0x00e9_0e00, fn_00e90e00(Ptr, u32, u8)),
        entry!(0x00e9_0e40, fn_00e90e40(Ptr) -> u32),
        entry!(0x00e9_0e50, fn_00e90e50(Ptr)),
        entry!(0x00e9_0e70, fn_00e90e70(Ptr, u32)),
        entry!(0x00e9_0fc0, fn_00e90fc0(Ptr, u32, u8)),
        entry!(0x00e9_1020, fn_00e91020(Ptr) -> u32),
        entry!(0x00e9_1060, fn_00e91060(Ptr, u32)),
        entry!(0x00e9_10a0, fn_00e910a0(Ptr, u32, u32, u32, u32)),
        entry!(0x00e9_1160, fn_00e91160(Ptr, u32) -> Ptr),
        entry!(0x00e9_11e0, fn_00e911e0(Ptr)),
        entry!(0x00e9_14e0, fn_00e914e0(Ptr)),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    const OBJECT_VTABLE: u32 = 0x6000;
    const ADD_REF: u32 = 0x6100;
    const DELETE: u32 = 0x6104;
    const STATE_SLOT_104: u32 = 0x6200;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose virtual table has `AddRef` at slot 4, delete at slot 4 for released
    /// objects (same slot), and the render state's slot 0x104.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x42];
        slots[1] = ADD_REF;
        slots[0x41] = STATE_SLOT_104;
        e.put_vtable(OBJECT_VTABLE, &slots);
        for no_op in [
            ADD_REF,
            DELETE,
            STATE_SLOT_104,
            IMPORT_INTERLOCKED_INCREMENT,
            MEMSET,
            MAP_00B601A0,
            SHADER_CONSTRUCT_00E818A0,
            ERROR_SHADER_SET_UP_00E90850,
            SET_NAME_00A5B950,
            MEMCPY,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, OBJECT_VTABLE);
        e.call_log = Some(vec![]);
        (e, device)
    }

    #[test]
    fn fn_00e8fb20_builds_the_index_buffer_manager() {
        let (mut e, device) = rig();
        let this = e.mem.alloc(0x50);
        e.call(0x00e8_fb20, &args![this, device]);
        assert_eq!(e.mem.u32(this), NI_DX9_INDEX_BUFFER_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        for offset in [0x1c, 0x2c, 0x3c] {
            assert_eq!(e.mem.u32(this + offset), IB_MAP_VTABLE);
        }
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
        assert_eq!(calls(&e, MAP_00B601A0).len(), 3);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT),
            vec![vec![OBJECT_COUNT_011F4400]]
        );
    }

    #[test]
    fn fn_00e90a80_keeps_the_renderer_and_its_device() {
        let (mut e, device) = rig();
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00e9_0a80, &args![this, renderer]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_TEXTURE_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(e.mem.u32(this + 0xc), renderer);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
    }

    #[test]
    fn fn_00e909e0_names_the_error_shader() {
        let (mut e, _) = rig();
        e.map(FIXED_STRING_EMPTY, 4);
        e.register(FIXED_STRING_MAKE, |_, _| 0x7008u32.into_ret());
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e9_09e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_ERROR_SHADER_VTABLE);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(
            calls(&e, FIXED_STRING_MAKE),
            vec![vec![ERROR_SHADER_NAME_010F0854]]
        );
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT), vec![vec![0x7000]]);
        // The empty string is not released.
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x7008);
        e.call_log = Some(vec![]);
        e.call(0x00e9_09e0, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00e91590_makes_the_render_state() {
        let (mut e, _) = rig();
        let mut slots = vec![0u32; 0x42];
        slots[0x41] = STATE_SLOT_104;
        e.put_vtable(NI_DX9_RENDER_STATE_VTABLE, &slots);
        e.map(STATE_PAIRS_011BFCE8, 0x20);
        for (i, (id, value)) in [(3u32, 9u32), (7, 9), (0xffff_ffff, 0), (7, 9)]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32, *id);
            e.mem
                .set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32 + 4, *value);
        }
        e.register(RENDER_STATE_CONSTRUCT_00E881A0, |e, a| {
            e.mem.set_u32(a[0], OBJECT_VTABLE);
            a[0].into_ret()
        });
        let made = e.mem.alloc(8);
        e.register_double(MAKE_00E978A0, move |_, _| made.into_ret());
        let state = e
            .call(0x00e9_1590, &args![0x4000u32, 0x5000u32, 1u32])
            .u32();
        assert_eq!(e.mem.u32(state), NI_DX9_RENDER_STATE_VTABLE);
        assert_eq!(calls(&e, MEMCPY), vec![vec![state + 0x1118, 0x5000, 0x130]]);
        assert_eq!(calls(&e, STATE_SLOT_104).len(), 1);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0xc), 1);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0x1c), 9);
        assert_eq!(e.mem.u32(state + 8) & 2, 2);
        assert_eq!(e.mem.u32(state + 0x10f0), made);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_INCREMENT).last(),
            Some(&vec![made + 4])
        );
        // Without the byte, state 7 becomes 0.
        e.call(0x00e9_1590, &args![0x4000u32, 0x5000u32, 0u32]);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 0xc), 0);
    }
}

#[cfg(test)]
mod b000w_tests {
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

    #[test]
    fn fn_00e90850_sets_up_the_error_pass() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x14, 0x1414);
        e.mem.set_u32(pass + 0x64, 2);
        let group = e.mem.alloc(0x70);
        e.mem.set_u32(group + 0x10, 0x1010);
        e.mem.set_u32(group + 0x60, 1);
        e.register_double(MAKE_PASS_00E804D0, move |e, a| {
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register_double(MAKE_GROUP_00E7EDF0, move |e, a| {
            e.mem.set_u32(a[0], group);
            Ret::default()
        });
        for no_op in [
            SET_STATE_00E88FC0,
            SET_SAMPLER_STATE_00E890C0,
            ATTACH_00E802B0,
            RELEASE_GROUP_00E7ECD0,
            RELEASE_PASS_00E80190,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e9_0850, &args![this]).u8(), 1);
        assert_eq!(e.mem.u32(this + 0x3c), pass);
        assert_eq!(e.mem.u32(ERROR_PASS_0126F74C), pass);
        let states = calls(&e, SET_STATE_00E88FC0);
        assert_eq!(states.len(), 6);
        assert_eq!(states[0], vec![0x1010, 1, 2, 0]);
        assert_eq!(states[5], vec![0x1010, 0x18, 0, 0]);
        assert_eq!(calls(&e, SET_SAMPLER_STATE_00E890C0).len(), 5);
        assert_eq!(calls(&e, ATTACH_00E802B0), vec![vec![pass, 0x1414, group]]);
        // The group's count reaches zero, the pass's does not.
        assert_eq!(calls(&e, RELEASE_GROUP_00E7ECD0), vec![vec![group]]);
        assert!(calls(&e, RELEASE_PASS_00E80190).is_empty());
        assert_eq!(e.mem.u32(pass + 0x64), 1);
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

    /// An object of `size` bytes whose table at `vtable` has `slots` (byte offset, target).
    fn object(e: &mut Engine, vtable: u32, size: u32, slots: &[(u32, u32)]) -> u32 {
        let mut words = vec![0u32; 0x80];
        for (slot, target) in slots {
            words[(*slot / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &words);
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Doubles answering 0 at `0x7000 + slot` for every slot of the tables these tests make.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for slot in (0..0x400u32).step_by(4) {
            e.register(0x7000 + slot, |_, _| Ret::default());
        }
        for no_op in [
            REPORT_00A29680,
            MEMCPY,
            ENTRY_CLEAN_UP_00E95B80,
            FREE_00AA1460,
            FREE_00AA10F0,
            MAP_00B601A0,
            MAP_SET_00B57EA0,
            ENTRY_CONSTRUCT_00E95B60,
            TEXTURE_DATA_DOWN_00E8A2B0,
            TEXTURE_DATA_CONSTRUCT_00E8A260,
            SHADER_DOWN_00E806C0,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            RESET_SHADERS_00E87A50,
            RENDER_STATE_DOWN_00E88B00,
            MEMBER_RESET_00E94130,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ERROR_STRING_00E89D50, |_, _| 0x5151u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A device whose `CreateIndexBuffer` hands out `buffer` (or fails when it is 0).
    fn index_device(e: &mut Engine, buffer: u32) -> u32 {
        e.register_double(0x786c, move |e, a| {
            e.mem.set_u32(a[5], buffer);
            (if buffer != 0 { 0u32 } else { 0x8000_0000 }).into_ret()
        });
        object(
            e,
            0x8000,
            8,
            &[
                (CREATE_INDEX_BUFFER, 0x786c),
                (8, 0x7008),
                (SET_INDICES, 0x71a0),
            ],
        )
    }

    #[test]
    fn fn_00e8f190_creates_an_index_buffer() {
        let mut e = engine();
        let buffer = object(&mut e, 0x8200, 8, &[(INDEX_BUFFER_GET_DESC, 0x7034)]);
        let device = index_device(&mut e, buffer);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 8, device);
        assert_eq!(
            e.call(
                0x00e8_f190,
                &args![this, 0x40u32, 8u32, 0x65u32, 1u32, 0u32]
            )
            .u32(),
            buffer
        );
        let create = &calls(&e, 0x786c)[0];
        assert_eq!(create[..5], [device, 0x40, 8, 0x65, 1]);
        assert_eq!(create[6], 0);
        assert!(calls(&e, 0x7034).is_empty());
        e.call(
            0x00e8_f190,
            &args![this, 0x40u32, 8u32, 0x65u32, 1u32, 0x9000u32],
        );
        assert_eq!(calls(&e, 0x7034), vec![vec![buffer, 0x9000]]);
        let device = index_device(&mut e, 0);
        e.mem.set_u32(this + 8, device);
        assert_eq!(
            e.call(
                0x00e8_f190,
                &args![this, 0x40u32, 8u32, 0x65u32, 1u32, 0u32]
            )
            .u32(),
            0
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_INDEX_BUFFER_FAILED_010F0184]]
        );
    }

    /// An index buffer whose description is (format, type, usage, pool, size) and whose lock
    /// hands out `data` (failing when it is 0).
    fn index_buffer(e: &mut Engine, desc: [u32; 5], data: u32) -> u32 {
        e.register_double(0x7134, move |e, a| {
            for (i, word) in desc.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *word);
            }
            0u32.into_ret()
        });
        e.register_double(0x712c, move |e, a| {
            e.mem.set_u32(a[3], data);
            (if data != 0 { 0u32 } else { 0x8000_0000 }).into_ret()
        });
        object(
            e,
            0x8200,
            8,
            &[
                (INDEX_BUFFER_GET_DESC, 0x7134),
                (INDEX_BUFFER_LOCK, 0x712c),
                (INDEX_BUFFER_UNLOCK, 0x7130),
                (8, 0x7108),
            ],
        )
    }

    #[test]
    fn fn_00e8f200_reuses_or_creates_and_fills_the_buffer() {
        let mut e = engine();
        for target in [0x7130u32, 0x7108] {
            e.register(target, |_, _| Ret::default());
        }
        let data = e.mem.alloc(0x40);
        let reuse = index_buffer(&mut e, [0x65, 7, 8, 1, 0x40], data);
        let made = e.mem.alloc(8);
        e.mem.set_u32(made, 0x8200);
        let device = index_device(&mut e, made);
        let this = e.mem.alloc(0x4c);
        let length = e.mem.alloc(4);
        let indices = e.mem.alloc(0x20);
        assert_eq!(
            e.call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, reuse, length, 1u32, 8u32]
            )
            .u32(),
            0
        );
        e.mem.set_u32(this + 8, device);
        assert_eq!(
            e.call(
                0x00e8_f200,
                &args![this, 0u32, 6u32, 0x20u32, reuse, length, 1u32, 8u32]
            )
            .u32(),
            0
        );
        // Large enough and alike: reused, filled, unlocked.
        let result = e
            .call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, reuse, length, 1u32, 8u32],
            )
            .u32();
        assert_eq!(result, reuse);
        assert!(calls(&e, 0x786c).is_empty());
        assert_eq!(
            calls(&e, 0x712c),
            vec![vec![reuse, 0, 0xc, calls(&e, 0x712c)[0][3], 0]]
        );
        assert_eq!(calls(&e, MEMCPY), vec![vec![data, indices, 0xc]]);
        assert_eq!(calls(&e, 0x7130).len(), 1);
        assert_eq!(e.mem.u32(length), 0);
        // Too small: a new one, its length kept.
        let result = e
            .call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x21u32, reuse, length, 1u32, 8u32],
            )
            .u32();
        assert_eq!(result, made);
        let create = &calls(&e, 0x786c)[0];
        assert_eq!(create[..5], [device, 0x42, 8, 0x65, 1]);
        assert_eq!(e.mem.u32(length), 0x42);
        // Nothing to copy: no lock.
        e.call(
            0x00e8_f200,
            &args![this, indices, 0u32, 0x20u32, reuse, length, 1u32, 8u32],
        );
        assert_eq!(calls(&e, 0x712c).len(), 2);
    }

    #[test]
    fn fn_00e8f200_reports_failures() {
        let mut e = engine();
        for target in [0x7130u32, 0x7108] {
            e.register(target, |_, _| Ret::default());
        }
        let device = index_device(&mut e, 0);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 8, device);
        let length = e.mem.alloc(4);
        let indices = e.mem.alloc(0x20);
        assert_eq!(
            e.call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, 0u32, length, 1u32, 8u32]
            )
            .u32(),
            0
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_INDEX_BUFFER_FAILED_010F0184],
                vec![PACK_CREATE_FAILED_010F01B8]
            ]
        );
        // A failed lock: reported, unlocked and released, still answered.
        let reuse = index_buffer(&mut e, [0x65, 7, 8, 1, 0x40], 0);
        let result = e
            .call(
                0x00e8_f200,
                &args![this, indices, 6u32, 0x20u32, reuse, length, 1u32, 8u32],
            )
            .u32();
        assert_eq!(result, reuse);
        assert_eq!(
            calls(&e, REPORT_00A29680)[2],
            vec![PACK_LOCK_FAILED_010EDDD0]
        );
        assert_eq!(calls(&e, 0x7130), vec![vec![reuse]]);
        assert_eq!(calls(&e, 0x7108), vec![vec![reuse]]);
        assert!(calls(&e, MEMCPY).is_empty());
    }

    /// An index buffer manager whose map +0x2c has two items (keys 0 and 1, in buckets 0 and
    /// 1; the hash answers the key), the first holding an information block with one entry
    /// at its +0x10 that holds a buffer.
    fn index_manager(e: &mut Engine) -> (u32, u32, u32, u32, u32) {
        let device = index_device(e, 0);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 8, device);
        let map_table = 0x8400;
        e.register(0x7204, |_, a| a[1].into_ret());
        object(e, map_table, 4, &[(4, 0x7204)]);
        for offset in INDEX_BUFFER_MAPS {
            let buckets = e.mem.alloc(8);
            e.mem.set_u32(this + offset, map_table);
            e.mem.set_u32(this + offset + 4, 2);
            e.mem.set_u32(this + offset + 8, buckets);
        }
        let buckets = e.mem.u32(this + 0x2c + 8);
        let info = e.mem.alloc(0x1c);
        let entry = e.mem.alloc(0x38);
        let held = object(e, 0x8600, 8, &[(8, 0x7308)]);
        e.mem.set_u32(entry + 0x20, held);
        e.mem.set_u32(info + 0x10, entry);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0] + 8, info);
        e.mem.set_u32(items[1] + 4, 1);
        e.mem.set_u32(buckets, items[0]);
        e.mem.set_u32(buckets + 4, items[1]);
        (this, device, info, entry, held)
    }

    #[test]
    fn fn_00e8f840_frees_every_information_block() {
        let mut e = engine();
        let (this, device, info, entry, held) = index_manager(&mut e);
        e.call(0x00e8_f840, &args![this]);
        assert_eq!(calls(&e, 0x7204).len(), 2);
        assert_eq!(calls(&e, 0x7308), vec![vec![held]]);
        assert_eq!(calls(&e, ENTRY_CLEAN_UP_00E95B80), vec![vec![entry]]);
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![entry, 0x38], vec![info, 0x1c]]
        );
        assert_eq!(calls(&e, MAP_00B601A0).len(), 9);
        assert_eq!(calls(&e, 0x71a0), vec![vec![device, 0]]);
        assert_eq!(calls(&e, 0x7008), vec![vec![device]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls(&e, FREE_00AA10F0).len(), 3);
        assert_eq!(e.mem.u32(this + 0x1c), IB_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        let mut e = engine();
        let (this, _, info, entry, _) = index_manager(&mut e);
        assert_eq!(e.call(0x00e8_fc30, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![entry, 0x38], vec![info, 0x1c], vec![this, 0x4c]]
        );
    }

    #[test]
    fn fn_00e8fa80_finds_or_makes_the_information_block() {
        let mut e = engine();
        let found = e.mem.alloc(4);
        e.register_double(MAP_LOOKUP_00A53C30, move |e, a| {
            let block = e.mem.u32(found);
            e.mem.set_u32(a[2], block);
            u32::from(block != 0).into_ret()
        });
        let this = e.mem.alloc(0x4c);
        assert_eq!(
            e.call(0x00e8_fa80, &args![this, 8u32, 0x65u32, 3u32]).u32(),
            0
        );
        assert!(calls(&e, MAP_LOOKUP_00A53C30).is_empty());
        e.mem.set_u32(found, 0x1234);
        assert_eq!(
            e.call(0x00e8_fa80, &args![this, 8u32, 0x65u32, 1u32]).u32(),
            0x1234
        );
        assert_eq!(
            calls(&e, MAP_LOOKUP_00A53C30)[0][..2],
            [this + 0x2c, 0x8_0065]
        );
        e.mem.set_u32(found, 0);
        let block = e.call(0x00e8_fa80, &args![this, 8u32, 0x65u32, 2u32]).u32();
        assert_eq!(e.mem.u32(block), 0x8_0065);
        assert_eq!(
            calls(&e, MAP_SET_00B57EA0),
            vec![vec![this + 0x3c, 0x8_0065, block]]
        );
    }

    #[test]
    fn fn_00e8fc50_rotates_through_the_entries() {
        let mut e = engine();
        let info = e.mem.alloc(0x1c);
        e.register_double(MAP_LOOKUP_00A53C30, move |e, a| {
            e.mem.set_u32(a[2], info);
            1u32.into_ret()
        });
        let buffer = object(
            &mut e,
            0x8200,
            8,
            &[(INDEX_BUFFER_GET_DESC, 0x7034), (8, 0x7108)],
        );
        e.register(0x7108, |_, _| Ret::default());
        let device = index_device(&mut e, buffer);
        let this = e.mem.alloc(0x4c);
        assert_eq!(
            e.call(0x00e8_fc50, &args![this, 0x40u32, 8u32, 0x65u32, 0u32])
                .u32(),
            0
        );
        e.mem.set_u32(this + 8, device);
        // No entry: a new one (not kept in the block) and a new buffer.
        let answer = e
            .call(0x00e8_fc50, &args![this, 0x40u32, 8u32, 0x65u32, 0u32])
            .u32();
        assert_eq!(answer, buffer);
        let entry = calls(&e, ENTRY_CONSTRUCT_00E95B60)[0][0];
        assert_eq!(e.mem.u32(entry + 0x20), buffer);
        assert_eq!(e.mem.u32(entry + 0xc), 8);
        assert_eq!(e.mem.u32(entry + 0x14), 0x40);
        assert_eq!(calls(&e, 0x7034)[0][1], entry + 0x24);
        assert_eq!(e.mem.u8(info + 4), 1);
        assert_eq!(e.mem.u32(info + 0xc), 0);
        // An entry no longer than the length asked gives its buffer.
        e.mem.set_u32(info + 0xc, entry);
        assert_eq!(
            e.call(0x00e8_fc50, &args![this, 0x50u32, 8u32, 0x65u32, 0u32])
                .u32(),
            buffer
        );
        assert_eq!(e.mem.u8(info + 4), 2);
        assert_eq!(e.mem.u32(entry + 0x14), 0x40);
        // A longer one is released and made again; the index wraps at 5.
        e.mem.set_u8(info + 4, 4);
        e.mem.set_u32(info + 0x18, entry);
        assert_eq!(
            e.call(0x00e8_fc50, &args![this, 0x20u32, 8u32, 0x65u32, 0u32])
                .u32(),
            buffer
        );
        assert_eq!(calls(&e, 0x7108), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(entry + 0x14), 0x20);
        assert_eq!(e.mem.u8(info + 4), 0);
    }

    #[test]
    fn fn_00e8fdc0_takes_the_rendered_texture_data_down() {
        let mut e = engine();
        let renderer = object(&mut e, 0x8000, 8, &[(0x15c, 0x715c)]);
        let surface = object(&mut e, 0x8200, 8, &[(8, 0x7108)]);
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x60, renderer);
        e.mem.set_u32(this + 8, 0x42);
        e.mem.set_u32(this + 0x6c, surface);
        assert_eq!(e.call(0x00e8_fdc0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
        assert_eq!(calls(&e, 0x715c), vec![vec![renderer, 0x42]]);
        assert_eq!(calls(&e, 0x7108), vec![vec![surface]]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(calls(&e, TEXTURE_DATA_DOWN_00E8A2B0), vec![vec![this]]);
        assert_eq!(calls(&e, FREE_00AA1460), vec![vec![this, 0x70]]);
    }

    /// A rendered texture data object and a texture for `00e8fe10`; the device's
    /// `CreateTexture` answers `texture_result` and its `CreateRenderTarget` 0.
    fn surface_rig(e: &mut Engine, texture_result: u32) -> (u32, u32, u32) {
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(format + 0xc, 0x15);
        e.register_double(PICK_FORMAT_00E89ED0, move |_, _| format.into_ret());
        e.register(MULTISAMPLE_TYPE_00E7CCF0, |_, a| (a[0] * 2).into_ret());
        e.register_double(0x785c, move |e, a| {
            e.mem.set_u32(a[7], 0x7777);
            texture_result.into_ret()
        });
        let device = object(
            e,
            0x8000,
            8,
            &[(CREATE_TEXTURE, 0x785c), (CREATE_RENDER_TARGET, 0x7070)],
        );
        let renderer = e.mem.alloc(0x800);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        e.register(0x7194, |_, _| 256u32.into_ret());
        e.register(0x7198, |_, _| 128u32.into_ret());
        let texture = object(e, 0x8200, 0x48, &[(0x94, 0x7194), (0x98, 0x7198)]);
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x60, renderer);
        (this, texture, format)
    }

    #[test]
    fn fn_00e8fe10_creates_the_surfaces() {
        let mut e = engine();
        let (this, texture, format) = surface_rig(&mut e, 0);
        assert_eq!(e.call(0x00e8_fe10, &args![this, 0u32, 0u32]).u32(), 0);
        e.mem.set_u32(texture + 0x18, 0xa);
        assert_eq!(
            e.call(0x00e8_fe10, &args![this, texture, 0u32]).u32(),
            format
        );
        assert_eq!(e.mem.u32(this + 0xc), 256);
        assert_eq!(e.mem.u32(this + 0x10), 128);
        assert_eq!(e.mem.u32(this + 0x68), 1);
        assert_eq!(e.mem.u32(this + 0x14 + 0xc), 0x15);
        assert_eq!(e.mem.u32(this + 0x64), 0x7777);
        let renderer = e.mem.u32(this + 0x60);
        assert_eq!(calls(&e, PICK_FORMAT_00E89ED0)[0][1], renderer + 0x758);
        let device = e.mem.u32(renderer + RENDERER_DEVICE);
        let create = &calls(&e, 0x785c)[0];
        assert_eq!(create[..7], [device, 256, 128, 1, 0, 0x15, 1]);
        assert!(calls(&e, 0x7070).is_empty());
        // Byte +0x40: usage 1, pool 0; byte +0x41: the texture's own format; a sample kind.
        e.mem.set_u8(texture + 0x40, 1);
        e.mem.set_u8(texture + 0x41, 1);
        e.mem.set_u32(texture + 0x44, 0x33);
        e.call(0x00e8_fe10, &args![this, texture, 2u32]);
        assert_eq!(calls(&e, 0x785c)[1][..7], [device, 256, 128, 1, 1, 0x33, 0]);
        assert_eq!(
            calls(&e, 0x7070),
            vec![vec![device, 256, 128, 0x33, 4, 0, 0, this + 0x6c, 0]]
        );
    }

    #[test]
    fn fn_00e8fe10_reports_a_failed_texture() {
        let mut e = engine();
        let (this, texture, _) = surface_rig(&mut e, 0x8876_0868);
        e.mem.set_u8(POOL_FLAG_0126F0D0, 1);
        e.mem.set_u32(this + 0x6c, 0x99);
        assert_eq!(e.call(0x00e8_fe10, &args![this, texture, 0u32]).u32(), 0);
        assert_eq!(calls(&e, 0x785c)[0][6], 0);
        assert_eq!(calls(&e, ERROR_STRING_00E89D50), vec![vec![0x8876_0868]]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_SURFACE_FAILED_010F03B0, 0x5151]]
        );
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
    }

    #[test]
    fn fn_00e8ff60_makes_the_rendered_texture_data() {
        let mut e = engine();
        let (probe, texture, _) = surface_rig(&mut e, 0);
        let renderer = e.mem.u32(probe + 0x60);
        let owner = e.mem.alloc(0x28);
        // The base constructor sets the renderer and the owner.
        e.register_double(TEXTURE_DATA_CONSTRUCT_00E8A260, move |e, a| {
            e.mem.set_u32(a[0] + 0x60, renderer);
            e.mem.set_u32(a[0] + 8, owner);
            Ret::default()
        });
        let queried = object(&mut e, 0x8600, 8, &[(8, 0x7308)]);
        e.register_double(0x7400, move |e, a| {
            e.mem.set_u32(a[2], queried);
            0u32.into_ret()
        });
        let d3d_texture = object(&mut e, 0x8800, 8, &[(0, 0x7400)]);
        e.register_double(0x729c, move |_, _| d3d_texture.into_ret());
        let mut slots = vec![0u32; 0x40];
        slots[0x9c / 4] = 0x729c;
        slots[0] = 0x7200;
        e.put_vtable(NI_DX9_RENDERED_TEXTURE_DATA_VTABLE, &slots);
        e.register(0x71a4, |_, _| 0x4444u32.into_ret());
        let mut texture_slots = vec![0u32; 0x40];
        texture_slots[0x94 / 4] = 0x7194;
        texture_slots[0x98 / 4] = 0x7198;
        texture_slots[0xa4 / 4] = 0x71a4;
        e.put_vtable(0x8200, &texture_slots);
        let made = e.mem.alloc(4);
        e.register_double(MAKE_TEXTURE_BUFFER_DATA_00E7D250, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x4444);
            e.mem.u32(made).into_ret()
        });
        e.mem.set_u32(made, 0x5555);
        let data = e.call(0x00e8_ff60, &args![texture, 0x31u32, 1u32]).u32();
        assert_ne!(data, 0);
        assert_eq!(
            calls(&e, TEXTURE_DATA_CONSTRUCT_00E8A260),
            vec![vec![data, texture, 0x31]]
        );
        assert_eq!(e.mem.u32(owner + 0x24), data);
        assert_eq!(calls(&e, 0x7400)[0][1], IID_DIRECT3D_TEXTURE_01101014);
        let make = &calls(&e, MAKE_TEXTURE_BUFFER_DATA_00E7D250)[0];
        assert_eq!([make[0], make[2], make[3]], [queried, 1, 0]);
        assert_eq!(calls(&e, 0x7308), vec![vec![queried]]);
        assert!(calls(&e, 0x7200).is_empty());
        // Without buffer data: reported and deleted.
        e.mem.set_u32(made, 0);
        assert_eq!(e.call(0x00e8_ff60, &args![texture, 0x31u32, 1u32]).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![
                BUFFER_DATA_FAILED_010F03F8,
                BUFFER_DATA_FAILED_ARGUMENT_01011584
            ]]
        );
        assert_eq!(calls(&e, 0x7200)[0][1], 1);
        // A failed query: reported and deleted.
        e.register(0x7400, |_, _| 0x8000_4002u32.into_ret());
        assert_eq!(e.call(0x00e8_ff60, &args![texture, 0x31u32, 1u32]).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680)[1],
            vec![QUERY_FAILED_010F0440, 0x5151]
        );
        assert_eq!(calls(&e, 0x7200).len(), 2);
        // A failed surface (no texture): the object deleted before anything else.
        assert_eq!(e.call(0x00e8_ff60, &args![0u32, 0x31u32, 1u32]).u32(), 0);
        assert_eq!(calls(&e, 0x7200).len(), 3);
    }

    #[test]
    fn getters_answer_their_words() {
        let mut e = engine();
        let this = e.mem.alloc(0x1248);
        e.mem.set_u32(this + 0x6c, 1);
        e.mem.set_u32(this + 0x10e0, 2);
        e.mem.set_u32(this + 0x110c, 3);
        assert_eq!(e.call(0x00e9_02d0, &args![this]).u32(), 1);
        assert_eq!(e.call(0x00e9_0e40, &args![this]).u32(), 2);
        assert_eq!(e.call(0x00e9_1020, &args![this]).u32(), 0);
        e.mem.set_u8(this + 0x1100, 1);
        assert_eq!(e.call(0x00e9_1020, &args![this]).u32(), 3);
    }

    #[test]
    fn deleting_destructors_of_the_shader_texture_manager_and_render_state() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(ERROR_PASS_0126F74C, 0x99);
        assert_eq!(e.call(0x00e9_0560, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(ERROR_PASS_0126F74C), 0);
        assert_eq!(calls(&e, SHADER_DOWN_00E806C0), vec![vec![this]]);
        let device = object(&mut e, 0x8000, 8, &[(8, 0x7008)]);
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(manager + 8, device);
        e.call(0x00e9_0ac0, &args![manager, 0u32]);
        assert_eq!(calls(&e, 0x7008), vec![vec![device]]);
        assert_eq!(e.mem.u32(manager + 8), 0);
        assert_eq!(e.mem.u32(manager), NI_REF_OBJECT_VTABLE);
        let state = e.mem.alloc(0x10);
        e.call(0x00e9_1160, &args![state, 1u32]);
        assert_eq!(e.mem.u32(state), NI_DX9_RENDER_STATE_VTABLE);
        assert_eq!(calls(&e, RENDER_STATE_DOWN_00E88B00), vec![vec![state]]);
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![this, 0x40], vec![state, 0x1248]]
        );
    }

    #[test]
    fn fn_00e90970_and_fn_00e90a50_set_up_the_declaration() {
        let mut e = engine();
        let made = object(&mut e, 0x8000, 8, &[(4, 0x7004), (0x8c, 0x708c)]);
        e.register_double(SHADER_DECLARATION_CREATE_00E76700, move |_, _| {
            made.into_ret()
        });
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x8000);
        e.mem.set_u32(old + 4, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x18);
        e.mem.set_u32(this + 0x28, old);
        assert_eq!(e.call(0x00e9_0970, &args![this]).u8(), 1);
        assert_eq!(
            calls(&e, SHADER_DECLARATION_CREATE_00E76700),
            vec![vec![0x18, 1, 1]]
        );
        assert_eq!(calls(&e, 0x7004), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0x28), made);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(calls(&e, 0x708c), vec![vec![made, 0, 0, 0, 2, 0]]);
        let ok = e.mem.alloc(4);
        e.register_double(SHADER_INTERFACE_INITIALIZE_00E80570, move |e, _| {
            e.mem.u32(ok).into_ret()
        });
        assert!(!e.call(0x00e9_0a50, &args![this]).bool());
        e.mem.set_u32(ok, 1);
        assert!(e.call(0x00e9_0a50, &args![this]).bool());
        assert_eq!(calls(&e, 0x708c).len(), 2);
        e.mem.set_u8(this + 0x10, 1);
        assert!(e.call(0x00e9_0a50, &args![this]).bool());
        assert_eq!(calls(&e, SHADER_INTERFACE_INITIALIZE_00E80570).len(), 2);
    }

    /// Renderer data answering `a4`, `ac`, `a8`, `9c` and `98` from its slots; `target` answers
    /// 0x77 from its slot 0x9c.
    fn texture_rig(e: &mut Engine, words: [u32; 5]) -> (u32, u32, u32) {
        let renderer = e.mem.alloc(0x200);
        e.mem.set_u32(RENDERER_011F4748, renderer);
        for (slot, word) in [0xa4u32, 0xac, 0xa8, 0x9c, 0x98].iter().zip(words) {
            let answer = if word == 3 { 0x8e00 } else { word };
            e.register_double(0x7000 + slot, move |_, _| answer.into_ret());
        }
        let mut slots = vec![0u32; 0x40];
        for slot in [0x98u32, 0x9c, 0xa4, 0xa8, 0xac, 0xb4] {
            slots[(slot / 4) as usize] = 0x7000 + slot;
        }
        e.put_vtable(0x8000, &slots);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data, 0x8000);
        e.mem.set_u32(data + 0xc, 64);
        e.mem.set_u32(data + 0x10, 32);
        let mut target_slots = vec![0u32; 0x40];
        target_slots[0x9c / 4] = 0x7800;
        e.put_vtable(0x8e00, &target_slots);
        e.register(0x7800, |_, _| 0x77u32.into_ret());
        let texture = e.mem.alloc(0x44);
        e.mem.set_u32(texture + 0x24, data);
        let this = e.mem.alloc(0x10);
        (this, texture, data)
    }

    #[test]
    fn fn_00e90b10_answers_the_texture_and_its_flags() {
        let mut e = engine();
        // Slot 0xa4 answers an object (at 0x8e00, whose table is itself).
        let (this, texture, data) = texture_rig(&mut e, [3, 0, 0, 0, 0]);
        e.mem.set_u32(0x8e00, 0x8e00);
        let flags = e.mem.alloc(3);
        e.mem.write(flags, &[9, 9, 9]);
        assert_eq!(
            e.call(0x00e9_0b10, &args![this, 0u32, flags, flags + 1, flags + 2])
                .u32(),
            0
        );
        assert_eq!(e.mem.bytes(flags, 3), vec![0, 0, 0]);
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            0x77
        );
        assert_eq!(e.mem.bytes(flags, 3), vec![0, 0, 0]);
        e.mem.set_u32(data + 0x10, 48);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        assert_eq!(e.mem.u8(flags + 2), 1);
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION)[0],
            vec![e.mem.u32(RENDERER_011F4748) + 0x180]
        );
    }

    #[test]
    fn fn_00e90b10_without_a_target_reads_the_levels() {
        let mut e = engine();
        // No 0xa4, 0xac or 0xa8 object; 0x9c answers 0, 0x98 answers 2.
        let (this, texture, data) = texture_rig(&mut e, [0, 0, 0, 0, 2]);
        let flags = e.mem.alloc(3);
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.bytes(flags, 3), vec![0, 1, 0]);
        // Missing renderer data is made from the texture's source and marks `made`.
        e.mem.set_u32(texture + 0x24, 0);
        e.register(SOURCE_OF_00653290, |_, _| 0x5000u32.into_ret());
        e.register_double(MAKE_RENDERER_DATA_00E68EF0, move |_, _| data.into_ret());
        e.mem.set_u32(this + 0xc, 0x42);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        assert_eq!(e.mem.u8(flags), 1);
        assert_eq!(
            calls(&e, SOURCE_OF_00653290),
            vec![vec![texture, SOURCE_KIND_011F444C]]
        );
        assert_eq!(
            calls(&e, MAKE_RENDERER_DATA_00E68EF0),
            vec![vec![0x5000, 0x42]]
        );
        // No source: null, the lock left.
        e.register(SOURCE_OF_00653290, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            0
        );
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn fn_00e90b10_touches_the_other_object() {
        let mut e = engine();
        // 0xa8 answers the object at 0x8e00 (its own table; slot 0xb4 recorded).
        let (this, texture, _) = texture_rig(&mut e, [0, 0, 3, 0, 1]);
        e.mem.set_u32(0x8e00, 0x8e00);
        e.mem.set_u32(0x8e00 + 0xb4, 0x78b4);
        e.register(0x78b4, |_, _| Ret::default());
        let flags = e.mem.alloc(3);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        // Slot 0x9c of the data answers 0: the other object's 0xb4 and `made`.
        assert_eq!(calls(&e, 0x78b4), vec![vec![0x8e00]]);
        assert_eq!(e.mem.bytes(flags, 3), vec![1, 0, 0]);
        let (this, texture, _) = texture_rig(&mut e, [0, 0, 3, 1, 1]);
        e.mem.set_u32(0x8e00, 0x8e00);
        e.mem.set_u32(0x8e00 + 0xb4, 0x78b4);
        e.mem.set_u8(texture + 0x40, 1);
        e.call(
            0x00e9_0b10,
            &args![this, texture, flags, flags + 1, flags + 2],
        );
        assert_eq!(calls(&e, 0x78b4).len(), 1);
        e.mem.set_u8(texture + 0x40, 0);
        assert_eq!(
            e.call(
                0x00e9_0b10,
                &args![this, texture, flags, flags + 1, flags + 2]
            )
            .u32(),
            1
        );
        assert_eq!(calls(&e, 0x78b4).len(), 2);
        assert_eq!(e.mem.u8(flags), 0);
    }

    #[test]
    fn fn_00e90c70_answers_whether_the_target_has_a_texture() {
        let mut e = engine();
        let (this, texture, _) = texture_rig(&mut e, [0, 0, 0, 0, 0]);
        assert!(!e.call(0x00e9_0c70, &args![this, 0u32, 0u32, 0u32]).bool());
        assert!(e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
        let (this, texture, _) = texture_rig(&mut e, [0, 3, 0, 0, 0]);
        e.mem.set_u32(0x8e00, 0x8e00);
        assert!(e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
        e.register(0x7800, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
        e.mem.set_u32(texture + 0x24, 0);
        e.register(SOURCE_OF_00653290, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e9_0c70, &args![this, texture, 0u32, 0u32])
            .bool());
    }

    fn render_state(e: &mut Engine) -> (u32, u32) {
        let device = object(
            e,
            0x8000,
            8,
            &[
                (SET_VERTEX_SHADER, 0x7170),
                (SET_VERTEX_DECLARATION, 0x715c),
                (SET_SAMPLER_STATE, 0x7114),
            ],
        );
        let state = e.mem.alloc(0x1248);
        e.mem.set_u32(state + STATE_DEVICE, device);
        (state, device)
    }

    #[test]
    fn fn_00e90d80_resets_the_declaration_words() {
        let mut e = engine();
        let (state, _) = render_state(&mut e);
        e.mem.set_u8(state + 0x1100, 1);
        e.mem.set_u32(state + 0x110c, 5);
        e.call(0x00e9_0d80, &args![state]);
        assert_eq!(e.mem.u8(state + 0x1100), 0);
        assert_eq!(e.mem.u32(state + 0x110c), 0);
        assert_eq!(e.mem.u32(state + 0x1108), 1);
        assert_eq!(e.mem.u32(state + 0x1110), 1);
        assert_eq!(calls(&e, RESET_SHADERS_00E87A50), vec![vec![state]]);
    }

    #[test]
    fn vertex_shader_words_follow_the_device() {
        let mut e = engine();
        let (state, device) = render_state(&mut e);
        e.call(0x00e9_0e00, &args![state, 0x11u32, 0u32]);
        e.call(0x00e9_0e00, &args![state, 0x11u32, 0u32]);
        assert_eq!(calls(&e, 0x7170), vec![vec![device, 0x11]]);
        e.call(0x00e9_0e00, &args![state, 0x22u32, 1u32]);
        assert_eq!(e.mem.u32(state + 0x10e4), 0x11);
        assert_eq!(e.mem.u32(state + 0x10e0), 0x22);
        let mut slots = vec![0u32; 0x40];
        slots[0x8c / 4] = 0x758c;
        e.put_vtable(0x8c00, &slots);
        e.register(0x758c, |_, _| Ret::default());
        e.mem.set_u32(state, 0x8c00);
        e.call(0x00e9_0e50, &args![state]);
        assert_eq!(calls(&e, 0x758c), vec![vec![state, 0x11, 0]]);
        e.call(0x00e9_0e70, &args![state, 0x11u32]);
        assert_eq!(e.mem.u32(state + 0x10e4), 0);
        assert_eq!(e.mem.u32(state + 0x10e0), 0x22);
        e.call(0x00e9_0e70, &args![state, 0x22u32]);
        assert_eq!(e.mem.u32(state + 0x10e0), 0);
        assert_eq!(calls(&e, 0x7170).last().unwrap(), &vec![device, 0]);
    }

    #[test]
    fn vertex_declaration_words_follow_the_device() {
        let mut e = engine();
        let (state, device) = render_state(&mut e);
        e.call(0x00e9_0fc0, &args![state, 0u32, 1u32]);
        assert_eq!(e.mem.u8(state + 0x1100), 0);
        e.call(0x00e9_0fc0, &args![state, 0x31u32, 0u32]);
        e.call(0x00e9_0fc0, &args![state, 0x31u32, 1u32]);
        assert_eq!(calls(&e, 0x715c), vec![vec![device, 0x31]]);
        assert_eq!(e.mem.u8(state + 0x1100), 1);
        e.call(0x00e9_0fc0, &args![state, 0x32u32, 1u32]);
        assert_eq!(e.mem.u32(state + 0x1110), 0x31);
        assert_eq!(e.mem.u32(state + 0x110c), 0x32);
        e.call(0x00e9_1060, &args![state, 0x31u32]);
        assert_eq!(e.mem.u32(state + 0x1110), 0);
        e.mem.set_u8(state + 0x1100, 0);
        e.call(0x00e9_1060, &args![state, 0x32u32]);
        assert_eq!(e.mem.u32(state + 0x110c), 0x32);
        e.mem.set_u8(state + 0x1100, 1);
        e.call(0x00e9_1060, &args![state, 0x32u32]);
        assert_eq!(e.mem.u32(state + 0x110c), 0);
    }

    #[test]
    fn fn_00e910a0_caches_sampler_states() {
        let mut e = engine();
        let (state, device) = render_state(&mut e);
        e.mem.set_u16(SAMPLER_STATE_SLOTS_0126F92C + 2 * 6, 3);
        e.mem.set_u16(SAMPLER_STATE_SLOTS_0126F92C + 2 * 7, 5);
        e.call(0x00e9_10a0, &args![state, 2u32, 6u32, 9u32, 0u32]);
        e.call(0x00e9_10a0, &args![state, 2u32, 6u32, 9u32, 0u32]);
        e.call(0x00e9_10a0, &args![state, 2u32, 7u32, 9u32, 0u32]);
        assert_eq!(calls(&e, 0x7114), vec![vec![device, 2, 6, 9]]);
        assert_eq!(e.mem.u32(state + 0xe20 + (2 * 5 + 3) * 8), 9);
    }

    #[test]
    fn fn_00e911e0_compacts_the_pairs_sets_states_and_packs_the_colour() {
        let mut e = engine();
        e.map(0x011b_f000, 0x2000);
        e.map(0x0101_e000, 0x1000);
        e.mem.write(SCALE_0101E568, &255.0f64.to_le_bytes());
        for (i, (state, value)) in [(1u32, 10u32), (0xbe, 0), (2, 20), (0xffff_ffff, 0)]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32, *state);
            e.mem
                .set_u32(STATE_PAIRS_011BFCE8 + 8 * i as u32 + 4, *value);
        }
        e.mem.set_u32(DEFAULT_COLOR_011F4998, 1.0f32.to_bits());
        e.mem.set_u32(DEFAULT_COLOR_011F4998 + 4, 0.5f32.to_bits());
        e.mem.set_u32(DEFAULT_COLOR_011F4998 + 8, 0.25f32.to_bits());
        let mut slots = vec![0u32; 0x40];
        for slot in [0x58u32, 0x68, 0x70, 0xbc, 0xd8] {
            slots[(slot / 4) as usize] = 0x7500 + slot;
            e.register(0x7500 + slot, |_, _| Ret::default());
        }
        e.put_vtable(0x8a00, &slots);
        let state = e.mem.alloc(0x1248);
        e.mem.set_u32(state, 0x8a00);
        e.mem.set_u8(state + 8, 1);
        e.mem.set_u32(state + 0x38, 0x38);
        e.mem.set_u32(state + 0x3c, 0x3c);
        e.call(0x00e9_11e0, &args![state]);
        let pairs: Vec<u32> = (0..8)
            .map(|i| e.mem.u32(STATE_PAIRS_011BFCE8 + 4 * i))
            .collect();
        assert_eq!(pairs, vec![1, 10, 2, 20, 0xffff_ffff, 0, 0xffff_ffff, 0]);
        let set: Vec<(u32, u32)> = calls(&e, 0x7568).iter().map(|c| (c[1], c[2])).collect();
        assert_eq!(set.len(), 21);
        assert_eq!(set[4], (0x13, 0x38));
        assert_eq!(set[5], (0x14, 0x3c));
        assert_eq!(set[14], (0x23, 3));
        assert_eq!(set[20], (0x8d, 1));
        assert_eq!(calls(&e, MEMBER_RESET_00E94130), vec![vec![state + 0xf8]]);
        assert_eq!(e.mem.u32(state + 0x90), 0.5f32.to_bits());
        assert_eq!(e.mem.u32(state + 0x98), 0xffff_7f3f);
        // With bit 0x4000 the pairs stay.
        e.mem.set_u32(STATE_PAIRS_011BFCE8 + 8, 0xbe);
        e.mem.set_u32(state + 0x1138, 0x4000);
        e.mem.set_u8(state + 8, 0);
        e.call(0x00e9_11e0, &args![state]);
        assert_eq!(e.mem.u32(STATE_PAIRS_011BFCE8 + 8), 0xbe);
        assert_eq!(calls(&e, 0x7568)[35][2], 0);
    }

    #[test]
    fn fn_00e914e0_derives_the_capability_flags() {
        let mut e = engine();
        let renderer = e.mem.alloc(0x5d8);
        let state = e.mem.alloc(0x1248);
        e.mem.set_u32(state + 0x10fc, renderer);
        e.mem.set_u32(state + 8, 0xf0);
        e.mem.set_u32(state + 0x113c, 0x10_0100);
        e.mem.set_u32(state + 0x1144, 0x1);
        e.mem.set_u32(state + 0x1148, 0x2);
        e.mem.set_u32(state + 0x20, 1);
        e.mem.set_u32(state + 0x20 + 4 * 10, 3);
        e.mem.set_u8(state + 0x1164, 0x10);
        e.mem.set_u8(renderer + 0x5d4, 1);
        e.call(0x00e9_14e0, &args![state]);
        assert_eq!(e.mem.u32(state + 8), 0xf5);
        assert_eq!(e.mem.u32(state + 0x6c), 0x401);
        assert_eq!(e.mem.u32(state + 0x70), 0x400);
        assert_eq!(e.mem.u8(state + 0x1114), 1);
        e.mem.set_u32(state + 0x113c, 0x100);
        e.mem.set_u8(state + 0x1164, 0);
        e.mem.set_u8(renderer + 0x5d5, 1);
        e.call(0x00e9_14e0, &args![state]);
        assert_eq!(e.mem.u32(state + 8), 0xf0);
        assert_eq!(e.mem.u8(state + 0x1114), 0);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    /// An object whose virtual slots call quiet doubles at `doubles + slot`.
    fn object(e: &mut Engine, doubles: u32, size: u32) -> u32 {
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200).step_by(4) {
            e.mem.set_u32(table + slot, doubles + slot);
            quiet(e, doubles + slot);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    const DEVICE: u32 = 0x00ed_0000;
    const BUFFER: u32 = 0x00ec_0000;
    const OWN: u32 = 0x00ee_0000;

    /// An index buffer manager with a device; `buffer` doubles answer a description and a
    /// lock into `memory`.
    fn manager(e: &mut Engine, desc: [u32; 5], memory: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x50);
        let device = object(e, DEVICE, 8);
        e.mem.set_u32(this + 8, device);
        let buffer = object(e, BUFFER, 8);
        e.register_double(BUFFER + INDEX_BUFFER_GET_DESC, move |e, a| {
            for (i, value) in desc.iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *value);
            }
            word(0)
        });
        e.register_double(BUFFER + INDEX_BUFFER_LOCK, move |e, a| {
            e.mem.set_u32(a[3], memory);
            word(0)
        });
        e.register_double(DEVICE + CREATE_INDEX_BUFFER, move |e, a| {
            e.mem.set_u32(a[5], buffer);
            word(0)
        });
        quiet(e, REPORT_00A29680);
        e.call_log = Some(vec![]);
        (this, buffer)
    }

    fn shorts(e: &Engine, at: u32, count: u32) -> Vec<u16> {
        (0..count).map(|i| e.mem.u16(at + 2 * i)).collect()
    }

    #[test]
    fn fn_00e8f360_reuses_or_creates_and_fills_quads() {
        let mut e = Engine::new();
        let memory = e.mem.alloc(0x40);
        let (this, buffer) = manager(&mut e, [0x65, 7, 8, 1, 0x30], memory);
        e.mem.set_u32(this + 0xc, buffer);
        // A fitting buffer without a refill comes back as it is.
        let got = e.call(0x00e8_f360, &args![this, 2u32, 2u32, 0u32, 8u32, 1u32]);
        assert_eq!(got.u32(), buffer);
        assert!(calls(&e, BUFFER + INDEX_BUFFER_LOCK).is_empty());
        // With the refill byte it is filled with two quads.
        let got = e.call(0x00e8_f360, &args![this, 2u32, 2u32, 0x101u32, 8u32, 1u32]);
        assert_eq!(got.u32(), buffer);
        assert_eq!(
            shorts(&e, memory, 12),
            vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]
        );
        assert_eq!(e.mem.u32(this + 0x10), 24);
        // Another usage does not fit: released and created again.
        e.call_log = Some(vec![]);
        let got = e.call(0x00e8_f360, &args![this, 1u32, 1u32, 0u32, 9u32, 1u32]);
        assert_eq!(got.u32(), buffer);
        assert_eq!(calls(&e, BUFFER + 8), vec![vec![buffer]]);
        let device = e.mem.u32(this + 8);
        assert_eq!(
            calls(&e, DEVICE + CREATE_INDEX_BUFFER)[0][..5],
            [device, 12, 9, 0x65, 1]
        );
        assert_eq!(e.mem.u32(this + 0xc), buffer);
        assert_eq!(e.mem.u32(this + 0x10), 12);
    }

    #[test]
    fn fn_00e8f360_fails_cleanly() {
        let mut e = Engine::new();
        let memory = e.mem.alloc(0x40);
        let (this, buffer) = manager(&mut e, [0; 5], memory);
        // No needed count or no device: 0.
        assert_eq!(
            e.call(0x00e8_f360, &args![this, 0u32, 1u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        // A failed create reports both messages.
        e.register(DEVICE + CREATE_INDEX_BUFFER, |_, _| word(0x8876_0001));
        assert_eq!(
            e.call(0x00e8_f360, &args![this, 1u32, 1u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_INDEX_BUFFER_FAILED_010F0184],
                vec![POINT_CONNECTIVITY_FAILED_010F0200]
            ]
        );
        // A failed lock: unlock, release, cleared.
        e.register_double(DEVICE + CREATE_INDEX_BUFFER, move |e, a| {
            e.mem.set_u32(a[5], buffer);
            word(0)
        });
        e.register(BUFFER + INDEX_BUFFER_LOCK, |_, _| word(0x8876_0002));
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e8_f360, &args![this, 1u32, 1u32, 0u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(calls(&e, BUFFER + INDEX_BUFFER_UNLOCK).len(), 1);
        assert_eq!(calls(&e, BUFFER + 8).len(), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 12);
    }

    #[test]
    fn fn_00e8f520_writes_line_pairs_for_set_flags() {
        let mut e = Engine::new();
        let memory = e.mem.alloc(0x40);
        let (this, buffer) = manager(&mut e, [0x65, 7, 8, 1, 0x40], memory);
        e.mem.set_u32(this + 0x14, buffer);
        let flags = e.mem.alloc(4);
        for (i, f) in [1u8, 0, 1, 1].iter().enumerate() {
            e.mem.set_u8(flags + i as u32, *f);
        }
        let count = e.mem.alloc(4);
        let got = e.call(0x00e8_f520, &args![this, 4u32, flags, count, 8u32, 1u32]);
        assert_eq!(got.u32(), buffer);
        // Pairs (0,1), (2,3) and the closing (3,0).
        assert_eq!(shorts(&e, memory, 6), vec![0, 1, 2, 3, 3, 0]);
        assert_eq!(e.mem.u32(count), 6);
        assert_eq!(e.mem.u32(this + 0x18), 20);
        assert!(calls(&e, DEVICE + CREATE_INDEX_BUFFER).is_empty());
        // A missing flag array answers 0.
        assert_eq!(
            e.call(0x00e8_f520, &args![this, 4u32, 0u32, count, 8u32, 1u32])
                .u32(),
            0
        );
    }

    #[test]
    fn fn_00e8f520_creates_and_reports() {
        let mut e = Engine::new();
        let memory = e.mem.alloc(0x40);
        let (this, buffer) = manager(&mut e, [0; 5], memory);
        let flags = e.mem.alloc(4);
        let count = e.mem.alloc(4);
        // No kept buffer: `00e8f190` creates one.
        let got = e.call(0x00e8_f520, &args![this, 2u32, flags, count, 8u32, 1u32]);
        assert_eq!(got.u32(), buffer);
        let device = e.mem.u32(this + 8);
        assert_eq!(
            calls(&e, DEVICE + CREATE_INDEX_BUFFER)[0][..5],
            [device, 12, 8, 0x65, 1]
        );
        // A failed lock keeps a null buffer.
        e.register(BUFFER + INDEX_BUFFER_LOCK, |_, _| word(0x8876_0002));
        let got = e.call(0x00e8_f520, &args![this, 2u32, flags, count, 9u32, 1u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 12);
        // A failed create: the connectivity message.
        e.register(DEVICE + CREATE_INDEX_BUFFER, |_, _| word(0x8876_0001));
        e.call_log = Some(vec![]);
        let got = e.call(0x00e8_f520, &args![this, 2u32, flags, count, 9u32, 1u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680).last().unwrap(),
            &vec![LINE_CONNECTIVITY_FAILED_010F0258]
        );
    }

    #[test]
    fn fn_00e8f6d0_frees_records_and_releases_both_buffers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(this + 0x20, 2);
        e.mem.set_u32(this + 0x24, buckets);
        e.mem.set_u32(buckets + 4, 0x0e0a_0000);
        let held = object(&mut e, BUFFER, 8);
        let entry = e.mem.alloc(0x38);
        e.mem.set_u32(entry + 0x20, held);
        let record = e.mem.alloc(0x1c);
        e.mem.set_u32(record + 0xc, entry);
        e.register_double(MAP_ITERATE_00B57F20, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], record);
            Ret::default()
        });
        let first = object(&mut e, DEVICE, 8);
        e.mem.set_u32(this + 0xc, first);
        quiet(&mut e, ENTRY_CLEAN_UP_00E95B80);
        quiet(&mut e, FREE_00AA1460);
        quiet(&mut e, MAP_00B601A0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_f6d0, &args![this]);
        assert_eq!(
            calls(&e, FREE_00AA1460),
            vec![vec![entry, 0x38], vec![record, 0x1c]]
        );
        assert_eq!(calls(&e, BUFFER + 8), vec![vec![held]]);
        assert_eq!(calls(&e, MAP_00B601A0), vec![vec![this + 0x1c]]);
        assert_eq!(calls(&e, DEVICE + 8), vec![vec![first]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn index_buffer_map_deleting_destructors_free_on_bit_zero() {
        for (address, tables) in [
            (0x00e8_f7c0u32, vec![IB_MAP_BASE_VTABLE]),
            (0x00e8_f800, vec![IB_MAP_VTABLE, IB_MAP_BASE_VTABLE]),
        ] {
            for (flags, frees) in [(0u32, false), (1, true)] {
                let mut e = Engine::new();
                let seen = Rc::new(RefCell::new(vec![]));
                let tables_seen = seen.clone();
                e.register_double(MAP_00B601A0, move |e, a| {
                    tables_seen.borrow_mut().push(e.mem.u32(a[0]));
                    Ret::default()
                });
                quiet(&mut e, FREE_00AA10F0);
                quiet(&mut e, MEMORY_DEALLOCATE);
                let this = e.mem.alloc(0x10);
                e.mem.set_u32(this + 8, 0x0e0b_0000);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                assert_eq!(*seen.borrow(), tables);
                assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x0e0b_0000]]);
                assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), usize::from(frees));
            }
        }
    }

    #[test]
    fn rendered_texture_data_construct_and_destroy() {
        let mut e = Engine::new();
        quiet(&mut e, TEXTURE_DATA_CONSTRUCT_00E8A260);
        quiet(&mut e, TEXTURE_DATA_DOWN_00E8A2B0);
        quiet(&mut e, FREE_00AA1460);
        let this = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e8_fd60, &args![this, 0x11u32, 0x22u32, 0x33u32])
                .u32(),
            this
        );
        assert_eq!(
            calls(&e, TEXTURE_DATA_CONSTRUCT_00E8A260),
            vec![vec![this, 0x11, 0x22]]
        );
        assert_eq!(e.mem.u32(this), NI_DX9_RENDERED_TEXTURE_DATA_VTABLE);
        let renderer = object(&mut e, OWN, 0x300);
        let surface = object(&mut e, BUFFER, 8);
        e.mem.set_u32(this + 0x60, renderer);
        e.mem.set_u32(this + 8, 0x0e08_0000);
        e.mem.set_u32(this + 0x6c, surface);
        e.call(0x00e9_0080, &args![this, 1u32]);
        assert_eq!(calls(&e, OWN + 0x158), vec![vec![renderer, 0x0e08_0000]]);
        assert_eq!(calls(&e, OWN + 0x15c), vec![vec![renderer, 0x0e08_0000]]);
        assert_eq!(calls(&e, BUFFER + 8), vec![vec![surface]]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(calls(&e, TEXTURE_DATA_DOWN_00E8A2B0), vec![vec![this]]);
        assert_eq!(calls(&e, FREE_00AA1460), vec![vec![this, 0x70]]);
        // The dynamic texture data's destructor: slot 0x160, no surface, 0x74 bytes.
        e.call_log = Some(vec![]);
        e.call(0x00e9_0300, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), NI_DX9_DYNAMIC_TEXTURE_DATA_VTABLE);
        assert_eq!(calls(&e, OWN + 0x160), vec![vec![renderer, 0x0e08_0000]]);
        assert!(calls(&e, FREE_00AA1460).is_empty());
    }

    /// A texture whose size slots answer `width` and `height` and whose request words are set.
    fn texture(e: &mut Engine, width: u32, height: u32) -> u32 {
        let texture = object(e, 0x00eb_0000, 0x80);
        e.register_double(0x00eb_0094, move |_, _| word(width));
        e.register_double(0x00eb_0098, move |_, _| word(height));
        for i in 0..3u32 {
            e.mem.set_u32(texture + 0x18 + 4 * i, 0x40 + i);
        }
        texture
    }

    /// A renderer with a device and a picked pixel format (+0xc 0x15, byte +1 32 bits).
    fn renderer(e: &mut Engine) -> (u32, u32) {
        let renderer = e.mem.alloc(0x900);
        let device = object(e, DEVICE, 8);
        e.mem.set_u32(renderer + RENDERER_DEVICE, device);
        let format = e.mem.alloc(0x44);
        e.mem.set_u32(format + 0xc, 0x15);
        e.mem.set_u8(format + 1, 32);
        e.register_double(PICK_FORMAT_00E89ED0, move |_, _| word(format));
        quiet(e, REPORT_00A29680);
        e.register(ERROR_STRING_00E89D50, |_, _| word(0x0e0e_0000));
        (renderer, format)
    }

    #[test]
    fn fn_00e900c0_creates_the_cube_surface() {
        let mut e = Engine::new();
        let (renderer, format) = renderer(&mut e);
        let cube = e.mem.alloc(8);
        e.register_double(DEVICE + CREATE_CUBE_TEXTURE, move |e, a| {
            e.mem.set_u32(a[6], cube);
            word(0)
        });
        let tex = texture(&mut e, 64, 0);
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x60, renderer);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_00c0, &args![this, tex]).u32(), format);
        assert_eq!(e.mem.u32(this + 0xc), 64);
        assert_eq!(e.mem.u32(this + 0x10), 64);
        assert_eq!(e.mem.u32(this + 0x68), 1);
        assert_eq!(e.mem.u32(this + 0x64), cube);
        assert_eq!(calls(&e, PICK_FORMAT_00E89ED0)[0][1], renderer + 0x808);
        let device = e.mem.u32(renderer + RENDERER_DEVICE);
        assert_eq!(
            calls(&e, DEVICE + CREATE_CUBE_TEXTURE)[0][..6],
            [device, 64, 1, 1, 0x15, 0]
        );
        // The texture's own format with byte +0x41; a failure clears +0x64.
        e.mem.set_u8(tex + 0x41, 1);
        e.mem.set_u32(tex + 0x44, 0x33);
        e.register(DEVICE + CREATE_CUBE_TEXTURE, |_, _| word(0x8876_0001));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_00c0, &args![this, tex]).u32(), 0);
        assert_eq!(calls(&e, DEVICE + CREATE_CUBE_TEXTURE)[0][4], 0x33);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_SURF_FAILED_010F0550, 0x0e0e_0000]]
        );
        assert_eq!(e.call(0x00e9_00c0, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn fn_00e901a0_makes_six_faces_and_counts_the_memory() {
        let mut e = Engine::new();
        e.map(TARGET_BYTES_0126F9E0 & !0xfff, 0x1000);
        let (renderer, _) = renderer(&mut e);
        e.register(DEVICE + CREATE_CUBE_TEXTURE, |_, _| word(0));
        let tex = texture(&mut e, 16, 0);
        for face in 0..6u32 {
            e.mem.set_u32(tex + 0x4c + 4 * face, 0x100 + face);
        }
        e.register(TEXTURE_DATA_CONSTRUCT_00E8A260, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            e.mem.set_u32(a[0] + 0x60, a[2]);
            Ret::default()
        });
        e.register(NEW_OBJECT, |e, a| word(e.mem.alloc(a[0])));
        let faces = Rc::new(RefCell::new(vec![]));
        let seen = faces.clone();
        e.register_double(MAKE_CUBE_FACE_00E7D390, move |e, a| {
            seen.borrow_mut().push((a[1], a[2], e.mem.u32(a[4])));
            word(0x0e05_0000 + a[1])
        });
        // The class table's slot 0x9c (exe data) answers 0x9c.
        e.map(NI_DX9_RENDERED_CUBE_MAP_DATA_VTABLE & !0xfff, 0x1000);
        e.mem
            .set_u32(NI_DX9_RENDERED_CUBE_MAP_DATA_VTABLE + 0x9c, 0x00ea_009c);
        e.register(0x00ea_009c, |_, _| word(0x9c));
        e.call_log = Some(vec![]);
        let block = e.call(0x00e9_01a0, &args![tex, renderer]).u32();
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(block), NI_DX9_RENDERED_CUBE_MAP_DATA_VTABLE);
        assert_eq!(e.mem.u32(tex + 0x24), block);
        assert_eq!(
            *faces.borrow(),
            (0..6u32)
                .map(|f| (f, if f == 0 { 0 } else { 0x0e05_0000 }, 0x100 + f))
                .collect::<Vec<_>>()
        );
        // 4 bytes per pixel, 16 by 16, six faces: 6144 bytes, padded to 8192.
        assert_eq!(e.mem.u32(TARGET_BYTES_0126F9E0), 6144);
        assert_eq!(e.mem.u32(TARGET_PADDING_0126F9E4), 2048);
        // Slot 0x9c twice per face; the face maker gets its second answer.
        assert_eq!(calls(&e, 0x00ea_009c).len(), 12);
        assert!(calls(&e, MAKE_CUBE_FACE_00E7D390)
            .iter()
            .all(|c| c[0] == 0x9c));
    }

    #[test]
    fn fn_00e902e0_and_fn_00e902f0_keep_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x74);
        e.call(0x00e9_02f0, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(this + 0x70), 1);
        assert_eq!(e.call(0x00e9_02e0, &args![this]).u8(), 1);
    }

    #[test]
    fn fn_00e90340_creates_one_or_two_textures() {
        let mut e = Engine::new();
        let (renderer, format) = renderer(&mut e);
        let made = Rc::new(RefCell::new(0x0e00_1000u32));
        let next = made.clone();
        e.register_double(DEVICE + CREATE_TEXTURE, move |e, a| {
            let value = *next.borrow();
            *next.borrow_mut() += 0x10;
            e.mem.set_u32(a[7], value);
            word(0)
        });
        let tex = texture(&mut e, 32, 16);
        let this = e.mem.alloc(0x74);
        e.mem.set_u32(this + 0x60, renderer);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_0340, &args![this, tex, 1u32]).u32(), format);
        assert_eq!([e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)], [32, 16]);
        assert_eq!(e.mem.u32(this + 0x14 + 0xc), 0x15);
        assert_eq!(calls(&e, PICK_FORMAT_00E89ED0)[0][1], renderer + 0x700);
        let device = e.mem.u32(renderer + RENDERER_DEVICE);
        assert_eq!(
            calls(&e, DEVICE + CREATE_TEXTURE)[0][..7],
            [device, 32, 16, 1, 0x200, 0x15, 0]
        );
        assert_eq!(e.mem.u32(this + 0x64), 0x0e00_1000);
        // Not dynamic: usage 0 at +0x64, then pool 2 at +0x6c.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_0340, &args![this, tex, 0u32]).u32(), format);
        let created = calls(&e, DEVICE + CREATE_TEXTURE);
        assert_eq!([created[0][4], created[0][6]], [0, 0]);
        assert_eq!([created[1][4], created[1][6]], [0, 2]);
        assert_eq!(e.mem.u32(this + 0x64), 0x0e00_1010);
        assert_eq!(e.mem.u32(this + 0x6c), 0x0e00_1020);
        // The second failing clears +0x6c.
        let count = Rc::new(RefCell::new(0u32));
        let tries = count.clone();
        e.register_double(DEVICE + CREATE_TEXTURE, move |e, a| {
            *tries.borrow_mut() += 1;
            if *tries.borrow() == 2 {
                return word(0x8876_0001);
            }
            e.mem.set_u32(a[7], 0x0e00_2000);
            word(0)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_0340, &args![this, tex, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![DYNAMIC_SURF_FAILED_010F06B0, 0x0e0e_0000]]
        );
        assert_eq!(e.call(0x00e9_0340, &args![this, 0u32, 0u32]).u32(), 0);
    }

    #[test]
    fn fn_00e904c0_refuses_tiled_textures_and_builds_dynamic_data() {
        let mut e = Engine::new();
        e.map(RENDERER_011F4748 & !0xfff, 0x1000);
        let (renderer, _) = renderer(&mut e);
        e.mem.set_u32(RENDERER_011F4748, renderer);
        e.mem.set_u8(renderer + 0x6f5, 1);
        e.register(DEVICE + CREATE_TEXTURE, |_, _| word(0));
        e.register(NEW_OBJECT, |e, a| word(e.mem.alloc(a[0])));
        e.register(TEXTURE_DATA_CONSTRUCT_00E8A260, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            e.mem.set_u32(a[0] + 0x60, a[2]);
            Ret::default()
        });
        let tex = texture(&mut e, 8, 8);
        e.mem.set_u8(tex + 0x38, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_04c0, &args![tex]).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![TILED_DYNAMIC_FAILED_010F06F8]]
        );
        e.mem.set_u8(tex + 0x38, 0);
        let block = e.call(0x00e9_04c0, &args![tex]).u32();
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(block), NI_DX9_DYNAMIC_TEXTURE_DATA_VTABLE);
        assert_eq!(e.mem.u32(tex + 0x24), block);
        // The renderer's byte asked for one dynamic texture.
        assert_eq!(calls(&e, DEVICE + CREATE_TEXTURE)[0][4], 0x200);
    }

    #[test]
    fn fn_00e90550_answers_the_class_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e9_0550, &args![0u32]).u32(), 0x0126_f9f0);
    }

    fn constants(e: &mut Engine) {
        for (at, value) in [
            (MINUS_32767_0109ED88, -32767.0f64),
            (SCALE_127_0102DFE0, 127.0),
            (TENTH_0101FFA0, 0.1),
            (RAND_MAX_01029790, 32767.0),
            (ONE_01012070, 1.0),
        ] {
            e.map(at & !0xfff, 0x1000);
            e.set_global(at, value);
        }
    }

    #[test]
    fn fn_00e90590_sets_a_random_colour_and_renders() {
        let mut e = Engine::new();
        constants(&mut e);
        let values = Rc::new(RefCell::new(vec![0u32, 32767, 16384]));
        let source = values.clone();
        e.register_double(RAND, move |_, _| word(source.borrow_mut().remove(0)));
        let group = e.mem.alloc(8);
        e.register_double(MAKE_STATE_GROUP_00E7F5D0, move |_, _| word(group));
        quiet(&mut e, GROUP_SET_STATE_00E7F430);
        e.register(SHADER_RENDER_00A8F3E0, |_, _| word(0x5a));
        let holder = e.mem.alloc(0x34);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, holder);
        e.call_log = Some(vec![]);
        let got = e.call(
            0x00e9_0590,
            &args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(got.u32(), 0x5a);
        assert_eq!(calls(&e, MAKE_STATE_GROUP_00E7F5D0).len(), 1);
        // rand 0 gives 0x80, 32767 gives 0x80 - (-127) = 0xff, 16384 gives 0x80 - (-63) = 0xbf.
        let set = calls(&e, GROUP_SET_STATE_00E7F430);
        assert_eq!(set[0], vec![group, 0x3c, 0xff80_ffbf, 1]);
        let rest: Vec<u32> = set[1..].iter().map(|c| c[1]).collect();
        assert_eq!(rest, vec![0x1d, 0x1b, 0xf, 0x34]);
        assert_eq!(
            calls(&e, SHADER_RENDER_00A8F3E0),
            vec![vec![this, 1, 2, 3, 4, 5, 6, 7]]
        );
    }

    #[test]
    fn fn_00e90760_jitters_the_translation() {
        let mut e = Engine::new();
        constants(&mut e);
        let values = Rc::new(RefCell::new(vec![32767u32, 0, 16383]));
        let source = values.clone();
        e.register_double(RAND, move |_, _| word(source.borrow_mut().remove(0)));
        let seen = Rc::new(RefCell::new(vec![]));
        let copy = seen.clone();
        e.register_double(RENDER_PASS_00E807D0, move |e, a| {
            for i in 0..3u32 {
                copy.borrow_mut()
                    .push(f32::from_bits(e.mem.u32(a[7] + 0x24 + 4 * i)));
            }
            Ret::default()
        });
        let transform = e.mem.alloc(0x34);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_u32(transform + 0x24 + 4 * i as u32, v.to_bits());
        }
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 0xc, 50.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(
            0x00e9_0760,
            &args![
                0x0e00_0001u32,
                1u32,
                2u32,
                3u32,
                4u32,
                5u32,
                6u32,
                transform,
                bound
            ],
        );
        let call = &calls(&e, RENDER_PASS_00E807D0)[0];
        assert_eq!(&call[..7], &[0x0e00_0001, 1, 0, 0, 4, 5, 6]);
        assert_eq!(call[8], bound);
        let scale = f64::from(f32::from_bits(x87_store32(x87_mul(50.0, 0.1))));
        let ratio = |r: f64| f64::from(((r + r) / 32767.0 - 1.0) as f32);
        let expected: Vec<f32> = [(10.0, 32767.0), (20.0, 0.0), (30.0, 16383.0)]
            .iter()
            .map(|&(t, r)| (ratio(r) * scale + t) as f32)
            .collect();
        assert_eq!(*seen.borrow(), expected);
        // The caller's transform is not changed.
        assert_eq!(f32::from_bits(e.mem.u32(transform + 0x24)), 10.0);
    }

    #[test]
    fn fn_00e90760_answers_the_render_pass_eax() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(RAND, |_, _| word(0));
        e.register(RENDER_PASS_00E807D0, |_, _| word(6));
        let transform = e.mem.alloc(0x34);
        let bound = e.mem.alloc(0x10);
        let words = args![
            0x0e00_0001u32,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32,
            transform,
            bound
        ];
        assert_eq!(e.call(0x00e9_0760, &words).u32(), 6);
    }

    #[test]
    fn fn_00e90d20_answers_the_renderer_data_format() {
        let mut e = Engine::new();
        let renderer = object(&mut e, OWN, 8);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, renderer);
        let tex = e.mem.alloc(0x30);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(data + 0x24, 0x21);
        e.mem.set_u32(tex + 0x24, data);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_0d20, &args![this, tex]).u32(), 0x21);
        // Without data: the source is offered to the renderer, which makes it.
        e.mem.set_u32(tex + 0x24, 0);
        e.register(SOURCE_OF_00653290, |_, _| word(0x0e0f_0000));
        e.register_double(OWN + 0x13c, move |e, _| {
            e.mem.set_u32(tex + 0x24, data);
            word(1)
        });
        assert_eq!(e.call(0x00e9_0d20, &args![this, tex]).u32(), 0x21);
        assert_eq!(
            calls(&e, SOURCE_OF_00653290),
            vec![vec![tex, SOURCE_KIND_011F444C]]
        );
        // Refused: 0x16.
        e.mem.set_u32(tex + 0x24, 0);
        e.register(OWN + 0x13c, |_, _| word(0x100));
        assert_eq!(e.call(0x00e9_0d20, &args![this, tex]).u32(), 0x16);
        e.register(SOURCE_OF_00653290, |_, _| word(0));
        assert_eq!(e.call(0x00e9_0d20, &args![this, tex]).u32(), 0x16);
    }

    /// A render state with its own table and a device.
    fn state(e: &mut Engine) -> u32 {
        let this = object(e, OWN, 0x1248);
        let device = object(e, DEVICE, 8);
        e.mem.set_u32(this + STATE_DEVICE, device);
        e.call_log = Some(vec![]);
        this
    }

    #[test]
    fn vertex_format_slots_keep_and_restore() {
        let mut e = Engine::new();
        let this = state(&mut e);
        let device = e.mem.u32(this + STATE_DEVICE);
        e.mem.set_u32(this + 0x1104, 0x12);
        e.mem.set_u32(this + 0x110c, 0x34);
        e.call(0x00e9_0db0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1108), 0x12);
        assert_eq!(e.mem.u32(this + 0x1110), 0x34);
        // The same format again while no declaration is set: nothing.
        e.call(0x00e9_0ec0, &args![this, 0x12u32, 1u32]);
        assert!(calls(&e, DEVICE + SET_FVF).is_empty());
        e.call(0x00e9_0ec0, &args![this, 0x0u32, 1u32]);
        assert!(calls(&e, DEVICE + SET_FVF).is_empty());
        // A new one with the keep byte.
        e.mem.set_u8(this + 0x1100, 1);
        e.call(0x00e9_0ec0, &args![this, 0x12u32, 1u32]);
        assert_eq!(calls(&e, DEVICE + SET_FVF), vec![vec![device, 0x12]]);
        assert_eq!(e.mem.u8(this + 0x1100), 0);
        assert_eq!(e.call(0x00e9_0f20, &args![this]).u32(), 0x12);
        e.mem.set_u8(this + 0x1100, 1);
        assert_eq!(e.call(0x00e9_0f20, &args![this]).u32(), 0);
        e.mem.set_u8(this + 0x1100, 0);
        e.call(0x00e9_0f40, &args![this]);
        assert_eq!(calls(&e, OWN + 0x9c), vec![vec![this, 0x12, 0]]);
        e.call(0x00e9_1040, &args![this]);
        assert_eq!(calls(&e, OWN + 0xac), vec![vec![this, 0x34, 0]]);
        // A format going away becomes 2 where it was current or kept.
        e.call_log = Some(vec![]);
        e.call(0x00e9_0f60, &args![this, 0x12u32]);
        assert_eq!(e.mem.u32(this + 0x1104), 2);
        assert_eq!(e.mem.u32(this + 0x1108), 2);
        assert_eq!(calls(&e, DEVICE + SET_FVF), vec![vec![device, 2]]);
        e.mem.set_u8(this + 0x1100, 1);
        e.mem.set_u32(this + 0x1104, 5);
        e.call(0x00e9_0f60, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0x1104), 5);
    }

    #[test]
    fn render_state_small_slots() {
        let mut e = Engine::new();
        let this = state(&mut e);
        e.call(0x00e9_0dd0, &args![this]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|&a| a & 0xffff_0000 == OWN)
            .collect();
        assert_eq!(order, vec![OWN + 0xa4, OWN + 0xb4]);
        e.mem.set_u8(this + 0x1114, 9);
        assert_eq!(e.call(0x00e9_0df0, &args![this]).u8(), 9);
    }

    #[test]
    fn sampler_state_reads_and_resends() {
        let mut e = Engine::new();
        e.map(SAMPLER_STATE_SLOTS_0126F92C & !0xfff, 0x1000);
        e.mem.set_u16(SAMPLER_STATE_SLOTS_0126F92C + 2 * 6, 3);
        e.mem.set_u16(SAMPLER_STATE_SLOTS_0126F92C + 2 * 7, 5);
        let this = state(&mut e);
        e.mem.set_u32(this + 0xe20 + 8 * (2 * 5 + 3), 0x77);
        e.mem.set_u32(this + 0xe24 + 8 * (2 * 5 + 3), 0x88);
        assert_eq!(e.call(0x00e9_10f0, &args![this, 2u32, 6u32]).u32(), 0x77);
        assert_eq!(e.call(0x00e9_10f0, &args![this, 2u32, 7u32]).u32(), 0);
        e.call(0x00e9_1120, &args![this, 2u32, 6u32]);
        e.call(0x00e9_1120, &args![this, 2u32, 7u32]);
        assert_eq!(calls(&e, OWN + 0xcc), vec![vec![this, 2, 6, 0x88, 0]]);
    }

    #[test]
    fn fn_00e91190_switches_software_processing_when_allowed() {
        let mut e = Engine::new();
        let this = state(&mut e);
        let device = e.mem.u32(this + STATE_DEVICE);
        let renderer = e.mem.alloc(0x600);
        e.mem.set_u32(this + 0x10fc, renderer);
        e.mem.set_u32(this + 0x10e0, 0x0e0c_0000);
        // Not allowed by the renderer: nothing.
        e.call(0x00e9_1190, &args![this, 1u32]);
        assert!(calls(&e, DEVICE + 0x134).is_empty());
        e.mem.set_u8(renderer + 0x5d5, 1);
        e.call(0x00e9_1190, &args![this, 0x101u32]);
        assert_eq!(calls(&e, DEVICE + 0x134), vec![vec![device, 1]]);
        assert_eq!(calls(&e, OWN + 0x98), vec![vec![this, 0x0e0c_0000]]);
        assert_eq!(e.mem.u8(this + 0x1114), 1);
        // The same value again: nothing more.
        e.call(0x00e9_1190, &args![this, 1u32]);
        assert_eq!(calls(&e, DEVICE + 0x134).len(), 1);
    }

    #[test]
    fn fn_00e916d0_makes_the_renderer_data() {
        let mut e = Engine::new();
        e.register(NEW_OBJECT, |e, a| word(e.mem.alloc(a[0])));
        quiet(&mut e, RENDERER_DATA_CONSTRUCT_00A981D0);
        e.call_log = Some(vec![]);
        let block = e.call(0x00e9_16d0, &args![]).u32();
        assert_eq!(
            e.mem.u32(block),
            PERSISTENT_SRC_TEXTURE_RENDERER_DATA_VTABLE
        );
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![0x94]]);
        assert_eq!(
            calls(&e, RENDERER_DATA_CONSTRUCT_00A981D0),
            vec![vec![block]]
        );
        e.register(NEW_OBJECT, |_, _| word(0));
        assert_eq!(e.call(0x00e9_16d0, &args![]).u32(), 0);
    }
}
