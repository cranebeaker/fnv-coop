//! `BSShader/shadowscenelight.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::shadowscenenode::light_check;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

layout! {
    /// `ShadowSceneLight` (Xbox PDB), an `NiRefObject`. The PC constructor (`00b9fda0`)
    /// writes the Xbox PDB's offsets; the fields its functions here use.
    pub struct ShadowSceneLight: 0x244 {
        /// `fLuminance` (Xbox PDB).
        0x0C fLuminance: f32,
        /// `fLODDimmer` (Xbox PDB).
        0xD0 fLODDimmer: f32,
        /// `fShadowLODDimmer` (Xbox PDB).
        0xD4 fShadowLODDimmer: f32,
        /// `fShadowLODDimmerTarget` (Xbox PDB).
        0xD8 fShadowLODDimmerTarget: f32,
        /// `fShadowLODDimmerElapsed` (Xbox PDB).
        0xDC fShadowLODDimmerElapsed: f32,
        /// `bCastShadow` (Xbox PDB).
        0xEC bCastShadow: u8,
        /// `bCubeShadowMap` (Xbox PDB).
        0xED bCubeShadowMap: u8,
        /// `spSpecularAttenuationTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0xF0 spSpecularAttenuationTexture: u32,
        /// `bPointLight` (Xbox PDB).
        0xF4 bPointLight: u8,
        /// `bAmbientLight` (Xbox PDB).
        0xF5 bAmbientLight: u8,
        /// `spLight` (Xbox PDB): `NiPointer<NiLight>`.
        0xF8 spLight: u32,
        /// `bDynamicLight` (Xbox PDB).
        0xFC bDynamicLight: u8,
        /// `bPointPosition` (Xbox PDB): an `NiPoint3`, three words.
        0x100 bPointPosition: u32,
        /// `spDynamicShadowMap` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x10C spDynamicShadowMap: u32,
        /// `usFrustumCull` (Xbox PDB).
        0x110 usFrustumCull: u16,
        /// `spFrustumVis` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x114 spFrustumVis: u32,
        /// `bShadowSpotlight` (Xbox PDB).
        0x118 bShadowSpotlight: u8,
        /// `fSpotlightFOV` (Xbox PDB).
        0x11C fSpotlightFOV: f32,
        /// `fSpotlightFalloff` (Xbox PDB).
        0x120 fSpotlightFalloff: f32,
        /// `bShowDebugTexture` (Xbox PDB).
        0x124 bShowDebugTexture: u8,
        /// `spShadowScene` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x128 spShadowScene: u32,
        /// `kGeomListFence` (Xbox PDB).
        0x138 kGeomListFence: u32,
        /// `kGeomListFenceObject` (Xbox PDB): `NiPointer<NiTriBasedGeom>`.
        0x13C kGeomListFenceObject: u32,
        /// `spShadowCamera` (Xbox PDB): `NiPointer<NiCamera>`.
        0x140 spShadowCamera: u32,
        /// `kShadowCameraPlanes` (Xbox PDB): `NiFrustumPlanes`, six `NiPlane`s then the
        /// active-planes word at +0x1a4.
        0x144 kShadowCameraPlanes: u32,
        /// `pfClipSpacePlanes` (Xbox PDB): 0x60 bytes of floats.
        0x1A8 pfClipSpacePlanes: u32,
        /// `bClipSpacePlanesReady` (Xbox PDB).
        0x208 bClipSpacePlanesReady: u8,
        /// `spShadowAccumulator` (Xbox PDB): `NiPointer<BSShaderAccumulator>`.
        0x20C spShadowAccumulator: u32,
    }
}

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// An `NiPointer`'s release: the count at +4 decremented, the object deleted through its
/// vtable slot +4 when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `NiPointLight::ms_RTTI` (`011f4a98`) and `NiAmbientLight::ms_RTTI` (`011f5f6c`), the
/// addresses `nipointlight.rs` and `niambientlight.rs` answer from their `GetRTTI`.
const NI_POINT_LIGHT_RTTI: u32 = 0x011f_4a98;
const NI_AMBIENT_LIGHT_RTTI: u32 = 0x011f_5f6c;

/// The inlined kind-of test: the object's `GetRTTI` (vtable slot +8), then the chain of base
/// records (each `NiRTTI`'s word at +4) until `rtti` or null.
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, 8, &args![]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record.wrapping_add(4));
    }
    false
}

// Translated from 00b9dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::SetLight` (Xbox PDB). `thiscall`, `RET 4`, one word (the light).
/// Assigns the light to `spLight` (the `NiPointer` assignment: the old one released, the new
/// one's count incremented, nothing when they are the same), sets `bPointLight` to whether
/// the light is an `NiPointLight` (kind-of through its RTTI chain; false for none); when it
/// is not, `fLODDimmer` becomes 1.0, and when it is and `bDynamicLight` is set, the light's
/// three words at +0x8c are copied into `bPointPosition`. Then `bAmbientLight` is set to
/// whether the light is an `NiAmbientLight`.
pub fn shadow_scene_light_set_light(e: &mut Engine, this: Ptr<ShadowSceneLight>, light: Ptr) {
    let light = light.addr();
    let old = e.get(this, ShadowSceneLight::spLight);
    let point = if old != light {
        if old != 0 {
            release(e, old);
        }
        e.set(this, ShadowSceneLight::spLight, light);
        if light == 0 {
            false
        } else {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light.wrapping_add(4)]);
            is_kind_of(e, light, NI_POINT_LIGHT_RTTI)
        }
    } else {
        light != 0 && is_kind_of(e, light, NI_POINT_LIGHT_RTTI)
    };
    e.set(this, ShadowSceneLight::bPointLight, u8::from(point));
    if !point {
        e.set(
            this,
            ShadowSceneLight::fLODDimmer,
            f32::from_bits(x87_store32(1.0)),
        );
    } else if e.get(this, ShadowSceneLight::bDynamicLight) != 0 {
        let position = this.addr().wrapping_add(0x100);
        for word in 0..3u32 {
            let value = e.mem.u32(light.wrapping_add(0x8c + 4 * word));
            e.mem.set_u32(position.wrapping_add(4 * word), value);
        }
    }
    let ambient = light != 0 && is_kind_of(e, light, NI_AMBIENT_LIGHT_RTTI);
    e.set(this, ShadowSceneLight::bAmbientLight, u8::from(ambient));
}

/// `NiRefObject`'s vtable (`0101dce4`, RTTI `.?AVNiRefObject@@`) and its object count
/// (`NiRefObject::ms_uiObjects`, `011f4400`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// `ShadowSceneLight`'s vtable (`010b7eb8`, no RTTI locator before it).
const SHADOW_SCENE_LIGHT_VTABLE: u32 = 0x010b_7eb8;
/// `NiPlane::NiPlane` (engine map, `00a69940`): `thiscall`, no stack words.
const NI_PLANE_CONSTRUCT: u32 = 0x00a6_9940;
/// The vtables of the three `BSSimpleArray` members: `kMultiboundRooms` and `kPortals`
/// (`010b7ea8`, no RTTI locator) and `kProcessedNodes` (`0101a8e8`, RTTI
/// `.?AV?$BSSimpleArray@PAVNiNode@@$0EAA@@`).
const SIMPLE_ARRAY_VTABLE_010B7EA8: u32 = 0x010b_7ea8;
const SIMPLE_ARRAY_VTABLE_0101A8E8: u32 = 0x0101_a8e8;
/// The three words `bPointPosition` is copied from (`011f426c`).
const POINT_POSITION_011F426C: u32 = 0x011f_426c;
/// The `float` `fSpotlightFOV` is loaded from (`010249d8`).
const SPOTLIGHT_FOV_010249D8: u32 = 0x0102_49d8;
/// CRT `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;

/// The words the constructor clears first, in the exe's order: `lGeomList` (+0xe0: count,
/// head, tail), the `NiPointer`s, `lRootList` (+0x12c) and the fence objects.
const CLEARED_WORDS_00B9FDA0: [u32; 13] = [
    0xe8, 0xe0, 0xe4, 0xf0, 0xf8, 0x10c, 0x114, 0x128, 0x134, 0x12c, 0x130, 0x13c, 0x140,
];
/// The 4x4 float matrix at +0x10 (`kViewProjection`, Xbox PDB) set to the identity: the
/// words set to 0.0 and those set to 1.0, in the exe's order.
const MATRIX_ZEROS: [u32; 12] = [
    0x48, 0x44, 0x40, 0x3c, 0x34, 0x30, 0x2c, 0x28, 0x20, 0x1c, 0x18, 0x14,
];
const MATRIX_ONES: [u32; 4] = [0x4c, 0x38, 0x24, 0x10];

/// Releases and clears an `NiPointer` member when it is not null.
fn clear_pointer(e: &mut Engine, this: Ptr<ShadowSceneLight>, field: Field<ShadowSceneLight, u32>) {
    let object = e.get(this, field);
    if object != 0 {
        release(e, object);
        e.set(this, field, 0);
    }
}

// Translated from 00b9fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::ShadowSceneLight` (Xbox PDB). `thiscall`, no stack words. The
/// `NiRefObject` part (its vtable, count 0, `ms_uiObjects` incremented through
/// `InterlockedIncrement`), this class's vtable (`010b7eb8`), the lists and pointers cleared,
/// the six `NiPlane`s of `kShadowCameraPlanes` constructed (`00a69940`) and its active-planes
/// word set to 0x3f, the three `BSSimpleArray`s set up (vtable, buffer, size and reserved
/// size 0); `fLuminance`, `fLODDimmer`, `fShadowLODDimmer` 0.0, `bCubeShadowMap` set then
/// cleared, the other flags cleared, `bPointPosition` copied from `011f426c`, `usFrustumCull`
/// 0; the pointers released again (already null, as the exe does it); `kViewProjection` the
/// identity; `fSpotlightFOV` from the `float` at `010249d8`, `fSpotlightFalloff` 1.0; the
/// rest of the pointers released and `kGeomListFence` cleared; `fShadowLODDimmerElapsed` 0.0,
/// `fShadowLODDimmerTarget` 1.0, `bClipSpacePlanesReady` cleared and the 0x60 bytes of
/// `pfClipSpacePlanes` cleared (`_memset`). All in the exe's order. Answers `this`.
// Translation incomplete: 00b9fda0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xd) is not translated.
pub fn shadow_scene_light_shadow_scene_light(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
) -> Ptr<ShadowSceneLight> {
    let base = this.addr();
    let at = |offset: u32| base.wrapping_add(offset);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(at(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![NI_REF_OBJECT_COUNT]);
    e.mem.set_u32(base, SHADOW_SCENE_LIGHT_VTABLE);
    for offset in CLEARED_WORDS_00B9FDA0 {
        e.mem.set_u32(at(offset), 0);
    }
    for plane in 0..6u32 {
        e.call(NI_PLANE_CONSTRUCT, &args![at(0x144 + 0x10 * plane)]);
    }
    e.mem.set_u32(at(0x1a4), 0x3f);
    e.set(this, ShadowSceneLight::spShadowAccumulator, 0);
    for (array, vtable) in [
        (0x210u32, SIMPLE_ARRAY_VTABLE_010B7EA8),
        (0x220, SIMPLE_ARRAY_VTABLE_010B7EA8),
        (0x230, SIMPLE_ARRAY_VTABLE_0101A8E8),
    ] {
        e.mem.set_u32(at(array), vtable);
        for word in [4u32, 8, 0xc] {
            e.mem.set_u32(at(array + word), 0);
        }
    }

    let zero = f32::from_bits(x87_store32(0.0));
    let one = f32::from_bits(x87_store32(1.0));
    e.set(this, ShadowSceneLight::fLuminance, zero);
    e.set(this, ShadowSceneLight::fLODDimmer, zero);
    e.set(this, ShadowSceneLight::bCubeShadowMap, 1);
    e.set(this, ShadowSceneLight::fShadowLODDimmer, zero);
    e.set(this, ShadowSceneLight::bCastShadow, 0);
    e.set(this, ShadowSceneLight::bPointLight, 0);
    e.set(this, ShadowSceneLight::bAmbientLight, 0);
    e.set(this, ShadowSceneLight::bDynamicLight, 0);
    for word in 0..3u32 {
        let value = e.global::<u32>(POINT_POSITION_011F426C + 4 * word);
        e.mem.set_u32(at(0x100 + 4 * word), value);
    }
    e.set(this, ShadowSceneLight::usFrustumCull, 0);

    clear_pointer(e, this, ShadowSceneLight::spSpecularAttenuationTexture);
    clear_pointer(e, this, ShadowSceneLight::spLight);
    clear_pointer(e, this, ShadowSceneLight::spFrustumVis);
    clear_pointer(e, this, ShadowSceneLight::spDynamicShadowMap);

    for offset in MATRIX_ZEROS {
        e.mem.set_f32(at(offset), zero);
    }
    for offset in MATRIX_ONES {
        e.mem.set_f32(at(offset), one);
    }
    let fov = x87_quiet_bits(e.global::<u32>(SPOTLIGHT_FOV_010249D8));
    e.set(this, ShadowSceneLight::bCubeShadowMap, 0);
    e.set(this, ShadowSceneLight::fSpotlightFOV, f32::from_bits(fov));
    e.set(this, ShadowSceneLight::bShadowSpotlight, 0);
    e.set(this, ShadowSceneLight::bShowDebugTexture, 0);
    e.set(this, ShadowSceneLight::fSpotlightFalloff, one);

    clear_pointer(e, this, ShadowSceneLight::spShadowScene);
    e.set(this, ShadowSceneLight::kGeomListFence, 0);
    clear_pointer(e, this, ShadowSceneLight::kGeomListFenceObject);
    clear_pointer(e, this, ShadowSceneLight::spShadowCamera);

    e.set(this, ShadowSceneLight::fShadowLODDimmerElapsed, zero);
    e.set(this, ShadowSceneLight::fShadowLODDimmerTarget, one);
    e.set(this, ShadowSceneLight::bClipSpacePlanesReady, 0);
    e.call(MEMSET, &args![at(0x1a8), 0u32, 0x60u32]);
    this
}

/// The item free `00e6e830` (`thiscall` on the list, `RET 4`: the item; the engine map names
/// the body `NiTPointerMap<char const*,NiPointer<NiShaderLibrary>>::DeleteItem`).
const LIST_FREE_ITEM_00E6E830: u32 = 0x00e6_e830;

// Translated from 00b9e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `BSShader/shadowscenelight.cpp`): removes the head of an
/// `NiTPointerList` of `NiPointer`s into an `NiPointer` the caller provides. `thiscall`,
/// `RET 4`, one word (the out pointer, not yet holding anything). The head item is unlinked
/// (the head takes its next link; with a new head that one's previous link is cleared,
/// otherwise the tail is), its element is stored in the out pointer and held
/// (`InterlockedIncrement` on its +4) when not null, the item is freed (`00e6e830`) and the
/// count goes down by one. Answers the out pointer. An empty list is not checked (the head's
/// next link is read from address 0, as in the exe).
// Translation incomplete: 00b9e7c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the out pointer) is not translated.
pub fn fn_00b9e7c0(e: &mut Engine, this: Ptr<NiTPointerList>, out: Ptr) -> Ptr {
    let item = Ptr::<NiTListItem>::new(e.get(this, NiTPointerList::m_pkHead));
    let next = e.get(item, NiTListItem::m_pkNext);
    e.set(this, NiTPointerList::m_pkHead, next);
    if next == 0 {
        e.set(this, NiTPointerList::m_pkTail, 0);
    } else {
        e.set(Ptr::<NiTListItem>::new(next), NiTListItem::m_pkPrev, 0);
    }
    let element = e.get(item, NiTListItem::m_element);
    e.mem.set_u32(out.addr(), element);
    if element != 0 {
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![element.wrapping_add(4)],
        );
    }
    e.call(LIST_FREE_ITEM_00E6E830, &args![this, item]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    out
}

// Translated from 00b9e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `BSShader/shadowscenelight.cpp`): removes the tail of an
/// `NiTPointerList` of `NiPointer`s into an `NiPointer` the caller provides (the counterpart of
/// `00b9e7c0`). `thiscall`, `RET 4`, one word (the out pointer). The tail item is unlinked (the
/// tail takes its previous link; with a new tail that one's next link is cleared, otherwise the
/// head is), its element is stored in the out pointer and held (`InterlockedIncrement` on its
/// +4) when not null, the item is freed (`00e6e830`) and the count goes down by one. Answers
/// the out pointer. An empty list is not checked (the tail's link is read from address 4).
// Translation incomplete: 00b9e850 - the C++ exception frame (FS:[0] chain, unwind state 0 for the out pointer) is not translated.
pub fn fn_00b9e850(e: &mut Engine, this: Ptr<NiTPointerList>, out: Ptr) -> Ptr {
    let item = Ptr::<NiTListItem>::new(e.get(this, NiTPointerList::m_pkTail));
    let previous = e.get(item, NiTListItem::m_pkPrev);
    e.set(this, NiTPointerList::m_pkTail, previous);
    if previous == 0 {
        e.set(this, NiTPointerList::m_pkHead, 0);
    } else {
        e.set(Ptr::<NiTListItem>::new(previous), NiTListItem::m_pkNext, 0);
    }
    let element = e.get(item, NiTListItem::m_element);
    e.mem.set_u32(out.addr(), element);
    if element != 0 {
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![element.wrapping_add(4)],
        );
    }
    e.call(LIST_FREE_ITEM_00E6E830, &args![this, item]);
    let count = e.get(this, NiTPointerList::m_uiCount).wrapping_sub(1);
    e.set(this, NiTPointerList::m_uiCount, count);
    out
}

// ---- b064: the light's frustum, radius, geometry lists and depth map ----

/// Members by PC offset (Xbox PDB names): `lGeomList` (+0xe0, an `NiTPointerList`), the
/// `NiFrustumPlanes`' active mask (+0x1a4), `kShadowView` (+0x50), `kShadowProj` (+0x90),
/// `kViewProjection` (+0x10).
const GEOM_LIST: u32 = 0xe0;
const PLANES_ACTIVE: u32 = 0x1a4;
/// `NiAVObject` members (Xbox PDB names, PC offsets): `m_kWorldBound` (+0x20, radius at
/// +0xc), `m_uFlags` (+0x30), the world translation (+0x8c); `NiNode::m_kChildren` buffer
/// (+0xa0) and `m_usSize` (+0xa6); a geometry's property word (+0xa8, type at +0x1c).
const WORLD_BOUND: u32 = 0x20;
const FLAGS: u32 = 0x30;
const WORLD_TRANSLATE: u32 = 0x8c;
const CHILDREN_BASE: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;
const GEOMETRY_PROPERTY: u32 = 0xa8;
/// The null bound (`011f4288`), the scene node pointer global (`011f91c8`, its lighting offset
/// at +0x1e4) and the `NiRTTI` record a radius test accepts outright (`011f4a68`).
const NULL_BOUND: u32 = 0x011f_4288;
const SCENE_NODE_011F91C8: u32 = 0x011f_91c8;
const LIGHTING_OFFSET: u32 = 0x1e4;
const RADIUS_FREE_RTTI_011F4A68: u32 = 0x011f_4a68;
/// `_CIsqrt` (`00ec6040`) and 0.0 (`double`, `01012060`).
const CRT_SQRT: u32 = 0x00ec_6040;
const ZERO_01012060: u32 = 0x0101_2060;
/// Virtual slots: `IsNode` (+0xc), `IsTriBasedGeom` (+0x1c).
const SLOT_IS_NODE: u32 = 0xc;
const SLOT_IS_TRI_BASED_GEOM: u32 = 0x1c;

/// `FLD a; FLD b; FCOM; TEST AH,5; JP`: `a` when `b < a`, else `b` (NaN included).
fn x87_keep_larger(a: f64, b: f64) -> f64 {
    if b < a {
        a
    } else {
        b
    }
}

/// The squared length `(y * y + x * x) + z * z` as the exe's `FLD y; FLD x; FLD z; FLD ST1;
/// FMULP ST2; FLD ST2; FMULP ST3; FXCH; FADDP ST2; FMUL ST0; FADDP` sequence computes it.
fn squared_length(x: f64, y: f64, z: f64) -> f64 {
    x87_add(x87_add(x87_mul(y, y), x87_mul(x, x)), x87_mul(z, z))
}

/// `_CIsqrt` of a stored `float`, stored again as a `float`.
fn sqrt_stored(e: &mut Engine, squared: u32) -> u32 {
    let root = e.call(CRT_SQRT, &args![x87_load32(squared)]).st0;
    x87_store32(root)
}

/// The geometry's property when its type is 1 to 12 (the inlined `(type - 1) <= 0xb`), else 0.
fn shader_property(e: &Engine, property: u32) -> u32 {
    if e.mem.u32(property.wrapping_add(0x1c)).wrapping_sub(1) <= 0xb {
        property
    } else {
        0
    }
}

/// A child's bound test of the parse walks: flag 0x800 skips it; otherwise a missing bound or
/// a zero radius (`FLDZ; FCOMP`) rejects the child.
fn child_bound_passes(e: &Engine, child: u32) -> bool {
    let flags = e.mem.u32(child.wrapping_add(FLAGS));
    if (flags >> 0xb) & 1 != 0 {
        return true;
    }
    let bound = e.mem.u32(child.wrapping_add(WORLD_BOUND));
    if bound == 0 {
        return false;
    }
    x87_m32(e.mem.u32(bound.wrapping_add(0xc))) != 0.0
}

/// The bound of an object, or the null bound.
fn bound_or_null(e: &Engine, object: u32) -> u32 {
    let bound = e.mem.u32(object.wrapping_add(WORLD_BOUND));
    if bound == 0 {
        NULL_BOUND
    } else {
        bound
    }
}

// Translated from 00b9bbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::BoundWithinLightFrustum` (Xbox PDB). `thiscall`, `RET 4` (an `NiBound`).
/// For each active plane of `kShadowCameraPlanes` (bit `i` of the mask at +0x1a4): the signed
/// distance of the bound's centre (`(n.y * c.y + n.x * c.x) + c.z * n.z - d`) answers 0 when it
/// is not above `-radius`; a plane the whole bound is on the inner side of (distance at least
/// the radius) is dropped from the mask. Answers 1 otherwise.
pub fn shadow_scene_light_bound_within_light_frustum(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    bound: Ptr,
) -> u8 {
    let base = this.addr();
    let b = |e: &Engine, i: u32| e.mem.u32(bound.addr().wrapping_add(4 * i));
    for plane in 0..6u32 {
        let bit = 1u32 << plane;
        let mask = e.mem.u32(base.wrapping_add(PLANES_ACTIVE));
        if mask & bit == 0 {
            continue;
        }
        let at = base.wrapping_add(0x144 + 0x10 * plane);
        let n = |e: &Engine, i: u32| e.mem.u32(at.wrapping_add(4 * i));
        let y = x87_mul(x87_load32(n(e, 1)), x87_m32(b(e, 1)));
        let x = x87_mul(x87_load32(n(e, 0)), x87_m32(b(e, 0)));
        let z = x87_mul(x87_load32(b(e, 2)), x87_m32(n(e, 2)));
        let dot = x87_store32(x87_add(x87_add(y, x), z));
        let distance = x87_store32(x87_sub(x87_load32(dot), x87_m32(n(e, 3))));
        let distance = x87_load32(distance);
        let radius = x87_load32(b(e, 3));
        // `FCHS; FCOMP; TEST AH,1; JZ`: outside unless `-radius < distance` (or unordered).
        if -radius >= distance {
            return 0;
        }
        // `FCOMPP; TEST AH,0x41; JP`: the plane is dropped when `radius <= distance`.
        if radius <= distance {
            let mask = e.mem.u32(base.wrapping_add(PLANES_ACTIVE));
            e.mem.set_u32(base.wrapping_add(PLANES_ACTIVE), mask & !bit);
        }
    }
    1
}

// Translated from 00b9bc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the light's luminance at a point. `thiscall`, `RET 0x10`: the
/// point (three words) and an `NiLight` to leave out. 0.0 for that light and for a light with
/// `bCastShadow` 1. Otherwise `fLuminance` becomes 1.0, then for a point light
/// `1 - c * c` with `c` the distance over the `NiLight`'s radius (+0xe0) clamped to [0, 1];
/// finally it is scaled by the largest component of the `NiLight`'s colour (+0xd4) times its
/// dimmer (+0xc4) and answered (`ST0`).
pub fn fn_00b9bc50(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    x: u32,
    y: u32,
    z: u32,
    excluded: u32,
) -> f32 {
    let ni_light = e.get(this, ShadowSceneLight::spLight);
    if ni_light == excluded || e.get(this, ShadowSceneLight::bCastShadow) == 1 {
        return 0.0;
    }
    let point = e.get(this, ShadowSceneLight::bPointLight) != 0;
    let dimmer = x87_load32(x87_quiet_bits(e.mem.u32(ni_light.wrapping_add(0xc4))));
    let position = [
        e.mem.u32(ni_light.wrapping_add(WORLD_TRANSLATE)),
        e.mem.u32(ni_light.wrapping_add(WORLD_TRANSLATE + 4)),
        e.mem.u32(ni_light.wrapping_add(WORLD_TRANSLATE + 8)),
    ];
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
    let radius = x87_quiet_bits(e.mem.u32(ni_light.wrapping_add(0xe0)));
    e.set(
        this,
        ShadowSceneLight::fLuminance,
        f32::from_bits(x87_store32(1.0)),
    );
    if point {
        let dx = x87_store32(x87_sub(x87_load32(x), x87_m32(position[0])));
        let dy = x87_store32(x87_sub(x87_load32(y), x87_m32(position[1])));
        let dz = x87_store32(x87_sub(x87_load32(z), x87_m32(position[2])));
        let squared = x87_store32(squared_length(
            x87_load32(dx),
            x87_load32(dy),
            x87_load32(dz),
        ));
        let distance = sqrt_stored(e, squared);
        let mut ratio = x87_store32(x87_div(x87_load32(distance), x87_m32(radius)));
        let zero: f64 = e.global(ZERO_01012060);
        if x87_load32(ratio) < zero {
            ratio = x87_store32(0.0);
        }
        let c = x87_load32(ratio);
        let clamped = x87_store32(if 1.0 < c { 1.0 } else { c });
        let c = x87_load32(clamped);
        let falloff = x87_sub(1.0, x87_mul(c, c));
        e.set(
            this,
            ShadowSceneLight::fLuminance,
            f32::from_bits(x87_store32(falloff)),
        );
    }
    let gb = x87_store32(x87_keep_larger(x87_load32(g), x87_load32(b)));
    let peak = x87_store32(x87_keep_larger(x87_load32(r), x87_load32(gb)));
    let luminance = e.get(this, ShadowSceneLight::fLuminance).to_bits();
    let result = x87_store32(x87_mul(x87_load32(peak), x87_m32(luminance)));
    let result = x87_quiet_bits(result);
    e.set(this, ShadowSceneLight::fLuminance, f32::from_bits(result));
    f32::from_bits(result)
}

