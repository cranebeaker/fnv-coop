//! `fallout/ai/combat/combatthreatmap.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CombatThreatMap` (Xbox PDB) is an `NiTMap<unsigned int, CombatThreat *>`
//! (hash size 0x25) with a `BSReadWriteLock` at +0x10; the key is the word
//! `0084e3a0` reads from a reference (+0x0C). `CombatThreat` (Xbox PDB) is
//! 0x30 bytes.
//!
//! x87 notes: the code compares floats with `FCOM`/`FCOMPP` and tests the
//! status word, so every comparison below is written as the exact condition
//! that falls through (NaN included): `a < b` and `a > b` are false for NaN,
//! and the negations are spelled out where the exe branches on "not less".
//! A float that is pushed through `FLD` / `FSTP` (a copy from a parameter or
//! a global) leaves a signalling NaN quiet ([`quiet`]). Arithmetic is done
//! in `f64` and rounded to `f32` where the exe stores a `float`.
//!
//! Functions whose disassembly has an `FS:[0]` exception frame carry the
//! incomplete-translation marker below their doc comment (the unwinding
//! handlers are not translated; the ordinary behaviour is).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiPoint3, NiTMap};

/// `CombatThreatMap` vtable.
const VTABLE_COMBAT_THREAT_MAP: u32 = 0x0108_e2e8;
/// `NiTMap<unsigned int, CombatThreat *>` vtable.
const VTABLE_NI_TMAP: u32 = 0x0108_e308;
/// `NiTMapBase<...>` vtable.
const VTABLE_NI_TMAP_BASE: u32 = 0x0108_e328;

/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `2.0` (`double`).
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `FLT_MAX` as a `double`.
const FLOAT_MAX_DOUBLE: u32 = 0x0102_31b0;
/// The age (`double`, about 0.33) below which the cached linear velocity is
/// reused.
const AGE_LIMIT: u32 = 0x0102_6998;
/// `0.5` (`float`).
const HALF: u32 = 0x0101_6248;
/// `FLT_MAX` (`float`).
const FLOAT_MAX: u32 = 0x0108_e2b0;
/// `1024.0` (`float`).
const FLOAT_1024: u32 = 0x0102_36e0;
/// A global `NiPoint3` (three zero floats at start-up).
const ZERO_VECTOR: u32 = 0x011f_426c;
/// The `bool` setting object read through `00408d60`.
const SETTING_BOOL_011F1BA0: u32 = 0x011f_1ba0;
/// Float setting objects read through `00403e20` (value at +4).
const SETTING_011CF48C: u32 = 0x011c_f48c;
const SETTING_011CEF98: u32 = 0x011c_ef98;
const SETTING_011CE990: u32 = 0x011c_e990;
const SETTING_011CF3F0: u32 = 0x011c_f3f0;
const SETTING_011CEA74: u32 = 0x011c_ea74;
const SETTING_011CEABC: u32 = 0x011c_eabc;
const SETTING_011CF764: u32 = 0x011c_f764;
const SETTING_011CFBCC: u32 = 0x011c_fbcc;

/// `00403e20`: the address of a setting object's value (`this + 4`).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00408d60`: the same for a `bool` setting.
const SETTING_BOOL_VALUE: u32 = 0x0040_8d60;
/// `00435dd0`: the current combat time (a global float), in ST0.
const CURRENT_TIME: u32 = 0x0043_5dd0;
/// `00435de0` (`CombatTimeStamp` setter): stores its `float` argument at `*this`.
const TIME_STAMP_SET: u32 = 0x0043_5de0;
/// `00435e00`: current time minus the stamp at `*this`, in ST0.
const TIME_STAMP_AGE: u32 = 0x0043_5e00;
/// `004300a0`: constructs a `CombatTimeStamp` / `NiPoint3`-sized field.
const TIME_STAMP_INIT: u32 = 0x0043_00a0;
/// `00633c90`: constructs the `NiPointer` at +0x14 from one word.
const SMART_POINTER_INIT: u32 = 0x0063_3c90;
/// `0045cec0`: destroys the `NiPointer` at +0x14.
const SMART_POINTER_DESTROY: u32 = 0x0045_cec0;
/// `006815c0`: constructs a `NiPoint3`.
const VECTOR_INIT: u32 = 0x0068_15c0;
/// `00559450`: returns the word at `*this`.
const POINTER_VALUE: u32 = 0x0055_9450;
/// `009611e0`: returns the word at `this + 0x18`.
const WORD_AT_18: u32 = 0x0096_11e0;
/// `009a4690`: sets up the fields of a threat (reference, radius, stamps, damage).
const THREAT_SET_UP: u32 = 0x009a_4690;
/// `009a4960`: whether a time lies in a threat's time window.
const THREAT_IN_WINDOW: u32 = 0x009a_4960;
/// `009a49b0`: a weight from the threat and a reference.
const THREAT_WEIGHT: u32 = 0x009a_49b0;
/// `00457910`: unitizes the `NiPoint3` at `this` and returns its length.
const VECTOR_UNITIZE: u32 = 0x0045_7910;
/// `0045bb20`: `this * scalar` into `out`, returns `out`.
const VECTOR_SCALE: u32 = 0x0045_bb20;
/// `00439e90`: `this + other` into `out`, returns `out`.
const VECTOR_ADD: u32 = 0x0043_9e90;
/// `00439ef0`: `this - other` into `out`, returns `out`.
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
/// `004a7290`: squared length of the `NiPoint3` at `this`.
const VECTOR_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `004a3760` (cdecl): `scalar * vector` into `out`.
const VECTOR_SCALE_CDECL: u32 = 0x004a_3760;
/// `004b6190`: dot product of `this` and `other`.
const VECTOR_DOT: u32 = 0x004b_6190;
/// `0063c8a0`: adds `other` into `this`.
const VECTOR_ADD_IN_PLACE: u32 = 0x0063_c8a0;
/// `00404010` (cdecl): the larger of two floats (`b` when `b >= a` or unordered).
const FLOAT_MAXIMUM: u32 = 0x0040_4010;
/// `006479a0` (cdecl, three floats).
const FLOAT_REMAP: u32 = 0x0064_79a0;
/// `006915d0`: constructs the 0x24-byte avoid-node record from a point and two floats.
const AVOID_NODE_FROM_POINT: u32 = 0x0069_15d0;
/// `00905450`: appends a 0x24-byte record to the `BSSimpleArray` at `this`.
const AVOID_NODE_ARRAY_ADD: u32 = 0x0090_5450;
/// `00aa13e0` (cdecl): allocates `size` bytes.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `006e3850`: constructs the 0x18-byte avoid-node array at `this`.
const AVOID_NODE_ARRAY_INIT: u32 = 0x006e_3850;
/// `00401000` (cdecl): `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00401030` (cdecl): `operator delete`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00aa1070` (cdecl): allocates `size` bytes (the hash table).
const ALLOCATE_TABLE: u32 = 0x00aa_1070;
/// `00aa10f0` (cdecl): frees the hash table.
const FREE_TABLE: u32 = 0x00aa_10f0;
/// `00403d30` (cdecl): `memset(ptr, value, size)`.
const MEMSET: u32 = 0x0040_3d30;
/// `004b9ba0`: the first item of the first non-empty bucket, or 0.
const MAP_FIRST_ITEM: u32 = 0x004b_9ba0;
/// `006b7f20`: `(this, &item, &key, &value)` reads the item's key and value
/// and advances `*item` to the next item (0 at the end).
const MAP_NEXT_ITEM: u32 = 0x006b_7f20;
/// `00438af0`: clears the map's buckets.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00853130`: `(this, key, &value) -> bool` hash-map lookup.
const MAP_LOOKUP: u32 = 0x0085_3130;
/// `0084e3a0`: the word at `this + 0xC` (the map key of a reference).
const REFERENCE_KEY: u32 = 0x0084_e3a0;
/// `00aa6060` (`BSReadWriteLock::LockForWrite`, engine map) and its unlock.
const LOCK_FOR_WRITE: u32 = 0x00aa_6060;
const UNLOCK_AFTER_WRITE: u32 = 0x00aa_62e0;
/// The other lock pair used by the readers of the map.
const LOCK_ENTER: u32 = 0x00aa_5ff0;
const LOCK_LEAVE: u32 = 0x00aa_62c0;
/// `00aa5fc0`: constructs the lock.
const LOCK_INIT: u32 = 0x00aa_5fc0;
/// `00564db0` (`TESObjectREFR::SetTargeted`, engine map): sets or clears a flag
/// on the reference.
const SET_TARGETED: u32 = 0x0056_4db0;
/// `004534f0`: a one-word method of the threat, called with a flag.
const THREAT_SET_DEBUG: u32 = 0x0045_34f0;

layout! {
    /// `CombatThreat` (Xbox PDB), 0x30 bytes.
    pub struct CombatThreat: 0x30 {
        /// `pThreat` (Xbox PDB): `TESObjectREFR *` (the form ID while a saved
        /// threat is being loaded).
        0x00 pThreat: Ptr,
        /// `fRadius` (Xbox PDB).
        0x04 fRadius: f32,
        /// `fStartTime` (Xbox PDB): a `CombatTimeStamp` (one float).
        0x08 fStartTime: u32,
        /// `fEndTime` (Xbox PDB).
        0x0C fEndTime: u32,
        /// `fDamage` (Xbox PDB).
        0x10 fDamage: f32,
        /// `spDebugGeometry` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x14 spDebugGeometry: Ptr,
        /// `pCachedThreat3D` (Xbox PDB).
        0x18 pCachedThreat3D: Ptr,
        /// `pCachedRigidBody` (Xbox PDB).
        0x1C pCachedRigidBody: Ptr,
        /// `CachedLinearVelocity` (Xbox PDB).
        0x20 CachedLinearVelocity: Inline<NiPoint3>,
        /// `fCachedLinearVelocityTimeStamp` (Xbox PDB).
        0x2C fCachedLinearVelocityTimeStamp: u32,
    }

    /// `CombatThreatMap` (Xbox PDB): an `NiTMap` ([`NiTMap`] layout) followed by
    /// its lock.
    pub struct CombatThreatMap: 0x18 {
        /// `Lock` (Xbox PDB): a `BSReadWriteLock` (`uiLock`, `uiWriterThread`).
        0x10 Lock: u64,
    }
}

/// A float copied through the x87 stack (`FLD` then `FSTP`): a signalling NaN
/// comes out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

/// The value of the float setting object `setting` (`00403e20` returns the
/// address of its value).
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

fn current_time(e: &mut Engine) -> f32 {
    e.call(CURRENT_TIME, &args![]).f32()
}

/// Stores a `CombatTimeStamp` of the current time at `field`: the stamp is
/// built in a temporary (`00435de0`) and its word copied.
fn stamp_now(e: &mut Engine, field: u32) {
    let now = current_time(e);
    e.with_stack(4, |e, stamp| {
        e.call(TIME_STAMP_SET, &args![stamp, now]);
        let word = e.mem.u32(stamp.addr());
        e.mem.set_u32(field, word);
    });
}

/// Copies the three words of a `NiPoint3`.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

fn first_item(e: &mut Engine, map: Ptr) -> u32 {
    e.call(MAP_FIRST_ITEM, &args![map]).u32()
}

/// A new 0x18-byte avoid-node array (`BSSimpleArray`), or 0 when the
/// allocation fails.
fn new_avoid_node_array(e: &mut Engine) -> u32 {
    let memory = e.call(ALLOCATE, &args![0x18u32]).u32();
    if memory != 0 {
        e.call(AVOID_NODE_ARRAY_INIT, &args![memory]).u32()
    } else {
        0
    }
}

