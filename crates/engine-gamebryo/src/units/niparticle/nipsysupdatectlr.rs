//! `NiParticle/nipsysupdatectlr.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nitimecontroller::NiTimeController;

// Translated from 00c27140 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00a6d290`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0x34 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00c27140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_d290;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
}

// Translated from 00c27130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2690.
pub fn ni_p_sys_update_ctlr_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2690
}

/// `00aa13e0` (`cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00a6d220`: the `NiTimeController` constructor (`thiscall`).
const TIME_CONTROLLER_CONSTRUCTOR: u32 = 0x00a6_d220;
/// `00a6cfe0`: `NiTimeController`'s member copy, `thiscall`, `(clone,
/// process)`.
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
/// The vtable of `NiPSysUpdateCtlr` (RTTI `.?AVNiPSysUpdateCtlr@@`).
const VTABLE: u32 = 0x010b_e6b4;
/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, one `float`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// `NiPSysEmitterCtlr::ms_RTTI` (the class's `NiRTTI` record).
const NI_P_SYS_EMITTER_CTLR_RTTI: u32 = 0x0120_24e8;
/// The `float` the update stores as the low key time first (`ff7fffff`,
/// `-FLT_MAX`, in the exe's data).
const LO_KEY_TIME_START: u32 = 0x0101_5f5c;
/// The `float` it stores as the high key time first (`010be6a8`).
const HI_KEY_TIME_START: u32 = 0x010b_e6a8;
/// The `double` the last time is compared with (`010241b0`, `-FLT_MAX`):
/// no time yet.
const NO_TIME_DOUBLE: u32 = 0x0102_41b0;
/// `NiObjectNET::m_spControllers` (Xbox PDB) +0x0C.
const TARGET_CONTROLLERS: u32 = 0x0c;
/// `NiTimeController::m_spNext` (Xbox PDB) +0x30.
const CONTROLLER_NEXT: u32 = 0x30;
/// `NiTimeController::m_uFlags` (Xbox PDB) +0x08 of the emitter
/// controller; `NiPSysModifierCtlr::m_pkModifier` (Xbox PDB) +0x3C, its
/// emitter; `NiPSysEmitter::m_fLifeSpan` (Xbox PDB) +0x48.
const CONTROLLER_FLAGS: u32 = 0x08;
const CONTROLLER_MODIFIER: u32 = 0x3c;
const EMITTER_LIFE_SPAN: u32 = 0x48;
/// `NiObject::GetRTTI` (vtable slot +0x08); `NiRTTI::m_pkBaseRTTI` +4.
const GET_RTTI_SLOT: u32 = 0x08;
const RTTI_BASE: u32 = 0x04;
/// The target's vtable slot +0xF0 (`NiParticleSystem::UpdateSystem` in the
/// Xbox PDB), one `float`.
const UPDATE_SYSTEM_SLOT: u32 = 0xf0;

// Translated from 00c27160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::CreateClone` (Xbox PDB): a 0x34-byte object from
/// `00aa13e0` through the `NiTimeController` constructor, its low and high
/// key times set to 0 and this class's vtable (0 when the allocation fails),
/// then `NiTimeController`'s member copy into it (which still runs for a
/// null clone). Returns the clone.
pub fn ni_p_sys_update_ctlr_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x34u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        e.call(TIME_CONTROLLER_CONSTRUCTOR, &args![block]);
        let clone: Ptr<NiTimeController> = Ptr::new(block);
        // `FLDZ; FST [+0x18]; FSTP [+0x14]`.
        e.set(clone, NiTimeController::m_fHiKeyTime, 0);
        e.set(clone, NiTimeController::m_fLoKeyTime, 0);
        e.mem.set_u32(block, VTABLE);
        Ptr::new(block)
    };
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

