//! `NiParticle/nipsysmodifierboolctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a57130`: the jump target of `00a41220`.
const JUMP_TARGET_00A41220: u32 = 0x00a5_7130;

/// `00a57780`: the jump target of `00a44cd0`.
const JUMP_TARGET_00A44CD0: u32 = 0x00a5_7780;

/// `00c313a0`: the jump target of `00c32df0`.
const JUMP_TARGET_00C32DF0: u32 = 0x00c3_13a0;

// Translated from 00a41220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::SaveBinary` (Xbox PDB): a 5-byte jump to `00a57130` (engine map: `NiPSysModifierCtlr::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiPSysModifierBoolCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_bool_ctlr_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A41220, &args![this, stream]);
}

// Translated from 00a44cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a57780` (engine map: `NiPSysModifierCtlr::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysModifierBoolCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_modifier_bool_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A44CD0, &args![this, stream]).bool()
}

// Translated from 00c32df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::IsEqual` (Xbox PDB): a 5-byte jump to `00c313a0` (engine map: `NiPSysModifierCtlr::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiPSysModifierBoolCtlr@@UAA_NPAVNiObject@@@Z`.
pub fn ni_p_sys_modifier_bool_ctlr_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C32DF0, &args![this, other]).bool()
}

// Translated from 00c32be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27f0.
pub fn ni_p_sys_modifier_bool_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27f0
}

use crate::units::nimain::nitimecontroller::NiTimeController;
use crate::units::niparticle::nipsysmodifier::{
    GET_OBJECT_FROM_LINK_ID, INTERLOCKED_DECREMENT, INTERLOCKED_INCREMENT, READ_LINK_ID,
    STREAM_VERSION,
};
use crate::units::niparticle::nipsysmodifierctlr::NiPSysModifierCtlr;

/// `NiPSysModifierCtlr::LoadBinary` (`00a571a0`) and `LinkObject`
/// (`00a57630`): `thiscall`, one word.
const MODIFIER_CTLR_LOAD_BINARY: u32 = 0x00a5_71a0;
const MODIFIER_CTLR_LINK_OBJECT: u32 = 0x00a5_7630;
/// The file version from which the controller's interpolator is a link of
/// its own (`CMP [stream+0xd8], 0x0a010068`); older files store a link to
/// bool data instead.
const VERSION_INTERPOLATOR_LINK: u32 = 0x0a01_0068;
/// `00aa13e0` (unnamed, `cdecl`, one word): the Gamebryo allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00a52f00` (unnamed, `NiAnimation/nibooltimelineinterpolator.cpp`):
/// constructs a 0x20-byte interpolator over the data object (`thiscall`,
/// one word).
const BOOL_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_2f00;
/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, the time as a
/// `float`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The `float` the scaled time takes when bit 5 of the controller's flags
/// is set: the word at `01096324`.
const UNSET_TIME: u32 = 0x0109_6324;
/// Interpolator vtable slots: the "needs update" test (+0xd4, no stack
/// words) and the bool update (+0xa0: time, target, address of the value).
const INTERPOLATOR_IS_ACTIVE: u32 = 0xd4;
const INTERPOLATOR_UPDATE: u32 = 0xa0;
/// `NiPSysModifierBoolCtlr::SetTargetBoolValue` (Xbox PDB vtable slot
/// +0xf4, one word).
const SET_TARGET_BOOL_VALUE: u32 = 0xf4;

// Translated from 00a4fbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::LoadBinary` (Xbox PDB): `NiPSysModifierCtlr`'s
/// `LoadBinary`, then, for files before 10.1.0.104, a link id (`00a645b0`,
/// the old bool data).
pub fn ni_p_sys_modifier_bool_ctlr_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_CTLR_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) < VERSION_INTERPOLATOR_LINK {
        e.call(READ_LINK_ID, &args![stream]);
    }
}

