//! `BSShader/bsbatchrenderer.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSBatchRenderer` (Xbox PDB), 0x98 bytes (the size its users
    /// allocate); the offsets used below match the PC constructor.
    pub struct BSBatchRenderer: 0x98 {
        /// `pRenderPassA` (Xbox PDB): the pass groups (0x68 bytes each,
        /// after a count word).
        0x08 pRenderPassA: u32,
        /// `RenderPassMap` (Xbox PDB), an `NiTMap`: its vtable.
        0x0c RenderPassMap_vtable: u32,
        /// `RenderPassMap.m_uiHashSize` (Xbox PDB).
        0x10 RenderPassMap_m_uiHashSize: u32,
        /// `RenderPassMap.m_ppkHashTable` (Xbox PDB).
        0x14 RenderPassMap_m_ppkHashTable: u32,
        /// `RenderPassMap.m_kAllocator.m_uiCount` (Xbox PDB).
        0x18 RenderPassMap_m_uiCount: u32,
        /// `uiRenderPassCount` (Xbox PDB).
        0x1c uiRenderPassCount: u32,
        /// `pInstanceRenderer` (Xbox PDB).
        0x28 pInstanceRenderer: u32,
        /// `kActivePassIndexList` (Xbox PDB), a `BSSimpleList`: its item.
        0x2c kActivePassIndexList_item: u16,
        /// `kActivePassIndexList`'s next link.
        0x30 kActivePassIndexList_next: u32,
        /// `iGroupingAlphas` (Xbox PDB).
        0x34 iGroupingAlphas: u32,
    }
}

/// The base-class constructor (engine map unit `niobject.cpp`; thiscall,
/// no arguments).
const OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// The class's vtable.
const BATCH_RENDERER_VTABLE: u32 = 0x010b_7dc8;
/// The vtable the inlined `NiTMap` constructor sets first.
const MAP_BASE_VTABLE: u32 = 0x0106_cf88;
/// The map vtable it sets last.
const MAP_VTABLE: u32 = 0x0106_cf68;
/// The allocator the inlined map constructor calls for its buckets (cdecl,
/// one word: the size).
const ALLOCATE_BUCKETS: u32 = 0x00aa_1070;
/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// The compiler's `eh vector constructor iterator` (stdcall: array, element
/// size, count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// Constructor of one alpha-group list (0x0C bytes).
const ALPHA_LIST_CONSTRUCTOR: u32 = 0x00b9_98e0;
/// Destructor of one alpha-group list.
const ALPHA_LIST_DESTRUCTOR: u32 = 0x00b9_98f0;
/// Constructor of one pass group (0x68 bytes).
const PASS_GROUP_CONSTRUCTOR: u32 = 0x00b9_a530;
/// Destructor of one pass group.
const PASS_GROUP_DESTRUCTOR: u32 = 0x00b9_9bb0;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// Constructor of the 4-byte instance renderer (thiscall, no arguments).
const INSTANCE_RENDERER_CONSTRUCTOR: u32 = 0x00bd_9120;
/// The renderer's method `00b99bc0` (this unit, not yet translated;
/// thiscall, one word), called with 0.
const RENDERER_METHOD_B99BC0: u32 = 0x00b9_9bc0;
/// Constructor of a 0x24-byte geometry group (this unit, not yet
/// translated; thiscall, one word).
const GEOMETRY_GROUP_CONSTRUCTOR: u32 = 0x00b9_a560;
/// `NiTPointerListBase<..>::RemoveAll` (engine map; thiscall).
const POINTER_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// Word of the thread's TLS block the constructor sets to 0x26 and restores.
const TLS_WORD_2B4: u32 = 0x2b4;

/// The group slots +0x3C..+0x74 (13 `pGeometryGroupsA`, 2 `pAlphaGroupA`,
/// Xbox PDB) and the word each group of the kind-99 renderer is built with.
const ALL_GROUPS: [(u32, u32); 15] = [
    (0x3c, 0x2f7),
    (0x40, 0x2f7),
    (0x44, 0x2f7),
    (0x48, 0x2f7),
    (0x4c, 0x2f7),
    (0x50, 0x2f7),
    (0x54, 6),
    (0x58, 1),
    (0x5c, 4),
    (0x60, 0x2f7),
    (0x64, 0x2f7),
    (0x68, 6),
    (0x6c, 4),
    (0x70, 0x2f7),
    (0x74, 0x2f7),
];

/// Allocates a 0x24-byte geometry group and constructs it with `word`
/// (null when the allocation fails).
fn new_geometry_group(e: &mut Engine, word: u32) -> u32 {
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x24]).u32();
    if block != 0 {
        e.call(GEOMETRY_GROUP_CONSTRUCTOR, &[block, word]).u32()
    } else {
        0
    }
}

/// The tail the kinds 99 and 101 share: sets bit 0 of the first byte of the
/// two alpha groups (+0x70, +0x74) and empties the two alpha-group lists
/// (+0x78, +0x84).
fn finish_alpha_groups(e: &mut Engine, base: u32) {
    for slot in [0x70, 0x74] {
        let group = e.mem.u32(base + slot);
        let flags = e.mem.u8(group);
        e.mem.set_u8(group, flags | 1);
    }
    e.call(POINTER_LIST_REMOVE_ALL, &[base + 0x78]);
    e.call(POINTER_LIST_REMOVE_ALL, &[base + 0x84]);
}

// Translated from 00b9a5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSBatchRenderer` constructor (unnamed in the engine map; class and
/// fields from the Xbox PDB): runs the `NiObject` base constructor, sets the
/// vtable, builds the render-pass map (37 buckets), clears the active pass
/// list, constructs the two alpha-group lists, and with word +0x2B4 of the
/// thread block set to 0x26 builds the instance renderer and `pass_count`
/// pass groups (0x68 bytes each after a count word; the size saturates),
/// sets the pass count, clears `iGroupingAlphas` and calls `00b99bc0(0)`.
/// Then by `kind`: 99 builds all 15 geometry groups; 100 clears them; 101
/// clears all but +0x60, +0x70 and +0x74, which it builds (0x2F7); 99 and
/// 101 then flag the two alpha groups and empty the alpha-group lists. Any
/// other kind leaves the groups as they are. The TLS word is restored on
/// every path. Returns `this`.
// Translation incomplete: 00b9a5b0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00b9a5b0(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass_count: u32,
    kind: i32,
) -> Ptr<BSBatchRenderer> {
    let base = this.addr();
    e.call(OBJECT_CONSTRUCTOR, &[base]);
    e.mem.set_u32(base, BATCH_RENDERER_VTABLE);
    // The inlined NiTMap constructor (37 buckets).
    e.set(this, BSBatchRenderer::RenderPassMap_vtable, MAP_BASE_VTABLE);
    e.set(this, BSBatchRenderer::RenderPassMap_m_uiHashSize, 0x25);
    e.set(this, BSBatchRenderer::RenderPassMap_m_uiCount, 0);
    let buckets = e.call(ALLOCATE_BUCKETS, &[0x94]).u32();
    let hash_size = e.get(this, BSBatchRenderer::RenderPassMap_m_uiHashSize);
    e.set(this, BSBatchRenderer::RenderPassMap_m_ppkHashTable, buckets);
    e.call(MEMSET, &[buckets, 0, hash_size.wrapping_mul(4)]);
    e.set(this, BSBatchRenderer::RenderPassMap_vtable, MAP_VTABLE);
    e.set(this, BSBatchRenderer::kActivePassIndexList_item, 0);
    e.set(this, BSBatchRenderer::kActivePassIndexList_next, 0);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &[
            base + 0x78,
            0xc,
            2,
            ALPHA_LIST_CONSTRUCTOR,
            ALPHA_LIST_DESTRUCTOR,
        ],
    );
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_WORD_2B4);
    e.mem.set_u32(tls + TLS_WORD_2B4, 0x26);
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 4]).u32();
    let instance = if block != 0 {
        e.call(INSTANCE_RENDERER_CONSTRUCTOR, &[block]).u32()
    } else {
        0
    };
    e.set(this, BSBatchRenderer::pInstanceRenderer, instance);
    let groups_size = pass_count.checked_mul(0x68).unwrap_or(u32::MAX);
    let size = groups_size.checked_add(4).unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    let groups = if block != 0 {
        e.mem.set_u32(block, pass_count);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &[
                block + 4,
                0x68,
                pass_count,
                PASS_GROUP_CONSTRUCTOR,
                PASS_GROUP_DESTRUCTOR,
            ],
        );
        block + 4
    } else {
        0
    };
    e.set(this, BSBatchRenderer::pRenderPassA, groups);
    e.set(this, BSBatchRenderer::uiRenderPassCount, pass_count);
    e.set(this, BSBatchRenderer::iGroupingAlphas, 0);
    e.call(RENDERER_METHOD_B99BC0, &[base, 0]);
    match kind {
        99 => {
            for (slot, word) in ALL_GROUPS {
                let group = new_geometry_group(e, word);
                e.mem.set_u32(base + slot, group);
            }
            finish_alpha_groups(e, base);
        }
        100 => {
            for (slot, _) in ALL_GROUPS {
                e.mem.set_u32(base + slot, 0);
            }
        }
        101 => {
            for (slot, _) in &ALL_GROUPS[..9] {
                e.mem.set_u32(base + slot, 0);
            }
            let group = new_geometry_group(e, 0x2f7);
            e.mem.set_u32(base + 0x60, group);
            for slot in [0x64, 0x68, 0x6c] {
                e.mem.set_u32(base + slot, 0);
            }
            let group = new_geometry_group(e, 0x2f7);
            e.mem.set_u32(base + 0x70, group);
            let group = new_geometry_group(e, 0x2f7);
            e.mem.set_u32(base + 0x74, group);
            finish_alpha_groups(e, base);
        }
        _ => {}
    }
    e.mem.set_u32(tls + TLS_WORD_2B4, saved);
    this
}

/// The vtable of a `BSTPersistentList` element (0x14 bytes), as its
/// constructor `00b988c0` sets it.
const PERSISTENT_LIST_VTABLE: u32 = 0x010b_7dc0;
/// The element's clean-up `00b63e90` (`bsshaderaccumulator.cpp`; thiscall).
const PERSISTENT_LIST_CLEAR: u32 = 0x00b6_3e90;
/// Constructor of one `BSTPersistentList` element (0x14 bytes).
const PERSISTENT_LIST_CONSTRUCTOR: u32 = 0x00b9_88c0;
/// Destructor of one `BSTPersistentList` element.
const PERSISTENT_LIST_DESTRUCTOR: u32 = 0x00b6_56b0;
/// The compiler's `eh vector destructor iterator` (stdcall: array, element
/// size, count, destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The clear of `kActivePassIndexList` (`00b64ec0`, no map name; thiscall).
const ACTIVE_LIST_CLEAR: u32 = 0x00b6_4ec0;

// Translated from 00b998e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of one alpha-group list (`NiTPointerList`, 0x0C bytes; Ghidra
/// made no function here, read from the bytes): clears its count, head and
/// tail. Returns `this`.
pub fn fn_00b998e0(e: &mut Engine, this: Ptr<crate::types::NiTPointerList>) -> u32 {
    use crate::types::NiTPointerList;
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
    this.addr()
}

// Translated from 00b998f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of one alpha-group list (Ghidra made no function here; read
/// from the bytes): jumps to `NiTPointerListBase<..>::RemoveAll`
/// (`00b71300`).
pub fn fn_00b998f0(e: &mut Engine, this: Ptr) {
    e.call(POINTER_LIST_REMOVE_ALL, &[this.addr()]);
}

// Translated from 00b9a530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of one pass group (0x68 bytes; Ghidra made no function
/// here, read from the bytes): constructs its five `BSTPersistentList`
/// elements (0x14 bytes each, `00b988c0`/`00b656b0`) and clears the word
/// at +0x64. Returns `this`.
pub fn fn_00b9a530(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &[
            this.addr(),
            0x14,
            5,
            PERSISTENT_LIST_CONSTRUCTOR,
            PERSISTENT_LIST_DESTRUCTOR,
        ],
    );
    e.mem.set_u32(this.addr() + 0x64, 0);
    this
}

// Translated from 00b99bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of one pass group (Ghidra made no function here; read from
/// the bytes): destroys its five elements through the `eh vector destructor
/// iterator` (`00b656b0`), whose result it returns.
pub fn fn_00b99bb0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &[this.addr(), 0x14, 5, PERSISTENT_LIST_DESTRUCTOR],
    )
    .u32()
}

/// The first item of the render-pass map from bucket `start` on (0 when
/// none): the map's bucket count is read once, as is its bucket array.
fn map_item_from(e: &Engine, start: u32, count: u32, buckets: u32) -> u32 {
    let mut index = start;
    while index < count {
        let item = e.mem.u32(buckets + index * 4);
        if item != 0 {
            return item;
        }
        index += 1;
    }
    0
}

// Translated from 00b99bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the passes in use (unnamed in the engine map): for every item of
/// the render-pass map (`NiTMap`, items `next`, key, value) resets the pass
/// group its value indexes (0x68 bytes each from `pRenderPassA`): each of
/// its five elements that holds something (+0x10) is cleaned (`00b63e90`),
/// its +0x04 moved to +0x0C and +0x04, +0x08, +0x10 cleared; the group's
/// +0x64 is cleared. The next item comes from the item's link or, at a
/// bucket's end, from the buckets after the key's (the map's virtual
/// `KeyToHashIndex`, slot +4). Then clears the active pass list (+0x2C,
/// `00b64ec0`). The one stack word (`RET 4`) is never read.
pub fn fn_00b99bc0(e: &mut Engine, this: Ptr<BSBatchRenderer>, _unused_1: u32) {
    let base = this.addr();
    let count = e.get(this, BSBatchRenderer::RenderPassMap_m_uiHashSize);
    let mut item = if count != 0 {
        let buckets = e.get(this, BSBatchRenderer::RenderPassMap_m_ppkHashTable);
        map_item_from(e, 0, count, buckets)
    } else {
        0
    };
    while item != 0 {
        let link = e.mem.u32(item);
        let pass = e.mem.u32(item + 8);
        let next = if link != 0 {
            link
        } else {
            let key = e.mem.u32(item + 4);
            let map = base + 0xc;
            let index = e.vcall(map, 4, &[key]).u32().wrapping_add(1);
            let count = e.get(this, BSBatchRenderer::RenderPassMap_m_uiHashSize);
            if index < count {
                let buckets = e.get(this, BSBatchRenderer::RenderPassMap_m_ppkHashTable);
                map_item_from(e, index, count, buckets)
            } else {
                0
            }
        };
        let groups = e.get(this, BSBatchRenderer::pRenderPassA);
        let group = pass.wrapping_mul(0x68).wrapping_add(groups);
        for element in 0..5 {
            let element = group + element * 0x14;
            if e.mem.u32(element + 0x10) != 0 {
                e.call(PERSISTENT_LIST_CLEAR, &[element]);
                let first = e.mem.u32(element + 4);
                e.mem.set_u32(element + 0xc, first);
                e.mem.set_u32(element + 4, 0);
                e.mem.set_u32(element + 8, 0);
                e.mem.set_u32(element + 0x10, 0);
            }
        }
        e.mem.set_u32(group + 0x64, 0);
        item = next;
    }
    e.call(ACTIVE_LIST_CLEAR, &[base + 0x2c]);
}

// Translated from 00b9a560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x24-byte geometry group (unnamed in the engine map):
/// constructs its `BSTPersistentList` element at +0x0C inline (vtable,
/// three cleared words), clears the 16-bit +0x02, the byte +0x00 and +0x08,
/// stores `word` at +0x04, cleans the element (`00b63e90`), moves its +0x04
/// to +0x0C and clears its +0x04, +0x08 and +0x10. Returns `this`.
pub fn fn_00b9a560(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    let base = this.addr();
    let element = base + 0xc;
    e.mem.set_u32(element, PERSISTENT_LIST_VTABLE);
    e.mem.set_u32(element + 4, 0);
    e.mem.set_u32(element + 8, 0);
    e.mem.set_u32(element + 0xc, 0);
    e.mem.set_u16(base + 2, 0);
    e.mem.set_u8(base, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 4, word);
    e.call(PERSISTENT_LIST_CLEAR, &[element]);
    let first = e.mem.u32(element + 4);
    e.mem.set_u32(element + 4, 0);
    e.mem.set_u32(element + 8, 0);
    e.mem.set_u32(element + 0x10, 0);
    e.mem.set_u32(element + 0xc, first);
    this
}

/// The renderer singleton pointer (`011f9508`).
const RENDERER: u32 = 0x011f_9508;
/// The object at `011f4748` whose +0x288 is the Direct3D device and whose
/// +0x8AC/+0x8B0 are the geometry-group manager and its argument.
const RENDER_HOLDER: u32 = 0x011f_4748;
/// The shader of the pass that is begun, and the pass type.
const CURRENT_SHADER: u32 = 0x011f_fe2c;
const CURRENT_PASS_TYPE: u32 = 0x011f_fe30;
/// A byte flag `BeginPass` sets and the pass renderers clear after the
/// shader's virtual slot +0x110 ran; `EndPass` skips slot +0x114 when set.
const PASS_FLAG_011AD8EC: u32 = 0x011a_d8ec;
/// The camera and the flag the alpha sort compares with.
const SORT_CAMERA: u32 = 0x011f_fe38;
const SORT_SUBTRACT_RADIUS: u32 = 0x011f_fe35;
/// The bound used when an object has none (`011f4288`, four floats).
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// The memory manager's deallocation (`00aa4060`; thiscall, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The list-node allocator (engine map `NiTPointerListBase<..>::AddHead`,
/// `0043a010`; thiscall on a count word, answers a node).
const LIST_NODE_ALLOCATE: u32 = 0x0043_a010;
/// The node pool's lock (`this` of `0040fbf0`), its count and free list,
/// and its refill (`0043a080`, no arguments).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;
const LOCK_ENTER: u32 = 0x0040_fbf0;
const NODE_POOL_REFILL: u32 = 0x0043_a080;
/// The render-pass map's `GetAt` (`00853130`; thiscall: key, out slot;
/// answers whether found) and `SetAt` (`00844700`; thiscall: key, value).
const MAP_GET_AT: u32 = 0x0085_3130;
const MAP_SET_AT: u32 = 0x0084_4700;
/// `BSShaderAccumulator::IsShadowPass` (engine map; cdecl, one word).
const IS_SHADOW_PASS: u32 = 0x00b6_30f0;
/// The render-state setters of `bsrenderstate_xenon.cpp` and `run_00b97900`
/// / `run_00b984f0` (cdecl).
const SET_Z_WRITE_ENABLE: u32 = 0x00b9_7e30;
const SET_DEPTH_BIAS: u32 = 0x00b9_8320;
const SET_ALPHA_TEST_ENABLE: u32 = 0x00b9_7ed0;
const SET_Z_ENABLE: u32 = 0x00b9_7de0;
const SET_ALPHA_TO_COVERAGE: u32 = 0x00b9_8540;
/// The decal depth bias and the flag that selects it.
const DECAL_DEPTH_BIAS: u32 = 0x0120_0458;
const DECAL_DEPTH_BIAS_ENABLED: u32 = 0x011a_d80f;
/// Lock counters of slots 1, 3 and 4 (`011ff9d8` + 4 * slot) and the word
/// at `011ffa24` that `00b9a120` decrements when set, and the
/// alpha-to-coverage counter.
const LOCK_COUNT_1: u32 = 0x011f_f9dc;
const LOCK_COUNT_3: u32 = 0x011f_f9e4;
const LOCK_COUNT_4: u32 = 0x011f_f9e8;
const LOCK_COUNT_011FFA24: u32 = 0x011f_fa24;
const COVERAGE_LOCK: u32 = 0x011f_fa28;
/// Whether the renderer has an alpha-to-coverage switch (a byte).
const COVERAGE_AVAILABLE: u32 = 0x011f_9421;

/// Element `i` (0x14 bytes) of the pass group `pass` (0x68 bytes each from
/// `pRenderPassA`).
fn pass_element(e: &Engine, this: u32, pass: u32, element: u32) -> u32 {
    let groups = e.mem.u32(this + 8);
    pass.wrapping_mul(0x68)
        .wrapping_add(element.wrapping_mul(0x14))
        .wrapping_add(groups)
}

/// The bound (`NiBound`: center, radius) of the geometry a pass points to:
/// its +0x20, or the default bound at `011f4288` when null.
fn bound_of(e: &Engine, geometry: u32) -> u32 {
    let bound = e.mem.u32(geometry.wrapping_add(0x20));
    if bound == 0 {
        DEFAULT_BOUND
    } else {
        bound
    }
}

/// The camera direction floats (+0x68, +0x74, +0x80), each through an
/// `FLD`/`FSTP` into a local (so quieted) and loaded again.
fn camera_direction(e: &Engine, camera: u32) -> [f64; 3] {
    [0x68, 0x74, 0x80].map(|offset| x87_load32(x87_quiet_bits(e.mem.u32(camera + offset))))
}

// Translated from 00b984d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words: every caller pushes two and
/// cleans up two; the second, 0 or 1, is never read): calls
/// `00e879b0(value, 0)` on the renderer's render-state object (renderer
/// +0x8B8).
pub fn fn_00b984d0(e: &mut Engine, value: u32, _unused_1: u32) {
    let renderer = e.mem.u32(RENDERER);
    let object = e.mem.u32(renderer.wrapping_add(0x8b8));
    e.call(0x00e8_79b0, &[object, value, 0]);
}

// Translated from 00b988e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::EndPass` (Xbox PDB; no arguments): when a pass is
/// begun, calls its shader's virtual slot +0x104 with the pass type and,
/// unless the flag at `011ad8ec` is set, slot +0x114; for the pass types
/// 0xFE and 0x10B (sampler 7) or 0x230 (sampler 2) sets that sampler's
/// state 8 to 0 on the device (`IDirect3DDevice9` slot +0x114). Clears the
/// begun shader and type.
pub fn bs_batch_renderer_end_pass(e: &mut Engine) {
    let shader = e.mem.u32(CURRENT_SHADER);
    if shader != 0 {
        let pass_type = e.mem.u32(CURRENT_PASS_TYPE);
        e.vcall(shader, 0x104, &[pass_type]);
        if e.mem.u8(PASS_FLAG_011AD8EC) == 0 {
            let shader = e.mem.u32(CURRENT_SHADER);
            e.vcall(shader, 0x114, &[]);
        }
        let pass_type = e.mem.u32(CURRENT_PASS_TYPE);
        let sampler = match pass_type {
            0xfe | 0x10b => Some(7),
            0x230 => Some(2),
            _ => None,
        };
        if let Some(sampler) = sampler {
            let holder = e.mem.u32(RENDER_HOLDER);
            let device = e.mem.u32(holder.wrapping_add(0x288));
            e.vcall(device, 0x114, &[sampler, 8, 0]);
        }
    }
    e.mem.set_u32(CURRENT_PASS_TYPE, 0);
    e.mem.set_u32(CURRENT_SHADER, 0);
}

/// The pass types `00b98970` looks for.
fn is_listed_pass(value: u32) -> bool {
    matches!(
        value,
        0x28a
            | 0x28d
            | 0x28e
            | 0x291
            | 0x292
            | 0x295
            | 0x296
            | 0x299
            | 0x2ac
            | 0x2af
            | 0x2b4
            | 0x2b5
            | 0x2b0
            | 0x2b3
            | 0x2b8
            | 0x2bb
            | 0x29b
            | 0x29c
            | 0x29f
            | 0x2a0
            | 0x2a3
            | 0x2a4
            | 0x2a7
            | 0x2a8
            | 0x2be
            | 0x2c1
            | 0x2c6
            | 0x2c7
            | 0x2c2
            | 0x2c5
            | 0x2ca
            | 0x2cd
            | 0x2bd
    )
}

