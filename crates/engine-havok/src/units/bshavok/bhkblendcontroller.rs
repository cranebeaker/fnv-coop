//! `BSHavok/bhkblendcontroller.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBlendController` (Xbox PDB, 0x5c bytes on PC) is the `NiTimeController` that blends a
//! ragdoll's hierarchy and velocity gains over time. Its keys (`BLENDKEY`, 12 bytes: a time and
//! the two gains) live in an `NiTLargeArray<BLENDKEY>` embedded at `+0x38` (vtable, base `+0x3c`,
//! capacity `+0x40`, size `+0x44`, used count `+0x48`, grow-by `+0x4c`). The empty key is the
//! three floats `(-1.0, 0.0, 0.0)`: `-1.0` is read from `0x01012054`, and the zero is the x87
//! quotient `0.0 / 100.0` (divisor at `0x01017a40`), as the exe computes it.
//!
//! Floats: x87 `FLD`/`FSTP` copies go through [`x87_quiet_bits`]; comparisons follow the exe's
//! flag tests (`FCOM`, `TEST AH,0x41` / `0x44` / `5`) including NaN.
//!
//! The exception-unwinding frames (`FS:[0]`) and the `/GS` cookie check (`00ec408c`) of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiTLargeArray;

/// `-1.0f` (the empty key's time).
const MINUS_ONE: u32 = 0x0101_2054;
/// `100.0` (`double`): the divisor of the zero the exe builds as `0.0 / 100.0`.
const ZERO_DIVISOR: u32 = 0x0101_7a40;
/// `FLT_MAX` and `-FLT_MAX` as `double` (`0x47efffffe0000000`, `0xc7efffffe0000000`).
const FLOAT_MAX: u32 = 0x0102_31b0;
const NEGATIVE_FLOAT_MAX: u32 = 0x0102_41b0;
/// `BLENDKEY` comparisons (`this` = first key, one stack argument = second key): `00c9a620`
/// answers "all three floats equal", `00c9a660` answers its negation.
const KEY_EQUAL: u32 = 0x00c9_a620;
const KEY_NOT_EQUAL: u32 = 0x00c9_a660;
/// `BLENDKEY` constructor (`00c9a6c0`, no function in the exe's analysis; sets `(-1, 0, 0)`),
/// passed to the vector constructor iterator.
const KEY_CONSTRUCTOR: u32 = 0x00c9_a6c0;
/// `` `vector constructor iterator' `` (`00401050`, stdcall: array, element size, count,
/// constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `(1 - t) * a + t * b` (`00a42050`, cdecl: `t`, `a`, `b`; result in ST0).
const LERP: u32 = 0x00a4_2050;
/// The game heap: allocate (`00aa1070`, cdecl size), free (`00aa10f0`, cdecl pointer),
/// allocate with the new operator (`00aa13e0`, cdecl size).
const HEAP_ALLOCATE: u32 = 0x00aa_1070;
const HEAP_FREE: u32 = 0x00aa_10f0;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiTimeController` (`00a6d220` constructor, `00a6d290` destructor body, `00a6cc80` IsEqual,
/// `00a6cfe0` copy of the members, `00a6d5e0` LoadBinary, `00a6d730` SaveBinary, `00a6d830`
/// GetViewerStrings).
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
/// `NiObjectNET::GetController` (`00a5c570`, one stack argument: the RTTI) and
/// `RemoveController` (`00a5c480`).
const GET_CONTROLLER: u32 = 0x00a5_c570;
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;
/// Cast helper (`00653290`, thiscall on the object, one stack argument: the RTTI): the object
/// when it is of that class, else 0.
const IS_KIND_OF: u32 = 0x0065_3290;
/// `bhkBlendController::ms_RTTI` and the `bhkBlendCollisionObject` RTTI.
const BLEND_CONTROLLER_RTTI: u32 = 0x0126_81f0;
const BLEND_COLLISION_OBJECT_RTTI: u32 = 0x0126_7e64;
/// The RTTI of the force controller class `ClearForceController` removes (`012683f4`), and the
/// RTTI `00c9b470` and `00c9bdf0` look up on a node's collision data (`012043f8`).
const FORCE_CONTROLLER_RTTI: u32 = 0x0126_83f4;
const PHYSICS_OBJECT_RTTI: u32 = 0x0120_43f8;
/// `NiNode::ms_RTTI`.
const NODE_RTTI: u32 = 0x011f_4428;
/// `bhkBlendCollisionObject::SynchronizeMotionType` (Xbox PDB).
const SYNCHRONIZE_MOTION_TYPE: u32 = 0x00c8_1a50;
/// Called with the motion type flags of the world object (`00626cb0`, `thiscall` on the
/// world object, one stack argument).
const SET_MOTION_FLAGS: u32 = 0x0062_6cb0;
/// Reads the flags word of a physics object into the stack argument and answers its address
/// (`0043b4f0`, thiscall, one stack argument).
const READ_OBJECT_FLAGS: u32 = 0x0043_b4f0;
/// `NiPoint3` scaled by a float (`0045bb20`, thiscall on the vector: result vector, factor).
const POINT_SCALE: u32 = 0x0045_bb20;
/// Copies a 3-vector into a 4-lane vector (`004a3e00`, cdecl: result, source; answers result).
const VECTOR_FROM_POINT: u32 = 0x004a_3e00;
/// Applies a force to a vector (`00cb94a0`, cdecl: object, vector, float).
const APPLY_FORCE_VECTOR: u32 = 0x00cb_94a0;
/// `hkpRigidBody::applyForce` (Xbox PDB, thiscall: float, vector).
const RIGID_BODY_APPLY_FORCE: u32 = 0x0063_1100;
/// Two methods on a physics object, each with the global at `01267e30` as argument.
const PHYSICS_REFRESH_A: u32 = 0x0056_1690;
const PHYSICS_REFRESH_B: u32 = 0x0056_17c0;
const IDENTITY_TRANSFORM: u32 = 0x0126_7e30;
/// Gain tables indexed by a five-bit class number (each entry two floats unless noted).
const HIT_GAINS: u32 = 0x011b_0f90;
const HIT_GAINS_ALTERNATE: u32 = 0x011b_1090;
const GET_UP_GAINS: u32 = 0x011b_1190;
const GET_UP_TARGETS: u32 = 0x011b_06d8;
const KNOCK_DOWN_GAINS: u32 = 0x011b_1210;
const HIT_KEY_TIME_A: u32 = 0x0103_0ff4;
const HIT_KEY_TIME_B: u32 = 0x0101_6248;
const START_TIME: u32 = 0x0101_5f5c;
const PASS_OUT_FORCE_SCALE: u32 = 0x011b_0f88;
const PASS_OUT_FORCE_UP: u32 = 0x011b_0f8c;
const PASS_OUT_TIME: u32 = 0x011a_fe64;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;
/// `"iKeys"`, `"Key%d: %.4f, %.4f, %.4f"`, the printf routine (`00406d00`), the string
/// allocator (`00aa3e40` on the memory manager at `011f6238`) and `_strcpy_s` (`00ec65a6`).
const KEYS_LABEL: u32 = 0x010c_6be0;
const KEY_FORMAT: u32 = 0x010c_6bc8;
const SPRINTF: u32 = 0x0040_6d00;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const STRCPY_S: u32 = 0x00ec_65a6;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and its overload with an integer
/// (`00a7d510`, cdecl: name, value); the string array's resize (`0096ad30`, one argument) and
/// store (`0096ae90`, index and pointer to the value).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_INT: u32 = 0x00a7_d510;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;

layout! {
    /// `bhkBlendController` (Xbox PDB), 0x5c bytes: an `NiTimeController` (0x34 bytes) followed
    /// by the blend keys.
    pub struct BhkBlendController: 0x5c {
        /// `m_uFlags` (Xbox PDB, `NiTimeController`).
        0x08 m_uFlags: u16,
        /// `m_fFrequency` (Xbox PDB).
        0x0C m_fFrequency: f32,
        /// `m_fPhase` (Xbox PDB).
        0x10 m_fPhase: f32,
        /// `m_fLoKeyTime` (Xbox PDB).
        0x14 m_fLoKeyTime: f32,
        /// `m_fHiKeyTime` (Xbox PDB).
        0x18 m_fHiKeyTime: f32,
        /// `m_fLastTime` (Xbox PDB).
        0x20 m_fLastTime: f32,
        /// `m_pkTarget` (Xbox PDB): `NiObjectNET*`.
        0x2C m_pkTarget: Ptr,
        /// `iLastIndex` (Xbox PDB): the key index the last lookup ended on.
        0x34 iLastIndex: u32,
        /// `pBlendKeyA` (Xbox PDB): `NiTLargePrimitiveArray<BLENDKEY>`.
        0x38 pBlendKeyA: Inline<NiTLargeArray>,
        /// `fOriginalGain` (Xbox PDB).
        0x50 fOriginalGain: f32,
        /// `fOriginalVelGain` (Xbox PDB).
        0x54 fOriginalVelGain: f32,
        /// `iPriority` (Xbox PDB).
        0x58 iPriority: u32,
    }
}

/// `bhkBlendCollisionObject::fHeirGain` / `fVelGain` (Xbox PDB), the two gains the controller
/// writes.
const COLLISION_OBJECT_HEIR_GAIN: u32 = 0x18;
const COLLISION_OBJECT_VEL_GAIN: u32 = 0x1c;
/// `BLENDKEY` size.
const KEY_SIZE: u32 = 12;

/// The three words of the empty key: `-1.0` through an x87 load and store, and `0.0 / 100.0`
/// rounded to a float.
fn empty_key_bits(e: &mut Engine) -> [u32; 3] {
    let marker = x87_quiet_bits(e.mem.u32(MINUS_ONE));
    let zero = ((0.0f64 / e.global::<f64>(ZERO_DIVISOR)) as f32).to_bits();
    [marker, zero, zero]
}

fn write_key(e: &mut Engine, slot: u32, key: [u32; 3]) {
    for (lane, word) in key.into_iter().enumerate() {
        e.mem.set_u32(slot + 4 * lane as u32, word);
    }
}

/// `FLD dword` / `FSTP dword` copy of one float.
fn copy_float(e: &mut Engine, destination: u32, source: u32) {
    let bits = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(destination, bits);
}

/// `FLD dword` / `FSTP dword` copy of a whole key.
fn copy_key(e: &mut Engine, destination: u32, source: u32) {
    for lane in 0..3 {
        copy_float(e, destination + 4 * lane, source + 4 * lane);
    }
}

/// A float read through an x87 load (a signalling NaN comes back quiet).
fn load_float(e: &Engine, address: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(address)))
}

/// Sets the per-thread word `TLS + 0x2b4` to 0x10, answering the thread block and the old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    (thread, saved)
}

fn leave_scope(e: &mut Engine, scope: (u32, u32)) {
    e.mem.set_u32(scope.0 + SCOPE_WORD, scope.1);
}

// Translated from 00c9a6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<BLENDKEY,NiTMallocInterface<BLENDKEY>>::RemoveAll` (Xbox PDB): sets every
/// key up to the size to the empty key, then clears the size and the used count.
pub fn blend_key_array_remove_all(e: &mut Engine, this: Ptr<NiTLargeArray>) {
    let size = e.get(this, NiTLargeArray::m_uiSize);
    if size != 0 {
        let empty = empty_key_bits(e);
        let mut index = 0u32;
        loop {
            let base = e.get(this, NiTLargeArray::m_pBase);
            write_key(e, base.wrapping_add(index * KEY_SIZE), empty);
            index += 1;
            if index >= e.get(this, NiTLargeArray::m_uiSize) {
                break;
            }
        }
    }
    e.set(this, NiTLargeArray::m_uiESize, 0);
    e.set(this, NiTLargeArray::m_uiSize, 0);
}

// Translated from 00c9a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<BLENDKEY,NiTMallocInterface<BLENDKEY>>::SetAt` (unconfirmed name): stores the
/// key at `value` at `index`. Past the size, the size becomes `index + 1` and the used count
/// goes up when the key is not the empty key; inside it, the used count follows the slot
/// changing between empty and not empty (the two key comparisons `00c9a660` / `00c9a620`).
pub fn blend_key_array_set_at(e: &mut Engine, this: Ptr<NiTLargeArray>, index: u32, value: Ptr) {
    // The exe's frame (`SUB ESP,0x10`, three pushes): 0x1c bytes, the empty key at +0x10.
    e.with_stack(0x1c, |e, frame| {
        set_at_in_frame(e, this, index, value, frame.addr() + 0x10)
    });
}

/// [`blend_key_array_set_at`] in its frame: `temporary` is the empty key local.
fn set_at_in_frame(
    e: &mut Engine,
    this: Ptr<NiTLargeArray>,
    index: u32,
    value: Ptr,
    temporary: u32,
) {
    let empty = empty_key_bits(e);
    write_key(e, temporary, empty);
    if index >= e.get(this, NiTLargeArray::m_uiSize) {
        e.set(this, NiTLargeArray::m_uiSize, index.wrapping_add(1));
        if e.call(KEY_NOT_EQUAL, &args![value, temporary]).bool() {
            let used = e.get(this, NiTLargeArray::m_uiESize);
            e.set(this, NiTLargeArray::m_uiESize, used.wrapping_add(1));
        }
    } else {
        let set = e.call(KEY_NOT_EQUAL, &args![value, temporary]).bool();
        let base = e.get(this, NiTLargeArray::m_pBase);
        let slot = base.wrapping_add(index.wrapping_mul(KEY_SIZE));
        let used = e.get(this, NiTLargeArray::m_uiESize);
        if !set {
            if e.call(KEY_NOT_EQUAL, &args![slot, temporary]).bool() {
                e.set(this, NiTLargeArray::m_uiESize, used.wrapping_sub(1));
            }
        } else if e.call(KEY_EQUAL, &args![slot, temporary]).bool() {
            e.set(this, NiTLargeArray::m_uiESize, used.wrapping_add(1));
        }
    }
    let base = e.get(this, NiTLargeArray::m_pBase);
    let slot = base.wrapping_add(index.wrapping_mul(KEY_SIZE));
    copy_key(e, slot, value.addr());
}

// Translated from 00c9a820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::ClearKeys` (Xbox PDB): empties the key array, sets both original gains
/// to `-1.0` and the priority to 0.
pub fn bhk_blend_controller_clear_keys(e: &mut Engine, this: Ptr<BhkBlendController>) {
    blend_key_array_remove_all(e, this.at(BhkBlendController::pBlendKeyA));
    let marker = f32::from_bits(x87_quiet_bits(e.mem.u32(MINUS_ONE)));
    e.set(this, BhkBlendController::fOriginalVelGain, marker);
    e.set(this, BhkBlendController::iPriority, 0);
    e.set(this, BhkBlendController::fOriginalGain, marker);
}