// Translated from 00c271c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::Update` (Xbox PDB). The PC code takes the time by
/// address (`time` points at a `float`; the Xbox PDB declares a `float`).
/// The key times become the two start values of the exe's data and the
/// flags lose bit 1 and gain bit 2. The target's controller list is searched
/// for the first controller whose RTTI chain contains `NiPSysEmitterCtlr`'s.
/// When the last time is not `-FLT_MAX` (or is NaN), such a controller was
/// found, its flags have neither bit 1 nor bit 2, and its emitter's life
/// span is below the time elapsed since the last time, the last time jumps
/// to `time`. Then `DontDoUpdate` with the time, both key times 0, and,
/// when the target is set and the update is not skipped, the target's
/// system update (slot +0xF0) with the scaled time. The target's controller
/// list is read without a null check on the target.
pub fn ni_p_sys_update_ctlr_update(e: &mut Engine, this: Ptr<NiTimeController>, time: Ptr) {
    let lo_start = x87_quiet_bits(e.global::<u32>(LO_KEY_TIME_START));
    e.set(this, NiTimeController::m_fLoKeyTime, lo_start);
    let hi_start = x87_quiet_bits(e.global::<u32>(HI_KEY_TIME_START));
    e.set(this, NiTimeController::m_fHiKeyTime, hi_start);
    let flags = e.get(this, NiTimeController::m_uFlags);
    e.set(this, NiTimeController::m_uFlags, (flags & 0xfffd) | 4);

    let target = e.get(this, NiTimeController::m_pkTarget);
    let mut controller = e.mem.u32(target + TARGET_CONTROLLERS);
    let mut emitter_ctlr = 0u32;
    let mut emitter = 0u32;
    'search: while controller != 0 {
        let mut rtti = e.vcall(controller, GET_RTTI_SLOT, &[]).u32();
        while rtti != 0 {
            if rtti == NI_P_SYS_EMITTER_CTLR_RTTI {
                emitter = e.mem.u32(controller + CONTROLLER_MODIFIER);
                emitter_ctlr = controller;
                break 'search;
            }
            rtti = e.mem.u32(rtti + RTTI_BASE);
        }
        controller = e.mem.u32(controller + CONTROLLER_NEXT);
    }

    // `FCOMP double; TEST AH,0x44; JNP`: on when not equal or unordered.
    let last = e.get(this, NiTimeController::m_fLastTime);
    let no_time = f64::from_bits(e.global::<u64>(NO_TIME_DOUBLE));
    if !(x87_load32(last) == no_time)
        && emitter_ctlr != 0
        && e.mem.u8(emitter_ctlr + CONTROLLER_FLAGS) & 6 == 0
    {
        let now = e.mem.u32(time.addr());
        let last = e.get(this, NiTimeController::m_fLastTime);
        let elapsed = x87_sub(x87_load32(now), x87_m32(last));
        let life_span = x87_load32(e.mem.u32(emitter + EMITTER_LIFE_SPAN));
        // `FLD life; FCOMPP; TEST AH,5; JP`: on when the life span is below.
        if life_span < elapsed {
            let now = e.mem.u32(time.addr());
            e.set(this, NiTimeController::m_fLastTime, x87_quiet_bits(now));
        }
    }

    let now = x87_quiet_bits(e.mem.u32(time.addr()));
    let skip = e.call(DONT_DO_UPDATE, &args![this, now]).bool();
    e.set(this, NiTimeController::m_fHiKeyTime, 0);
    e.set(this, NiTimeController::m_fLoKeyTime, 0);
    let target = e.get(this, NiTimeController::m_pkTarget);
    if target != 0 && !skip {
        let scaled = x87_quiet_bits(e.get(this, NiTimeController::m_fScaledTime));
        e.vcall(target, UPDATE_SYSTEM_SLOT, &args![scaled]);
    }
}