// Translated from 00b98970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): whether the active
/// pass list (+0x2C; 16-bit item, next at +4) holds a pass type within
/// `low..=high` that is one of 33 fixed types (the immediates of the
/// code). The list address is tested for 0 first, as the exe does.
pub fn fn_00b98970(e: &mut Engine, this: Ptr<BSBatchRenderer>, low: u32, high: u32) -> bool {
    let mut node = this.addr().wrapping_add(0x2c);
    if node == 0 {
        return false;
    }
    loop {
        let value = u32::from(e.mem.u16(node));
        if value <= high && value >= low && is_listed_pass(value) {
            return true;
        }
        node = e.mem.u32(node + 4);
        if node == 0 {
            return false;
        }
    }
}

// Translated from 00b98ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): the first of the five
/// elements of pass group `pass` whose list is not empty (+0x04) goes to
/// `*first`; answers whether there was one.
pub fn fn_00b98ae0(e: &mut Engine, this: Ptr<BSBatchRenderer>, pass: u32, first: Ptr) -> bool {
    let mut index = 0i32;
    loop {
        let element = pass_element(e, this.addr(), pass, index as u32);
        if e.mem.u32(element + 4) != 0 {
            e.mem.set_u32(first.addr(), index as u32);
            index = 5;
        }
        index += 1;
        if index >= 5 {
            break;
        }
    }
    index == 6
}

/// The distance `00b98b20` sorts by for the pass the word at `entry`
/// points to: the camera direction dotted with the bound's center, minus
/// the radius when `subtract` (x87 order of its first operand).
fn sort_distance_first(e: &Engine, entry: u32, d: [f64; 3], subtract: bool) -> u32 {
    let geometry = e.mem.u32(e.mem.u32(entry));
    let bound = bound_of(e, geometry);
    let p = |i: u32| x87_load32(e.mem.u32(bound + i * 4));
    let sum = x87_add(x87_mul(p(0), d[0]), x87_mul(p(1), d[1]));
    let dot = x87_store32(x87_add(x87_mul(p(2), d[2]), sum));
    if subtract {
        let radius = x87_m32(e.mem.u32(bound + 0xc));
        x87_store32(x87_sub(x87_load32(dot), radius))
    } else {
        dot
    }
}

/// The same for the second operand of `00b98b20`, which keeps the camera
/// direction on the x87 stack (other operand order).
fn sort_distance_second(e: &Engine, entry: u32, d: [f64; 3], subtract: bool) -> u32 {
    let geometry = e.mem.u32(e.mem.u32(entry));
    let bound = bound_of(e, geometry);
    let p = |i: u32| x87_load32(e.mem.u32(bound + i * 4));
    let y = x87_mul(d[1], p(1));
    let x = x87_mul(d[0], p(0));
    let sum = x87_add(x, y);
    let dot = x87_add(sum, x87_mul(d[2], x87_m32(e.mem.u32(bound + 8))));
    if subtract {
        let dot = x87_store32(dot);
        let radius = x87_m32(e.mem.u32(bound + 0xc));
        x87_store32(x87_sub(x87_load32(dot), radius))
    } else {
        x87_store32(dot)
    }
}

// Translated from 00b98b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words; the alpha sort's compare
/// function, `00b99a10` passes it): compares the distances along the sort
/// camera's direction (`011ffe38`; minus the bound's radius when the flag
/// at `011ffe35` is set) of the passes the two entries point to: 0 when
/// equal, 1 when the second is farther, else -1 (also for NaN).
pub fn fn_00b98b20(e: &mut Engine, first: u32, second: u32) -> i32 {
    let subtract = e.mem.u8(SORT_SUBTRACT_RADIUS) != 0;
    let camera = e.mem.u32(SORT_CAMERA);
    let d = camera_direction(e, camera);
    let a = x87_load32(sort_distance_first(e, first, d, subtract));
    let b = x87_load32(sort_distance_second(e, second, d, subtract));
    if b == a {
        0
    } else if b > a {
        1
    } else {
        -1
    }
}

/// `-FLT_MAX` as `00b98c80` loads it for an empty group.
const NO_DISTANCE: u32 = 0x0101_5f5c;
/// The larger-of helper `00b98c80` ends in (`00404010`; cdecl, two floats).
const FLOAT_MAX_00404010: u32 = 0x0040_4010;

// Translated from 00b98c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, three words; answers in `ST0`):
/// the distance of alpha group `group` (+0x70 array) along `camera`'s
/// direction. An empty geometry group (16-bit +0x02 is 0) gives the first
/// item's distance (+0x20) of the alpha-group list (+0x78, 0x0C each)
/// when that list has items, else the float at `01015f5c`. Otherwise the
/// first pass of the geometry group gives the dot product of the camera
/// direction and its bound's center (minus the radius when `subtract`);
/// with items in the alpha-group list the answer is `00404010` of the
/// list's distance and that.
pub fn fn_00b98c80(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    group: u32,
    camera: Ptr,
    subtract: u8,
) -> f64 {
    let base = this.addr();
    let d = camera_direction(e, camera.addr());
    let geometry_group = e
        .mem
        .u32(base.wrapping_add(group.wrapping_mul(4)).wrapping_add(0x70));
    let count = e.mem.u16(geometry_group.wrapping_add(2));
    let list = base
        .wrapping_add(group.wrapping_mul(0xc))
        .wrapping_add(0x78);
    let list_count = e.mem.u32(list + 8);
    if count == 0 {
        if list_count == 0 {
            return x87_load32(e.mem.u32(NO_DISTANCE));
        }
        let head = e.mem.u32(list);
        let item = e.mem.u32(head + 8);
        return x87_load32(e.mem.u32(item + 0x20));
    }
    let first_pass = |e: &Engine| {
        let node = e.mem.u32(geometry_group + 0x10);
        let pass = e.mem.u32(node + 8);
        let geometry = e.mem.u32(pass);
        bound_of(e, geometry)
    };
    if list_count == 0 {
        let bound = first_pass(e);
        let p = |i: u32| x87_load32(e.mem.u32(bound + i * 4));
        let sum = x87_add(x87_mul(p(1), d[1]), x87_mul(p(0), d[0]));
        let dot = x87_store32(x87_add(sum, x87_mul(p(2), d[2])));
        if subtract != 0 {
            let radius = x87_m32(e.mem.u32(bound + 0xc));
            return x87_load32(x87_store32(x87_sub(x87_load32(dot), radius)));
        }
        return x87_load32(dot);
    }
    let head = e.mem.u32(list);
    let item = e.mem.u32(head + 8);
    let list_distance = x87_quiet_bits(e.mem.u32(item + 0x20));
    let bound = first_pass(e);
    let p = |i: u32| x87_load32(e.mem.u32(bound + i * 4));
    let distance = if subtract != 0 {
        let sum = x87_add(x87_mul(p(1), d[1]), x87_mul(p(0), d[0]));
        let dot = x87_store32(x87_add(sum, x87_mul(p(2), d[2])));
        let radius = x87_m32(e.mem.u32(bound + 0xc));
        x87_store32(x87_sub(x87_load32(dot), radius))
    } else {
        let sum = x87_add(
            x87_mul(p(1), d[1]),
            x87_mul(d[0], x87_m32(e.mem.u32(bound))),
        );
        x87_store32(x87_add(sum, x87_mul(p(2), d[2])))
    };
    e.call(FLOAT_MAX_00404010, &[list_distance, distance]).st0
}

// Translated from 00b98e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word: the address of a 16-bit
/// item): puts the item at the front of the 16-bit list `this` (item, next
/// at +4). An empty head just takes it; otherwise the head's item moves
/// into a new 8-byte node (memory manager) linked after the head. A zero
/// item does nothing. When the allocation fails the exe writes the old
/// link through the null node (address 4) and clears the head's link.
pub fn fn_00b98e20(e: &mut Engine, this: Ptr, item: Ptr) {
    let head = this.addr();
    let value = e.mem.u16(item.addr());
    if value == 0 {
        return;
    }
    if e.mem.u16(head) != 0 {
        let node = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 8]).u32();
        if node != 0 {
            let old = e.mem.u16(head);
            e.mem.set_u16(node, old);
            e.mem.set_u32(node + 4, 0);
            let link = e.mem.u32(head + 4);
            e.mem.set_u32(node + 4, link);
            e.mem.set_u32(head + 4, node);
            let value = e.mem.u16(item.addr());
            e.mem.set_u16(head, value);
            return;
        }
        let link = e.mem.u32(head + 4);
        e.mem.set_u32(4, link);
        e.mem.set_u32(head + 4, 0);
        let value = e.mem.u16(item.addr());
        e.mem.set_u16(head, value);
        return;
    }
    e.mem.set_u16(head, value);
}

/// The geometry-group manager's `AddObjectToGroup` (engine map
/// `NiD3DGeometryGroupManager::AddObjectToGroup_ov2`, `00e88dc0`;
/// thiscall, six words).
const GROUP_MANAGER_ADD_OBJECT: u32 = 0x00e8_8dc0;

/// What `RenderPassImmediately_Standard` and `_Skinned` share at the
/// start: tells the renderer holder the geometry's property state (+0x9C)
/// and no skin, and adds the geometry data (and skin) to the
/// geometry-group manager.
fn add_to_geometry_group(e: &mut Engine, geometry: u32, data: u32, skin: u32, skinned: u32) -> u32 {
    let holder = e.mem.u32(RENDER_HOLDER);
    let manager_argument = e.mem.u32(holder + 0x8b0);
    let manager = e.mem.u32(holder + 0x8ac);
    e.mem.set_u32(holder + 0xc, geometry + 0x9c);
    e.mem.set_u32(holder + 0x10, 0);
    e.call(
        GROUP_MANAGER_ADD_OBJECT,
        &[manager, manager_argument, data, skin, skinned, 0, 0],
    );
    holder
}

/// The shader set-up calls the pass types 0xCA, 0xD1, 0xFC/0xFD and 0x102
/// have (thiscall, one word).
const SHADER_SETUP_00B7DDE0: u32 = 0x00b7_dde0;
const SHADER_SETUP_00B7E150: u32 = 0x00b7_e150;
const SHADER_SETUP_00B7DED0: u32 = 0x00b7_ded0;
const SHADER_SETUP_00B7DFE0: u32 = 0x00b7_dfe0;

// Translated from 00b98e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::RenderPassImmediately_Standard` (Xbox PDB; cdecl,
/// four words): renders one pass of an unskinned geometry (pass: geometry,
/// 16-bit type +4, byte +7) with its shader (geometry +0xC0). After the
/// geometry-group set-up, by pass type (the compiler's jump table, as
/// match arms): 0xCA and 0xD1 run their set-up (`00b7ded0`, `00b7dfe0`),
/// 0xFC/0xFD and 0x102 theirs (`00b7dde0`, `00b7e150`), then the shader's
/// slots +0x84 (`first`) and +0x80 (`second`); 0xCA and the other types
/// then +0x88 (`third`, with the pass's byte +7), as 0xD1 does. Other
/// types run slots +0x78 and +0x7C, then with the byte +7 set +0x80
/// (`second`, unless a shadow pass) and +0x84 (`first`, not for the types
/// 4, 5, 0x0E, 0x0F and 0x23A); with it clear, slot +0x110 when the flag
/// at `011ad8ec` is set and the pass is not a shadow pass (clearing the
/// flag). Last: the shader's +0x6C (geometry, 0, the geometry data's
/// +0x34, the property state), the geometry's +0xE0 (the renderer holder)
/// and the shader's +0x8C.
pub fn bs_batch_renderer_render_pass_immediately_standard(
    e: &mut Engine,
    pass: Ptr,
    first: u8,
    second: u8,
    third: u8,
) {
    let pass = pass.addr();
    let pass_type = u32::from(e.mem.u16(pass + 4));
    let geometry = e.mem.u32(pass);
    let data = e.mem.u32(geometry + 0xb8);
    let shader = e.mem.u32(geometry + 0xc0);
    let state = geometry + 0x9c;
    let holder = add_to_geometry_group(e, geometry, data, 0, 0);
    let buffer = e.mem.u32(data + 0x34);
    let call_third = |e: &mut Engine| {
        if third != 0 {
            let flag = u32::from(e.mem.u8(pass + 7));
            e.vcall(shader, 0x88, &[state, flag]);
        }
    };
    let first_then_second = |e: &mut Engine| {
        if first != 0 {
            e.vcall(shader, 0x84, &[state]);
        }
        if second != 0 {
            e.vcall(shader, 0x80, &[state]);
        }
    };
    match pass_type {
        0xfc | 0xfd => {
            e.call(SHADER_SETUP_00B7DDE0, &[shader, state]);
            first_then_second(e);
        }
        0x102 => {
            e.call(SHADER_SETUP_00B7E150, &[shader, state]);
            first_then_second(e);
        }
        0xca => {
            e.call(SHADER_SETUP_00B7DED0, &[shader, state]);
            first_then_second(e);
            call_third(e);
        }
        0xd1 => {
            e.call(SHADER_SETUP_00B7DFE0, &[shader, state]);
            first_then_second(e);
            call_third(e);
        }
        _ => {
            e.vcall(shader, 0x78, &[state]);
            e.vcall(shader, 0x7c, &[state]);
            if e.mem.u8(pass + 7) != 0 {
                if second != 0 && e.call(IS_SHADOW_PASS, &[pass_type]).u8() == 0 {
                    e.vcall(shader, 0x80, &[state]);
                }
                if first != 0
                    && !(4..=5).contains(&pass_type)
                    && !(0xe..=0xf).contains(&pass_type)
                    && pass_type != 0x23a
                {
                    e.vcall(shader, 0x84, &[state]);
                }
            } else if e.mem.u8(PASS_FLAG_011AD8EC) != 0
                && e.call(IS_SHADOW_PASS, &[pass_type]).u8() == 0
            {
                e.vcall(shader, 0x110, &[state]);
                e.mem.set_u8(PASS_FLAG_011AD8EC, 0);
            }
            call_third(e);
        }
    }
    e.vcall(shader, 0x6c, &[geometry, 0, buffer, state]);
    e.vcall(geometry, 0xe0, &[holder]);
    e.vcall(shader, 0x8c, &[state]);
}

/// `NiAVObject::GetWorldBound` (engine map; thiscall).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// The renderer holder's draw call `00e6d310` (thiscall, three words).
const HOLDER_DRAW_00E6D310: u32 = 0x00e6_d310;
/// The skin instance's class record `GetRTTI` (slot +8) is compared with.
const SKIN_RTTI_011F49D8: u32 = 0x011f_49d8;

// Translated from 00b99110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::RenderPassImmediately_Skinned` (Xbox PDB; cdecl, four
/// words): renders one pass of a skinned geometry partition by partition
/// (skin instance +0xBC, its +0x0C partition data: count +0x08, 0x2C-byte
/// partitions at +0x0C). A skin instance of the class `011f49d8` has
/// per-partition flags (+0x38, four bytes each): a partition whose first
/// flag byte is 0 is skipped. For each partition the shader's slot +0x64
/// gets (geometry, skin, partition, its +0x28, the property state, 0, the
/// geometry's +0x68, its world bound); the first one also runs the
/// state calls of the standard pass (+0x78, +0x7C, +0x80/+0x84 or +0x110,
/// +0x88). Then +0x6C and the holder's draw `00e6d310`; with flags, the
/// following partitions whose second flag byte is 0 and first is set are
/// drawn with the same set-up (a +0x6C answering 0 ends the function at
/// once). Last the geometry data's 16-bit +0x0E keeps only its top four
/// bits and the shader's +0x8C runs.
pub fn bs_batch_renderer_render_pass_immediately_skinned(
    e: &mut Engine,
    pass: Ptr,
    first: u8,
    second: u8,
    third: u8,
) {
    let pass = pass.addr();
    let pass_type = u32::from(e.mem.u16(pass + 4));
    let geometry = e.mem.u32(pass);
    let skin = e.mem.u32(geometry + 0xbc);
    let data = e.mem.u32(geometry + 0xb8);
    let partition_data = e.mem.u32(skin + 0xc);
    let shader = e.mem.u32(geometry + 0xc0);
    let state = geometry + 0x9c;
    let partitions = e.mem.u32(partition_data + 0xc);
    let holder = add_to_geometry_group(e, geometry, data, skin, 1);
    let count = e.mem.u32(partition_data + 8);
    let mut flags = 0u32;
    if e.vcall(skin, 8, &[]).u32() == SKIN_RTTI_011F49D8 {
        flags = e.mem.u32(skin + 0x38);
    }
    let mut first_partition = true;
    let mut index = 0u32;
    while index < count {
        if flags == 0 || e.mem.u8(flags + index * 4) != 0 {
            let mut partition = partitions.wrapping_add(index.wrapping_mul(0x2c));
            let word = e.mem.u32(partition + 0x28);
            let bound = e.call(GET_WORLD_BOUND, &[geometry]).u32();
            e.vcall(
                shader,
                0x64,
                &[
                    geometry,
                    skin,
                    partition,
                    word,
                    state,
                    0,
                    geometry + 0x68,
                    bound,
                ],
            );
            if first_partition {
                e.vcall(shader, 0x78, &[state]);
                e.vcall(shader, 0x7c, &[state]);
                if e.mem.u8(pass + 7) != 0 && e.call(IS_SHADOW_PASS, &[pass_type]).u8() == 0 {
                    if second != 0 {
                        e.vcall(shader, 0x80, &[state]);
                    }
                    if first != 0
                        && !(4..=5).contains(&pass_type)
                        && !(0xe..=0xf).contains(&pass_type)
                    {
                        e.vcall(shader, 0x84, &[state]);
                    }
                } else if e.mem.u8(PASS_FLAG_011AD8EC) != 0 {
                    e.vcall(shader, 0x110, &[state]);
                    e.mem.set_u8(PASS_FLAG_011AD8EC, 0);
                }
                if third != 0 {
                    let flag = u32::from(e.mem.u8(pass + 7));
                    e.vcall(shader, 0x88, &[state, flag]);
                }
                first_partition = false;
            }
            let drawn = e
                .vcall(shader, 0x6c, &[geometry, partition, word, state])
                .u32();
            e.call(HOLDER_DRAW_00E6D310, &[holder, drawn, partition, data]);
            if flags != 0 {
                let mut next = index + 1;
                while next < count {
                    if e.mem.u8(flags + index * 4 + 5) != 0 {
                        break;
                    }
                    index += 1;
                    next += 1;
                    partition = partition.wrapping_add(0x2c);
                    if e.mem.u8(flags + index * 4) != 0 {
                        let word = e.mem.u32(partition + 0x28);
                        let drawn = e
                            .vcall(shader, 0x6c, &[geometry, partition, word, state])
                            .u32();
                        if drawn == 0 {
                            return;
                        }
                        e.call(HOLDER_DRAW_00E6D310, &[holder, drawn, partition, data]);
                    }
                }
            }
        }
        index += 1;
    }
    let data = e.mem.u32(geometry + 0xb8);
    let bits = e.mem.u16(data + 0xe) & 0xf000;
    e.mem.set_u16(data + 0xe, bits);
    e.vcall(shader, 0x8c, &[state]);
}

/// The 16-bit word `BeginPass` sets to 0xFFFF and the 16 words it clears.
const PASS_MARKER_011AE63C: u32 = 0x011a_e63c;
const PASS_WORDS_0126F680: u32 = 0x0126_f680;
/// The begin set-up calls of the pass types 0xCA/0xD1 and 0xFC/0xFD/0x102
/// (thiscall on the shader, no stack words).
const SHADER_BEGIN_00B7DEB0: u32 = 0x00b7_deb0;
const SHADER_BEGIN_00B7DD50: u32 = 0x00b7_dd50;

// Translated from 00b99390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::BeginPass` (Xbox PDB; cdecl, two words): ends the
/// current pass, sets the 16-bit word at `011ae63c` to 0xFFFF, calls the
/// shader's slot +0xF0 with the type, clears the 16 words at `0126f680`,
/// then by type (jump table, as match arms): 0xCA and 0xD1 `00b7deb0`;
/// 0xFC/0xFD `00b7dd50` and z-enable on (`00b97de0(1, 0)`); 0x102
/// `00b7dd50`; any other the shader's slots +0xFC, +0xF8 and +0x100 with the
/// type. Then slot +0xF4 with the type, and records the type and shader
/// as the current pass and sets the flag at `011ad8ec`.
pub fn bs_batch_renderer_begin_pass(e: &mut Engine, pass_type: u32, shader: Ptr) {
    let shader = shader.addr();
    bs_batch_renderer_end_pass(e);
    e.mem.set_u16(PASS_MARKER_011AE63C, 0xffff);
    e.vcall(shader, 0xf0, &[pass_type]);
    for i in 0..16 {
        e.mem.set_u32(PASS_WORDS_0126F680 + i * 4, 0);
    }
    match pass_type {
        0xca | 0xd1 => {
            e.call(SHADER_BEGIN_00B7DEB0, &[shader]);
        }
        0xfc | 0xfd => {
            e.call(SHADER_BEGIN_00B7DD50, &[shader]);
            e.call(SET_Z_ENABLE, &[1, 0]);
        }
        0x102 => {
            e.call(SHADER_BEGIN_00B7DD50, &[shader]);
        }
        _ => {
            e.vcall(shader, 0xfc, &[pass_type]);
            e.vcall(shader, 0xf8, &[pass_type]);
            e.vcall(shader, 0x100, &[pass_type]);
        }
    }
    e.vcall(shader, 0xf4, &[pass_type]);
    e.mem.set_u32(CURRENT_PASS_TYPE, pass_type);
    e.mem.set_u32(CURRENT_SHADER, shader);
    e.mem.set_u8(PASS_FLAG_011AD8EC, 1);
}

/// The pass and type being rendered (`011f91e0`, `011f91e4`).
const RENDERING_PASS: u32 = 0x011f_91e0;
const RENDERING_PASS_TYPE: u32 = 0x011f_91e4;
/// Whether the geometry is skinned for the renderer (`00e72c20`; thiscall
/// on the renderer, two words; answers in AL).
const RENDERER_IS_SKINNED: u32 = 0x00e7_2c20;
/// The shader tails `RenderPassImmediately` runs after a newly begun pass
/// (thiscall on the shader, one word).
const SHADER_TAIL_00B7AC20: u32 = 0x00b7_ac20;
const SHADER_TAIL_00BB96D0: u32 = 0x00bb_96d0;

