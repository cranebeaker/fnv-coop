//! `fallout shared/run_004a3c20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The range holds three groups of small functions:
//!
//! - the constructor and accessors of `bhkPickData` (Xbox PDB), the ray
//!   cast request/result the pick and line-of-sight code fills: a Havok
//!   `hkpWorldRayCastInput` (+0x00), `hkpWorldRayCastOutput` (+0x30), a
//!   length vector (+0x90) and the two collector pointers;
//! - getters of an object with an embedded sub-object at +0x54 (floats and
//!   flag bytes) and menu-mode tests;
//! - two `BGSDecalManager` clean-up functions.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::bgsdecalmanager::{
    BGSDecalEmitter, BGSDecalManager, DecalPlacement,
};

// ---------------------------------------------------------------------
// Constants

/// `hkVector4` constructor (does nothing; returns `this`).
const HK_VECTOR4_CONSTRUCT: u32 = 0x0068_15c0;
/// `hkBool` constructor: stores the byte argument at `this` and returns
/// `this`.
const HK_BOOL_CONSTRUCT: u32 = 0x0062_2570;
/// Returns the address of a global `hkVector4` (`0x01267e30`), the default
/// the pick data's length vector is set to.
const DEFAULT_LENGTH_VECTOR: u32 = 0x0045_8b20;
/// `hkVector4::operator()(int)`: the address of component `index`
/// (`this + 4 * index`).
const HK_VECTOR4_COMPONENT: u32 = 0x0056_0d30;
/// Setter in `tesobjectrefr.cpp` taking a `hkVector4` pointer; its body
/// copies the 16 bytes to `this` (it calls `fn_004a3f10`).
const REFR_SET_VECTOR: u32 = 0x0056_d340;
/// Pointer dereference: returns the word at the address in `this`
/// (`NiPointer`-style `operator T*`).
const LOAD_POINTER: u32 = 0x0055_9450;
/// List iteration helper: advances the iterator word at the argument to
/// the next node and returns the address of the current node's element.
const LIST_ITERATE: u32 = 0x0057_cbe0;
/// The list's count (the word at `this + 8`).
const LIST_COUNT: u32 = 0x0044_ddc0;
/// `NiTPointerListBase::RemoveAll`.
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// Called on `pInstance + 8` (the pending simple decal list) by the
/// shutdown.
const PENDING_LIST_RESET: u32 = 0x004a_4650;
/// `BGSDecalManager::GetInstance`.
const DECAL_MANAGER_INSTANCE: u32 = 0x0049_fef0;
/// `BGSDecalManager::AddDecal` (Xbox PDB).
const DECAL_MANAGER_ADD_DECAL: u32 = 0x004a_10d0;
/// `BGSDecalEmitter` scalar deleting destructor.
const DECAL_EMITTER_DELETE: u32 = 0x004a_0490;
/// Returns the global at `0x011f4748` (a singleton the decal code calls
/// around the pending-list clean-up).
const PENDING_LOCK_OWNER: u32 = 0x0043_c4b0;
/// Called on that singleton before the clean-up.
const PENDING_LOCK_BEGIN: u32 = 0x004a_0370;
/// Called on that singleton after the clean-up.
const PENDING_LOCK_END: u32 = 0x004a_03c0;
/// Releases the occlusion query (`this` is the query; arguments: a pointer
/// to a zero word and `1`).
const OCCLUSION_QUERY_RELEASE: u32 = 0x00c4_edf0;
/// `BSOcclusionQuery` destructor body, called by `fn_004a4410`.
const OCCLUSION_QUERY_DESTRUCT: u32 = 0x00c4_ed40;
/// `operator delete` (cdecl, one argument).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Returns the singleton at `0x011c4180`.
const ELAPSED_TIME_OWNER: u32 = 0x0047_6c00;
/// Returns a tick count of the object as an unsigned 32-bit integer.
const ELAPSED_TICKS: u32 = 0x0047_6be0;

/// A global `float` the vector conversion multiplies by.
const VECTOR_SCALE: u32 = 0x011c_582c;
/// `double` factor `fn_004a4260` multiplies the tick count by.
const TICK_SCALE: u32 = 0x0101_e5a0;
/// The four decal counters of `BGSDecalManager` (`iDecalsThisFrame`,
/// `iSkinnedDecalsThisFrame`, `iDecalCount`, `iSkinnedDecalCount`).
const DECAL_COUNTERS: u32 = 0x011c_57e4;
/// `InterfaceManager::pInstance` (Xbox PDB).
const INTERFACE_MANAGER_INSTANCE: u32 = 0x011d_aac0;

// ---------------------------------------------------------------------
// Layouts

layout! {
    /// `hkVector4` (Havok), 16 bytes.
    pub struct HkVector4: 0x10 {
        0x00 x: f32,
        0x04 y: f32,
        0x08 z: f32,
        0x0C w: f32,
    }

    /// `hkpWorldRayCastInput` (Havok), 0x30 bytes.
    pub struct HkpWorldRayCastInput: 0x30 {
        /// `m_from`.
        0x00 m_from: Inline<HkVector4>,
        /// `m_to`.
        0x10 m_to: Inline<HkVector4>,
        /// `m_enableShapeCollectionFilter` (`hkBool`).
        0x20 m_enableShapeCollectionFilter: u8,
        /// `m_filterInfo`.
        0x24 m_filterInfo: u32,
    }

    /// `hkpShapeRayCastCollectorOutput` (Havok), 0x20 bytes.
    pub struct HkpShapeRayCastCollectorOutput: 0x20 {
        /// `m_normal`.
        0x00 m_normal: Inline<HkVector4>,
        /// `m_hitFraction`.
        0x10 m_hitFraction: f32,
        /// `m_extraInfo`.
        0x14 m_extraInfo: i32,
    }

    /// `hkpShapeRayCastOutput` (Havok), 0x50 bytes: the collector output
    /// followed by the shape keys.
    pub struct HkpShapeRayCastOutput: 0x50 {
        /// The base class `hkpShapeRayCastCollectorOutput`.
        0x00 collector_output: Inline<HkpShapeRayCastCollectorOutput>,
        /// `m_shapeKeys[0]` (32 entries from here).
        0x20 m_shapeKey_0: i32,
        /// `m_shapeKeyIndex`.
        0x40 m_shapeKeyIndex: i32,
    }

    /// `hkpWorldRayCastOutput` (Havok), 0x60 bytes.
    pub struct HkpWorldRayCastOutput: 0x60 {
        /// The base class `hkpShapeRayCastOutput`.
        0x00 shape_output: Inline<HkpShapeRayCastOutput>,
        /// `m_rootCollidable` (`const hkpCollidable*`).
        0x50 m_rootCollidable: Ptr,
    }

    /// `bhkPickData` (Xbox PDB), 0xB0 bytes (the constructor
    /// `fn_004a3c20` writes up to +0xAC; offsets agree with the PC code).
    pub struct BhkPickData: 0xb0 {
        /// Base class `hkpWorldRayCastInput`.
        0x00 input: Inline<HkpWorldRayCastInput>,
        /// Base class `hkpWorldRayCastOutput`.
        0x30 output: Inline<HkpWorldRayCastOutput>,
        /// `hkLength` (Xbox PDB).
        0x90 hkLength: Inline<HkVector4>,
        /// `pCache` (Xbox PDB): `char*`.
        0xA0 pCache: Ptr,
        /// `pCloseCollector` (Xbox PDB): `hkpClosestRayHitCollector*`.
        0xA4 pCloseCollector: Ptr,
        /// `pAllCollector` (Xbox PDB): `hkpAllRayHitCollector*`.
        0xA8 pAllCollector: Ptr,
        /// `bPickFailed` (Xbox PDB).
        0xAC bPickFailed: bool,
    }
}

