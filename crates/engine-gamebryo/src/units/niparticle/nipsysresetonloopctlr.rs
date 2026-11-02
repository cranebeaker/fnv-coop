//! `NiParticle/nipsysresetonloopctlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a6d0f0`: the jump target of `00a2f8b0`.
const JUMP_TARGET_00A2F8B0: u32 = 0x00a6_d0f0;

/// `00a6d730`: the jump target of `00c27120`.
const JUMP_TARGET_00C27120: u32 = 0x00a6_d730;

/// `00a6d5e0`: the jump target of `00c283e0`.
const JUMP_TARGET_00C283E0: u32 = 0x00a6_d5e0;

/// `00a6cc80`: the jump target of `00c283f0`.
const JUMP_TARGET_00C283F0: u32 = 0x00a6_cc80;

// Translated from 00a2f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a6d0f0` (engine map: `NiTimeController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysResetOnLoopCtlr@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A2F8B0, &args![this, stream]).bool()
}

// Translated from 00c27120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::SaveBinary` (Xbox PDB): a 5-byte jump to `00a6d730` (engine map: `NiTimeController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiPSysResetOnLoopCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C27120, &args![this, stream]);
}

// Translated from 00c283e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::LoadBinary` (Xbox PDB): a 5-byte jump to `00a6d5e0` (engine map: `NiTimeController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiPSysResetOnLoopCtlr@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C283E0, &args![this, stream]);
}

// Translated from 00c283f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::IsEqual` (Xbox PDB): a 5-byte jump to `00a6cc80` (engine map: `NiTimeController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiPSysResetOnLoopCtlr@@UAA_NPAVNiObject@@@Z`.
pub fn ni_p_sys_reset_on_loop_ctlr_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C283F0, &args![this, other]).bool()
}

// Translated from 00c28400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26a8.
pub fn ni_p_sys_reset_on_loop_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26a8
}

use crate::units::nimain::nitimecontroller::NiTimeController;
use crate::units::niparticle::nipsysmodifier::{add_class_name, NI_DELETE};
use crate::units::niparticle::nipsysmodifierctlr::{is_kind_of, PARTICLE_SYSTEM_RTTI};

layout! {
    /// `NiPSysResetOnLoopCtlr` (Xbox PDB), 0x38 bytes on PC as on the Xbox:
    /// an `NiTimeController` and the last scaled time.
    pub struct NiPSysResetOnLoopCtlr: 0x38 {
        /// `m_fLastScaledTime` (Xbox PDB), as its bit pattern.
        0x34 m_fLastScaledTime: u32,
    }
}

/// The vtable of `NiPSysResetOnLoopCtlr` (RTTI
/// `.?AVNiPSysResetOnLoopCtlr@@`).
const VTABLE: u32 = 0x010b_e8e4;
/// The `NiRTTI` record of `NiPSysResetOnLoopCtlr` (`012026a8`).
const RTTI: u32 = 0x0120_26a8;
/// `00aa13e0` (`cdecl`, one word): the Gamebryo allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00a6d220` (unnamed, `NiMain/nitimecontroller.cpp`): the
/// `NiTimeController` constructor (`thiscall`, no stack words).
const TIME_CONTROLLER_CONSTRUCTOR: u32 = 0x00a6_d220;
/// `00a6cfe0` (unnamed, `NiMain/nitimecontroller.cpp`): the
/// `NiTimeController` member copy (`thiscall`, `(clone, process)`).
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
/// `00a6d290` (unnamed): `NiTimeController`'s destructor body (`thiscall`).
const TIME_CONTROLLER_DESTRUCTOR: u32 = 0x00a6_d290;
/// `NiTimeController::GetViewerStrings` (`00a6d830`, `thiscall`, one word).
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, the time as a
/// `float`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The `float` `-FLT_MAX` (`01015f5c`) a new clone's last scaled time
/// takes, and the `double` `-FLT_MAX` (`010241b0`) the update compares it
/// with ("no time yet").
const NO_TIME_FLOAT: u32 = 0x0101_5f5c;
const NO_TIME_DOUBLE: u32 = 0x0102_41b0;
/// The byte the update sets in the target when the time loops: +0xd8 as
/// the PC code writes it (not confirmed against a named field).
const TARGET_RESET: u32 = 0xd8;