// Translated from 00b994f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::RenderPassImmediately` (Xbox PDB; cdecl, five words):
/// renders `pass` when its 16-bit type (+4) is `pass_type`: records it at
/// `011f91e0`/`011f91e4`, begins the pass when the type or the shader
/// (geometry +0xC0) changed or the type is 0x2F6, switches alpha to
/// coverage (`00b98540`) by bit 9 of the property state's 16-bit +0x18
/// when the renderer has the switch, then renders it skinned (`00e72c20`
/// answers true), through the geometry's own slot +0xDC (when its slot
/// +0x30 or +0x34 answers non-zero) or as a standard pass. After a newly
/// begun pass, a shader whose +0x68 is 1..5 runs `00b7ac20` and one with
/// 0x11 `00bb96d0`, with the type.
pub fn bs_batch_renderer_render_pass_immediately(
    e: &mut Engine,
    pass: Ptr,
    pass_type: u32,
    first: u32,
    second: u32,
    third: u32,
) {
    let pass = pass.addr();
    if u32::from(e.mem.u16(pass + 4)) != pass_type {
        return;
    }
    e.mem.set_u32(RENDERING_PASS_TYPE, pass_type);
    e.mem.set_u32(RENDERING_PASS, pass);
    let geometry = e.mem.u32(pass);
    let shader = e.mem.u32(geometry + 0xc0);
    let mut begun = false;
    if e.mem.u32(CURRENT_PASS_TYPE) != pass_type
        || pass_type == 0x2f6
        || shader != e.mem.u32(CURRENT_SHADER)
    {
        begun = true;
        bs_batch_renderer_begin_pass(e, pass_type, Ptr::new(shader));
    }
    if e.mem.u8(COVERAGE_AVAILABLE) != 0 {
        let geometry = e.mem.u32(pass);
        let state = e.mem.u32(geometry + 0x9c);
        let on = state != 0 && (e.mem.u16(state + 0x18) >> 9) & 1 != 0;
        e.call(SET_ALPHA_TO_COVERAGE, &[u32::from(on), 0]);
    }
    let renderer = e.mem.u32(RENDERER);
    let geometry = e.mem.u32(pass);
    if e.call(RENDERER_IS_SKINNED, &[renderer, geometry, 0]).u8() != 0 {
        bs_batch_renderer_render_pass_immediately_skinned(
            e,
            Ptr::new(pass),
            first as u8,
            second as u8,
            third as u8,
        );
    } else {
        let geometry = e.mem.u32(pass);
        let own = e.vcall(geometry, 0x30, &[]).u32() != 0 || {
            let geometry = e.mem.u32(pass);
            e.vcall(geometry, 0x34, &[]).u32() != 0
        };
        if own {
            let geometry = e.mem.u32(pass);
            let holder = e.mem.u32(RENDER_HOLDER);
            e.vcall(geometry, 0xdc, &[holder]);
        } else {
            bs_batch_renderer_render_pass_immediately_standard(
                e,
                Ptr::new(pass),
                first as u8,
                second as u8,
                third as u8,
            );
        }
    }
    if begun {
        let shader = e.mem.u32(CURRENT_SHADER);
        if shader != 0 {
            let kind = e.mem.u32(shader + 0x68) as i32;
            if (1..=5).contains(&kind) {
                e.call(SHADER_TAIL_00B7AC20, &[shader, pass_type]);
            } else if kind == 0x11 {
                e.call(SHADER_TAIL_00BB96D0, &[shader, pass_type]);
            }
        }
    }
}

// Translated from 00b99630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSTPersistentList` element,
/// one word: a cdecl compare function): cleans the element (`00b63e90`)
/// and sorts its node list (head +0x04; nodes: next, previous, item) by
/// merging runs of 1, 2, 4, ... nodes until one merge pass is left
/// (stable: a node of the second run goes first only when `compare(first
/// item address, second item address)` is above 0), relinking the previous
/// links and storing the last node in +0x08.
pub fn fn_00b99630(e: &mut Engine, this: Ptr, compare: u32) {
    let base = this.addr();
    e.call(PERSISTENT_LIST_CLEAR, &[base]);
    if e.mem.u32(base + 4) == 0 {
        return;
    }
    let mut run = 1u32;
    let mut tail;
    loop {
        let mut p = e.mem.u32(base + 4);
        e.mem.set_u32(base + 4, 0);
        tail = 0u32;
        let mut merges = 0u32;
        while p != 0 {
            merges += 1;
            let mut p_size = 0i32;
            let mut q = p;
            let mut i = 0u32;
            if run != 0 {
                loop {
                    q = e.mem.u32(q);
                    p_size += 1;
                    if q == 0 {
                        break;
                    }
                    i += 1;
                    if i >= run {
                        break;
                    }
                }
            }
            let mut q_size = run as i32;
            // Whether the second run's node goes next (the exe's compare
            // branch at 00b996a7).
            let second_first = |e: &mut Engine, p: u32, q: u32, q_size: i32| {
                q_size != 0 && q != 0 && e.call(compare, &[p + 8, q + 8]).i32() > 0
            };
            loop {
                let from_second = if p_size > 0 {
                    second_first(e, p, q, q_size)
                } else {
                    if q_size <= 0 || q == 0 {
                        break;
                    }
                    // p_size never goes below 0, so this test always
                    // fails; translated as the exe has it.
                    p_size == 0 || second_first(e, p, q, q_size)
                };
                let next;
                if from_second {
                    next = q;
                    q = e.mem.u32(q);
                    q_size -= 1;
                } else {
                    next = p;
                    p = e.mem.u32(p);
                    p_size -= 1;
                }
                if tail != 0 {
                    e.mem.set_u32(tail, next);
                } else {
                    e.mem.set_u32(base + 4, next);
                }
                e.mem.set_u32(next + 4, tail);
                tail = next;
            }
            p = q;
        }
        run = run.wrapping_add(run);
        e.mem.set_u32(tail, 0);
        if merges == 1 {
            break;
        }
    }
    e.mem.set_u32(base + 8, tail);
}

// Translated from 00b99730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a 16-bit list: item, next at
/// +4): drops the head item. With a second node, its item and link move
/// into the head, the node's link is cleared, its clear `00b64ec0` runs
/// and the memory manager frees it; otherwise the head's item becomes 0.
pub fn fn_00b99730(e: &mut Engine, this: Ptr) {
    let head = this.addr();
    let node = e.mem.u32(head + 4);
    if node != 0 {
        let link = e.mem.u32(node + 4);
        e.mem.set_u32(head + 4, link);
        let item = e.mem.u16(node);
        e.mem.set_u16(head, item);
        e.mem.set_u32(node + 4, 0);
        e.call(ACTIVE_LIST_CLEAR, &[node]);
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, node]);
        return;
    }
    e.mem.set_u16(head, 0);
}

/// A new 8-byte node of a 16-bit list holding `value` (0 when the memory
/// manager fails).
fn new_short_node(e: &mut Engine, value: u16) -> u32 {
    let node = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 8]).u32();
    if node != 0 {
        e.mem.set_u16(node, value);
        e.mem.set_u32(node + 4, 0);
    }
    node
}

// Translated from 00b99770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a 16-bit list, two words: the
/// item word and a cdecl compare function): inserts the item (its low 16
/// bits; 0 does nothing) in order: an empty node takes it; before the
/// first node it does not compare above (`compare(item word, node item) <=
/// 0`; at the head through `00b98e20`); after the last. A failed node
/// allocation in the middle leaves the previous link null and writes the
/// node's link through the null pointer (address 4), as the exe does.
pub fn fn_00b99770(e: &mut Engine, this: Ptr, item: u32, compare: u32) {
    let value = item as u16;
    if value == 0 {
        return;
    }
    let mut previous = 0u32;
    let mut node = this.addr();
    while node != 0 {
        let current = e.mem.u16(node);
        if current == 0 {
            e.mem.set_u16(node, value);
            break;
        }
        if e.call(compare, &[item, u32::from(current)]).i32() > 0 {
            if e.mem.u32(node + 4) == 0 {
                let added = new_short_node(e, value);
                e.mem.set_u32(node + 4, added);
                break;
            }
        } else {
            if previous == 0 {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), item);
                    fn_00b98e20(e, this, slot);
                });
            } else {
                let added = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 8]).u32();
                if added != 0 {
                    e.mem.set_u16(added, value);
                    e.mem.set_u32(added + 4, 0);
                    e.mem.set_u32(previous + 4, added);
                    e.mem.set_u32(added + 4, node);
                } else {
                    e.mem.set_u32(previous + 4, 0);
                    e.mem.set_u32(4, node);
                }
            }
            break;
        }
        previous = node;
        node = e.mem.u32(node + 4);
    }
}

/// A node for a `BSTPersistentList` element: from its free list (+0x0C)
/// when there is one, else from the allocator `0043a010` (on its +0x10).
fn take_node(e: &mut Engine, element: u32) -> u32 {
    let free = e.mem.u32(element + 0xc);
    if free == 0 {
        e.call(LIST_NODE_ALLOCATE, &[element + 0x10]).u32()
    } else {
        let next = e.mem.u32(free);
        e.mem.set_u32(element + 0xc, next);
        free
    }
}

// Translated from 00b99840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSTPersistentList` element,
/// one word: the address of the item): appends the item (a node from the
/// free list or the allocator) and counts it (+0x10).
pub fn fn_00b99840(e: &mut Engine, this: Ptr, item: Ptr) {
    let element = this.addr();
    let node = take_node(e, element);
    let value = e.mem.u32(item.addr());
    e.mem.set_u32(node + 8, value);
    e.mem.set_u32(node, 0);
    let tail = e.mem.u32(element + 8);
    e.mem.set_u32(node + 4, tail);
    let tail = e.mem.u32(element + 8);
    let count = e.mem.u32(element + 0x10).wrapping_add(1);
    if tail != 0 {
        e.mem.set_u32(tail, node);
        e.mem.set_u32(element + 0x10, count);
        e.mem.set_u32(element + 8, node);
    } else {
        e.mem.set_u32(element + 0x10, count);
        e.mem.set_u32(element + 4, node);
        e.mem.set_u32(element + 8, node);
    }
}

// Translated from 00b99900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): finds the first pass
/// type of the active pass list (+0x2C) that is not below +0x20, writing
/// each one looked at to `*pass_type`. Answers false when the list is
/// empty, runs out, or the type is above +0x24; otherwise drops the
/// list's head (`00b99730`), looks the type up in the render-pass map
/// (+0x0C, `00853130`) and answers `00b98ae0` for that pass with `first`.
pub fn fn_00b99900(e: &mut Engine, this: Ptr<BSBatchRenderer>, pass_type: Ptr, first: u32) -> bool {
    let base = this.addr();
    let mut node = base + 0x2c;
    if e.mem.u32(node + 4) == 0 && e.mem.u16(node) == 0 {
        return false;
    }
    let mut value = u32::from(e.mem.u16(node));
    e.mem.set_u32(pass_type.addr(), value);
    while value < e.mem.u32(base + 0x20) {
        node = e.mem.u32(node + 4);
        if node == 0 {
            return false;
        }
        value = u32::from(e.mem.u16(node));
        e.mem.set_u32(pass_type.addr(), value);
    }
    if value > e.mem.u32(base + 0x24) {
        return false;
    }
    fn_00b99730(e, Ptr::new(base + 0x2c));
    let key = e.mem.u32(pass_type.addr());
    let pass = e.with_stack(4, |e, slot| {
        e.call(MAP_GET_AT, &[base + 0xc, key, slot.addr()]);
        e.mem.u32(slot.addr())
    });
    fn_00b98ae0(e, this, pass, Ptr::new(first))
}

// Translated from 00b99990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): renders the passes of a
/// `BSTPersistentList` element when it counts any (+0x10): ends the
/// current pass, renders each listed pass that has a geometry
/// (`RenderPassImmediately(pass, its type, 1, second, 1)`; the walk stops
/// at the first null item), cleans the element (`00b63e90`), moves its
/// nodes to the free list and clears it, and ends the pass again.
pub fn fn_00b99990(e: &mut Engine, list: Ptr, second: u32) {
    let list = list.addr();
    if e.mem.u32(list + 0x10) == 0 {
        return;
    }
    bs_batch_renderer_end_pass(e);
    let node = e.mem.u32(list + 4);
    if node != 0 {
        let mut next = e.mem.u32(node);
        let mut pass = e.mem.u32(node + 8);
        while pass != 0 {
            if e.mem.u32(pass) != 0 {
                let pass_type = u32::from(e.mem.u16(pass + 4));
                bs_batch_renderer_render_pass_immediately(
                    e,
                    Ptr::new(pass),
                    pass_type,
                    1,
                    second,
                    1,
                );
            }
            if next == 0 {
                break;
            }
            pass = e.mem.u32(next + 8);
            next = e.mem.u32(next);
        }
    }
    e.call(PERSISTENT_LIST_CLEAR, &[list]);
    let head = e.mem.u32(list + 4);
    e.mem.set_u32(list + 0xc, head);
    e.mem.set_u32(list + 4, 0);
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list + 0x10, 0);
    bs_batch_renderer_end_pass(e);
}

// Translated from 00b99a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): keeps `camera` and the
/// radius flag for the compare function and sorts the lists of the two
/// alpha geometry groups (+0x70, +0x74; their element at +0x0C) with
/// `00b98b20`.
pub fn fn_00b99a10(e: &mut Engine, this: Ptr<BSBatchRenderer>, camera: u32, subtract: u8) {
    let base = this.addr();
    e.mem.set_u8(SORT_SUBTRACT_RADIUS, subtract);
    e.mem.set_u32(SORT_CAMERA, camera);
    for slot in [0x70, 0x74] {
        let group = e.mem.u32(base + slot);
        fn_00b99630(e, Ptr::new(group.wrapping_add(0xc)), 0x00b9_8b20);
    }
}

// Translated from 00b99a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a pass group, `RET 4`: one word
/// never read): for each of its five elements that counts items (+0x10),
/// cleans it (`00b63e90`), moves its nodes to the free list and clears
/// it; then clears the group's +0x64.
pub fn fn_00b99a50(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    for index in 0..5 {
        let element = base + index * 0x14;
        if e.mem.u32(element + 0x10) != 0 {
            e.call(PERSISTENT_LIST_CLEAR, &[element]);
            let head = e.mem.u32(element + 4);
            e.mem.set_u32(element + 0xc, head);
            e.mem.set_u32(element + 4, 0);
            e.mem.set_u32(element + 8, 0);
            e.mem.set_u32(element + 0x10, 0);
        }
    }
    e.mem.set_u32(base + 0x64, 0);
}

// Translated from 00b99a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a pass group, two words):
/// prepends `item` to element `index` and counts it there and in the
/// group's +0x64.
pub fn fn_00b99a90(e: &mut Engine, this: Ptr, index: u32, item: u32) {
    let base = this.addr();
    let element = base.wrapping_add(index.wrapping_mul(0x14));
    let node = take_node(e, element);
    e.mem.set_u32(node + 8, item);
    let head = e.mem.u32(element + 4);
    e.mem.set_u32(node, head);
    e.mem.set_u32(node + 4, 0);
    let head = e.mem.u32(element + 4);
    if head != 0 {
        e.mem.set_u32(head + 4, node);
        e.mem.set_u32(element + 4, node);
    } else {
        e.mem.set_u32(element + 8, node);
        e.mem.set_u32(element + 4, node);
    }
    let count = e.mem.u32(element + 0x10).wrapping_add(1);
    e.mem.set_u32(element + 0x10, count);
    let total = e.mem.u32(base + 0x64).wrapping_add(1);
    e.mem.set_u32(base + 0x64, total);
}

// Translated from 00b99b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a pass group, two words):
/// appends `item` to element `index` and counts it there and in the
/// group's +0x64.
pub fn fn_00b99b00(e: &mut Engine, this: Ptr, index: u32, item: u32) {
    let base = this.addr();
    let element = base.wrapping_add(index.wrapping_mul(0x14));
    let node = take_node(e, element);
    e.mem.set_u32(node + 8, item);
    e.mem.set_u32(node, 0);
    let tail = e.mem.u32(element + 8);
    e.mem.set_u32(node + 4, tail);
    let tail = e.mem.u32(element + 8);
    if tail != 0 {
        e.mem.set_u32(tail, node);
        e.mem.set_u32(element + 8, node);
    } else {
        e.mem.set_u32(element + 4, node);
        e.mem.set_u32(element + 8, node);
    }
    let count = e.mem.u32(element + 0x10).wrapping_add(1);
    e.mem.set_u32(element + 0x10, count);
    let total = e.mem.u32(base + 0x64).wrapping_add(1);
    e.mem.set_u32(base + 0x64, total);
}

/// The comparator of the active pass list (`00b988a0`, `run_00b984f0`).
const PASS_TYPE_COMPARE: u32 = 0x00b9_88a0;

// Translated from 00b99c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, slot +0x8C of the `BSBatchRenderer` vtable
/// (`010b7e54`; Ghidra made no function here, read from the bytes;
/// thiscall, two words): registers `pass` (geometry, 16-bit type +4, byte
/// +7) under `pass_type` (or its own type when 0). Picks one of the five
/// element lists: 4 when the geometry's +0xA8 has bit 22 of +0x24;
/// otherwise 2 or 3 (with or without the "both bits 10 and 11" flag of the
/// geometry's +0xAC 16-bit +0x18) when the pass counts as alpha (property
/// state +0x9C with bit 9 of +0x18, the byte +7 set, and 1.0 not above
/// either float +0x28/+0x2C of +0xA8 or bit 0 of +0x18 or type 2; or the
/// types 4, 5, 0x0E, 0x0F, 0x23A), else 1 or 0 by that flag. A type not
/// in the render-pass map gets the next index (`SetAt`) and a cleared
/// group (`00b99a50`); a group with nothing counted puts the type into the
/// active list (`00b99770`). The pass is prepended when its byte +7 is set
/// and the type is not 0x238..0x23E, else appended.
pub fn fn_00b99c90(e: &mut Engine, this: Ptr<BSBatchRenderer>, pass: u32, pass_type: u32) {
    let base = this.addr();
    if pass == 0 {
        return;
    }
    let geometry = e.mem.u32(pass);
    if geometry == 0 {
        return;
    }
    let mut paired = false;
    let properties = e.mem.u32(geometry + 0xa8);
    let shade = e.mem.u32(geometry + 0xac);
    if shade != 0 && e.mem.u16(shade + 0x18) & 0xc00 == 0xc00 {
        paired = true;
    }
    let state = e.mem.u32(geometry + 0x9c);
    let mut alpha = false;
    if state != 0 && (e.mem.u16(state + 0x18) >> 9) & 1 != 0 && e.mem.u8(pass + 7) != 0 {
        alpha = true;
        let one = 1.0f64;
        let blocked = one > x87_m32(e.mem.u32(properties.wrapping_add(0x28)))
            || one > x87_m32(e.mem.u32(properties.wrapping_add(0x2c)));
        if blocked && e.mem.u8(state + 0x18) & 1 == 0 && pass_type != 2 {
            alpha = false;
        }
    }
    if matches!(pass_type, 0xe | 0xf | 4 | 5 | 0x23a) {
        alpha = true;
    }
    let list = if e.mem.u32(properties.wrapping_add(0x24)) & 0x40_0000 != 0 {
        4
    } else if alpha {
        u32::from(!paired) + 2
    } else {
        u32::from(paired)
    };
    let mut key = u32::from(e.mem.u16(pass + 4));
    if pass_type != 0 {
        key = pass_type;
    }
    let map = base + 0xc;
    // The out slot is the stack word of the `pass_type` argument.
    let (found, value) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), pass_type);
        let found = e.call(MAP_GET_AT, &[map, key, slot.addr()]).u8() != 0;
        (found, e.mem.u32(slot.addr()))
    });
    let index = if found {
        value
    } else {
        let index = e.mem.u32(base + 0x18);
        e.call(MAP_SET_AT, &[map, key, index]);
        let groups = e.mem.u32(base + 8);
        fn_00b99a50(
            e,
            Ptr::new(index.wrapping_mul(0x68).wrapping_add(groups)),
            0,
        );
        index
    };
    let groups = e.mem.u32(base + 8);
    let group = index.wrapping_mul(0x68).wrapping_add(groups);
    if e.mem.u32(group + 0x64) == 0 {
        fn_00b99770(e, Ptr::new(base + 0x2c), key, PASS_TYPE_COMPARE);
    }
    let groups = e.mem.u32(base + 8);
    let group = index.wrapping_mul(0x68).wrapping_add(groups);
    if e.mem.u8(pass + 7) != 0 && !(0x238..=0x23e).contains(&key) {
        fn_00b99a90(e, Ptr::new(group), list, pass);
    } else {
        fn_00b99b00(e, Ptr::new(group), list, pass);
    }
}

// Translated from 00b9a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, three words): finds the next
/// non-empty element list to render: for the pass type `*pass_type` (when
/// not 0) the first non-empty element of its group (`00b98ae0`), else the
/// next pass type from the active list (`00b99900`; false when none). Then
/// stores the byte +7 of the first pass of that element in `*flag`.
pub fn fn_00b9a090(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass_type: Ptr,
    element: Ptr,
    flag: Ptr,
) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let mut answer = true;
        let key = e.mem.u32(pass_type.addr());
        let mut ready = false;
        if key != 0 {
            e.call(MAP_GET_AT, &[base + 0xc, key, slot]);
            let pass = e.mem.u32(slot);
            ready = fn_00b98ae0(e, this, pass, element);
        }
        if !ready {
            answer = fn_00b99900(e, this, pass_type, element.addr());
            if !answer {
                return false;
            }
        }
        let key = e.mem.u32(pass_type.addr());
        e.call(MAP_GET_AT, &[base + 0xc, key, slot]);
        let index = e.mem.u32(element.addr());
        let pass = e.mem.u32(slot);
        let list = pass_element(e, base, pass, index);
        let node = e.mem.u32(list + 4);
        let first = e.mem.u32(node + 8);
        let value = e.mem.u8(first + 7);
        e.mem.set_u8(flag.addr(), value);
        answer
    })
}