/// `NiTimeController::SetTarget` (`00a6d450`, `thiscall`, one word) and
/// `GetViewerStrings` (`00a6d830`, `thiscall`, one word).
const TIME_CONTROLLER_SET_TARGET: u32 = 0x00a6_d450;
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
/// `NiTimeController::m_pkTarget` (Xbox PDB) +0x2C.
const CONTROLLER_TARGET: u32 = 0x2c;
/// `00a67260` (unnamed, `thiscall` on a controller, one word): sets its
/// next controller (a reference-counted assignment at +0x30); `00a5c000`
/// (unnamed, `NiMain/niobjectnet.cpp`, `thiscall` on an object, one word):
/// sets its controller list (a reference-counted assignment at +0xC).
const CONTROLLER_SET_NEXT: u32 = 0x00a6_7260;
const OBJECT_SET_CONTROLLERS: u32 = 0x00a5_c000;
/// Imported `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the class name), the
/// viewer-string array's grow (`0096ad30`) and store (`0096ae90`), and the
/// class's `NiRTTI` record.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const UPDATE_CTLR_RTTI: u32 = 0x0120_2690;

// Translated from 00c27290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::SetTarget` (Xbox PDB): `NiTimeController::SetTarget`,
/// then, with a target, moves this controller to the end of the target's
/// controller list: unless it already is the last one (the list walked
/// from the target's +0xC along +0x30), the last controller's next becomes
/// this one (`00a67260`), the target's list starts at this controller's
/// next (`00a5c000`), and this controller's next is released and cleared.
pub fn ni_p_sys_update_ctlr_set_target(e: &mut Engine, this: Ptr, target: Ptr) {
    e.call(TIME_CONTROLLER_SET_TARGET, &args![this, target]);
    let t = this.addr();
    let owner = e.mem.u32(t + CONTROLLER_TARGET);
    if owner == 0 {
        return;
    }
    let mut last = e.mem.u32(owner + TARGET_CONTROLLERS);
    loop {
        let next = e.mem.u32(last + CONTROLLER_NEXT);
        if next == 0 {
            break;
        }
        last = next;
    }
    if last == t {
        return;
    }
    e.call(CONTROLLER_SET_NEXT, &args![last, t]);
    let next = e.mem.u32(t + CONTROLLER_NEXT);
    let owner = e.mem.u32(t + CONTROLLER_TARGET);
    e.call(OBJECT_SET_CONTROLLERS, &args![owner, next]);
    let next = e.mem.u32(t + CONTROLLER_NEXT);
    if next != 0 {
        if e.call(INTERLOCKED_DECREMENT, &args![next + 4]).u32() == 0 {
            e.vcall(next, 4, &args![]);
        }
        e.mem.set_u32(t + CONTROLLER_NEXT, 0);
    }
}

