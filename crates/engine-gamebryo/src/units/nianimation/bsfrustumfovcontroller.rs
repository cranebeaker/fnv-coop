//! `NiAnimation/bsfrustumfovcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSFrustumFOVController::ms_RTTI` (the class's `NiRTTI` record).
const BS_FRUSTUM_FOV_CONTROLLER_RTTI: u32 = 0x011f_3ec8;

// Translated from 00a42e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_frustum_fov_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_FRUSTUM_FOV_CONTROLLER_RTTI)
}

/// `00a57c00`: the jump target of `00a44ce0`.
const JUMP_TARGET_00A44CE0: u32 = 0x00a5_7c00;

/// `00a57c10`: the jump target of `00a44cf0`.
const JUMP_TARGET_00A44CF0: u32 = 0x00a5_7c10;

// Translated from 00a44ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a57c00` (engine map: `NiBoolInterpController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSFrustumFOVController@@UAAXAAVNiStream@@@Z`.
pub fn bs_frustum_fov_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A44CE0, &args![this, stream]);
}

// Translated from 00a44cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::IsEqual` (Xbox PDB): a 5-byte jump to `00a57c10` (engine map: `NiBoolInterpController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSFrustumFOVController@@UAA_NPAVNiObject@@@Z`.
pub fn bs_frustum_fov_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A44CF0, &args![this, other]).bool()
}

/// `NiSingleInterpController::m_spInterpolator` (Xbox PDB; PC offset).
const INTERPOLATOR: u32 = 0x34;
/// `NiStream::m_uiNifFileVersion` +0xd8 (Xbox PDB; PC offset).
const STREAM_VERSION: u32 = 0xd8;
/// The first file version whose controllers link their interpolator (older
/// ones link float data).
const VERSION_WITH_INTERPOLATOR: u32 = 0x0a01_0068;
/// The size `00a42d00` allocates for the interpolator it makes,
/// and the controller size the deleting destructor frees.
const FLOAT_INTERPOLATOR_SIZE: u32 = 0x18;
const CONTROLLER_SIZE: u32 = 0x38;

/// `NiPSysModifierCtlr::LinkObject` (`00a57630`; the engine map's name for
/// the base link).
const BASE_LINK_OBJECT: u32 = 0x00a5_7630;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a3c340` (unnamed in the engine map; `thiscall`, one word): the
/// `NiFloatInterpolator` constructor taking a float data pointer.
const FLOAT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_c340;
/// `00a6d290` (unit `nitimecontroller.cpp`): the destructor body the
/// deleting destructor runs.
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
/// The interpolator's virtual slot `0xbc` (no arguments) that the link runs
/// on an interpolator it made.
const SLOT_INTERPOLATOR_COLLAPSE: u32 = 0xbc;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots) and
/// `NiRefObject::DeleteThis` (virtual slot 1).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at
/// `+4` and, when it reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

// Translated from 00a42d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsfrustumfovcontroller.cpp`; a
/// `LinkObject`): the base link (`00a57630`), then, in files older than
/// version `0x0a010068`, the linked float data (`GetObjectFromLinkID`)
/// wrapped in a new 0x18-byte `NiFloatInterpolator` (`00a3c340`; null when
/// the allocation fails) that becomes the interpolator (counted; the old
/// one released), and its slot `0xbc` runs.
pub fn fn_00a42d00(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_VERSION)) >= VERSION_WITH_INTERPOLATOR {
        return;
    }
    let data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![FLOAT_INTERPOLATOR_SIZE]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(FLOAT_INTERPOLATOR_CONSTRUCT, &args![block, data])
            .u32()
    };
    let slot = this.addr().wrapping_add(INTERPOLATOR);
    let old = e.mem.u32(slot);
    if old != interpolator {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
        }
    }
    let interpolator = e.mem.u32(slot);
    e.vcall(interpolator, SLOT_INTERPOLATOR_COLLAPSE, &args![]);
}