// Translated from 00c9a840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::IsEqual` (Xbox PDB): the base `NiTimeController::IsEqual` (`00a6cc80`)
/// and the same number of keys, every float of every key equal (NaN is never equal).
pub fn bhk_blend_controller_is_equal(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    other: Ptr<BhkBlendController>,
) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let count = e.mem.u32(this.addr() + 0x48);
    if count != e.mem.u32(other.addr() + 0x48) {
        return false;
    }
    for index in 0..count {
        let mine = e.mem.u32(this.addr() + 0x3c) + index * KEY_SIZE;
        let theirs = e.mem.u32(other.addr() + 0x3c) + index * KEY_SIZE;
        for lane in 0..3 {
            let a = f32::from_bits(e.mem.u32(mine + 4 * lane));
            let b = f32::from_bits(e.mem.u32(theirs + 4 * lane));
            if a != b {
                return false;
            }
        }
    }
    true
}

// Translated from 00c9a8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the two gains at `time` (unconfirmed name; `Update` calls it with the scaled
/// time): answers false when there are no keys; with one key, that key's two gains; else it
/// moves `iLastIndex` (`+0x34`) to the last key whose time is below `time` (starting over from
/// key 0 when the remembered key is later than `time`), and writes
/// `lerp(t, gains of that key, gains of the next key)` with
/// `t = (time - lower) / (upper - lower)` to `first_gain` and `second_gain`, or the lower
/// key's gains when the two key times are equal. The exe scans four keys per step; the result
/// is the same as scanning one by one.
pub fn fn_00c9a8c0(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    time: f32,
    first_gain: Ptr,
    second_gain: Ptr,
) -> bool {
    let me = this.addr();
    let count = e.mem.u32(me + 0x48);
    if count == 0 {
        return false;
    }
    if count == 1 {
        let keys = e.mem.u32(me + 0x3c);
        copy_float(e, first_gain.addr(), keys + 4);
        let keys = e.mem.u32(me + 0x3c);
        copy_float(e, second_gain.addr(), keys + 8);
        return true;
    }
    let keys = e.mem.u32(me + 0x3c);
    let last = count - 1;
    let mut current = e.mem.u32(me + 0x34);
    let mut lower = load_float(e, keys.wrapping_add(current.wrapping_mul(KEY_SIZE)));
    if lower > time {
        e.mem.set_u32(me + 0x34, 0);
        current = 0;
        lower = load_float(e, keys);
    }
    let mut index = current.wrapping_add(1);
    let mut upper = lower;
    let mut found = false;
    let scan = |e: &mut Engine, index: &mut u32, lower: &mut f32, upper: &mut f32| -> bool {
        let key_time = load_float(e, keys.wrapping_add(index.wrapping_mul(KEY_SIZE)));
        if time <= key_time {
            *upper = key_time;
            return true;
        }
        let passed = e.mem.u32(me + 0x34);
        e.mem.set_u32(me + 0x34, passed.wrapping_add(1));
        *lower = key_time;
        *index = index.wrapping_add(1);
        false
    };
    if (last.wrapping_sub(index).wrapping_add(1) as i32) >= 4 {
        'blocks: loop {
            for _ in 0..4 {
                if scan(e, &mut index, &mut lower, &mut upper) {
                    found = true;
                    break 'blocks;
                }
            }
            if index > last.wrapping_sub(3) {
                break;
            }
        }
    }
    if !found {
        while index <= last {
            if scan(e, &mut index, &mut lower, &mut upper) {
                found = true;
                break;
            }
        }
    }
    if !found {
        upper = lower;
    }
    let keys = e.mem.u32(me + 0x3c);
    let current = e.mem.u32(me + 0x34);
    let lower_key = keys.wrapping_add(current.wrapping_mul(KEY_SIZE));
    let first_lower = load_float(e, lower_key + 4);
    let second_lower = load_float(e, lower_key + 8);
    let difference = f64::from(upper) - f64::from(lower);
    if difference == 0.0 {
        e.mem.set_f32(first_gain.addr(), first_lower);
        e.mem.set_f32(second_gain.addr(), second_lower);
        return true;
    }
    let fraction = ((f64::from(time) - f64::from(lower)) / difference) as f32;
    let upper_key = e
        .mem
        .u32(me + 0x3c)
        .wrapping_add(index.wrapping_mul(KEY_SIZE));
    let first_upper = load_float(e, upper_key + 4);
    let first = e
        .call(LERP, &args![fraction, first_lower, first_upper])
        .f32();
    e.mem.set_f32(first_gain.addr(), first);
    let upper_key = e
        .mem
        .u32(me + 0x3c)
        .wrapping_add(index.wrapping_mul(KEY_SIZE));
    let second_upper = load_float(e, upper_key + 8);
    let second = e
        .call(LERP, &args![fraction, second_lower, second_upper])
        .f32();
    e.mem.set_f32(second_gain.addr(), second);
    true
}

// Translated from 00c9ab10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the low and high key times (`+0x14`, `+0x18`) from the keys (unconfirmed name):
/// with no keys both become 0.0. Otherwise the low time becomes the first key's time when that
/// is below it or the low time is `FLT_MAX`, and the high time becomes the last key's time when
/// that is below it or the high time is `-FLT_MAX` (comparisons as the exe's `FCOM` flags, NaN
/// included).
pub fn fn_00c9ab10(e: &mut Engine, this: Ptr<BhkBlendController>) {
    let me = this.addr();
    let mut low = 0.0f32;
    let mut high = 0.0f32;
    let count = e.mem.u32(me + 0x48);
    if count != 0 {
        let keys = e.mem.u32(me + 0x3c);
        let first = load_float(e, keys);
        low = load_float(e, me + 0x14);
        high = load_float(e, me + 0x18);
        let float_max: f64 = e.global(FLOAT_MAX);
        if first < low || f64::from(low) == float_max {
            low = first;
        }
        let last = load_float(
            e,
            keys.wrapping_add(count.wrapping_mul(KEY_SIZE))
                .wrapping_sub(KEY_SIZE),
        );
        let negative_float_max: f64 = e.global(NEGATIVE_FLOAT_MAX);
        if last < high || f64::from(high) == negative_float_max {
            high = last;
        }
    }
    e.set(this, BhkBlendController::m_fLoKeyTime, low);
    e.set(this, BhkBlendController::m_fHiKeyTime, high);
}

// Translated from 00c9abc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::RestoreOriginalGains` (Xbox PDB): when bit 8 of the flags is set and
/// the saved gain `+0x50` is not below zero (NaN counts as below), puts the two saved gains
/// back into the collision object (`+0x18`, `+0x1c`) and marks them `-1.0`.
pub fn bhk_blend_controller_restore_original_gains(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    collision_object: Ptr,
) {
    let me = this.addr();
    if e.mem.u8(me + 9) & 1 == 0 {
        return;
    }
    let gain = f32::from_bits(e.mem.u32(me + 0x50));
    let at_least_zero = 0.0 <= gain;
    if !at_least_zero {
        return;
    }
    copy_float(
        e,
        collision_object.addr() + COLLISION_OBJECT_HEIR_GAIN,
        me + 0x50,
    );
    copy_float(
        e,
        collision_object.addr() + COLLISION_OBJECT_VEL_GAIN,
        me + 0x54,
    );
    let marker = f32::from_bits(x87_quiet_bits(e.mem.u32(MINUS_ONE)));
    e.mem.set_f32(me + 0x54, marker);
    e.mem.set_f32(me + 0x50, marker);
}

// Translated from 00c9ac00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<BLENDKEY,NiTMallocInterface<BLENDKEY>>::SetSize` (unconfirmed name): does
/// nothing when `new_size` is the capacity. Shrinking first resets the keys from `new_size` to
/// the old size that are not the empty key (counting them out of the used count) and cuts the
/// size. Then the capacity becomes `new_size`; a new block (`00aa1070`) takes the surviving
/// keys (through x87 copies) and the rest is filled with the empty key; zero leaves no block.
/// The old block is freed (`00aa10f0`).
pub fn blend_key_array_set_size(e: &mut Engine, this: Ptr<NiTLargeArray>, new_size: u32) {
    if new_size == e.get(this, NiTLargeArray::m_uiMaxSize) {
        return;
    }
    let empty = empty_key_bits(e);
    if new_size < e.get(this, NiTLargeArray::m_uiSize) {
        let marker = f32::from_bits(empty[0]);
        let zero = f32::from_bits(empty[1]);
        let mut index = new_size;
        loop {
            let base = e.get(this, NiTLargeArray::m_pBase);
            let slot = base.wrapping_add(index.wrapping_mul(KEY_SIZE));
            let is_empty = f32::from_bits(e.mem.u32(slot)) == marker
                && f32::from_bits(e.mem.u32(slot + 4)) == zero
                && f32::from_bits(e.mem.u32(slot + 8)) == zero;
            if !is_empty {
                write_key(e, slot, empty);
                let used = e.get(this, NiTLargeArray::m_uiESize);
                e.set(this, NiTLargeArray::m_uiESize, used.wrapping_sub(1));
            }
            index += 1;
            if index >= e.get(this, NiTLargeArray::m_uiSize) {
                break;
            }
        }
        e.set(this, NiTLargeArray::m_uiSize, new_size);
    }
    let old_base = e.get(this, NiTLargeArray::m_pBase);
    e.set(this, NiTLargeArray::m_uiMaxSize, new_size);
    if new_size == 0 {
        e.set(this, NiTLargeArray::m_pBase, 0);
    } else {
        let new_base = e
            .call(HEAP_ALLOCATE, &args![new_size.wrapping_mul(KEY_SIZE)])
            .u32();
        e.set(this, NiTLargeArray::m_pBase, new_base);
        let size = e.get(this, NiTLargeArray::m_uiSize);
        for index in 0..size {
            let offset = index * KEY_SIZE;
            let destination = e.get(this, NiTLargeArray::m_pBase) + offset;
            copy_key(e, destination, old_base + offset);
        }
        let mut index = e.get(this, NiTLargeArray::m_uiSize);
        if index < e.get(this, NiTLargeArray::m_uiMaxSize) {
            loop {
                let base = e.get(this, NiTLargeArray::m_pBase);
                write_key(e, base.wrapping_add(index * KEY_SIZE), empty);
                index += 1;
                if index >= e.get(this, NiTLargeArray::m_uiMaxSize) {
                    break;
                }
            }
        }
    }
    e.call(HEAP_FREE, &args![old_base]);
}

// Translated from 00c9ad60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ad60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendController` destructor body (RTTI of vtable `010c6b14`): sets the class vtable and
/// the array's vtable (`010c6b04`), frees the key block (`00aa10f0`) and runs the
/// `NiTimeController` destructor body (`00a6d290`).
pub fn fn_00c9ad60(e: &mut Engine, this: Ptr<BhkBlendController>) {
    let me = this.addr();
    e.mem.set_u32(me, 0x010c_6b14);
    let base = e.mem.u32(me + 0x3c);
    e.mem.set_u32(me + 0x38, 0x010c_6b04);
    e.call(HEAP_FREE, &args![base]);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
}

// Translated from 00c9ade0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ade0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendController::ClearForceController` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10`
/// scope, when bit 15 of the flags byte at `+8` is set and the target (`+0x2c`) has a
/// controller of the force controller class, removes it (`00a5c480`) and answers true.
pub fn bhk_blend_controller_clear_force_controller(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
) -> bool {
    let scope = enter_scope(e);
    let mut removed = false;
    if (e.mem.u8(this.addr() + 8) >> 7) & 1 != 0 {
        let target = e.mem.u32(this.addr() + 0x2c);
        let controller = e
            .call(GET_CONTROLLER, &args![target, FORCE_CONTROLLER_RTTI])
            .u32();
        if controller != 0 {
            e.call(REMOVE_CONTROLLER, &args![target, controller]);
            removed = true;
        }
    }
    leave_scope(e, scope);
    removed
}

// Translated from 00c9aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9aeb0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendController::bhkBlendController` (Xbox PDB): the `NiTimeController` constructor
/// (`00a6d220`), the class vtable `010c6b14`, the key array empty (vtable `010c6b0c`, grow-by 1)
/// and both original gains `-1.0`. Answers `this`.
pub fn bhk_blend_controller_construct(e: &mut Engine, this: Ptr<BhkBlendController>) -> Ptr {
    let me = this.addr();
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![this]);
    let marker = f32::from_bits(x87_quiet_bits(e.mem.u32(MINUS_ONE)));
    e.mem.set_u32(me, 0x010c_6b14);
    e.mem.set_u32(me + 0x40, 0);
    e.mem.set_u32(me + 0x44, 0);
    e.mem.set_u32(me + 0x48, 0);
    e.mem.set_u32(me + 0x3c, 0);
    e.mem.set_u32(me + 0x4c, 1);
    e.mem.set_u32(me + 0x38, 0x010c_6b0c);
    e.mem.set_f32(me + 0x50, marker);
    e.mem.set_u32(me + 0x34, 0);
    e.mem.set_f32(me + 0x54, marker);
    e.mem.set_u32(me + 0x58, 0);
    this.cast()
}

// Translated from 00c9af20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c9ad60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x5c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_blend_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c9_ad60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x5cu32]);
    }
    this
}

// Translated from 00c9add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_81f0.
pub fn bhk_blend_controller_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_81f0
}

// Translated from 00c9af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::SaveBinary` (Xbox PDB): the base `NiTimeController::SaveBinary`
/// (`00a6d730`), then through the stream's writer (`stream + 0x250` is the output object,
/// whose function pointer at `+0xc` is called as `(output, buffer, size, &size, 1)` and whose
/// position `+4` advances by the answer) the key count (4 bytes) and each key (12 bytes). When
/// the byte at `stream + 0x1e4` is not 1 the three words of each key are byte-swapped; the
/// count never is.
pub fn bhk_blend_controller_save_binary(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    let count = e.mem.u32(this.addr() + 0x48);
    // The exe's frame (`SUB ESP,0x14`, three pushes): 0x20 bytes, `this` kept at +0x10 (00c9af4d),
    // the size word at +0xc (00c9af61) and one key at +0x14 (00c9afe7). The count goes into the
    // `stream` argument's own slot (00c9af6d), +0x24 past the return address at +0x20.
    e.with_stack(0x28, |e, frame| {
        let frame = frame.addr();
        let count_slot = frame + 0x24;
        let size_slot = frame + 0xc;
        let key = frame + 0x14;
        e.mem.set_u32(frame + 0x24, stream.addr());
        e.mem.set_u32(frame + 0x10, this.addr());
        e.mem.set_u32(count_slot, count);
        let output = e.mem.u32(stream.addr() + 0x250);
        e.mem.set_u32(size_slot, 4);
        let writer = e.mem.u32(output + 0xc);
        let written = e
            .call(writer, &args![output, count_slot, 4u32, size_slot, 1u32])
            .u32();
        let position = e.mem.u32(output + 4);
        e.mem.set_u32(output + 4, position.wrapping_add(written));
        let mut index = 0u32;
        if count != 0 {
            loop {
                let source = e.mem.u32(this.addr() + 0x3c) + index * KEY_SIZE;
                copy_key(e, key, source);
                if e.mem.u8(stream.addr() + 0x1e4) != 1 {
                    for lane in 0..3 {
                        let word = e.mem.u32(key + 4 * lane);
                        e.mem.set_u32(key + 4 * lane, word.swap_bytes());
                    }
                }
                let output = e.mem.u32(stream.addr() + 0x250);
                let writer = e.mem.u32(output + 0xc);
                e.mem.set_u32(size_slot, 0xc);
                let written = e
                    .call(writer, &args![output, key, 0xcu32, size_slot, 1u32])
                    .u32();
                let position = e.mem.u32(output + 4);
                e.mem.set_u32(output + 4, position.wrapping_add(written));
                index += 1;
                if index >= count {
                    break;
                }
            }
        }
    });
}

