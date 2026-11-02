//! `NiAnimation/nifloatinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_INTERP_CONTROLLER_RTTI: u32 = 0x011f_4220;

// Translated from 00a57660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_INTERP_CONTROLLER_RTTI)
}

/// `00a38f80`: the jump target of `00a57c70`.
const JUMP_TARGET_00A57C70: u32 = 0x00a3_8f80;

// Translated from 00a57c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a38f80` (engine map: `NiSingleInterpController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiFloatInterpController@@UAAXAAVNiStream@@@Z`.
pub fn ni_float_interp_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57C70, &args![this, stream]);
}

/// The controller's `GetTargetFloatValue` (virtual slot +0xEC, Xbox PDB
/// name; `(&value)`).
const SLOT_GET_TARGET_FLOAT_VALUE: u32 = 0xec;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiFloatInterpolator`'s constructor (`00a3c390`; `thiscall`: the float
/// value), `NiFloatInterpolator::SetPoseValue` (`00a3c500`; `thiscall`: the
/// float) and `NiFloatInterpolator::ms_RTTI` (`011f3738`).
const FLOAT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_c390;
const FLOAT_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a3_c500;
const NI_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;
/// `00653290` (`thiscall` on an object, `RET 4`: an `NiRTTI` record): the
/// object when it is of that class (or derived), else 0.
const OBJECT_CAST: u32 = 0x0065_3290;
/// `NiSingleInterpController::GetViewerStrings` (`00a390a0`),
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name) and the
/// viewer-string array's grow (`0096ad30`) and store (`0096ae90`).
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a3_90a0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a575e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpController::CreatePoseInterpolator` (Xbox PDB; `RET 4`;
/// the index word is not read): the controller's `GetTargetFloatValue`
/// (slot +0xEC) into a stack float, then a 0x18-byte `NiFloatInterpolator`
/// holding it (`00a3c390`, the float copied through the x87 stack). Answers
/// the constructor's result, 0 when the allocation fails.
pub fn ni_float_interp_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    e.with_stack(4, |e, value| {
        // The local is the word `PUSH ECX` reserved, holding `this`.
        e.mem.set_u32(value.addr(), this.addr());
        e.vcall(this.addr(), SLOT_GET_TARGET_FLOAT_VALUE, &args![value]);
        let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
        if block == 0 {
            return Ptr::new(0);
        }
        let value = x87_quiet_bits(e.mem.u32(value.addr()));
        e.call(FLOAT_INTERPOLATOR_CONSTRUCT, &args![block, value])
            .ptr()
    })
}

// Translated from 00a57670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpController::SynchronizePoseInterpolator` (Xbox PDB; `RET
/// 8`: interpolator, index; the index word is not read): the interpolator
/// cast to `NiFloatInterpolator` (`00653290`; a null interpolator gives 0
/// without the call), the controller's `GetTargetFloatValue` (slot +0xEC)
/// into the interpolator argument's own stack word, then `SetPoseValue` of
/// that float on the cast result (even when it is 0).
pub fn ni_float_interp_controller_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    _unused_2: u32,
) {
    let cast = if interpolator.addr() == 0 {
        0
    } else {
        e.call(
            OBJECT_CAST,
            &args![interpolator, NI_FLOAT_INTERPOLATOR_RTTI],
        )
        .u32()
    };
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), interpolator.addr());
        e.vcall(this.addr(), SLOT_GET_TARGET_FLOAT_VALUE, &args![value]);
        let value = x87_quiet_bits(e.mem.u32(value.addr()));
        e.call(FLOAT_INTERPOLATOR_SET_POSE_VALUE, &args![cast, value]);
    });
}

// Translated from 00a576c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpController::GetViewerStrings` (Xbox PDB):
/// `NiSingleInterpController::GetViewerStrings`, then the class name (the
/// `NiRTTI` record's first word through `NiGetViewerString`), appended (the
/// array grown by its step when full) through the argument's own stack
/// word.
pub fn ni_float_interp_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    let name = e.mem.u32(NI_FLOAT_INTERP_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
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
        entry!(0x00a57660, ni_float_interp_controller_get_rtti() -> Ptr),
        entry!(
            0x00a5_7c70,
            ni_float_interp_controller_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_75e0,
            ni_float_interp_controller_create_pose_interpolator(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a5_7670,
            ni_float_interp_controller_synchronize_pose_interpolator(Ptr, Ptr, u32)
        ),
        entry!(
            0x00a5_76c0,
            ni_float_interp_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_float_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57660, &args![]).u32(),
            NI_FLOAT_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a57c70_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7c70;
        const TARGET: u32 = 0x00a3_8f80;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    const TARGET_FLOAT_VALUE: u32 = 0x0370_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A controller whose `GetTargetFloatValue` writes the signalling NaN
    /// 0x7f800001.
    fn controller(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + SLOT_GET_TARGET_FLOAT_VALUE, TARGET_FLOAT_VALUE);
        e.register(TARGET_FLOAT_VALUE, |e, a| {
            e.mem.set_u32(a[1], 0x7f80_0001);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn create_pose_interpolator_holds_the_target_value() {
        let mut e = Engine::new();
        let this = controller(&mut e);
        let block = e.mem.alloc(0x18);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(FLOAT_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_75e0, &args![this, 0u32]).u32(), block);
        assert_eq!(calls(&e, TARGET_FLOAT_VALUE).len(), 1);
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_CONSTRUCT),
            vec![vec![block, 0x7fc0_0001]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_75e0, &args![this, 0u32]).u32(), 0);
        assert_eq!(calls(&e, TARGET_FLOAT_VALUE).len(), 2);
        assert_eq!(calls(&e, FLOAT_INTERPOLATOR_CONSTRUCT).len(), 1);
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_target_value() {
        let mut e = Engine::new();
        let this = controller(&mut e);
        e.register(OBJECT_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(FLOAT_INTERPOLATOR_SET_POSE_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a5_7670, &args![this, 0x5000u32, 0u32]);
        assert_eq!(
            calls(&e, OBJECT_CAST),
            vec![vec![0x5000, NI_FLOAT_INTERPOLATOR_RTTI]]
        );
        e.call(0x00a5_7670, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&e, OBJECT_CAST).len(), 1);
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0x5001, 0x7fc0_0001], vec![0, 0x7fc0_0001]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x7100u32.into_ret());
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7100);
            Ret::default()
        });
        e.map(NI_FLOAT_INTERP_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_FLOAT_INTERP_CONTROLLER_RTTI, 0x5151);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a5_76c0, &args![0x3000u32, strings]);
        assert_eq!(
            calls(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![0x3000, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151]]);
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 3]]);
        assert_eq!(calls(&e, STRINGS_SET_AT)[0][..2], [strings, 1]);
    }
}