// Translated from 00b9cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: assigns an object to the `NiPointer` at +0x1a0 (the old one
/// released, the new one held). `thiscall`, `RET 4`.
pub fn fn_00b9cfd0(e: &mut Engine, this: Ptr, object: u32) {
    let slot = this.addr().wrapping_add(0x1a0);
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

// Translated from 00b9dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::BoundWithinLightRadius` (Xbox PDB). `thiscall`, `RET 0xc`: an `NiBound`,
/// an `NiLight` (the light's own `spLight` when null) and a radius scale (`float`). A light
/// whose RTTI is the record at `011f4a68` is always in range. Otherwise the distance from the
/// bound's centre to the `NiLight`'s position moved by the scene node's lighting offset (the
/// node at `011f91c8`, +0x1e4), less the bound's radius times the scale, must be below the
/// `NiLight`'s radius (+0xe0).
pub fn shadow_scene_light_bound_within_light_radius(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    bound: Ptr,
    ni_light: u32,
    scale: f32,
) -> u8 {
    let light = if ni_light == 0 {
        e.get(this, ShadowSceneLight::spLight)
    } else {
        ni_light
    };
    if e.vcall(light, 8, &args![]).u32() == RADIUS_FREE_RTTI_011F4A68 {
        return 1;
    }
    let node = e.global::<u32>(SCENE_NODE_011F91C8);
    let offset = [
        e.mem.u32(node.wrapping_add(LIGHTING_OFFSET)),
        e.mem.u32(node.wrapping_add(LIGHTING_OFFSET + 4)),
        e.mem.u32(node.wrapping_add(LIGHTING_OFFSET + 8)),
    ];
    let b = bound.addr();
    let mut delta = [0u32; 3];
    for i in 0..3u32 {
        let p = x87_load32(e.mem.u32(light.wrapping_add(WORLD_TRANSLATE + 4 * i)));
        delta[i as usize] = x87_store32(x87_sub(p, x87_m32(e.mem.u32(b.wrapping_add(4 * i)))));
    }
    let mut moved = [0u32; 3];
    for i in 0..3 {
        moved[i] = x87_store32(x87_add(x87_load32(offset[i]), x87_m32(delta[i])));
    }
    let squared = x87_store32(squared_length(
        x87_load32(moved[0]),
        x87_load32(moved[1]),
        x87_load32(moved[2]),
    ));
    let distance = sqrt_stored(e, squared);
    let reach = x87_mul(
        x87_load32(e.mem.u32(b.wrapping_add(0xc))),
        x87_m32(scale.to_bits()),
    );
    let gap = x87_store32(x87_sub(x87_load32(distance), reach));
    let radius = x87_load32(e.mem.u32(light.wrapping_add(0xe0)));
    u8::from(radius > x87_load32(gap))
}

// Translated from 00b9dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::GetLuminanceToBound` (Xbox PDB). `thiscall`, `RET 4` (an `NiBound`):
/// `(distance - bound radius) / NiLight radius`, the distance being from the `NiLight`'s
/// position moved by the scene node's lighting offset to the bound's centre; stored in
/// `fLuminance` and answered (`ST0`).
pub fn shadow_scene_light_get_luminance_to_bound(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    bound: Ptr,
) -> f32 {
    let node = e.global::<u32>(SCENE_NODE_011F91C8);
    let offset = [
        e.mem.u32(node.wrapping_add(LIGHTING_OFFSET)),
        e.mem.u32(node.wrapping_add(LIGHTING_OFFSET + 4)),
        e.mem.u32(node.wrapping_add(LIGHTING_OFFSET + 8)),
    ];
    let light = e.get(this, ShadowSceneLight::spLight);
    let px = x87_store32(x87_add(
        x87_load32(offset[0]),
        x87_m32(e.mem.u32(light.wrapping_add(WORLD_TRANSLATE))),
    ));
    let py = x87_store32(x87_add(
        x87_load32(e.mem.u32(light.wrapping_add(WORLD_TRANSLATE + 4))),
        x87_m32(offset[1]),
    ));
    let pz = x87_store32(x87_add(
        x87_load32(e.mem.u32(light.wrapping_add(WORLD_TRANSLATE + 8))),
        x87_m32(offset[2]),
    ));
    let b = bound.addr();
    let dx = x87_store32(x87_sub(x87_load32(px), x87_m32(e.mem.u32(b))));
    let dy = x87_store32(x87_sub(
        x87_load32(py),
        x87_m32(e.mem.u32(b.wrapping_add(4))),
    ));
    let dz = x87_store32(x87_sub(
        x87_load32(pz),
        x87_m32(e.mem.u32(b.wrapping_add(8))),
    ));
    let squared = x87_store32(squared_length(
        x87_load32(dx),
        x87_load32(dy),
        x87_load32(dz),
    ));
    let distance = sqrt_stored(e, squared);
    let value = x87_div(
        x87_sub(
            x87_load32(distance),
            x87_m32(e.mem.u32(b.wrapping_add(0xc))),
        ),
        x87_m32(e.mem.u32(light.wrapping_add(0xe0))),
    );
    let value = x87_quiet_bits(x87_store32(value));
    e.set(this, ShadowSceneLight::fLuminance, f32::from_bits(value));
    f32::from_bits(value)
}

/// The TLS word `00b9dcb0` saves (+0x2b4), the renderer object (`011f91a8`) with its shadow
/// map release `00b6d4c0` (`RET 4`), and the globals of its test (`011f91c0` compared with 3,
/// bit 0x10 of the byte at `011f91d8`).
const TLS_WORD_2B4: u32 = 0x2b4;
const RENDERER_011F91A8: u32 = 0x011f_91a8;
const RENDERER_RELEASE_MAP_00B6D4C0: u32 = 0x00b6_d4c0;
const RENDER_LEVEL_011F91C0: u32 = 0x011f_91c0;
const RENDER_FLAGS_011F91D8: u32 = 0x011f_91d8;

// Translated from 00b9dcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `bCastShadow`. `thiscall`, `RET 4` (the flag). With the
/// TLS word +0x2b4 at 0x20 for its duration: the dynamic shadow map is handed to the renderer
/// object (`00b6d4c0`), released and cleared when the light stops casting shadows (flag clear
/// while set), or else unless `[011f91c0] >= 3` and bit 0x10 of `[011f91d8]` is set; then the
/// flag is stored.
// Translation incomplete: 00b9dcb0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the map temporary) is not translated.
pub fn fn_00b9dcb0(e: &mut Engine, this: Ptr<ShadowSceneLight>, cast: u8) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_WORD_2B4);
    e.mem.set_u32(tls + TLS_WORD_2B4, 0x20);
    let stops = e.get(this, ShadowSceneLight::bCastShadow) != 0 && cast == 0;
    let keep = !stops
        && (e.global::<u32>(RENDER_LEVEL_011F91C0) as i32) >= 3
        && e.global::<u8>(RENDER_FLAGS_011F91D8) & 0x10 != 0;
    if !keep {
        let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
        if map != 0 {
            let renderer = e.global::<u32>(RENDERER_011F91A8);
            e.call(RENDERER_RELEASE_MAP_00B6D4C0, &args![renderer, map]);
            let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
            if map != 0 {
                release(e, map);
                e.set(this, ShadowSceneLight::spDynamicShadowMap, 0);
            }
        }
    }
    e.set(this, ShadowSceneLight::bCastShadow, cast);
    e.mem.set_u32(tls + TLS_WORD_2B4, saved);
}

// Translated from 00b9e770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the geometry list fence. `thiscall`, no stack words. From
/// the fence item (`kGeomListFence`) to the end of `lGeomList`, each geometry's lighting
/// property (+0xa8, type 1 to 12) gets 0 at +0x38; then the fence is cleared.
pub fn fn_00b9e770(e: &mut Engine, this: Ptr<ShadowSceneLight>) {
    let mut item = e.get(this, ShadowSceneLight::kGeomListFence);
    while item != 0 {
        let geometry = e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_element);
        item = e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_pkNext);
        if geometry == 0 {
            continue;
        }
        let property = e.mem.u32(geometry.wrapping_add(GEOMETRY_PROPERTY));
        if property == 0 {
            continue;
        }
        let property = shader_property(e, property);
        if property != 0 {
            e.mem.set_u32(property.wrapping_add(0x38), 0);
        }
    }
    e.set(this, ShadowSceneLight::kGeomListFence, 0);
}

/// `AddTriGeom`'s callees and data: the function-local static `NiPointer` (`011fffe0`, guard
/// bit 0 of `011fffe4`, destructor `00fdce10` given to `_atexit` `00ec658f`), the geometry
/// test `00b4f580` (`thiscall` on its +0xc0, `RET 4`: the record at `0126f554`), the property
/// test `004a2020` (`RET 4`: 0xc), the property's `00b71560` (`RET 8`: the light and a bound)
/// and the list adds `00c1b4a0` (`RET 4`: an `NiPointer`'s address) and `00b702e0` (`RET 8`).
const ADD_TRI_GEOM_STATIC: u32 = 0x011f_ffe0;
const ADD_TRI_GEOM_GUARD: u32 = 0x011f_ffe4;
const ADD_TRI_GEOM_STATIC_DESTRUCT: u32 = 0x00fd_ce10;
const ATEXIT: u32 = 0x00ec_658f;
const GEOMETRY_TEST_00B4F580: u32 = 0x00b4_f580;
const GEOMETRY_TEST_RECORD_0126F554: u32 = 0x0126_f554;
const PROPERTY_TEST_004A2020: u32 = 0x004a_2020;
const PROPERTY_ADD_LIGHT_00B71560: u32 = 0x00b7_1560;
const LIST_ADD_00C1B4A0: u32 = 0x00c1_b4a0;
const LIST_MOVE_00B702E0: u32 = 0x00b7_02e0;

// Translated from 00b9f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::AddTriGeom` (Xbox PDB). `thiscall`, `RET 4` (a geometry). Initializes
/// its static `NiPointer` on the first call. Only for a geometry with a +0xc0 object that
/// passes `00b4f580` and a lighting property (type 1 to 12, -1 for none): when the geometry is
/// in `lGeomList` from the fence (or the head) on, its item moves to the fence (`00b702e0`, when
/// there is one) and the property gets the light (`00b71560` with the geometry's bound or the
/// null bound); otherwise, when the property's `004a2020` (0xc) fails or its type is 4, the
/// property gets the light and the geometry is appended through the static (`00c1b4a0`).
pub fn shadow_scene_light_add_tri_geom(e: &mut Engine, this: Ptr<ShadowSceneLight>, geometry: u32) {
    let guard: u32 = e.global(ADD_TRI_GEOM_GUARD);
    if guard & 1 == 0 {
        e.set_global(ADD_TRI_GEOM_GUARD, guard | 1);
        e.set_global(ADD_TRI_GEOM_STATIC, 0u32);
        e.call(ATEXIT, &args![ADD_TRI_GEOM_STATIC_DESTRUCT]);
    }
    let object = e.mem.u32(geometry.wrapping_add(0xc0));
    if object == 0 {
        return;
    }
    if e.call(
        GEOMETRY_TEST_00B4F580,
        &args![object, GEOMETRY_TEST_RECORD_0126F554],
    )
    .u32()
        == 0
    {
        return;
    }
    if geometry.wrapping_add(0x9c) == 0 {
        return;
    }
    let property = e.mem.u32(geometry.wrapping_add(GEOMETRY_PROPERTY));
    let kind = if property != 0 {
        e.mem.u32(property.wrapping_add(0x1c))
    } else {
        u32::MAX
    };
    if kind.wrapping_sub(1) > 0xb {
        return;
    }
    let fence = e.get(this, ShadowSceneLight::kGeomListFence);
    let mut item = if fence != 0 {
        fence
    } else {
        e.mem.u32(this.addr().wrapping_add(GEOM_LIST))
    };
    let mut found = 0;
    while item != 0 {
        let next = e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_pkNext);
        if e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_element) == geometry {
            found = item;
            break;
        }
        item = next;
    }
    let list = this.addr().wrapping_add(GEOM_LIST);
    if found != 0 {
        if fence != 0 {
            e.call(LIST_MOVE_00B702E0, &args![list, found, fence]);
        }
        let bound = bound_or_null(e, geometry);
        e.call(PROPERTY_ADD_LIGHT_00B71560, &args![property, this, bound]);
        return;
    }
    if e.call(PROPERTY_TEST_004A2020, &args![property, 0xcu32])
        .u8()
        != 0
        && kind != 4
    {
        return;
    }
    e.set_global(ADD_TRI_GEOM_STATIC, geometry);
    let bound = bound_or_null(e, geometry);
    e.call(PROPERTY_ADD_LIGHT_00B71560, &args![property, this, bound]);
    e.call(LIST_ADD_00C1B4A0, &args![list, ADD_TRI_GEOM_STATIC]);
    e.set_global(ADD_TRI_GEOM_STATIC, 0u32);
}

// Translated from 00b9f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::ParseObjectAddFrustum` (Xbox PDB). `thiscall`, `RET 4` (an object). Not
/// for the light's shadow scene or an object with flag 0x800000. With planes active, an
/// object without flag 0x800 must be within the frustum (`BoundWithinLightFrustum`). A node's
/// children with a usable bound (or flag 0x800) and without flag 1 are parsed the same way; a
/// tri-based geometry is added (`AddTriGeom`). The plane mask is restored at the end.
pub fn shadow_scene_light_parse_object_add_frustum(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    object: u32,
) {
    let base = this.addr();
    let saved = e.mem.u32(base.wrapping_add(PLANES_ACTIVE));
    parse_frustum_body(e, this, object, saved);
    e.mem.set_u32(base.wrapping_add(PLANES_ACTIVE), saved);
}

fn parse_frustum_body(e: &mut Engine, this: Ptr<ShadowSceneLight>, object: u32, saved: u32) {
    if object == e.get(this, ShadowSceneLight::spShadowScene) {
        return;
    }
    let flags = e.mem.u32(object.wrapping_add(FLAGS));
    if (flags >> 0x17) & 1 != 0 {
        return;
    }
    if saved != 0 && (flags >> 0xb) & 1 == 0 {
        let bound = bound_or_null(e, object);
        if shadow_scene_light_bound_within_light_frustum(e, this, Ptr::new(bound)) == 0 {
            return;
        }
    }
    let node = e.vcall(object, SLOT_IS_NODE, &args![]).u32();
    if node != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE))) {
            let children = e.mem.u32(node.wrapping_add(CHILDREN_BASE));
            let child = e.mem.u32(children.wrapping_add(4 * index));
            if child != 0
                && child_bound_passes(e, child)
                && e.mem.u32(child.wrapping_add(FLAGS)) & 1 == 0
            {
                shadow_scene_light_parse_object_add_frustum(e, this, child);
            }
            index += 1;
        }
    } else if e.vcall(object, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
        shadow_scene_light_add_tri_geom(e, this, object);
    }
}

// Translated from 00b9f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::ParseObjectAddRadius` (Xbox PDB). `thiscall`, `RET 4` (an object). An
/// object without flag 0x800 must be within the light's radius (`BoundWithinLightRadius` with
/// its `NiLight` and 1.0). A node's children with a usable bound (or flag 0x800) and without
/// flag 1 are parsed the same way; a tri-based geometry is added (`AddTriGeom` with what its
/// second `IsTriBasedGeom` answers). The plane mask is restored at the end.
pub fn shadow_scene_light_parse_object_add_radius(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    object: u32,
) {
    let base = this.addr();
    let saved = e.mem.u32(base.wrapping_add(PLANES_ACTIVE));
    parse_radius_body(e, this, object);
    e.mem.set_u32(base.wrapping_add(PLANES_ACTIVE), saved);
}

fn parse_radius_body(e: &mut Engine, this: Ptr<ShadowSceneLight>, object: u32) {
    let flags = e.mem.u32(object.wrapping_add(FLAGS));
    if (flags >> 0xb) & 1 == 0 {
        let bound = bound_or_null(e, object);
        let light = e.get(this, ShadowSceneLight::spLight);
        if shadow_scene_light_bound_within_light_radius(e, this, Ptr::new(bound), light, 1.0) == 0 {
            return;
        }
    }
    let node = e.vcall(object, SLOT_IS_NODE, &args![]).u32();
    if node != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE))) {
            let children = e.mem.u32(node.wrapping_add(CHILDREN_BASE));
            let child = e.mem.u32(children.wrapping_add(4 * index));
            if child != 0
                && child_bound_passes(e, child)
                && e.mem.u32(child.wrapping_add(FLAGS)) & 1 == 0
            {
                shadow_scene_light_parse_object_add_radius(e, this, child);
            }
            index += 1;
        }
    } else if e.vcall(object, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
        let geometry = e.vcall(object, SLOT_IS_TRI_BASED_GEOM, &args![]).u32();
        shadow_scene_light_add_tri_geom(e, this, geometry);
    }
}

/// The light's destructor (`00b9f0c0`) and the sized free (`00aa1460`, `cdecl`).
const SHADOW_SCENE_LIGHT_DESTRUCT: u32 = 0x00b9_f0c0;
const SIZED_FREE: u32 = 0x00aa_1460;

// Translated from 00ba00e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ShadowSceneLight` scalar deleting destructor (unnamed in the engine map). `thiscall`,
/// `RET 4` (the flags): the destructor (`00b9f0c0`), then the 0x250-byte block freed
/// (`00aa1460`) when bit 0 is set; answers `this`.
pub fn fn_00ba00e0(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    flags: u32,
) -> Ptr<ShadowSceneLight> {
    e.call(SHADOW_SCENE_LIGHT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_FREE, &args![this, 0x250u32]);
    }
    this
}

// Translated from 00ba0110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::AddObject` (Xbox PDB). `thiscall`, `RET 4` (an object). A node that is
/// not culled (flag 1) and has a bound with a radius is parsed by frustum (a light with
/// `bCastShadow`) or by radius, except that the light's shadow scene without flag 0x800000
/// goes the geometry way; that way (also for anything else) adds a tri-based geometry
/// (`AddTriGeom`).
pub fn shadow_scene_light_add_object(e: &mut Engine, this: Ptr<ShadowSceneLight>, object: u32) {
    if object == 0 {
        return;
    }
    let node = e.vcall(object, SLOT_IS_NODE, &args![]).u32();
    let mut parse = false;
    if node != 0 && e.mem.u32(object.wrapping_add(FLAGS)) & 1 == 0 {
        let bound = e.mem.u32(node.wrapping_add(WORLD_BOUND));
        if bound != 0 && x87_m32(e.mem.u32(bound.wrapping_add(0xc))) != 0.0 {
            parse = true;
        }
    }
    if parse {
        let cast = e.get(this, ShadowSceneLight::bCastShadow);
        let shadow_scene = cast != 0 && object == e.get(this, ShadowSceneLight::spShadowScene);
        let hidden = (e.mem.u32(object.wrapping_add(FLAGS)) >> 0x17) & 1 != 0;
        if !(shadow_scene && !hidden) {
            if cast != 0 {
                shadow_scene_light_parse_object_add_frustum(e, this, node);
            } else {
                shadow_scene_light_parse_object_add_radius(e, this, node);
            }
            return;
        }
    }
    if e.vcall(object, SLOT_IS_TRI_BASED_GEOM, &args![]).u32() != 0 {
        shadow_scene_light_add_tri_geom(e, this, object);
    }
}

/// `RemoveTriGeom`'s callees: the property's light removal `00b717a0` (`RET 4`) and the list
/// removals `00b9efa0` (engine map: `RemovePos`) and `00b9fcc0` (engine map: `Remove`), both
/// `RET 8` with an out `NiPointer`.
const PROPERTY_REMOVE_LIGHT_00B717A0: u32 = 0x00b7_17a0;
const LIST_REMOVE_POS: u32 = 0x00b9_efa0;
const LIST_REMOVE: u32 = 0x00b9_fcc0;

// Translated from 00ba01a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::RemoveTriGeom` (Xbox PDB). `thiscall`, `RET 8` (a geometry and its list
/// item, or 0). The geometry's lighting property (type 1 to 12) drops the light (`00b717a0`).
/// With an item, it is removed from `lGeomList` at that position (`00b9efa0`, the out
/// `NiPointer` being the geometry's own argument slot, released after); without one, the
/// geometry (held through the item's argument slot) is removed by value (`00b9fcc0`), and
/// both `NiPointer`s are released.
// Translation incomplete: 00ba01a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the geometry temporary) is not translated.
pub fn shadow_scene_light_remove_tri_geom(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    geometry: u32,
    item: u32,
) {
    let property = e.mem.u32(geometry.wrapping_add(GEOMETRY_PROPERTY));
    if property != 0 {
        let property = shader_property(e, property);
        if property != 0 {
            e.call(PROPERTY_REMOVE_LIGHT_00B717A0, &args![property, this]);
        }
    }
    let list = this.addr().wrapping_add(GEOM_LIST);
    e.with_stack(0xc, |e, frame| {
        let geometry_slot = frame.addr();
        let item_slot = geometry_slot + 4;
        let out = geometry_slot + 8;
        e.mem.set_u32(geometry_slot, geometry);
        e.mem.set_u32(item_slot, item);
        // The out slot is the word `PUSH ECX` reserved: it starts as `this`.
        e.mem.set_u32(out, this.addr());
        if item != 0 {
            e.call(LIST_REMOVE_POS, &args![list, geometry_slot, item_slot]);
            let removed = e.mem.u32(geometry_slot);
            if removed != 0 {
                release(e, removed);
            }
            return;
        }
        e.mem.set_u32(item_slot, geometry);
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![geometry.wrapping_add(4)],
        );
        e.call(LIST_REMOVE, &args![list, out, item_slot]);
        let removed = e.mem.u32(out);
        if removed != 0 {
            release(e, removed);
        }
        let held = e.mem.u32(item_slot);
        if held != 0 {
            release(e, held);
        }
    });
}

/// `MTRenderFrustumDepthMap`'s callees and data: the rendering system's `00ba3130`
/// (`thiscall` on `01200088`, `RET 8`); the renderer singleton (`011f4748`) and its virtual
/// slots +0xb4 (read a colour) and +0xac (set a colour); the colour at `011a9bd0` (four
/// words); `BSRenderedTexture::StopOffscreen` (`00b6b260`); `00b6b8d0` (`cdecl`: 7 and the
/// target); `00b6c0d0` (`cdecl`: camera, accumulator, 0); the camera setup `004e9bb0`
/// (`thiscall` on the state object `011f9508`, `RET 4`; its view and projection matrices at
/// +0x980 and +0x9c0); `D3DXMatrixMultiply` (`00ee6de0`, `stdcall`); `00b6b790` (no
/// arguments); the depth pass: the flag `011f91b0`, the renderer's map (`00b6d3f0`, released
/// with `00b6d4c0`), a 16-byte local (`00ba37a0` with the map, `00ba3a00` to destroy), a local
/// built by `004eba20` and destroyed by `004eba90`, the `float` at `010162c0` copied to
/// `012003c4`, `00ba3cb0` (`RET 0xc`, on the object at `[[011f91ac] + 8] + 0x44`) and
/// `00b97550` (`RET 0x18`, on `[011f91ac]`); the debug texture: `004bc320` (`RET 4`),
/// `00b55560` (`cdecl`, 9) and `00bd22c0` (`RET 4`).
const RENDERING_SYSTEM: u32 = 0x0120_0088;
const RENDERING_SYSTEM_00BA3130: u32 = 0x00ba_3130;
const RENDERER_SINGLETON_011F4748: u32 = 0x011f_4748;
const CLEAR_COLOUR_011A9BD0: u32 = 0x011a_9bd0;
const RENDERED_TEXTURE_STOP_OFFSCREEN: u32 = 0x00b6_b260;
const RENDER_TARGET_00B6B8D0: u32 = 0x00b6_b8d0;
const CAMERA_ACCUMULATOR_00B6C0D0: u32 = 0x00b6_c0d0;
const STATE_OBJECT_011F9508: u32 = 0x011f_9508;
const STATE_SET_CAMERA_004E9BB0: u32 = 0x004e_9bb0;
const D3DX_MATRIX_MULTIPLY: u32 = 0x00ee_6de0;
const RENDER_00B6B790: u32 = 0x00b6_b790;
const DEPTH_PASS_FLAG_011F91B0: u32 = 0x011f_91b0;
const RENDERER_00B6D3F0: u32 = 0x00b6_d3f0;
const PASS_LOCAL_00BA37A0: u32 = 0x00ba_37a0;
const PASS_LOCAL_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
const PASS_LIST_004EBA20: u32 = 0x004e_ba20;
const PASS_LIST_DESTRUCT_004EBA90: u32 = 0x004e_ba90;
const DEPTH_VALUE_010162C0: u32 = 0x0101_62c0;
const DEPTH_VALUE_012003C4: u32 = 0x0120_03c4;
const RENDER_MANAGER_011F91AC: u32 = 0x011f_91ac;
const PASS_00BA3CB0: u32 = 0x00ba_3cb0;
const RENDER_00B97550: u32 = 0x00b9_7550;
const DEBUG_TEXTURE_004BC320: u32 = 0x004b_c320;
const DEBUG_00B55560: u32 = 0x00b5_5560;
const DEBUG_00BD22C0: u32 = 0x00bd_22c0;

/// Copies `words` words from `from` to `to` (`MOVSD.REP`).
fn copy_words(e: &mut Engine, to: u32, from: u32, words: u32) {
    for i in 0..words {
        let value = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), value);
    }
}

