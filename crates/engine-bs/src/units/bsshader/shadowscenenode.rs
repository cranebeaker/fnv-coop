//! `BSShader/shadowscenenode.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00cb9ce0` (unnamed in the engine map): `thiscall`, `RET 8` (new capacity, element count);
/// reallocates the array's buffer.
const ARRAY_REALLOCATE: u32 = 0x00cb_9ce0;
/// The virtual slot (byte offset) that allocates a block of the requested number of elements.
const SLOT_ALLOCATE: u32 = 0x04;

/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,NiPointer<NiPSysModifier>>::RemoveAll`
/// (Xbox PDB name of `00e74d40`, `thiscall`, no stack words): the jump target of `00b5c7c0`.
const LIST_REMOVE_ALL_00E74D40: u32 = 0x00e7_4d40;

// Translated from 00b5c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `BSShader/shadowscenenode.cpp`; Ghidra made no function
/// of it, read with `disassemble_bytes`): a 5-byte jump to `00e74d40` (the `NiTPointerList`
/// of `NiPointer`s' `RemoveAll`), passing on `this` (`ECX`); no stack words.
pub fn fn_00b5c7c0(e: &mut Engine, this: Ptr) {
    e.call(LIST_REMOVE_ALL_00E74D40, &args![this]);
}

// Translated from 00b5c360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowscenenode.cpp`). `__fastcall`: reserves the next slot of a
/// `BSSimpleArray`-shaped array (vtable, buffer at +4, count at +8, capacity at +0xc) and answers
/// its index. When the count equals the capacity: with no capacity yet, a four-element buffer
/// comes from virtual slot +4; otherwise the capacity doubles (grows by `0x400` once it is above
/// `0x400`) through `00cb9ce0`. This is the same body as the generic slot reservation
/// `00761540`, with the capacity helper inlined.
pub fn fn_00b5c360(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let size = e.mem.u32(base.wrapping_add(8));
    let capacity = e.mem.u32(base.wrapping_add(0xc));
    if size == capacity {
        if capacity == 0 {
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![4u32]).u32();
            let count = e.mem.u32(base.wrapping_add(8)).wrapping_add(1);
            e.mem.set_u32(base.wrapping_add(8), count);
            e.mem.set_u32(base.wrapping_add(4), buffer);
            e.mem.set_u32(base.wrapping_add(0xc), 4);
            return count.wrapping_sub(1);
        }
        let new_capacity = if capacity > 0x400 {
            capacity.wrapping_add(0x400)
        } else {
            capacity.wrapping_mul(2)
        };
        e.call(ARRAY_REALLOCATE, &args![this, new_capacity, size]);
        e.mem.set_u32(base.wrapping_add(0xc), new_capacity);
    }
    let count = e.mem.u32(base.wrapping_add(8)).wrapping_add(1);
    e.mem.set_u32(base.wrapping_add(8), count);
    count.wrapping_sub(1)
}

layout! {
    /// `ShadowSceneNode` (Xbox PDB), an `NiNode`; 0x200 bytes on PC (`InitSceneGraph`
    /// allocates 0x200 for it at `0086d633`). Its own fields sit 0x24 lower on PC than in the
    /// Xbox PDB (the PC `NiNode` is 0xac bytes); the fields the constructor sets.
    pub struct ShadowSceneNode: 0x200 {
        /// `m_kWorldBound` (Xbox PDB, `NiAVObject`): `BSNiBound*`.
        0x20 m_kWorldBound: u32,
        /// `m_uFlags` (Xbox PDB, `NiAVObject`).
        0x30 m_uFlags: u32,
        /// `kShadowLightActiveFence` (Xbox PDB).
        0xD0 kShadowLightActiveFence: u32,
        /// `kShadowLightFadingInFence` (Xbox PDB).
        0xD4 kShadowLightFadingInFence: u32,
        /// `kShadowLightActiveFenceObject` (Xbox PDB): `NiPointer<ShadowSceneLight>`.
        0xD8 kShadowLightActiveFenceObject: u32,
        /// `kShadowLightFadingInFenceObject` (Xbox PDB): `NiPointer<ShadowSceneLight>`.
        0xDC kShadowLightFadingInFenceObject: u32,
        /// `pSunLight` (Xbox PDB): `ShadowSceneLight*`.
        0xE0 pSunLight: u32,
        /// `cSceneGraphIndex` (Xbox PDB).
        0x120 cSceneGraphIndex: u8,
        /// `pWindowReflectionEnvMap` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x124 pWindowReflectionEnvMap: u32,
        /// `spCubeMapCamera` (Xbox PDB): `NiPointer<BSCubeMapCamera>`.
        0x128 spCubeMapCamera: u32,
        /// `iCurrentCubeMapFace` (Xbox PDB).
        0x12C iCurrentCubeMapFace: u32,
        /// `bDisableLightUpdate` (Xbox PDB).
        0x130 bDisableLightUpdate: u8,
        /// `bWireframe` (Xbox PDB).
        0x131 bWireframe: u8,
        /// `spFog` (Xbox PDB): `NiPointer<BSFogProperty>`.
        0x134 spFog: u32,
        /// `bVisibleUnboundSpace` (Xbox PDB).
        0x1DC bVisibleUnboundSpace: u8,
        /// `pBSPortalGraph` (Xbox PDB).
        0x1E0 pBSPortalGraph: u32,
        /// `bAllowLightRemoveQueues` (Xbox PDB).
        0x1FC bAllowLightRemoveQueues: u8,
    }
}

/// The node's `NiTPointerList` members (Xbox PDB names), three words each, which the
/// constructor clears in the order `m_uiCount`, `m_pkHead`, `m_pkTail`: `lLightList`
/// (+0xb4), `lShadowLightList` (+0xc0), and the queues `lLightQueueAdd` (+0xe4),
/// `lLightQueueRemove` (+0xf0), `lLightQueueUpdate` (+0xfc), `lObjectQueueUpdate` (+0x108),
/// `lObjectQueueUpdateActiveOnly` (+0x114).
const LIGHT_LIST: u32 = 0xb4;
const SHADOW_LIGHT_LIST: u32 = 0xc0;
/// The `ShadowSceneNode` vtable (`010adcf8`).
const SHADOW_SCENE_NODE_VTABLE: u32 = 0x010a_dcf8;
/// `NiNode`'s constructor (`00a5ecb0`, `thiscall`, one word: the child array's size).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `BSCompoundFrustum::BSCompoundFrustum` (engine map, `00c47660`), on `+0x138`.
const COMPOUND_FRUSTUM_CONSTRUCT: u32 = 0x00c4_7660;
/// The node's name: `00a5b690` makes the `NiFixedString` of the text at `010adce0`, `00a5b950`
/// sets it (`thiscall`, a pointer to the string), and the string's release compares it with
/// the word at `0109b220` before decrementing the count 8 bytes before its text.
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const NAME_TEXT: u32 = 0x010a_dce0;
const FIXED_STRING_NONE: u32 = 0x0109_b220;
/// `NiTPointerListBase<..>::RemoveAll` (engine map, `00e74d40`), on the two light lists.
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// The allocation `00aa13e0` (`cdecl`, size), the memory manager singleton and its allocate.
const NEW_OBJECT: u32 = 0x00aa_13e0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `ShadowSceneLight::ShadowSceneLight` (`00b9fda0`, 0x250 bytes) and `SetLight`
/// (`00b9dda0`, `thiscall`, the light); `NiDirectionalLight::NiDirectionalLight` (`00a75c20`,
/// 0xfc bytes); `BSFogProperty::BSFogProperty` (`00bb8180`, 0x64 bytes).
const SHADOW_SCENE_LIGHT_CONSTRUCT: u32 = 0x00b9_fda0;
const SHADOW_SCENE_LIGHT_SET_LIGHT: u32 = 0x00b9_dda0;
const DIRECTIONAL_LIGHT_CONSTRUCT: u32 = 0x00a7_5c20;
const FOG_PROPERTY_CONSTRUCT: u32 = 0x00bb_8180;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Three words copied into the node's points (`011f426c`) and into the light's two colours
/// (`011f4998`).
const POINT_011F426C: u32 = 0x011f_426c;
const COLOR_011F4998: u32 = 0x011f_4998;
/// The `float`s the fog's distances (`010ad6e0`) and colours (`01016248`) are set to.
const FOG_DISTANCE_010AD6E0: u32 = 0x010a_d6e0;
const FOG_COLOR_01016248: u32 = 0x0101_6248;

/// Copies the three words at `from` to `to`.
fn copy_three_words(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The inlined `NiPointer` release of the pointer `object` (not null): the reference count
/// at `+4` is decremented and the object deleted through its vtable `+4` when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The inlined `NiPointer` assignment of `object` to the word at `slot`.
fn assign(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old != object {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
}

/// A new `ShadowSceneLight` (0x250 bytes), or 0 when the allocation fails.
fn new_shadow_scene_light(e: &mut Engine) -> u32 {
    let memory = e.call(NEW_OBJECT, &args![0x250u32]).u32();
    if memory != 0 {
        e.call(SHADOW_SCENE_LIGHT_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    }
}

// Translated from 00b5e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::ShadowSceneNode` (Xbox PDB), `thiscall`, no stack words; answers
/// `this`. Constructs the `NiNode` (child array size 0), sets the vtable, clears the light
/// lists, fences, queues and pointers, constructs the compound frustum (`+0x138`), names the
/// node (the string at `010adce0`), empties the two light lists (`00e74d40`), releases the
/// window reflection map, sets `bVisibleUnboundSpace`, copies the point at `011f426c` to
/// `+0x1f0`; makes the two fence-object lights and the sun light (a `ShadowSceneLight` set to
/// a new `NiDirectionalLight` whose two colours at `+0xc8` and `+0xd4` are the words at
/// `011f4998`, its word at `+0xa8` incremented twice; the light is written even when its
/// allocation failed); sets flag 0x800, gives the node a world bound when it has none
/// (`SetBound`, vtable `+0x78`, with 0x10 bytes from the memory manager: the point at
/// `011f426c` and radius 0) and sets its radius to 1.0; sets flag 0x2000, copies the point
/// to `+0x1e4` and sets `bAllowLightRemoveQueues`; then makes the fog property (`spFog`)
/// with its four distances at the `float` at `010ad6e0` and its two colours (`+0x20`,
/// `+0x54`) at the `float` at `01016248`.
// Translation incomplete: 00b5e0f0 - the C++ exception frame (FS:[0] chain, unwind states 12 to 19) is not translated.
pub fn shadow_scene_node_shadow_scene_node(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
) -> Ptr<ShadowSceneNode> {
    let base = this.addr();
    e.call(NI_NODE_CONSTRUCT, &args![base, 0u32]);
    e.mem.set_u32(base, SHADOW_SCENE_NODE_VTABLE);
    // `lLightList` and `lShadowLightList`: count, head, tail.
    for list in [LIGHT_LIST, SHADOW_LIGHT_LIST] {
        e.mem.set_u32(base + list + 8, 0);
        e.mem.set_u32(base + list, 0);
        e.mem.set_u32(base + list + 4, 0);
    }
    e.set(this, ShadowSceneNode::kShadowLightActiveFenceObject, 0);
    e.set(this, ShadowSceneNode::kShadowLightFadingInFenceObject, 0);
    // The five queues (`+0xe4` to `+0x11c`): count, head, tail.
    for list in [0xe4u32, 0xf0, 0xfc, 0x108, 0x114] {
        e.mem.set_u32(base + list + 8, 0);
        e.mem.set_u32(base + list, 0);
        e.mem.set_u32(base + list + 4, 0);
    }
    e.set(this, ShadowSceneNode::pWindowReflectionEnvMap, 0);
    e.set(this, ShadowSceneNode::spCubeMapCamera, 0);
    e.set(this, ShadowSceneNode::spFog, 0);
    e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![base + 0x138]);

    let name = e.call(FIXED_STRING_MAKE, &args![NAME_TEXT]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), name);
        e.call(SET_NAME, &args![base, slot]);
        let name = e.mem.u32(slot.addr());
        if name != e.global::<u32>(FIXED_STRING_NONE) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
    });
    e.call(LIST_REMOVE_ALL, &args![base + LIGHT_LIST]);
    e.call(LIST_REMOVE_ALL, &args![base + SHADOW_LIGHT_LIST]);

    let map = e.get(this, ShadowSceneNode::pWindowReflectionEnvMap);
    if map != 0 {
        release(e, map);
        e.set(this, ShadowSceneNode::pWindowReflectionEnvMap, 0);
    }
    e.set(this, ShadowSceneNode::iCurrentCubeMapFace, 0);
    e.set(this, ShadowSceneNode::bDisableLightUpdate, 0);
    e.set(this, ShadowSceneNode::bVisibleUnboundSpace, 1);
    copy_three_words(e, base + 0x1f0, POINT_011F426C);
    e.set(this, ShadowSceneNode::pBSPortalGraph, 0);

    let light = new_shadow_scene_light(e);
    assign(e, base + 0xd8, light);
    let light = new_shadow_scene_light(e);
    assign(e, base + 0xdc, light);
    e.set(this, ShadowSceneNode::kShadowLightActiveFence, 0);
    e.set(this, ShadowSceneNode::kShadowLightFadingInFence, 0);

    let memory = e.call(NEW_OBJECT, &args![0xfcu32]).u32();
    let directional = if memory != 0 {
        e.call(DIRECTIONAL_LIGHT_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    copy_three_words(e, directional.wrapping_add(0xc8), COLOR_011F4998);
    let count = e.mem.u32(directional.wrapping_add(0xa8)).wrapping_add(1);
    e.mem.set_u32(directional.wrapping_add(0xa8), count);
    copy_three_words(e, directional.wrapping_add(0xd4), COLOR_011F4998);
    let count = e.mem.u32(directional.wrapping_add(0xa8)).wrapping_add(1);
    e.mem.set_u32(directional.wrapping_add(0xa8), count);
    let sun = new_shadow_scene_light(e);
    e.set(this, ShadowSceneNode::pSunLight, sun);
    e.call(SHADOW_SCENE_LIGHT_SET_LIGHT, &args![sun, directional]);

    let flags = e.get(this, ShadowSceneNode::m_uFlags);
    e.set(this, ShadowSceneNode::m_uFlags, flags | 0x800);
    if e.get(this, ShadowSceneNode::m_kWorldBound) == 0 {
        let bound = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        e.vcall(base, 0x78, &args![bound]);
        let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
        let x = e.global::<u32>(POINT_011F426C);
        e.mem.set_u32(bound, x);
        let y = e.global::<u32>(POINT_011F426C + 4);
        e.mem.set_u32(bound + 4, y);
        let z = e.global::<u32>(POINT_011F426C + 8);
        e.mem.set_u32(bound + 0xc, 0);
        e.mem.set_u32(bound + 8, z);
    }
    let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
    e.mem.set_u32(bound + 0xc, 1.0f32.to_bits());

    let flags = e.get(this, ShadowSceneNode::m_uFlags);
    e.set(this, ShadowSceneNode::m_uFlags, flags | 0x2000);
    e.set(this, ShadowSceneNode::bWireframe, 0);
    copy_three_words(e, base + 0x1e4, POINT_011F426C);
    e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 1);

    let memory = e.call(NEW_OBJECT, &args![0x64u32]).u32();
    let fog = if memory != 0 {
        e.call(FOG_PROPERTY_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    assign(e, base + 0x134, fog);

    // `BSFogProperty` (Xbox PDB): `fStartDistance` to `fEndWaterDistance` (+0x2c to +0x38),
    // then `m_kColor` (+0x20) and `kWaterColor` (+0x54); `FLD`/`FST` copies.
    let distance = x87_quiet_bits(e.global::<u32>(FOG_DISTANCE_010AD6E0));
    let fog = e.get(this, ShadowSceneNode::spFog);
    e.mem.set_u32(fog + 0x2c, distance);
    e.mem.set_u32(fog + 0x30, distance);
    let fog = e.get(this, ShadowSceneNode::spFog);
    e.mem.set_u32(fog + 0x34, distance);
    e.mem.set_u32(fog + 0x38, distance);
    let color = x87_quiet_bits(e.global::<u32>(FOG_COLOR_01016248));
    let fog = e.get(this, ShadowSceneNode::spFog);
    for offset in [0x20u32, 0x24, 0x28] {
        e.mem.set_u32(fog + offset, color);
    }
    let fog = e.get(this, ShadowSceneNode::spFog);
    for offset in [0x54u32, 0x58, 0x5c] {
        e.mem.set_u32(fog + offset, color);
    }
    this
}

// ---- b064: the node's lists, lights and object lighting ----

use crate::types::{NiTListItem, NiTPointerList};

/// The node's other `NiTPointerList`s and members by PC offset (Xbox PDB names, 0x24 lower
/// than the Xbox PDB): `kShadowLightListIterator` (+0xcc), `lLightQueueAdd` (+0xe4),
/// `lLightQueueRemove` (+0xf0), `lLightQueueUpdate` (+0xfc), `lObjectQueueUpdate` (+0x108),
/// `lObjectQueueUpdateActiveOnly` (+0x114).
const SHADOW_LIGHT_LIST_ITERATOR: u32 = 0xcc;
const LIGHT_QUEUE_ADD: u32 = 0xe4;
const LIGHT_QUEUE_REMOVE: u32 = 0xf0;
const LIGHT_QUEUE_UPDATE: u32 = 0xfc;
const OBJECT_QUEUE_UPDATE: u32 = 0x108;
const OBJECT_QUEUE_UPDATE_ACTIVE_ONLY: u32 = 0x114;
/// `ShadowSceneLight` members by PC offset (Xbox PDB names; the light's own layout is in
/// `shadowscenelight.rs`): `spLight` (+0xf8), `bPointPosition` (+0x100), `spDynamicShadowMap`
/// (+0x10c), `spShadowScene` (+0x128).
const LIGHT_SP_LIGHT: u32 = 0xf8;
const LIGHT_POINT_POSITION: u32 = 0x100;
const LIGHT_DYNAMIC_SHADOW_MAP: u32 = 0x10c;
const LIGHT_SHADOW_SCENE: u32 = 0x128;
/// `NiAVObject` members (Xbox PDB names, PC offsets): `m_kWorldBound` (+0x20, a `BSNiBound*`
/// whose radius is at +0xc), `m_uFlags` (+0x30), the world translation (+0x8c, three
/// `float`s); `NiNode::m_kChildren` (+0x9c: buffer at +0xa0, `m_usSize` at +0xa6).
const AV_WORLD_BOUND: u32 = 0x20;
const AV_FLAGS: u32 = 0x30;
const AV_WORLD_TRANSLATE: u32 = 0x8c;
const NODE_CHILDREN_BASE: u32 = 0xa0;
const NODE_CHILDREN_SIZE: u32 = 0xa6;
/// A geometry's property word read through `+0x9c` (`+0x9c + 0xc` = `+0xa8`): a
/// `BSShaderProperty`-shaped object whose type word is at +0x1c.
const GEOMETRY_PROPERTY_BASE: u32 = 0x9c;
const GEOMETRY_PROPERTY: u32 = 0xa8;
const PROPERTY_TYPE: u32 = 0x1c;
/// An object's virtual slots: `IsNode` (+0xc) and `IsTriBasedGeom` (+0x1c) (Xbox PDB slot
/// names of `NiObject`), and the deleting destructor (+0, one word: the flags).
const SLOT_IS_NODE: u32 = 0xc;
const SLOT_IS_TRI_BASED_GEOM: u32 = 0x1c;
const SLOT_DELETING_DESTRUCTOR: u32 = 0;
/// `BSShaderLightingProperty::GetFirstLight` (`00b70490`, `RET 4`: an iterator's address),
/// `FenceAllLights` (`00b718b0`) and `RemoveFencedLights` (`00b71600`, `RET 4`).
const PROPERTY_NEXT_LIGHT: u32 = 0x00b7_0490;
const PROPERTY_FENCE_ALL_LIGHTS: u32 = 0x00b7_18b0;
const PROPERTY_REMOVE_FENCED_LIGHTS: u32 = 0x00b7_1600;
/// `ShadowSceneLight` functions of `shadowscenelight.cpp`, called by address:
/// `BoundWithinLightRadius` (`00b9dae0`, `RET 0xc`), `AddTriGeom` (`00b9f480`, `RET 4`),
/// `RemoveTriGeom` (`00ba01a0`, `RET 8`), `00b9e770` (no stack words) and `00b9bc50`
/// (`RET 0x10`: a point and a word; `float` in `ST0`).
const LIGHT_BOUND_WITHIN_RADIUS: u32 = 0x00b9_dae0;
const LIGHT_ADD_TRI_GEOM: u32 = 0x00b9_f480;
const LIGHT_REMOVE_TRI_GEOM: u32 = 0x00ba_01a0;
const LIGHT_00B9E770: u32 = 0x00b9_e770;
const LIGHT_00B9BC50: u32 = 0x00b9_bc50;
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, no arguments), the accumulator's scene node
/// word at +0x194, and the fallback node pointer global `011f91c8`.
const SHADER_MANAGER_GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const ACCUMULATOR_SCENE_NODE: u32 = 0x194;
const FALLBACK_SCENE_NODE_011F91C8: u32 = 0x011f_91c8;
/// The `NiBound` used when an object has none (`011f4288`, four words).
const NULL_BOUND_011F4288: u32 = 0x011f_4288;
/// The renderer object global `011f91a8` and its `00b6d4c0` (`RET 4`, a shadow map).
const RENDERER_011F91A8: u32 = 0x011f_91a8;
const RENDERER_RELEASE_MAP_00B6D4C0: u32 = 0x00b6_d4c0;
/// The kind-of cast `00653290` (`thiscall` on the object, `RET 4`: the `NiRTTI` record) to the
/// record at `011f5eb4`, and `00b5aa40` (the node's child at the index word +0xb0, or 0).
const CAST_00653290: u32 = 0x0065_3290;
const MULTIBOUND_RTTI_011F5EB4: u32 = 0x011f_5eb4;
const NODE_INDEXED_CHILD_00B5AA40: u32 = 0x00b5_aa40;
/// The light-queue spin lock (`011f9ea0`: owner, `011f9ea4`: count), its lock `0040fbf0`
/// (`thiscall`, `RET 4`) and the list append `00a5a310` (`RET 4`: an `NiPointer`'s address).
const QUEUE_LOCK: u32 = 0x011f_9ea0;
const QUEUE_LOCK_COUNT: u32 = 0x011f_9ea4;
const SPIN_LOCK_LOCK: u32 = 0x0040_fbf0;
const LIST_ADD_TAIL_00A5A310: u32 = 0x00a5_a310;
/// The sized free `00aa1460` (`cdecl`: the block and its size).
const SIZED_FREE: u32 = 0x00aa_1460;
/// The culling process local of `UpdateOcclusionPlaneVisibility` (0xf4 bytes): constructed by
/// `00a69400` (`RET 4`, 0), given the camera's frustum (`00a694a0`, `RET 4`: camera + 0xdc),
/// destroyed by `00a693e0`; its camera at +0xc, its planes (`NiPlane::NiPlane`, `00a69940`)
/// at +0x90 and their active mask at +0xf0; +0x2c is passed to the plane test.
const CULLING_PROCESS_CONSTRUCT: u32 = 0x00a6_9400;
const CULLING_PROCESS_SET_FRUSTUM: u32 = 0x00a6_94a0;
const CULLING_PROCESS_DESTRUCT: u32 = 0x00a6_93e0;
const NI_PLANE_CONSTRUCT_00A69940: u32 = 0x00a6_9940;
/// `BSOcclusionPlane::WithinFrustum` (`00c33cf0`, `RET 0x14`) and `ViewerOnCorrectSide`
/// (`00c358a0`, `RET 0xc`), both `u8` results; the plane's state word at +0x40.
const OCCLUSION_PLANE_WITHIN_FRUSTUM: u32 = 0x00c3_3cf0;
const OCCLUSION_PLANE_VIEWER_ON_CORRECT_SIDE: u32 = 0x00c3_58a0;

/// The list's head item.
fn list_head(e: &Engine, list: u32) -> u32 {
    e.get(Ptr::<NiTPointerList>::new(list), NiTPointerList::m_pkHead)
}

/// An item's next link.
fn item_next(e: &Engine, item: u32) -> u32 {
    e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_pkNext)
}

/// An item's element.
fn item_element(e: &Engine, item: u32) -> u32 {
    e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_element)
}

/// The inlined property lookup of the object-lighting functions: the property word (+0xa8)
/// unless its type word is -1, then that property if its type is 1 to 12; else 0.
fn lighting_property(e: &Engine, property: u32) -> u32 {
    if e.mem
        .u32(property.wrapping_add(PROPERTY_TYPE))
        .wrapping_add(1)
        == 0
    {
        return 0;
    }
    shader_property(e, property)
}

/// The property itself when its type word minus one is at most 0xb (types 1 to 12), else 0.
fn shader_property(e: &Engine, property: u32) -> u32 {
    let kind = e
        .mem
        .u32(property.wrapping_add(PROPERTY_TYPE))
        .wrapping_sub(1);
    if kind <= 0xb {
        property
    } else {
        0
    }
}

// Translated from 00b5a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: moves `item` of an `NiTPointerList` to just after `after`.
/// `thiscall` on the list, `RET 8`. Nothing when they are the same item. The head and tail
/// pass on from `item` when they point at it, the tail becomes `item` when it was `after`;
/// `item` is unlinked from its neighbours and linked in after `after`.
pub fn fn_00b5a8d0(e: &mut Engine, this: Ptr<NiTPointerList>, item: u32, after: u32) {
    if item == after {
        return;
    }
    let list = this;
    let it = Ptr::<NiTListItem>::new(item);
    let at = Ptr::<NiTListItem>::new(after);
    if e.get(list, NiTPointerList::m_pkHead) == item {
        let next = e.get(it, NiTListItem::m_pkNext);
        e.set(list, NiTPointerList::m_pkHead, next);
    }
    if e.get(list, NiTPointerList::m_pkTail) == item {
        let previous = e.get(it, NiTListItem::m_pkPrev);
        e.set(list, NiTPointerList::m_pkTail, previous);
    }
    if e.get(list, NiTPointerList::m_pkTail) == after {
        e.set(list, NiTPointerList::m_pkTail, item);
    }
    let next = e.get(it, NiTListItem::m_pkNext);
    if next != 0 {
        let previous = e.get(it, NiTListItem::m_pkPrev);
        e.set(
            Ptr::<NiTListItem>::new(next),
            NiTListItem::m_pkPrev,
            previous,
        );
    }
    let previous = e.get(it, NiTListItem::m_pkPrev);
    if previous != 0 {
        let next = e.get(it, NiTListItem::m_pkNext);
        e.set(
            Ptr::<NiTListItem>::new(previous),
            NiTListItem::m_pkNext,
            next,
        );
    }
    let following = e.get(at, NiTListItem::m_pkNext);
    e.set(it, NiTListItem::m_pkNext, following);
    e.set(it, NiTListItem::m_pkPrev, after);
    if following != 0 {
        e.set(
            Ptr::<NiTListItem>::new(following),
            NiTListItem::m_pkPrev,
            item,
        );
    }
    e.set(at, NiTListItem::m_pkNext, item);
}

// Translated from 00b5a9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores a point (three words, by value) at +0x100 (a
/// `ShadowSceneLight`'s `bPointPosition`, Xbox PDB). `thiscall`, `RET 0xc`.
pub fn fn_00b5a9d0(e: &mut Engine, this: Ptr, x: u32, y: u32, z: u32) {
    let base = this.addr().wrapping_add(LIGHT_POINT_POSITION);
    e.mem.set_u32(base, x);
    e.mem.set_u32(base.wrapping_add(4), y);
    e.mem.set_u32(base.wrapping_add(8), z);
}

// Translated from 00b5aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no Ghidra function; read from its bytes): a sort comparison
/// (`cdecl`, two pointers to object pointers) of the objects' `float`s at +0xc, each copied
/// through a `float` slot (`FLD`/`FSTP`). Answers -1 when the second is smaller than the first,
/// 1 when it is greater, and 0 when they are equal or unordered.
pub fn fn_00b5aa70(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a_object = e.mem.u32(first.addr());
    let b_object = e.mem.u32(second.addr());
    let a = x87_load32(e.mem.u32(a_object.wrapping_add(0xc)));
    let b = x87_load32(e.mem.u32(b_object.wrapping_add(0xc)));
    // `FLD a; FLD b; FCOM ST1; TEST AH,5; JP`, then `FCOMPP; TEST AH,0x41; JNZ`.
    if b < a {
        -1
    } else if b > a {
        1
    } else {
        0
    }
}

// Translated from 00b5aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::SetSunLight` (Xbox PDB). `thiscall`, `RET 4` (the light). Deletes the
/// current sun light (virtual slot 0 with 1) and clears `pSunLight`. With a light, a new
/// `ShadowSceneLight` (0x250 bytes) is set to it; without one, a new `NiDirectionalLight` (0xfc
/// bytes; null when the allocation fails) gets the two colours at `011f4998` (+0xc8, +0xd4),
/// its word at +0xa8 incremented twice, and a new `ShadowSceneLight` is set to that light. The
/// new `ShadowSceneLight` (null when its allocation fails) becomes `pSunLight` and
/// `ShadowSceneLight::SetLight` (`00b9dda0`) runs on it.
// Translation incomplete: 00b5aac0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the new objects) is not translated.
pub fn shadow_scene_node_set_sun_light(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32) {
    let sun = e.get(this, ShadowSceneNode::pSunLight);
    if sun != 0 {
        e.vcall(sun, SLOT_DELETING_DESTRUCTOR, &args![1u32]);
        e.set(this, ShadowSceneNode::pSunLight, 0);
    }
    let (shadow_light, light) = if light != 0 {
        (new_shadow_scene_light(e), light)
    } else {
        let memory = e.call(NEW_OBJECT, &args![0xfcu32]).u32();
        let directional = if memory != 0 {
            e.call(DIRECTIONAL_LIGHT_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        copy_three_words(e, directional.wrapping_add(0xc8), COLOR_011F4998);
        let count = e.mem.u32(directional.wrapping_add(0xa8)).wrapping_add(1);
        e.mem.set_u32(directional.wrapping_add(0xa8), count);
        copy_three_words(e, directional.wrapping_add(0xd4), COLOR_011F4998);
        e.mem
            .set_u32(directional.wrapping_add(0xa8), count.wrapping_add(1));
        (new_shadow_scene_light(e), directional)
    };
    e.set(this, ShadowSceneNode::pSunLight, shadow_light);
    e.call(SHADOW_SCENE_LIGHT_SET_LIGHT, &args![shadow_light, light]);
}

// Translated from 00b5ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the light at `index` in `lLightList`, or 0 when `index` is not
/// below the list's count. `thiscall`, `RET 4`.
pub fn fn_00b5ac20(e: &mut Engine, this: Ptr<ShadowSceneNode>, index: u32) -> u32 {
    let list = this.addr().wrapping_add(LIGHT_LIST);
    let count = e.get(Ptr::<NiTPointerList>::new(list), NiTPointerList::m_uiCount);
    if index >= count {
        return 0;
    }
    let head = list_head(e, list);
    let mut next = item_next(e, head);
    let mut element = item_element(e, head);
    for _ in 0..index {
        element = item_element(e, next);
        next = item_next(e, next);
    }
    element
}

// Translated from 00b5ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateOcclusionPlaneVisibility` (Xbox PDB). `thiscall`, `RET 4` (the
/// camera). Only with a portal graph (`pBSPortalGraph`): a culling process local is set up
/// for the camera (`00a69400`, `00a694a0` with camera + 0xdc, the camera at +0xc, six planes
/// and mask 0x3f), then each occlusion plane of the graph's list (head at graph + 8) gets the
/// state word 1 when it is within that frustum (`WithinFrustum`: the local's +0x2c, 0, the
/// point at `011f426c`) and the camera's world position (+0x8c) is on its correct side
/// (`ViewerOnCorrectSide`), else 2. The local is destroyed (`00a693e0`).
// Translation incomplete: 00b5ac90 - the C++ exception frame (FS:[0] chain, unwind state 0 for the culling process local) is not translated.
pub fn shadow_scene_node_update_occlusion_plane_visibility(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    camera: u32,
) {
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    if graph == 0 {
        return;
    }
    e.with_stack(0xf4, |e, local| {
        let local = local.addr();
        e.call(CULLING_PROCESS_CONSTRUCT, &args![local, 0u32]);
        e.mem.set_u32(local + 0xc, camera);
        e.call(
            CULLING_PROCESS_SET_FRUSTUM,
            &args![local, camera.wrapping_add(0xdc)],
        );
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        let mut item = e.mem.u32(graph.wrapping_add(8));
        for plane in 0..6u32 {
            e.call(
                NI_PLANE_CONSTRUCT_00A69940,
                &args![local + 0x90 + 0x10 * plane],
            );
        }
        e.mem.set_u32(local + 0xf0, 0x3f);
        while item != 0 {
            let plane = item_element(e, item);
            item = item_next(e, item);
            if plane == 0 {
                continue;
            }
            let x = e.global::<u32>(POINT_011F426C);
            let y = e.global::<u32>(POINT_011F426C + 4);
            let z = e.global::<u32>(POINT_011F426C + 8);
            let within = e
                .call(
                    OCCLUSION_PLANE_WITHIN_FRUSTUM,
                    &args![plane, local + 0x2c, 0u32, x, y, z],
                )
                .u8();
            let position = camera.wrapping_add(AV_WORLD_TRANSLATE);
            let px = e.mem.u32(position);
            let py = e.mem.u32(position.wrapping_add(4));
            let pz = e.mem.u32(position.wrapping_add(8));
            let side = e
                .call(
                    OCCLUSION_PLANE_VIEWER_ON_CORRECT_SIDE,
                    &args![plane, px, py, pz],
                )
                .u8();
            let state = if within & side == 0 { 2 } else { 1 };
            e.mem.set_u32(plane.wrapping_add(0x40), state);
        }
        e.call(CULLING_PROCESS_DESTRUCT, &args![local]);
    });
}

// Translated from 00b5aee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: assigns an object to the `NiPointer` at +0x10c (a
/// `ShadowSceneLight`'s `spDynamicShadowMap`, Xbox PDB). `thiscall`, `RET 4`.
pub fn fn_00b5aee0(e: &mut Engine, this: Ptr, object: u32) {
    assign(
        e,
        this.addr().wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP),
        object,
    );
}

// Translated from 00b5af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: assigns an object to the `NiPointer` at +0x128 (a
/// `ShadowSceneLight`'s `spShadowScene`, Xbox PDB). `thiscall`, `RET 4`.
pub fn fn_00b5af30(e: &mut Engine, this: Ptr, object: u32) {
    assign(e, this.addr().wrapping_add(LIGHT_SHADOW_SCENE), object);
}

/// The shadow-light iterator's walk of `00b5afc0` and `00b5b010` from the item already
/// taken: answers the first element that has an `NiLight` (`spLight`), or the element of the
/// last item reached.
fn shadow_light_iterate(e: &mut Engine, this: Ptr<ShadowSceneNode>, item: u32) -> u32 {
    let iterator = this.addr().wrapping_add(SHADOW_LIGHT_LIST_ITERATOR);
    let mut next = item_next(e, item);
    e.mem.set_u32(iterator, next);
    let mut element = item_element(e, item);
    while next != 0 {
        if element != 0 && e.mem.u32(element.wrapping_add(LIGHT_SP_LIGHT)) != 0 {
            break;
        }
        let item = e.mem.u32(iterator);
        next = item_next(e, item);
        e.mem.set_u32(iterator, next);
        element = item_element(e, item);
    }
    element
}

// Translated from 00b5afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: starts the walk of `lShadowLightList`
/// (`kShadowLightListIterator` set to its head) and answers the first light that has an
/// `NiLight`; the last light is answered without that test, and 0 for an empty list.
/// `thiscall`, no stack words.
pub fn fn_00b5afc0(e: &mut Engine, this: Ptr<ShadowSceneNode>) -> u32 {
    let head = list_head(e, this.addr().wrapping_add(SHADOW_LIGHT_LIST));
    let iterator = this.addr().wrapping_add(SHADOW_LIGHT_LIST_ITERATOR);
    e.mem.set_u32(iterator, head);
    if head == 0 {
        return 0;
    }
    shadow_light_iterate(e, this, head)
}

// Translated from 00b5b010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: continues the walk `00b5afc0` started from
/// `kShadowLightListIterator` (0 once it is exhausted). `thiscall`, no stack words.
pub fn fn_00b5b010(e: &mut Engine, this: Ptr<ShadowSceneNode>) -> u32 {
    let iterator = this.addr().wrapping_add(SHADOW_LIGHT_LIST_ITERATOR);
    let item = e.mem.u32(iterator);
    if item == 0 {
        return 0;
    }
    shadow_light_iterate(e, this, item)
}

// Translated from 00b5b060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: for each light of `lShadowLightList` that has an `NiLight`: its
/// dynamic shadow map is handed to the renderer object at `011f91a8` (`00b6d4c0`), released
/// and cleared, flag 1 of the `NiLight`'s `m_uFlags` is set, and the light's `00b9e770` runs.
/// `thiscall`, no stack words.
pub fn fn_00b5b060(e: &mut Engine, this: Ptr<ShadowSceneNode>) {
    let mut item = list_head(e, this.addr().wrapping_add(SHADOW_LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light == 0 || e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT)) == 0 {
            continue;
        }
        let map = e.mem.u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP));
        let renderer = e.global::<u32>(RENDERER_011F91A8);
        e.call(RENDERER_RELEASE_MAP_00B6D4C0, &args![renderer, map]);
        let map = e.mem.u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP));
        if map != 0 {
            release(e, map);
            e.mem
                .set_u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP), 0);
        }
        let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
        let flags = e.mem.u32(ni_light.wrapping_add(AV_FLAGS));
        e.mem.set_u32(ni_light.wrapping_add(AV_FLAGS), flags | 1);
        e.call(LIGHT_00B9E770, &args![light]);
    }
}

// Translated from 00b5b0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::RemoveGeometryFromPropertyLights` (Xbox PDB). `cdecl`, one word (the
/// geometry). With a lighting property (the word at +0xa8, read through +0x9c, of type 1 to
/// 12), the geometry is removed from each of the property's lights
/// (`ShadowSceneLight::RemoveTriGeom` with 0), walking them with
/// `BSShaderLightingProperty::GetFirstLight`, whose iterator is the function's own argument
/// slot.
pub fn shadow_scene_node_remove_geometry_from_property_lights(e: &mut Engine, geometry: u32) {
    if geometry == 0 {
        return;
    }
    let base = geometry.wrapping_add(GEOMETRY_PROPERTY_BASE);
    let mut property = 0;
    if base != 0 {
        property = e.mem.u32(base.wrapping_add(0xc));
    }
    if property == 0 {
        property = e.mem.u32(geometry.wrapping_add(GEOMETRY_PROPERTY));
    }
    let property = if property != 0 {
        shader_property(e, property)
    } else {
        0
    };
    if property == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), geometry);
        loop {
            let light = e.call(PROPERTY_NEXT_LIGHT, &args![property, slot]).u32();
            if light == 0 {
                break;
            }
            e.call(LIGHT_REMOVE_TRI_GEOM, &args![light, geometry, 0u32]);
        }
    });
}

// Translated from 00b5b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::RemoveObjectRecurse` (Xbox PDB). `thiscall`, `RET 4` (a node). Each child
/// of the node that is a tri-based geometry is removed from its property lights
/// (`RemoveGeometryFromPropertyLights`); each child that is a node is walked the same way.
pub fn shadow_scene_node_remove_object_recurse(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    node: u32,
) {
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(NODE_CHILDREN_SIZE))) {
        let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN_BASE));
        let child = e.mem.u32(children.wrapping_add(4 * index));
        if child != 0 {
            if e.vcall(child, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
                shadow_scene_node_remove_geometry_from_property_lights(e, child);
            } else {
                let child_node = e.vcall(child, SLOT_IS_NODE, &args![]).u32();
                if child_node != 0 {
                    shadow_scene_node_remove_object_recurse(e, this, child_node);
                }
            }
        }
        index += 1;
    }
}

// Translated from 00b5b1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::RemoveObject` (Xbox PDB). `thiscall`, `RET 4` (an object): an object that
/// is a node is walked by `RemoveObjectRecurse`.
pub fn shadow_scene_node_remove_object(e: &mut Engine, this: Ptr<ShadowSceneNode>, object: u32) {
    if object == 0 {
        return;
    }
    let node = e.vcall(object, SLOT_IS_NODE, &args![]).u32();
    if node != 0 {
        shadow_scene_node_remove_object_recurse(e, this, node);
    }
}

/// `UpdateQueuedObjectLighting`'s test of one light against the bound and its add.
fn add_if_within_radius(e: &mut Engine, light: u32, bound: Ptr, geometry: u32) {
    let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
    let within = e
        .call(
            LIGHT_BOUND_WITHIN_RADIUS,
            &args![light, bound, ni_light, 1.0f32],
        )
        .u8();
    if within != 0 {
        e.call(LIGHT_ADD_TRI_GEOM, &args![light, geometry]);
    }
}

// Translated from 00b5b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateQueuedObjectLighting` (Xbox PDB). `cdecl`, five words (the
/// geometry, a scene node, an unused word, and two light collections). Answers 1 when the
/// geometry has no property word (+0xa8) and 0 when that is not a lighting property (type -1,
/// or not 1 to 12). Otherwise the property's lights are fenced (`FenceAllLights`) and the
/// geometry's bound (the four words of `m_kWorldBound`, or of the null bound at `011f4288`) is
/// tested (`BoundWithinLightRadius` with the light's `NiLight` and 1.0) against each light of
/// the first collection that is given (count at +0xe4, array at +0xe0), else of the second
/// (count at +0xb8, array at +0xb4), else of the scene node's `lLightList` (the node being the
/// shader manager's accumulator's +0x194, or the node at `011f91c8`, when none is given); the
/// lights it is within get the geometry (`AddTriGeom`). Then the fenced lights are removed
/// (`RemoveFencedLights`), flag 0x400000 of the geometry is set, and 1 is answered.
pub fn shadow_scene_node_update_queued_object_lighting(
    e: &mut Engine,
    geometry: u32,
    scene_node: u32,
    _unused_2: u32,
    first: u32,
    second: u32,
) -> u8 {
    let word = e.mem.u32(geometry.wrapping_add(GEOMETRY_PROPERTY));
    if word == 0 {
        return 1;
    }
    let property = lighting_property(e, word);
    if property == 0 {
        return 0;
    }
    let mut node = scene_node;
    if node == 0 {
        let accumulator = e.call(SHADER_MANAGER_GET_ACCUMULATOR, &args![]).u32();
        node = e.mem.u32(accumulator.wrapping_add(ACCUMULATOR_SCENE_NODE));
        if node == 0 {
            node = e.global::<u32>(FALLBACK_SCENE_NODE_011F91C8);
        }
    }
    e.call(PROPERTY_FENCE_ALL_LIGHTS, &args![property]);
    let mut source = e.mem.u32(geometry.wrapping_add(AV_WORLD_BOUND));
    if source == 0 {
        source = NULL_BOUND_011F4288;
    }
    e.with_stack(0x10, |e, bound| {
        for word in 0..4u32 {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(bound.addr() + 4 * word, value);
        }
        if first != 0 {
            let mut index = 0u32;
            while index < e.mem.u32(first.wrapping_add(0xe4)) {
                let array = e.mem.u32(first.wrapping_add(0xe0));
                let light = e.mem.u32(array.wrapping_add(4 * index));
                if light != 0 {
                    add_if_within_radius(e, light, bound, geometry);
                }
                index += 1;
            }
        } else if second != 0 {
            let mut index = 0u32;
            while index < e.mem.u32(second.wrapping_add(0xb8)) {
                let array = e.mem.u32(second.wrapping_add(0xb4));
                let light = e.mem.u32(array.wrapping_add(4 * index));
                if light != 0 {
                    add_if_within_radius(e, light, bound, geometry);
                }
                index += 1;
            }
        } else {
            let mut item = e.mem.u32(node.wrapping_add(LIGHT_LIST));
            while item != 0 {
                let light = item_element(e, item);
                item = item_next(e, item);
                if light != 0 {
                    add_if_within_radius(e, light, bound, geometry);
                }
            }
        }
    });
    e.call(PROPERTY_REMOVE_FENCED_LIGHTS, &args![property, geometry]);
    let flags = e.mem.u32(geometry.wrapping_add(AV_FLAGS));
    e.mem
        .set_u32(geometry.wrapping_add(AV_FLAGS), flags | 0x40_0000);
    1
}

// Translated from 00b5b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateQueuedObjectLightingList` (Xbox PDB). `cdecl`, four words (its
/// callers push four: the geometry, an unused word, a null-terminated array of lights, an
/// unused word). Answers 1 when the geometry's property word (read through +0x9c) is 0 (or
/// the address +0x9c is 0), 0 when it is not a lighting property. Otherwise the property's
/// lights are fenced, each light of the array gets the geometry (`AddTriGeom`), the fenced
/// lights are removed, flag 0x400000 of the geometry is set and 1 is answered.
pub fn shadow_scene_node_update_queued_object_lighting_list(
    e: &mut Engine,
    geometry: u32,
    _unused_1: u32,
    lights: u32,
    _unused_3: u32,
) -> u8 {
    let base = geometry.wrapping_add(GEOMETRY_PROPERTY_BASE);
    if base == 0 {
        return 1;
    }
    let word = e.mem.u32(base.wrapping_add(0xc));
    if word == 0 {
        return 1;
    }
    let property = lighting_property(e, word);
    if property == 0 {
        return 0;
    }
    e.call(PROPERTY_FENCE_ALL_LIGHTS, &args![property]);
    let mut index = 0u32;
    while e.mem.u32(lights.wrapping_add(4 * index)) != 0 {
        let light = e.mem.u32(lights.wrapping_add(4 * index));
        if light != 0 {
            e.call(LIGHT_ADD_TRI_GEOM, &args![light, geometry]);
        }
        index += 1;
    }
    e.call(PROPERTY_REMOVE_FENCED_LIGHTS, &args![property, geometry]);
    let flags = e.mem.u32(geometry.wrapping_add(AV_FLAGS));
    e.mem
        .set_u32(geometry.wrapping_add(AV_FLAGS), flags | 0x40_0000);
    1
}

// Translated from 00b5b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::GetLight` (Xbox PDB). `thiscall`, `RET 4` (an `NiLight`): the light of
/// `lLightList` whose `spLight` is that `NiLight`, or 0.
pub fn shadow_scene_node_get_light(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    ni_light: u32,
) -> u32 {
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light != 0 && e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT)) == ni_light {
            return light;
        }
    }
    0
}

/// `FLD a; FLD b; FCOM; TEST AH,5; JP`, keeping `a` (`FSTP ST0`) when `b < a` and `b`
/// (`FSTP ST1`) otherwise (NaN included).
fn x87_keep_larger(a: f64, b: f64) -> f64 {
    if b < a {
        a
    } else {
        b
    }
}

/// The largest of three `float`s at `colour` as `00b5b4d0` picks it: `max(r, max(g, b))`
/// with each step through a `float` slot.
fn colour_peak(e: &Engine, colour: u32) -> u32 {
    let r = x87_load32(e.mem.u32(colour));
    let g = x87_load32(e.mem.u32(colour.wrapping_add(4)));
    let b = x87_load32(e.mem.u32(colour.wrapping_add(8)));
    let gb = x87_store32(x87_keep_larger(g, b));
    let peak = x87_store32(x87_keep_larger(r, x87_load32(gb)));
    x87_quiet_bits(peak)
}

// Translated from 00b5b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called from `CalculateLightValue`). `thiscall`, `RET 0x1c`: a
/// point (three words), a counter pointer, two `float` pointers and a word. Sums each light's
/// `00b9bc50` (the point and the word) over `lLightList`, counting each light after which the
/// running sum is above 0.0; the sun light's `NiLight` gives the largest component of its two
/// colours (+0xc8 and +0xd4) for the two `float` pointers. Answers the sum (`ST0`).
pub fn fn_00b5b4d0(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    x: u32,
    y: u32,
    z: u32,
    count: Ptr,
    first_peak: Ptr,
    second_peak: Ptr,
    word: u32,
) -> f32 {
    let mut sum = x87_store32(0.0);
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light == 0 {
            continue;
        }
        let value = e.call(LIGHT_00B9BC50, &args![light, x, y, z, word]).f64();
        sum = x87_store32(x87_add(value, x87_m32(sum)));
        if 0.0 < x87_load32(sum) {
            let n = e.mem.u32(count.addr()).wrapping_add(1);
            e.mem.set_u32(count.addr(), n);
        }
    }
    let sun = e.get(this, ShadowSceneNode::pSunLight);
    let ni_light = e.mem.u32(sun.wrapping_add(LIGHT_SP_LIGHT));
    let peak = colour_peak(e, ni_light.wrapping_add(0xc8));
    e.mem.set_u32(first_peak.addr(), peak);
    let peak = colour_peak(e, ni_light.wrapping_add(0xd4));
    e.mem.set_u32(second_peak.addr(), peak);
    x87_quiet(f32::from_bits(sum))
}

/// The tests both object-lighting walks start with: 1 for no object, 0 without a bound, for a
/// zero radius (`FCOMP`; NaN passes), for flag 1 (culled) or for an object that is not a node;
/// otherwise the node.
fn lighting_walk_start(e: &mut Engine, object: u32) -> Result<u32, u8> {
    if object == 0 {
        return Err(1);
    }
    let bound = e.mem.u32(object.wrapping_add(AV_WORLD_BOUND));
    if bound == 0 {
        return Err(0);
    }
    let radius = x87_m32(e.mem.u32(bound.wrapping_add(0xc)));
    if radius == 0.0 {
        return Err(0);
    }
    if e.mem.u32(object.wrapping_add(AV_FLAGS)) & 1 != 0 {
        return Err(0);
    }
    if e.vcall(object, SLOT_IS_NODE, &args![]).u32() == 0 {
        return Err(0);
    }
    Ok(e.vcall(object, SLOT_IS_NODE, &args![]).u32())
}

/// The multibound step of both walks: the node cast to the record at `011f5eb4`; answers the
/// cast and its indexed child (`00b5aa40`), or `None` when the cast fails.
fn multibound_child(e: &mut Engine, node: u32) -> Option<u32> {
    if node == 0 {
        return None;
    }
    let cast = e
        .call(CAST_00653290, &args![node, MULTIBOUND_RTTI_011F5EB4])
        .u32();
    if cast == 0 {
        return None;
    }
    Some(e.call(NODE_INDEXED_CHILD_00B5AA40, &args![cast]).u32())
}

// Translated from 00b5c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateQueuedObjectLighting` (Xbox PDB, overload 2). `cdecl`, five words
/// (an object and the four words passed on). After the start tests (1 for no object; 0
/// without a bound, for a zero radius, a culled object or one that is not a node): a node that
/// casts to the record at `011f5eb4` passes on to its indexed child (`00b5aa40`; 1 when there
/// is none); otherwise each child that is a tri-based geometry goes to
/// `UpdateQueuedObjectLighting` (`00b5b260`) and each other child to this function, and the
/// results are and-ed into the answer (1 for a node without children).
pub fn shadow_scene_node_update_queued_object_lighting_ov2(
    e: &mut Engine,
    object: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u8 {
    let node = match lighting_walk_start(e, object) {
        Ok(node) => node,
        Err(result) => return result,
    };
    let mut result = 1u8;
    if let Some(child) = multibound_child(e, node) {
        if child == 0 {
            return result;
        }
        result = shadow_scene_node_update_queued_object_lighting_ov2(
            e, child, first, second, third, fourth,
        ) & 1;
        return result;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(NODE_CHILDREN_SIZE))) {
        let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN_BASE));
        let child = e.mem.u32(children.wrapping_add(4 * index));
        if child != 0 {
            let r = if e.vcall(child, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
                shadow_scene_node_update_queued_object_lighting(
                    e, child, first, second, third, fourth,
                )
            } else {
                shadow_scene_node_update_queued_object_lighting_ov2(
                    e, child, first, second, third, fourth,
                )
            };
            result &= r;
        }
        index += 1;
    }
    result
}

// Translated from 00b5c5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateQueuedObjectLightingList` (Xbox PDB, overload 2). `cdecl`, four
/// words (an object and the three words passed on). The same walk as overload 2 of
/// `UpdateQueuedObjectLighting`, with `UpdateQueuedObjectLightingList` (`00b5b410`) for the
/// tri-based geometries.
pub fn shadow_scene_node_update_queued_object_lighting_list_ov2(
    e: &mut Engine,
    object: u32,
    first: u32,
    second: u32,
    third: u32,
) -> u8 {
    let node = match lighting_walk_start(e, object) {
        Ok(node) => node,
        Err(result) => return result,
    };
    let mut result = 1u8;
    if let Some(child) = multibound_child(e, node) {
        if child == 0 {
            return result;
        }
        result = shadow_scene_node_update_queued_object_lighting_list_ov2(
            e, child, first, second, third,
        ) & 1;
        return result;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(NODE_CHILDREN_SIZE))) {
        let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN_BASE));
        let child = e.mem.u32(children.wrapping_add(4 * index));
        if child != 0 {
            let r = if e.vcall(child, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
                shadow_scene_node_update_queued_object_lighting_list(e, child, first, second, third)
            } else {
                shadow_scene_node_update_queued_object_lighting_list_ov2(
                    e, child, first, second, third,
                )
            };
            result &= r;
        }
        index += 1;
    }
    result
}

// Translated from 00b5cff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: queues a light for removal. `thiscall`, `RET 4` (an
/// `NiPointer`'s address). When it holds a light and `bAllowLightRemoveQueues` is set, the
/// light-queue spin lock (`011f9ea0`) is taken (`0040fbf0` with 0), the pointer appended to
/// `lLightQueueRemove` (`00a5a310`), and the lock released (its count at `011f9ea4`
/// decremented; the owner word cleared when it reaches 0).
pub fn fn_00b5cff0(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: Ptr) {
    if e.mem.u32(light.addr()) == 0 || e.get(this, ShadowSceneNode::bAllowLightRemoveQueues) == 0 {
        return;
    }
    e.call(SPIN_LOCK_LOCK, &args![QUEUE_LOCK, 0u32]);
    e.call(
        LIST_ADD_TAIL_00A5A310,
        &args![this.addr().wrapping_add(LIGHT_QUEUE_REMOVE), light],
    );
    unlock_queue(e);
}

/// The inlined spin-lock release of the light queues: the count decremented, the owner
/// cleared when it reaches 0.
fn unlock_queue(e: &mut Engine) {
    let count = e.global::<u32>(QUEUE_LOCK_COUNT).wrapping_sub(1);
    e.set_global(QUEUE_LOCK_COUNT, count);
    if count == 0 {
        e.set_global(QUEUE_LOCK, 0u32);
    }
}

// Translated from 00b5ddb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8` (two flags). With the first flag, each light
/// of `lLightList` goes through `00b5d270`; then, unless the second flag is set, `00b5d180`
/// runs. Nothing without the first flag.
pub fn fn_00b5ddb0(e: &mut Engine, this: Ptr<ShadowSceneNode>, each: u8, skip: u8) {
    if each == 0 {
        return;
    }
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light != 0 {
            fn_00b5d270(e, this, light);
        }
    }
    if skip == 0 {
        fn_00b5d180(e, this);
    }
}

// Translated from 00b5eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::AddObject` (Xbox PDB). `thiscall`, `RET 4` (an object): an object without
/// flag 1 goes to `UpdateObjectLighting` (`00b5d9f0`) with 0.
pub fn shadow_scene_node_add_object(e: &mut Engine, this: Ptr<ShadowSceneNode>, object: u32) {
    if object != 0 && e.mem.u32(object.wrapping_add(AV_FLAGS)) & 1 == 0 {
        shadow_scene_node_update_object_lighting(e, this, object, 0);
    }
}

// Translated from 00b5f980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ShadowSceneNode` scalar deleting destructor (unnamed in the engine map). `thiscall`,
/// `RET 4` (the flags): the destructor (`00b5e5a0`), then the 0x200-byte block freed
/// (`00aa1460`) when bit 0 is set; answers `this`.
pub fn fn_00b5f980(e: &mut Engine, this: Ptr<ShadowSceneNode>, flags: u32) -> Ptr<ShadowSceneNode> {
    fn_00b5e5a0(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x200u32]);
    }
    this
}

/// More callees of the light and shadow functions: the TLS block's word at +0x2b4 (saved, set
/// to 0x20, restored), `BSCubeMapCamera::BSCubeMapCamera` (`00bd8290`, `RET 4`, 0x140 bytes),
/// the object update `00a59c60` (`RET 4`: a 9-byte local, a `float` and five bytes),
/// `ShadowSceneLight` code called by address (`00b9ef30`, no stack words;
/// `MTRenderFrustumDepthMap` `00b9f780`, `RET 8`; `00b9bb10`, `RET 8`; `00b9dcb0`, `RET 4`),
/// the rendering-system call `00ba3130` (`thiscall` on `01200088`, `RET 8`) and the global
/// `011f9174` the current shadow light is stored in.
const TLS_WORD_2B4: u32 = 0x2b4;
const CUBE_MAP_CAMERA_CONSTRUCT: u32 = 0x00bd_8290;
const OBJECT_UPDATE_00A59C60: u32 = 0x00a5_9c60;
const LIGHT_00B9EF30: u32 = 0x00b9_ef30;
const LIGHT_RENDER_FRUSTUM_DEPTH_MAP: u32 = 0x00b9_f780;
const LIGHT_00B9BB10: u32 = 0x00b9_bb10;
const LIGHT_00B9DCB0: u32 = 0x00b9_dcb0;
const RENDERING_SYSTEM_01200088: u32 = 0x0120_0088;
const RENDERING_SYSTEM_00BA3130: u32 = 0x00ba_3130;
const CURRENT_SHADOW_LIGHT_011F9174: u32 = 0x011f_9174;
/// `BSMultiBoundNode::VisibleAgainstActiveOccluders` (`00c47150`, `RET 0x14`, `u8` result).
const MULTIBOUND_VISIBLE_AGAINST_OCCLUDERS: u32 = 0x00c4_7150;
/// `NiTPointerList` helpers (`thiscall` on the list): `00c1b4a0` (`RET 4`), `00b702e0`
/// (`RET 8`), `00b9efa0` (engine map: `RemovePos`, `RET 8`) and `00b9fcc0` (engine map:
/// `Remove`, `RET 8`).
const LIST_00C1B4A0: u32 = 0x00c1_b4a0;
const LIST_00B702E0: u32 = 0x00b7_02e0;
const LIST_REMOVE_POS: u32 = 0x00b9_efa0;
const LIST_REMOVE: u32 = 0x00b9_fcc0;
/// The array search `00b5a930` (`RET 0xc`: the value, 0 and the comparison `00b5a980`) and
/// removal `00b5ade0` (`RET 8`: the index and 1), on `BSSimpleArray`-shaped members.
const ARRAY_FIND_00B5A930: u32 = 0x00b5_a930;
const ARRAY_COMPARE_00B5A980: u32 = 0x00b5_a980;
const ARRAY_REMOVE_00B5ADE0: u32 = 0x00b5_ade0;
/// The `NiRTTI` records `AddLight` accepts (`011f4a68`, and `NiPointLight::ms_RTTI`
/// `011f4a98`) and the record `AddAllLightsFromNode_Recurse` looks for (`011f4a28`).
const LIGHT_RTTI_011F4A68: u32 = 0x011f_4a68;
const POINT_LIGHT_RTTI_011F4A98: u32 = 0x011f_4a98;
const LIGHT_RTTI_011F4A28: u32 = 0x011f_4a28;
/// The `NiLight` constructor `00a7d670` (no stack words, 0xfc bytes).
const NI_LIGHT_CONSTRUCT_00A7D670: u32 = 0x00a7_d670;
/// `00439090` (`thiscall` on a point, `RET 4`: another point; `u8` result).
const POINT_COMPARE_00439090: u32 = 0x0043_9090;
/// The object-queue spin lock (`011f9ec0`: owner, `011f9ec4`: count).
const OBJECT_QUEUE_LOCK: u32 = 0x011f_9ec0;
const OBJECT_QUEUE_LOCK_COUNT: u32 = 0x011f_9ec4;
/// The fade constants of `fn_00b5cde0`: `double`s at `0101ffa0` and `0106b9e8`, `float` at
/// `011ad834`.
const FADE_LOW_0101FFA0: u32 = 0x0101_ffa0;
const FADE_HIGH_0106B9E8: u32 = 0x0106_b9e8;
const FADE_TIME_011AD834: u32 = 0x011a_d834;
/// The renderer's shadow-map calls (`thiscall` on the object at `011f91a8`): `00b6e110`
/// (`RET 0x14`), `00b6d120` and `00b6d3f0` (no stack words); `BSRenderedTexture::StopOffscreen`
/// (`00b6b260`); the word at `011f9508` passed to `00b6e110`.
const RENDERER_00B6E110: u32 = 0x00b6_e110;
const RENDERER_00B6D120: u32 = 0x00b6_d120;
const RENDERER_00B6D3F0: u32 = 0x00b6_d3f0;
const RENDERED_TEXTURE_STOP_OFFSCREEN: u32 = 0x00b6_b260;
const RENDERER_ARGUMENT_011F9508: u32 = 0x011f_9508;
/// `ShadowSceneLight` members (Xbox PDB names, PC offsets) the node reads: `bCastShadow`
/// (+0xec), `bCubeShadowMap` (+0xed), `bPointLight` (+0xf4), `bDynamicLight` (+0xfc),
/// `usFrustumCull` (+0x110), the light's arrays at +0x214/+0x218 and +0x224/+0x228 and its
/// portal graph word at +0x240 (PC only).
const LIGHT_CAST_SHADOW: u32 = 0xec;
const LIGHT_CUBE_SHADOW_MAP: u32 = 0xed;
const LIGHT_POINT_LIGHT: u32 = 0xf4;
const LIGHT_DYNAMIC: u32 = 0xfc;
const LIGHT_FRUSTUM_CULL: u32 = 0x110;
const LIGHT_PORTAL_GRAPH: u32 = 0x240;

/// The 9-byte local the object update `00a59c60` gets: a `float` 0.0 and five zero bytes.
fn update_object(e: &mut Engine, object: u32) {
    e.with_stack(0xc, |e, local| {
        e.mem.set_u32(local.addr(), x87_store32(0.0));
        for byte in 4..9u32 {
            e.mem.set_u8(local.addr() + byte, 0);
        }
        e.call(OBJECT_UPDATE_00A59C60, &args![object, local]);
    });
}

/// Appends `light` to the list at `this + list` through an `NiPointer` temporary (held, then
/// released), as the inlined `AddTail(NiPointer(light))` does. Most callers hold and release
/// without testing for null (`checked` false); `00b5cbd0` tests first.
fn add_tail_held(e: &mut Engine, this: Ptr<ShadowSceneNode>, list: u32, light: u32, checked: bool) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), light);
        let held = !checked || light != 0;
        if held {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light.wrapping_add(4)]);
        }
        e.call(
            LIST_ADD_TAIL_00A5A310,
            &args![this.addr().wrapping_add(list), slot],
        );
        if held {
            release(e, light);
        }
    });
}

// Translated from 00b5b880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateDynamicShadows` (Xbox PDB). `thiscall`, `RET 4` (a word passed on).
/// With the TLS word +0x2b4 set to 0x20 for its duration: makes the cube map camera
/// (`spCubeMapCamera`, a new `BSCubeMapCamera`) when there is none, clears its word at +0x114;
/// with shadow lights, each light with an `NiLight`: a culled `NiLight` (flag 1) gets the
/// light's `00b9ef30`; otherwise the `NiLight` is updated (`00a59c60`) and, unless
/// `usFrustumCull` is 0xff, the light becomes the current shadow light (`011f9174`) and renders
/// its depth map (`MTRenderFrustumDepthMap` with the word and the running count). The
/// rendering system's `00ba3130` (1, index) then runs for the indices from count + 0x11 up to
/// 0x14. Finally the camera's `NiPointer` at +0x130 is released and cleared.
// Translation incomplete: 00b5b880 - the C++ exception frame (FS:[0] chain, unwind state 0 for the new camera) is not translated.
pub fn shadow_scene_node_update_dynamic_shadows(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    word: u32,
) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_WORD_2B4);
    e.mem.set_u32(tls + TLS_WORD_2B4, 0x20);
    if e.get(this, ShadowSceneNode::spCubeMapCamera) == 0 {
        let memory = e.call(NEW_OBJECT, &args![0x140u32]).u32();
        let camera = if memory != 0 {
            e.call(CUBE_MAP_CAMERA_CONSTRUCT, &args![memory, 0u32])
                .u32()
        } else {
            0
        };
        assign(e, this.addr() + 0x128, camera);
    }
    let camera = e.get(this, ShadowSceneNode::spCubeMapCamera);
    e.mem.set_u32(camera.wrapping_add(0x114), 0);
    let list = this.addr().wrapping_add(SHADOW_LIGHT_LIST);
    if e.get(Ptr::<NiTPointerList>::new(list), NiTPointerList::m_uiCount) != 0 {
        let mut count = 0u32;
        let mut item = list_head(e, list);
        while item != 0 {
            let light = item_element(e, item);
            item = item_next(e, item);
            if light == 0 {
                continue;
            }
            let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
            if ni_light == 0 {
                continue;
            }
            if e.mem.u32(ni_light.wrapping_add(AV_FLAGS)) & 1 != 0 {
                e.call(LIGHT_00B9EF30, &args![light]);
                continue;
            }
            update_object(e, ni_light);
            if e.mem.u16(light.wrapping_add(LIGHT_FRUSTUM_CULL)) != 0xff {
                e.set_global(CURRENT_SHADOW_LIGHT_011F9174, light);
                e.call(LIGHT_RENDER_FRUSTUM_DEPTH_MAP, &args![light, word, count]);
                count = count.wrapping_add(1);
            }
        }
        let mut index = count.wrapping_add(0x11) as i32;
        while index < 0x15 {
            e.call(
                RENDERING_SYSTEM_00BA3130,
                &args![RENDERING_SYSTEM_01200088, 1u32, index],
            );
            index += 1;
        }
    }
    let camera = e.get(this, ShadowSceneNode::spCubeMapCamera);
    let slot = camera.wrapping_add(0x130);
    let held = e.mem.u32(slot);
    if held != 0 {
        release(e, held);
        e.mem.set_u32(slot, 0);
    }
    e.mem.set_u32(tls + TLS_WORD_2B4, saved);
}

// Translated from 00b5bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the occlusion test of a multibound node. `thiscall`, `RET 0x14`:
/// the node, a point (three words) and a camera-like object (its mode word at +0x90, its +0x2c
/// passed on). Nothing in modes 1 and 3, when the node's bound shape (+0xac, then +0xc) has a
/// state already (+8) or the node has no children (`m_usSize` +0xa8 here). Otherwise the
/// shape's virtual slot +0x9c (the object's +0x2c) decides the state (1 when it answers
/// non-zero, else 2), and when it passed, the node is tested against each occlusion plane of
/// the portal graph in state 1 (`VisibleAgainstActiveOccluders`: the point, the object, the
/// plane) until one test fails.
pub fn fn_00b5bbe0(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    node: u32,
    x: u32,
    y: u32,
    z: u32,
    viewer: u32,
) {
    let shape_holder = e.mem.u32(node.wrapping_add(0xac));
    let shape = e.mem.u32(shape_holder.wrapping_add(0xc));
    let mode = e.mem.u32(viewer.wrapping_add(0x90));
    let state = e.mem.u32(shape.wrapping_add(8));
    if mode == 1 || mode == 3 || state != 0 {
        return;
    }
    if e.mem.u16(node.wrapping_add(0xa8)) == 0 {
        return;
    }
    let passed = e.vcall(shape, 0x9c, &args![viewer.wrapping_add(0x2c)]).u8();
    let shape_holder = e.mem.u32(node.wrapping_add(0xac));
    let shape = e.mem.u32(shape_holder.wrapping_add(0xc));
    e.mem
        .set_u32(shape.wrapping_add(8), if passed == 0 { 2 } else { 1 });
    if passed == 0 {
        return;
    }
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    if graph == 0 {
        return;
    }
    let mut item = e.mem.u32(graph.wrapping_add(8));
    while item != 0 {
        let plane = item_element(e, item);
        item = item_next(e, item);
        if plane != 0 && e.mem.u32(plane.wrapping_add(0x40)) == 1 {
            let visible = e
                .call(
                    MULTIBOUND_VISIBLE_AGAINST_OCCLUDERS,
                    &args![node, x, y, z, viewer, plane],
                )
                .u8();
            if visible == 0 {
                return;
            }
        }
    }
}

// Translated from 00b5c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: hands each shadow light's dynamic shadow map to the renderer
/// object (`00b6d4c0`) and releases and clears it, then empties `lShadowLightList`
/// (`00e74d40`) and clears both fences. `thiscall`, no stack words.
pub fn fn_00b5c3c0(e: &mut Engine, this: Ptr<ShadowSceneNode>) {
    let mut item = list_head(e, this.addr().wrapping_add(SHADOW_LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light == 0 {
            continue;
        }
        let map = e.mem.u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP));
        let renderer = e.global::<u32>(RENDERER_011F91A8);
        e.call(RENDERER_RELEASE_MAP_00B6D4C0, &args![renderer, map]);
        let map = e.mem.u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP));
        if map != 0 {
            release(e, map);
            e.mem
                .set_u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP), 0);
        }
    }
    e.call(
        LIST_REMOVE_ALL,
        &args![this.addr().wrapping_add(SHADOW_LIGHT_LIST)],
    );
    e.set(this, ShadowSceneNode::kShadowLightActiveFence, 0);
    e.set(this, ShadowSceneNode::kShadowLightFadingInFence, 0);
}

// Translated from 00b5c6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called by `00b5f170` on the portal graph): appends an object to
/// the `NiPointer` list at +0x30 unless it is in it already. `thiscall`, `RET 4`. The object
/// is held by a local while the list is searched and released after; a missing object is
/// appended through an `NiPointer` temporary (held, appended with `00a5a310`, released).
// Translation incomplete: 00b5c6e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the temporary) is not translated.
pub fn fn_00b5c6e0(e: &mut Engine, this: Ptr, object: u32) {
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
    let list = this.addr().wrapping_add(0x30);
    let mut item = list_head(e, list);
    let mut found = 0;
    while item != 0 {
        let next = item_next(e, item);
        if object == item_element(e, item) {
            found = item;
            break;
        }
        item = next;
    }
    let missing = found == 0;
    if object != 0 {
        release(e, object);
    }
    if missing {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), object);
            if object != 0 {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
            }
            e.call(LIST_ADD_TAIL_00A5A310, &args![list, slot]);
            if object != 0 {
                release(e, object);
            }
        });
    }
}

/// The `GetRTTI` (virtual slot +8) of `object` compared with `rtti`.
fn rtti_is(e: &mut Engine, object: u32, rtti: u32) -> bool {
    e.vcall(object, 8, &args![]).u32() == rtti
}

// Translated from 00b5c940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::AddLight` (Xbox PDB). `thiscall`, `RET 8` (an `NiLight` and the dynamic
/// flag). Only for a light whose RTTI is exactly the record at `011f4a68` or
/// `NiPointLight::ms_RTTI` (`011f4a98`); else 0. Under the light-queue lock: a new
/// `ShadowSceneLight` (null when the allocation fails) is set to the light
/// (`ShadowSceneLight::SetLight`), gets the flag as `bDynamicLight`, is appended to
/// `lLightQueueAdd` through an `NiPointer` temporary, and takes the node's portal graph (its
/// +0x240). Answers the new light.
// Translation incomplete: 00b5c940 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the new light and the temporary) is not translated.
pub fn shadow_scene_node_add_light(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    light: u32,
    dynamic: u8,
) -> u32 {
    if light == 0 {
        return 0;
    }
    if !rtti_is(e, light, LIGHT_RTTI_011F4A68) && !rtti_is(e, light, POINT_LIGHT_RTTI_011F4A98) {
        return 0;
    }
    e.call(SPIN_LOCK_LOCK, &args![QUEUE_LOCK, 0u32]);
    let shadow_light = new_shadow_scene_light(e);
    e.call(SHADOW_SCENE_LIGHT_SET_LIGHT, &args![shadow_light, light]);
    e.mem
        .set_u8(shadow_light.wrapping_add(LIGHT_DYNAMIC), dynamic);
    add_tail_held(e, this, LIGHT_QUEUE_ADD, shadow_light, false);
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    e.mem
        .set_u32(shadow_light.wrapping_add(LIGHT_PORTAL_GRAPH), graph);
    unlock_queue(e);
    shadow_light
}

/// The kind-of test against `NiRTTI` record `rtti`: the object's `GetRTTI` (slot +8), then its
/// chain of base records (+4), as the exe inlines it.
fn rtti_chain_has(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, 8, &args![]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record.wrapping_add(4));
    }
    false
}

// Translated from 00b5ca70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::AddAllLightsFromNode_Recurse` (Xbox PDB). `thiscall`, `RET 8` (a node and
/// a `float`). Walks the node's children: a child node is walked the same way; a child that
/// is a light (kind of the record at `011f4a28`) is added (`AddLight` with 1) and updated
/// (`00a59c60`), and a point light (cast to `NiPointLight::ms_RTTI` with `00653290`) has its
/// word at +0xa8 incremented and its three `float`s at +0xe0 set to the `float` argument, 0.0
/// and 0.0. Answers the number of lights found.
pub fn shadow_scene_node_add_all_lights_from_node_recurse(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    node: u32,
    value: f32,
) -> u32 {
    let mut found = 0u32;
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(NODE_CHILDREN_SIZE))) {
        let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN_BASE));
        let child = e.mem.u32(children.wrapping_add(4 * index));
        if child != 0 {
            let child_node = e.vcall(child, SLOT_IS_NODE, &args![]).u32();
            if child_node != 0 {
                let n = shadow_scene_node_add_all_lights_from_node_recurse(
                    e,
                    this,
                    child_node,
                    x87_quiet(value),
                );
                found = found.wrapping_add(n);
            } else if rtti_chain_has(e, child, LIGHT_RTTI_011F4A28) {
                shadow_scene_node_add_light(e, this, child, 1);
                update_object(e, child);
                let point = e
                    .call(CAST_00653290, &args![child, POINT_LIGHT_RTTI_011F4A98])
                    .u32();
                if point != 0 {
                    let count = e.mem.u32(point.wrapping_add(0xa8)).wrapping_add(1);
                    e.mem.set_u32(point.wrapping_add(0xa8), count);
                    e.mem
                        .set_u32(point.wrapping_add(0xe0), x87_quiet_bits(value.to_bits()));
                    let zero = x87_store32(0.0);
                    e.mem.set_u32(point.wrapping_add(0xe4), zero);
                    e.mem.set_u32(point.wrapping_add(0xe8), zero);
                }
                found = found.wrapping_add(1);
            }
        }
        index += 1;
    }
    found
}

// Translated from 00b5cbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the shadow light of an object. `thiscall`, `RET 4` (the object).
/// A shadow light whose `spShadowScene` is the object has flag 1 of its `NiLight` cleared and
/// gets `00b9bb10` (1.0, 0), and is answered. Otherwise, under the light-queue lock, a new
/// `ShadowSceneLight` (null when the allocation fails) gets the object as its shadow scene
/// (`00b5af30`) and `00b9dcb0` (1), and is appended to `lLightQueueAdd` through an `NiPointer`
/// temporary; it is answered.
// Translation incomplete: 00b5cbd0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the new light and the temporary) is not translated.
pub fn fn_00b5cbd0(e: &mut Engine, this: Ptr<ShadowSceneNode>, object: u32) -> u32 {
    let mut item = list_head(e, this.addr().wrapping_add(SHADOW_LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light != 0 && e.mem.u32(light.wrapping_add(LIGHT_SHADOW_SCENE)) == object {
            let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
            let flags = e.mem.u32(ni_light.wrapping_add(AV_FLAGS));
            e.mem.set_u32(ni_light.wrapping_add(AV_FLAGS), flags & !1);
            e.call(LIGHT_00B9BB10, &args![light, 1.0f32, 0u32]);
            return light;
        }
    }
    e.call(SPIN_LOCK_LOCK, &args![QUEUE_LOCK, 0u32]);
    let shadow_light = new_shadow_scene_light(e);
    fn_00b5af30(e, Ptr::new(shadow_light), object);
    e.call(LIGHT_00B9DCB0, &args![shadow_light, 1u32]);
    add_tail_held(e, this, LIGHT_QUEUE_ADD, shadow_light, true);
    unlock_queue(e);
    shadow_light
}

// Translated from 00b5cd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: makes a shadow light dynamic. `thiscall`, `RET 4` (a
/// `ShadowSceneLight`). Sets `bDynamicLight`, gives it a new `NiLight` (`00a7d670`, 0xfc bytes;
/// null when the allocation fails) with flag 1 cleared (`ShadowSceneLight::SetLight`), runs
/// `00b9dcb0` (1), clears `bCubeShadowMap` and appends it to `lShadowLightList` through an
/// `NiPointer` temporary. Answers the light.
// Translation incomplete: 00b5cd00 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the new NiLight and the temporary) is not translated.
pub fn fn_00b5cd00(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32) -> u32 {
    e.mem.set_u8(light.wrapping_add(LIGHT_DYNAMIC), 1);
    let memory = e.call(NEW_OBJECT, &args![0xfcu32]).u32();
    let ni_light = if memory != 0 {
        e.call(NI_LIGHT_CONSTRUCT_00A7D670, &args![memory]).u32()
    } else {
        0
    };
    let flags = e.mem.u32(ni_light.wrapping_add(AV_FLAGS));
    e.mem.set_u32(ni_light.wrapping_add(AV_FLAGS), flags & !1);
    e.call(SHADOW_SCENE_LIGHT_SET_LIGHT, &args![light, ni_light]);
    e.call(LIGHT_00B9DCB0, &args![light, 1u32]);
    e.mem.set_u8(light.wrapping_add(LIGHT_CUBE_SHADOW_MAP), 0);
    add_tail_held(e, this, SHADOW_LIGHT_LIST, light, false);
    light
}

/// `fn_00b5cde0`'s inlined move of `item` to just before `before` in the list at `list`.
fn move_before(e: &mut Engine, list: u32, item: u32, before: u32) {
    let list = Ptr::<NiTPointerList>::new(list);
    let it = Ptr::<NiTListItem>::new(item);
    let at = Ptr::<NiTListItem>::new(before);
    if e.get(list, NiTPointerList::m_pkHead) == item {
        let next = e.get(it, NiTListItem::m_pkNext);
        e.set(list, NiTPointerList::m_pkHead, next);
    }
    if e.get(list, NiTPointerList::m_pkHead) == before {
        e.set(list, NiTPointerList::m_pkHead, item);
    }
    if e.get(list, NiTPointerList::m_pkTail) == item {
        let previous = e.get(it, NiTListItem::m_pkPrev);
        e.set(list, NiTPointerList::m_pkTail, previous);
    }
    let next = e.get(it, NiTListItem::m_pkNext);
    if next != 0 {
        let previous = e.get(it, NiTListItem::m_pkPrev);
        e.set(
            Ptr::<NiTListItem>::new(next),
            NiTListItem::m_pkPrev,
            previous,
        );
    }
    let previous = e.get(it, NiTListItem::m_pkPrev);
    if previous != 0 {
        let next = e.get(it, NiTListItem::m_pkNext);
        e.set(
            Ptr::<NiTListItem>::new(previous),
            NiTListItem::m_pkNext,
            next,
        );
    }
    let preceding = e.get(at, NiTListItem::m_pkPrev);
    e.set(it, NiTListItem::m_pkPrev, preceding);
    e.set(it, NiTListItem::m_pkNext, before);
    if preceding != 0 {
        e.set(
            Ptr::<NiTListItem>::new(preceding),
            NiTListItem::m_pkNext,
            item,
        );
    }
    e.set(at, NiTListItem::m_pkPrev, item);
}

// Translated from 00b5cde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sorts `lShadowLightList` around its two fences by each light's
/// LOD dimmer target (+0xd8) and elapsed time (+0xdc). `thiscall`, `RET 4` (a light). Each
/// fence is set first: a missing one is made by `00c1b4a0` on its fence object (+0xdc for the
/// fading-in fence, +0xd8 for the active one) and becomes the list's head, an existing one
/// goes through `00b702e0` with the head. Then for each item after the fading-in fence, with
/// `low` and `high` the `double`s at `0101ffa0` and `0106b9e8` and `time` the `float` at
/// `011ad834`: a light whose target is at most `low` and whose elapsed time is below `time`
/// moves before the active fence (unless it is that fence); one whose target is at least
/// `high`, elapsed below `time`, with `elapsed / time` (or `1 - elapsed / time` unless the
/// target is above 0) above `low`, moves after the active fence (`00b5a8d0`); otherwise a light
/// whose `NiLight` is not culled and whose target is below `high` or elapsed time not below
/// `time` goes after the fading-in fence (`00b702e0` for the argument light, `00b5a8d0` for any
/// other).
pub fn fn_00b5cde0(e: &mut Engine, this: Ptr<ShadowSceneNode>, argument: u32) {
    let base = this.addr();
    let list = base.wrapping_add(SHADOW_LIGHT_LIST);
    for (fence, object) in [(0xd4u32, 0xdcu32), (0xd0, 0xd8)] {
        let current = e.mem.u32(base.wrapping_add(fence));
        if current == 0 {
            e.call(LIST_00C1B4A0, &args![list, base.wrapping_add(object)]);
            let head = list_head(e, list);
            e.mem.set_u32(base.wrapping_add(fence), head);
        } else {
            let head = list_head(e, list);
            e.call(LIST_00B702E0, &args![list, current, head]);
        }
    }
    let fading = e.get(this, ShadowSceneNode::kShadowLightFadingInFence);
    if fading == 0 {
        return;
    }
    let mut next = item_next(e, fading);
    if next == 0 {
        return;
    }
    let mut time = x87_load32(e.global::<u32>(FADE_TIME_011AD834));
    while next != 0 {
        let low = x87_load64(e.global::<u64>(FADE_LOW_0101FFA0));
        let high = x87_load64(e.global::<u64>(FADE_HIGH_0106B9E8));
        let item = next;
        next = item_next(e, item);
        let light = item_element(e, item);
        if light == 0 {
            continue;
        }
        let target = x87_load32(e.mem.u32(light.wrapping_add(0xd8)));
        let elapsed = x87_load32(e.mem.u32(light.wrapping_add(0xdc)));
        if target <= low && elapsed < time {
            let active = e.get(this, ShadowSceneNode::kShadowLightActiveFence);
            if item != active {
                move_before(e, list, item, active);
                time = x87_load32(e.global::<u32>(FADE_TIME_011AD834));
            }
            continue;
        }
        if target >= high && elapsed < time {
            let mut share = x87_div(elapsed, time);
            if !(0.0 < x87_m32(e.mem.u32(light.wrapping_add(0xd8)))) {
                share = x87_sub(1.0, share);
            }
            let share = x87_load32(x87_store32(share));
            if share > low {
                let active = e.get(this, ShadowSceneNode::kShadowLightActiveFence);
                fn_00b5a8d0(e, Ptr::new(list), item, active);
                time = x87_load32(e.global::<u32>(FADE_TIME_011AD834));
                continue;
            }
        }
        let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
        if e.mem.u32(ni_light.wrapping_add(AV_FLAGS)) & 1 != 0 {
            continue;
        }
        let target = x87_load32(e.mem.u32(light.wrapping_add(0xd8)));
        if target >= high {
            let elapsed = x87_load32(e.mem.u32(light.wrapping_add(0xdc)));
            if elapsed < time {
                continue;
            }
        }
        let fading = e.get(this, ShadowSceneNode::kShadowLightFadingInFence);
        if light == argument {
            e.call(LIST_00B702E0, &args![list, item, fading]);
        } else {
            fn_00b5a8d0(e, Ptr::new(list), item, fading);
        }
        time = x87_load32(e.global::<u32>(FADE_TIME_011AD834));
    }
}

// Translated from 00b5d040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::RemoveQueuedLight` (Xbox PDB). `thiscall`, `RET 8` (an `NiPointer` to the
/// light and the list position word). With a portal graph whose +0x28 is set: the light is
/// taken out of the arrays at +0xdc (count +0xe4) of each object of its first array
/// (+0x214, count +0x218; `00b5a930` with the comparison `00b5a980`, then `00b5ade0`), and out
/// of the array at +0xb0 (count +0xb8, buffer +0xb4) of the +0x104 object of each entry of its
/// second array (+0x224, count +0x228). Then the position is removed from `lShadowLightList`
/// (a light with `bCastShadow`) or `lLightList` (`00b9efa0`, its out `NiPointer` being the
/// position's own slot) and the removed `NiPointer` is released.
pub fn shadow_scene_node_remove_queued_light(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    light_pointer: Ptr,
    position: u32,
) {
    let holder = light_pointer.addr();
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    if graph != 0 && e.mem.u32(graph.wrapping_add(0x28)) != 0 {
        let mut index = 0u32;
        while index < e.mem.u32(e.mem.u32(holder).wrapping_add(0x218)) {
            let light = e.mem.u32(holder);
            let array = e.mem.u32(light.wrapping_add(0x214));
            let object = e.mem.u32(array.wrapping_add(4 * index));
            if object != 0 && e.mem.u32(object.wrapping_add(0xe4)) != 0 {
                let members = object.wrapping_add(0xdc);
                let found = e
                    .call(
                        ARRAY_FIND_00B5A930,
                        &args![members, holder, 0u32, ARRAY_COMPARE_00B5A980],
                    )
                    .u32();
                if found != u32::MAX {
                    e.call(ARRAY_REMOVE_00B5ADE0, &args![members, found, 1u32]);
                }
            }
            index += 1;
        }
        let mut index = 0u32;
        while index < e.mem.u32(e.mem.u32(holder).wrapping_add(0x228)) {
            let light = e.mem.u32(holder);
            let array = e.mem.u32(light.wrapping_add(0x224));
            let entry = e.mem.u32(array.wrapping_add(4 * index));
            if entry != 0 {
                let object = e.mem.u32(entry.wrapping_add(0x104));
                let count = e.mem.u32(object.wrapping_add(0xb8));
                if count != 0 {
                    let members = object.wrapping_add(0xb0);
                    let mut slot = 0u32;
                    let mut found = None;
                    while slot < count {
                        let buffer = e.mem.u32(members.wrapping_add(4));
                        if buffer != 0 && e.mem.u32(buffer.wrapping_add(4 * slot)) == light {
                            found = Some(slot);
                            break;
                        }
                        slot += 1;
                    }
                    if let Some(slot) = found {
                        if slot != u32::MAX {
                            e.call(ARRAY_REMOVE_00B5ADE0, &args![members, slot, 1u32]);
                        }
                    }
                }
            }
            index += 1;
        }
    }
    let light = e.mem.u32(holder);
    let list = if e.mem.u8(light.wrapping_add(LIGHT_CAST_SHADOW)) != 0 {
        SHADOW_LIGHT_LIST
    } else {
        LIGHT_LIST
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), position);
        e.call(
            LIST_REMOVE_POS,
            &args![this.addr().wrapping_add(list), slot, position],
        );
        let removed = e.mem.u32(slot.addr());
        if removed != 0 {
            release(e, removed);
        }
    });
}

// Translated from 00b5d180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: queues every light of `lLightList` for removal (when
/// `bAllowLightRemoveQueues` is set: under the light-queue lock, appended to
/// `lLightQueueRemove` through an `NiPointer` temporary), then runs `00b5c3c0`, empties
/// `lShadowLightList`, clears both fences and runs `00b5c3c0` again. `thiscall`, no stack
/// words.
// Translation incomplete: 00b5d180 - the C++ exception frame (FS:[0] chain, unwind state 0 for the temporary) is not translated.
pub fn fn_00b5d180(e: &mut Engine, this: Ptr<ShadowSceneNode>) {
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light == 0 {
            continue;
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), light);
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light.wrapping_add(4)]);
            if e.get(this, ShadowSceneNode::bAllowLightRemoveQueues) != 0 {
                e.call(SPIN_LOCK_LOCK, &args![QUEUE_LOCK, 0u32]);
                e.call(
                    LIST_ADD_TAIL_00A5A310,
                    &args![this.addr().wrapping_add(LIGHT_QUEUE_REMOVE), slot],
                );
                unlock_queue(e);
            }
            release(e, light);
        });
    }
    fn_00b5c3c0(e, this);
    e.call(
        LIST_REMOVE_ALL,
        &args![this.addr().wrapping_add(SHADOW_LIGHT_LIST)],
    );
    e.set(this, ShadowSceneNode::kShadowLightActiveFence, 0);
    e.set(this, ShadowSceneNode::kShadowLightFadingInFence, 0);
    fn_00b5c3c0(e, this);
}

// Translated from 00b5d270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: appends a light to `lLightQueueUpdate` through an `NiPointer`
/// temporary. `thiscall`, `RET 4`; nothing for none.
// Translation incomplete: 00b5d270 - the C++ exception frame (FS:[0] chain, unwind state 0 for the temporary) is not translated.
pub fn fn_00b5d270(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32) {
    if light != 0 {
        add_tail_held(e, this, LIGHT_QUEUE_UPDATE, light, false);
    }
}

// Translated from 00b5d930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: follows a moved dynamic point light. `thiscall`, `RET 4` (a
/// `ShadowSceneLight`). Only for a dynamic point light without `bCastShadow`: with a culled
/// `NiLight` (flag 1) the light's `00b9ef30` runs; otherwise, when its `bPointPosition`
/// differs from the `NiLight`'s world position (`00439090`) and `usFrustumCull` is not 0xff,
/// the position is copied (`00b5a9d0`) and the light updated (`00b5d300`).
pub fn fn_00b5d930(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32) {
    if e.mem.u8(light.wrapping_add(LIGHT_POINT_LIGHT)) == 0
        || e.mem.u8(light.wrapping_add(LIGHT_DYNAMIC)) == 0
        || e.mem.u8(light.wrapping_add(LIGHT_CAST_SHADOW)) != 0
    {
        return;
    }
    let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
    if e.mem.u32(ni_light.wrapping_add(AV_FLAGS)) & 1 != 0 {
        e.call(LIGHT_00B9EF30, &args![light]);
        return;
    }
    let world = ni_light.wrapping_add(AV_WORLD_TRANSLATE);
    let differs = e.with_stack(0xc, |e, point| {
        for word in 0..3u32 {
            let value = e
                .mem
                .u32(light.wrapping_add(LIGHT_POINT_POSITION + 4 * word));
            e.mem.set_u32(point.addr() + 4 * word, value);
        }
        e.call(POINT_COMPARE_00439090, &args![point, world]).u8()
    });
    if differs == 0 || e.mem.u16(light.wrapping_add(LIGHT_FRUSTUM_CULL)) == 0xff {
        return;
    }
    let x = e.mem.u32(world);
    let y = e.mem.u32(world.wrapping_add(4));
    let z = e.mem.u32(world.wrapping_add(8));
    fn_00b5a9d0(e, Ptr::new(light), x, y, z);
    fn_00b5d300(e, this, light);
}

// Translated from 00b5d9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateObjectLighting` (Xbox PDB). `thiscall`, `RET 8` (an object and the
/// active-only flag). Under the object-queue lock (`011f9ec0`), the object is appended through
/// an `NiPointer` temporary to `lObjectQueueUpdate`, or to `lObjectQueueUpdateActiveOnly`
/// with the flag. Nothing for no object.
// Translation incomplete: 00b5d9f0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the temporaries) is not translated.
pub fn shadow_scene_node_update_object_lighting(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    object: u32,
    active_only: u8,
) {
    if object == 0 {
        return;
    }
    e.call(SPIN_LOCK_LOCK, &args![OBJECT_QUEUE_LOCK, 0u32]);
    let list = if active_only == 0 {
        OBJECT_QUEUE_UPDATE
    } else {
        OBJECT_QUEUE_UPDATE_ACTIVE_ONLY
    };
    add_tail_held(e, this, list, object, false);
    let count = e.global::<u32>(OBJECT_QUEUE_LOCK_COUNT).wrapping_sub(1);
    e.set_global(OBJECT_QUEUE_LOCK_COUNT, count);
    if count == 0 {
        e.set_global(OBJECT_QUEUE_LOCK, 0u32);
    }
}

// Translated from 00b5eca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds a light to `lLightList`. `thiscall`, `RET 4` (a
/// `ShadowSceneLight`). When a light with the same `NiLight` is listed already, it takes the
/// new light's `bDynamicLight`, is updated (`00b5d300`) and is answered. Otherwise the light is
/// appended through an `NiPointer` temporary; with `bCastShadow` it gets a dynamic shadow map
/// (`00b5aee0`): when the byte at address 0xed is set (a read of absolute address 0xed, as the
/// exe has it), the renderer's `00b6e110` (the word at `011f9508`, 0x2a, 0, 0, 0), its
/// `StopOffscreen` and that object's slot +0xac with the renderer's `00b6d120`; otherwise the
/// renderer's `00b6d3f0`. Then it is updated (`00b5d300`) and 0 is answered.
// Translation incomplete: 00b5eca0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the temporary) is not translated.
pub fn fn_00b5eca0(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32) -> u32 {
    let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    while item != 0 {
        let listed = item_element(e, item);
        item = item_next(e, item);
        if listed != 0 && e.mem.u32(listed.wrapping_add(LIGHT_SP_LIGHT)) == ni_light {
            let dynamic = e.mem.u8(light.wrapping_add(LIGHT_DYNAMIC));
            e.mem.set_u8(listed.wrapping_add(LIGHT_DYNAMIC), dynamic);
            fn_00b5d300(e, this, listed);
            return listed;
        }
    }
    add_tail_held(e, this, LIGHT_LIST, light, false);
    if e.mem.u8(light.wrapping_add(LIGHT_CAST_SHADOW)) != 0 {
        let map = if e.mem.u8(0xed) != 0 {
            let renderer = e.global::<u32>(RENDERER_011F91A8);
            let argument = e.global::<u32>(RENDERER_ARGUMENT_011F9508);
            let map = e
                .call(
                    RENDERER_00B6E110,
                    &args![renderer, argument, 0x2au32, 0u32, 0u32, 0u32],
                )
                .u32();
            let target = e.call(RENDERED_TEXTURE_STOP_OFFSCREEN, &args![map]).u32();
            // The slot is read before `00b6d120` runs.
            let function = e.mem.u32(e.mem.u32(target).wrapping_add(0xac));
            let renderer = e.global::<u32>(RENDERER_011F91A8);
            let value = e.call(RENDERER_00B6D120, &args![renderer]).u32();
            e.call(function, &args![target, value]);
            map
        } else {
            let renderer = e.global::<u32>(RENDERER_011F91A8);
            e.call(RENDERER_00B6D3F0, &args![renderer]).u32()
        };
        fn_00b5aee0(e, Ptr::new(light), map);
    }
    fn_00b5d300(e, this, light);
    0
}

/// `00b5edf0`'s and `00b5eed0`'s queueing of a found light for removal (`00b5cff0` through an
/// `NiPointer` temporary).
fn queue_removal_held(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), light);
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light.wrapping_add(4)]);
        fn_00b5cff0(e, this, slot);
        release(e, light);
    });
}

// Translated from 00b5edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: queues the shadow light whose `spShadowScene` is the object for
/// removal (`00b5cff0`). `thiscall`, `RET 4`; nothing for no object or no such light.
// Translation incomplete: 00b5edf0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the temporary) is not translated.
pub fn fn_00b5edf0(e: &mut Engine, this: Ptr<ShadowSceneNode>, object: u32) {
    if object == 0 {
        return;
    }
    let mut item = list_head(e, this.addr().wrapping_add(SHADOW_LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light != 0 && e.mem.u32(light.wrapping_add(LIGHT_SHADOW_SCENE)) == object {
            queue_removal_held(e, this, light);
            return;
        }
    }
}

// Translated from 00b5eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::RemoveLight` (Xbox PDB). `thiscall`, `RET 4` (an `NiLight`). When
/// `bAllowLightRemoveQueues` is set, the light of `lLightList` with that `NiLight` is queued
/// for removal (`00b5cff0`).
// Translation incomplete: 00b5eed0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the temporary) is not translated.
pub fn shadow_scene_node_remove_light(e: &mut Engine, this: Ptr<ShadowSceneNode>, ni_light: u32) {
    if ni_light == 0 || e.get(this, ShadowSceneNode::bAllowLightRemoveQueues) == 0 {
        return;
    }
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light != 0 && e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT)) == ni_light {
            queue_removal_held(e, this, light);
            return;
        }
    }
}

// Translated from 00b5ef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::RemoveQueuedLight` (Xbox PDB, overload 2). `thiscall`, `RET 4` (an
/// `NiPointer` to the light). A light of another portal graph (its +0x240) is removed from
/// `lShadowLightList` (with `bCastShadow`) or `lLightList` by `00b9fcc0` (its out `NiPointer`
/// being the argument's own slot, released after). A light of this graph is looked up in that
/// list and, when found, removed with `RemoveQueuedLight` (the item as the position).
pub fn shadow_scene_node_remove_queued_light_ov2(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    light_pointer: Ptr,
) {
    let light = e.mem.u32(light_pointer.addr());
    let list = if e.mem.u8(light.wrapping_add(LIGHT_CAST_SHADOW)) != 0 {
        SHADOW_LIGHT_LIST
    } else {
        LIGHT_LIST
    };
    let list = this.addr().wrapping_add(list);
    let graph = e.mem.u32(light.wrapping_add(LIGHT_PORTAL_GRAPH));
    if graph != e.get(this, ShadowSceneNode::pBSPortalGraph) {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), light_pointer.addr());
            e.call(LIST_REMOVE, &args![list, slot, light_pointer]);
            let removed = e.mem.u32(slot.addr());
            if removed != 0 {
                release(e, removed);
            }
        });
        return;
    }
    let mut item = list_head(e, list);
    let mut found = 0;
    while item != 0 {
        let next = item_next(e, item);
        if light == item_element(e, item) {
            found = item;
            break;
        }
        item = next;
    }
    if found != 0 {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), found);
            shadow_scene_node_remove_queued_light(e, this, light_pointer, slot.addr());
        });
    }
}

/// `fn_00b5bca0`'s data: the three `double`s (3.0 at `01021928`, 0.4 at `010228a0`, 1000.0 at
/// `01017b70`), 0.0 (`double`, `01012060`), the `float` 0.01 (`01013ea4`), the guard word of a
/// function-local static (`011f9e78`), and its three arrays: weights (`011f9838` and
/// `011f96a8`, 100 `float`s each) and light offsets (`011f99c8`, 100 points).
const LIGHT_SUM_DIVISOR_01021928: u32 = 0x0102_1928;
const SUN_LUMINANCE_LIMIT_010228A0: u32 = 0x0102_28a0;
const DIRECTION_SCALE_01017B70: u32 = 0x0101_7b70;
const ZERO_01012060: u32 = 0x0101_2060;
const WEIGHT_LIMIT_01013EA4: u32 = 0x0101_3ea4;
const STATIC_GUARD_011F9E78: u32 = 0x011f_9e78;
const WEIGHTS_011F9838: u32 = 0x011f_9838;
const WEIGHTS_011F96A8: u32 = 0x011f_96a8;
const OFFSETS_011F99C8: u32 = 0x011f_99c8;
/// `NiPoint3::Unitize`-shaped normalization in place (`004a0c10`, `thiscall`, no stack words)
/// and `_CIsqrt` (`00ec6040`, `ST0` in and out).
const POINT_NORMALIZE_004A0C10: u32 = 0x004a_0c10;
const CRT_SQRT: u32 = 0x00ec_6040;

/// `fn_00b5bca0`'s luminance of an `NiLight`: its dimmer (+0xc4) times each component of its
/// colour (+0xd4), summed `(g + r) + b` and divided by `[01021928]`, as a `float`.
fn light_luminance(e: &Engine, ni_light: u32) -> u32 {
    let dimmer = x87_load32(x87_store32(x87_load32(
        e.mem.u32(ni_light.wrapping_add(0xc4)),
    )));
    let r = x87_store32(x87_mul(
        dimmer,
        x87_m32(e.mem.u32(ni_light.wrapping_add(0xd4))),
    ));
    let g = x87_store32(x87_mul(
        dimmer,
        x87_m32(e.mem.u32(ni_light.wrapping_add(0xd8))),
    ));
    let b = x87_store32(x87_mul(
        dimmer,
        x87_m32(e.mem.u32(ni_light.wrapping_add(0xdc))),
    ));
    let divisor: f64 = e.global(LIGHT_SUM_DIVISOR_01021928);
    let sum = x87_add(x87_add(x87_load32(g), x87_m32(r)), x87_m32(b));
    x87_store32(x87_div(sum, divisor))
}

/// `fn_00b5bca0`'s fallback direction: the negated direction at `ni_light + 0xf0`, normalized
/// in a local (`004a0c10`) and scaled by `[01017b70]` (`DC C9`, `D8 C9`, then a memory
/// `FMUL`), as three `float` bit patterns.
fn scaled_reverse_direction(e: &mut Engine, ni_light: u32) -> [u32; 3] {
    e.with_stack(0xc, |e, local| {
        for word in 0..3u32 {
            let value = x87_load32(e.mem.u32(ni_light.wrapping_add(0xf0 + 4 * word)));
            e.mem.set_u32(local.addr() + 4 * word, x87_store32(-value));
        }
        e.call(POINT_NORMALIZE_004A0C10, &args![local]);
        let scale = x87_load64(e.global::<u64>(DIRECTION_SCALE_01017B70));
        let x = x87_store32(x87_mul(x87_load32(e.mem.u32(local.addr())), scale));
        let y = x87_store32(x87_mul(x87_load32(e.mem.u32(local.addr() + 4)), scale));
        let z = x87_store32(x87_mul(scale, x87_m32(e.mem.u32(local.addr() + 8))));
        [x, y, z]
    })
}

// Translated from 00b5bca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the dominant light direction at a point. `thiscall`, `RET 0xc`:
/// the point, a direction (both three `float`s) and the output point. When the sun's
/// luminance (dimmer times colour, averaged by `[01021928]`) is above `[010228a0]`, the
/// offset is the sun's reverse direction scaled by `[01017b70]` and 1 is answered. Otherwise,
/// for up to 100 lights of `lLightList` whose `NiLight` is not culled: the offset from the
/// point to the light, its normalized copy's dot product with the direction (clamped below
/// at 0), the distance (`_CIsqrt`) less the larger of the components of the point at
/// `011f426c`, over the light's radius (+0xe0), gives a falloff `1 - q` clamped to [0, 1]; the
/// weight `falloff * dot * luminance` is kept twice (`011f9838`, `011f96a8`) with the offset
/// (`011f99c8`), summed, and counted when above `[01013ea4]`. With no counted weight the sun's
/// direction is used as above (1 answered); otherwise each offset with a positive share of
/// the sum is added in by that share, and the light count is answered (as 0 or 1). The output
/// is the point plus the summed offset.
pub fn fn_00b5bca0(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    point: Ptr,
    direction: Ptr,
    out: Ptr,
) -> u32 {
    let sun = e.get(this, ShadowSceneNode::pSunLight);
    let sun_light = e.mem.u32(sun.wrapping_add(LIGHT_SP_LIGHT));
    let luminance = light_luminance(e, sun_light);
    let base = [
        e.global::<u32>(POINT_011F426C),
        e.global::<u32>(POINT_011F426C + 4),
        e.global::<u32>(POINT_011F426C + 8),
    ];
    let mut result = base;
    let mut count = 0u32;
    let limit: f64 = e.global(SUN_LUMINANCE_LIMIT_010228A0);
    let lum = x87_load32(luminance);
    let mut use_sun = false;
    if lum > limit {
        result = scaled_reverse_direction(e, sun_light);
        count = 1;
    } else {
        let guard: u32 = e.global(STATIC_GUARD_011F9E78);
        if guard & 1 == 0 {
            e.set_global(STATIC_GUARD_011F9E78, guard | 1);
        }
        let first_max = x87_store32(x87_keep_larger(x87_load32(base[0]), x87_load32(base[1])));
        let bias = x87_store32(x87_keep_larger(x87_load32(first_max), x87_load32(base[2])));
        let mut sum = x87_store32(0.0);
        let mut positive = 0i32;
        let mut index = 0u32;
        let mut list_index = 0u32;
        let mut light = fn_00b5ac20(e, this, 0);
        if light == 0 {
            use_sun = true;
        } else {
            let mut offset_bytes = 0u32;
            loop {
                if offset_bytes >= 0x4b0 {
                    break;
                }
                let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
                if e.mem.u32(ni_light.wrapping_add(AV_FLAGS)) & 1 == 0 {
                    let mut delta = [0u32; 3];
                    for word in 0..3u32 {
                        let a = x87_load32(
                            e.mem
                                .u32(ni_light.wrapping_add(AV_WORLD_TRANSLATE + 4 * word)),
                        );
                        let b = e.mem.u32(point.addr().wrapping_add(4 * word));
                        delta[word as usize] = x87_store32(x87_sub(a, x87_m32(b)));
                    }
                    let unit = e.with_stack(0xc, |e, local| {
                        for word in 0..3u32 {
                            e.mem.set_u32(local.addr() + 4 * word, delta[word as usize]);
                        }
                        e.call(POINT_NORMALIZE_004A0C10, &args![local]);
                        [
                            e.mem.u32(local.addr()),
                            e.mem.u32(local.addr() + 4),
                            e.mem.u32(local.addr() + 8),
                        ]
                    });
                    let d = |i: u32| e.mem.u32(direction.addr().wrapping_add(4 * i));
                    let (dx, dy, dz) = (d(0), d(1), d(2));
                    let y_term = x87_mul(x87_load32(dy), x87_m32(unit[1]));
                    let x_term = x87_mul(x87_load32(dx), x87_m32(unit[0]));
                    let z_term = x87_mul(x87_load32(dz), x87_m32(unit[2]));
                    let mut dot = x87_store32(x87_add(x87_add(y_term, x_term), z_term));
                    // `FLDZ; FCOM [dot]; TEST AH,0x41; JNZ`: 0 replaces a negative dot.
                    if 0.0 > x87_m32(dot) {
                        dot = x87_store32(0.0);
                    }
                    let light_lum = light_luminance(e, ni_light);
                    let (ex, ey, ez) = (
                        x87_load32(delta[0]),
                        x87_load32(delta[1]),
                        x87_load32(delta[2]),
                    );
                    let squared =
                        x87_add(x87_add(x87_mul(ey, ey), x87_mul(ex, ex)), x87_mul(ez, ez));
                    let squared = x87_store32(squared);
                    let root = e.call(CRT_SQRT, &args![x87_load32(squared)]).st0;
                    let distance = x87_store32(root);
                    let q = x87_div(
                        x87_sub(x87_load32(distance), x87_m32(bias)),
                        x87_m32(e.mem.u32(ni_light.wrapping_add(0xe0))),
                    );
                    let mut falloff = x87_store32(x87_sub(1.0, q));
                    let zero: f64 = e.global(ZERO_01012060);
                    if x87_load32(falloff) < zero {
                        falloff = x87_store32(0.0);
                    }
                    let f = x87_load32(falloff);
                    if f > 1.0 {
                        falloff = x87_store32(1.0);
                    }
                    let weight = x87_mul(
                        x87_mul(x87_load32(falloff), x87_m32(dot)),
                        x87_m32(light_lum),
                    );
                    let weight = x87_quiet_bits(x87_store32(weight));
                    e.set_global(WEIGHTS_011F9838 + 4 * index, weight);
                    e.set_global(WEIGHTS_011F96A8 + 4 * index, weight);
                    let weight_limit = x87_load32(e.global::<u32>(WEIGHT_LIMIT_01013EA4));
                    if weight_limit < x87_load32(weight) {
                        positive += 1;
                    }
                    sum = x87_store32(x87_add(x87_load32(weight), x87_m32(sum)));
                    let slot = OFFSETS_011F99C8 + 0xc * index;
                    for word in 0..3u32 {
                        e.set_global(slot + 4 * word, delta[word as usize]);
                    }
                    offset_bytes += 0xc;
                    index += 1;
                }
                // The next light, walked from the head (`m_uiCount` bounds the 16-bit index).
                let list = this.addr().wrapping_add(LIGHT_LIST);
                list_index = list_index.wrapping_add(1);
                let position = list_index & 0xffff;
                if position >= e.get(Ptr::<NiTPointerList>::new(list), NiTPointerList::m_uiCount) {
                    break;
                }
                let head = list_head(e, list);
                let mut element = item_element(e, head);
                let mut next = item_next(e, head);
                for _ in 0..position {
                    element = item_element(e, next);
                    next = item_next(e, next);
                }
                light = element;
                if light == 0 {
                    break;
                }
            }
            if positive <= 0 {
                use_sun = true;
            } else {
                let total = x87_load32(sum);
                for k in 0..index {
                    let share = x87_store32(x87_div(
                        x87_load32(e.global::<u32>(WEIGHTS_011F96A8 + 4 * k)),
                        total,
                    ));
                    let s = x87_load32(share);
                    if !(s > 0.0) {
                        continue;
                    }
                    let slot = OFFSETS_011F99C8 + 0xc * k;
                    let tx = x87_store32(x87_mul(x87_load32(e.global::<u32>(slot)), s));
                    let ty = x87_store32(x87_mul(x87_load32(e.global::<u32>(slot + 4)), s));
                    let tz = x87_store32(x87_mul(s, x87_m32(e.global::<u32>(slot + 8))));
                    result[0] = x87_store32(x87_add(x87_load32(tx), x87_m32(result[0])));
                    result[1] = x87_store32(x87_add(x87_load32(result[1]), x87_m32(ty)));
                    result[2] = x87_store32(x87_add(x87_load32(result[2]), x87_m32(tz)));
                }
                count = index;
            }
        }
    }
    if use_sun {
        let sun = e.get(this, ShadowSceneNode::pSunLight);
        let sun_light = e.mem.u32(sun.wrapping_add(LIGHT_SP_LIGHT));
        result = scaled_reverse_direction(e, sun_light);
        count = 1;
    }
    for word in 0..3u32 {
        let p = x87_load32(e.mem.u32(point.addr().wrapping_add(4 * word)));
        let value = x87_store32(x87_add(p, x87_m32(result[word as usize])));
        e.mem.set_u32(out.addr().wrapping_add(4 * word), value);
    }
    u32::from(count != 0)
}

/// More callees: `00b9fba0` and `00ba02b0` (`ShadowSceneLight`, no stack words),
/// `ShadowSceneLight::AddObject` (`00ba0110`, `RET 4`), the slot reservation `00761540`
/// (`thiscall` on a `BSSimpleArray`-shaped array), the array removal `00b5aea0` (`RET 8`),
/// `BSPortalGraph::GetAlwaysRenderArrayCount` (`00c5b5d0`) and `GetAlwaysRenderAt` (`00c5b5a0`,
/// `RET 4`), and the `double` at `0103c7c8` a light's dimmer is compared with.
const LIGHT_00B9FBA0: u32 = 0x00b9_fba0;
const LIGHT_00BA02B0: u32 = 0x00ba_02b0;
const LIGHT_ADD_OBJECT: u32 = 0x00ba_0110;
const ARRAY_RESERVE_00761540: u32 = 0x0076_1540;
const ARRAY_REMOVE_00B5AEA0: u32 = 0x00b5_aea0;
const PORTAL_GRAPH_ALWAYS_RENDER_COUNT: u32 = 0x00c5_b5d0;
const PORTAL_GRAPH_ALWAYS_RENDER_AT: u32 = 0x00c5_b5a0;
const DIMMER_LIMIT_0103C7C8: u32 = 0x0103_c7c8;

/// The index of `value` in the `BSSimpleArray`-shaped array at `array` (buffer at +4, read
/// once; count at +8), or `None`; `00b5d300`'s inlined search.
fn array_index_of(e: &Engine, array: u32, value: u32) -> Option<u32> {
    let count = e.mem.u32(array.wrapping_add(8));
    let buffer = e.mem.u32(array.wrapping_add(4));
    let mut index = 0u32;
    while index < count {
        if buffer != 0 && e.mem.u32(buffer.wrapping_add(4 * index)) == value {
            return Some(index);
        }
        index += 1;
    }
    None
}

/// `00b5d300`'s inlined append: a slot reserved by `fn_00b5c360` (or `00761540`) takes `value`.
fn array_append(e: &mut Engine, array: u32, value: u32, generic: bool) {
    let slot = if generic {
        e.call(ARRAY_RESERVE_00761540, &args![array]).u32()
    } else {
        fn_00b5c360(e, Ptr::new(array))
    };
    let buffer = e.mem.u32(array.wrapping_add(4));
    e.mem.set_u32(buffer.wrapping_add(4 * slot), value);
}

/// `00b5d300`'s link of a light and a portal: the light into the portal's +0x104 object's
/// array at +0xb0, that object into the light's +0x230 array (added with
/// `ShadowSceneLight::AddObject` when it is new), and the portal into the light's +0x220 array;
/// each only when missing.
fn link_light_and_portal(e: &mut Engine, light: u32, portal: u32) {
    let target = e.mem.u32(portal.wrapping_add(0x104));
    let lights = target.wrapping_add(0xb0);
    if array_index_of(e, lights, light).is_none() {
        array_append(e, lights, light, false);
    }
    let target = e.mem.u32(portal.wrapping_add(0x104));
    if array_index_of(e, light.wrapping_add(0x230), target).is_none() {
        e.call(LIGHT_ADD_OBJECT, &args![light, target]);
        let target = e.mem.u32(portal.wrapping_add(0x104));
        array_append(e, light.wrapping_add(0x230), target, true);
    }
    if array_index_of(e, light.wrapping_add(0x220), portal).is_none() {
        array_append(e, light.wrapping_add(0x220), portal, false);
    }
}

/// Frees a `BSSimpleArray`-shaped array's buffer (its virtual slot +8) and clears buffer,
/// reserved size and size, when it has a buffer.
fn clear_simple_array(e: &mut Engine, array: u32) {
    let buffer = e.mem.u32(array.wrapping_add(4));
    if buffer != 0 {
        e.vcall(array, 8, &args![buffer]);
        e.mem.set_u32(array.wrapping_add(4), 0);
        e.mem.set_u32(array.wrapping_add(0xc), 0);
        e.mem.set_u32(array.wrapping_add(8), 0);
    }
}

/// The portal graph's always-rendered objects passed to `AddObject` (`00b5d300`); with
/// `parentless`, only those without a parent (+0x18).
fn add_always_rendered(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32, parentless: bool) {
    let mut index = 0u32;
    loop {
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        if index
            >= e.call(PORTAL_GRAPH_ALWAYS_RENDER_COUNT, &args![graph])
                .u32()
        {
            break;
        }
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        let object = e
            .call(PORTAL_GRAPH_ALWAYS_RENDER_AT, &args![graph, index])
            .u32();
        if object != 0 && (!parentless || e.mem.u32(object.wrapping_add(0x18)) == 0) {
            e.call(LIGHT_ADD_OBJECT, &args![light, object]);
        }
        index += 1;
    }
}

/// `00b5d300`'s handling of a node child: its `IsNode` answer is added (`AddObject`).
fn add_child_node(e: &mut Engine, light: u32, child: u32) {
    if child != 0 {
        let node = e.vcall(child, SLOT_IS_NODE, &args![]).u32();
        if node != 0 {
            e.call(LIGHT_ADD_OBJECT, &args![light, node]);
        }
    }
}

// Translated from 00b5d300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: updates a light's rooms, portals and objects. `thiscall`,
/// `RET 4` (a `ShadowSceneLight`). A light whose `NiLight` is culled, or whose dimmer (+0xc4)
/// is below the `double` at `0103c7c8`, gets `00b9ef30` instead. Otherwise the light takes the
/// node's portal graph (+0x240) and runs `00b9fba0`. With a portal graph whose +0x28 is set:
/// its three arrays (+0x210, +0x220, +0x230) are cleared; for each room of the graph (list at
/// +0x20) the light (searched in the room's +0xdc array with `00b5a930`) is tested against the
/// room (the room shape's slot +0xb4 on the `NiLight`'s position, then the room's slot +0x110
/// on the bound made of that position and the `NiLight`'s radius at +0xe0): a light outside
/// it is taken out of the room (`00b5aea0`) and out of each room portal's +0x104 array where
/// the portal is not in the light's +0x220 array; a light inside it is put in the room's
/// array, linked with each of the room's portals, and the room is added (`AddObject`) and put
/// in the light's +0x210 array. Then the graph's +0x6c objects (count +0x70) are added, the
/// graph's own portals (list at +0x14) linked, and the always-rendered objects added. Without
/// a usable portal graph, the node's child nodes are added (only children 3 and 6 for the node
/// at `011f91c8`) and, with a graph, its always-rendered objects without a parent. Both paths
/// end with `00ba02b0`.
pub fn fn_00b5d300(e: &mut Engine, this: Ptr<ShadowSceneNode>, light: u32) {
    light_check::before(e, this.addr(), light);
    let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
    if e.mem.u32(ni_light.wrapping_add(AV_FLAGS)) & 1 != 0 {
        e.call(LIGHT_00B9EF30, &args![light]);
        return;
    }
    let dimmer = x87_load32(e.mem.u32(ni_light.wrapping_add(0xc4)));
    let limit: f64 = e.global(DIMMER_LIMIT_0103C7C8);
    if dimmer < limit {
        e.call(LIGHT_00B9EF30, &args![light]);
        return;
    }
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    e.mem.set_u32(light.wrapping_add(LIGHT_PORTAL_GRAPH), graph);
    e.call(LIGHT_00B9FBA0, &args![light]);
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    let base = this.addr();
    if graph == 0 || e.mem.u32(graph.wrapping_add(0x28)) == 0 {
        let size = |e: &Engine| u32::from(e.mem.u16(base.wrapping_add(NODE_CHILDREN_SIZE)));
        let child_at = |e: &Engine, i: u32| {
            let children = e.mem.u32(base.wrapping_add(NODE_CHILDREN_BASE));
            e.mem.u32(children.wrapping_add(4 * i))
        };
        if base == e.global::<u32>(FALLBACK_SCENE_NODE_011F91C8) {
            if size(e) > 3 {
                let child = child_at(e, 3);
                add_child_node(e, light, child);
            }
            if size(e) > 6 {
                let child = child_at(e, 6);
                add_child_node(e, light, child);
            }
        } else {
            let mut index = 0u32;
            while index < size(e) {
                let child = child_at(e, index);
                add_child_node(e, light, child);
                index += 1;
            }
        }
        if e.get(this, ShadowSceneNode::pBSPortalGraph) != 0 {
            add_always_rendered(e, this, light, true);
        }
        e.call(LIGHT_00BA02B0, &args![light]);
        light_check::after(e, this.addr(), light);
        return;
    }
    for array in [0x210u32, 0x220, 0x230] {
        clear_simple_array(e, light.wrapping_add(array));
    }
    let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
    let position = [
        e.mem.u32(ni_light.wrapping_add(AV_WORLD_TRANSLATE)),
        e.mem.u32(ni_light.wrapping_add(AV_WORLD_TRANSLATE + 4)),
        e.mem.u32(ni_light.wrapping_add(AV_WORLD_TRANSLATE + 8)),
    ];
    let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
    // `FLD; FSTP; FLD; FSTP` of the radius.
    let radius = x87_quiet_bits(e.mem.u32(ni_light.wrapping_add(0xe0)));
    e.with_stack(0x24, |e, frame| {
        // The function's frame: its own argument slot (the light), the point, the bound and
        // the portal temporary. The exe (ESP-based, `SUB ESP,0x2c`) keeps them apart: the
        // portal at +0x4, the point at +0x10, the bound's centre at +0x1c and its radius at
        // +0x28 (`FSTP [ESP+0x38]` after four pushes), the light argument at +0x30. The bound is
        // 0x10 bytes, so the portal slot sits after it rather than on its radius.
        let light_slot = frame.addr();
        let point = light_slot + 4;
        let bound = light_slot + 0x10;
        let portal_slot = light_slot + 0x20;
        e.mem.set_u32(light_slot, light);
        for word in 0..3u32 {
            e.mem.set_u32(point + 4 * word, position[word as usize]);
            e.mem.set_u32(bound + 4 * word, position[word as usize]);
        }
        e.mem.set_u32(bound + 0xc, radius);
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        let mut rooms = e.mem.u32(graph.wrapping_add(0x20));
        while rooms != 0 {
            let room = item_element(e, rooms);
            rooms = item_next(e, rooms);
            if room == 0 {
                continue;
            }
            let members = room.wrapping_add(0xdc);
            let found = e
                .call(
                    ARRAY_FIND_00B5A930,
                    &args![members, light_slot, 0u32, ARRAY_COMPARE_00B5A980],
                )
                .u32();
            let shape_holder = e.mem.u32(room.wrapping_add(0xac));
            let shape = e.mem.u32(shape_holder.wrapping_add(0xc));
            let mut inside = e.vcall(shape, 0xb4, &args![point]).u8() != 0;
            if !inside {
                inside = e.vcall(room, 0x110, &args![bound]).u32() != 0;
            }
            if !inside {
                if found == u32::MAX {
                    continue;
                }
                e.call(ARRAY_REMOVE_00B5AEA0, &args![members, found, 1u32]);
                let mut portals = e.mem.u32(room.wrapping_add(0xb4));
                while portals != 0 {
                    let portal = item_element(e, portals);
                    portals = item_next(e, portals);
                    e.mem.set_u32(portal_slot, portal);
                    if portal == 0 {
                        continue;
                    }
                    let listed = e
                        .call(
                            ARRAY_FIND_00B5A930,
                            &args![
                                light.wrapping_add(0x220),
                                portal_slot,
                                0u32,
                                ARRAY_COMPARE_00B5A980
                            ],
                        )
                        .u32();
                    if listed != u32::MAX {
                        continue;
                    }
                    let target = e.mem.u32(portal.wrapping_add(0x104));
                    let index = e
                        .call(
                            ARRAY_FIND_00B5A930,
                            &args![
                                target.wrapping_add(0xb0),
                                light_slot,
                                0u32,
                                ARRAY_COMPARE_00B5A980
                            ],
                        )
                        .u32();
                    if index != u32::MAX {
                        let portal = e.mem.u32(portal_slot);
                        let target = e.mem.u32(portal.wrapping_add(0x104));
                        e.call(
                            ARRAY_REMOVE_00B5AEA0,
                            &args![target.wrapping_add(0xb0), index, 1u32],
                        );
                    }
                }
                continue;
            }
            if found == u32::MAX {
                array_append(e, members, light, false);
            }
            let mut portals = e.mem.u32(room.wrapping_add(0xb4));
            while portals != 0 {
                let portal = item_element(e, portals);
                portals = item_next(e, portals);
                if portal != 0 {
                    link_light_and_portal(e, light, portal);
                }
            }
            e.call(LIGHT_ADD_OBJECT, &args![light, room]);
            array_append(e, light.wrapping_add(0x210), room, false);
        }
    });
    let mut index = 0u32;
    loop {
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        if index >= e.mem.u32(graph.wrapping_add(0x70)) {
            break;
        }
        let objects = e.mem.u32(graph.wrapping_add(0x6c));
        let object = e.mem.u32(objects.wrapping_add(4 * index));
        e.call(LIGHT_ADD_OBJECT, &args![light, object]);
        index += 1;
    }
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    let mut portals = e.mem.u32(graph.wrapping_add(0x14));
    while portals != 0 {
        let portal = item_element(e, portals);
        portals = item_next(e, portals);
        if portal != 0 {
            link_light_and_portal(e, light, portal);
        }
    }
    add_always_rendered(e, this, light, false);
    e.call(LIGHT_00BA02B0, &args![light]);
}

/// `MemoryManager::GetThreadScrapHeap` (`00aa42e0`, on the memory manager `011f6238`),
/// `ScrapHeap::Allocate` (`00aa54a0`, `RET 8`: size and alignment) and `Deallocate`
/// (`00aa5610`, `RET 4`), the alignment word at `010a2720`, the CRT `qsort` (`00ec6f20`,
/// `cdecl`) and `ShadowSceneLight::GetLuminanceToBound` (`00b9dbe0`, `RET 4`, `float` in
/// `ST0`, discarded here).
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
const SCRAP_ALIGNMENT_010A2720: u32 = 0x010a_2720;
const CRT_QSORT: u32 = 0x00ec_6f20;
const LIGHT_GET_LUMINANCE_TO_BOUND: u32 = 0x00b9_dbe0;
/// The exact RTTI records `UpdateQueuedObjectLightingStart` tests an object's parent against
/// (`01202828`: a room-like parent with lights at +0xe0/+0xe4; `012030b8`: one with lights at
/// +0xb4/+0xb8).
const PARENT_RTTI_01202828: u32 = 0x0120_2828;
const PARENT_RTTI_012030B8: u32 = 0x0120_30b8;

/// `UpdateQueuedObjectLightingStart`'s collection of the lights within an object's bound.
fn collect_light(
    e: &mut Engine,
    light: u32,
    skip_culled: u32,
    bound: Ptr,
    array: u32,
    count: &mut u32,
) {
    if light == 0 {
        return;
    }
    if skip_culled & 0xff != 0 && e.mem.u16(light.wrapping_add(LIGHT_FRUSTUM_CULL)) == 0xff {
        return;
    }
    let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
    let within = e
        .call(
            LIGHT_BOUND_WITHIN_RADIUS,
            &args![light, bound, ni_light, 1.0f32],
        )
        .u8();
    if within == 0 {
        return;
    }
    e.mem.set_u32(array.wrapping_add(4 * *count), light);
    *count = count.wrapping_add(1);
    e.call(LIGHT_GET_LUMINANCE_TO_BOUND, &args![light, bound]);
}

// Translated from 00b5dac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::UpdateQueuedObjectLightingStart` (Xbox PDB). `cdecl`, four words (the
/// object, a scene node, the skip-culled flag and the collect flag). The object's parent
/// (+0x18) whose exact RTTI is `01202828` or `012030b8` gives the light collection. Without
/// the collect flag, a tri-based geometry goes to `UpdateQueuedObjectLighting` and anything
/// else to its overload 2 (the node, the flag, the two collections). With it, the lights of
/// the collection (or of the node's `lLightList`) that are within the object's bound (or the
/// null bound) and, with the skip flag, not frustum-culled (0xff) are gathered in a
/// null-terminated scrap-heap array (with `GetLuminanceToBound` on each), sorted with `qsort`
/// and `00b5aa70` when there are any, and passed to `UpdateQueuedObjectLightingList`
/// overload 2; the array is freed. Answers 1.
pub fn shadow_scene_node_update_queued_object_lighting_start(
    e: &mut Engine,
    object: u32,
    scene_node: u32,
    skip_culled: u32,
    collect: u32,
) -> u8 {
    if object == 0 {
        return 1;
    }
    let parent = e.mem.u32(object.wrapping_add(0x18));
    let mut rooms = 0u32;
    let mut others = 0u32;
    if parent != 0 {
        if rtti_is(e, parent, PARENT_RTTI_01202828) {
            rooms = parent;
        } else if rtti_is(e, parent, PARENT_RTTI_012030B8) {
            others = parent;
        }
    }
    if collect & 0xff == 0 {
        if e.vcall(object, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
            shadow_scene_node_update_queued_object_lighting(
                e,
                object,
                scene_node,
                skip_culled,
                rooms,
                others,
            );
        } else {
            shadow_scene_node_update_queued_object_lighting_ov2(
                e,
                object,
                scene_node,
                skip_culled,
                rooms,
                others,
            );
        }
        return 1;
    }
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![MEMORY_MANAGER]).u32();
    let mut count = 0u32;
    let mut source = e.mem.u32(object.wrapping_add(AV_WORLD_BOUND));
    if source == 0 {
        source = NULL_BOUND_011F4288;
    }
    e.with_stack(0x10, |e, bound| {
        for word in 0..4u32 {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(bound.addr() + 4 * word, value);
        }
        let alignment = e.global::<u32>(SCRAP_ALIGNMENT_010A2720);
        let array;
        let mut sort = true;
        if rooms != 0 {
            let total = e.mem.u32(rooms.wrapping_add(0xe4));
            array = e
                .call(
                    SCRAP_HEAP_ALLOCATE,
                    &args![heap, total.wrapping_mul(4).wrapping_add(4), alignment],
                )
                .u32();
            if e.mem.u32(rooms.wrapping_add(0xe4)) == 0 {
                sort = false;
            }
            let mut index = 0u32;
            while index < e.mem.u32(rooms.wrapping_add(0xe4)) {
                let lights = e.mem.u32(rooms.wrapping_add(0xe0));
                let light = e.mem.u32(lights.wrapping_add(4 * index));
                collect_light(e, light, skip_culled, bound, array, &mut count);
                index += 1;
            }
        } else if others != 0 {
            let total = e.mem.u32(others.wrapping_add(0xb8));
            array = e
                .call(
                    SCRAP_HEAP_ALLOCATE,
                    &args![heap, total.wrapping_mul(4).wrapping_add(4), alignment],
                )
                .u32();
            if e.mem.u32(others.wrapping_add(0xb8)) == 0 {
                sort = false;
            }
            let mut index = 0u32;
            while index < e.mem.u32(others.wrapping_add(0xb8)) {
                let lights = e.mem.u32(others.wrapping_add(0xb4));
                let light = e.mem.u32(lights.wrapping_add(4 * index));
                collect_light(e, light, skip_culled, bound, array, &mut count);
                index += 1;
            }
        } else {
            let total = e.mem.u32(scene_node.wrapping_add(LIGHT_LIST + 8));
            array = e
                .call(
                    SCRAP_HEAP_ALLOCATE,
                    &args![heap, total.wrapping_mul(4).wrapping_add(4), alignment],
                )
                .u32();
            let mut item = e.mem.u32(scene_node.wrapping_add(LIGHT_LIST));
            if item == 0 {
                sort = false;
            }
            while item != 0 {
                let light = item_element(e, item);
                item = item_next(e, item);
                collect_light(e, light, skip_culled, bound, array, &mut count);
            }
        }
        if sort && (count as i32) > 0 {
            e.call(CRT_QSORT, &args![array, count, 4u32, 0x00b5_aa70u32]);
        }
        e.mem.set_u32(array.wrapping_add(4 * count), 0);
        shadow_scene_node_update_queued_object_lighting_list_ov2(
            e,
            object,
            scene_node,
            array,
            skip_culled,
        );
        e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, array]);
    });
    1
}

/// `fn_00b5ddf0`'s inlined `BSSimpleArray::RemoveAt` on the array at `array` (buffer +4, size
/// +8, capacity +0xc) for `index`, with `size` the size read before: the tail moves down
/// (`00cb9930`: destination, source, count) unless the array is sparse enough to shrink
/// (capacity halved, or less 0x400 above 0x800), in which case a new buffer (virtual slot +4)
/// takes both parts and the old one is freed (slot +8). The size goes down by one.
fn simple_array_remove_at(e: &mut Engine, array: u32, index: u32, size: u32) {
    let capacity = e.mem.u32(array.wrapping_add(0xc));
    let shift = if capacity <= 0x400 {
        size > capacity >> 2
    } else {
        size > capacity.wrapping_sub(0x800)
    };
    if shift {
        let buffer = e.mem.u32(array.wrapping_add(4));
        let at = buffer.wrapping_add(4 * index);
        e.call(
            ARRAY_MOVE_00CB9930,
            &args![
                array,
                at,
                at.wrapping_add(4),
                size.wrapping_sub(index).wrapping_sub(1)
            ],
        );
    } else {
        let new_capacity = if capacity > 0x800 {
            capacity.wrapping_sub(0x400)
        } else {
            capacity >> 1
        };
        let fresh = e.vcall(array, 4, &args![new_capacity]).u32();
        let buffer = e.mem.u32(array.wrapping_add(4));
        e.call(ARRAY_MOVE_00CB9930, &args![array, fresh, buffer, index]);
        let count = e.mem.u32(array.wrapping_add(8)).wrapping_sub(1);
        let buffer = e.mem.u32(array.wrapping_add(4));
        e.call(
            ARRAY_MOVE_00CB9930,
            &args![
                array,
                fresh.wrapping_add(4 * index),
                buffer.wrapping_add(4 * index).wrapping_add(4),
                count
            ],
        );
        let old = e.mem.u32(array.wrapping_add(4));
        e.vcall(array, 8, &args![old]);
        e.mem.set_u32(array.wrapping_add(4), fresh);
        e.mem.set_u32(array.wrapping_add(0xc), new_capacity);
    }
    let size = e.mem.u32(array.wrapping_add(8)).wrapping_sub(1);
    e.mem.set_u32(array.wrapping_add(8), size);
}

/// The element move `00cb9930` (`thiscall` on the array, `RET 0xc`).
const ARRAY_MOVE_00CB9930: u32 = 0x00cb_9930;

// Translated from 00b5ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the node's portal graph. `thiscall`, `RET 4` (the graph).
/// Nothing but the store when it is the current graph. Otherwise, under both queue locks with
/// `bAllowLightRemoveQueues` cleared, the five queues are emptied (`00e74d40`); for each room
/// of the old graph (list at +0x20): the room is taken out of the +0x210 array of each of its
/// lights (+0xe0, count +0xe4; `00b5ade0`), the room's light array (+0xdc) is freed, and for
/// each portal of the room (list at +0xb4) the portal is removed from the +0x220 array of each
/// light of the portal's +0x104 object (array at +0xb0, count +0xb8; the inlined `RemoveAt`)
/// and that object's light array is freed. Then the flag is set again, the locks released,
/// `00b5d180` runs and the graph is stored.
pub fn fn_00b5ddf0(e: &mut Engine, this: Ptr<ShadowSceneNode>, graph: u32) {
    if graph == e.get(this, ShadowSceneNode::pBSPortalGraph) {
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        return;
    }
    e.call(SPIN_LOCK_LOCK, &args![QUEUE_LOCK, 0u32]);
    e.call(SPIN_LOCK_LOCK, &args![OBJECT_QUEUE_LOCK, 0u32]);
    e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 0);
    for list in [
        LIGHT_QUEUE_ADD,
        LIGHT_QUEUE_REMOVE,
        LIGHT_QUEUE_UPDATE,
        OBJECT_QUEUE_UPDATE,
        OBJECT_QUEUE_UPDATE_ACTIVE_ONLY,
    ] {
        e.call(LIST_REMOVE_ALL, &args![this.addr().wrapping_add(list)]);
    }
    let old = e.get(this, ShadowSceneNode::pBSPortalGraph);
    let mut rooms = if old != 0 {
        e.mem.u32(old.wrapping_add(0x20))
    } else {
        0
    };
    while rooms != 0 {
        let room = item_element(e, rooms);
        rooms = item_next(e, rooms);
        if room == 0 {
            continue;
        }
        let mut index = 0u32;
        while index < e.mem.u32(room.wrapping_add(0xe4)) {
            let lights = e.mem.u32(room.wrapping_add(0xe0));
            let light = e.mem.u32(lights.wrapping_add(4 * index));
            let rooms_of_light = light.wrapping_add(0x210);
            if let Some(found) = array_index_of(e, rooms_of_light, room) {
                if found != u32::MAX {
                    e.call(ARRAY_REMOVE_00B5ADE0, &args![rooms_of_light, found, 1u32]);
                }
            }
            index += 1;
        }
        clear_simple_array(e, room.wrapping_add(0xdc));
        let mut portals = e.mem.u32(room.wrapping_add(0xb4));
        while portals != 0 {
            let portal = item_element(e, portals);
            portals = item_next(e, portals);
            if portal == 0 {
                continue;
            }
            let total = e
                .mem
                .u32(e.mem.u32(portal.wrapping_add(0x104)).wrapping_add(0xb8))
                as i32;
            let mut index = 0i32;
            while index < total {
                let target = e.mem.u32(portal.wrapping_add(0x104));
                let lights = e.mem.u32(target.wrapping_add(0xb4));
                let light = e.mem.u32(lights.wrapping_add(4 * index as u32));
                let portals_of_light = light.wrapping_add(0x220);
                let size = e.mem.u32(portals_of_light.wrapping_add(8));
                let mut found = None;
                let mut slot = 0u32;
                while slot < size {
                    let buffer = e.mem.u32(portals_of_light.wrapping_add(4));
                    if buffer != 0 && e.mem.u32(buffer.wrapping_add(4 * slot)) == portal {
                        found = Some(slot);
                        break;
                    }
                    slot += 1;
                }
                if let Some(slot) = found {
                    if slot != u32::MAX {
                        simple_array_remove_at(e, portals_of_light, slot, size);
                    }
                }
                index += 1;
            }
            let target = e.mem.u32(portal.wrapping_add(0x104));
            clear_simple_array(e, target.wrapping_add(0xb0));
        }
    }
    e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 1);
    unlock_queue(e);
    let count = e.global::<u32>(OBJECT_QUEUE_LOCK_COUNT).wrapping_sub(1);
    e.set_global(OBJECT_QUEUE_LOCK_COUNT, count);
    if count == 0 {
        e.set_global(OBJECT_QUEUE_LOCK, 0u32);
    }
    fn_00b5d180(e, this);
    e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
}

/// The destructor's callees: the scene graph slot release `00b4f2f0` (`cdecl`: the index and
/// 0), the renderer's `00b6da10` (`RET 4`) for the window reflection map,
/// `BSCompoundFrustum::~BSCompoundFrustum` (`00c47770`) and `NiNode::~NiNode` (`00a5e930`).
const SCENE_GRAPH_RELEASE_00B4F2F0: u32 = 0x00b4_f2f0;
const RENDERER_00B6DA10: u32 = 0x00b6_da10;
const COMPOUND_FRUSTUM_DESTRUCT: u32 = 0x00c4_7770;
const NI_NODE_DESTRUCT: u32 = 0x00a5_e930;

/// An `NiPointer` member's destructor: the object released when not null (the member keeps
/// its value).
fn destroy_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        release(e, object);
    }
}

/// An `NiPointer` member released and cleared when not null.
fn clear_member(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00b5e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ShadowSceneNode` destructor (unnamed in the engine map; by its body). `thiscall`, no
/// stack words. Sets the vtable; frees its scene graph slot when the node is the one at
/// `011f91c8 + 4 * cSceneGraphIndex` (`00b4f2f0`); clears the accumulator's scene node (+0x194)
/// when it is this node; runs `00b5d180`; releases and clears the fence objects; deletes the
/// sun light (slot 0 with 1); hands the window reflection map to the renderer (`00b6da10`)
/// and releases and clears it; empties the light queues; clears `pBSPortalGraph`; releases and
/// clears the cube map camera; destroys the compound frustum; then the member destructors in
/// reverse order (`spFog`, the cube map camera and reflection map, the queues and lists, the
/// fence objects) and `NiNode::~NiNode`.
// Translation incomplete: 00b5e5a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xd for the members) is not translated.
pub fn fn_00b5e5a0(e: &mut Engine, this: Ptr<ShadowSceneNode>) {
    let base = this.addr();
    e.mem.set_u32(base, SHADOW_SCENE_NODE_VTABLE);
    let index = u32::from(e.get(this, ShadowSceneNode::cSceneGraphIndex));
    if e.global::<u32>(FALLBACK_SCENE_NODE_011F91C8 + 4 * index) == base {
        e.call(SCENE_GRAPH_RELEASE_00B4F2F0, &args![index, 0u32]);
    }
    if e.call(SHADER_MANAGER_GET_ACCUMULATOR, &args![]).u32() != 0 {
        let accumulator = e.call(SHADER_MANAGER_GET_ACCUMULATOR, &args![]).u32();
        if e.mem.u32(accumulator.wrapping_add(ACCUMULATOR_SCENE_NODE)) == base {
            let accumulator = e.call(SHADER_MANAGER_GET_ACCUMULATOR, &args![]).u32();
            e.mem
                .set_u32(accumulator.wrapping_add(ACCUMULATOR_SCENE_NODE), 0);
        }
    }
    fn_00b5d180(e, this);
    clear_member(e, base + 0xd8);
    clear_member(e, base + 0xdc);
    let sun = e.get(this, ShadowSceneNode::pSunLight);
    if sun != 0 {
        e.vcall(sun, SLOT_DELETING_DESTRUCTOR, &args![1u32]);
    }
    let map = e.get(this, ShadowSceneNode::pWindowReflectionEnvMap);
    if map != 0 {
        let renderer = e.global::<u32>(RENDERER_011F91A8);
        e.call(RENDERER_00B6DA10, &args![renderer, map]);
    }
    clear_member(e, base + 0x124);
    for list in [LIGHT_QUEUE_ADD, LIGHT_QUEUE_REMOVE, LIGHT_QUEUE_UPDATE] {
        e.call(LIST_REMOVE_ALL, &args![base + list]);
    }
    e.set(this, ShadowSceneNode::pBSPortalGraph, 0);
    clear_member(e, base + 0x128);
    e.call(COMPOUND_FRUSTUM_DESTRUCT, &args![base + 0x138]);
    destroy_pointer(e, base + 0x134);
    destroy_pointer(e, base + 0x128);
    destroy_pointer(e, base + 0x124);
    for list in [
        OBJECT_QUEUE_UPDATE_ACTIVE_ONLY,
        OBJECT_QUEUE_UPDATE,
        LIGHT_QUEUE_UPDATE,
        LIGHT_QUEUE_REMOVE,
        LIGHT_QUEUE_ADD,
    ] {
        e.call(LIST_REMOVE_ALL, &args![base + list]);
    }
    destroy_pointer(e, base + 0xdc);
    destroy_pointer(e, base + 0xd8);
    e.call(LIST_REMOVE_ALL, &args![base + SHADOW_LIGHT_LIST]);
    e.call(LIST_REMOVE_ALL, &args![base + LIGHT_LIST]);
    e.call(NI_NODE_DESTRUCT, &args![base]);
}

/// `PreOnVisible`'s callees and globals: `00c4f360` (`thiscall` on the culler), the script
/// test `005bf7b0` (no arguments, `u8`), `ShadowSceneLight::TestFrustumCull` (`00b9e970`,
/// `RET 4`), `NiNode::OnVisible` (`00a5dbe0`, `RET 4`); the globals written: the scene graph
/// index byte (`011f91c4`), the camera (`011f917c`), the camera's position and two rows of
/// its rotation (`011fa2a0` to `011fa2cc`, each with 0.0 last), the fog's values
/// (`011fa280` to `011fa29c`) and the blend `011f91d8`/`011ad868`; read: the switch word
/// `011f91c0`, the byte `011f91a7`, and the three `float`s of the final blend (`01200504`,
/// `01200500`, `011ad824`, into `01200558`).
const CULLER_00C4F360: u32 = 0x00c4_f360;
const SCRIPT_TEST_005BF7B0: u32 = 0x005b_f7b0;
const LIGHT_TEST_FRUSTUM_CULL: u32 = 0x00b9_e970;
const NI_NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;
const SCENE_GRAPH_INDEX_011F91C4: u32 = 0x011f_91c4;
const CAMERA_011F917C: u32 = 0x011f_917c;
const CAMERA_VALUES_011FA2A0: u32 = 0x011f_a2a0;
const FOG_VALUES_011FA280: u32 = 0x011f_a280;
const SWITCH_011F91C0: u32 = 0x011f_91c0;
const MODE_FLAG_011F91A7: u32 = 0x011f_91a7;
const SAVED_MODE_011AD868: u32 = 0x011a_d868;
const MODE_011F91D8: u32 = 0x011f_91d8;
const BLEND_TARGET_01200504: u32 = 0x0120_0504;
const BLEND_BASE_01200500: u32 = 0x0120_0500;
const BLEND_FACTOR_011AD824: u32 = 0x011a_d824;
const BLEND_RESULT_01200558: u32 = 0x0120_0558;

// Translated from 00b5e870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::PreOnVisible` (Xbox PDB). `thiscall`, `RET 4` (the culler). Runs the
/// culler's `00c4f360`. Unless `bDisableLightUpdate` is set, each light of `lLightList` with an
/// `NiLight` held elsewhere too (count not 1) is queued for update when dynamic and
/// frustum-tested (`TestFrustumCull`) unless it casts shadows and the script test `005bf7b0`
/// passes; a light whose `NiLight` is gone or held only here is removed: from the shadow
/// list (`00b9fcc0`) or the light list (`00b9efa0` at its item) when it belongs to another
/// portal graph, else with `RemoveQueuedLight`. Then the scene graph index and the camera
/// (culler + 0xc) are published; with the switch word at `011f91c0` clear, `NiNode::OnVisible`
/// runs; otherwise the blend mode words are swapped by the byte at `011f91a7`, the camera's
/// position and rotation rows and the fog's distances and colour are copied to their globals,
/// and `[01200558] = [01200500] + ([01200504] - [01200500]) * [011ad824]`.
// Translation incomplete: 00b5e870 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the temporaries) is not translated.
pub fn shadow_scene_node_pre_on_visible(e: &mut Engine, this: Ptr<ShadowSceneNode>, culler: u32) {
    e.call(CULLER_00C4F360, &args![culler]);
    let camera = e.mem.u32(culler.wrapping_add(0xc));
    if e.get(this, ShadowSceneNode::bDisableLightUpdate) == 0 {
        let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
        while item != 0 {
            let current = item;
            let light = item_element(e, item);
            item = item_next(e, item);
            if light == 0 {
                continue;
            }
            let ni_light = e.mem.u32(light.wrapping_add(LIGHT_SP_LIGHT));
            if ni_light != 0 && e.mem.u32(ni_light.wrapping_add(4)) != 1 {
                if e.mem.u8(light.wrapping_add(LIGHT_DYNAMIC)) != 0 {
                    fn_00b5d270(e, this, light);
                }
                if e.mem.u8(light.wrapping_add(LIGHT_CAST_SHADOW)) != 0
                    && e.call(SCRIPT_TEST_005BF7B0, &args![]).u8() != 0
                {
                    continue;
                }
                e.call(LIGHT_TEST_FRUSTUM_CULL, &args![light, culler]);
                continue;
            }
            e.with_stack(0xc, |e, frame| {
                let item_slot = frame.addr();
                let held = item_slot + 4;
                let out = item_slot + 8;
                e.mem.set_u32(item_slot, current);
                let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
                if e.mem.u32(light.wrapping_add(LIGHT_PORTAL_GRAPH)) != graph {
                    if e.mem.u8(light.wrapping_add(LIGHT_CAST_SHADOW)) != 0 {
                        e.mem.set_u32(item_slot, light);
                        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light.wrapping_add(4)]);
                        e.call(
                            LIST_REMOVE,
                            &args![this.addr().wrapping_add(SHADOW_LIGHT_LIST), out, item_slot],
                        );
                        let removed = e.mem.u32(out);
                        if removed != 0 {
                            release(e, removed);
                        }
                        let held_light = e.mem.u32(item_slot);
                        if held_light != 0 {
                            release(e, held_light);
                        }
                    } else {
                        e.call(
                            LIST_REMOVE_POS,
                            &args![this.addr().wrapping_add(LIGHT_LIST), out, item_slot],
                        );
                        let removed = e.mem.u32(out);
                        if removed != 0 {
                            release(e, removed);
                        }
                    }
                } else {
                    e.mem.set_u32(held, light);
                    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light.wrapping_add(4)]);
                    shadow_scene_node_remove_queued_light(e, this, Ptr::new(held), item_slot);
                    let held_light = e.mem.u32(held);
                    if held_light != 0 {
                        release(e, held_light);
                    }
                }
            });
        }
    }
    let index = e.get(this, ShadowSceneNode::cSceneGraphIndex);
    e.set_global(SCENE_GRAPH_INDEX_011F91C4, index);
    e.set_global(CAMERA_011F917C, camera);
    if e.global::<u32>(SWITCH_011F91C0) == 0 {
        e.call(NI_NODE_ON_VISIBLE, &args![this, culler]);
        return;
    }
    if e.global::<u8>(MODE_FLAG_011F91A7) == 1 {
        if e.global::<u32>(SAVED_MODE_011AD868) == u32::MAX {
            let mode = e.global::<u32>(MODE_011F91D8);
            e.set_global(SAVED_MODE_011AD868, mode);
        }
        e.set_global(MODE_011F91D8, 4u32);
    } else {
        let saved = e.global::<u32>(SAVED_MODE_011AD868);
        if saved != u32::MAX {
            e.set_global(MODE_011F91D8, saved);
            e.set_global(SAVED_MODE_011AD868, u32::MAX);
        }
    }
    let zero = x87_store32(0.0);
    // Position (+0x8c), then the rotation rows at +0x68/+0x74/+0x80 and +0x70/+0x7c/+0x88,
    // each value through two `float` copies, each row ending with 0.0.
    let rows: [[u32; 3]; 3] = [[0x8c, 0x90, 0x94], [0x68, 0x74, 0x80], [0x70, 0x7c, 0x88]];
    for (row, offsets) in rows.iter().enumerate() {
        let target = CAMERA_VALUES_011FA2A0 + 0x10 * row as u32;
        for (column, offset) in offsets.iter().enumerate() {
            let value = x87_quiet_bits(e.mem.u32(camera.wrapping_add(*offset)));
            e.set_global(target + 4 * column as u32, value);
        }
        e.set_global(target + 0xc, zero);
    }
    let fog = e.get(this, ShadowSceneNode::spFog);
    if fog != 0 {
        let far = x87_quiet_bits(e.mem.u32(fog.wrapping_add(0x30)));
        let span = x87_store32(x87_sub(
            x87_load32(far),
            x87_m32(e.mem.u32(fog.wrapping_add(0x2c))),
        ));
        let value_60 = x87_quiet_bits(e.mem.u32(fog.wrapping_add(0x60)));
        let colour = [
            x87_quiet_bits(e.mem.u32(fog.wrapping_add(0x20))),
            x87_quiet_bits(e.mem.u32(fog.wrapping_add(0x24))),
            x87_quiet_bits(e.mem.u32(fog.wrapping_add(0x28))),
        ];
        let values = [
            far, span, value_60, zero, colour[0], colour[1], colour[2], zero,
        ];
        for (i, value) in values.iter().enumerate() {
            e.set_global(FOG_VALUES_011FA280 + 4 * i as u32, *value);
        }
    }
    let target = x87_load32(e.global::<u32>(BLEND_TARGET_01200504));
    let base = x87_load32(e.global::<u32>(BLEND_BASE_01200500));
    let span = x87_sub(target, base);
    let factor = x87_load32(e.global::<u32>(BLEND_FACTOR_011AD824));
    let result = x87_add(x87_mul(span, factor), base);
    e.set_global(BLEND_RESULT_01200558, x87_store32(result));
}

// Translated from 00b5f9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::OnVisible` (Xbox PDB). `thiscall`, `RET 4` (the culler). Runs
/// `PreOnVisible`, holds the shader manager's accumulator (twice when its count was 1 after
/// the first hold). Without a portal graph, `NiNode::OnVisible`. With one: each
/// always-rendered object's flag 1 is saved in a scrap-heap byte array and set, as is the
/// graph's +0x4c object's; then, for a visible unbound space or a culler in mode 1 or 3,
/// `NiNode::OnVisible` and the shared portals of the graph's room list (+0x20 for those
/// modes, else +0x30; each room's portals at +0xb4, the graph's at +0x14 for a null room) are
/// added (`BSCullingProcess::AddShared`, then their +0x104 object's `OnVisible`, slot +0xd4)
/// with a local compound frustum; otherwise each room (list at +0x30) is visited (slot +0xd4)
/// and its portals added with `BuildSharedCompoundFrustum`, and the children from index 4 are
/// culled (`NiAVObject::Cull`). Finally the always-rendered objects that had flag 1 clear get
/// it cleared again and are culled, the +0x4c object's flag is cleared, and the byte array is
/// freed. The accumulator is released once.
// Translation incomplete: 00b5f9b0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the accumulator and the compound frustum) is not translated.
pub fn shadow_scene_node_on_visible(e: &mut Engine, this: Ptr<ShadowSceneNode>, culler: u32) {
    shadow_scene_node_pre_on_visible(e, this, culler);
    let accumulator = e.call(SHADER_MANAGER_GET_ACCUMULATOR, &args![]).u32();
    if accumulator != 0 {
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![accumulator.wrapping_add(4)],
        );
    }
    if e.mem.u32(accumulator.wrapping_add(4)) == 1 {
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![accumulator.wrapping_add(4)],
        );
    }
    if e.get(this, ShadowSceneNode::pBSPortalGraph) == 0 {
        e.call(NI_NODE_ON_VISIBLE, &args![this, culler]);
    } else {
        on_visible_with_graph(e, this, culler);
    }
    release(e, accumulator);
}

/// The portal-graph part of `OnVisible`.
fn on_visible_with_graph(e: &mut Engine, this: Ptr<ShadowSceneNode>, culler: u32) {
    let graph_count = |e: &mut Engine| {
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        e.call(PORTAL_GRAPH_ALWAYS_RENDER_COUNT, &args![graph])
            .u32()
    };
    let graph_at = |e: &mut Engine, i: u32| {
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        e.call(PORTAL_GRAPH_ALWAYS_RENDER_AT, &args![graph, i])
            .u32()
    };
    let heap = e.call(GET_THREAD_SCRAP_HEAP, &args![MEMORY_MANAGER]).u32();
    let mut saved = 0u32;
    if graph_count(e) != 0 {
        let alignment = e.global::<u32>(SCRAP_ALIGNMENT_010A2720);
        let size = graph_count(e);
        saved = e
            .call(SCRAP_HEAP_ALLOCATE, &args![heap, size, alignment])
            .u32();
        let size = graph_count(e);
        e.call(MEMSET, &args![saved, 0u32, size]);
    }
    let mut index = 0u32;
    while index < graph_count(e) {
        let object = graph_at(e, index);
        if object != 0 {
            let flags = e.mem.u32(object.wrapping_add(AV_FLAGS));
            e.mem.set_u8(saved.wrapping_add(index), (flags & 1) as u8);
            e.mem.set_u32(object.wrapping_add(AV_FLAGS), flags | 1);
        }
        index += 1;
    }
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    let extra = e.mem.u32(graph.wrapping_add(0x4c));
    if extra != 0 {
        let flags = e.mem.u32(extra.wrapping_add(AV_FLAGS));
        e.mem.set_u32(extra.wrapping_add(AV_FLAGS), flags | 1);
    }
    let mode = e.mem.u32(culler.wrapping_add(0x90));
    if e.get(this, ShadowSceneNode::bVisibleUnboundSpace) != 0 || mode == 1 || mode == 3 {
        e.call(NI_NODE_ON_VISIBLE, &args![this, culler]);
        let mode = e.mem.u32(culler.wrapping_add(0x90));
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        let list = if mode == 1 || mode == 3 {
            graph.wrapping_add(0x20)
        } else {
            graph.wrapping_add(0x30)
        };
        let mut rooms = e.mem.u32(list);
        e.with_stack(0xa4, |e, frustum| {
            e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![frustum]);
            while rooms != 0 {
                let room = item_element(e, rooms);
                rooms = item_next(e, rooms);
                let portals = if room != 0 {
                    room.wrapping_add(0xb4)
                } else {
                    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
                    graph.wrapping_add(0x14)
                };
                if e.mem.u32(portals.wrapping_add(8)) != 0 {
                    let mut item = e.mem.u32(portals);
                    while item != 0 {
                        let portal = item_element(e, item);
                        item = item_next(e, item);
                        if portal == 0 {
                            continue;
                        }
                        let target = e.mem.u32(portal.wrapping_add(0x104));
                        if e.call(CULLING_PROCESS_ADD_SHARED, &args![culler, target])
                            .u8()
                            != 0
                        {
                            let target = e.mem.u32(portal.wrapping_add(0x104));
                            e.vcall(target, 0xd4, &args![culler]);
                        }
                    }
                }
                e.mem.set_u32(culler.wrapping_add(0xc0), 0);
            }
            e.call(COMPOUND_FRUSTUM_DESTRUCT, &args![frustum]);
        });
    } else {
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        let mut rooms = e.mem.u32(graph.wrapping_add(0x30));
        while rooms != 0 {
            let room = item_element(e, rooms);
            rooms = item_next(e, rooms);
            if room == 0 {
                continue;
            }
            e.vcall(room, 0xd4, &args![culler]);
            if e.mem.u32(room.wrapping_add(0xbc)) != 0 {
                let mut item = e.mem.u32(room.wrapping_add(0xb4));
                while item != 0 {
                    let portal = item_element(e, item);
                    item = item_next(e, item);
                    if portal == 0 {
                        continue;
                    }
                    let target = e.mem.u32(portal.wrapping_add(0x104));
                    if e.call(CULLING_PROCESS_ADD_SHARED, &args![culler, target])
                        .u8()
                        != 0
                    {
                        let camera = e.mem.u32(culler.wrapping_add(0xc));
                        e.call(BUILD_SHARED_COMPOUND_FRUSTUM, &args![this, camera, portal]);
                        let target = e.mem.u32(portal.wrapping_add(0x104));
                        e.vcall(target, 0xd4, &args![culler]);
                    }
                }
            }
            e.mem.set_u32(culler.wrapping_add(0xc0), 0);
        }
        let mut index = 4u32;
        while index < u32::from(e.mem.u16(this.addr().wrapping_add(NODE_CHILDREN_SIZE))) {
            let children = e.mem.u32(this.addr().wrapping_add(NODE_CHILDREN_BASE));
            let child = e.mem.u32(children.wrapping_add(4 * index));
            if child != 0 {
                e.call(NI_AV_OBJECT_CULL, &args![child, culler]);
            }
            index += 1;
        }
    }
    let mut index = 0u32;
    while index < graph_count(e) {
        let object = graph_at(e, index);
        if object != 0 && e.mem.u8(saved.wrapping_add(index)) == 0 {
            let flags = e.mem.u32(object.wrapping_add(AV_FLAGS));
            e.mem
                .set_u32(object.wrapping_add(AV_FLAGS), flags & 0xffff_fffe);
            e.call(NI_AV_OBJECT_CULL, &args![object, culler]);
        }
        index += 1;
    }
    let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
    let extra = e.mem.u32(graph.wrapping_add(0x4c));
    if extra != 0 {
        let flags = e.mem.u32(extra.wrapping_add(AV_FLAGS));
        e.mem
            .set_u32(extra.wrapping_add(AV_FLAGS), flags & 0xffff_fffe);
    }
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, saved]);
}

/// `BSCullingProcess::AddShared` (`00c4f320`, `RET 4`, `u8`), `BuildSharedCompoundFrustum`
/// (`00b5ba80`, `RET 8`), `NiAVObject::Cull` (`00a59e00`, `RET 4`) and the CRT `_memset`
/// (`00ec61c0`, `cdecl`).
const CULLING_PROCESS_ADD_SHARED: u32 = 0x00c4_f320;
const BUILD_SHARED_COMPOUND_FRUSTUM: u32 = 0x00b5_ba80;
const NI_AV_OBJECT_CULL: u32 = 0x00a5_9e00;
const MEMSET: u32 = 0x00ec_61c0;

/// `fn_00b5f170`'s callees: `BSCompoundFrustum` methods (`thiscall`): set from a camera
/// (`00c47980`, `RET 4`), `00c47a00`, `00c47ae0`, `00c48b90`, `00c49850` (no stack words),
/// add an occluder (`00c47d20`, `RET 4`), add a portal (`00c47b50`, `RET 8`: the portal and 0),
/// merge another (`00c48800`, `RET 4`), build from a frustum and a portal (`00c48db0`,
/// `RET 8`), test an object (`00c49560`, `RET 4`, `u8`); the occlusion plane test `00c344c0`
/// (`RET 8`: a word and the point; `u8`); the room's frustum setter `0045c4e0` (`RET 4`); the
/// portal's `00b5aa00`; the memory manager's allocate `00aa3e40` (`RET 4`); the list item free
/// list (`011c5f58`), its lock (`011c5f80`, count `011c5f84`), its refill `0043a080` and item
/// allocation `0043a010`; the list destructor `00b71300`.
const FRUSTUM_SET_CAMERA: u32 = 0x00c4_7980;
const FRUSTUM_00C47A00: u32 = 0x00c4_7a00;
const FRUSTUM_00C47AE0: u32 = 0x00c4_7ae0;
const FRUSTUM_00C48B90: u32 = 0x00c4_8b90;
const FRUSTUM_00C49850: u32 = 0x00c4_9850;
const FRUSTUM_ADD_OCCLUDER: u32 = 0x00c4_7d20;
const FRUSTUM_ADD_PORTAL: u32 = 0x00c4_7b50;
const FRUSTUM_MERGE: u32 = 0x00c4_8800;
const FRUSTUM_BUILD: u32 = 0x00c4_8db0;
const FRUSTUM_TEST: u32 = 0x00c4_9560;
const OCCLUSION_PLANE_TEST_00C344C0: u32 = 0x00c3_44c0;
const ROOM_SET_FRUSTUM_0045C4E0: u32 = 0x0045_c4e0;
const PORTAL_00B5AA00: u32 = 0x00b5_aa00;
const ITEM_FREE_LIST: u32 = 0x011c_5f58;
const ITEM_LOCK: u32 = 0x011c_5f80;
const ITEM_LOCK_COUNT: u32 = 0x011c_5f84;
const ITEM_REFILL_0043A080: u32 = 0x0043_a080;
const ITEM_NEW_0043A010: u32 = 0x0043_a010;
const LIST_DESTRUCT_00B71300: u32 = 0x00b7_1300;

/// The item lock's release (count `011c5f84`, owner `011c5f80`).
fn unlock_items(e: &mut Engine) {
    let count = e.global::<u32>(ITEM_LOCK_COUNT).wrapping_sub(1);
    e.set_global(ITEM_LOCK_COUNT, count);
    if count == 0 {
        e.set_global(ITEM_LOCK, 0u32);
    }
}

/// The occluder test both loops of `fn_00b5f170` make: state 1, or state 0 and
/// `00c344c0` (the word and the point) passing.
fn occluder_active(e: &mut Engine, occluder: u32, word: u32, point: u32) -> bool {
    match e.mem.u32(occluder.wrapping_add(0x40)) {
        1 => true,
        0 => {
            e.call(OCCLUSION_PLANE_TEST_00C344C0, &args![occluder, word, point])
                .u8()
                != 0
        }
        _ => false,
    }
}

/// Adds the active occluders of the list at `list` to `frustum`, bracketed by
/// `00c47a00`/`00c47ae0` when there is one.
fn add_active_occluders(e: &mut Engine, frustum: u32, list: u32, word: u32, point: u32) {
    let mut item = e.mem.u32(list);
    let mut started = false;
    while item != 0 {
        let occluder = item_element(e, item);
        item = item_next(e, item);
        if occluder == 0 || !occluder_active(e, occluder, word, point) {
            continue;
        }
        if !started {
            e.call(FRUSTUM_00C47A00, &args![frustum]);
            started = true;
        }
        e.call(FRUSTUM_ADD_OCCLUDER, &args![frustum, occluder]);
    }
    if started {
        e.call(FRUSTUM_00C47AE0, &args![frustum]);
    }
}

/// A frustum with planes: a non-null one whose +0x28 is set.
fn has_planes(e: &Engine, frustum: u32) -> bool {
    frustum != 0 && e.mem.u32(frustum.wrapping_add(0x28)) != 0
}

/// The merge of `frustum` cut by `portal` into `target` through a local compound frustum
/// (`00c47660`, `00c48db0`, `00c48800`, `00c47770`).
fn merge_cut(e: &mut Engine, target: u32, frustum: u32, portal: u32) {
    e.with_stack(0xa4, |e, local| {
        e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![local]);
        e.call(FRUSTUM_BUILD, &args![local, frustum, portal]);
        e.call(FRUSTUM_MERGE, &args![target, local]);
        e.call(COMPOUND_FRUSTUM_DESTRUCT, &args![local]);
    });
}

/// Adds each occluder of the list at `list` that `test` passes (`00c49560`; every one when
/// `test` is null and `unchecked`) to `frustum`.
fn add_tested(e: &mut Engine, frustum: u32, list: u32, test: u32, unchecked: bool) {
    let mut item = e.mem.u32(list);
    while item != 0 {
        let occluder = item_element(e, item);
        item = item_next(e, item);
        if occluder == 0 {
            continue;
        }
        if unchecked && test == 0 || e.call(FRUSTUM_TEST, &args![test, occluder]).u8() != 0 {
            e.call(FRUSTUM_ADD_OCCLUDER, &args![frustum, occluder]);
        }
    }
}

/// `fn_00b5f170`'s test before marking a frustum's flag +0xa0: no frustum, its own flag set,
/// or a frustum without planes whose portal's `00b5aa00` answers 0xf.
fn marks_flag(e: &mut Engine, frustum: u32, portal: u32) -> bool {
    if frustum == 0 || e.mem.u8(frustum.wrapping_add(0xa0)) != 0 {
        return true;
    }
    if e.mem.u32(frustum.wrapping_add(0x28)) != 0 {
        return false;
    }
    e.call(PORTAL_00B5AA00, &args![portal]).u32() == 0xf
}

// Translated from 00b5f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: walks the portal graph from a room (or the unbound space) and
/// builds each visible room's compound frustum. `thiscall`, `RET 0x10`: the start room, the
/// camera, a word for the occlusion test and a flag that clears `bVisibleUnboundSpace`. The
/// node's frustum is set from the camera; the start (with no room: the unbound space, whose
/// active occluders of the graph (+8, count +0x10) go into the node's frustum; with one: a new
/// frustum from the camera with the room's active occluders (+0xc0), given to the room with
/// `0045c4e0`) is queued in a local `NiTPointerList` (items from the shared free list at
/// `011c5f58`). Each room taken from the queue is marked visible (`00b5c6e0` on the graph; its
/// shape and its +0xd0 objects' shapes get state 1) and each of its portals (or the graph's, for
/// the unbound space) not yet marked and inside its frustum (`00c49560`) leads to the room on
/// its other side (+0xfc / +0x100), which is queued and whose frustum is built from the
/// camera, the portal, the current frustum (cut with `00c48db0` when it has planes; dropped
/// beyond 0x19) and the occluders; a portal to the unbound space builds the node's own
/// frustum the same way. Each portal handled gets state 1. Both local lists are destroyed.
// Translation incomplete: 00b5f170 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the lists, new frustums and local frustums) is not translated.
pub fn fn_00b5f170(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    start: u32,
    camera: u32,
    word: u32,
    clear_unbound: u32,
) {
    let base = this.addr();
    let node_frustum = base.wrapping_add(0x138);
    e.with_stack(0x24, |e, frame| {
        let queue = frame.addr();
        let other_list = queue + 0xc;
        let point = queue + 0x18;
        if clear_unbound & 0xff != 0 {
            e.set(this, ShadowSceneNode::bVisibleUnboundSpace, 0);
        }
        e.call(FRUSTUM_SET_CAMERA, &args![node_frustum, camera]);
        for word_index in 0..3u32 {
            let value = e
                .mem
                .u32(camera.wrapping_add(AV_WORLD_TRANSLATE + 4 * word_index));
            e.mem.set_u32(point + 4 * word_index, value);
        }
        let first_item;
        if start == 0 {
            let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
            e.set(this, ShadowSceneNode::bVisibleUnboundSpace, 1);
            if e.mem.u32(graph.wrapping_add(0x10)) != 0 {
                add_active_occluders(e, node_frustum, graph.wrapping_add(8), word, point);
            }
            let item = e.call(ITEM_NEW_0043A010, &args![queue]).u32();
            e.mem.set_u32(item + 8, 0);
            e.mem.set_u32(item, 0);
            e.mem.set_u32(item + 4, 0);
            first_item = item;
        } else {
            let memory = e
                .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xa4u32])
                .u32();
            let frustum = if memory != 0 {
                e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![memory]).u32()
            } else {
                0
            };
            e.call(FRUSTUM_SET_CAMERA, &args![frustum, camera]);
            add_active_occluders(e, frustum, start.wrapping_add(0xc0), word, point);
            e.call(ROOM_SET_FRUSTUM_0045C4E0, &args![start, frustum]);
            let item = e.call(ITEM_NEW_0043A010, &args![queue]).u32();
            e.mem.set_u32(item + 8, start);
            e.mem.set_u32(item, 0);
            e.mem.set_u32(item + 4, 0);
            first_item = item;
        }
        e.mem.set_u32(queue + 8, 1);
        e.mem.set_u32(queue + 4, first_item);
        let mut item = first_item;
        loop {
            // Take the head item.
            let next = e.mem.u32(item);
            e.mem.set_u32(queue, next);
            if next != 0 {
                e.mem.set_u32(next + 4, 0);
            } else {
                e.mem.set_u32(queue + 4, 0);
            }
            let room = e.mem.u32(item + 8);
            e.mem.set_u32(item + 8, 0);
            e.call(SPIN_LOCK_LOCK, &args![ITEM_LOCK, 0u32]);
            e.mem.set_u32(item + 4, 0);
            let free = e.global::<u32>(ITEM_FREE_LIST);
            e.mem.set_u32(item, free);
            e.set_global(ITEM_FREE_LIST, item);
            unlock_items(e);
            let count = e.mem.u32(queue + 8).wrapping_sub(1);
            e.mem.set_u32(queue + 8, count);
            let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
            fn_00b5c6e0(e, Ptr::new(graph), room);
            let (frustum, portals) = if room != 0 {
                let shape = e
                    .mem
                    .u32(e.mem.u32(room.wrapping_add(0xac)).wrapping_add(0xc));
                e.mem.set_u32(shape.wrapping_add(8), 1);
                let mut object = e.mem.u32(room.wrapping_add(0xd0));
                while object != 0 {
                    let element = item_element(e, object);
                    object = item_next(e, object);
                    if element != 0 {
                        let shape = e.mem.u32(element.wrapping_add(0xc));
                        e.mem.set_u32(shape.wrapping_add(8), 1);
                    }
                }
                (e.mem.u32(room.wrapping_add(0xcc)), room.wrapping_add(0xb4))
            } else {
                let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
                e.set(this, ShadowSceneNode::bVisibleUnboundSpace, 1);
                (node_frustum, graph.wrapping_add(0x14))
            };
            e.call(FRUSTUM_00C49850, &args![frustum]);
            let mut portal_item = e.mem.u32(portals);
            while portal_item != 0 {
                let portal = item_element(e, portal_item);
                portal_item = item_next(e, portal_item);
                if portal == 0 || e.mem.u32(portal.wrapping_add(0x40)) != 0 {
                    continue;
                }
                if e.call(FRUSTUM_TEST, &args![frustum, portal]).u8() == 0 {
                    continue;
                }
                let mut other = e.mem.u32(portal.wrapping_add(0xfc));
                if other == room {
                    other = e.mem.u32(portal.wrapping_add(0x100));
                    if other == room {
                        continue;
                    }
                }
                if other != start {
                    visit_other_side(e, this, queue, frustum, portal, other, camera);
                }
                e.mem.set_u32(portal.wrapping_add(0x40), 1);
            }
            if e.mem.u32(queue + 8) == 0 {
                break;
            }
            item = e.mem.u32(queue);
        }
        e.call(LIST_DESTRUCT_00B71300, &args![queue]);
        e.call(LIST_DESTRUCT_00B71300, &args![other_list]);
    });
}

/// `fn_00b5f170`'s step through a portal to the room `other` (or the unbound space for a
/// null one): queues it and builds its frustum.
fn visit_other_side(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    queue: u32,
    frustum: u32,
    portal: u32,
    other: u32,
    camera: u32,
) {
    // A new item from the shared free list, appended to the queue.
    e.call(SPIN_LOCK_LOCK, &args![ITEM_LOCK, 0u32]);
    let mut item = e.global::<u32>(ITEM_FREE_LIST);
    if item == 0 {
        e.call(ITEM_REFILL_0043A080, &args![]);
        item = e.global::<u32>(ITEM_FREE_LIST);
    }
    let following = e.mem.u32(item);
    e.set_global(ITEM_FREE_LIST, following);
    e.mem.set_u32(item + 8, 0);
    e.mem.set_u32(item, 0);
    e.mem.set_u32(item + 4, 0);
    unlock_items(e);
    let tail = e.mem.u32(queue + 4);
    e.mem.set_u32(item + 8, other);
    e.mem.set_u32(item, 0);
    e.mem.set_u32(item + 4, tail);
    if tail != 0 {
        e.mem.set_u32(tail, item);
    } else {
        e.mem.set_u32(queue, item);
    }
    let count = e.mem.u32(queue + 8).wrapping_add(1);
    e.mem.set_u32(queue + 8, count);
    e.mem.set_u32(queue + 4, item);

    let base = this.addr();
    if other == 0 {
        let graph = e.get(this, ShadowSceneNode::pBSPortalGraph);
        let occluders = graph.wrapping_add(8);
        let node_frustum = base.wrapping_add(0x138);
        let involved = |e: &Engine| has_planes(e, frustum) || e.mem.u32(occluders + 8) != 0;
        if e.mem.u32(base.wrapping_add(0x160)) == 0 {
            if involved(e) {
                e.call(FRUSTUM_00C47A00, &args![node_frustum]);
                if has_planes(e, frustum) {
                    merge_cut(e, node_frustum, frustum, portal);
                }
            }
            e.call(FRUSTUM_ADD_PORTAL, &args![node_frustum, portal, 0u32]);
            add_tested(e, node_frustum, occluders, frustum, false);
            if involved(e) {
                e.call(FRUSTUM_00C47AE0, &args![node_frustum]);
            }
            // Here without the test of the frustum's own flag: no frustum, or one without
            // planes whose portal's `00b5aa00` answers 0xf.
            let marks = frustum == 0
                || (e.mem.u32(frustum.wrapping_add(0x28)) == 0
                    && e.call(PORTAL_00B5AA00, &args![portal]).u32() == 0xf);
            if marks {
                e.mem.set_u8(base.wrapping_add(0x1d8), 1);
            }
        } else {
            e.call(FRUSTUM_00C48B90, &args![node_frustum]);
            if involved(e) {
                e.call(FRUSTUM_00C47A00, &args![node_frustum]);
                if has_planes(e, frustum) {
                    e.call(FRUSTUM_MERGE, &args![node_frustum, frustum]);
                }
            }
            e.call(FRUSTUM_ADD_PORTAL, &args![node_frustum, portal, 0u32]);
            add_tested(e, node_frustum, occluders, frustum, false);
            e.call(FRUSTUM_00C47AE0, &args![node_frustum]);
            if involved(e) {
                e.call(FRUSTUM_00C47AE0, &args![node_frustum]);
            }
        }
        return;
    }
    let mut target = e.mem.u32(other.wrapping_add(0xcc));
    if target != 0 && e.mem.u32(target.wrapping_add(0x28)) != 0 {
        e.call(FRUSTUM_00C48B90, &args![target]);
        let involved =
            |e: &Engine| has_planes(e, frustum) || e.mem.u32(other.wrapping_add(0xc8)) != 0;
        if involved(e) {
            e.call(FRUSTUM_00C47A00, &args![target]);
            if has_planes(e, frustum) {
                e.call(FRUSTUM_MERGE, &args![target, frustum]);
            }
        }
        e.call(FRUSTUM_ADD_PORTAL, &args![target, portal, 0u32]);
        add_tested(e, target, other.wrapping_add(0xc0), frustum, false);
        e.call(FRUSTUM_00C47AE0, &args![target]);
        if involved(e) {
            e.call(FRUSTUM_00C47AE0, &args![target]);
        }
        return;
    }
    if target == 0 {
        let memory = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xa4u32])
            .u32();
        target = if memory != 0 {
            e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
    }
    e.call(FRUSTUM_SET_CAMERA, &args![target, camera]);
    let planes = e.mem.u32(frustum.wrapping_add(0x28)) as i32;
    let source = if planes > 0x19 { 0 } else { frustum };
    let cut = planes > 0x19 || planes == 0;
    if !cut || e.mem.u32(other.wrapping_add(0xc8)) != 0 {
        e.call(FRUSTUM_00C47A00, &args![target]);
        if has_planes(e, source) {
            merge_cut(e, target, source, portal);
        }
    }
    e.call(FRUSTUM_ADD_PORTAL, &args![target, portal, 0u32]);
    add_tested(e, target, other.wrapping_add(0xc0), source, true);
    if has_planes(e, source) || e.mem.u32(other.wrapping_add(0xc8)) != 0 {
        if e.mem.u32(target.wrapping_add(0x28)) == 1 {
            e.call(FRUSTUM_SET_CAMERA, &args![target, camera]);
        } else {
            e.call(FRUSTUM_00C47AE0, &args![target]);
        }
    }
    if marks_flag(e, source, portal) {
        e.mem.set_u8(target.wrapping_add(0xa0), 1);
    }
    if e.mem.u32(other.wrapping_add(0xcc)) == 0 {
        e.call(ROOM_SET_FRUSTUM_0045C4E0, &args![other, target]);
    }
}

/// The `BSSimpleArray`-shaped arrays of the functions below: vtable (+0; slot +4 allocates
/// a buffer of a number of elements, slot +8 frees one), buffer (+4), count (+8), capacity
/// (+0xc).
const ARRAY_BUFFER: u32 = 4;
const ARRAY_COUNT: u32 = 8;
const ARRAY_CAPACITY: u32 = 0xc;
/// `NiNode`'s children (`m_kChildren` +0x9c: buffer +0xa0, used size +0xa6) and the word at
/// +0xb0 that [`fn_00b5aa40`] uses as a child index.
const NODE_CHILD_INDEX: u32 = 0xb0;
/// `lLightList`'s count (`NiTPointerList::m_uiCount`, +0xbc).
const LIGHT_LIST_COUNT: u32 = 0xbc;
/// The light's 16-bit word at +0x110 that [`fn_00b5ac60`] compares with 0xff.
const LIGHT_WORD_110: u32 = 0x110;
/// The renderer's `00b6d130` (thiscall on the renderer object, no stack words).
const RENDERER_00B6D130: u32 = 0x00b6_d130;
/// `00b9bdf0` (`ShadowSceneLight`, thiscall, `RET 0x1c`: two points and a word; answers a
/// `float` in `ST0`).
const LIGHT_00B9BDF0: u32 = 0x00b9_bdf0;
/// `0.25` and `0.75` (`double`s at `010290b0` and `0101de30`).
const QUARTER_010290B0: u32 = 0x0102_90b0;
const THREE_QUARTERS_0101DE30: u32 = 0x0101_de30;
/// `BSCompoundFrustum` methods (engine map names; thiscall): `SetCamera` (`00c47980`,
/// `RET 4`), `AddCompoundFrustum` (`00c48800`, `RET 4`), `StartGroupUnion` (`00c47a70`),
/// `EndGroup` (`00c47ae0`), `RemoveEmptyGroups` (`00c49700`) and `PrethreadOpList`
/// (`00c49850`), the last four without stack words.
const FRUSTUM_START_GROUP_UNION: u32 = 0x00c4_7a70;
const FRUSTUM_END_GROUP: u32 = 0x00c4_7ae0;
const FRUSTUM_REMOVE_EMPTY_GROUPS: u32 = 0x00c4_9700;
const FRUSTUM_PRETHREAD_OP_LIST: u32 = 0x00c4_9850;
/// `NiNode::GetViewerStrings` (`00a5ea70`, thiscall: string array), the string array's
/// `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`), and `NiGetViewerString` (`00a7d330`),
/// `_ov6` (`00a7d490`: label, word) and `_ov7` (`00a7d4d0`: label, signed word).
const NI_NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_SHORT: u32 = 0x00a7_d490;
const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
/// `ShadowSceneNode::ms_RTTI` (`011f9e80`; its first word is the class name),
/// `"light count"` and `"shadow light count"`.
const SHADOW_SCENE_NODE_RTTI: u32 = 0x011f_9e80;
const TEXT_LIGHT_COUNT: u32 = 0x010a_dcd4;
const TEXT_SHADOW_LIGHT_COUNT: u32 = 0x010a_dcc0;

// Translated from 00b5a930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the search of an array (thiscall, `RET 0xc`: a value, the
/// start index and a `cdecl` compare function taking the element's address and the value)
/// from `start` up to the count (read each step): the first index whose compare answers a
/// non-zero byte, skipping the calls while the buffer is null; else `0xffffffff`.
pub fn fn_00b5a930(e: &mut Engine, this: Ptr, value: u32, start: u32, compare: u32) -> u32 {
    let array = this.addr();
    let mut index = start;
    while index < e.mem.u32(array.wrapping_add(ARRAY_COUNT)) {
        let buffer = e.mem.u32(array.wrapping_add(ARRAY_BUFFER));
        if buffer != 0 {
            let element = buffer.wrapping_add(index.wrapping_mul(4));
            if e.call(compare, &args![element, value]).u8() != 0 {
                return index;
            }
        }
        index = index.wrapping_add(1);
    }
    0xffff_ffff
}

// Translated from 00b5aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall on a node): the child at the node's index word
/// (+0xb0) when it is not negative and below the children's used size (signed compare),
/// else null.
pub fn fn_00b5aa40(e: &mut Engine, this: Ptr) -> u32 {
    let node = this.addr();
    let index = e.mem.i32(node.wrapping_add(NODE_CHILD_INDEX));
    let size = i32::from(e.mem.u16(node.wrapping_add(NODE_CHILDREN_SIZE)));
    if !(0..size).contains(&index) {
        return 0;
    }
    let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN_BASE));
    e.mem
        .u32(children.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 00b5ac60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall): the number of lights in `lLightList` whose 16-bit
/// word at +0x110 is not 0xff (null elements not counted).
pub fn fn_00b5ac60(e: &mut Engine, this: Ptr<ShadowSceneNode>) -> u32 {
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    let mut count = 0u32;
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light != 0 && e.mem.u16(light.wrapping_add(LIGHT_WORD_110)) != 0xff {
            count = count.wrapping_add(1);
        }
    }
    count
}

// Translated from 00b5ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes element `index` from an array (thiscall, `RET 8`: the
/// index and a shrink flag byte). Without shrinking, or while the count is above a quarter
/// of the capacity (capacity up to 0x400) or above the capacity less 0x800 (larger), the
/// later elements move down one (`00cb9930`) and the count drops. Otherwise a smaller
/// buffer (half the capacity, or 0x400 less above 0x800) comes from virtual slot +4, the
/// elements before the index and then `count - 1` elements from after it are moved into it
/// (the exe's count, not the number after the index), the old buffer goes to virtual slot
/// +8, and the count, buffer and capacity are updated.
pub fn fn_00b5ade0(e: &mut Engine, this: Ptr, index: u32, shrink: u8) {
    let array = this.addr();
    let capacity = e.mem.u32(array.wrapping_add(ARRAY_CAPACITY));
    let shrinks = shrink != 0
        && if capacity > 0x400 {
            e.mem.u32(array.wrapping_add(ARRAY_COUNT)) <= capacity.wrapping_sub(0x800)
        } else {
            e.mem.u32(array.wrapping_add(ARRAY_COUNT)) <= capacity >> 2
        };
    if !shrinks {
        let count = e
            .mem
            .u32(array.wrapping_add(ARRAY_COUNT))
            .wrapping_sub(index)
            .wrapping_sub(1);
        let at = e
            .mem
            .u32(array.wrapping_add(ARRAY_BUFFER))
            .wrapping_add(index.wrapping_mul(4));
        e.call(
            ARRAY_MOVE_00CB9930,
            &args![array, at, at.wrapping_add(4), count],
        );
        let left = e.mem.u32(array.wrapping_add(ARRAY_COUNT)).wrapping_sub(1);
        e.mem.set_u32(array.wrapping_add(ARRAY_COUNT), left);
        return;
    }
    let new_capacity = if capacity > 0x800 {
        capacity.wrapping_sub(0x400)
    } else {
        capacity >> 1
    };
    let fresh = e.vcall(array, 4, &args![new_capacity]).u32();
    let buffer = e.mem.u32(array.wrapping_add(ARRAY_BUFFER));
    e.call(ARRAY_MOVE_00CB9930, &args![array, fresh, buffer, index]);
    let count = e.mem.u32(array.wrapping_add(ARRAY_COUNT)).wrapping_sub(1);
    let buffer = e.mem.u32(array.wrapping_add(ARRAY_BUFFER));
    e.call(
        ARRAY_MOVE_00CB9930,
        &args![
            array,
            fresh.wrapping_add(index.wrapping_mul(4)),
            buffer.wrapping_add(index.wrapping_mul(4)).wrapping_add(4),
            count
        ],
    );
    let buffer = e.mem.u32(array.wrapping_add(ARRAY_BUFFER));
    e.vcall(array, 8, &args![buffer]);
    let left = e.mem.u32(array.wrapping_add(ARRAY_COUNT)).wrapping_sub(1);
    e.mem.set_u32(array.wrapping_add(ARRAY_COUNT), left);
    e.mem.set_u32(array.wrapping_add(ARRAY_BUFFER), fresh);
    e.mem
        .set_u32(array.wrapping_add(ARRAY_CAPACITY), new_capacity);
}

// Translated from 00b5aea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes `count` elements at `index` from an array
/// (thiscall, `RET 8`) by moving the last `min(count, elements after them)` elements into
/// the gap (`00cb9930`; unsigned minimum), then lowers the count by `count`.
pub fn fn_00b5aea0(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let array = this.addr();
    let size = e.mem.u32(array.wrapping_add(ARRAY_COUNT));
    let mut moved = size.wrapping_sub(index).wrapping_sub(count);
    if moved >= count {
        moved = count;
    }
    let buffer = e.mem.u32(array.wrapping_add(ARRAY_BUFFER));
    e.call(
        ARRAY_MOVE_00CB9930,
        &args![
            array,
            buffer.wrapping_add(index.wrapping_mul(4)),
            buffer.wrapping_add(size.wrapping_sub(moved).wrapping_mul(4)),
            moved
        ],
    );
    let left = e
        .mem
        .u32(array.wrapping_add(ARRAY_COUNT))
        .wrapping_sub(count);
    e.mem.set_u32(array.wrapping_add(ARRAY_COUNT), left);
}

// Translated from 00b5af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties an array (thiscall, `RET 4`: a free flag byte). With
/// a buffer: when `free` is non-zero the buffer goes to virtual slot +8 and the buffer and
/// capacity are cleared; the count is cleared. Without a buffer nothing changes.
pub fn fn_00b5af80(e: &mut Engine, this: Ptr, free: u8) {
    let array = this.addr();
    let buffer = e.mem.u32(array.wrapping_add(ARRAY_BUFFER));
    if buffer == 0 {
        return;
    }
    if free != 0 {
        e.vcall(array, 8, &args![buffer]);
        e.mem.set_u32(array.wrapping_add(ARRAY_BUFFER), 0);
        e.mem.set_u32(array.wrapping_add(ARRAY_CAPACITY), 0);
    }
    e.mem.set_u32(array.wrapping_add(ARRAY_COUNT), 0);
}

// Translated from 00b5b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall): for each light of `lShadowLightList`, its dynamic
/// shadow map (+0x10c) is given to the renderer object's `00b6d4c0` and then released
/// (`NiPointer`, inlined) and cleared when present; at the end a jump to the renderer's
/// `00b6d130`.
pub fn fn_00b5b1f0(e: &mut Engine, this: Ptr<ShadowSceneNode>) {
    let mut item = list_head(e, this.addr().wrapping_add(SHADOW_LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light == 0 {
            continue;
        }
        let map = e.mem.u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP));
        let renderer = e.global::<u32>(RENDERER_011F91A8);
        e.call(RENDERER_RELEASE_MAP_00B6D4C0, &args![renderer, map]);
        let map = e.mem.u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP));
        if map != 0 {
            release(e, map);
            e.mem
                .set_u32(light.wrapping_add(LIGHT_DYNAMIC_SHADOW_MAP), 0);
        }
    }
    let renderer = e.global::<u32>(RENDERER_011F91A8);
    e.call(RENDERER_00B6D130, &args![renderer]);
}

// Translated from 00b5b610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a sibling of [`fn_00b5b4d0`]). `thiscall`, `RET 0x28`: a point
/// `a`, a counter pointer, two `float` pointers, a point `b` and a word. Sums each light's
/// `00b9bdf0` (both points and the word) over `lLightList`, counting each light after which
/// the running sum is above the largest sum so far (which it then becomes). The sun light's
/// `NiLight` gives the largest component of its two colours (+0xc8 and +0xd4) for the two
/// `float` pointers. Answers `(1 - c) * second + (sum + first * 0.25)`, where `c` is the dot
/// product of `0.75` times the normalised `b - a` (`004a0c10`) with the sun's normalised
/// direction (+0xf0) plus `0.25`, clamped to `[0, 1]` (NaN kept). Each x87 step is
/// translated in the exe's order, operands and `float` stores.
pub fn fn_00b5b610(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    ax: u32,
    ay: u32,
    az: u32,
    count: Ptr,
    first_peak: Ptr,
    second_peak: Ptr,
    bx: u32,
    by: u32,
    bz: u32,
    word: u32,
) -> f32 {
    let mut sum = x87_store32(0.0);
    let mut largest = x87_store32(0.0);
    e.mem.set_u32(count.addr(), 0);
    let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
    while item != 0 {
        let light = item_element(e, item);
        item = item_next(e, item);
        if light == 0 {
            continue;
        }
        let value = e
            .call(LIGHT_00B9BDF0, &args![light, ax, ay, az, bx, by, bz, word])
            .f64();
        sum = x87_store32(x87_add(value, x87_m32(sum)));
        let running = x87_load32(sum);
        if x87_load32(largest) < running {
            let n = e.mem.u32(count.addr()).wrapping_add(1);
            e.mem.set_u32(count.addr(), n);
            largest = x87_store32(running);
        }
    }
    let running = x87_load32(sum);
    let sun = e.get(this, ShadowSceneNode::pSunLight);
    let ni_light = e.mem.u32(sun.wrapping_add(LIGHT_SP_LIGHT));
    let first = colour_peak(e, ni_light.wrapping_add(0xc8));
    e.mem.set_u32(first_peak.addr(), first);
    let quarter = f64::from_bits(e.global::<u64>(QUARTER_010290B0));
    let direction = [
        e.mem.u32(ni_light.wrapping_add(0xf0)),
        e.mem.u32(ni_light.wrapping_add(0xf4)),
        e.mem.u32(ni_light.wrapping_add(0xf8)),
    ];
    let sum = x87_store32(x87_add(running, x87_mul(x87_load32(first), quarter)));
    let second = colour_peak(e, ni_light.wrapping_add(0xd4));
    e.mem.set_u32(second_peak.addr(), second);
    // The exe's two vectors on its stack: `b - a` (+0) and the sun's direction (+0xc).
    let (towards, sun_direction) = e.with_stack(0x18, |e, frame| {
        let towards = frame.addr();
        let sun_direction = frame.addr() + 0xc;
        for (i, (to, from)) in [(bx, ax), (by, ay), (bz, az)].iter().enumerate() {
            let difference = x87_store32(x87_sub(x87_load32(*to), x87_m32(*from)));
            e.mem.set_u32(towards + 4 * i as u32, difference);
            e.mem.set_u32(sun_direction + 4 * i as u32, direction[i]);
        }
        e.call(POINT_NORMALIZE_004A0C10, &args![towards]);
        e.call(POINT_NORMALIZE_004A0C10, &args![sun_direction]);
        let read =
            |e: &Engine, base: u32| [e.mem.u32(base), e.mem.u32(base + 4), e.mem.u32(base + 8)];
        (read(e, towards), read(e, sun_direction))
    });
    let scale = x87_load64(e.global::<u64>(THREE_QUARTERS_0101DE30));
    // `FMUL ST(1),ST(0)` (DC C9), then `FMUL ST(0),ST(1)` (D8 C9), then a `float` operand.
    let scaled_x = x87_store32(x87_mul(x87_load32(towards[0]), scale));
    let scaled_y = x87_store32(x87_mul(x87_load32(towards[1]), scale));
    let scaled_z = x87_store32(x87_mul(scale, x87_m32(towards[2])));
    let dot = x87_add(
        x87_mul(x87_load32(sun_direction[1]), x87_m32(scaled_y)),
        x87_mul(x87_load32(scaled_x), x87_m32(sun_direction[0])),
    );
    let dot = x87_add(
        dot,
        x87_mul(x87_load32(sun_direction[2]), x87_m32(scaled_z)),
    );
    let c = x87_store32(dot);
    let c = x87_store32(x87_add(x87_load32(c), quarter));
    let value = x87_load32(c);
    let c = if 1.0 < value {
        x87_store32(1.0)
    } else {
        x87_store32(value)
    };
    let value = x87_load32(c);
    let c = if value < 0.0 {
        x87_store32(0.0)
    } else {
        x87_store32(value)
    };
    let rest = x87_store32(x87_sub(1.0, x87_m32(c)));
    let result = x87_store32(x87_add(
        x87_mul(x87_load32(rest), x87_m32(second)),
        x87_m32(sum),
    ));
    x87_quiet(f32::from_bits(result))
}

// Translated from 00b5ba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::BuildSharedCompoundFrustum` (engine map; thiscall, `RET 8`: a camera and
/// a portal). The portal's two rooms (+0xfc, +0x100) each give a frustum (its +0xcc, or the
/// node's own at +0x138 for a missing room) and two flags: whether the room's object (+0xac,
/// then +0xc, then its word +8) is 1 (the node's `bVisibleUnboundSpace` for a missing room),
/// and whether that holds and the room has a frustum (for a missing room: whether the node's
/// word at +0x160 is 0). The compound frustum of the portal's target (+0x104, its +0xac) is
/// set to the camera; unless a room of the first kind has a frustum whose word +0x28 is 0,
/// the frustums used are added (both inside a union group when both rooms count, else the
/// one that counts), then the frustum's empty groups are removed and its op list prepared.
pub fn shadow_scene_node_build_shared_compound_frustum(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    camera: u32,
    portal: u32,
) {
    let node = this.addr();
    let target = e.mem.u32(portal.wrapping_add(0x104));
    let frustum = e.mem.u32(target.wrapping_add(0xac));
    let first_room = e.mem.u32(portal.wrapping_add(0xfc));
    let second_room = e.mem.u32(portal.wrapping_add(0x100));
    let unbound = e.get(this, ShadowSceneNode::bVisibleUnboundSpace);
    let room_is_one = |e: &Engine, room: u32| {
        let object = e.mem.u32(room.wrapping_add(0xac));
        let inner = e.mem.u32(object.wrapping_add(0xc));
        u8::from(e.mem.u32(inner.wrapping_add(8)) == 1)
    };
    let first_flag = if first_room != 0 {
        room_is_one(e, first_room)
    } else {
        unbound
    };
    let second_flag = if second_room != 0 {
        room_is_one(e, second_room)
    } else {
        unbound
    };
    let no_own_word = e.mem.u32(node.wrapping_add(0x160)) == 0;
    let first_counts = if first_room != 0 {
        first_flag != 0 && e.mem.u32(first_room.wrapping_add(0xcc)) != 0
    } else {
        no_own_word
    };
    let second_counts = if second_room != 0 {
        second_flag != 0 && e.mem.u32(second_room.wrapping_add(0xcc)) != 0
    } else {
        no_own_word
    };
    let first_frustum = if first_room != 0 {
        e.mem.u32(first_room.wrapping_add(0xcc))
    } else {
        node.wrapping_add(0x138)
    };
    let second_frustum = if second_room != 0 {
        e.mem.u32(second_room.wrapping_add(0xcc))
    } else {
        node.wrapping_add(0x138)
    };
    e.call(FRUSTUM_SET_CAMERA, &args![frustum, camera]);
    if first_flag != 0 && e.mem.u32(first_frustum.wrapping_add(0x28)) == 0 {
        return;
    }
    if second_flag != 0 && e.mem.u32(second_frustum.wrapping_add(0x28)) == 0 {
        return;
    }
    if first_counts {
        if second_counts {
            e.call(FRUSTUM_START_GROUP_UNION, &args![frustum]);
            e.call(FRUSTUM_MERGE, &args![frustum, first_frustum]);
            e.call(FRUSTUM_MERGE, &args![frustum, second_frustum]);
            e.call(FRUSTUM_END_GROUP, &args![frustum]);
        } else {
            e.call(FRUSTUM_MERGE, &args![frustum, first_frustum]);
        }
    } else if second_counts {
        e.call(FRUSTUM_MERGE, &args![frustum, second_frustum]);
    }
    e.call(FRUSTUM_REMOVE_EMPTY_GROUPS, &args![frustum]);
    e.call(FRUSTUM_PRETHREAD_OP_LIST, &args![frustum]);
}

// Translated from 00b5c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 8`: an `NiLight` and a word): the first light
/// of `lLightList` whose `spLight` is that `NiLight` gets `00b9dcb0` with the word; nothing
/// when there is none.
pub fn fn_00b5c450(e: &mut Engine, this: Ptr<ShadowSceneNode>, ni_light: u32, word: u32) {
    let light = shadow_scene_node_get_light(e, this, ni_light);
    if light != 0 {
        e.call(LIGHT_00B9DCB0, &args![light, word]);
    }
}

// Translated from 00b5c7d0 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `ShadowSceneNode`'s `GetViewerStrings` (its vtable's slot +0x60; thiscall, `RET 4`):
/// `NiNode::GetViewerStrings` (`00a5ea70`), the class name (the first word of `ms_RTTI`,
/// `011f9e80`), `"light count"` with `lLightList`'s count (`_ov6`, the whole word passed),
/// and `"shadow light count"` (`_ov7`) with the number of the list's lights whose
/// `bCastShadow` (+0xec) is set: for each index below the count (as a 16-bit value, signed
/// compare) the list is walked from its head to that index, a position at or past the count
/// reading the null light's byte. The first three strings are added through
/// `SetSize`/`SetAt` (held in the argument word), the last with `SetAt` inlined.
pub fn shadow_scene_node_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    strings: Ptr,
) {
    let list = strings.addr();
    e.call(NI_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let add = |e: &mut Engine, slot: u32| {
        let index = u32::from(e.mem.u16(list + 0xa));
        if index >= u32::from(e.mem.u16(list + 8)) {
            let grow = u32::from(e.mem.u16(list + 0xe));
            e.call(
                STRING_ARRAY_SET_SIZE,
                &args![list, grow.wrapping_add(index)],
            );
        }
        e.call(STRING_ARRAY_SET_AT, &args![list, index, slot]);
    };
    let shadow_lights = e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(SHADOW_SCENE_NODE_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        add(e, slot);
        let lights = e.mem.u32(this.addr().wrapping_add(LIGHT_LIST_COUNT));
        let text = e
            .call(VIEWER_STRING_SHORT, &args![TEXT_LIGHT_COUNT, lights])
            .u32();
        e.mem.set_u32(slot, text);
        add(e, slot);
        let lights = e.mem.u32(this.addr().wrapping_add(LIGHT_LIST_COUNT));
        e.mem.set_u32(slot, 0);
        let mut index = 0u32;
        while (index as i32) < i32::from(lights as u16) {
            let position = index & 0xffff;
            let light = if position >= lights {
                0
            } else {
                let mut item = list_head(e, this.addr().wrapping_add(LIGHT_LIST));
                let mut light = item_element(e, item);
                item = item_next(e, item);
                let mut steps = position;
                while steps != 0 {
                    steps -= 1;
                    light = item_element(e, item);
                    item = item_next(e, item);
                }
                light
            };
            if e.mem.u8(light.wrapping_add(LIGHT_CAST_SHADOW)) != 0 {
                let n = e.mem.u32(slot).wrapping_add(1);
                e.mem.set_u32(slot, n);
            }
            index = index.wrapping_add(1);
        }
        e.mem.u32(slot)
    });
    let text = e
        .call(
            VIEWER_STRING_INT,
            &args![TEXT_SHADOW_LIGHT_COUNT, shadow_lights],
        )
        .u32();
    let index = e.mem.u16(list + 0xa);
    if index >= e.mem.u16(list + 8) {
        let grow = u32::from(e.mem.u16(list + 0xe));
        e.call(
            STRING_ARRAY_SET_SIZE,
            &args![list, grow.wrapping_add(u32::from(index))],
        );
    }
    let element = |e: &Engine| {
        e.mem
            .u32(list + 4)
            .wrapping_add(u32::from(index).wrapping_mul(4))
    };
    if index >= e.mem.u16(list + 0xa) {
        e.mem.set_u16(list + 0xa, index.wrapping_add(1));
        if text != 0 {
            let used = e.mem.u16(list + 0xc).wrapping_add(1);
            e.mem.set_u16(list + 0xc, used);
        }
    } else if text != 0 {
        if e.mem.u32(element(e)) == 0 {
            let used = e.mem.u16(list + 0xc).wrapping_add(1);
            e.mem.set_u16(list + 0xc, used);
        }
    } else if e.mem.u32(element(e)) != 0 {
        let used = e.mem.u16(list + 0xc).wrapping_add(0xffff);
        e.mem.set_u16(list + 0xc, used);
    }
    let at = element(e);
    e.mem.set_u32(at, text);
}

// Translated from 00b5e590 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `ShadowSceneNode::GetRTTI` (its vtable's slot +8): the address of the class's `NiRTTI`
/// record (`011f9e80`).
pub fn shadow_scene_node_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(SHADOW_SCENE_NODE_RTTI)
}

// Translated from 00b5f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`: an `NiLight`): the first light of
/// `lLightList` whose `spLight` is that `NiLight` is written into the argument word and the
/// code jumps to [`fn_00b5d270`] with it; nothing when there is none.
pub fn fn_00b5f080(e: &mut Engine, this: Ptr<ShadowSceneNode>, ni_light: u32) {
    let light = shadow_scene_node_get_light(e, this, ni_light);
    if light != 0 {
        fn_00b5d270(e, this, light);
    }
}

// Translated from 00b5f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::FindAndCullLightRecurse` (engine map; thiscall, `RET 8`: a node and a
/// word whose low byte says remove). For each child below the node's used size (read once;
/// the children's buffer re-read): a child whose RTTI chain (`GetRTTI`, slot +8, then the
/// base records at +4) reaches the record at `011f4a28` is removed
/// ([`shadow_scene_node_remove_light`]) or added ([`shadow_scene_node_add_light`] with 1);
/// any other child that is a node (slot +0xc) is walked the same way with the whole word.
pub fn shadow_scene_node_find_and_cull_light_recurse(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    node: u32,
    remove: u32,
) {
    let count = u32::from(e.mem.u16(node.wrapping_add(NODE_CHILDREN_SIZE)));
    let mut index = 0u32;
    while index < count {
        let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN_BASE));
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        index = index.wrapping_add(1);
        if child == 0 {
            continue;
        }
        if rtti_chain_has(e, child, LIGHT_RTTI_011F4A28) {
            if remove as u8 != 0 {
                shadow_scene_node_remove_light(e, this, child);
            } else {
                shadow_scene_node_add_light(e, this, child, 1);
            }
            continue;
        }
        if e.vcall(child, SLOT_IS_NODE, &args![]).u32() != 0 {
            shadow_scene_node_find_and_cull_light_recurse(e, this, child, remove);
        }
    }
}

// Translated from 00b5f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::FindAndCullLight` (engine map; thiscall, `RET 8`): for a node, a jump
/// to [`shadow_scene_node_find_and_cull_light_recurse`] with the same words; nothing for
/// null.
pub fn shadow_scene_node_find_and_cull_light(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
    node: u32,
    remove: u32,
) {
    if node != 0 {
        shadow_scene_node_find_and_cull_light_recurse(e, this, node, remove);
    }
}

/// Diagnostic only, not exe behaviour (a22: in the race menu the port's
/// `GetRenderPasses_2x` sees no active non-shadow light on the head properties, where the exe
/// sees three, the lights of `Meshes\Terminals\PlayerFaceLights01.NIF` that
/// `FORenderedMenuRaceSex::OnMenuOpen` (`007fd6c0`) adds to scene node 3 with
/// `AddAllLightsFromNode_Recurse` (`00b5ca70`, radius = 3 x the player's bound radius)).
/// With `NV_LIGHT_CHECK` set, each light update `00b5d300` makes for a scene node other than
/// the main one (`011f91c8`) prints the light (its `NiLight`: the culled bit of +0x30, the
/// dimmer +0xc4, the radius +0xe0, the world position +0x8c) and, once the node's children
/// are parsed, the geometry list the light got (+0xe0: the count and the first geometries
/// with their bound centre and radius and their lighting property's light count +0x68).
/// Lines start "light check:" (at most 400). Read-only.
pub mod light_check {
    use super::{item_element, item_next, FALLBACK_SCENE_NODE_011F91C8, LIGHT_SP_LIGHT};
    use crate::prelude::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::OnceLock;

    pub fn on() -> bool {
        static ON: OnceLock<bool> = OnceLock::new();
        *ON.get_or_init(|| std::env::var_os("NV_LIGHT_CHECK").is_some())
    }

    /// Prints one line while under the cap.
    pub fn say(text: &str) {
        static LINES: AtomicU32 = AtomicU32::new(0);
        if LINES.fetch_add(1, Ordering::Relaxed) < 400 {
            println!("light check: {text}");
        }
    }

    fn word(e: &Engine, a: u32) -> Option<u32> {
        e.mem.is_mapped(a).then(|| e.mem.u32(a))
    }

    fn float(e: &Engine, a: u32) -> String {
        word(e, a).map_or("?".into(), |w| format!("{}", f32::from_bits(w)))
    }

    /// A light's `NiLight` in words.
    pub fn light_text(e: &Engine, light: u32) -> String {
        let Some(ni) = word(e, light.wrapping_add(LIGHT_SP_LIGHT)) else {
            return format!("light {light:08x} (unmapped)");
        };
        if ni == 0 || !e.mem.is_mapped(ni) {
            return format!("light {light:08x} NiLight {ni:08x}");
        }
        format!(
            "light {light:08x} NiLight {ni:08x} culled {} dimmer {} radius {} at ({}, {}, {}) shadow {}",
            e.mem.u32(ni + 0x30) & 1,
            float(e, ni + 0xc4),
            float(e, ni + 0xe0),
            float(e, ni + 0x8c),
            float(e, ni + 0x90),
            float(e, ni + 0x94),
            e.mem.u8(light.wrapping_add(0xec)),
        )
    }

    fn is_main(e: &Engine, node: u32) -> bool {
        node == e.global::<u32>(FALLBACK_SCENE_NODE_011F91C8)
    }

    /// At the start of `00b5d300`.
    pub fn before(e: &Engine, node: u32, light: u32) {
        if !on() || is_main(e, node) {
            return;
        }
        let children = u32::from(e.mem.u16(node.wrapping_add(0xa6)));
        say(&format!(
            "update on node {node:08x} ({children} children): {}",
            light_text(e, light)
        ));
    }

    /// After the node's children were parsed (the path without a usable portal graph).
    pub fn after(e: &Engine, node: u32, light: u32) {
        if !on() || is_main(e, node) {
            return;
        }
        let mut count = 0u32;
        let mut shown = Vec::new();
        let mut item = word(e, light.wrapping_add(0xe0)).unwrap_or(0);
        while item != 0 && e.mem.is_mapped(item) {
            let geometry = item_element(e, item);
            item = item_next(e, item);
            count += 1;
            if shown.len() < 8 && geometry != 0 && e.mem.is_mapped(geometry) {
                let bound = e.mem.u32(geometry + 0x20);
                let bound_text = if bound != 0 && e.mem.is_mapped(bound) {
                    format!(
                        "({}, {}, {}) r {}",
                        float(e, bound),
                        float(e, bound + 4),
                        float(e, bound + 8),
                        float(e, bound + 0xc)
                    )
                } else {
                    format!("{bound:08x}")
                };
                let property = e.mem.u32(geometry + 0xa8);
                let lights = if property != 0 && e.mem.is_mapped(property) {
                    e.mem.u32(property + 0x68)
                } else {
                    0
                };
                shown.push(format!(
                    "{geometry:08x} bound {bound_text} property {property:08x} lights {lights}"
                ));
            }
        }
        say(&format!(
            "node {node:08x} light {light:08x} has {count} geometries: {}",
            shown.join("; ")
        ));
    }

    /// `ShadowSceneLight::TestFrustumCull` (`00b9e970`) culled `light` (0xff): the first time
    /// for each light, prints why, the light, the culler's camera (+0xc: its world position
    /// +0x8c and `m_fLODAdjust` +0x110), the scale the distance took (the shader manager's
    /// accumulator's +8 object's +0x110) and the fade distances `011f9444` / `011f9448`.
    pub fn culled(e: &mut Engine, light: u32, culler: u32, reason: &str) {
        use std::collections::HashSet;
        use std::sync::Mutex;
        static SEEN: Mutex<Option<HashSet<u32>>> = Mutex::new(None);
        {
            let mut seen = SEEN
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if !seen.get_or_insert_with(HashSet::new).insert(light) {
                return;
            }
        }
        let camera = word(e, culler.wrapping_add(0xc)).unwrap_or(0);
        let camera_text = if camera != 0 && e.mem.is_mapped(camera) {
            format!(
                "camera {camera:08x} at ({}, {}, {}) lod {}",
                float(e, camera + 0x8c),
                float(e, camera + 0x90),
                float(e, camera + 0x94),
                float(e, camera + 0x110)
            )
        } else {
            format!("camera {camera:08x}")
        };
        let accumulator = e.call(0x00b4_f5c0, &[]).u32();
        let object = word(e, accumulator.wrapping_add(8)).unwrap_or(0);
        let scale = if object != 0 && e.mem.is_mapped(object) {
            float(e, object + 0x110)
        } else {
            "none".into()
        };
        say(&format!(
            "culled {}: {reason}; culler {culler:08x} {camera_text}; accumulator {accumulator:08x} \
             scale {scale}; fade near {} far {}",
            light_text(e, light),
            float(e, 0x011f_9444),
            float(e, 0x011f_9448)
        ));
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// The light line reads the `NiLight`'s culled bit, dimmer, radius and position and
        /// the shadow byte.
        #[test]
        fn light_text_reads_the_ni_light() {
            let mut e = Engine::new();
            let light = e.mem.alloc(0x100);
            let ni = e.mem.alloc(0x100);
            e.mem.set_u32(light + LIGHT_SP_LIGHT, ni);
            e.mem.set_u32(ni + 0x30, 1);
            e.mem.set_f32(ni + 0xc4, 0.5);
            e.mem.set_f32(ni + 0xe0, 96.0);
            e.mem.set_f32(ni + 0x8c, 1.0);
            e.mem.set_f32(ni + 0x90, 2.0);
            e.mem.set_f32(ni + 0x94, 3.0);
            let text = light_text(&e, light);
            assert!(
                text.ends_with("culled 1 dimmer 0.5 radius 96 at (1, 2, 3) shadow 0"),
                "{text}"
            );
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_a930, fn_00b5a930(Ptr, u32, u32, u32) -> u32),
        entry!(0x00b5_aa40, fn_00b5aa40(Ptr) -> u32),
        entry!(0x00b5_ac60, fn_00b5ac60(Ptr<ShadowSceneNode>) -> u32),
        entry!(0x00b5_ade0, fn_00b5ade0(Ptr, u32, u8)),
        entry!(0x00b5_aea0, fn_00b5aea0(Ptr, u32, u32)),
        entry!(0x00b5_af80, fn_00b5af80(Ptr, u8)),
        entry!(0x00b5_b1f0, fn_00b5b1f0(Ptr<ShadowSceneNode>)),
        entry!(
            0x00b5_b610,
            fn_00b5b610(
                Ptr<ShadowSceneNode>,
                u32,
                u32,
                u32,
                Ptr,
                Ptr,
                Ptr,
                u32,
                u32,
                u32,
                u32,
            ) -> f32
        ),
        entry!(
            0x00b5_ba80,
            shadow_scene_node_build_shared_compound_frustum(Ptr<ShadowSceneNode>, u32, u32)
        ),
        entry!(0x00b5_c450, fn_00b5c450(Ptr<ShadowSceneNode>, u32, u32)),
        entry!(
            0x00b5_c7d0,
            shadow_scene_node_get_viewer_strings(Ptr<ShadowSceneNode>, Ptr)
        ),
        entry!(0x00b5_e590, shadow_scene_node_get_rtti() -> Ptr),
        entry!(0x00b5_f080, fn_00b5f080(Ptr<ShadowSceneNode>, u32)),
        entry!(
            0x00b5_f0c0,
            shadow_scene_node_find_and_cull_light_recurse(Ptr<ShadowSceneNode>, u32, u32)
        ),
        entry!(
            0x00b5_f150,
            shadow_scene_node_find_and_cull_light(Ptr<ShadowSceneNode>, u32, u32)
        ),
        entry!(0x00b5_c7c0, fn_00b5c7c0(Ptr)),
        entry!(0x00b5_c360, fn_00b5c360(Ptr) -> u32),
        entry!(
            0x00b5_e0f0,
            shadow_scene_node_shadow_scene_node(Ptr<ShadowSceneNode>) -> Ptr<ShadowSceneNode>
        ),
        entry!(0x00b5_a8d0, fn_00b5a8d0(Ptr<NiTPointerList>, u32, u32)),
        entry!(0x00b5_a9d0, fn_00b5a9d0(Ptr, u32, u32, u32)),
        entry!(0x00b5_aa70, fn_00b5aa70(Ptr, Ptr) -> i32),
        entry!(
            0x00b5_aac0,
            shadow_scene_node_set_sun_light(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(0x00b5_ac20, fn_00b5ac20(Ptr<ShadowSceneNode>, u32) -> u32),
        entry!(
            0x00b5_ac90,
            shadow_scene_node_update_occlusion_plane_visibility(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(0x00b5_aee0, fn_00b5aee0(Ptr, u32)),
        entry!(0x00b5_af30, fn_00b5af30(Ptr, u32)),
        entry!(0x00b5_afc0, fn_00b5afc0(Ptr<ShadowSceneNode>) -> u32),
        entry!(0x00b5_b010, fn_00b5b010(Ptr<ShadowSceneNode>) -> u32),
        entry!(0x00b5_b060, fn_00b5b060(Ptr<ShadowSceneNode>)),
        entry!(
            0x00b5_b0e0,
            shadow_scene_node_remove_geometry_from_property_lights(u32)
        ),
        entry!(
            0x00b5_b150,
            shadow_scene_node_remove_object_recurse(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(
            0x00b5_b1c0,
            shadow_scene_node_remove_object(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(
            0x00b5_b260,
            shadow_scene_node_update_queued_object_lighting(u32, u32, u32, u32, u32) -> u8
        ),
        entry!(
            0x00b5_b410,
            shadow_scene_node_update_queued_object_lighting_list(u32, u32, u32, u32) -> u8
        ),
        entry!(
            0x00b5_b4a0,
            shadow_scene_node_get_light(Ptr<ShadowSceneNode>, u32) -> u32
        ),
        entry!(
            0x00b5_b4d0,
            fn_00b5b4d0(Ptr<ShadowSceneNode>, u32, u32, u32, Ptr, Ptr, Ptr, u32) -> f32
        ),
        entry!(
            0x00b5_b880,
            shadow_scene_node_update_dynamic_shadows(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(
            0x00b5_bbe0,
            fn_00b5bbe0(Ptr<ShadowSceneNode>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00b5_bca0,
            fn_00b5bca0(Ptr<ShadowSceneNode>, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x00b5_c3c0, fn_00b5c3c0(Ptr<ShadowSceneNode>)),
        entry!(
            0x00b5_c490,
            shadow_scene_node_update_queued_object_lighting_ov2(u32, u32, u32, u32, u32) -> u8
        ),
        entry!(
            0x00b5_c5c0,
            shadow_scene_node_update_queued_object_lighting_list_ov2(u32, u32, u32, u32) -> u8
        ),
        entry!(0x00b5_c6e0, fn_00b5c6e0(Ptr, u32)),
        entry!(
            0x00b5_c940,
            shadow_scene_node_add_light(Ptr<ShadowSceneNode>, u32, u8) -> u32
        ),
        entry!(
            0x00b5_ca70,
            shadow_scene_node_add_all_lights_from_node_recurse(
                Ptr<ShadowSceneNode>,
                u32,
                f32,
            ) -> u32
        ),
        entry!(0x00b5_cbd0, fn_00b5cbd0(Ptr<ShadowSceneNode>, u32) -> u32),
        entry!(0x00b5_cd00, fn_00b5cd00(Ptr<ShadowSceneNode>, u32) -> u32),
        entry!(0x00b5_cde0, fn_00b5cde0(Ptr<ShadowSceneNode>, u32)),
        entry!(0x00b5_cff0, fn_00b5cff0(Ptr<ShadowSceneNode>, Ptr)),
        entry!(
            0x00b5_d040,
            shadow_scene_node_remove_queued_light(Ptr<ShadowSceneNode>, Ptr, u32)
        ),
        entry!(0x00b5_d180, fn_00b5d180(Ptr<ShadowSceneNode>)),
        entry!(0x00b5_d270, fn_00b5d270(Ptr<ShadowSceneNode>, u32)),
        entry!(0x00b5_d300, fn_00b5d300(Ptr<ShadowSceneNode>, u32)),
        entry!(0x00b5_d930, fn_00b5d930(Ptr<ShadowSceneNode>, u32)),
        entry!(
            0x00b5_d9f0,
            shadow_scene_node_update_object_lighting(Ptr<ShadowSceneNode>, u32, u8)
        ),
        entry!(
            0x00b5_dac0,
            shadow_scene_node_update_queued_object_lighting_start(u32, u32, u32, u32) -> u8
        ),
        entry!(0x00b5_ddb0, fn_00b5ddb0(Ptr<ShadowSceneNode>, u8, u8)),
        entry!(0x00b5_ddf0, fn_00b5ddf0(Ptr<ShadowSceneNode>, u32)),
        entry!(0x00b5_e5a0, fn_00b5e5a0(Ptr<ShadowSceneNode>)),
        entry!(
            0x00b5_e870,
            shadow_scene_node_pre_on_visible(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(0x00b5_eca0, fn_00b5eca0(Ptr<ShadowSceneNode>, u32) -> u32),
        entry!(0x00b5_edf0, fn_00b5edf0(Ptr<ShadowSceneNode>, u32)),
        entry!(
            0x00b5_eeb0,
            shadow_scene_node_add_object(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(
            0x00b5_eed0,
            shadow_scene_node_remove_light(Ptr<ShadowSceneNode>, u32)
        ),
        entry!(
            0x00b5_ef90,
            shadow_scene_node_remove_queued_light_ov2(Ptr<ShadowSceneNode>, Ptr)
        ),
        entry!(
            0x00b5_f170,
            fn_00b5f170(Ptr<ShadowSceneNode>, u32, u32, u32, u32)
        ),
        entry!(
            0x00b5_f980,
            fn_00b5f980(Ptr<ShadowSceneNode>, u32) -> Ptr<ShadowSceneNode>
        ),
        entry!(
            0x00b5_f9b0,
            shadow_scene_node_on_visible(Ptr<ShadowSceneNode>, u32)
        ),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
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

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.put_vtable(0x0f00_d100, &[0, 0x0f00_d000]);
        e.register(0x0f00_d000, |_, a| (0x4000_0000 + a[1]).into_ret());
        e.register(ARRAY_REALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0f00_d100);
        (e, array)
    }

    fn set_counts(e: &mut Engine, array: u32, size: u32, capacity: u32) {
        e.mem.set_u32(array + 4, 0x5000);
        e.mem.set_u32(array + 8, size);
        e.mem.set_u32(array + 0xc, capacity);
    }

    #[test]
    fn at_00b5c360_allocates_four_elements_for_an_empty_array() {
        let (mut e, array) = rig();
        assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), 0);
        assert_eq!(calls(&e, 0x0f00_d000), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(array + 4), 0x4000_0004);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0xc), 4);
    }

    #[test]
    fn at_00b5c360_doubles_a_full_array_up_to_0x400_then_grows_by_0x400() {
        for (capacity, grown) in [
            (4u32, 8u32),
            (0x200, 0x400),
            (0x400, 0x800),
            (0x401, 0x801),
            (0x800, 0xc00),
        ] {
            let (mut e, array) = rig();
            set_counts(&mut e, array, capacity, capacity);
            assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), capacity);
            assert_eq!(
                calls(&e, ARRAY_REALLOCATE),
                vec![vec![array, grown, capacity]]
            );
            assert_eq!(e.mem.u32(array + 0xc), grown);
            assert_eq!(e.mem.u32(array + 8), capacity + 1);
            assert_eq!(e.mem.u32(array + 4), 0x5000);
        }
    }

    #[test]
    fn at_00b5c360_just_counts_when_there_is_room() {
        let (mut e, array) = rig();
        set_counts(&mut e, array, 3, 8);
        assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), 3);
        assert_eq!(e.mem.u32(array + 8), 4);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert!(calls(&e, ARRAY_REALLOCATE).is_empty());
        assert!(calls(&e, 0x0f00_d000).is_empty());
    }
}

#[cfg(test)]
mod b000_tests {
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

    const SET_BOUND: u32 = 0x0f00_2000;
    const OBJECT_VTABLE: u32 = 0x0f00_3000;
    const DELETE: u32 = 0x0f00_4000;

    /// Doubles for the constructor's callees: constructors answer `this` (objects get a
    /// vtable whose `+4` is `DELETE`), the node's vtable has `SetBound` at `+0x78`, which
    /// stores the bound at `+0x20`.
    fn rig(with_bound: bool) -> (Engine, Ptr<ShadowSceneNode>) {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x0109_b000, 0x010a_d000, 0x011f_4000] {
            e.map(page, 0x1000);
        }
        e.set_global(FOG_DISTANCE_010AD6E0, 0x7f80_0001u32);
        e.set_global(FOG_COLOR_01016248, 0.5f32);
        for i in 0..3u32 {
            e.set_global(POINT_011F426C + 4 * i, 0x100 + i);
            e.set_global(COLOR_011F4998 + 4 * i, 0x200 + i);
        }
        e.set_global(FIXED_STRING_NONE, 0x0bad_0000u32);
        let mut slots = vec![0u32; 0x20];
        slots[0x78 / 4] = SET_BOUND;
        e.put_vtable(SHADOW_SCENE_NODE_VTABLE, &slots);
        e.put_vtable(OBJECT_VTABLE, &[0, DELETE]);
        e.register(DELETE, |_, _| Ret::default());
        e.register_double(NI_NODE_CONSTRUCT, move |e, a| {
            if with_bound {
                let bound = e.mem.alloc(0x10);
                e.mem.set_u32(a[0] + 0x20, bound);
            }
            a[0].into_ret()
        });
        e.register(SET_BOUND, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        for constructor in [
            SHADOW_SCENE_LIGHT_CONSTRUCT,
            DIRECTIONAL_LIGHT_CONSTRUCT,
            FOG_PROPERTY_CONSTRUCT,
        ] {
            e.register(constructor, |e, a| {
                e.mem.set_u32(a[0], OBJECT_VTABLE);
                a[0].into_ret()
            });
        }
        e.register(FIXED_STRING_MAKE, |e, _| {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, 2);
            (block + 8).into_ret()
        });
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(COMPOUND_FRUSTUM_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(SHADOW_SCENE_LIGHT_SET_LIGHT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        let this: Ptr<ShadowSceneNode> = e.new_object();
        // Garbage in every word the constructor must clear.
        for offset in (0xb4..0x200).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xdddd_dddd);
        }
        (e, this)
    }

    #[test]
    fn constructor_builds_lights_bound_and_fog() {
        let (mut e, this) = rig(false);
        // The name setter leaves a reflection map behind: the constructor releases it.
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, OBJECT_VTABLE);
        e.mem.set_u32(map + 4, 1);
        e.register_double(SET_NAME, move |e, a| {
            e.mem.set_u32(a[0] + 0x124, map);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let base = this.addr();
        assert_eq!(e.call(0x00b5_e0f0, &args![this]).u32(), base);

        assert_eq!(calls(&e, NI_NODE_CONSTRUCT), vec![vec![base, 0]]);
        assert_eq!(e.mem.u32(base), SHADOW_SCENE_NODE_VTABLE);
        for list in [0xb4u32, 0xc0, 0xe4, 0xf0, 0xfc, 0x108, 0x114] {
            for word in 0..3 {
                assert_eq!(e.mem.u32(base + list + 4 * word), 0, "{list:x}");
            }
        }
        assert_eq!(
            calls(&e, COMPOUND_FRUSTUM_CONSTRUCT),
            vec![vec![base + 0x138]]
        );
        assert_eq!(calls(&e, FIXED_STRING_MAKE), vec![vec![NAME_TEXT]]);
        // The name's temporary was released once (count 2 -> 1).
        let name_block = calls(&e, IMPORT_INTERLOCKED_DECREMENT)[0][0];
        assert_eq!(e.mem.u32(name_block), 1);
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL),
            vec![vec![base + 0xb4], vec![base + 0xc0]]
        );
        assert_eq!(calls(&e, DELETE), vec![vec![map]]);
        assert_eq!(e.get(this, ShadowSceneNode::pWindowReflectionEnvMap), 0);
        assert_eq!(e.get(this, ShadowSceneNode::bVisibleUnboundSpace), 1);
        assert_eq!(e.get(this, ShadowSceneNode::bAllowLightRemoveQueues), 1);
        assert_eq!(e.get(this, ShadowSceneNode::bDisableLightUpdate), 0);
        assert_eq!(e.get(this, ShadowSceneNode::bWireframe), 0);
        assert_eq!(e.get(this, ShadowSceneNode::iCurrentCubeMapFace), 0);
        for (i, offset) in [0x1e4u32, 0x1e8, 0x1ec].iter().enumerate() {
            assert_eq!(e.mem.u32(base + offset), 0x100 + i as u32);
            assert_eq!(e.mem.u32(base + offset + 0xc), 0x100 + i as u32);
        }

        // Fence lights held once; the sun light is set to the directional light.
        for slot in [0xd8u32, 0xdc] {
            let light = e.mem.u32(base + slot);
            assert_ne!(light, 0);
            assert_eq!(e.mem.u32(light + 4), 1);
        }
        let directional = calls(&e, DIRECTIONAL_LIGHT_CONSTRUCT)[0][0];
        let sun = e.get(this, ShadowSceneNode::pSunLight);
        assert_eq!(
            calls(&e, SHADOW_SCENE_LIGHT_SET_LIGHT),
            vec![vec![sun, directional]]
        );
        assert_eq!(e.mem.u32(directional + 0xa8), 2);
        for i in 0..3u32 {
            assert_eq!(e.mem.u32(directional + 0xc8 + 4 * i), 0x200 + i);
            assert_eq!(e.mem.u32(directional + 0xd4 + 4 * i), 0x200 + i);
        }
        assert_eq!(calls(&e, NEW_OBJECT).len(), 5);

        // A new bound: the point, then radius 1.0.
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x10]]);
        let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
        assert_eq!(calls(&e, SET_BOUND), vec![vec![base, bound]]);
        assert_eq!(e.mem.u32(bound), 0x100);
        assert_eq!(e.mem.u32(bound + 8), 0x102);
        assert_eq!(e.mem.f32(bound + 0xc), 1.0);
        assert_eq!(e.get(this, ShadowSceneNode::m_uFlags) & 0x2800, 0x2800);

        // The fog: held once, distances a quiet NaN (FLD of a signalling one), colours 0.5.
        let fog = e.get(this, ShadowSceneNode::spFog);
        assert_eq!(e.mem.u32(fog + 4), 1);
        for offset in [0x2cu32, 0x30, 0x34, 0x38] {
            assert_eq!(e.mem.u32(fog + offset), 0x7fc0_0001);
        }
        for offset in [0x20u32, 0x24, 0x28, 0x54, 0x58, 0x5c] {
            assert_eq!(e.mem.f32(fog + offset), 0.5);
        }
    }

    #[test]
    fn constructor_keeps_an_existing_bound_and_sets_its_radius() {
        let (mut e, this) = rig(true);
        e.call_log = Some(vec![]);
        e.call(0x00b5_e0f0, &args![this]);
        assert!(calls(&e, SET_BOUND).is_empty());
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls(&e, DELETE).is_empty());
        let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
        assert_eq!(e.mem.f32(bound + 0xc), 1.0);
        assert_eq!(e.mem.u32(bound), 0);
    }
}

#[cfg(test)]
mod b000f_tests {
    use super::*;

    #[test]
    fn fn_00b5c7c0_jumps_to_the_list_remove_all() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE_ALL_00E74D40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_c7c0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|c| c.0 == LIST_REMOVE_ALL_00E74D40)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, [0x1000]);
    }
}

#[cfg(test)]
mod b064_tests {
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

    const DELETE: u32 = 0x0f10_0000;
    const IS_NODE: u32 = 0x0f10_0004;
    const IS_GEOM: u32 = 0x0f10_0008;
    const GET_RTTI: u32 = 0x0f10_000c;
    const SLOT_FREE: u32 = 0x0f10_0010;

    /// An engine with counting `InterlockedIncrement`/`Decrement`, a spin lock that counts,
    /// a list append that really appends, and the pages of the globals the node uses.
    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_3000,
            0x0101_6000,
            0x0101_7000,
            0x0102_1000,
            0x0102_2000,
            0x0103_c000,
            0x0106_b000,
            0x0101_f000,
            0x010a_2000,
            0x011a_d000,
            0x011c_5000,
            0x011f_4000,
            0x011f_9000,
            0x011f_a000,
            0x0120_0000,
            0x0120_2000,
            0x0120_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(SPIN_LOCK_LOCK, |e, a| {
            e.mem.set_u32(a[0], 1);
            let v = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, v);
            Ret::default()
        });
        e.register(LIST_ADD_TAIL_00A5A310, |e, a| {
            let value = e.mem.u32(a[1]);
            append(e, a[0], value);
            Ret::default()
        });
        e.register(LIST_REMOVE_ALL, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + 4 * word, 0);
            }
            Ret::default()
        });
        e.register(DELETE, |_, _| Ret::default());
        e
    }

    /// Appends an item holding `value` to the `NiTPointerList` at `list`.
    fn append(e: &mut Engine, list: u32, value: u32) -> u32 {
        let item = e.mem.alloc(12);
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(item + 8, value);
        e.mem.set_u32(item + 4, tail);
        if tail != 0 {
            e.mem.set_u32(tail, item);
        } else {
            e.mem.set_u32(list, item);
        }
        e.mem.set_u32(list + 4, item);
        let count = e.mem.u32(list + 8) + 1;
        e.mem.set_u32(list + 8, count);
        item
    }

    /// The elements of the list at `list`.
    fn elements(e: &Engine, list: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut item = e.mem.u32(list);
        while item != 0 {
            out.push(e.mem.u32(item + 8));
            item = e.mem.u32(item);
        }
        out
    }

    /// A reference-counted object (count 1) whose vtable has the shared doubles: delete
    /// (+4), `GetRTTI` (+8, answering the word at +0x14), `IsNode` (+0xc, the word at +0x10),
    /// `IsTriBasedGeom` (+0x1c, the word at +0x18); slot +8 of an array object frees.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 4, DELETE);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        e.mem.set_u32(vtable + 0xc, IS_NODE);
        e.mem.set_u32(vtable + 0x1c, IS_GEOM);
        let object = e.mem.alloc(size.max(0x250));
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, 1);
        object
    }

    fn object_doubles(e: &mut Engine) {
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(IS_NODE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(IS_GEOM, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
    }

    /// A node whose children buffer holds `children`.
    fn node_with(e: &mut Engine, children: &[u32]) -> u32 {
        let node = object(e, 0x200);
        e.mem.set_u32(node + 0x10, node);
        let buffer = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *c);
        }
        e.mem.set_u32(node + NODE_CHILDREN_BASE, buffer);
        e.mem
            .set_u16(node + NODE_CHILDREN_SIZE, children.len() as u16);
        node
    }

    fn scene(e: &mut Engine) -> Ptr<ShadowSceneNode> {
        e.new_object()
    }

    #[test]
    fn fn_00b5a8d0_moves_an_item_after_another() {
        let mut e = rig();
        let list = e.mem.alloc(12);
        let a = append(&mut e, list, 1);
        append(&mut e, list, 2);
        let c = append(&mut e, list, 3);
        e.call(0x00b5_a8d0, &args![list, a, c]);
        assert_eq!(elements(&e, list), vec![2, 3, 1]);
        assert_eq!(e.mem.u32(list + 4), a);
        assert_eq!(e.mem.u32(a + 4), c);
        // The same item: nothing.
        e.call(0x00b5_a8d0, &args![list, c, c]);
        assert_eq!(elements(&e, list), vec![2, 3, 1]);
    }

    #[test]
    fn fn_00b5a9d0_stores_the_point_position() {
        let mut e = rig();
        let light = e.mem.alloc(0x250);
        e.call(0x00b5_a9d0, &args![light, 1u32, 2u32, 3u32]);
        assert_eq!(
            [
                e.mem.u32(light + 0x100),
                e.mem.u32(light + 0x104),
                e.mem.u32(light + 0x108)
            ],
            [1, 2, 3]
        );
    }

    #[test]
    fn fn_00b5aa70_orders_by_the_float_at_0xc_descending() {
        let mut e = rig();
        let make = |e: &mut Engine, v: f32| {
            let o = e.mem.alloc(0x10);
            e.mem.set_f32(o + 0xc, v);
            let p = e.mem.alloc(4);
            e.mem.set_u32(p, o);
            p
        };
        let two = make(&mut e, 2.0);
        let one = make(&mut e, 1.0);
        let nan = make(&mut e, f32::NAN);
        assert_eq!(e.call(0x00b5_aa70, &args![two, one]).i32(), -1);
        assert_eq!(e.call(0x00b5_aa70, &args![one, two]).i32(), 1);
        assert_eq!(e.call(0x00b5_aa70, &args![one, one]).i32(), 0);
        assert_eq!(e.call(0x00b5_aa70, &args![one, nan]).i32(), 0);
    }

    #[test]
    fn set_sun_light_replaces_the_sun_with_a_light_or_a_new_directional_light() {
        let mut e = rig();
        let this = scene(&mut e);
        let old = object(&mut e, 0x250);
        let deleting = e.mem.u32(old);
        e.mem.set_u32(deleting, 0x0f10_0100);
        e.register(0x0f10_0100, |_, _| Ret::default());
        e.set(this, ShadowSceneNode::pSunLight, old);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SHADOW_SCENE_LIGHT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(DIRECTIONAL_LIGHT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SHADOW_SCENE_LIGHT_SET_LIGHT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_aac0, &args![this, 0x7777u32]);
        assert_eq!(calls(&e, 0x0f10_0100), vec![vec![old, 1]]);
        let sun = e.get(this, ShadowSceneNode::pSunLight);
        assert_eq!(
            calls(&e, SHADOW_SCENE_LIGHT_SET_LIGHT),
            vec![vec![sun, 0x7777]]
        );
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![0x250]]);

        // Without a light: a directional light with the two colours, counted twice.
        for i in 0..3u32 {
            e.set_global(COLOR_011F4998 + 4 * i, 0x40 + i);
        }
        e.set(this, ShadowSceneNode::pSunLight, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_aac0, &args![this, 0u32]);
        let directional = calls(&e, DIRECTIONAL_LIGHT_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(directional + 0xa8), 2);
        assert_eq!(e.mem.u32(directional + 0xd4), 0x40);
        assert_eq!(e.mem.u32(directional + 0xcc), 0x41);
        let sun = e.get(this, ShadowSceneNode::pSunLight);
        assert_eq!(
            calls(&e, SHADOW_SCENE_LIGHT_SET_LIGHT),
            vec![vec![sun, directional]]
        );
    }

    #[test]
    fn fn_00b5ac20_answers_the_indexed_light_or_0() {
        let mut e = rig();
        let this = scene(&mut e);
        let list = this.addr() + LIGHT_LIST;
        for v in [0x10u32, 0x20, 0x30] {
            append(&mut e, list, v);
        }
        assert_eq!(e.call(0x00b5_ac20, &args![this, 0u32]).u32(), 0x10);
        assert_eq!(e.call(0x00b5_ac20, &args![this, 2u32]).u32(), 0x30);
        assert_eq!(e.call(0x00b5_ac20, &args![this, 3u32]).u32(), 0);
    }

    #[test]
    fn occlusion_plane_visibility_sets_each_planes_state() {
        let mut e = rig();
        let this = scene(&mut e);
        e.call_log = Some(vec![]);
        // No portal graph: nothing.
        e.call(0x00b5_ac90, &args![this, 0x1234u32]);
        assert!(calls(&e, CULLING_PROCESS_CONSTRUCT).is_empty());
        let graph = e.mem.alloc(0x40);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        let planes = e.mem.alloc(12);
        let visible = e.mem.alloc(0x50);
        let hidden = e.mem.alloc(0x50);
        append(&mut e, planes, visible);
        append(&mut e, planes, hidden);
        e.mem.set_u32(graph + 8, e.mem.u32(planes));
        let camera = e.mem.alloc(0x100);
        e.mem.set_f32(camera + 0x8c, 5.0);
        for f in [
            CULLING_PROCESS_CONSTRUCT,
            CULLING_PROCESS_SET_FRUSTUM,
            CULLING_PROCESS_DESTRUCT,
            NI_PLANE_CONSTRUCT_00A69940,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(OCCLUSION_PLANE_WITHIN_FRUSTUM, |_, _| 1u32.into_ret());
        e.register_double(OCCLUSION_PLANE_VIEWER_ON_CORRECT_SIDE, move |_, a| {
            u32::from(a[0] == visible).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_ac90, &args![this, camera]);
        assert_eq!(e.mem.u32(visible + 0x40), 1);
        assert_eq!(e.mem.u32(hidden + 0x40), 2);
        let local = calls(&e, CULLING_PROCESS_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, CULLING_PROCESS_SET_FRUSTUM),
            vec![vec![local, camera + 0xdc]]
        );
        assert_eq!(
            calls(&e, OCCLUSION_PLANE_VIEWER_ON_CORRECT_SIDE)[0],
            vec![visible, 5.0f32.to_bits(), 0, 0]
        );
        assert_eq!(calls(&e, NI_PLANE_CONSTRUCT_00A69940).len(), 6);
        assert_eq!(calls(&e, CULLING_PROCESS_DESTRUCT), vec![vec![local]]);
    }

    #[test]
    fn fn_00b5aee0_and_fn_00b5af30_assign_their_pointers() {
        let mut e = rig();
        let light = e.mem.alloc(0x250);
        let old = object(&mut e, 0x10);
        let new = object(&mut e, 0x10);
        e.mem.set_u32(light + 0x10c, old);
        e.call(0x00b5_aee0, &args![light, new]);
        assert_eq!(e.mem.u32(light + 0x10c), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.mem.u32(old + 4), 0);
        e.call(0x00b5_af30, &args![light, new]);
        assert_eq!(e.mem.u32(light + 0x128), new);
        assert_eq!(e.mem.u32(new + 4), 3);
    }

    #[test]
    fn shadow_light_walk_skips_lights_without_an_ni_light_but_not_the_last() {
        let mut e = rig();
        let this = scene(&mut e);
        let list = this.addr() + SHADOW_LIGHT_LIST;
        let bare = e.mem.alloc(0x250);
        let lit = e.mem.alloc(0x250);
        e.mem.set_u32(lit + 0xf8, 0x99);
        let last = e.mem.alloc(0x250);
        for l in [bare, lit, last] {
            append(&mut e, list, l);
        }
        assert_eq!(e.call(0x00b5_afc0, &args![this]).u32(), lit);
        assert_eq!(e.call(0x00b5_b010, &args![this]).u32(), last);
        assert_eq!(e.call(0x00b5_b010, &args![this]).u32(), 0);
        let empty = scene(&mut e);
        assert_eq!(e.call(0x00b5_afc0, &args![empty]).u32(), 0);
    }

    #[test]
    fn fn_00b5b060_drops_the_shadow_maps_of_lit_shadow_lights() {
        let mut e = rig();
        let this = scene(&mut e);
        e.set_global(RENDERER_011F91A8, 0x5555u32);
        let light = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        let map = object(&mut e, 0x10);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.mem.set_u32(light + 0x10c, map);
        append(&mut e, this.addr() + SHADOW_LIGHT_LIST, light);
        append(&mut e, this.addr() + SHADOW_LIGHT_LIST, 0);
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        e.register(LIGHT_00B9E770, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_b060, &args![this]);
        assert_eq!(
            calls(&e, RENDERER_RELEASE_MAP_00B6D4C0),
            vec![vec![0x5555, map]]
        );
        assert_eq!(calls(&e, DELETE), vec![vec![map]]);
        assert_eq!(e.mem.u32(light + 0x10c), 0);
        assert_eq!(e.mem.u32(ni_light + 0x30) & 1, 1);
        assert_eq!(calls(&e, LIGHT_00B9E770), vec![vec![light]]);
    }

    /// A geometry whose property word (+0xa8) is a property of type `kind`.
    fn geometry(e: &mut Engine, kind: u32) -> (u32, u32) {
        let geometry = object(e, 0x100);
        e.mem.set_u32(geometry + 0x18, geometry);
        let property = e.mem.alloc(0x80);
        e.mem.set_u32(property + 0x1c, kind);
        e.mem.set_u32(geometry + 0xa8, property);
        (geometry, property)
    }

    #[test]
    fn remove_geometry_from_property_lights_walks_the_lights() {
        let mut e = rig();
        let (geometry, property) = geometry(&mut e, 5);
        let mut left = vec![0u32, 0x22, 0x11];
        e.register_double(PROPERTY_NEXT_LIGHT, move |_, _| {
            left.pop().unwrap().into_ret()
        });
        e.register(LIGHT_REMOVE_TRI_GEOM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_b0e0, &args![geometry]);
        assert_eq!(
            calls(&e, LIGHT_REMOVE_TRI_GEOM),
            vec![vec![0x11, geometry, 0], vec![0x22, geometry, 0]]
        );
        assert_eq!(calls(&e, PROPERTY_NEXT_LIGHT)[0][0], property);
        // Type 13 is not a lighting property.
        let (other, _) = self::geometry(&mut e, 13);
        e.call_log = Some(vec![]);
        e.call(0x00b5_b0e0, &args![other]);
        assert!(calls(&e, PROPERTY_NEXT_LIGHT).is_empty());
    }

    #[test]
    fn remove_object_walks_nodes_down_to_their_geometries() {
        let mut e = rig();
        object_doubles(&mut e);
        let this = scene(&mut e);
        let (leaf, property) = geometry(&mut e, 1);
        let inner = node_with(&mut e, &[leaf]);
        let (top_leaf, top_property) = geometry(&mut e, 2);
        let root = node_with(&mut e, &[top_leaf, 0, inner]);
        e.register(PROPERTY_NEXT_LIGHT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b5_b1c0, &args![this, root]);
        let walked: Vec<u32> = calls(&e, PROPERTY_NEXT_LIGHT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(walked, vec![top_property, property]);
        // An object that is not a node: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00b5_b1c0, &args![this, top_leaf]);
        assert!(calls(&e, PROPERTY_NEXT_LIGHT).is_empty());
    }

    #[test]
    fn update_queued_object_lighting_adds_the_lights_in_range() {
        let mut e = rig();
        let (geometry, property) = geometry(&mut e, 3);
        let near = e.mem.alloc(0x250);
        let far = e.mem.alloc(0x250);
        e.mem.set_u32(near + 0xf8, 0x61);
        e.mem.set_u32(far + 0xf8, 0x62);
        let collection = e.mem.alloc(0x100);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, near);
        e.mem.set_u32(array + 4, far);
        e.mem.set_u32(collection + 0xe0, array);
        e.mem.set_u32(collection + 0xe4, 2);
        e.register(PROPERTY_FENCE_ALL_LIGHTS, |_, _| Ret::default());
        e.register(PROPERTY_REMOVE_FENCED_LIGHTS, |_, _| Ret::default());
        e.register(LIGHT_BOUND_WITHIN_RADIUS, |_, a| {
            u32::from(a[2] == 0x61).into_ret()
        });
        e.register(LIGHT_ADD_TRI_GEOM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let r = e.call(
            0x00b5_b260,
            &args![geometry, 0x9000u32, 0u32, collection, 0u32],
        );
        assert_eq!(r.u8(), 1);
        assert_eq!(calls(&e, PROPERTY_FENCE_ALL_LIGHTS), vec![vec![property]]);
        let bound = calls(&e, LIGHT_BOUND_WITHIN_RADIUS)[0][1];
        assert_eq!(
            calls(&e, LIGHT_BOUND_WITHIN_RADIUS),
            vec![
                vec![near, bound, 0x61, 1.0f32.to_bits()],
                vec![far, bound, 0x62, 1.0f32.to_bits()]
            ]
        );
        assert_eq!(calls(&e, LIGHT_ADD_TRI_GEOM), vec![vec![near, geometry]]);
        assert_eq!(
            calls(&e, PROPERTY_REMOVE_FENCED_LIGHTS),
            vec![vec![property, geometry]]
        );
        assert_eq!(e.mem.u32(geometry + 0x30), 0x40_0000);
        // No property word: 1; type -1: 0.
        let (bare, _) = self::geometry(&mut e, 1);
        e.mem.set_u32(bare + 0xa8, 0);
        assert_eq!(
            e.call(0x00b5_b260, &args![bare, 0u32, 0u32, 0u32, 0u32])
                .u8(),
            1
        );
        let (odd, _) = self::geometry(&mut e, u32::MAX);
        assert_eq!(
            e.call(0x00b5_b260, &args![odd, 0u32, 0u32, 0u32, 0u32])
                .u8(),
            0
        );
    }

    #[test]
    fn update_queued_object_lighting_list_adds_every_listed_light() {
        let mut e = rig();
        let (geometry, property) = geometry(&mut e, 12);
        let lights = e.mem.alloc(12);
        e.mem.set_u32(lights, 0x71);
        e.mem.set_u32(lights + 4, 0x72);
        e.register(PROPERTY_FENCE_ALL_LIGHTS, |_, _| Ret::default());
        e.register(PROPERTY_REMOVE_FENCED_LIGHTS, |_, _| Ret::default());
        e.register(LIGHT_ADD_TRI_GEOM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let r = e.call(0x00b5_b410, &args![geometry, 0u32, lights, 0u32]);
        assert_eq!(r.u8(), 1);
        assert_eq!(
            calls(&e, LIGHT_ADD_TRI_GEOM),
            vec![vec![0x71, geometry], vec![0x72, geometry]]
        );
        assert_eq!(calls(&e, PROPERTY_FENCE_ALL_LIGHTS), vec![vec![property]]);
        assert_eq!(e.mem.u32(geometry + 0x30), 0x40_0000);
    }

    #[test]
    fn get_light_finds_the_light_of_an_ni_light() {
        let mut e = rig();
        let this = scene(&mut e);
        let a = e.mem.alloc(0x250);
        let b = e.mem.alloc(0x250);
        e.mem.set_u32(a + 0xf8, 0x1);
        e.mem.set_u32(b + 0xf8, 0x2);
        for l in [0, a, b] {
            append(&mut e, this.addr() + LIGHT_LIST, l);
        }
        assert_eq!(e.call(0x00b5_b4a0, &args![this, 2u32]).u32(), b);
        assert_eq!(e.call(0x00b5_b4a0, &args![this, 3u32]).u32(), 0);
    }

    #[test]
    fn fn_00b5b4d0_sums_the_lights_and_reports_the_sun_colour_peaks() {
        let mut e = rig();
        let this = scene(&mut e);
        for v in [0x31u32, 0x32] {
            append(&mut e, this.addr() + LIGHT_LIST, v);
        }
        let mut values = vec![0.25f64, -1.0];
        e.register_double(LIGHT_00B9BC50, move |_, _| Ret {
            st0: values.pop().unwrap(),
            ..Ret::default()
        });
        let sun = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(sun + 0xf8, ni_light);
        e.set(this, ShadowSceneNode::pSunLight, sun);
        for (i, v) in [0.2f32, 0.7, 0.4, 0.9, 0.1, 0.3].iter().enumerate() {
            e.mem.set_f32(ni_light + 0xc8 + 4 * i as u32, *v);
        }
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        let sum = e
            .call(
                0x00b5_b4d0,
                &args![this, 1u32, 2u32, 3u32, out, out + 4, out + 8, 9u32],
            )
            .f64();
        assert_eq!(sum, -0.75);
        // -1.0 then -0.75: never above 0.
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.f32(out + 4), 0.7);
        assert_eq!(e.mem.f32(out + 8), 0.9);
        assert_eq!(
            calls(&e, LIGHT_00B9BC50),
            vec![vec![0x31, 1, 2, 3, 9], vec![0x32, 1, 2, 3, 9]]
        );
    }

    /// A node object that is visible and has a bound of radius 1.
    fn bounded(e: &mut Engine, node: u32) {
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, 1.0);
        e.mem.set_u32(node + 0x20, bound);
    }

    #[test]
    fn lighting_walks_pass_geometries_to_the_leaf_functions() {
        let mut e = rig();
        object_doubles(&mut e);
        let (leaf, property) = geometry(&mut e, 4);
        let root = node_with(&mut e, &[leaf]);
        bounded(&mut e, root);
        e.register(CAST_00653290, |_, _| 0u32.into_ret());
        e.register(PROPERTY_FENCE_ALL_LIGHTS, |_, _| Ret::default());
        e.register(PROPERTY_REMOVE_FENCED_LIGHTS, |_, _| Ret::default());
        // Overload 2 of UpdateQueuedObjectLighting: the leaf reads the node's light list.
        let node = e.mem.alloc(0x200);
        e.call_log = Some(vec![]);
        let r = e.call(0x00b5_c490, &args![root, node, 0u32, 0u32, 0u32]);
        assert_eq!(r.u8(), 1);
        assert_eq!(calls(&e, PROPERTY_FENCE_ALL_LIGHTS), vec![vec![property]]);
        // The list overload: the leaf gets the array.
        let lights = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let r = e.call(0x00b5_c5c0, &args![root, 0u32, lights, 0u32]);
        assert_eq!(r.u8(), 1);
        assert_eq!(
            calls(&e, PROPERTY_REMOVE_FENCED_LIGHTS),
            vec![vec![property, leaf]]
        );
        // Start tests: no object 1; no bound 0; culled 0.
        assert_eq!(
            e.call(0x00b5_c490, &args![0u32, 0u32, 0u32, 0u32, 0u32])
                .u8(),
            1
        );
        e.mem.set_u32(root + 0x30, 1);
        assert_eq!(e.call(0x00b5_c5c0, &args![root, 0u32, 0u32, 0u32]).u8(), 0);
        e.mem.set_u32(root + 0x20, 0);
        assert_eq!(
            e.call(0x00b5_c490, &args![root, 0u32, 0u32, 0u32, 0u32])
                .u8(),
            0
        );
    }

    #[test]
    fn lighting_walk_follows_a_multibound_node_to_its_indexed_child() {
        let mut e = rig();
        object_doubles(&mut e);
        let root = node_with(&mut e, &[]);
        bounded(&mut e, root);
        e.register(CAST_00653290, |_, a| a[0].into_ret());
        e.register(NODE_INDEXED_CHILD_00B5AA40, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b5_c490, &args![root, 0u32, 0u32, 0u32, 0u32])
                .u8(),
            1
        );
        assert_eq!(
            calls(&e, CAST_00653290),
            vec![vec![root, MULTIBOUND_RTTI_011F5EB4]]
        );
        assert_eq!(calls(&e, NODE_INDEXED_CHILD_00B5AA40), vec![vec![root]]);
    }

    #[test]
    fn fn_00b5cff0_queues_a_light_for_removal_under_the_lock() {
        let mut e = rig();
        let this = scene(&mut e);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x4444);
        e.call(0x00b5_cff0, &args![this, slot]);
        assert!(elements(&e, this.addr() + LIGHT_QUEUE_REMOVE).is_empty());
        e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 1);
        e.call(0x00b5_cff0, &args![this, slot]);
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_REMOVE), vec![0x4444]);
        assert_eq!(e.global::<u32>(QUEUE_LOCK), 0);
        assert_eq!(e.global::<u32>(QUEUE_LOCK_COUNT), 0);
    }

    #[test]
    fn fn_00b5ddb0_queues_every_light_and_optionally_flushes() {
        let mut e = rig();
        let this = scene(&mut e);
        let light = object(&mut e, 0x250);
        append(&mut e, this.addr() + LIGHT_LIST, light);
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        e.call(0x00b5_ddb0, &args![this, 0u32, 0u32]);
        assert!(elements(&e, this.addr() + LIGHT_QUEUE_UPDATE).is_empty());
        e.call(0x00b5_ddb0, &args![this, 1u32, 1u32]);
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_UPDATE), vec![light]);
        assert!(elements(&e, this.addr() + LIGHT_QUEUE_REMOVE).is_empty());
        e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 1);
        e.call(0x00b5_ddb0, &args![this, 1u32, 0u32]);
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_REMOVE), vec![light]);
        assert_eq!(e.mem.u32(light + 4), 1);
    }

    #[test]
    fn add_object_queues_visible_objects() {
        let mut e = rig();
        let this = scene(&mut e);
        let shown = object(&mut e, 0x40);
        let culled = object(&mut e, 0x40);
        e.mem.set_u32(culled + 0x30, 1);
        e.call(0x00b5_eeb0, &args![this, culled]);
        e.call(0x00b5_eeb0, &args![this, shown]);
        assert_eq!(elements(&e, this.addr() + OBJECT_QUEUE_UPDATE), vec![shown]);
        e.call(0x00b5_d9f0, &args![this, shown, 1u32]);
        assert_eq!(
            elements(&e, this.addr() + OBJECT_QUEUE_UPDATE_ACTIVE_ONLY),
            vec![shown]
        );
        assert_eq!(e.global::<u32>(OBJECT_QUEUE_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(shown + 4), 1);
    }

    /// The destructor's callees as doubles.
    fn destructor_doubles(e: &mut Engine) {
        for f in [
            SCENE_GRAPH_RELEASE_00B4F2F0,
            RENDERER_00B6DA10,
            COMPOUND_FRUSTUM_DESTRUCT,
            NI_NODE_DESTRUCT,
            RENDERER_RELEASE_MAP_00B6D4C0,
            SIZED_FREE,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(SHADER_MANAGER_GET_ACCUMULATOR, |_, _| 0u32.into_ret());
    }

    #[test]
    fn destructor_releases_the_members_and_frees_with_bit_0() {
        let mut e = rig();
        destructor_doubles(&mut e);
        let this = scene(&mut e);
        let base = this.addr();
        e.set_global(FALLBACK_SCENE_NODE_011F91C8 + 4 * 2, base);
        e.set(this, ShadowSceneNode::cSceneGraphIndex, 2);
        let fence = object(&mut e, 0x10);
        e.mem.set_u32(base + 0xd8, fence);
        let map = object(&mut e, 0x10);
        e.set(this, ShadowSceneNode::pWindowReflectionEnvMap, map);
        e.set_global(RENDERER_011F91A8, 0x5151u32);
        e.call_log = Some(vec![]);
        let r = e.call(0x00b5_f980, &args![this, 1u32]).u32();
        assert_eq!(r, base);
        assert_eq!(e.mem.u32(base), SHADOW_SCENE_NODE_VTABLE);
        assert_eq!(calls(&e, SCENE_GRAPH_RELEASE_00B4F2F0), vec![vec![2, 0]]);
        assert_eq!(calls(&e, RENDERER_00B6DA10), vec![vec![0x5151, map]]);
        assert_eq!(calls(&e, DELETE), vec![vec![fence], vec![map]]);
        assert_eq!(e.mem.u32(base + 0xd8), 0);
        assert_eq!(
            calls(&e, COMPOUND_FRUSTUM_DESTRUCT),
            vec![vec![base + 0x138]]
        );
        assert_eq!(calls(&e, NI_NODE_DESTRUCT), vec![vec![base]]);
        assert_eq!(calls(&e, SIZED_FREE), vec![vec![base, 0x200]]);
        e.call_log = Some(vec![]);
        e.call(0x00b5_f980, &args![this, 0u32]);
        assert!(calls(&e, SIZED_FREE).is_empty());
    }

    #[test]
    fn update_dynamic_shadows_renders_each_lit_shadow_light() {
        let mut e = rig();
        let this = scene(&mut e);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CUBE_MAP_CAMERA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        e.register(OBJECT_UPDATE_00A59C60, |_, _| Ret::default());
        e.register(LIGHT_RENDER_FRUSTUM_DEPTH_MAP, |_, _| Ret::default());
        e.register(LIGHT_00B9EF30, |_, _| Ret::default());
        e.register(RENDERING_SYSTEM_00BA3130, |_, _| Ret::default());
        let culled = e.mem.alloc(0x250);
        let culled_light = e.mem.alloc(0x100);
        e.mem.set_u32(culled_light + 0x30, 1);
        e.mem.set_u32(culled + 0xf8, culled_light);
        let lit = e.mem.alloc(0x250);
        let lit_light = e.mem.alloc(0x100);
        e.mem.set_u32(lit + 0xf8, lit_light);
        for l in [culled, lit] {
            append(&mut e, this.addr() + SHADOW_LIGHT_LIST, l);
        }
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2B4, 7);
        e.call_log = Some(vec![]);
        e.call(0x00b5_b880, &args![this, 0x88u32]);
        let camera = e.get(this, ShadowSceneNode::spCubeMapCamera);
        assert_ne!(camera, 0);
        assert_eq!(e.mem.u32(camera + 4), 1);
        assert_eq!(calls(&e, LIGHT_00B9EF30), vec![vec![culled]]);
        assert_eq!(calls(&e, OBJECT_UPDATE_00A59C60)[0][0], lit_light);
        assert_eq!(
            calls(&e, LIGHT_RENDER_FRUSTUM_DEPTH_MAP),
            vec![vec![lit, 0x88, 0]]
        );
        assert_eq!(e.global::<u32>(CURRENT_SHADOW_LIGHT_011F9174), lit);
        let stages: Vec<u32> = calls(&e, RENDERING_SYSTEM_00BA3130)
            .iter()
            .map(|c| c[2])
            .collect();
        assert_eq!(stages, vec![0x12, 0x13, 0x14]);
        assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 7);
    }

    #[test]
    fn fn_00b5bbe0_tests_a_visible_multibound_against_the_occluders() {
        let mut e = rig();
        let this = scene(&mut e);
        let node = object(&mut e, 0x100);
        let holder = e.mem.alloc(0x10);
        let shape = object(&mut e, 0x10);
        let shape_vtable = e.mem.u32(shape);
        e.mem.set_u32(shape_vtable + 0x9c, 0x0f10_0200);
        e.register(0x0f10_0200, |_, _| 1u32.into_ret());
        e.mem.set_u32(holder + 0xc, shape);
        e.mem.set_u32(node + 0xac, holder);
        e.mem.set_u16(node + 0xa8, 1);
        let viewer = e.mem.alloc(0x100);
        let graph = e.mem.alloc(0x40);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        let list = e.mem.alloc(12);
        let plane = e.mem.alloc(0x50);
        e.mem.set_u32(plane + 0x40, 1);
        append(&mut e, list, plane);
        e.mem.set_u32(graph + 8, e.mem.u32(list));
        e.register(MULTIBOUND_VISIBLE_AGAINST_OCCLUDERS, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b5_bbe0, &args![this, node, 1u32, 2u32, 3u32, viewer]);
        assert_eq!(e.mem.u32(shape + 8), 1);
        assert_eq!(calls(&e, 0x0f10_0200), vec![vec![shape, viewer + 0x2c]]);
        assert_eq!(
            calls(&e, MULTIBOUND_VISIBLE_AGAINST_OCCLUDERS),
            vec![vec![node, 1, 2, 3, viewer, plane]]
        );
        // Mode 3: nothing.
        e.mem.set_u32(shape + 8, 0);
        e.mem.set_u32(viewer + 0x90, 3);
        e.call_log = Some(vec![]);
        e.call(0x00b5_bbe0, &args![this, node, 1u32, 2u32, 3u32, viewer]);
        assert!(calls(&e, 0x0f10_0200).is_empty());
    }

    /// A sun light with dimmer 1 and the given colour and direction.
    fn sun_rig(e: &mut Engine, this: Ptr<ShadowSceneNode>, colour: f32) -> u32 {
        let sun = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(sun + 0xf8, ni_light);
        e.set(this, ShadowSceneNode::pSunLight, sun);
        e.mem.set_f32(ni_light + 0xc4, 1.0);
        for i in 0..3 {
            e.mem.set_f32(ni_light + 0xd4 + 4 * i, colour);
        }
        e.mem.set_f32(ni_light + 0xf8, -2.0);
        e.set_global(LIGHT_SUM_DIVISOR_01021928, 3.0f64);
        e.set_global(SUN_LUMINANCE_LIMIT_010228A0, 0.4f64);
        e.set_global(DIRECTION_SCALE_01017B70, 1000.0f64);
        e.set_global(WEIGHT_LIMIT_01013EA4, 0.01f32);
        // Normalize in place by dividing by the largest component magnitude (test only).
        e.register(POINT_NORMALIZE_004A0C10, |e, a| {
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for i in 0..3 {
                e.mem.set_f32(a[0] + 4 * i, v[i as usize] / n);
            }
            Ret::default()
        });
        e.register(CRT_SQRT, |_, a| Ret {
            st0: f64::take(a, &mut 0).sqrt(),
            ..Ret::default()
        });
        ni_light
    }

    #[test]
    fn fn_00b5bca0_uses_a_bright_sun_s_direction() {
        let mut e = rig();
        let this = scene(&mut e);
        sun_rig(&mut e, this, 1.0);
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 10.0);
        let direction = e.mem.alloc(12);
        let out = e.mem.alloc(12);
        let r = e
            .call(0x00b5_bca0, &args![this, point, direction, out])
            .u32();
        assert_eq!(r, 1);
        assert_eq!(e.mem.f32(out), 10.0);
        assert_eq!(e.mem.f32(out + 8), 1000.0);
    }

    #[test]
    fn fn_00b5bca0_weights_the_lights_offsets() {
        let mut e = rig();
        let this = scene(&mut e);
        sun_rig(&mut e, this, 0.0);
        let light = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.mem.set_f32(ni_light + 0x8c, 4.0);
        e.mem.set_f32(ni_light + 0xc4, 1.0);
        for i in 0..3 {
            e.mem.set_f32(ni_light + 0xd4 + 4 * i, 1.0);
        }
        e.mem.set_f32(ni_light + 0xe0, 8.0);
        append(&mut e, this.addr() + LIGHT_LIST, light);
        let point = e.mem.alloc(12);
        let direction = e.mem.alloc(12);
        e.mem.set_f32(direction, 1.0);
        let out = e.mem.alloc(12);
        let r = e
            .call(0x00b5_bca0, &args![this, point, direction, out])
            .u32();
        assert_eq!(r, 1);
        // One light: distance 4 of radius 8, falloff 0.5, dot 1, luminance 1: weight 0.5,
        // its whole share: the offset itself.
        assert_eq!(e.global::<f32>(WEIGHTS_011F9838), 0.5);
        assert_eq!(e.mem.f32(out), 4.0);
        assert_eq!(e.mem.f32(out + 4), 0.0);
    }

    #[test]
    fn fn_00b5c3c0_drops_the_shadow_maps_and_empties_the_shadow_list() {
        let mut e = rig();
        let this = scene(&mut e);
        let light = e.mem.alloc(0x250);
        let map = object(&mut e, 0x10);
        e.mem.set_u32(light + 0x10c, map);
        append(&mut e, this.addr() + SHADOW_LIGHT_LIST, light);
        e.set(this, ShadowSceneNode::kShadowLightActiveFence, 5);
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_c3c0, &args![this]);
        assert_eq!(calls(&e, RENDERER_RELEASE_MAP_00B6D4C0).len(), 1);
        assert_eq!(e.mem.u32(light + 0x10c), 0);
        assert!(elements(&e, this.addr() + SHADOW_LIGHT_LIST).is_empty());
        assert_eq!(e.get(this, ShadowSceneNode::kShadowLightActiveFence), 0);
    }

    #[test]
    fn fn_00b5c6e0_appends_an_object_once() {
        let mut e = rig();
        let graph = e.mem.alloc(0x40);
        let room = object(&mut e, 0x10);
        e.call(0x00b5_c6e0, &args![graph, room]);
        e.call(0x00b5_c6e0, &args![graph, room]);
        assert_eq!(elements(&e, graph + 0x30), vec![room]);
        assert_eq!(e.mem.u32(room + 4), 1);
    }

    #[test]
    fn add_light_queues_a_new_shadow_light_for_point_lights_only() {
        let mut e = rig();
        object_doubles(&mut e);
        let this = scene(&mut e);
        e.set(this, ShadowSceneNode::pBSPortalGraph, 0x6060);
        let lamp = object(&mut e, 0x100);
        e.mem.set_u32(lamp + 0x14, POINT_LIGHT_RTTI_011F4A98);
        let other = object(&mut e, 0x100);
        e.mem.set_u32(other + 0x14, 0x1234);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(SHADOW_SCENE_LIGHT_CONSTRUCT, |e, a| {
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(vtable + 4, DELETE);
            e.mem.set_u32(a[0], vtable);
            a[0].into_ret()
        });
        e.register(SHADOW_SCENE_LIGHT_SET_LIGHT, |_, _| Ret::default());
        assert_eq!(e.call(0x00b5_c940, &args![this, other, 1u32]).u32(), 0);
        let made = e.call(0x00b5_c940, &args![this, lamp, 1u32]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u8(made + 0xfc), 1);
        assert_eq!(e.mem.u32(made + 0x240), 0x6060);
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_ADD), vec![made]);
        assert_eq!(e.global::<u32>(QUEUE_LOCK_COUNT), 0);
    }

    #[test]
    fn add_all_lights_from_node_counts_lights_and_sets_point_light_values() {
        let mut e = rig();
        object_doubles(&mut e);
        let this = scene(&mut e);
        let light_record = e.mem.alloc(8);
        let lamp_record = e.mem.alloc(8);
        e.mem.set_u32(lamp_record + 4, LIGHT_RTTI_011F4A28);
        let lamp = object(&mut e, 0x100);
        e.mem.set_u32(lamp + 0x14, lamp_record);
        let plain = object(&mut e, 0x100);
        e.mem.set_u32(plain + 0x14, light_record);
        let inner = node_with(&mut e, &[lamp]);
        let root = node_with(&mut e, &[plain, inner]);
        e.register(OBJECT_UPDATE_00A59C60, |_, _| Ret::default());
        e.register(CAST_00653290, |_, a| a[0].into_ret());
        // AddLight answers 0 for RTTIs other than its two: no shadow light is made.
        e.call_log = Some(vec![]);
        let found = e.call(0x00b5_ca70, &args![this, root, 2.5f32]).u32();
        assert_eq!(found, 1);
        assert_eq!(e.mem.u32(lamp + 0xa8), 1);
        assert_eq!(e.mem.f32(lamp + 0xe0), 2.5);
        assert_eq!(e.mem.f32(lamp + 0xe4), 0.0);
        assert_eq!(calls(&e, OBJECT_UPDATE_00A59C60)[0][0], lamp);
    }

    #[test]
    fn fn_00b5cbd0_reuses_or_queues_the_shadow_light_of_an_object() {
        let mut e = rig();
        let this = scene(&mut e);
        let existing = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(ni_light + 0x30, 3);
        e.mem.set_u32(existing + 0xf8, ni_light);
        e.mem.set_u32(existing + 0x128, 0x5000);
        append(&mut e, this.addr() + SHADOW_LIGHT_LIST, existing);
        e.register(LIGHT_00B9BB10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_cbd0, &args![this, 0x5000u32]).u32(), existing);
        assert_eq!(e.mem.u32(ni_light + 0x30), 2);
        assert_eq!(
            calls(&e, LIGHT_00B9BB10),
            vec![vec![existing, 1.0f32.to_bits(), 0]]
        );
        // Another object: a new light, queued.
        e.register(NEW_OBJECT, |_, _| 0u32.into_ret());
        e.register(LIGHT_00B9DCB0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // The allocation fails: the null light is still passed on and queued.
        e.map(0, 0x1000);
        let scene_object = object(&mut e, 0x10);
        assert_eq!(e.call(0x00b5_cbd0, &args![this, scene_object]).u32(), 0);
        assert_eq!(e.mem.u32(0x128), scene_object);
        assert_eq!(e.mem.u32(scene_object + 4), 2);
        assert_eq!(calls(&e, LIGHT_00B9DCB0), vec![vec![0, 1]]);
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_ADD), vec![0]);
    }

    #[test]
    fn fn_00b5cd00_gives_a_light_a_new_ni_light_and_lists_it() {
        let mut e = rig();
        let this = scene(&mut e);
        let light = object(&mut e, 0x250);
        e.mem.set_u8(light + 0xed, 1);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_LIGHT_CONSTRUCT_00A7D670, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0xff);
            a[0].into_ret()
        });
        e.register(SHADOW_SCENE_LIGHT_SET_LIGHT, |_, _| Ret::default());
        e.register(LIGHT_00B9DCB0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_cd00, &args![this, light]).u32(), light);
        let ni_light = calls(&e, SHADOW_SCENE_LIGHT_SET_LIGHT)[0][1];
        assert_eq!(e.mem.u32(ni_light + 0x30), 0xfe);
        assert_eq!(e.mem.u8(light + 0xfc), 1);
        assert_eq!(e.mem.u8(light + 0xed), 0);
        assert_eq!(elements(&e, this.addr() + SHADOW_LIGHT_LIST), vec![light]);
        assert_eq!(e.mem.u32(light + 4), 1);
    }

    #[test]
    fn fn_00b5cde0_moves_faded_lights_around_the_fences() {
        let mut e = rig();
        let this = scene(&mut e);
        let list = this.addr() + SHADOW_LIGHT_LIST;
        e.set_global(FADE_LOW_0101FFA0, 0.1f64);
        e.set_global(FADE_HIGH_0106B9E8, 0.9f64);
        e.set_global(FADE_TIME_011AD834, 2.0f32);
        let make = |e: &mut Engine, target: f32, elapsed: f32| {
            let light = e.mem.alloc(0x250);
            let ni_light = e.mem.alloc(0x100);
            e.mem.set_u32(light + 0xf8, ni_light);
            e.mem.set_f32(light + 0xd8, target);
            e.mem.set_f32(light + 0xdc, elapsed);
            light
        };
        let fading = append(&mut e, list, 0);
        let active = append(&mut e, list, 0);
        let off = make(&mut e, 0.0, 1.0);
        let off_item = append(&mut e, list, off);
        let steady = make(&mut e, 0.5, 5.0);
        let steady_item = append(&mut e, list, steady);
        e.set(this, ShadowSceneNode::kShadowLightFadingInFence, fading);
        e.set(this, ShadowSceneNode::kShadowLightActiveFence, active);
        e.register(LIST_00B702E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_cde0, &args![this, 0u32]);
        // Both fences existed: re-placed at the head.
        assert_eq!(
            calls(&e, LIST_00B702E0)[..2],
            [vec![list, fading, fading], vec![list, active, fading]]
        );
        // The faded-out light moved before the active fence, the steady one after the
        // fading-in fence.
        let order = elements(&e, list);
        assert_eq!(order.len(), 4);
        assert_eq!(e.mem.u32(off_item), active);
        assert_eq!(e.mem.u32(fading), steady_item);
    }

    #[test]
    fn remove_queued_light_removes_the_position_from_the_light_list() {
        let mut e = rig();
        let this = scene(&mut e);
        let light = e.mem.alloc(0x250);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, light);
        let removed = object(&mut e, 0x10);
        e.register_double(LIST_REMOVE_POS, move |e, a| {
            e.mem.set_u32(a[1], removed);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_d040, &args![this, holder, 0x3333u32]);
        let call = &calls(&e, LIST_REMOVE_POS)[0];
        assert_eq!(call[0], this.addr() + LIGHT_LIST);
        assert_eq!(call[2], 0x3333);
        assert_eq!(calls(&e, DELETE), vec![vec![removed]]);
        // A shadow caster goes to the shadow list.
        e.mem.set_u8(light + 0xec, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_d040, &args![this, holder, 0x3333u32]);
        assert_eq!(
            calls(&e, LIST_REMOVE_POS)[0][0],
            this.addr() + SHADOW_LIGHT_LIST
        );
    }

    #[test]
    fn remove_queued_light_takes_the_light_out_of_the_portal_arrays() {
        let mut e = rig();
        let this = scene(&mut e);
        let graph = e.mem.alloc(0x40);
        e.mem.set_u32(graph + 0x28, 1);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        let light = e.mem.alloc(0x250);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, light);
        let room = e.mem.alloc(0x100);
        e.mem.set_u32(room + 0xe4, 1);
        let rooms = e.mem.alloc(4);
        e.mem.set_u32(rooms, room);
        e.mem.set_u32(light + 0x214, rooms);
        e.mem.set_u32(light + 0x218, 1);
        let target = e.mem.alloc(0x100);
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(buffer + 4, light);
        e.mem.set_u32(target + 0xb4, buffer);
        e.mem.set_u32(target + 0xb8, 2);
        let portal = e.mem.alloc(0x110);
        e.mem.set_u32(portal + 0x104, target);
        let portals = e.mem.alloc(4);
        e.mem.set_u32(portals, portal);
        e.mem.set_u32(light + 0x224, portals);
        e.mem.set_u32(light + 0x228, 1);
        e.register(ARRAY_FIND_00B5A930, |_, _| 0u32.into_ret());
        e.register(ARRAY_REMOVE_00B5ADE0, |_, _| Ret::default());
        e.register(LIST_REMOVE_POS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_d040, &args![this, holder, 0u32]);
        assert_eq!(
            calls(&e, ARRAY_FIND_00B5A930),
            vec![vec![room + 0xdc, holder, 0, ARRAY_COMPARE_00B5A980]]
        );
        assert_eq!(
            calls(&e, ARRAY_REMOVE_00B5ADE0),
            vec![vec![room + 0xdc, 0, 1], vec![target + 0xb0, 1, 1]]
        );
    }

    #[test]
    fn fn_00b5d180_queues_every_light_for_removal_and_clears_the_shadow_list() {
        let mut e = rig();
        let this = scene(&mut e);
        e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 1);
        let light = object(&mut e, 0x250);
        append(&mut e, this.addr() + LIGHT_LIST, light);
        append(&mut e, this.addr() + SHADOW_LIGHT_LIST, 0);
        e.set(this, ShadowSceneNode::kShadowLightFadingInFence, 9);
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        e.call(0x00b5_d180, &args![this]);
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_REMOVE), vec![light]);
        assert!(elements(&e, this.addr() + SHADOW_LIGHT_LIST).is_empty());
        assert_eq!(e.get(this, ShadowSceneNode::kShadowLightFadingInFence), 0);
        assert_eq!(e.mem.u32(light + 4), 1);
    }

    #[test]
    fn fn_00b5d270_queues_a_light_for_update() {
        let mut e = rig();
        let this = scene(&mut e);
        let light = object(&mut e, 0x250);
        e.call(0x00b5_d270, &args![this, 0u32]);
        e.call(0x00b5_d270, &args![this, light]);
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_UPDATE), vec![light]);
    }

    /// The light update's callees as doubles.
    fn light_update_doubles(e: &mut Engine) {
        e.set_global(DIMMER_LIMIT_0103C7C8, 0.05f64);
        for f in [
            LIGHT_00B9EF30,
            LIGHT_00B9FBA0,
            LIGHT_00BA02B0,
            LIGHT_ADD_OBJECT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_00b5d300_without_a_portal_graph_adds_the_child_nodes() {
        let mut e = rig();
        object_doubles(&mut e);
        light_update_doubles(&mut e);
        let child = node_with(&mut e, &[]);
        let leaf = object(&mut e, 0x100);
        let this: Ptr<ShadowSceneNode> = Ptr::new(node_with(&mut e, &[child, leaf]));
        let light = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_f32(ni_light + 0xc4, 1.0);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.call_log = Some(vec![]);
        e.call(0x00b5_d300, &args![this, light]);
        assert_eq!(calls(&e, LIGHT_00B9FBA0), vec![vec![light]]);
        assert_eq!(calls(&e, LIGHT_ADD_OBJECT), vec![vec![light, child]]);
        assert_eq!(calls(&e, LIGHT_00BA02B0), vec![vec![light]]);
        // A dim light gets 00b9ef30 only.
        e.mem.set_f32(ni_light + 0xc4, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_d300, &args![this, light]);
        assert_eq!(calls(&e, LIGHT_00B9EF30), vec![vec![light]]);
        assert!(calls(&e, LIGHT_00B9FBA0).is_empty());
    }

    #[test]
    fn fn_00b5d300_links_a_light_with_the_rooms_it_is_in() {
        let mut e = rig();
        object_doubles(&mut e);
        light_update_doubles(&mut e);
        let this = scene(&mut e);
        let graph = e.mem.alloc(0x80);
        e.mem.set_u32(graph + 0x28, 1);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        let room = object(&mut e, 0x100);
        let room_vtable = e.mem.u32(room);
        e.mem.set_u32(room_vtable + 0x110, 0x0f10_0300);
        e.register(0x0f10_0300, |_, _| 0u32.into_ret());
        let shape = object(&mut e, 0x10);
        let shape_vtable = e.mem.u32(shape);
        e.mem.set_u32(shape_vtable + 0xb4, 0x0f10_0304);
        e.register(0x0f10_0304, |_, _| 1u32.into_ret());
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 0xc, shape);
        e.mem.set_u32(room + 0xac, holder);
        let rooms = e.mem.alloc(12);
        append(&mut e, rooms, room);
        e.mem.set_u32(graph + 0x20, e.mem.u32(rooms));
        let light = object(&mut e, 0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_f32(ni_light + 0xc4, 1.0);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.register(ARRAY_FIND_00B5A930, |_, _| u32::MAX.into_ret());
        e.register(ARRAY_ALLOCATE_SLOT, |e, a| e.mem.alloc(4 * a[1]).into_ret());
        for array in [0x210u32, 0x220, 0x230] {
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable + 4, ARRAY_ALLOCATE_SLOT);
            e.mem.set_u32(light + array, vtable);
        }
        let room_array_vtable = e.mem.alloc(8);
        e.mem.set_u32(room_array_vtable + 4, ARRAY_ALLOCATE_SLOT);
        e.mem.set_u32(room + 0xdc, room_array_vtable);
        e.register(PORTAL_GRAPH_ALWAYS_RENDER_COUNT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b5_d300, &args![this, light]);
        assert_eq!(e.mem.u32(light + 0x240), graph);
        // The room keeps the light, the light the room.
        let room_buffer = e.mem.u32(room + 0xe0);
        assert_eq!(e.mem.u32(room_buffer), light);
        let rooms_of_light = e.mem.u32(light + 0x214);
        assert_eq!(e.mem.u32(rooms_of_light), room);
        assert_eq!(calls(&e, LIGHT_ADD_OBJECT), vec![vec![light, room]]);
    }

    /// The portal temporary of 00b5d300 does not overlap the bound: a room the light has
    /// left walks its portals (writing the portal slot), and the next room still gets the
    /// light's radius at bound +0xc.
    #[test]
    fn fn_00b5d300_keeps_the_bound_radius_after_walking_a_rooms_portals() {
        let mut e = rig();
        object_doubles(&mut e);
        light_update_doubles(&mut e);
        let this = scene(&mut e);
        let graph = e.mem.alloc(0x80);
        e.mem.set_u32(graph + 0x28, 1);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        // Every room bound test records the radius it was handed.
        let radii = e.mem.alloc(0x10);
        e.register_double(0x0f10_0300, move |e, a| {
            let count = e.mem.u32(radii);
            let radius = e.mem.u32(a[1] + 0xc);
            e.mem.set_u32(radii + 4 + 4 * count, radius);
            e.mem.set_u32(radii, count + 1);
            0u32.into_ret()
        });
        e.register(0x0f10_0304, |_, _| 0u32.into_ret());
        let shape = object(&mut e, 0x10);
        let shape_vtable = e.mem.u32(shape);
        e.mem.set_u32(shape_vtable + 0xb4, 0x0f10_0304);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 0xc, shape);
        let rooms = e.mem.alloc(12);
        let portal = e.mem.alloc(0x110);
        for with_portal in [true, false] {
            let room = object(&mut e, 0x100);
            let room_vtable = e.mem.u32(room);
            e.mem.set_u32(room_vtable + 0x110, 0x0f10_0300);
            e.mem.set_u32(room + 0xac, holder);
            if with_portal {
                let portals = e.mem.alloc(12);
                append(&mut e, portals, portal);
                e.mem.set_u32(room + 0xb4, e.mem.u32(portals));
            }
            append(&mut e, rooms, room);
        }
        e.mem.set_u32(graph + 0x20, e.mem.u32(rooms));
        let light = object(&mut e, 0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_f32(ni_light + 0xc4, 1.0);
        e.mem.set_f32(ni_light + 0xe0, 5.0);
        e.mem.set_u32(light + 0xf8, ni_light);
        // Every find answers the first slot: the rooms list the light (so it is removed)
        // and the portal is already in the light's portal array.
        e.register(ARRAY_FIND_00B5A930, |_, _| 0u32.into_ret());
        e.register(ARRAY_REMOVE_00B5AEA0, |_, _| Ret::default());
        e.register(PORTAL_GRAPH_ALWAYS_RENDER_COUNT, |_, _| 0u32.into_ret());
        e.call(0x00b5_d300, &args![this, light]);
        assert_eq!(e.mem.u32(radii), 2);
        assert_eq!(e.mem.f32(radii + 4), 5.0);
        assert_eq!(e.mem.f32(radii + 8), 5.0);
    }

    /// The array slot allocation double (`BSSimpleArray` virtual slot +4).
    const ARRAY_ALLOCATE_SLOT: u32 = 0x0f10_0400;

    #[test]
    fn fn_00b5d930_follows_a_moved_dynamic_point_light() {
        let mut e = rig();
        light_update_doubles(&mut e);
        let this = scene(&mut e);
        let light = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.mem.set_u8(light + 0xf4, 1);
        e.mem.set_u8(light + 0xfc, 1);
        e.mem.set_f32(ni_light + 0x8c, 3.0);
        e.register(POINT_COMPARE_00439090, |_, _| 1u32.into_ret());
        // The update itself (00b5d300) answers early for a dim light.
        e.call_log = Some(vec![]);
        e.call(0x00b5_d930, &args![this, light]);
        assert_eq!(e.mem.f32(light + 0x100), 3.0);
        assert_eq!(calls(&e, LIGHT_00B9EF30), vec![vec![light]]);
        // A static light: nothing.
        e.mem.set_u8(light + 0xfc, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_d930, &args![this, light]);
        assert!(calls(&e, POINT_COMPARE_00439090).is_empty());
    }

    #[test]
    fn queued_lighting_start_collects_and_sorts_the_lights_in_range() {
        let mut e = rig();
        object_doubles(&mut e);
        let node = scene(&mut e);
        let (geometry, _) = geometry(&mut e, 2);
        let light = e.mem.alloc(0x250);
        append(&mut e, node.addr() + LIGHT_LIST, light);
        e.set_global(SCRAP_ALIGNMENT_010A2720, 4u32);
        e.register(GET_THREAD_SCRAP_HEAP, |_, _| 0x7000u32.into_ret());
        e.register(SCRAP_HEAP_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SCRAP_HEAP_DEALLOCATE, |_, _| Ret::default());
        e.register(LIGHT_BOUND_WITHIN_RADIUS, |_, _| 1u32.into_ret());
        e.register(LIGHT_GET_LUMINANCE_TO_BOUND, |_, _| Ret::default());
        e.register(CRT_QSORT, |_, _| Ret::default());
        e.register(PROPERTY_FENCE_ALL_LIGHTS, |_, _| Ret::default());
        e.register(PROPERTY_REMOVE_FENCED_LIGHTS, |_, _| Ret::default());
        e.register(LIGHT_ADD_TRI_GEOM, |_, _| Ret::default());
        // A geometry is not a node: the lighting walk answers 0 after the collection.
        bounded(&mut e, geometry);
        e.call_log = Some(vec![]);
        let r = e.call(0x00b5_dac0, &args![geometry, node, 0u32, 1u32]).u8();
        assert_eq!(r, 1);
        let allocation = calls(&e, SCRAP_HEAP_ALLOCATE)[0].clone();
        assert_eq!(allocation[1], 8);
        let array = calls(&e, CRT_QSORT)[0][0];
        assert_eq!(calls(&e, CRT_QSORT), vec![vec![array, 1, 4, 0x00b5_aa70]]);
        assert_eq!(e.mem.u32(array), light);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(calls(&e, SCRAP_HEAP_DEALLOCATE), vec![vec![0x7000, array]]);
        // Without the collect flag: the geometry goes straight to UpdateQueuedObjectLighting.
        e.call_log = Some(vec![]);
        e.call(0x00b5_dac0, &args![geometry, node, 0u32, 0u32]);
        assert_eq!(calls(&e, LIGHT_ADD_TRI_GEOM), vec![vec![light, geometry]]);
    }

    #[test]
    fn fn_00b5ddf0_switches_the_portal_graph_and_frees_the_old_rooms() {
        let mut e = rig();
        let this = scene(&mut e);
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        // The same graph: just stored.
        e.call(0x00b5_ddf0, &args![this, 0u32]);
        let old = e.mem.alloc(0x40);
        e.set(this, ShadowSceneNode::pBSPortalGraph, old);
        let rooms = e.mem.alloc(12);
        let room = object(&mut e, 0x100);
        let room_array_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(room_array_vtable + 8, SLOT_FREE);
        e.register(SLOT_FREE, |_, _| Ret::default());
        e.mem.set_u32(room + 0xdc, room_array_vtable);
        e.mem.set_u32(room + 0xe0, 0x4444);
        append(&mut e, rooms, room);
        e.mem.set_u32(old + 0x20, e.mem.u32(rooms));
        append(&mut e, this.addr() + LIGHT_QUEUE_ADD, 5);
        e.call_log = Some(vec![]);
        e.call(0x00b5_ddf0, &args![this, 0x9999u32]);
        assert_eq!(calls(&e, SLOT_FREE), vec![vec![room + 0xdc, 0x4444]]);
        assert_eq!(e.mem.u32(room + 0xe0), 0);
        assert!(elements(&e, this.addr() + LIGHT_QUEUE_ADD).is_empty());
        assert_eq!(e.get(this, ShadowSceneNode::bAllowLightRemoveQueues), 1);
        assert_eq!(e.get(this, ShadowSceneNode::pBSPortalGraph), 0x9999);
        assert_eq!(e.global::<u32>(QUEUE_LOCK_COUNT), 0);
        assert_eq!(e.global::<u32>(OBJECT_QUEUE_LOCK_COUNT), 0);
    }

    #[test]
    fn simple_array_remove_at_shifts_or_shrinks() {
        let mut e = rig();
        e.register(ARRAY_MOVE_00CB9930, |_, _| Ret::default());
        e.register(ARRAY_ALLOCATE_SLOT, |_, _| 0x8800u32.into_ret());
        e.register(SLOT_FREE, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, ARRAY_ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, SLOT_FREE);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, vtable);
        e.mem.set_u32(array + 4, 0x1000);
        e.mem.set_u32(array + 8, 3);
        e.mem.set_u32(array + 0xc, 8);
        e.call_log = Some(vec![]);
        simple_array_remove_at(&mut e, array, 1, 3);
        assert_eq!(
            calls(&e, ARRAY_MOVE_00CB9930),
            vec![vec![array, 0x1004, 0x1008, 1]]
        );
        assert_eq!(e.mem.u32(array + 8), 2);
        // Size 2 of capacity 8 is sparse: shrink to 4.
        e.call_log = Some(vec![]);
        simple_array_remove_at(&mut e, array, 0, 2);
        assert_eq!(
            calls(&e, ARRAY_MOVE_00CB9930),
            vec![
                vec![array, 0x8800, 0x1000, 0],
                vec![array, 0x8800, 0x1004, 1]
            ]
        );
        assert_eq!(e.mem.u32(array + 4), 0x8800);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        assert_eq!(e.mem.u32(array + 8), 1);
    }

    #[test]
    fn pre_on_visible_tests_held_lights_and_publishes_the_camera() {
        let mut e = rig();
        let this = scene(&mut e);
        e.set(this, ShadowSceneNode::cSceneGraphIndex, 3);
        let culler = e.mem.alloc(0x100);
        let camera = e.mem.alloc(0x100);
        e.mem.set_f32(camera + 0x8c, 7.0);
        e.mem.set_u32(culler + 0xc, camera);
        let light = object(&mut e, 0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(ni_light + 4, 2);
        e.mem.set_u32(light + 0xf8, ni_light);
        e.mem.set_u8(light + 0xfc, 1);
        append(&mut e, this.addr() + LIGHT_LIST, light);
        e.register(CULLER_00C4F360, |_, _| Ret::default());
        e.register(LIGHT_TEST_FRUSTUM_CULL, |_, _| Ret::default());
        e.register(NI_NODE_ON_VISIBLE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_e870, &args![this, culler]);
        assert_eq!(
            calls(&e, LIGHT_TEST_FRUSTUM_CULL),
            vec![vec![light, culler]]
        );
        assert_eq!(elements(&e, this.addr() + LIGHT_QUEUE_UPDATE), vec![light]);
        assert_eq!(e.global::<u8>(SCENE_GRAPH_INDEX_011F91C4), 3);
        assert_eq!(e.global::<u32>(CAMERA_011F917C), camera);
        assert_eq!(
            calls(&e, NI_NODE_ON_VISIBLE),
            vec![vec![this.addr(), culler]]
        );

        // With the switch word set: the camera values and the blend are published.
        e.set_global(SWITCH_011F91C0, 1u32);
        e.set_global(BLEND_TARGET_01200504, 3.0f32);
        e.set_global(BLEND_BASE_01200500, 1.0f32);
        e.set_global(BLEND_FACTOR_011AD824, 0.5f32);
        e.set_global(SAVED_MODE_011AD868, u32::MAX);
        e.set_global(MODE_FLAG_011F91A7, 1u8);
        e.set_global(MODE_011F91D8, 9u32);
        e.call(0x00b5_e870, &args![this, culler]);
        assert_eq!(e.global::<f32>(CAMERA_VALUES_011FA2A0), 7.0);
        assert_eq!(e.global::<f32>(BLEND_RESULT_01200558), 2.0);
        assert_eq!(e.global::<u32>(SAVED_MODE_011AD868), 9);
        assert_eq!(e.global::<u32>(MODE_011F91D8), 4);
    }

    #[test]
    fn pre_on_visible_removes_a_light_whose_ni_light_is_held_only_here() {
        let mut e = rig();
        let this = scene(&mut e);
        let culler = e.mem.alloc(0x100);
        let light = object(&mut e, 0x250);
        e.mem.set_u32(light + 0x240, 0x1111);
        append(&mut e, this.addr() + LIGHT_LIST, light);
        e.register(CULLER_00C4F360, |_, _| Ret::default());
        e.register(NI_NODE_ON_VISIBLE, |_, _| Ret::default());
        e.register(LIST_REMOVE_POS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_e870, &args![this, culler]);
        let removal = &calls(&e, LIST_REMOVE_POS)[0];
        assert_eq!(removal[0], this.addr() + LIGHT_LIST);
        assert_eq!(e.mem.u32(light + 4), 1);
    }

    #[test]
    fn fn_00b5eca0_updates_a_known_light_or_lists_a_new_one() {
        let mut e = rig();
        light_update_doubles(&mut e);
        let this = scene(&mut e);
        let ni_light = e.mem.alloc(0x100);
        let listed = e.mem.alloc(0x250);
        e.mem.set_u32(listed + 0xf8, ni_light);
        append(&mut e, this.addr() + LIGHT_LIST, listed);
        let fresh = object(&mut e, 0x250);
        e.mem.set_u32(fresh + 0xf8, ni_light);
        e.mem.set_u8(fresh + 0xfc, 1);
        assert_eq!(e.call(0x00b5_eca0, &args![this, fresh]).u32(), listed);
        assert_eq!(e.mem.u8(listed + 0xfc), 1);
        // A light of another NiLight casting shadows: listed, with a map from the renderer.
        let other_ni = e.mem.alloc(0x100);
        let caster = object(&mut e, 0x250);
        e.mem.set_u32(caster + 0xf8, other_ni);
        e.mem.set_u8(caster + 0xec, 1);
        let map = object(&mut e, 0x10);
        e.register_double(RENDERER_00B6D3F0, move |_, _| map.into_ret());
        e.map(0, 0x1000);
        assert_eq!(e.call(0x00b5_eca0, &args![this, caster]).u32(), 0);
        assert_eq!(elements(&e, this.addr() + LIGHT_LIST), vec![listed, caster]);
        assert_eq!(e.mem.u32(caster + 0x10c), map);
        assert_eq!(e.mem.u32(map + 4), 2);
    }

    #[test]
    fn removal_queues_find_lights_by_shadow_scene_and_by_ni_light() {
        let mut e = rig();
        let this = scene(&mut e);
        e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 1);
        let shadow = object(&mut e, 0x250);
        e.mem.set_u32(shadow + 0x128, 0x7000);
        append(&mut e, this.addr() + SHADOW_LIGHT_LIST, shadow);
        let plain = object(&mut e, 0x250);
        e.mem.set_u32(plain + 0xf8, 0x7100);
        append(&mut e, this.addr() + LIGHT_LIST, plain);
        e.call(0x00b5_edf0, &args![this, 0x7000u32]);
        e.call(0x00b5_eed0, &args![this, 0x7100u32]);
        e.call(0x00b5_eed0, &args![this, 0x7200u32]);
        assert_eq!(
            elements(&e, this.addr() + LIGHT_QUEUE_REMOVE),
            vec![shadow, plain]
        );
        assert_eq!(e.mem.u32(shadow + 4), 1);
        assert_eq!(e.mem.u32(plain + 4), 1);
    }

    #[test]
    fn remove_queued_light_ov2_removes_by_value_or_by_position() {
        let mut e = rig();
        let this = scene(&mut e);
        let graph = e.mem.alloc(0x40);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        let light = e.mem.alloc(0x250);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, light);
        e.register(LIST_REMOVE, |_, _| Ret::default());
        // The position is the address of a slot holding the item: record the item.
        e.register(LIST_REMOVE_POS, |e, a| e.mem.u32(a[2]).into_ret());
        e.call_log = Some(vec![]);
        // Another graph: removed by value.
        e.call(0x00b5_ef90, &args![this, holder]);
        let call = &calls(&e, LIST_REMOVE)[0];
        assert_eq!(call[0], this.addr() + LIGHT_LIST);
        assert_eq!(call[2], holder);
        // This graph: found in the list and removed at its item.
        e.mem.set_u32(light + 0x240, graph);
        let item = append(&mut e, this.addr() + LIGHT_LIST, light);
        let seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let record = seen.clone();
        e.register_double(LIST_REMOVE_POS, move |e, a| {
            record.set(e.mem.u32(a[2]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_ef90, &args![this, holder]);
        assert_eq!(calls(&e, LIST_REMOVE_POS).len(), 1);
        assert_eq!(seen.get(), item);
        assert!(calls(&e, LIST_REMOVE).is_empty());
    }

    #[test]
    fn fn_00b5f170_walks_from_the_unbound_space_through_a_portal() {
        let mut e = rig();
        let this = scene(&mut e);
        let graph = e.mem.alloc(0x80);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        let room = object(&mut e, 0x100);
        let holder = e.mem.alloc(0x10);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 0xc, shape);
        e.mem.set_u32(room + 0xac, holder);
        let portal = e.mem.alloc(0x110);
        e.mem.set_u32(portal + 0xfc, room);
        let portals = e.mem.alloc(12);
        append(&mut e, portals, portal);
        e.mem.set_u32(graph + 0x14, e.mem.u32(portals));
        let camera = e.mem.alloc(0x100);
        for f in [
            FRUSTUM_SET_CAMERA,
            FRUSTUM_00C47A00,
            FRUSTUM_00C47AE0,
            FRUSTUM_00C48B90,
            FRUSTUM_00C49850,
            FRUSTUM_ADD_OCCLUDER,
            FRUSTUM_ADD_PORTAL,
            FRUSTUM_MERGE,
            ROOM_SET_FRUSTUM_0045C4E0,
            LIST_DESTRUCT_00B71300,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(FRUSTUM_TEST, |_, _| 1u32.into_ret());
        e.register(PORTAL_00B5AA00, |_, _| 0u32.into_ret());
        e.register(ITEM_NEW_0043A010, |e, _| e.mem.alloc(12).into_ret());
        e.register(ITEM_REFILL_0043A080, |e, _| {
            let item = e.mem.alloc(12);
            e.set_global(ITEM_FREE_LIST, item);
            Ret::default()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(COMPOUND_FRUSTUM_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x28, 0);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_f170, &args![this, 0u32, camera, 0u32, 1u32]);
        // The unbound space and then the room were visited.
        assert_eq!(elements(&e, graph + 0x30), vec![0, room]);
        assert_eq!(e.mem.u32(portal + 0x40), 1);
        assert_eq!(e.mem.u32(shape + 8), 1);
        assert_eq!(e.get(this, ShadowSceneNode::bVisibleUnboundSpace), 1);
        let built = calls(&e, ROOM_SET_FRUSTUM_0045C4E0);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], room);
        assert_eq!(
            calls(&e, FRUSTUM_ADD_PORTAL)[0],
            vec![built[0][1], portal, 0]
        );
        assert_eq!(calls(&e, LIST_DESTRUCT_00B71300).len(), 2);
        assert_eq!(e.global::<u32>(ITEM_LOCK_COUNT), 0);
    }

    #[test]
    fn on_visible_without_a_portal_graph_visits_the_node() {
        let mut e = rig();
        let this = scene(&mut e);
        let culler = e.mem.alloc(0x100);
        let accumulator = object(&mut e, 0x200);
        e.register_double(SHADER_MANAGER_GET_ACCUMULATOR, move |_, _| {
            accumulator.into_ret()
        });
        e.register(CULLER_00C4F360, |_, _| Ret::default());
        e.register(NI_NODE_ON_VISIBLE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_f9b0, &args![this, culler]);
        assert_eq!(calls(&e, NI_NODE_ON_VISIBLE).len(), 2);
        // Held once and released once (the count was 2 after the first hold).
        assert_eq!(e.mem.u32(accumulator + 4), 1);
    }

    #[test]
    fn on_visible_with_a_portal_graph_restores_the_always_rendered_flags() {
        let mut e = rig();
        let this = scene(&mut e);
        let graph = e.mem.alloc(0x80);
        e.set(this, ShadowSceneNode::pBSPortalGraph, graph);
        let culler = e.mem.alloc(0x100);
        let accumulator = object(&mut e, 0x200);
        e.register_double(SHADER_MANAGER_GET_ACCUMULATOR, move |_, _| {
            accumulator.into_ret()
        });
        e.register(CULLER_00C4F360, |_, _| Ret::default());
        e.register(NI_NODE_ON_VISIBLE, |_, _| Ret::default());
        let shown = e.mem.alloc(0x40);
        let hidden = e.mem.alloc(0x40);
        e.mem.set_u32(hidden + 0x30, 1);
        e.register(PORTAL_GRAPH_ALWAYS_RENDER_COUNT, |_, _| 2u32.into_ret());
        e.register_double(PORTAL_GRAPH_ALWAYS_RENDER_AT, move |_, a| {
            [shown, hidden][a[1] as usize].into_ret()
        });
        e.register(GET_THREAD_SCRAP_HEAP, |_, _| 0x7000u32.into_ret());
        e.register(SCRAP_HEAP_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SCRAP_HEAP_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(NI_AV_OBJECT_CULL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_f9b0, &args![this, culler]);
        assert_eq!(calls(&e, NI_AV_OBJECT_CULL), vec![vec![shown, culler]]);
        assert_eq!(e.mem.u32(shown + 0x30), 0);
        assert_eq!(e.mem.u32(hidden + 0x30), 1);
        assert_eq!(calls(&e, SCRAP_HEAP_DEALLOCATE).len(), 1);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    const TARGET: u32 = 0x03b0_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A pointer list at `list` holding `elements` in order (items of three words).
    fn fill_list(e: &mut Engine, list: u32, elements: &[u32]) {
        let mut next = 0u32;
        let mut items = vec![];
        for element in elements.iter().rev() {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 8, *element);
            next = item;
            items.push(item);
        }
        e.mem.set_u32(list, next);
        e.mem.set_u32(list + 4, items.first().copied().unwrap_or(0));
        e.mem.set_u32(list + 8, elements.len() as u32);
    }

    /// An array (vtable, buffer, count, capacity) holding `values`, its capacity `capacity`.
    fn array(e: &mut Engine, values: &[u32], capacity: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(4 * capacity.max(1));
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *value);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, values.len() as u32);
        e.mem.set_u32(array + 0xc, capacity);
        array
    }

    #[test]
    fn array_find_answers_the_first_match_from_the_start() {
        let mut e = Engine::new();
        // Matches when the element equals the value.
        e.register(TARGET, |e, a| u32::from(e.mem.u32(a[0]) == a[1]).into_ret());
        let this = array(&mut e, &[5, 7, 5, 9], 4);
        assert_eq!(
            e.call(0x00b5_a930, &args![this, 5u32, 0u32, TARGET]).u32(),
            0
        );
        assert_eq!(
            e.call(0x00b5_a930, &args![this, 5u32, 1u32, TARGET]).u32(),
            2
        );
        assert_eq!(
            e.call(0x00b5_a930, &args![this, 8u32, 0u32, TARGET]).u32(),
            0xffff_ffff
        );
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b5_a930, &args![this, 5u32, 0u32, TARGET]).u32(),
            0xffff_ffff
        );
        assert!(calls_to(&e, TARGET).is_empty());
    }

    #[test]
    fn indexed_child_checks_the_index_range() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0xb4);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, 0x1111);
        e.mem.set_u32(children + 4, 0x2222);
        e.mem.set_u32(node + NODE_CHILDREN_BASE, children);
        e.mem.set_u16(node + NODE_CHILDREN_SIZE, 2);
        for (index, expected) in [(1i32, 0x2222u32), (0, 0x1111), (2, 0), (-1, 0)] {
            e.mem.set_i32(node + NODE_CHILD_INDEX, index);
            assert_eq!(e.call(0x00b5_aa40, &args![node]).u32(), expected);
        }
    }

    #[test]
    fn light_count_skips_null_and_marked_lights() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        let lights: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x120)).collect();
        e.mem.set_u16(lights[1] + LIGHT_WORD_110, 0xff);
        e.mem.set_u16(lights[2] + LIGHT_WORD_110, 0x1ff);
        fill_list(
            &mut e,
            this + LIGHT_LIST,
            &[lights[0], 0, lights[1], lights[2]],
        );
        assert_eq!(e.call(0x00b5_ac60, &args![this]).u32(), 2);
    }

    #[test]
    fn remove_at_moves_down_or_shrinks_the_buffer() {
        const ALLOCATE: u32 = 0x03b1_0000;
        const FREE: u32 = 0x03b2_0000;
        let mut e = Engine::new();
        e.register(ARRAY_MOVE_00CB9930, |e, a| {
            for i in 0..a[3] {
                let value = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, value);
            }
            Ret::default()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(4 * a[1]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, ALLOCATE);
        e.mem.set_u32(vtable + 8, FREE);
        // Plain removal: count 3 of 8, no shrink flag.
        let this = array(&mut e, &[1, 2, 3], 8);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x00b5_ade0, &args![this, 0u32, 0u32]);
        let buffer = e.mem.u32(this + 4);
        assert_eq!((e.mem.u32(buffer), e.mem.u32(buffer + 4)), (2, 3));
        assert_eq!(e.mem.u32(this + 8), 2);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        // Shrinking: count 2 of 8 is at most a quarter; the new buffer holds 4.
        e.call(0x00b5_ade0, &args![this, 0u32, 1u32]);
        let fresh = e.mem.u32(this + 4);
        assert_ne!(fresh, buffer);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![this, 4]]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this, buffer]]);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (1, 4));
        // The second move takes `count - 1` elements from after the index.
        assert_eq!(calls_to(&e, ARRAY_MOVE_00CB9930)[2][3], 1);
        assert_eq!(e.mem.u32(fresh), 3);
        // A large capacity shrinks by 0x400 only when the count is at most capacity - 0x800.
        let big = array(&mut e, &[1, 2, 3], 0x1000);
        e.mem.set_u32(big, vtable);
        e.mem.set_u32(big + 8, 0x801);
        e.call(0x00b5_ade0, &args![big, 0u32, 1u32]);
        assert_eq!(e.mem.u32(big + 0xc), 0x1000);
        e.call(0x00b5_ade0, &args![big, 0u32, 1u32]);
        assert_eq!(e.mem.u32(big + 0xc), 0xc00);
    }

    #[test]
    fn remove_range_moves_the_tail_into_the_gap() {
        let mut e = Engine::new();
        e.register(ARRAY_MOVE_00CB9930, |e, a| {
            for i in 0..a[3] {
                let value = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, value);
            }
            Ret::default()
        });
        let this = array(&mut e, &[1, 2, 3, 4, 5, 6], 6);
        e.call_log = Some(vec![]);
        e.call(0x00b5_aea0, &args![this, 1u32, 2u32]);
        let buffer = e.mem.u32(this + 4);
        let values: Vec<u32> = (0..4).map(|i| e.mem.u32(buffer + 4 * i)).collect();
        assert_eq!(values, vec![1, 5, 6, 4]);
        assert_eq!(e.mem.u32(this + 8), 4);
        // Fewer elements after the gap than removed: only those move.
        e.call(0x00b5_aea0, &args![this, 1u32, 2u32]);
        assert_eq!(calls_to(&e, ARRAY_MOVE_00CB9930)[1][3], 1);
        assert_eq!(e.mem.u32(this + 8), 2);
    }

    #[test]
    fn clear_empties_and_frees_on_request() {
        const FREE: u32 = 0x03b3_0000;
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, FREE);
        let this = array(&mut e, &[1, 2], 4);
        e.mem.set_u32(this, vtable);
        let buffer = e.mem.u32(this + 4);
        e.call_log = Some(vec![]);
        e.call(0x00b5_af80, &args![this, 0u32]);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 8)), (buffer, 0));
        e.mem.set_u32(this + 8, 2);
        e.call(0x00b5_af80, &args![this, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this, buffer]]);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0, 0, 0)
        );
        // Without a buffer the count stays.
        e.mem.set_u32(this + 8, 5);
        e.call(0x00b5_af80, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 8), 5);
    }

    #[test]
    fn release_shadow_maps_gives_each_map_back_and_clears_it() {
        let mut e = Engine::new();
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        e.register(RENDERER_00B6D130, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let left = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], left);
            left.into_ret()
        });
        e.register(TARGET, |_, _| Ret::default());
        e.mem.map(RENDERER_011F91A8, 4);
        e.mem.set_u32(RENDERER_011F91A8, 0x5555);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, TARGET);
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(map + 4, 1);
        let with_map = e.mem.alloc(0x120);
        e.mem.set_u32(with_map + LIGHT_DYNAMIC_SHADOW_MAP, map);
        let without = e.mem.alloc(0x120);
        let this = e.mem.alloc(0x200);
        fill_list(&mut e, this + SHADOW_LIGHT_LIST, &[with_map, 0, without]);
        e.call_log = Some(vec![]);
        e.call(0x00b5_b1f0, &args![this]);
        assert_eq!(
            calls_to(&e, RENDERER_RELEASE_MAP_00B6D4C0),
            vec![vec![0x5555, map], vec![0x5555, 0]]
        );
        assert_eq!(calls_to(&e, TARGET), vec![vec![map]]);
        assert_eq!(e.mem.u32(with_map + LIGHT_DYNAMIC_SHADOW_MAP), 0);
        assert_eq!(calls_to(&e, RENDERER_00B6D130), vec![vec![0x5555]]);
    }

    #[test]
    fn light_sum_counts_new_maxima_and_blends_the_sun() {
        let mut e = Engine::new();
        e.mem.map(QUARTER_010290B0, 8);
        e.mem.set_u64(QUARTER_010290B0, 0.25f64.to_bits());
        e.mem.map(THREE_QUARTERS_0101DE30, 8);
        e.mem.set_u64(THREE_QUARTERS_0101DE30, 0.75f64.to_bits());
        // The lights answer 2, -1 and 3: sums 2, 1, 4; new maxima after the first and third.
        e.register(LIGHT_00B9BDF0, |e, a| {
            assert_eq!(a[1..], [1.0f32.to_bits(), 0, 0, 0, 0, 0, 0x99]);
            Ret {
                st0: f64::from(e.mem.f32(a[0])),
                ..Ret::default()
            }
        });
        // The normalisation leaves the vectors as they are (both unit already).
        e.register(POINT_NORMALIZE_004A0C10, |_, _| Ret::default());
        let this = e.mem.alloc(0x200);
        let lights: Vec<u32> = [2.0f32, -1.0, 3.0]
            .iter()
            .map(|v| {
                let light = e.mem.alloc(0x120);
                e.mem.set_f32(light, *v);
                light
            })
            .collect();
        fill_list(
            &mut e,
            this + LIGHT_LIST,
            &[lights[0], 0, lights[1], lights[2]],
        );
        let ni_light = e.mem.alloc(0x100);
        for (offset, value) in [
            (0xc8u32, 0.5f32),
            (0xcc, 2.0),
            (0xd0, 1.0),
            (0xd4, 4.0),
            (0xd8, 1.0),
            (0xdc, 3.0),
            (0xf0, -1.0),
            (0xf4, 0.0),
            (0xf8, 0.0),
        ] {
            e.mem.set_f32(ni_light + offset, value);
        }
        let sun = e.mem.alloc(0x100);
        e.mem.set_u32(sun + LIGHT_SP_LIGHT, ni_light);
        e.mem.set_u32(this + 0xe0, sun);
        let outputs = e.mem.alloc(12);
        let one = 1.0f32.to_bits();
        let result = e
            .call(
                0x00b5_b610,
                &args![
                    this,
                    one,
                    0u32,
                    0u32,
                    outputs,
                    outputs + 4,
                    outputs + 8,
                    0u32,
                    0u32,
                    0u32,
                    0x99u32
                ],
            )
            .f32();
        assert_eq!(e.mem.u32(outputs), 2);
        assert_eq!(e.mem.f32(outputs + 4), 2.0);
        assert_eq!(e.mem.f32(outputs + 8), 4.0);
        // b - a = (-1, 0, 0); times 0.75 dotted with (-1, 0, 0) is 0.75; plus 0.25 is 1.0.
        // (1 - 1) * 4 + (4 + 2 * 0.25) = 4.5.
        assert_eq!(result, 4.5);
    }

    #[test]
    fn build_shared_compound_frustum_unions_both_rooms() {
        let mut e = Engine::new();
        for address in [
            FRUSTUM_SET_CAMERA,
            FRUSTUM_MERGE,
            FRUSTUM_START_GROUP_UNION,
            FRUSTUM_END_GROUP,
            FRUSTUM_REMOVE_EMPTY_GROUPS,
            FRUSTUM_PRETHREAD_OP_LIST,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x200);
        let frustum = e.mem.alloc(0x40);
        let target = e.mem.alloc(0xb0);
        e.mem.set_u32(target + 0xac, frustum);
        // A room whose object chain answers 1 and whose frustum has a word +0x28.
        let room = |e: &mut Engine, word: u32| {
            let room = e.mem.alloc(0xd0);
            let object = e.mem.alloc(0x10);
            let inner = e.mem.alloc(0x10);
            e.mem.set_u32(inner + 8, 1);
            e.mem.set_u32(object + 0xc, inner);
            e.mem.set_u32(room + 0xac, object);
            let room_frustum = e.mem.alloc(0x30);
            e.mem.set_u32(room_frustum + 0x28, word);
            e.mem.set_u32(room + 0xcc, room_frustum);
            (room, room_frustum)
        };
        let (first, first_frustum) = room(&mut e, 1);
        let (second, second_frustum) = room(&mut e, 1);
        let portal = e.mem.alloc(0x108);
        e.mem.set_u32(portal + 0xfc, first);
        e.mem.set_u32(portal + 0x100, second);
        e.mem.set_u32(portal + 0x104, target);
        e.call_log = Some(vec![]);
        e.call(0x00b5_ba80, &args![this, 0x7777u32, portal]);
        assert_eq!(
            calls_to(&e, FRUSTUM_SET_CAMERA),
            vec![vec![frustum, 0x7777]]
        );
        assert_eq!(calls_to(&e, FRUSTUM_START_GROUP_UNION).len(), 1);
        assert_eq!(
            calls_to(&e, FRUSTUM_MERGE),
            vec![vec![frustum, first_frustum], vec![frustum, second_frustum]]
        );
        assert_eq!(calls_to(&e, FRUSTUM_PRETHREAD_OP_LIST).len(), 1);
        // A missing second room uses the node's own frustum and counts by the word +0x160.
        e.mem.set_u32(portal + 0x100, 0);
        e.mem.set_u32(this + 0x160, 1);
        e.mem.set_u8(this + 0x1dc, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_ba80, &args![this, 0x7777u32, portal]);
        assert_eq!(
            calls_to(&e, FRUSTUM_MERGE),
            vec![vec![frustum, first_frustum]]
        );
        assert!(calls_to(&e, FRUSTUM_START_GROUP_UNION).is_empty());
        // A counting room whose frustum word +0x28 is 0 stops after the camera.
        e.mem.set_u32(first_frustum + 0x28, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_ba80, &args![this, 0x7777u32, portal]);
        assert_eq!(calls_to(&e, FRUSTUM_SET_CAMERA).len(), 1);
        assert!(calls_to(&e, FRUSTUM_REMOVE_EMPTY_GROUPS).is_empty());
    }

    #[test]
    fn light_word_goes_to_the_matching_light() {
        let mut e = Engine::new();
        e.register(LIGHT_00B9DCB0, |_, _| Ret::default());
        let this = e.mem.alloc(0x200);
        let a = e.mem.alloc(0x120);
        let b = e.mem.alloc(0x120);
        e.mem.set_u32(a + LIGHT_SP_LIGHT, 0x10);
        e.mem.set_u32(b + LIGHT_SP_LIGHT, 0x20);
        fill_list(&mut e, this + LIGHT_LIST, &[a, 0, b]);
        e.call_log = Some(vec![]);
        e.call(0x00b5_c450, &args![this, 0x20u32, 7u32]);
        e.call(0x00b5_c450, &args![this, 0x30u32, 8u32]);
        assert_eq!(calls_to(&e, LIGHT_00B9DCB0), vec![vec![b, 7]]);
    }

    #[test]
    fn viewer_strings_count_lights_and_shadow_lights() {
        let mut e = Engine::new();
        e.mem.map(SHADOW_SCENE_NODE_RTTI, 4);
        e.mem.set_u32(SHADOW_SCENE_NODE_RTTI, 0x0109_0000);
        e.register(NI_NODE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_SHORT, |_, a| (a[0] + a[1]).into_ret());
        e.register(VIEWER_STRING_INT, |_, a| (a[0] + a[1]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            for i in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let value = e.mem.u32(old + 4 * i);
                e.mem.set_u32(base + 4 * i, value);
            }
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x200);
        let lights: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x120)).collect();
        e.mem.set_u8(lights[0] + LIGHT_CAST_SHADOW, 1);
        e.mem.set_u8(lights[2] + LIGHT_CAST_SHADOW, 1);
        fill_list(&mut e, this + LIGHT_LIST, &lights);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        e.call(0x00b5_c7d0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_NODE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        let base = e.mem.u32(strings + 4);
        let shown: Vec<u32> = (0..u32::from(e.mem.u16(strings + 0xa)))
            .map(|i| e.mem.u32(base + 4 * i))
            .collect();
        assert_eq!(
            shown,
            vec![
                0x0109_0001,
                TEXT_LIGHT_COUNT + 3,
                TEXT_SHADOW_LIGHT_COUNT + 2
            ]
        );
        // The inlined `SetAt` counts the last string.
        assert_eq!(e.mem.u16(strings + 0xc), 1);
        assert_eq!(e.call(0x00b5_e590, &args![]).u32(), SHADOW_SCENE_NODE_RTTI);
    }

    #[test]
    fn queue_update_by_ni_light_finds_the_light() {
        const ADDED: u32 = 0x03b6_0000;
        let mut e = Engine::new();
        e.mem.map(ADDED, 4);
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(LIST_ADD_TAIL_00A5A310, |e, a| {
            let light = e.mem.u32(a[1]);
            e.mem.set_u32(ADDED, light);
            Ret::default()
        });
        let this = e.mem.alloc(0x200);
        let light = e.mem.alloc(0x120);
        e.mem.set_u32(light + 4, 1);
        e.mem.set_u32(light + LIGHT_SP_LIGHT, 0x10);
        fill_list(&mut e, this + LIGHT_LIST, &[0, light]);
        e.call_log = Some(vec![]);
        e.call(0x00b5_f080, &args![this, 0x30u32]);
        assert!(calls_to(&e, LIST_ADD_TAIL_00A5A310).is_empty());
        e.call(0x00b5_f080, &args![this, 0x10u32]);
        let adds = calls_to(&e, LIST_ADD_TAIL_00A5A310);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], this + LIGHT_QUEUE_UPDATE);
        assert_eq!(e.mem.u32(ADDED), light);
        assert_eq!(e.mem.u32(light + 4), 1);
    }

    #[test]
    fn find_and_cull_light_walks_child_nodes_and_lights() {
        const GET_RTTI: u32 = 0x03b4_0000;
        const IS_NODE: u32 = 0x03b5_0000;
        let mut e = Engine::new();
        // The object's word +0x10 is its RTTI record; +0x14 says node.
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(IS_NODE, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        e.mem.set_u32(vtable + 0xc, IS_NODE);
        // An RTTI chain whose base is the light record.
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, LIGHT_RTTI_011F4A28);
        let object = |e: &mut Engine, rtti: u32, node: u32| {
            let object = e.mem.alloc(0xb0);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(object + 0x10, rtti);
            e.mem.set_u32(object + 0x14, node);
            object
        };
        let light = object(&mut e, derived, 0);
        let other = object(&mut e, 0, 0);
        let inner = object(&mut e, 0, 1);
        let inner_light = object(&mut e, LIGHT_RTTI_011F4A28, 0);
        let set_children = |e: &mut Engine, node: u32, children: &[u32]| {
            let buffer = e.mem.alloc(4 * children.len() as u32);
            for (i, child) in children.iter().enumerate() {
                e.mem.set_u32(buffer + 4 * i as u32, *child);
            }
            e.mem.set_u32(node + NODE_CHILDREN_BASE, buffer);
            e.mem
                .set_u16(node + NODE_CHILDREN_SIZE, children.len() as u16);
        };
        set_children(&mut e, inner, &[inner_light]);
        let root = object(&mut e, 0, 1);
        set_children(&mut e, root, &[light, 0, other, inner]);
        // Removal needs the remove queues allowed and finds no shadow light: no change.
        let this = e.mem.alloc(0x200);
        e.call_log = Some(vec![]);
        e.call(0x00b5_f150, &args![this, root, 0x101u32]);
        let rtti_calls: Vec<u32> = calls_to(&e, GET_RTTI).iter().map(|c| c[0]).collect();
        assert_eq!(rtti_calls, vec![light, other, inner, inner_light]);
        let node_calls: Vec<u32> = calls_to(&e, IS_NODE).iter().map(|c| c[0]).collect();
        assert_eq!(node_calls, vec![other, inner]);
        // Null node: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00b5_f150, &args![this, 0u32, 1u32]);
        assert!(calls_to(&e, GET_RTTI).is_empty());
    }
}