// Translated from 00c28410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::_scalar_deleting_destructor_` (Xbox PDB):
/// `NiTimeController`'s destructor body (`00a6d290`), then, with bit 0 of
/// `flags`, the sized delete of 0x38 bytes. Returns `this`.
pub fn ni_p_sys_reset_on_loop_ctlr_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(TIME_CONTROLLER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x38u32]);
    }
    this
}

// Translated from 00c28430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::CreateClone` (Xbox PDB): a 0x38-byte object
/// through the `NiTimeController` constructor (`00a6d220`), its last scaled
/// time the `float` at `01015f5c` and this class's vtable (0 when the
/// allocation fails), then `NiTimeController`'s member copy into it (which
/// still runs for a null clone). Returns the clone.
pub fn ni_p_sys_reset_on_loop_ctlr_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: Ptr,
) -> Ptr<NiPSysResetOnLoopCtlr> {
    let block = e.call(NI_NEW, &args![0x38u32]).u32();
    let clone = Ptr::<NiPSysResetOnLoopCtlr>::new(block);
    if block != 0 {
        e.call(TIME_CONTROLLER_CONSTRUCTOR, &args![block]);
        let no_time = x87_quiet_bits(e.global::<u32>(NO_TIME_FLOAT));
        e.set(clone, NiPSysResetOnLoopCtlr::m_fLastScaledTime, no_time);
        e.mem.set_u32(block, VTABLE);
    }
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

// Translated from 00c28490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::Update` (Xbox PDB): clears flag bits 1 and 2;
/// with a target and when `NiTimeController::DontDoUpdate` (with the
/// `float` at `time`) does not skip, the last scaled time becomes the scaled
/// time if it is still exactly `-FLT_MAX` (the `double` at `010241b0`); when
/// the scaled time is below the last one (the animation looped) the
/// target's byte at +0xd8 is set to 1; then the last scaled time takes the
/// scaled time.
pub fn ni_p_sys_reset_on_loop_ctlr_update(
    e: &mut Engine,
    this: Ptr<NiPSysResetOnLoopCtlr>,
    time: Ptr,
) {
    let controller = this.cast::<NiTimeController>();
    let flags = e.get(controller, NiTimeController::m_uFlags);
    e.set(controller, NiTimeController::m_uFlags, flags & 0xfff9);
    if e.get(controller, NiTimeController::m_pkTarget) == 0 {
        return;
    }
    let now = x87_quiet_bits(e.mem.u32(time.addr()));
    if e.call(DONT_DO_UPDATE, &args![this, now]).bool() {
        return;
    }
    // `FCOMP qword; TEST AH,0x44; JP`: copied only when equal (not NaN).
    let last = x87_load32(e.get(this, NiPSysResetOnLoopCtlr::m_fLastScaledTime));
    let no_time: f64 = e.global(NO_TIME_DOUBLE);
    if last == no_time {
        let scaled = x87_quiet_bits(e.get(controller, NiTimeController::m_fScaledTime));
        e.set(this, NiPSysResetOnLoopCtlr::m_fLastScaledTime, scaled);
    }
    // `FLD last; FLD scaled; FCOMPP; TEST AH,5; JP`: set only when the scaled
    // time is below the last one (not NaN).
    let last = x87_load32(e.get(this, NiPSysResetOnLoopCtlr::m_fLastScaledTime));
    let scaled = x87_load32(e.get(controller, NiTimeController::m_fScaledTime));
    if scaled < last {
        let target = e.get(controller, NiTimeController::m_pkTarget);
        e.mem.set_u8(target.wrapping_add(TARGET_RESET), 1);
    }
    let scaled = x87_quiet_bits(e.get(controller, NiTimeController::m_fScaledTime));
    e.set(this, NiPSysResetOnLoopCtlr::m_fLastScaledTime, scaled);
}

// Translated from 00c284f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::TargetIsRequiredType` (Xbox PDB): true when the
/// target is non-null and of kind `NiParticleSystem` (its `NiRTTI` chain
/// reaches `012024e0`).
pub fn ni_p_sys_reset_on_loop_ctlr_target_is_required_type(e: &mut Engine, this: Ptr) -> bool {
    let target = e.get(
        this.cast::<NiTimeController>(),
        NiTimeController::m_pkTarget,
    );
    target != 0 && is_kind_of(e, target, PARTICLE_SYSTEM_RTTI)
}

// Translated from 00c28520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysResetOnLoopCtlr::GetViewerStrings` (Xbox PDB):
/// `NiTimeController::GetViewerStrings`, then the class name (`NiRTTI`
/// `012026a8`) appended.
pub fn ni_p_sys_reset_on_loop_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    add_class_name(e, strings.addr(), RTTI);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_8410,
            ni_p_sys_reset_on_loop_ctlr_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00c2_8430,
            ni_p_sys_reset_on_loop_ctlr_create_clone(Ptr, Ptr) -> Ptr<NiPSysResetOnLoopCtlr>
        ),
        entry!(
            0x00c2_8490,
            ni_p_sys_reset_on_loop_ctlr_update(Ptr<NiPSysResetOnLoopCtlr>, Ptr)
        ),
        entry!(
            0x00c2_84f0,
            ni_p_sys_reset_on_loop_ctlr_target_is_required_type(Ptr) -> bool
        ),
        entry!(
            0x00c2_8520,
            ni_p_sys_reset_on_loop_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c28400, ni_p_sys_reset_on_loop_ctlr_get_r_t_t_i() -> u32),
        entry!(0x00a2_f8b0, ni_p_sys_reset_on_loop_ctlr_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_7120,
            ni_p_sys_reset_on_loop_ctlr_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c2_83e0,
            ni_p_sys_reset_on_loop_ctlr_load_binary(Ptr, Ptr)
        ),
        entry!(0x00c2_83f0, ni_p_sys_reset_on_loop_ctlr_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a2f8b0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a2_f8b0;
        const TARGET: u32 = 0x00a6_d0f0;
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
    fn thunk_00c27120_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_7120;
        const TARGET: u32 = 0x00a6_d730;
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
    fn thunk_00c283e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_83e0;
        const TARGET: u32 = 0x00a6_d5e0;
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
    fn thunk_00c283f0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_83f0;
        const TARGET: u32 = 0x00a6_cc80;
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
    fn ni_p_sys_reset_on_loop_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_8400, &args![]).u32(), 0x0120_26a8);
    }
}