// Translated from 00b9f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::MTRenderFrustumDepthMap` (Xbox PDB). `thiscall`, `RET 8`: an object
/// (its `NiPointer` at +0xc4 holds the accumulator meanwhile) and the light's index. Selects
/// the rendering system's target 0x11 + index, clears the dynamic shadow map's target in white
/// (the renderer's colour saved and restored around `00b6b8d0`), gives the accumulator to the
/// object and to the shadow camera (`00b6c0d0`), releases the accumulator's +0x1a0 and the
/// object's +0xc4, sets the camera on the state object (`004e9bb0`, with the camera's three
/// rotation rows), copies its view matrix to `kShadowView` with the light's translation turned
/// into the camera frame (negated dot products with the rows) in the last row, its projection
/// to `kShadowProj`, and `kViewProjection = kShadowView * kShadowProj`; runs `00b6b790`; when
/// the depth pass flag (`011f91b0`) is clear, renders the depth pass into a map from the
/// renderer; with `bShowDebugTexture` shows the map; clears `bClipSpacePlanesReady`.
// Translation incomplete: 00b9f780 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the pass locals) is not translated.
pub fn shadow_scene_light_mt_render_frustum_depth_map(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    holder: u32,
    index: u32,
) {
    let base = this.addr();
    e.call(
        RENDERING_SYSTEM_00BA3130,
        &args![RENDERING_SYSTEM, 1u32, index.wrapping_add(0x11)],
    );
    e.with_stack(0x24, |e, colours| {
        let saved = colours.addr();
        let white = saved + 0x10;
        copy_words(e, saved, CLEAR_COLOUR_011A9BD0, 4);
        let renderer = e.global::<u32>(RENDERER_SINGLETON_011F4748);
        e.vcall(renderer, 0xb4, &args![saved]);
        let one = x87_store32(1.0);
        for word in 0..4u32 {
            e.mem.set_u32(white + 4 * word, one);
        }
        let renderer = e.global::<u32>(RENDERER_SINGLETON_011F4748);
        e.vcall(renderer, 0xac, &args![white]);
        let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
        let target = e.call(RENDERED_TEXTURE_STOP_OFFSCREEN, &args![map]).u32();
        e.call(RENDER_TARGET_00B6B8D0, &args![7u32, target]);
        let renderer = e.global::<u32>(RENDERER_SINGLETON_011F4748);
        e.vcall(renderer, 0xac, &args![saved]);
    });
    let accumulator = e.get(this, ShadowSceneLight::spShadowAccumulator);
    let slot = holder.wrapping_add(0xc4);
    let old = e.mem.u32(slot);
    if old != accumulator {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, accumulator);
        if accumulator != 0 {
            e.call(
                IMPORT_INTERLOCKED_INCREMENT,
                &args![accumulator.wrapping_add(4)],
            );
        }
    }
    let camera = e.get(this, ShadowSceneLight::spShadowCamera);
    let accumulator = e.get(this, ShadowSceneLight::spShadowAccumulator);
    e.call(
        CAMERA_ACCUMULATOR_00B6C0D0,
        &args![camera, accumulator, 0u32],
    );
    let accumulator = e.get(this, ShadowSceneLight::spShadowAccumulator);
    let held = e.mem.u32(accumulator.wrapping_add(0x1a0));
    if held != 0 {
        release(e, held);
        e.mem.set_u32(accumulator.wrapping_add(0x1a0), 0);
    }
    let held = e.mem.u32(slot);
    if held != 0 {
        release(e, held);
        e.mem.set_u32(slot, 0);
    }

    let camera = e.get(this, ShadowSceneLight::spShadowCamera);
    let state = e.global::<u32>(STATE_OBJECT_011F9508);
    // The camera's rotation rows, each through a `float` slot.
    let row = |e: &Engine, offsets: [u32; 3]| {
        offsets.map(|offset| x87_quiet_bits(e.mem.u32(camera.wrapping_add(offset))))
    };
    let c = row(e, [0x70, 0x7c, 0x88]);
    let b = row(e, [0x6c, 0x78, 0x84]);
    let a = row(e, [0x68, 0x74, 0x80]);
    e.call(STATE_SET_CAMERA_004E9BB0, &args![state, camera]);
    let light = e.get(this, ShadowSceneLight::spLight);
    e.with_stack(0xb4, |e, frame| {
        let view = frame.addr();
        let projection = view + 0x40;
        let transform = view + 0x80;
        copy_words(e, view, state.wrapping_add(0x980), 0x10);
        copy_words(e, projection, state.wrapping_add(0x9c0), 0x10);
        copy_words(e, transform, light.wrapping_add(0x68), 0xd);
        let tx = x87_load32(e.mem.u32(transform + 0x24));
        let ty = x87_load32(e.mem.u32(transform + 0x28));
        let tz = x87_load32(e.mem.u32(transform + 0x2c));
        let s1 = x87_add(
            x87_add(x87_mul(ty, x87_m32(c[1])), x87_mul(tx, x87_m32(c[0]))),
            x87_mul(tz, x87_m32(c[2])),
        );
        copy_words(e, base + 0x50, view, 0x10);
        let s1 = x87_store32(s1);
        e.mem.set_u32(base + 0x80, x87_store32(-x87_load32(s1)));
        let s2 = x87_add(
            x87_add(x87_mul(ty, x87_m32(b[1])), x87_mul(tx, x87_m32(b[0]))),
            x87_mul(tz, x87_m32(b[2])),
        );
        let s2 = x87_store32(s2);
        e.mem.set_u32(base + 0x84, x87_store32(-x87_load32(s2)));
        let s3 = x87_add(
            x87_add(x87_mul(ty, x87_load32(a[1])), x87_mul(tx, x87_m32(a[0]))),
            x87_mul(tz, x87_m32(a[2])),
        );
        let s3 = x87_store32(s3);
        e.mem.set_u32(base + 0x88, x87_store32(-x87_load32(s3)));
        copy_words(e, base + 0x90, projection, 0x10);
        e.call(
            D3DX_MATRIX_MULTIPLY,
            &args![projection, base + 0x50, base + 0x90],
        );
        copy_words(e, base + 0x10, projection, 0x10);
    });
    e.call(RENDER_00B6B790, &args![]);
    if e.global::<u32>(DEPTH_PASS_FLAG_011F91B0) == 0 {
        let renderer = e.global::<u32>(RENDERER_011F91A8);
        let map = e.call(RENDERER_00B6D3F0, &args![renderer]).u32();
        e.with_stack(0x34, |e, frame| {
            let pass = frame.addr();
            let list = pass + 0x10;
            for byte in 0..3u32 {
                e.mem.set_u8(pass + byte, 0);
            }
            for word in 1..4u32 {
                e.mem.set_u32(pass + 4 * word, 0);
            }
            e.call(PASS_LOCAL_00BA37A0, &args![pass, map]);
            e.call(PASS_LIST_004EBA20, &args![list]);
            let value = x87_quiet_bits(e.global::<u32>(DEPTH_VALUE_010162C0));
            e.set_global(DEPTH_VALUE_012003C4, value);
            let manager = e.global::<u32>(RENDER_MANAGER_011F91AC);
            let inner = e.mem.u32(manager.wrapping_add(8));
            let target = e.mem.u32(inner.wrapping_add(0x44));
            e.call(PASS_00BA3CB0, &args![target, 2u32, pass, 0u32]);
            let shadow_map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
            let state = e.global::<u32>(STATE_OBJECT_011F9508);
            let manager = e.global::<u32>(RENDER_MANAGER_011F91AC);
            e.call(
                RENDER_00B97550,
                &args![manager, 0x11u32, state, shadow_map, shadow_map, list, 1u32],
            );
            let renderer = e.global::<u32>(RENDERER_011F91A8);
            e.call(RENDERER_RELEASE_MAP_00B6D4C0, &args![renderer, map]);
            e.call(PASS_LIST_DESTRUCT_004EBA90, &args![list]);
            e.call(PASS_LOCAL_DESTRUCT_00BA3A00, &args![pass]);
        });
    }
    if e.get(this, ShadowSceneLight::bShowDebugTexture) != 0 {
        let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
        let texture = e.call(DEBUG_TEXTURE_004BC320, &args![map, 0u32]).u32();
        let shower = e.call(DEBUG_00B55560, &args![9u32]).u32();
        e.call(DEBUG_00BD22C0, &args![shower, texture]);
    }
    e.set(this, ShadowSceneLight::bClipSpacePlanesReady, 0);
}

/// `TestFrustumCull`'s callees and data: the shadow camera test `00b9c020` (`RET 8`), the cube
/// face test `00b9de90` (`RET 0x18`: a point, the radius, the culler and the face), the shader
/// manager's accumulator (`00b4f5c0`) whose +8 object's `float` at +0x110 scales the distance,
/// the fade distances (`011f9444`, `011f9448` for the dimmer; `011f944c`, `011f9450` for the
/// shadow dimmer), the elapsed-time step (`011f9410`) and fade time (`011ad834`), and
/// `NiAVObject::GetProperty` (`00a59d30`, `RET 4`).
const LIGHT_CAMERA_TEST_00B9C020: u32 = 0x00b9_c020;
const CUBE_FACE_TEST_00B9DE90: u32 = 0x00b9_de90;
const SHADER_MANAGER_GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const DIMMER_NEAR_011F9444: u32 = 0x011f_9444;
const DIMMER_FAR_011F9448: u32 = 0x011f_9448;
const SHADOW_NEAR_011F944C: u32 = 0x011f_944c;
const SHADOW_FAR_011F9450: u32 = 0x011f_9450;
const ELAPSED_STEP_011F9410: u32 = 0x011f_9410;
const FADE_TIME_011AD834: u32 = 0x011a_d834;
const GET_PROPERTY: u32 = 0x00a5_9d30;

/// `FCOM`-style status bits of comparing `a` (`ST0`) with `b`: C0 (0x01) for less, C3 (0x40)
/// for equal, C3 | C2 | C0 (0x45) for unordered, none for greater.
fn x87_compare(a: f64, b: f64) -> u8 {
    if a.is_nan() || b.is_nan() {
        0x45
    } else if a < b {
        0x01
    } else if a == b {
        0x40
    } else {
        0
    }
}

/// The parity flag of `TEST AH,mask` (set for an even number of bits).
fn parity(ah: u8, mask: u8) -> bool {
    (ah & mask).count_ones() % 2 == 0
}

// Translated from 00b9e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::TestFrustumCull` (Xbox PDB). `thiscall`, `RET 4` (the culler). The
/// frustum-cull word: for a shadow-casting light with a shadow camera, 0 when `00b9c020` (the
/// culler's camera and the shadow camera) passes and 0xff otherwise; else 0xff when the
/// `NiLight`'s sphere is outside one of the culler's active planes (+0x2c, mask at +0x8c),
/// and, when the sphere crosses a plane, the light casts shadows, `[011f91c0] >= 3`, bit
/// 0x10 of `[011f91d8]` is set and the light uses a cube map, one bit per cube face that
/// `00b9de90` rejects. Unless culled (0xff), the distance from the culler's camera less the
/// radius (scaled by the accumulator's object's +0x110 when there is one) fades the dimmer
/// (1 below `[011f9444]` or with `[011f9448]` zero, 0 and culled beyond `[011f9448]`, linear
/// between) and, for a shadow caster, the shadow dimmer the same way with `[011f944c]` and
/// `[011f9450]`; the shadow dimmer is scaled by the shadow scene's property 2 (+0x3c), and by
/// the fade-in `elapsed / [011ad834]` (elapsed time advanced by `[011f9410]`, clamped to 1;
/// reversed for a zero target). When the culled state did not change and a dimmer crossed
/// zero, or the state changed, every geometry's lighting property gets 1 at +0x74 and, with
/// the old shadow dimmer zero, the shadow map is handed back and released. The word and both
/// dimmers are stored.
pub fn shadow_scene_light_test_frustum_cull(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    culler: u32,
) {
    let ni_light = e.get(this, ShadowSceneLight::spLight);
    let x = e.mem.u32(ni_light.wrapping_add(WORLD_TRANSLATE));
    let y = e.mem.u32(ni_light.wrapping_add(WORLD_TRANSLATE + 4));
    let z = e.mem.u32(ni_light.wrapping_add(WORLD_TRANSLATE + 8));
    let radius_bits = x87_quiet_bits(e.mem.u32(ni_light.wrapping_add(0xe0)));
    let radius = x87_load32(radius_bits);
    let cast = e.get(this, ShadowSceneLight::bCastShadow);
    let mut result = 0u32;
    let mut inside = true;
    // `NV_LIGHT_CHECK` (diagnostic only): why the light was culled.
    let mut reason = String::new();
    let camera = e.get(this, ShadowSceneLight::spShadowCamera);
    if cast != 0 && camera != 0 {
        let culler_camera = e.mem.u32(culler.wrapping_add(0xc));
        let passed = e
            .call(
                LIGHT_CAMERA_TEST_00B9C020,
                &args![this, culler_camera, camera],
            )
            .u8();
        result = if passed != 0 { 0 } else { 0xff };
        if passed == 0 && light_check::on() {
            reason = "the shadow camera test 00b9c020 failed".into();
        }
    } else {
        let planes: Vec<u32> = (0..0x19u32)
            .map(|i| e.mem.u32(culler.wrapping_add(0x2c + 4 * i)))
            .collect();
        let mask = planes[0x18];
        let mut outside = false;
        for plane in 0..6usize {
            if mask & (1 << plane) == 0 {
                continue;
            }
            let [nx, ny, nz, d] = [
                planes[4 * plane],
                planes[4 * plane + 1],
                planes[4 * plane + 2],
                planes[4 * plane + 3],
            ];
            let dot = x87_add(
                x87_add(
                    x87_mul(x87_load32(ny), x87_load32(y)),
                    x87_mul(x87_load32(x), x87_m32(nx)),
                ),
                x87_mul(x87_load32(nz), x87_load32(z)),
            );
            let stored = x87_store32(dot);
            let distance = x87_store32(x87_sub(x87_load32(stored), x87_load32(d)));
            // `FCHS; FCOMPP; TEST AH,1; JZ`: outside unless `-radius < distance`.
            if x87_compare(-radius, x87_load32(distance)) & 1 == 0 {
                if light_check::on() {
                    reason = format!(
                        "outside plane {plane} (n ({}, {}, {}) d {}): distance {} <= -radius",
                        f32::from_bits(nx),
                        f32::from_bits(ny),
                        f32::from_bits(nz),
                        f32::from_bits(d),
                        f32::from_bits(distance)
                    );
                }
                outside = true;
                break;
            }
            let crossing = x87_store32(x87_sub(dot, x87_load32(d)));
            if x87_compare(0.0, x87_m32(crossing)) & 0x41 == 0 {
                inside = false;
            }
        }
        if outside {
            result = 0xff;
        } else if !inside
            && cast != 0
            && (e.global::<u32>(RENDER_LEVEL_011F91C0) as i32) >= 3
            && e.global::<u8>(RENDER_FLAGS_011F91D8) & 0x10 != 0
            && e.get(this, ShadowSceneLight::bCubeShadowMap) != 0
        {
            let mut face = 0u32;
            while (face as u16) < 6 {
                let hit = e
                    .call(
                        CUBE_FACE_TEST_00B9DE90,
                        &args![this, x, y, z, radius_bits, culler, face],
                    )
                    .u8();
                if hit == 0 {
                    result |= 1 << face;
                }
                face += 1;
            }
        }
    }

    let mut dimmer = x87_quiet_bits(e.get(this, ShadowSceneLight::fLODDimmer).to_bits());
    let mut shadow = x87_quiet_bits(e.get(this, ShadowSceneLight::fShadowLODDimmer).to_bits());
    let one = x87_store32(1.0);
    let zero = x87_store32(0.0);
    if result as u16 != 0xff {
        let culler_camera = e.mem.u32(culler.wrapping_add(0xc));
        let mut delta = [0u32; 3];
        for (i, p) in [x, y, z].iter().enumerate() {
            let c = e
                .mem
                .u32(culler_camera.wrapping_add(WORLD_TRANSLATE + 4 * i as u32));
            delta[i] = x87_store32(x87_sub(x87_load32(*p), x87_m32(c)));
        }
        let squared = x87_store32(squared_length(
            x87_load32(delta[0]),
            x87_load32(delta[1]),
            x87_load32(delta[2]),
        ));
        let root = sqrt_stored(e, squared);
        let mut distance = x87_store32(x87_sub(x87_load32(root), x87_m32(radius_bits)));
        let accumulator = e.call(SHADER_MANAGER_GET_ACCUMULATOR, &args![]).u32();
        if e.mem.u32(accumulator.wrapping_add(8)) != 0 {
            let accumulator = e.call(SHADER_MANAGER_GET_ACCUMULATOR, &args![]).u32();
            let object = e.mem.u32(accumulator.wrapping_add(8));
            let scale = x87_load32(e.mem.u32(object.wrapping_add(0x110)));
            distance = x87_store32(x87_mul(scale, x87_m32(distance)));
        }
        let d = x87_load32(distance);
        let near = x87_load32(e.global::<u32>(DIMMER_NEAR_011F9444));
        let mut fade_shadow = true;
        if x87_compare(near, d) & 0x41 == 0 {
            // Closer than the near distance.
            dimmer = one;
        } else {
            let far = x87_load32(e.global::<u32>(DIMMER_FAR_011F9448));
            if !parity(x87_compare(far, 0.0), 0x44) {
                dimmer = one;
            } else if !parity(x87_compare(far, d), 5) {
                // Beyond the far distance: culled.
                if light_check::on() {
                    reason = format!("beyond the fade distance: {d} > far {far} (near {near})");
                }
                result = 0xff;
                dimmer = zero;
                shadow = zero;
                fade_shadow = false;
            } else {
                let q = x87_div(x87_sub(d, near), x87_sub(far, near));
                dimmer = x87_store32(x87_sub(1.0, q));
                if parity(x87_compare(0.0, x87_m32(dimmer)), 5) {
                    fade_shadow = false;
                }
            }
        }
        if fade_shadow && e.get(this, ShadowSceneLight::bCastShadow) != 0 {
            let near = x87_load32(e.global::<u32>(SHADOW_NEAR_011F944C));
            if x87_compare(near, d) & 0x41 == 0 {
                shadow = one;
            } else {
                let far = x87_load32(e.global::<u32>(SHADOW_FAR_011F9450));
                if !parity(x87_compare(far, 0.0), 0x44) {
                    shadow = one;
                } else if x87_compare(d, far) & 0x41 == 0 {
                    shadow = zero;
                } else {
                    let q = x87_div(x87_sub(d, near), x87_sub(far, near));
                    shadow = x87_store32(x87_sub(1.0, q));
                }
            }
        }
        {
            let scene = e.get(this, ShadowSceneLight::spShadowScene);
            if scene != 0 {
                let property = e.call(GET_PROPERTY, &args![scene, 2u32]).u32();
                if property != 0 {
                    let factor = x87_load32(e.mem.u32(property.wrapping_add(0x3c)));
                    shadow = x87_store32(x87_mul(factor, x87_m32(shadow)));
                }
            }
            let step = e.global::<u32>(ELAPSED_STEP_011F9410);
            let elapsed = x87_store32(x87_add(
                x87_load32(
                    e.get(this, ShadowSceneLight::fShadowLODDimmerElapsed)
                        .to_bits(),
                ),
                x87_m32(step),
            ));
            let elapsed = x87_quiet_bits(elapsed);
            e.set(
                this,
                ShadowSceneLight::fShadowLODDimmerElapsed,
                f32::from_bits(elapsed),
            );
            let time = e.global::<u32>(FADE_TIME_011AD834);
            let mut fraction = x87_store32(x87_div(x87_load32(elapsed), x87_m32(time)));
            if x87_compare(x87_load32(fraction), 1.0) & 0x41 == 0 {
                fraction = one;
            }
            let fraction = x87_quiet_bits(fraction);
            let target = e
                .get(this, ShadowSceneLight::fShadowLODDimmerTarget)
                .to_bits();
            if !parity(x87_compare(0.0, x87_m32(target)), 0x44) {
                let reverse = x87_sub(1.0, x87_load32(fraction));
                shadow = x87_store32(x87_mul(reverse, x87_load32(shadow)));
            } else {
                shadow = x87_store32(x87_mul(x87_load32(fraction), x87_m32(shadow)));
            }
        }
    }

    // `L_edd0`: the comparisons against the 0.0 left on the x87 stack.
    let frustum = e.get(this, ShadowSceneLight::usFrustumCull);
    let old_dimmer = e.get(this, ShadowSceneLight::fLODDimmer).to_bits();
    let old_shadow = e.get(this, ShadowSceneLight::fShadowLODDimmer).to_bits();
    let same_state = (frustum == 0xff) == (result as u16 == 0xff);
    let mark = !same_state || {
        // A dimmer that was 0 and is now above 0, or the other way round (`TEST AH,0x44`
        // isolates equality, `TEST AH,5` "less").
        let shadow_value = x87_load32(shadow);
        let to_old_dimmer = x87_compare(0.0, x87_m32(old_dimmer));
        let to_old_shadow = x87_compare(0.0, x87_m32(old_shadow));
        let to_dimmer = x87_compare(0.0, x87_m32(dimmer));
        let to_shadow = x87_compare(0.0, shadow_value);
        (!parity(to_old_dimmer, 0x44) && !parity(to_dimmer, 5))
            || (!parity(to_old_dimmer, 5) && !parity(to_dimmer, 0x44))
            || (!parity(to_old_shadow, 0x44) && !parity(to_shadow, 5))
            || (!parity(to_old_shadow, 5) && !parity(to_shadow, 0x44))
    };
    if mark {
        let mut item = e.mem.u32(this.addr().wrapping_add(GEOM_LIST));
        while item != 0 {
            let geometry = e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_element);
            item = e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_pkNext);
            if geometry == 0 {
                continue;
            }
            let property = e.mem.u32(geometry.wrapping_add(GEOMETRY_PROPERTY));
            if property != 0 && shader_property(e, property) != 0 {
                e.mem.set_u8(property.wrapping_add(0x74), 1);
            }
        }
        if !parity(x87_compare(0.0, x87_m32(old_shadow)), 0x44) {
            let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
            if map != 0 {
                let renderer = e.global::<u32>(RENDERER_011F91A8);
                e.call(RENDERER_RELEASE_MAP_00B6D4C0, &args![renderer, map]);
                let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
                if map != 0 {
                    release(e, map);
                    e.set(this, ShadowSceneLight::spDynamicShadowMap, 0);
                }
            }
        }
    }
    if result as u16 == 0xff && light_check::on() {
        light_check::culled(e, this.addr(), culler, &reason);
    }
    e.set(this, ShadowSceneLight::usFrustumCull, result as u16);
    e.set(
        this,
        ShadowSceneLight::fLODDimmer,
        f32::from_bits(x87_quiet_bits(dimmer)),
    );
    e.set(
        this,
        ShadowSceneLight::fShadowLODDimmer,
        f32::from_bits(x87_quiet_bits(shadow)),
    );
}

/// `MTSetupFrustumDepthMap`'s callees and data: the dimmer limit (`double` 0.05 at
/// `01082cb0`), the sun direction (`011a9484`, three `float`s) and the fallback up vector
/// (`011a946c`), the small-cross bounds (`double`s 0.01 at `01031148`, -0.01 at `010b7ea0`),
/// `NiPoint3::UnitCross` (`0053d1a0`, `RET 8`: out, other), `NiPoint3::Length` (`00457990`,
/// `ST0`), `NiFrustum::NiFrustum` (`00a71b70`, `RET 4`), `NiObjectNET::GetExtraData`
/// (`00a5bdd0`, `RET 4`) of the name at `012031e4`, the extra data's scaled point
/// (`0045bb20`, `RET 8`: out, scale), the larger and smaller of two `float`s (`00404010`,
/// `0040ebd0`, `cdecl`, `ST0`), the scale 1.5 (`01016ff0`), the cap 256.0 (`float`,
/// `01032adc`), 150.0 (`0106c948`), 10.0 (`01020758`), 400.0 (`0106b178`), 0.2
/// (`01018a90`); the accumulator source `00b540f0` (no arguments) and `NiPointer`
/// assignment `0066b0d0`; `NiCamera::NiCamera` (`00a712f0`, 0x114 bytes),
/// `LookAtWorldPoint` (`00a701b0`, `RET 8`), `SetViewFrustum` (`00a6faf0`, `RET 4`), the view
/// port setter `00712e60` (`RET 4`), the near/far fit `00a70080` (`RET 4`);
/// `NiFrustumPlanes::Set` (`00a74e10`, `RET 8`); the point add `0063c8a0` (`RET 4`); the
/// rendering system's `AddAccumTask` (`00ba3390`, `RET 0x24`) and `SetThreadStage`
/// (`00ba30f0`, `RET 8`); the lit-node map setter `00b5aee0` (`RET 4`).
const SHADOW_DIMMER_LIMIT_01082CB0: u32 = 0x0108_2cb0;
const SUN_DIRECTION_011A9484: u32 = 0x011a_9484;
const FALLBACK_UP_011A946C: u32 = 0x011a_946c;
const SMALL_POSITIVE_01031148: u32 = 0x0103_1148;
const SMALL_NEGATIVE_010B7EA0: u32 = 0x010b_7ea0;
const POINT_UNIT_CROSS: u32 = 0x0053_d1a0;
const POINT_LENGTH: u32 = 0x0045_7990;
const POINT_NORMALIZE_004A0C10: u32 = 0x004a_0c10;
const NI_FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const EXTRA_DATA_NAME_012031E4: u32 = 0x0120_31e4;
const EXTRA_SCALED_POINT_0045BB20: u32 = 0x0045_bb20;
const FLOAT_MAX: u32 = 0x0040_4010;
const FLOAT_MIN: u32 = 0x0040_ebd0;
const EXTENT_SCALE_01016FF0: u32 = 0x0101_6ff0;
const EXTENT_CAP_01032ADC: u32 = 0x0103_2adc;
const FOV_SCALE_0106C948: u32 = 0x0106_c948;
const SUN_OFFSET_SCALE_01020758: u32 = 0x0102_0758;
const FAR_EXTRA_0106B178: u32 = 0x0106_b178;
const NEAR_SHARE_01018A90: u32 = 0x0101_8a90;
const ACCUMULATOR_SOURCE_00B540F0: u32 = 0x00b5_40f0;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
const CAMERA_LOOK_AT_WORLD_POINT: u32 = 0x00a7_01b0;
const CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
const CAMERA_SET_VIEW_PORT_00712E60: u32 = 0x0071_2e60;
const CAMERA_FIT_00A70080: u32 = 0x00a7_0080;
const FRUSTUM_PLANES_SET: u32 = 0x00a7_4e10;
const POINT_ADD_0063C8A0: u32 = 0x0063_c8a0;
const RENDERING_ADD_ACCUM_TASK: u32 = 0x00ba_3390;
const RENDERING_SET_THREAD_STAGE: u32 = 0x00ba_30f0;
const NODE_SET_SHADOW_MAP_00B5AEE0: u32 = 0x00b5_aee0;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const OBJECT_UPDATE_00A59C60: u32 = 0x00a5_9c60;
const SCRIPT_TEST_005BF7B0: u32 = 0x005b_f7b0;

/// The 9-byte local of the object update `00a59c60` (a `float` 0.0 and five zero bytes).
fn update_object(e: &mut Engine, object: u32) {
    e.with_stack(0xc, |e, local| {
        e.mem.set_u32(local.addr(), x87_store32(0.0));
        for byte in 4..9u32 {
            e.mem.set_u8(local.addr() + byte, 0);
        }
        e.call(OBJECT_UPDATE_00A59C60, &args![object, local]);
    });
}