// Translated from 00a56a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::_scalar_deleting_destructor_` (Xbox PDB name
/// from the engine map; the body would fit any 0x38-byte single-interpolator
/// controller): releases the interpolator, runs the destructor body
/// `00a6d290`, then the sized delete of 0x38 bytes when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_frustum_fov_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let interpolator = e.mem.u32(this.addr().wrapping_add(INTERPOLATOR));
    if interpolator != 0 {
        release(e, interpolator);
    }
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, CONTROLLER_SIZE]);
    }
    this
}

/// `NiTimeController` fields as the PC code uses them: the flags byte
/// +0x08 (bit 5 forces the update), the scaled time +0x28 and the target
/// +0x2C.
const FLAGS: u32 = 0x08;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
/// The target camera's field of view `GetTargetFloatValue` reads (+0xE0)
/// and its frustum (+0xDC, seven words: left, right, top, bottom, near, far
/// and the orthographic flag; the PC offsets `Update` uses).
const CAMERA_FOV: u32 = 0xe0;
const CAMERA_FRUSTUM: u32 = 0xdc;
/// `NiCamera::ms_RTTI` (`011f4a18`; `NiCamera::GetRTTI` `00a70040` answers
/// it).
const NI_CAMERA_RTTI: u32 = 0x011f_4a18;
/// The scaled time a forced update uses (a float in `.rdata`).
const FORCED_UPDATE_TIME: u32 = 0x0109_6324;
/// The two `double` factors `Update` scales the field of view by for the
/// frustum's top and bottom (`0101de30`, `01097800`).
const TOP_FACTOR: u32 = 0x0101_de30;
const BOTTOM_FACTOR: u32 = 0x0109_7800;
/// The CRT x87 helper `00ec9050` (`ST0` in, `ST0` out; `units::crt` has it
/// as an arctangent, name unconfirmed).
const CRT_ATAN: u32 = 0x00ec_9050;
/// `NiTimeController::DontDoUpdate` (`00a36250`, `thiscall`, one float),
/// `NiCamera::SetViewFrustum` (`00a6faf0`, `thiscall (camera, &frustum)`).
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
const SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// The interpolator's virtual slots `Update` uses: `AlwaysUpdate` +0xD4 and
/// the float `Update` +0x9C (`(time, target, &value)`).
const SLOT_ALWAYS_UPDATE: u32 = 0xd4;
const SLOT_UPDATE_FLOAT: u32 = 0x9c;
/// The class's vtable; `00a57640` (the constructor body `CreateObject` and
/// `CreateClone` run) and `00a57620` (the member copy `CreateClone` runs,
/// `thiscall (this, clone, cloning)`).
const VTABLE: u32 = 0x0109_780c;
const FLOAT_INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a5_7640;
const COPY_MEMBERS: u32 = 0x00a5_7620;
/// `NiFloatInterpController::GetViewerStrings` (`00a576c0`) and
/// `NiGetViewerString` (`00a7d330`, `cdecl`).
const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_76c0;
const VIEWER_STRING: u32 = 0x00a7_d330;

// Translated from 00a42cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::GetTargetFloatValue` (Xbox PDB): the target's
/// field of view (+0xE0) through the CRT helper `00ec9050`, rounded to a
/// float, doubled (x87 `FADD ST0,ST0`) and stored to `value`.
pub fn bs_frustum_fov_controller_get_target_float_value(e: &mut Engine, this: Ptr, value: Ptr) {
    let target = e.mem.u32(this.addr() + TARGET);
    let fov = x87_load32(e.mem.u32(target.wrapping_add(CAMERA_FOV)));
    let angle = e.call(CRT_ATAN, &args![fov]).f64();
    let angle = x87_load32(x87_store32(angle));
    e.mem
        .set_u32(value.addr(), x87_store32(x87_add(angle, angle)));
}

