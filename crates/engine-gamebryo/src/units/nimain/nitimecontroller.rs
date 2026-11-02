//! `NiMain/nitimecontroller.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;
use crate::units::nimain::nimainsdm::{interlocked_decrement, interlocked_increment};
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiTimeController::ms_RTTI` (the class's `NiRTTI` record).
const NI_TIME_CONTROLLER_RTTI: u32 = 0x011f_49e4;

// Translated from 00a6d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_time_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TIME_CONTROLLER_RTTI)
}

layout! {
    /// `NiTimeController` (Xbox PDB), 0x34 bytes on PC as on the Xbox: an `NiObject`
    /// (vtable, reference count) and the controller's timing and links. The float fields are
    /// declared as their bit patterns (the code copies them through the x87 stack).
    pub struct NiTimeController: 0x34 {
        /// `NiRefObject::m_uiRefCount` (Xbox PDB).
        0x04 m_uiRefCount: u32,
        /// `m_uFlags` (Xbox PDB).
        0x08 m_uFlags: u16,
        /// `m_fFrequency` (Xbox PDB).
        0x0C m_fFrequency: u32,
        /// `m_fPhase` (Xbox PDB).
        0x10 m_fPhase: u32,
        /// `m_fLoKeyTime` (Xbox PDB).
        0x14 m_fLoKeyTime: u32,
        /// `m_fHiKeyTime` (Xbox PDB).
        0x18 m_fHiKeyTime: u32,
        /// `m_fStartTime` (Xbox PDB).
        0x1C m_fStartTime: u32,
        /// `m_fLastTime` (Xbox PDB).
        0x20 m_fLastTime: u32,
        /// `m_fWeightedLastTime` (Xbox PDB).
        0x24 m_fWeightedLastTime: u32,
        /// `m_fScaledTime` (Xbox PDB).
        0x28 m_fScaledTime: u32,
        /// `m_pkTarget` (Xbox PDB): `NiObjectNET *`.
        0x2C m_pkTarget: u32,
        /// `m_spNext` (Xbox PDB): `NiPointer<NiTimeController>`.
        0x30 m_spNext: u32,
    }
}

/// `NiTimeController`'s vtable.
const NI_TIME_CONTROLLER_VTABLE: u32 = 0x0109_c874;
/// The float `-FLT_MAX` (`ff7fffff`) the code loads as "no time yet".
const NO_TIME_FLOAT: u32 = 0x0101_5f5c;
/// The float `FLT_MAX` (`7f7fffff`) the constructor gives the low key time.
const MAX_FLOAT: u32 = 0x0109_c820;
/// The double `-FLT_MAX` the scaled-time computation compares the start, last and high key
/// times with.
const NO_TIME_DOUBLE: u32 = 0x0102_41b0;
/// The double `FLT_MAX` it compares the low key time with.
const MAX_DOUBLE: u32 = 0x0102_31b0;
/// `NiAVObject::ms_RTTI` (the class's `NiRTTI` record).
const NI_AV_OBJECT_RTTI: u32 = 0x011f_4280;
/// `NiObject`'s constructor and destructor body (`thiscall`, no stack words).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::ProcessClone` (`thiscall`, the cloning process).
const OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `00a5d450` (unit `niobject.cpp`, `thiscall`, (clone, cloning process)): the base
/// object's member copy.
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::LoadBinary` (`thiscall`, the stream).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a53c30` (`thiscall` on the clone map, (key, &value)): the map lookup; answers whether
/// the key is there (`AL`).
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// `00a5c400` (unit `niobjectnet.cpp`, `thiscall` on the target, the controller): adds a
/// controller to the target's list.
const TARGET_ADD_CONTROLLER: u32 = 0x00a5_c400;
/// `NiObjectNET::RemoveController` (`thiscall` on the target, the controller).
const TARGET_REMOVE_CONTROLLER: u32 = 0x00a5_c480;
/// `NiStream::GetObjectFromLinkID` (`thiscall`, no stack words).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a645b0` (unit `nistream.cpp`, `thiscall` on the stream, no stack words).
const STREAM_00A645B0: u32 = 0x00a6_45b0;
/// `NiGeometryData::SetConsistency` (`thiscall`, one word).
const GEOMETRY_DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
/// `_CIfmod` (`ST1` modulo `ST0`, both doubles; answers in `ST0`).
const CI_FMOD: u32 = 0x00ec_913a;

/// Reads `size` bytes from `stream`'s input (`m_pkIstr`) into `destination`: the input's read
/// function is called as `function(input, destination, size, &count, 1)` with the stack word
/// `count` set to `size`, and the byte count it answers is added to the input's position.
fn stream_read(e: &mut Engine, stream: Ptr<NiStream>, destination: u32, size: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        e.call(function, &args![input, destination, size, count, 1u32])
            .u32()
    });
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// Releases a counted object (count at `+4`): when the decrement reaches 0 its virtual slot
/// `+4` (`DeleteThis`) runs.
fn release(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The compiler's `NiPointer` assignment to the word at `slot`: nothing when `new` is already
/// there; otherwise the old object (when there is one) is released, `new` stored and counted.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        interlocked_increment(e, new.wrapping_add(4));
    }
}

// Translated from 00a6cd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::Start` (Xbox PDB): sets flag bit 3 (`0x8`), sets the last time to
/// `-FLT_MAX` (the float at `01015f5c`) and, when flag bit 0 is set, the start time to `time`.
pub fn ni_time_controller_start(e: &mut Engine, this: Ptr<NiTimeController>, time: f32) {
    let flags = e.get(this, NiTimeController::m_uFlags) | 0x8;
    e.set(this, NiTimeController::m_uFlags, flags);
    let no_time = x87_store32(x87_load32(e.mem.u32(NO_TIME_FLOAT)));
    e.set(this, NiTimeController::m_fLastTime, no_time);
    if e.get(this, NiTimeController::m_uFlags) & 1 != 0 {
        let start = x87_store32(x87_load32(time.to_bits()));
        e.set(this, NiTimeController::m_fStartTime, start);
    }
}

// Translated from 00a6cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::ComputeScaledTime` (Xbox PDB): the controller's time for the
/// application time `time`. The first call takes `time` as the start time (when the start
/// time is the double `-FLT_MAX` at `010241b0`); the elapsed time since the last call (on the
/// first call 0 when flag bit 0 is set, `time` itself otherwise) times the frequency is
/// added to the weighted last time, then the phase. A result cache in the thread's TLS block
/// (`+0x2a0` result, `+0x2a4` cycle type, `+0x2a8` input, `+0x2ac` low and `+0x2b0` high key
/// time) answers the cached result when all four inputs match. Otherwise the cycle type
/// (flag bits 1-2) folds the time into the key range: 0 loops it (`fmod` of the time past the
/// low key over the range), 1 mirrors it (`fmod` over twice the range, folded back), any
/// other type keeps it; a range of 0 gives the low key time, and an unset key range (high
/// key `-FLT_MAX`, or low key the double `FLT_MAX` at `010231b0`) keeps the time. The time is
/// then clamped to the key range and, with flag bit 4 set, reversed in it; it is cached and
/// answered.
pub fn ni_time_controller_compute_scaled_time(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    time: f32,
) -> f32 {
    let time_value = x87_load32(time.to_bits());
    let no_time = x87_load64(e.mem.u64(NO_TIME_DOUBLE));
    let start = x87_load32(e.get(this, NiTimeController::m_fStartTime));
    if no_time == start {
        e.set(
            this,
            NiTimeController::m_fStartTime,
            x87_store32(time_value),
        );
    }
    let last = e.get(this, NiTimeController::m_fLastTime);
    let delta = if x87_load32(last) == no_time {
        e.set(
            this,
            NiTimeController::m_fWeightedLastTime,
            x87_store32(0.0),
        );
        if e.get(this, NiTimeController::m_uFlags) & 1 != 0 {
            x87_store32(0.0)
        } else {
            x87_store32(time_value)
        }
    } else {
        x87_store32(x87_sub(time_value, x87_m32(last)))
    };
    let frequency = x87_load32(e.get(this, NiTimeController::m_fFrequency));
    let flags = e.get(this, NiTimeController::m_uFlags);
    let weighted = e.get(this, NiTimeController::m_fWeightedLastTime);
    let weighted = x87_store32(x87_add(
        x87_mul(frequency, x87_m32(delta)),
        x87_m32(weighted),
    ));
    let weighted_value = x87_load32(weighted);
    e.set(
        this,
        NiTimeController::m_fWeightedLastTime,
        x87_store32(weighted_value),
    );
    let phase = e.get(this, NiTimeController::m_fPhase);
    let scaled = x87_store32(x87_add(weighted_value, x87_m32(phase)));
    let cycle = u32::from(flags >> 1) & 3;
    e.set(this, NiTimeController::m_fLastTime, x87_store32(time_value));
    let tls = e.tls();
    let scaled_value = x87_load32(scaled);
    let hit = x87_load32(e.get(this, NiTimeController::m_fHiKeyTime))
        == x87_load32(e.mem.u32(tls + 0x2b0))
        && x87_load32(e.get(this, NiTimeController::m_fLoKeyTime))
            == x87_load32(e.mem.u32(tls + 0x2ac))
        && scaled_value == x87_load32(e.mem.u32(tls + 0x2a8))
        && e.mem.u32(tls + 0x2a4) == cycle;
    if hit {
        return f32::from_bits(x87_quiet_bits(e.mem.u32(tls + 0x2a0)));
    }
    let high_bits = e.get(this, NiTimeController::m_fHiKeyTime);
    e.mem.set_u32(tls + 0x2a4, cycle);
    e.mem
        .set_u32(tls + 0x2b0, x87_store32(x87_load32(high_bits)));
    let low_bits = e.get(this, NiTimeController::m_fLoKeyTime);
    e.mem
        .set_u32(tls + 0x2ac, x87_store32(x87_load32(low_bits)));
    e.mem.set_u32(tls + 0x2a8, x87_store32(scaled_value));
    let mut result = scaled_value;
    let max = f64::from_bits(e.mem.u64(MAX_DOUBLE));
    if !(x87_load32(high_bits) == no_time) && !(x87_load32(low_bits) == max) {
        match cycle {
            0 => {
                let range = x87_store32(x87_sub(x87_load32(high_bits), x87_m32(low_bits)));
                let range_value = x87_load32(range);
                if range_value == 0.0 {
                    result = x87_load32(x87_store32(x87_load32(low_bits)));
                } else {
                    let past_low =
                        x87_load32(x87_store32(x87_sub(scaled_value, x87_load32(low_bits))));
                    let remainder = e.call(CI_FMOD, &args![past_low, range_value]).f64();
                    let remainder = x87_load32(x87_store32(remainder));
                    let mut folded = x87_load32(x87_store32(x87_add(remainder, x87_m32(low_bits))));
                    if x87_load32(low_bits) > folded {
                        folded = x87_load32(x87_store32(x87_add(folded, x87_m32(range))));
                    }
                    result = folded;
                }
            }
            1 => {
                let range = x87_store32(x87_sub(x87_load32(high_bits), x87_m32(low_bits)));
                let range_value = x87_load32(range);
                if range_value == 0.0 {
                    result = x87_load32(x87_store32(x87_load32(low_bits)));
                } else {
                    let double_range = x87_store32(x87_add(range_value, range_value));
                    let remainder = e
                        .call(CI_FMOD, &args![scaled_value, x87_load32(double_range)])
                        .f64();
                    let mut folded = x87_load32(x87_store32(remainder));
                    if folded < 0.0 {
                        folded = x87_load32(x87_store32(x87_add(folded, x87_m32(double_range))));
                    }
                    if range_value < folded {
                        folded = x87_sub(x87_m32(double_range), folded);
                    }
                    result = x87_load32(x87_store32(x87_add(folded, x87_m32(low_bits))));
                }
            }
            _ => {}
        }
    }
    let high = x87_load32(e.get(this, NiTimeController::m_fHiKeyTime));
    if high < result {
        result = x87_load32(x87_store32(high));
    } else {
        let low = x87_load32(e.get(this, NiTimeController::m_fLoKeyTime));
        if low > result {
            result = x87_load32(x87_store32(low));
        }
    }
    if flags & 0x10 != 0 {
        let low = x87_load32(e.get(this, NiTimeController::m_fLoKeyTime));
        let high = e.get(this, NiTimeController::m_fHiKeyTime);
        result = x87_load32(x87_store32(x87_sub(x87_m32(high), x87_sub(result, low))));
    }
    let stored = x87_store32(result);
    e.mem.set_u32(tls + 0x2a0, stored);
    f32::from_bits(stored)
}