/// A `float` through two `float` slots (`FLD`/`FSTP` twice).
fn copy_float(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

// Translated from 00b9dfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::MTSetupFrustumDepthMap` (Xbox PDB). `thiscall`, `RET 8`: an unused word
/// and the light's index. A light with a high-level cube shadow map (`bCastShadow`,
/// `[011f91c0] >= 3`, bit 0x10 of `[011f91d8]`) or a shadow spotlight gets a dynamic shadow map
/// from the renderer when it has none (its `NiLight` culled when there is none to get). Unless
/// it is that kind of light, not frustum-culled (0xff), with a shadow dimmer above 0.05 and a
/// map, its map and accumulator are released. Otherwise the direction to the shadow scene and
/// the up vector (the reverse sun direction, or `011a946c` when nearly parallel) give the
/// camera axes, an orthographic `NiFrustum` sized by the scene's extents (the extra data named
/// at `012031e4`, 1.5 times, at least the bound's radius, at most 256) and the spotlight FOV
/// (150 times width over distance); for a shadow caster passing the script test `005bf7b0`,
/// the accumulator (`00b540f0`) and a shadow camera (a new `NiCamera` when there is none) are
/// set up to look at the scene (moved by ten times the sun direction when the extents' x plus
/// y are below their z), the camera planes set with the far plane extended by 400, and the
/// accumulation task added. The thread stage of the index is set last.
// Translation incomplete: 00b9dfc0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the frustum local and the new camera) is not translated.
pub fn shadow_scene_light_mt_setup_frustum_depth_map(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    _unused_0: u32,
    index: u32,
) {
    let high_level = e.get(this, ShadowSceneLight::bCastShadow) != 0
        && (e.global::<u32>(RENDER_LEVEL_011F91C0) as i32) >= 3
        && e.global::<u8>(RENDER_FLAGS_011F91D8) & 0x10 != 0;
    let flag = high_level || e.get(this, ShadowSceneLight::bShadowSpotlight) != 0;
    if e.get(this, ShadowSceneLight::spDynamicShadowMap) == 0 {
        let renderer = e.global::<u32>(RENDERER_011F91A8);
        let map = e.call(RENDERER_00B6D3F0, &args![renderer]).u32();
        if map == 0 {
            let ni_light = e.get(this, ShadowSceneLight::spLight);
            let flags = e.mem.u32(ni_light.wrapping_add(FLAGS));
            e.mem.set_u32(ni_light.wrapping_add(FLAGS), flags | 1);
        }
        e.call(NODE_SET_SHADOW_MAP_00B5AEE0, &args![this, map]);
    }
    let dimmer = x87_load32(e.get(this, ShadowSceneLight::fShadowLODDimmer).to_bits());
    let limit: f64 = e.global(SHADOW_DIMMER_LIMIT_01082CB0);
    let proceed = flag
        && e.get(this, ShadowSceneLight::usFrustumCull) != 0xff
        && x87_compare(dimmer, limit) & 0x41 == 0
        && e.get(this, ShadowSceneLight::spDynamicShadowMap) != 0;
    if !proceed {
        let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
        if map != 0 {
            let renderer = e.global::<u32>(RENDERER_011F91A8);
            e.call(RENDERER_RELEASE_MAP_00B6D4C0, &args![renderer, map]);
            let map = e.get(this, ShadowSceneLight::spDynamicShadowMap);
            if map != 0 {
                release(e, map);
                e.set(this, ShadowSceneLight::spDynamicShadowMap, 0);
            }
            let accumulator = e.get(this, ShadowSceneLight::spShadowAccumulator);
            if accumulator != 0 {
                release(e, accumulator);
                e.set(this, ShadowSceneLight::spShadowAccumulator, 0);
            }
        }
    } else {
        setup_frustum_main(e, this, index);
    }
    e.call(
        RENDERING_SET_THREAD_STAGE,
        &args![RENDERING_SYSTEM, 0u32, index.wrapping_add(0x11)],
    );
}

/// The main path of `MTSetupFrustumDepthMap` (the frame's locals in one block).
fn setup_frustum_main(e: &mut Engine, this: Ptr<ShadowSceneLight>, index: u32) {
    let base = this.addr();
    e.with_stack(0xd0, |e, frame| {
        let f = frame.addr();
        // Locals: D (+0x00), R (+0x0c), U (+0x18), cross out (+0x24), up (+0x30), P (+0x3c),
        // Q (+0x48), extents (+0x54), the NiFrustum (+0x60, 0x1c bytes), the view port
        // (+0x7c), the light transform (+0x8c, 0x34 bytes), an out point (+0xc0).
        let d = f;
        let r = f + 0x0c;
        let u = f + 0x18;
        let cross_out = f + 0x24;
        let up = f + 0x30;
        let p = f + 0x3c;
        let q = f + 0x48;
        let extents = f + 0x54;
        let frustum = f + 0x60;
        let port = f + 0x7c;
        let transform = f + 0x8c;
        let out = f + 0xc0;
        let ni_light = e.get(this, ShadowSceneLight::spLight);
        copy_words(e, transform, ni_light.wrapping_add(0x68), 0xd);
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        for i in 0..3u32 {
            let s = x87_load32(e.mem.u32(scene.wrapping_add(WORLD_TRANSLATE + 4 * i)));
            let t = e.mem.u32(transform + 0x24 + 4 * i);
            let value = x87_quiet_bits(x87_store32(x87_sub(s, x87_m32(t))));
            e.mem.set_u32(d + 4 * i, value);
        }
        e.call(POINT_NORMALIZE_004A0C10, &args![d]);
        for i in 0..3u32 {
            let sun = x87_load32(e.global::<u32>(SUN_DIRECTION_011A9484 + 4 * i));
            e.mem.set_u32(up + 4 * i, x87_quiet_bits(x87_store32(-sun)));
        }
        e.call(POINT_UNIT_CROSS, &args![d, r, up]);
        let k: f64 = e.global(SMALL_POSITIVE_01031148);
        let m: f64 = e.global(SMALL_NEGATIVE_010B7EA0);
        let small = (0..3u32).all(|i| {
            let v = x87_load32(e.mem.u32(r + 4 * i));
            m < v && v < k
        });
        if small {
            copy_words(e, up, FALLBACK_UP_011A946C, 3);
            let result = e.call(POINT_UNIT_CROSS, &args![d, cross_out, up]).u32();
            copy_words(e, r, result, 3);
        }
        let result = e.call(POINT_UNIT_CROSS, &args![d, cross_out, r]).u32();
        copy_words(e, u, result, 3);
        e.call(NI_FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let name = e.global::<u32>(EXTRA_DATA_NAME_012031E4);
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        e.mem.set_u8(frustum + 0x18, 1);
        let extra = e.call(GET_EXTRA_DATA, &args![scene, name]).u32();
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        copy_words(e, d, scene.wrapping_add(WORLD_TRANSLATE), 3);
        let source = if extra != 0 {
            let scale = copy_float(e, scene.wrapping_add(0x98));
            e.call(
                EXTRA_SCALED_POINT_0045BB20,
                &args![extra.wrapping_add(0x18), cross_out, scale],
            )
            .u32()
        } else {
            POINT_POSITION_011F426C
        };
        copy_words(e, extents, source, 3);
        let bound = bound_or_null(e, scene);
        let radius = copy_float(e, bound.wrapping_add(0xc));
        let factor: f64 = e.global(EXTENT_SCALE_01016FF0);
        let mut larger = radius;
        for i in [2u32, 1, 0] {
            let scaled = x87_quiet_bits(x87_store32(x87_mul(
                x87_load32(e.mem.u32(extents + 4 * i)),
                factor,
            )));
            let value = e.call(FLOAT_MAX, &args![scaled, larger]).st0;
            larger = x87_store32(value);
        }
        let cap = x87_quiet_bits(e.global::<u32>(EXTENT_CAP_01032ADC));
        let width = x87_store32(e.call(FLOAT_MIN, &args![cap, larger]).st0);
        let ni_light = e.get(this, ShadowSceneLight::spLight);
        let mut offset = [0u32; 3];
        for i in 0..3u32 {
            let l = x87_load32(e.mem.u32(ni_light.wrapping_add(WORLD_TRANSLATE + 4 * i)));
            offset[i as usize] = x87_store32(x87_sub(l, x87_m32(e.mem.u32(d + 4 * i))));
        }
        for i in 0..3u32 {
            e.mem.set_u32(q + 4 * i, x87_quiet_bits(offset[i as usize]));
        }
        let length = x87_store32(e.call(POINT_LENGTH, &args![q]).st0);
        let w = x87_load32(width);
        let fov_scale: f64 = e.global(FOV_SCALE_0106C948);
        let fov = x87_store32(x87_mul(x87_div(w, x87_m32(length)), fov_scale));
        e.set(this, ShadowSceneLight::fSpotlightFOV, f32::from_bits(fov));
        let negative = x87_store32(-w);
        let negative_value = x87_quiet_bits(negative);
        let w_bits = x87_store32(w);
        // left, right, top, bottom, near, far
        e.mem.set_u32(frustum, negative_value);
        e.mem.set_u32(frustum + 0xc, negative_value);
        e.mem.set_u32(frustum + 4, w_bits);
        e.mem.set_u32(frustum + 8, w_bits);
        let one = x87_store32(1.0);
        let zero = x87_store32(0.0);
        e.mem.set_u32(frustum + 0x10, one);
        e.mem
            .set_u32(frustum + 0x14, copy_float(e, ni_light.wrapping_add(0xe0)));
        e.mem.set_u32(port, zero);
        e.mem.set_u32(port + 0xc, zero);
        e.mem.set_u32(port + 4, one);
        e.mem.set_u32(port + 8, one);
        if e.get(this, ShadowSceneLight::bCastShadow) == 0
            || e.call(SCRIPT_TEST_005BF7B0, &args![]).u8() == 0
        {
            return;
        }
        let accumulator = e.call(ACCUMULATOR_SOURCE_00B540F0, &args![]).u32();
        e.call(NI_POINTER_ASSIGN, &args![base + 0x20c, accumulator]);
        let accumulator = e.get(this, ShadowSceneLight::spShadowAccumulator);
        e.mem.set_u32(accumulator.wrapping_add(0x19c), 7);
        if e.get(this, ShadowSceneLight::spShadowCamera) == 0 {
            let memory = e.call(NEW_OBJECT, &args![0x114u32]).u32();
            let camera = if memory != 0 {
                e.call(NI_CAMERA_CONSTRUCT, &args![memory]).u32()
            } else {
                0
            };
            e.call(NI_POINTER_ASSIGN, &args![base + 0x140, camera]);
        }
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        copy_words(e, camera.wrapping_add(0x58), transform + 0x24, 3);
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        update_object(e, camera);
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        copy_words(e, p, scene.wrapping_add(WORLD_TRANSLATE), 3);
        let name = e.global::<u32>(EXTRA_DATA_NAME_012031E4);
        let extra = e.call(GET_EXTRA_DATA, &args![scene, name]).u32();
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        let scale = copy_float(e, scene.wrapping_add(0x98));
        let moved = e
            .call(
                EXTRA_SCALED_POINT_0045BB20,
                &args![extra.wrapping_add(0xc), out, scale],
            )
            .u32();
        e.call(POINT_ADD_0063C8A0, &args![p, moved]);
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        let scale = x87_quiet_bits(e.mem.u32(scene.wrapping_add(0x98)));
        e.call(
            EXTRA_SCALED_POINT_0045BB20,
            &args![extra.wrapping_add(0x18), q, scale],
        );
        let qz = x87_load32(e.mem.u32(q + 8));
        let sum = x87_add(x87_load32(e.mem.u32(q + 4)), x87_m32(e.mem.u32(q)));
        if sum < qz {
            let ten: f64 = x87_load64(e.global::<u64>(SUN_OFFSET_SCALE_01020758));
            let sx = x87_load32(e.global::<u32>(SUN_DIRECTION_011A9484));
            e.mem.set_u32(q, x87_store32(x87_mul(sx, ten)));
            let sy = x87_load32(e.global::<u32>(SUN_DIRECTION_011A9484 + 4));
            e.mem.set_u32(q + 4, x87_store32(x87_mul(sy, ten)));
            let sz = e.global::<u32>(SUN_DIRECTION_011A9484 + 8);
            e.mem.set_u32(q + 8, x87_store32(x87_mul(ten, x87_m32(sz))));
            let px = x87_store32(x87_add(x87_load32(e.mem.u32(q)), x87_m32(e.mem.u32(p))));
            e.mem.set_u32(p, px);
            let py = x87_store32(x87_add(
                x87_load32(e.mem.u32(p + 4)),
                x87_m32(e.mem.u32(q + 4)),
            ));
            e.mem.set_u32(p + 4, py);
            let pz = x87_store32(x87_add(
                x87_load32(e.mem.u32(p + 8)),
                x87_m32(e.mem.u32(q + 8)),
            ));
            e.mem.set_u32(p + 8, pz);
        }
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        e.call(CAMERA_LOOK_AT_WORLD_POINT, &args![camera, p, u]);
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        e.call(CAMERA_SET_VIEW_FRUSTUM, &args![camera, frustum]);
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        e.call(CAMERA_SET_VIEW_PORT_00712E60, &args![camera, port]);
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        update_object(e, camera);
        let saved_near = copy_float(e, frustum + 0x10);
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        let bound = bound_or_null(e, scene);
        let saved_far = copy_float(e, frustum + 0x14);
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        e.call(CAMERA_FIT_00A70080, &args![camera, bound]);
        let depth_pass = e.global::<u32>(DEPTH_PASS_FLAG_011F91B0);
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        copy_words(e, frustum, camera.wrapping_add(0xdc), 7);
        let extra_far: f64 = e.global(FAR_EXTRA_0106B178);
        let far = x87_store32(x87_add(x87_load32(e.mem.u32(frustum + 0x14)), extra_far));
        e.mem.set_u32(frustum + 0x14, far);
        if depth_pass == 0 {
            let share: f64 = e.global(NEAR_SHARE_01018A90);
            let near = x87_store32(x87_mul(x87_load32(far), share));
            e.mem.set_u32(frustum + 0x10, near);
        }
        e.call(
            FRUSTUM_PLANES_SET,
            &args![base + 0x144, frustum, camera.wrapping_add(0x68)],
        );
        e.mem.set_u32(frustum + 0x10, x87_quiet_bits(saved_near));
        e.mem.set_u32(frustum + 0x14, x87_quiet_bits(saved_far));
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        e.call(CAMERA_SET_VIEW_FRUSTUM, &args![camera, frustum]);
        let accumulator = e.get(this, ShadowSceneLight::spShadowAccumulator);
        fn_00b9cfd0(e, Ptr::new(accumulator), base);
        let accumulator = e.get(this, ShadowSceneLight::spShadowAccumulator);
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        let camera = e.get(this, ShadowSceneLight::spShadowCamera);
        e.call(
            RENDERING_ADD_ACCUM_TASK,
            &args![
                RENDERING_SYSTEM,
                camera,
                0u32,
                scene,
                0u32,
                0u32,
                accumulator,
                0u32,
                index.wrapping_add(0x11),
                0u32
            ],
        );
    });
}

/// `00b9c020`'s callees: the `NiCullingProcess`-like local's constructor (`00a69400`, `RET
/// 4`), its frustum set-up from a camera's frustum (`00a694a0`, `RET 4`; it fills the six
/// planes and the active mask at +0x2c of the local from the camera at +0xc) and its
/// destructor (`00a693e0`).
const CULLER_CONSTRUCT_00A69400: u32 = 0x00a6_9400;
const CULLER_SET_FRUSTUM_00A694A0: u32 = 0x00a6_94a0;
const CULLER_DESTROY_00A693E0: u32 = 0x00a6_93e0;
/// The frustum's edges as pairs of its five points (the camera position, then the four
/// far corners), as the code's byte table builds them.
const FRUSTUM_EDGES: [(usize, usize); 8] = [
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 2),
    (1, 3),
    (2, 4),
    (3, 4),
];

/// A product stored as a `float` (`FMUL` then `FSTP dword`).
fn mul32(a: f64, b: f64) -> u32 {
    x87_store32(x87_mul(a, b))
}

/// A sum or difference of two `float`s stored as a `float`.
fn add32(a: u32, b: u32) -> u32 {
    x87_store32(x87_add(x87_load32(a), x87_m32(b)))
}

fn sub32(a: u32, b: u32) -> u32 {
    x87_store32(x87_sub(x87_load32(a), x87_m32(b)))
}

/// The five points of a camera's view volume `00b9c020` builds, in the exe's order of
/// operations (every product and sum stored as a `float`): the world translation (+0x8c,
/// copied as words) and the four far corners `t + f*r0 +/- f*u*r1 +/- f*s*r2`, `r0..r2`
/// being the rotation's columns (+0x68 ... +0x88, read as rows of three), `f` the far
/// distance (+0xf0), `u` the frustum's +0xe4 and `s` its +0xe0.
fn camera_points(e: &mut Engine, camera: u32) -> [[u32; 3]; 5] {
    let m = |e: &Engine, offset: u32| x87_quiet_bits(e.mem.u32(camera.wrapping_add(offset)));
    let p = [
        e.mem.u32(camera.wrapping_add(0x8c)),
        e.mem.u32(camera.wrapping_add(0x90)),
        e.mem.u32(camera.wrapping_add(0x94)),
    ];
    let up = x87_load32(m(e, 0xe4));
    let side = x87_load32(m(e, 0xe0));
    let far = x87_load32(m(e, 0xf0));
    let l = x87_load32;
    // `FLD a; FLD u; FLD ST0; FMULP ST2` (a * u), `FMUL ST1` (b * u), `FMUL dword` (u * c).
    let up_a = mul32(l(m(e, 0x6c)), up);
    let up_b = mul32(l(m(e, 0x78)), up);
    let up_c = mul32(up, x87_m32(m(e, 0x84)));
    let side_a = mul32(l(m(e, 0x70)), side);
    let side_b = mul32(l(m(e, 0x7c)), side);
    let side_c = mul32(side, x87_m32(m(e, 0x88)));
    // Each scaled by the far distance (`FMUL ST(i)` with it on the stack).
    let c1 = mul32(l(side_a), far);
    let c2 = mul32(l(side_b), far);
    let c3 = mul32(l(side_c), far);
    let c4 = mul32(l(up_a), far);
    let c5 = mul32(l(up_b), far);
    let c6 = mul32(l(up_c), far);
    let c7 = mul32(l(m(e, 0x68)), far);
    let c8 = mul32(l(m(e, 0x74)), far);
    // `FMULP ST7`: far * value.
    let c9 = mul32(far, x87_load32(m(e, 0x80)));
    // The base of every corner: `c7 + x`, `y + c8`, `z + c9`.
    let b = [
        add32(c7, p[0]),
        x87_store32(x87_add(x87_load32(p[1]), x87_m32(c8))),
        x87_store32(x87_add(x87_load32(p[2]), x87_m32(c9))),
    ];
    let corner = |up_sign: bool, side_sign: bool| {
        let step = |value: u32, delta: u32, plus: bool| {
            if plus {
                add32(value, delta)
            } else {
                sub32(value, delta)
            }
        };
        let u = [
            step(b[0], c4, up_sign),
            step(b[1], c5, up_sign),
            step(b[2], c6, up_sign),
        ];
        [
            step(u[0], c1, side_sign),
            step(u[1], c2, side_sign),
            step(u[2], c3, side_sign),
        ]
    };
    [
        p,
        corner(true, true),
        corner(true, false),
        corner(false, true),
        corner(false, false),
    ]
}

/// A plane of a `NiFrustumPlanes`-sized array in memory: its normal and constant words.
fn plane_words(e: &Engine, planes: u32, index: u32) -> [u32; 4] {
    let plane = planes + index * 0x10;
    [
        e.mem.u32(plane),
        e.mem.u32(plane + 4),
        e.mem.u32(plane + 8),
        e.mem.u32(plane + 0xc),
    ]
}

/// The signed distance of a point from a plane as `00b9c020` computes it for the inside
/// test: `((n.x * p.x + n.y * p.y) + p.z * n.z) - d`, stored as a `float`.
fn point_distance(plane: [u32; 4], point: [u32; 3]) -> u32 {
    let x = x87_mul(x87_load32(plane[0]), x87_m32(point[0]));
    let y = x87_mul(x87_load32(plane[1]), x87_m32(point[1]));
    let z = x87_mul(x87_load32(point[2]), x87_m32(plane[2]));
    x87_store32(x87_sub(x87_add(x87_add(x, y), z), x87_m32(plane[3])))
}

/// `00b9c020`'s side of a point for the edge test: 2 behind the plane, 1 in front, 0 on it
/// or unordered (`FCOM` with 0.0: `TEST AH,5; JP` then `TEST AH,0x41; JNZ`).
fn side_of(distance: u32) -> u32 {
    let ah = x87_compare(x87_load32(distance), 0.0);
    if !parity(ah, 0x05) {
        2
    } else if ah & 0x41 != 0 {
        0
    } else {
        1
    }
}

// Translated from 00b9c020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9c020 - the C++ exception frame (FS:[0] chain, unwind state 0 for the culling local) and the /GS cookie check (00ec408c) are not translated.
/// Unnamed (`thiscall` on a `ShadowSceneLight`, which it does not read; `RET 8`: two
/// cameras): whether the two cameras' view volumes intersect. Each camera's five points
/// (position and far corners, `camera_points`) are built and each camera's six frustum
/// planes taken from a local culler (`00a69400`, `00a694a0` on the camera's frustum +0xdc,
/// the planes copied out of the culler's +0x2c; the local plane arrays built by the
/// `NiPlane` constructor). True when a point of one camera lies strictly inside all planes
/// of the other; or, for an edge of one camera's volume (`FRUSTUM_EDGES`), when both ends
/// lie on a plane of the other; or when an edge crosses a plane at a parameter `t` in
/// [0, 1] (`t = ((d n - P) . n) / ((Q - P) . n)`, every step as the exe stores it) and the
/// crossing point is not behind any other plane. The culler is destroyed on every path.
pub fn fn_00b9c020(e: &mut Engine, _this: Ptr<ShadowSceneLight>, first: Ptr, second: Ptr) -> bool {
    e.with_stack(0x2c0, |e, frame| {
        let frame = frame.addr();
        let planes = frame + 0xdc;
        let culler = frame + 0x144;
        let first_planes = frame + 0x1d4;
        let second_planes = frame + 0x23c;
        for array in [first_planes, second_planes] {
            for index in 0..6u32 {
                e.call(NI_PLANE_CONSTRUCT, &args![array + index * 0x10]);
            }
            e.mem.set_u32(array + 0x60, 0x3f);
        }
        let first_points = camera_points(e, first.addr());
        let second_points = camera_points(e, second.addr());
        e.call(CULLER_CONSTRUCT_00A69400, &args![culler, 0u32]);
        for (camera, array) in [(first.addr(), first_planes), (second.addr(), second_planes)] {
            e.mem.set_u32(culler + 0xc, camera);
            e.call(
                CULLER_SET_FRUSTUM_00A694A0,
                &args![culler, camera.wrapping_add(0xdc)],
            );
            for word in 0..0x19u32 {
                let value = e.mem.u32(culler + 0x2c + 4 * word);
                e.mem.set_u32(array + 4 * word, value);
            }
        }
        for index in 0..6u32 {
            e.call(NI_PLANE_CONSTRUCT, &args![planes + index * 0x10]);
        }
        e.mem.set_u32(planes + 0x60, 0x3f);
        // Camera 0's points against camera 1's planes, then the other way round.
        let select = |e: &mut Engine, camera: usize| {
            let source = if camera == 0 {
                second_planes
            } else {
                first_planes
            };
            for word in 0..0x19u32 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(planes + 4 * word, value);
            }
            if camera == 0 {
                first_points
            } else {
                second_points
            }
        };
        let result = 'test: {
            for camera in 0..2 {
                let points = select(e, camera);
                for point in points {
                    let inside = (0..6u32).all(|index| {
                        let distance = point_distance(plane_words(e, planes, index), point);
                        // `FCOM`/`TEST AH,5; JNP` (below 0: out), then `FCOMP`/`TEST
                        // AH,0x41; JNZ` (0 or unordered: out).
                        let ah = x87_compare(x87_load32(distance), 0.0);
                        parity(ah, 0x05) && ah & 0x41 == 0
                    });
                    if inside {
                        break 'test true;
                    }
                }
            }
            for camera in 0..2 {
                let points = select(e, camera);
                for (from, to) in FRUSTUM_EDGES {
                    let start = points[from];
                    let end = points[to];
                    for index in 0..6u32 {
                        let plane = plane_words(e, planes, index);
                        let side_start = side_of(point_distance_edge(plane, start));
                        let side_end = side_of(point_distance_edge(plane, end));
                        if side_start == side_end {
                            if side_start == 0 {
                                break 'test true;
                            }
                            continue;
                        }
                        if edge_crosses_inside(e, planes, index, plane, start, end) {
                            break 'test true;
                        }
                    }
                }
            }
            false
        };
        e.call(CULLER_DESTROY_00A693E0, &args![culler]);
        result
    })
}

/// The edge test's distance of a point from a plane: `((p.x * n.x + n.y * p.y) + p.z *
/// n.z) - d` (the point's coordinates on the x87 stack), stored as a `float`.
fn point_distance_edge(plane: [u32; 4], point: [u32; 3]) -> u32 {
    let x = x87_mul(x87_load32(point[0]), x87_m32(plane[0]));
    let y = x87_mul(x87_load32(plane[1]), x87_load32(point[1]));
    let z = x87_mul(x87_load32(point[2]), x87_m32(plane[2]));
    x87_store32(x87_sub(x87_add(x87_add(x, y), z), x87_m32(plane[3])))
}