// ---------------------------------------------------------------------
// hkVector4 and the Havok ray cast structures

// Translated from 004a3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16 bytes at `source` to `this` (an aligned `hkVector4`
/// copy, `movaps`) and returns `this`.
pub fn fn_004a3c90(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let bytes = e.mem.bytes(source.addr(), 16);
    e.mem.write(this.addr(), &bytes);
    this
}

// Translated from 004a3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16 bytes at `source` to `this` (the same `movaps` copy as
/// `fn_004a3c90`, without a return value).
pub fn fn_004a3f10(e: &mut Engine, this: Ptr, source: Ptr) {
    let bytes = e.mem.bytes(source.addr(), 16);
    e.mem.write(this.addr(), &bytes);
}

// Translated from 004a3d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the collector output's scalars: `m_hitFraction` = 1.0 and
/// `m_extraInfo` = -1.
pub fn fn_004a3d60(e: &mut Engine, this: Ptr<HkpShapeRayCastCollectorOutput>) {
    e.set(this, HkpShapeRayCastCollectorOutput::m_hitFraction, 1.0);
    e.set(this, HkpShapeRayCastCollectorOutput::m_extraInfo, -1);
}

// Translated from 004a3d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeRayCastCollectorOutput` constructor: constructs the normal
/// vector, then `fn_004a3d60`. Returns `this`.
pub fn fn_004a3d40(
    e: &mut Engine,
    this: Ptr<HkpShapeRayCastCollectorOutput>,
) -> Ptr<HkpShapeRayCastCollectorOutput> {
    e.call(HK_VECTOR4_CONSTRUCT, &args![this]);
    fn_004a3d60(e, this);
    this
}

// Translated from 004a3d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the shape ray cast output: `m_shapeKeyIndex` = 0 and the
/// first shape key = -1.
pub fn fn_004a3d80(e: &mut Engine, this: Ptr<HkpShapeRayCastOutput>) {
    e.set(this, HkpShapeRayCastOutput::m_shapeKeyIndex, 0);
    e.set(this, HkpShapeRayCastOutput::m_shapeKey_0, -1);
}

// Translated from 004a3d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeRayCastOutput` constructor: the collector output's
/// constructor, then `fn_004a3d80`. Returns `this`.
pub fn fn_004a3d20(e: &mut Engine, this: Ptr<HkpShapeRayCastOutput>) -> Ptr<HkpShapeRayCastOutput> {
    fn_004a3d40(e, this.at(HkpShapeRayCastOutput::collector_output));
    fn_004a3d80(e, this);
    this
}

// Translated from 004a3d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldRayCastOutput` constructor: the shape output's constructor and
/// a null `m_rootCollidable`. Returns `this`.
pub fn fn_004a3d00(e: &mut Engine, this: Ptr<HkpWorldRayCastOutput>) -> Ptr<HkpWorldRayCastOutput> {
    fn_004a3d20(e, this.at(HkpWorldRayCastOutput::shape_output));
    e.set(this, HkpWorldRayCastOutput::m_rootCollidable, Ptr::NULL);
    this
}

// Translated from 004a3cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldRayCastInput` constructor: constructs `m_from` and `m_to`,
/// the `hkBool` filter flag (false) and clears `m_filterInfo`. Returns
/// `this`.
pub fn fn_004a3cc0(e: &mut Engine, this: Ptr<HkpWorldRayCastInput>) -> Ptr<HkpWorldRayCastInput> {
    let from = this.at(HkpWorldRayCastInput::m_from);
    let to = this.at(HkpWorldRayCastInput::m_to);
    let filter = this.byte_add(HkpWorldRayCastInput::m_enableShapeCollectionFilter.off);
    e.call(HK_VECTOR4_CONSTRUCT, &args![from]);
    e.call(HK_VECTOR4_CONSTRUCT, &args![to]);
    e.call(HK_BOOL_CONSTRUCT, &args![filter, 0u32]);
    e.set(this, HkpWorldRayCastInput::m_filterInfo, 0);
    this
}

// Translated from 004a3c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData` constructor (layout from the Xbox PDB): constructs the
/// input (`fn_004a3cc0`) and output (`fn_004a3d00`), copies the default
/// length vector (the global `0x01267e30`, from `0x00458b20`) to
/// `hkLength` and clears the cache, both collectors and `bPickFailed`.
/// Returns `this`.
pub fn fn_004a3c20(e: &mut Engine, this: Ptr<BhkPickData>) -> Ptr<BhkPickData> {
    fn_004a3cc0(e, this.at(BhkPickData::input));
    fn_004a3d00(e, this.at(BhkPickData::output));
    let default_length = e.call(DEFAULT_LENGTH_VECTOR, &args![]).ptr::<()>();
    let length = this.at(BhkPickData::hkLength).cast();
    fn_004a3c90(e, length, default_length);
    e.set(this, BhkPickData::pCache, Ptr::NULL);
    e.set(this, BhkPickData::pCloseCollector, Ptr::NULL);
    e.set(this, BhkPickData::pAllCollector, Ptr::NULL);
    e.set(this, BhkPickData::bPickFailed, false);
    this
}

// Translated from 004a3e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `value` times the global `float` at `0x011c582c` (cdecl, result in
/// ST0 as a `float`).
pub fn fn_004a3e90(e: &mut Engine, value: f32) -> f32 {
    let scale = e.global::<f32>(VECTOR_SCALE);
    (scale as f64 * value as f64) as f32
}

