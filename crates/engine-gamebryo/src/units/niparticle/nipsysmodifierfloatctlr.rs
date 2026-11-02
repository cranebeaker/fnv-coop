//! `NiParticle/nipsysmodifierfloatctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c32f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c315b0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `012027f8`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_p_sys_modifier_float_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c3_15b0;
    const CLASS_NAME_POINTER: u32 = 0x0120_27f8;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00c32e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27f8.
pub fn ni_p_sys_modifier_float_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27f8
}

/// `NiPSysModifierCtlr::LinkObject` (`00a57630`, `thiscall`, one word).
const MODIFIER_CTLR_LINK_OBJECT: u32 = 0x00a5_7630;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall`, no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00a3c340` (unnamed, `thiscall`, one word: the linked object): the
/// constructor of the 0x18-byte object made from it.
const LINKED_OBJECT_CONSTRUCT: u32 = 0x00a3_c340;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The stream version below which the old layout's extra link is read
/// (`NiStream` `+0xD8`, compared unsigned).
const OLD_FORMAT_VERSION: u32 = 0x0a01_0068;

// Translated from 00c32e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::LinkObject` (Xbox PDB): the modifier-controller
/// part (`00a57630`); for a stream version (`+0xD8`) below `0a010068` the
/// next linked object is wrapped in a new 0x18-byte object (`00a3c340`, null
/// when the allocation fails) that goes into the `NiPointer` at `+0x34`.
pub fn ni_p_sys_modifier_float_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_CTLR_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + 0xd8) >= OLD_FORMAT_VERSION {
        return;
    }
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    let wrapped = if block == 0 {
        0
    } else {
        e.call(LINKED_OBJECT_CONSTRUCT, &args![block, object]).u32()
    };
    let slot = this.addr() + 0x34;
    let old = e.mem.u32(slot);
    if old != wrapped {
        if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
            e.vcall(old, 4, &[]);
        }
        e.mem.set_u32(slot, wrapped);
        if wrapped != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![wrapped + 4]);
        }
    }
}

use crate::units::nimain::nitimecontroller::NiTimeController;
use crate::units::niparticle::nipsysmodifierctlr::NiPSysModifierCtlr;

/// `00c32d80` (`thiscall`, no stack words): the `NiPSysModifierFloatCtlr`
/// constructor (it stores this class's vtable `010c0e44`).
pub(crate) const FLOAT_CTLR_CONSTRUCTOR: u32 = 0x00c3_2d80;
/// `00c32de0` (a 5-byte jump to `00c312f0`): the controller member copy,
/// `thiscall`, `(clone, process)`.
pub(crate) const FLOAT_CTLR_COPY_MEMBERS: u32 = 0x00c3_2de0;
/// `NiFloatInterpolator::NiFloatInterpolator` (`00a3c390`, `thiscall`, the
/// pose value as a `float`) and `NiFloatInterpolator::SetPoseValue`
/// (`00a3c500`, `thiscall`, a `float`).
const FLOAT_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_c390;
const FLOAT_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a3_c500;
/// `00653290` (unnamed): `this` when it is of kind of the `NiRTTI` record it
/// is given, else 0 (`thiscall`, one word).
const DYNAMIC_CAST: u32 = 0x0065_3290;
/// The `NiRTTI` record of `NiFloatInterpolator` (`011f3738`).
const FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;
/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, the time as a
/// `float`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The `float` the scaled time takes when bit 5 of the controller's flags
/// is set: the word at `01096324`.
const UNSET_TIME: u32 = 0x0109_6324;
/// Interpolator vtable slots: the float-interpolator test (+0xa8, no stack
/// words), the "needs update" test (+0xd4) and the float update (+0x9c:
/// time, target, address of the value).
const INTERPOLATOR_IS_FLOAT: u32 = 0xa8;
const INTERPOLATOR_IS_ACTIVE: u32 = 0xd4;
const INTERPOLATOR_UPDATE: u32 = 0x9c;
/// `NiPSysModifierFloatCtlr` vtable slots: `GetTargetFloatValue` (+0xf0,
/// the address of a `float`) and `SetTargetFloatValue` (+0xf4, a `float`).
const GET_TARGET_FLOAT_VALUE: u32 = 0xf0;
const SET_TARGET_FLOAT_VALUE: u32 = 0xf4;
/// The modifier field of the modifier controllers (`m_pkModifier`, +0x3c).
const MODIFIER: u32 = 0x3c;