/// An edge from `start` to `end` crossing plane `index`: the crossing parameter and point
/// as the exe computes them (each `FSTP dword` rounding), and whether the point, with `t` in
/// [0, 1], is on or in front of every other plane (`FCOM dword`/`TEST AH,0x41; JZ`: only a
/// negative distance rejects it).
fn edge_crosses_inside(
    e: &Engine,
    planes: u32,
    index: u32,
    plane: [u32; 4],
    start: [u32; 3],
    end: [u32; 3],
) -> bool {
    let [nx, ny, nz, d] = plane;
    let l = x87_load32;
    let d_nx = mul32(l(nx), l(d));
    let d_ny = mul32(l(d), l(ny));
    let d_nz = mul32(l(d), l(nz));
    let a = x87_store32(x87_sub(l(d_nx), l(start[0])));
    let b = x87_store32(x87_sub(l(d_ny), l(start[1])));
    let c = x87_store32(x87_sub(l(d_nz), l(start[2])));
    // `FSUBRP ST6`: end - start.
    let dx = x87_store32(x87_sub(l(end[0]), l(start[0])));
    let dy = x87_store32(x87_sub(l(end[1]), l(start[1])));
    let dz = x87_store32(x87_sub(l(end[2]), l(start[2])));
    let numerator = x87_store32(x87_add(
        x87_add(x87_mul(l(b), l(ny)), x87_mul(l(a), l(nx))),
        x87_mul(l(c), l(nz)),
    ));
    let denominator = x87_store32(x87_add(
        x87_add(x87_mul(l(nx), l(dx)), x87_mul(l(ny), l(dy))),
        x87_mul(l(nz), x87_m32(dz)),
    ));
    let t = x87_store32(x87_div(l(numerator), l(denominator)));
    let point_x = x87_store32(x87_add(l(mul32(l(dx), l(t))), l(start[0])));
    let point_y = x87_store32(x87_add(l(mul32(l(dy), l(t))), l(start[1])));
    let point_z = x87_store32(x87_add(l(mul32(l(dz), l(t))), l(start[2])));
    // `FLDZ; FCOM ST2; TEST AH,0x41; JP`: on only for 0 below or equal to t.
    if parity(x87_compare(0.0, l(t)), 0x41) {
        return false;
    }
    // `FLD1; FCOMP ST3; TEST AH,1; JNZ`: off when 1 is below t or unordered.
    if x87_compare(1.0, l(t)) & 0x01 != 0 {
        return false;
    }
    (0..6u32).filter(|&other| other != index).all(|other| {
        let [ox, oy, oz, od] = plane_words(e, planes, other);
        let x = x87_mul(l(ox), x87_m32(point_x));
        let y = x87_mul(l(oy), x87_m32(point_y));
        let z = x87_mul(l(point_z), x87_m32(oz));
        let distance = x87_store32(x87_sub(x87_add(x87_add(x, y), z), x87_m32(od)));
        x87_compare(0.0, x87_m32(distance)) & 0x41 != 0
    })
}

/// The light list walk of `00b9d150`: the lighting property's active non-shadow lights
/// (`GetFirstActiveNonShadowLight` `00b70600` / `GetNextActiveNonShadowLight` `00b70700`,
/// `thiscall`, `RET 4`, the iterator local) when there is a property, else the shadow scene
/// node's light at an index (`00b5ac20`, `RET 4`).
const GET_FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
const GET_NEXT_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0700;
const SCENE_NODE_LIGHT_AT_00B5AC20: u32 = 0x00b5_ac20;
/// The weights of the counted lights (`float[40]`, written twice; the second array is read).
const LIGHT_WEIGHTS_011FFF40: u32 = 0x011f_ff40;
const LIGHT_WEIGHTS_011FFEA0: u32 = 0x011f_fea0;
/// The colour sum's start (three `float` words).
const COLOUR_START_011F4998: u32 = 0x011f_4998;
/// `double` 0.0625, 75.0, 1000.0, 0.75, 256.0, 1.4 (a widened `float`), 8.0, 0.4 and 0.8
/// (widened `float`s) and 1.0; `float` 0.75 and 0.8.
const RADIUS_SHARE_010B7E98: u32 = 0x010b_7e98;
const RADIUS_DIVISOR_010731A8: u32 = 0x0107_31a8;
const SUN_RANGE_01017B70: u32 = 0x0101_7b70;
const LOWEST_HEIGHT_0101DE30: u32 = 0x0101_de30;
const RADIUS_CAP_010231D8: u32 = 0x0102_31d8;
const OFFSET_SCALE_010B7E90: u32 = 0x010b_7e90;
const SPECULAR_SCALE_010240B0: u32 = 0x0102_40b0;
const COLOUR_SHARE_010290C0: u32 = 0x0102_90c0;
const LOWEST_GREY_010217D8: u32 = 0x0102_17d8;
const ONE_01012070: u32 = 0x0101_2070;
const LOWEST_HEIGHT_01016264: u32 = 0x0101_6264;
const LOWEST_GREY_01030FF0: u32 = 0x0103_0ff0;

/// The first light of `00b9d150`'s walk.
fn first_light(e: &mut Engine, property: u32, iterator: u32) -> u32 {
    if property != 0 {
        e.call(
            GET_FIRST_ACTIVE_NON_SHADOW_LIGHT,
            &args![property, iterator],
        )
        .u32()
    } else {
        let node = e.global::<u32>(SCENE_NODE_011F91C8);
        e.call(SCENE_NODE_LIGHT_AT_00B5AC20, &args![node, 0u32])
            .u32()
    }
}

/// The next light of `00b9d150`'s walk (`index` used without a property).
fn next_light(e: &mut Engine, property: u32, iterator: u32, index: u32) -> u32 {
    if property != 0 {
        e.call(GET_NEXT_ACTIVE_NON_SHADOW_LIGHT, &args![property, iterator])
            .u32()
    } else {
        let node = e.global::<u32>(SCENE_NODE_011F91C8);
        e.call(SCENE_NODE_LIGHT_AT_00B5AC20, &args![node, index])
            .u32()
    }
}

/// `FLD a; FLD b; FLD ST0; FMULP ST2; FXCH; FSTP`, `FLD c; FMUL ST1; FSTP`, `FMUL d; FSTP`:
/// the three words of a point times a loaded factor, each stored as a `float`.
fn scaled_words(words: [u32; 3], factor: f64) -> [u32; 3] {
    [
        x87_store32(x87_mul(x87_load32(words[0]), factor)),
        x87_store32(x87_mul(x87_load32(words[1]), factor)),
        x87_store32(x87_mul(factor, x87_m32(words[2]))),
    ]
}

fn read_words(e: &Engine, address: u32) -> [u32; 3] {
    [0, 4, 8].map(|offset| e.mem.u32(address.wrapping_add(offset)))
}

fn write_words(e: &mut Engine, address: u32, words: [u32; 3]) {
    for (i, word) in words.into_iter().enumerate() {
        e.mem.set_u32(address.wrapping_add(4 * i as u32), word);
    }
}

// Translated from 00b9d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::ComputeWeightedShadowLight` (Ghidra). `thiscall`, `RET 0x10`: the
/// lighting property (0 for the shadow scene node's lights) and three words the function only
/// uses as storage for the position. Translated from the disassembly (the decompiler drops x87
/// work). The position is the shadow scene's world translation (+0x8c), moved by its scale
/// (+0x98) times the extra data's offset (`GetExtraData` `00a5bdd0` with the name at
/// `012031e4`, +0xc); the radius is the largest component of the scale times the extra data's
/// extents (+0x18), or of the point at `011f426c` without extra data. Up to 40 usable lights
/// (not +0xec, `NiLight` flag bit 0 clear, dimmer above 0) get the weight `1 - (distance -
/// radius) / range` clamped to [0, 1] (`011fff40`, `011ffea0`), summed. The direction starts at
/// `011f426c` and the colour at `011f4998`. Without weight the direction is the reversed sun
/// direction (the scene node's +0xe0 light's `NiLight` +0xf0, z times 0.0625 * radius),
/// normalized (`004a0c10`), its z raised to radius / 75, and the colour the sun's diffuse
/// colour. Otherwise every light of the walk, up to the counted number, with a positive share
/// `w = weight / sum` adds `w * (b * light direction + (1 - b) * sun direction)` and
/// `w * (b * light colour + (1 - b) * sun colour)` (`b` its weight, by walk position), and
/// `w * (b * range + (1 - b) * 1000)` to a range the function leaves unused; the direction's z
/// is raised to 0.75. The direction is normalized (twice). The shadow light (`spLight`) is
/// placed at the position plus `1.4 * min(radius, 256)` times the direction and updated
/// (`00a59c60`); its diffuse colour becomes the grey `0.4 * (r + g + b)` clamped to [0.8, 1],
/// its specular colour `(8 * min(radius, 256), 0, 0)`, and its revision (+0xa8) goes up by 2.
pub fn shadow_scene_light_compute_weighted_shadow_light(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    property: u32,
    _position_x: u32,
    _position_y: u32,
    _position_z: u32,
) {
    e.with_stack(0x30, |e, frame| {
        // Locals: the light iterator (+0x00), the sun direction (+0x04), a light's direction
        // (+0x10), the direction (+0x1c).
        let iterator = frame.addr();
        let sun_direction = iterator + 0x04;
        let light_direction = iterator + 0x10;
        let direction = iterator + 0x1c;
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        let name = e.global::<u32>(EXTRA_DATA_NAME_012031E4);
        let extra = e.call(GET_EXTRA_DATA, &args![scene, name]).u32();
        let scene = e.get(this, ShadowSceneLight::spShadowScene);
        let mut position = read_words(e, scene.wrapping_add(WORLD_TRANSLATE));
        let extents = if extra != 0 {
            let scale = x87_load32(copy_float(e, scene.wrapping_add(0x98)));
            let offset = scaled_words(read_words(e, extra.wrapping_add(0xc)), scale);
            position = [
                add32(offset[0], position[0]),
                add32(position[1], offset[1]),
                add32(position[2], offset[2]),
            ];
            let scale = x87_load32(copy_float(e, scene.wrapping_add(0x98)));
            scaled_words(read_words(e, extra.wrapping_add(0x18)), scale)
        } else {
            read_words(e, POINT_POSITION_011F426C)
        };
        let larger = x87_keep_larger(x87_load32(extents[0]), x87_load32(extents[1]));
        let larger = x87_load32(x87_store32(larger));
        let radius = x87_store32(x87_keep_larger(larger, x87_load32(extents[2])));

        // The weights of the usable lights.
        let mut sum = x87_store32(0.0);
        let mut count = 0u32;
        let mut index = 0u32;
        let mut light = first_light(e, property, iterator);
        while light != 0 && count < 0x28 {
            let usable = e.mem.u8(light.wrapping_add(0xec)) == 0 && {
                let ni_light = e.mem.u32(light.wrapping_add(0xf8));
                e.mem.u8(ni_light.wrapping_add(FLAGS)) & 1 == 0
                    && 0.0 < x87_m32(e.mem.u32(ni_light.wrapping_add(0xc4)))
            };
            if usable {
                let ni_light = e.mem.u32(light.wrapping_add(0xf8));
                let mut delta = [0u32; 3];
                for (i, d) in delta.iter_mut().enumerate() {
                    let p = e
                        .mem
                        .u32(ni_light.wrapping_add(WORLD_TRANSLATE + 4 * i as u32));
                    *d = sub32(p, position[i]);
                }
                let squared = x87_store32(squared_length(
                    x87_load32(delta[1]),
                    x87_load32(delta[0]),
                    x87_load32(delta[2]),
                ));
                let distance = sqrt_stored(e, squared);
                let range = e.mem.u32(ni_light.wrapping_add(0xe0));
                let ratio = x87_div(
                    x87_sub(x87_load32(distance), x87_m32(radius)),
                    x87_m32(range),
                );
                let mut weight = x87_store32(x87_sub(1.0, ratio));
                let zero: f64 = e.global(ZERO_01012060);
                if x87_load32(weight) < zero {
                    weight = x87_store32(0.0);
                }
                let w = x87_load32(weight);
                let weight = x87_store32(if 1.0 < w { 1.0 } else { w });
                e.mem.set_u32(LIGHT_WEIGHTS_011FFF40 + 4 * count, weight);
                e.mem.set_u32(LIGHT_WEIGHTS_011FFEA0 + 4 * count, weight);
                count += 1;
                sum = x87_store32(x87_add(x87_load32(weight), x87_m32(sum)));
            }
            index += 1;
            light = next_light(e, property, iterator, index & 0xffff);
        }

        let node = e.global::<u32>(SCENE_NODE_011F91C8);
        let mut range_sum = x87_store32(0.0);
        write_words(e, direction, read_words(e, POINT_POSITION_011F426C));
        let mut colour = read_words(e, COLOUR_START_011F4998);
        let sun = e
            .mem
            .u32(e.mem.u32(node.wrapping_add(0xe0)).wrapping_add(0xf8));
        let sun_words = read_words(e, sun.wrapping_add(0xf0));
        let share: f64 = e.global(RADIUS_SHARE_010B7E98);
        let reach = x87_mul(x87_load32(radius), share);
        write_words(
            e,
            sun_direction,
            [
                x87_store32(-x87_load32(sun_words[0])),
                x87_store32(-x87_load32(sun_words[1])),
                x87_store32(x87_mul(-x87_load32(sun_words[2]), reach)),
            ],
        );
        e.call(POINT_NORMALIZE_004A0C10, &args![sun_direction]);
        if x87_m32(sum) == 0.0 {
            // No weight: the sun alone.
            write_words(e, direction, read_words(e, sun_direction));
            colour = read_words(e, sun.wrapping_add(0xd4));
            e.call(POINT_NORMALIZE_004A0C10, &args![direction]);
            let divisor: f64 = e.global(RADIUS_DIVISOR_010731A8);
            let lowest = x87_store32(x87_div(x87_load32(radius), divisor));
            if x87_load32(e.mem.u32(direction + 8)) < x87_load32(lowest) {
                e.mem.set_u32(direction + 8, x87_quiet_bits(lowest));
            }
        } else {
            // The node read before the sun direction is the one asked without a property.
            let mut light = if property != 0 {
                first_light(e, property, iterator)
            } else {
                e.call(SCENE_NODE_LIGHT_AT_00B5AC20, &args![node, 0u32])
                    .u32()
            };
            let mut node_index = 0u32;
            let mut walked = 0u32;
            while walked < count {
                if light != 0 && e.mem.u8(light.wrapping_add(0xec)) == 0 {
                    let ni_light = e.mem.u32(light.wrapping_add(0xf8));
                    if e.mem.u8(ni_light.wrapping_add(FLAGS)) & 1 == 0 {
                        let slot = LIGHT_WEIGHTS_011FFEA0 + 4 * walked;
                        let w = x87_store32(x87_div(x87_load32(e.mem.u32(slot)), x87_m32(sum)));
                        let w = x87_load32(w);
                        if 0.0 < w {
                            let b = x87_load32(e.mem.u32(slot));
                            let rest = x87_sub(1.0, b);
                            let reach = x87_mul(
                                x87_load32(e.mem.u32(ni_light.wrapping_add(0xe0))),
                                x87_m32(e.mem.u32(slot)),
                            );
                            let sun_range: f64 = e.global(SUN_RANGE_01017B70);
                            let blend = x87_add(reach, x87_mul(rest, sun_range));
                            range_sum = x87_store32(x87_add(x87_mul(w, blend), x87_m32(range_sum)));
                            let mut delta = [0u32; 3];
                            for (i, d) in delta.iter_mut().enumerate() {
                                let p = e
                                    .mem
                                    .u32(ni_light.wrapping_add(WORLD_TRANSLATE + 4 * i as u32));
                                *d = sub32(p, position[i]);
                            }
                            write_words(e, light_direction, delta);
                            e.call(POINT_NORMALIZE_004A0C10, &args![light_direction]);
                            // `FST qword` then `FSTP dword` of 1 - b.
                            let rest = x87_load32(x87_store32(x87_load64(rest.to_bits())));
                            let sun_part = read_words(e, sun_direction);
                            let sun_part = [
                                x87_store32(x87_mul(rest, x87_m32(sun_part[0]))),
                                x87_store32(x87_mul(x87_load32(sun_part[1]), rest)),
                                x87_store32(x87_mul(rest, x87_m32(sun_part[2]))),
                            ];
                            let b = x87_load32(x87_quiet_bits(e.mem.u32(slot)));
                            let light_part = scaled_words(read_words(e, light_direction), b);
                            let mut part = [0u32; 3];
                            for (i, p) in part.iter_mut().enumerate() {
                                *p = add32(light_part[i], sun_part[i]);
                            }
                            // `FLD; FMUL ST1` for all three (no memory operand).
                            let part = part.map(|p| x87_store32(x87_mul(x87_load32(p), w)));
                            let sum_of = read_words(e, direction);
                            write_words(
                                e,
                                direction,
                                [
                                    add32(part[0], sum_of[0]),
                                    add32(sum_of[1], part[1]),
                                    add32(part[2], sum_of[2]),
                                ],
                            );
                            let sun_colour = read_words(e, sun.wrapping_add(0xd4));
                            let sun_colour = scaled_words(sun_colour, rest);
                            let b = x87_load32(x87_quiet_bits(e.mem.u32(slot)));
                            let light_colour =
                                scaled_words(read_words(e, ni_light.wrapping_add(0xd4)), b);
                            let mut mixed = [0u32; 3];
                            for (i, m) in mixed.iter_mut().enumerate() {
                                *m = add32(light_colour[i], sun_colour[i]);
                            }
                            let mixed = scaled_words(mixed, w);
                            for (i, c) in colour.iter_mut().enumerate() {
                                *c = add32(mixed[i], *c);
                            }
                        }
                    }
                }
                light = if property != 0 {
                    next_light(e, property, iterator, 0)
                } else {
                    node_index += 1;
                    next_light(e, 0, iterator, node_index)
                };
                walked += 1;
            }
            e.call(POINT_NORMALIZE_004A0C10, &args![direction]);
            let lowest: f64 = e.global(LOWEST_HEIGHT_0101DE30);
            if x87_load32(e.mem.u32(direction + 8)) < lowest {
                let height = x87_quiet_bits(e.global::<u32>(LOWEST_HEIGHT_01016264));
                e.mem.set_u32(direction + 8, height);
            }
        }
        e.call(POINT_NORMALIZE_004A0C10, &args![direction]);
        let _ = range_sum;

        // The shadow light.
        let cap: f64 = e.global(RADIUS_CAP_010231D8);
        let r = x87_load32(radius);
        let reach = x87_store32(if cap < r {
            x87_load32(e.global::<u32>(EXTENT_CAP_01032ADC))
        } else {
            r
        });
        let ni_light = e.get(this, ShadowSceneLight::spLight);
        let offset = scaled_words(read_words(e, direction), x87_load32(reach));
        let scale: f64 = e.global(OFFSET_SCALE_010B7E90);
        let offset = scaled_words(offset, scale);
        write_words(
            e,
            ni_light.wrapping_add(0x58),
            [
                add32(offset[0], position[0]),
                add32(offset[1], position[1]),
                add32(offset[2], position[2]),
            ],
        );
        let specular_scale: f64 = e.global(SPECULAR_SCALE_010240B0);
        let specular = x87_store32(x87_mul(x87_load32(reach), specular_scale));
        let ni_light = e.get(this, ShadowSceneLight::spLight);
        update_object(e, ni_light);
        let colour_share: f64 = e.global(COLOUR_SHARE_010290C0);
        let grey = x87_add(
            x87_add(
                x87_mul(x87_load32(colour[1]), colour_share),
                x87_mul(x87_load32(colour[0]), colour_share),
            ),
            x87_mul(colour_share, x87_m32(colour[2])),
        );
        let grey = x87_load32(x87_store32(grey));
        let lowest: f64 = e.global(LOWEST_GREY_010217D8);
        let grey = if grey < lowest {
            x87_load32(e.global::<u32>(LOWEST_GREY_01030FF0))
        } else {
            grey
        };
        let grey = x87_load32(x87_store32(grey));
        let one: f64 = e.global(ONE_01012070);
        let grey = x87_store32(if one < grey { 1.0 } else { grey });
        let ni_light = e.get(this, ShadowSceneLight::spLight);
        write_words(e, ni_light.wrapping_add(0xd4), [grey; 3]);
        let revision = e.mem.u32(ni_light.wrapping_add(0xa8));
        e.mem
            .set_u32(ni_light.wrapping_add(0xa8), revision.wrapping_add(1));
        let ni_light = e.get(this, ShadowSceneLight::spLight);
        let revision = e.mem.u32(ni_light.wrapping_add(0xa8));
        e.mem
            .set_u32(ni_light.wrapping_add(0xa8), revision.wrapping_add(1));
        let zero = x87_store32(0.0);
        write_words(
            e,
            ni_light.wrapping_add(0xe0),
            [x87_quiet_bits(specular), zero, zero],
        );
    });
}

/// `NiLight` fields `00b9bdf0` reads (PC offsets): the dimmer (+0xc4), the world translate
/// (+0x8c), the colour at +0xd4 (three `float`s) and the radius `float` at +0xe0.
const NI_LIGHT_DIMMER: u32 = 0xc4;
const NI_LIGHT_COLOUR_D4: u32 = 0xd4;
const NI_LIGHT_RADIUS_E0: u32 = 0xe0;

// Translated from 00b9bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x1c`: points `a` and `b`, three words each,
/// and an `NiLight`; answers a `float` in `ST0`): 0.0 when the light's `spLight` is that
/// `NiLight`, the light casts shadows (`bCastShadow` 1) or its `NiLight` is culled (flag
/// bit 0). Otherwise the luminance (`fLuminance`, +0xc) becomes 1.0, for a point light
/// `1 - t * t` with `t` the distance from the light to `a` (`00457990`) over the radius,
/// clamped with `00404010` (against 0.0) and `0040ebd0` (against 1.0); then it is multiplied
/// by the facing term (the dot product of `-(b - a)` normalised with `a - light` normalised,
/// above 1.0 cut to 1.0 and below 0.0 to 0.0, NaN kept) and by the largest component of the
/// dimmer times the colour at +0xd4, stored and answered. Each x87 step follows the exe.
pub fn fn_00b9bdf0(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    ax: u32,
    ay: u32,
    az: u32,
    bx: u32,
    by: u32,
    bz: u32,
    ni_light_argument: u32,
) -> f32 {
    let ni_light = e.get(this, ShadowSceneLight::spLight);
    if ni_light == ni_light_argument
        || e.get(this, ShadowSceneLight::bCastShadow) == 1
        || e.mem.u8(ni_light.wrapping_add(FLAGS)) & 1 != 0
    {
        return 0.0;
    }
    let point_light = e.get(this, ShadowSceneLight::bPointLight) != 0;
    let dimmer = x87_load32(x87_store32(x87_load32(
        e.mem.u32(ni_light.wrapping_add(NI_LIGHT_DIMMER)),
    )));
    let position = read_words(e, ni_light.wrapping_add(WORLD_TRANSLATE));
    let colour = read_words(e, ni_light.wrapping_add(NI_LIGHT_COLOUR_D4));
    let scaled = [
        x87_store32(x87_mul(dimmer, x87_m32(colour[0]))),
        x87_store32(x87_mul(dimmer, x87_m32(colour[1]))),
        x87_store32(x87_mul(dimmer, x87_m32(colour[2]))),
    ];
    let radius = x87_store32(x87_load32(
        e.mem.u32(ni_light.wrapping_add(NI_LIGHT_RADIUS_E0)),
    ));
    e.set(this, ShadowSceneLight::fLuminance, 1.0);
    let a = [ax, ay, az];
    let b = [bx, by, bz];
    // The exe's two vectors on its stack: `a - light` (+0) and `b - a` (+0xc).
    let (to_point, along) = e.with_stack(0x18, |e, frame| {
        let to_point = frame.addr();
        let along = frame.addr() + 0xc;
        for i in 0..3 {
            let difference = x87_store32(x87_sub(x87_load32(a[i]), x87_m32(position[i])));
            e.mem.set_u32(to_point + 4 * i as u32, difference);
        }
        if point_light {
            let length = e.call(POINT_LENGTH, &args![to_point]).st0;
            let ratio = x87_quiet_bits(x87_store32(x87_div(length, x87_m32(radius))));
            let zero = x87_store32(0.0);
            let larger = x87_store32(e.call(FLOAT_MAX, &args![zero, ratio]).st0);
            let one = x87_store32(1.0);
            let t = x87_load32(x87_store32(e.call(FLOAT_MIN, &args![one, larger]).st0));
            let luminance = x87_store32(x87_sub(1.0, x87_mul(t, t)));
            e.mem.set_u32(this.addr() + 0xc, luminance);
        }
        for i in 0..3 {
            let difference = x87_store32(x87_sub(x87_load32(b[i]), x87_m32(a[i])));
            e.mem.set_u32(along + 4 * i as u32, difference);
        }
        e.call(POINT_NORMALIZE_004A0C10, &args![along]);
        e.call(POINT_NORMALIZE_004A0C10, &args![to_point]);
        (read_words(e, to_point), read_words(e, along))
    });
    let negated = [
        x87_store32(-x87_load32(along[0])),
        x87_store32(-x87_load32(along[1])),
        x87_store32(-x87_load32(along[2])),
    ];
    let dot = x87_add(
        x87_mul(x87_load32(negated[1]), x87_m32(to_point[1])),
        x87_mul(x87_load32(negated[0]), x87_m32(to_point[0])),
    );
    let dot = x87_add(dot, x87_mul(x87_load32(negated[2]), x87_m32(to_point[2])));
    let facing = x87_load32(x87_store32(x87_load32(x87_store32(dot))));
    // `FCOM 1.0; TEST AH,0x41; JNZ`: cut to 1.0 only when greater (NaN kept).
    let one: f64 = e.global(ONE_01012070);
    let facing = if facing > one { 1.0 } else { facing };
    let facing = x87_load32(x87_store32(facing));
    let zero: f64 = e.global(ZERO_01012060);
    let facing = x87_store32(if facing < zero { 0.0 } else { facing });
    let peak = x87_store32(x87_keep_larger(
        x87_load32(scaled[1]),
        x87_load32(scaled[2]),
    ));
    let peak = x87_store32(x87_keep_larger(x87_load32(scaled[0]), x87_load32(peak)));
    let luminance = e.mem.u32(this.addr() + 0xc);
    let result = x87_store32(x87_mul(
        x87_mul(x87_load32(luminance), x87_m32(facing)),
        x87_m32(peak),
    ));
    let result = x87_quiet(f32::from_bits(result));
    e.set(this, ShadowSceneLight::fLuminance, result);
    result
}

/// The pointer the code reads at `011f91ec` (a `cdecl` function of the geometry, called when
/// set), the byte flags at `011ad82c`, the words `011f91e0` and `011f91e4`, the renderer's
/// object at its +8 (`011f4748`) and its `00aa0ff0` (thiscall, answers a byte), the two
/// `cdecl` setters `00b7a870` and `00bbd880` (a pass number and 0), and `NiAVObject::Cull`
/// (`00a59e00`, thiscall: culler).
const GEOMETRY_CALLBACK_011F91EC: u32 = 0x011f_91ec;
const RENDER_FLAGS_011AD82C: u32 = 0x011a_d82c;
const PASS_STATE_011F91E0: u32 = 0x011f_91e0;
const PASS_011F91E4: u32 = 0x011f_91e4;
const RENDERER_00AA0FF0: u32 = 0x00aa_0ff0;
const PASS_SETTER_00B7A870: u32 = 0x00b7_a870;
const PASS_SETTER_00BBD880: u32 = 0x00bb_d880;
const NI_AV_OBJECT_CULL: u32 = 0x00a5_9e00;

