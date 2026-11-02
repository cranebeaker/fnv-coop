//! `NiAnimation/nifloatextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_3e6c;

// Translated from 00a41230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_EXTRA_DATA_CONTROLLER_RTTI)
}

// Translated from 00a41380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraDataController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a57420`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f3e6c`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_float_extra_data_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_7420;
    const CLASS_NAME_POINTER: u32 = 0x011f_3e6c;
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

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiFloatInterpolator`'s constructor (`00a3c390`; `thiscall`: the float value).
const FLOAT_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_c390;
/// `NiExtraDataController`'s constructor (`00a572a0`; `thiscall`, no stack word).
const EXTRA_DATA_CONTROLLER_CONSTRUCTOR: u32 = 0x00a5_72a0;
/// The member copy `CreateClone` runs on the source (`00c312f0`; `thiscall`: clone,
/// cloning process).
const COPY_MEMBERS: u32 = 0x00c3_12f0;
/// The `NiFloatExtraDataController` virtual table (`010975fc`).
const FLOAT_EXTRA_DATA_CONTROLLER_VTABLE: u32 = 0x0109_75fc;
/// `NiTimeController::DontDoUpdate` (`00a36250`; `thiscall`: time).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// The float the scaled time is set to when the controller's flag 0x20 is set
/// (`01096324`).
const SCALED_TIME_RESET: u32 = 0x0109_6324;

// Translated from 00a41240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraDataController::CreatePoseInterpolator` (Xbox PDB; `thiscall`, `RET 4`; the
/// index word is not read): allocates 0x18 bytes and constructs an `NiFloatInterpolator`
/// (`00a3c390`) holding the float at `+0xC` of the controller's extra data (through the
/// x87 stack twice). Answers the constructor's result, null when the allocation fails.
pub fn ni_float_extra_data_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    // NiExtraDataController::m_spExtraData (Xbox PDB) +0x3C; NiFloatExtraData's value +0x0C.
    let extra = e.mem.u32(this.addr() + 0x3c);
    let value = x87_quiet_bits(e.mem.u32(extra.wrapping_add(0xc)));
    e.call(FLOAT_INTERPOLATOR_CONSTRUCTOR, &args![block, value])
        .ptr()
}

// Translated from 00a41280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraDataController::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning
/// process): a new 0x40-byte controller (`NiExtraDataController`'s constructor
/// `00a572a0`, then this class's virtual table `010975fc`), filled by the source's member
/// copy (`00c312f0`; also called with a null clone when the allocation fails). Answers the
/// clone.
pub fn ni_float_extra_data_controller_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    if block == 0 {
        e.call(COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::new(0);
    }
    e.call(EXTRA_DATA_CONTROLLER_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, FLOAT_EXTRA_DATA_CONTROLLER_VTABLE);
    e.call(COPY_MEMBERS, &args![this, block, cloning]);
    Ptr::new(block)
}

// Translated from 00a41300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraDataController::Update` (Xbox PDB; `thiscall`, `RET 4`: the update data,
/// whose first word is the time). With flag 0x20 of the controller's flags byte the scaled
/// time becomes the float at `01096324`; otherwise `NiTimeController::DontDoUpdate`
/// (`00a36250`) decides, and when it says skip the update still runs if the interpolator
/// answers true from its virtual slot `0xD4`. The interpolator (virtual slot `0x9C`:
/// scaled time, target, address of the value) then updates; on success the value is
/// stored into the extra data's float at `+0xC` (x87 copy). The value goes into the
/// argument's own word.
pub fn ni_float_extra_data_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
    let object = this.addr();
    // NiTimeController::m_uFlags (Xbox PDB) +0x08, bit 5.
    if e.mem.u8(object + 8) & 0x20 != 0 {
        let reset = x87_quiet_bits(e.global::<u32>(SCALED_TIME_RESET));
        e.mem.set_u32(object + 0x28, reset);
    } else {
        let time = x87_quiet_bits(e.mem.u32(update.addr()));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(object + 0x34);
            if interpolator == 0 {
                return;
            }
            if !e.vcall(interpolator, 0xd4, &args![]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(object + 0x34);
    if interpolator == 0 {
        return;
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), update.addr());
        let scaled_time = x87_quiet_bits(e.mem.u32(object + 0x28));
        let target = e.mem.u32(object + 0x2c);
        if !e
            .vcall(interpolator, 0x9c, &args![scaled_time, target, value])
            .bool()
        {
            return;
        }
        let extra = e.mem.u32(object + 0x3c);
        if extra == 0 {
            return;
        }
        let stored = x87_quiet_bits(e.mem.u32(value.addr()));
        e.mem.set_u32(extra + 0xc, stored);
    });
}