// Translated from 00b9a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, four words): renders the passes of
/// element `*element` of the group of `*pass_type` that share the first
/// pass's byte +7, taking each off the list (its node goes to the free
/// list; the group's +0x64 drops by one). The element picks the states
/// first: 0 `00b984d0(1, 0)`, alpha test off and coverage off (locks 1);
/// 1 `00b984d0(3, 1)`, alpha test off, coverage off; 2 `00b984d0(3, 1)`,
/// alpha test on, coverage on; 3 `00b984d0(1, 1)`, alpha test on, coverage
/// on; 4 `00b984d0(1, 1)`, alpha test on, coverage off; 2..4 render with
/// blending (and set coverage only when the renderer has the switch). A
/// pass with another byte +7 stops the walk and goes to `*flag`. Then the
/// locks are released (the counters at `011ffa24` and slot 4 drop when
/// set, the coverage counter with a coverage reset when the switch
/// exists) and coverage is reset. Answers true when stopped by such a
/// pass, else `00b9a090` for the next list.
///
/// The blending word passed on is the map's value word with its low byte
/// replaced (the exe reuses that stack slot); the callees read the byte.
pub fn fn_00b9a120(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass_type: Ptr,
    element: Ptr,
    flag: Ptr,
    second: u32,
) -> bool {
    let base = this.addr();
    let index_ptr = element.addr();
    let value = e.with_stack(4, |e, slot| {
        let key = e.mem.u32(pass_type.addr());
        e.call(MAP_GET_AT, &[base + 0xc, key, slot.addr()]);
        e.mem.u32(slot.addr())
    });
    let group_offset = value.wrapping_mul(0x68);
    let index = e.mem.u32(index_ptr);
    let list = group_offset
        .wrapping_add(index.wrapping_mul(0x14))
        .wrapping_add(e.mem.u32(base + 8));
    let head = e.mem.u32(list + 4);
    let first_flag = e.mem.u8(e.mem.u32(head + 8) + 7);
    let mut blend = 0u8;
    // `00b984d0`'s second word (never read) is the element index here (0,
    // the tested `EAX`), else 1, as the exe pushes it.
    if e.mem.u32(index_ptr) == 0 {
        fn_00b984d0(e, 1, 0);
        e.call(SET_ALPHA_TEST_ENABLE, &[0, 1]);
        e.call(SET_ALPHA_TO_COVERAGE, &[0, 1]);
    }
    if e.mem.u32(index_ptr) == 1 {
        fn_00b984d0(e, 3, 1);
        e.call(SET_ALPHA_TEST_ENABLE, &[0, 1]);
        e.call(SET_ALPHA_TO_COVERAGE, &[0, 1]);
    }
    for (element, mode, coverage) in [(2, 3, 1), (3, 1, 1), (4, 1, 0)] {
        if e.mem.u32(index_ptr) == element {
            fn_00b984d0(e, mode, 1);
            e.call(SET_ALPHA_TEST_ENABLE, &[1, 1]);
            blend = 1;
            if e.mem.u8(COVERAGE_AVAILABLE) != 0 {
                e.call(SET_ALPHA_TO_COVERAGE, &[coverage, 1]);
            }
        }
    }
    let blend_word = (value & 0xffff_ff00) | u32::from(blend);
    let element_list = |e: &Engine| {
        let index = e.mem.u32(index_ptr);
        group_offset
            .wrapping_add(index.wrapping_mul(0x14))
            .wrapping_add(e.mem.u32(base + 8))
    };
    let list = element_list(e);
    let mut pass = e.mem.u32(e.mem.u32(list + 4) + 8);
    let mut stopped = false;
    if pass != 0 {
        loop {
            if e.mem.u8(pass + 7) != first_flag {
                let value = e.mem.u8(pass + 7);
                e.mem.set_u8(flag.addr(), value);
                stopped = true;
                break;
            }
            let key = e.mem.u32(pass_type.addr());
            bs_batch_renderer_render_pass_immediately(
                e,
                Ptr::new(pass),
                key,
                blend_word,
                second,
                0,
            );
            let index = e.mem.u32(index_ptr);
            let group = e.mem.u32(base + 8).wrapping_add(group_offset);
            let list = group.wrapping_add(index.wrapping_mul(0x14));
            let node = e.mem.u32(list + 4);
            if node != 0 {
                if node == e.mem.u32(list + 8) {
                    e.mem.set_u32(list + 8, 0);
                }
                let mut next = 0;
                if e.mem.u32(node) != 0 {
                    next = e.mem.u32(node);
                    e.mem.set_u32(next + 4, 0);
                }
                let free = e.mem.u32(list + 0xc);
                if free != 0 {
                    let head = e.mem.u32(list + 4);
                    e.mem.set_u32(free + 4, head);
                }
                let head = e.mem.u32(list + 4);
                let free = e.mem.u32(list + 0xc);
                e.mem.set_u32(head, free);
                let head = e.mem.u32(list + 4);
                e.mem.set_u32(list + 0xc, head);
                let count = e.mem.u32(list + 0x10).wrapping_sub(1);
                e.mem.set_u32(list + 0x10, count);
                e.mem.set_u32(list + 4, next);
            }
            let total = e.mem.u32(group + 0x64).wrapping_sub(1);
            e.mem.set_u32(group + 0x64, total);
            let list = element_list(e);
            let head = e.mem.u32(list + 4);
            if head == 0 {
                break;
            }
            pass = e.mem.u32(head + 8);
            if pass == 0 {
                break;
            }
        }
    }
    for counter in [LOCK_COUNT_011FFA24, LOCK_COUNT_4] {
        let count = e.mem.u32(counter);
        if count != 0 {
            e.mem.set_u32(counter, count - 1);
        }
    }
    if e.mem.u8(COVERAGE_AVAILABLE) != 0 {
        let count = e.mem.u32(COVERAGE_LOCK).wrapping_sub(1);
        e.mem.set_u32(COVERAGE_LOCK, count);
        e.call(SET_ALPHA_TO_COVERAGE, &[0, 0]);
    }
    e.call(SET_ALPHA_TO_COVERAGE, &[0, 0]);
    if stopped {
        return true;
    }
    fn_00b9a090(e, this, pass_type, element, flag)
}

// Translated from 00b9a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::GeometryGroup::~GeometryGroup` (Xbox PDB; thiscall):
/// deletes the group's own batch renderer (+0x08, its virtual slot 0 with
/// 1) and clears the word, then destroys its `BSTPersistentList` element
/// (+0x0C: vtable, clean-up `00b63e90`, nodes to the free list, words
/// cleared, clean-up again).
// Translation incomplete: 00b9a410 - the C++ exception frame (FS:[0] chain, unwind state 0 for the element) is not translated.
pub fn bs_batch_renderer_geometry_group_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let renderer = e.mem.u32(base + 8);
    if renderer != 0 {
        e.vcall(renderer, 0, &[1]);
        e.mem.set_u32(base + 8, 0);
    }
    let element = base + 0xc;
    e.mem.set_u32(element, PERSISTENT_LIST_VTABLE);
    e.call(PERSISTENT_LIST_CLEAR, &[element]);
    let head = e.mem.u32(element + 4);
    e.mem.set_u32(element + 0xc, head);
    e.mem.set_u32(element + 4, 0);
    e.mem.set_u32(element + 8, 0);
    e.mem.set_u32(element + 0x10, 0);
    e.call(PERSISTENT_LIST_CLEAR, &[element]);
}

// Translated from 00b9a490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a geometry group): empties its
/// element (+0x0C: clean-up, nodes to the free list, words cleared), clears
/// its own batch renderer's passes (`00b99bc0(0)`) when it has one, and
/// clears the 16-bit count +0x02.
pub fn fn_00b9a490(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let element = base + 0xc;
    e.call(PERSISTENT_LIST_CLEAR, &[element]);
    let head = e.mem.u32(element + 4);
    e.mem.set_u32(element + 0xc, head);
    e.mem.set_u32(element + 4, 0);
    e.mem.set_u32(element + 8, 0);
    e.mem.set_u32(element + 0x10, 0);
    let renderer = e.mem.u32(base + 8);
    if renderer != 0 {
        fn_00b99bc0(e, Ptr::new(renderer), 0);
    }
    e.mem.set_u16(base + 2, 0);
}

/// The instance renderer's destructor (`00bd9150`; thiscall).
const INSTANCE_RENDERER_DESTRUCTOR: u32 = 0x00bd_9150;
/// The render-pass map's destructor (`006e2150`; thiscall).
const MAP_DESTRUCTOR: u32 = 0x006e_2150;
/// `NiObject`'s destructor (`00a5d3d0`; thiscall).
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;

// Translated from 00b9aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSBatchRenderer` destructor (unnamed in the engine map): sets the
/// vtable, clears the passes in use (`00b99bc0(0)`), destroys the pass
/// groups (the count before them; `00b99bb0` each) and frees their block,
/// clears +0x1C, destroys and frees the instance renderer (+0x28), destroys
/// and frees every geometry group (+0x3C..+0x74, the slots keep their
/// words), destroys the two alpha-group lists (+0x78), clears the active
/// pass list (+0x2C), destroys the render-pass map (+0x0C) and runs the
/// `NiObject` destructor.
// Translation incomplete: 00b9aa80 - the C++ exception frame (FS:[0] chain, unwind states 0..3 for the members) is not translated.
pub fn fn_00b9aa80(e: &mut Engine, this: Ptr<BSBatchRenderer>) {
    let base = this.addr();
    e.mem.set_u32(base, BATCH_RENDERER_VTABLE);
    fn_00b99bc0(e, this, 0);
    let groups = e.mem.u32(base + 8);
    if groups != 0 {
        let count = e.mem.u32(groups - 4);
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &[groups, 0x68, count, PASS_GROUP_DESTRUCTOR],
        );
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, groups - 4]);
    }
    let instance = e.mem.u32(base + 0x28);
    e.mem.set_u32(base + 0x1c, 0);
    if instance != 0 {
        e.call(INSTANCE_RENDERER_DESTRUCTOR, &[instance]);
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, instance]);
        e.mem.set_u32(base + 0x28, 0);
    }
    for slot in (0x3c..=0x6c).step_by(4).chain([0x70, 0x74]) {
        let group = e.mem.u32(base + slot);
        if group != 0 {
            bs_batch_renderer_geometry_group_destructor(e, Ptr::new(group));
            e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, group]);
        }
    }
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &[base + 0x78, 0xc, 2, ALPHA_LIST_DESTRUCTOR],
    );
    e.call(ACTIVE_LIST_CLEAR, &[base + 0x2c]);
    e.call(MAP_DESTRUCTOR, &[base + 0xc]);
    e.call(OBJECT_DESTRUCTOR, &[base]);
}

/// The allocator for a geometry group's own batch renderer (cdecl, one
/// word: the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The prepend of a `BSTPersistentList` element (`00b99890`, this unit,
/// not translated here; thiscall, one word: the item's address).
const PERSISTENT_LIST_PREPEND: u32 = 0x00b9_9890;

// Translated from 00b9abc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a geometry group, two words):
/// adds `pass`. A group with bit 0 of its first byte set keeps passes in
/// its element (+0x0C): prepended (`00b99890`) when `prepend` is set,
/// else appended (`00b99840`). Otherwise the pass goes to the group's own
/// batch renderer (+0x08; made on first use: 0x98 bytes, constructed with
/// the group's +0x04 passes and kind 100), through its virtual slot +0x8C
/// with the pass's type. Either way the 16-bit count +0x02 grows by one.
// Translation incomplete: 00b9abc0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the renderer's construction) is not translated.
pub fn fn_00b9abc0(e: &mut Engine, this: Ptr, pass: u32, prepend: u8) {
    let base = this.addr();
    if e.mem.u8(base) & 1 != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), pass);
            if prepend != 0 {
                e.call(PERSISTENT_LIST_PREPEND, &[base + 0xc, slot.addr()]);
            } else {
                fn_00b99840(e, Ptr::new(base + 0xc), slot);
            }
        });
    } else {
        if e.mem.u32(base + 8) == 0 {
            let block = e.call(OPERATOR_NEW, &[0x98]).u32();
            let renderer = if block != 0 {
                let passes = e.mem.u32(base + 4);
                fn_00b9a5b0(e, Ptr::new(block), passes, 100).addr()
            } else {
                0
            };
            e.mem.set_u32(base + 8, renderer);
        }
        let pass_type = u32::from(e.mem.u16(pass + 4));
        let renderer = e.mem.u32(base + 8);
        e.vcall(renderer, 0x8c, &[pass, pass_type]);
    }
    let count = e.mem.u16(base + 2).wrapping_add(1);
    e.mem.set_u16(base + 2, count);
}

/// Takes a node from the shared node pool: enters the pool's lock, refills
/// the pool (`0043a080`) when empty, unlinks the first free node, clears
/// it and leaves the lock.
fn pool_node(e: &mut Engine) -> u32 {
    e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
    let mut node = e.mem.u32(NODE_POOL_FREE_LIST);
    if node == 0 {
        e.call(NODE_POOL_REFILL, &[]);
        node = e.mem.u32(NODE_POOL_FREE_LIST);
    }
    let next = e.mem.u32(node);
    e.mem.set_u32(NODE_POOL_FREE_LIST, next);
    e.mem.set_u32(node + 8, 0);
    e.mem.set_u32(node, 0);
    e.mem.set_u32(node + 4, 0);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
    node
}

// Translated from 00b9ac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::StartGroupingAlphas` (Xbox PDB; thiscall, four
/// words): when already grouping (+0x34) or without a camera only counts
/// one more level. Otherwise makes the two alpha geometry groups (0x24
/// bytes each, word 0x2F7; their element cleaned twice and any own
/// renderer cleared), gives each the distance of `point` along the
/// camera's direction (minus `point`'s fourth float when `subtract`), bit
/// 0 of its first byte set unless `keep`, and inserts it into the
/// matching alpha-group list (+0x78, +0x84) before the first group it is
/// farther than (a pool node; else appended), storing it in +0x90/+0x94.
/// Then counts the level and stores `keep` in +0x38.
pub fn bs_batch_renderer_start_grouping_alphas(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    point: Ptr,
    keep: u8,
    camera: Ptr,
    subtract: u8,
) {
    let base = this.addr();
    if e.mem.u32(base + 0x34) != 0 || camera.addr() == 0 {
        let level = e.mem.u32(base + 0x34).wrapping_add(1);
        e.mem.set_u32(base + 0x34, level);
        return;
    }
    let mut slot = base + 0x90;
    let mut tail_slot = base + 0x7c;
    for _ in 0..2 {
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x24]).u32();
        let group = if block != 0 {
            let element = block + 0xc;
            e.mem.set_u32(element, PERSISTENT_LIST_VTABLE);
            e.mem.set_u32(element + 4, 0);
            e.mem.set_u32(element + 8, 0);
            e.mem.set_u32(element + 0xc, 0);
            e.mem.set_u16(block + 2, 0);
            e.mem.set_u8(block, 0);
            e.mem.set_u32(block + 8, 0);
            e.mem.set_u32(block + 4, 0x2f7);
            e.call(PERSISTENT_LIST_CLEAR, &[element]);
            let head = e.mem.u32(element + 4);
            e.mem.set_u32(element + 0xc, head);
            e.mem.set_u32(element + 4, 0);
            e.mem.set_u32(element + 8, 0);
            e.mem.set_u32(element + 0x10, 0);
            block
        } else {
            0
        };
        let element = group.wrapping_add(0xc);
        e.call(PERSISTENT_LIST_CLEAR, &[element]);
        let head = e.mem.u32(element.wrapping_add(4));
        e.mem.set_u32(element.wrapping_add(0xc), head);
        e.mem.set_u32(element.wrapping_add(4), 0);
        e.mem.set_u32(element.wrapping_add(8), 0);
        e.mem.set_u32(element.wrapping_add(0x10), 0);
        let renderer = e.mem.u32(group.wrapping_add(8));
        if renderer != 0 {
            fn_00b99bc0(e, Ptr::new(renderer), 0);
        }
        e.mem.set_u16(group.wrapping_add(2), 0);
        let cam = camera.addr();
        let d = camera_direction(e, cam);
        let p = point.addr();
        let c = |e: &Engine, i: u32| x87_load32(e.mem.u32(p + i * 4));
        let sum = x87_add(
            x87_mul(c(e, 0), x87_m32(x87_store32(d[0]))),
            x87_mul(c(e, 1), x87_m32(x87_store32(d[1]))),
        );
        let dot = x87_add(sum, x87_mul(c(e, 2), x87_m32(x87_store32(d[2]))));
        let distance = if subtract != 0 {
            let dot = x87_store32(dot);
            x87_store32(x87_sub(x87_load32(dot), x87_m32(e.mem.u32(p + 0xc))))
        } else {
            x87_store32(dot)
        };
        e.mem.set_u32(group.wrapping_add(0x20), distance);
        let flags = e.mem.u8(group);
        e.mem
            .set_u8(group, if keep == 0 { flags | 1 } else { flags & 0xfe });
        let head_slot = tail_slot - 4;
        let count_slot = tail_slot + 4;
        let mut node = e.mem.u32(head_slot);
        let mut inserted = false;
        while node != 0 {
            let other = e.mem.u32(node + 8);
            let other_distance = x87_load32(e.mem.u32(other + 0x20));
            let own = x87_load32(e.mem.u32(group.wrapping_add(0x20)));
            if own > other_distance {
                let added = pool_node(e);
                e.mem.set_u32(added + 8, group);
                e.mem.set_u32(added, node);
                let previous = e.mem.u32(node + 4);
                e.mem.set_u32(added + 4, previous);
                let previous = e.mem.u32(node + 4);
                if previous != 0 {
                    e.mem.set_u32(previous, added);
                } else {
                    e.mem.set_u32(head_slot, added);
                }
                e.mem.set_u32(node + 4, added);
                let count = e.mem.u32(count_slot).wrapping_add(1);
                e.mem.set_u32(count_slot, count);
                inserted = true;
            }
            node = e.mem.u32(node);
            if inserted {
                break;
            }
        }
        if !inserted {
            let added = e.call(LIST_NODE_ALLOCATE, &[count_slot]).u32();
            e.mem.set_u32(added + 8, group);
            e.mem.set_u32(added, 0);
            let tail = e.mem.u32(tail_slot);
            e.mem.set_u32(added + 4, tail);
            let tail = e.mem.u32(tail_slot);
            if tail != 0 {
                e.mem.set_u32(tail, added);
            } else {
                e.mem.set_u32(head_slot, added);
            }
            let count = e.mem.u32(count_slot).wrapping_add(1);
            e.mem.set_u32(count_slot, count);
            e.mem.set_u32(tail_slot, added);
        }
        e.mem.set_u32(slot, group);
        slot += 4;
        tail_slot += 0xc;
    }
    let level = e.mem.u32(base + 0x34).wrapping_add(1);
    e.mem.set_u32(base + 0x34, level);
    e.mem.set_u8(base + 0x38, keep);
}

/// The alpha-group list's removal of a group (`00e7db70`; thiscall, one
/// word: the address of the slot holding it).
const ALPHA_LIST_REMOVE: u32 = 0x00e7_db70;

// Translated from 00b9aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::StopGroupingAlphas` (Xbox PDB; thiscall): drops one
/// grouping level (not below 0); at level 0, each of the two current alpha
/// groups (+0x90, +0x94) that holds nothing (16-bit +0x02) is taken out of
/// its list (+0x78, +0x84; `00e7db70`), destroyed and freed; both slots and
/// the byte +0x38 are cleared.
pub fn bs_batch_renderer_stop_grouping_alphas(e: &mut Engine, this: Ptr<BSBatchRenderer>) {
    let base = this.addr();
    let level = e.mem.u32(base + 0x34).wrapping_sub(1) as i32;
    let level = if level < 0 { 0 } else { level };
    e.mem.set_u32(base + 0x34, level as u32);
    if level > 0 {
        return;
    }
    for (slot, list) in [(0x90, 0x78), (0x94, 0x84)] {
        let group = e.mem.u32(base + slot);
        if e.mem.u16(group.wrapping_add(2)) == 0 {
            e.call(ALPHA_LIST_REMOVE, &[base + list, base + slot]);
            let group = e.mem.u32(base + slot);
            if group != 0 {
                bs_batch_renderer_geometry_group_destructor(e, Ptr::new(group));
                e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, group]);
            }
        }
        e.mem.set_u32(base + slot, 0);
    }
    e.mem.set_u8(base + 0x38, 0);
}

/// The double `RenderNextAlpha` takes off its limit (`01016978`).
const LIMIT_STEP: u32 = 0x0101_6978;
/// The word whose value 9 makes `RenderNextAlpha` skip the property kinds
/// 0x15 and 0x11.
const RENDER_MODE_011F91B8: u32 = 0x011f_91b8;
/// `_isnan` (CRT; cdecl, a double).
const ISNAN: u32 = 0x00ec_75b1;

/// The distance of the pass `pass` along `d` as `RenderNextAlpha` first
/// computes it (camera direction kept on the x87 stack).
fn next_alpha_distance_first(e: &Engine, pass: u32, d: [f64; 3], subtract: bool) -> u32 {
    let geometry = e.mem.u32(pass);
    let bound = bound_of(e, geometry);
    let p = |i: u32| x87_load32(e.mem.u32(bound + i * 4));
    let y = x87_mul(d[1], p(1));
    let x = x87_mul(d[0], x87_m32(e.mem.u32(bound)));
    let sum = x87_add(y, x);
    let dot = x87_add(x87_mul(d[2], p(2)), sum);
    if subtract {
        let dot = x87_store32(dot);
        x87_store32(x87_sub(x87_load32(dot), x87_m32(e.mem.u32(bound + 0xc))))
    } else {
        x87_store32(dot)
    }
}

/// The same as `RenderNextAlpha` computes it for the following passes
/// (camera direction read from its locals).
fn next_alpha_distance_next(e: &Engine, pass: u32, d: [u32; 3], subtract: bool) -> u32 {
    let geometry = e.mem.u32(pass);
    let bound = bound_of(e, geometry);
    let p = |i: u32| x87_load32(e.mem.u32(bound + i * 4));
    let sum = x87_add(x87_mul(p(1), x87_m32(d[1])), x87_mul(p(0), x87_m32(d[0])));
    let dot = x87_add(sum, x87_mul(p(2), x87_m32(d[2])));
    if subtract {
        let dot = x87_store32(dot);
        x87_store32(x87_sub(x87_load32(dot), x87_m32(e.mem.u32(bound + 0xc))))
    } else {
        x87_store32(dot)
    }
}

// Translated from 00b9af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::RenderNextAlpha` (Xbox PDB; thiscall, four words):
/// renders, back to front, the alpha passes of geometry group `group`
/// (+0x70 array) and the alpha groups of list `group` (+0x78, 0x0C each)
/// down to `limit` (taken 0.001, the double at `01016978`, lower first).
/// The current pass's distance is the camera direction dotted with its
/// bound's center (minus the radius when `subtract`). While a group is
/// not nearer than the current pass (or there is no pass) and the limit is
/// not above the group's distance, the group is rendered (its own passes
/// through `00b99990(.., 1)` when bit 0 of its first byte is set, else its
/// batch renderer's slot +0x90 with 1, 0x2F1, 1), its renderer's active
/// list cleared, the group destroyed and freed, and its node given back to
/// the pool. While the limit is not above the pass's distance, the pass is
/// rendered (`RenderPassImmediately(pass, type, 1, 1, 1)`, with z-writes
/// off and the decal depth bias around it when its property +0xA8 has bit
/// 26 or 27 of +0x20; skipped without a property, or with the property
/// kinds 0x15 and 0x11 when the word at `011f91b8` is 9) and taken off the
/// geometry group's list. The loop ends when neither is left within the
/// limit or the current distance is NaN (`_isnan`).
pub fn bs_batch_renderer_render_next_alpha(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    group: u32,
    camera: Ptr,
    subtract: u8,
    limit: f32,
) {
    let base = this.addr();
    let cam = camera.addr();
    let locals = [0x68, 0x74, 0x80].map(|offset| x87_quiet_bits(e.mem.u32(cam + offset)));
    let d = locals.map(x87_load32);
    let subtract = subtract != 0;
    let geometry_group_of = |e: &Engine| {
        e.mem
            .u32(base.wrapping_add(group.wrapping_mul(4)).wrapping_add(0x70))
    };
    let head = e.mem.u32(geometry_group_of(e) + 0x10);
    let mut pass = if head != 0 { e.mem.u32(head + 8) } else { 0 };
    let mut current = x87_store32(0.0);
    if pass != 0 {
        current = next_alpha_distance_first(e, pass, d, subtract);
    }
    let list = base
        .wrapping_add(group.wrapping_mul(0xc))
        .wrapping_add(0x78);
    let mut alpha_group = if e.mem.u32(list + 8) != 0 {
        e.mem.u32(e.mem.u32(list) + 8)
    } else {
        0
    };
    let limit = x87_store32(x87_sub(x87_load32(limit.to_bits()), e.mem.f64(LIMIT_STEP)));
    let limit_value = x87_load32(limit);
    loop {
        // The group loop (L060).
        let mut pass_turn = false;
        while alpha_group != 0 {
            let group_distance = x87_load32(e.mem.u32(alpha_group + 0x20));
            let cur = x87_load32(current);
            if !(cur < group_distance) && pass != 0 {
                pass_turn = true;
                break;
            }
            if limit_value > group_distance || limit_value.is_nan() || group_distance.is_nan() {
                break;
            }
            if e.mem.u8(alpha_group) & 1 != 0 {
                fn_00b99990(e, Ptr::new(alpha_group + 0xc), 1);
            } else {
                let renderer = e.mem.u32(alpha_group + 8);
                if renderer != 0 {
                    e.vcall(renderer, 0x90, &[1, 0x2f1, 1]);
                }
            }
            let renderer = e.mem.u32(alpha_group + 8);
            if renderer != 0 {
                e.call(ACTIVE_LIST_CLEAR, &[renderer + 0x2c]);
            }
            e.mem.set_u16(alpha_group + 2, 0);
            bs_batch_renderer_geometry_group_destructor(e, Ptr::new(alpha_group));
            e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, alpha_group]);
            let node = e.mem.u32(list);
            let next = e.mem.u32(node);
            e.mem.set_u32(list, next);
            if next != 0 {
                e.mem.set_u32(next + 4, 0);
            } else {
                e.mem.set_u32(list + 4, 0);
            }
            e.mem.set_u32(node + 8, 0);
            e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
            e.mem.set_u32(node + 4, 0);
            let free = e.mem.u32(NODE_POOL_FREE_LIST);
            e.mem.set_u32(node, free);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            e.mem.set_u32(NODE_POOL_FREE_LIST, node);
            if count == 0 {
                e.mem.set_u32(NODE_POOL_LOCK, 0);
            }
            let remaining = e.mem.u32(list + 8).wrapping_sub(1);
            e.mem.set_u32(list + 8, remaining);
            alpha_group = if e.mem.u32(list + 8) != 0 {
                e.mem.u32(e.mem.u32(list) + 8)
            } else {
                0
            };
        }
        // L154 / L15c: the pass.
        let mut check_pass = pass_turn || pass != 0;
        if check_pass {
            let cur = x87_load32(current);
            if !(limit_value > cur || limit_value.is_nan() || cur.is_nan()) {
                render_alpha_pass(e, pass);
                let geometry_group = geometry_group_of(e);
                take_first_pass(e, geometry_group);
                let geometry_group = geometry_group_of(e);
                let head = e.mem.u32(geometry_group + 0x10);
                if head == 0 {
                    pass = 0;
                    check_pass = false;
                } else {
                    pass = e.mem.u32(head + 8);
                    if pass == 0 {
                        check_pass = false;
                    } else {
                        current = next_alpha_distance_next(e, pass, locals, subtract);
                    }
                }
            }
            // L324: the pass within the limit goes on to the NaN test.
            if check_pass {
                let cur = x87_load32(current);
                if limit_value <= cur {
                    if e.call(ISNAN, &args![x87_load32(current)]).u32() != 0 {
                        return;
                    }
                    continue;
                }
            }
        }
        // L33f: a group within the limit goes on to the NaN test.
        if alpha_group == 0 {
            return;
        }
        let group_distance = x87_load32(e.mem.u32(alpha_group + 0x20));
        if limit_value > group_distance || limit_value.is_nan() || group_distance.is_nan() {
            return;
        }
        if e.call(ISNAN, &args![x87_load32(current)]).u32() != 0 {
            return;
        }
    }
}