// Translated from 00c32bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::LinkObject` (Xbox PDB): `NiPSysModifierCtlr`'s
/// `LinkObject`; for files before 10.1.0.104 the linked object (the old
/// bool data) gets a new interpolator (0x20 bytes from `00aa13e0`,
/// constructed by `00a52f00` on the data; none when the allocation fails)
/// that becomes the controller's interpolator (`NiPointer` assignment: the
/// old one released and deleted through vtable slot +4 at zero, the new one
/// held).
pub fn ni_p_sys_modifier_bool_ctlr_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysModifierCtlr>,
    stream: Ptr,
) {
    e.call(MODIFIER_CTLR_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_INTERPOLATOR_LINK {
        return;
    }
    let data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_NEW, &args![0x20u32]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(BOOL_INTERPOLATOR_CONSTRUCTOR, &args![block, data])
            .u32()
    };
    let old = e.get(this, NiPSysModifierCtlr::m_spInterpolator);
    if old == interpolator {
        return;
    }
    if old != 0 {
        let left = e
            .call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.set(this, NiPSysModifierCtlr::m_spInterpolator, interpolator);
    if interpolator != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
    }
}

// Translated from 00c32cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::Update` (Xbox PDB). With bit 5 of the flags set
/// the scaled time takes the `float` at `01096324`; otherwise, when
/// `NiTimeController::DontDoUpdate` (with the `float` at `time`) says to
/// skip, the update goes on only with an interpolator whose slot +0xd4
/// answers true. Then, with an interpolator, its bool update (slot +0xa0:
/// the scaled time, the target and the address of the stack word that held
/// `time`) runs; when it answers true, that whole word (the bool in its low
/// byte, the rest still the bytes of `time`) goes to `SetTargetBoolValue`
/// (slot +0xf4).
pub fn ni_p_sys_modifier_bool_ctlr_update(
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
            let word = e.mem.u32(value.addr());
            e.vcall(this.addr(), SET_TARGET_BOOL_VALUE, &args![word]);
        }
    });
}

use crate::units::niparticle::nipsysmodifier::add_class_name;

/// `NiBoolInterpolator::NiBoolInterpolator` (`00a532d0`, `thiscall`, the
/// pose value word) and `NiBoolInterpolator::SetPoseValue` (`00a531a0`,
/// `thiscall`, one word).
const BOOL_INTERPOLATOR_POSE_CONSTRUCTOR: u32 = 0x00a5_32d0;
const BOOL_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a5_31a0;
/// `00653290` (unnamed): `this` when it is of kind of the `NiRTTI` record it
/// is given, else 0 (`thiscall`, one word).
const DYNAMIC_CAST: u32 = 0x0065_3290;
/// The `NiRTTI` records of `NiBoolInterpolator` (`011f404c`) and of
/// `NiPSysModifierBoolCtlr` (`012027f0`).
const BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_404c;
const RTTI: u32 = 0x0120_27f0;
/// `NiPSysModifierCtlr::GetViewerStrings` (`00c315b0`, `thiscall`, one
/// word).
const MODIFIER_CTLR_GET_VIEWER_STRINGS: u32 = 0x00c3_15b0;
/// Interpolator vtable slot +0xa4: the bool-interpolator test (no stack
/// words).
const INTERPOLATOR_IS_BOOL: u32 = 0xa4;
/// `NiPSysModifierBoolCtlr::GetTargetBoolValue` (Xbox PDB vtable slot
/// +0xf0, the address of the value).
const GET_TARGET_BOOL_VALUE: u32 = 0xf0;

// Translated from 00c32b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::CreatePoseInterpolator` (Xbox PDB; `RET 4`,
/// the index word is not read): the target's value (`GetTargetBoolValue`,
/// slot +0xf0, into a stack word that starts as `this`, the pushed `ECX`),
/// then a 0x18-byte `NiBoolInterpolator` constructed with that whole word;
/// 0 when the allocation fails. Returns the constructor's `EAX`.
pub fn ni_p_sys_modifier_bool_ctlr_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> u32 {
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), this.addr());
        e.vcall(this.addr(), GET_TARGET_BOOL_VALUE, &args![value]);
        let block = e.call(NI_NEW, &args![0x18u32]).u32();
        if block == 0 {
            return 0;
        }
        let word = e.mem.u32(value.addr());
        e.call(BOOL_INTERPOLATOR_POSE_CONSTRUCTOR, &args![block, word])
            .u32()
    })
}