// Translated from 00c9b020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9b020 - the FS:[0] exception-unwinding frame is not modeled.
/// Stream factory of the class (unconfirmed name): allocates 0x5c bytes (`00aa13e0`) and
/// constructs a `bhkBlendController` in them; answers the object, or 0 when the allocation
/// failed.
pub fn fn_00c9b020(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    bhk_blend_controller_construct(e, Ptr::new(block))
}

// Translated from 00c9b090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::Update` (Xbox PDB): `update_data` points to the new time. Needs a
/// target (`+0x2c`), bit 3 of the flags and keys. When the target's collision object
/// (`target + 0x1c`) is a `bhkBlendCollisionObject`, the scaled time (virtual `+0xa4`) selects
/// the two gains (`00c9a8c0`); if there are any, a negative saved gain is first filled from the
/// object's gains, and the gains are written to the object. When the scaled time equals the
/// high key time and the flags masked with 6 are 4, the keys are cleared (when bit 6 is set),
/// the force controller removed, the original gains restored and the controller stopped
/// (virtual `+0x90`). Without a blend collision object, the time is stored at `+0x20`.
pub fn bhk_blend_controller_update(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    update_data: Ptr,
) {
    let me = this.addr();
    let target = e.mem.u32(me + 0x2c);
    if target == 0 {
        return;
    }
    if (e.mem.u8(me + 8) >> 3) & 1 == 0 {
        return;
    }
    if e.mem.u32(me + 0x48) == 0 {
        return;
    }
    let collision_object = e.mem.u32(target + 0x1c);
    let mut object = 0;
    if collision_object != 0 {
        object = e
            .call(
                IS_KIND_OF,
                &args![collision_object, BLEND_COLLISION_OBJECT_RTTI],
            )
            .u32();
    }
    if object == 0 {
        copy_float(e, me + 0x20, update_data.addr());
        return;
    }
    let time = load_float(e, update_data.addr());
    let scaled = e.vcall(me, 0xa4, &args![time]).f32();
    e.with_stack(8, |e, gains| {
        let found = fn_00c9a8c0(e, this, scaled, gains, gains.byte_add(4));
        if found {
            if f32::from_bits(e.mem.u32(me + 0x50)) < 0.0 {
                copy_float(e, me + 0x50, object + COLLISION_OBJECT_HEIR_GAIN);
                copy_float(e, me + 0x54, object + COLLISION_OBJECT_VEL_GAIN);
            }
            copy_float(e, object + COLLISION_OBJECT_HEIR_GAIN, gains.addr());
            copy_float(e, object + COLLISION_OBJECT_VEL_GAIN, gains.addr() + 4);
        }
    });
    let high = f32::from_bits(e.mem.u32(me + 0x18));
    if scaled != high {
        return;
    }
    let flags = e.mem.u16(me + 8);
    if flags & 6 != 4 {
        return;
    }
    if (flags >> 6) & 1 != 0 {
        bhk_blend_controller_clear_keys(e, this);
    }
    bhk_blend_controller_clear_force_controller(e, this);
    bhk_blend_controller_restore_original_gains(e, this, Ptr::new(object));
    e.vcall(me, 0x90, &args![]);
}

// Translated from 00c9b1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9b1a0 - the FS:[0] exception-unwinding frame is not modeled.
/// Appends the key at `key` (unconfirmed name): inside the `TLS + 0x2b4 = 0x10` scope, grows
/// the array by its grow-by (`+0x4c`) when the size has reached the capacity, stores the key
/// at the old size (`00c9a730`), resets `iLastIndex` and refreshes the key times (`00c9ab10`).
pub fn fn_00c9b1a0(e: &mut Engine, this: Ptr<BhkBlendController>, key: Ptr) {
    let scope = enter_scope(e);
    let array = this.at(BhkBlendController::pBlendKeyA);
    let size = e.get(array, NiTLargeArray::m_uiSize);
    if size >= e.get(array, NiTLargeArray::m_uiMaxSize) {
        let grow_by = e.get(array, NiTLargeArray::m_uiGrowBy);
        blend_key_array_set_size(e, array, grow_by.wrapping_add(size));
    }
    blend_key_array_set_at(e, array, size, key);
    e.set(this, BhkBlendController::iLastIndex, 0);
    fn_00c9ab10(e, this);
    leave_scope(e, scope);
}

/// The children of a node: virtual `+0xc` (`IsNode`) answers the node, whose child pointers
/// are at `+0xa0` (the count is the 16-bit word at `+0xa6`, read again on each step).
fn for_each_child(e: &mut Engine, object: u32, mut visit: impl FnMut(&mut Engine, u32)) {
    let node = e.vcall(object, 0xc, &args![]).u32();
    if node == 0 {
        return;
    }
    if e.mem.u16(node + 0xa6) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let children = e.mem.u32(node + 0xa0);
        let child = e.mem.u32(children + index * 4);
        visit(e, child);
        index += 1;
        if index >= u32::from(e.mem.u16(node + 0xa6)) {
            break;
        }
    }
}

/// The class number the gain tables are indexed by: the flags word `+0x2c` of the rigid body
/// behind the world object (`object + 0x10`, body at `+8`, flags at `body + 0x10 + 0x1c`), 0
/// when any link is missing; as `(flags >> 8) & 0x1f` after `adjust`.
fn collision_class_flags(e: &mut Engine, blend_object: u32) -> u32 {
    let world_object = e.mem.u32(blend_object + 0x10);
    if world_object == 0 {
        return 0;
    }
    let body = e.mem.u32(world_object + 8);
    if body == 0 {
        return 0;
    }
    let shifted = body.wrapping_add(0x10);
    if shifted == 0 {
        return 0;
    }
    e.mem.u32(shifted + 0x1c)
}

// Translated from 00c9b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendController::DoHit` (Xbox PDB), cdecl `(object, pass_through, quad)`: for an object
/// whose collision object is a `bhkBlendCollisionObject`, takes the gain pair of the object's
/// class number (table `011b0f90`, or `011b1090` when the low byte of `quad` is not 0), each
/// capped at the object's current gain, and, when the object has a blend controller of at most
/// priority 1, replaces its keys by three (times 0, `01030ff4`, `01016248`) and starts it.
/// Then recurses on the children (`IsNode`) with the other two arguments.
pub fn bhk_blend_controller_do_hit(e: &mut Engine, object: Ptr, pass_through: f32, quad: u32) {
    if object.addr() == 0 {
        return;
    }
    let collision_object = e.mem.u32(object.addr() + 0x1c);
    let mut blend_object = 0;
    if collision_object != 0 {
        blend_object = e
            .call(
                IS_KIND_OF,
                &args![collision_object, BLEND_COLLISION_OBJECT_RTTI],
            )
            .u32();
    }
    if blend_object != 0 {
        let class = (collision_class_flags(e, blend_object) >> 8) & 0x1f;
        let (heir_table, vel_table) = if quad as u8 != 0 {
            (HIT_GAINS_ALTERNATE, HIT_GAINS_ALTERNATE + 4)
        } else {
            (HIT_GAINS, HIT_GAINS + 4)
        };
        let table_a = load_float(e, heir_table + class * 8);
        let table_b = load_float(e, vel_table + class * 8);
        let heir_current = load_float(e, blend_object + COLLISION_OBJECT_HEIR_GAIN);
        let heir = if table_a > heir_current {
            heir_current
        } else {
            table_a
        };
        let vel_current = load_float(e, blend_object + COLLISION_OBJECT_VEL_GAIN);
        let vel = if table_b > vel_current {
            vel_current
        } else {
            table_b
        };
        let controller = e
            .call(GET_CONTROLLER, &args![object, BLEND_CONTROLLER_RTTI])
            .u32();
        if controller != 0 && e.mem.u32(controller + 0x58) <= 1 {
            let controller = Ptr::<BhkBlendController>::new(controller);
            bhk_blend_controller_clear_keys(e, controller);
            e.set(controller, BhkBlendController::iPriority, 1);
            if e.mem.u32(controller.addr() + 0x48) == 0 {
                e.with_stack(0x24, |e, keys| {
                    e.call(
                        VECTOR_CONSTRUCTOR_ITERATOR,
                        &args![keys, KEY_SIZE, 3u32, KEY_CONSTRUCTOR],
                    );
                    let key = |index: u32| keys.addr() + index * KEY_SIZE;
                    let zero = 0.0f32;
                    e.mem.set_f32(key(0) + 4, heir);
                    e.mem.set_f32(key(0) + 8, vel);
                    e.mem.set_f32(key(0), zero);
                    e.mem.set_f32(key(1) + 4, table_a);
                    e.mem.set_f32(key(1) + 8, table_b);
                    let time_a = load_float(e, HIT_KEY_TIME_A);
                    e.mem.set_f32(key(1), time_a);
                    e.mem.set_f32(key(2) + 4, 1.0);
                    e.mem.set_f32(key(2) + 8, 1.0);
                    let time_b = load_float(e, HIT_KEY_TIME_B);
                    e.mem.set_f32(key(2), time_b);
                    let array = controller.at(BhkBlendController::pBlendKeyA);
                    blend_key_array_set_size(e, array, 3);
                    for index in 0..3 {
                        fn_00c9b1a0(e, controller, Ptr::new(key(index)));
                    }
                });
                let me = controller.addr();
                e.mem.set_f32(me + 0x14, 0.0);
                let flags = e.mem.u16(me + 8);
                e.mem.set_f32(me + 0x18, 1.0);
                e.mem.set_f32(me + 0xc, 1.0);
                e.mem.set_f32(me + 0x10, 0.0);
                e.mem.set_u16(me + 8, (flags & 0xfff5) | 0x1c5);
                let start = load_float(e, START_TIME);
                e.vcall(me, 0x8c, &args![start]);
            }
        }
    }
    let pass = x87_quiet(pass_through);
    for_each_child(e, object.addr(), |e, child| {
        bhk_blend_controller_do_hit(e, Ptr::new(child), pass, quad);
    });
}

// Translated from 00c9b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Get-up blend (unconfirmed name), cdecl `(object, key_time)`: for an object whose collision
/// object is a `bhkBlendCollisionObject` (its world object's motion flags are first passed to
/// `00626cb0` with bits masked by `0xffffff88 | 8`), a blend controller of the object gets
/// keys `(0, 0, 0)` and `(table gain, table entry)` with the table `011b1190` giving the time
/// (only when it is not below zero, NaN counts as below) and priority 2, then starts. Without
/// such an object, a physics object (`012043f8`) whose flags say character-like (class 8 or
/// 0x1d with 0x1f00 bits equal to 0x1400, or neither) gets its virtual `+0xe4` called with 4.
/// Recurses on the children.
pub fn fn_00c9b470(e: &mut Engine, object: Ptr, key_time: f32) {
    if object.addr() == 0 {
        return;
    }
    let collision_object = e.mem.u32(object.addr() + 0x1c);
    let mut blend_object = 0;
    if collision_object != 0 {
        blend_object = e
            .call(
                IS_KIND_OF,
                &args![collision_object, BLEND_COLLISION_OBJECT_RTTI],
            )
            .u32();
    }
    if blend_object != 0 {
        let world_object = e.mem.u32(blend_object + 0x10);
        let mut flags = 0u32;
        if world_object != 0 {
            let body = e.mem.u32(world_object + 8);
            if body != 0 {
                let shifted = body.wrapping_add(0x10);
                if shifted != 0 {
                    flags = e.mem.u32(shifted + 0x1c);
                }
            }
            flags = (flags & 0xffff_ff88) | 8;
            // 00c9b4c3: `00626cb0` (RET 4) is a thiscall on the world object (ECX from
            // 00c9b4a0).
            e.call(SET_MOTION_FLAGS, &args![world_object, flags]);
        }
        let class = (flags >> 8) & 0x1f;
        let gain = load_float(e, GET_UP_GAINS + class * 4);
        let controller = e
            .call(GET_CONTROLLER, &args![object, BLEND_CONTROLLER_RTTI])
            .u32();
        if controller != 0 && 0.0 <= gain {
            let first = load_float(e, GET_UP_TARGETS + class * 8);
            let second = load_float(e, GET_UP_TARGETS + class * 8 + 4);
            let controller = Ptr::<BhkBlendController>::new(controller);
            bhk_blend_controller_clear_keys(e, controller);
            e.set(controller, BhkBlendController::iPriority, 2);
            if e.mem.u32(controller.addr() + 0x48) == 0 {
                e.with_stack(0x18, |e, keys| {
                    e.call(
                        VECTOR_CONSTRUCTOR_ITERATOR,
                        &args![keys, KEY_SIZE, 2u32, KEY_CONSTRUCTOR],
                    );
                    let key = |index: u32| keys.addr() + index * KEY_SIZE;
                    e.mem.set_f32(key(0) + 4, 0.0);
                    e.mem.set_f32(key(0) + 8, 0.0);
                    e.mem.set_f32(key(0), 0.0);
                    e.mem.set_f32(key(1) + 4, first);
                    e.mem.set_f32(key(1) + 8, second);
                    e.mem.set_f32(key(1), gain);
                    let array = controller.at(BhkBlendController::pBlendKeyA);
                    blend_key_array_set_size(e, array, 2);
                    for index in 0..2 {
                        fn_00c9b1a0(e, controller, Ptr::new(key(index)));
                    }
                });
                let me = controller.addr();
                e.mem.set_f32(me + 0x14, x87_quiet(key_time));
                let flags = e.mem.u16(me + 8);
                e.mem.set_f32(me + 0x18, gain);
                e.mem.set_f32(me + 0x10, 0.0);
                e.mem.set_f32(me + 0xc, 1.0);
                e.mem.set_u16(me + 8, (flags & 0xfef5) | 0xc5);
                let start = load_float(e, START_TIME);
                e.vcall(me, 0x8c, &args![start]);
            }
        }
    } else if collision_object != 0 {
        let physics = e
            .call(IS_KIND_OF, &args![collision_object, PHYSICS_OBJECT_RTTI])
            .u32();
        if physics != 0 {
            let body = e.mem.u32(physics + 0x10);
            if body != 0 {
                let flags = e.with_stack(4, |e, out| {
                    let flags_address = e.call(READ_OBJECT_FLAGS, &args![body, out]).u32();
                    e.mem.u32(flags_address)
                });
                let class = flags & 0x7f;
                let skip = (class == 8 || class == 0x1d) && (flags & 0x1f00) != 0x1400;
                if !skip {
                    e.vcall(body, 0xe4, &args![4u32]);
                }
            }
        }
    }
    let pass = x87_quiet(key_time);
    for_each_child(e, object.addr(), |e, child| {
        fn_00c9b470(e, Ptr::new(child), pass);
    });
}