/// Renders one pass of `RenderNextAlpha` (see there).
fn render_alpha_pass(e: &mut Engine, pass: u32) {
    let geometry = e.mem.u32(pass);
    let property = e.mem.u32(geometry + 0xa8);
    if property == 0 {
        return;
    }
    if e.mem.u32(RENDER_MODE_011F91B8) == 9 {
        let kind = e.mem.u32(property + 0x1c);
        if kind == 0x15 || kind == 0x11 {
            return;
        }
    }
    let flags = e.mem.u32(property + 0x20);
    let biased = flags & 0x400_0000 != 0 || flags & 0x800_0000 != 0;
    if biased {
        let mut bias = x87_quiet_bits(e.mem.u32(DECAL_DEPTH_BIAS));
        if e.mem.u8(DECAL_DEPTH_BIAS_ENABLED) == 0 {
            bias = x87_store32(0.0);
        }
        e.call(SET_Z_WRITE_ENABLE, &[0, 1]);
        e.call(SET_DEPTH_BIAS, &[x87_quiet_bits(bias), 1]);
    }
    let pass_type = u32::from(e.mem.u16(pass + 4));
    bs_batch_renderer_render_pass_immediately(e, Ptr::new(pass), pass_type, 1, 1, 1);
    if biased {
        let count = e.mem.u32(LOCK_COUNT_1).wrapping_sub(1);
        e.mem.set_u32(LOCK_COUNT_1, count);
        e.call(SET_Z_WRITE_ENABLE, &[1, 0]);
        let count = e.mem.u32(LOCK_COUNT_3).wrapping_sub(1);
        e.mem.set_u32(LOCK_COUNT_3, count);
        e.call(SET_DEPTH_BIAS, &[x87_store32(0.0), 0]);
    }
}

/// Takes the first node off a geometry group's element (+0x0C; head +0x10,
/// tail +0x14, free list +0x18, count +0x1C) as `RenderNextAlpha` does,
/// counting the group's 16-bit +0x02 down first.
fn take_first_pass(e: &mut Engine, group: u32) {
    if e.mem.u32(group + 0x10) == 0 {
        return;
    }
    let count = e.mem.u16(group + 2).wrapping_add(0xffff);
    e.mem.set_u16(group + 2, count);
    let node = e.mem.u32(group + 0x10);
    if node == 0 {
        return;
    }
    if node == e.mem.u32(group + 0x14) {
        e.mem.set_u32(group + 0x14, 0);
    }
    let mut next = 0;
    if e.mem.u32(node) != 0 {
        next = e.mem.u32(node);
        e.mem.set_u32(next + 4, 0);
    }
    let free = e.mem.u32(group + 0x18);
    if free != 0 {
        let head = e.mem.u32(group + 0x10);
        e.mem.set_u32(free + 4, head);
    }
    let head = e.mem.u32(group + 0x10);
    let free = e.mem.u32(group + 0x18);
    e.mem.set_u32(head, free);
    let head = e.mem.u32(group + 0x10);
    let items = e.mem.u32(group + 0x1c).wrapping_sub(1);
    e.mem.set_u32(group + 0x1c, items);
    e.mem.set_u32(group + 0x18, head);
    e.mem.set_u32(group + 0x10, next);
}

// Translated from 00b99890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSTPersistentList` element,
/// one word: the address of the item): prepends the item (a node from the
/// free list or the allocator) and counts it (+0x10); an empty list's
/// tail becomes the node too.
pub fn fn_00b99890(e: &mut Engine, this: Ptr, item: Ptr) {
    let element = this.addr();
    let node = take_node(e, element);
    let value = e.mem.u32(item.addr());
    e.mem.set_u32(node + 8, value);
    let head = e.mem.u32(element + 4);
    e.mem.set_u32(node, head);
    e.mem.set_u32(node + 4, 0);
    let head = e.mem.u32(element + 4);
    let count = e.mem.u32(element + 0x10).wrapping_add(1);
    if head != 0 {
        e.mem.set_u32(head + 4, node);
        e.mem.set_u32(element + 0x10, count);
        e.mem.set_u32(element + 4, node);
    } else {
        e.mem.set_u32(element + 0x10, count);
        e.mem.set_u32(element + 8, node);
        e.mem.set_u32(element + 4, node);
    }
}

// Translated from 00b99b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of a
/// `BSTPersistentList` element (thiscall, one word: the flags). Sets the
/// element's vtable (`010b7dc0`), cleans it (`00b63e90`), moves its nodes
/// to the free list (+0x0C) and clears head, tail and count, cleans it
/// again, and with flag bit 0 gives the block back to the memory manager
/// (`00aa4060`). Answers `this`.
pub fn fn_00b99b70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let element = this.addr();
    e.mem.set_u32(element, PERSISTENT_LIST_VTABLE);
    e.call(PERSISTENT_LIST_CLEAR, &[element]);
    let head = e.mem.u32(element + 4);
    e.mem.set_u32(element + 0xc, head);
    e.mem.set_u32(element + 4, 0);
    e.mem.set_u32(element + 8, 0);
    e.mem.set_u32(element + 0x10, 0);
    e.call(PERSISTENT_LIST_CLEAR, &[element]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, element]);
    }
    this
}

/// `BSShaderManager::GetAccumulator` (engine map, `00b4f5c0`; no arguments).
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
/// The byte at `011f94a6` that makes `RenderBatches` first run `00bbb840`
/// (no arguments; it copies four words of `011a9bd0` to `01201b30` and sets
/// three flags).
const BATCH_SETUP_FLAG_011F94A6: u32 = 0x011f_94a6;
const BATCH_SETUP_00BBB840: u32 = 0x00bb_b840;

// Translated from 00b99e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer::RenderBatches` (Xbox PDB name, engine map): thiscall,
/// three words (the first and last pass type to render, `0` for 1..0x2F1,
/// and a word passed on to `RenderPassImmediately`). Runs `00bbb840` when
/// the byte at `011f94a6` is set. Then, when the active pass list (+0x2C)
/// is not empty, for each listed pass type within the range: looks up its
/// pass group in the render-pass map (+0x0C, `00853130`); a shadow pass
/// (`IsShadowPass`) while the accumulator's byte +0x164 or +0x165 is set
/// only has its group cleared (`00b99a50`). Otherwise each of the group's
/// five element lists that has a head node is rendered with its own state
/// (`00b984d0`, the alpha test `00b97ed0`, alpha to coverage `00b98540`
/// when the renderer has the switch at `011f9421`): every item through
/// `RenderPassImmediately(item, type, flag, third word, 0)` with the flag
/// 0 for lists 0 and 1 and 1 for lists 2..4, then alpha to coverage off
/// (twice with the switch, which also drops the count at `011ffa28`) and
/// the counters at `011ffa24` and `011ff9e8` dropped when not zero; then the
/// group is cleared. Ends with `00b984d0(1)`, the alpha test off and
/// `EndPass`.
///
/// The flag is a byte the exe stores in the second argument's stack word and
/// pushes as the whole word: its upper three bytes are those of the second
/// argument as passed (`RenderPassImmediately` reads only the low byte).
/// `00b984d0` is pushed a second word it does not read (cdecl).
pub fn bs_batch_renderer_render_batches(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    first: u32,
    last: u32,
    third: u32,
) {
    let base = this.addr();
    let (low, high) = if first == 0 {
        (1, 0x2f1)
    } else {
        (first, last)
    };
    if e.mem.u8(BATCH_SETUP_FLAG_011F94A6) != 0 {
        e.call(BATCH_SETUP_00BBB840, &[]);
    }
    let list = base + 0x2c;
    if e.mem.u32(base + 0x30) != 0 || e.mem.u16(list) != 0 {
        let mut node = list;
        while node != 0 {
            let pass_type = u32::from(e.mem.u16(node));
            if pass_type > high || pass_type < low {
                node = e.mem.u32(node + 4);
                continue;
            }
            let group = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), 0);
                e.call(MAP_GET_AT, &[base + 0xc, pass_type, slot.addr()]);
                e.mem.u32(slot.addr())
            });
            let skip = e.call(IS_SHADOW_PASS, &[pass_type]).u8() != 0 && {
                let accumulator = e.call(GET_ACCUMULATOR, &[]).u32();
                e.mem.u8(accumulator + 0x164) != 0 || {
                    let accumulator = e.call(GET_ACCUMULATOR, &[]).u32();
                    e.mem.u8(accumulator + 0x165) != 0
                }
            };
            let offset = group.wrapping_mul(0x68);
            if !skip {
                for index in 0..5u32 {
                    let groups = e.mem.u32(base + 8);
                    let element = (index * 0x14).wrapping_add(offset).wrapping_add(groups);
                    if e.mem.u32(element.wrapping_add(4)) == 0 {
                        continue;
                    }
                    let mut flag = 0u8;
                    let coverage = match index {
                        0 => {
                            fn_00b984d0(e, 1, 0);
                            e.call(SET_ALPHA_TEST_ENABLE, &[0, 0]);
                            Some(0)
                        }
                        1 => {
                            fn_00b984d0(e, 3, 1);
                            e.call(SET_ALPHA_TEST_ENABLE, &[0, 0]);
                            Some(0)
                        }
                        2 | 3 => {
                            fn_00b984d0(e, if index == 2 { 3 } else { 1 }, 1);
                            e.call(SET_ALPHA_TEST_ENABLE, &[1, 1]);
                            flag = 1;
                            Some(1)
                        }
                        _ => {
                            fn_00b984d0(e, 1, 1);
                            e.call(SET_ALPHA_TEST_ENABLE, &[1, 1]);
                            flag = 1;
                            Some(0)
                        }
                    };
                    if let Some(on) = coverage {
                        if e.mem.u8(COVERAGE_AVAILABLE) != 0 {
                            e.call(SET_ALPHA_TO_COVERAGE, &[on, 1]);
                        }
                    }
                    let flag_word = (last & 0xffff_ff00) | u32::from(flag);
                    let groups = e.mem.u32(base + 8);
                    let element = (index * 0x14).wrapping_add(offset).wrapping_add(groups);
                    let mut item_node = e.mem.u32(element.wrapping_add(4));
                    while item_node != 0 {
                        let item = e.mem.u32(item_node + 8);
                        item_node = e.mem.u32(item_node);
                        bs_batch_renderer_render_pass_immediately(
                            e,
                            Ptr::new(item),
                            pass_type,
                            flag_word,
                            third,
                            0,
                        );
                    }
                    if e.mem.u8(COVERAGE_AVAILABLE) != 0 {
                        let count = e.mem.u32(COVERAGE_LOCK).wrapping_sub(1);
                        e.mem.set_u32(COVERAGE_LOCK, count);
                        e.call(SET_ALPHA_TO_COVERAGE, &[0, 0]);
                    }
                    e.call(SET_ALPHA_TO_COVERAGE, &[0, 0]);
                    let count = e.mem.u32(LOCK_COUNT_011FFA24);
                    if count != 0 {
                        e.mem.set_u32(LOCK_COUNT_011FFA24, count - 1);
                    }
                    let count = e.mem.u32(LOCK_COUNT_4);
                    if count != 0 {
                        e.mem.set_u32(LOCK_COUNT_4, count - 1);
                    }
                }
            }
            let groups = e.mem.u32(base + 8);
            fn_00b99a50(e, Ptr::new(groups.wrapping_add(offset)), 0);
            node = e.mem.u32(node + 4);
        }
    }
    fn_00b984d0(e, 1, 0);
    e.call(SET_ALPHA_TEST_ENABLE, &[0, 0]);
    bs_batch_renderer_end_pass(e);
}

// Translated from 00b9a3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, four words; `RET 0x10`, the fourth
/// never read): looks up the pass group of the pass type `*pass_type` in
/// the render-pass map (+0x0C, `00853130`, the slot cleared first), clears
/// that group (`00b99a50`) and answers `00b9a090(pass_type, element,
/// flag)`.
pub fn fn_00b9a3c0(
    e: &mut Engine,
    this: Ptr<BSBatchRenderer>,
    pass_type: Ptr,
    element: Ptr,
    flag: Ptr,
    _unused_4: u32,
) -> bool {
    let base = this.addr();
    let key = e.mem.u32(pass_type.addr());
    let group = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(MAP_GET_AT, &[base + 0xc, key, slot.addr()]);
        e.mem.u32(slot.addr())
    });
    let groups = e.mem.u32(base + 8);
    fn_00b99a50(
        e,
        Ptr::new(group.wrapping_mul(0x68).wrapping_add(groups)),
        0,
    );
    fn_00b9a090(e, this, pass_type, element, flag)
}

// Translated from 00b9a4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: renders a `BSBatchRenderer::GeometryGroup`
/// (Xbox PDB; thiscall, one word passed on). With `ucFlags` bit 0 its own
/// pass list (+0x0C) through `00b99990`; else, when it has a
/// `pBatchRenderer` (+0x08), that renderer's virtual slot +0x90 with (1,
/// 0x2F1, the word). Then, with a renderer, clears the renderer's active
/// pass list (+0x2C, `00b64ec0`); either way clears `usCount` (+0x02).
pub fn fn_00b9a4d0(e: &mut Engine, this: Ptr, word: u32) {
    let group = this.addr();
    if e.mem.u8(group) & 1 != 0 {
        fn_00b99990(e, Ptr::new(group + 0xc), word);
    } else {
        let renderer = e.mem.u32(group + 8);
        if renderer != 0 {
            e.vcall(renderer, 0x90, &[1, 0x2f1, word]);
        }
    }
    let renderer = e.mem.u32(group + 8);
    if renderer != 0 {
        e.call(ACTIVE_LIST_CLEAR, &[renderer + 0x2c]);
    }
    e.mem.set_u16(group + 2, 0);
}

/// `BSBatchRenderer::ms_RTTI` (`011ffe3c`).
const BATCH_RENDERER_RTTI: u32 = 0x011f_fe3c;

