//! `BSShader/bsshaderaccumulator.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTPointerList, NiTPointerMap};

layout! {
    /// `BSShaderAccumulator` (Xbox PDB; 0x280 bytes on PC, the size
    /// `Main::Main` allocates; the Xbox build has a second map and critical
    /// section after +0x280). The offsets used below match the PC code.
    pub struct BSShaderAccumulator: 0x280 {
        /// `bDoDepthPasses` (Xbox PDB).
        0x03a bDoDepthPasses: u8,
        /// `uiSunPixelCount` (Xbox PDB).
        0x03c uiSunPixelCount: u32,
        /// `bWaitingForSunQueury` (Xbox PDB).
        0x040 bWaitingForSunQueury: u8,
        /// `fPercentSunOccludedStored` (Xbox PDB), as its bit pattern.
        0x044 fPercentSunOccludedStored: u32,
        /// `bAccumulate` (Xbox PDB).
        0x084 bAccumulate: u8,
        /// `b1stPerson` (Xbox PDB).
        0x085 b1stPerson: u8,
        /// `bRenderNoShader` (Xbox PDB).
        0x086 bRenderNoShader: u8,
        /// `usQueryCount` (Xbox PDB).
        0x088 usQueryCount: u16,
        /// `usQuerySize` (Xbox PDB).
        0x08a usQuerySize: u16,
        /// `ppOcclusionQuery` (Xbox PDB).
        0x08c ppOcclusionQuery: u32,
        /// `pVolumetricFogGeometry` (Xbox PDB), an `NiTPointerList`.
        0x090 pVolumetricFogGeometry: Inline<NiTPointerList>,
        /// `spBoundTestGeometry` (Xbox PDB).
        0x13c spBoundTestGeometry: u32,
        /// `ReferenceVolumeList` (Xbox PDB), an `NiTPointerList`.
        0x140 ReferenceVolumeList: Inline<NiTPointerList>,
        /// `uiLastOcclusionRef` (Xbox PDB).
        0x14c uiLastOcclusionRef: u32,
        /// `bLastOcclusionResult` (Xbox PDB).
        0x150 bLastOcclusionResult: u8,
        /// `kSilhouetteColor.r` (Xbox PDB, `NiColorA`), as its bit pattern.
        0x154 kSilhouetteColor_r: u32,
        /// `kSilhouetteColor.g` (Xbox PDB).
        0x158 kSilhouetteColor_g: u32,
        /// `kSilhouetteColor.b` (Xbox PDB).
        0x15c kSilhouetteColor_b: u32,
        /// `kSilhouetteColor.a` (Xbox PDB).
        0x160 kSilhouetteColor_a: u32,
        /// `bRenderDecals` (Xbox PDB).
        0x16c bRenderDecals: u8,
        /// `pInstanceRenderer` (Xbox PDB).
        0x170 pInstanceRenderer: u32,
        /// `pBatchRenderer[0]` (Xbox PDB).
        0x174 pBatchRenderer0: u32,
        /// `pBatchRenderer[1]` (Xbox PDB).
        0x178 pBatchRenderer1: u32,
        /// `puicurrentPassA` (Xbox PDB).
        0x17c puicurrentPassA: u32,
        /// `picurrentBucketA` (Xbox PDB).
        0x180 picurrentBucketA: u32,
        /// `pbcurrentFirstPassA` (Xbox PDB).
        0x184 pbcurrentFirstPassA: u32,
        /// `pbcurrentActiveA` (Xbox PDB).
        0x188 pbcurrentActiveA: u32,
        /// `uiBatchRendererCount` (Xbox PDB).
        0x18c uiBatchRendererCount: u32,
        /// `uiBatchRendererPassCount` (Xbox PDB).
        0x190 uiBatchRendererPassCount: u32,
        /// `spWaterFogRenderTarget` (Xbox PDB), an `NiPointer`.
        0x198 spWaterFogRenderTarget: u32,
        /// `eRenderMode` (Xbox PDB).
        0x19c eRenderMode: u32,
        /// `spShadowLight` (Xbox PDB), an `NiPointer`.
        0x1a0 spShadowLight: u32,
        /// `kRoomSharedMap` (Xbox PDB), an `NiTPointerMap`.
        0x1b4 kRoomSharedMap: Inline<NiTPointerMap>,
        /// `BSCPSharedCrit` (Xbox PDB): its `CRITICAL_SECTION` comes first.
        0x200 BSCPSharedCrit: u32,
    }
}

/// The base-class constructor `00b660d0` runs first (thiscall, no
/// arguments; unnamed in the engine map).
const BASE_CONSTRUCTOR: u32 = 0x00a9_b480;
/// The class's vtable.
const ACCUMULATOR_VTABLE: u32 = 0x010a_dff8;
/// The compiler's `eh vector constructor iterator` (stdcall: array, element
/// size, count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// Constructor of one `BSTPersistentList` element (0x14 bytes).
const PERSISTENT_LIST_CONSTRUCTOR: u32 = 0x00b9_88c0;
/// Destructor of one `BSTPersistentList` element.
const PERSISTENT_LIST_DESTRUCTOR: u32 = 0x00b6_56b0;
/// `BSShaderProperty::RenderPass::RenderPass` (engine map; cdecl, six words).
const RENDER_PASS_CONSTRUCTOR: u32 = 0x00ba_8ec0;
/// The vtable the inlined `NiTPointerMap` constructor sets first.
const MAP_BASE_VTABLE: u32 = 0x010a_dfbc;
/// The map vtable it sets last.
const MAP_VTABLE: u32 = 0x010a_dfd8;
/// The allocator the inlined map constructor calls for its buckets (cdecl,
/// one word: the size).
const ALLOCATE_BUCKETS: u32 = 0x00aa_1070;
/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// `InitializeCriticalSection` (import slot).
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The allocator used for the batch renderers (cdecl, one word: the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// Constructor of the 4-byte object kept in `pInstanceRenderer` (thiscall,
/// no arguments).
const INSTANCE_RENDERER_CONSTRUCTOR: u32 = 0x00bd_9120;
/// Constructor of a 0x98-byte batch renderer (engine map unit
/// `bsbatchrenderer.cpp`; thiscall, two words).
const BATCH_RENDERER_CONSTRUCTOR: u32 = 0x00b9_a5b0;
/// `NiTPointerListBase<..>::RemoveAll` (engine map; thiscall).
const POINTER_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// The float the three colour channels of `kSilhouetteColor` take.
const SILHOUETTE_CHANNEL: u32 = 0x0101_7870;
/// Word of the thread's TLS block the constructor clears.
const TLS_WORD_2BC: u32 = 0x2bc;
/// `1.0f` as the `FLD1; FSTP dword` stores it.
const ONE_BITS: u32 = 0x3f80_0000;

/// Drops the `NiPointer` held at `slot` (the inlined smart-pointer
/// release): when the count at +4 reaches zero calls the object's virtual
/// method at +4, then clears the slot.
fn release_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        if e.call(IMPORT_INTERLOCKED_DECREMENT, &[object + 4]).u32() == 0 {
            e.vcall(object, 4, &[]);
        }
        e.mem.set_u32(slot, 0);
    }
}

/// `count * 4` as the code computes the array size (`MUL`, `SETO`, `NEG`,
/// `OR`: on overflow the size is `0xFFFFFFFF`).
fn word_array_size(count: u32) -> u32 {
    count.checked_mul(4).unwrap_or(u32::MAX)
}

// Translated from 00b660d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::BSShaderAccumulator` (Xbox PDB): runs the base
/// constructor, sets the vtable, clears the sun tests, constructs the two
/// rendered-geometry caches (4 `BSTPersistentList`s each), the alpha render
/// pass, the room map (37 buckets) and its critical section, allocates the
/// 50 occlusion-query slots, the instance renderer, one batch renderer (two
/// when `batch_renderer_count` is 2) built with `pass_count` and
/// `batch_argument`, and the four per-renderer arrays; drops the shadow
/// light and water fog target (both already cleared earlier in the
/// constructor, so the releases only run if a callee in between set them),
/// clears word +0x2BC of the thread block and
/// sets the silhouette colour to the constant at `01017870` (alpha 1).
/// Returns `this`.
// Translation incomplete: 00b660d0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn bs_shader_accumulator_bs_shader_accumulator(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    batch_argument: u32,
    batch_renderer_count: u32,
    pass_count: u32,
) -> Ptr<BSShaderAccumulator> {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR, &[base]);
    e.mem.set_u32(base, ACCUMULATOR_VTABLE);
    // FLDZ; FST to the three sun tests' floats.
    e.mem.set_f32(base + 0x50, 0.0);
    e.mem.set_f32(base + 0x64, 0.0);
    e.mem.set_f32(base + 0x78, 0.0);
    // pSunTests[0..3] (Xbox PDB `SunOcclusionTest`, 0x14 bytes each).
    for test in [0x48, 0x5c, 0x70] {
        e.mem.set_u32(base + test, 0);
        e.mem.set_u8(base + test + 4, 0);
        e.mem.set_u32(base + test + 0xc, 0);
        e.mem.set_u32(base + test + 0x10, 0);
    }
    e.mem.set_u32(base + 0x80, 0);
    let fog = this.at(BSShaderAccumulator::pVolumetricFogGeometry);
    e.set(fog, NiTPointerList::m_uiCount, 0);
    e.set(fog, NiTPointerList::m_pkHead, 0);
    e.set(fog, NiTPointerList::m_pkTail, 0);
    for cache in [0x9c, 0xec] {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &[
                base + cache,
                0x14,
                4,
                PERSISTENT_LIST_CONSTRUCTOR,
                PERSISTENT_LIST_DESTRUCTOR,
            ],
        );
    }
    e.set(this, BSShaderAccumulator::spBoundTestGeometry, 0);
    let volumes = this.at(BSShaderAccumulator::ReferenceVolumeList);
    e.set(volumes, NiTPointerList::m_uiCount, 0);
    e.set(volumes, NiTPointerList::m_pkHead, 0);
    e.set(volumes, NiTPointerList::m_pkTail, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_r, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_g, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_b, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_a, 0);
    e.set(this, BSShaderAccumulator::spWaterFogRenderTarget, 0);
    e.set(this, BSShaderAccumulator::spShadowLight, 0);
    e.call(RENDER_PASS_CONSTRUCTOR, &[base + 0x1a4, 0, 0x2f5, 1, 0, 0]);
    // The inlined NiTPointerMap constructor (37 buckets).
    let map = this.at(BSShaderAccumulator::kRoomSharedMap);
    e.mem.set_u32(map.addr(), MAP_BASE_VTABLE);
    e.set(map, NiTPointerMap::m_uiHashSize, 0x25);
    e.set(map, NiTPointerMap::m_uiCount, 0);
    let buckets = e.call(ALLOCATE_BUCKETS, &[0x94]).u32();
    let hash_size = e.get(map, NiTPointerMap::m_uiHashSize);
    e.set(map, NiTPointerMap::m_ppkHashTable, buckets);
    e.call(MEMSET, &[buckets, 0, hash_size.wrapping_mul(4)]);
    e.mem.set_u32(map.addr(), MAP_VTABLE);
    e.call(IMPORT_INITIALIZE_CRITICAL_SECTION, &[base + 0x200]);
    e.set(this, BSShaderAccumulator::bAccumulate, 1);
    e.set(this, BSShaderAccumulator::b1stPerson, 0);
    e.set(this, BSShaderAccumulator::bRenderNoShader, 1);
    let queries = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 200]).u32();
    e.set(this, BSShaderAccumulator::ppOcclusionQuery, queries);
    e.set(this, BSShaderAccumulator::usQueryCount, 0);
    e.set(this, BSShaderAccumulator::usQuerySize, 0x32);
    let mut offset = 0u32;
    while offset < 200 {
        let slots = e.get(this, BSShaderAccumulator::ppOcclusionQuery);
        e.mem.set_u32(offset.wrapping_add(slots), 0);
        offset += 4;
    }
    e.call(POINTER_LIST_REMOVE_ALL, &[fog.addr()]);
    e.set(
        this,
        BSShaderAccumulator::fPercentSunOccludedStored,
        ONE_BITS,
    );
    e.set(this, BSShaderAccumulator::bWaitingForSunQueury, 0);
    e.set(this, BSShaderAccumulator::uiSunPixelCount, 0);
    for test in [0x4c, 0x60, 0x74] {
        e.mem.set_f32(base + test + 4, 0.0);
        e.mem.set_u32(base + test - 4, 0);
        e.mem.set_u8(base + test, 0);
        e.mem.set_u32(base + test + 8, 0);
        e.mem.set_u32(base + test + 0xc, 0);
    }
    e.set(this, BSShaderAccumulator::uiLastOcclusionRef, 0);
    e.set(this, BSShaderAccumulator::bLastOcclusionResult, 0);
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 4]).u32();
    let instance = if block != 0 {
        e.call(INSTANCE_RENDERER_CONSTRUCTOR, &[block]).u32()
    } else {
        0
    };
    e.set(this, BSShaderAccumulator::pInstanceRenderer, instance);
    e.set(
        this,
        BSShaderAccumulator::uiBatchRendererCount,
        batch_renderer_count,
    );
    e.set(
        this,
        BSShaderAccumulator::uiBatchRendererPassCount,
        pass_count,
    );
    let block = e.call(OPERATOR_NEW, &[0x98]).u32();
    let first = if block != 0 {
        let passes = e.get(this, BSShaderAccumulator::uiBatchRendererPassCount);
        e.call(BATCH_RENDERER_CONSTRUCTOR, &[block, passes, batch_argument])
            .u32()
    } else {
        0
    };
    e.set(this, BSShaderAccumulator::pBatchRenderer0, first);
    if e.get(this, BSShaderAccumulator::uiBatchRendererCount) == 2 {
        let block = e.call(OPERATOR_NEW, &[0x98]).u32();
        let second = if block != 0 {
            let passes = e.get(this, BSShaderAccumulator::uiBatchRendererPassCount);
            e.call(BATCH_RENDERER_CONSTRUCTOR, &[block, passes, batch_argument])
                .u32()
        } else {
            0
        };
        e.set(this, BSShaderAccumulator::pBatchRenderer1, second);
    } else {
        e.set(this, BSShaderAccumulator::pBatchRenderer1, 0);
    }
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let passes = e
        .call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, word_array_size(count)])
        .u32();
    e.set(this, BSShaderAccumulator::puicurrentPassA, passes);
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let buckets = e
        .call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, word_array_size(count)])
        .u32();
    e.set(this, BSShaderAccumulator::picurrentBucketA, buckets);
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let first_pass = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, count]).u32();
    e.set(this, BSShaderAccumulator::pbcurrentFirstPassA, first_pass);
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let active = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, count]).u32();
    e.set(this, BSShaderAccumulator::pbcurrentActiveA, active);
    e.set(this, BSShaderAccumulator::eRenderMode, 0);
    release_pointer(e, base + 0x1a0);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_WORD_2BC, 0);
    e.set(this, BSShaderAccumulator::bRenderDecals, 1);
    release_pointer(e, base + 0x198);
    // FLD dword [01017870]; FST to three temporaries (an x87 load quiets a
    // signalling NaN), then FLD1 for the alpha.
    let channel = x87_store32(x87_load32(e.mem.u32(SILHOUETTE_CHANNEL)));
    e.set(this, BSShaderAccumulator::bDoDepthPasses, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_r, channel);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_g, channel);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_b, channel);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_a, ONE_BITS);
    this
}

/// The vtable of a rendered-geometry cache element (`BSTPersistentList`,
/// 0x14 bytes), set by its constructor `00b988c0` and its destructor.
const PERSISTENT_LIST_VTABLE: u32 = 0x010b_7dc0;
/// The element's clean-up `00b63e90` (this unit, not yet translated;
/// thiscall), run before and after the words are reset.
const PERSISTENT_LIST_CLEAR: u32 = 0x00b6_3e90;

// Translated from 00b656b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a rendered-geometry cache element (unnamed in the engine
/// map; the `eh vector constructor iterator` of
/// `BSShaderAccumulator::BSShaderAccumulator` names it): sets the vtable,
/// runs `00b63e90`, copies the word at +0x04 to +0x0C, clears +0x04, +0x08
/// and +0x10, and jumps to `00b63e90` again, passing its result on.
pub fn fn_00b656b0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, PERSISTENT_LIST_VTABLE);
    e.call(PERSISTENT_LIST_CLEAR, &[base]);
    let first = e.mem.u32(base + 4);
    e.mem.set_u32(base + 0xc, first);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.call(PERSISTENT_LIST_CLEAR, &[base]).u32()
}

/// The node pool's lock object (`this` of the lock enter `0040fbf0`); its
/// owner word.
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
/// The lock's entry count, which the inlined leave decrements.
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
/// The lock enter (`0040fbf0`; thiscall, one word).
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// Head of the node pool's free list.
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;

// Translated from 00b63e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clean-up of a rendered-geometry cache element (`BSTPersistentList`;
/// unnamed in the engine map): when its node list (+0x0C) is not empty,
/// enters the node pool's lock and, for each node, enters it again, clears
/// the node's +0x04, pushes the node on the free list at `011c5f58` and
/// leaves (the count at `011c5f84` drops by one; at zero the owner word
/// `011c5f80` is cleared); then leaves the outer entry the same way. Clears
/// +0x0C and the word the pointer at +0x08 points to (when set). Returns
/// that pointer.
pub fn fn_00b63e90(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let mut next = e.mem.u32(base + 0xc);
    if next != 0 {
        e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
        loop {
            let node = next;
            next = e.mem.u32(node);
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
            if next == 0 {
                break;
            }
        }
        let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        if count == 0 {
            e.mem.set_u32(NODE_POOL_LOCK, 0);
        }
    }
    let tail = e.mem.u32(base + 8);
    e.mem.set_u32(base + 0xc, 0);
    if tail != 0 {
        e.mem.set_u32(tail, 0);
    }
    tail
}

#[cfg(test)]
mod element_clear_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        // The lock enter double takes the lock: owner 1, count + 1.
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        e
    }

    #[test]
    fn element_clear_returns_the_nodes_to_the_pool() {
        let mut e = engine();
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0099_0000);
        let this = e.mem.alloc(0x14);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        let tail_slot = e.mem.alloc(4);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x11);
        e.mem.set_u32(first + 8, 0x12);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, 0x21);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(this + 8, tail_slot);
        e.mem.set_u32(tail_slot, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_3e90, &[this]).u32(), tail_slot);
        let locks = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == LOCK_ENTER)
            .count();
        assert_eq!(locks, 3);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), second);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(first), 0x0099_0000);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(second + 4), 0);
        // The node's element (+0x08) is left as it is.
        assert_eq!(e.mem.u32(first + 8), 0x12);
        // Balanced: the lock is released.
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(tail_slot), 0);
    }

    #[test]
    fn element_clear_of_an_empty_list_takes_no_lock() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_3e90, &[this]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 5);
    }
}

/// The renderer holder (`011f4748`): its +0x200 mode, byte +0x208 and the
/// virtual slots +0xAC (set background colour), +0xB4 (get it) and +0x18C
/// (camera set-up) are used here.
const RENDER_HOLDER: u32 = 0x011f_4748;
/// The renderer singleton pointer.
const RENDERER: u32 = 0x011f_9508;
/// The texture manager and the image-space manager singletons.
const TEXTURE_MANAGER: u32 = 0x011f_91a8;
const IMAGE_SPACE_MANAGER: u32 = 0x011f_91ac;
/// The memory manager's deallocation (`00aa4060`; thiscall, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSBatchRenderer`'s functions this unit calls (`bsbatchrenderer.cpp`).
const BATCH_STOP_GROUPING_ALPHAS: u32 = 0x00b9_aec0;
const BATCH_START_GROUPING_ALPHAS: u32 = 0x00b9_ac80;
const BATCH_CLEAR_PASSES: u32 = 0x00b9_9bc0;
const BATCH_HAS_LISTED_PASS: u32 = 0x00b9_8970;
const BATCH_NEXT_LIST: u32 = 0x00b9_a090;
const BATCH_RENDER_LIST: u32 = 0x00b9_a120;
const BATCH_RENDER_LIST_00B9A3C0: u32 = 0x00b9_a3c0;
const BATCH_SORT_ALPHAS: u32 = 0x00b9_9a10;
const BATCH_END_PASS: u32 = 0x00b9_88e0;
const BATCH_RENDER_PASS_IMMEDIATELY: u32 = 0x00b9_94f0;
const BATCH_GROUP_DISTANCE: u32 = 0x00b9_8c80;
const BATCH_RENDER_NEXT_ALPHA: u32 = 0x00b9_af60;
const GEOMETRY_GROUP_DESTRUCTOR: u32 = 0x00b9_a410;
const GEOMETRY_GROUP_RESET: u32 = 0x00b9_a490;
const GEOMETRY_GROUP_RENDER_00B9A4D0: u32 = 0x00b9_a4d0;
/// The clear of a 16-bit list (`00b64ec0`; thiscall).
const SHORT_LIST_CLEAR: u32 = 0x00b6_4ec0;
/// An occlusion query's destructor (`00c4ed40`; thiscall), its result
/// read (`00c4ee40` and `00c4edf0`; thiscall, two words: out slot, 1;
/// answer 0 ready, 1 not yet, other failed), constructor (`00c4ecf0`),
/// begin (`00c4ed70`) and `BSOcclusionQuery::End` (`00c4edb0`).
const QUERY_DESTRUCTOR: u32 = 0x00c4_ed40;
const QUERY_RESULT_00C4EE40: u32 = 0x00c4_ee40;
const QUERY_RESULT_00C4EDF0: u32 = 0x00c4_edf0;
const QUERY_CONSTRUCTOR: u32 = 0x00c4_ecf0;
const QUERY_BEGIN: u32 = 0x00c4_ed70;
const QUERY_END: u32 = 0x00c4_edb0;
/// The float `2^32` the unsigned-to-float conversion adds and the double
/// `1500.0` the sun pixel count is divided by.
const TWO_TO_32: u32 = 0x0109_9378;
const SUN_PIXELS_010737C8: u32 = 0x0107_37c8;

/// The batch renderer of the running thread: `pBatchRenderer[word +0x2BC
/// of the thread block % uiBatchRendererCount]` (the exe's `DIV`, which
/// faults on a zero count as this does).
fn thread_batch_renderer(e: &mut Engine, this: Ptr<BSShaderAccumulator>) -> u32 {
    let tls = e.tls();
    let index = e.mem.u32(tls + TLS_WORD_2BC);
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let slot = index % count;
    e.mem.u32(this.addr() + 0x174 + slot * 4)
}

/// Batch renderer `i` (`pBatchRenderer[i]`).
fn batch_renderer(e: &Engine, this: u32, i: u32) -> u32 {
    e.mem
        .u32(this.wrapping_add(0x174).wrapping_add(i.wrapping_mul(4)))
}

fn batch_count(e: &Engine, this: u32) -> u32 {
    e.mem.u32(this + 0x18c)
}

// Translated from 00b63090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::StopGroupingAlphas` (Xbox PDB): jumps to
/// `BSBatchRenderer::StopGroupingAlphas` on the thread's batch renderer.
pub fn bs_shader_accumulator_stop_grouping_alphas(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let renderer = thread_batch_renderer(e, this);
    e.call(BATCH_STOP_GROUPING_ALPHAS, &[renderer]);
}

// Translated from 00b630c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the passes in use of every batch
/// renderer (`00b99bc0(0)`).
pub fn fn_00b630c0(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    let mut i = 0;
    while i < batch_count(e, base) {
        let renderer = batch_renderer(e, base, i);
        e.call(BATCH_CLEAR_PASSES, &[renderer, 0]);
        i += 1;
    }
}

// Translated from 00b630f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::IsShadowPass` (Xbox PDB; cdecl, one word): the
/// pass types 0x1C4..0x1C7, 0x231..0x234 and 0x284..0x287.
pub fn bs_shader_accumulator_is_shadow_pass(_e: &mut Engine, pass_type: u32) -> bool {
    (0x1c4..=0x1c7).contains(&pass_type)
        || (0x231..=0x234).contains(&pass_type)
        || pass_type.wrapping_sub(0x284) <= 3
}

// Translated from 00b63120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): whether any batch
/// renderer's active pass list holds one of the listed pass types within
/// `low..=high` (`00b98970`; the walk stops at the first that does).
pub fn fn_00b63120(e: &mut Engine, this: Ptr<BSShaderAccumulator>, low: u32, high: u32) -> bool {
    let base = this.addr();
    let mut found = false;
    let mut i = 0;
    while i < batch_count(e, base) {
        if found {
            break;
        }
        let renderer = batch_renderer(e, base, i);
        found = e.call(BATCH_HAS_LISTED_PASS, &[renderer, low, high]).u8() != 0;
        i += 1;
    }
    found
}

// Translated from 00b63500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::StartGroupingAlphas` (Xbox PDB; thiscall, two
/// words): `BSBatchRenderer::StartGroupingAlphas(point, keep, camera
/// (+0x08), the byte +0x31)` on the thread's batch renderer.
pub fn bs_shader_accumulator_start_grouping_alphas(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    point: u32,
    keep: u32,
) {
    let base = this.addr();
    let flag = u32::from(e.mem.u8(base + 0x31));
    let camera = e.mem.u32(base + 8);
    let renderer = thread_batch_renderer(e, this);
    e.call(
        BATCH_START_GROUPING_ALPHAS,
        &[renderer, point, keep, camera, flag],
    );
}

// Translated from 00b63540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): collects the result of
/// sun occlusion test `index` (0..2; 0x14 bytes from +0x48: query, waiting
/// byte, visible fraction, pixel count) while it waits for one. A ready
/// result (`00c4ee40` answers 1, the count written into the argument's own
/// slot) ends the wait and stores the count and `1 - min(count, 1500) /
/// 1500` (the double at `010737c8`; 1.0 for a zero count); a failed query
/// is destroyed and freed and the wait ends; "not yet" changes nothing.
pub fn fn_00b63540(e: &mut Engine, this: Ptr<BSShaderAccumulator>, index: u32) {
    if index >= 3 {
        return;
    }
    let base = this.addr();
    let test = base + index * 0x14;
    if e.mem.u8(test + 0x4c) == 0 {
        return;
    }
    let query = e.mem.u32(test + 0x48);
    if query == 0 {
        return;
    }
    let (state, count) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), index);
        let state = e.call(QUERY_RESULT_00C4EE40, &[query, slot.addr(), 1]).u8();
        (state, e.mem.u32(slot.addr()))
    });
    if state == 1 {
        if count == 0 {
            e.mem.set_u32(test + 0x54, 0);
            e.mem.set_u8(test + 0x4c, 0);
            e.mem.set_u32(test + 0x50, x87_store32(1.0));
            return;
        }
        let clamped = if count > 0x5dc { 0x5dc } else { count };
        let mut value = f64::from(clamped as i32);
        if (clamped as i32) < 0 {
            value = x87_add(value, x87_m32(e.mem.u32(TWO_TO_32)));
        }
        let fraction = x87_div(value, e.mem.f64(SUN_PIXELS_010737C8));
        e.mem.set_u32(test + 0x54, count);
        e.mem.set_u8(test + 0x4c, 0);
        let visible = x87_store32(x87_sub(1.0, fraction));
        e.mem.set_u32(test + 0x50, visible);
        return;
    }
    if state != 0 {
        let query = e.mem.u32(test + 0x48);
        if query != 0 {
            e.call(QUERY_DESTRUCTOR, &[query]);
            e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, query]);
        }
        e.mem.set_u32(test + 0x48, 0);
        e.mem.set_u8(test + 0x4c, 0);
    }
}