// Translated from 00a6cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitimecontroller.cpp`; by its body the controller's
/// member copy): the base object's copy (`00a5d450`), then the flags and the times from the
/// frequency to the weighted last time (not the scaled time) into `clone`. A target is copied
/// as its clone from the cloning process's clone map (`00a53c30` on `*cloning_process`) when
/// the map has one of the same class (the two `GetRTTI` slot `+8` answers equal), as null
/// otherwise. The next controller is cloned (its `CreateClone`, slot `+0x48`) into the
/// clone's counted `m_spNext`.
pub fn fn_00a6cfe0(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    clone: Ptr<NiTimeController>,
    cloning_process: Ptr,
) {
    e.call(OBJECT_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.get(this, NiTimeController::m_uFlags);
    e.set(clone, NiTimeController::m_uFlags, flags);
    for offset in [0x0cu32, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24] {
        let value = x87_store32(x87_load32(e.mem.u32(this.addr() + offset)));
        e.mem.set_u32(clone.addr() + offset, value);
    }
    let target = e.get(this, NiTimeController::m_pkTarget);
    if target != 0 {
        let map = e.mem.u32(cloning_process.addr());
        let cloned_target = e.with_stack(4, |e, out| {
            e.mem.set_u32(out.addr(), clone.addr());
            let found = e.call(CLONE_MAP_GET_AT, &args![map, target, out]).bool();
            if !found {
                return 0;
            }
            let cloned = e.mem.u32(out.addr());
            let cloned_rtti = e.vcall(cloned, 8, &args![]).u32();
            let target = e.get(this, NiTimeController::m_pkTarget);
            let target_rtti = e.vcall(target, 8, &args![]).u32();
            if target_rtti == cloned_rtti {
                cloned
            } else {
                0
            }
        });
        e.set(clone, NiTimeController::m_pkTarget, cloned_target);
    }
    let next = e.get(this, NiTimeController::m_spNext);
    if next != 0 {
        let cloned_next = e.vcall(next, 0x48, &args![cloning_process]).u32();
        assign_pointer(
            e,
            clone.addr() + NiTimeController::m_spNext.off,
            cloned_next,
        );
    }
}

// Translated from 00a6d0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`, then the next
/// controller's `ProcessClone` (slot `+0x68`) when there is one.
pub fn ni_time_controller_process_clone(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    cloning_process: Ptr,
) {
    e.call(OBJECT_PROCESS_CLONE, &args![this, cloning_process]);
    let next = e.get(this, NiTimeController::m_spNext);
    if next != 0 {
        e.vcall(next, 0x68, &args![cloning_process]);
    }
}

// Translated from 00a6d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitimecontroller.cpp`; constructor-shaped): the
/// `NiObject` constructor, then frequency 1, the flags 0, phase 0, no target, the vtable
/// `0109c874`, the low key time `FLT_MAX` (`0109c820`), the high key, start, last and scaled
/// times `-FLT_MAX` (`01015f5c`), the weighted last time 0, no next controller, and the
/// flags' bits 0, 1, 4 and 7 cleared and bits 2, 3 and 6 set (`0x4c`). Answers `this`.
pub fn fn_00a6d220(e: &mut Engine, this: Ptr<NiTimeController>) -> Ptr<NiTimeController> {
    e.call(OBJECT_CONSTRUCT, &args![this]);
    e.set(this, NiTimeController::m_fFrequency, 1.0f32.to_bits());
    e.set(this, NiTimeController::m_uFlags, 0);
    e.set(this, NiTimeController::m_fPhase, 0);
    e.set(this, NiTimeController::m_pkTarget, 0);
    let max = x87_store32(x87_load32(e.mem.u32(MAX_FLOAT)));
    e.mem.set_u32(this.addr(), NI_TIME_CONTROLLER_VTABLE);
    e.set(this, NiTimeController::m_fLoKeyTime, max);
    let no_time = x87_store32(x87_load32(e.mem.u32(NO_TIME_FLOAT)));
    e.set(this, NiTimeController::m_fHiKeyTime, no_time);
    e.set(this, NiTimeController::m_fStartTime, no_time);
    e.set(this, NiTimeController::m_fLastTime, no_time);
    e.set(this, NiTimeController::m_fScaledTime, no_time);
    e.set(this, NiTimeController::m_fWeightedLastTime, 0);
    e.set(this, NiTimeController::m_spNext, 0);
    let flags = (e.get(this, NiTimeController::m_uFlags) & 0xff6c) | 0x4c;
    e.set(this, NiTimeController::m_uFlags, flags);
    this
}

// Translated from 00a6d290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitimecontroller.cpp`; destructor-shaped): stores the
/// vtable `0109c874`, releases the next controller (when there is one; the word is left as
/// it is) and runs the `NiObject` destructor body (`00a5d3d0`).
pub fn fn_00a6d290(e: &mut Engine, this: Ptr<NiTimeController>) {
    e.mem.set_u32(this.addr(), NI_TIME_CONTROLLER_VTABLE);
    let next = e.get(this, NiTimeController::m_spNext);
    if next != 0 {
        release(e, next);
    }
    e.call(OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a6d2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::StartAnimations` (Xbox PDB; `cdecl`, the object): starts every
/// controller of `object`'s list (`NiObjectNET::m_spControllers` +0xc, linked through
/// `m_spNext`) with the time `-FLT_MAX` (`Start`, slot `+0x8c`). When the object is an
/// `NiAVObject` (its `GetRTTI`, slot `+8`, or a base of it is `011f4280`) it recurses into
/// each property of its list (+0x24: items `{next, previous, data}`) that has controllers;
/// when it is a node (`IsNode`, slot `+0xc`) into each non-null child of its child array
/// (`NiNode::m_kChildren`, base +0xa0, count +0xa6).
pub fn ni_time_controller_start_animations(e: &mut Engine, object: Ptr) {
    let mut controller = e.mem.u32(object.addr() + 0xc);
    while controller != 0 {
        let start = x87_store32(x87_load32(e.mem.u32(NO_TIME_FLOAT)));
        e.vcall(controller, 0x8c, &args![start]);
        controller = e.mem.u32(controller + 0x30);
    }
    let mut rtti = e.vcall(object.addr(), 8, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_AV_OBJECT_RTTI {
            let mut item = e.mem.u32(object.addr() + 0x24);
            while item != 0 {
                let property = e.mem.u32(item + 8);
                item = e.mem.u32(item);
                if property != 0 && e.mem.u32(property + 0xc) != 0 {
                    ni_time_controller_start_animations(e, Ptr::new(property));
                }
            }
            break;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    if e.vcall(object.addr(), 0xc, &args![]).u32() != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(object.addr() + 0xa6)) {
            let child = e.mem.u32(e.mem.u32(object.addr() + 0xa0) + index * 4);
            if child != 0 {
                ni_time_controller_start_animations(e, Ptr::new(child));
            }
            index += 1;
        }
    }
}

// Translated from 00a6d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::SetTarget` (Xbox PDB): nothing when `target` is already the target.
/// Otherwise, holding a count on itself, the controller leaves the old target's controllers
/// (`NiObjectNET::RemoveController`, when the old target has any), takes `target` and, unless
/// it is already in the new target's list, adds itself (`00a5c400`) and, when it is a vertex
/// controller (slot `+0xa0`), clears the consistency of the target's geometry data (+0xb8,
/// `NiGeometryData::SetConsistency(0)`). The held count is released at the end (slot `+4`
/// when it reaches 0).
pub fn ni_time_controller_set_target(e: &mut Engine, this: Ptr<NiTimeController>, target: Ptr) {
    if e.get(this, NiTimeController::m_pkTarget) == target.addr() {
        return;
    }
    interlocked_increment(e, this.addr() + 4);
    let old = e.get(this, NiTimeController::m_pkTarget);
    if old != 0 && e.mem.u32(old + 0xc) != 0 {
        e.call(TARGET_REMOVE_CONTROLLER, &args![old, this]);
    }
    e.set(this, NiTimeController::m_pkTarget, target.addr());
    if target.addr() != 0 {
        let mut controller = e.mem.u32(target.addr() + 0xc);
        while controller != 0 {
            if controller == this.addr() {
                release(e, this.addr());
                return;
            }
            controller = e.mem.u32(controller + 0x30);
        }
        e.call(TARGET_ADD_CONTROLLER, &args![target, this]);
        if e.vcall(this.addr(), 0xa0, &args![]).bool() {
            let target = e.get(this, NiTimeController::m_pkTarget);
            let data = e.mem.u32(target + 0xb8);
            e.call(GEOMETRY_DATA_SET_CONSISTENCY, &args![data, 0u32]);
        }
    }
    release(e, this.addr());
}

// Translated from 00a6d500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::LinkObject` (Xbox PDB): the next controller and the target from the
/// stream's link ids (`NiStream::GetObjectFromLinkID`, the next one counted into
/// `m_spNext`). For files older than version `0x0a000110` (`NiStream` +0xd8) a vertex
/// controller (slot `+0xa0`) with a target clears the consistency of the target's geometry
/// data (+0xb8, `NiGeometryData::SetConsistency(0)`).
pub fn ni_time_controller_link_object(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    stream: Ptr<NiStream>,
) {
    let next = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + NiTimeController::m_spNext.off, next);
    let target = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiTimeController::m_pkTarget, target);
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a00_0110
        && e.vcall(this.addr(), 0xa0, &args![]).bool()
    {
        let target = e.get(this, NiTimeController::m_pkTarget);
        if target != 0 {
            let data = e.mem.u32(target + 0xb8);
            e.call(GEOMETRY_DATA_SET_CONSISTENCY, &args![data, 0u32]);
        }
    }
}

// Translated from 00a6d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the stream's
/// `00a645b0`, the flags (2 bytes), frequency, phase, low and high key times (4 bytes each)
/// read straight into the fields, `00a645b0` again, then the flags brought to the current
/// layout: before version `0x0a000101` bits 4 and up move up one; before `0x0a000103` the
/// flags are kept in the stream's word at +0x286 and cut to bits 0-4; before `0x0a01006d`
/// bit 5 is cleared; always bit 7 cleared and bit 6 set.
pub fn ni_time_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    stream: Ptr<NiStream>,
) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_00A645B0, &args![stream]);
    stream_read(e, stream, this.addr() + 0x08, 2);
    stream_read(e, stream, this.addr() + 0x0c, 4);
    stream_read(e, stream, this.addr() + 0x10, 4);
    stream_read(e, stream, this.addr() + 0x14, 4);
    stream_read(e, stream, this.addr() + 0x18, 4);
    e.call(STREAM_00A645B0, &args![stream]);
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a00_0101 {
        let flags = u32::from(e.get(this, NiTimeController::m_uFlags));
        let moved = ((flags & 0xfff0) * 2) | (flags & 0xf);
        e.set(this, NiTimeController::m_uFlags, moved as u16);
    }
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a00_0103 {
        let flags = e.get(this, NiTimeController::m_uFlags);
        e.mem.set_u16(stream.addr() + 0x286, flags);
        e.set(this, NiTimeController::m_uFlags, flags & 0x1f);
    }
    if e.get(stream, NiStream::m_uiNifFileVersion) < 0x0a01_006d {
        let flags = e.get(this, NiTimeController::m_uFlags);
        e.set(this, NiTimeController::m_uFlags, flags & 0xffdf);
    }
    let flags = e.get(this, NiTimeController::m_uFlags);
    e.set(this, NiTimeController::m_uFlags, (flags & 0xff7f) | 0x40);
}