// Translated from 00c32bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::InterpolatorIsCorrectType` (Xbox PDB;
/// `thiscall`, `RET 8`): the interpolator's bool test (its slot +0xa4);
/// `this` and the index word are not read. Returns `EAX` as the slot leaves
/// it.
pub fn ni_p_sys_modifier_bool_ctlr_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) -> u32 {
    e.vcall(interpolator.addr(), INTERPOLATOR_IS_BOOL, &args![])
        .u32()
}

// Translated from 00c32c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::SynchronizePoseInterpolator` (Xbox PDB;
/// `RET 8`, the index word is not read): the interpolator as an
/// `NiBoolInterpolator` (`00653290` with `011f404c`; 0 for a null one), the
/// target's value (slot +0xf0, into the stack word that held the
/// interpolator argument), then `NiBoolInterpolator::SetPoseValue` on the
/// cast (null included) with that whole word.
pub fn ni_p_sys_modifier_bool_ctlr_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) {
    let bool_interpolator = if interpolator.is_null() {
        0
    } else {
        e.call(DYNAMIC_CAST, &args![interpolator, BOOL_INTERPOLATOR_RTTI])
            .u32()
    };
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), interpolator.addr());
        e.vcall(this.addr(), GET_TARGET_BOOL_VALUE, &args![value]);
        let word = e.mem.u32(value.addr());
        e.call(
            BOOL_INTERPOLATOR_SET_POSE_VALUE,
            &args![bool_interpolator, word],
        );
    });
}