#[cfg(test)]
mod loop_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    #[test]
    fn deleting_destructor_deletes_0x38_bytes_with_bit_0() {
        for flags in [0u32, 1, 0x100] {
            let mut e = Engine::new();
            e.register(TIME_CONTROLLER_DESTRUCTOR, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c2_8410, &args![0x5000u32, flags]).u32(), 0x5000);
            assert_eq!(calls_to(&e, TIME_CONTROLLER_DESTRUCTOR), vec![vec![0x5000]]);
            let deletes = calls_to(&e, NI_DELETE);
            if flags & 1 != 0 {
                assert_eq!(deletes, vec![vec![0x5000, 0x38]]);
            } else {
                assert!(deletes.is_empty());
            }
        }
    }

    #[test]
    fn create_clone_starts_with_no_last_time() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, NO_TIME_FLOAT, 0xff7f_ffff);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(TIME_CONTROLLER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_8430, &args![0x5000u32, 0x6000u32]).u32();
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x38]]);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(e.mem.u32(clone + 0x34), 0xff7f_ffff);
        assert_eq!(calls_to(&e, TIME_CONTROLLER_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_8430, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }

    /// A controller with flags 0xffff, a target, scaled time `scaled` and
    /// last scaled time `last`; `DontDoUpdate` answers `skip`.
    fn update(scaled: f32, last: u32, skip: bool, with_target: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(NO_TIME_DOUBLE & !0xfff, 0x1000);
        e.set_global(NO_TIME_DOUBLE, -(f32::MAX as f64));
        let skip = u32::from(skip);
        e.register_double(DONT_DO_UPDATE, move |_, _| Ret {
            eax: skip,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x38);
        let target = if with_target { e.mem.alloc(0x100) } else { 0 };
        e.mem.set_u16(this + 8, 0xffff);
        e.mem.set_u32(this + 0x28, scaled.to_bits());
        e.mem.set_u32(this + 0x2c, target);
        e.mem.set_u32(this + 0x34, last);
        let time = e.mem.alloc(4);
        e.mem.set_u32(time, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00c2_8490, &args![this, time]);
        (e, this, target)
    }

    #[test]
    fn update_flags_the_target_when_the_time_loops() {
        let (e, this, target) = update(1.0, 2.0f32.to_bits(), false, true);
        assert_eq!(e.mem.u16(this + 8), 0xfff9);
        assert_eq!(calls_to(&e, DONT_DO_UPDATE), vec![vec![this, 0x7fc0_0001]]);
        assert_eq!(e.mem.u8(target + 0xd8), 1);
        assert_eq!(e.mem.u32(this + 0x34), 1.0f32.to_bits());
    }

    #[test]
    fn update_moving_forward_only_keeps_the_time() {
        let (e, this, target) = update(3.0, 2.0f32.to_bits(), false, true);
        assert_eq!(e.mem.u8(target + 0xd8), 0);
        assert_eq!(e.mem.u32(this + 0x34), 3.0f32.to_bits());
        // The first update: the last time is still -FLT_MAX.
        let (e, this, target) = update(-5.0, 0xff7f_ffff, false, true);
        assert_eq!(e.mem.u8(target + 0xd8), 0);
        assert_eq!(e.mem.u32(this + 0x34), (-5.0f32).to_bits());
        // A NaN last time is not "no time", and nothing loops.
        let (e, this, target) = update(1.0, 0x7fc0_0000, false, true);
        assert_eq!(e.mem.u8(target + 0xd8), 0);
        assert_eq!(e.mem.u32(this + 0x34), 1.0f32.to_bits());
    }

    #[test]
    fn update_without_target_or_when_skipped_only_clears_flags() {
        let (e, this, _) = update(1.0, 2.0f32.to_bits(), false, false);
        assert_eq!(e.mem.u16(this + 8), 0xfff9);
        assert!(calls_to(&e, DONT_DO_UPDATE).is_empty());
        assert_eq!(e.mem.u32(this + 0x34), 2.0f32.to_bits());
        let (e, this, target) = update(1.0, 2.0f32.to_bits(), true, true);
        assert_eq!(e.mem.u8(target + 0xd8), 0);
        assert_eq!(e.mem.u32(this + 0x34), 2.0f32.to_bits());
    }

    #[test]
    fn target_is_required_type_needs_a_particle_system() {
        const GET_RTTI: u32 = 0x0f40_0000;
        let mut e = Engine::new();
        e.map(PARTICLE_SYSTEM_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(PARTICLE_SYSTEM_RTTI + 4, 0);
        e.register(GET_RTTI, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, vtable);
        e.mem.set_u32(target + 4, PARTICLE_SYSTEM_RTTI);
        let this = e.mem.alloc(0x38);
        assert!(!e.call(0x00c2_84f0, &args![this]).bool());
        e.mem.set_u32(this + 0x2c, target);
        assert!(e.call(0x00c2_84f0, &args![this]).bool());
        e.mem.set_u32(target + 4, 0);
        assert!(!e.call(0x00c2_84f0, &args![this]).bool());
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0002);
        e.register(TIME_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_8520, &args![0x5000u32, strings]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![0x5000, strings]]
        );
        assert_eq!(*stored.borrow(), vec![0x0abc_0002]);
    }
}