/// The `CreateClone` the float controller subclasses share: a 0x40-byte
/// object from `00aa13e0` through the `NiPSysModifierFloatCtlr`
/// constructor and the subclass's `vtable` (0 when the allocation fails),
/// then the member copy into it (which still runs for a null clone).
/// Returns the clone.
pub(crate) fn create_clone(e: &mut Engine, this: Ptr, process: Ptr, vtable: u32) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x40u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(FLOAT_CTLR_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, vtable);
        Ptr::new(block)
    };
    e.call(FLOAT_CTLR_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

/// The `GetTargetFloatValue` the subclasses share: the `float` at `offset`
/// in the controller's modifier (`FLD`/`FSTP`) into `value`.
pub(crate) fn get_target_float(e: &mut Engine, this: Ptr, offset: u32, value: Ptr) {
    let modifier = e.mem.u32(this.addr() + MODIFIER);
    let bits = e.mem.u32(modifier.wrapping_add(offset));
    e.mem.set_u32(value.addr(), x87_quiet_bits(bits));
}

/// The `SetTargetFloatValue` the subclasses share: `value` (`FLD`/`FSTP`)
/// into the `float` at `offset` in the controller's modifier.
pub(crate) fn set_target_float(e: &mut Engine, this: Ptr, offset: u32, value: f32) {
    let modifier = e.mem.u32(this.addr() + MODIFIER);
    e.mem.set_u32(
        modifier.wrapping_add(offset),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 00a3a710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::InterpolatorIsCorrectType` (Xbox PDB;
/// `thiscall`, `RET 8`): the interpolator's float test (its slot +0xa8);
/// `this` and the index word are not read. Returns `EAX` as the slot leaves
/// it.
pub fn ni_p_sys_modifier_float_ctlr_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) -> u32 {
    e.vcall(interpolator.addr(), INTERPOLATOR_IS_FLOAT, &args![])
        .u32()
}

// Translated from 00c32da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::CreatePoseInterpolator` (Xbox PDB; `RET 4`,
/// the index word is not read): the target's value (`GetTargetFloatValue`,
/// slot +0xf0, into a stack word that starts as `this`, the pushed `ECX`),
/// then a 0x18-byte `NiFloatInterpolator` with it as the pose value; 0 when
/// the allocation fails. Returns the constructor's `EAX`.
pub fn ni_p_sys_modifier_float_ctlr_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> u32 {
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), this.addr());
        e.vcall(this.addr(), GET_TARGET_FLOAT_VALUE, &args![value]);
        let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
        if block == 0 {
            return 0;
        }
        let pose = x87_quiet_bits(e.mem.u32(value.addr()));
        e.call(FLOAT_INTERPOLATOR_CONSTRUCTOR, &args![block, pose])
            .u32()
    })
}

// Translated from 00c32e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::SynchronizePoseInterpolator` (Xbox PDB;
/// `RET 8`, the index word is not read): the interpolator as an
/// `NiFloatInterpolator` (`00653290` with `011f3738`; 0 for a null one),
/// the target's value (slot +0xf0, into the stack word that held the
/// interpolator argument), then `NiFloatInterpolator::SetPoseValue` on the
/// cast (null included) with it.
pub fn ni_p_sys_modifier_float_ctlr_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) {
    let float_interpolator = if interpolator.is_null() {
        0
    } else {
        e.call(DYNAMIC_CAST, &args![interpolator, FLOAT_INTERPOLATOR_RTTI])
            .u32()
    };
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), interpolator.addr());
        e.vcall(this.addr(), GET_TARGET_FLOAT_VALUE, &args![value]);
        let pose = x87_quiet_bits(e.mem.u32(value.addr()));
        e.call(
            FLOAT_INTERPOLATOR_SET_POSE_VALUE,
            &args![float_interpolator, pose],
        );
    });
}