// Translated from 00b9d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a culler; answers 1). Unless the light's
/// `NiLight` is culled (flag bit 0), for each geometry of the light's list (+0xe0) with a
/// property at +0xc0: the callback at `011f91ec` (when set) gets the geometry; with bit 0 of
/// `011ad82c`, the renderer's `00aa0ff0` answers a byte and the property's word +0x68 picks a
/// pass: for 1 to 5, when the byte is 0, `011f91e0` is cleared and pass 0x5c (geometry word
/// +0xbc set) or 0x58 goes to `011f91e4` and `00b7a870`; for 0x11 likewise 0x1ca or 0x1c8
/// through `00bbd880`; after a pass the property's objects at +0x34 and +0x30 get their
/// virtual slot +0x24. The geometry is then culled (`NiAVObject::Cull`); a word +0x68 of any
/// other value skips it.
pub fn fn_00b9d020(e: &mut Engine, this: Ptr<ShadowSceneLight>, culler: u32) -> bool {
    let ni_light = e.get(this, ShadowSceneLight::spLight);
    if e.mem.u8(ni_light.wrapping_add(FLAGS)) & 1 != 0 {
        return true;
    }
    let mut item = e.mem.u32(this.addr() + GEOM_LIST);
    while item != 0 {
        let geometry = e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_element);
        item = e.get(Ptr::<NiTListItem>::new(item), NiTListItem::m_pkNext);
        if geometry == 0 {
            continue;
        }
        let property = e.mem.u32(geometry.wrapping_add(0xc0));
        if property == 0 {
            continue;
        }
        let callback = e.global::<u32>(GEOMETRY_CALLBACK_011F91EC);
        if callback != 0 {
            e.call(callback, &args![geometry]);
        }
        if e.global::<u8>(RENDER_FLAGS_011AD82C) & 1 == 0 || property == 0 {
            continue;
        }
        let renderer = e.global::<u32>(RENDERER_SINGLETON_011F4748);
        let object = e.mem.u32(renderer.wrapping_add(8));
        let busy = e.call(RENDERER_00AA0FF0, &args![object]).u8();
        let kind = e.mem.i32(property.wrapping_add(0x68));
        let with_word = e.mem.u32(geometry.wrapping_add(0xbc)) != 0;
        let pass = if (1..=5).contains(&kind) {
            if busy != 0 {
                None
            } else {
                Some((PASS_SETTER_00B7A870, if with_word { 0x5c } else { 0x58 }))
            }
        } else if kind == 0x11 {
            if busy != 0 {
                None
            } else {
                Some((PASS_SETTER_00BBD880, if with_word { 0x1ca } else { 0x1c8 }))
            }
        } else {
            continue;
        };
        if let Some((setter, pass)) = pass {
            e.set_global(PASS_STATE_011F91E0, 0u32);
            e.set_global(PASS_011F91E4, pass);
            e.call(setter, &args![pass, 0u32]);
            let first = e.mem.u32(property.wrapping_add(0x34));
            e.vcall(first, 0x24, &args![]);
            let second = e.mem.u32(property.wrapping_add(0x30));
            e.vcall(second, 0x24, &args![]);
        }
        e.call(NI_AV_OBJECT_CULL, &args![geometry, culler]);
    }
    true
}

/// `BSCubeMapCamera`'s constructor (`00bd8290`, thiscall: a word, `RET 4`),
/// `SetupTransformation` (`00bd7200`, thiscall: the face, `RET 4`) and destructor
/// (`00bd74a0`); `NiAVObject::Update` (`00a59c60`, thiscall: the update data).
const CUBE_MAP_CAMERA_CONSTRUCT: u32 = 0x00bd_8290;
const CUBE_MAP_CAMERA_SETUP_TRANSFORMATION: u32 = 0x00bd_7200;
const CUBE_MAP_CAMERA_DESTRUCT: u32 = 0x00bd_74a0;

// Translated from 00b9de90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9de90 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the local camera) and the /GS cookie (011c16bc, __security_check_cookie) are not translated.
/// `ShadowSceneLight::CubeFaceFrustumFrustumIntersection` (engine map; `thiscall`, `RET 0x18`:
/// a position, three words; an unread word; an object whose camera is at +0xc; a 16-bit cube
/// face). With the TLS word +0x2b4 set to 0x20 for its duration: a local `BSCubeMapCamera`
/// (constructed with 0) gets the position at its +0x58, is set up for the face, updated with
/// zeroed update data (`float` 0.0 and five zero bytes), and the answer is [`fn_00b9c020`]
/// of the object's camera and the local camera; the local camera is destroyed.
pub fn shadow_scene_light_cube_face_frustum_frustum_intersection(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
    x: u32,
    y: u32,
    z: u32,
    _unused_4: u32,
    source: Ptr,
    face: u16,
) -> bool {
    let tls = e.tls();
    let camera_of_source = e.mem.u32(source.addr().wrapping_add(0xc));
    let saved = e.mem.u32(tls + TLS_WORD_2B4);
    e.mem.set_u32(tls + TLS_WORD_2B4, 0x20);
    // The exe's frame: the update data (+0) and the local camera (+0x10, 0x140 bytes).
    let answer = e.with_stack(0x150, |e, frame| {
        let data = frame.addr();
        let camera = frame.addr() + 0x10;
        e.call(CUBE_MAP_CAMERA_CONSTRUCT, &args![camera, 0u32]);
        e.mem.set_u32(camera + 0x58, x);
        e.mem.set_u32(camera + 0x5c, y);
        e.mem.set_u32(camera + 0x60, z);
        e.call(
            CUBE_MAP_CAMERA_SETUP_TRANSFORMATION,
            &args![camera, u32::from(face)],
        );
        e.mem.set_u32(data, x87_store32(0.0));
        for offset in 4..9 {
            e.mem.set_u8(data + offset, 0);
        }
        e.call(OBJECT_UPDATE_00A59C60, &args![camera, data]);
        let answer = fn_00b9c020(e, this, Ptr::new(camera_of_source), Ptr::new(camera));
        e.call(CUBE_MAP_CAMERA_DESTRUCT, &args![camera]);
        answer
    });
    e.mem.set_u32(tls + TLS_WORD_2B4, saved);
    answer
}

/// The `BSSimpleArray` vtable these destructors store (`010b7ea8`) and the word at
/// `010b7eb0` (its slot +8, the buffer free, called directly: thiscall with the buffer).
const SIMPLE_ARRAY_FREE_SLOT_010B7EB0: u32 = 0x010b_7eb0;
/// The memory manager (`011f6238`) and its `Deallocate` (`00aa4060`, thiscall: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

/// The destructor body both `00b9e8f0` and `00b9e920` run: the vtable, then a present buffer
/// freed (the function at `010b7eb0`) and the buffer, capacity and count cleared.
fn simple_array_destruct(e: &mut Engine, array: u32) {
    let buffer = e.mem.u32(array.wrapping_add(4));
    e.mem.set_u32(array, SIMPLE_ARRAY_VTABLE_010B7EA8);
    if buffer != 0 {
        let free = e.global::<u32>(SIMPLE_ARRAY_FREE_SLOT_010B7EB0);
        e.call(free, &args![array, buffer]);
        e.mem.set_u32(array.wrapping_add(4), 0);
        e.mem.set_u32(array.wrapping_add(0xc), 0);
        e.mem.set_u32(array.wrapping_add(8), 0);
    }
}

// Translated from 00b9e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor of the `BSSimpleArray` with vtable `010b7ea8`
/// (fastcall): the vtable stored, a present buffer freed through the function at `010b7eb0`
/// and the buffer, capacity and count cleared.
pub fn fn_00b9e8f0(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this.addr());
}

// Translated from 00b9e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: that array's scalar deleting destructor (thiscall, `RET 4`):
/// the destructor body, then with bit 0 of `flags` the memory manager's `Deallocate`
/// (`00aa4060`) of the array. Answers `this`.
pub fn fn_00b9e920(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    simple_array_destruct(e, this.addr());
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b9_bdf0,
            fn_00b9bdf0(Ptr<ShadowSceneLight>, u32, u32, u32, u32, u32, u32, u32) -> f32
        ),
        entry!(0x00b9_d020, fn_00b9d020(Ptr<ShadowSceneLight>, u32) -> bool),
        entry!(
            0x00b9_de90,
            shadow_scene_light_cube_face_frustum_frustum_intersection(
                Ptr<ShadowSceneLight>,
                u32,
                u32,
                u32,
                u32,
                Ptr,
                u16,
            ) -> bool
        ),
        entry!(0x00b9_e8f0, fn_00b9e8f0(Ptr)),
        entry!(0x00b9_e920, fn_00b9e920(Ptr, u8) -> Ptr),
        entry!(
            0x00b9_d150,
            shadow_scene_light_compute_weighted_shadow_light(
                Ptr<ShadowSceneLight>,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x00b9_c020,
            fn_00b9c020(Ptr<ShadowSceneLight>, Ptr, Ptr) -> bool
        ),
        entry!(0x00b9_e7c0, fn_00b9e7c0(Ptr<NiTPointerList>, Ptr) -> Ptr),
        entry!(
            0x00b9_dda0,
            shadow_scene_light_set_light(Ptr<ShadowSceneLight>, Ptr)
        ),
        entry!(
            0x00b9_fda0,
            shadow_scene_light_shadow_scene_light(Ptr<ShadowSceneLight>) -> Ptr<ShadowSceneLight>
        ),
        entry!(0x00b9_e850, fn_00b9e850(Ptr<NiTPointerList>, Ptr) -> Ptr),
        entry!(
            0x00b9_bbb0,
            shadow_scene_light_bound_within_light_frustum(Ptr<ShadowSceneLight>, Ptr) -> u8
        ),
        entry!(
            0x00b9_bc50,
            fn_00b9bc50(Ptr<ShadowSceneLight>, u32, u32, u32, u32) -> f32
        ),
        entry!(0x00b9_cfd0, fn_00b9cfd0(Ptr, u32)),
        entry!(
            0x00b9_dae0,
            shadow_scene_light_bound_within_light_radius(
                Ptr<ShadowSceneLight>,
                Ptr,
                u32,
                f32,
            ) -> u8
        ),
        entry!(
            0x00b9_dbe0,
            shadow_scene_light_get_luminance_to_bound(Ptr<ShadowSceneLight>, Ptr) -> f32
        ),
        entry!(0x00b9_dcb0, fn_00b9dcb0(Ptr<ShadowSceneLight>, u8)),
        entry!(0x00b9_e770, fn_00b9e770(Ptr<ShadowSceneLight>)),
        entry!(
            0x00b9_e970,
            shadow_scene_light_test_frustum_cull(Ptr<ShadowSceneLight>, u32)
        ),
        entry!(
            0x00b9_f480,
            shadow_scene_light_add_tri_geom(Ptr<ShadowSceneLight>, u32)
        ),
        entry!(
            0x00b9_f5b0,
            shadow_scene_light_parse_object_add_frustum(Ptr<ShadowSceneLight>, u32)
        ),
        entry!(
            0x00b9_f6a0,
            shadow_scene_light_parse_object_add_radius(Ptr<ShadowSceneLight>, u32)
        ),
        entry!(
            0x00b9_f780,
            shadow_scene_light_mt_render_frustum_depth_map(Ptr<ShadowSceneLight>, u32, u32)
        ),
        entry!(
            0x00ba_00e0,
            fn_00ba00e0(Ptr<ShadowSceneLight>, u32) -> Ptr<ShadowSceneLight>
        ),
        entry!(
            0x00ba_0110,
            shadow_scene_light_add_object(Ptr<ShadowSceneLight>, u32)
        ),
        entry!(
            0x00ba_01a0,
            shadow_scene_light_remove_tri_geom(Ptr<ShadowSceneLight>, u32, u32)
        ),
        entry!(
            0x00b9_dfc0,
            shadow_scene_light_mt_setup_frustum_depth_map(Ptr<ShadowSceneLight>, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn counting(e: &mut Engine) {
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
    }

    const DELETE: u32 = 0x0f00_2000;
    const GET_RTTI: u32 = 0x0f00_2008;

    /// A light whose RTTI chain is `own -> base -> 0`, held `count` times, with the words
    /// 1, 2, 3 at +0x8c.
    fn light(e: &mut Engine, own: u32, base: u32, count: u32) -> u32 {
        e.mem.set_u32(own + 4, base);
        e.mem.set_u32(base + 4, 0);
        let light = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(16);
        e.mem.set_u32(vtable + 4, DELETE);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        e.mem.set_u32(light, vtable);
        e.mem.set_u32(light + 4, count);
        e.mem.set_u32(light + 0xf8, own);
        for i in 0..3 {
            e.mem.set_u32(light + 0x8c + 4 * i, i + 1);
        }
        light
    }

    fn set_light_rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        counting(&mut e);
        e.register(DELETE, |_, _| Ret::default());
        // GetRTTI answers the record kept at the light's +0xf8 (test only).
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 0xf8).into_ret());
        e
    }

    #[test]
    fn set_light_with_a_point_light_copies_its_position_when_dynamic() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x200);
        // A point light that derives from the ambient record (both flags).
        let lamp = light(&mut e, NI_POINT_LIGHT_RTTI, NI_AMBIENT_LIGHT_RTTI, 0);
        let old = light(&mut e, other, other + 0x40, 1);
        let this = e.mem.alloc(0x244);
        e.mem.set_u32(this + 0xf8, old);
        e.mem.set_u8(this + 0xfc, 1);
        e.mem.set_f32(this + 0xd0, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dda0, &args![this, lamp]);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0xf8), lamp);
        assert_eq!(e.mem.u32(lamp + 4), 1);
        assert_eq!(e.mem.u8(this + 0xf4), 1);
        assert_eq!(e.mem.u8(this + 0xf5), 1);
        assert_eq!(e.mem.f32(this + 0xd0), 0.25);
        assert_eq!(
            [
                e.mem.u32(this + 0x100),
                e.mem.u32(this + 0x104),
                e.mem.u32(this + 0x108)
            ],
            [1, 2, 3]
        );
    }

    #[test]
    fn set_light_with_the_same_ambient_light_keeps_the_count_and_resets_the_dimmer() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x100);
        let lamp = light(&mut e, other, NI_AMBIENT_LIGHT_RTTI, 1);
        let this = e.mem.alloc(0x244);
        e.mem.set_u32(this + 0xf8, lamp);
        e.mem.set_u8(this + 0xfc, 1);
        e.mem.set_u8(this + 0xf4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dda0, &args![this, lamp]);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.mem.u32(lamp + 4), 1);
        assert_eq!(e.mem.u8(this + 0xf4), 0);
        assert_eq!(e.mem.u8(this + 0xf5), 1);
        assert_eq!(e.mem.f32(this + 0xd0), 1.0);
        assert_eq!(e.mem.u32(this + 0x100), 0);
    }

    #[test]
    fn set_light_with_a_static_point_light_keeps_the_position() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x100);
        let lamp = light(&mut e, NI_POINT_LIGHT_RTTI, other, 0);
        let this = e.mem.alloc(0x244);
        e.call(0x00b9_dda0, &args![this, lamp]);
        assert_eq!(e.mem.u8(this + 0xf4), 1);
        assert_eq!(e.mem.u8(this + 0xf5), 0);
        assert_eq!(e.mem.u32(this + 0x100), 0);
        assert_eq!(e.mem.f32(this + 0xd0), 0.0);
    }

    #[test]
    fn set_light_to_none_releases_the_old_light() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x100);
        let old = light(&mut e, NI_POINT_LIGHT_RTTI, other, 2);
        let this = e.mem.alloc(0x244);
        e.mem.set_u32(this + 0xf8, old);
        e.mem.set_u8(this + 0xf4, 1);
        e.mem.set_u8(this + 0xf5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dda0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(old + 4), 1);
        assert!(calls(&e, DELETE).is_empty());
        assert!(calls(&e, GET_RTTI).is_empty());
        assert_eq!(e.mem.u32(this + 0xf8), 0);
        assert_eq!(e.mem.u8(this + 0xf4), 0);
        assert_eq!(e.mem.u8(this + 0xf5), 0);
        assert_eq!(e.mem.f32(this + 0xd0), 1.0);
    }

    #[test]
    fn constructor_sets_every_field_it_owns() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        counting(&mut e);
        e.set_global(NI_REF_OBJECT_COUNT, 5u32);
        e.set_global(POINT_POSITION_011F426C, 0x11u32);
        e.set_global(POINT_POSITION_011F426C + 4, 0x22u32);
        e.set_global(POINT_POSITION_011F426C + 8, 0x33u32);
        // A signalling NaN: the FLD/FSTP copy quiets it.
        e.set_global(SPOTLIGHT_FOV_010249D8, 0x7f80_0001u32);
        e.register(NI_PLANE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x244);
        for offset in (0..0x244).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_fda0, &args![this]).u32(), this);

        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT), 6);
        assert_eq!(e.mem.u32(this), SHADOW_SCENE_LIGHT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        let planes: Vec<u32> = calls(&e, NI_PLANE_CONSTRUCT).iter().map(|c| c[0]).collect();
        assert_eq!(
            planes,
            (0..6).map(|i| this + 0x144 + 0x10 * i).collect::<Vec<_>>()
        );
        assert_eq!(e.mem.u32(this + 0x1a4), 0x3f);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x1a8, 0, 0x60]]);
        assert_eq!(e.mem.bytes(this + 0x1a8, 0x60), vec![0; 0x60]);
        // No pointer was held, so nothing was released.
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        for offset in CLEARED_WORDS_00B9FDA0 {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        for (array, vtable) in [
            (0x210u32, SIMPLE_ARRAY_VTABLE_010B7EA8),
            (0x220, SIMPLE_ARRAY_VTABLE_010B7EA8),
            (0x230, SIMPLE_ARRAY_VTABLE_0101A8E8),
        ] {
            assert_eq!(e.mem.u32(this + array), vtable);
            assert_eq!(e.mem.bytes(this + array + 4, 12), vec![0; 12]);
        }
        assert_eq!(e.mem.u32(this + 0x20c), 0);
        for (row, column) in (0..4u32).flat_map(|r| (0..4u32).map(move |c| (r, c))) {
            let want = if row == column { 1.0 } else { 0.0 };
            assert_eq!(e.mem.f32(this + 0x10 + 16 * row + 4 * column), want);
        }
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0xd0), 0);
        assert_eq!(e.mem.u32(this + 0xd4), 0);
        assert_eq!(e.mem.f32(this + 0xd8), 1.0);
        assert_eq!(e.mem.u32(this + 0xdc), 0);
        assert_eq!(e.mem.u32(this + 0xec), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0xf4), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0xfc), 0xdead_be00);
        assert_eq!(
            e.mem.bytes(this + 0x100, 12),
            [0x11u32, 0x22, 0x33]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        assert_eq!(e.mem.u32(this + 0x110), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0x118), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x11c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x120), 1.0);
        assert_eq!(e.mem.u32(this + 0x124), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x138), 0);
        assert_eq!(e.mem.u32(this + 0x208), 0xdead_be00);
        // +0x8 (iEnabledPasses) is not the constructor's.
        assert_eq!(e.mem.u32(this + 8), 0xdead_beef);
    }

    /// A list of `n` items (1 or 2) whose first element is `element`.
    fn pop_rig(n: u32, element: u32) -> (Engine, u32, Vec<u32>) {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(LIST_FREE_ITEM_00E6E830, |_, _| Ret::default());
        let items: Vec<u32> = (0..n).map(|_| e.mem.alloc(12)).collect();
        if n == 2 {
            e.mem.set_u32(items[0], items[1]);
            e.mem.set_u32(items[1] + 4, items[0]);
        }
        e.mem.set_u32(items[0] + 8, element);
        let list = e.mem.alloc(12);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[n as usize - 1]);
        e.mem.set_u32(list + 8, n);
        (e, list, items)
    }

    #[test]
    fn fn_00b9e7c0_takes_the_head_and_holds_its_element() {
        let (mut e, list, items) = pop_rig(2, 0x0f00_7000);
        e.map(0x0f00_7000, 8);
        e.mem.set_u32(0x0f00_7004, 3);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_e7c0, &args![list, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x0f00_7000);
        assert_eq!(e.mem.u32(0x0f00_7004), 4);
        assert_eq!(
            calls(&e, LIST_FREE_ITEM_00E6E830),
            vec![vec![list, items[0]]]
        );
        assert_eq!(e.mem.u32(list), items[1]);
        assert_eq!(e.mem.u32(items[1] + 4), 0);
        assert_eq!(e.mem.u32(list + 4), items[1]);
        assert_eq!(e.mem.u32(list + 8), 1);
    }

    #[test]
    fn fn_00b9e7c0_empties_a_one_item_list_and_holds_nothing_for_a_null_element() {
        let (mut e, list, items) = pop_rig(1, 0);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00b9_e7c0, &args![list, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(
            calls(&e, LIST_FREE_ITEM_00E6E830),
            vec![vec![list, items[0]]]
        );
        assert_eq!(e.mem.bytes(list, 12), vec![0; 12]);
    }
}

#[cfg(test)]
mod remove_tail_tests {
    use super::*;