// Translated from 004a3e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the 3-float vector at `input`, each component scaled by
/// `fn_004a3e90`, into the `hkVector4` at `out` (components addressed with
/// `0x00560d30`), with `w` = 0.0 (cdecl). Returns `out`.
pub fn fn_004a3e00(e: &mut Engine, out: Ptr, input: Ptr) -> Ptr {
    for index in 0..3u32 {
        let component = e.mem.f32(input.addr() + 4 * index);
        let scaled = fn_004a3e90(e, component);
        let slot = e.call(HK_VECTOR4_COMPONENT, &args![out, index]).ptr::<()>();
        e.mem.set_f32(slot.addr(), scaled);
    }
    let w = e.call(HK_VECTOR4_COMPONENT, &args![out, 3u32]).ptr::<()>();
    e.mem.set_f32(w.addr(), 0.0);
    out
}

// Translated from 004a3da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the vector at `input` with `fn_004a3e00` into a temporary
/// `hkVector4` and passes it to the setter `0x0056d340` on `this`.
pub fn fn_004a3da0(e: &mut Engine, this: Ptr, input: Ptr) {
    e.with_stack(16, |e, temp| {
        e.call(HK_VECTOR4_CONSTRUCT, &args![temp]);
        let converted = fn_004a3e00(e, temp, input);
        e.call(REFR_SET_VECTOR, &args![this, converted]);
    });
}

// Translated from 004a3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData`: copies the vector at `vector` to `hkLength`.
pub fn fn_004a3f90(e: &mut Engine, this: Ptr<BhkPickData>, vector: Ptr) {
    fn_004a3f10(e, this.at(BhkPickData::hkLength).cast(), vector);
}

// Translated from 004a3f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData`: resets `hkLength` to the default vector (`0x00458b20`)
/// and copies the vector at `to` into `m_to`.
pub fn fn_004a3f40(e: &mut Engine, this: Ptr<BhkPickData>, to: Ptr) {
    let default_length = e.call(DEFAULT_LENGTH_VECTOR, &args![]).ptr::<()>();
    fn_004a3f90(e, this, default_length);
    let m_to = this.at(BhkPickData::input).at(HkpWorldRayCastInput::m_to);
    fn_004a3f10(e, m_to.cast(), to);
}

// Translated from 004a3eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `fn_004a3da0`, but passes the converted temporary vector to
/// `fn_004a3f40` (`bhkPickData`: set the end point).
pub fn fn_004a3eb0(e: &mut Engine, this: Ptr<BhkPickData>, input: Ptr) {
    e.with_stack(16, |e, temp| {
        e.call(HK_VECTOR4_CONSTRUCT, &args![temp]);
        let converted = fn_004a3e00(e, temp, input);
        fn_004a3f40(e, this, converted);
    });
}

// Translated from 004a3f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData`: sets `m_filterInfo` to `value`, read back through the
/// pointer dereference `0x00559450` on the address of the argument.
pub fn fn_004a3f70(e: &mut Engine, this: Ptr<BhkPickData>, value: u32) {
    let loaded = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(LOAD_POINTER, &args![slot]).u32()
    });
    let input = this.at(BhkPickData::input);
    e.set(input, HkpWorldRayCastInput::m_filterInfo, loaded);
}

// Translated from 004a3fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPickData`: sets `pAllCollector` and clears `pCloseCollector`.
pub fn fn_004a3fb0(e: &mut Engine, this: Ptr<BhkPickData>, collector: Ptr) {
    e.set(this, BhkPickData::pAllCollector, collector);
    e.set(this, BhkPickData::pCloseCollector, Ptr::NULL);
}

// ---------------------------------------------------------------------
// Decals

// Translated from 004a3fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDecalEmitter` (`this`): adds a decal through the manager. When the
/// placement has no `object_48` yet it is set to the emitter, then
/// `BGSDecalManager::AddDecal` (Xbox PDB) runs on the singleton with the
/// placement, the decal type and the force flag (a byte).
pub fn fn_004a3fe0(
    e: &mut Engine,
    this: Ptr<BGSDecalEmitter>,
    placement: Ptr<DecalPlacement>,
    decal_type: u32,
    force: u8,
) {
    if e.get(placement, DecalPlacement::object_48).is_null() {
        e.set(placement, DecalPlacement::object_48, this.cast());
    }
    let manager = e
        .call(DECAL_MANAGER_INSTANCE, &args![])
        .ptr::<BGSDecalManager>();
    e.call(
        DECAL_MANAGER_ADD_DECAL,
        &args![manager, placement, decal_type, force as u32],
    );
}

// Translated from 004a42a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDecalManager` shutdown: zeroes the four decal counters, resets the
/// pending simple decal list (`0x004a4650` on `pInstance + 8`), then
/// destroys every emitter of `DecalEmitterList` (the scalar deleting
/// destructor `0x004a0490`, flag 1) and empties the list.
pub fn fn_004a42a0(e: &mut Engine) {
    for index in 0..4u32 {
        e.mem.set_u32(DECAL_COUNTERS + 4 * index, 0);
    }
    let manager = e
        .call(DECAL_MANAGER_INSTANCE, &args![])
        .ptr::<BGSDecalManager>();
    let pending = manager.at(BGSDecalManager::PendingSimpleDecalList);
    e.call(PENDING_LIST_RESET, &args![pending]);
    let list = manager.at(BGSDecalManager::DecalEmitterList);
    e.with_stack(4, |e, iterator| {
        let head = e.call(LOAD_POINTER, &args![list]).u32();
        e.mem.set_u32(iterator.addr(), head);
        while e.mem.u32(iterator.addr()) != 0 {
            let element = e.call(LIST_ITERATE, &args![list, iterator]).ptr::<()>();
            let emitter = e.mem.u32(element.addr());
            if emitter != 0 {
                e.call(DECAL_EMITTER_DELETE, &args![emitter, 1u32]);
            }
        }
    });
    e.call(LIST_REMOVE_ALL, &args![list]);
}