// Translated from 00844700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::SetAt` (Xbox PDB, folded instantiation): replaces a
/// value or prepends a new item to the selected bucket. Hash, equality,
/// allocation and key/value storage dispatch through the supplied vtable.
pub fn set_at(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let bucket = e.vcall(this.addr(), 4, &args![key]).u32().wrapping_mul(4);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let mut node = e.mem.u32(table.wrapping_add(bucket));
    while node != 0 {
        let existing_key = e.mem.u32(node.wrapping_add(4));
        if e.vcall(this.addr(), 8, &args![key, existing_key]).u8() != 0 {
            e.mem.set_u32(node.wrapping_add(8), value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(this.addr(), 0x14, &args![]).u32();
    e.vcall(this.addr(), 0xc, &args![node, key, value]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let next = e.mem.u32(table.wrapping_add(bucket));
    e.mem.set_u32(node, next);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.mem.set_u32(table.wrapping_add(bucket), node);
    let count = e.mem.u32(this.addr().wrapping_add(0xc));
    e.mem
        .set_u32(this.addr().wrapping_add(0xc), count.wrapping_add(1));
}

// Translated from 00405430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::RemoveAt` (Xbox PDB, folded instantiation): unlinks the
/// item with the given key from its bucket, destroys its value and frees it
/// through the vtable (slots 0x10 and 0x18), and counts it out. Returns
/// whether an item was removed.
pub fn remove_at(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let index = e.vcall(this.addr(), 4, &args![key]).u32();
    let table = e.mem.u32(this.addr().wrapping_add(8));
    let slot = table.wrapping_add(index.wrapping_mul(4));
    let node = e.mem.u32(slot);
    if node == 0 {
        return false;
    }
    let node_key = e.mem.u32(node.wrapping_add(4));
    if e.vcall(this.addr(), 8, &args![key, node_key]).u8() != 0 {
        let table = e.mem.u32(this.addr().wrapping_add(8));
        let next = e.mem.u32(node);
        e.mem
            .set_u32(table.wrapping_add(index.wrapping_mul(4)), next);
        release_item(e, this, node);
        return true;
    }
    let mut previous = node;
    let mut current = e.mem.u32(previous);
    while current != 0 {
        let current_key = e.mem.u32(current.wrapping_add(4));
        if e.vcall(this.addr(), 8, &args![key, current_key]).u8() != 0 {
            break;
        }
        previous = current;
        current = e.mem.u32(current);
    }
    if current == 0 {
        return false;
    }
    let next = e.mem.u32(current);
    e.mem.set_u32(previous, next);
    release_item(e, this, current);
    true
}

/// The tail both removal paths share: `ClearValue` (slot 0x10), `DeleteItem`
/// (slot 0x18) and the item count at +0xC.
fn release_item(e: &mut Engine, this: Ptr, node: u32) {
    e.vcall(this.addr(), 0x10, &args![node]);
    e.vcall(this.addr(), 0x18, &args![node]);
    let count = e.mem.u32(this.addr().wrapping_add(0xc));
    e.mem
        .set_u32(this.addr().wrapping_add(0xc), count.wrapping_sub(1));
}

// Translated from 009a44e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreat` constructor (the body matches the Xbox PDB class; the map
/// has no name for it): constructs the sub-objects, sets the threat up with
/// `009a4690` (flag 0), clears the cached 3D object and rigid body, copies
/// the zero vector into the cached velocity and stamps it with `-FLT_MAX`.
// Translation incomplete: 009a44e0 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a44e0(
    e: &mut Engine,
    this: Ptr<CombatThreat>,
    threat: Ptr,
    radius: f32,
    start_offset: f32,
    end_offset: f32,
    damage: f32,
) -> Ptr<CombatThreat> {
    e.call(TIME_STAMP_INIT, &args![this.byte_add(8)]);
    e.call(TIME_STAMP_INIT, &args![this.byte_add(0xc)]);
    e.call(SMART_POINTER_INIT, &args![this.byte_add(0x14), 0u32]);
    e.call(VECTOR_INIT, &args![this.byte_add(0x20)]);
    e.call(TIME_STAMP_INIT, &args![this.byte_add(0x2c)]);
    e.call(
        THREAT_SET_UP,
        &args![
            this,
            threat,
            quiet(radius),
            quiet(start_offset),
            quiet(end_offset),
            quiet(damage),
            0u32
        ],
    );
    e.set(this, CombatThreat::pCachedThreat3D, Ptr::NULL);
    e.set(this, CombatThreat::pCachedRigidBody, Ptr::NULL);
    copy_vector(e, ZERO_VECTOR, this.addr().wrapping_add(0x20));
    let lowest = -quiet(e.global::<f32>(FLOAT_MAX));
    let stamp = e.with_stack(4, |e, stamp| {
        e.call(TIME_STAMP_SET, &args![stamp, lowest]);
        e.mem.u32(stamp.addr())
    });
    e.set(this, CombatThreat::fCachedLinearVelocityTimeStamp, stamp);
    this
}

// Translated from 009a45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreat` destructor body: when the debug geometry pointer (+0x14)
/// holds an object whose word at +0x18 is set, calls that word's virtual
/// slot 0xE8 with the object, then destroys the pointer.
// Translation incomplete: 009a45e0 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a45e0(e: &mut Engine, this: Ptr<CombatThreat>) {
    let geometry = this.byte_add(0x14);
    if e.call(POINTER_VALUE, &args![geometry]).u32() != 0 {
        let object = e.call(POINTER_VALUE, &args![geometry]).u32();
        if e.call(WORD_AT_18, &args![object]).u32() != 0 {
            let object = e.call(POINTER_VALUE, &args![geometry]).u32();
            let parent = e.call(WORD_AT_18, &args![object]).u32();
            let object = e.call(POINTER_VALUE, &args![geometry]).u32();
            e.vcall(parent, 0xe8, &args![object]);
        }
    }
    e.call(SMART_POINTER_DESTROY, &args![geometry]);
}

// Translated from 009a4710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Score of the threat for `point` (a `NiPoint3 *`) and `other`; the map has no
/// name for it. Returns 0 when `time` is not negative and `009a4960` says it is
/// outside the threat's time window, or when the squared distance `d` found below
/// is not under `reach^2` (`reach = radius + extra`). Otherwise the weight `w`
/// (`009a49b0`) times `1 - d / reach^2`. `d` is the squared length of
/// `predicted - point` (predicted = reference position + velocity direction *
/// speed * 0.5) when `reach * setting 011cf48c` exceeds the speed; else the
/// squared distance of `point` from the line through the velocity direction,
/// or `FLT_MAX` when `point` is behind or beyond that segment.
pub fn fn_009a4710(
    e: &mut Engine,
    this: Ptr<CombatThreat>,
    point: Ptr,
    other: Ptr,
    time: f32,
    extra: f32,
    fallback: f32,
) -> f32 {
    // Not `time < 0`: the time must lie in the window.
    let negative_time = (time as f64) < e.global::<f64>(ZERO_DOUBLE);
    if !negative_time && !e.call(THREAT_IN_WINDOW, &args![this, quiet(time)]).bool() {
        return 0.0;
    }
    let radius = e.get(this, CombatThreat::fRadius);
    let reach = (radius as f64 + extra as f64) as f32;
    let scale = setting_value(e, SETTING_011CF48C);
    let limit = (reach as f64 * scale as f64) as f32;
    let score = e.with_stack(0x70, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(offset);
        let velocity = at(0x00);
        combat_threat_get_threat_linear_velocity(e, this, Ptr::new(velocity));
        let length = e.call(VECTOR_UNITIZE, &args![velocity]).f32();
        let scaled = e
            .call(VECTOR_SCALE, &args![velocity, at(0x0c), length])
            .u32();
        let half = quiet(e.global::<f32>(HALF));
        let offset = e.call(VECTOR_SCALE, &args![scaled, at(0x18), half]).u32();
        let threat = e.get(this, CombatThreat::pThreat);
        let position = e.vcall(threat.addr(), 0x1f4, &args![]).u32();
        let predicted = at(0x24);
        e.call(VECTOR_ADD, &args![position, predicted, offset]);
        if (limit as f64) > (length as f64) {
            e.call(VECTOR_SUBTRACT, &args![predicted, at(0x30), point]);
            return e.call(VECTOR_LENGTH_SQUARED, &args![at(0x30)]).f32();
        }
        let slack = e.call(FLOAT_MAXIMUM, &args![0.0f32, 0.0f32]).f32();
        let shortfall = (length as f64 * slack as f64 - reach as f64) as f32;
        let moved = e
            .call(VECTOR_SCALE, &args![velocity, at(0x48), shortfall])
            .u32();
        e.call(VECTOR_ADD, &args![predicted, at(0x3c), moved]);
        let double_reach = reach as f64 * e.global::<f64>(TWO_DOUBLE);
        let falloff = setting_value(e, SETTING_011CE990);
        let x = ((falloff as f64 - slack as f64) * length as f64 + double_reach) as f32;
        let x_squared = (x as f64 * x as f64) as f32;
        e.call(VECTOR_SCALE_CDECL, &args![at(0x54), x, velocity]);
        e.call(VECTOR_SUBTRACT, &args![point, at(0x60), at(0x3c)]);
        let dot = e.call(VECTOR_DOT, &args![at(0x60), at(0x54)]).f32();
        if (dot as f64) < e.global::<f64>(ZERO_DOUBLE) || (x_squared as f64) < (dot as f64) {
            return quiet(e.global::<f32>(FLOAT_MAX));
        }
        let length_squared = e.call(VECTOR_LENGTH_SQUARED, &args![at(0x60)]).f64();
        (length_squared - dot as f64 * dot as f64 / x_squared as f64) as f32
    });
    if (score as f64) < reach as f64 * reach as f64 {
        let weight = e
            .call(THREAT_WEIGHT, &args![this, other, quiet(fallback)])
            .f64();
        return (weight * (1.0 - score as f64 / (reach as f64 * reach as f64))) as f32;
    }
    0.0
}

// Translated from 009a4a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreat::GetThreatLinearVelocity` (Xbox PDB): writes the threat's
/// linear velocity to `result` and returns it. A cached velocity younger than
/// the age limit is reused; otherwise it is recomputed from the threat
/// reference (no 3D object: the zero vector; a character controller: its
/// velocity; a rigid body found for the reference's 3D object: its velocity),
/// cached with a new stamp.
pub fn combat_threat_get_threat_linear_velocity(
    e: &mut Engine,
    this: Ptr<CombatThreat>,
    result: Ptr,
) -> Ptr {
    let cache = this.addr().wrapping_add(0x20);
    let age = e.call(TIME_STAMP_AGE, &args![this.byte_add(0x2c)]).f64();
    if age < e.global::<f64>(AGE_LIMIT) {
        copy_vector(e, cache, result.addr());
        return result;
    }
    let threat = e.get(this, CombatThreat::pThreat);
    let object = e.vcall(threat.addr(), 0x1d0, &args![]).u32();
    if object == 0 {
        stamp_now(e, this.addr().wrapping_add(0x2c));
        copy_vector(e, ZERO_VECTOR, result.addr());
        return result;
    }
    let threat = e.get(this, CombatThreat::pThreat);
    if e.vcall(threat.addr(), 0x224, &args![]).u8() != 0 {
        let reference = e.get(this, CombatThreat::pThreat);
        // `MobileObject::GetCharController` (engine map name).
        let controller = e.call(0x0093_06d0, &args![reference]).u32();
        if controller != 0 {
            e.call(0x0066_ca00, &args![controller, cache]);
            stamp_now(e, this.addr().wrapping_add(0x2c));
            copy_vector(e, cache, result.addr());
            return result;
        }
        if e.call(0x0044_4d00, &args![reference, 8u32]).u8() != 0 {
            copy_vector(e, ZERO_VECTOR, cache);
            stamp_now(e, this.addr().wrapping_add(0x2c));
            copy_vector(e, cache, result.addr());
            return result;
        }
    }
    if e.get(this, CombatThreat::pCachedThreat3D).addr() != object {
        e.set(this, CombatThreat::pCachedThreat3D, Ptr::new(object));
        let cached = e.get(this, CombatThreat::pCachedThreat3D);
        let found = e.call(0x004b_5260, &args![cached]).u32();
        if found != 0 {
            let collision = e.call(0x006f_a820, &args![found]).u32();
            if collision != 0 {
                let body = e.call(0x0065_3270, &args![0x0126_81c0u32, collision]).u32();
                e.set(this, CombatThreat::pCachedRigidBody, Ptr::new(body));
            }
        }
    }
    let body = e.get(this, CombatThreat::pCachedRigidBody);
    if !body.is_null() {
        e.call(0x0056_0d50, &args![body, cache]);
    } else {
        copy_vector(e, ZERO_VECTOR, cache);
    }
    stamp_now(e, this.addr().wrapping_add(0x2c));
    copy_vector(e, cache, result.addr());
    result
}

// Translated from 009a4c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the 0x24-byte avoid-node record (`PathingAvoidNode` layout in the
/// Xbox PDB: two points, radius, cost, type) for a threat into `result` and
/// returns it. Inside the threat's reach it is a single point record
/// (`006915d0`); outside, a segment record (`009a4e30`) from the predicted
/// position to a point beyond it along the velocity.
pub fn fn_009a4c70(e: &mut Engine, this: Ptr<CombatThreat>, result: Ptr, other: Ptr) -> Ptr {
    e.with_stack(0x54, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(offset);
        let velocity = at(0x00);
        let predicted = at(0x0c);
        combat_threat_get_threat_linear_velocity(e, this, Ptr::new(velocity));
        let scale = setting_value(e, SETTING_011CF48C);
        let reach = (e.get(this, CombatThreat::fRadius) as f64 * scale as f64) as f32;
        let weight = e.call(THREAT_WEIGHT, &args![this, other, 0.0f32]).f32();
        let length = e.call(VECTOR_UNITIZE, &args![velocity]).f32();
        let scaled = e
            .call(VECTOR_SCALE, &args![velocity, at(0x18), length])
            .u32();
        let half = quiet(e.global::<f32>(HALF));
        let offset = e.call(VECTOR_SCALE, &args![scaled, at(0x24), half]).u32();
        let threat = e.get(this, CombatThreat::pThreat);
        let position = e.vcall(threat.addr(), 0x1f4, &args![]).u32();
        e.call(VECTOR_ADD, &args![position, predicted, offset]);
        if (reach as f64) > (length as f64) {
            let scale = setting_value(e, SETTING_011CEF98);
            let cost = (weight as f64 * scale as f64) as f32;
            let radius = quiet(e.get(this, CombatThreat::fRadius));
            e.call(
                AVOID_NODE_FROM_POINT,
                &args![result, predicted, radius, cost],
            );
            return result;
        }
        let length = e.call(VECTOR_UNITIZE, &args![velocity]).f32();
        let slack = e.call(FLOAT_MAXIMUM, &args![0.0f32, 0.0f32]).f32();
        let shortfall =
            (length as f64 * slack as f64 - e.get(this, CombatThreat::fRadius) as f64) as f32;
        let moved = e
            .call(VECTOR_SCALE, &args![velocity, at(0x3c), shortfall])
            .u32();
        e.call(VECTOR_ADD_IN_PLACE, &args![predicted, moved]);
        let double_radius = e.get(this, CombatThreat::fRadius) as f64 * e.global::<f64>(TWO_DOUBLE);
        let falloff = setting_value(e, SETTING_011CE990);
        let x = ((falloff as f64 - slack as f64) * length as f64 + double_radius) as f32;
        let beyond = e.call(VECTOR_SCALE, &args![velocity, at(0x48), x]).u32();
        let far = at(0x30);
        e.call(VECTOR_ADD, &args![predicted, far, beyond]);
        let scale = setting_value(e, SETTING_011CEF98);
        let cost = (weight as f64 * scale as f64) as f32;
        let radius = quiet(e.get(this, CombatThreat::fRadius));
        fn_009a4e30(e, result, Ptr::new(predicted), Ptr::new(far), radius, cost)
    })
}

// Translated from 009a4e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x24-byte avoid-node record (`PathingAvoidNode` layout
/// in the Xbox PDB) from two points, a radius and a cost: `Point1`, `Point2`,
/// `fRadius`, `fCost`, and type 1. The map has no name for it.
pub fn fn_009a4e30(
    e: &mut Engine,
    this: Ptr,
    point1: Ptr,
    point2: Ptr,
    radius: f32,
    cost: f32,
) -> Ptr {
    copy_vector(e, point1.addr(), this.addr());
    copy_vector(e, point2.addr(), this.addr().wrapping_add(0xc));
    e.mem.set_f32(this.addr().wrapping_add(0x18), quiet(radius));
    e.mem.set_f32(this.addr().wrapping_add(0x1c), quiet(cost));
    e.mem.set_u32(this.addr().wrapping_add(0x20), 1);
    this
}

// Translated from 009a4e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a threat to a save buffer: radius (4 bytes), the two time stamps,
/// damage (4 bytes). Called by `CombatThreatMap::SaveGame`; the map has no
/// name for it.
pub fn fn_009a4e90(e: &mut Engine, this: Ptr<CombatThreat>, buffer: Ptr) {
    // `00865e50`: (buffer, data, size, 0) writes `size` bytes.
    e.call(0x0086_5e50, &args![buffer, this.byte_add(4), 4u32, 0u32]);
    // `CombatTimeStamp::SaveGame` (engine map).
    e.call(0x009a_5f90, &args![this.byte_add(8), buffer]);
    e.call(0x009a_5f90, &args![this.byte_add(0xc), buffer]);
    e.call(0x0086_5e50, &args![buffer, this.byte_add(0x10), 4u32, 0u32]);
}

// Translated from 009a4ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a threat from a save buffer: the inverse of `009a4e90`.
pub fn fn_009a4ef0(e: &mut Engine, this: Ptr<CombatThreat>, buffer: Ptr) {
    // `00864980`: (buffer, data, size) reads `size` bytes.
    e.call(0x0086_4980, &args![buffer, this.byte_add(4), 4u32]);
    // `CombatTimeStamp::LoadGame` (engine map).
    e.call(0x009a_5ff0, &args![this.byte_add(8), buffer]);
    e.call(0x009a_5ff0, &args![this.byte_add(0xc), buffer]);
    e.call(0x0086_4980, &args![buffer, this.byte_add(0x10), 4u32]);
}

// Translated from 009a4f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resolves the form ID loaded into `pThreat` (+0x00): looks the form up
/// (`004839c0`), casts it with `__RTDynamicCast` between two type descriptors
/// (`01183028` to `011841cc`), stores the result (0 on failure) and, when it
/// is not null, calls `00564db0` with 1. The word argument is not read.
pub fn fn_009a4f40(e: &mut Engine, this: Ptr<CombatThreat>, _unused_1: u32) {
    let form_id = e.get(this, CombatThreat::pThreat);
    let resolved = if !form_id.is_null() {
        let form = e.call(0x0048_39c0, &args![form_id]).u32();
        e.call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_41ccu32, 0u32],
        )
        .u32()
    } else {
        0
    };
    e.set(this, CombatThreat::pThreat, Ptr::new(resolved));
    let stored = e.get(this, CombatThreat::pThreat);
    if !stored.is_null() {
        e.call(SET_TARGETED, &args![stored, 1u32]);
    }
}

// Translated from 009a4fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreatMap` constructor: the `NiTMap` constructor with hash size
/// 0x25, the `CombatThreatMap` vtable, and the lock constructor.
// Translation incomplete: 009a4fb0 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a4fb0(e: &mut Engine, this: Ptr<CombatThreatMap>) -> Ptr<CombatThreatMap> {
    fn_009a5d20(e, this.cast(), 0x25);
    e.mem.set_u32(this.addr(), VTABLE_COMBAT_THREAT_MAP);
    e.call(LOCK_INIT, &args![this.byte_add(0x10)]);
    this
}