/// `00653290` (`thiscall` on an object, `RET 4`: an `NiRTTI` record): the
/// object when it is of that class (or derived), else 0.
const OBJECT_CAST: u32 = 0x0065_3290;
/// `NiFloatInterpolator::ms_RTTI` (`011f3738`).
const NI_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;
/// `NiFloatInterpolator::SetPoseValue` (`00a3c500`; `thiscall`: the float).
const FLOAT_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a3_c500;

// Translated from 00a412d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraDataController::SynchronizePoseInterpolator` (Xbox PDB;
/// `thiscall`, `RET 8`: interpolator, index; the index word is not read):
/// the interpolator cast to `NiFloatInterpolator` (`00653290`; a null
/// interpolator gives 0 without the call) gets the float at +0xC of the
/// controller's extra data as its pose value (`SetPoseValue`, the float
/// copied through the x87 stack; called on the cast result even when it is
/// 0).
pub fn ni_float_extra_data_controller_synchronize_pose_interpolator(
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
    // NiExtraDataController::m_spExtraData (Xbox PDB) +0x3C; NiFloatExtraData's value +0x0C.
    let extra = e.mem.u32(this.addr() + 0x3c);
    let value = x87_quiet_bits(e.mem.u32(extra.wrapping_add(0xc)));
    e.call(FLOAT_INTERPOLATOR_SET_POSE_VALUE, &args![cast, value]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a41230, ni_float_extra_data_controller_get_rtti() -> Ptr),
        entry!(
            0x00a41380,
            ni_float_extra_data_controller_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a4_1240, ni_float_extra_data_controller_create_pose_interpolator(Ptr, u32) -> Ptr),
        entry!(0x00a4_1280, ni_float_extra_data_controller_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a4_1300, ni_float_extra_data_controller_update(Ptr, Ptr)),
        entry!(
            0x00a4_12d0,
            ni_float_extra_data_controller_synchronize_pose_interpolator(Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_float_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a41230, &args![]).u32(),
            NI_FLOAT_EXTRA_DATA_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_float_extra_data_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array(
    ) {
        const PARENT: u32 = 0x00a5_7420;
        const NAME_POINTER: u32 = 0x011f_3e6c;
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
            ni_float_extra_data_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_pose_interpolator_holds_the_extra_data_value() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x18);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.register(FLOAT_INTERPOLATOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        let extra = e.mem.alloc(0x10);
        e.mem.set_u32(extra + 0xc, 0x7f80_0001);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, extra);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_1240, &args![this, 3u32]).u32(), block);
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![block, 0x7fc0_0001]]
        );
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_1240, &args![this, 3u32]).u32(), 0);
        assert_eq!(calls(&e, FLOAT_INTERPOLATOR_CONSTRUCTOR).len(), 1);
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.register(EXTRA_DATA_CONTROLLER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a4_1280, &args![0x3000u32, 0x4000u32]).u32(),
            block
        );
        assert_eq!(e.mem.u32(block), FLOAT_EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(
            calls(&e, EXTRA_DATA_CONTROLLER_CONSTRUCTOR),
            vec![vec![block]]
        );
        assert_eq!(calls(&e, COPY_MEMBERS), vec![vec![0x3000, block, 0x4000]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_1280, &args![0x3000u32, 0x4000u32]).u32(), 0);
        assert_eq!(calls(&e, COPY_MEMBERS)[1], vec![0x3000, 0, 0x4000]);
    }

    const INTERPOLATOR_UPDATE: u32 = 0x0320_0000;
    const INTERPOLATOR_ALWAYS: u32 = 0x0320_0010;

    fn controller(e: &mut Engine, flags: u8, always: bool) -> (u32, u32) {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x9c, INTERPOLATOR_UPDATE);
        e.mem.set_u32(vtable + 0xd4, INTERPOLATOR_ALWAYS);
        let interpolator = e.mem.alloc(8);
        e.mem.set_u32(interpolator, vtable);
        e.register(INTERPOLATOR_UPDATE, |e, a| {
            e.mem.set_u32(a[3], 2.5f32.to_bits());
            1u32.into_ret()
        });
        if always {
            e.register(INTERPOLATOR_ALWAYS, |_, _| 1u32.into_ret());
        } else {
            e.register(INTERPOLATOR_ALWAYS, |_, _| Ret::default());
        }
        let extra = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + 8, flags);
        e.mem.set_u32(this + 0x28, 1.25f32.to_bits());
        e.mem.set_u32(this + 0x2c, 0x6000);
        e.mem.set_u32(this + 0x34, interpolator);
        e.mem.set_u32(this + 0x3c, extra);
        (this, extra)
    }

    #[test]
    fn update_runs_when_time_allows() {
        let mut e = Engine::new();
        e.register(DONT_DO_UPDATE, |_, _| Ret::default());
        let (this, extra) = controller(&mut e, 0, false);
        let update = e.mem.alloc(4);
        e.mem.set_u32(update, 4.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x00a4_1300, &args![this, update]);
        assert_eq!(
            calls(&e, DONT_DO_UPDATE),
            vec![vec![this, 4.0f32.to_bits()]]
        );
        let updates = calls(&e, INTERPOLATOR_UPDATE);
        assert_eq!(updates[0][1..3], [1.25f32.to_bits(), 0x6000]);
        assert_eq!(e.mem.f32(extra + 0xc), 2.5);
    }

    #[test]
    fn update_skipped_unless_the_interpolator_always_updates() {
        let mut e = Engine::new();
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        let (this, extra) = controller(&mut e, 0, false);
        let update = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00a4_1300, &args![this, update]);
        assert!(calls(&e, INTERPOLATOR_UPDATE).is_empty());
        assert_eq!(e.mem.u32(extra + 0xc), 0);
        let (this, extra) = controller(&mut e, 0, true);
        e.call(0x00a4_1300, &args![this, update]);
        assert_eq!(calls(&e, INTERPOLATOR_UPDATE).len(), 1);
        assert_eq!(e.mem.f32(extra + 0xc), 2.5);
    }

    #[test]
    fn update_with_flag_0x20_resets_the_scaled_time() {
        let mut e = Engine::new();
        e.register(DONT_DO_UPDATE, |_, _| 1u32.into_ret());
        e.map(SCALED_TIME_RESET, 4);
        e.set_global(SCALED_TIME_RESET, f32::MIN.to_bits());
        let (this, extra) = controller(&mut e, 0x20, false);
        let update = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00a4_1300, &args![this, update]);
        assert!(calls(&e, DONT_DO_UPDATE).is_empty());
        assert_eq!(e.mem.f32(this + 0x28), f32::MIN);
        assert_eq!(calls(&e, INTERPOLATOR_UPDATE)[0][1], f32::MIN.to_bits());
        assert_eq!(e.mem.f32(extra + 0xc), 2.5);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_extra_data_value() {
        let mut e = Engine::new();
        e.register(OBJECT_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(FLOAT_INTERPOLATOR_SET_POSE_VALUE, |_, _| Ret::default());
        let extra = e.mem.alloc(0x10);
        e.mem.set_u32(extra + 0xc, 0x7f80_0001);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, extra);
        e.call_log = Some(vec![]);
        e.call(0x00a4_12d0, &args![this, 0x5000u32, 7u32]);
        assert_eq!(
            calls(&e, OBJECT_CAST),
            vec![vec![0x5000, NI_FLOAT_INTERPOLATOR_RTTI]]
        );
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0x5001, 0x7fc0_0001]]
        );
        // A null interpolator: no cast, the value still goes to 0.
        e.mem.set_f32(extra + 0xc, 2.5);
        e.call(0x00a4_12d0, &args![this, 0u32, 7u32]);
        assert_eq!(calls(&e, OBJECT_CAST).len(), 1);
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_SET_POSE_VALUE)[1],
            vec![0, 2.5f32.to_bits()]
        );
    }
}