// Translated from 004a4410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the occlusion query held by a pending
/// decal: runs the destructor body `0x00c4ed40` and, when bit 0 of `flags`
/// is set, `operator delete`. Returns `this`.
pub fn fn_004a4410(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(OCCLUSION_QUERY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004a4340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDecalManager` (`this`): when the pending simple decal list is not
/// empty, brackets the work with `0x004a0370` / `0x004a03c0` on the
/// singleton `0x011f4748` and, for every pending decal that holds an
/// occlusion query (`pOcclusionQuery`, +0x1C), releases it (`0x00c4edf0`
/// with a pointer to a zero word and `1`), destroys it (`fn_004a4410`,
/// flag 1) and clears the pointer. The list itself is left as it is.
pub fn fn_004a4340(e: &mut Engine, this: Ptr<BGSDecalManager>) {
    let list = this.at(BGSDecalManager::PendingSimpleDecalList);
    if e.call(LIST_COUNT, &args![list]).u32() == 0 {
        return;
    }
    let lock_owner = e.call(PENDING_LOCK_OWNER, &args![]).u32();
    e.call(PENDING_LOCK_BEGIN, &args![lock_owner]);
    e.with_stack(8, |e, iterator| {
        let zero_word = iterator.addr() + 4;
        let head = e.call(LOAD_POINTER, &args![list]).u32();
        e.mem.set_u32(iterator.addr(), head);
        while e.mem.u32(iterator.addr()) != 0 {
            let element = e.call(LIST_ITERATE, &args![list, iterator]).ptr::<()>();
            let decal = e.call(LOAD_POINTER, &args![element]).u32();
            if decal == 0 {
                continue;
            }
            // BSTempEffectSimpleDecal::pOcclusionQuery (Xbox PDB) +0x1C
            let query = e.mem.u32(decal + 0x1c);
            if query == 0 {
                continue;
            }
            e.mem.set_u32(zero_word, 0);
            e.call(OCCLUSION_QUERY_RELEASE, &args![query, zero_word, 1u32]);
            let query = e.mem.u32(decal + 0x1c);
            if query != 0 {
                fn_004a4410(e, Ptr::new(query), 1);
            }
            e.mem.set_u32(decal + 0x1c, 0);
        }
    });
    let lock_owner = e.call(PENDING_LOCK_OWNER, &args![]).u32();
    e.call(PENDING_LOCK_END, &args![lock_owner]);
}

// ---------------------------------------------------------------------
// Menu mode tests

// Translated from 004a4020 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the signed byte at `this + 0x38` is positive (the callers are
/// the menu-topmost checks).
pub fn fn_004a4020(e: &mut Engine, this: Ptr) -> bool {
    e.mem.i8(this.addr() + 0x38) > 0
}

// Translated from 004a4080 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the word at `this + 0x1a8` has any bit of `mask` set.
pub fn fn_004a4080(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x1a8) & mask != 0
}

// Translated from 004a4040 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when `InterfaceManager::pInstance` (`0x011daac0`) is null, else
/// `fn_004a4080(instance, 1)`.
pub fn fn_004a4040(e: &mut Engine) -> bool {
    let instance = e.global::<Ptr>(INTERFACE_MANAGER_INSTANCE);
    !instance.is_null() && fn_004a4080(e, instance, 1)
}

// ---------------------------------------------------------------------
// Getters on the sub-object at +0x54

/// Address of the sub-object at `this + 0x54`.
fn sub_object(this: Ptr) -> Ptr {
    this.byte_add(0x54)
}

// Translated from 004a40a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at the sub-object's +0x00 (`0x006a7f50`).
pub fn fn_004a40a0(e: &mut Engine, this: Ptr) -> f32 {
    e.call(0x006a_7f50, &args![sub_object(this)]).f32()
}

// Translated from 004a40c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at the sub-object's +0x04 (`0x006b9130`).
pub fn fn_004a40c0(e: &mut Engine, this: Ptr) -> f32 {
    e.call(0x006b_9130, &args![sub_object(this)]).f32()
}

// Translated from 004a40e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at the sub-object's +0x14 (`0x0047c860`, which the engine
/// map names `TESActorBaseData::GetKarma` through code folding).
pub fn fn_004a40e0(e: &mut Engine, this: Ptr) -> f32 {
    e.call(0x0047_c860, &args![sub_object(this)]).f32()
}

// Translated from 004a4120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the byte at `this + 0x1D`.
pub fn fn_004a4120(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x1d) & 1 != 0
}

// Translated from 004a4100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_004a4120` on the sub-object at +0x54.
pub fn fn_004a4100(e: &mut Engine, this: Ptr) -> bool {
    fn_004a4120(e, sub_object(this))
}

// Translated from 004a4160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 1 of the byte at `this + 0x1D`.
pub fn fn_004a4160(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x1d) & 2 != 0
}

// Translated from 004a4140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_004a4160` on the sub-object at +0x54.
pub fn fn_004a4140(e: &mut Engine, this: Ptr) -> bool {
    fn_004a4160(e, sub_object(this))
}

// Translated from 004a41a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 2 of the byte at `this + 0x1D`.
pub fn fn_004a41a0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x1d) & 4 != 0
}

// Translated from 004a4180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_004a41a0` on the sub-object at +0x54.
pub fn fn_004a4180(e: &mut Engine, this: Ptr) -> bool {
    fn_004a41a0(e, sub_object(this))
}

// Translated from 004a41c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at the sub-object's +0x18 (`0x004a7bd0`, named
/// `VirtualActorPathHandler::GetDistanceTraveled` in the engine map).
pub fn fn_004a41c0(e: &mut Engine, this: Ptr) -> f32 {
    e.call(0x004a_7bd0, &args![sub_object(this)]).f32()
}

// Translated from 004a4200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x1C`, zero-extended to 16 bits.
pub fn fn_004a4200(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u8(this.addr() + 0x1c) as u16
}

// Translated from 004a41e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_004a4200` on the sub-object at +0x54.
pub fn fn_004a41e0(e: &mut Engine, this: Ptr) -> u16 {
    fn_004a4200(e, sub_object(this))
}

// Translated from 004a4220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at the sub-object's +0x20 (`0x007af430`, named
/// `BGSSaveFormBuffer::GetForm` in the engine map by code folding).
pub fn fn_004a4220(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x007a_f430, &args![sub_object(this)]).u32()
}

// ---------------------------------------------------------------------
// Tick conversion

// Translated from 004a4260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tick count `0x00476be0` returns for `this` (an unsigned 32-bit
/// value), times the `double` at `0x0101e5a0`, rounded to a `float`.
pub fn fn_004a4260(e: &mut Engine, this: Ptr) -> f32 {
    let ticks = e.call(ELAPSED_TICKS, &args![this]).u32();
    let factor = e.global::<f64>(TICK_SCALE);
    (ticks as f64 * factor) as f32
}