// Translated from 00b9aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBatchRenderer`'s `GetRTTI` (slot +8 of its vtable `010b7dc8`, the
/// only reference; Ghidra made no function of it, read from the bytes):
/// answers `&ms_RTTI` (`011ffe3c`).
pub fn bs_batch_renderer_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    BATCH_RENDERER_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_9890, fn_00b99890(Ptr, Ptr)),
        entry!(0x00b9_9b70, fn_00b99b70(Ptr, u32) -> Ptr),
        entry!(
            0x00b9_9e20,
            bs_batch_renderer_render_batches(Ptr<BSBatchRenderer>, u32, u32, u32)
        ),
        entry!(
            0x00b9_a3c0,
            fn_00b9a3c0(Ptr<BSBatchRenderer>, Ptr, Ptr, Ptr, u32) -> bool
        ),
        entry!(0x00b9_a4d0, fn_00b9a4d0(Ptr, u32)),
        entry!(0x00b9_aa70, bs_batch_renderer_get_rtti(Ptr) -> u32),
        entry!(0x00b9_84d0, fn_00b984d0(u32, u32)),
        entry!(0x00b9_88e0, bs_batch_renderer_end_pass()),
        entry!(
            0x00b9_8970,
            fn_00b98970(Ptr<BSBatchRenderer>, u32, u32) -> bool
        ),
        entry!(
            0x00b9_8ae0,
            fn_00b98ae0(Ptr<BSBatchRenderer>, u32, Ptr) -> bool
        ),
        entry!(0x00b9_8b20, fn_00b98b20(u32, u32) -> i32),
        entry!(
            0x00b9_8c80,
            fn_00b98c80(Ptr<BSBatchRenderer>, u32, Ptr, u8) -> f64
        ),
        entry!(0x00b9_8e20, fn_00b98e20(Ptr, Ptr)),
        entry!(
            0x00b9_8e80,
            bs_batch_renderer_render_pass_immediately_standard(Ptr, u8, u8, u8)
        ),
        entry!(
            0x00b9_9110,
            bs_batch_renderer_render_pass_immediately_skinned(Ptr, u8, u8, u8)
        ),
        entry!(0x00b9_9390, bs_batch_renderer_begin_pass(u32, Ptr)),
        entry!(
            0x00b9_94f0,
            bs_batch_renderer_render_pass_immediately(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00b9_9630, fn_00b99630(Ptr, u32)),
        entry!(0x00b9_9730, fn_00b99730(Ptr)),
        entry!(0x00b9_9770, fn_00b99770(Ptr, u32, u32)),
        entry!(0x00b9_9840, fn_00b99840(Ptr, Ptr)),
        entry!(
            0x00b9_9900,
            fn_00b99900(Ptr<BSBatchRenderer>, Ptr, u32) -> bool
        ),
        entry!(0x00b9_9990, fn_00b99990(Ptr, u32)),
        entry!(0x00b9_9a10, fn_00b99a10(Ptr<BSBatchRenderer>, u32, u8)),
        entry!(0x00b9_9a50, fn_00b99a50(Ptr, u32)),
        entry!(0x00b9_9a90, fn_00b99a90(Ptr, u32, u32)),
        entry!(0x00b9_9b00, fn_00b99b00(Ptr, u32, u32)),
        entry!(0x00b9_9c90, fn_00b99c90(Ptr<BSBatchRenderer>, u32, u32)),
        entry!(
            0x00b9_a090,
            fn_00b9a090(Ptr<BSBatchRenderer>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00b9_a120,
            fn_00b9a120(Ptr<BSBatchRenderer>, Ptr, Ptr, Ptr, u32) -> bool
        ),
        entry!(
            0x00b9_a410,
            bs_batch_renderer_geometry_group_destructor(Ptr)
        ),
        entry!(0x00b9_a490, fn_00b9a490(Ptr)),
        entry!(0x00b9_aa80, fn_00b9aa80(Ptr<BSBatchRenderer>)),
        entry!(0x00b9_abc0, fn_00b9abc0(Ptr, u32, u8)),
        entry!(
            0x00b9_ac80,
            bs_batch_renderer_start_grouping_alphas(Ptr<BSBatchRenderer>, Ptr, u8, Ptr, u8)
        ),
        entry!(
            0x00b9_aec0,
            bs_batch_renderer_stop_grouping_alphas(Ptr<BSBatchRenderer>)
        ),
        entry!(
            0x00b9_af60,
            bs_batch_renderer_render_next_alpha(Ptr<BSBatchRenderer>, u32, Ptr, u8, f32)
        ),
        entry!(
            0x00b9_a5b0,
            fn_00b9a5b0(Ptr<BSBatchRenderer>, u32, i32) -> Ptr<BSBatchRenderer>
        ),
        entry!(
            0x00b9_98e0,
            fn_00b998e0(Ptr<crate::types::NiTPointerList>) -> u32
        ),
        entry!(0x00b9_98f0, fn_00b998f0(Ptr)),
        entry!(0x00b9_a530, fn_00b9a530(Ptr) -> Ptr),
        entry!(0x00b9_9bb0, fn_00b99bb0(Ptr) -> u32),
        entry!(0x00b9_9bc0, fn_00b99bc0(Ptr<BSBatchRenderer>, u32)),
        entry!(0x00b9_a560, fn_00b9a560(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod pass_group_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn list_and_group_constructors_and_destructors() {
        let mut e = Engine::new();
        e.register(POINTER_LIST_REMOVE_ALL, |_, _| 0u32.into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, a| a[0].into_ret());
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| 0x66u32.into_ret());
        let list = e.mem.alloc(0xc);
        e.mem.write(list, &[0xff; 0xc]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_98e0, &[list]).u32(), list);
        assert_eq!(e.mem.bytes(list, 0xc), vec![0; 0xc]);
        e.call(0x00b9_98f0, &[list]);
        assert_eq!(calls_to(&e, POINTER_LIST_REMOVE_ALL), vec![vec![list]]);
        let group = e.mem.alloc(0x68);
        e.mem.set_u32(group + 0x64, 9);
        assert_eq!(e.call(0x00b9_a530, &[group]).u32(), group);
        assert_eq!(e.mem.u32(group + 0x64), 0);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![
                group,
                0x14,
                5,
                PERSISTENT_LIST_CONSTRUCTOR,
                PERSISTENT_LIST_DESTRUCTOR
            ]]
        );
        assert_eq!(e.call(0x00b9_9bb0, &[group]).u32(), 0x66);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![group, 0x14, 5, PERSISTENT_LIST_DESTRUCTOR]]
        );
    }

    #[test]
    fn geometry_group_constructor_cleans_its_element() {
        let mut e = Engine::new();
        // The clean-up sees the inline-constructed element and leaves a
        // value in +0x04 to be moved.
        e.register(PERSISTENT_LIST_CLEAR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), PERSISTENT_LIST_VTABLE);
            e.mem.set_u32(a[0] + 4, 0x44);
            e.mem.set_u32(a[0] + 0x10, 0x10);
            0u32.into_ret()
        });
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xee; 0x24]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_a560, &[this, 0x2f7]).u32(), this);
        assert_eq!(calls_to(&e, PERSISTENT_LIST_CLEAR), vec![vec![this + 0xc]]);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(e.mem.u8(this + 1), 0xee);
        assert_eq!(e.mem.u16(this + 2), 0);
        assert_eq!(e.mem.u32(this + 4), 0x2f7);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), PERSISTENT_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0x44);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0xeeee_eeee);
    }

    /// A renderer with a 4-bucket render-pass map holding `items`
    /// (bucket, key, pass) in bucket order, three pass groups, and doubles
    /// for the map's `KeyToHashIndex` (the key's bucket), the clean-up and
    /// the active list clear.
    fn renderer(e: &mut Engine, items: &[(u32, u32, u32)]) -> u32 {
        let this = e.mem.alloc(0x98);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0077_0004);
        e.register(0x0077_0004, |_, a| (a[1] >> 8).into_ret());
        e.mem.set_u32(this + 0xc, vtable);
        e.mem.set_u32(this + 0x10, 4);
        let buckets = e.mem.alloc(16);
        e.mem.set_u32(this + 0x14, buckets);
        let mut last_in_bucket: [u32; 4] = [0; 4];
        for (bucket, key, pass) in items {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item + 4, *key);
            e.mem.set_u32(item + 8, *pass);
            let slot = *bucket as usize;
            if last_in_bucket[slot] == 0 {
                e.mem.set_u32(buckets + bucket * 4, item);
            } else {
                e.mem.set_u32(last_in_bucket[slot], item);
            }
            last_in_bucket[slot] = item;
        }
        let groups = e.mem.alloc(0x68 * 3);
        e.mem.set_u32(this + 8, groups);
        for group in 0..3 {
            let group = groups + group * 0x68;
            e.mem.set_u32(group + 0x64, 7);
            for index in 0..5 {
                let element = group + index * 0x14;
                e.mem.set_u32(element + 4, 0x40 + index);
                e.mem.set_u32(element + 8, 0x80);
                e.mem.set_u32(element + 0x10, u32::from(index % 2 == 0));
            }
        }
        e.register(PERSISTENT_LIST_CLEAR, |_, _| 0u32.into_ret());
        e.register(ACTIVE_LIST_CLEAR, |_, _| 0u32.into_ret());
        this
    }

    #[test]
    fn clear_passes_resets_every_mapped_group_and_the_active_list() {
        let mut e = Engine::new();
        // Bucket 0: passes 0 then 2 (keys 0x001, 0x002); bucket 3: pass 2
        // again (key 0x300). Pass 1 is not mapped.
        let this = renderer(&mut e, &[(0, 0x001, 0), (0, 0x002, 2), (3, 0x300, 2)]);
        e.call_log = Some(vec![]);
        e.call(0x00b9_9bc0, &[this, 0x1234]);
        let groups = e.mem.u32(this + 8);
        for pass in [0u32, 2] {
            let group = groups + pass * 0x68;
            assert_eq!(e.mem.u32(group + 0x64), 0);
            for index in 0..5 {
                let element = group + index * 0x14;
                if index % 2 == 0 {
                    assert_eq!(e.mem.u32(element + 0xc), 0x40 + index);
                    assert_eq!(e.mem.u32(element + 4), 0);
                    assert_eq!(e.mem.u32(element + 8), 0);
                    assert_eq!(e.mem.u32(element + 0x10), 0);
                } else {
                    assert_eq!(e.mem.u32(element + 4), 0x40 + index);
                }
            }
        }
        // Pass 1 is untouched.
        assert_eq!(e.mem.u32(groups + 0x68 + 0x64), 7);
        // The second bucket-0 item ends its bucket: the map is asked once
        // there (and once at the end of bucket 3).
        assert_eq!(
            calls_to(&e, 0x0077_0004),
            vec![vec![this + 0xc, 0x002], vec![this + 0xc, 0x300]]
        );
        assert_eq!(calls_to(&e, ACTIVE_LIST_CLEAR), vec![vec![this + 0x2c]]);
    }

    #[test]
    fn clear_passes_with_an_empty_map_only_clears_the_active_list() {
        let mut e = Engine::new();
        let this = renderer(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00b9_9bc0, &[this, 0]);
        assert!(calls_to(&e, PERSISTENT_LIST_CLEAR).is_empty());
        assert_eq!(calls_to(&e, ACTIVE_LIST_CLEAR), vec![vec![this + 0x2c]]);
        // No buckets at all.
        e.mem.set_u32(this + 0x10, 0);
        e.call(0x00b9_9bc0, &[this, 0]);
        assert_eq!(calls_to(&e, ACTIVE_LIST_CLEAR).len(), 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An engine with doubles for every callee: allocations succeed except
    /// for the size `fail`, constructors return their `this`; the TLS word
    /// starts at 7 and each double checks it is 0x26 while it runs.
    fn renderer_engine(fail: u32) -> Engine {
        let mut e = Engine::new();
        fn in_context(e: &mut Engine) {
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 0x26);
        }
        e.register(OBJECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ALLOCATE_BUCKETS, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, a| a[0].into_ret());
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            in_context(e);
            if a[1] == fail {
                0u32.into_ret()
            } else {
                e.mem.alloc(a[1].min(0x1000)).into_ret()
            }
        });
        e.register(INSTANCE_RENDERER_CONSTRUCTOR, |e, a| {
            in_context(e);
            a[0].into_ret()
        });
        e.register(RENDERER_METHOD_B99BC0, |e, _| {
            in_context(e);
            0u32.into_ret()
        });
        e.register(GEOMETRY_GROUP_CONSTRUCTOR, |e, a| {
            in_context(e);
            e.mem.set_u8(a[0], 0x10);
            a[0].into_ret()
        });
        e.register(POINTER_LIST_REMOVE_ALL, |_, _| 0u32.into_ret());
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2B4, 7);
        e
    }

    fn build(e: &mut Engine, pass_count: u32, kind: i32) -> u32 {
        let this = e.mem.alloc(0x98);
        for off in (4..0x98).step_by(4) {
            e.mem.set_u32(this + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        let result = e.call(0x00b9_a5b0, &[this, pass_count, kind as u32]).u32();
        assert_eq!(result, this);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 7);
        this
    }

    #[test]
    fn kind_99_builds_every_group_and_the_pass_groups() {
        let mut e = renderer_engine(u32::MAX);
        let this = build(&mut e, 3, 99);
        assert_eq!(e.mem.u32(this), BATCH_RENDERER_VTABLE);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 0xc), MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0x25);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        let buckets = e.mem.u32(this + 0x14);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![buckets, 0, 0x94]]);
        assert_eq!(e.mem.u16(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        let groups = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(groups - 4), 3);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![
                vec![
                    this + 0x78,
                    0xc,
                    2,
                    ALPHA_LIST_CONSTRUCTOR,
                    ALPHA_LIST_DESTRUCTOR
                ],
                vec![
                    groups,
                    0x68,
                    3,
                    PASS_GROUP_CONSTRUCTOR,
                    PASS_GROUP_DESTRUCTOR
                ],
            ]
        );
        assert_ne!(e.mem.u32(this + 0x28), 0);
        assert_eq!(calls_to(&e, RENDERER_METHOD_B99BC0), vec![vec![this, 0]]);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[1]).collect();
        let mut expected = vec![4, 3 * 0x68 + 4];
        expected.extend([0x24; 15]);
        assert_eq!(sizes, expected);
        let words: Vec<u32> = calls_to(&e, GEOMETRY_GROUP_CONSTRUCTOR)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(
            words,
            ALL_GROUPS.iter().map(|(_, word)| *word).collect::<Vec<_>>()
        );
        for (slot, _) in ALL_GROUPS {
            assert_ne!(e.mem.u32(this + slot), 0);
        }
        // Only the two alpha groups get bit 0.
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x6c)), 0x10);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x70)), 0x11);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x74)), 0x11);
        assert_eq!(
            calls_to(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![this + 0x78], vec![this + 0x84]]
        );
    }

    #[test]
    fn kind_100_clears_the_groups_and_kind_101_builds_three() {
        let mut e = renderer_engine(u32::MAX);
        let this = build(&mut e, 1, 100);
        for (slot, _) in ALL_GROUPS {
            assert_eq!(e.mem.u32(this + slot), 0);
        }
        assert!(calls_to(&e, GEOMETRY_GROUP_CONSTRUCTOR).is_empty());
        assert!(calls_to(&e, POINTER_LIST_REMOVE_ALL).is_empty());

        let mut e = renderer_engine(u32::MAX);
        let this = build(&mut e, 1, 101);
        for (slot, _) in ALL_GROUPS {
            let built = matches!(slot, 0x60 | 0x70 | 0x74);
            assert_eq!(e.mem.u32(this + slot) != 0, built, "+{slot:#x}");
        }
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_CONSTRUCTOR)
                .iter()
                .map(|a| a[1])
                .collect::<Vec<_>>(),
            vec![0x2f7; 3]
        );
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x70)), 0x11);
        assert_eq!(calls_to(&e, POINTER_LIST_REMOVE_ALL).len(), 2);
    }

    #[test]
    fn other_kinds_leave_the_groups_and_failed_allocations_stay_null() {
        // 0x24-byte allocations never happen; the pass-group block fails.
        let mut e = renderer_engine(0x68 * 2 + 4);
        let this = build(&mut e, 2, 5);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        assert_eq!(calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR).len(), 1);
        for (slot, _) in ALL_GROUPS {
            assert_eq!(e.mem.u32(this + slot), 0xdead_beef);
        }
        // A failed instance renderer stays null; a huge pass count saturates.
        let mut e = renderer_engine(4);
        let this = build(&mut e, 0x0400_0000, 100);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert!(calls_to(&e, INSTANCE_RENDERER_CONSTRUCTOR).is_empty());
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[1]).collect();
        assert_eq!(sizes, vec![4, u32::MAX]);
        // Kind 99 with failing group allocations stores nulls.
        let mut e = renderer_engine(0x24);
        let this = e.mem.alloc(0x98);
        e.call_log = Some(vec![]);
        // Flagging the null alpha groups writes at address 0: map it.
        e.map(0, 0x1000);
        e.call(0x00b9_a5b0, &[this, 1, 99]);
        for (slot, _) in ALL_GROUPS {
            assert_eq!(e.mem.u32(this + slot), 0);
        }
        assert!(calls_to(&e, GEOMETRY_GROUP_CONSTRUCTOR).is_empty());
        assert_eq!(e.mem.u8(0), 1);
    }
}

#[cfg(test)]
mod b061_tests {
    use super::*;

    /// An engine with the globals' pages mapped and the logging on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x2000);
        e.map(0x011a_d000, 0x2000);
        e.map(0x011c_5000, 0x1000);
        e.map(0x011f_4000, 0xc000);
        e.map(0x0120_0000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn quiet(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The addresses called, in order, among `of`.
    fn order(e: &Engine, of: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| of.contains(a))
            .collect()
    }

    /// An object whose vtable slots `slots` call doubles at `base + slot`
    /// (answering 0).
    fn object(e: &mut Engine, base: u32, slots: &[u32]) -> u32 {
        let vtable = e.mem.alloc(0x120);
        for &slot in slots {
            e.mem.set_u32(vtable + slot, base + slot);
            quiet(e, base + slot);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A camera whose direction floats are `d`.
    fn camera(e: &mut Engine, d: [f32; 3]) -> u32 {
        let camera = e.mem.alloc(0x84);
        for (offset, value) in [0x68, 0x74, 0x80].into_iter().zip(d) {
            e.mem.set_f32(camera + offset, value);
        }
        camera
    }

    fn bound(e: &mut Engine, values: [f32; 4]) -> u32 {
        let bound = e.mem.alloc(0x10);
        for (i, value) in values.into_iter().enumerate() {
            e.mem.set_f32(bound + i as u32 * 4, value);
        }
        bound
    }

    /// The geometry doubles' base.
    const GEOMETRY: u32 = 0x0068_0000;
    /// The shader doubles' base.
    const SHADER: u32 = 0x0067_0000;

    /// A pass of type `pass_type` (byte +7 `flag`) whose geometry renders
    /// itself (slot +0x30 answers 1, then slot +0xDC), with the shader
    /// doubles `RenderPassImmediately` needs.
    fn own_pass(e: &mut Engine, pass_type: u16, flag: u8) -> (u32, u32) {
        let shader = object(e, SHADER, &[0xf0, 0xf4, 0xf8, 0xfc, 0x100, 0x104, 0x114]);
        let geometry = object(e, GEOMETRY, &[0x30, 0x34, 0xdc]);
        e.register(GEOMETRY + 0x30, |_, _| 1u32.into_ret());
        e.register(RENDERER_IS_SKINNED, |_, _| 0u32.into_ret());
        let properties = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0xc0, shader);
        e.mem.set_u32(geometry + 0xa8, properties);
        let pass = e.mem.alloc(8);
        e.mem.set_u32(pass, geometry);
        e.mem.set_u16(pass + 4, pass_type);
        e.mem.set_u8(pass + 7, flag);
        (pass, geometry)
    }

    #[test]
    fn state_helper_calls_the_render_state_object() {
        let mut e = engine();
        let renderer = e.mem.alloc(0x8bc);
        e.mem.set_u32(renderer + 0x8b8, 0x4444);
        e.mem.set_u32(RENDERER, renderer);
        quiet(&mut e, 0x00e8_79b0);
        e.call(0x00b9_84d0, &[3, 1]);
        assert_eq!(calls_to(&e, 0x00e8_79b0), vec![vec![0x4444, 3, 0]]);
    }

    #[test]
    fn end_pass_resets_the_samplers_of_three_types() {
        let mut e = engine();
        let shader = object(&mut e, SHADER, &[0x104, 0x114]);
        let device = object(&mut e, 0x0069_0000, &[0x114]);
        let holder = e.mem.alloc(0x28c);
        e.mem.set_u32(holder + 0x288, device);
        e.mem.set_u32(RENDER_HOLDER, holder);
        e.mem.set_u32(CURRENT_SHADER, shader);
        e.mem.set_u32(CURRENT_PASS_TYPE, 0xfe);
        e.call(0x00b9_88e0, &[]);
        assert_eq!(calls_to(&e, SHADER + 0x104), vec![vec![shader, 0xfe]]);
        assert_eq!(calls_to(&e, SHADER + 0x114), vec![vec![shader]]);
        assert_eq!(calls_to(&e, 0x0069_0114), vec![vec![device, 7, 8, 0]]);
        assert_eq!(e.mem.u32(CURRENT_SHADER), 0);
        assert_eq!(e.mem.u32(CURRENT_PASS_TYPE), 0);
        // Type 0x230 uses sampler 2; the flag skips slot +0x114.
        e.mem.set_u32(CURRENT_SHADER, shader);
        e.mem.set_u32(CURRENT_PASS_TYPE, 0x230);
        e.mem.set_u8(PASS_FLAG_011AD8EC, 1);
        e.call(0x00b9_88e0, &[]);
        assert_eq!(calls_to(&e, SHADER + 0x114).len(), 1);
        assert_eq!(calls_to(&e, 0x0069_0114)[1], vec![device, 2, 8, 0]);
        // Another type: no sampler; no pass: nothing.
        e.mem.set_u32(CURRENT_SHADER, shader);
        e.mem.set_u32(CURRENT_PASS_TYPE, 0x10);
        e.call(0x00b9_88e0, &[]);
        e.call(0x00b9_88e0, &[]);
        assert_eq!(calls_to(&e, 0x0069_0114).len(), 2);
        assert_eq!(calls_to(&e, SHADER + 0x104).len(), 3);
    }

    #[test]
    fn listed_pass_types_are_found_within_the_range() {
        let mut e = engine();
        let this = e.mem.alloc(0x98);
        let node = e.mem.alloc(8);
        e.mem.set_u16(this + 0x2c, 0x100);
        e.mem.set_u32(this + 0x30, node);
        e.mem.set_u16(node, 0x28a);
        assert!(e.call(0x00b9_8970, &[this, 0x200, 0x300]).bool());
        assert!(!e.call(0x00b9_8970, &[this, 0x28b, 0x300]).bool());
        assert!(!e.call(0x00b9_8970, &[this, 0, 0x289]).bool());
        // 0x100 is in range but not listed.
        assert!(!e.call(0x00b9_8970, &[this, 0, 0x200]).bool());
    }

    fn renderer_with_groups(e: &mut Engine, passes: u32) -> u32 {
        let this = e.mem.alloc(0x98);
        let groups = e.mem.alloc(0x68 * passes);
        e.mem.set_u32(this + 8, groups);
        this
    }

    #[test]
    fn first_full_element_of_a_pass_group() {
        let mut e = engine();
        let this = renderer_with_groups(&mut e, 2);
        let groups = e.mem.u32(this + 8);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 9);
        assert!(!e.call(0x00b9_8ae0, &[this, 1, out]).bool());
        assert_eq!(e.mem.u32(out), 9);
        e.mem.set_u32(groups + 0x68 + 2 * 0x14 + 4, 0x55);
        e.mem.set_u32(groups + 0x68 + 4 * 0x14 + 4, 0x55);
        assert!(e.call(0x00b9_8ae0, &[this, 1, out]).bool());
        assert_eq!(e.mem.u32(out), 2);
    }

    /// An entry pointing to a pass whose geometry has `bound` at +0x20.
    fn sort_entry(e: &mut Engine, bound: u32) -> u32 {
        let geometry = e.mem.alloc(0x24);
        e.mem.set_u32(geometry + 0x20, bound);
        let pass = e.mem.alloc(8);
        e.mem.set_u32(pass, geometry);
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, pass);
        entry
    }

    #[test]
    fn alpha_sort_compares_distances_along_the_camera() {
        let mut e = engine();
        let cam = camera(&mut e, [1.0, 0.0, 0.0]);
        e.mem.set_u32(SORT_CAMERA, cam);
        e.mem.set_f32(DEFAULT_BOUND, 2.0);
        let near = bound(&mut e, [5.0, 0.0, 0.0, 1.0]);
        let a = sort_entry(&mut e, near);
        let b = sort_entry(&mut e, 0);
        assert_eq!(e.call(0x00b9_8b20, &[a, b]).i32(), -1);
        assert_eq!(e.call(0x00b9_8b20, &[b, a]).i32(), 1);
        assert_eq!(e.call(0x00b9_8b20, &[a, a]).i32(), 0);
        // With the radius taken off: 4 against 2, and 2 against 2.
        e.mem.set_u8(SORT_SUBTRACT_RADIUS, 1);
        assert_eq!(e.call(0x00b9_8b20, &[b, a]).i32(), 1);
        let far = bound(&mut e, [3.0, 0.0, 0.0, 1.0]);
        let c = sort_entry(&mut e, far);
        assert_eq!(e.call(0x00b9_8b20, &[c, b]).i32(), 0);
    }