/// `NiObject::IsEqual` (`00a5d470`) and `NiObject::RegisterStreamables` (`00a5d2f0`),
/// thiscall, one word each (`AL`).
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// The base `SaveBinary` `00a89af0` (unnamed in the engine map, three bytes; thiscall, the
/// stream).
const BASE_SAVE_BINARY_00A89AF0: u32 = 0x00a8_9af0;
/// The sized delete (`00aa1460`, cdecl `(pointer, size)`) and `NiAlloc` (`00aa1070`, cdecl:
/// size).
const SIZED_DELETE: u32 = 0x00aa_1460;
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, cdecl: buffer, size, format, then the format words).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiGetViewerString` (cdecl, the class name), `NiGetViewerString_ov2` (cdecl, a name and a
/// `bool` byte, `"%s = true"` / `"%s = false"`), `NiGetViewerString_ov9` (cdecl, a name and
/// a `float`, `"%s = %g"`) and `NiGetViewerString_ov10` (cdecl, a name and a pointer,
/// `"%s = %p"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// The strings array `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The formats of the two type strings: `"%s = APP_TIME"`, `"%s = APP_INIT"`;
/// `"%s = LOOP"`, `"%s = REVERSE"`, `"%s = CLAMP"`, `"%s = UNKNOWN!!!"`.
const FORMAT_APP_TIME: u32 = 0x0109_c824;
const FORMAT_APP_INIT: u32 = 0x0109_c834;
const FORMAT_LOOP: u32 = 0x0109_c844;
const FORMAT_REVERSE: u32 = 0x0109_c850;
const FORMAT_CLAMP: u32 = 0x0109_c860;
const FORMAT_UNKNOWN: u32 = 0x0109_b2ec;
/// The viewer-string names: `"m_fFrequency"`, `"m_fPhase"`, `"m_fLoKeyTime"`,
/// `"m_fHiKeyTime"`, `"m_fStartTime"`, `"m_fLastTime"`, `"m_pkTarget"`, `"anim type"`,
/// `"cycle type"`, `"Active"` and `"Play Backwards"`.
const TEXT_FREQUENCY: u32 = 0x0109_c99c;
const TEXT_PHASE: u32 = 0x0109_c990;
const TEXT_LO_KEY_TIME: u32 = 0x0109_c980;
const TEXT_HI_KEY_TIME: u32 = 0x0109_c970;
const TEXT_START_TIME: u32 = 0x0109_af98;
const TEXT_LAST_TIME: u32 = 0x0109_c964;
const TEXT_TARGET: u32 = 0x0109_c958;
const TEXT_ANIM_TYPE: u32 = 0x0109_c94c;
const TEXT_CYCLE_TYPE: u32 = 0x0109_c940;
const TEXT_ACTIVE: u32 = 0x0109_c938;
const TEXT_PLAY_BACKWARDS: u32 = 0x0109_c928;
/// The controller vtable slots this file calls: `RegisterStreamables` +0x54, `Start` +0x8C
/// and `IsStreamable` +0xAC (Xbox PDB slot order); the object slots `GetRTTI` +0x08 and
/// `IsNode` +0x0C; the stream slot `SaveLinkID` +0x30.
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_START: u32 = 0x8c;
const SLOT_IS_STREAMABLE: u32 = 0xac;
const SLOT_GET_RTTI: u32 = 0x08;
const SLOT_IS_NODE: u32 = 0x0c;
const SLOT_SAVE_LINK_ID: u32 = 0x30;

/// Adds the string word at `slot` at the end of the strings array (`SetSize` to the size
/// plus the grow-by step when the array is full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

/// Whether two `float` fields compare equal as `FLD`/`FLD`/`FUCOMPP` with `TEST AH,0x44`
/// finds them (a NaN is never equal; +0 equals -0).
fn floats_equal(a: u32, b: u32) -> bool {
    f32::from_bits(a) == f32::from_bits(b)
}