/// Whether `target`'s class is, or derives from, the class of `rtti`: its
/// `GetRTTI` (vtable +8) and then each base (`NiRTTI` +4) compared; false
/// for a null target or chain end.
fn is_kind_of(e: &mut Engine, target: u32, rtti: u32) -> bool {
    if target == 0 {
        return false;
    }
    let mut class = e.vcall(target, 8, &args![]).u32();
    while class != 0 {
        if class == rtti {
            return true;
        }
        class = e.mem.u32(class.wrapping_add(4));
    }
    false
}

// Translated from 00a42d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::InterpTargetIsCorrectType` (Xbox PDB;
/// `thiscall`, one stack word): whether the target is an `NiCamera` (its
/// RTTI chain reaches `NiCamera::ms_RTTI`); `this` is not read.
pub fn bs_frustum_fov_controller_interp_target_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    target: Ptr,
) -> bool {
    is_kind_of(e, target.addr(), NI_CAMERA_RTTI)
}

// Translated from 00a42dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::Update` (Xbox PDB; `thiscall (NiUpdateData&)`):
/// a forced update (flags bit 5) sets the scaled time to the float at
/// `01096324`; otherwise `DontDoUpdate` runs on the data's time and, when it
/// says to skip, the update only goes on when there is an interpolator whose
/// `AlwaysUpdate` (vtable +0xD4) says so. Then the interpolator's float
/// `Update` (vtable +0x9C, the scaled time, the target and the argument's
/// own stack word as the value); on success the target camera's frustum is
/// copied and set (`SetViewFrustum`) with left `-v`, right `v`, top `v`
/// times the `double` at `0101de30` and bottom `v` times the one at
/// `01097800` (x87, stored as floats).
pub fn bs_frustum_fov_controller_update(e: &mut Engine, this: Ptr, data: Ptr) {
    let this = this.addr();
    if (e.mem.u8(this + FLAGS) >> 5) & 1 != 0 {
        let forced = x87_quiet_bits(e.mem.u32(FORCED_UPDATE_TIME));
        e.mem.set_u32(this + SCALED_TIME, forced);
    } else {
        let time = x87_quiet_bits(e.mem.u32(data.addr()));
        if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
            let interpolator = e.mem.u32(this + INTERPOLATOR);
            if interpolator == 0 || !e.vcall(interpolator, SLOT_ALWAYS_UPDATE, &args![]).bool() {
                return;
            }
        }
    }
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator == 0 {
        return;
    }
    e.with_stack(0x20, |e, frame| {
        // The argument's stack word (the value) after the frustum local.
        let frustum = frame.addr();
        let slot = frustum + 0x1c;
        e.mem.set_u32(slot, data.addr());
        let time = x87_quiet_bits(e.mem.u32(this + SCALED_TIME));
        let target = e.mem.u32(this + TARGET);
        if !e
            .vcall(interpolator, SLOT_UPDATE_FLOAT, &args![time, target, slot])
            .bool()
        {
            return;
        }
        let camera = e.mem.u32(this + TARGET);
        let value = x87_load32(e.mem.u32(slot));
        for i in 0..7 {
            let word = e.mem.u32(camera.wrapping_add(CAMERA_FRUSTUM + 4 * i));
            e.mem.set_u32(frustum + 4 * i, word);
        }
        e.mem.set_u32(frustum, x87_store32(-value));
        e.mem.set_u32(frustum + 4, x87_store32(value));
        let top = x87_mul(x87_load64(e.global::<u64>(TOP_FACTOR)), value);
        e.mem.set_u32(frustum + 8, x87_store32(top));
        let bottom = x87_mul(value, f64::from_bits(e.global::<u64>(BOTTOM_FACTOR)));
        e.mem.set_u32(frustum + 0xc, x87_store32(bottom));
        e.call(SET_VIEW_FRUSTUM, &args![camera, frustum]);
    });
}

// Translated from 00a42e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::CreateClone` (Xbox PDB): 0x38 bytes from the Ni
/// allocator; when that succeeds the constructor body `00a57640` and the
/// class's vtable, then the member copy (`00a57620`) into it; when it fails
/// the copy still runs, into address 0, and 0 is answered.
pub fn bs_frustum_fov_controller_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![CONTROLLER_SIZE]).u32();
    if object == 0 {
        e.call(COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::NULL;
    }
    e.call(FLOAT_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE);
    e.call(COPY_MEMBERS, &args![this, object, cloning]);
    Ptr::new(object)
}