// Translated from 00c32d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifierBoolCtlr::GetViewerStrings` (Xbox PDB):
/// `NiPSysModifierCtlr::GetViewerStrings` (`00c315b0`), then the class name
/// (`NiRTTI` `012027f0`) appended.
pub fn ni_p_sys_modifier_bool_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(MODIFIER_CTLR_GET_VIEWER_STRINGS, &args![this, strings]);
    add_class_name(e, strings.addr(), RTTI);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c3_2b80,
            ni_p_sys_modifier_bool_ctlr_create_pose_interpolator(Ptr, u32) -> u32
        ),
        entry!(
            0x00c3_2bc0,
            ni_p_sys_modifier_bool_ctlr_interpolator_is_correct_type(Ptr, Ptr, u32) -> u32
        ),
        entry!(
            0x00c3_2c70,
            ni_p_sys_modifier_bool_ctlr_synchronize_pose_interpolator(Ptr, Ptr, u32)
        ),
        entry!(
            0x00c3_2d30,
            ni_p_sys_modifier_bool_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00a4_fbd0,
            ni_p_sys_modifier_bool_ctlr_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c3_2bf0,
            ni_p_sys_modifier_bool_ctlr_link_object(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(
            0x00c3_2cb0,
            ni_p_sys_modifier_bool_ctlr_update(Ptr<NiPSysModifierCtlr>, Ptr)
        ),
        entry!(0x00c32be0, ni_p_sys_modifier_bool_ctlr_get_r_t_t_i() -> u32),
        entry!(
            0x00a4_1220,
            ni_p_sys_modifier_bool_ctlr_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_4cd0, ni_p_sys_modifier_bool_ctlr_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c3_2df0, ni_p_sys_modifier_bool_ctlr_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a41220_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_1220;
        const TARGET: u32 = 0x00a5_7130;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a44cd0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_4cd0;
        const TARGET: u32 = 0x00a5_7780;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00c32df0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c3_2df0;
        const TARGET: u32 = 0x00c3_13a0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn ni_p_sys_modifier_bool_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_2be0, &args![]).u32(), 0x0120_27f0);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn load_binary_reads_the_old_data_link_only_in_old_files() {
        for (version, links) in [(0x0a01_0067u32, 1usize), (0x0a01_0068, 0)] {
            let mut e = Engine::new();
            e.register(MODIFIER_CTLR_LOAD_BINARY, |_, _| Ret::default());
            e.register(READ_LINK_ID, |_, _| Ret::default());
            let stream = test_stream::new(&mut e, version, &[]);
            e.call_log = Some(vec![]);
            e.call(0x00a4_fbd0, &args![0x5000u32, stream]);
            assert_eq!(
                calls_to(&e, MODIFIER_CTLR_LOAD_BINARY),
                vec![vec![0x5000, stream]]
            );
            assert_eq!(calls_to(&e, READ_LINK_ID).len(), links);
        }
    }

    #[test]
    fn link_object_makes_an_interpolator_for_old_files() {
        const DELETE: u32 = 0x0f10_0000;
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        e.register(MODIFIER_CTLR_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x6600,
            ..Ret::default()
        });
        e.register(NI_NEW, |e, _| Ret {
            eax: e.mem.alloc(0x20),
            ..Ret::default()
        });
        e.register(BOOL_INTERPOLATOR_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifierCtlr>();
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, vtable);
        e.mem.set_u32(old + 4, 1);
        e.set(this, NiPSysModifierCtlr::m_spInterpolator, old);
        // New files: only the base link.
        let stream = test_stream::new(&mut e, 0x0a01_0068, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2bf0, &args![this, stream]);
        assert!(calls_to(&e, NI_NEW).is_empty());
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_spInterpolator), old);
        let stream = test_stream::new(&mut e, 0x0a01_0067, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2bf0, &args![this, stream]);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x20]]);
        let made = e.get(this, NiPSysModifierCtlr::m_spInterpolator);
        assert_eq!(
            calls_to(&e, BOOL_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![made, 0x6600]]
        );
        assert_eq!(calls_to(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(made + 4), 1);
        // A failed allocation clears the interpolator.
        e.register(NI_NEW, |_, _| Ret::default());
        e.mem.set_u32(made, vtable);
        e.mem.set_u32(made + 4, 2);
        e.call(0x00c3_2bf0, &args![this, stream]);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_spInterpolator), 0);
        assert_eq!(e.mem.u32(made + 4), 1);
    }

    const INTERP_ACTIVE_FN: u32 = 0x0f20_0000;
    const INTERP_UPDATE_FN: u32 = 0x0f20_0010;
    const SET_VALUE_FN: u32 = 0x0f20_0020;

    /// A controller with an interpolator whose update writes 1 into the
    /// value's low byte and answers `updated`.
    fn update_setup(e: &mut Engine, flags: u16, active: bool, updated: bool) -> (u32, u32) {
        e.map(UNSET_TIME & !0xfff, 0x1000);
        e.set_global(UNSET_TIME, 0xff7f_ffffu32);
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + SET_TARGET_BOOL_VALUE, SET_VALUE_FN);
        e.mem.set_u16(this + 8, flags);
        e.mem.set_u32(this + 0x28, 2.5f32.to_bits());
        e.mem.set_u32(this + 0x2c, 0x7700);
        let interpolator = e.mem.alloc(8);
        let ivtable = e.mem.alloc(0x100);
        e.mem.set_u32(interpolator, ivtable);
        e.mem
            .set_u32(ivtable + INTERPOLATOR_IS_ACTIVE, INTERP_ACTIVE_FN);
        e.mem
            .set_u32(ivtable + INTERPOLATOR_UPDATE, INTERP_UPDATE_FN);
        e.mem.set_u32(this + 0x34, interpolator);
        e.register(
            INTERP_ACTIVE_FN,
            if active {
                |_, _| Ret {
                    eax: 1,
                    ..Ret::default()
                }
            } else {
                |_, _| Ret::default()
            },
        );
        e.register(
            INTERP_UPDATE_FN,
            if updated {
                |e, a| {
                    e.mem.set_u8(a[3], 1);
                    Ret {
                        eax: 1,
                        ..Ret::default()
                    }
                }
            } else {
                |_, _| Ret::default()
            },
        );
        e.register(SET_VALUE_FN, |_, _| Ret::default());
        let time = e.mem.alloc(4);
        e.mem.set_u32(time, 1.0f32.to_bits());
        (this, time)
    }

    #[test]
    fn update_with_flag_5_uses_the_unset_time_and_sets_the_value() {
        let mut e = Engine::new();
        let (this, time) = update_setup(&mut e, 0x20, false, true);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2cb0, &args![this, time]);
        assert!(calls_to(&e, DONT_DO_UPDATE).is_empty());
        assert_eq!(e.mem.u32(this + 0x28), 0xff7f_ffff);
        let update = calls_to(&e, INTERP_UPDATE_FN);
        assert_eq!(update.len(), 1);
        assert_eq!(update[0][1..3], [0xff7f_ffff, 0x7700]);
        assert_eq!(
            calls_to(&e, SET_VALUE_FN),
            vec![vec![this, (time & !0xff) | 1]]
        );
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
            e.call(0x00c3_2cb0, &args![this, time]);
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
        e.call(0x00c3_2cb0, &args![this, time]);
        assert!(calls_to(&e, INTERP_ACTIVE_FN).is_empty());
        let update = calls_to(&e, INTERP_UPDATE_FN);
        assert_eq!(update[0][1], 2.5f32.to_bits());
        assert_eq!(calls_to(&e, SET_VALUE_FN).len(), 1);
        // Without an interpolator nothing runs.
        e.mem.set_u32(this + 0x34, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2cb0, &args![this, time]);
        assert!(calls_to(&e, INTERP_UPDATE_FN).is_empty());
    }
}

