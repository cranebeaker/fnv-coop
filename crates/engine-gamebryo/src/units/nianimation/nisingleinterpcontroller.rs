//! `NiAnimation/nisingleinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSingleInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_SINGLE_INTERP_CONTROLLER_RTTI: u32 = 0x011f_3714;

// Translated from 00a38e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_single_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SINGLE_INTERP_CONTROLLER_RTTI)
}

/// `00c9a6b0`: the jump target of `00a35850`.
const JUMP_TARGET_00A35850: u32 = 0x00c9_a6b0;

// Translated from 00a35850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::LinkObject` (Xbox PDB): a 5-byte jump to `00c9a6b0` (engine map: `NiInterpController::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiSingleInterpController@@UAAXAAVNiStream@@@Z`.
pub fn ni_single_interp_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A35850, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a38e20, ni_single_interp_controller_get_rtti() -> Ptr),
        entry!(
            0x00a3_5850,
            ni_single_interp_controller_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a3_8e30,
            ni_single_interp_controller_get_interpolator(u32, u32) -> u32
        ),
        entry!(
            0x00a3_8e40,
            ni_single_interp_controller_set_interpolator(u32, u32, u32)
        ),
        entry!(
            0x00a3_8f50,
            ni_single_interp_controller_process_clone(u32, u32)
        ),
        entry!(
            0x00a3_8f80,
            ni_single_interp_controller_load_binary(u32, u32)
        ),
        entry!(
            0x00a3_9080,
            ni_single_interp_controller_get_interpolator_index_fx(u32, u32) -> u16
        ),
        entry!(
            0x00a3_8df0,
            ni_single_interp_controller_target_is_required_type(u32) -> u32
        ),
        entry!(
            0x00a3_8e90,
            ni_single_interp_controller_guarantee_time_range(u32, f32, f32)
        ),
        entry!(
            0x00a3_8ec0,
            ni_single_interp_controller_reset_time_extrema(u32)
        ),
        entry!(
            0x00a3_8fe0,
            ni_single_interp_controller_register_streamables(u32, u32) -> bool
        ),
        entry!(
            0x00a3_9010,
            ni_single_interp_controller_save_binary(u32, u32)
        ),
        entry!(
            0x00a3_9030,
            ni_single_interp_controller_is_equal(u32, u32) -> bool
        ),
        entry!(
            0x00a3_90a0,
            ni_single_interp_controller_get_viewer_strings(u32, u32)
        ),
    ]
}

/// `NiSingleInterpController::m_spInterpolator` (Xbox PDB +0x34, the same on PC).
const INTERPOLATOR: u32 = 0x34;
/// The imported `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiTimeController::ProcessClone` (`00a6d0c0`), `NiInterpController::LoadBinary`
/// (`00a2f910`; both `thiscall`, one word) and `NiStream::ResolveLinkID` (`00a64620`;
/// `thiscall` on the stream).
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const INTERP_CONTROLLER_LOAD_BINARY: u32 = 0x00a2_f910;
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The 16-bit "invalid index" answer (`.rdata`, 0xFFFF in the exe).
const INVALID_INDEX: u32 = 0x0109_6320;

/// The inlined `NiPointer` assignment of `object` to `m_spInterpolator`: when it differs,
/// the old one released (`InterlockedDecrement`; `DeleteThis`, slot +4, at zero) and the new
/// one counted up.
fn assign_interpolator(e: &mut Engine, this: u32, object: u32) {
    let old = e.mem.u32(this + INTERPOLATOR);
    if old == object {
        return;
    }
    if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 0x4, &[]);
    }
    e.mem.set_u32(this + INTERPOLATOR, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00a38e30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GetInterpolator` (Xbox PDB; `thiscall`, the index it does not
/// read, `RET 4`): `m_spInterpolator`.
pub fn ni_single_interp_controller_get_interpolator(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
) -> u32 {
    e.mem.u32(this + INTERPOLATOR)
}

// Translated from 00a38e40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::SetInterpolator` (Xbox PDB; `thiscall`, (interpolator, index
/// it does not read), `RET 8`): assigns `m_spInterpolator`.
pub fn ni_single_interp_controller_set_interpolator(
    e: &mut Engine,
    this: u32,
    interpolator: u32,
    _unused_2: u32,
) {
    assign_interpolator(e, this, interpolator);
}

// Translated from 00a38f50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::ProcessClone` (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `NiTimeController::ProcessClone`, then the interpolator (when there is one) gets the
/// process too (vtable slot +0x68).
pub fn ni_single_interp_controller_process_clone(e: &mut Engine, this: u32, process: u32) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, process]);
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator != 0 {
        e.vcall(interpolator, 0x68, &args![process]);
    }
}