// Translated from 00c32ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierFloatCtlr::Update` (Xbox PDB). With bit 5 of the flags
/// set the scaled time takes the `float` at `01096324`; otherwise, when
/// `NiTimeController::DontDoUpdate` (with the `float` at `time`) says to
/// skip, the update goes on only with an interpolator whose slot +0xd4
/// answers true. Then, with an interpolator, its float update (slot +0x9c:
/// the scaled time, the target and the address of the stack word that held
/// `time`) runs; when it answers true, that word's `float` goes to
/// `SetTargetFloatValue` (slot +0xf4).
pub fn ni_p_sys_modifier_float_ctlr_update(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    time: Ptr,
) {
    let controller = this.cast::<NiTimeController>();
    if (e.get(controller, NiTimeController::m_uFlags) as u8 >> 5) & 1 != 0 {
        let unset = x87_quiet_bits(e.global::<u32>(UNSET_TIME));
        e.set(controller, NiTimeController::m_fScaledTime, unset);
    } else {
        let now = x87_quiet_bits(e.mem.u32(time.addr()));
        if e.call(DONT_DO_UPDATE, &args![this, now]).bool() {
            let interpolator = e.get(this, NiPSysModifierCtlr::m_spInterpolator);
            if interpolator == 0 {
                return;
            }
            if !e
                .vcall(interpolator, INTERPOLATOR_IS_ACTIVE, &args![])
                .bool()
            {
                return;
            }
        }
    }
    let interpolator = e.get(this, NiPSysModifierCtlr::m_spInterpolator);
    if interpolator == 0 {
        return;
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), time.addr());
        let scaled = x87_quiet_bits(e.get(controller, NiTimeController::m_fScaledTime));
        let target = e.get(controller, NiTimeController::m_pkTarget);
        let updated = e
            .vcall(
                interpolator,
                INTERPOLATOR_UPDATE,
                &args![scaled, target, value],
            )
            .bool();
        if updated {
            let result = x87_quiet_bits(e.mem.u32(value.addr()));
            e.vcall(this.addr(), SET_TARGET_FLOAT_VALUE, &args![result]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a3_a710,
            ni_p_sys_modifier_float_ctlr_interpolator_is_correct_type(Ptr, Ptr, u32) -> u32
        ),
        entry!(
            0x00c3_2da0,
            ni_p_sys_modifier_float_ctlr_create_pose_interpolator(Ptr, u32) -> u32
        ),
        entry!(
            0x00c3_2e90,
            ni_p_sys_modifier_float_ctlr_synchronize_pose_interpolator(Ptr, Ptr, u32)
        ),
        entry!(
            0x00c3_2ee0,
            ni_p_sys_modifier_float_ctlr_update(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00c3_2e10,
            ni_p_sys_modifier_float_ctlr_link_object(Ptr, Ptr)
        ),
        entry!(0x00c32e00, ni_p_sys_modifier_float_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00c32f60,
            ni_p_sys_modifier_float_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_p_sys_modifier_float_ctlr_get_viewer_strings_appends_the_class_name_growing_a_full_array()
    {
        const PARENT: u32 = 0x00c3_15b0;
        const NAME_POINTER: u32 = 0x0120_27f8;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_p_sys_modifier_float_ctlr_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }

    #[test]
    fn ni_p_sys_modifier_float_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_2e00, &args![]).u32(), 0x0120_27f8);
    }
}

#[cfg(test)]
mod link_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn link_object_wraps_the_old_formats_link() {
        const DELETE_THIS: u32 = 0x0100_0004;
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| word(0x4444));
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LINKED_OBJECT_CONSTRUCT, |_, a| word(a[0]));
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, vtable);
        e.mem.set_u32(old + 4, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, old);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, OLD_FORMAT_VERSION);
        e.call_log = Some(vec![]);
        // The current format: only the controller part.
        e.call(0x00c3_2e10, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x34), old);
        e.mem.set_u32(stream + 0xd8, OLD_FORMAT_VERSION - 1);
        e.call(0x00c3_2e10, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(DELETE_THIS, vec![old])));
        let made = e.mem.u32(this + 0x34);
        assert!(log.contains(&(LINKED_OBJECT_CONSTRUCT, vec![made, 0x4444])));
        assert_eq!(e.mem.u32(made + 4), 1);
    }
}