// Translated from 00a42ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (registered through `NiStream::RegisterLoader` at
/// `00a2d8ad`; Ghidra made no function of it): 0x38 bytes from the Ni
/// allocator and, when that succeeds, the constructor body `00a57640` and
/// the class's vtable. Null when the allocation fails.
pub fn fn_00a42ee0(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![CONTROLLER_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(FLOAT_INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE);
    Ptr::new(object)
}

// Translated from 00a42f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::GetViewerStrings` (Xbox PDB):
/// `NiFloatInterpController`'s strings, then the class name (through the
/// argument's stack word).
pub fn bs_frustum_fov_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(BS_FRUSTUM_FOV_CONTROLLER_RTTI);
    let text = e.call(VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        append_viewer_string(e, strings, slot.addr(), text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a42e80, bs_frustum_fov_controller_get_rtti() -> Ptr),
        entry!(0x00a4_4ce0, bs_frustum_fov_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a4_4cf0, bs_frustum_fov_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4_2d00, fn_00a42d00(Ptr, Ptr)),
        entry!(0x00a5_6a90, bs_frustum_fov_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a4_2cd0,
            bs_frustum_fov_controller_get_target_float_value(Ptr, Ptr)
        ),
        entry!(
            0x00a4_2d90,
            bs_frustum_fov_controller_interp_target_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_2dc0, bs_frustum_fov_controller_update(Ptr, Ptr)),
        entry!(
            0x00a4_2e90,
            bs_frustum_fov_controller_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a4_2ee0, fn_00a42ee0() -> Ptr),
        entry!(
            0x00a4_2f10,
            bs_frustum_fov_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_frustum_fov_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a42e80, &args![]).u32(),
            BS_FRUSTUM_FOV_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a44ce0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_4ce0;
        const TARGET: u32 = 0x00a5_7c00;
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
    fn thunk_00a44cf0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_4cf0;
        const TARGET: u32 = 0x00a5_7c10;
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

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const COLLAPSE: u32 = 0x00ee_0300;
    const DELETE_THIS: u32 = 0x00ee_0400;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn counting(e: &mut Engine, log: &Log) {
        for (address, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let count = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], count);
                Ret {
                    eax: count,
                    ..Ret::default()
                }
            });
        }
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem.set_u32(vtable + SLOT_INTERPOLATOR_COLLAPSE, COLLAPSE);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn fn_00a42d00_wraps_old_float_data_in_an_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        counting(&mut e, &log);
        let made = object(&mut e, 0);
        logged(&mut e, &log, BASE_LINK_OBJECT, 0);
        logged(&mut e, &log, GET_OBJECT_FROM_LINK_ID, 0x7700);
        logged(&mut e, &log, NI_ALLOCATE, made);
        logged(&mut e, &log, FLOAT_INTERPOLATOR_CONSTRUCT, made);
        logged(&mut e, &log, COLLAPSE, 0);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0400_0002);
        let this = e.mem.alloc(0x38);
        e.call(0x00a4_2d00, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (BASE_LINK_OBJECT, vec![this, stream]),
                (GET_OBJECT_FROM_LINK_ID, vec![stream]),
                (NI_ALLOCATE, vec![0x18]),
                (FLOAT_INTERPOLATOR_CONSTRUCT, vec![made, 0x7700]),
                (INTERLOCKED_INCREMENT, vec![made + 4]),
                (COLLAPSE, vec![made]),
            ]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATOR), made);
        log.borrow_mut().clear();
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0068);
        e.call(0x00a4_2d00, &args![this, stream]);
        assert_eq!(*log.borrow(), [(BASE_LINK_OBJECT, vec![this, stream])]);
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        counting(&mut e, &log);
        logged(&mut e, &log, TIME_CONTROLLER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        logged(&mut e, &log, DELETE_THIS, 0);
        let interpolator = object(&mut e, 1);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        assert_eq!(e.call(0x00a5_6a90, &args![this, 1u32]).u32(), this);
        assert_eq!(
            *log.borrow(),
            [
                (INTERLOCKED_DECREMENT, vec![interpolator + 4]),
                (DELETE_THIS, vec![interpolator]),
                (TIME_CONTROLLER_DESTRUCT, vec![this]),
                (NI_SIZED_DELETE, vec![this, 0x38]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a5_6a90, &args![this, 2u32]);
        assert_eq!(*log.borrow(), [(TIME_CONTROLLER_DESTRUCT, vec![this])]);
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{calls, viewer_strings};

    const VTABLE_TEST: u32 = 0x7400_0000;
    fn slot(offset: u32) -> u32 {
        0x7500_0000 + offset
    }

    /// Doubles: the interpolator's `AlwaysUpdate` answers its word +8 and its
    /// float `Update` writes its float +0xC to the value, answering its word
    /// +0x10; `GetRTTI` answers the object's word +0x10; `DontDoUpdate`
    /// answers whether the time is negative; the frustum setter records.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        e.put_vtable(VTABLE_TEST, &slots);
        e.register(slot(SLOT_ALWAYS_UPDATE), |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register(slot(SLOT_UPDATE_FLOAT), |e, a| {
            let v = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[3], v);
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(slot(8), |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(DONT_DO_UPDATE, |_, a| {
            (f32::from_bits(a[1]) < 0.0).into_ret()
        });
        e.register(SET_VIEW_FRUSTUM, |_, _| Ret::default());
        e.map(0x0109_6000, 0x2000);
        e.map(0x0101_d000, 0x1000);
        e.mem.set_f32(FORCED_UPDATE_TIME, 7.0);
        e.set_global(TOP_FACTOR, 0.75f64);
        e.set_global(BOTTOM_FACTOR, -0.75f64);
        e
    }

    fn interpolator(e: &mut Engine, always: u32, value: f32, ok: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, VTABLE_TEST);
        e.mem.set_u32(object + 8, always);
        e.mem.set_f32(object + 0xc, value);
        e.mem.set_u32(object + 0x10, ok);
        object
    }

    fn controller(e: &mut Engine, interp: u32, camera: u32) -> u32 {
        let this = e.mem.alloc(CONTROLLER_SIZE);
        e.mem.set_u32(this + INTERPOLATOR, interp);
        e.mem.set_u32(this + TARGET, camera);
        this
    }

    #[test]
    fn get_target_float_value_doubles_the_helper_result() {
        let mut e = engine();
        e.register(CRT_ATAN, |_, a| {
            let x = f64::take(a, &mut 0);
            (x / 4.0).into_ret()
        });
        let camera = e.mem.alloc(0x100);
        e.mem.set_f32(camera + CAMERA_FOV, 2.0);
        let this = controller(&mut e, 0, camera);
        let value = e.mem.alloc(4);
        e.call(0x00a4_2cd0, &args![this, value]);
        assert_eq!(e.mem.f32(value), 1.0);
    }

    #[test]
    fn interp_target_is_correct_type_walks_the_rtti_chain() {
        let mut e = engine();
        e.map(0x011f_4000, 0x1000);
        let base = e.mem.alloc(8);
        e.mem.set_u32(base + 4, NI_CAMERA_RTTI);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(target, VTABLE_TEST);
        e.mem.set_u32(target + 0x10, base);
        assert!(e.call(0x00a4_2d90, &args![0u32, target]).bool());
        e.mem.set_u32(base + 4, 0);
        assert!(!e.call(0x00a4_2d90, &args![0u32, target]).bool());
        assert!(!e.call(0x00a4_2d90, &args![0u32, 0u32]).bool());
    }

    #[test]
    fn update_sets_the_frustum_from_the_interpolated_value() {
        let mut e = engine();
        let camera = e.mem.alloc(0x100);
        for i in 0..7 {
            e.mem.set_u32(camera + CAMERA_FRUSTUM + 4 * i, 0x100 + i);
        }
        let interp = interpolator(&mut e, 0, 2.0, 1);
        let this = controller(&mut e, interp, camera);
        let data = e.mem.alloc(8);
        e.mem.set_f32(data, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_2dc0, &args![this, data]);
        assert_eq!(
            calls(&e, DONT_DO_UPDATE),
            vec![vec![this, 1.0f32.to_bits()]]
        );
        let update = calls(&e, slot(SLOT_UPDATE_FLOAT));
        assert_eq!(update[0][..3], [interp, 0, camera]);
        let set = calls(&e, SET_VIEW_FRUSTUM);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], camera);
    }

    #[test]
    fn update_writes_the_four_frustum_planes() {
        let mut e = engine();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(SET_VIEW_FRUSTUM, move |e, a| {
            let words: Vec<u32> = (0..7).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push(words);
            Ret::default()
        });
        let camera = e.mem.alloc(0x100);
        e.mem.set_u32(camera + CAMERA_FRUSTUM + 0x10, 0x4444);
        let interp = interpolator(&mut e, 0, 2.0, 1);
        let this = controller(&mut e, interp, camera);
        // Forced: the scaled time becomes 7.0 and no DontDoUpdate runs.
        e.mem.set_u8(this + FLAGS, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x00a4_2dc0, &args![this, 0u32]);
        assert_eq!(e.mem.f32(this + SCALED_TIME), 7.0);
        assert!(calls(&e, DONT_DO_UPDATE).is_empty());
        let words = seen.borrow()[0].clone();
        let floats: Vec<f32> = words[..4].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(floats, [-2.0, 2.0, 1.5, -1.5]);
        assert_eq!(words[4], 0x4444);
    }

    #[test]
    fn update_stops_when_not_due_or_not_updated() {
        let mut e = engine();
        let camera = e.mem.alloc(0x100);
        let data = e.mem.alloc(8);
        e.mem.set_f32(data, -1.0);
        let interp = interpolator(&mut e, 0, 2.0, 1);
        let this = controller(&mut e, interp, camera);
        e.call_log = Some(vec![]);
        e.call(0x00a4_2dc0, &args![this, data]);
        assert!(calls(&e, slot(SLOT_UPDATE_FLOAT)).is_empty(), "skipped");
        e.mem.set_u32(interp + 8, 1);
        e.mem.set_u32(interp + 0x10, 0);
        e.call(0x00a4_2dc0, &args![this, data]);
        assert_eq!(calls(&e, slot(SLOT_UPDATE_FLOAT)).len(), 1, "always update");
        assert!(calls(&e, SET_VIEW_FRUSTUM).is_empty(), "update failed");
        let bare = controller(&mut e, 0, camera);
        e.call(0x00a4_2dc0, &args![bare, data]);
    }

    #[test]
    fn create_clone_and_factory_build_the_controller() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FLOAT_INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        let object = e.call(0x00a4_2ee0, &args![]).u32();
        assert_eq!(e.mem.u32(object), VTABLE);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_2e90, &args![0x10u32, 0x20u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(calls(&e, COPY_MEMBERS), vec![vec![0x10, clone, 0x20]]);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_2ee0, &args![]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_2e90, &args![0x10u32, 0x20u32]).u32(), 0);
        assert_eq!(calls(&e, COPY_MEMBERS), vec![vec![0x10, 0, 0x20]]);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = engine();
        e.register(PARENT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let (strings, stored) = viewer_strings(&mut e);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(BS_FRUSTUM_FOV_CONTROLLER_RTTI, 0x0109_7000);
        e.call_log = Some(vec![]);
        e.call(0x00a4_2f10, &args![0x500u32, strings]);
        assert_eq!(*stored.borrow(), vec![0x0109_7000]);
        assert_eq!(
            calls(&e, PARENT_GET_VIEWER_STRINGS),
            vec![vec![0x500, strings.addr()]]
        );
    }
}