// Translated from 00c9b670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9b670 - the /GS cookie check (00ec408c) is not modeled.
/// `bhkBlendController::DoKnockDown` (Xbox PDB), cdecl
/// `(object, position, pass_through, key_time, apply_only)`: for an object whose collision
/// object is a `bhkBlendCollisionObject` (synchronized first, `00c81a50`), reads the knock-down
/// gain of its class (table `011b1210`) and, when it is not below zero and the object has a
/// blend controller: with a nonzero low byte of `apply_only`, calls the collision object's
/// virtual `+0xb0` with `(1, 0, 0)`; else replaces the keys by two (times 0 and the gain, the
/// gains 0.0 and the collision object's) and starts the controller, and, when the object has no
/// force-class controller, pushes it by the force `position * 011b0f88` (up component
/// `011b0f8c`, `00cb94a0`). Without a blend collision object, a physics object's virtual
/// `+0xe4` is called with 1 and, unless `apply_only`, `hkpRigidBody::applyForce` with the same
/// force. Recurses on the children.
pub fn bhk_blend_controller_do_knock_down(
    e: &mut Engine,
    object: Ptr,
    position: Ptr,
    pass_through: u32,
    key_time: f32,
    apply_only: u32,
) {
    if object.addr() == 0 {
        return;
    }
    let apply_only_byte = apply_only as u8;
    let collision_object = e.mem.u32(object.addr() + 0x1c);
    let mut blend_object = 0;
    if collision_object != 0 {
        blend_object = e
            .call(
                IS_KIND_OF,
                &args![collision_object, BLEND_COLLISION_OBJECT_RTTI],
            )
            .u32();
    }
    if blend_object != 0 {
        e.call(SYNCHRONIZE_MOTION_TYPE, &args![blend_object]);
        let flags = collision_class_flags(e, blend_object);
        let gain = load_float(e, KNOCK_DOWN_GAINS + ((flags >> 8) & 0x1f) * 4);
        let controller = e
            .call(GET_CONTROLLER, &args![object, BLEND_CONTROLLER_RTTI])
            .u32();
        if controller != 0 && 0.0 <= gain {
            if apply_only_byte != 0 {
                e.vcall(blend_object, 0xb0, &args![1u32, 0u32, 0u32]);
            } else {
                let heir = load_float(e, blend_object + COLLISION_OBJECT_HEIR_GAIN);
                let vel = load_float(e, blend_object + COLLISION_OBJECT_VEL_GAIN);
                let controller = Ptr::<BhkBlendController>::new(controller);
                bhk_blend_controller_clear_keys(e, controller);
                e.set(controller, BhkBlendController::iPriority, 2);
                if e.mem.u32(controller.addr() + 0x48) == 0 {
                    e.with_stack(0x18, |e, keys| {
                        e.call(
                            VECTOR_CONSTRUCTOR_ITERATOR,
                            &args![keys, KEY_SIZE, 2u32, KEY_CONSTRUCTOR],
                        );
                        let key = |index: u32| keys.addr() + index * KEY_SIZE;
                        e.mem.set_f32(key(0) + 4, heir);
                        e.mem.set_f32(key(0) + 8, vel);
                        e.mem.set_f32(key(0), 0.0);
                        e.mem.set_f32(key(1) + 4, 0.0);
                        e.mem.set_f32(key(1) + 8, 0.0);
                        e.mem.set_f32(key(1), gain);
                        let array = controller.at(BhkBlendController::pBlendKeyA);
                        blend_key_array_set_size(e, array, 2);
                        for index in 0..2 {
                            fn_00c9b1a0(e, controller, Ptr::new(key(index)));
                        }
                    });
                    let me = controller.addr();
                    e.mem.set_f32(me + 0x14, x87_quiet(key_time));
                    e.mem.set_f32(me + 0x18, gain);
                    e.mem.set_f32(me + 0x10, 0.0);
                    let flags = e.mem.u16(me + 8);
                    e.mem.set_u16(me + 8, (flags & 0xfef5) | 0xc5);
                    e.mem.set_f32(me + 0xc, 1.0);
                    let start = load_float(e, START_TIME);
                    e.vcall(me, 0x8c, &args![start]);
                }
                let force_controller = e
                    .call(GET_CONTROLLER, &args![object, FORCE_CONTROLLER_RTTI])
                    .u32();
                if force_controller == 0 {
                    push_by_position(e, object, position, None);
                }
            }
        }
    } else if collision_object != 0 {
        let physics = e
            .call(IS_KIND_OF, &args![collision_object, PHYSICS_OBJECT_RTTI])
            .u32();
        if physics != 0 {
            let body = e.mem.u32(physics + 0x10);
            if body != 0 {
                let flags = e.with_stack(4, |e, out| {
                    let flags_address = e.call(READ_OBJECT_FLAGS, &args![body, out]).u32();
                    e.mem.u32(flags_address)
                });
                let class = flags & 0x7f;
                let skip = (class == 8 || class == 0x1d) && (flags & 0x1f00) != 0x1400;
                if !skip {
                    e.vcall(body, 0xe4, &args![1u32]);
                    if apply_only_byte == 0 {
                        push_by_position(e, object, position, Some(body));
                    }
                }
            }
        }
    }
    let pass_key_time = x87_quiet(key_time);
    for_each_child(e, object.addr(), |e, child| {
        bhk_blend_controller_do_knock_down(
            e,
            Ptr::new(child),
            position,
            pass_through,
            pass_key_time,
            apply_only,
        );
    });
}

/// The force of `DoKnockDown`: `position` scaled by `011b0f88` (`0045bb20`), widened to a
/// 4-lane vector (`004a3e00`). With `rigid_body` (the physics object's body), the vector goes to
/// `hkpRigidBody::applyForce(body + 8, 011afe64, vector)`; else to `00cb94a0(object, vector,
/// 011b0f8c)`.
fn push_by_position(e: &mut Engine, object: Ptr, position: Ptr, rigid_body: Option<u32>) {
    e.with_stack(0x1c, |e, frame| {
        let scaled = frame;
        let widened = frame.byte_add(0xc);
        let scale = load_float(e, PASS_OUT_FORCE_SCALE);
        e.call(POINT_SCALE, &args![position, scaled, scale]);
        let vector = e.call(VECTOR_FROM_POINT, &args![widened, scaled]).u32();
        match rigid_body {
            None => {
                let up = load_float(e, PASS_OUT_FORCE_UP);
                e.call(APPLY_FORCE_VECTOR, &args![object, vector, up]);
            }
            Some(body) => {
                let time = load_float(e, PASS_OUT_TIME);
                let rigid = e.mem.u32(body + 8);
                e.call(RIGID_BODY_APPLY_FORCE, &args![rigid, time, vector]);
            }
        }
    });
}

// Translated from 00c9b970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9b970 - the FS:[0] exception-unwinding frame is not modeled.
/// Copies this controller's members into the clone `destination` (unconfirmed name; clone
/// creation calls it): inside the `TLS + 0x2b4 = 0x10` scope, `NiTimeController`'s copy
/// (`00a6cfe0`, `destination`, `process`), `iLastIndex`, then the destination's key array is
/// sized to the key count (`00c9ac00`) and every key is appended (`00c9b1a0`).
pub fn fn_00c9b970(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    destination: Ptr<BhkBlendController>,
    process: Ptr,
) {
    let scope = enter_scope(e);
    e.call(
        TIME_CONTROLLER_COPY_MEMBERS,
        &args![this, destination, process],
    );
    let last_index = e.get(this, BhkBlendController::iLastIndex);
    e.set(destination, BhkBlendController::iLastIndex, last_index);
    let count = e.mem.u32(this.addr() + 0x48);
    blend_key_array_set_size(e, destination.at(BhkBlendController::pBlendKeyA), count);
    if count != 0 {
        let mut index = 0u32;
        loop {
            let key = e.mem.u32(this.addr() + 0x3c) + index * KEY_SIZE;
            fn_00c9b1a0(e, destination, Ptr::new(key));
            index += 1;
            if index >= e.mem.u32(this.addr() + 0x48) {
                break;
            }
        }
    }
    leave_scope(e, scope);
}

// Translated from 00c9ba40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9ba40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendController::LoadBinary` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10` scope, the base
/// `NiTimeController::LoadBinary` (`00a6d5e0`), then through the stream's reader
/// (`stream + 0x24c`, function pointer at `+8`, called as `(input, buffer, size, &size, 1)`;
/// the position `+4` advances by the answer) the key count; the array is sized to it and each
/// key is read (12 bytes, `(input, buffer, 0xc, 0, 0)`, preset to the empty key), byte-swapped
/// when the byte at `stream + 0x1e5` is not 1, and appended (`00c9b1a0`).
pub fn bhk_blend_controller_load_binary(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    stream: Ptr,
) {
    let scope = enter_scope(e);
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    // The exe's frame (exception record, `SUB ESP,0x18`, four pushes and the cookie): 0x38
    // bytes, the size word at +0x18 (00c9baab) and the key at +0x20 (00c9bb1a). The count is
    // read into the `stream` argument's own slot (00c9bab2), +0x3c past the return address.
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x3c, stream.addr());
        let count_slot = frame + 0x3c;
        let size_slot = frame + 0x18;
        let key = frame + 0x20;
        let input = e.mem.u32(stream.addr() + 0x24c);
        e.mem.set_u32(size_slot, 4);
        let reader = e.mem.u32(input + 8);
        let read = e
            .call(reader, &args![input, count_slot, 4u32, size_slot, 1u32])
            .u32();
        let position = e.mem.u32(input + 4);
        e.mem.set_u32(input + 4, position.wrapping_add(read));
        let count = e.mem.u32(count_slot);
        blend_key_array_set_size(e, this.at(BhkBlendController::pBlendKeyA), count);
        if count != 0 {
            let empty = empty_key_bits(e);
            let mut index = 0u32;
            loop {
                let input = e.mem.u32(stream.addr() + 0x24c);
                write_key(e, key, empty);
                let reader = e.mem.u32(input + 8);
                let read = e.call(reader, &args![input, key, 0xcu32, 0u32, 0u32]).u32();
                let position = e.mem.u32(input + 4);
                e.mem.set_u32(input + 4, position.wrapping_add(read));
                if e.mem.u8(stream.addr() + 0x1e5) != 1 {
                    for lane in 0..3 {
                        let word = e.mem.u32(key + 4 * lane);
                        e.mem.set_u32(key + 4 * lane, word.swap_bytes());
                    }
                }
                fn_00c9b1a0(e, this, Ptr::new(key));
                index += 1;
                if index >= count {
                    break;
                }
            }
        }
    });
    leave_scope(e, scope);
}

/// The inline `NiTArray<char*>` append of the viewer-string code: stores `text` at the array's
/// size (growing by the grow-by, `0096ad30`, when the size has reached the capacity) and keeps
/// the used count. Field offsets: base `+4`, capacity `+8`, size `+0xa`, used `+0xc`, grow-by
/// `+0xe` (16-bit counts).
fn append_text(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    let current = u32::from(e.mem.u16(array + 0xa));
    let used = e.mem.u16(array + 0xc);
    let base = e.mem.u32(array + 4);
    if size >= current {
        e.mem.set_u16(array + 0xa, (size + 1) as u16);
        if text != 0 {
            e.mem.set_u16(array + 0xc, used.wrapping_add(1));
        }
    } else if text == 0 {
        if e.mem.u32(base + size * 4) != 0 {
            e.mem.set_u16(array + 0xc, used.wrapping_sub(1));
        }
    } else if e.mem.u32(base + size * 4) == 0 {
        e.mem.set_u16(array + 0xc, used.wrapping_add(1));
    }
    let base = e.mem.u32(array + 4);
    e.mem.set_u32(base + size * 4, text);
}

/// The viewer-string code stores a string through `0096ae90` after growing the array.
fn store_text(e: &mut Engine, array: u32, value_slot: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![array, size, value_slot]);
}