// Translated from 004a4240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_004a4260` on the object `0x00476c00` returns.
pub fn fn_004a4240(e: &mut Engine) -> f32 {
    let owner = e.call(ELAPSED_TIME_OWNER, &args![]).ptr::<()>();
    fn_004a4260(e, owner)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004a3c20,
            fn_004a3c20(Ptr<BhkPickData>) -> Ptr<BhkPickData>
        ),
        entry!(0x004a3c90, fn_004a3c90(Ptr, Ptr) -> Ptr),
        entry!(
            0x004a3cc0,
            fn_004a3cc0(Ptr<HkpWorldRayCastInput>) -> Ptr<HkpWorldRayCastInput>
        ),
        entry!(
            0x004a3d00,
            fn_004a3d00(Ptr<HkpWorldRayCastOutput>) -> Ptr<HkpWorldRayCastOutput>
        ),
        entry!(
            0x004a3d20,
            fn_004a3d20(Ptr<HkpShapeRayCastOutput>) -> Ptr<HkpShapeRayCastOutput>
        ),
        entry!(
            0x004a3d40,
            fn_004a3d40(Ptr<HkpShapeRayCastCollectorOutput>) -> Ptr<HkpShapeRayCastCollectorOutput>
        ),
        entry!(0x004a3d60, fn_004a3d60(Ptr<HkpShapeRayCastCollectorOutput>)),
        entry!(0x004a3d80, fn_004a3d80(Ptr<HkpShapeRayCastOutput>)),
        entry!(0x004a3da0, fn_004a3da0(Ptr, Ptr)),
        entry!(0x004a3e00, fn_004a3e00(Ptr, Ptr) -> Ptr),
        entry!(0x004a3e90, fn_004a3e90(f32) -> f32),
        entry!(0x004a3eb0, fn_004a3eb0(Ptr<BhkPickData>, Ptr)),
        entry!(0x004a3f10, fn_004a3f10(Ptr, Ptr)),
        entry!(0x004a3f40, fn_004a3f40(Ptr<BhkPickData>, Ptr)),
        entry!(0x004a3f70, fn_004a3f70(Ptr<BhkPickData>, u32)),
        entry!(0x004a3f90, fn_004a3f90(Ptr<BhkPickData>, Ptr)),
        entry!(0x004a3fb0, fn_004a3fb0(Ptr<BhkPickData>, Ptr)),
        entry!(
            0x004a3fe0,
            fn_004a3fe0(Ptr<BGSDecalEmitter>, Ptr<DecalPlacement>, u32, u8)
        ),
        entry!(0x004a4020, fn_004a4020(Ptr) -> bool),
        entry!(0x004a4040, fn_004a4040() -> bool),
        entry!(0x004a4080, fn_004a4080(Ptr, u32) -> bool),
        entry!(0x004a40a0, fn_004a40a0(Ptr) -> f32),
        entry!(0x004a40c0, fn_004a40c0(Ptr) -> f32),
        entry!(0x004a40e0, fn_004a40e0(Ptr) -> f32),
        entry!(0x004a4100, fn_004a4100(Ptr) -> bool),
        entry!(0x004a4120, fn_004a4120(Ptr) -> bool),
        entry!(0x004a4140, fn_004a4140(Ptr) -> bool),
        entry!(0x004a4160, fn_004a4160(Ptr) -> bool),
        entry!(0x004a4180, fn_004a4180(Ptr) -> bool),
        entry!(0x004a41a0, fn_004a41a0(Ptr) -> bool),
        entry!(0x004a41c0, fn_004a41c0(Ptr) -> f32),
        entry!(0x004a41e0, fn_004a41e0(Ptr) -> u16),
        entry!(0x004a4200, fn_004a4200(Ptr) -> u16),
        entry!(0x004a4220, fn_004a4220(Ptr) -> u32),
        entry!(0x004a4240, fn_004a4240() -> f32),
        entry!(0x004a4260, fn_004a4260(Ptr) -> f32),
        entry!(0x004a42a0, fn_004a42a0()),
        entry!(0x004a4340, fn_004a4340(Ptr<BGSDecalManager>)),
        entry!(0x004a4410, fn_004a4410(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The default length vector the exe keeps at `0x01267e30`.
    const DEFAULT_VECTOR: u32 = 0x0126_7e30;

    /// An engine with doubles for the trivial callees outside this file
    /// and the pages holding the globals the code reads.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011c_5000, 0x011d_a000, 0x0101_e000, 0x0126_7000] {
            e.map(page, 0x1000);
        }
        e.register(HK_VECTOR4_CONSTRUCT, |_, a| Ptr::<()>::new(a[0]).into_ret());
        e.register(HK_BOOL_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ptr::<()>::new(a[0]).into_ret()
        });
        e.register(DEFAULT_LENGTH_VECTOR, |_, _| {
            Ptr::<()>::new(DEFAULT_VECTOR).into_ret()
        });
        e.register(HK_VECTOR4_COMPONENT, |_, a| {
            Ptr::<()>::new(a[0] + 4 * a[1]).into_ret()
        });
        e.register(LOAD_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        e
    }

    fn set_vector(e: &mut Engine, at: u32, v: [f32; 4]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *x);
        }
    }

    fn vector(e: &Engine, at: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|o| e.mem.f32(at + o))
    }

    /// The arguments of every logged call to `addr`.
    fn calls(e: &Engine, log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        let _ = e;
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn vector_copy_with_result_returns_this() {
        let mut e = test_engine();
        let source = e.mem.alloc(16);
        let target = e.mem.alloc(16);
        set_vector(&mut e, source, [1.0, 2.0, 3.0, 4.0]);
        let back = e.call(0x004a_3c90, &args![target, source]).u32();
        assert_eq!(back, target);
        assert_eq!(vector(&e, target), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn vector_copy_copies_16_bytes() {
        let mut e = test_engine();
        let source = e.mem.alloc(16);
        let target = e.mem.alloc(32);
        set_vector(&mut e, source, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_u32(target + 16, 0x1234);
        e.call(0x004a_3f10, &args![target, source]);
        assert_eq!(vector(&e, target), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.u32(target + 16), 0x1234);
    }

    #[test]
    fn collector_output_scalars_start_at_one_and_minus_one() {
        let mut e = test_engine();
        let output: Ptr<HkpShapeRayCastCollectorOutput> = e.new_object();
        e.call(0x004a_3d60, &args![output]);
        assert_eq!(
            e.get(output, HkpShapeRayCastCollectorOutput::m_hitFraction),
            1.0
        );
        assert_eq!(
            e.get(output, HkpShapeRayCastCollectorOutput::m_extraInfo),
            -1
        );
    }

    #[test]
    fn collector_output_constructor_builds_the_normal_first() {
        let mut e = test_engine();
        let output: Ptr<HkpShapeRayCastCollectorOutput> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x004a_3d40, &args![output]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, output.addr());
        assert_eq!(
            calls(&e, &log, HK_VECTOR4_CONSTRUCT),
            vec![vec![output.addr()]]
        );
        assert_eq!(
            e.get(output, HkpShapeRayCastCollectorOutput::m_hitFraction),
            1.0
        );
    }

    #[test]
    fn shape_output_initializer_sets_key_and_index() {
        let mut e = test_engine();
        let output: Ptr<HkpShapeRayCastOutput> = e.new_object();
        e.set(output, HkpShapeRayCastOutput::m_shapeKeyIndex, 5);
        e.call(0x004a_3d80, &args![output]);
        assert_eq!(e.get(output, HkpShapeRayCastOutput::m_shapeKeyIndex), 0);
        assert_eq!(e.get(output, HkpShapeRayCastOutput::m_shapeKey_0), -1);
    }

    #[test]
    fn shape_output_constructor_runs_both_parts() {
        let mut e = test_engine();
        let output: Ptr<HkpShapeRayCastOutput> = e.new_object();
        e.set(output, HkpShapeRayCastOutput::m_shapeKeyIndex, 5);
        let back = e.call(0x004a_3d20, &args![output]).u32();
        assert_eq!(back, output.addr());
        assert_eq!(e.get(output, HkpShapeRayCastOutput::m_shapeKeyIndex), 0);
        assert_eq!(e.get(output, HkpShapeRayCastOutput::m_shapeKey_0), -1);
        let collector = output.at(HkpShapeRayCastOutput::collector_output);
        assert_eq!(
            e.get(collector, HkpShapeRayCastCollectorOutput::m_hitFraction),
            1.0
        );
    }

    #[test]
    fn world_output_constructor_clears_the_root_collidable() {
        let mut e = test_engine();
        let output: Ptr<HkpWorldRayCastOutput> = e.new_object();
        e.set(
            output,
            HkpWorldRayCastOutput::m_rootCollidable,
            Ptr::new(0x77),
        );
        let back = e.call(0x004a_3d00, &args![output]).u32();
        assert_eq!(back, output.addr());
        assert!(e
            .get(output, HkpWorldRayCastOutput::m_rootCollidable)
            .is_null());
        let shape = output.at(HkpWorldRayCastOutput::shape_output);
        assert_eq!(e.get(shape, HkpShapeRayCastOutput::m_shapeKey_0), -1);
    }

    #[test]
    fn world_input_constructor_builds_both_vectors_and_clears_the_filter() {
        let mut e = test_engine();
        let input: Ptr<HkpWorldRayCastInput> = e.new_object();
        e.set(input, HkpWorldRayCastInput::m_filterInfo, 9);
        e.set(
            input,
            HkpWorldRayCastInput::m_enableShapeCollectionFilter,
            1,
        );
        e.call_log = Some(vec![]);
        let back = e.call(0x004a_3cc0, &args![input]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, input.addr());
        assert_eq!(
            calls(&e, &log, HK_VECTOR4_CONSTRUCT),
            vec![vec![input.addr()], vec![input.addr() + 0x10]]
        );
        assert_eq!(
            calls(&e, &log, HK_BOOL_CONSTRUCT),
            vec![vec![input.addr() + 0x20, 0]]
        );
        assert_eq!(e.get(input, HkpWorldRayCastInput::m_filterInfo), 0);
        assert_eq!(
            e.get(input, HkpWorldRayCastInput::m_enableShapeCollectionFilter),
            0
        );
    }

    #[test]
    fn pick_data_constructor_resets_every_field() {
        let mut e = test_engine();
        set_vector(&mut e, DEFAULT_VECTOR, [10.0, 20.0, 30.0, 40.0]);
        let pick: Ptr<BhkPickData> = e.new_object();
        e.set(pick, BhkPickData::pCache, Ptr::new(1));
        e.set(pick, BhkPickData::pCloseCollector, Ptr::new(2));
        e.set(pick, BhkPickData::pAllCollector, Ptr::new(3));
        e.set(pick, BhkPickData::bPickFailed, true);
        let back = e.call(0x004a_3c20, &args![pick]).u32();
        assert_eq!(back, pick.addr());
        assert_eq!(vector(&e, pick.addr() + 0x90), [10.0, 20.0, 30.0, 40.0]);
        assert!(e.get(pick, BhkPickData::pCache).is_null());
        assert!(e.get(pick, BhkPickData::pCloseCollector).is_null());
        assert!(e.get(pick, BhkPickData::pAllCollector).is_null());
        assert!(!e.get(pick, BhkPickData::bPickFailed));
        assert_eq!(e.mem.u32(pick.addr() + 0x70), 0);
        assert_eq!(e.mem.u32(pick.addr() + 0x50), 0xffff_ffff);
    }

    #[test]
    fn scaling_multiplies_by_the_global() {
        let mut e = test_engine();
        e.set_global(VECTOR_SCALE, 0.5f32);
        assert_eq!(e.call(0x004a_3e90, &args![3.0f32]).f32(), 1.5);
    }

    #[test]
    fn vector_conversion_scales_three_components_and_zeroes_w() {
        let mut e = test_engine();
        e.set_global(VECTOR_SCALE, 2.0f32);
        let input = e.mem.alloc(12);
        let out = e.mem.alloc(16);
        set_vector(&mut e, input, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_f32(out + 12, 9.0);
        let back = e.call(0x004a_3e00, &args![out, input]).u32();
        assert_eq!(back, out);
        assert_eq!(vector(&e, out), [2.0, 4.0, 6.0, 0.0]);
    }

    #[test]
    fn set_vector_on_object_passes_the_converted_temporary() {
        let mut e = test_engine();
        e.set_global(VECTOR_SCALE, 2.0f32);
        e.register(REFR_SET_VECTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 16);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let input = e.mem.alloc(12);
        let target = e.mem.alloc(16);
        set_vector(&mut e, input, [1.0, 1.5, -2.0, 0.0]);
        e.call(0x004a_3da0, &args![target, input]);
        assert_eq!(vector(&e, target), [2.0, 3.0, -4.0, 0.0]);
    }

    #[test]
    fn pick_length_copies_the_vector() {
        let mut e = test_engine();
        let pick: Ptr<BhkPickData> = e.new_object();
        let source = e.mem.alloc(16);
        set_vector(&mut e, source, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x004a_3f90, &args![pick, source]);
        assert_eq!(vector(&e, pick.addr() + 0x90), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn pick_end_point_sets_m_to_and_resets_the_length() {
        let mut e = test_engine();
        set_vector(&mut e, DEFAULT_VECTOR, [7.0, 7.0, 7.0, 7.0]);
        let pick: Ptr<BhkPickData> = e.new_object();
        let to = e.mem.alloc(16);
        set_vector(&mut e, to, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x004a_3f40, &args![pick, to]);
        assert_eq!(vector(&e, pick.addr() + 0x10), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(vector(&e, pick.addr() + 0x90), [7.0, 7.0, 7.0, 7.0]);
        assert_eq!(vector(&e, pick.addr()), [0.0; 4]);
    }

    #[test]
    fn pick_end_point_from_a_three_float_vector() {
        let mut e = test_engine();
        e.set_global(VECTOR_SCALE, 10.0f32);
        let pick: Ptr<BhkPickData> = e.new_object();
        let input = e.mem.alloc(12);
        set_vector(&mut e, input, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x004a_3eb0, &args![pick, input]);
        assert_eq!(vector(&e, pick.addr() + 0x10), [10.0, 20.0, 30.0, 0.0]);
    }

    #[test]
    fn pick_filter_info_goes_through_the_dereference() {
        let mut e = test_engine();
        let pick: Ptr<BhkPickData> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x004a_3f70, &args![pick, 0x0012_3456u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(pick.addr() + 0x24), 0x0012_3456);
        assert_eq!(calls(&e, &log, LOAD_POINTER).len(), 1);
    }

    #[test]
    fn pick_all_collector_replaces_the_closest_collector() {
        let mut e = test_engine();
        let pick: Ptr<BhkPickData> = e.new_object();
        e.set(pick, BhkPickData::pCloseCollector, Ptr::new(0x55));
        e.call(0x004a_3fb0, &args![pick, 0x66u32]);
        assert_eq!(e.get(pick, BhkPickData::pAllCollector), Ptr::new(0x66));
        assert!(e.get(pick, BhkPickData::pCloseCollector).is_null());
    }

    fn decal_engine() -> Engine {
        let mut e = test_engine();
        e.register(DECAL_MANAGER_INSTANCE, |_, _| {
            Ptr::<()>::new(0x7000).into_ret()
        });
        e.register(DECAL_MANAGER_ADD_DECAL, |_, _| Ret::default());
        e
    }

    #[test]
    fn emitter_add_decal_fills_object_48_when_empty() {
        let mut e = decal_engine();
        let placement: Ptr<DecalPlacement> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x004a_3fe0, &args![0x4444u32, placement, 3u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            e.get(placement, DecalPlacement::object_48),
            Ptr::new(0x4444)
        );
        assert_eq!(
            calls(&e, &log, DECAL_MANAGER_ADD_DECAL),
            vec![vec![0x7000, placement.addr(), 3, 1]]
        );
    }

    #[test]
    fn emitter_add_decal_keeps_an_existing_object_48() {
        let mut e = decal_engine();
        let placement: Ptr<DecalPlacement> = e.new_object();
        e.set(placement, DecalPlacement::object_48, Ptr::new(0x9999));
        e.call(0x004a_3fe0, &args![0x4444u32, placement, 1u32, 0x1_00u32]);
        assert_eq!(
            e.get(placement, DecalPlacement::object_48),
            Ptr::new(0x9999)
        );
    }

    #[test]
    fn menu_flag_is_a_signed_positive_test() {
        let mut e = test_engine();
        let object = e.mem.alloc(0x40);
        assert!(!e.call(0x004a_4020, &args![object]).bool());
        e.mem.set_u8(object + 0x38, 1);
        assert!(e.call(0x004a_4020, &args![object]).bool());
        e.mem.set_u8(object + 0x38, 0x80);
        assert!(!e.call(0x004a_4020, &args![object]).bool());
    }

    #[test]
    fn mask_test_checks_any_bit() {
        let mut e = test_engine();
        let object = e.mem.alloc(0x1b0);
        e.mem.set_u32(object + 0x1a8, 0b100);
        assert!(!e.call(0x004a_4080, &args![object, 0b011u32]).bool());
        assert!(e.call(0x004a_4080, &args![object, 0b110u32]).bool());
    }

    #[test]
    fn menu_mode_needs_the_interface_manager() {
        let mut e = test_engine();
        e.set_global(INTERFACE_MANAGER_INSTANCE, Ptr::<()>::NULL);
        assert!(!e.call(0x004a_4040, &args![]).bool());
        let manager = e.mem.alloc(0x1b0);
        e.set_global(INTERFACE_MANAGER_INSTANCE, Ptr::<()>::new(manager));
        assert!(!e.call(0x004a_4040, &args![]).bool());
        e.mem.set_u32(manager + 0x1a8, 1);
        assert!(e.call(0x004a_4040, &args![]).bool());
    }

    /// A double that returns the `float` at its `this` plus `offset`.
    fn float_double(e: &mut Engine, addr: u32) {
        e.register(addr, |e, a| e.mem.f32(a[0]).into_ret());
    }

    #[test]
    fn float_getters_forward_the_sub_object() {
        let mut e = test_engine();
        for (entry, callee) in [
            (0x004a_40a0u32, 0x006a_7f50u32),
            (0x004a_40c0, 0x006b_9130),
            (0x004a_40e0, 0x0047_c860),
            (0x004a_41c0, 0x004a_7bd0),
        ] {
            float_double(&mut e, callee);
            let object = e.mem.alloc(0x60);
            e.mem.set_f32(object + 0x54, 2.5);
            assert_eq!(e.call(entry, &args![object]).f32(), 2.5);
        }
    }

    #[test]
    fn word_getter_forwards_the_sub_object() {
        let mut e = test_engine();
        e.register(0x007a_f430, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x004a_4220, &args![0x1000u32]).u32(), 0x1055);
    }

    #[test]
    fn flag_bits_are_tested_one_by_one() {
        let mut e = test_engine();
        let object = e.mem.alloc(0x80);
        // Direct tests on the object, then the forwarding ones on +0x54.
        for (direct, forwarding, bit) in [
            (0x004a_4120u32, 0x004a_4100u32, 1u8),
            (0x004a_4160, 0x004a_4140, 2),
            (0x004a_41a0, 0x004a_4180, 4),
        ] {
            e.mem.set_u8(object + 0x1d, 0);
            e.mem.set_u8(object + 0x54 + 0x1d, 0);
            assert!(!e.call(direct, &args![object]).bool());
            assert!(!e.call(forwarding, &args![object]).bool());
            e.mem.set_u8(object + 0x1d, bit);
            assert!(e.call(direct, &args![object]).bool());
            assert!(!e.call(forwarding, &args![object]).bool());
            e.mem.set_u8(object + 0x1d, !bit);
            assert!(!e.call(direct, &args![object]).bool());
            e.mem.set_u8(object + 0x54 + 0x1d, bit);
            assert!(e.call(forwarding, &args![object]).bool());
        }
    }

    #[test]
    fn byte_getter_widens_to_16_bits() {
        let mut e = test_engine();
        let object = e.mem.alloc(0x80);
        e.mem.set_u8(object + 0x1c, 0xf0);
        e.mem.set_u8(object + 0x54 + 0x1c, 0x07);
        assert_eq!(e.call(0x004a_4200, &args![object]).u32() & 0xffff, 0xf0);
        assert_eq!(e.call(0x004a_41e0, &args![object]).u32() & 0xffff, 7);
    }

    #[test]
    fn ticks_are_scaled_as_an_unsigned_number() {
        let mut e = test_engine();
        e.set_global(TICK_SCALE, 0.5f64);
        e.register(ELAPSED_TICKS, |_, a| (a[0] + 0x8000_0000).into_ret());
        // 0x80000000 + 4 as unsigned, halved.
        assert_eq!(
            e.call(0x004a_4260, &args![4u32]).f32(),
            (0x8000_0004u32 as f64 * 0.5) as f32
        );
    }

    #[test]
    fn current_ticks_use_the_singleton() {
        let mut e = test_engine();
        e.set_global(TICK_SCALE, 0.25f64);
        e.register(ELAPSED_TIME_OWNER, |_, _| Ptr::<()>::new(40).into_ret());
        e.register(ELAPSED_TICKS, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x004a_4240, &args![]).f32(), 10.0);
    }

    /// Builds a list at `list` of `elements` (nodes with next, previous and
    /// element words) and registers the head, iteration and count doubles.
    fn build_list(e: &mut Engine, list: u32, elements: &[u32]) {
        let mut next = 0;
        for element in elements.iter().rev() {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *element);
            next = node;
        }
        e.mem.set_u32(list, next);
        e.mem.set_u32(list + 8, elements.len() as u32);
    }

    fn list_engine() -> Engine {
        let mut e = decal_engine();
        e.register(LIST_ITERATE, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            (node + 8).into_ret()
        });
        e.register(LIST_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        for addr in [
            PENDING_LIST_RESET,
            LIST_REMOVE_ALL,
            DECAL_EMITTER_DELETE,
            PENDING_LOCK_BEGIN,
            PENDING_LOCK_END,
            OCCLUSION_QUERY_RELEASE,
            OCCLUSION_QUERY_DESTRUCT,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(PENDING_LOCK_OWNER, |_, _| Ptr::<()>::new(0x5000).into_ret());
        e
    }

    #[test]
    fn shutdown_zeroes_counters_and_deletes_every_emitter() {
        let mut e = list_engine();
        let manager = Ptr::<BGSDecalManager>::new(0x7000);
        e.map(0x7000, 0x1000);
        for i in 0..4 {
            e.mem.set_u32(DECAL_COUNTERS + 4 * i, 9);
        }
        build_list(&mut e, 0x7014, &[0x111, 0, 0x222]);
        e.call_log = Some(vec![]);
        e.call(0x004a_42a0, &args![]);
        let log = e.call_log.take().unwrap();
        for i in 0..4 {
            assert_eq!(e.mem.u32(DECAL_COUNTERS + 4 * i), 0);
        }
        assert_eq!(
            calls(&e, &log, PENDING_LIST_RESET),
            vec![vec![manager.addr() + 8]]
        );
        assert_eq!(
            calls(&e, &log, DECAL_EMITTER_DELETE),
            vec![vec![0x111, 1], vec![0x222, 1]]
        );
        assert_eq!(calls(&e, &log, LIST_REMOVE_ALL), vec![vec![0x7014]]);
    }

    #[test]
    fn shutdown_of_an_empty_list_still_empties_it() {
        let mut e = list_engine();
        e.map(0x7000, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x004a_42a0, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&e, &log, DECAL_EMITTER_DELETE).is_empty());
        assert_eq!(calls(&e, &log, LIST_REMOVE_ALL).len(), 1);
    }

    #[test]
    fn query_destructor_deletes_only_with_the_flag() {
        let mut e = list_engine();
        e.call_log = Some(vec![]);
        let back = e.call(0x004a_4410, &args![0x300u32, 1u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x300);
        assert_eq!(calls(&e, &log, OCCLUSION_QUERY_DESTRUCT), vec![vec![0x300]]);
        assert_eq!(calls(&e, &log, OPERATOR_DELETE), vec![vec![0x300]]);
        e.call_log = Some(vec![]);
        e.call(0x004a_4410, &args![0x300u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&e, &log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn pending_decals_with_queries_are_cleaned() {
        let mut e = list_engine();
        e.map(0x7000, 0x1000);
        let with_query = e.mem.alloc(0x40);
        let without_query = e.mem.alloc(0x40);
        e.mem.set_u32(with_query + 0x1c, 0x6000);
        build_list(&mut e, 0x7008, &[with_query, 0, without_query]);
        e.call_log = Some(vec![]);
        e.call(0x004a_4340, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(with_query + 0x1c), 0);
        let release = calls(&e, &log, OCCLUSION_QUERY_RELEASE);
        assert_eq!(release.len(), 1);
        assert_eq!(release[0][0], 0x6000);
        assert_eq!(release[0][2], 1);
        assert_eq!(
            calls(&e, &log, OCCLUSION_QUERY_DESTRUCT),
            vec![vec![0x6000]]
        );
        assert_eq!(calls(&e, &log, OPERATOR_DELETE), vec![vec![0x6000]]);
        assert_eq!(calls(&e, &log, PENDING_LOCK_BEGIN), vec![vec![0x5000]]);
        assert_eq!(calls(&e, &log, PENDING_LOCK_END), vec![vec![0x5000]]);
    }

    #[test]
    fn empty_pending_list_does_nothing() {
        let mut e = list_engine();
        e.map(0x7000, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x004a_4340, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&e, &log, PENDING_LOCK_BEGIN).is_empty());
        assert!(calls(&e, &log, PENDING_LOCK_END).is_empty());
    }
}