// Translated from 00a38f80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::LoadBinary` (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `NiInterpController::LoadBinary`, then for a file of version 10.1.0.104 or later the
/// interpolator link (`NiStream::ResolveLinkID`) assigned to `m_spInterpolator`.
pub fn ni_single_interp_controller_load_binary(e: &mut Engine, this: u32, stream: u32) {
    e.call(INTERP_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream + 0xd8) < 0x0a01_0068 {
        return;
    }
    let linked = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_interpolator(e, this, linked);
}

// Translated from 00a39080 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GetInterpolatorIndexFx` (Xbox PDB; `thiscall`, the address of
/// a fixed string, `RET 4`): 0 when the target's name (+0x8 of +0x2C) is that string, else
/// the 16-bit value at `01096320`.
pub fn ni_single_interp_controller_get_interpolator_index_fx(
    e: &mut Engine,
    this: u32,
    name: u32,
) -> u16 {
    let target = e.mem.u32(this + 0x2c);
    if e.mem.u32(target + 0x8) == e.mem.u32(name) {
        return 0;
    }
    e.mem.u16(INVALID_INDEX)
}

/// `NiInterpController::ResetTimeExtrema` (`00a2f8f0`, `thiscall`), `RegisterStreamables`
/// (`00a2f8b0`, `thiscall`, the stream; a bool), `SaveBinary` (`00c27120`), `IsEqual`
/// (`00c283f0`, a bool) and `GetViewerStrings` (`00a2f980`), all `thiscall` with one word.
const INTERP_CONTROLLER_RESET_TIME_EXTREMA: u32 = 0x00a2_f8f0;
const INTERP_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a2_f8b0;
const INTERP_CONTROLLER_SAVE_BINARY: u32 = 0x00c2_7120;
const INTERP_CONTROLLER_IS_EQUAL: u32 = 0x00c2_83f0;
const INTERP_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a2_f980;
/// `NiGetViewerString` (`cdecl`, the text; answers a new string) and the viewer-string
/// array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`, (index, address of the string)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a38df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::TargetIsRequiredType` (Xbox PDB; `thiscall`, no stack words):
/// this controller's own virtual slot +0xE8 run on its target (`NiTimeController::m_pkTarget`
/// +0x2C); answers what that slot answers (`EAX`).
pub fn ni_single_interp_controller_target_is_required_type(e: &mut Engine, this: u32) -> u32 {
    let target = e.mem.u32(this + 0x2c);
    e.vcall(this, 0xe8, &args![target]).u32()
}

// Translated from 00a38e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GuaranteeTimeRange` (Xbox PDB; `thiscall`, the start and end
/// times, `RET 8`): with an interpolator, its `GuaranteeTimeRange` (slot +0xC4) on the two
/// times (copied through the x87 stack).
pub fn ni_single_interp_controller_guarantee_time_range(
    e: &mut Engine,
    this: u32,
    start: f32,
    end: f32,
) {
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator != 0 {
        e.vcall(
            interpolator,
            0xc4,
            &args![
                x87_quiet_bits(start.to_bits()),
                x87_quiet_bits(end.to_bits())
            ],
        );
    }
}