    #[test]
    fn fn_00b9e850_moves_the_tail_element_out() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(LIST_FREE_ITEM_00E6E830, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        let element = e.mem.alloc(8);
        e.mem.set_u32(element + 4, 1);
        e.mem.set_u32(first, second);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(second + 8, element);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00b9_e850, &args![list, out]).u32(), out);
        assert_eq!(e.mem.u32(out), element);
        assert_eq!(e.mem.u32(element + 4), 2);
        assert_eq!(e.mem.u32(list + 4), first);
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(list + 8), 1);
        // The last item: the head is cleared and a null element is not held.
        e.call(0x00b9_e850, &args![list, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        let log = e.call_log.take().unwrap();
        let freed: Vec<_> = log
            .iter()
            .filter(|c| c.0 == LIST_FREE_ITEM_00E6E830)
            .collect();
        assert_eq!(
            freed,
            vec![
                &(LIST_FREE_ITEM_00E6E830, vec![list, second]),
                &(LIST_FREE_ITEM_00E6E830, vec![list, first])
            ]
        );
        let held: Vec<_> = log
            .iter()
            .filter(|c| c.0 == IMPORT_INTERLOCKED_INCREMENT)
            .collect();
        assert_eq!(held.len(), 1);
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

    const DELETE: u32 = 0x0f20_0000;
    const IS_NODE: u32 = 0x0f20_0004;
    const IS_GEOM: u32 = 0x0f20_0008;
    const GET_RTTI: u32 = 0x0f20_000c;

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x011a_d000,
            0x011f_4000,
            0x011f_9000,
            0x011f_f000,
            0x011a_9000,
            0x0101_6000,
            0x0120_0000,
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
        e.register(DELETE, |_, _| Ret::default());
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 0x14).into_ret());
        e.register(IS_NODE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(IS_GEOM, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(CRT_SQRT, |_, a| Ret {
            st0: f64::take(a, &mut 0).sqrt(),
            ..Ret::default()
        });
        e
    }

    /// An object (count 1) whose vtable holds the doubles: delete (+4), `GetRTTI` (+8: the
    /// word at +0x14), `IsNode` (+0xc: the word at +0x10), `IsTriBasedGeom` (+0x1c: the word
    /// at +0x1c).
    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 4, DELETE);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        e.mem.set_u32(vtable + 0xc, IS_NODE);
        e.mem.set_u32(vtable + 0x1c, IS_GEOM);
        let object = e.mem.alloc(0x250);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, 1);
        object
    }

    fn light(e: &mut Engine) -> (Ptr<ShadowSceneLight>, u32) {
        let this: Ptr<ShadowSceneLight> = e.new_object();
        let ni_light = object(e);
        e.set(this, ShadowSceneLight::spLight, ni_light);
        (this, ni_light)
    }

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

    fn bound(e: &mut Engine, center: [f32; 3], radius: f32) -> u32 {
        let b = e.mem.alloc(0x10);
        for (i, v) in center.iter().enumerate() {
            e.mem.set_f32(b + 4 * i as u32, *v);
        }
        e.mem.set_f32(b + 0xc, radius);
        b
    }

    #[test]
    fn bound_within_frustum_rejects_outside_bounds_and_drops_passed_planes() {
        let mut e = rig();
        let (this, _) = light(&mut e);
        let base = this.addr();
        // Plane 0: x >= 0 (normal +x, constant 0); plane 1 inactive.
        e.mem.set_f32(base + 0x144, 1.0);
        e.mem.set_u32(base + PLANES_ACTIVE, 1);
        let inside = bound(&mut e, [5.0, 0.0, 0.0], 1.0);
        assert_eq!(e.call(0x00b9_bbb0, &args![this, inside]).u8(), 1);
        // Wholly inside: the plane is dropped.
        assert_eq!(e.mem.u32(base + PLANES_ACTIVE), 0);
        e.mem.set_u32(base + PLANES_ACTIVE, 1);
        let crossing = bound(&mut e, [0.5, 0.0, 0.0], 1.0);
        assert_eq!(e.call(0x00b9_bbb0, &args![this, crossing]).u8(), 1);
        assert_eq!(e.mem.u32(base + PLANES_ACTIVE), 1);
        let outside = bound(&mut e, [-5.0, 0.0, 0.0], 1.0);
        assert_eq!(e.call(0x00b9_bbb0, &args![this, outside]).u8(), 0);
    }

    #[test]
    fn fn_00b9bc50_scales_a_point_lights_falloff_by_its_colour() {
        let mut e = rig();
        let (this, ni_light) = light(&mut e);
        e.mem.set_f32(ni_light + 0xc4, 2.0);
        e.mem.set_f32(ni_light + 0xd4, 0.5);
        e.mem.set_f32(ni_light + 0xd8, 0.25);
        e.mem.set_f32(ni_light + 0xe0, 10.0);
        // Not a point light: luminance 1 times the peak 2 * 0.5.
        let v = e
            .call(0x00b9_bc50, &args![this, 0u32, 0u32, 0u32, 0u32])
            .f64();
        assert_eq!(v, 1.0);
        e.set(this, ShadowSceneLight::bPointLight, 1);
        // Distance 5 of radius 10: 1 - 0.25.
        let v = e
            .call(0x00b9_bc50, &args![this, 5.0f32, 0.0f32, 0.0f32, 0u32])
            .f64();
        assert_eq!(v, 0.75);
        assert_eq!(e.get(this, ShadowSceneLight::fLuminance), 0.75);
        // The excluded light, or a shadow caster: 0.
        let v = e
            .call(0x00b9_bc50, &args![this, 0u32, 0u32, 0u32, ni_light])
            .f64();
        assert_eq!(v, 0.0);
        e.set(this, ShadowSceneLight::bCastShadow, 1);
        let v = e
            .call(0x00b9_bc50, &args![this, 0u32, 0u32, 0u32, 0u32])
            .f64();
        assert_eq!(v, 0.0);
    }

    #[test]
    fn fn_00b9cfd0_assigns_the_pointer_at_0x1a0() {
        let mut e = rig();
        let holder = e.mem.alloc(0x200);
        let old = object(&mut e);
        let new = object(&mut e);
        e.mem.set_u32(holder + 0x1a0, old);
        e.call_log = Some(vec![]);
        e.call(0x00b9_cfd0, &args![holder, new]);
        assert_eq!(e.mem.u32(holder + 0x1a0), new);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
    }

    #[test]
    fn bound_within_radius_measures_from_the_offset_light() {
        let mut e = rig();
        let (this, ni_light) = light(&mut e);
        let node = e.mem.alloc(0x200);
        e.set_global(SCENE_NODE_011F91C8, node);
        e.mem.set_f32(node + LIGHTING_OFFSET, 1.0);
        e.mem.set_f32(ni_light + 0xe0, 4.0);
        // Light at the origin moved to x = 1; bound at x = 4 of radius 1 scaled by 2: 3 - 2 = 1.
        let b = bound(&mut e, [4.0, 0.0, 0.0], 1.0);
        let r = e.call(0x00b9_dae0, &args![this, b, 0u32, 2.0f32]).u8();
        assert_eq!(r, 1);
        e.mem.set_f32(ni_light + 0xe0, 1.0);
        let r = e.call(0x00b9_dae0, &args![this, b, 0u32, 2.0f32]).u8();
        assert_eq!(r, 0);
        // The radius-free RTTI is always in range.
        e.mem.set_u32(ni_light + 0x14, RADIUS_FREE_RTTI_011F4A68);
        let r = e.call(0x00b9_dae0, &args![this, b, ni_light, 2.0f32]).u8();
        assert_eq!(r, 1);
    }

    #[test]
    fn luminance_to_bound_is_the_relative_gap() {
        let mut e = rig();
        let (this, ni_light) = light(&mut e);
        let node = e.mem.alloc(0x200);
        e.set_global(SCENE_NODE_011F91C8, node);
        e.mem.set_f32(ni_light + 0xe0, 4.0);
        let b = bound(&mut e, [0.0, 6.0, 0.0], 2.0);
        let v = e.call(0x00b9_dbe0, &args![this, b]).f64();
        assert_eq!(v, 1.0);
        assert_eq!(e.get(this, ShadowSceneLight::fLuminance), 1.0);
    }

    #[test]
    fn fn_00b9dcb0_drops_the_shadow_map_when_shadows_stop() {
        let mut e = rig();
        let (this, _) = light(&mut e);
        let map = object(&mut e);
        e.set(this, ShadowSceneLight::spDynamicShadowMap, map);
        e.set(this, ShadowSceneLight::bCastShadow, 1);
        e.set_global(RENDERER_011F91A8, 0x4242u32);
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        // Keeping shadows at a high render level with bit 0x10: the map stays.
        e.set_global(RENDER_LEVEL_011F91C0, 3u32);
        e.set_global(RENDER_FLAGS_011F91D8, 0x10u32);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dcb0, &args![this, 1u32]);
        assert!(calls(&e, RENDERER_RELEASE_MAP_00B6D4C0).is_empty());
        // Stopping: released.
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2B4, 5);
        e.call(0x00b9_dcb0, &args![this, 0u32]);
        assert_eq!(
            calls(&e, RENDERER_RELEASE_MAP_00B6D4C0),
            vec![vec![0x4242, map]]
        );
        assert_eq!(e.get(this, ShadowSceneLight::spDynamicShadowMap), 0);
        assert_eq!(e.get(this, ShadowSceneLight::bCastShadow), 0);
        assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 5);
    }

    /// A geometry whose property word is a property of type `kind`.
    fn geometry(e: &mut Engine, kind: u32) -> (u32, u32) {
        let geometry = object(e);
        e.mem.set_u32(geometry + 0x1c, geometry);
        let property = e.mem.alloc(0x80);
        e.mem.set_u32(property + 0x1c, kind);
        e.mem.set_u32(geometry + GEOMETRY_PROPERTY, property);
        (geometry, property)
    }

    #[test]
    fn fn_00b9e770_clears_from_the_fence_on() {
        let mut e = rig();
        let (this, _) = light(&mut e);
        let list = this.addr() + GEOM_LIST;
        let (first, first_property) = geometry(&mut e, 1);
        let (second, second_property) = geometry(&mut e, 1);
        e.mem.set_u32(first_property + 0x38, 7);
        e.mem.set_u32(second_property + 0x38, 7);
        append(&mut e, list, first);
        let fence = append(&mut e, list, second);
        e.set(this, ShadowSceneLight::kGeomListFence, fence);
        e.call(0x00b9_e770, &args![this]);
        assert_eq!(e.mem.u32(first_property + 0x38), 7);
        assert_eq!(e.mem.u32(second_property + 0x38), 0);
        assert_eq!(e.get(this, ShadowSceneLight::kGeomListFence), 0);
    }

    fn add_tri_geom_rig(e: &mut Engine) {
        e.register(ATEXIT, |_, _| Ret::default());
        e.register(GEOMETRY_TEST_00B4F580, |_, _| 1u32.into_ret());
        e.register(PROPERTY_TEST_004A2020, |_, _| 0u32.into_ret());
        e.register(PROPERTY_ADD_LIGHT_00B71560, |_, _| Ret::default());
        e.register(LIST_MOVE_00B702E0, |_, _| Ret::default());
        e.register(LIST_ADD_00C1B4A0, |e, a| {
            let value = e.mem.u32(a[1]);
            append(e, a[0], value);
            Ret::default()
        });
    }

    #[test]
    fn add_tri_geom_lists_a_new_geometry_and_refreshes_a_known_one() {
        let mut e = rig();
        add_tri_geom_rig(&mut e);
        let (this, _) = light(&mut e);
        let (geometry, property) = geometry(&mut e, 2);
        e.mem.set_u32(geometry + 0xc0, 0x5150);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f480, &args![this, geometry]);
        assert_eq!(calls(&e, ATEXIT), vec![vec![ADD_TRI_GEOM_STATIC_DESTRUCT]]);
        assert_eq!(
            calls(&e, PROPERTY_ADD_LIGHT_00B71560),
            vec![vec![property, this.addr(), NULL_BOUND]]
        );
        let list = this.addr() + GEOM_LIST;
        assert_eq!(e.mem.u32(e.mem.u32(list) + 8), geometry);
        assert_eq!(e.global::<u32>(ADD_TRI_GEOM_STATIC), 0);
        // Known now: no second item, the light refreshed again.
        e.call_log = Some(vec![]);
        e.call(0x00b9_f480, &args![this, geometry]);
        assert!(calls(&e, ATEXIT).is_empty());
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).len(), 1);
        // Not a lighting property: nothing.
        let (other, _) = self::geometry(&mut e, 0);
        e.mem.set_u32(other + 0xc0, 0x5150);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f480, &args![this, other]);
        assert!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).is_empty());
    }

    /// A node holding `children`.
    fn node_with(e: &mut Engine, children: &[u32]) -> u32 {
        let node = object(e);
        e.mem.set_u32(node + 0x10, node);
        let buffer = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *c);
        }
        e.mem.set_u32(node + CHILDREN_BASE, buffer);
        e.mem.set_u16(node + CHILDREN_SIZE, children.len() as u16);
        node
    }

    #[test]
    fn parse_object_add_frustum_adds_the_geometries_of_a_node() {
        let mut e = rig();
        add_tri_geom_rig(&mut e);
        let (this, _) = light(&mut e);
        let (geometry, property) = geometry(&mut e, 3);
        e.mem.set_u32(geometry + 0xc0, 1);
        let b = bound(&mut e, [0.0, 0.0, 0.0], 1.0);
        e.mem.set_u32(geometry + WORLD_BOUND, b);
        let (culled, _) = self::geometry(&mut e, 3);
        e.mem.set_u32(culled + WORLD_BOUND, b);
        e.mem.set_u32(culled + FLAGS, 1);
        let node = node_with(&mut e, &[geometry, culled]);
        e.mem.set_u32(this.addr() + PLANES_ACTIVE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f5b0, &args![this, node]);
        assert_eq!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).len(), 1);
        assert_eq!(calls(&e, PROPERTY_ADD_LIGHT_00B71560)[0][0], property);
        // The shadow scene itself is skipped.
        e.set(this, ShadowSceneLight::spShadowScene, node);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f5b0, &args![this, node]);
        assert!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).is_empty());
    }

    #[test]
    fn parse_object_add_radius_tests_the_radius_first() {
        let mut e = rig();
        add_tri_geom_rig(&mut e);
        let (this, ni_light) = light(&mut e);
        let node = e.mem.alloc(0x200);
        e.set_global(SCENE_NODE_011F91C8, node);
        e.mem.set_f32(ni_light + 0xe0, 2.0);
        let (geometry, _) = geometry(&mut e, 3);
        e.mem.set_u32(geometry + 0xc0, 1);
        let near = bound(&mut e, [1.0, 0.0, 0.0], 0.5);
        e.mem.set_u32(geometry + WORLD_BOUND, near);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f6a0, &args![this, geometry]);
        assert_eq!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).len(), 1);
        let far = bound(&mut e, [10.0, 0.0, 0.0], 0.5);
        e.mem.set_u32(geometry + WORLD_BOUND, far);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f6a0, &args![this, geometry]);
        assert!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).is_empty());
    }

    #[test]
    fn scalar_deleting_destructor_frees_with_bit_0() {
        let mut e = rig();
        let (this, _) = light(&mut e);
        e.register(SHADOW_SCENE_LIGHT_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_00e0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls(&e, SIZED_FREE), vec![vec![this.addr(), 0x250]]);
        e.call_log = Some(vec![]);
        e.call(0x00ba_00e0, &args![this, 0u32]);
        assert!(calls(&e, SIZED_FREE).is_empty());
        assert_eq!(
            calls(&e, SHADOW_SCENE_LIGHT_DESTRUCT),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn add_object_parses_nodes_by_radius_or_frustum_and_adds_geometries() {
        let mut e = rig();
        add_tri_geom_rig(&mut e);
        let (this, ni_light) = light(&mut e);
        let scene_node = e.mem.alloc(0x200);
        e.set_global(SCENE_NODE_011F91C8, scene_node);
        e.mem.set_f32(ni_light + 0xe0, 100.0);
        let (geometry, _) = geometry(&mut e, 3);
        e.mem.set_u32(geometry + 0xc0, 1);
        let b = bound(&mut e, [0.0, 0.0, 0.0], 1.0);
        e.mem.set_u32(geometry + WORLD_BOUND, b);
        let node = node_with(&mut e, &[geometry]);
        e.mem.set_u32(node + WORLD_BOUND, b);
        e.call_log = Some(vec![]);
        e.call(0x00ba_0110, &args![this, node]);
        assert_eq!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).len(), 1);
        // A geometry on its own is added directly.
        e.call_log = Some(vec![]);
        e.call(0x00ba_0110, &args![this, geometry]);
        assert_eq!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).len(), 1);
        // A culled node: not parsed, and not a geometry.
        e.mem.set_u32(node + FLAGS, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ba_0110, &args![this, node]);
        assert!(calls(&e, PROPERTY_ADD_LIGHT_00B71560).is_empty());
    }

    #[test]
    fn remove_tri_geom_removes_by_position_or_by_value() {
        let mut e = rig();
        let (this, _) = light(&mut e);
        let (geometry, property) = geometry(&mut e, 4);
        e.register(PROPERTY_REMOVE_LIGHT_00B717A0, |_, _| Ret::default());
        let removed = object(&mut e);
        e.register_double(LIST_REMOVE_POS, move |e, a| {
            e.mem.set_u32(a[1], removed);
            Ret::default()
        });
        e.register(LIST_REMOVE, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ba_01a0, &args![this, geometry, 0x5000u32]);
        assert_eq!(
            calls(&e, PROPERTY_REMOVE_LIGHT_00B717A0),
            vec![vec![property, this.addr()]]
        );
        assert_eq!(calls(&e, DELETE), vec![vec![removed]]);
        e.call_log = Some(vec![]);
        e.call(0x00ba_01a0, &args![this, geometry, 0u32]);
        let by_value = &calls(&e, LIST_REMOVE)[0];
        assert_eq!(by_value[0], this.addr() + GEOM_LIST);
        assert_eq!(e.mem.u32(geometry + 4), 1);
    }

    #[test]
    fn render_frustum_depth_map_builds_the_view_and_runs_the_depth_pass() {
        let mut e = rig();
        let (this, ni_light) = light(&mut e);
        let base = this.addr();
        let renderer = object(&mut e);
        let slots = e.mem.alloc(0x100);
        e.mem.set_u32(slots + 0xb4, 0x0f20_0100);
        e.mem.set_u32(slots + 0xac, 0x0f20_0104);
        e.mem.set_u32(renderer, slots);
        e.register(0x0f20_0100, |_, _| Ret::default());
        e.register(0x0f20_0104, |_, _| Ret::default());
        e.set_global(RENDERER_SINGLETON_011F4748, renderer);
        let state = e.mem.alloc(0xa00);
        e.set_global(STATE_OBJECT_011F9508, state);
        for i in 0..16u32 {
            e.mem.set_u32(state + 0x980 + 4 * i, 0x100 + i);
        }
        let camera = e.mem.alloc(0x100);
        // The first rotation row (+0x68, +0x74, +0x80) is x: the translation's x in it.
        e.mem.set_f32(camera + 0x68, 1.0);
        e.set(this, ShadowSceneLight::spShadowCamera, camera);
        e.mem.set_f32(ni_light + 0x68 + 0x24, 3.0);
        let accumulator = object(&mut e);
        e.set(this, ShadowSceneLight::spShadowAccumulator, accumulator);
        let holder = e.mem.alloc(0x100);
        for f in [
            RENDERING_SYSTEM_00BA3130,
            RENDERED_TEXTURE_STOP_OFFSCREEN,
            RENDER_TARGET_00B6B8D0,
            CAMERA_ACCUMULATOR_00B6C0D0,
            STATE_SET_CAMERA_004E9BB0,
            D3DX_MATRIX_MULTIPLY,
            RENDER_00B6B790,
            RENDERER_00B6D3F0,
            PASS_LOCAL_00BA37A0,
            PASS_LOCAL_DESTRUCT_00BA3A00,
            PASS_LIST_004EBA20,
            PASS_LIST_DESTRUCT_004EBA90,
            PASS_00BA3CB0,
            RENDER_00B97550,
            RENDERER_RELEASE_MAP_00B6D4C0,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let manager = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0x50);
        e.mem.set_u32(manager + 8, inner);
        e.set_global(RENDER_MANAGER_011F91AC, manager);
        e.call_log = Some(vec![]);
        e.call(0x00b9_f780, &args![this, holder, 2u32]);
        assert_eq!(
            calls(&e, RENDERING_SYSTEM_00BA3130),
            vec![vec![RENDERING_SYSTEM, 1, 0x13]]
        );
        // The view copied, its last row the negated translation in the camera frame.
        assert_eq!(e.mem.u32(base + 0x50), 0x100);
        assert_eq!(e.mem.f32(base + 0x88), -3.0);
        assert_eq!(e.mem.f32(base + 0x80), 0.0);
        assert_eq!(
            calls(&e, D3DX_MATRIX_MULTIPLY)[0][1..],
            [base + 0x50, base + 0x90]
        );
        // The accumulator went to the holder and was released again.
        assert_eq!(e.mem.u32(holder + 0xc4), 0);
        assert_eq!(e.mem.u32(accumulator + 4), 1);
        assert_eq!(calls(&e, RENDER_00B97550)[0][1], 0x11);
        assert_eq!(e.get(this, ShadowSceneLight::bClipSpacePlanesReady), 0);
    }

    fn frustum_cull_rig(e: &mut Engine) -> (Ptr<ShadowSceneLight>, u32, u32) {
        let (this, ni_light) = light(e);
        e.mem.set_f32(ni_light + 0xe0, 1.0);
        let culler = e.mem.alloc(0x100);
        let camera = e.mem.alloc(0x100);
        e.mem.set_u32(culler + 0xc, camera);
        e.register(SHADER_MANAGER_GET_ACCUMULATOR, |e, _| {
            let accumulator = e.mem.alloc(0x10);
            accumulator.into_ret()
        });
        e.set_global(DIMMER_NEAR_011F9444, 10.0f32);
        e.set_global(DIMMER_FAR_011F9448, 20.0f32);
        e.set_global(SHADOW_NEAR_011F944C, 10.0f32);
        e.set_global(SHADOW_FAR_011F9450, 20.0f32);
        e.set_global(ELAPSED_STEP_011F9410, 0.5f32);
        e.set_global(FADE_TIME_011AD834, 1.0f32);
        (this, ni_light, culler)
    }

    #[test]
    fn test_frustum_cull_fades_with_the_distance() {
        let mut e = rig();
        let (this, ni_light, culler) = frustum_cull_rig(&mut e);
        // No active plane: inside; at distance 16 - 1 = 15: dimmer 1 - 5/10 = 0.5.
        e.mem.set_f32(ni_light + 0x8c, 16.0);
        e.set(this, ShadowSceneLight::fShadowLODDimmerTarget, 1.0);
        e.call(0x00b9_e970, &args![this, culler]);
        assert_eq!(e.get(this, ShadowSceneLight::usFrustumCull), 0);
        assert_eq!(e.get(this, ShadowSceneLight::fLODDimmer), 0.5);
        assert_eq!(e.get(this, ShadowSceneLight::fShadowLODDimmerElapsed), 0.5);
        // Not casting shadows: the shadow dimmer (0) only scaled by the fade-in.
        assert_eq!(e.get(this, ShadowSceneLight::fShadowLODDimmer), 0.0);
        // Far away: culled, both dimmers 0.
        e.mem.set_f32(ni_light + 0x8c, 100.0);
        e.call(0x00b9_e970, &args![this, culler]);
        assert_eq!(e.get(this, ShadowSceneLight::usFrustumCull), 0xff);
        assert_eq!(e.get(this, ShadowSceneLight::fLODDimmer), 0.0);
    }

    #[test]
    fn test_frustum_cull_marks_the_geometries_when_the_state_changes() {
        let mut e = rig();
        let (this, ni_light, culler) = frustum_cull_rig(&mut e);
        let (geometry, property) = geometry(&mut e, 2);
        append(&mut e, this.addr() + GEOM_LIST, geometry);
        // A plane the sphere is outside of: x >= 0 with the light at x = -5.
        e.mem.set_f32(culler + 0x2c, 1.0);
        e.mem.set_u32(culler + 0x2c + 0x60, 1);
        e.mem.set_f32(ni_light + 0x8c, -5.0);
        e.call(0x00b9_e970, &args![this, culler]);
        assert_eq!(e.get(this, ShadowSceneLight::usFrustumCull), 0xff);
        assert_eq!(e.mem.u8(property + 0x74), 1);
    }

    #[test]
    fn test_frustum_cull_asks_the_camera_test_for_shadow_casters() {
        let mut e = rig();
        let (this, _, culler) = frustum_cull_rig(&mut e);
        e.set(this, ShadowSceneLight::bCastShadow, 1);
        e.set(this, ShadowSceneLight::spShadowCamera, 0x6600);
        e.register(LIGHT_CAMERA_TEST_00B9C020, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b9_e970, &args![this, culler]);
        let camera = e.mem.u32(culler + 0xc);
        assert_eq!(
            calls(&e, LIGHT_CAMERA_TEST_00B9C020),
            vec![vec![this.addr(), camera, 0x6600]]
        );
        assert_eq!(e.get(this, ShadowSceneLight::usFrustumCull), 0xff);
    }
}

#[cfg(test)]
mod setup_frustum_tests {
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

    const DELETE: u32 = 0x0f30_0000;

