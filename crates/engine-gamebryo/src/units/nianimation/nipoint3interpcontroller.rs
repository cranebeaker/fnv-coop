//! `NiAnimation/nipoint3interpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPoint3InterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_POINT3_INTERP_CONTROLLER_RTTI: u32 = 0x011f_4238;

// Translated from 00a57ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3InterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_point3_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_POINT3_INTERP_CONTROLLER_RTTI)
}

// Translated from 00a57d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3InterpController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a390a0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f4238`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_point3_interp_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a3_90a0;
    const CLASS_NAME_POINTER: u32 = 0x011f_4238;
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

/// `operator new` (`00aa13e0`, `cdecl`, size), the `NiPoint3Interpolator(NiPoint3)`
/// constructor (`00a49650`, `thiscall`, the point by value; answers the object),
/// `NiPoint3Interpolator::SetPoseValue` (`00a494b0`, `thiscall`, the point by value), and
/// the RTTI-checked cast `00653290` (`thiscall` on the object, the `NiRTTI` record; answers
/// the object or null), here with `NiPoint3Interpolator::ms_RTTI` (`011f3f70`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const POINT3_INTERPOLATOR_CONSTRUCT: u32 = 0x00a4_9650;
const POINT3_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a4_94b0;
const RTTI_CAST: u32 = 0x0065_3290;
const NI_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_3f70;

// Translated from 00a57c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3InterpController::CreatePoseInterpolator` (Xbox PDB; `thiscall`, the index it
/// does not read, `RET 4`): the target's point (this controller's slot +0xEC into a local),
/// then a 0x20-byte `NiPoint3Interpolator` made with it (`00a49650`), or null when the
/// allocation fails.
pub fn ni_point3_interp_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    e.with_stack(0xc, |e, point| {
        e.vcall(this.addr(), 0xec, &args![point]);
        let object = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
        if object == 0 {
            return Ptr::new(0);
        }
        let p = point.addr();
        let (x, y, z) = (e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8));
        e.call(POINT3_INTERPOLATOR_CONSTRUCT, &args![object, x, y, z])
            .ptr()
    })
}

// Translated from 00a57cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3InterpController::SynchronizePoseInterpolator` (Xbox PDB; `thiscall`, (the
/// interpolator, an index it does not read), `RET 8`): the interpolator cast to
/// `NiPoint3Interpolator` (`00653290`; null stays null), the target's point (slot +0xEC into
/// a local), and that interpolator's `SetPoseValue` with it (run on the cast's answer, null
/// or not).
pub fn ni_point3_interp_controller_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: u32,
    _unused_2: u32,
) {
    let pose = if interpolator == 0 {
        0
    } else {
        e.call(RTTI_CAST, &args![interpolator, NI_POINT3_INTERPOLATOR_RTTI])
            .u32()
    };
    e.with_stack(0xc, |e, point| {
        e.vcall(this.addr(), 0xec, &args![point]);
        let p = point.addr();
        let (x, y, z) = (e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8));
        e.call(POINT3_INTERPOLATOR_SET_POSE_VALUE, &args![pose, x, y, z]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a57ca0, ni_point3_interp_controller_get_rtti() -> Ptr),
        entry!(
            0x00a5_7c20,
            ni_point3_interp_controller_create_pose_interpolator(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a5_7cb0,
            ni_point3_interp_controller_synchronize_pose_interpolator(Ptr, u32, u32)
        ),
        entry!(
            0x00a57d10,
            ni_point3_interp_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_point3_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57ca0, &args![]).u32(),
            NI_POINT3_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_point3_interp_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array()
    {
        const PARENT: u32 = 0x00a3_90a0;
        const NAME_POINTER: u32 = 0x011f_4238;
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
            ni_point3_interp_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
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

    /// A controller whose slot +0xEC writes the point (1, 2, 3) (as words).
    fn controller(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + 0xec, 0x0099_00ec);
        e.mem.set_u32(this, table);
        e.register(0x0099_00ec, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[1] + 4 * word, word + 1);
            }
            Ret::default()
        });
        this
    }

    #[test]
    fn create_pose_interpolator_builds_one_at_the_target_point() {
        let mut e = Engine::new();
        let this = controller(&mut e);
        e.register(OPERATOR_NEW, |_, _| 0x5000u32.into_ret());
        e.register(POINT3_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_7c20, &[this, 9]).u32(), 0x5000);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x20]]);
        assert_eq!(
            calls_to(&e, POINT3_INTERPOLATOR_CONSTRUCT),
            vec![vec![0x5000, 1, 2, 3]]
        );
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_7c20, &[this, 9]).u32(), 0);
        assert!(calls_to(&e, POINT3_INTERPOLATOR_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, 0x0099_00ec).len(), 1);
    }

    #[test]
    fn synchronize_sets_the_cast_interpolator_pose() {
        let mut e = Engine::new();
        let this = controller(&mut e);
        e.register(RTTI_CAST, |_, a| (a[0] + 0x10).into_ret());
        e.register(POINT3_INTERPOLATOR_SET_POSE_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a5_7cb0, &[this, 0x6000, 0]);
        assert_eq!(
            calls_to(&e, RTTI_CAST),
            vec![vec![0x6000, NI_POINT3_INTERPOLATOR_RTTI]]
        );
        assert_eq!(
            calls_to(&e, POINT3_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0x6010, 1, 2, 3]]
        );
        // No interpolator: no cast, the pose set on null.
        e.call_log = Some(vec![]);
        e.call(0x00a5_7cb0, &[this, 0, 0]);
        assert!(calls_to(&e, RTTI_CAST).is_empty());
        assert_eq!(
            calls_to(&e, POINT3_INTERPOLATOR_SET_POSE_VALUE),
            vec![vec![0, 1, 2, 3]]
        );
    }
}