// Translated from 009a5020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreatMap::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, `operator delete`.
pub fn combat_threat_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CombatThreatMap>,
    flags: u32,
) -> Ptr<CombatThreatMap> {
    fn_009a5050(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a5050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreatMap` destructor: sets the vtable, frees every threat
/// (`009a5270`), then runs the `NiTMap` destructor body (`009a5df0`).
// Translation incomplete: 009a5050 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a5050(e: &mut Engine, this: Ptr<CombatThreatMap>) {
    e.mem.set_u32(this.addr(), VTABLE_COMBAT_THREAT_MAP);
    fn_009a5270(e, this);
    fn_009a5df0(e, this.cast());
}

// Translated from 009a50b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the write lock, frees and removes every threat whose end time is
/// before the current time (`009a5170`). Does nothing for an empty map.
pub fn fn_009a50b0(e: &mut Engine, this: Ptr<CombatThreatMap>) {
    let first = first_item(e, this.cast());
    if first == 0 {
        return;
    }
    e.call(LOCK_FOR_WRITE, &args![this.byte_add(0x10)]);
    let now = current_time(e);
    e.with_stack(12, |e, frame| {
        let (position, key, value) = (
            frame.addr(),
            frame.addr().wrapping_add(4),
            frame.addr().wrapping_add(8),
        );
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT_ITEM, &args![this, position, key, value]);
            let threat = e.mem.u32(value);
            if threat != 0 && fn_009a5170(e, Ptr::new(threat), now) {
                fn_009a51b0(e, Ptr::new(threat), 1);
                let key = e.mem.u32(key);
                remove_at(e, this.cast(), key);
            }
        }
    });
    e.call(UNLOCK_AFTER_WRITE, &args![this.byte_add(0x10)]);
}

// Translated from 009a5170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `time` is after the threat's end time (+0x0C): false for equal
/// and for NaN.
pub fn fn_009a5170(e: &mut Engine, this: Ptr<CombatThreat>, time: f32) -> bool {
    // `006a7f50` returns the float at `*this`.
    let end = e.call(0x006a_7f50, &args![this.byte_add(0xc)]).f64();
    (time as f64) > end
}

// Translated from 009a51b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreat` scalar deleting destructor: the destructor body and, when
/// bit 0 of `flags` is set, `operator delete`.
pub fn fn_009a51b0(e: &mut Engine, this: Ptr<CombatThreat>, flags: u32) -> Ptr<CombatThreat> {
    fn_009a45e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a51e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the write lock, looks the reference's key up; if there is a
/// threat, frees it and removes the key from the map.
pub fn fn_009a51e0(e: &mut Engine, this: Ptr<CombatThreatMap>, reference: Ptr) {
    e.call(LOCK_FOR_WRITE, &args![this.byte_add(0x10)]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let key = e.call(REFERENCE_KEY, &args![reference]).u32();
        if e.call(MAP_LOOKUP, &args![this, key, slot]).bool() {
            let threat = e.mem.u32(slot.addr());
            if threat != 0 {
                fn_009a51b0(e, Ptr::new(threat), 1);
            }
            let key = e.call(REFERENCE_KEY, &args![reference]).u32();
            remove_at(e, this.cast(), key);
        }
    });
    e.call(UNLOCK_AFTER_WRITE, &args![this.byte_add(0x10)]);
}

// Translated from 009a5270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every threat of the map (without removing the keys one by one),
/// then clears the map (`00438af0`).
pub fn fn_009a5270(e: &mut Engine, this: Ptr<CombatThreatMap>) {
    let first = first_item(e, this.cast());
    e.with_stack(12, |e, frame| {
        let (position, key, value) = (
            frame.addr(),
            frame.addr().wrapping_add(4),
            frame.addr().wrapping_add(8),
        );
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT_ITEM, &args![this, position, key, value]);
            let threat = e.mem.u32(value);
            if threat != 0 {
                fn_009a51b0(e, Ptr::new(threat), 1);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
}

// Translated from 009a52f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a threat for `reference` from its base form's numbers, when the
/// reference passes the filters: flag 1 clear (`00444d00`), a base form
/// (`004181e0`) with a sub-object (`004fd3c0`) and the capability test
/// `004fd360`, a value (`006a78f0`) at least the `011cf3f0` setting, and a
/// remapped radius (`00477900`, `006479a0`) at least the `011cea74` setting,
/// and a positive end offset. The end offset is `FLT_MAX`, or, when
/// `00975300` holds for the base form, setting `011cf764` if `009a5480`
/// equals `FLT_MAX`, else `009a5480 + 1` (with a start offset of the
/// excess over setting `011ceabc` when that is smaller).
pub fn fn_009a52f0(e: &mut Engine, this: Ptr<CombatThreatMap>, reference: Ptr) {
    if e.call(0x0044_4d00, &args![reference, 1u32]).u8() != 0 {
        return;
    }
    let base = e.call(0x0041_81e0, &args![reference]).u32();
    let sub_object = e.call(0x004f_d3c0, &args![base]).u32();
    if e.call(0x004f_d360, &args![base]).u8() == 0 || sub_object == 0 {
        return;
    }
    let radius = e.call(0x0047_7900, &args![sub_object]).f32();
    let value = e.call(0x006a_78f0, &args![sub_object]).f32();
    let floor = setting_value(e, SETTING_011CF3F0);
    let value_reaches_floor = (floor as f64) <= (value as f64);
    if !value_reaches_floor {
        return;
    }
    let floor = quiet(setting_value(e, SETTING_011CF3F0));
    let radius = e.call(FLOAT_REMAP, &args![radius, value, floor]).f32();
    let minimum = setting_value(e, SETTING_011CEA74);
    let radius_reaches_minimum = (minimum as f64) <= (radius as f64);
    if !radius_reaches_minimum {
        return;
    }
    let mut start_offset = 0.0f32;
    let mut end_offset = e.global::<f32>(FLOAT_MAX);
    if e.call(0x0097_5300, &args![base]).u8() != 0 {
        if fn_009a5480(e, reference) as f64 == e.global::<f64>(FLOAT_MAX_DOUBLE) {
            end_offset = setting_value(e, SETTING_011CF764);
        } else {
            end_offset = (fn_009a5480(e, reference) as f64 + e.global::<f64>(ONE_DOUBLE)) as f32;
            let limit = setting_value(e, SETTING_011CEABC);
            if (limit as f64) < (end_offset as f64) {
                let limit = setting_value(e, SETTING_011CEABC);
                start_offset = (end_offset as f64 - limit as f64) as f32;
            }
        }
    }
    let end_is_positive = (end_offset as f64) > e.global::<f64>(ZERO_DOUBLE);
    if !end_is_positive {
        return;
    }
    combat_threat_map_add_threat(e, this, reference, radius, start_offset, end_offset, value);
}

// Translated from 009a5480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `this + 0xE4`.
pub fn fn_009a5480(e: &mut Engine, this: Ptr) -> f32 {
    quiet(e.mem.f32(this.addr().wrapping_add(0xe4)))
}

// Translated from 009a54a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a threat for `reference` from `other`'s numbers: needs the value
/// (`006a78f0`) to be at least the `011cf3f0` setting; the radius is
/// remapped (`00477900`, `006479a0`); the end offset is `extra`, the start
/// offset 0.
pub fn fn_009a54a0(
    e: &mut Engine,
    this: Ptr<CombatThreatMap>,
    reference: Ptr,
    other: Ptr,
    extra: f32,
) {
    let value = e.call(0x006a_78f0, &args![other]).f32();
    let floor = setting_value(e, SETTING_011CF3F0);
    let value_reaches_floor = (floor as f64) <= (value as f64);
    if !value_reaches_floor {
        return;
    }
    let radius = e.call(0x0047_7900, &args![other]).f32();
    let floor = quiet(setting_value(e, SETTING_011CF3F0));
    let radius = e.call(FLOAT_REMAP, &args![radius, value, floor]).f32();
    combat_threat_map_add_threat(e, this, reference, radius, 0.0, extra, value);
}

// Translated from 009a5530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreatMap::AddThreat` (Xbox PDB): under the write lock, marks the
/// reference (`00564db0` with 1) and either re-sets up the existing threat of
/// its key (`009a4690`, flag 1) or allocates a new `CombatThreat` (0x30
/// bytes), constructs it and adds it to the map; when the `bool` setting
/// `011f1ba0` is set, calls `004534f0` on the threat with 0.
// Translation incomplete: 009a5530 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn combat_threat_map_add_threat(
    e: &mut Engine,
    this: Ptr<CombatThreatMap>,
    reference: Ptr,
    radius: f32,
    start_offset: f32,
    end_offset: f32,
    damage: f32,
) {
    e.call(LOCK_FOR_WRITE, &args![this.byte_add(0x10)]);
    e.call(SET_TARGETED, &args![reference, 1u32]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let key = e.call(REFERENCE_KEY, &args![reference]).u32();
        if e.call(MAP_LOOKUP, &args![this, key, slot]).bool() {
            let threat = e.mem.u32(slot.addr());
            e.call(
                THREAT_SET_UP,
                &args![
                    threat,
                    reference,
                    quiet(radius),
                    quiet(start_offset),
                    quiet(end_offset),
                    quiet(damage),
                    1u32
                ],
            );
        } else {
            let memory = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
            let threat = if memory != 0 {
                fn_009a44e0(
                    e,
                    Ptr::new(memory),
                    reference,
                    radius,
                    start_offset,
                    end_offset,
                    damage,
                )
                .addr()
            } else {
                0
            };
            e.mem.set_u32(slot.addr(), threat);
            let key = e.call(REFERENCE_KEY, &args![reference]).u32();
            set_at(e, this.cast(), key, threat);
        }
        let flag = e
            .call(SETTING_BOOL_VALUE, &args![SETTING_BOOL_011F1BA0])
            .u32();
        if e.mem.u8(flag) != 0 {
            let threat = e.mem.u32(slot.addr());
            e.call(THREAT_SET_DEBUG, &args![threat, 0u32]);
        }
    });
    e.call(UNLOCK_AFTER_WRITE, &args![this.byte_add(0x10)]);
}

// Translated from 009a5670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sums the scores (`009a4710`) of the threats whose start time is after
/// `compare`, under the lock, for the point (`x`, `y`, `z`), stopping at a
/// total above 1 (clamped to 1). A negative `time` is used as is, otherwise it
/// is added to the current time. Returns the total.
#[allow(clippy::too_many_arguments)]
pub fn fn_009a5670(
    e: &mut Engine,
    this: Ptr<CombatThreatMap>,
    x: f32,
    y: f32,
    z: f32,
    other: Ptr,
    time: f32,
    extra: f32,
    compare: f32,
    fallback: f32,
) -> f32 {
    e.call(LOCK_ENTER, &args![this.byte_add(0x10)]);
    let mut total = 0.0f32;
    let deadline = if (time as f64) < e.global::<f64>(ZERO_DOUBLE) {
        quiet(time)
    } else {
        (current_time(e) as f64 + time as f64) as f32
    };
    let first = first_item(e, this.cast());
    e.with_stack(0x30, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(offset);
        // The point and the compare value are the function's own stack
        // parameters in the exe; here they are copied to the frame.
        let (point, compare_slot) = (at(0), at(0x10));
        e.mem.set_f32(point, x);
        e.mem.set_f32(point.wrapping_add(4), y);
        e.mem.set_f32(point.wrapping_add(8), z);
        e.mem.set_f32(compare_slot, compare);
        let (position, key, value, start) = (at(0x14), at(0x18), at(0x1c), at(0x20));
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT_ITEM, &args![this, position, key, value]);
            let threat: Ptr<CombatThreat> = Ptr::new(e.mem.u32(value));
            let start_time = fn_009a5780(e, threat, Ptr::new(start));
            let start_time = e.call(0x006a_7f50, &args![start_time]).f64();
            let compare_time = e.call(0x006a_7f50, &args![compare_slot]).f64();
            let starts_later = compare_time < start_time;
            if !starts_later {
                continue;
            }
            let score = fn_009a4710(e, threat, Ptr::new(point), other, deadline, extra, fallback);
            total = (total as f64 + score as f64) as f32;
            if (total as f64) > e.global::<f64>(ONE_DOUBLE) {
                total = 1.0;
                break;
            }
        }
    });
    e.call(LOCK_LEAVE, &args![this.byte_add(0x10)]);
    total
}

// Translated from 009a5780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the threat's start-time stamp word (+0x08) to `result` and
/// returns `result`.
pub fn fn_009a5780(e: &mut Engine, this: Ptr<CombatThreat>, result: Ptr) -> Ptr {
    let word = e.get(this, CombatThreat::fStartTime);
    e.mem.set_u32(result.addr(), word);
    result
}

// Translated from 009a57a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects the avoid-node records (`009a4c70`) of the threats whose score
/// (`009a4710` with extra reach 1024 and no fallback) exceeds the `011cfbcc`
/// setting into a new `BSSimpleArray`, under the lock; when none does, the
/// array holds the record of the best-scoring threat, if any. Returns the
/// array, or 0 (also for a `other` whose `009989d0` is set).
// Translation incomplete: 009a57a0 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a57a0(
    e: &mut Engine,
    this: Ptr<CombatThreatMap>,
    point: Ptr,
    other: Ptr,
    time: f32,
) -> Ptr {
    let process = e.call(0x008a_7f80, &args![other]).u32();
    if e.call(0x0099_89d0, &args![process]).bool() {
        return Ptr::NULL;
    }
    let deadline = if (time as f64) < e.global::<f64>(ZERO_DOUBLE) {
        quiet(time)
    } else {
        (current_time(e) as f64 + time as f64) as f32
    };
    let mut array = 0u32;
    let mut best = 0u32;
    let mut best_score = 0.0f32;
    e.call(LOCK_ENTER, &args![this.byte_add(0x10)]);
    let first = first_item(e, this.cast());
    e.with_stack(0x40, |e, frame| {
        let at = |offset: u32| frame.addr().wrapping_add(offset);
        let (position, key, value, node) = (at(0), at(4), at(8), at(0x10));
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT_ITEM, &args![this, position, key, value]);
            let threat: Ptr<CombatThreat> = Ptr::new(e.mem.u32(value));
            let limit = quiet(e.global::<f32>(FLOAT_1024));
            let score = fn_009a4710(e, threat, point, other, deadline, limit, 0.0);
            if (best_score as f64) < (score as f64) {
                best_score = score;
                best = threat.addr();
            }
            let minimum = setting_value(e, SETTING_011CFBCC);
            if (minimum as f64) < (score as f64) {
                if array == 0 {
                    array = new_avoid_node_array(e);
                }
                let record = fn_009a4c70(e, threat, Ptr::new(node), other);
                e.call(AVOID_NODE_ARRAY_ADD, &args![array, record]);
            }
        }
    });
    e.call(LOCK_LEAVE, &args![this.byte_add(0x10)]);
    if array == 0 && best != 0 {
        array = new_avoid_node_array(e);
        e.with_stack(0x24, |e, node| {
            let record = fn_009a4c70(e, Ptr::new(best), node, other);
            e.call(AVOID_NODE_ARRAY_ADD, &args![array, record]);
        });
    }
    Ptr::new(array)
}

// Translated from 009a59e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreatMap::SaveGame` (Xbox PDB): a variable-sized value holding the
/// count and, for every non-null threat, its key as a form ID followed by
/// the threat's data (`009a4e90`).
pub fn combat_threat_map_save_game(e: &mut Engine, this: Ptr<CombatThreatMap>, buffer: Ptr) {
    let mut count = 0u32;
    // `BGSSaveGameBuffer::StartVariableSizedValue` (engine map).
    let start = e.call(0x0086_5f20, &args![buffer]).u32();
    let first = first_item(e, this.cast());
    e.with_stack(12, |e, frame| {
        let (position, key, value) = (
            frame.addr(),
            frame.addr().wrapping_add(4),
            frame.addr().wrapping_add(8),
        );
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT_ITEM, &args![this, position, key, value]);
            let threat = e.mem.u32(value);
            if threat != 0 {
                let key = e.mem.u32(key);
                // `BGSSaveGameBuffer::SaveFormID` (engine map).
                e.call(0x0086_5db0, &args![buffer, key, 0u32]);
                fn_009a4e90(e, Ptr::new(threat), buffer);
                count = count.wrapping_add(1);
            }
        }
    });
    // `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map).
    e.call(0x0086_5ff0, &args![buffer, count, start]);
}