    fn counted(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, DELETE);
        let object = e.mem.alloc(0x250);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, 1);
        object
    }

    fn rig() -> (Engine, Ptr<ShadowSceneLight>) {
        let mut e = Engine::new();
        for page in [
            0x0101_6000,
            0x0101_8000,
            0x0102_0000,
            0x0103_1000,
            0x0103_2000,
            0x0106_b000,
            0x0106_c000,
            0x0108_2000,
            0x010b_7000,
            0x011a_9000,
            0x011f_4000,
            0x011f_9000,
            0x0120_0000,
            0x0120_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(SHADOW_DIMMER_LIMIT_01082CB0, 0.05f64);
        e.set_global(SMALL_POSITIVE_01031148, 0.01f64);
        e.set_global(SMALL_NEGATIVE_010B7EA0, -0.01f64);
        e.set_global(EXTENT_SCALE_01016FF0, 1.5f64);
        e.set_global(EXTENT_CAP_01032ADC, 256.0f32);
        e.set_global(FOV_SCALE_0106C948, 150.0f64);
        e.set_global(SUN_OFFSET_SCALE_01020758, 10.0f64);
        e.set_global(FAR_EXTRA_0106B178, 400.0f64);
        e.set_global(NEAR_SHARE_01018A90, 0.2f64);
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
        e.register(DELETE, |_, _| Ret::default());
        e.register(RENDERING_SET_THREAD_STAGE, |_, _| Ret::default());
        e.register(RENDERER_RELEASE_MAP_00B6D4C0, |_, _| Ret::default());
        let this: Ptr<ShadowSceneLight> = e.new_object();
        let ni_light = e.mem.alloc(0x100);
        e.set(this, ShadowSceneLight::spLight, ni_light);
        (e, this)
    }

    #[test]
    fn without_shadows_the_map_and_accumulator_are_released() {
        let (mut e, this) = rig();
        let map = counted(&mut e);
        let accumulator = counted(&mut e);
        e.set(this, ShadowSceneLight::spDynamicShadowMap, map);
        e.set(this, ShadowSceneLight::spShadowAccumulator, accumulator);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dfc0, &args![this, 0u32, 3u32]);
        assert_eq!(calls(&e, DELETE), vec![vec![map], vec![accumulator]]);
        assert_eq!(e.get(this, ShadowSceneLight::spDynamicShadowMap), 0);
        assert_eq!(e.get(this, ShadowSceneLight::spShadowAccumulator), 0);
        assert_eq!(
            calls(&e, RENDERING_SET_THREAD_STAGE),
            vec![vec![RENDERING_SYSTEM, 0, 0x14]]
        );
    }

    #[test]
    fn a_shadow_spotlight_gets_its_camera_frustum_and_task() {
        let (mut e, this) = rig();
        e.set(this, ShadowSceneLight::bShadowSpotlight, 1);
        e.set(this, ShadowSceneLight::bCastShadow, 1);
        e.set(this, ShadowSceneLight::fShadowLODDimmer, 1.0);
        let map = counted(&mut e);
        e.set(this, ShadowSceneLight::spDynamicShadowMap, map);
        let ni_light = e.get(this, ShadowSceneLight::spLight);
        e.mem.set_f32(ni_light + 0x8c, 0.0);
        e.mem.set_f32(ni_light + 0xe0, 50.0);
        let scene = e.mem.alloc(0x100);
        e.mem.set_f32(scene + 0x8c, 10.0);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, 4.0);
        e.mem.set_u32(scene + 0x20, bound);
        e.set(this, ShadowSceneLight::spShadowScene, scene);
        let camera = e.mem.alloc(0x200);
        e.set(this, ShadowSceneLight::spShadowCamera, camera);
        let accumulator = counted(&mut e);
        e.register_double(ACCUMULATOR_SOURCE_00B540F0, move |_, _| {
            accumulator.into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINT_NORMALIZE_004A0C10, |_, _| Ret::default());
        // The cross product double writes (0, 0, 1) and answers the out pointer.
        e.register(POINT_UNIT_CROSS, |e, a| {
            e.mem.set_f32(a[1] + 8, 1.0);
            a[1].into_ret()
        });
        e.register(GET_EXTRA_DATA, |_, _| 0u32.into_ret());
        e.register(FLOAT_MAX, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(FLOAT_MIN, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e.register(POINT_LENGTH, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(SCRIPT_TEST_005BF7B0, |_, _| 1u32.into_ret());
        for f in [
            NI_FRUSTUM_CONSTRUCT,
            OBJECT_UPDATE_00A59C60,
            EXTRA_SCALED_POINT_0045BB20,
            POINT_ADD_0063C8A0,
            CAMERA_LOOK_AT_WORLD_POINT,
            CAMERA_SET_VIEW_FRUSTUM,
            CAMERA_SET_VIEW_PORT_00712E60,
            CAMERA_FIT_00A70080,
            FRUSTUM_PLANES_SET,
            RENDERING_ADD_ACCUM_TASK,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00b9_dfc0, &args![this, 0u32, 1u32]);
        // Width max(bound 4, 0) = 4 over distance 10, times 150.
        assert_eq!(e.get(this, ShadowSceneLight::fSpotlightFOV), 60.0);
        assert_eq!(
            e.get(this, ShadowSceneLight::spShadowAccumulator),
            accumulator
        );
        assert_eq!(e.mem.u32(accumulator + 0x19c), 7);
        assert_eq!(calls(&e, CAMERA_SET_VIEW_FRUSTUM).len(), 2);
        assert_eq!(
            calls(&e, RENDERING_ADD_ACCUM_TASK),
            vec![vec![
                RENDERING_SYSTEM,
                camera,
                0,
                scene,
                0,
                0,
                accumulator,
                0,
                0x12,
                0
            ]]
        );
        // The accumulator holds the light (00b9cfd0).
        assert_eq!(e.mem.u32(accumulator + 0x1a0), this.addr());
    }
}

#[cfg(test)]
mod camera_test_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn set(e: &mut Engine, at: u32, value: f32) {
        e.mem.set_u32(at, value.to_bits());
    }

    /// A camera with an identity rotation, its translation, far distance, `+0xe0` and
    /// `+0xe4`, and the six planes (normal, constant) the culler double hands out for it
    /// (kept at its +0x100).
    fn camera(e: &mut Engine, position: [f32; 3], far: f32, planes: [[f32; 4]; 6]) -> u32 {
        let camera = e.mem.alloc(0x200);
        for (index, value) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .into_iter()
            .enumerate()
        {
            set(e, camera + 0x68 + 4 * index as u32, value);
        }
        for (index, value) in position.into_iter().enumerate() {
            set(e, camera + 0x8c + 4 * index as u32, value);
        }
        set(e, camera + 0xe0, 0.5);
        set(e, camera + 0xe4, 0.25);
        set(e, camera + 0xf0, far);
        for (index, plane) in planes.into_iter().enumerate() {
            for (word, value) in plane.into_iter().enumerate() {
                set(
                    e,
                    camera + 0x100 + 0x10 * index as u32 + 4 * word as u32,
                    value,
                );
            }
        }
        e.mem.set_u32(camera + 0x160, 0x3f);
        camera
    }

    /// The inward planes of a box.
    fn boxed(x: [f32; 2], y: [f32; 2], z: [f32; 2]) -> [[f32; 4]; 6] {
        [
            [1.0, 0.0, 0.0, x[0]],
            [-1.0, 0.0, 0.0, -x[1]],
            [0.0, 1.0, 0.0, y[0]],
            [0.0, -1.0, 0.0, -y[1]],
            [0.0, 0.0, 1.0, z[0]],
            [0.0, 0.0, -1.0, -z[1]],
        ]
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_PLANE_CONSTRUCT, |_, _| Ret::default());
        e.register(CULLER_CONSTRUCT_00A69400, |_, _| Ret::default());
        e.register(CULLER_DESTROY_00A693E0, |_, _| Ret::default());
        e.register(CULLER_SET_FRUSTUM_00A694A0, |e, a| {
            let camera = e.mem.u32(a[0] + 0xc);
            assert_eq!(a[1], camera + 0xdc);
            for word in 0..0x19u32 {
                let value = e.mem.u32(camera + 0x100 + 4 * word);
                e.mem.set_u32(a[0] + 0x2c + 4 * word, value);
            }
            Ret::default()
        });
        e
    }

    fn test(e: &mut Engine, first: u32, second: u32) -> bool {
        e.call(0x00b9_c020, &args![0x4000u32, first, second]).bool()
    }

    const FAR_AWAY: [[f32; 4]; 6] = [
        [1.0, 0.0, 0.0, -1000.0],
        [-1.0, 0.0, 0.0, 900.0],
        [0.0, 1.0, 0.0, -10.0],
        [0.0, -1.0, 0.0, -10.0],
        [0.0, 0.0, 1.0, -10.0],
        [0.0, 0.0, -1.0, -10.0],
    ];

    #[test]
    fn camera_points_are_the_position_and_the_far_corners() {
        let mut e = engine();
        let cam = camera(&mut e, [1.0, 2.0, 3.0], 10.0, FAR_AWAY);
        let points = camera_points(&mut e, cam);
        let floats: Vec<[f32; 3]> = points.iter().map(|p| p.map(f32::from_bits)).collect();
        assert_eq!(
            floats,
            [
                [1.0, 2.0, 3.0],
                [11.0, 4.5, 8.0],
                [11.0, 4.5, -2.0],
                [11.0, -0.5, 8.0],
                [11.0, -0.5, -2.0]
            ]
        );
    }

    #[test]
    fn a_point_strictly_inside_the_other_volume_intersects() {
        let mut e = engine();
        let first = camera(&mut e, [1.0, 2.0, 3.0], 10.0, FAR_AWAY);
        let second = camera(
            &mut e,
            [500.0, 0.0, 0.0],
            5.0,
            boxed([0.0, 20.0], [-10.0, 10.0], [-10.0, 10.0]),
        );
        e.call_log = Some(vec![]);
        assert!(test(&mut e, first, second));
        assert_eq!(calls_to(&e, NI_PLANE_CONSTRUCT).len(), 18);
        assert_eq!(calls_to(&e, CULLER_CONSTRUCT_00A69400).len(), 1);
        assert_eq!(calls_to(&e, CULLER_SET_FRUSTUM_00A694A0).len(), 2);
        assert_eq!(calls_to(&e, CULLER_DESTROY_00A693E0).len(), 1);
    }

    #[test]
    fn separate_volumes_do_not_intersect() {
        let mut e = engine();
        let first = camera(&mut e, [1.0, 2.0, 3.0], 10.0, FAR_AWAY);
        let second = camera(
            &mut e,
            [110.0, 0.0, 0.0],
            5.0,
            boxed([100.0, 120.0], [-10.0, 10.0], [-10.0, 10.0]),
        );
        e.call_log = Some(vec![]);
        assert!(!test(&mut e, first, second));
        assert_eq!(calls_to(&e, CULLER_DESTROY_00A693E0).len(), 1);
    }

    #[test]
    fn an_edge_crossing_into_the_other_volume_intersects() {
        let mut e = engine();
        let first = camera(&mut e, [1.0, 2.0, 3.0], 10.0, FAR_AWAY);
        // A thin slab x in [5, 6]: no point inside, the first edge crosses x = 5 at t 0.4.
        let second = camera(
            &mut e,
            [500.0, 0.0, 0.0],
            5.0,
            boxed([5.0, 6.0], [-10.0, 10.0], [-10.0, 10.0]),
        );
        assert!(test(&mut e, first, second));
        // Moving the slab's y range away rejects every crossing point.
        let second = camera(
            &mut e,
            [500.0, 0.0, 0.0],
            5.0,
            boxed([5.0, 6.0], [50.0, 60.0], [-10.0, 10.0]),
        );
        assert!(!test(&mut e, first, second));
    }

    #[test]
    fn an_edge_lying_on_a_plane_intersects() {
        let mut e = engine();
        let first = camera(&mut e, [1.0, 2.0, 3.0], 10.0, FAR_AWAY);
        // The far corners lie on x = 11; every crossing point is outside y in [0, 4].
        let second = camera(
            &mut e,
            [500.0, 0.0, 0.0],
            5.0,
            boxed([11.0, 20.0], [0.0, 4.0], [-10.0, 10.0]),
        );
        assert!(test(&mut e, first, second));
    }

    #[test]
    fn the_crossing_parameter_must_lie_in_zero_to_one() {
        // The crossing of plane x = 5 by an edge from x = 1 to x = 11 at t 0.4.
        let plane = [1.0f32, 0.0, 0.0, 5.0].map(f32::to_bits);
        let start = [1.0f32, 2.0, 3.0].map(f32::to_bits);
        let end = [11.0f32, 4.5, 8.0].map(f32::to_bits);
        let mut e = engine();
        let planes = e.mem.alloc(0x64);
        for (index, row) in boxed([5.0, 6.0], [-10.0, 10.0], [-10.0, 10.0])
            .into_iter()
            .enumerate()
        {
            for (word, value) in row.into_iter().enumerate() {
                set(
                    &mut e,
                    planes + 0x10 * index as u32 + 4 * word as u32,
                    value,
                );
            }
        }
        assert!(edge_crosses_inside(&e, planes, 0, plane, start, end));
        // The same plane seen from beyond the end: t is above 1.
        let short_end = [3.0f32, 2.5, 4.0].map(f32::to_bits);
        assert!(!edge_crosses_inside(&e, planes, 0, plane, start, short_end));
        assert_eq!(side_of(0.0f32.to_bits()), 0);
        assert_eq!(side_of((-1.0f32).to_bits()), 2);
        assert_eq!(side_of(1.0f32.to_bits()), 1);
        assert_eq!(side_of(f32::NAN.to_bits()), 0);
    }
}

#[cfg(test)]
mod weighted_light_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn set3(e: &mut Engine, at: u32, values: [f32; 3]) {
        for (i, value) in values.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, value);
        }
    }

    fn get3(e: &Engine, at: u32) -> [f32; 3] {
        [0, 4, 8].map(|offset| e.mem.f32(at + offset))
    }

    /// The light, its shadow scene at (10, 20, 30) with scale 2, the scene node with its sun
    /// (direction (0, 0, -0.1), colour (0.5, 0.25, 1.0)) and the constants.
    fn rig() -> (Engine, Ptr<ShadowSceneLight>, u32) {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0102_1000,
            0x0102_3000,
            0x0102_4000,
            0x0102_9000,
            0x0103_0000,
            0x0103_2000,
            0x0107_3000,
            0x010b_7000,
            0x011f_4000,
            0x011f_9000,
            0x011f_f000,
            0x0120_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_01012060, 0.0f64);
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(RADIUS_SHARE_010B7E98, 0.0625f64);
        e.set_global(RADIUS_DIVISOR_010731A8, 75.0f64);
        e.set_global(SUN_RANGE_01017B70, 1000.0f64);
        e.set_global(LOWEST_HEIGHT_0101DE30, 0.75f64);
        e.set_global(RADIUS_CAP_010231D8, 256.0f64);
        e.set_global(OFFSET_SCALE_010B7E90, f64::from(1.4f32));
        e.set_global(SPECULAR_SCALE_010240B0, 8.0f64);
        e.set_global(COLOUR_SHARE_010290C0, f64::from(0.4f32));
        e.set_global(LOWEST_GREY_010217D8, f64::from(0.8f32));
        e.set_global(LOWEST_HEIGHT_01016264, 0.75f32);
        e.set_global(EXTENT_CAP_01032ADC, 256.0f32);
        e.set_global(LOWEST_GREY_01030FF0, 0.8f32);
        e.set_global(EXTRA_DATA_NAME_012031E4, 0x5150u32);
        e.register(CRT_SQRT, |_, a| Ret {
            st0: f64::take(a, &mut 0).sqrt(),
            ..Ret::default()
        });
        e.register(POINT_NORMALIZE_004A0C10, |_, _| Ret::default());
        e.register(OBJECT_UPDATE_00A59C60, |_, _| Ret::default());
        e.register(SCENE_NODE_LIGHT_AT_00B5AC20, |_, _| 0u32.into_ret());
        e.register(GET_EXTRA_DATA, |_, _| 0u32.into_ret());
        let this = Ptr::new(e.mem.alloc(0x300));
        let scene = e.mem.alloc(0x100);
        set3(&mut e, scene + 0x8c, [10.0, 20.0, 30.0]);
        e.mem.set_f32(scene + 0x98, 2.0);
        e.set(this, ShadowSceneLight::spShadowScene, scene);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(ni_light + 0xa8, 5);
        e.set(this, ShadowSceneLight::spLight, ni_light);
        let node = e.mem.alloc(0x100);
        let sun_light = e.mem.alloc(0x100);
        let sun = e.mem.alloc(0x100);
        e.mem.set_u32(node + 0xe0, sun_light);
        e.mem.set_u32(sun_light + 0xf8, sun);
        set3(&mut e, sun + 0xf0, [0.0, 0.0, -0.1]);
        set3(&mut e, sun + 0xd4, [0.5, 0.25, 1.0]);
        e.set_global(SCENE_NODE_011F91C8, node);
        (e, this, ni_light)
    }

    #[test]
    fn without_lights_the_sun_places_the_shadow_light() {
        let (mut e, this, ni_light) = rig();
        // No extra data: the radius is the largest word of 011f426c, 4.
        set3(&mut e, POINT_POSITION_011F426C, [1.0, 4.0, 2.0]);
        e.call_log = Some(vec![]);
        e.call(0x00b9_d150, &args![this, 0u32, 7u32, 8u32, 9u32]);
        assert_eq!(
            calls_to(&e, GET_EXTRA_DATA),
            vec![vec![e.get(this, ShadowSceneLight::spShadowScene), 0x5150]]
        );
        let node = e.global::<u32>(SCENE_NODE_011F91C8);
        assert_eq!(
            calls_to(&e, SCENE_NODE_LIGHT_AT_00B5AC20),
            vec![vec![node, 0]]
        );
        assert_eq!(calls_to(&e, POINT_NORMALIZE_004A0C10).len(), 3);
        assert_eq!(calls_to(&e, OBJECT_UPDATE_00A59C60).len(), 1);
        assert_eq!(calls_to(&e, OBJECT_UPDATE_00A59C60)[0][0], ni_light);
        // The sun's z, 0.1 * 4 * 0.0625 = 0.025, is raised to 4 / 75.
        let z = (4.0f64 / 75.0) as f32;
        let offset = (z * 4.0) * 1.4f32;
        assert_eq!(get3(&e, ni_light + 0x58), [10.0, 20.0, 30.0 + offset]);
        // Grey 0.4 * 1.75 = 0.7, raised to 0.8; specular 8 * 4.
        assert_eq!(get3(&e, ni_light + 0xd4), [0.8; 3]);
        assert_eq!(get3(&e, ni_light + 0xe0), [32.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(ni_light + 0xa8), 7);
    }

    #[test]
    fn weighted_lights_blend_with_the_sun() {
        let (mut e, this, ni_light) = rig();
        // Extra data: offset (1, 0, 0) and extents (0.5, 1, 0.25), scaled by 2: the position
        // (12, 20, 30) and the radius 2.
        let extra = e.mem.alloc(0x40);
        set3(&mut e, extra + 0xc, [1.0, 0.0, 0.0]);
        set3(&mut e, extra + 0x18, [0.5, 1.0, 0.25]);
        e.register_double(GET_EXTRA_DATA, move |_, _| extra.into_ret());
        set3(&mut e, POINT_POSITION_011F426C, [0.0; 3]);
        set3(&mut e, COLOUR_START_011F4998, [0.0; 3]);
        // Light a at distance 5, range 10; light b has flag bit 0 set.
        let mut lights = vec![];
        for (flags, at) in [(0u32, [15.0f32, 24.0, 30.0]), (1, [0.0, 0.0, 0.0])] {
            let light = e.mem.alloc(0x100);
            let ni = e.mem.alloc(0x100);
            e.mem.set_u32(light + 0xf8, ni);
            e.mem.set_u32(ni + 0x30, flags);
            set3(&mut e, ni + 0x8c, at);
            e.mem.set_f32(ni + 0xc4, 1.0);
            set3(&mut e, ni + 0xd4, [1.0; 3]);
            e.mem.set_f32(ni + 0xe0, 10.0);
            lights.push(light);
        }
        let list = lights.clone();
        e.register_double(GET_FIRST_ACTIVE_NON_SHADOW_LIGHT, move |e, a| {
            e.mem.set_u32(a[1], 0);
            list[0].into_ret()
        });
        let list = lights.clone();
        e.register_double(GET_NEXT_ACTIVE_NON_SHADOW_LIGHT, move |e, a| {
            let next = e.mem.u32(a[1]) + 1;
            e.mem.set_u32(a[1], next);
            list.get(next as usize).copied().unwrap_or(0).into_ret()
        });
        let sun = e
            .mem
            .u32(e.mem.u32(e.global::<u32>(SCENE_NODE_011F91C8) + 0xe0) + 0xf8);
        set3(&mut e, sun + 0xf0, [0.0, 0.0, -1.0]);
        set3(&mut e, sun + 0xd4, [0.5; 3]);
        e.call_log = Some(vec![]);
        e.call(0x00b9_d150, &args![this, 0x7700u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls_to(&e, GET_FIRST_ACTIVE_NON_SHADOW_LIGHT).len(), 2);
        assert!(calls_to(&e, SCENE_NODE_LIGHT_AT_00B5AC20).is_empty());
        let weight = (1.0f64 - 3.0 / 10.0) as f32;
        assert_eq!(e.mem.f32(LIGHT_WEIGHTS_011FFF40), weight);
        assert_eq!(e.mem.f32(LIGHT_WEIGHTS_011FFEA0), weight);
        // The share is 1; the sun direction (-0, -0, 2 * 0.0625) and colour 0.5 take 1 - b.
        let rest = 1.0 - weight;
        let x = 3.0 * weight + rest * -0.0;
        let y = 4.0 * weight + rest * -0.0;
        // z (0 + 0.125 * rest) is raised to 0.75.
        let reach = 2.0f32;
        let place = |d: f32| (d * reach) * 1.4f32;
        assert_eq!(
            get3(&e, ni_light + 0x58),
            [12.0 + place(x), 20.0 + place(y), 30.0 + place(0.75)]
        );
        // Colour b * 1 + (1 - b) * 0.5 per channel, grey 0.4 * 3 of it above 1.
        assert_eq!(get3(&e, ni_light + 0xd4), [1.0; 3]);
        assert_eq!(get3(&e, ni_light + 0xe0), [16.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(ni_light + 0xa8), 7);
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A light whose `NiLight` sits at the origin with dimmer 2, colour (0.5, 1.5, 1.0)
    /// and radius 10.
    fn light(e: &mut Engine, point: bool) -> (u32, u32) {
        let this = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0xf8, ni_light);
        e.mem.set_u8(this + 0xf4, u8::from(point));
        e.mem.set_f32(ni_light + NI_LIGHT_DIMMER, 2.0);
        e.mem.set_f32(ni_light + NI_LIGHT_COLOUR_D4, 0.5);
        e.mem.set_f32(ni_light + NI_LIGHT_COLOUR_D4 + 4, 1.5);
        e.mem.set_f32(ni_light + NI_LIGHT_COLOUR_D4 + 8, 1.0);
        e.mem.set_f32(ni_light + NI_LIGHT_RADIUS_E0, 10.0);
        (this, ni_light)
    }

    fn light_engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(ONE_01012070, 8);
        e.mem.set_u64(ONE_01012070, 1.0f64.to_bits());
        e.mem.map(ZERO_01012060, 8);
        e.mem.set_u64(ZERO_01012060, 0);
        // The vectors are unit already; the length of (5, 0, 0) is 5.
        e.register(POINT_NORMALIZE_004A0C10, |_, _| Ret::default());
        e.register(POINT_LENGTH, |e, a| st0(f64::from(e.mem.f32(a[0]))));
        e.register(FLOAT_MAX, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(FLOAT_MIN, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e
    }

    #[test]
    fn luminance_is_zero_for_the_excluded_shadow_or_culled_lights() {
        let mut e = light_engine();
        let (this, ni_light) = light(&mut e, false);
        let words = |ni: u32| args![this, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, ni];
        assert_eq!(e.call(0x00b9_bdf0, &words(ni_light)).f32(), 0.0);
        e.mem.set_u8(this + 0xec, 1);
        assert_eq!(e.call(0x00b9_bdf0, &words(0)).f32(), 0.0);
        e.mem.set_u8(this + 0xec, 0);
        e.mem.set_u32(ni_light + FLAGS, 1);
        assert_eq!(e.call(0x00b9_bdf0, &words(0)).f32(), 0.0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn luminance_combines_attenuation_facing_and_the_colour_peak() {
        let mut e = light_engine();
        let (this, _) = light(&mut e, true);
        // a = (5, 0, 0): a - light = (5, 0, 0) (taken as unit by the double), length 5,
        // t = 0.5, attenuation 0.75. b - a = (-1, 0, 0): facing -(-1) * 5 = 5, cut to 1.
        // Peak of 2 * (0.5, 1.5, 1.0) is 3.
        let five = 5.0f32.to_bits();
        let four = 4.0f32.to_bits();
        let value = e
            .call(
                0x00b9_bdf0,
                &args![this, five, 0u32, 0u32, four, 0u32, 0u32, 0u32],
            )
            .f32();
        assert_eq!(value, 0.75 * 3.0);
        assert_eq!(e.mem.f32(this + 0xc), 2.25);
        // Not a point light: no attenuation; b behind gives a negative facing cut to 0.
        let (other, _) = light(&mut e, false);
        let six = 6.0f32.to_bits();
        let value = e
            .call(
                0x00b9_bdf0,
                &args![other, five, 0u32, 0u32, six, 0u32, 0u32, 0u32],
            )
            .f32();
        assert_eq!(value, 0.0);
        // A NaN facing is kept through both cuts.
        let nan = 0x7fc0_0000u32;
        let value = e
            .call(
                0x00b9_bdf0,
                &args![other, five, 0u32, 0u32, nan, 0u32, 0u32, 0u32],
            )
            .f32();
        assert!(value.is_nan());
    }

    #[test]
    fn geometry_pass_runs_for_the_known_kinds_then_culls() {
        const SLOT_24: u32 = 0x03c0_0000;
        let mut e = Engine::new();
        for address in [
            RENDERER_00AA0FF0,
            PASS_SETTER_00B7A870,
            PASS_SETTER_00BBD880,
            NI_AV_OBJECT_CULL,
            SLOT_24,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.mem.map(GEOMETRY_CALLBACK_011F91EC, 4);
        e.mem.map(RENDER_FLAGS_011AD82C, 4);
        e.mem.set_u8(RENDER_FLAGS_011AD82C, 1);
        e.mem.map(PASS_STATE_011F91E0, 8);
        e.mem.set_u32(PASS_STATE_011F91E0, 9);
        e.mem.map(RENDERER_SINGLETON_011F4748, 4);
        let renderer = e.mem.alloc(0x10);
        e.mem.set_u32(renderer + 8, 0x4444);
        e.mem.set_u32(RENDERER_SINGLETON_011F4748, renderer);
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0x24, SLOT_24);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        let geometry = |e: &mut Engine, kind: i32, word: u32| {
            let geometry = e.mem.alloc(0xc4);
            let property = e.mem.alloc(0x70);
            e.mem.set_i32(property + 0x68, kind);
            e.mem.set_u32(property + 0x30, object);
            e.mem.set_u32(property + 0x34, object);
            e.mem.set_u32(geometry + 0xc0, property);
            e.mem.set_u32(geometry + 0xbc, word);
            geometry
        };
        let a = geometry(&mut e, 3, 1);
        let b = geometry(&mut e, 0x11, 0);
        let c = geometry(&mut e, 7, 0);
        let bare = e.mem.alloc(0xc4);
        let this = e.mem.alloc(0x250);
        let ni_light = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xf8, ni_light);
        // The list at +0xe0: a, null, b, c, a geometry without a property.
        let mut next = 0u32;
        for element in [bare, c, b, 0, a] {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 8, element);
            next = item;
        }
        e.mem.set_u32(this + GEOM_LIST, next);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b9_d020, &args![this, 0x7777u32]).bool());
        assert_eq!(calls_to(&e, PASS_SETTER_00B7A870), vec![vec![0x5c, 0]]);
        assert_eq!(calls_to(&e, PASS_SETTER_00BBD880), vec![vec![0x1c8, 0]]);
        assert_eq!(e.mem.u32(PASS_011F91E4), 0x1c8);
        assert_eq!(e.mem.u32(PASS_STATE_011F91E0), 0);
        assert_eq!(calls_to(&e, SLOT_24).len(), 4);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_CULL),
            vec![vec![a, 0x7777], vec![b, 0x7777]]
        );
        assert_eq!(calls_to(&e, RENDERER_00AA0FF0), vec![vec![0x4444]; 3]);
        // A busy renderer skips the passes but still culls.
        e.register(RENDERER_00AA0FF0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b9_d020, &args![this, 0x7777u32]);
        assert!(calls_to(&e, PASS_SETTER_00B7A870).is_empty());
        assert_eq!(calls_to(&e, NI_AV_OBJECT_CULL).len(), 2);
        // A culled `NiLight` does nothing.
        e.mem.set_u8(ni_light + FLAGS, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b9_d020, &args![this, 0x7777u32]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn cube_face_test_builds_a_camera_for_the_face() {
        let mut e = Engine::new();
        for address in [
            CUBE_MAP_CAMERA_CONSTRUCT,
            CUBE_MAP_CAMERA_DESTRUCT,
            NI_PLANE_CONSTRUCT,
            CULLER_CONSTRUCT_00A69400,
            CULLER_DESTROY_00A693E0,
            CULLER_SET_FRUSTUM_00A694A0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CUBE_MAP_CAMERA_SETUP_TRANSFORMATION, |e, a| {
            // The position is in place before the set-up.
            assert_eq!(e.mem.u32(a[0] + 0x58), 1);
            assert_eq!(e.mem.u32(a[0] + 0x60), 3);
            Ret::default()
        });
        e.register(OBJECT_UPDATE_00A59C60, |e, a| {
            assert_eq!(e.mem.bytes(a[1], 9), vec![0; 9]);
            Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2B4, 0x55);
        e.register_double(CUBE_MAP_CAMERA_DESTRUCT, move |e, _| {
            assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 0x20);
            Ret::default()
        });
        let this = e.mem.alloc(0x250);
        let source_camera = e.mem.alloc(0x200);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, source_camera);
        e.call_log = Some(vec![]);
        e.call(
            0x00b9_de90,
            &args![this, 1u32, 2u32, 3u32, 0xdeadu32, source, 0x2_0005u32],
        );
        let camera = calls_to(&e, CUBE_MAP_CAMERA_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, CUBE_MAP_CAMERA_CONSTRUCT),
            vec![vec![camera, 0]]
        );
        assert_eq!(
            calls_to(&e, CUBE_MAP_CAMERA_SETUP_TRANSFORMATION),
            vec![vec![camera, 5]]
        );
        assert_eq!(calls_to(&e, OBJECT_UPDATE_00A59C60)[0][0], camera);
        // The intersection test runs on the source's camera and the local one.
        assert_eq!(
            calls_to(&e, CULLER_SET_FRUSTUM_00A694A0),
            vec![
                vec![
                    calls_to(&e, CULLER_CONSTRUCT_00A69400)[0][0],
                    source_camera + 0xdc
                ],
                vec![calls_to(&e, CULLER_CONSTRUCT_00A69400)[0][0], camera + 0xdc]
            ]
        );
        assert_eq!(calls_to(&e, CUBE_MAP_CAMERA_DESTRUCT), vec![vec![camera]]);
        assert_eq!(e.mem.u32(tls + TLS_WORD_2B4), 0x55);
    }

    #[test]
    fn simple_array_destructor_frees_the_buffer() {
        const FREE: u32 = 0x03c1_0000;
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        e.mem.map(SIMPLE_ARRAY_FREE_SLOT_010B7EB0, 4);
        e.mem.set_u32(SIMPLE_ARRAY_FREE_SLOT_010B7EB0, FREE);
        let this = e.mem.alloc(0x10);
        for (offset, value) in [(4u32, 0x5000u32), (8, 3), (0xc, 4)] {
            e.mem.set_u32(this + offset, value);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b9_e8f0, &args![this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![this, 0x5000]]);
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_VTABLE_010B7EA8);
        assert_eq!(e.mem.bytes(this + 4, 12), vec![0; 12]);
        e.call(0x00b9_e8f0, &args![this]);
        assert_eq!(calls_to(&e, FREE).len(), 1);
    }

    #[test]
    fn simple_array_deleting_destructor_deallocates_on_request() {
        const FREE: u32 = 0x03c2_0000;
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.mem.map(SIMPLE_ARRAY_FREE_SLOT_010B7EB0, 4);
        e.mem.set_u32(SIMPLE_ARRAY_FREE_SLOT_010B7EB0, FREE);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_e920, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00b9_e920, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        assert_eq!(calls_to(&e, FREE).len(), 1);
    }
}