/// Checks the float controller subclasses share.
#[cfg(test)]
pub(crate) mod subclass_checks {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;
    use crate::units::niparticle::nipsysmodifierctlr::{
        INTERP_TARGET_IS_CORRECT_TYPE, SYSTEM_FIND_MODIFIER,
    };

    /// The `CreateClone` at `addr` makes a 0x40-byte clone with `vtable`
    /// through the float controller's constructor and copies into it, and
    /// still copies into null when the allocation fails.
    pub fn create_clone(addr: u32, vtable: u32) {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FLOAT_CTLR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(FLOAT_CTLR_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(addr, &args![0x5000u32, 0x6000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), vtable);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, FLOAT_CTLR_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(addr, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert!(calls_to(&e, FLOAT_CTLR_CONSTRUCTOR).is_empty());
        assert_eq!(
            calls_to(&e, FLOAT_CTLR_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }

    /// A controller whose modifier is a fresh block; returns both.
    fn controller(e: &mut Engine) -> (u32, u32) {
        let modifier = e.mem.alloc(0x40);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + MODIFIER, modifier);
        (this, modifier)
    }

    /// The `GetTargetFloatValue` at `addr` copies the modifier's `float` at
    /// `offset` out, quieting a signalling NaN.
    pub fn get_value(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let (this, modifier) = controller(&mut e);
        let out = e.mem.alloc(4);
        e.mem.set_u32(modifier + offset, 2.5f32.to_bits());
        e.call(addr, &args![this, out]);
        assert_eq!(e.mem.u32(out), 2.5f32.to_bits());
        e.mem.set_u32(modifier + offset, 0xff80_0002);
        e.call(addr, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0xffc0_0002);
    }

    /// The `SetTargetFloatValue` at `addr` stores the value into the
    /// modifier's `float` at `offset`, quieting a signalling NaN.
    pub fn set_value(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let (this, modifier) = controller(&mut e);
        e.call(addr, &args![this, -1.5f32]);
        assert_eq!(e.mem.u32(modifier + offset), (-1.5f32).to_bits());
        e.call(addr, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(modifier + offset), 0x7fc0_0001);
    }

    /// The `InterpTargetIsCorrectType` at `addr` needs the base check and a
    /// named modifier of kind `rtti` in the target system.
    pub fn interp_target(addr: u32, rtti: u32) {
        const GET_RTTI: u32 = 0x0f30_0000;
        const OTHER: u32 = 0x0f31_0000;
        let mut e = Engine::new();
        e.map(OTHER, 0x10);
        e.map(rtti & !0xfff, 0x1000);
        e.mem.set_u32(OTHER + 4, rtti);
        e.mem.set_u32(rtti + 4, 0);
        e.register(GET_RTTI, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        let modifier = e.mem.alloc(8);
        e.mem.set_u32(modifier, vtable);
        e.mem.set_u32(modifier + 4, OTHER);
        e.register(INTERP_TARGET_IS_CORRECT_TYPE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(SYSTEM_FIND_MODIFIER, move |_, _| Ret {
            eax: modifier,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert!(e.call(addr, &args![this, 0x7000u32]).bool());
        assert_eq!(
            calls_to(&e, INTERP_TARGET_IS_CORRECT_TYPE),
            vec![vec![this, 0x7000]]
        );
        assert_eq!(
            calls_to(&e, SYSTEM_FIND_MODIFIER),
            vec![vec![0x7000, this + 0x38]]
        );
        // A modifier of another kind.
        e.mem.set_u32(OTHER + 4, 0);
        assert!(!e.call(addr, &args![this, 0x7000u32]).bool());
        e.mem.set_u32(OTHER + 4, rtti);
        // No modifier of that name.
        e.register(SYSTEM_FIND_MODIFIER, |_, _| Ret::default());
        assert!(!e.call(addr, &args![this, 0x7000u32]).bool());
        // The base check fails: no lookup.
        e.register(INTERP_TARGET_IS_CORRECT_TYPE, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(addr, &args![this, 0x7000u32]).bool());
        assert!(calls_to(&e, SYSTEM_FIND_MODIFIER).is_empty());
    }
}

#[cfg(test)]
mod pose_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    const GET_VALUE_FN: u32 = 0x0f20_0000;
    const SET_VALUE_FN: u32 = 0x0f20_0010;
    const INTERP_FLOAT_FN: u32 = 0x0f20_0020;
    const INTERP_ACTIVE_FN: u32 = 0x0f20_0030;
    const INTERP_UPDATE_FN: u32 = 0x0f20_0040;
    /// A signalling NaN: `FLD`/`FSTP` quiet it.
    const SIGNALLING: u32 = 0x7f80_0001;

    /// A controller whose `GetTargetFloatValue` writes `SIGNALLING` and an
    /// interpolator with the test slots.
    fn controller(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + GET_TARGET_FLOAT_VALUE, GET_VALUE_FN);
        e.mem.set_u32(vtable + SET_TARGET_FLOAT_VALUE, SET_VALUE_FN);
        e.register(GET_VALUE_FN, |e, a| {
            e.mem.set_u32(a[1], SIGNALLING);
            Ret::default()
        });
        e.register(SET_VALUE_FN, |_, _| Ret::default());
        let interpolator = e.mem.alloc(8);
        let ivtable = e.mem.alloc(0x100);
        e.mem.set_u32(interpolator, ivtable);
        e.mem
            .set_u32(ivtable + INTERPOLATOR_IS_FLOAT, INTERP_FLOAT_FN);
        e.mem
            .set_u32(ivtable + INTERPOLATOR_IS_ACTIVE, INTERP_ACTIVE_FN);
        e.mem
            .set_u32(ivtable + INTERPOLATOR_UPDATE, INTERP_UPDATE_FN);
        (this, interpolator)
    }

    #[test]
    fn interpolator_is_correct_type_answers_the_float_test() {
        let mut e = Engine::new();
        let (this, interpolator) = controller(&mut e);
        e.register(INTERP_FLOAT_FN, |_, _| Ret {
            eax: 0x1234_5601,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00a3_a710, &args![this, interpolator, 3u32]).u32();
        assert_eq!(answer, 0x1234_5601);
        assert_eq!(calls_to(&e, INTERP_FLOAT_FN), vec![vec![interpolator]]);
    }

    #[test]
    fn create_pose_interpolator_makes_one_from_the_target_value() {
        let mut e = Engine::new();
        let (this, _) = controller(&mut e);
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FLOAT_INTERPOLATOR_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00c3_2da0, &args![this, 0u32]).u32();
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x18]]);
        assert_eq!(
            calls_to(&e, FLOAT_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![made, 0x7fc0_0001]]
        );
        // The value word starts as `this` when the getter leaves it.
        e.register(GET_VALUE_FN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_2da0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, FLOAT_INTERPOLATOR_CONSTRUCTOR)[0][1], this);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_2da0, &args![this, 0u32]).u32(), 0);
        assert!(calls_to(&e, FLOAT_INTERPOLATOR_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_cast_pose_value() {
        let mut e = Engine::new();
        let (this, interpolator) = controller(&mut e);
        e.register(DYNAMIC_CAST, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(FLOAT_INTERPOLATOR_SET_POSE_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_2e90, &args![this, interpolator, 0u32]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![interpolator, FLOAT_INTERPOLATOR_RTTI]]
        );
        assert_eq!(
            calls_to(&e, FLOAT_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![interpolator + 0x100, 0x7fc0_0001]]
        );
        // A null interpolator: no cast, the pose value still set on null.
        e.call_log = Some(vec![]);
        e.call(0x00c3_2e90, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, DYNAMIC_CAST).is_empty());
        assert_eq!(
            calls_to(&e, FLOAT_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0, 0x7fc0_0001]]
        );
    }