/// The bytes that switch on waiting for a reference volume's query
/// (`011f9fc3`) and the "no query, treat as occluded" fallback (`011f9fc4`).
const VOLUME_QUERY_ENABLED: u32 = 0x011f_9fc3;
const VOLUME_QUERY_FALLBACK: u32 = 0x011f_9fc4;

/// Destroys and frees the occlusion query a reference volume holds at +0x14
/// and clears the word.
fn drop_volume_query(e: &mut Engine, volume: u32) {
    let query = e.mem.u32(volume + 0x14);
    if query != 0 {
        e.call(QUERY_DESTRUCTOR, &[query]);
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, query]);
    }
    e.mem.set_u32(volume + 0x14, 0);
}

// Translated from 00b63680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): the occlusion result
/// of the reference volume whose +0x10 is `reference` (the
/// `ReferenceVolumeList` at +0x140), cached in `uiLastOcclusionRef` /
/// `bLastOcclusionResult`. No volume answers false. A volume still waiting
/// (+0x19) and asked to `wait` polls its query (`00c4edf0` until it is not
/// "not yet"; the count goes to the argument's own slot) when the byte at
/// `011f9fc3` is set: ready gives occluded = count 0 and the count; a
/// failure drops the query and gives 999999, not occluded (as the
/// fallback byte `011f9fc4` does without a query). Only a volume no longer
/// waiting is cached. Answers the volume's +0x18.
pub fn fn_00b63680(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    reference: u32,
    wait: u32,
) -> u8 {
    let base = this.addr();
    if reference == e.get(this, BSShaderAccumulator::uiLastOcclusionRef) {
        return e.get(this, BSShaderAccumulator::bLastOcclusionResult);
    }
    let mut node = e.mem.u32(base + 0x140);
    let volume = loop {
        if node == 0 {
            return 0;
        }
        let item = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if item != 0 && e.mem.u32(item + 0x10) == reference {
            break item;
        }
    };
    if e.mem.u8(volume + 0x19) != 0 {
        if wait as u8 != 0 {
            let mut fallback = false;
            if e.mem.u8(VOLUME_QUERY_ENABLED) != 0 && e.mem.u32(volume + 0x14) != 0 {
                let (state, count) = e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), wait);
                    let state = loop {
                        let query = e.mem.u32(volume + 0x14);
                        let state = e.call(QUERY_RESULT_00C4EDF0, &[query, slot.addr(), 1]).u8();
                        if state != 1 {
                            break state;
                        }
                    };
                    (state, e.mem.u32(slot.addr()))
                });
                if state == 0 {
                    e.mem.set_u8(volume + 0x18, u8::from(count == 0));
                    e.mem.set_u32(volume + 0x1c, count);
                } else {
                    drop_volume_query(e, volume);
                    fallback = true;
                }
            } else if e.mem.u8(VOLUME_QUERY_FALLBACK) != 0 {
                fallback = true;
            }
            if fallback {
                e.mem.set_u32(volume + 0x1c, 999_999);
                e.mem.set_u8(volume + 0x18, 0);
            }
            e.mem.set_u8(volume + 0x19, 0);
        }
        if e.mem.u8(volume + 0x19) != 0 {
            return e.mem.u8(volume + 0x18);
        }
    }
    e.set(this, BSShaderAccumulator::uiLastOcclusionRef, reference);
    let result = e.mem.u8(volume + 0x18);
    e.set(this, BSShaderAccumulator::bLastOcclusionResult, result);
    e.mem.u8(volume + 0x18)
}

/// The byte that makes `00b63770` run `00bbb840` first.
const LIGHTING_FLAG_011F94A6: u32 = 0x011f_94a6;
/// `00bbb840` (`lighting30shader.cpp`; no arguments).
const LIGHTING_00BBB840: u32 = 0x00bb_b840;

// Translated from 00b63770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, four words): renders the pass
/// types `low..=high` of the batch renderers (or, for `group` >= 0, of
/// each renderer's geometry group `group`'s own renderer), interleaved by
/// pass type: each renderer gets the range (+0x20, +0x24) and its first
/// list (`00b9a090` into `puicurrentPassA`/`picurrentBucketA`/
/// `pbcurrentFirstPassA`, the result in `pbcurrentActiveA`); then while
/// any is active the one with the lowest pass type (then bucket, then the
/// one with its first-pass flag) renders its list: a shadow pass with the
/// byte +0x164 or +0x165 set through `00b9a3c0(.., 0)`, any other through
/// `00b9a120(.., second)`. Then the geometry groups are reset
/// (`00b9a490`) and the pass ends.
///
/// The exe keeps the renderers and groups in two 2-word stack arrays and
/// fills `uiBatchRendererCount` words of each; the frame here has room for
/// that many.
pub fn fn_00b63770(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    low: u32,
    high: u32,
    second: u32,
    group: i32,
) {
    let base = this.addr();
    if e.mem.u8(LIGHTING_FLAG_011F94A6) != 0 {
        e.call(LIGHTING_00BBB840, &[]);
    }
    let count = batch_count(e, base);
    let frame_size = 0x10u32.max(8u32.wrapping_add(count.wrapping_mul(4)));
    e.with_stack(frame_size, |e, frame| {
        let renderers = frame.addr();
        let groups = frame.addr() + 8;
        if group > -1 {
            for i in 0..count {
                let renderer = batch_renderer(e, base, i);
                let own = e.mem.u32(
                    renderer
                        .wrapping_add(0x3c)
                        .wrapping_add((group as u32).wrapping_mul(4)),
                );
                e.mem.set_u32(groups + i * 4, own);
                let inner = e.mem.u32(own + 8);
                e.mem.set_u32(renderers + i * 4, inner);
            }
        } else if count != 0 {
            for i in 0..count {
                e.mem.set_u32(groups + i * 4, 0);
            }
            for i in 0..count {
                let renderer = batch_renderer(e, base, i);
                e.mem.set_u32(renderers + i * 4, renderer);
            }
        }
        let passes = |e: &Engine| e.get(this, BSShaderAccumulator::puicurrentPassA);
        let buckets = |e: &Engine| e.get(this, BSShaderAccumulator::picurrentBucketA);
        let firsts = |e: &Engine| e.get(this, BSShaderAccumulator::pbcurrentFirstPassA);
        let actives = |e: &Engine| e.get(this, BSShaderAccumulator::pbcurrentActiveA);
        let mut i = 0;
        if count != 0 {
            loop {
                let renderer = e.mem.u32(renderers + i * 4);
                e.mem.set_u32(passes(e) + i * 4, 0);
                if renderer != 0 {
                    e.mem.set_u32(renderer + 0x20, low);
                    e.mem.set_u32(renderer + 0x24, high);
                    let first = firsts(e) + i;
                    let bucket = buckets(e) + i * 4;
                    let pass = passes(e) + i * 4;
                    let active = e
                        .call(BATCH_NEXT_LIST, &[renderer, pass, bucket, first])
                        .u8();
                    e.mem.set_u8(actives(e) + i, active);
                } else {
                    e.mem.set_u8(actives(e) + i, 0);
                }
                i += 1;
                if i >= batch_count(e, base) {
                    break;
                }
            }
        }
        loop {
            let mut any = e.mem.u8(actives(e)) != 0;
            let mut best = 0u32;
            let mut index = 1u32;
            while index < batch_count(e, base) {
                let active = e.mem.u8(actives(e) + index) != 0;
                let better = if !any && active {
                    any = true;
                    true
                } else if active {
                    let pass_of = |e: &Engine, k: u32| e.mem.u32(passes(e) + k * 4);
                    let bucket_of = |e: &Engine, k: u32| e.mem.i32(buckets(e) + k * 4);
                    let p = pass_of(e, index);
                    let q = pass_of(e, best);
                    p < q
                        || (p == q
                            && (bucket_of(e, index) < bucket_of(e, best)
                                || (bucket_of(e, index) == bucket_of(e, best)
                                    && e.mem.u8(firsts(e) + index) != 0)))
                } else {
                    false
                };
                if better {
                    best = index;
                }
                index += 1;
            }
            if !any {
                break;
            }
            let pass_slot = passes(e) + best * 4;
            let pass_type = e.mem.u32(pass_slot);
            let renderer = e.mem.u32(renderers + best * 4);
            let shadow = (0x1c4..=0x1c7).contains(&pass_type)
                || (0x231..=0x234).contains(&pass_type)
                || pass_type.wrapping_sub(0x284) <= 3;
            let first = firsts(e) + best;
            let bucket = buckets(e) + best * 4;
            let active = if shadow && (e.mem.u8(base + 0x164) != 0 || e.mem.u8(base + 0x165) != 0) {
                e.call(
                    BATCH_RENDER_LIST_00B9A3C0,
                    &[renderer, pass_slot, bucket, first, 0],
                )
                .u8()
            } else {
                e.call(
                    BATCH_RENDER_LIST,
                    &[renderer, pass_slot, bucket, first, second],
                )
                .u8()
            };
            e.mem.set_u8(actives(e) + best, active);
        }
        let mut i = 0;
        while i < batch_count(e, base) {
            let own = e.mem.u32(groups + i * 4);
            if own != 0 {
                e.call(GEOMETRY_GROUP_RESET, &[own]);
            }
            i += 1;
        }
    });
    e.call(BATCH_END_PASS, &[]);
}

// Translated from 00b639e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): renders geometry group
/// `group` (+0x3C array of the batch renderers): when the first renderer's
/// group keeps its passes itself (bit 0 of its first byte) each renderer's
/// group renders them (`00b9a4d0(flag)`), otherwise its pass types 1..0x2F1
/// go through `00b63770(1, 0x2F1, flag, group)`. Nothing without a group.
pub fn fn_00b639e0(e: &mut Engine, this: Ptr<BSShaderAccumulator>, group: u32, flag: u32) {
    let base = this.addr();
    let slot = |e: &Engine, i: u32| {
        batch_renderer(e, base, i)
            .wrapping_add(group.wrapping_mul(4))
            .wrapping_add(0x3c)
    };
    let first = e.mem.u32(slot(e, 0));
    if first == 0 {
        return;
    }
    if e.mem.u8(first) & 1 == 0 {
        fn_00b63770(e, this, 1, 0x2f1, flag, group as i32);
        return;
    }
    let mut i = 0;
    while i < batch_count(e, base) {
        let own = e.mem.u32(slot(e, i));
        e.call(GEOMETRY_GROUP_RENDER_00B9A4D0, &[own, flag]);
        i += 1;
    }
}

// Translated from 00b63a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sorts every batch renderer's alpha groups
/// along the camera (+0x08) with the byte +0x31 (`00b99a10`).
pub fn fn_00b63a50(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    let mut i = 0;
    while i < batch_count(e, base) {
        let flag = u32::from(e.mem.u8(base + 0x31));
        let camera = e.mem.u32(base + 8);
        let renderer = batch_renderer(e, base, i);
        e.call(BATCH_SORT_ALPHAS, &[renderer, camera, flag]);
        i += 1;
    }
}

/// The 16-bit count of a batch renderer's geometry group in slot `offset`.
fn group_count(e: &Engine, renderer: u32, offset: u32) -> u32 {
    let group = e.mem.u32(renderer + offset);
    u32::from(e.mem.u16(group + 2))
}

// Translated from 00b63a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the summed 16-bit counts of the geometry
/// group at +0x54 of every batch renderer (the exe adds them two at a
/// time, then the odd one).
pub fn fn_00b63a90(e: &mut Engine, this: Ptr<BSShaderAccumulator>) -> u32 {
    let base = this.addr();
    let count = batch_count(e, base);
    let mut even = 0u32;
    let mut odd = 0u32;
    let mut index = 0u32;
    if count as i32 >= 2 {
        let pairs = ((count - 2) >> 1) + 1;
        index = pairs * 2;
        for pair in 0..pairs {
            even = even.wrapping_add(group_count(e, batch_renderer(e, base, pair * 2), 0x54));
            odd = odd.wrapping_add(group_count(e, batch_renderer(e, base, pair * 2 + 1), 0x54));
        }
    }
    let mut last = 0u32;
    if index < count {
        last = group_count(e, batch_renderer(e, base, index), 0x54);
    }
    odd.wrapping_add(even).wrapping_add(last)
}

/// `EnterCriticalSection`, `LeaveCriticalSection` and
/// `DeleteCriticalSection` (import slots).
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// The room map's `RemoveAll` (`00b601a0`; thiscall).
const ROOM_MAP_REMOVE_ALL: u32 = 0x00b6_01a0;

// Translated from 00b63b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the room map (+0x1B4, `00b601a0`)
/// inside its critical section (+0x200).
pub fn fn_00b63b60(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &[base + 0x200]);
    e.call(ROOM_MAP_REMOVE_ALL, &[base + 0x1b4]);
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &[base + 0x200]);
}

// Translated from 00b63b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): resets geometry group
/// `group` of every batch renderer (`00b9a490`).
pub fn fn_00b63b90(e: &mut Engine, this: Ptr<BSShaderAccumulator>, group: u32) {
    let base = this.addr();
    let mut i = 0;
    while i < batch_count(e, base) {
        let renderer = batch_renderer(e, base, i);
        let own = e.mem.u32(
            renderer
                .wrapping_add(group.wrapping_mul(4))
                .wrapping_add(0x3c),
        );
        e.call(GEOMETRY_GROUP_RESET, &[own]);
        i += 1;
    }
}

/// `ImageSpaceTexture::GetRenderedTexture` and `::SetTexture` (engine map;
/// thiscall), and `BSTextureManager::ReturnRenderedTexture` (thiscall, one
/// word).
const IMAGE_SPACE_TEXTURE_GET_RENDERED: u32 = 0x00ba_3770;
const IMAGE_SPACE_TEXTURE_SET: u32 = 0x00ba_37a0;
const TEXTURE_MANAGER_RETURN_RENDERED: u32 = 0x00b6_da10;

// Translated from 00b63d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderVolumetricFog::ReturnVolumetricFogTexture` (engine
/// map; the body fits it): when the image-space texture at +0x08 of the
/// object at +0x98 has a rendered texture, gives it back to the texture
/// manager and clears the image-space texture (`SetTexture(0)`).
pub fn image_space_shader_volumetric_fog_return_volumetric_fog_texture(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let holder = e.mem.u32(base + 0x98);
    if e.mem.u32(holder + 8) == 0 {
        return;
    }
    let texture = e.mem.u32(holder + 8);
    if e.call(IMAGE_SPACE_TEXTURE_GET_RENDERED, &[texture]).u32() == 0 {
        return;
    }
    let holder = e.mem.u32(base + 0x98);
    let texture = e.mem.u32(holder + 8);
    let manager = e.mem.u32(TEXTURE_MANAGER);
    let rendered = e.call(IMAGE_SPACE_TEXTURE_GET_RENDERED, &[texture]).u32();
    e.call(TEXTURE_MANAGER_RETURN_RENDERED, &[manager, rendered]);
    let holder = e.mem.u32(base + 0x98);
    let texture = e.mem.u32(holder + 8);
    e.call(IMAGE_SPACE_TEXTURE_SET, &[texture, 0]);
}

/// The skin instance class record `00b63f10` checks.
const SKIN_RTTI_011F49D8: u32 = 0x011f_49d8;
/// The per-render-mode register functions (cdecl, five words:
/// accumulator, geometry, property state, properties, shader); entry 0 is
/// the fallback for an empty entry.
const REGISTER_FUNCTIONS: u32 = 0x011f_9f80;

// Translated from 00b63f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, slot +0x98 of `BSShaderAccumulator`'s vtable
/// (Ghidra made no function here; read from the bytes; thiscall, one
/// word): registers a geometry with the render mode's function (the table
/// at `011f9f80`, entry 0 when the mode's is null), passing the
/// accumulator, the geometry, its property state (+0x9C), properties
/// (+0xA8) and shader (+0xC0), and answers its result. Answers true
/// without calling when the geometry's skin instance (+0xBC) is of the
/// class `011f49d8` with its byte +0x3C clear, or when it has no shader
/// or properties.
pub fn fn_00b63f10(e: &mut Engine, this: Ptr<BSShaderAccumulator>, geometry: u32) -> bool {
    let skin = e.mem.u32(geometry + 0xbc);
    if skin != 0 && e.vcall(skin, 8, &[]).u32() == SKIN_RTTI_011F49D8 && e.mem.u8(skin + 0x3c) == 0
    {
        return true;
    }
    let shader = e.mem.u32(geometry + 0xc0);
    let properties = e.mem.u32(geometry + 0xa8);
    if shader == 0 || properties == 0 {
        return true;
    }
    let mode = e.get(this, BSShaderAccumulator::eRenderMode);
    let function = e
        .mem
        .u32(REGISTER_FUNCTIONS.wrapping_add(mode.wrapping_mul(4)));
    let words = [this.addr(), geometry, geometry + 0x9c, properties, shader];
    if function != 0 {
        e.call(function, &words).bool()
    } else {
        let fallback = e.mem.u32(REGISTER_FUNCTIONS);
        e.call(fallback, &words).bool()
    }
}

/// The calls the shadow-map passes share: the normals texture
/// (`ImageSpaceEffectRefraction::GetNormalsTexture`, `00bd9380`; thiscall,
/// one word), a render-target getter (`00b6b260`, named
/// `BSRenderedTexture::StopOffscreen` in the engine map; thiscall), the
/// target switch (`00b6b8d0`; cdecl: flag, target), the renderer clears
/// `00b6b790` and the state reset `00b98380` (cdecl, two words).
const GET_NORMALS_TEXTURE: u32 = 0x00bd_9380;
const RENDER_TARGET_OF: u32 = 0x00b6_b260;
const SET_RENDER_TARGET: u32 = 0x00b6_b8d0;
const RENDERER_CLEAR: u32 = 0x00b6_b790;
const STATE_RESET_00B98380: u32 = 0x00b9_8380;
/// The current rendered texture (`00b4f530`, which jumps to `00b8b190` on
/// the image-space manager).
const CURRENT_RENDERED_TEXTURE: u32 = 0x00b4_f530;
/// The half the background colour of the normals pass uses (`01016248`).
const HALF: u32 = 0x0101_6248;
/// The byte that shows the normals texture through the debug shader
/// (`011f9fc2`), `BSShaderManager::GetShader` and
/// `DebugShader::SetTextureToRender` (thiscall, one word).
const SHOW_NORMALS: u32 = 0x011f_9fc2;
const GET_SHADER: u32 = 0x00b5_5560;
const DEBUG_SET_TEXTURE: u32 = 0x00bd_22c0;

/// Copies the camera's three rotation columns into `v68` (+0x68, +0x74,
/// +0x80), `v6c` (+0x6C, ..) and `v70` (+0x70, ..), each float through x87.
fn camera_columns(e: &mut Engine, camera: u32, v68: u32, v6c: u32, v70: u32) {
    for (target, start) in [(v70, 0x70), (v6c, 0x6c), (v68, 0x68)] {
        for k in 0..3 {
            let value = x87_quiet_bits(e.mem.u32(camera + start + k * 0xc));
            e.mem.set_u32(target + k * 4, value);
        }
    }
}

/// The renderer holder's camera set-up (slot +0x18C) when its mode (+0x200)
/// is 1 or 2 and its byte +0x208 is 1.
fn holder_camera_setup(e: &mut Engine, holder: u32, camera: u32, vectors: [u32; 3], eye: u32) {
    let mode = e.mem.u32(holder + 0x200);
    if (mode == 1 || mode == 2) && e.mem.u8(holder + 0x208) == 1 {
        e.vcall(
            holder,
            0x18c,
            &[
                camera + 0x8c,
                vectors[0],
                vectors[1],
                vectors[2],
                eye,
                camera + 0x100,
            ],
        );
    }
}

/// Copies what the render target of `from` holds into that of `to` (the
/// first target's slot +0xB4 answers the value the second's slot +0xAC
/// takes).
fn copy_render_target(e: &mut Engine, from: u32, to: u32) {
    let source = e.call(RENDER_TARGET_OF, &[from]).u32();
    let target = e.call(RENDER_TARGET_OF, &[to]).u32();
    let value = e.vcall(source, 0xb4, &[]).u32();
    e.vcall(target, 0xac, &[value]);
}

/// Saves the background colour into `saved` and sets (h, h, 0, 0) with
/// h the float at `01016248`, in `colour`.
fn set_normals_background(e: &mut Engine, holder: u32, saved: u32, colour: u32) {
    e.vcall(holder, 0xb4, &[saved]);
    let half = x87_quiet_bits(e.mem.u32(HALF));
    e.mem.set_u32(colour, half);
    e.mem.set_u32(colour + 4, half);
    e.mem.set_u32(colour + 8, x87_store32(0.0));
    e.mem.set_u32(colour + 0xc, x87_store32(0.0));
    e.vcall(holder, 0xac, &[colour]);
}

// Translated from 00b64570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): renders the normals
/// passes for `camera` into the normals texture (`00bd9380(0)`, made with
/// `00bd9380(1)` when needed for the second part) and copies it with
/// `target`. Nothing without a `target`. The camera replaces +0x08 for the
/// duration. With a texture and items in the geometry groups +0x5C, the
/// background becomes (h, h, 0, 0), the texture is set as the render target
/// (`00b6b8d0(0, ..)`), the camera set up and group 8 rendered; the +0x5C
/// groups are then reset. With items in the +0x54 groups the same set-up
/// (if not done) renders group 6, and the debug shader may show the
/// texture (`011f9fc2`). Afterwards the renderer is cleared when anything
/// rendered, the target copied back, and the background restored. Answers
/// false.
pub fn fn_00b64570(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    camera: u32,
    target: u32,
) -> bool {
    let base = this.addr();
    if target == 0 {
        return false;
    }
    let old_camera = e.mem.u32(base + 8);
    e.mem.set_u32(base + 8, camera);
    let holder = e.mem.u32(RENDER_HOLDER);
    e.with_stack(0x50, |e, frame| {
        let f = frame.addr();
        let saved = f + 0x40;
        let colour = f + 0x30;
        let zero = x87_store32(0.0);
        for k in 0..4 {
            e.mem.set_u32(saved + k * 4, zero);
        }
        let mut flags_c = 0u32;
        let mut restore = false;
        let mut rendered = false;
        let normals_owner = |e: &Engine| {
            let manager = e.mem.u32(IMAGE_SPACE_MANAGER);
            e.mem.u32(e.mem.u32(manager + 8) + 8)
        };
        let owner = normals_owner(e);
        let mut texture = e.call(GET_NORMALS_TEXTURE, &[owner, 0]).u32();
        e.call(STATE_RESET_00B98380, &[0xf, 0]);
        let mut items = 0u32;
        for i in 0..batch_count(e, base) {
            items = items.wrapping_add(group_count(e, batch_renderer(e, base, i), 0x5c));
        }
        let mut slot_54 = camera;
        if texture != 0 && items as i32 > 0 {
            let source = e.call(RENDER_TARGET_OF, &[target]).u32();
            let second = e.call(RENDER_TARGET_OF, &[texture]).u32();
            slot_54 = second;
            let value = e.vcall(source, 0xb4, &[]).u32();
            e.vcall(second, 0xac, &[value]);
            set_normals_background(e, holder, saved, colour);
            let holder_now = e.mem.u32(RENDER_HOLDER);
            restore = true;
            if e.mem.u32(holder_now + 0x200) != 0 {
                e.call(RENDERER_CLEAR, &[]);
            }
            let rt = e.call(RENDER_TARGET_OF, &[texture]).u32();
            e.call(SET_RENDER_TARGET, &[0, rt]);
            let cam = e.mem.u32(base + 8);
            camera_columns(e, cam, f + 0x30, f + 0x24, f + 0x18);
            holder_camera_setup(e, holder, cam, [f + 0x30, f + 0x24, f + 0x18], cam + 0xdc);
            fn_00b639e0(e, this, 8, 0);
            rendered = true;
        }
        for i in 0..batch_count(e, base) {
            let group = e.mem.u32(batch_renderer(e, base, i) + 0x5c);
            e.call(GEOMETRY_GROUP_RESET, &[group]);
        }
        let mut sum = 0u32;
        if batch_count(e, base) != 0 {
            for i in 0..batch_count(e, base) {
                sum = sum.wrapping_add(group_count(e, batch_renderer(e, base, i), 0x54));
            }
            if sum != 0 {
                if !rendered {
                    let mut made = false;
                    if texture == 0 {
                        let owner = normals_owner(e);
                        texture = e.call(GET_NORMALS_TEXTURE, &[owner, 1]).u32();
                        made = true;
                    }
                    slot_54 = (slot_54 & 0xffff_ff00) | u32::from(made);
                    copy_render_target(e, target, texture);
                    set_normals_background(e, holder, saved, colour);
                    restore = true;
                    e.call(RENDERER_CLEAR, &[]);
                    let rt = e.call(RENDER_TARGET_OF, &[texture]).u32();
                    e.call(SET_RENDER_TARGET, &[u32::from(made), rt]);
                    let cam = e.mem.u32(base + 8);
                    camera_columns(e, cam, f + 0x18, f + 0x24, f + 0x30);
                    holder_camera_setup(e, holder, cam, [f + 0x18, f + 0x24, f + 0x30], cam + 0xdc);
                }
                fn_00b639e0(e, this, 6, 0);
                if e.mem.u8(SHOW_NORMALS) != 0 {
                    let (held, shown) = if texture != 0 {
                        (slot_54, e.mem.u32(texture + 0x30))
                    } else {
                        flags_c = 1;
                        (0, 0)
                    };
                    // The inlined `NiPointer` temporary's release: only
                    // with bit 0 of the frame word set, which only the null
                    // texture path sets (with nothing held).
                    if flags_c & 1 != 0
                        && held != 0
                        && e.call(IMPORT_INTERLOCKED_DECREMENT, &[held + 4]).u32() == 0
                    {
                        e.vcall(held, 4, &[]);
                    }
                    let shader = e.call(GET_SHADER, &[9]).u32();
                    e.call(DEBUG_SET_TEXTURE, &[shader, shown]);
                }
            }
        }
        if rendered || sum != 0 {
            e.call(RENDERER_CLEAR, &[]);
        }
        if texture != 0 {
            copy_render_target(e, target, texture);
        }
        if restore {
            e.vcall(holder, 0xac, &[saved]);
        }
    });
    e.mem.set_u32(base + 8, old_camera);
    false
}

// Translated from 00b64a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): renders geometry group
/// 0x0B for `camera` (kept in +0x08) into the current rendered texture with
/// a white background: saves the background, sets (1, 1, 1, 1), clears
/// the renderer when its +0x200 is set, sets the texture as render target
/// 7, sets the camera up and renders group 0x0B; with `second_camera`
/// (whose +0xDC is the eye) also group 0x0C. Then clears the renderer and
/// restores the background.
pub fn fn_00b64a00(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    camera: u32,
    second_camera: u32,
) {
    let base = this.addr();
    let holder = e.mem.u32(RENDER_HOLDER);
    e.with_stack(0x38, |e, frame| {
        let f = frame.addr();
        let saved = f + 0x28;
        let white = f + 0x18;
        let zero = x87_store32(0.0);
        for k in 0..4 {
            e.mem.set_u32(saved + k * 4, zero);
        }
        e.vcall(holder, 0xb4, &[saved]);
        let one = x87_store32(1.0);
        for k in 0..4 {
            e.mem.set_u32(white + k * 4, one);
        }
        e.vcall(holder, 0xac, &[white]);
        let texture = e.call(CURRENT_RENDERED_TEXTURE, &[]).u32();
        let holder_now = e.mem.u32(RENDER_HOLDER);
        if e.mem.u32(holder_now + 0x200) != 0 {
            e.call(RENDERER_CLEAR, &[]);
        }
        let rt = e.call(RENDER_TARGET_OF, &[texture]).u32();
        e.call(SET_RENDER_TARGET, &[7, rt]);
        e.mem.set_u32(base + 8, camera);
        camera_columns(e, camera, f + 0x18, f + 0xc, f);
        holder_camera_setup(e, holder, camera, [f + 0x18, f + 0xc, f], camera + 0xdc);
        fn_00b639e0(e, this, 0xb, 0);
        if second_camera != 0 {
            let cam = e.mem.u32(base + 8);
            camera_columns(e, cam, f, f + 0xc, f + 0x18);
            holder_camera_setup(e, holder, cam, [f, f + 0xc, f + 0x18], second_camera + 0xdc);
            fn_00b639e0(e, this, 0xc, 0);
        }
        e.call(RENDERER_CLEAR, &[]);
        e.vcall(holder, 0xac, &[saved]);
    });
}