// Translated from 00a6cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::IsEqual` (Xbox PDB; thiscall, `RET 4`): `NiObject::IsEqual`, the same
/// flags (16 bits), frequency, phase, low and high key times (`FUCOMPP`: a NaN differs), and
/// a target on both or on neither.
pub fn ni_time_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    other: Ptr<NiTimeController>,
) -> bool {
    if e.call(OBJECT_IS_EQUAL, &args![this, other]).u32() as u8 == 0 {
        return false;
    }
    if e.get(this, NiTimeController::m_uFlags) != e.get(other, NiTimeController::m_uFlags) {
        return false;
    }
    for field in [
        NiTimeController::m_fFrequency,
        NiTimeController::m_fPhase,
        NiTimeController::m_fLoKeyTime,
        NiTimeController::m_fHiKeyTime,
    ] {
        if !floats_equal(e.get(this, field), e.get(other, field)) {
            return false;
        }
    }
    let target = e.get(this, NiTimeController::m_pkTarget);
    if target != 0 {
        if e.get(other, NiTimeController::m_pkTarget) == 0 {
            return false;
        }
    } else if e.get(other, NiTimeController::m_pkTarget) != 0 {
        return false;
    }
    true
}

// Translated from 00a6cd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body counts the controllers of the chain that starts at
/// `this` (thiscall, no arguments; `this` may be null), following `m_spNext` (+0x30).
pub fn fn_00a6cd00(e: &mut Engine, this: Ptr<NiTimeController>) -> u32 {
    let mut count = 0u32;
    let mut controller = this;
    while !controller.is_null() {
        controller = Ptr::new(e.get(controller, NiTimeController::m_spNext));
        count = count.wrapping_add(1);
    }
    count
}

// Translated from 00a6cd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::Stop` (Xbox PDB; thiscall, no arguments): clears flag bit 3 (`0x8`),
/// sets the last time to `-FLT_MAX` (the float at `01015f5c`) and, when flag bit 0 is set,
/// the start time to the same value.
pub fn ni_time_controller_stop(e: &mut Engine, this: Ptr<NiTimeController>) {
    let no_time = x87_store32(x87_load32(e.mem.u32(NO_TIME_FLOAT)));
    let flags = e.get(this, NiTimeController::m_uFlags) & 0xfff7;
    e.set(this, NiTimeController::m_uFlags, flags);
    e.set(this, NiTimeController::m_fLastTime, no_time);
    if e.get(this, NiTimeController::m_uFlags) & 1 != 0 {
        e.set(this, NiTimeController::m_fStartTime, no_time);
    }
}

// Translated from 00a6d0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::RegisterStreamables` (Xbox PDB; thiscall, `RET 4`): false when
/// `NiObject::RegisterStreamables` is; otherwise the next controller (+0x30, when there is
/// one) registers too (its slot +0x54, result ignored) and the answer is true.
pub fn ni_time_controller_register_streamables(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    stream: Ptr,
) -> bool {
    if e.call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .u32() as u8
        == 0
    {
        return false;
    }
    let next = e.get(this, NiTimeController::m_spNext);
    if next != 0 {
        e.vcall(next, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

/// The length of the zero-terminated string at `text` (the code inline `strlen`).
fn string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

// Translated from 00a6d120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::GetViewerString` (Xbox PDB; cdecl: a name and the animation type): a
/// new `NiAlloc` buffer of the name length plus 12, filled by `NiSprintf` with
/// `"%s = APP_TIME"` for type 0 or `"%s = APP_INIT"` for type 1; any other type leaves the
/// buffer as allocated. Returns the buffer.
pub fn ni_time_controller_get_viewer_string(e: &mut Engine, name: u32, kind: u32) -> u32 {
    let size = string_length(e, name).wrapping_add(0xc);
    let buffer = e.call(NI_ALLOC, &args![size]).u32();
    let format = match kind {
        0 => FORMAT_APP_TIME,
        1 => FORMAT_APP_INIT,
        _ => return buffer,
    };
    e.call(NI_SPRINTF, &args![buffer, size, format, name]);
    buffer
}

// Translated from 00a6d190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::GetViewerString_ov2` (Xbox PDB; cdecl: a name and the cycle type): a
/// new `NiAlloc` buffer of the name length plus 11, filled by `NiSprintf` with
/// `"%s = LOOP"` (0), `"%s = REVERSE"` (1), `"%s = CLAMP"` (2) or `"%s = UNKNOWN!!!"`.
/// Returns the buffer.
pub fn ni_time_controller_get_viewer_string_ov2(e: &mut Engine, name: u32, kind: u32) -> u32 {
    let size = string_length(e, name).wrapping_add(0xb);
    let buffer = e.call(NI_ALLOC, &args![size]).u32();
    let format = match kind {
        0 => FORMAT_LOOP,
        1 => FORMAT_REVERSE,
        2 => FORMAT_CLAMP,
        _ => FORMAT_UNKNOWN,
    };
    e.call(NI_SPRINTF, &args![buffer, size, format, name]);
    buffer
}

// Translated from 00a6d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body is the `NiTimeController::StartAnimations` overload
/// with a time (cdecl: the object and a `float`): as [`ni_time_controller_start_animations`],
/// but every controller starts (`Start`, slot +0x8C) at `time` (pushed through `FLD`/`FSTP`)
/// and the recursion passes it on.
pub fn fn_00a6d390(e: &mut Engine, object: Ptr, time: f32) {
    let pushed = x87_quiet_bits(time.to_bits());
    let mut controller = e.mem.u32(object.addr() + 0xc);
    while controller != 0 {
        e.vcall(controller, SLOT_START, &args![pushed]);
        controller = e.mem.u32(controller + 0x30);
    }
    let mut rtti = e.vcall(object.addr(), SLOT_GET_RTTI, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_AV_OBJECT_RTTI {
            let mut item = e.mem.u32(object.addr() + 0x24);
            while item != 0 {
                let property = e.mem.u32(item + 8);
                item = e.mem.u32(item);
                if property != 0 && e.mem.u32(property + 0xc) != 0 {
                    fn_00a6d390(e, Ptr::new(property), f32::from_bits(pushed));
                }
            }
            break;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    if e.vcall(object.addr(), SLOT_IS_NODE, &args![]).u32() != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(object.addr() + 0xa6)) {
            let child = e.mem.u32(e.mem.u32(object.addr() + 0xa0) + index * 4);
            if child != 0 {
                fn_00a6d390(e, Ptr::new(child), f32::from_bits(pushed));
            }
            index += 1;
        }
    }
}

// Translated from 00a6d590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body is `NiTimeController`'s scalar deleting destructor
/// (thiscall, `RET 4`): sets the class vtable (`0109c874`), releases the next controller
/// (+0x30; `DeleteThis` when its count reaches 0; the field is left as it is), runs
/// `NiObject`'s destructor body (`00a5d3d0`) and frees the 0x34-byte object when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00a6d590(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    flags: u32,
) -> Ptr<NiTimeController> {
    e.mem.set_u32(this.addr(), NI_TIME_CONTROLLER_VTABLE);
    let next = e.get(this, NiTimeController::m_spNext);
    if next != 0 {
        release(e, next);
    }
    e.call(OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
}

/// One transfer of `size` bytes of the field at `field` through the stream being written
/// (`m_pkOstr`, read again each time; its `m_pfnWrite`), the component size kept in the
/// stream argument stack word.
fn stream_write(e: &mut Engine, stream: Ptr<NiStream>, field: u32, size: u32) {
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let function = e.get(output, NiBinaryStream::m_pfnWrite);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        e.call(function, &args![output, field, size, count, 1u32])
            .u32()
    });
    let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        output,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

// Translated from 00a6d730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::SaveBinary` (Xbox PDB; thiscall, `RET 4`): the base `SaveBinary`
/// (`00a89af0`); the link to the first controller after this one whose slot +0xAC
/// (`IsStreamable`) answers true, or null (the stream slot +0x30, `SaveLinkID`); the flags
/// (2 bytes), frequency, phase, low and high key times (4 bytes each); then the link to the
/// target.
pub fn ni_time_controller_save_binary(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    stream: Ptr<NiStream>,
) {
    e.call(BASE_SAVE_BINARY_00A89AF0, &args![this, stream]);
    let mut next = e.get(this, NiTimeController::m_spNext);
    while next != 0 {
        if e.vcall(next, SLOT_IS_STREAMABLE, &args![]).u32() as u8 != 0 {
            break;
        }
        next = e.mem.u32(next + 0x30);
    }
    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![next]);
    let base = this.addr();
    stream_write(e, stream, base + 0x08, 2);
    for field in [0x0c, 0x10, 0x14, 0x18] {
        stream_write(e, stream, base + field, 4);
    }
    let target = e.get(this, NiTimeController::m_pkTarget);
    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![target]);
}