#[cfg(test)]
mod pose_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    const GET_VALUE_FN: u32 = 0x0f21_0000;
    const INTERP_BOOL_FN: u32 = 0x0f21_0010;

    /// A controller whose `GetTargetBoolValue` writes 1 into the value's low
    /// byte, and an interpolator with the bool-test slot.
    fn controller(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + GET_TARGET_BOOL_VALUE, GET_VALUE_FN);
        e.register(GET_VALUE_FN, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let interpolator = e.mem.alloc(8);
        let ivtable = e.mem.alloc(0x100);
        e.mem.set_u32(interpolator, ivtable);
        e.mem
            .set_u32(ivtable + INTERPOLATOR_IS_BOOL, INTERP_BOOL_FN);
        (this, interpolator)
    }

    #[test]
    fn create_pose_interpolator_passes_the_whole_value_word() {
        let mut e = Engine::new();
        let (this, _) = controller(&mut e);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(BOOL_INTERPOLATOR_POSE_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00c3_2b80, &args![this, 0u32]).u32();
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x18]]);
        assert_eq!(
            calls_to(&e, BOOL_INTERPOLATOR_POSE_CONSTRUCTOR),
            vec![vec![made, (this & !0xff) | 1]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_2b80, &args![this, 0u32]).u32(), 0);
        assert!(calls_to(&e, BOOL_INTERPOLATOR_POSE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn interpolator_is_correct_type_answers_the_bool_test() {
        let mut e = Engine::new();
        let (this, interpolator) = controller(&mut e);
        e.register(INTERP_BOOL_FN, |_, _| Ret {
            eax: 0x5600,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c3_2bc0, &args![this, interpolator, 1u32]).u32(),
            0x5600
        );
        assert_eq!(calls_to(&e, INTERP_BOOL_FN), vec![vec![interpolator]]);
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_cast_pose_value() {
        let mut e = Engine::new();
        let (this, interpolator) = controller(&mut e);
        e.register(DYNAMIC_CAST, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(BOOL_INTERPOLATOR_SET_POSE_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_2c70, &args![this, interpolator, 0u32]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![interpolator, BOOL_INTERPOLATOR_RTTI]]
        );
        assert_eq!(
            calls_to(&e, BOOL_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![interpolator + 0x100, (interpolator & !0xff) | 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c3_2c70, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, DYNAMIC_CAST).is_empty());
        assert_eq!(
            calls_to(&e, BOOL_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0, 1]]
        );
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0003);
        e.register(MODIFIER_CTLR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2d30, &args![0x5000u32, strings]);
        assert_eq!(
            calls_to(&e, MODIFIER_CTLR_GET_VIEWER_STRINGS),
            vec![vec![0x5000, strings]]
        );
        assert_eq!(*stored.borrow(), vec![0x0abc_0003]);
    }
}