/// The shader-manager calls of the sun query: `BSShaderManager::GetShader`
/// (cdecl, one word) and `00b7a870` (cdecl, two words).
const SHADER_SETUP_00B7A870: u32 = 0x00b7_a870;
/// The holder's method `004e9bb0` (`teswater.cpp`; thiscall, one word).
const HOLDER_METHOD_004E9BB0: u32 = 0x004e_9bb0;
/// `BSShaderProperty::RenderPass::~RenderPass` (engine map; thiscall).
const RENDER_PASS_DESTRUCTOR: u32 = 0x00ba_8cd0;

// Translated from 00b64bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::DoSunOcclusionQueury` (Xbox PDB; thiscall, three
/// words): unless the accumulator waits for a sun query (+0x40) or test
/// `index` (0..2) already waits, makes the test's occlusion query when it
/// has none (4 bytes, `00c4ecf0`; a failure sets the test's fraction to 0,
/// clears its wait byte and +0x58 and stops), sets up shader 1
/// (`00b7a870(9, 0)`, slots +0x24 of its +0x34 and +0x30), begins the
/// query, passes `argument` to the holder (`004e9bb0`), renders `geometry`
/// as pass type 9 through a stack `RenderPass` (`00ba8ec0(.., geometry,
/// 9, 1, 0, 0)`, `RenderPassImmediately(.., 9, 0, 0, 0)`), ends the query,
/// sets the wait byte, clears +0x58 and destroys the pass.
// Translation incomplete: 00b64bf0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the query and the render pass) is not translated.
pub fn bs_shader_accumulator_do_sun_occlusion_queury(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    geometry: u32,
    index: u32,
    argument: u32,
) {
    let base = this.addr();
    if e.get(this, BSShaderAccumulator::bWaitingForSunQueury) != 0 || index >= 3 {
        return;
    }
    let test = base + index * 0x14;
    if e.mem.u8(test + 0x4c) != 0 {
        return;
    }
    if e.mem.u32(test + 0x48) == 0 {
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 4]).u32();
        let query = if block != 0 {
            e.call(QUERY_CONSTRUCTOR, &[block]).u32()
        } else {
            0
        };
        e.mem.set_u32(test + 0x48, query);
        if query == 0 {
            e.mem.set_u32(test + 0x50, x87_store32(0.0));
            e.mem.set_u8(test + 0x4c, 0);
            e.mem.set_u32(test + 0x58, 0);
            return;
        }
    }
    let shader = e.call(GET_SHADER, &[1]).u32();
    e.call(SHADER_SETUP_00B7A870, &[9, 0]);
    let first = e.mem.u32(shader + 0x34);
    e.vcall(first, 0x24, &[]);
    let second = e.mem.u32(shader + 0x30);
    e.vcall(second, 0x24, &[]);
    let query = e.mem.u32(test + 0x48);
    e.call(QUERY_BEGIN, &[query]);
    let holder = e.mem.u32(RENDER_HOLDER);
    e.call(HOLDER_METHOD_004E9BB0, &[holder, argument]);
    e.with_stack(0x10, |e, pass| {
        let pass = pass.addr();
        e.call(RENDER_PASS_CONSTRUCTOR, &[pass, geometry, 9, 1, 0, 0]);
        e.call(BATCH_RENDER_PASS_IMMEDIATELY, &[pass, 9, 0, 0, 0]);
        let query = e.mem.u32(test + 0x48);
        e.call(QUERY_END, &[query]);
        e.mem.set_u8(test + 0x4c, 1);
        e.mem.set_u32(test + 0x58, 0);
        e.call(RENDER_PASS_DESTRUCTOR, &[pass]);
    });
}

// Translated from 00b64f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall on a `BSBatchRenderer`, one word):
/// resets alpha geometry group `index` (+0x70 array, `00b9a490`),
/// destroys and frees every alpha group of its list (+0x78, 0x0C each),
/// empties the list (`RemoveAll`) and clears the current group
/// (+0x90 array).
pub fn fn_00b64f20(e: &mut Engine, renderer: Ptr, index: u32) {
    let base = renderer.addr();
    let group = e.mem.u32(base + index * 4 + 0x70);
    e.call(GEOMETRY_GROUP_RESET, &[group]);
    let list = base + index * 0xc + 0x78;
    let mut node = e.mem.u32(list);
    while node != 0 {
        let item = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if item != 0 {
            e.call(GEOMETRY_GROUP_DESTRUCTOR, &[item]);
            e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, item]);
        }
    }
    e.call(POINTER_LIST_REMOVE_ALL, &[list]);
    e.mem.set_u32(base + index * 4 + 0x90, 0);
}

/// Gives the node `node` back to the node pool (lock, free list, count),
/// with its item cleared before the lock is taken, as the exe does.
fn pool_return(e: &mut Engine, node: u32) {
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
}

// Translated from 00b64f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: ends the current pass, renders the sorted
/// alpha items back to front (the +0x24 array from +0x1C down, each through
/// the render pass at +0x1A4: `RenderPassImmediately(pass, its type, 1, 1,
/// 1)`), then empties the item list: the one at +0x18 when set (head,
/// tail, count), otherwise the own list at +0x0C (count +0x14), each node
/// going back to the pool. Ends the pass again.
pub fn fn_00b64f90(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    e.call(BATCH_END_PASS, &[]);
    let mut index = e.mem.u32(base + 0x1c);
    e.mem.set_u32(base + 0x2c, index);
    if index != 0 {
        index -= 1;
        e.mem.set_u32(base + 0x2c, index);
        let mut item = e.mem.u32(e.mem.u32(base + 0x24) + index * 4);
        while item != 0 {
            let pass_type = u32::from(e.mem.u16(base + 0x1a8));
            e.mem.set_u32(base + 0x1a4, item);
            e.call(
                BATCH_RENDER_PASS_IMMEDIATELY,
                &[base + 0x1a4, pass_type, 1, 1, 1],
            );
            let index = e.mem.u32(base + 0x2c);
            if index == 0 {
                break;
            }
            e.mem.set_u32(base + 0x2c, index - 1);
            item = e.mem.u32(e.mem.u32(base + 0x24) + (index - 1) * 4);
        }
    }
    let list = e.mem.u32(base + 0x18);
    if list != 0 {
        if e.mem.u32(list + 8) != 0 {
            loop {
                let list = e.mem.u32(base + 0x18);
                let node = e.mem.u32(list);
                let next = e.mem.u32(node);
                e.mem.set_u32(list, next);
                if next != 0 {
                    e.mem.set_u32(next + 4, 0);
                } else {
                    e.mem.set_u32(list + 4, 0);
                }
                pool_return(e, node);
                let count = e.mem.u32(list + 8).wrapping_sub(1);
                e.mem.set_u32(list + 8, count);
                if e.mem.u32(e.mem.u32(base + 0x18) + 8) == 0 {
                    break;
                }
            }
        }
    } else if e.mem.u32(base + 0x14) != 0 {
        loop {
            let node = e.mem.u32(base + 0xc);
            let next = e.mem.u32(node);
            e.mem.set_u32(base + 0xc, next);
            if next != 0 {
                e.mem.set_u32(next + 4, 0);
            } else {
                e.mem.set_u32(base + 0x10, 0);
            }
            pool_return(e, node);
            let count = e.mem.u32(base + 0x14).wrapping_sub(1);
            e.mem.set_u32(base + 0x14, count);
            if count == 0 {
                break;
            }
        }
    }
    e.call(BATCH_END_PASS, &[]);
}

/// `-FLT_MAX` as a float (`01015f5c`) and as the double the loop compares
/// with (`010241b0`).
const NO_DISTANCE: u32 = 0x0101_5f5c;
const NO_DISTANCE_DOUBLE: u32 = 0x0102_41b0;

// Translated from 00b650c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::RenderAlphaGeometry` (Xbox PDB; thiscall, one
/// word): renders alpha group `group` of the one or two batch renderers
/// back to front: the distance of each renderer's group (`00b98c80` with
/// the camera +0x08 and the byte +0x31; `-FLT_MAX` for a missing second
/// renderer); while either is above the double at `010241b0`, the
/// renderer with the farther one (the second on a tie) renders its next
/// alphas down to the other's distance (`RenderNextAlpha`) and its
/// distance is taken again. Then each renderer's groups are cleared
/// (`00b64f20`) and the pass ends.
///
/// The flag goes out as a word whose upper bytes are a stack slot's: an
/// uninitialised local for the first distance (passed as 0 here) and the
/// `group` argument's for the others, as the exe pushes them.
pub fn bs_shader_accumulator_render_alpha_geometry(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    group: u32,
) {
    let base = this.addr();
    e.call(BATCH_END_PASS, &[]);
    let flag = u32::from(e.mem.u8(base + 0x31));
    let camera = e.mem.u32(base + 8);
    let first = batch_renderer(e, base, 0);
    let mut distances = [0u32; 2];
    distances[0] = x87_store32(
        e.call(BATCH_GROUP_DISTANCE, &[first, group, camera, flag])
            .st0,
    );
    let second = if batch_count(e, base) == 2 {
        let flag = (group & 0xffff_ff00) | u32::from(e.mem.u8(base + 0x31));
        let camera = e.mem.u32(base + 8);
        let renderer = batch_renderer(e, base, 1);
        e.call(BATCH_GROUP_DISTANCE, &[renderer, group, camera, flag])
            .st0
    } else {
        x87_load32(e.mem.u32(NO_DISTANCE))
    };
    distances[1] = x87_store32(second);
    loop {
        let floor = e.mem.f64(NO_DISTANCE_DOUBLE);
        let a = x87_load32(distances[0]);
        let b = x87_load32(distances[1]);
        if !(a > floor) && !(b > floor) {
            break;
        }
        // FCOMPP: the second renderer when the first is not farther.
        let which = usize::from(!(a > b));
        let limit = x87_quiet_bits(distances[1 - which]);
        let flag = u32::from(e.mem.u8(base + 0x31));
        let camera = e.mem.u32(base + 8);
        let renderer = batch_renderer(e, base, which as u32);
        e.call(
            BATCH_RENDER_NEXT_ALPHA,
            &[renderer, group, camera, flag, limit],
        );
        let flag = (group & 0xffff_ff00) | u32::from(e.mem.u8(base + 0x31));
        let camera = e.mem.u32(base + 8);
        let renderer = batch_renderer(e, base, which as u32);
        let distance = e
            .call(BATCH_GROUP_DISTANCE, &[renderer, group, camera, flag])
            .st0;
        distances[which] = x87_store32(distance);
    }
    fn_00b64f20(e, Ptr::new(batch_renderer(e, base, 0)), group);
    if batch_count(e, base) == 2 {
        fn_00b64f20(e, Ptr::new(batch_renderer(e, base, 1)), group);
    }
    e.call(BATCH_END_PASS, &[]);
}

// Translated from 00b65550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0xC`: three words, the third
/// never read): `00b64a00(first, second)`.
pub fn fn_00b65550(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    first: u32,
    second: u32,
    _unused_2: u32,
) {
    fn_00b64a00(e, this, first, second);
}

// Translated from 00b65570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: jumps to `NiTPointerListBase<..>::RemoveAll`
/// on the item list at +0x0C.
pub fn fn_00b65570(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    e.call(POINTER_LIST_REMOVE_ALL, &[this.addr() + 0xc]);
}

// Translated from 00b65580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears every batch renderer's active pass
/// list (+0x2C, `00b64ec0`).
pub fn fn_00b65580(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    let mut i = 0;
    while i < batch_count(e, base) {
        let renderer = batch_renderer(e, base, i);
        e.call(SHORT_LIST_CLEAR, &[renderer + 0x2c]);
        i += 1;
    }
}

// Translated from 00b655b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::ClearAllBoundVolumes` (Xbox PDB): while the
/// `ReferenceVolumeList` (+0x140) counts items, destroys the head's volume
/// (its query first) and frees it, and unlinks the head node back to the
/// node pool.
pub fn bs_shader_accumulator_clear_all_bound_volumes(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
) {
    let base = this.addr();
    while e.mem.u32(base + 0x148) != 0 {
        let volume = e.mem.u32(e.mem.u32(base + 0x140) + 8);
        if volume != 0 {
            drop_volume_query_kept(e, volume);
            e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, volume]);
        }
        let node = e.mem.u32(base + 0x140);
        let next = e.mem.u32(node);
        e.mem.set_u32(base + 0x140, next);
        if next != 0 {
            e.mem.set_u32(next + 4, 0);
        } else {
            e.mem.set_u32(base + 0x144, 0);
        }
        pool_return(e, node);
        let count = e.mem.u32(base + 0x148).wrapping_sub(1);
        e.mem.set_u32(base + 0x148, count);
    }
}

/// The volume clean-up of `ClearAllBoundVolumes` and the destructor: the
/// query (+0x14) is destroyed and freed and the word cleared only when set.
fn drop_volume_query_kept(e: &mut Engine, volume: u32) {
    let query = e.mem.u32(volume + 0x14);
    if query != 0 {
        e.call(QUERY_DESTRUCTOR, &[query]);
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, query]);
        e.mem.set_u32(volume + 0x14, 0);
    }
}

/// Empties a rendered-geometry cache element: clean-up (`00b63e90`), nodes
/// to its free list, words cleared.
fn empty_element(e: &mut Engine, element: u32) {
    fn_00b63e90(e, Ptr::new(element));
    let head = e.mem.u32(element + 4);
    e.mem.set_u32(element + 0xc, head);
    e.mem.set_u32(element + 4, 0);
    e.mem.set_u32(element + 8, 0);
    e.mem.set_u32(element + 0x10, 0);
}

// Translated from 00b65660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the two rendered-geometry caches'
/// elements pairwise (+0x9C and +0xEC, four of 0x14 bytes each).
pub fn fn_00b65660(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    for i in 0..4 {
        empty_element(e, base + 0x9c + i * 0x14);
        empty_element(e, base + 0xec + i * 0x14);
    }
}

/// The `NiAccumulator` base destructor (`00a9b4a0`; thiscall).
const ACCUMULATOR_BASE_DESTRUCTOR: u32 = 0x00a9_b4a0;
/// The bucket array's release (`00aa10f0`; cdecl, one word).
const FREE_BUCKETS: u32 = 0x00aa_10f0;
/// The instance renderer's destructor (`00bd9150`; thiscall).
const INSTANCE_RENDERER_DESTRUCTOR: u32 = 0x00bd_9150;
/// The compiler's `eh vector destructor iterator` (stdcall: array, element
/// size, count, destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The reference volume list's removal by item (`00e7db70`; thiscall, one
/// word: the address of the item).
const VOLUME_LIST_REMOVE: u32 = 0x00e7_db70;

/// Drops the `NiPointer` at `slot` without clearing it (the member
/// destructors' inlined release).
fn release_kept(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 && e.call(IMPORT_INTERLOCKED_DECREMENT, &[object + 4]).u32() == 0 {
        e.vcall(object, 4, &[]);
    }
}

// Translated from 00b656e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSShaderAccumulator` destructor (unnamed in the engine map; the
/// scalar deleting destructor `00b664f0` calls it): sets the vtable,
/// empties the cache elements (as `00b65660`), frees the occlusion query
/// slots (+0x8C), clears the reference volumes (as
/// `ClearAllBoundVolumes`), drops the bound test geometry (+0x13C), the
/// three sun queries, the instance renderer (+0x170), deletes both batch
/// renderers (their slot 0 with 1), frees the four per-renderer arrays,
/// drops the shadow light (+0x1A0), empties the room map, drops the water
/// fog target (+0x198), deletes the critical section, then runs the member
/// destructors (room map with its vtables and bucket array, the render
/// pass +0x1A4, the two pointers again, the volume list, the bound test
/// geometry again, both caches, the fog list) and the `NiAccumulator` base
/// destructor.
// Translation incomplete: 00b656e0 - the C++ exception frame (FS:[0] chain, unwind states 1..0xB for the members) is not translated.
pub fn fn_00b656e0(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    e.mem.set_u32(base, ACCUMULATOR_VTABLE);
    fn_00b65660(e, this);
    let queries = e.get(this, BSShaderAccumulator::ppOcclusionQuery);
    e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, queries]);
    bs_shader_accumulator_clear_all_bound_volumes(e, this);
    let bound = e.mem.u32(base + 0x13c);
    if bound != 0 {
        if e.call(IMPORT_INTERLOCKED_DECREMENT, &[bound + 4]).u32() == 0 {
            e.vcall(bound, 4, &[]);
        }
        e.mem.set_u32(base + 0x13c, 0);
    }
    for i in 0..3 {
        let slot = base + 0x48 + i * 0x14;
        let query = e.mem.u32(slot);
        if query != 0 {
            e.call(QUERY_DESTRUCTOR, &[query]);
            e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, query]);
            e.mem.set_u32(slot, 0);
        }
    }
    let instance = e.get(this, BSShaderAccumulator::pInstanceRenderer);
    if instance != 0 {
        e.call(INSTANCE_RENDERER_DESTRUCTOR, &[instance]);
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, instance]);
        e.set(this, BSShaderAccumulator::pInstanceRenderer, 0);
    }
    for slot in [0x174, 0x178] {
        let renderer = e.mem.u32(base + slot);
        if renderer != 0 {
            e.vcall(renderer, 0, &[1]);
        }
    }
    for slot in [0x17c, 0x180, 0x184, 0x188] {
        let block = e.mem.u32(base + slot);
        e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, block]);
    }
    release_pointer(e, base + 0x1a0);
    let map = base + 0x1b4;
    e.call(ROOM_MAP_REMOVE_ALL, &[map]);
    release_pointer(e, base + 0x198);
    e.call(IMPORT_DELETE_CRITICAL_SECTION, &[base + 0x200]);
    e.mem.set_u32(map, MAP_VTABLE);
    e.call(ROOM_MAP_REMOVE_ALL, &[map]);
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.call(ROOM_MAP_REMOVE_ALL, &[map]);
    let buckets = e.mem.u32(map + 8);
    e.call(FREE_BUCKETS, &[buckets]);
    e.call(RENDER_PASS_DESTRUCTOR, &[base + 0x1a4]);
    release_kept(e, base + 0x1a0);
    release_kept(e, base + 0x198);
    e.call(POINTER_LIST_REMOVE_ALL, &[base + 0x140]);
    release_kept(e, base + 0x13c);
    for cache in [0xec, 0x9c] {
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &[base + cache, 0x14, 4, PERSISTENT_LIST_DESTRUCTOR],
        );
    }
    e.call(POINTER_LIST_REMOVE_ALL, &[base + 0x90]);
    e.call(ACCUMULATOR_BASE_DESTRUCTOR, &[base]);
}

/// The list-node allocator (engine map `NiTPointerListBase<..>::AddHead`,
/// `0043a010`; thiscall on a count word, answers a node).
const LIST_NODE_ALLOCATE: u32 = 0x0043_a010;

/// The class record `GetRTTI` answers for `BSShaderAccumulator`.
const ACCUMULATOR_RTTI: u32 = 0x011f_a000;

// Translated from 00b65a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +8 (`GetRTTI`) of `BSShaderAccumulator`'s vtable (Ghidra made no
/// function here; read from the bytes): answers `011fa000`.
pub fn fn_00b65a80(_e: &mut Engine, _this: Ptr) -> u32 {
    ACCUMULATOR_RTTI
}

// Translated from 00b65a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, a register function the shader manager's
/// set-up (`00b57420`) stores (Ghidra made no function here; read from the
/// bytes; cdecl, five words, the third and fourth never read): with a
/// shader, appends `geometry` to the accumulator's item list (+0x0C head,
/// +0x10 tail, +0x14 count; node from `0043a010`). Answers true.
pub fn fn_00b65a90(
    e: &mut Engine,
    accumulator: u32,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
    shader: u32,
) -> bool {
    if shader == 0 {
        return true;
    }
    let node = e.call(LIST_NODE_ALLOCATE, &[accumulator + 0x14]).u32();
    e.mem.set_u32(node + 8, geometry);
    e.mem.set_u32(node, 0);
    let tail = e.mem.u32(accumulator + 0x10);
    e.mem.set_u32(node + 4, tail);
    let tail = e.mem.u32(accumulator + 0x10);
    let count = e.mem.u32(accumulator + 0x14).wrapping_add(1);
    if tail != 0 {
        e.mem.set_u32(tail, node);
        e.mem.set_u32(accumulator + 0x14, count);
        e.mem.set_u32(accumulator + 0x10, node);
    } else {
        e.mem.set_u32(accumulator + 0x14, count);
        e.mem.set_u32(accumulator + 0xc, node);
        e.mem.set_u32(accumulator + 0x10, node);
    }
    true
}

/// The byte that asks for the render-pass tree (`011f9fc1`), its maker
/// `BSShaderAccumulator::CreateRenderPassTree` (`00b63420`, not translated
/// here; thiscall) and its consumer `0066b0d0` (thiscall on `011fa008`, one
/// word).
const MAKE_PASS_TREE: u32 = 0x011f_9fc1;
const CREATE_RENDER_PASS_TREE: u32 = 0x00b6_3420;
const PASS_TREE_OWNER: u32 = 0x011f_a008;
const PASS_TREE_TAKE: u32 = 0x0066_b0d0;
/// The decal pass (`00b64e30`, `bsrenderstate_xenon.cpp`), alpha blending
/// (`00b97fa0`) and the clip planes (`00b98280`).
const DECAL_PASS: u32 = 0x00b6_4e30;
const SET_ALPHA_BLEND_ENABLE: u32 = 0x00b9_7fa0;
const SET_CLIP_PLANES: u32 = 0x00b9_8280;
/// The word `00b65ae0` sets to 1 around group 0.
const GROUP_ZERO_FLAG_011A9608: u32 = 0x011a_9608;

fn make_pass_tree(e: &mut Engine, accumulator: u32) {
    let tree = e.call(CREATE_RENDER_PASS_TREE, &[accumulator]).u32();
    e.call(PASS_TREE_TAKE, &[PASS_TREE_OWNER, tree]);
    e.mem.set_u8(MAKE_PASS_TREE, 0);
}

// Translated from 00b65ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word; the render-mode 0 pass of
/// `00b66520`): with a camera, render mode 8 renders the pass types
/// 6..8 and 0x1D3..0x1D6, mode 7 the types 0x10..0x13 and 0x1C8..0x1CB
/// (both then clear the active lists). Other modes render group 0x0A,
/// make the render-pass tree when asked (`011f9fc1`), render the types
/// 1..0x25B (split at 0x237 with clip planes off for the second part when
/// the byte +0x164 is set), group 0 with the word at `011a9608` set to 1,
/// the decal pass, the types 0x25C..0x287, sort the alphas and render alpha
/// group `(+0x167 == 0)`.
pub fn fn_00b65ae0(e: &mut Engine, accumulator: Ptr<BSShaderAccumulator>) {
    let base = accumulator.addr();
    if e.mem.u32(base + 8) == 0 {
        return;
    }
    let mode = e.get(accumulator, BSShaderAccumulator::eRenderMode);
    if mode == 8 {
        fn_00b63770(e, accumulator, 6, 8, 0, -1);
        fn_00b63770(e, accumulator, 0x1d3, 0x1d6, 0, -1);
        fn_00b65580(e, accumulator);
        return;
    }
    if mode == 7 {
        fn_00b63770(e, accumulator, 0x10, 0x13, 0, -1);
        fn_00b63770(e, accumulator, 0x1c8, 0x1cb, 0, -1);
        fn_00b65580(e, accumulator);
        return;
    }
    fn_00b639e0(e, accumulator, 0xa, 0);
    if e.mem.u8(MAKE_PASS_TREE) != 0 && e.get(accumulator, BSShaderAccumulator::eRenderMode) != 7 {
        make_pass_tree(e, base);
    }
    if e.mem.u8(base + 0x164) != 0 {
        fn_00b63770(e, accumulator, 1, 0x237, 0, -1);
        e.call(SET_CLIP_PLANES, &[0, 0, 0, 0, 0, 0, 0]);
        fn_00b63770(e, accumulator, 0x238, 0x25b, 0, -1);
        e.call(SET_CLIP_PLANES, &[1, 0, 0, 0, 0, 0, 0]);
    } else {
        fn_00b63770(e, accumulator, 1, 0x25b, 0, -1);
    }
    let saved = e.mem.u32(GROUP_ZERO_FLAG_011A9608);
    e.mem.set_u32(GROUP_ZERO_FLAG_011A9608, 1);
    fn_00b639e0(e, accumulator, 0, 0);
    e.mem.set_u32(GROUP_ZERO_FLAG_011A9608, saved);
    e.call(DECAL_PASS, &[base]);
    fn_00b63770(e, accumulator, 0x25c, 0x287, 0, -1);
    fn_00b63a50(e, accumulator);
    let group = u32::from(e.mem.u8(base + 0x167) == 0);
    bs_shader_accumulator_render_alpha_geometry(e, accumulator, group);
}

/// The water-reflection source target and its rendered copy
/// (`011f9438`, `011f943c`).
const REFLECTION_SOURCE: u32 = 0x011f_9438;
const REFLECTION_COPY: u32 = 0x011f_943c;
/// The texture manager's borrow (`00b6e110`; thiscall, five words) and the
/// image-space effect render (`00b97550`; thiscall, six words).
const TEXTURE_MANAGER_BORROW: u32 = 0x00b6_e110;
const IMAGE_SPACE_RENDER_EFFECT: u32 = 0x00b9_7550;