// Translated from 00a6d830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTimeController::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`; no base class strings):
/// the class name (the first word of `ms_RTTI`), the frequency, phase, low and high key,
/// start and last times (`float`s pushed through `FLD`/`FSTP`), the target address, the
/// animation type (flag bit 0) and cycle type (flag bits 1 and 2) through this class's two
/// `GetViewerString`s, then `"Active"` (flag bit 3) and `"Play Backwards"` (bit 4) as
/// `bool`s. The first string is held in a local, the others in the argument stack word;
/// each `bool` is written over the low byte of that word, so its upper bytes are those of
/// the string before it.
pub fn ni_time_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiTimeController>,
    strings: Ptr<NiTArray>,
) {
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let slot = frame.addr() + 4;
        let class_name = e.global::<u32>(NI_TIME_CONTROLLER_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(local, text);
        add_viewer_string(e, strings, local);
        for (name, field) in [
            (TEXT_FREQUENCY, NiTimeController::m_fFrequency),
            (TEXT_PHASE, NiTimeController::m_fPhase),
            (TEXT_LO_KEY_TIME, NiTimeController::m_fLoKeyTime),
            (TEXT_HI_KEY_TIME, NiTimeController::m_fHiKeyTime),
            (TEXT_START_TIME, NiTimeController::m_fStartTime),
            (TEXT_LAST_TIME, NiTimeController::m_fLastTime),
        ] {
            let value = x87_quiet_bits(e.get(this, field));
            let text = e.call(VIEWER_STRING_FLOAT, &args![name, value]).u32();
            e.mem.set_u32(slot, text);
            add_viewer_string(e, strings, slot);
        }
        let target = e.get(this, NiTimeController::m_pkTarget);
        let text = e
            .call(VIEWER_STRING_POINTER, &args![TEXT_TARGET, target])
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let flags = u32::from(e.get(this, NiTimeController::m_uFlags));
        let text = ni_time_controller_get_viewer_string(e, TEXT_ANIM_TYPE, flags & 1);
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let flags = u32::from(e.get(this, NiTimeController::m_uFlags));
        let text = ni_time_controller_get_viewer_string_ov2(e, TEXT_CYCLE_TYPE, (flags >> 1) & 3);
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        for (name, bit) in [(TEXT_ACTIVE, 3), (TEXT_PLAY_BACKWARDS, 4)] {
            let flag = (e.mem.u8(this.addr() + 0x08) >> bit) & 1;
            e.mem.set_u8(slot, flag);
            let word = e.mem.u32(slot);
            let text = e.call(VIEWER_STRING_BOOL, &args![name, word]).u32();
            e.mem.set_u32(slot, text);
            add_viewer_string(e, strings, slot);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6d280, ni_time_controller_get_rtti() -> Ptr),
        entry!(
            0x00a6_cd10,
            ni_time_controller_start(Ptr<NiTimeController>, f32)
        ),
        entry!(
            0x00a6_cd60,
            ni_time_controller_compute_scaled_time(Ptr<NiTimeController>, f32) -> f32
        ),
        entry!(
            0x00a6_cfe0,
            fn_00a6cfe0(Ptr<NiTimeController>, Ptr<NiTimeController>, Ptr)
        ),
        entry!(
            0x00a6_d0c0,
            ni_time_controller_process_clone(Ptr<NiTimeController>, Ptr)
        ),
        entry!(
            0x00a6_d220,
            fn_00a6d220(Ptr<NiTimeController>) -> Ptr<NiTimeController>
        ),
        entry!(0x00a6_d290, fn_00a6d290(Ptr<NiTimeController>)),
        entry!(0x00a6_d2d0, ni_time_controller_start_animations(Ptr)),
        entry!(
            0x00a6_d450,
            ni_time_controller_set_target(Ptr<NiTimeController>, Ptr)
        ),
        entry!(
            0x00a6_d500,
            ni_time_controller_link_object(Ptr<NiTimeController>, Ptr<NiStream>)
        ),
        entry!(
            0x00a6_d5e0,
            ni_time_controller_load_binary(Ptr<NiTimeController>, Ptr<NiStream>)
        ),
        entry!(
            0x00a6_cc80,
            ni_time_controller_is_equal(Ptr<NiTimeController>, Ptr<NiTimeController>) -> bool
        ),
        entry!(0x00a6_cd00, fn_00a6cd00(Ptr<NiTimeController>) -> u32),
        entry!(0x00a6_cd30, ni_time_controller_stop(Ptr<NiTimeController>)),
        entry!(
            0x00a6_d0f0,
            ni_time_controller_register_streamables(Ptr<NiTimeController>, Ptr) -> bool
        ),
        entry!(
            0x00a6_d120,
            ni_time_controller_get_viewer_string(u32, u32) -> u32
        ),
        entry!(
            0x00a6_d190,
            ni_time_controller_get_viewer_string_ov2(u32, u32) -> u32
        ),
        entry!(0x00a6_d390, fn_00a6d390(Ptr, f32)),
        entry!(
            0x00a6_d590,
            fn_00a6d590(Ptr<NiTimeController>, u32) -> Ptr<NiTimeController>
        ),
        entry!(
            0x00a6_d730,
            ni_time_controller_save_binary(Ptr<NiTimeController>, Ptr<NiStream>)
        ),
        entry!(
            0x00a6_d830,
            ni_time_controller_get_viewer_strings(Ptr<NiTimeController>, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_time_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a6d280, &args![]).u32(), NI_TIME_CONTROLLER_RTTI);
    }
}