// Translated from 00c27300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateCtlr::GetViewerStrings` (Xbox PDB): the time-controller
/// strings (`00a6d830`, `this` passed on in ECX), then the class name.
pub fn ni_p_sys_update_ctlr_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(UPDATE_CTLR_RTTI);
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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2_7290, ni_p_sys_update_ctlr_set_target(Ptr, Ptr)),
        entry!(
            0x00c2_7300,
            ni_p_sys_update_ctlr_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c27130, ni_p_sys_update_ctlr_get_r_t_t_i() -> u32),
        entry!(0x00c27140, fn_00c27140(Ptr, u32) -> Ptr),
        entry!(0x00c2_7160, ni_p_sys_update_ctlr_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00c2_71c0,
            ni_p_sys_update_ctlr_update(Ptr<NiTimeController>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c27140_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_d290, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00c27140, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c27140, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, 0x00a6_d290), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_update_ctlr_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_7130, &args![]).u32(), 0x0120_2690);
    }

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
    fn create_clone_constructs_clears_the_key_times_and_copies() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(TIME_CONTROLLER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0x7f7f_ffff);
            e.mem.set_u32(a[0] + 0x18, 0xff7f_ffff);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_7160, &args![0x5000u32, 0x6000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(e.mem.u32(clone + 0x14), 0);
        assert_eq!(e.mem.u32(clone + 0x18), 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34]]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_7160, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert!(calls_to(&e, TIME_CONTROLLER_CONSTRUCTOR).is_empty());
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }

    /// A target with one unrelated controller and then an emitter
    /// controller (RTTI chain: own record, then `NiPSysEmitterCtlr`'s), the
    /// emitter's life span `life`; the updated controller's last time
    /// `last`. Returns (engine, this, time slot, emitter controller).
    fn update_world(life: f32, last: f32, skip: bool) -> (Engine, Ptr<NiTimeController>, u32, u32) {
        const OTHER_GET_RTTI: u32 = 0x0040_2000;
        const EMITTER_GET_RTTI: u32 = 0x0040_2010;
        const UPDATE_SYSTEM: u32 = 0x0040_2020;
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x1000);
        e.map(0x010b_e000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        e.set_global(LO_KEY_TIME_START, 0xff7f_ffffu32);
        e.set_global(HI_KEY_TIME_START, 0x7f7f_ffffu32);
        e.set_global(NO_TIME_DOUBLE, -(f32::MAX as f64));
        // The emitter controller's RTTI: a record whose base is the
        // emitter controller's.
        let own_rtti = e.mem.alloc(8);
        e.mem
            .set_u32(own_rtti + RTTI_BASE, NI_P_SYS_EMITTER_CTLR_RTTI);
        e.register(OTHER_GET_RTTI, |_, _| Ret::default());
        e.register_double(EMITTER_GET_RTTI, move |_, _| Ret {
            eax: own_rtti,
            ..Ret::default()
        });
        e.register(UPDATE_SYSTEM, |_, _| Ret::default());
        let skip_word = u32::from(skip);
        e.register_double(DONT_DO_UPDATE, move |_, _| Ret {
            eax: skip_word,
            ..Ret::default()
        });
        e.put_vtable(0x0090_0000, &[0, 0, OTHER_GET_RTTI]);
        e.put_vtable(0x0090_1000, &[0, 0, EMITTER_GET_RTTI]);
        let target_vtable = 0x0090_2000;
        e.put_vtable(target_vtable, &[0; 0xf4 / 4]);
        e.mem
            .set_u32(target_vtable + UPDATE_SYSTEM_SLOT, UPDATE_SYSTEM);

        let emitter = e.mem.alloc(0x60);
        e.mem.set_f32(emitter + EMITTER_LIFE_SPAN, life);
        let emitter_ctlr = e.mem.alloc(0x40);
        e.mem.set_u32(emitter_ctlr, 0x0090_1000);
        e.mem.set_u32(emitter_ctlr + CONTROLLER_MODIFIER, emitter);
        let other = e.mem.alloc(0x34);
        e.mem.set_u32(other, 0x0090_0000);
        e.mem.set_u32(other + CONTROLLER_NEXT, emitter_ctlr);
        let target = e.mem.alloc(0x100);
        e.mem.set_u32(target, target_vtable);
        e.mem.set_u32(target + TARGET_CONTROLLERS, other);

        let this: Ptr<NiTimeController> = e.new_object();
        e.set(this, NiTimeController::m_pkTarget, target);
        e.set(this, NiTimeController::m_uFlags, 0x0003);
        e.set(this, NiTimeController::m_fLastTime, last.to_bits());
        e.set(this, NiTimeController::m_fScaledTime, 2.5f32.to_bits());
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 10.0);
        (e, this, time, emitter_ctlr)
    }

    #[test]
    fn update_jumps_the_last_time_past_the_emitter_life_span_and_updates() {
        let (mut e, this, time, _) = update_world(1.0, 5.0, false);
        e.call_log = Some(vec![]);
        e.call(0x00c2_71c0, &args![this, time]);
        // 10 - 5 > 1: the last time becomes 10.
        assert_eq!(
            e.get(this, NiTimeController::m_fLastTime),
            10.0f32.to_bits()
        );
        assert_eq!(e.get(this, NiTimeController::m_uFlags), 0x0005);
        assert_eq!(e.get(this, NiTimeController::m_fLoKeyTime), 0);
        assert_eq!(e.get(this, NiTimeController::m_fHiKeyTime), 0);
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this.addr(), 10.0f32.to_bits()]]
        );
        let target = e.get(this, NiTimeController::m_pkTarget);
        assert_eq!(
            calls_to(&e, 0x0040_2020),
            vec![vec![target, 2.5f32.to_bits()]]
        );
    }

    #[test]
    fn update_keeps_the_last_time_within_the_life_span_or_without_a_last_time() {
        // Within the life span (10 - 5 < 6).
        let (mut e, this, time, _) = update_world(6.0, 5.0, false);
        e.call(0x00c2_71c0, &args![this, time]);
        assert_eq!(e.get(this, NiTimeController::m_fLastTime), 5.0f32.to_bits());
        // No last time yet (-FLT_MAX).
        let (mut e, this, time, _) = update_world(1.0, -f32::MAX, false);
        e.call(0x00c2_71c0, &args![this, time]);
        assert_eq!(
            e.get(this, NiTimeController::m_fLastTime),
            (-f32::MAX).to_bits()
        );
        // The emitter controller's flags have bit 2.
        let (mut e, this, time, emitter_ctlr) = update_world(1.0, 5.0, false);
        e.mem.set_u16(emitter_ctlr + CONTROLLER_FLAGS, 4);
        e.call(0x00c2_71c0, &args![this, time]);
        assert_eq!(e.get(this, NiTimeController::m_fLastTime), 5.0f32.to_bits());
    }

    #[test]
    fn update_skipped_by_dont_do_update_does_not_update_the_system() {
        let (mut e, this, time, _) = update_world(1.0, 5.0, true);
        e.call_log = Some(vec![]);
        e.call(0x00c2_71c0, &args![this, time]);
        assert!(calls_to(&e, 0x0040_2020).is_empty());
        assert_eq!(e.get(this, NiTimeController::m_fLoKeyTime), 0);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;

    const DELETE_THIS: u32 = 0x0100_0070;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn set_target_moves_the_controller_to_the_end_of_the_list() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_SET_TARGET, |e, a| {
            e.mem.set_u32(a[0] + CONTROLLER_TARGET, a[1]);
            Ret::default()
        });
        e.register(CONTROLLER_SET_NEXT, |_, _| Ret::default());
        e.register(OBJECT_SET_CONTROLLERS, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        // The target's list: this -> other (the last).
        let this = e.mem.alloc(0x34);
        let other = e.mem.alloc(0x34);
        e.mem.set_u32(other, vtable);
        e.mem.set_u32(other + 4, 1);
        e.mem.set_u32(this + CONTROLLER_NEXT, other);
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target + TARGET_CONTROLLERS, this);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7290, &args![this, target]);
        assert_eq!(calls_to(&e, CONTROLLER_SET_NEXT), vec![vec![other, this]]);
        assert_eq!(
            calls_to(&e, OBJECT_SET_CONTROLLERS),
            vec![vec![target, other]]
        );
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![other]]);
        assert_eq!(e.mem.u32(this + CONTROLLER_NEXT), 0);
        // Already last: nothing more than the base call.
        e.mem.set_u32(target + TARGET_CONTROLLERS, this);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7290, &args![this, target]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        // No target: only the base call.
        e.call_log = Some(vec![]);
        e.call(0x00c2_7290, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        e.map(UPDATE_CTLR_RTTI, 4);
        e.set_global(UPDATE_CTLR_RTTI, 0x0555_0000u32);
        e.register(TIME_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7000_0001);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xe, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7300, &args![0x4000u32, strings]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![0x4000, strings]]
        );
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 3]);
    }
}