// Translated from 00b65c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word; the render-mode 0 finish of
/// `00b66570`): with the byte +0x38 set and a pass of 0x288..0x2CE listed
/// (`00b63120`), copies the reflection source (`011f9438`, when set) into a
/// borrowed target (`011f943c`, borrowed once: `00b6e110(renderer, 5, 0,
/// 0, 0)`) through image-space effect 0x22 and makes it the render target,
/// then passes the camera to the renderer (`004e9bb0`). Renders the types
/// 0x288..0x289, 0x28A..0x2AB and 0x2AC..0x2CE, returns the borrowed
/// target when +0x38 is set, renders group 9 (flag 1), alpha group
/// `(+0x167 != 0)`, turns alpha blending off twice, renders the types
/// 0x2CF..0x2D7 and group 1.
pub fn fn_00b65c60(e: &mut Engine, accumulator: Ptr<BSShaderAccumulator>) {
    let base = accumulator.addr();
    if e.mem.u8(base + 0x38) != 0 && fn_00b63120(e, accumulator, 0x288, 0x2ce) {
        if e.mem.u32(REFLECTION_SOURCE) != 0 {
            e.call(RENDERER_CLEAR, &[]);
            let mut copy = e.mem.u32(REFLECTION_COPY);
            if copy == 0 {
                let manager = e.mem.u32(TEXTURE_MANAGER);
                let renderer = e.mem.u32(RENDERER);
                copy = e
                    .call(TEXTURE_MANAGER_BORROW, &[manager, renderer, 5, 0, 0, 0])
                    .u32();
                e.mem.set_u32(REFLECTION_COPY, copy);
            }
            let source = e.mem.u32(REFLECTION_SOURCE);
            let renderer = e.mem.u32(RENDERER);
            let image_space = e.mem.u32(IMAGE_SPACE_MANAGER);
            e.call(
                IMAGE_SPACE_RENDER_EFFECT,
                &[image_space, 0x22, renderer, source, copy, 0, 1],
            );
            let source = e.mem.u32(REFLECTION_SOURCE);
            let rt = e.call(RENDER_TARGET_OF, &[source]).u32();
            e.call(SET_RENDER_TARGET, &[0, rt]);
        }
        let camera = e.mem.u32(base + 8);
        let renderer = e.mem.u32(RENDERER);
        e.call(HOLDER_METHOD_004E9BB0, &[renderer, camera]);
    }
    fn_00b63770(e, accumulator, 0x288, 0x289, 0, -1);
    fn_00b63770(e, accumulator, 0x28a, 0x2ab, 0, -1);
    fn_00b63770(e, accumulator, 0x2ac, 0x2ce, 0, -1);
    if e.mem.u8(base + 0x38) != 0 {
        let copy = e.mem.u32(REFLECTION_COPY);
        let manager = e.mem.u32(TEXTURE_MANAGER);
        e.call(TEXTURE_MANAGER_RETURN_RENDERED, &[manager, copy]);
        e.mem.set_u32(REFLECTION_COPY, 0);
    }
    fn_00b639e0(e, accumulator, 9, 1);
    let group = u32::from(e.mem.u8(base + 0x167) != 0);
    bs_shader_accumulator_render_alpha_geometry(e, accumulator, group);
    e.call(SET_ALPHA_BLEND_ENABLE, &[0, 0]);
    e.call(SET_ALPHA_BLEND_ENABLE, &[0, 0]);
    fn_00b63770(e, accumulator, 0x2cf, 0x2d7, 0, -1);
    fn_00b639e0(e, accumulator, 1, 0);
}

// Translated from 00b65dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word; a render-mode function of
/// the table at `011f9f40`): with a camera, makes the render-pass tree when
/// asked, clears +0x18 and runs the accumulator's slot +0xA4, renders the
/// types 1..0x287, the decal pass, sorts the alphas, renders both alpha
/// groups (`(+0x167 == 0)` first), turns alpha blending off, renders
/// 0x2CF..0x2D7 and clears the active lists.
pub fn fn_00b65dc0(e: &mut Engine, accumulator: Ptr<BSShaderAccumulator>) {
    let base = accumulator.addr();
    if e.mem.u32(base + 8) == 0 {
        return;
    }
    if e.mem.u8(MAKE_PASS_TREE) != 0 {
        make_pass_tree(e, base);
    }
    e.mem.set_u32(base + 0x18, 0);
    e.vcall(base, 0xa4, &[]);
    fn_00b63770(e, accumulator, 1, 0x287, 0, -1);
    e.call(DECAL_PASS, &[base]);
    fn_00b63a50(e, accumulator);
    let group = u32::from(e.mem.u8(base + 0x167) == 0);
    bs_shader_accumulator_render_alpha_geometry(e, accumulator, group);
    let group = u32::from(e.mem.u8(base + 0x167) != 0);
    bs_shader_accumulator_render_alpha_geometry(e, accumulator, group);
    e.call(SET_ALPHA_BLEND_ENABLE, &[0, 0]);
    fn_00b63770(e, accumulator, 0x2cf, 0x2d7, 0, -1);
    fn_00b65580(e, accumulator);
}

// Translated from 00b65e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word; a render-mode function):
/// with a camera, clears +0x18, runs the accumulator's slot +0xA4, renders
/// the sorted alpha items (`00b64f90`) and turns alpha blending off.
pub fn fn_00b65e80(e: &mut Engine, accumulator: Ptr<BSShaderAccumulator>) {
    let base = accumulator.addr();
    if e.mem.u32(base + 8) == 0 {
        return;
    }
    e.mem.set_u32(base + 0x18, 0);
    e.vcall(base, 0xa4, &[]);
    fn_00b64f90(e, accumulator);
    e.call(SET_ALPHA_BLEND_ENABLE, &[0, 0]);
}

// Translated from 00b66050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): removes the reference
/// volume whose +0x10 is `reference`: its query is destroyed and freed,
/// the volume freed, and it is taken out of the `ReferenceVolumeList`
/// (`00e7db70` with the argument's own slot, which then holds the volume).
pub fn fn_00b66050(e: &mut Engine, this: Ptr<BSShaderAccumulator>, reference: u32) {
    let base = this.addr();
    let mut node = e.mem.u32(base + 0x140);
    if node == 0 {
        return;
    }
    let volume = loop {
        let item = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if item != 0 && e.mem.u32(item + 0x10) == reference {
            break item;
        }
        if node == 0 {
            return;
        }
    };
    if e.mem.u32(volume + 0x14) != 0 {
        drop_volume_query(e, volume);
    }
    e.call(MEMORY_DEALLOCATE, &[MEMORY_MANAGER, volume]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), volume);
        e.call(VOLUME_LIST_REMOVE, &[base + 0x140, slot.addr()]);
    });
}

use crate::types::NiTListItem;

/// The calls `CreateRenderPassTree` makes: the `NiNode` constructor (`00a5ecb0`, `thiscall`,
/// one word), the fixed-string maker (`00a5b690`, `cdecl`, the text), `SetName` (`00a5b950`,
/// `thiscall`, the address of the handle) and the update with a local update record
/// (`00a59c60`, `thiscall`, its address); the name (`"RenderPasses"`, `010adfac`) and the shared empty handle word (`0109b220`).
const NI_NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const PASS_TREE_NAME_010ADFAC: u32 = 0x010a_dfac;
const FIXED_STRING_EMPTY_0109B220: u32 = 0x0109_b220;
/// The map lookup `00b63170` (`GetAt`, `thiscall`: key, the address of the value) and store
/// `00a5d320` (`SetAt`, `thiscall`: key, value) on the room map.
const ROOM_MAP_GET_AT: u32 = 0x00b6_3170;
const ROOM_MAP_SET_AT: u32 = 0x00a5_d320;
/// The generic register function `00b64020` the filtering ones jump to (Ghidra made no
/// function there; `cdecl`, the five register words).
const REGISTER_00B64020: u32 = 0x00b6_4020;
/// `50.0` (`double`, `0101e2c0`): the value `00b64440` compares the geometry's float with.
const LIMIT_0101E2C0: u32 = 0x0101_e2c0;
/// The pass state call `00b97fa0` (`cdecl`, two words) and the counter `00b64530` lowers
/// (`011ff9f0`).
const PASS_STATE_00B97FA0: u32 = 0x00b9_7fa0;
const COUNTER_011FF9F0: u32 = 0x011f_f9f0;
/// The `ImageSpaceTexture` set-up `00b63c60` makes: 0x10 bytes, through the array store
/// `00ba3aa0` (`thiscall`: index, the address of the element).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
/// The byte `00b651e0` copies (`011ad80e`) and the two words it points the shaders at
/// (`011f91e0` the pass, `011f91e4` its enum).
const FLAG_011AD80E: u32 = 0x011a_d80e;
const CURRENT_PASS_011F91E0: u32 = 0x011f_91e0;
const CURRENT_PASS_ENUM_011F91E4: u32 = 0x011f_91e4;
/// The list node allocation (`0043a010`, `thiscall` on the list's count, no words).
const LIST_NEW_NODE: u32 = 0x0043_a010;

// Translated from 00b63420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b63420 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `BSShaderAccumulator::CreateRenderPassTree` (engine map; the body matches), no
/// arguments: a new 0xac-byte `NiNode` (`00aa13e0`, `00a5ecb0` with 0; null when the
/// allocation fails, and the exe goes on with null) named with the fixed string made from
/// `010adfac` (the handle released afterwards unless it is the shared word at `0109b220`:
/// `InterlockedDecrement` on the handle minus 8), then updated (`00a59c60`) with a local
/// record of a `float` 0.0 and five zero bytes. Answers the node.
///
/// The record is 12 bytes on the exe's stack and its last three bytes are never written
/// there; the translation's are zero.
pub fn bs_shader_accumulator_create_render_pass_tree(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NI_NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, handle| {
        let name = e
            .call(FIXED_STRING_MAKE, &args![PASS_TREE_NAME_010ADFAC])
            .u32();
        e.mem.set_u32(handle.addr(), name);
        e.call(SET_NAME, &args![node, handle]);
        let name = e.mem.u32(handle.addr());
        if name != e.mem.u32(FIXED_STRING_EMPTY_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
    });
    e.with_stack(0xc, |e, record| {
        e.mem.set_u32(record.addr(), x87_store32(0.0));
        for k in 4..9 {
            e.mem.set_u8(record.addr() + k, 0);
        }
        e.call(UPDATE_00A59C60, &args![node, record]);
    });
    node
}

// Translated from 00b63620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` without words: for each of the three occlusion
/// query records at `+0x48` (0x14 bytes apart) that holds a query, reads its result
/// (`00c4edf0` with the address of a zeroed local word and 1), destroys (`00c4ed40`) and
/// frees the query and clears the record's word and its byte `+4`; then clears
/// `bWaitingForSunQueury`.
pub fn fn_00b63620(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let mut record = this.addr().wrapping_add(0x48);
    for _ in 0..3 {
        let query = e.mem.u32(record);
        if query != 0 {
            e.with_stack(4, |e, result| {
                e.call(QUERY_RESULT_00C4EDF0, &args![query, result, 1u32]);
            });
            let query = e.mem.u32(record);
            if query != 0 {
                e.call(QUERY_DESTRUCTOR, &args![query]);
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, query]);
            }
            e.mem.set_u32(record, 0);
            e.mem.set_u8(record.wrapping_add(4), 0);
        }
        record = record.wrapping_add(0x14);
    }
    e.set(this, BSShaderAccumulator::bWaitingForSunQueury, 0);
}

// Translated from 00b63b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word (a key): inside the critical section
/// at `+0x200`, looks the key up in the room map (`+0x1b4`, `00b63170`, its value written
/// into the argument's own stack word) and, when it is not there, stores it with the value 1
/// (`00a5d320`). Answers whether the key was new.
pub fn fn_00b63b10(e: &mut Engine, this: Ptr<BSShaderAccumulator>, key: u32) -> bool {
    let base = this.addr();
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![base + 0x200]);
    let map = base.wrapping_add(0x1b4);
    let found = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), key);
        e.call(ROOM_MAP_GET_AT, &args![map, key, slot]).u8()
    });
    if found == 0 {
        e.call(ROOM_MAP_SET_AT, &args![map, key, 1u32]);
    }
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![base + 0x200]);
    found == 0
}

// Translated from 00b63bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here): slot 3 of the room map's base vtable `010adfbc`
/// (`thiscall`, `this` not read, three words): stores the second word at `+4` of the first
/// (a map item) and the third's low byte at `+8`.
pub fn fn_00b63bd0(e: &mut Engine, _this: u32, target: u32, value: u32, flag: u8) {
    e.mem.set_u32(target.wrapping_add(4), value);
    e.mem.set_u8(target.wrapping_add(8), flag);
}

// Translated from 00b63bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word (the address of a 16-bit value): the
/// list of 16-bit values at `this` (each 8-byte node: the value, then the next node). A zero
/// value adds nothing. Otherwise the last node is found; when its value is zero it takes the
/// new one, else a new node (8 bytes from the memory manager, the value and no next) is
/// linked after it (a failed allocation links null).
pub fn fn_00b63bf0(e: &mut Engine, this: u32, value: u32) {
    let v = e.mem.u16(value);
    if v == 0 {
        return;
    }
    let mut last = this;
    while e.mem.u32(last.wrapping_add(4)) != 0 {
        last = e.mem.u32(last.wrapping_add(4));
    }
    if e.mem.u16(last) == 0 {
        e.mem.set_u16(last, v);
        return;
    }
    let node = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
    if node != 0 {
        let v = e.mem.u16(value);
        e.mem.set_u16(node, v);
        e.mem.set_u32(node.wrapping_add(4), 0);
    }
    e.mem.set_u32(last.wrapping_add(4), node);
}

// Translated from 00b63c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` without words (the counterpart of
/// [`image_space_shader_volumetric_fog_return_volumetric_fog_texture`]): when element 2 of
/// the array at `+0x94` (buffer at `+0x98`) is empty, a new zeroed 0x10-byte image-space
/// texture is stored there (`00ba3aa0`; null when the allocation fails); when it has no
/// rendered texture (`00ba3770`), it gets one borrowed from the texture manager at
/// `011f91a8` (`00b6e110` with the renderer at `011f4748`, 0x20, 0, 0, 0; set through
/// `00ba37a0`). Answers the element's rendered texture (`00ba3770`, a tail jump).
pub fn fn_00b63c60(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let holder = e.mem.u32(base.wrapping_add(0x98));
    if e.mem.u32(holder.wrapping_add(8)) == 0 {
        let texture = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        if texture != 0 {
            e.mem.set_u8(texture, 0);
            e.mem.set_u8(texture + 1, 0);
            e.mem.set_u8(texture + 2, 0);
            e.mem.set_u32(texture + 4, 0);
            e.mem.set_u32(texture + 8, 0);
            e.mem.set_u32(texture + 0xc, 0);
        }
        e.with_stack(4, |e, element| {
            e.mem.set_u32(element.addr(), texture);
            e.call(
                ARRAY_SET_AT_00BA3AA0,
                &args![base.wrapping_add(0x94), 2u32, element],
            );
        });
    }
    let holder = e.mem.u32(base.wrapping_add(0x98));
    let texture = e.mem.u32(holder.wrapping_add(8));
    if e.call(IMAGE_SPACE_TEXTURE_GET_RENDERED, &args![texture])
        .u32()
        == 0
    {
        let renderer = e.mem.u32(RENDER_HOLDER);
        let manager = e.mem.u32(TEXTURE_MANAGER);
        let holder = e.mem.u32(base.wrapping_add(0x98));
        let borrowed = e
            .call(
                TEXTURE_MANAGER_BORROW,
                &args![manager, renderer, 0x20u32, 0u32, 0u32, 0u32],
            )
            .u32();
        let texture = e.mem.u32(holder.wrapping_add(8));
        e.call(IMAGE_SPACE_TEXTURE_SET, &args![texture, borrowed]);
    }
    let holder = e.mem.u32(base.wrapping_add(0x98));
    let texture = e.mem.u32(holder.wrapping_add(8));
    e.call(IMAGE_SPACE_TEXTURE_GET_RENDERED, &args![texture])
        .u32()
}

// Translated from 00b63d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` without words: the room map's base destructor
/// (`NiTMapBase`): puts the base vtable `010adfbc` back, empties the map (`00b601a0`) and
/// frees its buckets (`+8`, `00aa10f0`).
pub fn fn_00b63d60(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    let base = this.addr();
    e.mem.set_u32(base, MAP_BASE_VTABLE);
    e.call(ROOM_MAP_REMOVE_ALL, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_BUCKETS, &args![buckets]);
}