// Translated from 00a38ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::ResetTimeExtrema` (Xbox PDB; `thiscall`): unless flag bit 5
/// (`NiTimeController::m_uFlags` +0x8) is set, `NiInterpController::ResetTimeExtrema` and,
/// with an interpolator, its `GetActiveTimeRange` (slot +0xC0) into the low and high key
/// times (+0x14, +0x18).
pub fn ni_single_interp_controller_reset_time_extrema(e: &mut Engine, this: u32) {
    if (e.mem.u8(this + 8) >> 5) & 1 != 0 {
        return;
    }
    e.call(INTERP_CONTROLLER_RESET_TIME_EXTREMA, &args![this]);
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator != 0 {
        e.vcall(interpolator, 0xc0, &args![this + 0x14, this + 0x18]);
    }
}

// Translated from 00a38fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::RegisterStreamables` (Xbox PDB; `thiscall`, the stream): false
/// when `NiInterpController::RegisterStreamables` says so; otherwise the interpolator's own
/// (slot +0x54) when there is one, and true.
pub fn ni_single_interp_controller_register_streamables(
    e: &mut Engine,
    this: u32,
    stream: u32,
) -> bool {
    if !e
        .call(INTERP_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator != 0 {
        e.vcall(interpolator, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a39010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::SaveBinary` (Xbox PDB; `thiscall`, the stream):
/// `NiInterpController::SaveBinary`, then the stream's link ID of the interpolator (the
/// stream's slot +0x30, `SaveLinkID`).
pub fn ni_single_interp_controller_save_binary(e: &mut Engine, this: u32, stream: u32) {
    e.call(INTERP_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    e.vcall(stream, 0x30, &args![interpolator]);
}

// Translated from 00a39030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::IsEqual` (Xbox PDB; `thiscall`, the other object): false
/// unless `NiInterpController::IsEqual` holds and both have an interpolator or neither does;
/// with both, the interpolators' own `IsEqual` (slot +0x5C) decides.
pub fn ni_single_interp_controller_is_equal(e: &mut Engine, this: u32, other: u32) -> bool {
    if !e
        .call(INTERP_CONTROLLER_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    let own = e.mem.u32(this + INTERPOLATOR);
    if (own == 0) != (e.mem.u32(other + INTERPOLATOR) == 0) {
        return false;
    }
    if own == 0 {
        return true;
    }
    let theirs = e.mem.u32(other + INTERPOLATOR);
    e.vcall(own, 0x5c, &args![theirs]).bool()
}

// Translated from 00a390a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string
/// array): `NiInterpController::GetViewerStrings`, the class name (the first word of the
/// `NiRTTI` record at `011f3714`, through `NiGetViewerString`) appended (the array grown by
/// its step when full, then `SetAt` at the old size with the address of the string's stack
/// word), then the interpolator's own `GetViewerStrings` (slot +0x60) when there is one.
pub fn ni_single_interp_controller_get_viewer_strings(e: &mut Engine, this: u32, strings: u32) {
    e.call(INTERP_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_SINGLE_INTERP_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_SET_SIZE, &args![strings, step + size]);
    }
    // The string's word is the array argument's own stack slot.
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator != 0 {
        e.vcall(interpolator, 0x60, &args![strings]);
    }
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 4, 0x0099_0004);
        e.mem.set_u32(table + 0x68, 0x0099_0068);
        e.mem.set_u32(object, table);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn doubles(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0099_0004, |_, _| Ret::default());
        e.register(0x0099_0068, |_, _| Ret::default());
    }

    #[test]
    fn get_and_set_interpolator_count_references() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x38);
        let first = counted(&mut e, 0);
        let second = counted(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8e40, &[this, first, 9]);
        assert_eq!(e.call(0x00a3_8e30, &[this, 0]).u32(), first);
        assert_eq!(e.mem.u32(first + 4), 1);
        // The same one again: nothing changes.
        e.call(0x00a3_8e40, &[this, first, 9]);
        assert_eq!(e.mem.u32(first + 4), 1);
        e.call(0x00a3_8e40, &[this, second, 9]);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
        e.call(0x00a3_8e40, &[this, 0, 9]);
        assert_eq!(e.call(0x00a3_8e30, &[this, 0]).u32(), 0);
    }

    #[test]
    fn process_clone_passes_the_process_to_the_interpolator() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(TIME_CONTROLLER_PROCESS_CLONE, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8f50, &[this, 0x77]);
        assert!(calls_to(&e, 0x0099_0068).is_empty());
        let interpolator = counted(&mut e, 1);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.call(0x00a3_8f50, &[this, 0x77]);
        assert_eq!(calls_to(&e, 0x0099_0068), vec![vec![interpolator, 0x77]]);
        assert_eq!(calls_to(&e, TIME_CONTROLLER_PROCESS_CLONE).len(), 2);
    }

    #[test]
    fn load_binary_resolves_the_link_for_new_files() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(INTERP_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        let linked = counted(&mut e, 0);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| linked.into_ret());
        let this = e.mem.alloc(0x38);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, 0x0a01_0067);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8f80, &[this, stream]);
        assert!(calls_to(&e, STREAM_RESOLVE_LINK_ID).is_empty());
        e.mem.set_u32(stream + 0xd8, 0x0a01_0068);
        e.call(0x00a3_8f80, &[this, stream]);
        assert_eq!(e.mem.u32(this + INTERPOLATOR), linked);
        assert_eq!(e.mem.u32(linked + 4), 1);
        assert_eq!(calls_to(&e, INTERP_CONTROLLER_LOAD_BINARY).len(), 2);
    }

    #[test]
    fn interpolator_index_matches_the_target_name() {
        let mut e = Engine::new();
        e.map(INVALID_INDEX & !0xfff, 0x1000);
        e.mem.set_u16(INVALID_INDEX, 0xffff);
        let this = e.mem.alloc(0x38);
        let target = e.mem.alloc(0x10);
        e.mem.set_u32(target + 8, 0x4444);
        e.mem.set_u32(this + 0x2c, target);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x4444);
        assert_eq!(e.call(0x00a3_9080, &[this, name]).u16(), 0);
        e.mem.set_u32(name, 0x5555);
        assert_eq!(e.call(0x00a3_9080, &[this, name]).u16(), 0xffff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_single_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a38e20, &args![]).u32(),
            NI_SINGLE_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a35850_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a3_5850;
        const TARGET: u32 = 0x00c9_a6b0;
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
mod n005_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable sends every slot `0..0x100` to `0x0099_0000 + slot`.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        let table = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(table + slot, 0x0099_0000 + slot);
        }
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn target_is_required_type_asks_its_own_slot_about_the_target() {
        let mut e = Engine::new();
        e.register(0x0099_00e8, |_, a| (a[1] + 1).into_ret());
        let this = object(&mut e, 0x38);
        e.mem.set_u32(this + 0x2c, 0x4444);
        assert_eq!(e.call(0x00a3_8df0, &[this]).u32(), 0x4445);
    }

    #[test]
    fn guarantee_time_range_passes_quieted_times_to_the_interpolator() {
        let mut e = Engine::new();
        e.register(0x0099_00c4, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8e90, &[this, 1.0f32.to_bits(), 2.0f32.to_bits()]);
        assert!(calls_to(&e, 0x0099_00c4).is_empty());
        let interpolator = object(&mut e, 8);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.call(0x00a3_8e90, &[this, 0x7f80_0001, 2.0f32.to_bits()]);
        assert_eq!(
            calls_to(&e, 0x0099_00c4),
            vec![vec![interpolator, 0x7fc0_0001, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn reset_time_extrema_skips_with_flag_bit_5() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_RESET_TIME_EXTREMA, |_, _| Ret::default());
        e.register(0x0099_00c0, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        let interpolator = object(&mut e, 8);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.mem.set_u8(this + 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8ec0, &[this]);
        assert!(calls_to(&e, INTERP_CONTROLLER_RESET_TIME_EXTREMA).is_empty());
        e.mem.set_u8(this + 8, 0xdf);
        e.call(0x00a3_8ec0, &[this]);
        assert_eq!(
            calls_to(&e, INTERP_CONTROLLER_RESET_TIME_EXTREMA),
            vec![vec![this]]
        );
        assert_eq!(
            calls_to(&e, 0x0099_00c0),
            vec![vec![interpolator, this + 0x14, this + 0x18]]
        );
        // Without an interpolator only the base runs.
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a3_8ec0, &[this]);
        assert_eq!(calls_to(&e, 0x0099_00c0).len(), 1);
    }

    #[test]
    fn register_streamables_follows_the_base() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_REGISTER_STREAMABLES, |_, a| {
            (a[1] != 0x5000).into_ret()
        });
        e.register(0x0099_0054, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        let interpolator = object(&mut e, 8);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a3_8fe0, &[this, 0x5000]).bool());
        assert!(calls_to(&e, 0x0099_0054).is_empty());
        assert!(e.call(0x00a3_8fe0, &[this, 0x6000]).bool());
        assert_eq!(calls_to(&e, 0x0099_0054), vec![vec![interpolator, 0x6000]]);
        e.mem.set_u32(this + INTERPOLATOR, 0);
        assert!(e.call(0x00a3_8fe0, &[this, 0x6000]).bool());
    }

    #[test]
    fn save_binary_saves_the_interpolator_link() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0099_0030, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + INTERPOLATOR, 0x7777);
        let stream = object(&mut e, 0x300);
        e.call_log = Some(vec![]);
        e.call(0x00a3_9010, &[this, stream]);
        assert_eq!(
            calls_to(&e, INTERP_CONTROLLER_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls_to(&e, 0x0099_0030), vec![vec![stream, 0x7777]]);
    }

    #[test]
    fn is_equal_compares_the_interpolators() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_IS_EQUAL, |e, a| {
            (e.mem.u32(a[1] + 0x30) == 0).into_ret()
        });
        e.register(0x0099_005c, |_, a| (a[1] == 0x9999).into_ret());
        let this = e.mem.alloc(0x38);
        let other = e.mem.alloc(0x38);
        // Base unequal.
        e.mem.set_u32(other + 0x30, 1);
        assert!(!e.call(0x00a3_9030, &[this, other]).bool());
        e.mem.set_u32(other + 0x30, 0);
        // Neither has an interpolator.
        assert!(e.call(0x00a3_9030, &[this, other]).bool());
        // Only the other has one.
        e.mem.set_u32(other + INTERPOLATOR, 0x9999);
        assert!(!e.call(0x00a3_9030, &[this, other]).bool());
        // Both: the interpolators decide.
        let interpolator = object(&mut e, 8);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        assert!(e.call(0x00a3_9030, &[this, other]).bool());
        e.mem.set_u32(other + INTERPOLATOR, 0x8888);
        assert!(!e.call(0x00a3_9030, &[this, other]).bool());
        // Only this one has one.
        e.mem.set_u32(other + INTERPOLATOR, 0);
        assert!(!e.call(0x00a3_9030, &[this, other]).bool());
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name_then_asks_the_interpolator() {
        let mut e = Engine::new();
        e.map(NI_SINGLE_INTERP_CONTROLLER_RTTI & !0xfff, 0x1000);
        e.set_global(NI_SINGLE_INTERP_CONTROLLER_RTTI, 0x0123_4567u32);
        e.register(INTERP_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0123_4568);
            Ret::default()
        });
        e.register(0x0099_0060, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 0xe, 2);
        let interpolator = object(&mut e, 8);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.call_log = Some(vec![]);
        e.call(0x00a3_90a0, &[this, strings]);
        assert_eq!(
            calls_to(&e, INTERP_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_SIZE), vec![vec![strings, 6]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 4]);
        assert_eq!(calls_to(&e, 0x0099_0060), vec![vec![interpolator, strings]]);
        // Room left and no interpolator: no growth, no interpolator call.
        e.mem.set_u16(strings + 8, 8);
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_90a0, &[this, strings]);
        assert!(calls_to(&e, STRINGS_SET_SIZE).is_empty());
        assert!(calls_to(&e, 0x0099_0060).is_empty());
    }
}