#[cfg(test)]
mod controller_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    const NO_TIME_BITS: u32 = 0xff7f_ffff;

    /// The counting engine with the float constants the code reads, and `_CIfmod` doubled.
    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0101_5000, 0x1000);
        e.map(0x0109_c000, 0x1000);
        e.map(0x0102_3000, 0x2000);
        e.mem.set_u32(NO_TIME_FLOAT, NO_TIME_BITS);
        e.mem.set_u32(MAX_FLOAT, 0x7f7f_ffff);
        e.mem.set_u64(NO_TIME_DOUBLE, f64::from(f32::MIN).to_bits());
        e.mem.set_u64(MAX_DOUBLE, f64::from(f32::MAX).to_bits());
        e.register(CI_FMOD, |_, a| {
            let dividend = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let divisor = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: dividend % divisor,
                ..Ret::default()
            }
        });
        e
    }

    fn set_f(e: &mut Engine, at: u32, value: f32) {
        e.mem.set_u32(at, value.to_bits());
    }

    fn get_f(e: &Engine, at: u32) -> f32 {
        f32::from_bits(e.mem.u32(at))
    }

    /// A controller with the given flags and key range, never run (start and last time unset).
    fn controller(e: &mut Engine, flags: u16, low: f32, high: f32) -> Ptr<NiTimeController> {
        let c = e.new_object::<NiTimeController>();
        e.set(c, NiTimeController::m_uFlags, flags);
        e.set(c, NiTimeController::m_fFrequency, 1.0f32.to_bits());
        e.set(c, NiTimeController::m_fLoKeyTime, low.to_bits());
        e.set(c, NiTimeController::m_fHiKeyTime, high.to_bits());
        e.set(c, NiTimeController::m_fStartTime, NO_TIME_BITS);
        e.set(c, NiTimeController::m_fLastTime, NO_TIME_BITS);
        c
    }

    fn scaled(e: &mut Engine, c: Ptr<NiTimeController>, time: f32) -> f32 {
        e.call(0x00a6_cd60, &args![c, time]).f32()
    }

    #[test]
    fn start_marks_the_controller_and_takes_the_start_time_with_bit_0() {
        let mut e = engine();
        let c = e.new_object::<NiTimeController>();
        e.set(c, NiTimeController::m_uFlags, 1);
        e.call(0x00a6_cd10, &args![c, 2.5f32]);
        assert_eq!(e.get(c, NiTimeController::m_uFlags), 9);
        assert_eq!(e.get(c, NiTimeController::m_fLastTime), NO_TIME_BITS);
        assert_eq!(e.get(c, NiTimeController::m_fStartTime), 2.5f32.to_bits());
        let d = e.new_object::<NiTimeController>();
        e.call(0x00a6_cd10, &args![d, 2.5f32]);
        assert_eq!(e.get(d, NiTimeController::m_uFlags), 8);
        assert_eq!(e.get(d, NiTimeController::m_fStartTime), 0);
    }

    #[test]
    fn compute_scaled_time_loops_caches_mirrors_clamps_and_reverses() {
        let mut e = engine();
        // Loop (cycle 0) over [0, 10].
        let c = controller(&mut e, 0, 0.0, 10.0);
        assert_eq!(scaled(&mut e, c, 3.0), 3.0);
        assert_eq!(get_f(&e, c.addr() + 0x1c), 3.0);
        assert_eq!(get_f(&e, c.addr() + 0x20), 3.0);
        assert_eq!(scaled(&mut e, c, 15.0), 5.0);
        assert_eq!(get_f(&e, c.addr() + 0x24), 15.0);
        e.call_log = Some(vec![]);
        // The same input again: the cached result, no fmod.
        assert_eq!(scaled(&mut e, c, 15.0), 5.0);
        assert!(calls_to(&e, CI_FMOD).is_empty());
        // Mirror (cycle 1): 13 over [0, 10] folds back to 7.
        let m = controller(&mut e, 2, 0.0, 10.0);
        assert_eq!(scaled(&mut e, m, 13.0), 7.0);
        assert_eq!(calls_to(&e, CI_FMOD).len(), 1);
        // Cycle 2 keeps the time, clamped to the range; bit 4 reverses it.
        let k = controller(&mut e, 4, 1.0, 10.0);
        assert_eq!(scaled(&mut e, k, 20.0), 10.0);
        let r = controller(&mut e, 0x14, 1.0, 10.0);
        assert_eq!(scaled(&mut e, r, 4.0), 7.0);
        // Bit 0 starts from 0; frequency and phase apply.
        let f = controller(&mut e, 5, 0.0, 100.0);
        set_f(&mut e, f.addr() + 0xc, 2.0);
        set_f(&mut e, f.addr() + 0x10, 1.0);
        assert_eq!(scaled(&mut e, f, 40.0), 1.0);
        assert_eq!(scaled(&mut e, f, 43.0), 7.0);
        // A zero range gives the low key time.
        let z = controller(&mut e, 0, 4.0, 4.0);
        assert_eq!(scaled(&mut e, z, 9.0), 4.0);
    }

    #[test]
    fn copy_members_copies_times_target_clone_and_next_clone() {
        let mut e = engine();
        let this = controller(&mut e, 0x4c, 1.0, 2.0);
        let clone = e.new_object::<NiTimeController>();
        let target = e.mem.alloc(0x10);
        let cloned_target = e.mem.alloc(0x10);
        let next = e.mem.alloc(0x10);
        let cloned_next = e.mem.alloc(0x10);
        e.put_vtable(0x0300_1000, &[0, 0, 0x0300_0008]);
        let mut slots = vec![0u32; 19];
        slots[18] = 0x0300_0048;
        e.put_vtable(0x0300_2000, &slots);
        e.mem.set_u32(target, 0x0300_1000);
        e.mem.set_u32(cloned_target, 0x0300_1000);
        e.mem.set_u32(next, 0x0300_2000);
        e.register(0x0300_0008, |_, _| 0x1234u32.into_ret());
        e.register_double(0x0300_0048, move |_, _| cloned_next.into_ret());
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], cloned_target);
            1u32.into_ret()
        });
        e.set(this, NiTimeController::m_pkTarget, target);
        e.set(this, NiTimeController::m_spNext, next);
        set_f(&mut e, this.addr() + 0x24, 6.0);
        let process = e.mem.alloc(8);
        e.mem.set_u32(process, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00a6_cfe0, &args![this, clone, process]);
        assert_eq!(e.get(clone, NiTimeController::m_uFlags), 0x4c);
        assert_eq!(get_f(&e, clone.addr() + 0x14), 1.0);
        assert_eq!(get_f(&e, clone.addr() + 0x18), 2.0);
        assert_eq!(get_f(&e, clone.addr() + 0x24), 6.0);
        assert_eq!(e.get(clone, NiTimeController::m_fScaledTime), 0);
        assert_eq!(e.get(clone, NiTimeController::m_pkTarget), cloned_target);
        assert_eq!(calls_to(&e, CLONE_MAP_GET_AT)[0][..2], [0x4444, target]);
        assert_eq!(e.get(clone, NiTimeController::m_spNext), cloned_next);
        assert_eq!(e.mem.u32(cloned_next + 4), 1);
        // A target missing from the map leaves null.
        e.register(CLONE_MAP_GET_AT, |_, _| 0u32.into_ret());
        e.set(clone, NiTimeController::m_pkTarget, 7);
        e.call(0x00a6_cfe0, &args![this, clone, process]);
        assert_eq!(e.get(clone, NiTimeController::m_pkTarget), 0);
        assert_eq!(e.mem.u32(cloned_next + 4), 1);
    }

    #[test]
    fn process_clone_passes_on_to_the_next_controller() {
        let mut e = engine();
        let this = e.new_object::<NiTimeController>();
        let next = e.mem.alloc(0x34);
        let mut slots = vec![0u32; 26];
        slots.push(0x0300_0068);
        e.put_vtable(0x0300_3000, &slots);
        e.mem.set_u32(next, 0x0300_3000);
        e.register(OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(0x0300_0068, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a6_d0c0, &args![this, 0x55u32]);
        assert!(calls_to(&e, 0x0300_0068).is_empty());
        e.set(this, NiTimeController::m_spNext, next);
        e.call(0x00a6_d0c0, &args![this, 0x55u32]);
        assert_eq!(calls_to(&e, OBJECT_PROCESS_CLONE).len(), 2);
        assert_eq!(calls_to(&e, 0x0300_0068), vec![vec![next, 0x55]]);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = engine();
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let c = e.new_object::<NiTimeController>();
        e.mem.set_u32(c.addr() + 0x30, 9);
        assert_eq!(e.call(0x00a6_d220, &args![c]).u32(), c.addr());
        assert_eq!(e.mem.u32(c.addr()), NI_TIME_CONTROLLER_VTABLE);
        assert_eq!(e.get(c, NiTimeController::m_uFlags), 0x4c);
        assert_eq!(get_f(&e, c.addr() + 0xc), 1.0);
        assert_eq!(e.get(c, NiTimeController::m_fLoKeyTime), 0x7f7f_ffff);
        for offset in [0x18u32, 0x1c, 0x20, 0x28] {
            assert_eq!(e.mem.u32(c.addr() + offset), NO_TIME_BITS);
        }
        assert_eq!(e.get(c, NiTimeController::m_spNext), 0);
    }

    #[test]
    fn destructor_releases_the_next_controller() {
        let mut e = engine();
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.put_vtable(0x0300_4000, &[0, 0x0300_0004]);
        e.register(0x0300_0004, |_, _| Ret::default());
        let c = e.new_object::<NiTimeController>();
        let next = e.mem.alloc(0x34);
        e.mem.set_u32(next, 0x0300_4000);
        e.mem.set_u32(next + 4, 1);
        e.set(c, NiTimeController::m_spNext, next);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d290, &args![c]);
        assert_eq!(e.mem.u32(c.addr()), NI_TIME_CONTROLLER_VTABLE);
        assert_eq!(calls_to(&e, 0x0300_0004), vec![vec![next]]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![c.addr()]]);
        assert_eq!(e.get(c, NiTimeController::m_spNext), next);
    }

    #[test]
    fn start_animations_starts_controllers_and_recurses_into_properties_and_children() {
        let mut e = engine();
        // Slots: +8 GetRTTI, +0xc IsNode, +0x8c Start.
        let mut slots = vec![0u32; 36];
        slots[2] = 0x0300_0108;
        slots[3] = 0x0300_010c;
        slots[35] = 0x0300_018c;
        e.put_vtable(0x0300_5000, &slots);
        // Node objects answer the NiAVObject record through a derived record.
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, NI_AV_OBJECT_RTTI);
        e.register_double(0x0300_0108, move |e, a| {
            let rtti = if e.mem.u32(a[0] + 0x40) != 0 {
                derived
            } else {
                0
            };
            rtti.into_ret()
        });
        e.register(0x0300_010c, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(0x0300_018c, |_, _| Ret::default());
        let object = |e: &mut Engine, node: bool| {
            let o = e.mem.alloc(0xb0);
            e.mem.set_u32(o, 0x0300_5000);
            e.mem.set_u32(o + 0x40, u32::from(node));
            o
        };
        let root = object(&mut e, true);
        let child = object(&mut e, false);
        let property = object(&mut e, false);
        let controller_a = e.mem.alloc(0x34);
        let controller_b = e.mem.alloc(0x34);
        let controller_c = e.mem.alloc(0x34);
        for c in [controller_a, controller_b, controller_c] {
            e.mem.set_u32(c, 0x0300_5000);
        }
        e.mem.set_u32(root + 0xc, controller_a);
        e.mem.set_u32(controller_a + 0x30, controller_b);
        e.mem.set_u32(child + 0xc, controller_c);
        e.mem.set_u32(property + 0xc, controller_c);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, property);
        e.mem.set_u32(root + 0x24, item);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children + 4, child);
        e.mem.set_u32(root + 0xa0, children);
        e.mem.set_u16(root + 0xa6, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d2d0, &args![root]);
        let starts = calls_to(&e, 0x0300_018c);
        assert_eq!(
            starts,
            vec![
                vec![controller_a, NO_TIME_BITS],
                vec![controller_b, NO_TIME_BITS],
                vec![controller_c, NO_TIME_BITS],
                vec![controller_c, NO_TIME_BITS],
            ]
        );
    }

    /// A controller whose vtable answers `vertex` from `IsVertexController` (slot `+0xa0`) and
    /// records deletes (slot `+4`), with count 1.
    fn counted_controller(e: &mut Engine, vertex: bool) -> Ptr<NiTimeController> {
        let mut slots = vec![0u32; 41];
        slots[1] = 0x0300_0204;
        slots[40] = if vertex { 0x0300_02a1 } else { 0x0300_02a0 };
        e.put_vtable(0x0300_6000 + 0x100 * u32::from(vertex), &slots);
        e.register(0x0300_0204, |_, _| Ret::default());
        e.register(0x0300_02a0, |_, _| 0u32.into_ret());
        e.register(0x0300_02a1, |_, _| 1u32.into_ret());
        let c = e.new_object::<NiTimeController>();
        e.mem
            .set_u32(c.addr(), 0x0300_6000 + 0x100 * u32::from(vertex));
        e.set(c, NiTimeController::m_uiRefCount, 1);
        c
    }

    #[test]
    fn set_target_moves_the_controller_and_marks_vertex_data() {
        let mut e = engine();
        e.register(TARGET_REMOVE_CONTROLLER, |_, _| Ret::default());
        e.register(TARGET_ADD_CONTROLLER, |_, _| Ret::default());
        e.register(GEOMETRY_DATA_SET_CONSISTENCY, |_, _| Ret::default());
        let c = counted_controller(&mut e, true);
        let old = e.mem.alloc(0xc0);
        e.mem.set_u32(old + 0xc, c.addr());
        e.set(c, NiTimeController::m_pkTarget, old);
        let target = e.mem.alloc(0xc0);
        e.mem.set_u32(target + 0xb8, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d450, &args![c, target]);
        assert_eq!(
            calls_to(&e, TARGET_REMOVE_CONTROLLER),
            vec![vec![old, c.addr()]]
        );
        assert_eq!(
            calls_to(&e, TARGET_ADD_CONTROLLER),
            vec![vec![target, c.addr()]]
        );
        assert_eq!(
            calls_to(&e, GEOMETRY_DATA_SET_CONSISTENCY),
            vec![vec![0x7777, 0]]
        );
        assert_eq!(e.get(c, NiTimeController::m_pkTarget), target);
        assert_eq!(e.get(c, NiTimeController::m_uiRefCount), 1);
        // The same target: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a6_d450, &args![c, target]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // Already in the new target's list: not added again.
        let listed = e.mem.alloc(0xc0);
        e.mem.set_u32(listed + 0xc, c.addr());
        e.call(0x00a6_d450, &args![c, listed]);
        assert!(calls_to(&e, TARGET_ADD_CONTROLLER).is_empty());
        assert_eq!(e.get(c, NiTimeController::m_pkTarget), listed);
        // A null target with a count of 0 held: the controller deletes itself.
        let d = counted_controller(&mut e, false);
        e.set(d, NiTimeController::m_uiRefCount, 0);
        e.set(d, NiTimeController::m_pkTarget, old);
        e.mem.set_u32(old + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d450, &args![d, 0u32]);
        assert!(calls_to(&e, TARGET_REMOVE_CONTROLLER).is_empty());
        assert_eq!(calls_to(&e, 0x0300_0204), vec![vec![d.addr()]]);
    }

    #[test]
    fn link_object_takes_next_and_target_from_the_stream() {
        let mut e = engine();
        e.register(GEOMETRY_DATA_SET_CONSISTENCY, |_, _| Ret::default());
        let c = counted_controller(&mut e, true);
        let next = e.mem.alloc(0x34);
        let target = e.mem.alloc(0xc0);
        e.mem.set_u32(target + 0xb8, 0x8888);
        let mut answers = vec![target, next];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            answers.pop().unwrap().into_ret()
        });
        let stream = e.new_object::<NiStream>();
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a00_0100);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d500, &args![c, stream]);
        assert_eq!(e.get(c, NiTimeController::m_spNext), next);
        assert_eq!(e.mem.u32(next + 4), 1);
        assert_eq!(e.get(c, NiTimeController::m_pkTarget), target);
        assert_eq!(
            calls_to(&e, GEOMETRY_DATA_SET_CONSISTENCY),
            vec![vec![0x8888, 0]]
        );
        // A current file leaves the data alone.
        let mut answers = vec![target, next];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            answers.pop().unwrap().into_ret()
        });
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a00_0110);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d500, &args![c, stream]);
        assert!(calls_to(&e, GEOMETRY_DATA_SET_CONSISTENCY).is_empty());
        assert_eq!(e.mem.u32(next + 4), 1);
    }

    /// A stream of `version` whose input answers `values` in order (each `size` bytes).
    fn stream_with(e: &mut Engine, version: u32, values: Vec<u32>) -> Ptr<NiStream> {
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(stream, NiStream::m_uiNifFileVersion, version);
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0300);
        let mut queue = std::collections::VecDeque::from(values);
        e.register_double(0x0300_0300, move |e, a| {
            let value = queue.pop_front().unwrap();
            assert_eq!(e.mem.u32(a[3]), a[2]);
            e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
            a[2].into_ret()
        });
        stream
    }

    #[test]
    fn load_binary_reads_the_fields_and_updates_old_flags() {
        let mut e = engine();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_00A645B0, |_, _| Ret::default());
        let values = |flags: u32| {
            vec![
                flags,
                2.0f32.to_bits(),
                0.5f32.to_bits(),
                1.0f32.to_bits(),
                3.0f32.to_bits(),
            ]
        };
        let c = e.new_object::<NiTimeController>();
        let stream = stream_with(&mut e, 0x0a01_006d, values(0xffa5));
        e.call(0x00a6_d5e0, &args![c, stream]);
        assert_eq!(e.get(c, NiTimeController::m_uFlags), 0xff65);
        assert_eq!(get_f(&e, c.addr() + 0xc), 2.0);
        assert_eq!(get_f(&e, c.addr() + 0x10), 0.5);
        assert_eq!(get_f(&e, c.addr() + 0x14), 1.0);
        assert_eq!(get_f(&e, c.addr() + 0x18), 3.0);
        let input = e.get(stream, NiStream::m_pkIstr);
        assert_eq!(e.mem.u32(input + 4), 18);
        // Version 0x0a000100: bits move up, the old flags are kept, cut to 5 bits.
        let old = e.new_object::<NiTimeController>();
        let stream = stream_with(&mut e, 0x0a00_0100, values(0x0035));
        e.call(0x00a6_d5e0, &args![old, stream]);
        assert_eq!(e.mem.u16(stream.addr() + 0x286), 0x0065);
        assert_eq!(e.get(old, NiTimeController::m_uFlags), 0x0045);
        // Version 0x0a000102: kept and cut without the move; bit 5 cleared.
        let mid = e.new_object::<NiTimeController>();
        let stream = stream_with(&mut e, 0x0a00_0102, values(0x00ff));
        e.call(0x00a6_d5e0, &args![mid, stream]);
        assert_eq!(e.mem.u16(stream.addr() + 0x286), 0x00ff);
        assert_eq!(e.get(mid, NiTimeController::m_uFlags), 0x005f);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    const VTABLE: u32 = 0x0710_0000;
    const START_DOUBLE: u32 = 0x0720_0000;
    const RTTI_DOUBLE: u32 = 0x0720_0010;
    const IS_NODE_DOUBLE: u32 = 0x0720_0020;
    const STREAMABLE_DOUBLE: u32 = 0x0720_0030;
    const REGISTER_DOUBLE: u32 = 0x0720_0040;
    const DELETE_DOUBLE: u32 = 0x0720_0050;
    const SAVE_LINK_DOUBLE: u32 = 0x0720_0060;
    const WRITE_DOUBLE: u32 = 0x0720_0070;
    /// The answer of the `GetRTTI` double: an `NiAVObject` record below a made-up one.
    const OWN_RTTI: u32 = 0x0730_0000;

    /// One vtable for controllers, objects and streams: every slot this file calls is a
    /// double; `IsNode` answers the object word at +0xA8, `IsStreamable` the word at +0x10.
    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0101_5000, 0x1000);
        e.mem.set_u32(NO_TIME_FLOAT, 0xff7f_ffff);
        let mut slots = vec![0u32; 0xb0 / 4];
        slots[SLOT_START as usize / 4] = START_DOUBLE;
        slots[SLOT_GET_RTTI as usize / 4] = RTTI_DOUBLE;
        slots[SLOT_IS_NODE as usize / 4] = IS_NODE_DOUBLE;
        slots[SLOT_IS_STREAMABLE as usize / 4] = STREAMABLE_DOUBLE;
        slots[SLOT_REGISTER_STREAMABLES as usize / 4] = REGISTER_DOUBLE;
        slots[1] = DELETE_DOUBLE;
        slots[SLOT_SAVE_LINK_ID as usize / 4] = SAVE_LINK_DOUBLE;
        e.put_vtable(VTABLE, &slots);
        e.map(OWN_RTTI, 8);
        e.mem.set_u32(OWN_RTTI + 4, NI_AV_OBJECT_RTTI);
        for addr in [
            START_DOUBLE,
            REGISTER_DOUBLE,
            DELETE_DOUBLE,
            SAVE_LINK_DOUBLE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(RTTI_DOUBLE, |e, a| e.mem.u32(a[0] + 0xac).into_ret());
        e.register(IS_NODE_DOUBLE, |e, a| e.mem.u32(a[0] + 0xa8).into_ret());
        e.register(STREAMABLE_DOUBLE, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e
    }

    fn controller(e: &mut Engine) -> Ptr<NiTimeController> {
        let c = e.new_object::<NiTimeController>();
        e.mem.set_u32(c.addr(), VTABLE);
        c
    }

    #[test]
    fn is_equal_compares_flags_times_and_target_presence() {
        let mut e = engine();
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        let a = controller(&mut e);
        let b = controller(&mut e);
        e.set(a, NiTimeController::m_fPhase, 0x8000_0000);
        assert_eq!(e.call(0x00a6_cc80, &args![a, b]).u32() & 0xff, 1);
        e.set(a, NiTimeController::m_fHiKeyTime, 0x7fc0_0000);
        e.set(b, NiTimeController::m_fHiKeyTime, 0x7fc0_0000);
        assert_eq!(e.call(0x00a6_cc80, &args![a, b]).u32() & 0xff, 0);
        e.set(a, NiTimeController::m_fHiKeyTime, 0);
        e.set(b, NiTimeController::m_fHiKeyTime, 0);
        e.set(a, NiTimeController::m_pkTarget, 0x100);
        assert_eq!(e.call(0x00a6_cc80, &args![a, b]).u32() & 0xff, 0);
        e.set(b, NiTimeController::m_pkTarget, 0x200);
        assert_eq!(e.call(0x00a6_cc80, &args![a, b]).u32() & 0xff, 1);
        e.set(b, NiTimeController::m_uFlags, 1);
        assert_eq!(e.call(0x00a6_cc80, &args![a, b]).u32() & 0xff, 0);
        e.set(b, NiTimeController::m_uFlags, 0);
        e.register(OBJECT_IS_EQUAL, |_, _| 0xff00u32.into_ret());
        assert_eq!(e.call(0x00a6_cc80, &args![a, b]).u32() & 0xff, 0);
    }

    #[test]
    fn chain_count_follows_the_next_links() {
        let mut e = engine();
        assert_eq!(e.call(0x00a6_cd00, &args![0u32]).u32(), 0);
        let a = controller(&mut e);
        let b = controller(&mut e);
        e.set(a, NiTimeController::m_spNext, b.addr());
        assert_eq!(e.call(0x00a6_cd00, &args![a]).u32(), 2);
    }

    #[test]
    fn stop_clears_bit_3_and_resets_the_times() {
        let mut e = engine();
        let c = controller(&mut e);
        e.set(c, NiTimeController::m_uFlags, 0x0f);
        e.call(0x00a6_cd30, &args![c]);
        assert_eq!(e.get(c, NiTimeController::m_uFlags), 0x07);
        assert_eq!(e.get(c, NiTimeController::m_fLastTime), 0xff7f_ffff);
        assert_eq!(e.get(c, NiTimeController::m_fStartTime), 0xff7f_ffff);
        e.set(c, NiTimeController::m_uFlags, 0x0e);
        e.set(c, NiTimeController::m_fStartTime, 5);
        e.call(0x00a6_cd30, &args![c]);
        assert_eq!(e.get(c, NiTimeController::m_fStartTime), 5);
    }

    #[test]
    fn register_streamables_passes_on_to_the_next_controller() {
        let mut e = engine();
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        let a = controller(&mut e);
        let b = controller(&mut e);
        e.set(a, NiTimeController::m_spNext, b.addr());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_d0f0, &args![a, 0x50u32]).u32() & 0xff, 1);
        assert_eq!(calls_to(&e, REGISTER_DOUBLE), vec![vec![b.addr(), 0x50]]);
        e.register(OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a6_d0f0, &args![a, 0x50u32]).u32() & 0xff, 0);
        assert_eq!(calls_to(&e, REGISTER_DOUBLE).len(), 1);
    }

    fn text(e: &mut Engine, s: &str) -> u32 {
        let block = e.mem.alloc(s.len() as u32 + 1);
        e.mem.write(block, s.as_bytes());
        block
    }

    #[test]
    fn type_strings_pick_their_formats() {
        let mut e = engine();
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let name = text(&mut e, "abcd");
        e.call_log = Some(vec![]);
        for kind in 0..3u32 {
            e.call(0x00a6_d120, &args![name, kind]);
        }
        for kind in 0..4u32 {
            e.call(0x00a6_d190, &args![name, kind]);
        }
        let formats: Vec<u32> = calls_to(&e, NI_SPRINTF).iter().map(|w| w[2]).collect();
        assert_eq!(
            formats,
            vec![
                FORMAT_APP_TIME,
                FORMAT_APP_INIT,
                FORMAT_LOOP,
                FORMAT_REVERSE,
                FORMAT_CLAMP,
                FORMAT_UNKNOWN
            ]
        );
        let sizes: Vec<u32> = calls_to(&e, NI_ALLOC).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![16, 16, 16, 15, 15, 15, 15]);
        assert!(calls_to(&e, NI_SPRINTF).iter().all(|w| w[3] == name));
    }

    #[test]
    fn start_animations_with_a_time_walks_controllers_properties_and_children() {
        let mut e = engine();
        // A node (an NiAVObject) with one controller, one property with a controller and
        // one child without anything.
        let node = e.mem.alloc(0xb0);
        e.mem.set_u32(node, VTABLE);
        e.mem.set_u32(node + 0xac, OWN_RTTI);
        e.mem.set_u32(node + 0xa8, 1);
        let c = controller(&mut e);
        e.mem.set_u32(node + 0xc, c.addr());
        let property = e.mem.alloc(0xb0);
        e.mem.set_u32(property, VTABLE);
        let pc = controller(&mut e);
        e.mem.set_u32(property + 0xc, pc.addr());
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, property);
        e.mem.set_u32(node + 0x24, item);
        let child = e.mem.alloc(0xb0);
        e.mem.set_u32(child, VTABLE);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children + 4, child);
        e.mem.set_u32(node + 0xa0, children);
        e.mem.set_u16(node + 0xa6, 2);
        e.call_log = Some(vec![]);
        // A signalling NaN is pushed quiet.
        e.call(0x00a6_d390, &args![node, 0x7f80_0001u32]);
        assert_eq!(
            calls_to(&e, START_DOUBLE),
            vec![vec![c.addr(), 0x7fc0_0001], vec![pc.addr(), 0x7fc0_0001]]
        );
        let rtti_asked: Vec<u32> = calls_to(&e, RTTI_DOUBLE).iter().map(|w| w[0]).collect();
        assert_eq!(rtti_asked, vec![node, property, child]);
    }

    #[test]
    fn deleting_destructor_releases_the_next_controller() {
        let mut e = engine();
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let a = controller(&mut e);
        let b = controller(&mut e);
        e.set(b, NiTimeController::m_uiRefCount, 1);
        e.set(a, NiTimeController::m_spNext, b.addr());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_d590, &args![a, 1u32]).u32(), a.addr());
        assert_eq!(e.mem.u32(a.addr()), NI_TIME_CONTROLLER_VTABLE);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![b.addr()]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![a.addr(), 0x34]]);
        assert_eq!(e.get(a, NiTimeController::m_spNext), b.addr());
        e.call(0x00a6_d590, &args![a, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn save_binary_links_the_first_streamable_next_and_writes_the_fields() {
        let mut e = engine();
        e.register(BASE_SAVE_BINARY_00A89AF0, |_, _| Ret::default());
        e.register(WRITE_DOUBLE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x5c4);
        e.mem.set_u32(stream, VTABLE);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE_DOUBLE);
        e.mem.set_u32(stream + 0x250, output);
        let a = controller(&mut e);
        let b = controller(&mut e);
        let c = controller(&mut e);
        e.set(a, NiTimeController::m_spNext, b.addr());
        e.set(b, NiTimeController::m_spNext, c.addr());
        e.set(c, NiTimeController::m_fPhase, 1);
        e.set(a, NiTimeController::m_pkTarget, 0x0abc_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d730, &args![a, stream]);
        assert_eq!(
            calls_to(&e, SAVE_LINK_DOUBLE),
            vec![vec![stream, c.addr()], vec![stream, 0x0abc_0000]]
        );
        let writes: Vec<(u32, u32)> = calls_to(&e, WRITE_DOUBLE)
            .iter()
            .map(|w| (w[1] - a.addr(), w[2]))
            .collect();
        assert_eq!(
            writes,
            vec![(0x8, 2), (0xc, 4), (0x10, 4), (0x14, 4), (0x18, 4)]
        );
        assert_eq!(e.mem.u32(output + 4), 18);
        // No streamable controller: a null link.
        e.set(c, NiTimeController::m_fPhase, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a6_d730, &args![a, stream]);
        assert_eq!(calls_to(&e, SAVE_LINK_DOUBLE)[0], vec![stream, 0]);
    }

    #[test]
    fn viewer_strings_list_times_target_types_and_flags() {
        let mut e = engine();
        e.mem.set_u32(NI_TIME_CONTROLLER_RTTI, 0x0104_0000);
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_FLOAT, |_, a| (a[1] & 0xffff).into_ret());
        e.register(VIEWER_STRING_POINTER, |_, a| a[1].into_ret());
        e.register(VIEWER_STRING_BOOL, |_, a| (0x0b00_0000 + a[1]).into_ret());
        e.register(NI_ALLOC, |_, a| (0x0c00_0000 + a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        e.map(0x0109_c000, 0x1000);
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 0xe, 4);
        let c = controller(&mut e);
        for (i, field) in [
            NiTimeController::m_fFrequency,
            NiTimeController::m_fPhase,
            NiTimeController::m_fLoKeyTime,
            NiTimeController::m_fHiKeyTime,
            NiTimeController::m_fStartTime,
            NiTimeController::m_fLastTime,
        ]
        .into_iter()
        .enumerate()
        {
            e.set(c, field, 0x3f80_0000 + i as u32);
        }
        e.set(c, NiTimeController::m_pkTarget, 0x0d00_0000);
        // Anim type 1, cycle type 2, active, not backwards.
        e.set(c, NiTimeController::m_uFlags, 0b0_1101);
        e.call(0x00a6_d830, &args![c, strings]);
        let words: Vec<u32> = (0..13).map(|i| e.mem.u32(base + 4 * i)).collect();
        // The names are not mapped here (length 0): the cycle-type buffer is 0x0c00_0000
        // plus its size 11; "Active" keeps its upper bytes, and "Play Backwards" those of
        // the "Active" string.
        let cycle = 0x0c00_0000 + 11;
        let active = 0x0b00_0000 + ((cycle & !0xff) | 1);
        let backwards = 0x0b00_0000 + (active & !0xff);
        assert_eq!(
            words,
            vec![
                0x0104_0001,
                0,
                1,
                2,
                3,
                4,
                5,
                0x0d00_0000,
                0x0c00_0000 + 12,
                cycle,
                active,
                backwards,
                0
            ]
        );
        assert_eq!(e.mem.u16(strings + 0xa), 12);
    }
}