    #[test]
    fn group_distance_by_its_contents() {
        let mut e = engine();
        e.mem.set_u32(NO_DISTANCE, 0xff7f_ffff);
        let this = e.mem.alloc(0x98);
        let group = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x70, group);
        let cam = camera(&mut e, [1.0, 1.0, 0.0]);
        let r = |e: &mut Engine, flag: u32| e.call(0x00b9_8c80, &[this, 0, cam, flag]).st0;
        assert_eq!(r(&mut e, 0), f64::from(f32::MIN));
        // The alpha-group list's first item.
        let item = e.mem.alloc(0x24);
        e.mem.set_f32(item + 0x20, 7.5);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, item);
        e.mem.set_u32(this + 0x78, node);
        e.mem.set_u32(this + 0x80, 1);
        assert_eq!(r(&mut e, 0), 7.5);
        // A group with a pass: its bound's distance, through 00404010 when
        // the list has items.
        let b = bound(&mut e, [3.0, 4.0, 0.0, 1.0]);
        let entry = sort_entry(&mut e, b);
        let pass = e.mem.u32(entry);
        let pass_node = e.mem.alloc(0xc);
        e.mem.set_u32(pass_node + 8, pass);
        e.mem.set_u32(group + 0x10, pass_node);
        e.mem.set_u16(group + 2, 1);
        e.register(FLOAT_MAX_00404010, |_, _| 9.0f64.into_ret());
        assert_eq!(r(&mut e, 1), 9.0);
        assert_eq!(
            calls_to(&e, FLOAT_MAX_00404010),
            vec![vec![7.5f32.to_bits(), 6.0f32.to_bits()]]
        );
        e.mem.set_u32(this + 0x80, 0);
        assert_eq!(r(&mut e, 0), 7.0);
        assert_eq!(r(&mut e, 1), 6.0);
    }

    #[test]
    fn short_list_front_insert() {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let head = e.mem.alloc(8);
        let item = e.mem.alloc(4);
        e.mem.set_u16(item, 4);
        e.call(0x00b9_8e20, &[head, item]);
        assert_eq!(e.mem.u16(head), 4);
        assert_eq!(e.mem.u32(head + 4), 0);
        e.mem.set_u16(item, 2);
        e.call(0x00b9_8e20, &[head, item]);
        assert_eq!(e.mem.u16(head), 2);
        let node = e.mem.u32(head + 4);
        assert_eq!(e.mem.u16(node), 4);
        assert_eq!(e.mem.u32(node + 4), 0);
        // A zero item changes nothing.
        e.mem.set_u16(item, 0);
        e.call(0x00b9_8e20, &[head, item]);
        assert_eq!(e.mem.u16(head), 2);
    }

    /// A standard pass with a shader whose used slots are doubles.
    fn standard_pass(e: &mut Engine, pass_type: u16, flag: u8) -> (u32, u32, u32) {
        let shader = object(
            e,
            SHADER,
            &[0x6c, 0x78, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x110],
        );
        let geometry = object(e, GEOMETRY, &[0xe0]);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 0x34, 0x3434);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xc0, shader);
        let holder = e.mem.alloc(0x8b4);
        e.mem.set_u32(holder + 0x8ac, 0x8ac0);
        e.mem.set_u32(holder + 0x8b0, 0x8b00);
        e.mem.set_u32(RENDER_HOLDER, holder);
        for addr in [
            GROUP_MANAGER_ADD_OBJECT,
            SHADER_SETUP_00B7DDE0,
            SHADER_SETUP_00B7E150,
            SHADER_SETUP_00B7DED0,
            SHADER_SETUP_00B7DFE0,
            IS_SHADOW_PASS,
        ] {
            quiet(e, addr);
        }
        let pass = e.mem.alloc(8);
        e.mem.set_u32(pass, geometry);
        e.mem.set_u16(pass + 4, pass_type);
        e.mem.set_u8(pass + 7, flag);
        (pass, geometry, shader)
    }

    const SHADER_SLOTS: [u32; 8] = [
        SHADER + 0x6c,
        SHADER + 0x78,
        SHADER + 0x7c,
        SHADER + 0x80,
        SHADER + 0x84,
        SHADER + 0x88,
        SHADER + 0x8c,
        SHADER + 0x110,
    ];

    #[test]
    fn standard_pass_by_type() {
        let mut e = engine();
        let (pass, geometry, shader) = standard_pass(&mut e, 0xca, 1);
        e.call(0x00b9_8e80, &[pass, 1, 1, 1]);
        let holder = e.mem.u32(RENDER_HOLDER);
        let data = e.mem.u32(geometry + 0xb8);
        assert_eq!(
            calls_to(&e, GROUP_MANAGER_ADD_OBJECT),
            vec![vec![0x8ac0, 0x8b00, data, 0, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(holder + 0xc), geometry + 0x9c);
        assert_eq!(
            order(&e, &SHADER_SLOTS),
            vec![
                SHADER + 0x84,
                SHADER + 0x80,
                SHADER + 0x88,
                SHADER + 0x6c,
                SHADER + 0x8c
            ]
        );
        let state = geometry + 0x9c;
        assert_eq!(
            calls_to(&e, SHADER_SETUP_00B7DED0),
            vec![vec![shader, state]]
        );
        assert_eq!(calls_to(&e, SHADER + 0x88), vec![vec![shader, state, 1]]);
        assert_eq!(
            calls_to(&e, SHADER + 0x6c),
            vec![vec![shader, geometry, 0, 0x3434, state]]
        );
        assert_eq!(calls_to(&e, GEOMETRY + 0xe0), vec![vec![geometry, holder]]);
        // 0xFC: no +0x88 (third is 0 here and its arm has none); another
        // type with the byte set; type 4 skips +0x84.
        let cases: [(u16, Vec<u32>); 3] = [
            (0xfc, vec![0x84, 0x80, 0x6c, 0x8c]),
            (3, vec![0x78, 0x7c, 0x80, 0x84, 0x6c, 0x8c]),
            (4, vec![0x78, 0x7c, 0x80, 0x6c, 0x8c]),
        ];
        for (pass_type, expected) in cases {
            let mut e = engine();
            let (pass, _, _) = standard_pass(&mut e, pass_type, 1);
            e.call(0x00b9_8e80, &[pass, 1, 1, 0]);
            let expected: Vec<u32> = expected.iter().map(|s| SHADER + s).collect();
            assert_eq!(order(&e, &SHADER_SLOTS), expected, "{pass_type:#x}");
        }
        // Byte +7 clear: slot +0x110 when the flag is set, then cleared.
        let mut e = engine();
        let (pass, _, _) = standard_pass(&mut e, 3, 0);
        e.mem.set_u8(PASS_FLAG_011AD8EC, 1);
        e.call(0x00b9_8e80, &[pass, 1, 1, 0]);
        let expected: Vec<u32> = [0x78, 0x7c, 0x110, 0x6c, 0x8c]
            .iter()
            .map(|s| SHADER + s)
            .collect();
        assert_eq!(order(&e, &SHADER_SLOTS), expected);
        assert_eq!(e.mem.u8(PASS_FLAG_011AD8EC), 0);
    }

    #[test]
    fn skinned_pass_draws_partitions_by_their_flags() {
        let mut e = engine();
        let (pass, geometry, shader) = standard_pass(&mut e, 3, 1);
        let vtable = e.mem.u32(shader);
        e.mem.set_u32(vtable + 0x64, SHADER + 0x64);
        quiet(&mut e, SHADER + 0x64);
        e.register(SHADER + 0x6c, |_, a| (a[2] + 1).into_ret());
        let skin = object(&mut e, 0x0069_0000, &[8]);
        e.register(0x0069_0008, |_, _| SKIN_RTTI_011F49D8.into_ret());
        let partition_data = e.mem.alloc(0x10);
        let partitions = e.mem.alloc(3 * 0x2c);
        for i in 0..3 {
            e.mem.set_u32(partitions + i * 0x2c + 0x28, 0x100 + i);
        }
        e.mem.set_u32(partition_data + 8, 3);
        e.mem.set_u32(partition_data + 0xc, partitions);
        e.mem.set_u32(skin + 0xc, partition_data);
        // Partition 0 drawn, 1 drawn with it (its second byte clear), 2
        // skipped (first byte clear).
        let flags = e.mem.alloc(12);
        e.mem.set_u8(flags, 1);
        e.mem.set_u8(flags + 4, 1);
        e.mem.set_u8(flags + 9, 1);
        e.mem.set_u32(skin + 0x38, flags);
        e.mem.set_u32(geometry + 0xbc, skin);
        let data = e.mem.u32(geometry + 0xb8);
        e.mem.set_u16(data + 0xe, 0x1234);
        e.register(GET_WORLD_BOUND, |_, _| 0xb0b0u32.into_ret());
        quiet(&mut e, HOLDER_DRAW_00E6D310);
        e.call(0x00b9_9110, &[pass, 1, 1, 0]);
        let state = geometry + 0x9c;
        assert_eq!(
            calls_to(&e, SHADER + 0x64),
            vec![vec![
                shader,
                geometry,
                skin,
                partitions,
                0x100,
                state,
                0,
                geometry + 0x68,
                0xb0b0
            ]]
        );
        let p1 = partitions + 0x2c;
        let holder = e.mem.u32(RENDER_HOLDER);
        assert_eq!(
            calls_to(&e, HOLDER_DRAW_00E6D310),
            vec![
                vec![holder, partitions + 1, partitions, data],
                vec![holder, p1 + 1, p1, data],
            ]
        );
        let expected: Vec<u32> = [0x78, 0x7c, 0x80, 0x84, 0x6c, 0x6c, 0x8c]
            .iter()
            .map(|s| SHADER + s)
            .collect();
        assert_eq!(order(&e, &SHADER_SLOTS), expected);
        assert_eq!(e.mem.u16(data + 0xe), 0x1000);
    }

    #[test]
    fn begin_pass_sets_up_by_type() {
        let mut e = engine();
        let shader = object(
            &mut e,
            SHADER,
            &[0xf0, 0xf4, 0xf8, 0xfc, 0x100, 0x104, 0x114],
        );
        quiet(&mut e, SHADER_BEGIN_00B7DD50);
        quiet(&mut e, SET_Z_ENABLE);
        e.mem.set_u32(PASS_WORDS_0126F680 + 0x3c, 7);
        e.call(0x00b9_9390, &[0x33, shader]);
        let slots = [0xf0, 0xf4, 0xf8, 0xfc, 0x100, 0x104].map(|s| SHADER + s);
        let expected: Vec<u32> = [0xf0, 0xfc, 0xf8, 0x100, 0xf4]
            .iter()
            .map(|s| SHADER + s)
            .collect();
        assert_eq!(order(&e, &slots), expected);
        assert_eq!(e.mem.u16(PASS_MARKER_011AE63C), 0xffff);
        assert_eq!(e.mem.u32(PASS_WORDS_0126F680 + 0x3c), 0);
        assert_eq!(e.mem.u32(CURRENT_PASS_TYPE), 0x33);
        assert_eq!(e.mem.u32(CURRENT_SHADER), shader);
        assert_eq!(e.mem.u8(PASS_FLAG_011AD8EC), 1);
        // 0xFC: the current pass ends first (the flag skips +0x114).
        e.call_log = Some(vec![]);
        e.call(0x00b9_9390, &[0xfc, shader]);
        assert_eq!(calls_to(&e, SHADER + 0x104), vec![vec![shader, 0x33]]);
        assert!(calls_to(&e, SHADER + 0x114).is_empty());
        assert_eq!(calls_to(&e, SHADER_BEGIN_00B7DD50), vec![vec![shader]]);
        assert_eq!(calls_to(&e, SET_Z_ENABLE), vec![vec![1, 0]]);
        assert!(calls_to(&e, SHADER + 0xfc).is_empty());
    }

    #[test]
    fn render_pass_begins_once_and_runs_the_tail() {
        let mut e = engine();
        let (pass, geometry) = own_pass(&mut e, 0x40, 1);
        let shader = e.mem.u32(geometry + 0xc0);
        e.mem.set_u32(shader + 0x68, 3);
        quiet(&mut e, SHADER_TAIL_00B7AC20);
        quiet(&mut e, SET_ALPHA_TO_COVERAGE);
        e.mem.set_u8(COVERAGE_AVAILABLE, 1);
        let state = e.mem.alloc(0x1a);
        e.mem.set_u16(state + 0x18, 0x200);
        e.mem.set_u32(geometry + 0x9c, state);
        // Another type: nothing.
        e.call(0x00b9_94f0, &[pass, 0x41, 1, 1, 1]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x00b9_94f0, &[pass, 0x40, 1, 1, 1]);
        assert_eq!(e.mem.u32(RENDERING_PASS), pass);
        assert_eq!(e.mem.u32(RENDERING_PASS_TYPE), 0x40);
        assert_eq!(calls_to(&e, SHADER + 0xf0), vec![vec![shader, 0x40]]);
        assert_eq!(calls_to(&e, SET_ALPHA_TO_COVERAGE), vec![vec![1, 0]]);
        assert_eq!(calls_to(&e, GEOMETRY + 0xdc), vec![vec![geometry, 0]]);
        assert_eq!(calls_to(&e, SHADER_TAIL_00B7AC20), vec![vec![shader, 0x40]]);
        // Same type and shader: no new begin, no tail.
        e.call(0x00b9_94f0, &[pass, 0x40, 1, 1, 1]);
        assert_eq!(calls_to(&e, SHADER + 0xf0).len(), 1);
        assert_eq!(calls_to(&e, SHADER_TAIL_00B7AC20).len(), 1);
        assert_eq!(calls_to(&e, GEOMETRY + 0xdc).len(), 2);
    }

    /// A `BSTPersistentList` element holding `items` in this order.
    fn persistent_list(e: &mut Engine, items: &[u32]) -> (u32, Vec<u32>) {
        let element = e.mem.alloc(0x14);
        let mut nodes = vec![];
        let mut previous = 0;
        for &item in items {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node + 8, item);
            e.mem.set_u32(node + 4, previous);
            if previous == 0 {
                e.mem.set_u32(element + 4, node);
            } else {
                e.mem.set_u32(previous, node);
            }
            previous = node;
            nodes.push(node);
        }
        e.mem.set_u32(element + 8, previous);
        e.mem.set_u32(element + 0x10, items.len() as u32);
        (element, nodes)
    }

    fn items_of(e: &Engine, element: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut previous = 0;
        let mut node = e.mem.u32(element + 4);
        while node != 0 {
            assert_eq!(e.mem.u32(node + 4), previous);
            out.push(e.mem.u32(node + 8));
            previous = node;
            node = e.mem.u32(node);
        }
        assert_eq!(e.mem.u32(element + 8), previous);
        out
    }

    const COMPARE: u32 = 0x0066_0c00;

    #[test]
    fn merge_sort_orders_the_list() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        e.register(COMPARE, |e, a| {
            (e.mem.i32(a[0]) - e.mem.i32(a[1])).into_ret()
        });
        let (element, _) = persistent_list(&mut e, &[5, 1, 4, 1, 3]);
        e.call(0x00b9_9630, &[element, COMPARE]);
        assert_eq!(items_of(&e, element), vec![1, 1, 3, 4, 5]);
        assert_eq!(calls_to(&e, PERSISTENT_LIST_CLEAR), vec![vec![element]]);
        // An empty list only gets cleaned.
        let element = e.mem.alloc(0x14);
        e.call(0x00b9_9630, &[element, COMPARE]);
        assert_eq!(e.mem.u32(element + 8), 0);
    }

    #[test]
    fn short_list_head_drop() {
        let mut e = engine();
        quiet(&mut e, ACTIVE_LIST_CLEAR);
        quiet(&mut e, MEMORY_DEALLOCATE);
        let head = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u16(head, 5);
        e.mem.set_u32(head + 4, node);
        e.mem.set_u16(node, 7);
        e.call(0x00b9_9730, &[head]);
        assert_eq!(e.mem.u16(head), 7);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(calls_to(&e, ACTIVE_LIST_CLEAR), vec![vec![node]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, node]]
        );
        e.call(0x00b9_9730, &[head]);
        assert_eq!(e.mem.u16(head), 0);
    }

    fn shorts(e: &Engine, head: u32) -> Vec<u16> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 {
            out.push(e.mem.u16(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn short_list_ordered_insert() {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(COMPARE, |_, a| (a[0] as i32 - a[1] as i32).into_ret());
        let head = e.mem.alloc(8);
        for value in [5, 3, 9, 4] {
            e.call(0x00b9_9770, &[head, value, COMPARE]);
        }
        assert_eq!(shorts(&e, head), vec![3, 4, 5, 9]);
        // The compare gets the whole word: 0x1_0002 sorts last.
        e.call(0x00b9_9770, &[head, 0x1_0002, COMPARE]);
        assert_eq!(shorts(&e, head), vec![3, 4, 5, 9, 2]);
        // A zero item: nothing.
        e.call(0x00b9_9770, &[head, 0x1_0000, COMPARE]);
        assert_eq!(shorts(&e, head).len(), 5);
    }

    #[test]
    fn persistent_list_append_reuses_free_nodes() {
        let mut e = engine();
        e.register(LIST_NODE_ALLOCATE, |e, _| e.mem.alloc(0xc).into_ret());
        let element = e.mem.alloc(0x14);
        let item = e.mem.alloc(4);
        for value in [0x11, 0x22] {
            e.mem.set_u32(item, value);
            e.call(0x00b9_9840, &[element, item]);
        }
        assert_eq!(items_of(&e, element), vec![0x11, 0x22]);
        assert_eq!(e.mem.u32(element + 0x10), 2);
        let free = e.mem.alloc(0xc);
        e.mem.set_u32(element + 0xc, free);
        e.mem.set_u32(item, 0x33);
        e.call(0x00b9_9840, &[element, item]);
        assert_eq!(e.mem.u32(element + 8), free);
        assert_eq!(e.mem.u32(element + 0xc), 0);
        assert_eq!(calls_to(&e, LIST_NODE_ALLOCATE).len(), 2);
    }

    #[test]
    fn next_pass_type_within_the_bounds() {
        let mut e = engine();
        quiet(&mut e, ACTIVE_LIST_CLEAR);
        quiet(&mut e, MEMORY_DEALLOCATE);
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 1);
            1u32.into_ret()
        });
        let this = renderer_with_groups(&mut e, 2);
        let groups = e.mem.u32(this + 8);
        e.mem.set_u32(groups + 0x68 + 0x14 + 4, 0x99);
        e.mem.set_u32(this + 0x20, 5);
        e.mem.set_u32(this + 0x24, 8);
        let out = e.mem.alloc(4);
        let first = e.mem.alloc(4);
        // Empty list.
        assert!(!e.call(0x00b9_9900, &[this, out, first]).bool());
        let node = e.mem.alloc(8);
        e.mem.set_u16(this + 0x2c, 3);
        e.mem.set_u32(this + 0x30, node);
        e.mem.set_u16(node, 6);
        assert!(e.call(0x00b9_9900, &[this, out, first]).bool());
        assert_eq!(e.mem.u32(out), 6);
        assert_eq!(e.mem.u32(first), 1);
        let lookups = calls_to(&e, MAP_GET_AT);
        assert_eq!(lookups.len(), 1);
        assert_eq!(&lookups[0][..2], &[this + 0xc, 6]);
        // The head was dropped: the list now holds 6 only.
        assert_eq!(e.mem.u16(this + 0x2c), 6);
        // Above the upper bound: false.
        e.mem.set_u32(this + 0x24, 5);
        assert!(!e.call(0x00b9_9900, &[this, out, first]).bool());
    }

    #[test]
    fn element_render_walks_and_resets() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        let (pass, _) = own_pass(&mut e, 0x40, 0);
        let empty = e.mem.alloc(8);
        let (element, nodes) = persistent_list(&mut e, &[pass, empty, pass]);
        e.call(0x00b9_9990, &[element, 7]);
        // The pass without geometry is skipped.
        assert_eq!(calls_to(&e, GEOMETRY + 0xdc).len(), 2);
        assert_eq!(e.mem.u32(element + 0xc), nodes[0]);
        for offset in [4, 8, 0x10] {
            assert_eq!(e.mem.u32(element + offset), 0);
        }
        assert_eq!(e.mem.u32(CURRENT_SHADER), 0);
        // Nothing counted: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00b9_9990, &[element, 7]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn alpha_sort_sorts_both_groups() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        let this = e.mem.alloc(0x98);
        let first = e.mem.alloc(0x24);
        let second = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x70, first);
        e.mem.set_u32(this + 0x74, second);
        e.call(0x00b9_9a10, &[this, 0x1234, 1]);
        assert_eq!(e.mem.u32(SORT_CAMERA), 0x1234);
        assert_eq!(e.mem.u8(SORT_SUBTRACT_RADIUS), 1);
        assert_eq!(
            calls_to(&e, PERSISTENT_LIST_CLEAR),
            vec![vec![first + 0xc], vec![second + 0xc]]
        );
    }

    #[test]
    fn pass_group_clear_resets_counted_elements() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        let group = e.mem.alloc(0x68);
        e.mem.set_u32(group + 0x14 + 4, 0x40);
        e.mem.set_u32(group + 0x14 + 0x10, 2);
        e.mem.set_u32(group + 0x28 + 4, 0x50);
        e.mem.set_u32(group + 0x64, 9);
        e.call(0x00b9_9a50, &[group, 0]);
        assert_eq!(
            calls_to(&e, PERSISTENT_LIST_CLEAR),
            vec![vec![group + 0x14]]
        );
        assert_eq!(e.mem.u32(group + 0x14 + 0xc), 0x40);
        assert_eq!(e.mem.u32(group + 0x14 + 4), 0);
        assert_eq!(e.mem.u32(group + 0x28 + 4), 0x50);
        assert_eq!(e.mem.u32(group + 0x64), 0);
    }

    #[test]
    fn pass_group_prepend_and_append() {
        let mut e = engine();
        e.register(LIST_NODE_ALLOCATE, |e, _| e.mem.alloc(0xc).into_ret());
        let group = e.mem.alloc(0x68);
        let element = group + 3 * 0x14;
        e.call(0x00b9_9a90, &[group, 3, 0x22]);
        e.call(0x00b9_9a90, &[group, 3, 0x11]);
        e.call(0x00b9_9b00, &[group, 3, 0x33]);
        assert_eq!(e.mem.u32(element + 0x10), 3);
        assert_eq!(e.mem.u32(group + 0x64), 3);
        let mut items = vec![];
        let mut node = e.mem.u32(element + 4);
        while node != 0 {
            items.push(e.mem.u32(node + 8));
            node = e.mem.u32(node);
        }
        assert_eq!(items, vec![0x11, 0x22, 0x33]);
        // The prepend links the old head back.
        let head = e.mem.u32(element + 4);
        assert_eq!(e.mem.u32(e.mem.u32(head) + 4), head);
        // An append to an empty element sets the head too.
        e.call(0x00b9_9b00, &[group, 0, 0x44]);
        assert_eq!(e.mem.u32(group + 4), e.mem.u32(group + 8));
    }

    #[test]
    fn register_pass_picks_the_list_and_the_group() {
        let mut e = engine();
        e.register(LIST_NODE_ALLOCATE, |e, _| e.mem.alloc(0xc).into_ret());
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        quiet(&mut e, MAP_SET_AT);
        e.register(MAP_GET_AT, |_, _| 0u32.into_ret());
        let this = renderer_with_groups(&mut e, 2);
        e.mem.set_u32(this + 0x18, 1);
        let geometry = e.mem.alloc(0xc4);
        let properties = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0xa8, properties);
        let pass = e.mem.alloc(8);
        e.mem.set_u32(pass, geometry);
        e.mem.set_u16(pass + 4, 0x50);
        e.mem.set_u8(pass + 7, 1);
        e.call(0x00b9_9c90, &[this, pass, 0]);
        assert_eq!(calls_to(&e, MAP_SET_AT), vec![vec![this + 0xc, 0x50, 1]]);
        let groups = e.mem.u32(this + 8);
        let group = groups + 0x68;
        assert_eq!(e.mem.u32(e.mem.u32(group + 4) + 8), pass);
        assert_eq!(e.mem.u16(this + 0x2c), 0x50);
        // An alpha type goes to list 3 of the group the map names (0); bit
        // 22 of +0x24 to list 4, appended for the types 0x238..0x23E.
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 0);
            1u32.into_ret()
        });
        e.call(0x00b9_9c90, &[this, pass, 5]);
        assert_eq!(e.mem.u32(e.mem.u32(groups + 3 * 0x14 + 4) + 8), pass);
        e.mem.set_u32(properties + 0x24, 0x40_0000);
        e.mem.set_u16(pass + 4, 0x23a);
        e.call(0x00b9_9c90, &[this, pass, 0]);
        let element = groups + 4 * 0x14;
        assert_eq!(e.mem.u32(e.mem.u32(element + 8) + 8), pass);
        assert_eq!(calls_to(&e, MAP_SET_AT).len(), 1);
        // No pass, or a pass without geometry: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00b9_9c90, &[this, 0, 0]);
        let bare = e.mem.alloc(8);
        e.call(0x00b9_9c90, &[this, bare, 0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    /// A renderer whose pass group 0 element `index` holds `passes`.
    fn renderer_with_passes(e: &mut Engine, index: u32, passes: &[u32]) -> (u32, u32) {
        let this = renderer_with_groups(e, 1);
        let groups = e.mem.u32(this + 8);
        let (list, _) = persistent_list(e, passes);
        let element = groups + index * 0x14;
        for offset in (0..0x14).step_by(4) {
            let word = e.mem.u32(list + offset);
            e.mem.set_u32(element + offset, word);
        }
        // The first node's previous link pointed at nothing; keep it so.
        e.mem.set_u32(groups + 0x64, passes.len() as u32);
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 0);
            1u32.into_ret()
        });
        (this, element)
    }

    #[test]
    fn next_list_reports_its_first_flag() {
        let mut e = engine();
        let pass = e.mem.alloc(8);
        e.mem.set_u8(pass + 7, 1);
        let (this, _) = renderer_with_passes(&mut e, 2, &[pass]);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, 0x40);
        let index = e.mem.alloc(4);
        let flag = e.mem.alloc(4);
        assert!(e.call(0x00b9_a090, &[this, key, index, flag]).bool());
        assert_eq!(e.mem.u32(index), 2);
        assert_eq!(e.mem.u8(flag), 1);
        // No key and an empty active list: false.
        e.mem.set_u32(key, 0);
        assert!(!e.call(0x00b9_a090, &[this, key, index, flag]).bool());
    }

    #[test]
    fn element_passes_render_until_the_flag_changes() {
        let mut e = engine();
        let renderer = e.mem.alloc(0x8bc);
        e.mem.set_u32(RENDERER, renderer);
        for addr in [0x00e8_79b0, SET_ALPHA_TEST_ENABLE, SET_ALPHA_TO_COVERAGE] {
            quiet(&mut e, addr);
        }
        let (a, _) = own_pass(&mut e, 0x40, 1);
        let (b, _) = own_pass(&mut e, 0x40, 1);
        let (c, _) = own_pass(&mut e, 0x40, 0);
        let (this, element) = renderer_with_passes(&mut e, 2, &[a, b, c]);
        e.mem.set_u32(LOCK_COUNT_4, 2);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, 0x40);
        let index = e.mem.alloc(4);
        e.mem.set_u32(index, 2);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 9);
        assert!(e.call(0x00b9_a120, &[this, key, index, flag, 1]).bool());
        assert_eq!(calls_to(&e, GEOMETRY + 0xdc).len(), 2);
        assert_eq!(e.mem.u8(flag), 0);
        assert_eq!(e.mem.u32(element + 0x10), 1);
        assert_eq!(e.mem.u32(e.mem.u32(element + 4) + 8), c);
        let groups = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(groups + 0x64), 1);
        assert_eq!(calls_to(&e, 0x00e8_79b0), vec![vec![0, 3, 0]]);
        assert_eq!(calls_to(&e, SET_ALPHA_TEST_ENABLE), vec![vec![1, 1]]);
        assert_eq!(calls_to(&e, SET_ALPHA_TO_COVERAGE), vec![vec![0, 0]]);
        assert_eq!(e.mem.u32(LOCK_COUNT_4), 1);
        assert_eq!(e.mem.u32(LOCK_COUNT_011FFA24), 0);
    }

    fn geometry_group(e: &mut Engine) -> u32 {
        let group = e.mem.alloc(0x24);
        e.mem.set_u32(group + 0xc, PERSISTENT_LIST_VTABLE);
        group
    }

    #[test]
    fn geometry_group_destructor_deletes_its_renderer() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        let group = geometry_group(&mut e);
        let renderer = object(&mut e, 0x0069_0000, &[0]);
        e.mem.set_u32(group + 8, renderer);
        e.mem.set_u32(group + 0x10, 0x77);
        e.call(0x00b9_a410, &[group]);
        assert_eq!(calls_to(&e, 0x0069_0000), vec![vec![renderer, 1]]);
        assert_eq!(e.mem.u32(group + 8), 0);
        assert_eq!(calls_to(&e, PERSISTENT_LIST_CLEAR).len(), 2);
        assert_eq!(e.mem.u32(group + 0x18), 0x77);
        assert_eq!(e.mem.u32(group + 0x10), 0);
    }

    #[test]
    fn geometry_group_reset_clears_its_renderer() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        quiet(&mut e, ACTIVE_LIST_CLEAR);
        let group = geometry_group(&mut e);
        e.mem.set_u16(group + 2, 4);
        e.call(0x00b9_a490, &[group]);
        assert_eq!(e.mem.u16(group + 2), 0);
        assert!(calls_to(&e, ACTIVE_LIST_CLEAR).is_empty());
        let renderer = e.mem.alloc(0x98);
        e.mem.set_u32(group + 8, renderer);
        e.call(0x00b9_a490, &[group]);
        assert_eq!(calls_to(&e, ACTIVE_LIST_CLEAR), vec![vec![renderer + 0x2c]]);
    }

    #[test]
    fn destructor_releases_every_member() {
        let mut e = engine();
        for addr in [
            PERSISTENT_LIST_CLEAR,
            ACTIVE_LIST_CLEAR,
            VECTOR_DESTRUCTOR_ITERATOR,
            MEMORY_DEALLOCATE,
            INSTANCE_RENDERER_DESTRUCTOR,
            MAP_DESTRUCTOR,
            OBJECT_DESTRUCTOR,
        ] {
            quiet(&mut e, addr);
        }
        let this = e.mem.alloc(0x98);
        let block = e.mem.alloc(4 + 2 * 0x68);
        e.mem.set_u32(block, 2);
        e.mem.set_u32(this + 8, block + 4);
        e.mem.set_u32(this + 0x28, 0x2828);
        let group = geometry_group(&mut e);
        e.mem.set_u32(this + 0x3c, group);
        e.call(0x00b9_aa80, &[this]);
        assert_eq!(e.mem.u32(this), BATCH_RENDERER_VTABLE);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![
                vec![block + 4, 0x68, 2, PASS_GROUP_DESTRUCTOR],
                vec![this + 0x78, 0xc, 2, ALPHA_LIST_DESTRUCTOR],
            ]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, block],
                vec![MEMORY_MANAGER, 0x2828],
                vec![MEMORY_MANAGER, group],
            ]
        );
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x3c), group);
        assert_eq!(
            calls_to(&e, INSTANCE_RENDERER_DESTRUCTOR),
            vec![vec![0x2828]]
        );
        assert_eq!(calls_to(&e, ACTIVE_LIST_CLEAR).len(), 2);
        assert_eq!(calls_to(&e, MAP_DESTRUCTOR), vec![vec![this + 0xc]]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn geometry_group_add_by_its_kind() {
        let mut e = engine();
        e.register(LIST_NODE_ALLOCATE, |e, _| e.mem.alloc(0xc).into_ret());
        quiet(&mut e, PERSISTENT_LIST_PREPEND);
        let group = geometry_group(&mut e);
        e.mem.set_u8(group, 1);
        e.call(0x00b9_abc0, &[group, 0x5555, 0]);
        assert_eq!(e.mem.u32(e.mem.u32(group + 0x10) + 8), 0x5555);
        e.call(0x00b9_abc0, &[group, 0x6666, 1]);
        let prepends = calls_to(&e, PERSISTENT_LIST_PREPEND);
        assert_eq!(prepends.len(), 1);
        assert_eq!(prepends[0][0], group + 0xc);
        assert_eq!(e.mem.u16(group + 2), 2);
        // Bit 0 clear: the group's own renderer's slot +0x8C.
        let pass = e.mem.alloc(8);
        e.mem.set_u16(pass + 4, 0x31);
        let renderer = object(&mut e, 0x0069_0000, &[0x8c]);
        let group = geometry_group(&mut e);
        e.mem.set_u32(group + 8, renderer);
        e.call(0x00b9_abc0, &[group, pass, 0]);
        assert_eq!(calls_to(&e, 0x0069_008c), vec![vec![renderer, pass, 0x31]]);
        assert_eq!(e.mem.u16(group + 2), 1);
        // No renderer yet and a failed allocation: the null renderer is
        // called through address 0, as in the exe.
        e.map(0, 0x1000);
        quiet(&mut e, 0);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        let group = geometry_group(&mut e);
        e.call(0x00b9_abc0, &[group, pass, 0]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x98]]);
        assert_eq!(calls_to(&e, 0), vec![vec![0, pass, 0x31]]);
    }

    fn grouping_engine() -> (Engine, u32) {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(LIST_NODE_ALLOCATE, |e, _| e.mem.alloc(0xc).into_ret());
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x98);
        (e, this)
    }

    #[test]
    fn start_grouping_alphas_makes_two_groups() {
        let (mut e, this) = grouping_engine();
        let cam = camera(&mut e, [1.0, 0.0, 0.0]);
        let point = bound(&mut e, [5.0, 0.0, 0.0, 1.0]);
        // Without a camera only the level counts.
        e.call(0x00b9_ac80, &[this, point, 0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0x34), 1);
        e.mem.set_u32(this + 0x34, 0);
        e.call(0x00b9_ac80, &[this, point, 0, cam, 1]);
        let first = e.mem.u32(this + 0x90);
        let second = e.mem.u32(this + 0x94);
        for (group, list) in [(first, 0x78), (second, 0x84)] {
            assert_eq!(e.mem.u32(group + 4), 0x2f7);
            assert_eq!(e.mem.f32(group + 0x20), 4.0);
            assert_eq!(e.mem.u8(group) & 1, 1);
            let head = e.mem.u32(this + list);
            assert_eq!(e.mem.u32(head + 8), group);
            assert_eq!(e.mem.u32(this + list + 4), head);
            assert_eq!(e.mem.u32(this + list + 8), 1);
        }
        assert_eq!(e.mem.u32(this + 0x34), 1);
        assert_eq!(e.mem.u8(this + 0x38), 0);
        // A farther group goes before a nearer one (a pool node); `keep`
        // clears bit 0.
        let (mut e, this) = grouping_engine();
        let pool = e.mem.alloc(0xc);
        e.mem.set_u32(NODE_POOL_FREE_LIST, pool);
        let near = e.mem.alloc(0x24);
        e.mem.set_f32(near + 0x20, 2.0);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, near);
        e.mem.set_u32(this + 0x78, node);
        e.mem.set_u32(this + 0x7c, node);
        e.mem.set_u32(this + 0x80, 1);
        let cam = camera(&mut e, [1.0, 0.0, 0.0]);
        let point = bound(&mut e, [5.0, 0.0, 0.0, 1.0]);
        e.call(0x00b9_ac80, &[this, point, 1, cam, 0]);
        let first = e.mem.u32(this + 0x90);
        assert_eq!(e.mem.f32(first + 0x20), 5.0);
        assert_eq!(e.mem.u8(first) & 1, 0);
        assert_eq!(e.mem.u32(this + 0x78), pool);
        assert_eq!(e.mem.u32(pool + 8), first);
        assert_eq!(e.mem.u32(pool), node);
        assert_eq!(e.mem.u32(node + 4), pool);
        assert_eq!(e.mem.u32(this + 0x80), 2);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_eq!(e.mem.u8(this + 0x38), 1);
    }

    #[test]
    fn stop_grouping_alphas_drops_empty_groups_at_level_zero() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        quiet(&mut e, ALPHA_LIST_REMOVE);
        quiet(&mut e, MEMORY_DEALLOCATE);
        let this = e.mem.alloc(0x98);
        let empty = geometry_group(&mut e);
        let used = geometry_group(&mut e);
        e.mem.set_u16(used + 2, 1);
        e.mem.set_u32(this + 0x90, empty);
        e.mem.set_u32(this + 0x94, used);
        e.mem.set_u8(this + 0x38, 1);
        e.mem.set_u32(this + 0x34, 2);
        e.call(0x00b9_aec0, &[this]);
        assert_eq!(e.mem.u32(this + 0x34), 1);
        assert_eq!(e.mem.u32(this + 0x90), empty);
        e.call(0x00b9_aec0, &[this]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(
            calls_to(&e, ALPHA_LIST_REMOVE),
            vec![vec![this + 0x78, this + 0x90]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, empty]]
        );
        assert_eq!(e.mem.u32(this + 0x90), 0);
        assert_eq!(e.mem.u32(this + 0x94), 0);
        assert_eq!(e.mem.u8(this + 0x38), 0);
    }

    #[test]
    fn render_next_alpha_renders_back_to_front() {
        let mut e = engine();
        quiet(&mut e, PERSISTENT_LIST_CLEAR);
        quiet(&mut e, MEMORY_DEALLOCATE);
        e.register(ISNAN, |_, _| 0u32.into_ret());
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            Ret::default()
        });
        e.mem.set_f64(LIMIT_STEP, 0.001);
        let this = e.mem.alloc(0x98);
        // The geometry group holds one pass at distance 5.
        let (pass, geometry) = own_pass(&mut e, 0x40, 1);
        let b = bound(&mut e, [5.0, 0.0, 0.0, 0.0]);
        e.mem.set_u32(geometry + 0x20, b);
        let group = geometry_group(&mut e);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, pass);
        e.mem.set_u32(group + 0x10, node);
        e.mem.set_u32(group + 0x14, node);
        e.mem.set_u32(group + 0x1c, 1);
        e.mem.set_u16(group + 2, 1);
        e.mem.set_u32(this + 0x70, group);
        // The alpha-group list holds one empty group at distance 3.
        let alpha = geometry_group(&mut e);
        e.mem.set_u8(alpha, 1);
        e.mem.set_f32(alpha + 0x20, 3.0);
        let alpha_node = e.mem.alloc(0xc);
        e.mem.set_u32(alpha_node + 8, alpha);
        e.mem.set_u32(this + 0x78, alpha_node);
        e.mem.set_u32(this + 0x7c, alpha_node);
        e.mem.set_u32(this + 0x80, 1);
        let cam = camera(&mut e, [1.0, 0.0, 0.0]);
        e.call(0x00b9_af60, &args![this, 0u32, cam, 0u8, 1.0f32]);
        assert_eq!(calls_to(&e, GEOMETRY + 0xdc).len(), 1);
        assert_eq!(e.mem.u32(group + 0x10), 0);
        assert_eq!(e.mem.u32(group + 0x18), node);
        assert_eq!(e.mem.u16(group + 2), 0);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, alpha]]
        );
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), alpha_node);
        assert_eq!(calls_to(&e, ISNAN).len(), 1);
        // A limit above both distances renders nothing.
        let mut e = engine();
        e.mem.set_f64(LIMIT_STEP, 0.001);
        let this = e.mem.alloc(0x98);
        let group = geometry_group(&mut e);
        e.mem.set_u32(this + 0x70, group);
        let cam = camera(&mut e, [1.0, 0.0, 0.0]);
        e.call(0x00b9_af60, &args![this, 0u32, cam, 0u8, 10.0f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0xc000);
        e.call_log = Some(vec![]);
        e
    }

    fn quiet(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A `BSTPersistentList` element (0x14 bytes) with the given head and tail.
    fn element(e: &mut Engine, head: u32, tail: u32, count: u32) -> u32 {
        let element = e.mem.alloc(0x14);
        e.mem.set_u32(element + 4, head);
        e.mem.set_u32(element + 8, tail);
        e.mem.set_u32(element + 0x10, count);
        element
    }

    #[test]
    fn prepend_links_a_free_node_before_the_head() {
        let mut e = engine();
        let old = e.mem.alloc(0xc);
        let free = e.mem.alloc(0xc);
        let spare = e.mem.alloc(0xc);
        e.mem.set_u32(free, spare);
        let list = element(&mut e, old, old, 1);
        e.mem.set_u32(list + 0xc, free);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x0e0a_0001);
        e.call(0x00b9_9890, &[list, item]);
        assert_eq!(e.mem.u32(list + 0xc), spare);
        assert_eq!(e.mem.u32(list + 4), free);
        assert_eq!(e.mem.u32(list + 8), old);
        assert_eq!(e.mem.u32(list + 0x10), 2);
        assert_eq!(e.mem.u32(free), old);
        assert_eq!(e.mem.u32(free + 4), 0);
        assert_eq!(e.mem.u32(free + 8), 0x0e0a_0001);
        assert_eq!(e.mem.u32(old + 4), free);
    }

    #[test]
    fn prepend_to_an_empty_list_allocates_and_sets_the_tail() {
        let mut e = engine();
        let node = e.mem.alloc(0xc);
        e.register_double(LIST_NODE_ALLOCATE, move |_, _| word(node));
        let list = element(&mut e, 0, 0, 0);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x0e0a_0002);
        e.call(0x00b9_9890, &[list, item]);
        assert_eq!(calls_to(&e, LIST_NODE_ALLOCATE), vec![vec![list + 0x10]]);
        assert_eq!(e.mem.u32(list + 4), node);
        assert_eq!(e.mem.u32(list + 8), node);
        assert_eq!(e.mem.u32(list + 0x10), 1);
        assert_eq!(e.mem.u32(node), 0);
        assert_eq!(e.mem.u32(node + 8), 0x0e0a_0002);
    }

    #[test]
    fn persistent_list_deleting_destructor_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (0x100, false), (1, true)] {
            let mut e = engine();
            quiet(&mut e, PERSISTENT_LIST_CLEAR);
            quiet(&mut e, MEMORY_DEALLOCATE);
            let head = e.mem.alloc(0xc);
            let list = element(&mut e, head, head, 1);
            assert_eq!(e.call(0x00b9_9b70, &[list, flags]).u32(), list);
            assert_eq!(e.mem.u32(list), PERSISTENT_LIST_VTABLE);
            assert_eq!(calls_to(&e, PERSISTENT_LIST_CLEAR), vec![vec![list]; 2]);
            assert_eq!(e.mem.u32(list + 0xc), head);
            for offset in [4, 8, 0x10] {
                assert_eq!(e.mem.u32(list + offset), 0);
            }
            let freed = calls_to(&e, MEMORY_DEALLOCATE);
            if frees {
                assert_eq!(freed, vec![vec![MEMORY_MANAGER, list]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }

    /// A renderer with two pass groups and an active list of the pass types `types`; the
    /// render-pass map answers group `type >> 4` minus one.
    fn batches(e: &mut Engine, types: &[u16]) -> (Ptr<BSBatchRenderer>, u32) {
        let this: Ptr<BSBatchRenderer> = e.new_object();
        let groups = e.mem.alloc(0x68 * 2);
        e.set(this, BSBatchRenderer::pRenderPassA, groups);
        let mut node = this.addr() + 0x2c;
        for (i, &value) in types.iter().enumerate() {
            e.mem.set_u16(node, value);
            if i + 1 < types.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], (a[1] >> 4).wrapping_sub(1));
            Ret::default()
        });
        quiet(e, PERSISTENT_LIST_CLEAR);
        quiet(e, SET_ALPHA_TEST_ENABLE);
        quiet(e, SET_ALPHA_TO_COVERAGE);
        quiet(e, 0x00e8_79b0);
        // `00b984d0`'s render-state object.
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x8b8, 0x0e0c_0000);
        e.mem.set_u32(RENDERER, renderer);
        quiet(e, RENDERER_IS_SKINNED);
        (this, groups)
    }

    /// A pass of type `pass_type` whose geometry renders itself (slot +0x30 answers 1,
    /// slot +0xDC records the call).
    fn own_pass(e: &mut Engine, pass_type: u16) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x30, 0x00ee_0030);
        e.mem.set_u32(vtable + 0xdc, 0x00ee_00dc);
        e.register(0x00ee_0030, |_, _| word(1));
        quiet(e, 0x00ee_00dc);
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(geometry, vtable);
        let pass = e.mem.alloc(8);
        e.mem.set_u32(pass, geometry);
        e.mem.set_u16(pass + 4, pass_type);
        pass
    }

    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node, next);
        e.mem.set_u32(node + 8, item);
        node
    }

    #[test]
    fn render_batches_renders_each_list_with_its_state_and_skips_shadow_passes() {
        let mut e = engine();
        e.register(IS_SHADOW_PASS, |_, a| word(u32::from(a[0] == 0x20)));
        let accumulator = e.mem.alloc(0x200);
        e.mem.set_u8(accumulator + 0x165, 1);
        e.register_double(GET_ACCUMULATOR, move |_, _| word(accumulator));
        // 0x10 renders (group 0), 0x20 is a shadow pass (group 1), 0x400 is out of range.
        let (this, groups) = batches(&mut e, &[0x10, 0x20, 0x400]);
        e.mem.set_u32(CURRENT_PASS_TYPE, 0x10);
        let first = own_pass(&mut e, 0x10);
        let second = own_pass(&mut e, 0x10);
        let third = own_pass(&mut e, 0x10);
        let tail = list_node(&mut e, second, 0);
        let head = list_node(&mut e, first, tail);
        e.mem.set_u32(groups + 4, head);
        e.mem.set_u32(groups + 0x10, 2);
        let only = list_node(&mut e, third, 0);
        e.mem.set_u32(groups + 2 * 0x14 + 4, only);
        e.mem.set_u32(groups + 2 * 0x14 + 0x10, 1);
        e.mem.set_u32(groups + 0x68 + 0x64, 9);
        e.mem.set_u32(groups + 0x64, 9);
        e.mem.set_u32(LOCK_COUNT_011FFA24, 1);
        e.mem.set_u32(LOCK_COUNT_4, 0);
        e.call(0x00b9_9e20, &[this.addr(), 0, 7, 0x55]);

        let lookups: Vec<Vec<u32>> = calls_to(&e, MAP_GET_AT)
            .iter()
            .map(|c| c[..2].to_vec())
            .collect();
        assert_eq!(
            lookups,
            vec![vec![this.addr() + 0xc, 0x10], vec![this.addr() + 0xc, 0x20]]
        );
        // List 0 at state 1, list 2 at state 3 with the alpha test, then state 1 at the end.
        let states: Vec<u32> = calls_to(&e, 0x00e8_79b0).iter().map(|c| c[1]).collect();
        assert_eq!(states, vec![1, 3, 1]);
        assert_eq!(
            calls_to(&e, SET_ALPHA_TEST_ENABLE),
            vec![vec![0, 0], vec![1, 1], vec![0, 0]]
        );
        // Coverage switch absent: only the closing call after each list.
        assert_eq!(calls_to(&e, SET_ALPHA_TO_COVERAGE), vec![vec![0, 0]; 2]);
        // Every item rendered through its own slot.
        assert_eq!(calls_to(&e, 0x00ee_00dc).len(), 3);
        assert_eq!(e.mem.u32(LOCK_COUNT_011FFA24), 0);
        assert_eq!(e.mem.u32(LOCK_COUNT_4), 0);
        // Both groups cleared: the rendered one and the skipped shadow pass.
        assert_eq!(e.mem.u32(groups + 0x64), 0);
        assert_eq!(e.mem.u32(groups + 0x68 + 0x64), 0);
        assert_eq!(
            calls_to(&e, PERSISTENT_LIST_CLEAR),
            vec![vec![groups], vec![groups + 0x28]]
        );
        assert_eq!(e.mem.u32(groups + 0xc), head);
    }

    #[test]
    fn render_batches_switches_coverage_and_keeps_the_range() {
        let mut e = engine();
        quiet(&mut e, IS_SHADOW_PASS);
        let (this, groups) = batches(&mut e, &[0x10, 0x20]);
        e.mem.set_u8(COVERAGE_AVAILABLE, 1);
        e.mem.set_u32(COVERAGE_LOCK, 5);
        e.mem.set_u32(LOCK_COUNT_4, 2);
        e.mem.set_u32(CURRENT_PASS_TYPE, 0x20);
        let pass = own_pass(&mut e, 0x20);
        // Group 1, list 3 (state 1, alpha test, coverage on).
        let only = list_node(&mut e, pass, 0);
        e.mem.set_u32(groups + 0x68 + 3 * 0x14 + 4, only);
        // The range 0x18..0x20 leaves out 0x10.
        e.call(0x00b9_9e20, &[this.addr(), 0x18, 0x20, 0]);
        assert_eq!(calls_to(&e, MAP_GET_AT).len(), 1);
        assert_eq!(
            calls_to(&e, SET_ALPHA_TO_COVERAGE),
            vec![vec![1, 1], vec![0, 0], vec![0, 0], vec![0, 0]]
        );
        let states: Vec<u32> = calls_to(&e, 0x00e8_79b0).iter().map(|c| c[1]).collect();
        assert_eq!(states, vec![1, 1]);
        assert_eq!(e.mem.u32(COVERAGE_LOCK), 4);
        assert_eq!(e.mem.u32(LOCK_COUNT_4), 1);
        assert_eq!(calls_to(&e, 0x00ee_00dc).len(), 1);
    }

    #[test]
    fn render_batches_with_an_empty_list_only_resets_the_state() {
        let mut e = engine();
        e.mem.set_u8(BATCH_SETUP_FLAG_011F94A6, 1);
        quiet(&mut e, BATCH_SETUP_00BBB840);
        let (this, _) = batches(&mut e, &[0]);
        e.call(0x00b9_9e20, &[this.addr(), 0, 0, 0]);
        assert_eq!(calls_to(&e, BATCH_SETUP_00BBB840).len(), 1);
        assert!(calls_to(&e, MAP_GET_AT).is_empty());
        assert_eq!(calls_to(&e, SET_ALPHA_TEST_ENABLE), vec![vec![0, 0]]);
        assert_eq!(e.mem.u32(CURRENT_PASS_TYPE), 0);
    }

    #[test]
    fn render_list_clears_the_group_and_finds_the_next_element() {
        let mut e = engine();
        let (this, groups) = batches(&mut e, &[0x20]);
        let item = e.mem.alloc(8);
        e.mem.set_u8(item + 7, 0x5a);
        let node = list_node(&mut e, item, 0);
        e.mem.set_u32(groups + 0x68 + 2 * 0x14 + 4, node);
        e.mem.set_u32(groups + 0x68 + 0x64, 3);
        let words = e.mem.alloc(0xc);
        e.mem.set_u32(words, 0x20);
        let answer = e.call(0x00b9_a3c0, &[this.addr(), words, words + 4, words + 8, 0]);
        assert!(answer.bool());
        assert_eq!(e.mem.u32(groups + 0x68 + 0x64), 0);
        assert_eq!(e.mem.u32(words + 4), 2);
        assert_eq!(e.mem.u8(words + 8), 0x5a);
    }

    #[test]
    fn geometry_group_render_picks_its_own_list_or_the_renderer() {
        // Bit 0 set: its own list (empty here), no renderer.
        let mut e = engine();
        let group = e.mem.alloc(0x24);
        e.mem.set_u8(group, 1);
        e.mem.set_u16(group + 2, 7);
        e.call(0x00b9_a4d0, &[group, 1]);
        assert_eq!(e.mem.u16(group + 2), 0);
        assert!(calls_to(&e, ACTIVE_LIST_CLEAR).is_empty());

        // Bit 0 clear: the renderer's slot +0x90, then its active list cleared.
        let mut e = engine();
        quiet(&mut e, ACTIVE_LIST_CLEAR);
        quiet(&mut e, 0x00ee_0090);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x90, 0x00ee_0090);
        let renderer = e.mem.alloc(0x98);
        e.mem.set_u32(renderer, vtable);
        let group = e.mem.alloc(0x24);
        e.mem.set_u8(group, 2);
        e.mem.set_u16(group + 2, 7);
        e.mem.set_u32(group + 8, renderer);
        e.call(0x00b9_a4d0, &[group, 0x33]);
        assert_eq!(
            calls_to(&e, 0x00ee_0090),
            vec![vec![renderer, 1, 0x2f1, 0x33]]
        );
        assert_eq!(calls_to(&e, ACTIVE_LIST_CLEAR), vec![vec![renderer + 0x2c]]);
        assert_eq!(e.mem.u16(group + 2), 0);
    }

    #[test]
    fn get_rtti_answers_the_class_record() {
        let mut e = engine();
        assert_eq!(e.call(0x00b9_aa70, &[0x0e00_1000]).u32(), 0x011f_fe3c);
    }
}