    /// A controller ready for `Update`: flags, scaled time 2.5, target
    /// 0x7700, the interpolator's "active" and update answers.
    fn update_setup(e: &mut Engine, flags: u16, active: bool, updated: bool) -> (u32, u32) {
        e.map(UNSET_TIME & !0xfff, 0x1000);
        e.set_global(UNSET_TIME, 0xff7f_ffffu32);
        let (this, interpolator) = controller(e);
        e.mem.set_u16(this + 8, flags);
        e.mem.set_u32(this + 0x28, 2.5f32.to_bits());
        e.mem.set_u32(this + 0x2c, 0x7700);
        e.mem.set_u32(this + 0x34, interpolator);
        let active = u32::from(active);
        e.register_double(INTERP_ACTIVE_FN, move |_, _| Ret {
            eax: active,
            ..Ret::default()
        });
        let updated = u32::from(updated);
        e.register_double(INTERP_UPDATE_FN, move |e, a| {
            e.mem.set_u32(a[3], SIGNALLING);
            Ret {
                eax: updated,
                ..Ret::default()
            }
        });
        let time = e.mem.alloc(4);
        e.mem.set_u32(time, 1.0f32.to_bits());
        (this, time)
    }

    #[test]
    fn update_with_flag_5_uses_the_unset_time_and_sets_the_value() {
        let mut e = Engine::new();
        let (this, time) = update_setup(&mut e, 0x20, false, true);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2ee0, &args![this, time]);
        assert!(calls_to(&e, DONT_DO_UPDATE).is_empty());
        assert_eq!(e.mem.u32(this + 0x28), 0xff7f_ffff);
        let update = calls_to(&e, INTERP_UPDATE_FN);
        assert_eq!(update.len(), 1);
        assert_eq!(update[0][1..3], [0xff7f_ffff, 0x7700]);
        assert_eq!(calls_to(&e, SET_VALUE_FN), vec![vec![this, 0x7fc0_0001]]);
    }

    #[test]
    fn update_skipped_unless_the_interpolator_is_active() {
        for (active, updates) in [(false, 0usize), (true, 1)] {
            let mut e = Engine::new();
            let (this, time) = update_setup(&mut e, 0, active, false);
            e.register(DONT_DO_UPDATE, |_, _| Ret {
                eax: 1,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x00c3_2ee0, &args![this, time]);
            assert_eq!(
                calls_to(&e, DONT_DO_UPDATE),
                vec![vec![this, 1.0f32.to_bits()]]
            );
            assert_eq!(calls_to(&e, INTERP_UPDATE_FN).len(), updates);
            assert!(calls_to(&e, SET_VALUE_FN).is_empty());
        }
    }

    #[test]
    fn update_runs_when_the_controller_needs_it() {
        let mut e = Engine::new();
        let (this, time) = update_setup(&mut e, 0, false, true);
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_2ee0, &args![this, time]);
        assert!(calls_to(&e, INTERP_ACTIVE_FN).is_empty());
        let update = calls_to(&e, INTERP_UPDATE_FN);
        assert_eq!(update[0][1], 2.5f32.to_bits());
        assert_eq!(calls_to(&e, SET_VALUE_FN).len(), 1);
        e.mem.set_u32(this + 0x34, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2ee0, &args![this, time]);
        assert!(calls_to(&e, INTERP_UPDATE_FN).is_empty());
    }

    #[test]
    fn shared_clone_and_target_value_helpers() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FLOAT_CTLR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(FLOAT_CTLR_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = create_clone(&mut e, Ptr::new(0x5000), Ptr::new(0x6000), 0x0101_0101);
        assert_eq!(e.mem.u32(clone.addr()), 0x0101_0101);
        assert_eq!(
            calls_to(&e, FLOAT_CTLR_COPY_MEMBERS),
            vec![vec![0x5000, clone.addr(), 0x6000]]
        );
        let modifier = e.mem.alloc(0x40);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + MODIFIER, modifier);
        set_target_float(&mut e, Ptr::new(this), 0x18, f32::from_bits(SIGNALLING));
        assert_eq!(e.mem.u32(modifier + 0x18), 0x7fc0_0001);
        e.mem.set_u32(modifier + 0x1c, SIGNALLING);
        let out = e.mem.alloc(4);
        get_target_float(&mut e, Ptr::new(this), 0x1c, Ptr::new(out));
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
    }
}