// Translated from 00c9bbb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9bbb0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkBlendController::GetViewerStrings` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10` scope, the
/// base strings (`00a6d830`), the class name (`NiGetViewerString` of the first word of the
/// RTTI at `012681f0`) and `"iKeys"` with the key count are stored in the string array
/// (`0096ae90`, after growing it by `0096ad30`), then one string per key,
/// `"Key%d: %.4f, %.4f, %.4f"` printed (`00406d00`) into a 64-byte buffer, copied into a block
/// of the memory manager and appended with the used-count bookkeeping.
pub fn bhk_blend_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    strings: Ptr,
) {
    let scope = enter_scope(e);
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    // The exe's frame (`AND ESP,0xffffffc0`, exception record, `SUB ESP,0xa8`, four pushes):
    // 0xc4 bytes below the 64-byte boundary, the string slot at +0x28 (00c9bc5c) and the
    // 64-byte print buffer at +0x3c (00c9bcd7), which sits on that boundary.
    e.with_stack(0xc4 + 0x40, |e, raw| {
        let frame = ((raw.addr() + 0x3c + 0x3f) & !0x3f) - 0x3c;
        let slot = frame + 0x28;
        let buffer = frame + 0x3c;
        let name = e.mem.u32(BLEND_CONTROLLER_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
        e.mem.set_u32(slot, text);
        store_text(e, strings.addr(), slot);
        let count = e.mem.u32(this.addr() + 0x48);
        let text = e
            .call(GET_VIEWER_STRING_INT, &args![KEYS_LABEL, count])
            .u32();
        e.mem.set_u32(slot, text);
        store_text(e, strings.addr(), slot);
        if count != 0 {
            let mut index = 0u32;
            loop {
                let key = e.mem.u32(this.addr() + 0x3c) + index * KEY_SIZE;
                let a = f64::from(load_float(e, key));
                let b = f64::from(load_float(e, key + 4));
                let c = f64::from(load_float(e, key + 8));
                e.call(SPRINTF, &args![buffer, 0x40u32, KEY_FORMAT, index, a, b, c]);
                let mut length = 0u32;
                while e.mem.u8(buffer + length) != 0 {
                    length += 1;
                }
                let size = length + 1;
                let copy = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
                e.call(STRCPY_S, &args![copy, size, buffer]);
                append_text(e, strings.addr(), copy);
                index += 1;
                if index >= e.mem.u32(this.addr() + 0x48) {
                    break;
                }
            }
        }
    });
    leave_scope(e, scope);
}

// Translated from 00c9bdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Get-up entry (unconfirmed name), cdecl `(object, key_time)`: when the object is a node
/// (its RTTI chain, virtual `+8` and base links at `+4`, reaches `NiNode`) whose first child is
/// also a node whose first child's collision object (`+0x1c`) is a physics object (`012043f8`)
/// with a body (`+0x10`), calls the body's virtual `+0xe4` with 4 and both refresh methods
/// (`00561690`, `005617c0`) with `01267e30`; in every case then runs `00c9b470(object,
/// key_time)`.
// The nested tests follow the exe's branch order.
#[allow(clippy::collapsible_if)]
pub fn fn_00c9bdf0(e: &mut Engine, object: Ptr, key_time: f32) {
    if object.addr() != 0 && is_node(e, object.addr()) {
        let me = object.addr();
        if e.mem.u16(me + 0xa6) != 0 {
            let first_child = e.mem.u32(e.mem.u32(me + 0xa0));
            if first_child != 0 {
                if is_node(e, first_child) && e.mem.u16(first_child + 0xa6) != 0 {
                    let grandchild = e.mem.u32(e.mem.u32(first_child + 0xa0));
                    if grandchild != 0 {
                        let collision_object = e.mem.u32(grandchild + 0x1c);
                        if collision_object != 0 {
                            let physics = e
                                .call(IS_KIND_OF, &args![collision_object, PHYSICS_OBJECT_RTTI])
                                .u32();
                            if physics != 0 {
                                let body = e.mem.u32(physics + 0x10);
                                if body != 0 {
                                    e.vcall(body, 0xe4, &args![4u32]);
                                    e.call(PHYSICS_REFRESH_A, &args![body, IDENTITY_TRANSFORM]);
                                    e.call(PHYSICS_REFRESH_B, &args![body, IDENTITY_TRANSFORM]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn_00c9b470(e, object, key_time);
}

/// Walks the RTTI chain of `object` (virtual `+8`, base link at `+4`) looking for `NiNode`.
/// A missing RTTI counts as not a node.
fn is_node(e: &mut Engine, object: u32) -> bool {
    let mut rtti = e.vcall(object, 8, &args![]).u32();
    while rtti != 0 {
        if rtti == NODE_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00c9bef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9bef0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendController::CreateClone` (Xbox PDB): inside the `TLS + 0x2b4 = 0x10` scope,
/// allocates 0x5c bytes (`00aa13e0`), constructs a controller in them when the allocation
/// worked, and copies this controller's members into it (`00c9b970`, `(clone, process)`, also
/// when the clone is 0). Answers the clone.
pub fn bhk_blend_controller_create_clone(
    e: &mut Engine,
    this: Ptr<BhkBlendController>,
    process: Ptr,
) -> Ptr {
    let scope = enter_scope(e);
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    let mut clone = Ptr::<BhkBlendController>::NULL;
    if block != 0 {
        clone = bhk_blend_controller_construct(e, Ptr::new(block)).cast();
    }
    fn_00c9b970(e, this, clone, process);
    leave_scope(e, scope);
    clone.cast()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9a6e0, blend_key_array_remove_all(Ptr<NiTLargeArray>)),
        entry!(
            0x00c9a730,
            blend_key_array_set_at(Ptr<NiTLargeArray>, u32, Ptr)
        ),
        entry!(
            0x00c9a820,
            bhk_blend_controller_clear_keys(Ptr<BhkBlendController>)
        ),
        entry!(
            0x00c9a840,
            bhk_blend_controller_is_equal(Ptr<BhkBlendController>, Ptr<BhkBlendController>) -> bool
        ),
        entry!(
            0x00c9a8c0,
            fn_00c9a8c0(Ptr<BhkBlendController>, f32, Ptr, Ptr) -> bool
        ),
        entry!(0x00c9ab10, fn_00c9ab10(Ptr<BhkBlendController>)),
        entry!(
            0x00c9abc0,
            bhk_blend_controller_restore_original_gains(Ptr<BhkBlendController>, Ptr)
        ),
        entry!(
            0x00c9ac00,
            blend_key_array_set_size(Ptr<NiTLargeArray>, u32)
        ),
        entry!(0x00c9ad60, fn_00c9ad60(Ptr<BhkBlendController>)),
        entry!(0x00c9add0, bhk_blend_controller_get_r_t_t_i() -> u32),
        entry!(
            0x00c9ade0,
            bhk_blend_controller_clear_force_controller(Ptr<BhkBlendController>) -> bool
        ),
        entry!(
            0x00c9aeb0,
            bhk_blend_controller_construct(Ptr<BhkBlendController>) -> Ptr
        ),
        entry!(0x00c9af20, bhk_blend_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c9af40,
            bhk_blend_controller_save_binary(Ptr<BhkBlendController>, Ptr)
        ),
        entry!(0x00c9b020, fn_00c9b020() -> Ptr),
        entry!(
            0x00c9b090,
            bhk_blend_controller_update(Ptr<BhkBlendController>, Ptr)
        ),
        entry!(0x00c9b1a0, fn_00c9b1a0(Ptr<BhkBlendController>, Ptr)),
        entry!(0x00c9b250, bhk_blend_controller_do_hit(Ptr, f32, u32)),
        entry!(0x00c9b470, fn_00c9b470(Ptr, f32)),
        entry!(
            0x00c9b670,
            bhk_blend_controller_do_knock_down(Ptr, Ptr, u32, f32, u32)
        ),
        entry!(
            0x00c9b970,
            fn_00c9b970(Ptr<BhkBlendController>, Ptr<BhkBlendController>, Ptr)
        ),
        entry!(
            0x00c9ba40,
            bhk_blend_controller_load_binary(Ptr<BhkBlendController>, Ptr)
        ),
        entry!(
            0x00c9bbb0,
            bhk_blend_controller_get_viewer_strings(Ptr<BhkBlendController>, Ptr)
        ),
        entry!(0x00c9bdf0, fn_00c9bdf0(Ptr, f32)),
        entry!(
            0x00c9bef0,
            bhk_blend_controller_create_clone(Ptr<BhkBlendController>, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_blend_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c9_ad60, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c9af20, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c9af20, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x5c]]);
        assert_eq!(calls_to(&e, 0x00c9_ad60), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_blend_controller_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_add0, &args![]).u32(), 0x0126_81f0);
    }

    type Key = [f32; 3];
    const EMPTY: Key = [-1.0, 0.0, 0.0];

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Writes a vtable at `address` whose slots (byte offsets) point at the targets.
    fn put_vtable_at(e: &mut Engine, address: u32, entries: &[(u32, u32)]) {
        let words = entries
            .iter()
            .map(|(offset, _)| offset / 4 + 1)
            .max()
            .unwrap();
        let mut table = vec![0u32; words as usize];
        for (offset, target) in entries {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(address, &table);
    }

    fn keys_equal(e: &Engine, a: u32, b: u32) -> bool {
        (0..3).all(|lane| e.mem.f32(a + 4 * lane) == e.mem.f32(b + 4 * lane))
    }

    /// An engine with the float constants and the callees outside this file that most tests
    /// need: the key comparisons, `lerp`, the heap and a vector constructor that does nothing.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [MINUS_ONE, ZERO_DIVISOR, FLOAT_MAX, NEGATIVE_FLOAT_MAX] {
            e.map(address, 8);
        }
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(ZERO_DIVISOR, 100.0f64);
        e.set_global(FLOAT_MAX, f64::from(f32::MAX));
        e.set_global(NEGATIVE_FLOAT_MAX, -f64::from(f32::MAX));
        e.call_log = Some(vec![]);
        e.register(KEY_EQUAL, |e, a| keys_equal(e, a[0], a[1]).into_ret());
        e.register(KEY_NOT_EQUAL, |e, a| {
            (!keys_equal(e, a[0], a[1])).into_ret()
        });
        e.register(LERP, |_, a| {
            let (t, from, to) = (
                f32::from_bits(a[0]),
                f32::from_bits(a[1]),
                f32::from_bits(a[2]),
            );
            ((1.0 - t) * from + t * to).into_ret()
        });
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(HEAP_FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e
    }

    fn put_key(e: &mut Engine, address: u32, key: Key) {
        for (lane, value) in key.into_iter().enumerate() {
            e.mem.set_f32(address + 4 * lane as u32, value);
        }
    }

    fn read_key(e: &Engine, address: u32) -> Key {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
        ]
    }

    fn key_at(e: &Engine, array: Ptr<NiTLargeArray>, index: u32) -> Key {
        read_key(e, e.get(array, NiTLargeArray::m_pBase) + 12 * index)
    }

    /// Fills the array at `array`: `keys` stored from index 0 in a block of `capacity` keys (the
    /// rest empty), size `keys.len()`, used count the number of non-empty keys.
    fn fill_array(e: &mut Engine, array: Ptr<NiTLargeArray>, keys: &[Key], capacity: u32) {
        let base = if capacity == 0 {
            0
        } else {
            e.mem.alloc(capacity * 12)
        };
        for index in 0..capacity {
            put_key(e, base + 12 * index, EMPTY);
        }
        for (index, key) in keys.iter().enumerate() {
            put_key(e, base + 12 * index as u32, *key);
        }
        e.set(array, NiTLargeArray::m_pBase, base);
        e.set(array, NiTLargeArray::m_uiMaxSize, capacity);
        e.set(array, NiTLargeArray::m_uiSize, keys.len() as u32);
        let used = keys.iter().filter(|key| **key != EMPTY).count() as u32;
        e.set(array, NiTLargeArray::m_uiESize, used);
        e.set(array, NiTLargeArray::m_uiGrowBy, 1);
    }

    fn make_array(e: &mut Engine, keys: &[Key], capacity: u32) -> Ptr<NiTLargeArray> {
        let array = e.new_object::<NiTLargeArray>();
        fill_array(e, array, keys, capacity);
        array
    }

    /// A controller whose embedded key array holds `keys` with room for `capacity` keys.
    fn make_controller(e: &mut Engine, keys: &[Key], capacity: u32) -> Ptr<BhkBlendController> {
        let controller = e.new_object::<BhkBlendController>();
        fill_array(
            e,
            controller.at(BhkBlendController::pBlendKeyA),
            keys,
            capacity,
        );
        controller
    }

    fn controller_keys(e: &Engine, controller: Ptr<BhkBlendController>) -> Vec<Key> {
        let array = controller.at(BhkBlendController::pBlendKeyA);
        (0..e.get(array, NiTLargeArray::m_uiSize))
            .map(|index| key_at(e, array, index))
            .collect()
    }

    #[test]
    fn remove_all_resets_every_key_and_the_counts() {
        let mut e = engine();
        let array = make_array(&mut e, &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], 4);
        e.call(0x00c9a6e0, &args![array]);
        assert_eq!(key_at(&e, array, 0), EMPTY);
        assert_eq!(key_at(&e, array, 1), EMPTY);
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 0);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 0);
        // With a size of 0 no key is touched.
        let array = make_array(&mut e, &[], 2);
        let base = e.get(array, NiTLargeArray::m_pBase);
        put_key(&mut e, base, [9.0, 9.0, 9.0]);
        e.call(0x00c9a6e0, &args![array]);
        assert_eq!(key_at(&e, array, 0), [9.0, 9.0, 9.0]);
    }

    #[test]
    fn set_at_counts_the_slots_that_change_between_empty_and_used() {
        let mut e = engine();
        let array = make_array(&mut e, &[[1.0, 1.0, 1.0]], 4);
        let value = e.mem.alloc(12);
        // Past the size: the size follows, the used count only for a real key.
        put_key(&mut e, value, [2.0, 2.0, 2.0]);
        e.call(0x00c9a730, &args![array, 2u32, value]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 3);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 2);
        assert_eq!(key_at(&e, array, 2), [2.0, 2.0, 2.0]);
        put_key(&mut e, value, EMPTY);
        e.call(0x00c9a730, &args![array, 3u32, value]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 4);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 2);
        // Inside the size: empty slot gets a key, key gets another key, key gets emptied.
        put_key(&mut e, value, [3.0, 3.0, 3.0]);
        e.call(0x00c9a730, &args![array, 1u32, value]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 3);
        put_key(&mut e, value, [4.0, 4.0, 4.0]);
        e.call(0x00c9a730, &args![array, 1u32, value]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 3);
        put_key(&mut e, value, EMPTY);
        e.call(0x00c9a730, &args![array, 1u32, value]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 2);
        assert_eq!(key_at(&e, array, 1), EMPTY);
    }

    #[test]
    fn set_at_keeps_the_empty_key_at_frame_offset_0x10() {
        let mut e = engine();
        let array = make_array(&mut e, &[[1.0, 1.0, 1.0]], 4);
        let value = e.mem.alloc(12);
        put_key(&mut e, value, [2.0, 2.0, 2.0]);
        // The frame is the next 0x1c-byte local the game thread's stack gives out.
        let frame = e.next_stack_local(0x1c);
        e.call(0x00c9a730, &args![array, 2u32, value]);
        assert_eq!(calls_to(&e, KEY_NOT_EQUAL), vec![vec![value, frame + 0x10]]);
    }

    #[test]
    fn clear_keys_empties_the_array_and_resets_the_gains_and_priority() {
        let mut e = engine();
        let controller = make_controller(&mut e, &[[1.0, 2.0, 3.0]], 2);
        e.set(controller, BhkBlendController::fOriginalGain, 0.5);
        e.set(controller, BhkBlendController::fOriginalVelGain, 0.25);
        e.set(controller, BhkBlendController::iPriority, 2);
        e.call(0x00c9a820, &args![controller]);
        assert_eq!(controller_keys(&e, controller), Vec::<Key>::new());
        assert_eq!(e.get(controller, BhkBlendController::fOriginalGain), -1.0);
        assert_eq!(
            e.get(controller, BhkBlendController::fOriginalVelGain),
            -1.0
        );
        assert_eq!(e.get(controller, BhkBlendController::iPriority), 0);
    }

    #[test]
    fn is_equal_needs_the_base_the_count_and_every_float() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_IS_EQUAL, |e, a| {
            (e.mem.u32(a[0] + 0x24) == 0).into_ret()
        });
        let keys = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let first = make_controller(&mut e, &keys, 2);
        let second = make_controller(&mut e, &keys, 2);
        let equal = |e: &mut Engine| e.call(0x00c9a840, &args![first, second]).bool();
        assert!(equal(&mut e));
        // The base comparison failing.
        e.mem.set_u32(first.addr() + 0x24, 1);
        assert!(!equal(&mut e));
        e.mem.set_u32(first.addr() + 0x24, 0);
        // A different float in the last lane of the last key.
        let second_base = e.mem.u32(second.addr() + 0x3c);
        e.mem.set_f32(second_base + 20, 7.0);
        assert!(!equal(&mut e));
        e.mem.set_f32(second_base + 20, 6.0);
        // NaN is never equal, even to itself.
        let first_base = e.mem.u32(first.addr() + 0x3c);
        e.mem.set_u32(second_base, 0x7fc0_0000);
        e.mem.set_u32(first_base, 0x7fc0_0000);
        assert!(!equal(&mut e));
        // Different counts.
        e.mem.set_u32(first_base, 0x3f80_0000);
        e.mem.set_u32(second_base, 0x3f80_0000);
        e.mem.set_u32(second.addr() + 0x48, 1);
        assert!(!equal(&mut e));
    }

    fn six_keys(e: &mut Engine) -> Ptr<BhkBlendController> {
        let keys: Vec<Key> = (0..6)
            .map(|index| [index as f32, 10.0 * index as f32, 100.0 * index as f32])
            .collect();
        make_controller(e, &keys, 6)
    }

    fn lookup(e: &mut Engine, controller: Ptr<BhkBlendController>, time: f32) -> (bool, f32, f32) {
        let out = e.mem.alloc(8);
        let found = e
            .call(0x00c9a8c0, &args![controller, time, out, out + 4])
            .bool();
        let result = (found, e.mem.f32(out), e.mem.f32(out + 4));
        e.mem.free(out);
        result
    }

    #[test]
    fn key_lookup_interpolates_between_the_keys_around_the_time() {
        let mut e = engine();
        let empty = make_controller(&mut e, &[], 0);
        assert_eq!(lookup(&mut e, empty, 1.0), (false, 0.0, 0.0));
        let single = make_controller(&mut e, &[[5.0, 7.0, 8.0]], 1);
        assert_eq!(lookup(&mut e, single, 100.0), (true, 7.0, 8.0));
        // Six keys at times 0..5 (gains 10 and 100 times the index): the scan runs in blocks.
        let controller = six_keys(&mut e);
        assert_eq!(lookup(&mut e, controller, 2.5), (true, 25.0, 250.0));
        assert_eq!(e.mem.u32(controller.addr() + 0x34), 2);
        // The remembered key is later than the time: start over from key 0.
        e.mem.set_u32(controller.addr() + 0x34, 4);
        assert_eq!(lookup(&mut e, controller, 0.5), (true, 5.0, 50.0));
        assert_eq!(e.mem.u32(controller.addr() + 0x34), 0);
        // A time past the last key gives the last key's gains.
        assert_eq!(lookup(&mut e, controller, 9.0), (true, 50.0, 500.0));
        assert_eq!(e.mem.u32(controller.addr() + 0x34), 5);
        // Two keys with the same time: the lower key's gains, no interpolation.
        let twin = make_controller(&mut e, &[[1.0, 5.0, 6.0], [1.0, 7.0, 8.0]], 2);
        assert_eq!(lookup(&mut e, twin, 1.0), (true, 5.0, 6.0));
        // Exactly on a key time: the weight is 1 towards that key.
        let controller = six_keys(&mut e);
        assert_eq!(lookup(&mut e, controller, 3.0), (true, 30.0, 300.0));
        assert_eq!(calls_to(&e, LERP).len(), 6);
    }

    #[test]
    fn key_time_refresh_takes_the_first_and_last_key_times() {
        let mut e = engine();
        let controller = make_controller(&mut e, &[], 0);
        e.set(controller, BhkBlendController::m_fLoKeyTime, 5.0);
        e.set(controller, BhkBlendController::m_fHiKeyTime, 6.0);
        e.call(0x00c9ab10, &args![controller]);
        assert_eq!(e.get(controller, BhkBlendController::m_fLoKeyTime), 0.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fHiKeyTime), 0.0);
        let controller = make_controller(&mut e, &[[2.0, 0.0, 0.0], [7.0, 0.0, 0.0]], 2);
        // First key below the low time, last key below the high time: both replaced.
        e.set(controller, BhkBlendController::m_fLoKeyTime, 10.0);
        e.set(controller, BhkBlendController::m_fHiKeyTime, 20.0);
        e.call(0x00c9ab10, &args![controller]);
        assert_eq!(e.get(controller, BhkBlendController::m_fLoKeyTime), 2.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fHiKeyTime), 7.0);
        // Neither below: both kept.
        e.set(controller, BhkBlendController::m_fLoKeyTime, 1.0);
        e.set(controller, BhkBlendController::m_fHiKeyTime, 3.0);
        e.call(0x00c9ab10, &args![controller]);
        assert_eq!(e.get(controller, BhkBlendController::m_fLoKeyTime), 1.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fHiKeyTime), 3.0);
        // The extreme markers are replaced even when the key is above them.
        e.set(controller, BhkBlendController::m_fLoKeyTime, f32::MAX);
        e.set(controller, BhkBlendController::m_fHiKeyTime, -f32::MAX);
        e.call(0x00c9ab10, &args![controller]);
        assert_eq!(e.get(controller, BhkBlendController::m_fLoKeyTime), 2.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fHiKeyTime), 7.0);
        // A NaN low time is kept: neither test replaces it.
        e.set(controller, BhkBlendController::m_fLoKeyTime, f32::NAN);
        e.call(0x00c9ab10, &args![controller]);
        assert!(e.get(controller, BhkBlendController::m_fLoKeyTime).is_nan());
    }

    #[test]
    fn restore_original_gains_needs_the_flag_and_a_saved_gain() {
        let mut e = engine();
        let controller = make_controller(&mut e, &[], 0);
        let object = e.mem.alloc(0x20);
        e.set(controller, BhkBlendController::fOriginalGain, 0.5);
        e.set(controller, BhkBlendController::fOriginalVelGain, 0.75);
        // Bit 8 of the flags clear: nothing happens.
        e.call(0x00c9abc0, &args![controller, object]);
        assert_eq!(e.mem.f32(object + 0x18), 0.0);
        e.mem.set_u8(controller.addr() + 9, 1);
        e.call(0x00c9abc0, &args![controller, object]);
        assert_eq!(e.mem.f32(object + 0x18), 0.5);
        assert_eq!(e.mem.f32(object + 0x1c), 0.75);
        assert_eq!(e.get(controller, BhkBlendController::fOriginalGain), -1.0);
        assert_eq!(
            e.get(controller, BhkBlendController::fOriginalVelGain),
            -1.0
        );
        // The saved gain is -1.0 now: a second call changes nothing.
        e.mem.set_f32(object + 0x18, 0.125);
        e.call(0x00c9abc0, &args![controller, object]);
        assert_eq!(e.mem.f32(object + 0x18), 0.125);
        // Zero is accepted, NaN is not.
        e.set(controller, BhkBlendController::fOriginalGain, 0.0);
        e.call(0x00c9abc0, &args![controller, object]);
        assert_eq!(e.mem.f32(object + 0x18), 0.0);
        e.set(controller, BhkBlendController::fOriginalGain, f32::NAN);
        e.mem.set_f32(object + 0x18, 0.125);
        e.call(0x00c9abc0, &args![controller, object]);
        assert_eq!(e.mem.f32(object + 0x18), 0.125);
    }

    #[test]
    fn set_size_grows_shrinks_and_frees() {
        let mut e = engine();
        let keys = [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]];
        let array = make_array(&mut e, &keys, 3);
        let old_base = e.get(array, NiTLargeArray::m_pBase);
        // The capacity already is the new size: nothing happens.
        e.call(0x00c9ac00, &args![array, 3u32]);
        assert_eq!(e.get(array, NiTLargeArray::m_pBase), old_base);
        // Growing copies the keys into a new block, fills the rest with empty keys, and frees
        // the old block.
        e.call(0x00c9ac00, &args![array, 5u32]);
        assert_ne!(e.get(array, NiTLargeArray::m_pBase), old_base);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE).last(), Some(&vec![60]));
        assert_eq!(calls_to(&e, HEAP_FREE).last(), Some(&vec![old_base]));
        assert_eq!(e.get(array, NiTLargeArray::m_uiMaxSize), 5);
        assert_eq!(key_at(&e, array, 2), [3.0, 3.0, 3.0]);
        assert_eq!(key_at(&e, array, 3), EMPTY);
        assert_eq!(key_at(&e, array, 4), EMPTY);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 3);
        // Shrinking to 1 removes the two keys cut off from the used count and cuts the size.
        e.call(0x00c9ac00, &args![array, 1u32]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 1);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 1);
        assert_eq!(e.get(array, NiTLargeArray::m_uiMaxSize), 1);
        assert_eq!(key_at(&e, array, 0), [1.0, 1.0, 1.0]);
        // Zero leaves no block.
        let block = e.get(array, NiTLargeArray::m_pBase);
        e.call(0x00c9ac00, &args![array, 0u32]);
        assert_eq!(e.get(array, NiTLargeArray::m_pBase), 0);
        assert_eq!(calls_to(&e, HEAP_FREE).last(), Some(&vec![block]));
        assert_eq!(e.get(array, NiTLargeArray::m_uiSize), 0);
        // Shrinking past an empty key does not count it.
        let array = make_array(&mut e, &[[1.0, 1.0, 1.0], EMPTY, [3.0, 3.0, 3.0]], 3);
        e.call(0x00c9ac00, &args![array, 0u32]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiESize), 0);
    }

    #[test]
    fn destructor_body_resets_the_vtables_and_frees_the_keys() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        let controller = make_controller(&mut e, &[[1.0, 1.0, 1.0]], 1);
        let base = e.mem.u32(controller.addr() + 0x3c);
        e.call(0x00c9ad60, &args![controller]);
        assert_eq!(e.mem.u32(controller.addr()), 0x010c_6b14);
        assert_eq!(e.mem.u32(controller.addr() + 0x38), 0x010c_6b04);
        assert_eq!(calls_to(&e, HEAP_FREE).last(), Some(&vec![base]));
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_DESTRUCT),
            vec![vec![controller.addr()]]
        );
    }

    #[test]
    fn clear_force_controller_removes_a_controller_of_the_force_class() {
        let mut e = engine();
        let controller = make_controller(&mut e, &[], 0);
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(controller.addr() + 0x2c, target);
        e.register(GET_CONTROLLER, |_, a| {
            assert_eq!(a[1], FORCE_CONTROLLER_RTTI);
            0x7777u32.into_ret()
        });
        e.register(REMOVE_CONTROLLER, |_, _| Ret::default());
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x1234);
        // Bit 15 of the flags clear: nothing is looked up.
        assert!(!e.call(0x00c9ade0, &args![controller]).bool());
        assert!(calls_to(&e, GET_CONTROLLER).is_empty());
        e.mem.set_u8(controller.addr() + 8, 0x80);
        assert!(e.call(0x00c9ade0, &args![controller]).bool());
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER), vec![vec![target, 0x7777]]);
        // No controller of that class: nothing removed.
        e.register(GET_CONTROLLER, |_, _| Ret::default());
        assert!(!e.call(0x00c9ade0, &args![controller]).bool());
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER).len(), 1);
        // The scope word is restored.
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x1234);
    }

    #[test]
    fn constructor_sets_the_vtables_and_the_empty_state() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        let controller = e.new_object::<BhkBlendController>();
        for offset in (0x34..0x5c).step_by(4) {
            e.mem.set_u32(controller.addr() + offset, 0xdead_beef);
        }
        let result = e.call(0x00c9aeb0, &args![controller]).u32();
        assert_eq!(result, controller.addr());
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_CONSTRUCT),
            vec![vec![controller.addr()]]
        );
        assert_eq!(e.mem.u32(controller.addr()), 0x010c_6b14);
        assert_eq!(e.mem.u32(controller.addr() + 0x38), 0x010c_6b0c);
        for offset in [0x34, 0x3c, 0x40, 0x44, 0x48, 0x58] {
            assert_eq!(e.mem.u32(controller.addr() + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(controller.addr() + 0x4c), 1);
        assert_eq!(e.mem.f32(controller.addr() + 0x50), -1.0);
        assert_eq!(e.mem.f32(controller.addr() + 0x54), -1.0);
    }

    const WRITER: u32 = 0x7000_0010;
    const READER: u32 = 0x7000_0011;
    const STOP: u32 = 0x7000_0002;
    const START: u32 = 0x7000_0004;
    const COLLISION_SLOT_B0: u32 = 0x7000_0006;
    const BODY_SLOT_E4: u32 = 0x7000_0007;
    const FLAGS_WORD: u32 = 0x7100_0000;

    /// A stream whose output object (`+0x250`) writes through a double that appends the bytes
    /// it is given to a log (`output + 0x10` = the log, `+0x14` = bytes so far).
    fn writer_stream(e: &mut Engine, little_endian: bool) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x20);
        let log = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u8(stream + 0x1e4, little_endian as u8);
        e.mem.set_u32(output + 0xc, WRITER);
        e.mem.set_u32(output + 0x10, log);
        e.register(WRITER, |e, a| {
            let used = e.mem.u32(a[0] + 0x14);
            let log = e.mem.u32(a[0] + 0x10);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(log + used, &bytes);
            e.mem.set_u32(a[0] + 0x14, used + a[2]);
            a[2].into_ret()
        });
        (stream, output)
    }

    /// A stream whose input object (`+0x24c`) reads from `words` through a double.
    fn reader_stream(e: &mut Engine, words: &[u32], little_endian: bool) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x20);
        let source = e.mem.alloc(0x100);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(source + 4 * index as u32, *word);
        }
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u8(stream + 0x1e5, little_endian as u8);
        e.mem.set_u32(input + 8, READER);
        e.mem.set_u32(input + 0x10, source);
        e.register(READER, |e, a| {
            let used = e.mem.u32(a[0] + 0x14);
            let source = e.mem.u32(a[0] + 0x10);
            let bytes = e.mem.bytes(source + used, a[2]);
            e.mem.write(a[1], &bytes);
            e.mem.set_u32(a[0] + 0x14, used + a[2]);
            a[2].into_ret()
        });
        (stream, input)
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|value| value.to_bits()).collect()
    }

    #[test]
    fn save_binary_writes_the_count_then_each_key() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        let controller = make_controller(&mut e, &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], 2);
        let (stream, output) = writer_stream(&mut e, true);
        e.call(0x00c9af40, &args![controller, stream]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_SAVE_BINARY),
            vec![vec![controller.addr(), stream]]
        );
        let log = e.mem.u32(output + 0x10);
        let words: Vec<u32> = (0..7).map(|index| e.mem.u32(log + 4 * index)).collect();
        let mut expected = vec![2];
        expected.extend(bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        assert_eq!(words, expected);
        assert_eq!(e.mem.u32(output + 4), 28);
        // Not little-endian: the key words are byte-swapped, the count is not.
        let (stream, output) = writer_stream(&mut e, false);
        e.call(0x00c9af40, &args![controller, stream]);
        let log = e.mem.u32(output + 0x10);
        let words: Vec<u32> = (0..7).map(|index| e.mem.u32(log + 4 * index)).collect();
        let mut expected = vec![2];
        expected.extend(
            bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
                .iter()
                .map(|w| w.swap_bytes()),
        );
        assert_eq!(words, expected);
        // Without keys only the count is written.
        let empty = make_controller(&mut e, &[], 0);
        let (stream, output) = writer_stream(&mut e, true);
        e.call(0x00c9af40, &args![empty, stream]);
        assert_eq!(e.mem.u32(output + 4), 4);
    }

    #[test]
    fn load_binary_uses_the_exe_frame_and_the_argument_slot() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        let mut words = vec![1];
        words.extend(bits(&[1.0, 2.0, 3.0]));
        let controller = make_controller(&mut e, &[], 0);
        let (stream, input) = reader_stream(&mut e, &words, true);
        // The frame is the next 0x40-byte local the game thread's stack gives out.
        let frame = e.next_stack_local(0x40);
        e.call(0x00c9ba40, &args![controller, stream]);
        // The count into the argument slot (+0x3c), the size word at +0x18, the key at +0x20.
        assert_eq!(
            calls_to(&e, READER),
            vec![
                vec![input, frame + 0x3c, 4, frame + 0x18, 1],
                vec![input, frame + 0x20, 0xc, 0, 0]
            ]
        );
        assert_eq!(controller_keys(&e, controller), vec![[1.0, 2.0, 3.0]]);
    }

    #[test]
    fn save_binary_uses_the_exe_frame_and_the_argument_slot() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        let controller = make_controller(&mut e, &[[1.0, 2.0, 3.0]], 1);
        let (stream, output) = writer_stream(&mut e, true);
        // The frame is the next 0x28-byte local the game thread's stack gives out.
        let frame = e.next_stack_local(0x28);
        e.call(0x00c9af40, &args![controller, stream]);
        // The count from the argument slot (+0x24), the key from +0x14, the size word at +0xc.
        assert_eq!(
            calls_to(&e, WRITER),
            vec![
                vec![output, frame + 0x24, 4, frame + 0xc, 1],
                vec![output, frame + 0x14, 0xc, frame + 0xc, 1]
            ]
        );
    }

    #[test]
    fn load_binary_reads_the_count_and_appends_each_key() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        let mut words = vec![2];
        words.extend(bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let controller = make_controller(&mut e, &[], 0);
        let (stream, input) = reader_stream(&mut e, &words, true);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00c9ba40, &args![controller, stream]);
        assert_eq!(
            controller_keys(&e, controller),
            vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
        );
        assert_eq!(e.mem.u32(input + 4), 28);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        // Not little-endian: the key words are swapped on the way in.
        let swapped: Vec<u32> = std::iter::once(2)
            .chain(words[1..].iter().map(|word| word.swap_bytes()))
            .collect();
        let controller = make_controller(&mut e, &[], 0);
        let (stream, _) = reader_stream(&mut e, &swapped, false);
        e.call(0x00c9ba40, &args![controller, stream]);
        assert_eq!(
            controller_keys(&e, controller),
            vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
        );
    }

    #[test]
    fn stream_factory_constructs_in_a_new_block() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c9b020, &args![]).u32(), 0);
        assert!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT).is_empty());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let object = e.call(0x00c9b020, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x5c]; 2]);
        assert_eq!(e.mem.u32(object), 0x010c_6b14);
        assert_eq!(e.mem.f32(object + 0x50), -1.0);
    }

    fn update_scene(e: &mut Engine) -> (Ptr<BhkBlendController>, u32, u32) {
        e.register(IS_KIND_OF, |_, a| {
            let kind = if a[1] == BLEND_COLLISION_OBJECT_RTTI {
                a[0]
            } else {
                0
            };
            kind.into_ret()
        });
        e.register(0x7000_0001, |_, _| 0.5f32.into_ret());
        e.register(STOP, |_, _| Ret::default());
        put_vtable_at(e, 0x7001_2000, &[(0x90, STOP), (0xa4, 0x7000_0001)]);
        let controller = make_controller(e, &[[0.0, 10.0, 20.0], [1.0, 30.0, 40.0]], 2);
        e.mem.set_u32(controller.addr(), 0x7001_2000);
        e.set(controller, BhkBlendController::fOriginalGain, -1.0);
        let target = e.mem.alloc(0x40);
        let collision = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0x1c, collision);
        (controller, target, collision)
    }

    #[test]
    fn update_blends_the_gains_and_stops_at_the_last_key_time() {
        let mut e = engine();
        let (controller, target, collision) = update_scene(&mut e);
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 0.25);
        // No target, no flag bit 3, no keys: nothing is looked at.
        e.call(0x00c9b090, &args![controller, time]);
        e.mem.set_u32(controller.addr() + 0x2c, target);
        e.call(0x00c9b090, &args![controller, time]);
        e.mem.set_u8(controller.addr() + 8, 0x08);
        let keys = e.mem.u32(controller.addr() + 0x48);
        e.mem.set_u32(controller.addr() + 0x48, 0);
        e.call(0x00c9b090, &args![controller, time]);
        e.mem.set_u32(controller.addr() + 0x48, keys);
        assert!(calls_to(&e, IS_KIND_OF).is_empty());
        // No collision object: the time is stored.
        e.mem.set_u32(target + 0x1c, 0);
        e.call(0x00c9b090, &args![controller, time]);
        assert_eq!(e.get(controller, BhkBlendController::m_fLastTime), 0.25);
        e.mem.set_u32(target + 0x1c, collision);
        // A blend collision object: the scaled time 0.5 is half way between the two keys.
        e.mem.set_f32(collision + 0x18, 0.7);
        e.mem.set_f32(collision + 0x1c, 0.8);
        e.call(0x00c9b090, &args![controller, time]);
        assert_eq!(
            calls_to(&e, 0x7000_0001),
            vec![vec![controller.addr(), 0.25f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(collision + 0x18), 20.0);
        assert_eq!(e.mem.f32(collision + 0x1c), 30.0);
        // The negative saved gains were filled from the object's old gains first.
        assert_eq!(e.get(controller, BhkBlendController::fOriginalGain), 0.7);
        assert_eq!(e.get(controller, BhkBlendController::fOriginalVelGain), 0.8);
        assert!(calls_to(&e, STOP).is_empty());
        // The scaled time equals the high key time but the flags masked with 6 are 6.
        e.set(controller, BhkBlendController::m_fHiKeyTime, 0.5);
        e.mem.set_u16(controller.addr() + 8, 0x4e);
        e.call(0x00c9b090, &args![controller, time]);
        assert!(calls_to(&e, STOP).is_empty());
        // Masked flags 4: the keys are cleared (bit 6), the controller stopped.
        e.mem.set_u16(controller.addr() + 8, 0x4c);
        e.call(0x00c9b090, &args![controller, time]);
        assert_eq!(calls_to(&e, STOP), vec![vec![controller.addr()]]);
        assert_eq!(controller_keys(&e, controller), Vec::<Key>::new());
    }

    #[test]
    fn add_key_grows_the_array_and_resets_the_lookup_index() {
        let mut e = engine();
        let controller = make_controller(&mut e, &[], 0);
        e.mem.set_u32(controller.addr() + 0x34, 7);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x55);
        let key = e.mem.alloc(12);
        put_key(&mut e, key, [1.0, 2.0, 3.0]);
        e.call(0x00c9b1a0, &args![controller, key]);
        let array = controller.at(BhkBlendController::pBlendKeyA);
        assert_eq!(controller_keys(&e, controller), vec![[1.0, 2.0, 3.0]]);
        assert_eq!(e.get(array, NiTLargeArray::m_uiMaxSize), 1);
        assert_eq!(e.get(controller, BhkBlendController::iLastIndex), 0);
        put_key(&mut e, key, [4.0, 5.0, 6.0]);
        e.call(0x00c9b1a0, &args![controller, key]);
        assert_eq!(
            controller_keys(&e, controller),
            vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
        );
        assert_eq!(e.get(array, NiTLargeArray::m_uiMaxSize), 2);
        // Room left: no resize.
        let sizes = calls_to(&e, HEAP_ALLOCATE).len();
        e.set(array, NiTLargeArray::m_uiSize, 1);
        e.call(0x00c9b1a0, &args![controller, key]);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE).len(), sizes);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x55);
    }

    /// An object with a collision object whose world object and rigid body give the class
    /// number `class`, and a blend controller; `IS_KIND_OF` and `GET_CONTROLLER` find them.
    struct Scene {
        object: u32,
        collision: u32,
        controller: Ptr<BhkBlendController>,
    }

    fn blend_scene(e: &mut Engine, class: u32) -> Scene {
        put_vtable_at(e, 0x7001_0000, &[(0xc, 0x7000_0003)]);
        put_vtable_at(e, 0x7001_2000, &[(0x8c, START), (0x90, STOP)]);
        put_vtable_at(e, 0x7001_3000, &[(0xb0, COLLISION_SLOT_B0)]);
        e.register(0x7000_0003, |_, _| Ret::default());
        e.register(START, |_, _| Ret::default());
        e.register(COLLISION_SLOT_B0, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        let collision = e.mem.alloc(0x40);
        let world = e.mem.alloc(0x20);
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x7001_0000);
        e.mem.set_u32(object + 0x1c, collision);
        e.mem.set_u32(collision, 0x7001_3000);
        e.mem.set_u32(collision + 0x10, world);
        e.mem.set_u32(world + 8, body);
        e.mem.set_u32(body + 0x2c, class << 8);
        e.mem.set_f32(collision + 0x18, 0.5);
        e.mem.set_f32(collision + 0x1c, 0.25);
        let controller = make_controller(e, &[], 0);
        e.mem.set_u32(controller.addr(), 0x7001_2000);
        let found = controller.addr();
        e.register_double(GET_CONTROLLER, move |_, a| {
            let wanted = if a[1] == BLEND_CONTROLLER_RTTI {
                found
            } else {
                0
            };
            wanted.into_ret()
        });
        e.register(IS_KIND_OF, |_, a| {
            let kind = if a[1] == BLEND_COLLISION_OBJECT_RTTI {
                a[0]
            } else {
                0
            };
            kind.into_ret()
        });
        Scene {
            object,
            collision,
            controller,
        }
    }

    fn put_floats(e: &mut Engine, address: u32, values: &[f32]) {
        e.map(address, 4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * index as u32, *value);
        }
    }

    #[test]
    fn do_hit_replaces_the_keys_by_three_and_starts_the_controller() {
        let mut e = engine();
        let scene = blend_scene(&mut e, 3);
        let mut table = vec![0.0f32; 8];
        table[6] = 0.8;
        table[7] = 0.9;
        put_floats(&mut e, HIT_GAINS, &table);
        table[6] = 0.6;
        table[7] = 0.1;
        put_floats(&mut e, HIT_GAINS_ALTERNATE, &table);
        put_floats(&mut e, HIT_KEY_TIME_A, &[0.125]);
        put_floats(&mut e, HIT_KEY_TIME_B, &[0.5]);
        put_floats(&mut e, START_TIME, &[0.0625]);
        let controller = scene.controller;
        e.call(0x00c9b250, &args![scene.object, 0.0f32, 0u32]);
        assert_eq!(
            controller_keys(&e, controller),
            vec![[0.0, 0.5, 0.25], [0.125, 0.8, 0.9], [0.5, 1.0, 1.0]]
        );
        assert_eq!(e.get(controller, BhkBlendController::iPriority), 1);
        assert_eq!(e.get(controller, BhkBlendController::m_fLoKeyTime), 0.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fHiKeyTime), 1.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fFrequency), 1.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fPhase), 0.0);
        assert_eq!(e.get(controller, BhkBlendController::m_uFlags), 0x1c5);
        assert_eq!(
            calls_to(&e, START),
            vec![vec![controller.addr(), 0.0625f32.to_bits()]]
        );
        // A controller already at priority 2 is left alone.
        e.set(controller, BhkBlendController::iPriority, 2);
        e.call(0x00c9b250, &args![scene.object, 0.0f32, 0u32]);
        assert_eq!(calls_to(&e, START).len(), 1);
        assert_eq!(e.get(controller, BhkBlendController::iPriority), 2);
        // The low byte of the last argument selects the other table (capped by the gains).
        e.set(controller, BhkBlendController::iPriority, 0);
        e.call(0x00c9b250, &args![scene.object, 0.0f32, 0x101u32]);
        assert_eq!(controller_keys(&e, controller)[0], [0.0, 0.5, 0.1]);
        assert_eq!(controller_keys(&e, controller)[1], [0.125, 0.6, 0.1]);
        // Without a collision object nothing is looked up.
        e.mem.set_u32(scene.object + 0x1c, 0);
        let kinds = calls_to(&e, IS_KIND_OF).len();
        e.call(0x00c9b250, &args![scene.object, 0.0f32, 0u32]);
        assert_eq!(calls_to(&e, IS_KIND_OF).len(), kinds);
    }

    #[test]
    fn do_hit_visits_the_children_of_a_node() {
        let mut e = engine();
        let scene = blend_scene(&mut e, 3);
        put_floats(&mut e, HIT_GAINS, &[0.0; 8]);
        put_floats(&mut e, HIT_KEY_TIME_A, &[0.125]);
        put_floats(&mut e, HIT_KEY_TIME_B, &[0.5]);
        put_floats(&mut e, START_TIME, &[0.0625]);
        e.register(0x7000_0005, |_, a| a[0].into_ret());
        put_vtable_at(&mut e, 0x7001_1000, &[(0xc, 0x7000_0005)]);
        let parent = e.mem.alloc(0xb0);
        let children = e.mem.alloc(8);
        e.mem.set_u32(parent, 0x7001_1000);
        e.mem.set_u32(parent + 0xa0, children);
        e.mem.set_u16(parent + 0xa6, 1);
        e.mem.set_u32(children, scene.object);
        e.call(0x00c9b250, &args![parent, 0.0f32, 0u32]);
        assert_eq!(controller_keys(&e, scene.controller).len(), 3);
        assert_eq!(calls_to(&e, START).len(), 1);
    }

    #[test]
    fn get_up_keys_two_gains_and_starts_the_controller() {
        let mut e = engine();
        let scene = blend_scene(&mut e, 3);
        e.register(SET_MOTION_FLAGS, |_, _| Ret::default());
        let mut gains = vec![0.0f32; 8];
        gains[3] = 0.75;
        put_floats(&mut e, GET_UP_GAINS, &gains);
        let mut targets = vec![0.0f32; 8];
        targets[6] = 0.2;
        targets[7] = 0.4;
        put_floats(&mut e, GET_UP_TARGETS, &targets);
        put_floats(&mut e, START_TIME, &[0.0625]);
        e.mem.set_u16(scene.controller.addr() + 8, 0xffff);
        e.call(0x00c9b470, &args![scene.object, 2.0f32]);
        let controller = scene.controller;
        let flags = calls_to(&e, SET_MOTION_FLAGS);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0][1..], [0x308]);
        assert_eq!(
            controller_keys(&e, controller),
            vec![[0.0, 0.0, 0.0], [0.75, 0.2, 0.4]]
        );
        assert_eq!(e.get(controller, BhkBlendController::iPriority), 2);
        assert_eq!(e.get(controller, BhkBlendController::m_fLoKeyTime), 2.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fHiKeyTime), 0.75);
        assert_eq!(e.get(controller, BhkBlendController::m_fPhase), 0.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fFrequency), 1.0);
        assert_eq!(
            e.get(controller, BhkBlendController::m_uFlags),
            0xfef5 | 0xc5
        );
        assert_eq!(calls_to(&e, START).len(), 1);
        // A negative gain leaves the controller alone.
        gains[3] = -0.5;
        put_floats(&mut e, GET_UP_GAINS, &gains);
        e.set(controller, BhkBlendController::iPriority, 0);
        e.call(0x00c9b470, &args![scene.object, 2.0f32]);
        assert_eq!(e.get(controller, BhkBlendController::iPriority), 0);
        assert_eq!(calls_to(&e, START).len(), 1);
    }

    /// A physics object (`012043f8`) in place of a blend collision object: the object's
    /// collision object finds a physics object whose body (`+0x10`) records its virtual `+0xe4`.
    fn physics_scene(e: &mut Engine, flags: u32) -> (u32, u32) {
        e.register(IS_KIND_OF, |_, a| {
            let kind = if a[1] == PHYSICS_OBJECT_RTTI {
                0x7200_0000
            } else {
                0
            };
            let _ = a;
            kind.into_ret()
        });
        e.map(FLAGS_WORD, 4);
        e.mem.set_u32(FLAGS_WORD, flags);
        e.register(READ_OBJECT_FLAGS, |_, _| FLAGS_WORD.into_ret());
        e.register(BODY_SLOT_E4, |_, _| Ret::default());
        e.register(0x7000_0003, |_, _| Ret::default());
        put_vtable_at(e, 0x7001_0000, &[(0xc, 0x7000_0003)]);
        put_vtable_at(e, 0x7001_4000, &[(0xe4, BODY_SLOT_E4)]);
        e.map(0x7200_0000, 0x20);
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body, 0x7001_4000);
        e.mem.set_u32(0x7200_0010, body);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, 0x7001_0000);
        e.mem.set_u32(object + 0x1c, 0x7300_0000);
        (object, body)
    }

    #[test]
    fn get_up_calls_the_body_of_a_physics_object_unless_it_is_class_8_or_1d() {
        let mut e = engine();
        let (object, body) = physics_scene(&mut e, 5);
        e.call(0x00c9b470, &args![object, 1.0f32]);
        assert_eq!(calls_to(&e, BODY_SLOT_E4), vec![vec![body, 4]]);
        // Class 8 with the 0x1f00 bits not 0x1400: skipped. With 0x1400: called.
        e.mem.set_u32(FLAGS_WORD, 0x0308);
        e.call(0x00c9b470, &args![object, 1.0f32]);
        assert_eq!(calls_to(&e, BODY_SLOT_E4).len(), 1);
        e.mem.set_u32(FLAGS_WORD, 0x141d);
        e.call(0x00c9b470, &args![object, 1.0f32]);
        assert_eq!(calls_to(&e, BODY_SLOT_E4).len(), 2);
    }

    #[test]
    fn knock_down_keys_two_gains_and_pushes_the_object() {
        let mut e = engine();
        let scene = blend_scene(&mut e, 3);
        e.register(SYNCHRONIZE_MOTION_TYPE, |_, _| Ret::default());
        e.register(POINT_SCALE, |_, a| a[1].into_ret());
        e.register(VECTOR_FROM_POINT, |_, a| a[0].into_ret());
        e.register(APPLY_FORCE_VECTOR, |_, _| Ret::default());
        let mut gains = vec![0.0f32; 8];
        gains[3] = 0.9;
        put_floats(&mut e, KNOCK_DOWN_GAINS, &gains);
        put_floats(&mut e, START_TIME, &[0.0625]);
        put_floats(&mut e, PASS_OUT_FORCE_SCALE, &[2.0, 3.0]);
        let position = e.mem.alloc(12);
        let controller = scene.controller;
        e.mem.set_u16(controller.addr() + 8, 0);
        e.call(
            0x00c9b670,
            &args![scene.object, position, 0x1234u32, 3.0f32, 0u32],
        );
        assert_eq!(
            calls_to(&e, SYNCHRONIZE_MOTION_TYPE),
            vec![vec![scene.collision]]
        );
        assert_eq!(
            controller_keys(&e, controller),
            vec![[0.0, 0.5, 0.25], [0.9, 0.0, 0.0]]
        );
        assert_eq!(e.get(controller, BhkBlendController::iPriority), 2);
        assert_eq!(e.get(controller, BhkBlendController::m_fLoKeyTime), 3.0);
        assert_eq!(e.get(controller, BhkBlendController::m_fHiKeyTime), 0.9);
        assert_eq!(e.get(controller, BhkBlendController::m_uFlags), 0xc5);
        assert_eq!(calls_to(&e, START).len(), 1);
        // No force controller: the object is pushed by the scaled position.
        let scales = calls_to(&e, POINT_SCALE);
        assert_eq!(scales.len(), 1);
        assert_eq!(scales[0][0], position);
        assert_eq!(scales[0][2], 2.0f32.to_bits());
        let pushes = calls_to(&e, APPLY_FORCE_VECTOR);
        assert_eq!(pushes.len(), 1);
        assert_eq!(pushes[0][0], scene.object);
        assert_eq!(pushes[0][2], 3.0f32.to_bits());
        // With the low byte of the last argument set, the collision object's virtual +0xb0 is
        // called with (1, 0, 0) and nothing else happens.
        e.call(
            0x00c9b670,
            &args![scene.object, position, 0u32, 3.0f32, 0x100u32],
        );
        assert!(calls_to(&e, COLLISION_SLOT_B0).is_empty());
        e.call(
            0x00c9b670,
            &args![scene.object, position, 0u32, 3.0f32, 1u32],
        );
        assert_eq!(
            calls_to(&e, COLLISION_SLOT_B0),
            vec![vec![scene.collision, 1, 0, 0]]
        );
    }

    #[test]
    fn knock_down_applies_a_force_to_the_body_of_a_physics_object() {
        let mut e = engine();
        let (object, body) = physics_scene(&mut e, 5);
        e.register(POINT_SCALE, |_, a| a[1].into_ret());
        e.register(VECTOR_FROM_POINT, |_, a| a[0].into_ret());
        e.register(RIGID_BODY_APPLY_FORCE, |_, _| Ret::default());
        put_floats(&mut e, PASS_OUT_FORCE_SCALE, &[2.0]);
        put_floats(&mut e, PASS_OUT_TIME, &[0.5]);
        let position = e.mem.alloc(12);
        e.mem.set_u32(body + 8, 0x7400_0000);
        e.call(0x00c9b670, &args![object, position, 0u32, 3.0f32, 0u32]);
        assert_eq!(calls_to(&e, BODY_SLOT_E4), vec![vec![body, 1]]);
        let forces = calls_to(&e, RIGID_BODY_APPLY_FORCE);
        assert_eq!(forces.len(), 1);
        assert_eq!(forces[0][0], 0x7400_0000);
        assert_eq!(forces[0][1], 0.5f32.to_bits());
        // With the low byte of the last argument set only the virtual call is made.
        e.call(0x00c9b670, &args![object, position, 0u32, 3.0f32, 1u32]);
        assert_eq!(calls_to(&e, BODY_SLOT_E4).len(), 2);
        assert_eq!(calls_to(&e, RIGID_BODY_APPLY_FORCE).len(), 1);
    }

    #[test]
    fn copy_members_appends_every_key_to_the_clone() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let keys = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let source = make_controller(&mut e, &keys, 2);
        e.set(source, BhkBlendController::iLastIndex, 1);
        let clone = make_controller(&mut e, &[], 0);
        let process = e.mem.alloc(4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x66);
        e.call(0x00c9b970, &args![source, clone, process]);
        assert_eq!(controller_keys(&e, clone), keys.to_vec());
        // The index is copied first, then every key added resets it (AddKey).
        assert_eq!(e.get(clone, BhkBlendController::iLastIndex), 0);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![source.addr(), clone.addr(), process]]
        );
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x66);
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let keys = [[1.0, 2.0, 3.0]];
        let source = make_controller(&mut e, &keys, 1);
        let process = e.mem.alloc(4);
        let clone = e.call(0x00c9bef0, &args![source, process]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), 0x010c_6b14);
        assert_eq!(controller_keys(&e, Ptr::new(clone)), keys.to_vec());
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![source.addr(), clone, process]]
        );
    }

    #[test]
    fn viewer_strings_list_the_class_the_count_and_each_key() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.map(BLEND_CONTROLLER_RTTI, 4);
        e.mem.set_u32(BLEND_CONTROLLER_RTTI, 0x4444);
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(GET_VIEWER_STRING_INT, |_, _| 0x2222u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        // The store the real array does: the slot, the size and the used count follow.
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, (a[1] + 1) as u16);
            e.mem.set_u16(a[0] + 0xc, (a[1] + 1) as u16);
            Ret::default()
        });
        e.register(SPRINTF, |e, a| {
            e.mem.set_cstr(a[0], b"Key");
            Ret::default()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let controller = make_controller(&mut e, &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], 2);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 16);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00c9bbb0, &args![controller, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4444]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_INT),
            vec![vec![KEYS_LABEL, 2]]
        );
        let formats = calls_to(&e, SPRINTF);
        assert_eq!(formats.len(), 2);
        assert_eq!(formats[1][1..4], [0x40, KEY_FORMAT, 1]);
        let doubles: Vec<f64> = (0..3)
            .map(|lane| {
                f64::from_bits(
                    u64::from(formats[1][4 + 2 * lane]) | u64::from(formats[1][5 + 2 * lane]) << 32,
                )
            })
            .collect();
        assert_eq!(doubles, vec![4.0, 5.0, 6.0]);
        assert_eq!(e.mem.u16(strings + 0xa), 4);
        assert_eq!(e.mem.u16(strings + 0xc), 4);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(base + 4), 0x2222);
        assert_eq!(e.mem.cstr(e.mem.u32(base + 8)), b"Key");
        assert_eq!(e.mem.cstr(e.mem.u32(base + 12)), b"Key");
        // The exe's frame: the print buffer on a 64-byte boundary at +0x3c, the string slot
        // at +0x28 (0x14 below it), the same buffer for every key.
        let buffer = formats[0][0];
        assert_eq!(buffer & 0x3f, 0);
        assert_eq!(formats[1][0], buffer);
        let slots: Vec<u32> = calls_to(&e, STRING_ARRAY_SET_AT)
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(slots, vec![buffer - 0x14; 2]);
    }

    #[test]
    fn node_get_up_refreshes_the_body_under_the_first_grandchild() {
        let mut e = engine();
        // Four small functions of `engine-fallout` this test passes through, which this
        // crate's tests do not link since the engine split: doubles with the bodies of
        // their translations for the case reached here. `0044ddc0`: the word at +8;
        // `00620b80`: `0044ddc0(this)`; `008c71b0`: stores the word and answers the
        // address; `0043b4f0`: stores into `out` the value of the holder `00620b80`
        // finds, 0 without one (the only case here), and answers `out`.
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0062_0b80, |e, a| e.call(0x0044_ddc0, &[a[0]]));
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(READ_OBJECT_FLAGS, |e, a| {
            let holder = e.call(0x0062_0b80, &[a[0]]).u32();
            assert_eq!(
                holder, 0,
                "the double covers only an object without a holder"
            );
            e.call(0x008c_71b0, &[a[1], 0]);
            a[1].into_ret()
        });
        e.register(SET_MOTION_FLAGS, |_, _| Ret::default());
        e.register(PHYSICS_REFRESH_A, |_, _| Ret::default());
        e.register(PHYSICS_REFRESH_B, |_, _| Ret::default());
        // Every object is a node (its RTTI is NiNode's) with the given children; the
        // grandchild's collision object is a physics object whose body records virtual +0xe4.
        e.register(0x7000_0008, |_, _| NODE_RTTI.into_ret());
        e.register(0x7000_0005, |_, a| a[0].into_ret());
        e.register(BODY_SLOT_E4, |_, _| Ret::default());
        e.register(IS_KIND_OF, |_, a| {
            let kind = if a[1] == PHYSICS_OBJECT_RTTI {
                0x7200_0000
            } else {
                0
            };
            kind.into_ret()
        });
        put_vtable_at(
            &mut e,
            0x7001_1000,
            &[(0x8, 0x7000_0008), (0xc, 0x7000_0005)],
        );
        put_vtable_at(&mut e, 0x7001_4000, &[(0xe4, BODY_SLOT_E4)]);
        e.map(0x7200_0000, 0x20);
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body, 0x7001_4000);
        e.mem.set_u32(0x7200_0010, body);
        let node = |e: &mut Engine| {
            let node = e.mem.alloc(0xb0);
            e.mem.set_u32(node, 0x7001_1000);
            node
        };
        let (parent, child, grandchild) = (node(&mut e), node(&mut e), node(&mut e));
        for (owner, member) in [(parent, child), (child, grandchild)] {
            let children = e.mem.alloc(4);
            e.mem.set_u32(children, member);
            e.mem.set_u32(owner + 0xa0, children);
            e.mem.set_u16(owner + 0xa6, 1);
        }
        e.mem.set_u32(grandchild + 0x1c, 0x7300_0000);
        e.mem.set_u16(grandchild + 0xa6, 0);
        e.call(0x00c9bdf0, &args![parent, 1.0f32]);
        let transform = IDENTITY_TRANSFORM;
        assert_eq!(calls_to(&e, PHYSICS_REFRESH_A), vec![vec![body, transform]]);
        assert_eq!(calls_to(&e, PHYSICS_REFRESH_B), vec![vec![body, transform]]);
        // Once from 00c9bdf0, then again by 00c9b470 on the grandchild.
        assert!(calls_to(&e, BODY_SLOT_E4)
            .iter()
            .all(|call| call == &vec![body, 4]));
        assert!(!calls_to(&e, BODY_SLOT_E4).is_empty());
        // A first child that is not a node: only 00c9b470 runs.
        e.register(0x7000_0009, |_, _| 0u32.into_ret());
        put_vtable_at(
            &mut e,
            0x7001_5000,
            &[(0x8, 0x7000_0009), (0xc, 0x7000_0005)],
        );
        e.mem.set_u32(child, 0x7001_5000);
        let before = calls_to(&e, PHYSICS_REFRESH_A).len();
        e.call(0x00c9bdf0, &args![parent, 1.0f32]);
        assert_eq!(calls_to(&e, PHYSICS_REFRESH_A).len(), before);
    }
}