// Translated from 009a5a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the threats of a save buffer: a count (`00864a60`), then per threat a
/// new `CombatThreat` (0x30 bytes, default-constructed by `009a5b90`), its form
/// ID (`008648e0`), its data (`009a4ef0`); a threat with form ID 0 is
/// freed, any other is set in the map under that ID.
// Translation incomplete: 009a5a70 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a5a70(e: &mut Engine, this: Ptr<CombatThreatMap>, buffer: Ptr) {
    let count = e.call(0x0086_4a60, &args![buffer]).u32();
    let mut index = 0u32;
    while index < count {
        let memory = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
        let threat = if memory != 0 {
            fn_009a5b90(e, Ptr::new(memory)).addr()
        } else {
            0
        };
        e.with_stack(4, |e, form_id| {
            e.call(0x0086_48e0, &args![buffer, form_id]);
            let id = e.mem.u32(form_id.addr());
            // `00537e90` stores the form ID in the threat's first word.
            e.call(0x0053_7e90, &args![threat, id]);
            fn_009a4ef0(e, Ptr::new(threat), buffer);
            if id != 0 {
                set_at(e, this.cast(), id, threat);
            } else if threat != 0 {
                fn_009a51b0(e, Ptr::new(threat), 1);
            }
        });
        index += 1;
    }
}

// Translated from 009a5b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreat` default constructor: zero reference, radius, damage and
/// both time stamps; constructs the debug geometry pointer, the cached
/// velocity and its stamp. The cached 3D object and rigid body (+0x18,
/// +0x1C) are not written.
// Translation incomplete: 009a5b90 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a5b90(e: &mut Engine, this: Ptr<CombatThreat>) -> Ptr<CombatThreat> {
    e.set(this, CombatThreat::pThreat, Ptr::NULL);
    e.set(this, CombatThreat::fRadius, 0.0);
    e.call(TIME_STAMP_SET, &args![this.byte_add(8), 0.0f32]);
    e.call(TIME_STAMP_SET, &args![this.byte_add(0xc), 0.0f32]);
    e.set(this, CombatThreat::fDamage, 0.0);
    e.call(SMART_POINTER_INIT, &args![this.byte_add(0x14), 0u32]);
    e.call(VECTOR_INIT, &args![this.byte_add(0x20)]);
    e.call(TIME_STAMP_INIT, &args![this.byte_add(0x2c)]);
    this
}

// Translated from 009a5c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatThreatMap::InitLoadGame` (Xbox PDB): resolves every loaded
/// threat's form ID (`009a4f40`) and frees and removes the threats whose
/// reference did not resolve.
pub fn combat_threat_map_init_load_game(e: &mut Engine, this: Ptr<CombatThreatMap>, argument: u32) {
    let first = first_item(e, this.cast());
    e.with_stack(12, |e, frame| {
        let (position, key, value) = (
            frame.addr(),
            frame.addr().wrapping_add(4),
            frame.addr().wrapping_add(8),
        );
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT_ITEM, &args![this, position, key, value]);
            let threat = e.mem.u32(value);
            if threat != 0 {
                fn_009a4f40(e, Ptr::new(threat), argument);
                if e.call(POINTER_VALUE, &args![threat]).u32() == 0 {
                    fn_009a51b0(e, Ptr::new(threat), 1);
                    let key = e.mem.u32(key);
                    remove_at(e, this.cast(), key);
                }
            }
        }
    });
}

// Translated from 009a5cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004534f0` on every non-null threat with `flag`.
pub fn fn_009a5cd0(e: &mut Engine, this: Ptr<CombatThreatMap>, flag: u32) {
    let first = first_item(e, this.cast());
    e.with_stack(12, |e, frame| {
        let (position, key, value) = (
            frame.addr(),
            frame.addr().wrapping_add(4),
            frame.addr().wrapping_add(8),
        );
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(value, 0);
            e.call(MAP_NEXT_ITEM, &args![this, position, key, value]);
            let threat = e.mem.u32(value);
            if threat != 0 {
                e.call(THREAT_SET_DEBUG, &args![threat, flag]);
            }
        }
    });
}

// Translated from 009a5d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, CombatThreat *>` constructor: the `NiTMapBase`
/// constructor, then the `NiTMap` vtable.
pub fn fn_009a5d20(e: &mut Engine, this: Ptr<NiTMap>, hash_size: u32) -> Ptr<NiTMap> {
    fn_009a5d80(e, this, hash_size);
    e.mem.set_u32(this.addr(), VTABLE_NI_TMAP);
    this
}

// Translated from 009a5d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, CombatThreat *>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor body and, when bit 0 of `flags` is set,
/// `operator delete`.
pub fn ni_tmap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_009a5df0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a5d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor: the base vtable, the hash size, count 0, and a
/// zeroed table of `hash_size` pointers.
pub fn fn_009a5d80(e: &mut Engine, this: Ptr<NiTMap>, hash_size: u32) -> Ptr<NiTMap> {
    e.mem.set_u32(this.addr(), VTABLE_NI_TMAP_BASE);
    e.set(this, NiTMap::m_uiHashSize, hash_size);
    e.set(this, NiTMap::m_uiCount, 0);
    let bytes = e.get(this, NiTMap::m_uiHashSize).wrapping_shl(2);
    let table = e.call(ALLOCATE_TABLE, &args![bytes]).u32();
    e.set(this, NiTMap::m_ppkHashTable, table);
    let bytes = e.get(this, NiTMap::m_uiHashSize).wrapping_shl(2);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 009a5df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap` destructor body: the `NiTMap` vtable, clear the map
/// (`00438af0`), then the `NiTMapBase` destructor body (`009a5e50`).
// Translation incomplete: 009a5df0 - the FS:[0] C++ exception unwinding frame is not translated.
pub fn fn_009a5df0(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), VTABLE_NI_TMAP);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_009a5e50(e, this);
}

// Translated from 009a5e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` destructor body: the base vtable, clear the map, free the
/// table.
pub fn fn_009a5e50(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), VTABLE_NI_TMAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(FREE_TABLE, &args![table]);
}