// Translated from 00b63d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b63d80 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` without words: the room map's destructor
/// (`NiTPointerMap`): the map vtable `010adfd8`, emptied (`00b601a0`), then the base part as
/// [`fn_00b63d60`] does it (base vtable, emptied again, buckets freed).
pub fn fn_00b63d80(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    let base = this.addr();
    e.mem.set_u32(base, MAP_VTABLE);
    e.call(ROOM_MAP_REMOVE_ALL, &args![base]);
    e.mem.set_u32(base, MAP_BASE_VTABLE);
    e.call(ROOM_MAP_REMOVE_ALL, &args![base]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.call(FREE_BUCKETS, &args![buckets]);
}

// Translated from 00b63df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; slot 6 of the room map's vtable `010adfd8` (`thiscall` with one
/// word, a map item; `this` is not read):
/// clears the node's byte `+8`, enters the node pool's lock, clears its `+4`, pushes it on
/// the pool's free list (`011c5f58`) and leaves the lock (the count at `011c5f84` drops by
/// one; at zero the owner word `011c5f80` is cleared).
pub fn fn_00b63df0(e: &mut Engine, _this: u32, node: u32) {
    e.mem.set_u8(node.wrapping_add(8), 0);
    e.call(LOCK_ENTER, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(node.wrapping_add(4), 0);
    let free = e.mem.u32(NODE_POOL_FREE_LIST);
    e.mem.set_u32(node, free);
    let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
    e.mem.set_u32(NODE_POOL_FREE_LIST, node);
    if count == 0 {
        e.mem.set_u32(NODE_POOL_LOCK, 0);
    }
}

// Translated from 00b63e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the deleting destructor of the room map's base: the body of
/// [`fn_00b63d60`], then the map freed through the memory manager when bit 0 of the flags is
/// set. Answers `this`.
pub fn fn_00b63e30(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00b63d60(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b63e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the deleting destructor of the room map: [`fn_00b63d80`], then
/// the map freed through the memory manager when bit 0 of the flags is set. Answers `this`.
pub fn fn_00b63e70(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00b63d80(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// Hands each pass of a render-pass array to the running thread's batch renderer
/// (`pBatchRenderer[word +0x2bc of the thread block]`, slot +0x8c with the pass and its
/// enum), skipping passes with enum 0 when `skip_zero` is set.
fn register_passes(e: &mut Engine, accumulator: u32, list: u32, skip_zero: bool) {
    let mut index = 0u32;
    while index < e.mem.u32(list.wrapping_add(0x10)) {
        let base = e.mem.u32(list.wrapping_add(4));
        let pass = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        let pass_enum = u32::from(e.mem.u16(pass.wrapping_add(4)));
        if pass_enum != 0 || !skip_zero {
            let tls = e.tls();
            let thread = e.mem.u32(tls + TLS_WORD_2BC);
            let renderer = batch_renderer(e, accumulator, thread);
            e.vcall(renderer, 0x8c, &args![pass, pass_enum]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00b63f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the register function of render mode 13 (`011f9fb4`):
/// `cdecl` with the five register words (accumulator, geometry, property state, property,
/// shader; the third and fifth are not read). Asks the property for its render passes
/// (slot +0xa0 with the geometry, the shader flags at `011f91d8`, the address of a zeroed
/// local word, `eRenderMode`, the accumulator and 1) and hands each pass with a non-zero
/// enum to the thread's batch renderer. Answers true.
pub fn fn_00b63f90(
    e: &mut Engine,
    accumulator: Ptr<BSShaderAccumulator>,
    geometry: u32,
    _unused_3: u32,
    property: u32,
    _unused_5: u32,
) -> bool {
    let mode = e.get(accumulator, BSShaderAccumulator::eRenderMode);
    let flags = e.mem.u32(SHADER_FLAGS_011F91D8);
    let list = e.with_stack(4, |e, local| {
        e.vcall(
            property,
            0xa0,
            &args![geometry, flags, local, mode, accumulator, 1u32],
        )
        .u32()
    });
    if list != 0 && e.mem.u32(list.wrapping_add(0x10)) != 0 {
        register_passes(e, accumulator.addr(), list, true);
    }
    true
}

/// The shader flags word the render-pass queries take (`011f91d8`).
const SHADER_FLAGS_011F91D8: u32 = 0x011f_91d8;

/// The batch renderer's pass registration `00b9b3b0` (`thiscall`: pass, group, flag) and
/// `00b9b3d0` (`thiscall`: pass, flag), and the pass render `00b63320` (`cdecl`: pass, enum).
const BATCH_REGISTER_00B9B3B0: u32 = 0x00b9_b3b0;
const BATCH_REGISTER_00B9B3D0: u32 = 0x00b9_b3d0;
const RENDER_PASS_00B63320: u32 = 0x00b6_3320;
/// The object `00b64020` reads the `float` `+8` of when a pass's object has no `+0x20`
/// (`011f4288`, the `MOV EAX, 0x11f4288` immediate).
const DEFAULT_OBJECT_011F4288: u32 = 0x011f_4288;
/// The switch of `00b64020` on the pass's byte `+6` less one: the 14 code addresses of its
/// jump table at `00b643fc` (read from the exe), and the default case (above 0xd).
const JUMP_TABLE_00B643FC: [u32; 14] = [
    0x00b6_41d8,
    0x00b6_4234,
    0x00b6_424f,
    0x00b6_426a,
    0x00b6_4290,
    0x00b6_429e,
    0x00b6_42b6,
    0x00b6_430d,
    0x00b6_430d,
    0x00b6_42ce,
    0x00b6_430d,
    0x00b6_430d,
    0x00b6_430d,
    0x00b6_42d5,
];
const SWITCH_DEFAULT_00B642ED: u32 = 0x00b6_42ed;

/// `pBatchRenderer[word +0x2bc of the thread block]` (no modulo, as the exe indexes it at
/// each call of [`fn_00b64020`] but the first).
fn thread_renderer_unmodded(e: &mut Engine, accumulator: u32) -> u32 {
    let tls = e.tls();
    let thread = e.mem.u32(tls + TLS_WORD_2BC);
    batch_renderer(e, accumulator, thread)
}

// Translated from 00b64020 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; 00b64020 to the `RET` at 00b643f8, with the
/// jump table at 00b643fc): the generic register function of the render modes (`011f9f80`,
/// `011f9f3c`) that the filtering ones jump to. `cdecl` with the five register words
/// (accumulator, geometry, property state (not read), property, shader); ECX is not read.
/// Answers true (`MOV AL, 1`) on every path.
///
/// Answers at once when the accumulator's byte `+0x164` is set and the shader's word `+0x68`
/// is 0xd. With `bDoDepthPasses` (`+0x3a`) set, asks the property (slot +0xac with the
/// geometry) for a depth pass and registers it with the thread's batch renderer
/// (`00b9b3b0` with group 0xc in first person (`+0x85`), else 0xb, and 0). Then sets bit 9
/// (`0x200`) of the property's flags `+0x20` to whether the batch renderer
/// `[thread % uiBatchRendererCount]` has `+0x34 > 0` and its byte `+0x38` set (a change
/// clears the property's `+0x38` when the bit was clear before a set or set before a clear,
/// and the word `+0x10` of the property's `+0x3c` object when there is one). Asks the
/// property for its render passes (slot +0xa0: geometry, the shader flags `011f91d8`, the
/// address of a zeroed local, `eRenderMode`, the accumulator, 1); sets bit 6 (`0x40`) of
/// `+0x24` to `b1stPerson` the same way. Each pass with a non-zero enum (`+4`) is
/// registered by its byte `+6` through the jump table: 1 `00b9b3d0` with 1 when the
/// accumulator's byte `+0x166` is set and its `int` `+0x168` is above the `float` `+8` of
/// the pass object's `+0x20` (or of `011f4288`), else 0; 2, 3, 6, 7, 10, 14 `00b9b3b0`
/// with groups 0xa, 1, 6, 8, 0, 9; 4 and 5 (only with `bRenderDecals`) groups 2 (flag 1)
/// and 3; 8, 9, 11 to 13 nothing; any other byte the renderer's slot +0x8c with the pass and
/// its enum. Without passes (null or none) it answers there. Last, with `b1stPerson` and
/// `bAccumulate` (`+0x84`) both set, takes the property's slot +0xa8 list and for each pass
/// either hands it to the renderer's slot +0x8c with 0 (`bAccumulate` still set) or renders
/// it now: `BSShaderManager::GetShader(1)`, `00b7a870(enum, 0)`, slots +0x24 of the shader's
/// `+0x34` and `+0x30` (when there is a shader), `00b63320(pass, enum)`. The exe keeps the
/// pass index in the caller's first argument word; it is a local here.
pub fn fn_00b64020(
    e: &mut Engine,
    accumulator: u32,
    geometry: u32,
    _unused_3: u32,
    property: u32,
    shader: u32,
) -> bool {
    if e.mem.u8(accumulator.wrapping_add(0x164)) != 0 && e.mem.u32(shader.wrapping_add(0x68)) == 0xd
    {
        return true;
    }
    if e.mem.u8(accumulator.wrapping_add(0x3a)) != 0 {
        let depth = e.vcall(property, 0xac, &args![geometry]).u32();
        if depth != 0 {
            let group = if e.mem.u8(accumulator.wrapping_add(0x85)) != 0 {
                0xcu32
            } else {
                0xb
            };
            let renderer = thread_renderer_unmodded(e, accumulator);
            e.call(
                BATCH_REGISTER_00B9B3B0,
                &args![renderer, depth, group, 0u32],
            );
        }
    }
    let tls = e.tls();
    let thread = e.mem.u32(tls + TLS_WORD_2BC);
    let current = batch_renderer(e, accumulator, thread % batch_count(e, accumulator));
    // The exe tests `+0x34 > 0 && +0x38` again inside the change; nothing writes in between.
    let active =
        e.mem.i32(current.wrapping_add(0x34)) > 0 && e.mem.u8(current.wrapping_add(0x38)) != 0;
    let flagged = e.mem.u32(property.wrapping_add(0x20)) & 0x200 != 0;
    if active != flagged {
        let flags = e.mem.u32(property.wrapping_add(0x20));
        if active {
            if flags & 0x200 == 0 {
                e.mem.set_u32(property.wrapping_add(0x38), 0);
            }
            let flags = e.mem.u32(property.wrapping_add(0x20));
            e.mem.set_u32(property.wrapping_add(0x20), flags | 0x200);
        } else {
            if flags & 0x200 != 0 {
                e.mem.set_u32(property.wrapping_add(0x38), 0);
            }
            let flags = e.mem.u32(property.wrapping_add(0x20));
            e.mem
                .set_u32(property.wrapping_add(0x20), flags & 0xffff_fdff);
        }
        let object = e.mem.u32(property.wrapping_add(0x3c));
        if object != 0 {
            e.mem.set_u32(object.wrapping_add(0x10), 0);
        }
    }
    let mode = e.mem.u32(accumulator.wrapping_add(0x19c));
    let flags = e.mem.u32(SHADER_FLAGS_011F91D8);
    let list = e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        e.vcall(
            property,
            0xa0,
            &args![geometry, flags, local, mode, accumulator, 1u32],
        )
        .u32()
    });
    let first_person = e.mem.u8(accumulator.wrapping_add(0x85)) != 0;
    let bits = e.mem.u8(property.wrapping_add(0x24));
    if first_person {
        if bits & 0x40 == 0 {
            e.mem.set_u32(property.wrapping_add(0x38), 0);
        }
        let word = e.mem.u32(property.wrapping_add(0x24));
        e.mem.set_u32(property.wrapping_add(0x24), word | 0x40);
    } else {
        if bits & 0x40 != 0 {
            e.mem.set_u32(property.wrapping_add(0x38), 0);
        }
        let word = e.mem.u32(property.wrapping_add(0x24));
        e.mem
            .set_u32(property.wrapping_add(0x24), word & 0xffff_ffbf);
    }
    if list == 0 || e.mem.u32(list.wrapping_add(0x10)) == 0 {
        return true;
    }
    let mut index = 0u32;
    while index < e.mem.u32(list.wrapping_add(0x10)) {
        let base = e.mem.u32(list.wrapping_add(4));
        let pass = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        let pass_enum = e.mem.u16(pass.wrapping_add(4));
        if pass_enum != 0 {
            register_by_type(e, accumulator, pass, pass_enum);
        }
        index = index.wrapping_add(1);
    }
    if e.mem.u8(accumulator.wrapping_add(0x85)) == 0
        || e.mem.u8(accumulator.wrapping_add(0x84)) == 0
    {
        return true;
    }
    let late = e.vcall(property, 0xa8, &[]).u32();
    if late == 0 || e.mem.u32(late.wrapping_add(0x10)) == 0 {
        return true;
    }
    let mut index = 0u32;
    loop {
        let accumulate = e.mem.u8(accumulator.wrapping_add(0x84)) != 0;
        let base = e.mem.u32(late.wrapping_add(4));
        let pass = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if accumulate {
            let renderer = thread_renderer_unmodded(e, accumulator);
            e.vcall(renderer, 0x8c, &args![pass, 0u32]);
        } else {
            let shader = e.call(GET_SHADER, &args![1u32]).u32();
            let pass_enum = u32::from(e.mem.u16(pass.wrapping_add(4)));
            e.call(SHADER_SETUP_00B7A870, &args![pass_enum, 0u32]);
            if shader != 0 {
                let first = e.mem.u32(shader.wrapping_add(0x34));
                e.vcall(first, 0x24, &[]);
                let second = e.mem.u32(shader.wrapping_add(0x30));
                e.vcall(second, 0x24, &[]);
            }
            let pass_enum = u32::from(e.mem.u16(pass.wrapping_add(4)));
            e.call(RENDER_PASS_00B63320, &args![pass, pass_enum]);
        }
        index = index.wrapping_add(1);
        if index >= e.mem.u32(late.wrapping_add(0x10)) {
            return true;
        }
    }
}

/// The switch of [`fn_00b64020`] for one pass with a non-zero enum (00b641c3 to 00b6430d).
fn register_by_type(e: &mut Engine, accumulator: u32, pass: u32, pass_enum: u16) {
    let case = u32::from(e.mem.u8(pass.wrapping_add(6))).wrapping_sub(1);
    let target = if case > 0xd {
        SWITCH_DEFAULT_00B642ED
    } else {
        JUMP_TABLE_00B643FC[case as usize]
    };
    let group = |e: &mut Engine, group: u32, flag: u32| {
        let renderer = thread_renderer_unmodded(e, accumulator);
        e.call(BATCH_REGISTER_00B9B3B0, &args![renderer, pass, group, flag]);
    };
    match target {
        0x00b6_41d8 => {
            let mut flag = 0u32;
            if e.mem.u8(accumulator.wrapping_add(0x166)) != 0 {
                let object = e.mem.u32(pass);
                let mut source = e.mem.u32(object.wrapping_add(0x20));
                if source == 0 {
                    source = DEFAULT_OBJECT_011F4288;
                }
                let value = crate::x87::X87Ext::load32(e.mem.u32(source.wrapping_add(8)));
                let count = e.mem.i32(accumulator.wrapping_add(0x168));
                let limit = crate::x87::X87Ext::load64(f64::from(count));
                // FILD; FCOMPP (ST0 the int); TEST AH, 0x41; JNZ: above and ordered.
                let status = crate::x87::x87_fcom(limit, value);
                if status & (crate::x87::X87_STATUS_C0 | crate::x87::X87_STATUS_C3) == 0 {
                    flag = 1;
                }
            }
            let renderer = thread_renderer_unmodded(e, accumulator);
            e.call(BATCH_REGISTER_00B9B3D0, &args![renderer, pass, flag]);
        }
        0x00b6_4234 => group(e, 0xa, 0),
        0x00b6_424f => group(e, 1, 0),
        0x00b6_426a => {
            if e.mem.u8(accumulator.wrapping_add(0x16c)) != 0 {
                group(e, 2, 1);
            }
        }
        0x00b6_4290 => {
            if e.mem.u8(accumulator.wrapping_add(0x16c)) != 0 {
                group(e, 3, 0);
            }
        }
        0x00b6_429e => group(e, 6, 0),
        0x00b6_42b6 => group(e, 8, 0),
        0x00b6_42ce => group(e, 0, 0),
        0x00b6_42d5 => group(e, 9, 0),
        SWITCH_DEFAULT_00B642ED => {
            let renderer = thread_renderer_unmodded(e, accumulator);
            e.vcall(renderer, 0x8c, &args![pass, u32::from(pass_enum)]);
        }
        // 0x00b6_430d: the loop's continue.
        _ => {}
    }
}

// Translated from 00b64440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here); the register function of render mode 9
/// (`011f9fa4`), `cdecl` with the five register words. Answers true without registering
/// when the `float` at `+0x1c` of the geometry's object at `+0xb8` is below 50.0, when the
/// shader's word `+0x68` is 0x16, or when bit 18 (`0x40000`) of the property's second flag
/// word is clear; otherwise jumps to the generic register function `00b64020` with the same
/// words.
pub fn fn_00b64440(
    e: &mut Engine,
    accumulator: u32,
    geometry: u32,
    state: u32,
    property: u32,
    shader: u32,
) -> bool {
    let object = e.mem.u32(geometry.wrapping_add(0xb8));
    let value = x87_m32(e.mem.u32(object.wrapping_add(0x1c)));
    let limit: f64 = e.global(LIMIT_0101E2C0);
    if value < limit {
        return true;
    }
    if e.mem.u32(shader.wrapping_add(0x68)) == 0x16 {
        return true;
    }
    if e.mem.u32(property.wrapping_add(0x24)) & 0x40000 == 0 {
        return true;
    }
    e.call(
        REGISTER_00B64020,
        &args![accumulator, geometry, state, property, shader],
    )
    .bool()
}

// Translated from 00b64490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here); the register function of render mode 8
/// (`011f9fa0`), `cdecl` with the five register words: answers true when the shader's word
/// `+0x68` is 0x16, else jumps to `00b64020` with the same words.
pub fn fn_00b64490(
    e: &mut Engine,
    accumulator: u32,
    geometry: u32,
    state: u32,
    property: u32,
    shader: u32,
) -> bool {
    if e.mem.u32(shader.wrapping_add(0x68)) == 0x16 {
        return true;
    }
    e.call(
        REGISTER_00B64020,
        &args![accumulator, geometry, state, property, shader],
    )
    .bool()
}

// Translated from 00b644b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here); the register function of render mode 14
/// (`011f9fb8`), `cdecl` with the five register words (the third and fifth not read). With
/// the accumulator's byte `+0x165` set, asks the property for a pass list (slot +0xb8 with
/// the geometry) and hands each pass to the thread's batch renderer (slot +0x8c with the
/// pass and its enum). Answers true.
pub fn fn_00b644b0(
    e: &mut Engine,
    accumulator: u32,
    geometry: u32,
    _unused_3: u32,
    property: u32,
    _unused_5: u32,
) -> bool {
    if e.mem.u8(accumulator.wrapping_add(0x165)) == 0 {
        return true;
    }
    let list = e.vcall(property, 0xb8, &args![geometry]).u32();
    if list != 0 && e.mem.u32(list.wrapping_add(0x10)) != 0 {
        register_passes(e, accumulator, list, false);
    }
    true
}

// Translated from 00b64530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the function at `011f9f78`: `cdecl`, one word (an accumulator).
/// When its `+8` is set: `00b97fa0(0, 1)`, the pass types 0x2d8 to 0x2ef rendered
/// ([`fn_00b63770`] with 0 and -1), the word at `011ff9f0` lowered by one and
/// `00b97fa0(0, 0)`.
pub fn fn_00b64530(e: &mut Engine, accumulator: Ptr<BSShaderAccumulator>) {
    if e.mem.u32(accumulator.addr().wrapping_add(8)) == 0 {
        return;
    }
    e.call(PASS_STATE_00B97FA0, &args![0u32, 1u32]);
    fn_00b63770(e, accumulator, 0x2d8, 0x2ef, 0, -1);
    let v = e.mem.u32(COUNTER_011FF9F0).wrapping_sub(1);
    e.mem.set_u32(COUNTER_011FF9F0, v);
    e.call(PASS_STATE_00B97FA0, &args![0u32, 0u32]);
}

/// The object `00b651e0` takes the fog texture from: `+0xa8` of the object at `+8` of the
/// image-space manager, less 0x6c (null stays null).
fn fog_owner(e: &Engine) -> u32 {
    let manager = e.mem.u32(IMAGE_SPACE_MANAGER);
    let inner = e.mem.u32(manager.wrapping_add(8));
    let field = e.mem.u32(inner.wrapping_add(0xa8));
    if field != 0 {
        field.wrapping_sub(0x6c)
    } else {
        0
    }
}

// Translated from 00b651e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b651e0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` with two words (a camera and a render target):
/// renders the volumetric fog geometry. Nothing happens unless the word at `+0x98` is
/// non-zero (the answer is whether it is). Then: the renderer at `011f4748` cleared
/// (`00b6b790`) when its `+0x200` is set; the fog texture fetched ([`fn_00b63c60`] on the
/// object [`fog_owner`] finds); a local render pass made (`00ba8ec0` with 0, 0, 1, 0, 0);
/// the camera put in `+8` (the old one kept); the background colour saved (renderer slot
/// +0xb4) and set to (0, 0, 0, 1) (slot +0xac); `+0x18` pointed at the fog list `+0x90` and
/// the accumulator's slot +0xa4 called; the pass enum 0x25b. For each geometry of the list
/// whose shader property (`+0xa8`) is of type 0x16: the fog texture's target set (`00b6b8d0`
/// with 3), the camera's columns copied and the renderer's camera set-up made; the shaders
/// pointed at the pass (`011f91e0`, `011f91e4`); the geometry rendered (slot +0xdc with the
/// renderer) once with the current enum when the byte at `011ad80e` was set, else twice,
/// with enum 0x25a and then 0x259; the renderer cleared, the target's target set (with 0),
/// the property's four words `+0x60` to `+0x6c` copied to `+0x58` of the image-space
/// manager's effect (`+0x34` of its `+8`, byte `+5` set) and effect 0xd rendered (`00b97550`
/// with the renderer, the target twice, 0 and 1). Afterwards the list is emptied
/// (`00b71300`), the background restored, the camera put back, the fog texture returned
/// ([`image_space_shader_volumetric_fog_return_volumetric_fog_texture`]) and the pass
/// destroyed (`00ba8cd0`).
pub fn fn_00b651e0(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    camera: u32,
    target: u32,
) -> u32 {
    let base = this.addr();
    let count = e.mem.u32(base.wrapping_add(0x98));
    if count == 0 {
        return 0;
    }
    if e.mem.u32(e.mem.u32(RENDER_HOLDER).wrapping_add(0x200)) != 0 {
        e.call(RENDERER_CLEAR, &args![]);
    }
    let renderer = e.mem.u32(RENDER_HOLDER);
    e.with_stack(0x70, |e, frame| {
        let f = frame.addr();
        let pass = f + 0x50;
        let saved = f + 0x60;
        let black = f + 0x40;
        let zero = x87_store32(0.0);
        for k in 0..4 {
            e.mem.set_u32(saved + k * 4, zero);
        }
        let owner = fog_owner(e);
        let fog = fn_00b63c60(e, Ptr::new(owner));
        e.call(
            RENDER_PASS_CONSTRUCTOR,
            &args![pass, 0u32, 0u32, 1u32, 0u32, 0u32],
        );
        let old_camera = e.mem.u32(base.wrapping_add(8));
        let skip_second = e.mem.u8(FLAG_011AD80E);
        e.mem.set_u32(base.wrapping_add(8), camera);
        e.vcall(renderer, 0xb4, &args![saved]);
        e.mem.set_u32(black, zero);
        e.mem.set_u32(black + 4, zero);
        e.mem.set_u32(black + 8, zero);
        e.mem.set_u32(black + 0xc, x87_store32(1.0));
        e.vcall(renderer, 0xac, &args![black]);
        e.mem
            .set_u32(base.wrapping_add(0x18), base.wrapping_add(0x90));
        e.vcall(base, 0xa4, &args![]);
        e.mem.set_u16(pass + 4, 0x25b);
        let mut node = e.mem.u32(base.wrapping_add(0x90));
        while node != 0 {
            let next = e.mem.u32(node);
            let geometry = e.mem.u32(node.wrapping_add(8));
            node = next;
            let property = e.mem.u32(geometry.wrapping_add(0xa8));
            if property == 0 || e.mem.u32(property.wrapping_add(0x1c)) != 0x16 {
                continue;
            }
            let fog_target = e.call(RENDER_TARGET_OF, &args![fog]).u32();
            e.call(SET_RENDER_TARGET, &args![3u32, fog_target]);
            let cam = e.mem.u32(base.wrapping_add(8));
            camera_columns(e, cam, f + 0x40, f + 0x34, f + 0x28);
            holder_camera_setup(e, renderer, cam, [f + 0x40, f + 0x34, f + 0x28], cam + 0xdc);
            e.mem.set_u32(CURRENT_PASS_011F91E0, pass);
            if skip_second != 0 {
                let pass_enum = u32::from(e.mem.u16(pass + 4));
                e.mem.set_u32(CURRENT_PASS_ENUM_011F91E4, pass_enum);
            } else {
                e.mem.set_u16(pass + 4, 0x25a);
                e.mem.set_u32(CURRENT_PASS_ENUM_011F91E4, 0x25a);
                let holder = e.mem.u32(RENDER_HOLDER);
                e.vcall(geometry, 0xdc, &args![holder]);
                e.mem.set_u32(CURRENT_PASS_011F91E0, pass);
                e.mem.set_u16(pass + 4, 0x259);
                e.mem.set_u32(CURRENT_PASS_ENUM_011F91E4, 0x259);
            }
            let holder = e.mem.u32(RENDER_HOLDER);
            e.vcall(geometry, 0xdc, &args![holder]);
            e.call(RENDERER_CLEAR, &args![]);
            let rendered = e.call(RENDER_TARGET_OF, &args![target]).u32();
            e.call(SET_RENDER_TARGET, &args![0u32, rendered]);
            let manager = e.mem.u32(IMAGE_SPACE_MANAGER);
            let effect = e
                .mem
                .u32(e.mem.u32(manager.wrapping_add(8)).wrapping_add(0x34));
            let w0 = e.mem.u32(property.wrapping_add(0x60));
            let w1 = e.mem.u32(property.wrapping_add(0x64));
            let w2 = e.mem.u32(property.wrapping_add(0x68));
            let w3 = e.mem.u32(property.wrapping_add(0x6c));
            e.mem.set_u32(effect.wrapping_add(0x58), w0);
            e.mem.set_u32(effect.wrapping_add(0x5c), w1);
            e.mem.set_u32(effect.wrapping_add(0x60), w2);
            e.mem.set_u8(effect.wrapping_add(5), 1);
            e.mem.set_u32(effect.wrapping_add(0x64), w3);
            let manager = e.mem.u32(IMAGE_SPACE_MANAGER);
            e.call(
                IMAGE_SPACE_RENDER_EFFECT,
                &args![manager, 0xdu32, renderer, target, target, 0u32, 1u32],
            );
        }
        e.call(POINTER_LIST_REMOVE_ALL, &args![base.wrapping_add(0x90)]);
        e.vcall(renderer, 0xac, &args![saved]);
        e.mem.set_u32(base.wrapping_add(8), old_camera);
        let owner = fog_owner(e);
        image_space_shader_volumetric_fog_return_volumetric_fog_texture(e, Ptr::new(owner));
        e.call(RENDER_PASS_DESTRUCTOR, &args![pass]);
    });
    u32::from(count != 0)
}

// Translated from 00b65ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` without words: adds the pass types 0x2f0 and 0x2f1
/// to each batch renderer's 16-bit list at `+0x2c` ([`fn_00b63bf0`] with the addresses of two
/// local words), renders them ([`fn_00b63770`] with 0 and -1) and clears each list
/// (`00b64ec0`).
pub fn fn_00b65ec0(e: &mut Engine, this: Ptr<BSShaderAccumulator>) {
    let base = this.addr();
    e.with_stack(8, |e, locals| {
        let first = locals.addr();
        let second = first + 4;
        let mut i = 0u32;
        if batch_count(e, base) != 0 {
            e.mem.set_u32(first, 0x2f0);
            e.mem.set_u32(second, 0x2f1);
            while i < batch_count(e, base) {
                let list = batch_renderer(e, base, i).wrapping_add(0x2c);
                fn_00b63bf0(e, list, first);
                let list = batch_renderer(e, base, i).wrapping_add(0x2c);
                fn_00b63bf0(e, list, second);
                i += 1;
            }
        }
    });
    fn_00b63770(e, this, 0x2f0, 0x2f1, 0, -1);
    let mut i = 0u32;
    while i < batch_count(e, base) {
        let list = batch_renderer(e, base, i).wrapping_add(0x2c);
        e.call(SHORT_LIST_CLEAR, &args![list]);
        i += 1;
    }
}

// Translated from 00b65f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with three words (a reference, the address of a
/// point and a `float`, and a byte): the reference volume list (`ReferenceVolumeList`,
/// `+0x140`). An entry whose `+0x10` is the reference gets the three words, the `float`
/// (`FLD`/`FSTP`) and the byte at `+0x1a`. Otherwise a new 0x20-byte entry (null when the
/// allocation fails) gets the point and `float`, the reference, zeros at `+0x14`, `+0x18`,
/// `+0x19` and `+0x1c` and 1 at `+0x1a` (the byte argument is not used; the exe's stack
/// word holding it takes the `float` on the way), and is put at the head of the list in a
/// new node (`0043a010` on the count at `+0x148`).
pub fn fn_00b65f60(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    reference: u32,
    point: u32,
    flag: u8,
) {
    let list = Ptr::<NiTPointerList>::new(this.addr().wrapping_add(0x140));
    let mut item = e.get(list, NiTPointerList::m_pkHead);
    while item != 0 {
        let node = Ptr::<NiTListItem>::new(item);
        let entry = e.get(node, NiTListItem::m_element);
        item = e.get(node, NiTListItem::m_pkNext);
        if entry != 0 && e.mem.u32(entry.wrapping_add(0x10)) == reference {
            for k in 0..3 {
                let v = e.mem.u32(point + 4 * k);
                e.mem.set_u32(entry + 4 * k, v);
            }
            let v = x87_quiet_bits(e.mem.u32(point.wrapping_add(0xc)));
            e.mem.set_u32(entry.wrapping_add(0xc), v);
            e.mem.set_u8(entry.wrapping_add(0x1a), flag);
            return;
        }
    }
    let entry = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x20u32])
        .u32();
    if entry != 0 {
        let v = e.mem.u32(point);
        e.mem.set_u32(entry, v);
        let scale = x87_quiet_bits(x87_quiet_bits(e.mem.u32(point.wrapping_add(0xc))));
        let v = e.mem.u32(point.wrapping_add(4));
        e.mem.set_u32(entry.wrapping_add(4), v);
        let v = e.mem.u32(point.wrapping_add(8));
        e.mem.set_u32(entry.wrapping_add(0xc), scale);
        e.mem.set_u32(entry.wrapping_add(8), v);
        e.mem.set_u32(entry.wrapping_add(0x10), reference);
        e.mem.set_u32(entry.wrapping_add(0x14), 0);
        e.mem.set_u8(entry.wrapping_add(0x18), 0);
        e.mem.set_u8(entry.wrapping_add(0x19), 0);
        e.mem.set_u8(entry.wrapping_add(0x1a), 1);
        e.mem.set_u32(entry.wrapping_add(0x1c), 0);
    }
    let count = list.addr().wrapping_add(8);
    let node = Ptr::<NiTListItem>::new(e.call(LIST_NEW_NODE, &args![count]).u32());
    e.set(node, NiTListItem::m_element, entry);
    e.set(node, NiTListItem::m_pkPrev, 0);
    let head = e.get(list, NiTPointerList::m_pkHead);
    e.set(node, NiTListItem::m_pkNext, head);
    let head = e.get(list, NiTPointerList::m_pkHead);
    if head != 0 {
        e.set(
            Ptr::<NiTListItem>::new(head),
            NiTListItem::m_pkPrev,
            node.addr(),
        );
        let n = e.get(list, NiTPointerList::m_uiCount);
        e.set(list, NiTPointerList::m_uiCount, n.wrapping_add(1));
    } else {
        let n = e.get(list, NiTPointerList::m_uiCount);
        e.set(list, NiTPointerList::m_uiCount, n.wrapping_add(1));
        e.set(list, NiTPointerList::m_pkTail, node.addr());
    }
    e.set(list, NiTPointerList::m_pkHead, node.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b6_3090,
            bs_shader_accumulator_stop_grouping_alphas(Ptr<BSShaderAccumulator>)
        ),
        entry!(0x00b6_30c0, fn_00b630c0(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_30f0, bs_shader_accumulator_is_shadow_pass(u32) -> bool),
        entry!(
            0x00b6_3120,
            fn_00b63120(Ptr<BSShaderAccumulator>, u32, u32) -> bool
        ),
        entry!(
            0x00b6_3500,
            bs_shader_accumulator_start_grouping_alphas(Ptr<BSShaderAccumulator>, u32, u32)
        ),
        entry!(0x00b6_3540, fn_00b63540(Ptr<BSShaderAccumulator>, u32)),
        entry!(
            0x00b6_3680,
            fn_00b63680(Ptr<BSShaderAccumulator>, u32, u32) -> u8
        ),
        entry!(
            0x00b6_3770,
            fn_00b63770(Ptr<BSShaderAccumulator>, u32, u32, u32, i32)
        ),
        entry!(0x00b6_39e0, fn_00b639e0(Ptr<BSShaderAccumulator>, u32, u32)),
        entry!(0x00b6_3a50, fn_00b63a50(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_3a90, fn_00b63a90(Ptr<BSShaderAccumulator>) -> u32),
        entry!(0x00b6_3b60, fn_00b63b60(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_3b90, fn_00b63b90(Ptr<BSShaderAccumulator>, u32)),
        entry!(
            0x00b6_3d10,
            image_space_shader_volumetric_fog_return_volumetric_fog_texture(Ptr)
        ),
        entry!(
            0x00b6_3f10,
            fn_00b63f10(Ptr<BSShaderAccumulator>, u32) -> bool
        ),
        entry!(
            0x00b6_4570,
            fn_00b64570(Ptr<BSShaderAccumulator>, u32, u32) -> bool
        ),
        entry!(0x00b6_4a00, fn_00b64a00(Ptr<BSShaderAccumulator>, u32, u32)),
        entry!(
            0x00b6_4bf0,
            bs_shader_accumulator_do_sun_occlusion_queury(Ptr<BSShaderAccumulator>, u32, u32, u32)
        ),
        entry!(0x00b6_4f20, fn_00b64f20(Ptr, u32)),
        entry!(0x00b6_4f90, fn_00b64f90(Ptr<BSShaderAccumulator>)),
        entry!(
            0x00b6_50c0,
            bs_shader_accumulator_render_alpha_geometry(Ptr<BSShaderAccumulator>, u32)
        ),
        entry!(
            0x00b6_5550,
            fn_00b65550(Ptr<BSShaderAccumulator>, u32, u32, u32)
        ),
        entry!(0x00b6_5570, fn_00b65570(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_5580, fn_00b65580(Ptr<BSShaderAccumulator>)),
        entry!(
            0x00b6_55b0,
            bs_shader_accumulator_clear_all_bound_volumes(Ptr<BSShaderAccumulator>)
        ),
        entry!(0x00b6_5660, fn_00b65660(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_56e0, fn_00b656e0(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_5a80, fn_00b65a80(Ptr) -> u32),
        entry!(0x00b6_5a90, fn_00b65a90(u32, u32, u32, u32, u32) -> bool),
        entry!(0x00b6_5ae0, fn_00b65ae0(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_5c60, fn_00b65c60(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_5dc0, fn_00b65dc0(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_5e80, fn_00b65e80(Ptr<BSShaderAccumulator>)),
        entry!(0x00b6_6050, fn_00b66050(Ptr<BSShaderAccumulator>, u32)),
        entry!(
            0x00b660d0,
            bs_shader_accumulator_bs_shader_accumulator(
                Ptr<BSShaderAccumulator>,
                u32,
                u32,
                u32,
            ) -> Ptr<BSShaderAccumulator>
        ),
        entry!(0x00b6_56b0, fn_00b656b0(Ptr) -> u32),
        entry!(0x00b6_3e90, fn_00b63e90(Ptr) -> u32),
        entry!(
            0x00b6_3420,
            bs_shader_accumulator_create_render_pass_tree() -> u32
        ),
        entry!(0x00b6_3620, fn_00b63620(Ptr<BSShaderAccumulator>)),
        entry!(
            0x00b6_3b10,
            fn_00b63b10(Ptr<BSShaderAccumulator>, u32) -> bool
        ),
        entry!(0x00b6_3bd0, fn_00b63bd0(u32, u32, u32, u8)),
        entry!(0x00b6_3bf0, fn_00b63bf0(u32, u32)),
        entry!(0x00b6_3c60, fn_00b63c60(Ptr) -> u32),
        entry!(0x00b6_3d60, fn_00b63d60(Ptr<NiTPointerMap>)),
        entry!(0x00b6_3d80, fn_00b63d80(Ptr<NiTPointerMap>)),
        entry!(0x00b6_3df0, fn_00b63df0(u32, u32)),
        entry!(
            0x00b6_3e30,
            fn_00b63e30(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00b6_3e70,
            fn_00b63e70(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00b6_3f90,
            fn_00b63f90(Ptr<BSShaderAccumulator>, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00b6_4020, fn_00b64020(u32, u32, u32, u32, u32) -> bool),
        entry!(0x00b6_4440, fn_00b64440(u32, u32, u32, u32, u32) -> bool),
        entry!(0x00b6_4490, fn_00b64490(u32, u32, u32, u32, u32) -> bool),
        entry!(0x00b6_44b0, fn_00b644b0(u32, u32, u32, u32, u32) -> bool),
        entry!(0x00b6_4530, fn_00b64530(Ptr<BSShaderAccumulator>)),
        entry!(
            0x00b6_51e0,
            fn_00b651e0(Ptr<BSShaderAccumulator>, u32, u32) -> u32
        ),
        entry!(0x00b6_5ec0, fn_00b65ec0(Ptr<BSShaderAccumulator>)),
        entry!(
            0x00b6_5f60,
            fn_00b65f60(Ptr<BSShaderAccumulator>, u32, u32, u8)
        ),
    ]
}

#[cfg(test)]
mod element_destructor_tests {
    use super::*;

    #[test]
    fn element_destructor_resets_the_words_between_two_clean_ups() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        for (off, value) in [(0, 1u32), (4, 0x40), (8, 0x80), (0xc, 0xc0), (0x10, 0x100)] {
            e.mem.set_u32(this + off, value);
        }
        // The clean-up sees the vtable set; its second run sees the words
        // reset and its result is passed on.
        e.register_double(PERSISTENT_LIST_CLEAR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), PERSISTENT_LIST_VTABLE);
            (e.mem.u32(a[0] + 0xc) + e.mem.u32(a[0] + 4)).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_56b0, &[this]).u32(), 0x40);
        let clears = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == PERSISTENT_LIST_CLEAR)
            .count();
        assert_eq!(clears, 2);
        assert_eq!(e.mem.u32(this + 0xc), 0x40);
        for off in [4, 8, 0x10] {
            assert_eq!(e.mem.u32(this + off), 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine whose callees outside this file are doubles: allocators
    /// hand out fresh blocks (0 for the size `fail` names), the
    /// constructors return their `this`.
    fn accumulator_engine(fail: u32) -> Engine {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| ret(0));
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(RENDER_PASS_CONSTRUCTOR, |_, _| ret(0));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| ret(0));
        e.register(ALLOCATE_BUCKETS, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            if a[1] == fail {
                ret(0)
            } else {
                ret(e.mem.alloc(a[1]))
            }
        });
        e.register_double(OPERATOR_NEW, move |e, a| {
            if a[0] == fail {
                ret(0)
            } else {
                ret(e.mem.alloc(a[0]))
            }
        });
        e.register(INSTANCE_RENDERER_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(BATCH_RENDERER_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(POINTER_LIST_REMOVE_ALL, |_, _| ret(0));
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u32(SILHOUETTE_CHANNEL, 0x3e99_999a);
        e
    }

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
    fn constructor_builds_one_batch_renderer_and_sets_the_defaults() {
        let mut e = accumulator_engine(u32::MAX);
        let this = e.new_object::<BSShaderAccumulator>();
        let base = this.addr();
        // Garbage the constructor must overwrite (no smart pointers held).
        for off in (0x04..0x280).step_by(4) {
            e.mem.set_u32(base + off, 0xdead_beef);
        }
        e.mem.set_u32(base + 0x198, 0);
        e.mem.set_u32(base + 0x1a0, 0);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2BC, 7);
        e.call_log = Some(vec![]);
        let result = e.call(0x00b6_60d0, &[base, 0x55, 1, 9]).u32();
        assert_eq!(result, base);
        assert_eq!(e.mem.u32(base), ACCUMULATOR_VTABLE);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![base]]);
        let element = [PERSISTENT_LIST_CONSTRUCTOR, PERSISTENT_LIST_DESTRUCTOR];
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![
                vec![base + 0x9c, 0x14, 4, element[0], element[1]],
                vec![base + 0xec, 0x14, 4, element[0], element[1]],
            ]
        );
        assert_eq!(
            calls_to(&e, RENDER_PASS_CONSTRUCTOR),
            vec![vec![base + 0x1a4, 0, 0x2f5, 1, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, IMPORT_INITIALIZE_CRITICAL_SECTION),
            vec![vec![base + 0x200]]
        );
        assert_eq!(
            calls_to(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![base + 0x90]]
        );
        // The room map.
        assert_eq!(e.mem.u32(base + 0x1b4), MAP_VTABLE);
        assert_eq!(e.mem.u32(base + 0x1b8), 0x25);
        assert_eq!(e.mem.u32(base + 0x1c0), 0);
        let buckets = e.mem.u32(base + 0x1bc);
        assert_eq!(calls_to(&e, ALLOCATE_BUCKETS), vec![vec![0x94]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![buckets, 0, 0x94]]);
        // Sun tests and lists cleared.
        for off in [
            0x3c, 0x48, 0x50, 0x54, 0x58, 0x5c, 0x64, 0x68, 0x6c, 0x70, 0x78, 0x7c, 0x80,
        ] {
            assert_eq!(e.mem.u32(base + off), 0, "+{off:#x}");
        }
        for off in [0x4c, 0x60, 0x74, 0x40, 0x85, 0x150, 0x3a] {
            assert_eq!(e.mem.u8(base + off), 0, "+{off:#x}");
        }
        for off in [0x90, 0x94, 0x98, 0x13c, 0x140, 0x144, 0x148, 0x14c, 0x19c] {
            assert_eq!(e.mem.u32(base + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(base + 0x44), ONE_BITS);
        assert_eq!(e.mem.u8(base + 0x84), 1);
        assert_eq!(e.mem.u8(base + 0x86), 1);
        assert_eq!(e.mem.u8(base + 0x16c), 1);
        // Occlusion queries: 50 cleared slots.
        assert_eq!(e.mem.u16(base + 0x88), 0);
        assert_eq!(e.mem.u16(base + 0x8a), 0x32);
        let queries = e.mem.u32(base + 0x8c);
        assert_ne!(queries, 0);
        assert!((0..200).step_by(4).all(|i| e.mem.u32(queries + i) == 0));
        // One batch renderer built from the pass count and the argument.
        assert_ne!(e.mem.u32(base + 0x170), 0);
        assert_eq!(e.mem.u32(base + 0x18c), 1);
        assert_eq!(e.mem.u32(base + 0x190), 9);
        let first = e.mem.u32(base + 0x174);
        assert_eq!(
            calls_to(&e, BATCH_RENDERER_CONSTRUCTOR),
            vec![vec![first, 9, 0x55]]
        );
        assert_eq!(e.mem.u32(base + 0x178), 0);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[1]).collect();
        assert_eq!(sizes, vec![200, 4, 4, 4, 1, 1]);
        for off in [0x17c, 0x180, 0x184, 0x188] {
            assert_ne!(e.mem.u32(base + off), 0);
        }
        assert_eq!(e.mem.u32(tls + TLS_WORD_2BC), 0);
        // Silhouette colour: the constant three times, alpha 1.
        assert_eq!(e.mem.u32(base + 0x154), 0x3e99_999a);
        assert_eq!(e.mem.u32(base + 0x158), 0x3e99_999a);
        assert_eq!(e.mem.u32(base + 0x15c), 0x3e99_999a);
        assert_eq!(e.mem.u32(base + 0x160), ONE_BITS);
        assert!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn constructor_releases_pointers_and_keeps_failed_renderers_null() {
        // The 0x98-byte allocations fail: both batch renderers stay null.
        let mut e = accumulator_engine(0x98);
        let this = e.new_object::<BSShaderAccumulator>();
        let base = this.addr();
        // A held shadow light (last reference) and water fog target (shared).
        let light = e.mem.alloc(8);
        let target = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0077_0001);
        e.register(0x0077_0001, |_, _| ret(0));
        e.mem.set_u32(light, vtable);
        e.mem.set_u32(light + 4, 1);
        e.mem.set_u32(target, vtable);
        e.mem.set_u32(target + 4, 3);
        // The constructor clears both pointers before it releases them; the
        // instance-renderer double stores them in between so the release
        // runs.
        e.register_double(INSTANCE_RENDERER_CONSTRUCTOR, move |e, a| {
            e.mem.set_u32(base + 0x1a0, light);
            e.mem.set_u32(base + 0x198, target);
            ret(a[0])
        });
        // A signalling NaN constant is quieted by the x87 load.
        e.mem.set_u32(SILHOUETTE_CHANNEL, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00b6_60d0, &[base, 0x55, 2, 3]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x98], vec![0x98]]);
        assert!(calls_to(&e, BATCH_RENDERER_CONSTRUCTOR).is_empty());
        assert_eq!(e.mem.u32(base + 0x174), 0);
        assert_eq!(e.mem.u32(base + 0x178), 0);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[1]).collect();
        assert_eq!(sizes, vec![200, 4, 8, 8, 2, 2]);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![light + 4], vec![target + 4]]
        );
        assert_eq!(calls_to(&e, 0x0077_0001), vec![vec![light]]);
        assert_eq!(e.mem.u32(base + 0x1a0), 0);
        assert_eq!(e.mem.u32(base + 0x198), 0);
        assert_eq!(e.mem.u32(base + 0x154), 0x7fc0_0001);
        assert_eq!(e.mem.u32(base + 0x160), ONE_BITS);
    }

    #[test]
    fn constructor_builds_two_renderers_and_a_null_instance_renderer() {
        let mut e = accumulator_engine(u32::MAX);
        let this = e.new_object::<BSShaderAccumulator>();
        let base = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x00b6_60d0, &[base, 0x66, 2, 5]);
        let first = e.mem.u32(base + 0x174);
        let second = e.mem.u32(base + 0x178);
        assert_ne!(first, 0);
        assert_ne!(second, 0);
        assert_eq!(
            calls_to(&e, BATCH_RENDERER_CONSTRUCTOR),
            vec![vec![first, 5, 0x66], vec![second, 5, 0x66]]
        );
        // The instance renderer's allocation failing leaves it null.
        let mut e = accumulator_engine(4);
        let this = e.new_object::<BSShaderAccumulator>();
        e.call_log = Some(vec![]);
        e.call(0x00b6_60d0, &[this.addr(), 0, 3, 1]);
        assert_eq!(e.mem.u32(this.addr() + 0x170), 0);
        assert!(calls_to(&e, INSTANCE_RENDERER_CONSTRUCTOR).is_empty());
        // The overflow rule of the word arrays.
        assert_eq!(word_array_size(0x4000_0000), u32::MAX);
        assert_eq!(word_array_size(3), 12);
    }
}

#[cfg(test)]
mod b061_tests {
    use super::*;

    const QUIET: &[u32] = &[
        BATCH_STOP_GROUPING_ALPHAS,
        BATCH_START_GROUPING_ALPHAS,
        BATCH_CLEAR_PASSES,
        BATCH_HAS_LISTED_PASS,
        BATCH_NEXT_LIST,
        BATCH_RENDER_LIST,
        BATCH_RENDER_LIST_00B9A3C0,
        BATCH_SORT_ALPHAS,
        BATCH_END_PASS,
        BATCH_RENDER_PASS_IMMEDIATELY,
        BATCH_RENDER_NEXT_ALPHA,
        GEOMETRY_GROUP_DESTRUCTOR,
        GEOMETRY_GROUP_RESET,
        GEOMETRY_GROUP_RENDER_00B9A4D0,
        SHORT_LIST_CLEAR,
        QUERY_DESTRUCTOR,
        QUERY_RESULT_00C4EE40,
        QUERY_RESULT_00C4EDF0,
        QUERY_BEGIN,
        QUERY_END,
        MEMORY_DEALLOCATE,
        LIGHTING_00BBB840,
        IMPORT_ENTER_CRITICAL_SECTION,
        IMPORT_LEAVE_CRITICAL_SECTION,
        IMPORT_DELETE_CRITICAL_SECTION,
        ROOM_MAP_REMOVE_ALL,
        IMAGE_SPACE_TEXTURE_GET_RENDERED,
        IMAGE_SPACE_TEXTURE_SET,
        TEXTURE_MANAGER_RETURN_RENDERED,
        GET_NORMALS_TEXTURE,
        RENDER_TARGET_OF,
        SET_RENDER_TARGET,
        RENDERER_CLEAR,
        STATE_RESET_00B98380,
        CURRENT_RENDERED_TEXTURE,
        GET_SHADER,
        DEBUG_SET_TEXTURE,
        SHADER_SETUP_00B7A870,
        HOLDER_METHOD_004E9BB0,
        RENDER_PASS_DESTRUCTOR,
        RENDER_PASS_CONSTRUCTOR,
        POINTER_LIST_REMOVE_ALL,
        FREE_BUCKETS,
        INSTANCE_RENDERER_DESTRUCTOR,
        VECTOR_DESTRUCTOR_ITERATOR,
        VOLUME_LIST_REMOVE,
        ACCUMULATOR_BASE_DESTRUCTOR,
        CREATE_RENDER_PASS_TREE,
        PASS_TREE_TAKE,
        DECAL_PASS,
        SET_ALPHA_BLEND_ENABLE,
        SET_CLIP_PLANES,
        TEXTURE_MANAGER_BORROW,
        IMAGE_SPACE_RENDER_EFFECT,
        QUERY_CONSTRUCTOR,
        MEMORY_ALLOCATE,
        LIST_NODE_ALLOCATE,
        LOCK_ENTER,
        IMPORT_INTERLOCKED_DECREMENT,
        BATCH_GROUP_DISTANCE,
        0x0066_00a4,
        0x0066_0004,
    ];

    /// An accumulator with `count` empty batch renderers, its four
    /// per-renderer arrays, a camera, a renderer holder and doubles for
    /// every callee outside this file (allocators hand out fresh blocks,
    /// the node pool's lock and the interlocked decrement count, the group
    /// distance answers `-FLT_MAX`).
    fn engine(count: u32) -> (Engine, Ptr<BSShaderAccumulator>) {
        let mut e = Engine::new();
        for (page, size) in [
            (0x0101_5000, 0x2000),
            (0x0102_4000, 0x1000),
            (0x0107_3000, 0x1000),
            (0x0109_9000, 0x1000),
            (0x011a_9000, 0x1000),
            (0x011c_5000, 0x1000),
            (0x011f_4000, 0xc000),
        ] {
            e.map(page, size);
        }
        for &addr in QUIET {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(LIST_NODE_ALLOCATE, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(LOCK_ENTER, |e, _| {
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(BATCH_GROUP_DISTANCE, |_, _| f64::from(f32::MIN).into_ret());
        e.mem.set_u32(NO_DISTANCE, f32::MIN.to_bits());
        e.mem.set_f64(NO_DISTANCE_DOUBLE, f64::from(f32::MIN));
        let this = e.new_object::<BSShaderAccumulator>();
        let base = this.addr();
        for i in 0..count {
            let renderer = e.mem.alloc(0x98);
            e.mem.set_u32(base + 0x174 + i * 4, renderer);
        }
        e.set(this, BSShaderAccumulator::uiBatchRendererCount, count);
        for slot in [0x17c, 0x180, 0x184, 0x188] {
            let block = e.mem.alloc(8);
            e.mem.set_u32(base + slot, block);
        }
        let camera = e.mem.alloc(0x110);
        e.mem.set_u32(base + 8, camera);
        let holder = e.mem.alloc(0x210);
        let vtable = e.mem.alloc(0x190);
        for slot in [0xac, 0xb4, 0x18c] {
            e.mem.set_u32(vtable + slot, 0x0069_0000 + slot);
            e.register(0x0069_0000 + slot, |_, _| Ret::default());
        }
        e.mem.set_u32(holder, vtable);
        e.mem.set_u32(RENDER_HOLDER, holder);
        let accumulator_vtable = e.mem.alloc(0xa8);
        e.mem.set_u32(accumulator_vtable + 0xa4, 0x0066_00a4);
        e.mem.set_u32(base, accumulator_vtable);
        e.call_log = Some(vec![]);
        (e, this)
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

    fn order(e: &Engine, of: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| of.contains(a))
            .collect()
    }

    fn renderer(e: &Engine, this: Ptr<BSShaderAccumulator>, i: u32) -> u32 {
        e.mem.u32(this.addr() + 0x174 + i * 4)
    }

    #[test]
    fn stop_grouping_uses_the_thread_renderer() {
        let (mut e, this) = engine(2);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2BC, 3);
        e.call(0x00b6_3090, &[this.addr()]);
        assert_eq!(
            calls_to(&e, BATCH_STOP_GROUPING_ALPHAS),
            vec![vec![renderer(&e, this, 1)]]
        );
    }

    #[test]
    fn clear_passes_of_every_renderer() {
        let (mut e, this) = engine(2);
        e.call(0x00b6_30c0, &[this.addr()]);
        let (a, b) = (renderer(&e, this, 0), renderer(&e, this, 1));
        assert_eq!(
            calls_to(&e, BATCH_CLEAR_PASSES),
            vec![vec![a, 0], vec![b, 0]]
        );
    }

    #[test]
    fn shadow_pass_types() {
        let mut e = Engine::new();
        for (pass_type, shadow) in [
            (0x1c3, false),
            (0x1c4, true),
            (0x1c7, true),
            (0x230, false),
            (0x234, true),
            (0x283, false),
            (0x287, true),
            (0x288, false),
        ] {
            assert_eq!(
                e.call(0x00b6_30f0, &[pass_type]).bool(),
                shadow,
                "{pass_type:#x}"
            );
        }
    }

    #[test]
    fn listed_pass_stops_at_the_first_renderer_that_has_one() {
        let (mut e, this) = engine(2);
        let second = renderer(&e, this, 1);
        e.register_double(BATCH_HAS_LISTED_PASS, move |_, a| {
            u32::from(a[0] == second).into_ret()
        });
        assert!(e.call(0x00b6_3120, &[this.addr(), 5, 6]).bool());
        assert_eq!(calls_to(&e, BATCH_HAS_LISTED_PASS).len(), 2);
        assert_eq!(calls_to(&e, BATCH_HAS_LISTED_PASS)[1], vec![second, 5, 6]);
        e.register(BATCH_HAS_LISTED_PASS, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b6_3120, &[this.addr(), 5, 6]).bool());
        assert_eq!(calls_to(&e, BATCH_HAS_LISTED_PASS).len(), 1);
        let (mut e, this) = engine(0);
        assert!(!e.call(0x00b6_3120, &[this.addr(), 5, 6]).bool());
    }

    #[test]
    fn start_grouping_passes_the_camera_and_flag() {
        let (mut e, this) = engine(1);
        e.mem.set_u8(this.addr() + 0x31, 1);
        e.call(0x00b6_3500, &[this.addr(), 0x5000, 1]);
        let camera = e.mem.u32(this.addr() + 8);
        assert_eq!(
            calls_to(&e, BATCH_START_GROUPING_ALPHAS),
            vec![vec![renderer(&e, this, 0), 0x5000, 1, camera, 1]]
        );
    }

    fn sun_query(
        e: &mut Engine,
        this: Ptr<BSShaderAccumulator>,
        index: u32,
        state: u8,
        count: u32,
    ) -> u32 {
        let test = this.addr() + index * 0x14;
        e.mem.set_u32(test + 0x48, 0x7700);
        e.mem.set_u8(test + 0x4c, 1);
        e.register_double(QUERY_RESULT_00C4EE40, move |e, a| {
            e.mem.set_u32(a[1], count);
            u32::from(state).into_ret()
        });
        e.call(0x00b6_3540, &[this.addr(), index]);
        test
    }

    #[test]
    fn sun_query_result_sets_the_visible_fraction() {
        let (mut e, this) = engine(1);
        e.mem.set_f64(SUN_PIXELS_010737C8, 1500.0);
        let test = sun_query(&mut e, this, 1, 1, 750);
        assert_eq!(e.mem.f32(test + 0x50), 0.5);
        assert_eq!(e.mem.u32(test + 0x54), 750);
        assert_eq!(e.mem.u8(test + 0x4c), 0);
        let test = sun_query(&mut e, this, 2, 1, 3000);
        assert_eq!(e.mem.f32(test + 0x50), 0.0);
        assert_eq!(e.mem.u32(test + 0x54), 3000);
        let test = sun_query(&mut e, this, 0, 1, 0);
        assert_eq!(e.mem.f32(test + 0x50), 1.0);
        // Not ready: unchanged; failed: the query goes.
        let test = sun_query(&mut e, this, 0, 0, 5);
        assert_eq!(e.mem.u8(test + 0x4c), 1);
        let test = sun_query(&mut e, this, 0, 2, 5);
        assert_eq!(e.mem.u32(test + 0x48), 0);
        assert_eq!(e.mem.u8(test + 0x4c), 0);
        assert_eq!(calls_to(&e, QUERY_DESTRUCTOR), vec![vec![0x7700]]);
        // Index 3 is out of range.
        e.call_log = Some(vec![]);
        e.call(0x00b6_3540, &[this.addr(), 3]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// A reference volume for `reference` in the accumulator's list.
    fn volume(e: &mut Engine, this: Ptr<BSShaderAccumulator>, reference: u32) -> u32 {
        let volume = e.mem.alloc(0x20);
        e.mem.set_u32(volume + 0x10, reference);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, volume);
        let head = e.mem.u32(this.addr() + 0x140);
        e.mem.set_u32(node, head);
        if head != 0 {
            e.mem.set_u32(head + 4, node);
        } else {
            e.mem.set_u32(this.addr() + 0x144, node);
        }
        e.mem.set_u32(this.addr() + 0x140, node);
        let count = e.mem.u32(this.addr() + 0x148) + 1;
        e.mem.set_u32(this.addr() + 0x148, count);
        volume
    }

    #[test]
    fn occlusion_result_is_polled_and_cached() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        let v = volume(&mut e, this, 0x42);
        // No such volume.
        assert_eq!(e.call(0x00b6_3680, &[base, 0x43, 1]).u8(), 0);
        // Not waiting: cached as is.
        e.mem.set_u8(v + 0x18, 1);
        assert_eq!(e.call(0x00b6_3680, &[base, 0x42, 0]).u8(), 1);
        assert_eq!(e.mem.u32(base + 0x14c), 0x42);
        assert_eq!(e.mem.u8(base + 0x150), 1);
        // A cached reference answers without looking.
        e.mem.set_u8(v + 0x18, 0);
        assert_eq!(e.call(0x00b6_3680, &[base, 0x42, 0]).u8(), 1);
        // Waiting, polled: "not yet" twice, then 0 pixels: occluded.
        let w = volume(&mut e, this, 0x50);
        e.mem.set_u8(w + 0x19, 1);
        e.mem.set_u32(w + 0x14, 0x7700);
        e.mem.set_u8(VOLUME_QUERY_ENABLED, 1);
        let mut polls = 0;
        e.register_double(QUERY_RESULT_00C4EDF0, move |e, a| {
            polls += 1;
            e.mem.set_u32(a[1], 0);
            u32::from(polls < 3).into_ret()
        });
        assert_eq!(e.call(0x00b6_3680, &[base, 0x50, 1]).u8(), 1);
        assert_eq!(calls_to(&e, QUERY_RESULT_00C4EDF0).len(), 3);
        assert_eq!(e.mem.u8(w + 0x19), 0);
        assert_eq!(e.mem.u32(base + 0x14c), 0x50);
        // A failed query: dropped, 999999 and not occluded.
        let f = volume(&mut e, this, 0x60);
        e.mem.set_u8(f + 0x19, 1);
        e.mem.set_u32(f + 0x14, 0x7800);
        e.register(QUERY_RESULT_00C4EDF0, |_, _| 2u32.into_ret());
        assert_eq!(e.call(0x00b6_3680, &[base, 0x60, 1]).u8(), 0);
        assert_eq!(e.mem.u32(f + 0x14), 0);
        assert_eq!(e.mem.u32(f + 0x1c), 999_999);
        // Waiting but not asked to wait: not cached.
        let g = volume(&mut e, this, 0x70);
        e.mem.set_u8(g + 0x19, 1);
        e.mem.set_u8(g + 0x18, 1);
        assert_eq!(e.call(0x00b6_3680, &[base, 0x70, 0]).u8(), 1);
        assert_eq!(e.mem.u32(base + 0x14c), 0x60);
    }

    #[test]
    fn pass_range_renders_the_lowest_pass_first() {
        let (mut e, this) = engine(2);
        let (a, b) = (renderer(&e, this, 0), renderer(&e, this, 1));
        e.register_double(BATCH_NEXT_LIST, move |e, w| {
            e.mem.set_u32(w[1], if w[0] == a { 5 } else { 3 });
            1u32.into_ret()
        });
        e.call(0x00b6_3770, &[this.addr(), 1, 0x2f1, 7, 0xffff_ffff]);
        assert_eq!(e.mem.u32(a + 0x20), 1);
        assert_eq!(e.mem.u32(b + 0x24), 0x2f1);
        let renders: Vec<u32> = calls_to(&e, BATCH_RENDER_LIST)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(renders, vec![b, a]);
        assert_eq!(calls_to(&e, BATCH_RENDER_LIST)[0][4], 7);
        assert_eq!(calls_to(&e, BATCH_END_PASS).len(), 1);
        assert!(calls_to(&e, GEOMETRY_GROUP_RESET).is_empty());
        // A shadow pass with +0x164 set goes through 00b9a3c0; a group
        // index uses the groups' own renderers and resets the groups.
        let (mut e, this) = engine(1);
        let r = renderer(&e, this, 0);
        let group = e.mem.alloc(0x24);
        let own = e.mem.alloc(0x98);
        e.mem.set_u32(group + 8, own);
        e.mem.set_u32(r + 0x3c + 2 * 4, group);
        e.mem.set_u8(this.addr() + 0x164, 1);
        e.register(BATCH_NEXT_LIST, |e, w| {
            e.mem.set_u32(w[1], 0x1c5);
            1u32.into_ret()
        });
        e.call(0x00b6_3770, &[this.addr(), 1, 0x2f1, 0, 2]);
        assert_eq!(calls_to(&e, BATCH_RENDER_LIST_00B9A3C0)[0][0], own);
        assert_eq!(calls_to(&e, BATCH_RENDER_LIST_00B9A3C0)[0][4], 0);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_RESET), vec![vec![group]]);
    }

    #[test]
    fn group_render_by_its_kind() {
        let (mut e, this) = engine(2);
        let (a, b) = (renderer(&e, this, 0), renderer(&e, this, 1));
        // No group: nothing.
        e.call(0x00b6_39e0, &[this.addr(), 3, 1]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let ga = e.mem.alloc(0x24);
        let gb = e.mem.alloc(0x24);
        e.mem.set_u32(a + 0x3c + 12, ga);
        e.mem.set_u32(b + 0x3c + 12, gb);
        e.mem.set_u8(ga, 1);
        e.call(0x00b6_39e0, &[this.addr(), 3, 1]);
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_RENDER_00B9A4D0),
            vec![vec![ga, 1], vec![gb, 1]]
        );
        // Bit 0 clear: the pass range of the groups' renderers.
        e.mem.set_u8(ga, 0);
        e.call(0x00b6_39e0, &[this.addr(), 3, 1]);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_RESET), vec![vec![ga], vec![gb]]);
    }

    #[test]
    fn alpha_sort_for_every_renderer() {
        let (mut e, this) = engine(2);
        e.mem.set_u8(this.addr() + 0x31, 1);
        e.call(0x00b6_3a50, &[this.addr()]);
        let camera = e.mem.u32(this.addr() + 8);
        let sorts = calls_to(&e, BATCH_SORT_ALPHAS);
        assert_eq!(
            sorts,
            vec![
                vec![renderer(&e, this, 0), camera, 1],
                vec![renderer(&e, this, 1), camera, 1]
            ]
        );
    }

    #[test]
    fn group_counts_are_summed() {
        let (mut e, this) = engine(2);
        for (i, count) in [(0, 3u16), (1, 4)] {
            let group = e.mem.alloc(0x24);
            e.mem.set_u16(group + 2, count);
            let r = renderer(&e, this, i);
            e.mem.set_u32(r + 0x54, group);
        }
        assert_eq!(e.call(0x00b6_3a90, &[this.addr()]).u32(), 7);
        e.set(this, BSShaderAccumulator::uiBatchRendererCount, 1);
        assert_eq!(e.call(0x00b6_3a90, &[this.addr()]).u32(), 3);
        e.set(this, BSShaderAccumulator::uiBatchRendererCount, 0);
        assert_eq!(e.call(0x00b6_3a90, &[this.addr()]).u32(), 0);
    }

    #[test]
    fn room_map_is_emptied_inside_its_lock() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        e.call(0x00b6_3b60, &[base]);
        assert_eq!(
            order(
                &e,
                &[
                    IMPORT_ENTER_CRITICAL_SECTION,
                    ROOM_MAP_REMOVE_ALL,
                    IMPORT_LEAVE_CRITICAL_SECTION
                ]
            ),
            vec![
                IMPORT_ENTER_CRITICAL_SECTION,
                ROOM_MAP_REMOVE_ALL,
                IMPORT_LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(calls_to(&e, ROOM_MAP_REMOVE_ALL), vec![vec![base + 0x1b4]]);
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![base + 0x200]]
        );
    }

    #[test]
    fn group_reset_for_every_renderer() {
        let (mut e, this) = engine(2);
        let (a, b) = (renderer(&e, this, 0), renderer(&e, this, 1));
        e.mem.set_u32(a + 0x3c + 8, 0xa0);
        e.mem.set_u32(b + 0x3c + 8, 0xb0);
        e.call(0x00b6_3b90, &[this.addr(), 2]);
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_RESET),
            vec![vec![0xa0], vec![0xb0]]
        );
    }

    #[test]
    fn fog_texture_goes_back() {
        let (mut e, _) = engine(1);
        let this = e.mem.alloc(0x9c);
        let holder = e.mem.alloc(0xc);
        e.mem.set_u32(this + 0x98, holder);
        e.mem.set_u32(TEXTURE_MANAGER, 0x7777);
        e.call(0x00b6_3d10, &[this]);
        assert!(calls_to(&e, IMAGE_SPACE_TEXTURE_GET_RENDERED).is_empty());
        e.mem.set_u32(holder + 8, 0x5500);
        e.call(0x00b6_3d10, &[this]);
        assert!(calls_to(&e, TEXTURE_MANAGER_RETURN_RENDERED).is_empty());
        e.register(IMAGE_SPACE_TEXTURE_GET_RENDERED, |_, _| {
            0x6600u32.into_ret()
        });
        e.call(0x00b6_3d10, &[this]);
        assert_eq!(
            calls_to(&e, TEXTURE_MANAGER_RETURN_RENDERED),
            vec![vec![0x7777, 0x6600]]
        );
        assert_eq!(calls_to(&e, IMAGE_SPACE_TEXTURE_SET), vec![vec![0x5500, 0]]);
    }

    #[test]
    fn register_geometry_through_the_mode_function() {
        let (mut e, this) = engine(1);
        let geometry = e.mem.alloc(0xc4);
        // No shader or properties: true without a call.
        assert!(e.call(0x00b6_3f10, &[this.addr(), geometry]).bool());
        e.mem.set_u32(geometry + 0xc0, 0x5100);
        e.mem.set_u32(geometry + 0xa8, 0x5200);
        e.register(0x0066_0100, |_, _| 0u32.into_ret());
        e.register(0x0066_0200, |_, _| 1u32.into_ret());
        e.mem.set_u32(REGISTER_FUNCTIONS, 0x0066_0100);
        e.mem.set_u32(REGISTER_FUNCTIONS + 8, 0x0066_0200);
        e.set(this, BSShaderAccumulator::eRenderMode, 2);
        assert!(e.call(0x00b6_3f10, &[this.addr(), geometry]).bool());
        assert_eq!(
            calls_to(&e, 0x0066_0200),
            vec![vec![this.addr(), geometry, geometry + 0x9c, 0x5200, 0x5100]]
        );
        // An empty entry falls back to entry 0.
        e.set(this, BSShaderAccumulator::eRenderMode, 1);
        assert!(!e.call(0x00b6_3f10, &[this.addr(), geometry]).bool());
        // A skin of the class with +0x3C clear: true at once.
        let skin = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, 0x0066_0008);
        e.register(0x0066_0008, |_, _| SKIN_RTTI_011F49D8.into_ret());
        e.mem.set_u32(skin, vtable);
        e.mem.set_u32(geometry + 0xbc, skin);
        assert!(e.call(0x00b6_3f10, &[this.addr(), geometry]).bool());
        assert_eq!(calls_to(&e, 0x0066_0100).len(), 1);
    }

    #[test]
    fn normals_pass_renders_and_restores() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        let camera = e.mem.alloc(0x110);
        for i in 0..9 {
            e.mem.set_f32(camera + 0x68 + i * 4, i as f32);
        }
        assert!(!e.call(0x00b6_4570, &[base, camera, 0]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let old = e.mem.u32(base + 8);
        let r = renderer(&e, this, 0);
        let group = e.mem.alloc(0x24);
        e.mem.set_u16(group + 2, 2);
        e.mem.set_u32(r + 0x5c, group);
        let empty = e.mem.alloc(0x24);
        e.mem.set_u32(r + 0x54, empty);
        e.register(GET_NORMALS_TEXTURE, |_, _| 0x7100u32.into_ret());
        // Every render-target lookup answers one object.
        let rt = e.mem.alloc(4);
        let vtable = e.mem.alloc(0xb8);
        for slot in [0xac, 0xb4] {
            e.mem.set_u32(vtable + slot, 0x006a_0000 + slot);
            e.register(0x006a_0000 + slot, |_, _| 0x99u32.into_ret());
        }
        e.mem.set_u32(rt, vtable);
        e.register_double(RENDER_TARGET_OF, move |_, _| rt.into_ret());
        let target = 0x7300;
        let manager = e.mem.alloc(0xc);
        let inner = e.mem.alloc(0xc);
        e.mem.set_u32(manager + 8, inner);
        e.mem.set_u32(inner + 8, 0x4400);
        e.mem.set_u32(IMAGE_SPACE_MANAGER, manager);
        e.mem.set_f32(HALF, 0.5);
        e.call(0x00b6_4570, &[base, camera, target]);
        assert_eq!(calls_to(&e, GET_NORMALS_TEXTURE), vec![vec![0x4400, 0]]);
        assert_eq!(calls_to(&e, SET_RENDER_TARGET), vec![vec![0, rt]]);
        let holder = e.mem.u32(RENDER_HOLDER);
        let backgrounds = calls_to(&e, 0x0069_00ac);
        assert_eq!(backgrounds.len(), 2);
        // Group 8 is the +0x5C group: reset by its pass range and by the loop.
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_RESET),
            vec![vec![group], vec![group]]
        );
        assert_eq!(calls_to(&e, RENDERER_CLEAR).len(), 1);
        // The texture's target got the target's value twice (to it and back).
        assert_eq!(calls_to(&e, 0x006a_00ac).len(), 2);
        assert_eq!(e.mem.u32(base + 8), old);
        let _ = holder;
    }

    #[test]
    fn white_background_pass_for_one_or_two_cameras() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        let holder = e.mem.u32(RENDER_HOLDER);
        e.mem.set_u32(holder + 0x200, 1);
        e.mem.set_u8(holder + 0x208, 1);
        e.register(CURRENT_RENDERED_TEXTURE, |_, _| 0x7200u32.into_ret());
        e.register(RENDER_TARGET_OF, |_, a| (a[0] + 1).into_ret());
        let camera = e.mem.alloc(0x110);
        let second = e.mem.alloc(0x110);
        e.call(0x00b6_4a00, &[base, camera, second]);
        assert_eq!(e.mem.u32(base + 8), camera);
        assert_eq!(calls_to(&e, SET_RENDER_TARGET), vec![vec![7, 0x7201]]);
        assert_eq!(calls_to(&e, RENDERER_CLEAR).len(), 2);
        let setups = calls_to(&e, 0x0069_018c);
        assert_eq!(setups.len(), 2);
        assert_eq!(setups[0][1], camera + 0x8c);
        assert_eq!(setups[0][5], camera + 0xdc);
        assert_eq!(setups[1][5], second + 0xdc);
        assert_eq!(setups[1][6], camera + 0x100);
        // Saved, white, restored.
        assert_eq!(calls_to(&e, 0x0069_00ac).len(), 2);
        assert_eq!(calls_to(&e, 0x0069_00b4).len(), 1);
        // The thiscall wrapper `00b65550` passes its first two words on.
        e.call_log = Some(vec![]);
        e.call(0x00b6_5550, &[base, camera, 0, 0xdead]);
        assert_eq!(calls_to(&e, 0x0069_018c).len(), 1);
    }

    #[test]
    fn sun_occlusion_query_renders_the_geometry() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        let shader = e.mem.alloc(0x38);
        let object = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x28);
        e.mem.set_u32(vtable + 0x24, 0x0066_0024);
        e.register(0x0066_0024, |_, _| Ret::default());
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(shader + 0x30, object);
        e.mem.set_u32(shader + 0x34, object);
        e.register_double(GET_SHADER, move |_, _| shader.into_ret());
        e.register(QUERY_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.call(0x00b6_4bf0, &[base, 0x6000, 1, 0x77]);
        let test = base + 0x14;
        let query = e.mem.u32(test + 0x48);
        assert_ne!(query, 0);
        assert_eq!(e.mem.u8(test + 0x4c), 1);
        assert_eq!(calls_to(&e, QUERY_BEGIN), vec![vec![query]]);
        assert_eq!(calls_to(&e, QUERY_END), vec![vec![query]]);
        let pass = calls_to(&e, RENDER_PASS_CONSTRUCTOR)[0][0];
        assert_eq!(
            calls_to(&e, RENDER_PASS_CONSTRUCTOR)[0],
            vec![pass, 0x6000, 9, 1, 0, 0]
        );
        assert_eq!(
            calls_to(&e, BATCH_RENDER_PASS_IMMEDIATELY),
            vec![vec![pass, 9, 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, HOLDER_METHOD_004E9BB0)[0][1], 0x77);
        // Already waiting: nothing more.
        e.call_log = Some(vec![]);
        e.call(0x00b6_4bf0, &[base, 0x6000, 1, 0x77]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // A failed query: fraction 0, nothing rendered.
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.mem.set_u32(base + 0x50, 0x3f80_0000);
        e.call(0x00b6_4bf0, &[base, 0x6000, 0, 0x77]);
        assert_eq!(e.mem.u32(base + 0x50), 0);
        assert!(calls_to(&e, RENDER_PASS_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn alpha_groups_of_a_renderer_are_destroyed() {
        let (mut e, _) = engine(1);
        let r = e.mem.alloc(0x98);
        e.mem.set_u32(r + 0x74, 0x7400);
        let g = e.mem.alloc(0x24);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, g);
        e.mem.set_u32(r + 0x84, node);
        e.mem.set_u32(r + 0x94, 0x9400);
        e.call(0x00b6_4f20, &[r, 1]);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_RESET), vec![vec![0x7400]]);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_DESTRUCTOR), vec![vec![g]]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, g]]
        );
        assert_eq!(calls_to(&e, POINTER_LIST_REMOVE_ALL), vec![vec![r + 0x84]]);
        assert_eq!(e.mem.u32(r + 0x94), 0);
    }

    #[test]
    fn sorted_items_render_back_to_front() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, 0x1111);
        e.mem.set_u32(items + 4, 0x2222);
        e.mem.set_u32(base + 0x24, items);
        e.mem.set_u32(base + 0x1c, 2);
        e.mem.set_u16(base + 0x1a8, 0x2f5);
        // The own list holds two nodes.
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(base + 0xc, first);
        e.mem.set_u32(base + 0x10, second);
        e.mem.set_u32(base + 0x14, 2);
        let mut seen = vec![];
        let log = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = log.clone();
        e.register_double(BATCH_RENDER_PASS_IMMEDIATELY, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        e.call(0x00b6_4f90, &[base]);
        seen.extend(log.borrow().iter().copied());
        assert_eq!(seen, vec![0x2222, 0x1111]);
        assert_eq!(
            calls_to(&e, BATCH_RENDER_PASS_IMMEDIATELY)[0],
            vec![base + 0x1a4, 0x2f5, 1, 1, 1]
        );
        assert_eq!(e.mem.u32(base + 0x14), 0);
        assert_eq!(e.mem.u32(base + 0xc), 0);
        assert_eq!(e.mem.u32(base + 0x10), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), second);
        assert_eq!(calls_to(&e, BATCH_END_PASS).len(), 2);
        // An outside list (+0x18) is emptied instead.
        let list = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(list, node);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(list + 8, 1);
        e.mem.set_u32(base + 0x18, list);
        e.mem.set_u32(base + 0x1c, 0);
        e.call(0x00b6_4f90, &[base]);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), node);
    }

    #[test]
    fn alpha_geometry_renders_the_farther_renderer_first() {
        let (mut e, this) = engine(2);
        let base = this.addr();
        let (a, b) = (renderer(&e, this, 0), renderer(&e, this, 1));
        for r in [a, b] {
            e.mem.set_u32(r + 0x70, 0x7000);
        }
        // a starts at 5, b at 8; each render drops that renderer to -MAX.
        let distances = std::rc::Rc::new(std::cell::RefCell::new([5.0f64, 8.0]));
        let d = distances.clone();
        e.register_double(BATCH_GROUP_DISTANCE, move |_, w| {
            let i = usize::from(w[0] == b);
            d.borrow()[i].into_ret()
        });
        let d = distances.clone();
        e.register_double(BATCH_RENDER_NEXT_ALPHA, move |_, w| {
            let i = usize::from(w[0] == b);
            d.borrow_mut()[i] = f64::from(f32::MIN);
            Ret::default()
        });
        e.call(0x00b6_50c0, &[base, 0]);
        let renders = calls_to(&e, BATCH_RENDER_NEXT_ALPHA);
        assert_eq!(renders.len(), 2);
        assert_eq!(renders[0][0], b);
        assert_eq!(renders[0][4], 5.0f32.to_bits());
        assert_eq!(renders[1][0], a);
        assert_eq!(renders[1][4], f32::MIN.to_bits());
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_RESET).len(), 2);
        assert_eq!(calls_to(&e, BATCH_END_PASS).len(), 2);
    }

    #[test]
    fn item_list_and_active_lists_are_cleared() {
        let (mut e, this) = engine(2);
        let base = this.addr();
        e.call(0x00b6_5570, &[base]);
        assert_eq!(
            calls_to(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![base + 0xc]]
        );
        e.call(0x00b6_5580, &[base]);
        let (a, b) = (renderer(&e, this, 0), renderer(&e, this, 1));
        assert_eq!(
            calls_to(&e, SHORT_LIST_CLEAR),
            vec![vec![a + 0x2c], vec![b + 0x2c]]
        );
    }

    #[test]
    fn active_lists_cleared_for_none() {
        let (mut e, this) = engine(0);
        e.call(0x00b6_5580, &[this.addr()]);
        assert!(calls_to(&e, SHORT_LIST_CLEAR).is_empty());
    }

    #[test]
    fn bound_volumes_are_all_freed() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        let v1 = volume(&mut e, this, 1);
        let v2 = volume(&mut e, this, 2);
        e.mem.set_u32(v1 + 0x14, 0x7700);
        e.call(0x00b6_55b0, &[base]);
        assert_eq!(e.mem.u32(base + 0x148), 0);
        assert_eq!(e.mem.u32(base + 0x140), 0);
        assert_eq!(e.mem.u32(base + 0x144), 0);
        assert_eq!(calls_to(&e, QUERY_DESTRUCTOR), vec![vec![0x7700]]);
        assert_eq!(e.mem.u32(v1 + 0x14), 0);
        let freed: Vec<u32> = calls_to(&e, MEMORY_DEALLOCATE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(freed, vec![v2, 0x7700, v1]);
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 2);
    }

    #[test]
    fn cache_elements_are_emptied() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        for offset in [0x9c, 0xec + 3 * 0x14] {
            e.mem.set_u32(base + offset + 4, 0x40);
            e.mem.set_u32(base + offset + 0x10, 3);
        }
        e.call(0x00b6_5660, &[base]);
        for offset in [0x9c, 0xec + 3 * 0x14] {
            assert_eq!(e.mem.u32(base + offset + 0xc), 0x40);
            assert_eq!(e.mem.u32(base + offset + 4), 0);
            assert_eq!(e.mem.u32(base + offset + 0x10), 0);
        }
    }

    #[test]
    fn destructor_releases_the_members() {
        let (mut e, this) = engine(2);
        let base = this.addr();
        let (a, b) = (renderer(&e, this, 0), renderer(&e, this, 1));
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable, 0x0066_0000);
        e.mem.set_u32(vtable + 4, 0x0066_0004);
        e.register(0x0066_0000, |_, _| Ret::default());
        for r in [a, b] {
            e.mem.set_u32(r, vtable);
        }
        let light = e.mem.alloc(8);
        e.mem.set_u32(light, vtable);
        e.mem.set_u32(light + 4, 1);
        e.mem.set_u32(base + 0x1a0, light);
        e.mem.set_u32(base + 0x48 + 0x14, 0x7700);
        e.mem.set_u32(base + 0x170, 0x1700);
        e.mem.set_u32(base + 0x8c, 0x8c00);
        e.mem.set_u32(base + 0x1b4 + 8, 0xb0c0);
        e.call(0x00b6_56e0, &[base]);
        assert_eq!(e.mem.u32(base), ACCUMULATOR_VTABLE);
        assert_eq!(calls_to(&e, 0x0066_0000), vec![vec![a, 1], vec![b, 1]]);
        assert_eq!(calls_to(&e, 0x0066_0004), vec![vec![light]]);
        assert_eq!(e.mem.u32(base + 0x1a0), 0);
        assert_eq!(calls_to(&e, QUERY_DESTRUCTOR), vec![vec![0x7700]]);
        assert_eq!(
            calls_to(&e, INSTANCE_RENDERER_DESTRUCTOR),
            vec![vec![0x1700]]
        );
        assert_eq!(calls_to(&e, ROOM_MAP_REMOVE_ALL).len(), 3);
        assert_eq!(calls_to(&e, FREE_BUCKETS), vec![vec![0xb0c0]]);
        assert_eq!(e.mem.u32(base + 0x1b4), MAP_BASE_VTABLE);
        assert_eq!(
            calls_to(&e, IMPORT_DELETE_CRITICAL_SECTION),
            vec![vec![base + 0x200]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![
                vec![base + 0xec, 0x14, 4, PERSISTENT_LIST_DESTRUCTOR],
                vec![base + 0x9c, 0x14, 4, PERSISTENT_LIST_DESTRUCTOR],
            ]
        );
        assert_eq!(
            calls_to(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![base + 0x140], vec![base + 0x90]]
        );
        assert_eq!(calls_to(&e, ACCUMULATOR_BASE_DESTRUCTOR), vec![vec![base]]);
        let freed: Vec<u32> = calls_to(&e, MEMORY_DEALLOCATE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(freed[0], 0x8c00);
        assert!(freed.contains(&0x1700));
    }

    #[test]
    fn rtti_and_the_item_register_function() {
        let (mut e, _) = engine(1);
        assert_eq!(e.call(0x00b6_5a80, &[0x1234]).u32(), ACCUMULATOR_RTTI);
        let accumulator = e.mem.alloc(0x18);
        assert!(e.call(0x00b6_5a90, &[accumulator, 0x11, 0, 0, 0]).bool());
        assert!(calls_to(&e, LIST_NODE_ALLOCATE).is_empty());
        assert!(e.call(0x00b6_5a90, &[accumulator, 0x11, 0, 0, 1]).bool());
        assert!(e.call(0x00b6_5a90, &[accumulator, 0x22, 0, 0, 1]).bool());
        let head = e.mem.u32(accumulator + 0xc);
        let tail = e.mem.u32(accumulator + 0x10);
        assert_eq!(e.mem.u32(head + 8), 0x11);
        assert_eq!(e.mem.u32(tail + 8), 0x22);
        assert_eq!(e.mem.u32(head), tail);
        assert_eq!(e.mem.u32(tail + 4), head);
        assert_eq!(e.mem.u32(accumulator + 0x14), 2);
    }

    const RANGE: u32 = BATCH_END_PASS;

    #[test]
    fn mode_zero_pass_by_render_mode() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        // Modes 8 and 7: two pass ranges and the active-list clear.
        e.set(this, BSShaderAccumulator::eRenderMode, 8);
        e.call(0x00b6_5ae0, &[base]);
        assert_eq!(calls_to(&e, RANGE).len(), 2);
        assert_eq!(calls_to(&e, SHORT_LIST_CLEAR).len(), 1);
        e.set(this, BSShaderAccumulator::eRenderMode, 7);
        e.call_log = Some(vec![]);
        e.call(0x00b6_5ae0, &[base]);
        assert_eq!(calls_to(&e, RANGE).len(), 2);
        // Mode 0 with +0x164 set: the split range with clip planes and the
        // tree made once.
        e.set(this, BSShaderAccumulator::eRenderMode, 0);
        e.mem.set_u8(base + 0x164, 1);
        e.mem.set_u8(MAKE_PASS_TREE, 1);
        e.mem.set_u32(GROUP_ZERO_FLAG_011A9608, 5);
        e.call_log = Some(vec![]);
        e.call(0x00b6_5ae0, &[base]);
        assert_eq!(calls_to(&e, CREATE_RENDER_PASS_TREE), vec![vec![base]]);
        assert_eq!(e.mem.u8(MAKE_PASS_TREE), 0);
        let planes = calls_to(&e, SET_CLIP_PLANES);
        assert_eq!(
            planes,
            vec![vec![0, 0, 0, 0, 0, 0, 0], vec![1, 0, 0, 0, 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, DECAL_PASS), vec![vec![base]]);
        assert_eq!(e.mem.u32(GROUP_ZERO_FLAG_011A9608), 5);
        // Three ranges and the alpha geometry's two pass ends.
        assert_eq!(calls_to(&e, RANGE).len(), 5);
        // No camera: nothing.
        e.mem.set_u32(base + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_5ae0, &[base]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn mode_zero_finish_with_the_reflection_copy() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        e.mem.set_u8(base + 0x38, 1);
        e.register(BATCH_HAS_LISTED_PASS, |_, _| 1u32.into_ret());
        e.register(TEXTURE_MANAGER_BORROW, |_, _| 0x4300u32.into_ret());
        e.mem.set_u32(REFLECTION_SOURCE, 0x4200);
        e.mem.set_u32(TEXTURE_MANAGER, 0x7777);
        e.mem.set_u32(RENDERER, 0x9508);
        e.mem.set_u32(IMAGE_SPACE_MANAGER, 0x91ac);
        e.call(0x00b6_5c60, &[base]);
        assert_eq!(
            calls_to(&e, TEXTURE_MANAGER_BORROW),
            vec![vec![0x7777, 0x9508, 5, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, IMAGE_SPACE_RENDER_EFFECT),
            vec![vec![0x91ac, 0x22, 0x9508, 0x4200, 0x4300, 0, 1]]
        );
        let camera = e.mem.u32(base + 8);
        assert_eq!(
            calls_to(&e, HOLDER_METHOD_004E9BB0),
            vec![vec![0x9508, camera]]
        );
        assert_eq!(
            calls_to(&e, TEXTURE_MANAGER_RETURN_RENDERED),
            vec![vec![0x7777, 0x4300]]
        );
        assert_eq!(e.mem.u32(REFLECTION_COPY), 0);
        assert_eq!(
            calls_to(&e, SET_ALPHA_BLEND_ENABLE),
            vec![vec![0, 0], vec![0, 0]]
        );
        // Four ranges and the alpha geometry's two pass ends.
        assert_eq!(calls_to(&e, RANGE).len(), 6);
    }

    #[test]
    fn render_mode_functions_with_a_camera() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        e.mem.set_u32(base + 0x18, 9);
        e.call(0x00b6_5dc0, &[base]);
        assert_eq!(e.mem.u32(base + 0x18), 0);
        assert_eq!(calls_to(&e, 0x0066_00a4), vec![vec![base]]);
        assert_eq!(calls_to(&e, DECAL_PASS), vec![vec![base]]);
        assert_eq!(calls_to(&e, SET_ALPHA_BLEND_ENABLE), vec![vec![0, 0]]);
        assert_eq!(calls_to(&e, SHORT_LIST_CLEAR).len(), 1);
        // 00b65e80: the slot, the sorted items, blending off.
        e.call_log = Some(vec![]);
        e.mem.set_u32(base + 0x18, 9);
        e.call(0x00b6_5e80, &[base]);
        assert_eq!(e.mem.u32(base + 0x18), 0);
        assert_eq!(calls_to(&e, 0x0066_00a4).len(), 1);
        assert_eq!(calls_to(&e, BATCH_END_PASS).len(), 2);
        assert_eq!(calls_to(&e, SET_ALPHA_BLEND_ENABLE), vec![vec![0, 0]]);
        // No camera: neither does anything.
        e.mem.set_u32(base + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_5dc0, &[base]);
        e.call(0x00b6_5e80, &[base]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn sorted_items_mode_function_alone() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        e.call(0x00b6_5e80, &[base]);
        assert_eq!(calls_to(&e, SET_ALPHA_BLEND_ENABLE).len(), 1);
    }

    #[test]
    fn reference_volume_removal() {
        let (mut e, this) = engine(1);
        let base = this.addr();
        let v = volume(&mut e, this, 0x42);
        e.mem.set_u32(v + 0x14, 0x7700);
        e.call(0x00b6_6050, &[base, 0x43]);
        assert!(calls_to(&e, VOLUME_LIST_REMOVE).is_empty());
        e.call(0x00b6_6050, &[base, 0x42]);
        assert_eq!(calls_to(&e, QUERY_DESTRUCTOR), vec![vec![0x7700]]);
        let freed: Vec<u32> = calls_to(&e, MEMORY_DEALLOCATE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(freed, vec![0x7700, v]);
        let removes = calls_to(&e, VOLUME_LIST_REMOVE);
        assert_eq!(removes.len(), 1);
        assert_eq!(removes[0][0], base + 0x140);
    }
}

#[cfg(test)]
mod n012_tests {
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x011c_5000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// An accumulator with one batch renderer (its own small vtable) and the pass arrays
    /// `00b63770` uses; `BSBatchRenderer` calls doubled to report no lists.
    fn accumulator(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x210);
        let renderer = e.mem.alloc(0x98);
        e.mem.set_u32(this + 0x174, renderer);
        e.mem.set_u32(this + 0x18c, 1);
        for offset in [0x17cu32, 0x180, 0x184, 0x188] {
            let array = e.mem.alloc(8);
            e.mem.set_u32(this + offset, array);
        }
        e.register(BATCH_NEXT_LIST, |_, _| Ret::default());
        e.register(BATCH_END_PASS, |_, _| Ret::default());
        e.register(SHORT_LIST_CLEAR, |_, _| Ret::default());
        (this, renderer)
    }

    #[test]
    fn create_render_pass_tree_names_and_updates_a_node() {
        let mut e = rig();
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(FIXED_STRING_EMPTY_0109B220, 0x5555);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NODE_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_MAKE, |_, _| 0x7008u32.into_ret());
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register(UPDATE_00A59C60, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 0.0);
            assert_eq!(e.mem.bytes(a[1] + 4, 5), vec![0; 5]);
            Ret::default()
        });
        let node = e.call(0x00b6_3420, &args![]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0xac]]);
        assert_eq!(calls(&e, NI_NODE_CONSTRUCTOR), vec![vec![node, 0]]);
        assert_eq!(
            calls(&e, FIXED_STRING_MAKE),
            vec![vec![PASS_TREE_NAME_010ADFAC]]
        );
        assert_eq!(calls(&e, SET_NAME)[0][0], node);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT), vec![vec![0x7000]]);
        assert_eq!(calls(&e, UPDATE_00A59C60)[0][0], node);
        // The shared handle is not released.
        e.register(FIXED_STRING_MAKE, |_, _| 0x5555u32.into_ret());
        e.call(0x00b6_3420, &args![]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn fn_00b63620_drops_each_query() {
        let mut e = rig();
        e.register(QUERY_RESULT_00C4EDF0, |e, a| {
            assert_eq!((e.mem.u32(a[1]), a[2]), (0, 1));
            e.mem.set_u32(a[1], 9);
            Ret::default()
        });
        e.register(QUERY_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this + 0x48, 0x6000);
        e.mem.set_u8(this + 0x4c, 1);
        e.mem.set_u32(this + 0x70, 0x6100);
        e.mem.set_u8(this + 0x74, 1);
        e.mem.set_u8(this + 0x60, 1);
        e.mem.set_u8(this + 0x40, 1);
        e.call(0x00b6_3620, &args![this]);
        assert_eq!(
            calls(&e, QUERY_DESTRUCTOR),
            vec![vec![0x6000], vec![0x6100]]
        );
        assert_eq!(calls(&e, QUERY_RESULT_00C4EDF0).len(), 2);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x6000], vec![MEMORY_MANAGER, 0x6100]]
        );
        assert_eq!((e.mem.u32(this + 0x48), e.mem.u8(this + 0x4c)), (0, 0));
        assert_eq!((e.mem.u32(this + 0x70), e.mem.u8(this + 0x74)), (0, 0));
        assert_eq!(e.mem.u8(this + 0x60), 1);
        assert_eq!(e.mem.u8(this + 0x40), 0);
    }

    #[test]
    fn fn_00b63b10_adds_new_keys_only() {
        let mut e = rig();
        e.register(IMPORT_ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(IMPORT_LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(ROOM_MAP_GET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), a[1]);
            u32::from(a[1] == 7).into_ret()
        });
        e.register(ROOM_MAP_SET_AT, |_, _| Ret::default());
        let this = e.mem.alloc(0x210);
        assert!(e.call(0x00b6_3b10, &args![this, 3u32]).bool());
        assert!(!e.call(0x00b6_3b10, &args![this, 7u32]).bool());
        assert_eq!(calls(&e, ROOM_MAP_SET_AT), vec![vec![this + 0x1b4, 3, 1]]);
        assert_eq!(calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len(), 2);
    }

    #[test]
    fn fn_00b63bd0_stores_value_and_flag() {
        let mut e = rig();
        let item = e.mem.alloc(0xc);
        e.call(0x00b6_3bd0, &args![0u32, item, 0x1234u32, 0x1ffu32]);
        assert_eq!((e.mem.u32(item + 4), e.mem.u8(item + 8)), (0x1234, 0xff));
    }

    #[test]
    fn fn_00b63bf0_fills_the_head_then_appends() {
        let mut e = rig();
        let list = e.mem.alloc(8);
        let value = e.mem.alloc(4);
        e.mem.set_u16(value, 0);
        e.call(0x00b6_3bf0, &args![list, value]);
        assert_eq!(e.mem.u16(list), 0);
        e.mem.set_u16(value, 0x2f0);
        e.call(0x00b6_3bf0, &args![list, value]);
        assert_eq!(e.mem.u16(list), 0x2f0);
        assert_eq!(e.mem.u32(list + 4), 0);
        e.mem.set_u16(value, 0x2f1);
        e.call(0x00b6_3bf0, &args![list, value]);
        e.mem.set_u16(value, 0x2f2);
        e.call(0x00b6_3bf0, &args![list, value]);
        let second = e.mem.u32(list + 4);
        let third = e.mem.u32(second + 4);
        assert_eq!((e.mem.u16(second), e.mem.u16(third)), (0x2f1, 0x2f2));
        assert_eq!(e.mem.u32(third + 4), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 2);
    }

    #[test]
    fn fn_00b63c60_makes_and_fills_the_fog_texture() {
        let mut e = rig();
        let this = e.mem.alloc(0xa0);
        let holder = e.mem.alloc(0xc);
        e.mem.set_u32(this + 0x98, holder);
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| {
            let holder = e.mem.u32(a[0] + 4);
            let v = e.mem.u32(a[2]);
            e.mem.set_u32(holder + 4 * a[1], v);
            Ret::default()
        });
        e.register(IMAGE_SPACE_TEXTURE_GET_RENDERED, |e, a| {
            e.mem.u32(a[0] + 0xc).into_ret()
        });
        e.register(IMAGE_SPACE_TEXTURE_SET, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        e.register(TEXTURE_MANAGER_BORROW, |_, _| 0x8800u32.into_ret());
        e.mem.set_u32(RENDER_HOLDER, 0x4747);
        e.mem.set_u32(TEXTURE_MANAGER, 0x91a8);
        assert_eq!(e.call(0x00b6_3c60, &args![this]).u32(), 0x8800);
        let texture = e.mem.u32(holder + 8);
        assert_eq!(calls(&e, ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x94, 2]);
        assert_eq!(
            calls(&e, TEXTURE_MANAGER_BORROW),
            vec![vec![0x91a8, 0x4747, 0x20, 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, IMAGE_SPACE_TEXTURE_SET),
            vec![vec![texture, 0x8800]]
        );
        // Made and filled: nothing more to do.
        assert_eq!(e.call(0x00b6_3c60, &args![this]).u32(), 0x8800);
        assert_eq!(calls(&e, TEXTURE_MANAGER_BORROW).len(), 1);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
    }

    #[test]
    fn map_destructors_empty_and_free() {
        let mut e = rig();
        e.register(ROOM_MAP_REMOVE_ALL, |e, a| {
            assert!(e.mem.u32(a[0]) == MAP_VTABLE || e.mem.u32(a[0]) == MAP_BASE_VTABLE);
            Ret::default()
        });
        e.register(FREE_BUCKETS, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x9900);
        e.call(0x00b6_3d60, &args![map]);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(calls(&e, ROOM_MAP_REMOVE_ALL).len(), 1);
        e.call(0x00b6_3d80, &args![map]);
        assert_eq!(calls(&e, ROOM_MAP_REMOVE_ALL).len(), 3);
        assert_eq!(calls(&e, FREE_BUCKETS), vec![vec![0x9900], vec![0x9900]]);
        assert_eq!(e.call(0x00b6_3e30, &args![map, 0u32]).u32(), map);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00b6_3e30, &args![map, 1u32]).u32(), map);
        assert_eq!(e.call(0x00b6_3e70, &args![map, 1u32]).u32(), map);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, map], vec![MEMORY_MANAGER, map]]
        );
        assert_eq!(calls(&e, ROOM_MAP_REMOVE_ALL).len(), 7);
    }

    #[test]
    fn fn_00b63df0_returns_the_item_to_the_pool() {
        let mut e = rig();
        e.register(LOCK_ENTER, |e, _| {
            let n = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, n + 1);
            e.mem.set_u32(NODE_POOL_LOCK, 0x1111);
            Ret::default()
        });
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x2222);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 4, 5);
        e.mem.set_u8(item + 8, 1);
        e.call(0x00b6_3df0, &args![0u32, item]);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), item);
        assert_eq!(e.mem.u32(item), 0x2222);
        assert_eq!((e.mem.u32(item + 4), e.mem.u8(item + 8)), (0, 0));
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        // A lock still held: the owner stays.
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.call(0x00b6_3df0, &args![0u32, item]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0x1111);
    }

    #[test]
    fn register_functions_filter_then_forward() {
        let mut e = rig();
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f64(LIMIT_0101E2C0, 50.0);
        e.register(REGISTER_00B64020, |_, _| 0x100u32.into_ret());
        let geometry = e.mem.alloc(0xc0);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0xb8, object);
        let shader = e.mem.alloc(0x70);
        let property = e.mem.alloc(0x60);
        e.mem.set_u32(property + 0x24, 0x40000);
        let words = |state: u32| args![0x10u32, geometry, state, property, shader];
        e.mem.set_f32(object + 0x1c, 49.5);
        assert!(e.call(0x00b6_4440, &words(1)).bool());
        e.mem.set_f32(object + 0x1c, f32::NAN);
        assert!(!e.call(0x00b6_4440, &words(2)).bool());
        e.mem.set_f32(object + 0x1c, 50.0);
        e.mem.set_u32(shader + 0x68, 0x16);
        assert!(e.call(0x00b6_4440, &words(3)).bool());
        assert!(e.call(0x00b6_4490, &words(4)).bool());
        e.mem.set_u32(shader + 0x68, 0x15);
        e.mem.set_u32(property + 0x24, 0x3ffff);
        assert!(e.call(0x00b6_4440, &words(5)).bool());
        assert!(!e.call(0x00b6_4490, &words(6)).bool());
        assert_eq!(calls(&e, REGISTER_00B64020), vec![words(2), words(6)]);
    }

    #[test]
    fn fn_00b63f90_and_00b644b0_hand_passes_to_the_thread_renderer() {
        let mut e = rig();
        let (this, renderer) = accumulator(&mut e);
        let other = e.mem.alloc(0x98);
        e.mem.set_u32(this + 0x178, other);
        e.mem.set_u32(this + 0x18c, 2);
        e.mem.set_u32(this + 0x19c, 6);
        e.mem.set_u32(SHADER_FLAGS_011F91D8, 0x2f);
        let mut slots = [0u32; 0x30];
        slots[0x8c / 4] = 0x0f00_008c;
        slots[0xa0 / 4] = 0x0f00_00a0;
        slots[0xb8 / 4] = 0x0f00_00b8;
        e.put_vtable(0x0f00_1000, &slots);
        e.mem.set_u32(renderer, 0x0f00_1000);
        e.mem.set_u32(other, 0x0f00_1000);
        e.register(0x0f00_008c, |_, _| Ret::default());
        let list = e.mem.alloc(0x14);
        let passes = e.mem.alloc(8);
        let zero = e.mem.alloc(0x10);
        let lit = e.mem.alloc(0x10);
        e.mem.set_u16(lit + 4, 0x2d);
        e.mem.set_u32(passes, zero);
        e.mem.set_u32(passes + 4, lit);
        e.mem.set_u32(list + 4, passes);
        e.mem.set_u32(list + 0x10, 2);
        let property = e.mem.alloc(0x60);
        e.mem.set_u32(property, 0x0f00_1000);
        e.register_double(0x0f00_00a0, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0);
            list.into_ret()
        });
        e.register_double(0x0f00_00b8, move |_, _| list.into_ret());
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2BC, 1);
        assert!(e
            .call(0x00b6_3f90, &args![this, 0x77u32, 0u32, property, 0u32])
            .bool());
        let asked = calls(&e, 0x0f00_00a0);
        assert_eq!(asked[0][0..3], [property, 0x77, 0x2f]);
        assert_eq!(asked[0][4..], [6, this, 1]);
        assert_eq!(calls(&e, 0x0f00_008c), vec![vec![other, lit, 0x2d]]);
        // 00b644b0: nothing without the byte +0x165, then every pass.
        assert!(e
            .call(0x00b6_44b0, &args![this, 0x77u32, 0u32, property, 0u32])
            .bool());
        assert!(calls(&e, 0x0f00_00b8).is_empty());
        e.mem.set_u8(this + 0x165, 1);
        e.mem.set_u32(tls + TLS_WORD_2BC, 0);
        assert!(e
            .call(0x00b6_44b0, &args![this, 0x77u32, 0u32, property, 0u32])
            .bool());
        assert_eq!(calls(&e, 0x0f00_00b8), vec![vec![property, 0x77]]);
        assert_eq!(
            calls(&e, 0x0f00_008c)[1..],
            [vec![renderer, zero, 0], vec![renderer, lit, 0x2d]]
        );
    }

    /// An accumulator for `00b64020` with renderers 0 (`[thread % 2]`) and 2 (`[thread]`,
    /// thread word 2), a property whose slots +0xa0, +0xa8 and +0xac answer `list`, `late`
    /// and `depth`, and the batch calls doubled. Answers (accumulator, renderer 0, renderer
    /// 2, property).
    fn register_rig(e: &mut Engine, list: u32, late: u32, depth: u32) -> (u32, u32, u32, u32) {
        let (this, renderer) = accumulator(e);
        let other = e.mem.alloc(0x98);
        e.mem.set_u32(this + 0x17c, other);
        e.mem.set_u32(this + 0x18c, 2);
        e.mem.set_u32(this + 0x19c, 6);
        e.mem.set_u32(SHADER_FLAGS_011F91D8, 0x2f);
        let mut slots = [0u32; 0x30];
        slots[0x8c / 4] = 0x0f00_008c;
        slots[0xa0 / 4] = 0x0f00_00a0;
        slots[0xa8 / 4] = 0x0f00_00a8;
        slots[0xac / 4] = 0x0f00_00ac;
        e.put_vtable(0x0f00_1000, &slots);
        e.mem.set_u32(other, 0x0f00_1000);
        e.register(0x0f00_008c, |_, _| Ret::default());
        e.register_double(0x0f00_00a0, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0);
            list.into_ret()
        });
        e.register_double(0x0f00_00a8, move |_, _| late.into_ret());
        e.register_double(0x0f00_00ac, move |_, _| depth.into_ret());
        e.register(BATCH_REGISTER_00B9B3B0, |_, _| Ret::default());
        e.register(BATCH_REGISTER_00B9B3D0, |_, _| Ret::default());
        let property = e.mem.alloc(0x60);
        e.mem.set_u32(property, 0x0f00_1000);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2BC, 2);
        (this, renderer, other, property)
    }

    /// A pass array of `passes` (count `+0x10`, items `+4`).
    fn pass_list(e: &mut Engine, passes: &[u32]) -> u32 {
        let list = e.mem.alloc(0x14);
        let items = e.mem.alloc(4 * passes.len() as u32 + 4);
        for (i, pass) in passes.iter().enumerate() {
            e.mem.set_u32(items + 4 * i as u32, *pass);
        }
        e.mem.set_u32(list + 4, items);
        e.mem.set_u32(list + 0x10, passes.len() as u32);
        list
    }

    /// A pass with its object (`+0`), enum (`+4`) and type byte (`+6`).
    fn pass(e: &mut Engine, object: u32, pass_enum: u16, kind: u8) -> u32 {
        let pass = e.mem.alloc(0x10);
        e.mem.set_u32(pass, object);
        e.mem.set_u16(pass + 4, pass_enum);
        e.mem.set_u8(pass + 6, kind);
        pass
    }

    #[test]
    fn fn_00b64020_registers_each_pass_by_type() {
        let mut e = rig();
        let object = e.mem.alloc(0x30);
        let lit = e.mem.alloc(0x10);
        e.mem.set_f32(lit + 8, f32::NAN);
        let lit_object = e.mem.alloc(0x30);
        e.mem.set_u32(lit_object + 0x20, lit);
        e.mem.set_f32(DEFAULT_OBJECT_011F4288 + 8, 4.5);
        let mut passes = vec![
            pass(&mut e, object, 0x31, 1),
            pass(&mut e, lit_object, 0x32, 1),
            pass(&mut e, object, 0, 2),
            pass(&mut e, object, 0x40, 0),
            pass(&mut e, object, 0x41, 15),
        ];
        for kind in 2..=14u8 {
            passes.push(pass(&mut e, object, 0x100 + u16::from(kind), kind));
        }
        let list = pass_list(&mut e, &passes);
        let (this, renderer, other, property) = register_rig(&mut e, list, 0, 0x7777);
        // Byte +0x164 and a shader of type 0xd: nothing.
        let shader = e.mem.alloc(0x70);
        e.mem.set_u32(shader + 0x68, 0xd);
        e.mem.set_u8(this + 0x164, 1);
        let words = args![this, 0x77u32, 0u32, property, shader];
        assert!(e.call(0x00b6_4020, &words).bool());
        assert!(calls(&e, 0x0f00_00a0).is_empty());
        e.mem.set_u32(shader + 0x68, 0xc);
        e.mem.set_u8(this + 0x3a, 1);
        e.mem.set_u8(this + 0x166, 1);
        e.mem.set_u32(this + 0x168, 5);
        e.mem.set_u8(this + 0x16c, 1);
        e.mem.set_u32(renderer + 0x34, 1);
        e.mem.set_u8(renderer + 0x38, 1);
        let changed = e.mem.alloc(0x14);
        e.mem.set_u32(changed + 0x10, 9);
        e.mem.set_u32(property + 0x3c, changed);
        e.mem.set_u32(property + 0x38, 0x55);
        e.mem.set_u32(property + 0x24, 0x41);
        assert!(e.call(0x00b6_4020, &words).bool());
        assert_eq!(calls(&e, 0x0f00_00ac), vec![vec![property, 0x77]]);
        let asked = calls(&e, 0x0f00_00a0);
        assert_eq!(asked[0][0..3], [property, 0x77, 0x2f]);
        assert_eq!(asked[0][4..], [6, this, 1]);
        assert_eq!(e.mem.u32(property + 0x20), 0x200);
        assert_eq!(e.mem.u32(property + 0x24), 1);
        assert_eq!(e.mem.u32(property + 0x38), 0);
        assert_eq!(e.mem.u32(changed + 0x10), 0);
        let p = |i: usize| passes[i];
        assert_eq!(
            calls(&e, BATCH_REGISTER_00B9B3B0),
            vec![
                vec![other, 0x7777, 0xb, 0],
                vec![other, p(5), 0xa, 0],
                vec![other, p(6), 1, 0],
                vec![other, p(7), 2, 1],
                vec![other, p(8), 3, 0],
                vec![other, p(9), 6, 0],
                vec![other, p(10), 8, 0],
                vec![other, p(13), 0, 0],
                vec![other, p(17), 9, 0],
            ]
        );
        assert_eq!(
            calls(&e, BATCH_REGISTER_00B9B3D0),
            vec![vec![other, p(0), 1], vec![other, p(1), 0]]
        );
        assert_eq!(
            calls(&e, 0x0f00_008c),
            vec![vec![other, p(3), 0x40], vec![other, p(4), 0x41]]
        );
        assert!(calls(&e, 0x0f00_00a8).is_empty());
        // Without decals the types 4 and 5 register nothing; the bit set stays.
        e.mem.set_u8(this + 0x16c, 0);
        e.mem.set_u8(this + 0x3a, 0);
        e.mem.set_u32(property + 0x38, 0x55);
        assert!(e.call(0x00b6_4020, &words).bool());
        assert_eq!(calls(&e, BATCH_REGISTER_00B9B3B0).len(), 9 + 6);
        assert_eq!(e.mem.u32(property + 0x38), 0x55);
        assert_eq!(calls(&e, 0x0f00_00ac).len(), 1);
    }

    #[test]
    fn fn_00b64020_first_person_late_passes() {
        let mut e = rig();
        let object = e.mem.alloc(0x30);
        let skipped = pass(&mut e, object, 0, 3);
        let list = pass_list(&mut e, &[skipped]);
        let first = pass(&mut e, object, 0x51, 0);
        let second = pass(&mut e, object, 0x52, 0);
        let late = pass_list(&mut e, &[first, second]);
        let (this, renderer, other, property) = register_rig(&mut e, list, late, 0x7777);
        e.mem.set_u8(this + 0x3a, 1);
        e.mem.set_u8(this + 0x84, 1);
        e.mem.set_u8(this + 0x85, 1);
        e.mem.set_u32(renderer + 0x34, 0);
        e.mem.set_u8(renderer + 0x38, 1);
        e.mem.set_u32(property + 0x20, 0x201);
        e.mem.set_u32(property + 0x38, 0x55);
        // The renderer's slot +0x8c clears bAccumulate: the second pass renders now.
        e.register_double(0x0f00_008c, move |e, _| {
            e.mem.set_u8(this + 0x84, 0);
            Ret::default()
        });
        let shader = e.mem.alloc(0x40);
        let mut slots = [0u32; 0x10];
        slots[0x24 / 4] = 0x0f00_0024;
        e.put_vtable(0x0f00_3000, &slots);
        let (a, b) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(a, 0x0f00_3000);
        e.mem.set_u32(b, 0x0f00_3000);
        e.mem.set_u32(shader + 0x34, a);
        e.mem.set_u32(shader + 0x30, b);
        e.register(0x0f00_0024, |_, _| Ret::default());
        e.register_double(GET_SHADER, move |_, _| shader.into_ret());
        e.register(SHADER_SETUP_00B7A870, |_, _| Ret::default());
        e.register(RENDER_PASS_00B63320, |_, _| Ret::default());
        let words = args![this, 0x77u32, 0u32, property, 0u32];
        assert!(e.call(0x00b6_4020, &words).bool());
        assert_eq!(
            calls(&e, BATCH_REGISTER_00B9B3B0),
            vec![vec![other, 0x7777, 0xc, 0]]
        );
        assert_eq!(e.mem.u32(property + 0x20), 1);
        assert_eq!(e.mem.u32(property + 0x24), 0x40);
        assert_eq!(e.mem.u32(property + 0x38), 0);
        assert_eq!(calls(&e, 0x0f00_008c), vec![vec![other, first, 0]]);
        assert_eq!(calls(&e, GET_SHADER), vec![vec![1]]);
        assert_eq!(calls(&e, SHADER_SETUP_00B7A870), vec![vec![0x52, 0]]);
        assert_eq!(calls(&e, 0x0f00_0024), vec![vec![a], vec![b]]);
        assert_eq!(calls(&e, RENDER_PASS_00B63320), vec![vec![second, 0x52]]);
        // No shader: the two slots are skipped.
        e.register_double(GET_SHADER, |_, _| 0u32.into_ret());
        e.mem.set_u8(this + 0x84, 1);
        assert!(e.call(0x00b6_4020, &words).bool());
        assert_eq!(calls(&e, 0x0f00_0024).len(), 2);
        assert_eq!(calls(&e, RENDER_PASS_00B63320).len(), 2);
        // Without bAccumulate the late list is not asked for.
        assert!(e.call(0x00b6_4020, &words).bool());
        assert_eq!(calls(&e, 0x0f00_00a8).len(), 2);
    }

    #[test]
    fn fn_00b64530_renders_the_late_passes() {
        let mut e = rig();
        let (this, renderer) = accumulator(&mut e);
        e.register(PASS_STATE_00B97FA0, |_, _| Ret::default());
        e.mem.set_u32(COUNTER_011FF9F0, 3);
        e.call(0x00b6_4530, &args![this]);
        assert!(calls(&e, PASS_STATE_00B97FA0).is_empty());
        e.mem.set_u32(this + 8, 1);
        e.call(0x00b6_4530, &args![this]);
        assert_eq!(calls(&e, PASS_STATE_00B97FA0), vec![vec![0, 1], vec![0, 0]]);
        assert_eq!(
            (e.mem.u32(renderer + 0x20), e.mem.u32(renderer + 0x24)),
            (0x2d8, 0x2ef)
        );
        assert_eq!(e.mem.u32(COUNTER_011FF9F0), 2);
    }

    #[test]
    fn fn_00b65ec0_adds_renders_and_clears_the_types() {
        let mut e = rig();
        let (this, renderer) = accumulator(&mut e);
        e.call(0x00b6_5ec0, &args![this]);
        let list = renderer + 0x2c;
        assert_eq!(e.mem.u16(list), 0x2f0);
        let next = e.mem.u32(list + 4);
        assert_eq!(e.mem.u16(next), 0x2f1);
        assert_eq!(
            (e.mem.u32(renderer + 0x20), e.mem.u32(renderer + 0x24)),
            (0x2f0, 0x2f1)
        );
        assert_eq!(calls(&e, SHORT_LIST_CLEAR), vec![vec![list]]);
    }

    #[test]
    fn fn_00b65f60_updates_or_adds_a_reference_volume() {
        let mut e = rig();
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let this = e.mem.alloc(0x210);
        let point = e.mem.alloc(0x10);
        for (k, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(point + 4 * k as u32, *v);
        }
        e.mem.set_u32(point + 0xc, 0x7f80_0001);
        e.call(0x00b6_5f60, &args![this, 0x500u32, point, 0u8]);
        let head = e.mem.u32(this + 0x140);
        assert_eq!(e.mem.u32(this + 0x144), head);
        assert_eq!(e.mem.u32(this + 0x148), 1);
        let entry = e.mem.u32(head + 8);
        assert_eq!(e.mem.f32(entry + 8), 3.0);
        assert_eq!(e.mem.u32(entry + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(entry + 0x10), 0x500);
        assert_eq!(e.mem.u8(entry + 0x1a), 1);
        // A second reference goes in front.
        e.call(0x00b6_5f60, &args![this, 0x600u32, point, 0u8]);
        let first = e.mem.u32(this + 0x140);
        assert_eq!(e.mem.u32(first), head);
        assert_eq!(e.mem.u32(head + 4), first);
        assert_eq!(e.mem.u32(this + 0x148), 2);
        // The first one again: updated in place with the byte.
        e.mem.set_f32(point, 9.0);
        e.call(0x00b6_5f60, &args![this, 0x500u32, point, 0u8]);
        assert_eq!(e.mem.f32(entry), 9.0);
        assert_eq!(e.mem.u8(entry + 0x1a), 0);
        assert_eq!(e.mem.u32(this + 0x148), 2);
    }

    #[test]
    fn fn_00b651e0_renders_each_fog_geometry() {
        let mut e = rig();
        let this = e.mem.alloc(0x210);
        let renderer = e.mem.alloc(0x210);
        e.mem.set_u32(RENDER_HOLDER, renderer);
        let mut slots = [0u32; 0x70];
        for slot in [0xa4u32, 0xac, 0xb4, 0xdc, 0x18c] {
            slots[slot as usize / 4] = 0x0f00_0000 + slot;
            e.register(0x0f00_0000 + slot, |_, _| Ret::default());
        }
        e.put_vtable(0x0f00_1000, &slots);
        e.mem.set_u32(this, 0x0f00_1000);
        e.mem.set_u32(renderer, 0x0f00_1000);
        // The fog texture holder (`fog_owner` reaches it through the image-space manager).
        let manager = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0xb0);
        let effect = e.mem.alloc(0x70);
        let owner = e.mem.alloc(0xa0);
        let holder = e.mem.alloc(0xc);
        e.mem.set_u32(holder + 8, 0x3330);
        e.mem.set_u32(owner + 0x98, holder);
        e.mem.set_u32(inner + 0xa8, owner + 0x6c);
        e.mem.set_u32(inner + 0x34, effect);
        e.mem.set_u32(manager + 8, inner);
        e.mem.set_u32(IMAGE_SPACE_MANAGER, manager);
        for addr in [
            RENDERER_CLEAR,
            RENDER_PASS_CONSTRUCTOR,
            RENDER_PASS_DESTRUCTOR,
            SET_RENDER_TARGET,
            POINTER_LIST_REMOVE_ALL,
            IMAGE_SPACE_RENDER_EFFECT,
            TEXTURE_MANAGER_RETURN_RENDERED,
            IMAGE_SPACE_TEXTURE_SET,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(IMAGE_SPACE_TEXTURE_GET_RENDERED, |_, a| {
            (a[0] + 1).into_ret()
        });
        e.register(RENDER_TARGET_OF, |_, a| (a[0] + 0x10).into_ret());
        // Nothing without the word at +0x98.
        assert_eq!(e.call(0x00b6_51e0, &args![this, 0x40u32, 0x50u32]).u32(), 0);
        assert!(calls(&e, RENDER_PASS_CONSTRUCTOR).is_empty());
        e.mem.set_u32(this + 0x98, 2);
        let geometry = |e: &mut Engine, kind: u32| {
            let g = e.mem.alloc(0xb0);
            e.mem.set_u32(g, 0x0f00_1000);
            let p = e.mem.alloc(0x70);
            e.mem.set_u32(p + 0x1c, kind);
            for k in 0..4 {
                e.mem.set_u32(p + 0x60 + 4 * k, 0x60 + k);
            }
            e.mem.set_u32(g + 0xa8, p);
            g
        };
        let fog = geometry(&mut e, 0x16);
        let plain = geometry(&mut e, 0x15);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, plain);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, fog);
        e.mem.set_u32(this + 0x90, first);
        let camera = e.mem.alloc(0x110);
        e.mem.set_u32(this + 8, 0x7777);
        assert_eq!(e.call(0x00b6_51e0, &args![this, camera, 0x50u32]).u32(), 1);
        assert_eq!(e.mem.u32(this + 8), 0x7777);
        assert_eq!(e.mem.u32(this + 0x18), this + 0x90);
        assert_eq!(calls(&e, 0x0f00_00a4), vec![vec![this]]);
        // Rendered twice (the byte at 011ad80e clear), with 0x25a then 0x259.
        assert_eq!(calls(&e, 0x0f00_00dc), vec![vec![fog, renderer]; 2]);
        assert_eq!(e.mem.u32(CURRENT_PASS_ENUM_011F91E4), 0x259);
        assert_eq!(
            calls(&e, SET_RENDER_TARGET),
            vec![vec![3, 0x3331 + 0x10], vec![0, 0x60]]
        );
        assert_eq!(
            calls(&e, IMAGE_SPACE_RENDER_EFFECT),
            vec![vec![manager, 0xd, renderer, 0x50, 0x50, 0, 1]]
        );
        assert_eq!(e.mem.u32(effect + 0x58), 0x60);
        assert_eq!(e.mem.u32(effect + 0x64), 0x63);
        assert_eq!(e.mem.u8(effect + 5), 1);
        assert_eq!(calls(&e, POINTER_LIST_REMOVE_ALL), vec![vec![this + 0x90]]);
        let saved = calls(&e, 0x0f00_00b4)[0][1];
        assert_eq!(calls(&e, 0x0f00_00ac).last().unwrap()[1], saved);
        assert_eq!(calls(&e, RENDER_PASS_DESTRUCTOR).len(), 1);
        // With the byte set: once, keeping the enum 0x25b.
        e.mem.set_u8(FLAG_011AD80E, 1);
        e.call(0x00b6_51e0, &args![this, camera, 0x50u32]);
        assert_eq!(calls(&e, 0x0f00_00dc).len(), 3);
        assert_eq!(e.mem.u32(CURRENT_PASS_ENUM_011F91E4), 0x25b);
    }
}