// Translated from 009a5e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::_scalar_deleting_destructor_` (Xbox PDB): the base
/// destructor body and, when bit 0 of `flags` is set, `operator delete`.
pub fn ni_tmap_base_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_009a5e50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00405430, remove_at(Ptr, u32) -> bool),
        entry!(0x00844700, set_at(Ptr, u32, u32)),
        entry!(
            0x009a44e0,
            fn_009a44e0(Ptr<CombatThreat>, Ptr, f32, f32, f32, f32) -> Ptr<CombatThreat>
        ),
        entry!(0x009a45e0, fn_009a45e0(Ptr<CombatThreat>)),
        entry!(
            0x009a4710,
            fn_009a4710(Ptr<CombatThreat>, Ptr, Ptr, f32, f32, f32) -> f32
        ),
        entry!(
            0x009a4a10,
            combat_threat_get_threat_linear_velocity(Ptr<CombatThreat>, Ptr) -> Ptr
        ),
        entry!(0x009a4c70, fn_009a4c70(Ptr<CombatThreat>, Ptr, Ptr) -> Ptr),
        entry!(0x009a4e30, fn_009a4e30(Ptr, Ptr, Ptr, f32, f32) -> Ptr),
        entry!(0x009a4e90, fn_009a4e90(Ptr<CombatThreat>, Ptr)),
        entry!(0x009a4ef0, fn_009a4ef0(Ptr<CombatThreat>, Ptr)),
        entry!(0x009a4f40, fn_009a4f40(Ptr<CombatThreat>, u32)),
        entry!(
            0x009a4fb0,
            fn_009a4fb0(Ptr<CombatThreatMap>) -> Ptr<CombatThreatMap>
        ),
        entry!(
            0x009a5020,
            combat_threat_map_scalar_deleting_destructor(
                Ptr<CombatThreatMap>,
                u32,
            ) -> Ptr<CombatThreatMap>
        ),
        entry!(0x009a5050, fn_009a5050(Ptr<CombatThreatMap>)),
        entry!(0x009a50b0, fn_009a50b0(Ptr<CombatThreatMap>)),
        entry!(0x009a5170, fn_009a5170(Ptr<CombatThreat>, f32) -> bool),
        entry!(
            0x009a51b0,
            fn_009a51b0(Ptr<CombatThreat>, u32) -> Ptr<CombatThreat>
        ),
        entry!(0x009a51e0, fn_009a51e0(Ptr<CombatThreatMap>, Ptr)),
        entry!(0x009a5270, fn_009a5270(Ptr<CombatThreatMap>)),
        entry!(0x009a52f0, fn_009a52f0(Ptr<CombatThreatMap>, Ptr)),
        entry!(0x009a5480, fn_009a5480(Ptr) -> f32),
        entry!(0x009a54a0, fn_009a54a0(Ptr<CombatThreatMap>, Ptr, Ptr, f32)),
        entry!(
            0x009a5530,
            combat_threat_map_add_threat(Ptr<CombatThreatMap>, Ptr, f32, f32, f32, f32)
        ),
        entry!(
            0x009a5670,
            fn_009a5670(Ptr<CombatThreatMap>, f32, f32, f32, Ptr, f32, f32, f32, f32) -> f32
        ),
        entry!(0x009a5780, fn_009a5780(Ptr<CombatThreat>, Ptr) -> Ptr),
        entry!(
            0x009a57a0,
            fn_009a57a0(Ptr<CombatThreatMap>, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(
            0x009a59e0,
            combat_threat_map_save_game(Ptr<CombatThreatMap>, Ptr)
        ),
        entry!(0x009a5a70, fn_009a5a70(Ptr<CombatThreatMap>, Ptr)),
        entry!(
            0x009a5b90,
            fn_009a5b90(Ptr<CombatThreat>) -> Ptr<CombatThreat>
        ),
        entry!(
            0x009a5c40,
            combat_threat_map_init_load_game(Ptr<CombatThreatMap>, u32)
        ),
        entry!(0x009a5cd0, fn_009a5cd0(Ptr<CombatThreatMap>, u32)),
        entry!(0x009a5d20, fn_009a5d20(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x009a5d50,
            ni_tmap_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(0x009a5d80, fn_009a5d80(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x009a5df0, fn_009a5df0(Ptr<NiTMap>)),
        entry!(0x009a5e50, fn_009a5e50(Ptr<NiTMap>)),
        entry!(
            0x009a5e80,
            ni_tmap_base_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let map = e.mem.alloc(16);
        let table = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x1c);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u32(vtable + 4, 0xf0100000);
        e.mem.set_u32(vtable + 8, 0xf0100004);
        e.mem.set_u32(vtable + 0xc, 0xf0100008);
        e.mem.set_u32(vtable + 0x14, 0xf010000c);
        e.register_double(0xf0100000, move |_, a| {
            assert_eq!(a, &[map, 22]);
            1u32.into_ret()
        });
        (e, map, table, vtable)
    }

    #[test]
    fn lookup_reloads_buckets_links_and_value_after_callbacks() {
        let (mut e, map, old_table, _) = fixture();
        let table = e.mem.alloc(8);
        let first = e.mem.alloc(12);
        let last = e.mem.alloc(12);
        let out = e.mem.alloc(4);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(first + 4, 11);
        e.mem.set_u32(last + 4, 22);
        e.register_double(0xf0100000, move |e, a| {
            assert_eq!(a, &[map, 22]);
            e.mem.set_u32(map + 8, table);
            1u32.into_ret()
        });
        e.register_double(0xf0100004, move |e, a| {
            if a[2] == 11 {
                assert_eq!(a, &[map, 22, 11]);
                e.mem.set_u32(first, last);
                0x100u32.into_ret()
            } else {
                assert_eq!(a, &[map, 22, 22]);
                e.mem.set_u32(last + 8, 0x12345678);
                0x101u32.into_ret()
            }
        });
        assert_eq!(e.call(0x00853130, &args![map, 22u32, out]).u8(), 1);
        assert_eq!(e.mem.u32(out), 0x12345678);
        assert_eq!(e.mem.u32(old_table + 4), 0);
        e.mem.set_u32(table + 4, 0);
        e.mem.set_u32(out, 0xdeadbeef);
        assert_eq!(e.call(0x00853130, &args![map, 22u32, out]).u8(), 0);
        assert_eq!(e.mem.u32(out), 0xdeadbeef);
    }

    #[test]
    fn set_existing_follows_callback_changed_link_and_does_not_add() {
        let (mut e, map, table, _) = fixture();
        let first = e.mem.alloc(12);
        let last = e.mem.alloc(12);
        e.mem.set_u32(table + 4, first);
        e.mem.set_u32(first + 4, 11);
        e.mem.set_u32(last + 4, 22);
        e.mem.set_u32(map + 12, 2);
        e.register_double(0xf0100004, move |e, a| {
            assert_eq!(&a[..2], &[map, 22]);
            if a[2] == 11 {
                e.mem.set_u32(first, last);
                0x100u32.into_ret()
            } else {
                assert_eq!(a[2], 22);
                0x101u32.into_ret()
            }
        });
        e.call(0x00844700, &args![map, 22u32, 0xfedcba98u32]);
        assert_eq!(e.mem.u32(last + 8), 0xfedcba98);
        assert_eq!(e.mem.u32(map + 12), 2);
        assert_eq!(e.mem.u32(table + 4), first);
    }

    #[test]
    fn insertion_reloads_vtable_and_buckets_and_wraps_fresh_count() {
        let (mut e, map, old_table, _) = fixture();
        let table = e.mem.alloc(8);
        let new_vtable = e.mem.alloc(0x1c);
        let node = e.mem.alloc(12);
        let old_node = e.mem.alloc(12);
        e.mem.set_u32(new_vtable + 0xc, 0xf0100010);
        e.register_double(0xf010000c, move |e, a| {
            assert_eq!(a, &[map]);
            e.mem.set_u32(map, new_vtable);
            node.into_ret()
        });
        e.register_double(0xf0100010, move |e, a| {
            assert_eq!(a, &[map, node, 22, 77]);
            e.mem.set_u32(node + 4, a[2]);
            e.mem.set_u32(node + 8, a[3]);
            e.mem.set_u32(map + 8, table);
            e.mem.set_u32(table + 4, old_node);
            e.mem.set_u32(map + 12, u32::MAX);
            Ret::default()
        });
        e.call(0x00844700, &args![map, 22u32, 77u32]);
        assert_eq!(e.mem.u32(table + 4), node);
        assert_eq!(e.mem.u32(node), old_node);
        assert_eq!(e.mem.u32(node + 4), 22);
        assert_eq!(e.mem.u32(node + 8), 77);
        assert_eq!(e.mem.u32(map + 12), 0);
        assert_eq!(e.mem.u32(old_table + 4), 0);
    }

    // ---- Helpers for the tests of the other functions of this unit. ----

    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    const NOW: f32 = 100.0;
    /// Fake addresses of the vtable slots of the test map and references.
    const FAKE_HASH: u32 = 0xf010_0000;
    const FAKE_EQUAL: u32 = 0xf010_0004;
    const FAKE_SET_VALUE: u32 = 0xf010_0008;
    const FAKE_CLEAR: u32 = 0xf010_000c;
    const FAKE_NEW_ITEM: u32 = 0xf010_0010;
    const FAKE_DELETE_ITEM: u32 = 0xf010_0014;
    const FAKE_POSITION: u32 = 0xf010_0020;
    const FAKE_OBJECT: u32 = 0xf010_0024;
    const FAKE_FLAG: u32 = 0xf010_0028;
    const FAKE_DETACH: u32 = 0xf010_002c;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0102_3000,
            0x0102_6000,
            0x0108_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(FLOAT_MAX_DOUBLE, f32::MAX as f64);
        e.set_global(AGE_LIMIT, 0.33f64);
        e.set_global(HALF, 0.5f32);
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(FLOAT_1024, 1024.0f32);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn addresses(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    fn near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-5,
            "{actual} is not near {expected}"
        );
    }

    fn vector(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn put_vector(e: &mut Engine, at: u32, v: [f32; 3]) {
        e.mem.set_f32(at, v[0]);
        e.mem.set_f32(at + 4, v[1]);
        e.mem.set_f32(at + 8, v[2]);
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// The float setting objects: `00403e20` returns the address of the
    /// value for the setting passed in.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        let mut table = Vec::new();
        for (setting, value) in values {
            let cell = e.mem.alloc(4);
            e.mem.set_f32(cell, *value);
            table.push((*setting, cell));
        }
        e.register_double(SETTING_VALUE, move |_, a| {
            table
                .iter()
                .find(|(setting, _)| *setting == a[0])
                .map(|(_, cell)| *cell)
                .unwrap_or_else(|| panic!("unexpected setting {:08x}", a[0]))
                .into_ret()
        });
    }

    /// The combat clock and the time stamp helpers.
    fn clock(e: &mut Engine, now: f32) {
        e.register_double(CURRENT_TIME, move |_, _| now.into_ret());
        e.register(TIME_STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register_double(TIME_STAMP_AGE, move |e, a| {
            (now - e.mem.f32(a[0])).into_ret()
        });
    }

    /// Doubles of the `NiPoint3` arithmetic the exe calls.
    fn vector_math(e: &mut Engine) {
        e.register(VECTOR_UNITIZE, |e, a| {
            let v = vector(e, a[0]);
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if length > 0.0 {
                put_vector(e, a[0], [v[0] / length, v[1] / length, v[2] / length]);
            }
            length.into_ret()
        });
        e.register(VECTOR_SCALE, |e, a| {
            let v = vector(e, a[0]);
            let s = f32::from_bits(a[2]);
            put_vector(e, a[1], [v[0] * s, v[1] * s, v[2] * s]);
            a[1].into_ret()
        });
        e.register(VECTOR_ADD, |e, a| {
            let (l, r) = (vector(e, a[0]), vector(e, a[2]));
            put_vector(e, a[1], [l[0] + r[0], l[1] + r[1], l[2] + r[2]]);
            a[1].into_ret()
        });
        e.register(VECTOR_SUBTRACT, |e, a| {
            let (l, r) = (vector(e, a[0]), vector(e, a[2]));
            put_vector(e, a[1], [l[0] - r[0], l[1] - r[1], l[2] - r[2]]);
            a[1].into_ret()
        });
        e.register(VECTOR_LENGTH_SQUARED, |e, a| {
            let v = vector(e, a[0]);
            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).into_ret()
        });
        e.register(VECTOR_SCALE_CDECL, |e, a| {
            let v = vector(e, a[2]);
            let s = f32::from_bits(a[1]);
            put_vector(e, a[0], [v[0] * s, v[1] * s, v[2] * s]);
            a[0].into_ret()
        });
        e.register(VECTOR_DOT, |e, a| {
            let (l, r) = (vector(e, a[0]), vector(e, a[1]));
            (l[0] * r[0] + l[1] * r[1] + l[2] * r[2]).into_ret()
        });
        e.register(VECTOR_ADD_IN_PLACE, |e, a| {
            let (l, r) = (vector(e, a[0]), vector(e, a[1]));
            put_vector(e, a[0], [l[0] + r[0], l[1] + r[1], l[2] + r[2]]);
            a[0].into_ret()
        });
        e.register(FLOAT_MAXIMUM, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y >= x || x.is_nan() || y.is_nan() {
                y
            } else {
                x
            })
            .into_ret()
        });
        e.register(AVOID_NODE_FROM_POINT, |e, a| {
            let v = vector(e, a[1]);
            put_vector(e, a[0], v);
            e.mem.set_u32(a[0] + 0x18, a[2]);
            e.mem.set_u32(a[0] + 0x1c, a[3]);
            e.mem.set_u32(a[0] + 0x20, 0);
            a[0].into_ret()
        });
    }

    /// A one-bucket map whose vtable slots are doubles: the hash is 0, keys
    /// are equal when the words are, `SetValue` fills the item, and the
    /// release slots do nothing. Returns the map and its item addresses.
    fn map_with(e: &mut Engine, items: &[(u32, u32)]) -> (Ptr<CombatThreatMap>, Vec<u32>) {
        let map: Ptr<CombatThreatMap> = e.new_object();
        let table = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 4, 1);
        e.mem.set_u32(map.addr() + 8, table);
        e.mem.set_u32(map.addr() + 0xc, items.len() as u32);
        for (slot, target) in [
            (4, FAKE_HASH),
            (8, FAKE_EQUAL),
            (0xc, FAKE_SET_VALUE),
            (0x10, FAKE_CLEAR),
            (0x14, FAKE_NEW_ITEM),
            (0x18, FAKE_DELETE_ITEM),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        e.register(FAKE_HASH, |_, _| 0u32.into_ret());
        e.register(FAKE_EQUAL, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(FAKE_SET_VALUE, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(FAKE_CLEAR, |_, _| Ret::default());
        e.register(FAKE_NEW_ITEM, |e, _| e.mem.alloc(12).into_ret());
        e.register(FAKE_DELETE_ITEM, |_, _| Ret::default());
        let mut nodes = Vec::new();
        let mut next = 0;
        for (key, value) in items.iter().rev() {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 4, *key);
            e.mem.set_u32(node + 8, *value);
            next = node;
            nodes.push(node);
        }
        nodes.reverse();
        e.mem.set_u32(table, next);
        (map, nodes)
    }

    fn map_items(e: &Engine, map: Ptr<CombatThreatMap>) -> Vec<(u32, u32)> {
        let table = e.mem.u32(map.addr() + 8);
        let mut node = e.mem.u32(table);
        let mut items = Vec::new();
        while node != 0 {
            items.push((e.mem.u32(node + 4), e.mem.u32(node + 8)));
            node = e.mem.u32(node);
        }
        items
    }

    /// Doubles of the map walk: `004b9ba0` returns an item while any is
    /// left; `006b7f20` hands out the next (key, value) and clears the
    /// position after the last one.
    fn walk(e: &mut Engine, items: &[(u32, u32)]) {
        let queue = Rc::new(RefCell::new(VecDeque::from(items.to_vec())));
        let first = queue.clone();
        e.register_double(MAP_FIRST_ITEM, move |_, _| {
            u32::from(!first.borrow().is_empty()).into_ret()
        });
        e.register_double(MAP_NEXT_ITEM, move |e, a| {
            let (key, value) = queue.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            if queue.borrow().is_empty() {
                e.mem.set_u32(a[1], 0);
            }
            Ret::default()
        });
    }

    /// The vtable slots 0x1d0 (3D object), 0x1f4 (position) and 0x224
    /// (flag) of a reference read words of the reference itself.
    fn reference_doubles(e: &mut Engine) {
        e.register(FAKE_OBJECT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FAKE_POSITION, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FAKE_FLAG, |e, a| e.mem.u32(a[0] + 12).into_ret());
    }

    fn new_reference(e: &mut Engine, position: [f32; 3], object: u32, flag: u32) -> Ptr {
        let reference = e.mem.alloc(16);
        let table = e.mem.alloc(0x240);
        let at = e.mem.alloc(12);
        put_vector(e, at, position);
        e.mem.set_u32(reference, table);
        e.mem.set_u32(reference + 4, at);
        e.mem.set_u32(reference + 8, object);
        e.mem.set_u32(reference + 12, flag);
        e.mem.set_u32(table + 0x1d0, FAKE_OBJECT);
        e.mem.set_u32(table + 0x1f4, FAKE_POSITION);
        e.mem.set_u32(table + 0x224, FAKE_FLAG);
        Ptr::new(reference)
    }

    /// A threat of radius 2 at `(10, 0, 0)` with a fresh cached velocity
    /// `(3, 0, 0)` and the given damage.
    fn scoring_threat(e: &mut Engine, damage: f32) -> Ptr<CombatThreat> {
        let reference = new_reference(e, [10.0, 0.0, 0.0], 0x900, 0);
        let threat: Ptr<CombatThreat> = e.new_object();
        e.set(threat, CombatThreat::pThreat, reference);
        e.set(threat, CombatThreat::fRadius, 2.0);
        e.set(threat, CombatThreat::fDamage, damage);
        e.set(
            threat,
            CombatThreat::fCachedLinearVelocityTimeStamp,
            NOW.to_bits(),
        );
        put_vector(e, threat.addr() + 0x20, [3.0, 0.0, 0.0]);
        threat
    }

    /// The doubles the scoring functions share: the weight is the threat's
    /// damage, every time is inside the window.
    fn scoring_world() -> Engine {
        let mut e = engine();
        vector_math(&mut e);
        clock(&mut e, NOW);
        reference_doubles(&mut e);
        e.register(THREAT_WEIGHT, |e, a| e.mem.f32(a[0] + 0x10).into_ret());
        e.register(THREAT_IN_WINDOW, |_, _| 1u32.into_ret());
        settings(
            &mut e,
            &[
                (SETTING_011CF48C, 10.0),
                (SETTING_011CE990, 4.0),
                (SETTING_011CEF98, 4.0),
            ],
        );
        e
    }

    fn point(e: &mut Engine, v: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(12);
        put_vector(e, at, v);
        Ptr::new(at)
    }

    // ---- 00405430 ----

    #[test]
    fn remove_at_unlinks_head_and_later_items_and_reports_missing_keys() {
        let mut e = engine();
        let (map, nodes) = map_with(&mut e, &[(1, 10), (2, 20), (3, 30)]);
        e.call_log = Some(vec![]);
        assert!(remove_at(&mut e, map.cast(), 1));
        assert_eq!(map_items(&e, map), [(2, 20), (3, 30)]);
        assert!(remove_at(&mut e, map.cast(), 3));
        assert_eq!(map_items(&e, map), [(2, 20)]);
        assert!(!remove_at(&mut e, map.cast(), 9));
        assert_eq!(map_items(&e, map), [(2, 20)]);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 1);
        let m = map.addr();
        assert_eq!(
            calls_to(&e, FAKE_CLEAR),
            [vec![m, nodes[0]], vec![m, nodes[2]]]
        );
        assert_eq!(
            calls_to(&e, FAKE_DELETE_ITEM),
            [vec![m, nodes[0]], vec![m, nodes[2]]]
        );
        // The hash slot gets the key first; the comparison gets (key, item key).
        assert_eq!(calls_to(&e, FAKE_HASH)[0], [m, 1]);
        assert_eq!(calls_to(&e, FAKE_EQUAL)[0], [m, 1, 1]);
    }

    #[test]
    fn remove_at_on_an_empty_bucket_does_nothing() {
        let mut e = engine();
        let (map, _) = map_with(&mut e, &[]);
        e.call_log = Some(vec![]);
        assert!(!remove_at(&mut e, map.cast(), 5));
        assert!(calls_to(&e, FAKE_EQUAL).is_empty());
        assert!(calls_to(&e, FAKE_DELETE_ITEM).is_empty());
    }

    // ---- 009a44e0, 009a5b90: constructors ----

    #[test]
    fn threat_constructor_sets_up_then_resets_the_cache() {
        let mut e = engine();
        noop(
            &mut e,
            &[
                TIME_STAMP_INIT,
                SMART_POINTER_INIT,
                VECTOR_INIT,
                THREAT_SET_UP,
            ],
        );
        clock(&mut e, NOW);
        put_vector(&mut e, ZERO_VECTOR, [7.0, 8.0, 9.0]);
        let threat: Ptr<CombatThreat> = e.new_object();
        e.mem.set_u32(threat.addr() + 0x18, 0xdead);
        e.mem.set_u32(threat.addr() + 0x1c, 0xbeef);
        e.call_log = Some(vec![]);
        let signalling = f32::from_bits(0x7f80_0001);
        let result = e
            .call(
                0x009a44e0,
                &args![threat, 0x1234u32, signalling, 1.0f32, 2.0f32, 3.0f32],
            )
            .ptr::<CombatThreat>();
        assert_eq!(result, threat);
        let t = threat.addr();
        assert_eq!(
            addresses(&e),
            [
                0x009a44e0,
                TIME_STAMP_INIT,
                TIME_STAMP_INIT,
                SMART_POINTER_INIT,
                VECTOR_INIT,
                TIME_STAMP_INIT,
                THREAT_SET_UP,
                TIME_STAMP_SET
            ]
        );
        assert_eq!(calls_to(&e, SMART_POINTER_INIT), [[t + 0x14, 0]]);
        // The signalling NaN leaves the FLD/FSTP copy quiet.
        assert_eq!(
            calls_to(&e, THREAT_SET_UP),
            [vec![
                t,
                0x1234,
                0x7fc0_0001,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(e.mem.u32(t + 0x18), 0);
        assert_eq!(e.mem.u32(t + 0x1c), 0);
        assert_eq!(vector(&e, t + 0x20), [7.0, 8.0, 9.0]);
        assert_eq!(e.mem.u32(t + 0x2c), (-f32::MAX).to_bits());
    }

    #[test]
    fn default_threat_constructor_zeroes_fields_but_not_the_cached_pointers() {
        let mut e = engine();
        noop(&mut e, &[TIME_STAMP_INIT, SMART_POINTER_INIT, VECTOR_INIT]);
        clock(&mut e, NOW);
        let threat: Ptr<CombatThreat> = e.new_object();
        let t = threat.addr();
        for offset in [0u32, 4, 0x10, 0x18, 0x1c] {
            e.mem.set_u32(t + offset, 0x5555_5555);
        }
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009a5b90, &args![threat]).ptr::<CombatThreat>(),
            threat
        );
        assert_eq!(e.mem.u32(t), 0);
        assert_eq!(e.mem.u32(t + 4), 0);
        assert_eq!(e.mem.u32(t + 0x10), 0);
        assert_eq!(e.mem.u32(t + 0x18), 0x5555_5555);
        assert_eq!(e.mem.u32(t + 0x1c), 0x5555_5555);
        assert_eq!(calls_to(&e, TIME_STAMP_SET), [[t + 8, 0], [t + 0xc, 0]]);
        assert_eq!(calls_to(&e, SMART_POINTER_INIT), [[t + 0x14, 0]]);
        assert_eq!(calls_to(&e, VECTOR_INIT), [[t + 0x20]]);
        assert_eq!(calls_to(&e, TIME_STAMP_INIT), [[t + 0x2c]]);
    }

    // ---- 009a45e0: destructor body ----

    fn destructor_world() -> Engine {
        let mut e = engine();
        e.register(POINTER_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_AT_18, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        noop(&mut e, &[SMART_POINTER_DESTROY, FAKE_DETACH]);
        e
    }

    #[test]
    fn destructor_calls_the_parent_slot_with_the_geometry_then_destroys_the_pointer() {
        let mut e = destructor_world();
        let threat: Ptr<CombatThreat> = e.new_object();
        let object = e.mem.alloc(0x20);
        let parent = e.mem.alloc(8);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + 0xe8, FAKE_DETACH);
        e.mem.set_u32(parent, table);
        e.mem.set_u32(object + 0x18, parent);
        e.mem.set_u32(threat.addr() + 0x14, object);
        e.call_log = Some(vec![]);
        e.call(0x009a45e0, &args![threat]);
        assert_eq!(calls_to(&e, FAKE_DETACH), [[parent, object]]);
        assert_eq!(
            calls_to(&e, SMART_POINTER_DESTROY),
            [[threat.addr() + 0x14]]
        );
        assert_eq!(calls_to(&e, POINTER_VALUE).len(), 4);
    }

    #[test]
    fn destructor_skips_the_parent_slot_without_a_parent_or_a_geometry() {
        let mut e = destructor_world();
        let threat: Ptr<CombatThreat> = e.new_object();
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(threat.addr() + 0x14, object);
        e.call_log = Some(vec![]);
        e.call(0x009a45e0, &args![threat]);
        assert!(calls_to(&e, FAKE_DETACH).is_empty());
        assert_eq!(calls_to(&e, POINTER_VALUE).len(), 2);
        assert_eq!(calls_to(&e, SMART_POINTER_DESTROY).len(), 1);

        e.mem.set_u32(threat.addr() + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x009a45e0, &args![threat]);
        assert_eq!(calls_to(&e, POINTER_VALUE).len(), 1);
        assert_eq!(calls_to(&e, SMART_POINTER_DESTROY).len(), 1);
    }

    // ---- 009a4710 ----

    #[test]
    fn score_inside_the_reach_uses_the_distance_to_the_predicted_position() {
        let mut e = scoring_world();
        let threat = scoring_threat(&mut e, 0.75);
        let p = point(&mut e, [11.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        let score = e
            .call(0x009a4710, &args![threat, p, 0u32, 1.0f32, 1.0f32, 0.5f32])
            .f32();
        // Predicted position (11.5, 0, 0): squared distance 1.25 of reach^2 9.
        near(score, 0.75 * (1.0 - 1.25 / 9.0));
        let t = threat.addr();
        assert_eq!(calls_to(&e, THREAT_IN_WINDOW), [vec![t, 1.0f32.to_bits()]]);
        assert_eq!(calls_to(&e, THREAT_WEIGHT), [vec![t, 0, 0.5f32.to_bits()]]);
    }

    #[test]
    fn score_of_a_negative_time_skips_the_window_test() {
        let mut e = scoring_world();
        let threat = scoring_threat(&mut e, 0.75);
        let p = point(&mut e, [11.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        let score = e
            .call(0x009a4710, &args![threat, p, 0u32, -1.0f32, 1.0f32, 0.5f32])
            .f32();
        near(score, 0.75 * (1.0 - 1.25 / 9.0));
        assert!(calls_to(&e, THREAT_IN_WINDOW).is_empty());
    }

    #[test]
    fn score_is_zero_outside_the_time_window() {
        let mut e = scoring_world();
        e.register(THREAT_IN_WINDOW, |_, _| 0u32.into_ret());
        let threat = scoring_threat(&mut e, 0.75);
        let p = point(&mut e, [11.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        let score = e
            .call(0x009a4710, &args![threat, p, 0u32, 1.0f32, 1.0f32, 0.5f32])
            .f32();
        assert_eq!(score, 0.0);
        assert_eq!(addresses(&e), [0x009a4710, THREAT_IN_WINDOW]);
    }

    #[test]
    fn score_is_zero_when_the_point_is_outside_the_reach() {
        let mut e = scoring_world();
        let threat = scoring_threat(&mut e, 0.75);
        let p = point(&mut e, [100.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        let score = e
            .call(0x009a4710, &args![threat, p, 0u32, 1.0f32, 1.0f32, 0.5f32])
            .f32();
        assert_eq!(score, 0.0);
        assert!(calls_to(&e, THREAT_WEIGHT).is_empty());
    }

    #[test]
    fn score_beyond_the_speed_limit_uses_the_distance_from_the_path() {
        let mut e = scoring_world();
        settings(
            &mut e,
            &[
                (SETTING_011CF48C, 0.5),
                (SETTING_011CE990, 4.0),
                (SETTING_011CEF98, 4.0),
            ],
        );
        let threat = scoring_threat(&mut e, 0.75);
        let p = point(&mut e, [11.0, 1.0, 0.0]);
        let score = e
            .call(0x009a4710, &args![threat, p, 0u32, 1.0f32, 1.0f32, 0.5f32])
            .f32();
        // |(2.5, 1, 0)|^2 - (2.5 * 18)^2 / 18^2 = 1: the distance 1 of reach^2 9.
        near(score, 0.75 * (1.0 - 1.0 / 9.0));
    }

    #[test]
    fn score_is_zero_for_a_point_behind_or_beyond_the_path() {
        for target in [[0.0, 1.0, 0.0], [40.0, 0.0, 0.0]] {
            let mut e = scoring_world();
            settings(
                &mut e,
                &[
                    (SETTING_011CF48C, 0.5),
                    (SETTING_011CE990, 4.0),
                    (SETTING_011CEF98, 4.0),
                ],
            );
            let threat = scoring_threat(&mut e, 0.75);
            let p = point(&mut e, target);
            e.call_log = Some(vec![]);
            let score = e
                .call(0x009a4710, &args![threat, p, 0u32, 1.0f32, 1.0f32, 0.5f32])
                .f32();
            assert_eq!(score, 0.0);
            assert!(calls_to(&e, VECTOR_LENGTH_SQUARED).is_empty());
            assert!(calls_to(&e, THREAT_WEIGHT).is_empty());
        }
    }

    // ---- 009a4a10 ----

    fn velocity_world() -> Engine {
        let mut e = engine();
        clock(&mut e, NOW);
        reference_doubles(&mut e);
        put_vector(&mut e, ZERO_VECTOR, [4.0, 5.0, 6.0]);
        e
    }

    fn stale_threat(e: &mut Engine, object: u32, flag: u32) -> Ptr<CombatThreat> {
        let reference = new_reference(e, [0.0; 3], object, flag);
        let threat: Ptr<CombatThreat> = e.new_object();
        e.set(threat, CombatThreat::pThreat, reference);
        e.set(
            threat,
            CombatThreat::fCachedLinearVelocityTimeStamp,
            (NOW - 1.0).to_bits(),
        );
        put_vector(e, threat.addr() + 0x20, [1.0, 1.0, 1.0]);
        threat
    }

    #[test]
    fn linear_velocity_reuses_a_young_cache() {
        let mut e = velocity_world();
        let threat = stale_threat(&mut e, 0x900, 0);
        e.set(
            threat,
            CombatThreat::fCachedLinearVelocityTimeStamp,
            (NOW - 0.1).to_bits(),
        );
        put_vector(&mut e, threat.addr() + 0x20, [3.0, 2.0, 1.0]);
        let result = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a4a10, &args![threat, result]).u32(), result);
        assert_eq!(vector(&e, result), [3.0, 2.0, 1.0]);
        assert_eq!(addresses(&e), [0x009a4a10, TIME_STAMP_AGE]);
    }

    #[test]
    fn linear_velocity_without_a_3d_object_is_the_zero_vector() {
        let mut e = velocity_world();
        let threat = stale_threat(&mut e, 0, 0);
        let result = e.mem.alloc(12);
        e.call(0x009a4a10, &args![threat, result]);
        assert_eq!(vector(&e, result), [4.0, 5.0, 6.0]);
        // The cache itself is not rewritten, only the stamp.
        assert_eq!(vector(&e, threat.addr() + 0x20), [1.0, 1.0, 1.0]);
        assert_eq!(e.mem.f32(threat.addr() + 0x2c), NOW);
    }

    #[test]
    fn linear_velocity_of_a_character_controller_is_cached() {
        let mut e = velocity_world();
        let threat = stale_threat(&mut e, 0x900, 1);
        e.register(0x0093_06d0, |_, _| 0x700u32.into_ret());
        e.register(0x0066_ca00, |e, a| {
            assert_eq!(a[0], 0x700);
            put_vector(e, a[1], [1.5, 2.5, 3.5]);
            Ret::default()
        });
        let result = e.mem.alloc(12);
        e.call(0x009a4a10, &args![threat, result]);
        assert_eq!(vector(&e, result), [1.5, 2.5, 3.5]);
        assert_eq!(vector(&e, threat.addr() + 0x20), [1.5, 2.5, 3.5]);
        assert_eq!(e.mem.f32(threat.addr() + 0x2c), NOW);
    }

    #[test]
    fn linear_velocity_without_a_controller_but_with_the_flag_is_zero() {
        let mut e = velocity_world();
        let threat = stale_threat(&mut e, 0x900, 1);
        e.register(0x0093_06d0, |_, _| 0u32.into_ret());
        e.register(0x0044_4d00, |_, a| {
            assert_eq!(a[1], 8);
            1u32.into_ret()
        });
        let result = e.mem.alloc(12);
        e.call(0x009a4a10, &args![threat, result]);
        assert_eq!(vector(&e, result), [4.0, 5.0, 6.0]);
        assert_eq!(vector(&e, threat.addr() + 0x20), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn linear_velocity_looks_up_the_rigid_body_when_the_3d_object_changes() {
        let mut e = velocity_world();
        let threat = stale_threat(&mut e, 0x900, 0);
        e.register(0x004b_5260, |_, a| {
            assert_eq!(a[0], 0x900);
            0x500u32.into_ret()
        });
        e.register(0x006f_a820, |_, a| {
            assert_eq!(a[0], 0x500);
            0x600u32.into_ret()
        });
        e.register(0x0065_3270, |_, a| {
            assert_eq!(a, [0x0126_81c0, 0x600]);
            0x700u32.into_ret()
        });
        e.register(0x0056_0d50, |e, a| {
            assert_eq!(a[0], 0x700);
            put_vector(e, a[1], [7.0, 8.0, 9.0]);
            Ret::default()
        });
        let result = e.mem.alloc(12);
        e.call(0x009a4a10, &args![threat, result]);
        assert_eq!(e.get(threat, CombatThreat::pCachedThreat3D).addr(), 0x900);
        assert_eq!(e.get(threat, CombatThreat::pCachedRigidBody).addr(), 0x700);
        assert_eq!(vector(&e, result), [7.0, 8.0, 9.0]);
    }

    #[test]
    fn linear_velocity_keeps_the_known_3d_object_and_falls_back_to_zero_without_a_body() {
        let mut e = velocity_world();
        let threat = stale_threat(&mut e, 0x900, 0);
        e.set(threat, CombatThreat::pCachedThreat3D, Ptr::new(0x900));
        e.call_log = Some(vec![]);
        let result = e.mem.alloc(12);
        e.call(0x009a4a10, &args![threat, result]);
        assert_eq!(vector(&e, result), [4.0, 5.0, 6.0]);
        assert_eq!(vector(&e, threat.addr() + 0x20), [4.0, 5.0, 6.0]);
        assert!(calls_to(&e, 0x004b_5260).is_empty());
    }

    // ---- 009a4c70, 009a4e30 ----

    #[test]
    fn avoid_node_inside_the_reach_is_a_point_record() {
        let mut e = scoring_world();
        let threat = scoring_threat(&mut e, 0.5);
        let result = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009a4c70, &args![threat, result, 0u32]).u32(),
            result
        );
        // Position (10, 0, 0) plus (3, 0, 0) * 0.5; radius 2; cost 0.5 * 4.
        assert_eq!(vector(&e, result), [11.5, 0.0, 0.0]);
        assert_eq!(e.mem.f32(result + 0x18), 2.0);
        assert_eq!(e.mem.f32(result + 0x1c), 2.0);
        assert_eq!(e.mem.u32(result + 0x20), 0);
        assert_eq!(
            calls_to(&e, THREAT_WEIGHT),
            [vec![threat.addr(), 0, 0.0f32.to_bits()]]
        );
    }

    #[test]
    fn avoid_node_outside_the_reach_is_a_segment_record() {
        let mut e = scoring_world();
        settings(
            &mut e,
            &[
                (SETTING_011CF48C, 1.0),
                (SETTING_011CE990, 4.0),
                (SETTING_011CEF98, 4.0),
            ],
        );
        let threat = scoring_threat(&mut e, 0.5);
        let result = e.mem.alloc(0x24);
        e.call(0x009a4c70, &args![threat, result, 0u32]);
        assert_eq!(vector(&e, result), [9.5, 0.0, 0.0]);
        assert_eq!(vector(&e, result + 0xc), [17.5, 0.0, 0.0]);
        assert_eq!(e.mem.f32(result + 0x18), 2.0);
        assert_eq!(e.mem.f32(result + 0x1c), 2.0);
        assert_eq!(e.mem.u32(result + 0x20), 1);
    }

    #[test]
    fn segment_record_copies_both_points_and_quiets_the_floats() {
        let mut e = engine();
        let first = point(&mut e, [1.0, 2.0, 3.0]);
        let second = point(&mut e, [4.0, 5.0, 6.0]);
        let node = e.mem.alloc(0x24);
        let signalling = f32::from_bits(0x7f80_0002);
        let result = e.call(0x009a4e30, &args![node, first, second, signalling, 8.0f32]);
        assert_eq!(result.u32(), node);
        assert_eq!(vector(&e, node), [1.0, 2.0, 3.0]);
        assert_eq!(vector(&e, node + 0xc), [4.0, 5.0, 6.0]);
        assert_eq!(e.mem.u32(node + 0x18), 0x7fc0_0002);
        assert_eq!(e.mem.f32(node + 0x1c), 8.0);
        assert_eq!(e.mem.u32(node + 0x20), 1);
    }

    // ---- 009a4e90, 009a4ef0, 009a4f40 ----

    #[test]
    fn threat_save_and_load_touch_radius_stamps_and_damage_in_order() {
        let mut e = engine();
        noop(
            &mut e,
            &[0x0086_5e50, 0x009a_5f90, 0x0086_4980, 0x009a_5ff0],
        );
        let threat: Ptr<CombatThreat> = e.new_object();
        let t = threat.addr();
        let buffer = 0x7777u32;
        e.call_log = Some(vec![]);
        e.call(0x009a4e90, &args![threat, buffer]);
        e.call(0x009a4ef0, &args![threat, buffer]);
        let log = e.call_log.clone().unwrap();
        assert_eq!(
            log,
            vec![
                (0x009a4e90, vec![t, buffer]),
                (0x0086_5e50, vec![buffer, t + 4, 4, 0]),
                (0x009a_5f90, vec![t + 8, buffer]),
                (0x009a_5f90, vec![t + 0xc, buffer]),
                (0x0086_5e50, vec![buffer, t + 0x10, 4, 0]),
                (0x009a4ef0, vec![t, buffer]),
                (0x0086_4980, vec![buffer, t + 4, 4]),
                (0x009a_5ff0, vec![t + 8, buffer]),
                (0x009a_5ff0, vec![t + 0xc, buffer]),
                (0x0086_4980, vec![buffer, t + 0x10, 4]),
            ]
        );
    }

    fn resolve_world(cast: u32) -> Engine {
        let mut e = engine();
        e.register(0x0048_39c0, |_, a| {
            assert_eq!(a[0], 0x100);
            0x300u32.into_ret()
        });
        e.register_double(0x00ec_43fb, move |_, a| {
            assert_eq!(a, [0x300, 0, 0x0118_3028, 0x0118_41cc, 0]);
            cast.into_ret()
        });
        noop(&mut e, &[SET_TARGETED]);
        e
    }

    #[test]
    fn resolving_a_form_id_stores_the_cast_form_and_marks_it() {
        let mut e = resolve_world(0x400);
        let threat: Ptr<CombatThreat> = e.new_object();
        e.set(threat, CombatThreat::pThreat, Ptr::new(0x100));
        e.call_log = Some(vec![]);
        e.call(0x009a4f40, &args![threat, 0u32]);
        assert_eq!(e.get(threat, CombatThreat::pThreat).addr(), 0x400);
        assert_eq!(calls_to(&e, SET_TARGETED), [[0x400, 1]]);
    }

    #[test]
    fn resolving_a_failed_cast_or_a_null_id_leaves_null() {
        let mut e = resolve_world(0);
        let threat: Ptr<CombatThreat> = e.new_object();
        e.set(threat, CombatThreat::pThreat, Ptr::new(0x100));
        e.call_log = Some(vec![]);
        e.call(0x009a4f40, &args![threat, 0u32]);
        assert_eq!(e.get(threat, CombatThreat::pThreat).addr(), 0);
        assert!(calls_to(&e, SET_TARGETED).is_empty());

        e.call_log = Some(vec![]);
        e.call(0x009a4f40, &args![threat, 0u32]);
        assert_eq!(addresses(&e), [0x009a4f40]);
    }

    // ---- map and NiTMap constructors and destructors ----

    fn table_world() -> Engine {
        let mut e = engine();
        e.register(ALLOCATE_TABLE, |e, a| e.mem.alloc(a[0]).into_ret());
        noop(
            &mut e,
            &[
                MEMSET,
                LOCK_INIT,
                MAP_REMOVE_ALL,
                FREE_TABLE,
                OPERATOR_DELETE,
            ],
        );
        e.register(MAP_FIRST_ITEM, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn table_base_constructor_allocates_and_clears_the_buckets() {
        let mut e = table_world();
        let map: Ptr<NiTMap> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009a5d80, &args![map, 0x25u32]).ptr::<NiTMap>(),
            map
        );
        let m = map.addr();
        assert_eq!(e.mem.u32(m), VTABLE_NI_TMAP_BASE);
        assert_eq!(e.mem.u32(m + 4), 0x25);
        assert_eq!(e.mem.u32(m + 0xc), 0);
        let table = e.mem.u32(m + 8);
        assert_eq!(calls_to(&e, ALLOCATE_TABLE), [[0x94]]);
        assert_eq!(calls_to(&e, MEMSET), [[table, 0, 0x94]]);
    }

    #[test]
    fn map_constructors_set_their_vtables_last() {
        let mut e = table_world();
        let map: Ptr<NiTMap> = e.new_object();
        e.call(0x009a5d20, &args![map, 7u32]);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_NI_TMAP);
        assert_eq!(e.mem.u32(map.addr() + 4), 7);

        let threat_map: Ptr<CombatThreatMap> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009a4fb0, &args![threat_map])
                .ptr::<CombatThreatMap>(),
            threat_map
        );
        let m = threat_map.addr();
        assert_eq!(e.mem.u32(m), VTABLE_COMBAT_THREAT_MAP);
        assert_eq!(e.mem.u32(m + 4), 0x25);
        assert_eq!(calls_to(&e, LOCK_INIT), [[m + 0x10]]);
    }

    #[test]
    fn destructors_clear_the_map_and_free_the_table() {
        let mut e = table_world();
        let map: Ptr<NiTMap> = e.new_object();
        e.mem.set_u32(map.addr() + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x009a5e50, &args![map]);
        assert_eq!(e.mem.u32(map.addr()), VTABLE_NI_TMAP_BASE);
        assert_eq!(addresses(&e), [0x009a5e50, MAP_REMOVE_ALL, FREE_TABLE]);
        assert_eq!(calls_to(&e, FREE_TABLE), [[0x4444]]);

        e.call_log = Some(vec![]);
        e.call(0x009a5df0, &args![map]);
        assert_eq!(
            addresses(&e),
            [0x009a5df0, MAP_REMOVE_ALL, MAP_REMOVE_ALL, FREE_TABLE]
        );
        assert_eq!(e.mem.u32(map.addr()), VTABLE_NI_TMAP_BASE);
    }

    #[test]
    fn scalar_deleting_destructors_delete_only_when_bit_zero_is_set() {
        for (address, vtable) in [
            (0x009a5d50u32, VTABLE_NI_TMAP_BASE),
            (0x009a5e80, VTABLE_NI_TMAP_BASE),
            (0x009a5020, VTABLE_NI_TMAP_BASE),
        ] {
            let mut e = table_world();
            let map: Ptr<NiTMap> = e.new_object();
            for (flags, deleted) in [(1u32, true), (0, false), (2, false), (3, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![map, flags]).u32(), map.addr());
                assert_eq!(
                    calls_to(&e, OPERATOR_DELETE),
                    if deleted {
                        vec![vec![map.addr()]]
                    } else {
                        vec![]
                    }
                );
                assert_eq!(e.mem.u32(map.addr()), vtable);
            }
        }
    }

    #[test]
    fn threat_map_destructor_frees_the_threats_then_runs_the_map_destructors() {
        let mut e = table_world();
        let map: Ptr<CombatThreatMap> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x009a5050, &args![map]);
        assert_eq!(
            addresses(&e),
            [
                0x009a5050,
                MAP_FIRST_ITEM,
                MAP_REMOVE_ALL,
                MAP_REMOVE_ALL,
                MAP_REMOVE_ALL,
                FREE_TABLE
            ]
        );
        assert_eq!(e.mem.u32(map.addr()), VTABLE_NI_TMAP_BASE);
    }

    // ---- threat deletion, expiry, removal ----

    fn deletion_world() -> Engine {
        let mut e = engine();
        e.register(POINTER_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x006a_7f50, |e, a| e.mem.f32(a[0]).into_ret());
        clock(&mut e, NOW);
        noop(
            &mut e,
            &[
                SMART_POINTER_DESTROY,
                OPERATOR_DELETE,
                LOCK_FOR_WRITE,
                UNLOCK_AFTER_WRITE,
                MAP_REMOVE_ALL,
            ],
        );
        e
    }

    fn threat_ending(e: &mut Engine, end: f32) -> u32 {
        let threat: Ptr<CombatThreat> = e.new_object();
        e.mem.set_f32(threat.addr() + 0xc, end);
        threat.addr()
    }

    #[test]
    fn end_time_test_is_a_strict_greater_than() {
        let mut e = deletion_world();
        let threat = threat_ending(&mut e, 100.0);
        let call = |e: &mut Engine, time: f32| e.call(0x009a5170, &args![threat, time]).bool();
        assert!(!call(&mut e, 99.0));
        assert!(!call(&mut e, 100.0));
        assert!(call(&mut e, 100.5));
        assert!(!call(&mut e, f32::NAN));
        e.mem.set_f32(threat + 0xc, f32::NAN);
        assert!(!call(&mut e, 100.5));
    }

    #[test]
    fn expiry_frees_and_removes_only_the_expired_threats() {
        let mut e = deletion_world();
        let (t1, t2, t3) = (
            threat_ending(&mut e, 90.0),
            threat_ending(&mut e, 110.0),
            threat_ending(&mut e, 95.0),
        );
        let (map, _) = map_with(&mut e, &[(1, t1), (2, t2), (3, t3)]);
        walk(&mut e, &[(1, t1), (2, t2), (3, t3)]);
        e.call_log = Some(vec![]);
        e.call(0x009a50b0, &args![map]);
        assert_eq!(map_items(&e, map), [(2, t2)]);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 1);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), [[t1], [t3]]);
        let lock = map.addr() + 0x10;
        assert_eq!(calls_to(&e, LOCK_FOR_WRITE), [[lock]]);
        assert_eq!(calls_to(&e, UNLOCK_AFTER_WRITE), [[lock]]);
        let order = addresses(&e);
        assert_eq!(order[1], MAP_FIRST_ITEM);
        assert_eq!(order[2], LOCK_FOR_WRITE);
        assert_eq!(*order.last().unwrap(), UNLOCK_AFTER_WRITE);
    }

    #[test]
    fn expiry_of_an_empty_map_takes_no_lock() {
        let mut e = deletion_world();
        let (map, _) = map_with(&mut e, &[]);
        walk(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x009a50b0, &args![map]);
        assert_eq!(addresses(&e), [0x009a50b0, MAP_FIRST_ITEM]);
    }

    #[test]
    fn threat_removal_by_reference_frees_the_threat_and_its_key() {
        let mut e = deletion_world();
        e.register(REFERENCE_KEY, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(MAP_LOOKUP, |e, a| {
            // `(map, key, out)`: the test map holds the threat for key 5 only.
            if a[1] == 5 {
                e.mem.set_u32(a[2], e.mem.u32(a[0] + 0x14));
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let threat = threat_ending(&mut e, 0.0);
        let (map, _) = map_with(&mut e, &[(5, threat), (6, 0x66)]);
        e.mem.set_u32(map.addr() + 0x14, threat);
        let found = e.mem.alloc(0x10);
        e.mem.set_u32(found + 0xc, 5);
        let missing = e.mem.alloc(0x10);
        e.mem.set_u32(missing + 0xc, 8);
        e.call_log = Some(vec![]);
        e.call(0x009a51e0, &args![map, missing]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(map_items(&e, map), [(5, threat), (6, 0x66)]);
        e.call(0x009a51e0, &args![map, found]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), [[threat]]);
        assert_eq!(map_items(&e, map), [(6, 0x66)]);
        let lock = map.addr() + 0x10;
        assert_eq!(calls_to(&e, LOCK_FOR_WRITE), [[lock], [lock]]);
        assert_eq!(calls_to(&e, UNLOCK_AFTER_WRITE), [[lock], [lock]]);
    }

    #[test]
    fn clearing_the_map_frees_every_non_null_threat_then_empties_it() {
        let mut e = deletion_world();
        let (t1, t3) = (threat_ending(&mut e, 0.0), threat_ending(&mut e, 0.0));
        let (map, _) = map_with(&mut e, &[(1, t1), (2, 0), (3, t3)]);
        walk(&mut e, &[(1, t1), (2, 0), (3, t3)]);
        e.call_log = Some(vec![]);
        e.call(0x009a5270, &args![map]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), [[t1], [t3]]);
        assert_eq!(*addresses(&e).last().unwrap(), MAP_REMOVE_ALL);
        assert_eq!(calls_to(&e, MAP_REMOVE_ALL), [[map.addr()]]);
    }

    // ---- adding threats: 009a5530, 009a52f0, 009a54a0 ----

    /// Doubles for everything `AddThreat` calls outside this unit; the debug
    /// flag byte is returned through `00408d60`.
    fn add_world(debug_flag: bool) -> Engine {
        let mut e = engine();
        noop(
            &mut e,
            &[
                LOCK_FOR_WRITE,
                UNLOCK_AFTER_WRITE,
                SET_TARGETED,
                THREAT_SET_UP,
                TIME_STAMP_INIT,
                SMART_POINTER_INIT,
                VECTOR_INIT,
                THREAT_SET_DEBUG,
            ],
        );
        clock(&mut e, NOW);
        e.register(REFERENCE_KEY, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MAP_LOOKUP, |e, a| {
            // The test map keeps one existing threat for key 5 at map + 0x14.
            if a[1] == 5 {
                e.mem.set_u32(a[2], e.mem.u32(a[0] + 0x14));
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, u8::from(debug_flag));
        e.register_double(SETTING_BOOL_VALUE, move |_, a| {
            assert_eq!(a[0], SETTING_BOOL_011F1BA0);
            flag.into_ret()
        });
        e
    }

    fn keyed_reference(e: &mut Engine, key: u32) -> Ptr {
        let reference = e.mem.alloc(0x100);
        e.mem.set_u32(reference + 0xc, key);
        Ptr::new(reference)
    }

    #[test]
    fn add_threat_creates_and_registers_a_new_threat() {
        let mut e = add_world(false);
        let (map, _) = map_with(&mut e, &[]);
        let reference = keyed_reference(&mut e, 9);
        e.call_log = Some(vec![]);
        e.call(
            0x009a5530,
            &args![map, reference, 5.0f32, 1.0f32, 2.0f32, 3.0f32],
        );
        let items = map_items(&e, map);
        assert_eq!(items.len(), 1);
        let (key, threat) = items[0];
        assert_eq!(key, 9);
        let r = reference.addr();
        assert_eq!(calls_to(&e, OPERATOR_NEW), [[0x30]]);
        assert_eq!(
            calls_to(&e, THREAT_SET_UP),
            [vec![
                threat,
                r,
                5.0f32.to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(calls_to(&e, SET_TARGETED), [[r, 1]]);
        assert!(calls_to(&e, THREAT_SET_DEBUG).is_empty());
        let lock = map.addr() + 0x10;
        assert_eq!(calls_to(&e, LOCK_FOR_WRITE), [[lock]]);
        assert_eq!(calls_to(&e, UNLOCK_AFTER_WRITE), [[lock]]);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 1);
    }

    #[test]
    fn add_threat_resets_up_an_existing_threat_with_flag_one() {
        let mut e = add_world(true);
        let (map, _) = map_with(&mut e, &[(5, 0x1111)]);
        e.mem.set_u32(map.addr() + 0x14, 0x1111);
        let reference = keyed_reference(&mut e, 5);
        e.call_log = Some(vec![]);
        e.call(
            0x009a5530,
            &args![map, reference, 5.0f32, 1.0f32, 2.0f32, 3.0f32],
        );
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&e, THREAT_SET_UP),
            [vec![
                0x1111,
                reference.addr(),
                5.0f32.to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1
            ]]
        );
        // The debug flag is set: the existing threat gets `004534f0(0)`.
        assert_eq!(calls_to(&e, THREAT_SET_DEBUG), [[0x1111, 0]]);
        assert_eq!(map_items(&e, map), [(5, 0x1111)]);
    }

    #[test]
    fn add_threat_calls_the_debug_hook_for_a_new_threat_when_the_flag_is_set() {
        let mut e = add_world(true);
        let (map, _) = map_with(&mut e, &[]);
        let reference = keyed_reference(&mut e, 9);
        e.call_log = Some(vec![]);
        e.call(
            0x009a5530,
            &args![map, reference, 1.0f32, 1.0f32, 1.0f32, 1.0f32],
        );
        let threat = map_items(&e, map)[0].1;
        assert_eq!(calls_to(&e, THREAT_SET_DEBUG), [[threat, 0]]);
    }

    /// The base-form filters `009a52f0` applies.
    fn filter_world() -> Engine {
        let mut e = add_world(false);
        e.register(0x0044_4d00, |_, a| {
            assert_eq!(a[1], 1);
            0u32.into_ret()
        });
        e.register(0x0041_81e0, |_, _| 0xb00u32.into_ret());
        e.register(0x004f_d3c0, |_, a| {
            assert_eq!(a[0], 0xb00);
            0xc00u32.into_ret()
        });
        e.register(0x004f_d360, |_, _| 1u32.into_ret());
        e.register(0x0047_7900, |_, _| 3.0f32.into_ret());
        e.register(0x006a_78f0, |_, _| 4.0f32.into_ret());
        e.register(FLOAT_REMAP, |_, _| 5.0f32.into_ret());
        e.register(0x0097_5300, |_, _| 0u32.into_ret());
        settings(
            &mut e,
            &[
                (SETTING_011CF3F0, 2.0),
                (SETTING_011CEA74, 1.0),
                (SETTING_011CF764, 50.0),
                (SETTING_011CEABC, 10.0),
            ],
        );
        e
    }

    fn filtered_add(e: &mut Engine, last: f32) -> Option<Vec<u32>> {
        let (map, _) = map_with(e, &[]);
        let reference = keyed_reference(e, 9);
        e.mem.set_f32(reference.addr() + 0xe4, last);
        e.call_log = Some(vec![]);
        e.call(0x009a52f0, &args![map, reference]);
        let added = calls_to(e, THREAT_SET_UP);
        assert!(added.len() <= 1);
        added.into_iter().next().map(|mut words| {
            // Without the threat and reference addresses: radius, start, end, damage.
            words.drain(..2);
            words.truncate(4);
            words
        })
    }

    fn bits(values: [f32; 4]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    #[test]
    fn filter_adds_a_threat_with_the_remapped_radius_and_an_unbounded_end() {
        let mut e = filter_world();
        assert_eq!(
            filtered_add(&mut e, 0.0),
            Some(bits([5.0, 0.0, f32::MAX, 4.0]))
        );
        // `006479a0` gets (radius, value, floor setting).
        assert_eq!(
            calls_to(&e, FLOAT_REMAP),
            [vec![3.0f32.to_bits(), 4.0f32.to_bits(), 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn filter_uses_the_setting_when_the_last_value_is_float_max() {
        let mut e = filter_world();
        e.register(0x0097_5300, |_, _| 1u32.into_ret());
        assert_eq!(
            filtered_add(&mut e, f32::MAX),
            Some(bits([5.0, 0.0, 50.0, 4.0]))
        );
    }

    #[test]
    fn filter_derives_the_offsets_from_the_last_value() {
        let mut e = filter_world();
        e.register(0x0097_5300, |_, _| 1u32.into_ret());
        // 20 + 1 = 21 is above the setting 10: the start offset is the excess.
        assert_eq!(
            filtered_add(&mut e, 20.0),
            Some(bits([5.0, 11.0, 21.0, 4.0]))
        );
        // 5 + 1 = 6 is not: no start offset.
        assert_eq!(filtered_add(&mut e, 5.0), Some(bits([5.0, 0.0, 6.0, 4.0])));
        // -5 + 1 is not positive: nothing is added.
        assert_eq!(filtered_add(&mut e, -5.0), None);
    }

    #[test]
    fn filter_rejects_references_that_fail_a_test() {
        // Flag 1 set.
        let mut e = filter_world();
        e.register(0x0044_4d00, |_, _| 1u32.into_ret());
        assert_eq!(filtered_add(&mut e, 0.0), None);
        assert!(calls_to(&e, 0x0041_81e0).is_empty());
        // The capability test fails.
        let mut e = filter_world();
        e.register(0x004f_d360, |_, _| 0u32.into_ret());
        assert_eq!(filtered_add(&mut e, 0.0), None);
        // No sub-object.
        let mut e = filter_world();
        e.register(0x004f_d3c0, |_, _| 0u32.into_ret());
        assert_eq!(filtered_add(&mut e, 0.0), None);
        // The value is below the floor setting.
        let mut e = filter_world();
        e.register(0x006a_78f0, |_, _| 1.0f32.into_ret());
        assert_eq!(filtered_add(&mut e, 0.0), None);
        assert!(calls_to(&e, FLOAT_REMAP).is_empty());
        // A NaN value fails the floor test too.
        let mut e = filter_world();
        e.register(0x006a_78f0, |_, _| f32::NAN.into_ret());
        assert_eq!(filtered_add(&mut e, 0.0), None);
        // The remapped radius is below the minimum setting.
        let mut e = filter_world();
        e.register(FLOAT_REMAP, |_, _| 0.5f32.into_ret());
        assert_eq!(filtered_add(&mut e, 0.0), None);
    }

    #[test]
    fn last_value_getter_reads_the_float_at_0xe4() {
        let mut e = engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_f32(object + 0xe4, 12.5);
        assert_eq!(e.call(0x009a5480, &args![object]).f32(), 12.5);
        e.mem.set_u32(object + 0xe4, 0x7f80_0001);
        assert_eq!(
            e.call(0x009a5480, &args![object]).f32().to_bits(),
            0x7fc0_0001
        );
    }

    #[test]
    fn add_from_another_object_remaps_the_radius_and_starts_at_zero() {
        let mut e = filter_world();
        let (map, _) = map_with(&mut e, &[]);
        let reference = keyed_reference(&mut e, 9);
        e.call_log = Some(vec![]);
        e.call(0x009a54a0, &args![map, reference, 0x9999u32, 7.0f32]);
        let added = calls_to(&e, THREAT_SET_UP);
        assert_eq!(added.len(), 1);
        assert_eq!(
            added[0][1..],
            [
                reference.addr(),
                5.0f32.to_bits(),
                0.0f32.to_bits(),
                7.0f32.to_bits(),
                4.0f32.to_bits(),
                0
            ]
        );

        // A value below the floor setting adds nothing.
        e.register(0x006a_78f0, |_, a| {
            assert_eq!(a[0], 0x9999);
            1.0f32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x009a54a0, &args![map, reference, 0x9999u32, 7.0f32]);
        assert!(calls_to(&e, THREAT_SET_UP).is_empty());
    }

    // ---- scoring over the map: 009a5670, 009a5780, 009a57a0 ----

    #[test]
    fn start_time_getter_copies_the_stamp_word() {
        let mut e = engine();
        let threat: Ptr<CombatThreat> = e.new_object();
        e.set(threat, CombatThreat::fStartTime, 0x1234_5678);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x009a5780, &args![threat, out]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x1234_5678);
    }

    fn summing_world() -> (Engine, Ptr<CombatThreatMap>, Vec<Ptr<CombatThreat>>) {
        let mut e = scoring_world();
        noop(&mut e, &[LOCK_ENTER, LOCK_LEAVE]);
        e.register(0x006a_7f50, |e, a| e.mem.f32(a[0]).into_ret());
        let early = scoring_threat(&mut e, 1.0);
        let late = scoring_threat(&mut e, 0.5);
        e.set(early, CombatThreat::fStartTime, 50.0f32.to_bits());
        e.set(late, CombatThreat::fStartTime, 150.0f32.to_bits());
        let (map, _) = map_with(&mut e, &[(1, early.addr()), (2, late.addr())]);
        walk(&mut e, &[(1, early.addr()), (2, late.addr())]);
        (e, map, vec![early, late])
    }

    #[test]
    fn summed_score_counts_threats_that_start_after_the_compare_value() {
        let (mut e, map, _) = summing_world();
        e.call_log = Some(vec![]);
        let total = e
            .call(
                0x009a5670,
                &args![map, 11.0f32, 1.0f32, 0.0f32, 0u32, 1.0f32, 1.0f32, 100.0f32, 0.5f32],
            )
            .f32();
        // Only the late threat (weight 0.5) passes `100 < start time`.
        near(total, 0.5 * (1.0 - 1.25 / 9.0));
        // The time 1.0 is added to the current time 100.
        let window = calls_to(&e, THREAT_IN_WINDOW);
        assert_eq!(window.len(), 1);
        assert_eq!(window[0][1], 101.0f32.to_bits());
        let lock = map.addr() + 0x10;
        assert_eq!(calls_to(&e, LOCK_ENTER), [[lock]]);
        assert_eq!(calls_to(&e, LOCK_LEAVE), [[lock]]);
    }

    #[test]
    fn summed_score_is_clamped_to_one_and_stops_there() {
        let (mut e, map, threats) = summing_world();
        e.set(threats[0], CombatThreat::fDamage, 2.0);
        e.set(threats[0], CombatThreat::fStartTime, 150.0f32.to_bits());
        e.call_log = Some(vec![]);
        let total = e
            .call(
                0x009a5670,
                &args![map, 11.0f32, 1.0f32, 0.0f32, 0u32, -1.0f32, 1.0f32, 100.0f32, 0.5f32],
            )
            .f32();
        assert_eq!(total, 1.0);
        // A negative time is used as it is, and the loop stopped after the
        // first threat.
        assert!(calls_to(&e, THREAT_IN_WINDOW).is_empty());
        assert_eq!(calls_to(&e, THREAT_WEIGHT).len(), 1);
    }

    #[test]
    fn summed_score_is_zero_when_no_threat_starts_after_the_compare_value() {
        let (mut e, map, _) = summing_world();
        let total = e
            .call(
                0x009a5670,
                &args![map, 11.0f32, 1.0f32, 0.0f32, 0u32, 1.0f32, 1.0f32, 200.0f32, 0.5f32],
            )
            .f32();
        assert_eq!(total, 0.0);
    }

    fn collection_world() -> (Engine, Rc<RefCell<Vec<[f32; 3]>>>) {
        let mut e = scoring_world();
        noop(&mut e, &[LOCK_ENTER, LOCK_LEAVE]);
        e.register(0x008a_7f80, |_, a| a[0].into_ret());
        e.register(0x0099_89d0, |_, _| 0u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(AVOID_NODE_ARRAY_INIT, |_, a| a[0].into_ret());
        let added = Rc::new(RefCell::new(Vec::new()));
        let record = added.clone();
        e.register_double(AVOID_NODE_ARRAY_ADD, move |e, a| {
            record.borrow_mut().push(vector(e, a[1]));
            Ret::default()
        });
        settings(
            &mut e,
            &[
                (SETTING_011CF48C, 10.0),
                (SETTING_011CE990, 4.0),
                (SETTING_011CEF98, 4.0),
                (SETTING_011CFBCC, 0.5),
            ],
        );
        (e, added)
    }

    #[test]
    fn collection_gathers_the_records_of_the_strong_threats() {
        let (mut e, added) = collection_world();
        let strong = scoring_threat(&mut e, 0.9);
        let weak = scoring_threat(&mut e, 0.3);
        let (map, _) = map_with(&mut e, &[(1, strong.addr()), (2, weak.addr())]);
        walk(&mut e, &[(1, strong.addr()), (2, weak.addr())]);
        let p = point(&mut e, [11.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        let array = e.call(0x009a57a0, &args![map, p, 0x55u32, 1.0f32]).u32();
        assert_ne!(array, 0);
        // One 0x18-byte array, one record: the strong threat's point record.
        assert_eq!(calls_to(&e, ALLOCATE), [[0x18]]);
        assert_eq!(*added.borrow(), [[11.5, 0.0, 0.0]]);
        // The score is called with the extra reach 1024 and no fallback.
        assert_eq!(calls_to(&e, THREAT_WEIGHT).len(), 3);
        let lock = map.addr() + 0x10;
        assert_eq!(calls_to(&e, LOCK_ENTER), [[lock]]);
        assert_eq!(calls_to(&e, LOCK_LEAVE), [[lock]]);
    }

    #[test]
    fn collection_falls_back_to_the_best_threat_when_none_passes_the_threshold() {
        let (mut e, added) = collection_world();
        settings(
            &mut e,
            &[
                (SETTING_011CF48C, 10.0),
                (SETTING_011CE990, 4.0),
                (SETTING_011CEF98, 4.0),
                (SETTING_011CFBCC, 0.95),
            ],
        );
        let best = scoring_threat(&mut e, 0.9);
        let other = scoring_threat(&mut e, 0.3);
        let (map, _) = map_with(&mut e, &[(1, other.addr()), (2, best.addr())]);
        walk(&mut e, &[(1, other.addr()), (2, best.addr())]);
        let p = point(&mut e, [11.0, 1.0, 0.0]);
        let array = e.call(0x009a57a0, &args![map, p, 0x55u32, 1.0f32]).u32();
        assert_ne!(array, 0);
        assert_eq!(added.borrow().len(), 1);
        // The record is built after the loop from the best threat; with the
        // weight 0.9 its cost would be 3.6 but only the point is observed.
        assert_eq!(added.borrow()[0], [11.5, 0.0, 0.0]);
    }

    #[test]
    fn collection_returns_null_without_scores_or_for_a_busy_other() {
        let (mut e, added) = collection_world();
        e.register(THREAT_IN_WINDOW, |_, _| 0u32.into_ret());
        let threat = scoring_threat(&mut e, 0.9);
        let (map, _) = map_with(&mut e, &[(1, threat.addr())]);
        walk(&mut e, &[(1, threat.addr())]);
        let p = point(&mut e, [11.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a57a0, &args![map, p, 0x55u32, 1.0f32]).u32(), 0);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert!(added.borrow().is_empty());

        // `009989d0` set for the object `008a7f80` returns: nothing is done.
        let (mut e, _) = collection_world();
        e.register(0x0099_89d0, |_, a| {
            assert_eq!(a[0], 0x55);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009a57a0, &args![0x1000u32, 0x2000u32, 0x55u32, 1.0f32])
                .u32(),
            0
        );
        assert_eq!(addresses(&e), [0x009a57a0, 0x008a_7f80, 0x0099_89d0]);
    }

    // ---- save and load: 009a59e0, 009a5a70, 009a5c40, 009a5cd0 ----

    #[test]
    fn save_game_writes_count_and_each_non_null_threat() {
        let mut e = engine();
        noop(
            &mut e,
            &[0x0086_5e50, 0x009a_5f90, 0x0086_5db0, 0x0086_5ff0],
        );
        e.register(0x0086_5f20, |_, _| 0x77u32.into_ret());
        let (t1, t3) = (threat_ending(&mut e, 0.0), threat_ending(&mut e, 0.0));
        let (map, _) = map_with(&mut e, &[(10, t1), (20, 0), (30, t3)]);
        walk(&mut e, &[(10, t1), (20, 0), (30, t3)]);
        let buffer = 0x7777u32;
        e.call_log = Some(vec![]);
        e.call(0x009a59e0, &args![map, buffer]);
        assert_eq!(
            calls_to(&e, 0x0086_5db0),
            [[buffer, 10, 0], [buffer, 30, 0]]
        );
        assert_eq!(calls_to(&e, 0x0086_5ff0), [[buffer, 2, 0x77]]);
        assert_eq!(calls_to(&e, 0x0086_5e50).len(), 4);
        let order = addresses(&e);
        assert_eq!(order[1], 0x0086_5f20);
        assert_eq!(*order.last().unwrap(), 0x0086_5ff0);
    }

    #[test]
    fn load_game_adds_threats_with_a_form_id_and_frees_the_others() {
        let mut e = engine();
        noop(
            &mut e,
            &[
                TIME_STAMP_INIT,
                SMART_POINTER_INIT,
                VECTOR_INIT,
                SMART_POINTER_DESTROY,
                OPERATOR_DELETE,
                0x0086_4980,
                0x009a_5ff0,
            ],
        );
        clock(&mut e, NOW);
        e.register(POINTER_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0086_4a60, |_, _| 2u32.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let ids = Rc::new(RefCell::new(VecDeque::from([0x100u32, 0])));
        e.register_double(0x0086_48e0, move |e, a| {
            let id = ids.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[1], id);
            Ret::default()
        });
        e.register(0x0053_7e90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let (map, _) = map_with(&mut e, &[]);
        let buffer = 0x7777u32;
        e.call_log = Some(vec![]);
        e.call(0x009a5a70, &args![map, buffer]);
        let items = map_items(&e, map);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].0, 0x100);
        let created = calls_to(&e, OPERATOR_NEW);
        assert_eq!(created, [[0x30], [0x30]]);
        // The first threat holds its form ID; the second was freed.
        assert_eq!(e.mem.u32(items[0].1), 0x100);
        let freed = calls_to(&e, OPERATOR_DELETE);
        assert_eq!(freed.len(), 1);
        assert_ne!(freed[0][0], items[0].1);
        assert_eq!(calls_to(&e, 0x0086_4980).len(), 4);
    }

    #[test]
    fn init_load_game_keeps_resolved_threats_and_removes_the_others() {
        let mut e = engine();
        noop(
            &mut e,
            &[SET_TARGETED, SMART_POINTER_DESTROY, OPERATOR_DELETE],
        );
        e.register(POINTER_VALUE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0048_39c0, |_, a| a[0].into_ret());
        // The form with ID 0x100 casts to 0x4000; 0x200 does not cast.
        e.register(0x00ec_43fb, |_, a| {
            if a[0] == 0x100 {
                0x4000u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let (kept, dropped) = (threat_ending(&mut e, 0.0), threat_ending(&mut e, 0.0));
        e.mem.set_u32(kept, 0x100);
        e.mem.set_u32(dropped, 0x200);
        let (map, _) = map_with(&mut e, &[(1, kept), (2, dropped)]);
        walk(&mut e, &[(1, kept), (2, dropped)]);
        e.call_log = Some(vec![]);
        e.call(0x009a5c40, &args![map, 0x1u32]);
        assert_eq!(map_items(&e, map), [(1, kept)]);
        assert_eq!(e.mem.u32(kept), 0x4000);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), [[dropped]]);
        assert_eq!(calls_to(&e, SET_TARGETED), [[0x4000, 1]]);
    }

    #[test]
    fn debug_pass_calls_the_hook_on_every_non_null_threat() {
        let mut e = engine();
        noop(&mut e, &[THREAT_SET_DEBUG]);
        let (map, _) = map_with(&mut e, &[]);
        walk(&mut e, &[(1, 0x1000), (2, 0), (3, 0x3000)]);
        e.call_log = Some(vec![]);
        e.call(0x009a5cd0, &args![map, 1u32]);
        assert_eq!(calls_to(&e, THREAT_SET_DEBUG), [[0x1000, 1], [0x3000, 1]]);
    }
}
