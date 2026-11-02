//! `NiAnimation/niextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_4218;

// Translated from 00a57290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_EXTRA_DATA_CONTROLLER_RTTI)
}

/// The base class constructor (`00a38e00`) and destructor (`00a6d290`) the controller's
/// own run; `thiscall`, no stack word.
const BASE_CONSTRUCTOR: u32 = 0x00a3_8e00;
const BASE_DESTRUCTOR: u32 = 0x00a6_d290;
/// The base `ProcessClone` (`00a38f50`; `thiscall`: cloning process).
const BASE_PROCESS_CLONE: u32 = 0x00a3_8f50;
/// The base `PostLinkObject` (`00a89af0`; `thiscall`: stream).
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`; `thiscall` on the target: address of the
/// name).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// The cloning process's map lookup (`00a53c30`; `thiscall` on the map: key, address of
/// the result word; answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPointer` assignment (`0066b0d0`; `thiscall` on the pointer: the new object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The `NiExtraDataController` virtual table (`0109aa14`).
const EXTRA_DATA_CONTROLLER_VTABLE: u32 = 0x0109_aa14;
/// The pooled empty string's handle (`0109b220`), never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::DeleteThis`, virtual slot 4.
const SLOT_DELETE_THIS: u32 = 0x04;

layout! {
    /// `NiExtraDataController` (Xbox PDB), 0x40 bytes.
    pub struct NiExtraDataController: 0x40 {
        /// `m_pkTarget` (Xbox PDB, `NiTimeController`): `NiObjectNET*`.
        0x2C m_pkTarget: u32,
        /// `m_spInterpolator` (Xbox PDB, `NiSingleInterpController`).
        0x34 m_spInterpolator: u32,
        /// `m_kExtraDataName` (Xbox PDB): `NiFixedString` handle.
        0x38 m_kExtraDataName: u32,
        /// `m_spExtraData` (Xbox PDB): `NiExtraData*`.
        0x3C m_spExtraData: u32,
    }
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at `+4` and, when it
/// reaches zero, the object's `DeleteThis` (virtual slot 4).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

// Translated from 00a572a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController`'s constructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): the base constructor (`00a38e00`), the class's virtual table, the extra data
/// name set to the empty string's handle (`0109b220`, read, not counted) and no extra data.
/// Answers `this`.
pub fn ni_extra_data_controller_constructor(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
) -> Ptr<NiExtraDataController> {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), EXTRA_DATA_CONTROLLER_VTABLE);
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.set(this, NiExtraDataController::m_kExtraDataName, empty);
    e.set(this, NiExtraDataController::m_spExtraData, 0);
    this
}

// Translated from 00a57240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): looks the extra data up on the
/// target by name (`NiObjectNET::GetExtraData`, `00a5bdd0`) and assigns it to the
/// controller's extra data pointer (the old one released, the new one counted, when they
/// differ).
pub fn fn_00a57240(e: &mut Engine, this: Ptr<NiExtraDataController>) {
    let target = e.get(this, NiExtraDataController::m_pkTarget);
    let name = this.addr() + 0x38;
    let found = e.call(GET_EXTRA_DATA, &args![target, name]).u32();
    let old = e.get(this, NiExtraDataController::m_spExtraData);
    if old == found {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.set(this, NiExtraDataController::m_spExtraData, found);
    if found != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![found + 4]);
    }
}

// Translated from 00a57400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::PostLinkObject` (Xbox PDB; `thiscall`, `RET 4`: stream): the
/// base `PostLinkObject` (`00a89af0`), then, with a target, the extra data looked up again
/// ([`fn_00a57240`]).
pub fn ni_extra_data_controller_post_link_object(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    if e.get(this, NiExtraDataController::m_pkTarget) != 0 {
        fn_00a57240(e, this);
    }
}

// Translated from 00a57150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::ProcessClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process).
/// The base `ProcessClone` (`00a38f50`), then this controller's clone looked up in the
/// cloning process's map (`00a53c30`; the result goes into the argument's own word, which
/// keeps the cloning process when nothing is found). When this controller and the clone
/// both have a target, the clone of the extra data is looked up the same way and assigned
/// to the clone's extra data pointer (`0066b0d0`).
pub fn ni_extra_data_controller_process_clone(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    cloning: Ptr,
) {
    e.call(BASE_PROCESS_CLONE, &args![this, cloning]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), cloning.addr());
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        if e.get(this, NiExtraDataController::m_pkTarget) == 0 {
            return;
        }
        let clone = e.mem.u32(slot.addr());
        // NiTimeController::m_pkTarget (Xbox PDB) +0x2C of the clone.
        if e.mem.u32(clone.wrapping_add(0x2c)) == 0 {
            return;
        }
        let extra = e.get(this, NiExtraDataController::m_spExtraData);
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, extra, slot]);
        let found = e.mem.u32(slot.addr());
        e.call(POINTER_ASSIGN, &args![clone.wrapping_add(0x3c), found]);
    });
}

// Translated from 00a572d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController`'s destructor (unnamed in the engine map; `thiscall`): stores the
/// class's virtual table, releases the extra data, the extra data name (unless it is the
/// empty string) and the interpolator (`+0x34`), then the base destructor (`00a6d290`, a
/// jump).
pub fn ni_extra_data_controller_destructor(e: &mut Engine, this: Ptr<NiExtraDataController>) {
    e.mem.set_u32(this.addr(), EXTRA_DATA_CONTROLLER_VTABLE);
    let extra = e.get(this, NiExtraDataController::m_spExtraData);
    if extra != 0 {
        release(e, extra);
    }
    let name = e.get(this, NiExtraDataController::m_kExtraDataName);
    if name != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    let interpolator = e.get(this, NiExtraDataController::m_spInterpolator);
    if interpolator != 0 {
        release(e, interpolator);
    }
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// `NiSingleInterpController::RegisterStreamables` (`00a38fe0`),
/// `::IsEqual` (`00a39030`) and `::GetViewerStrings` (`00a390a0`), and
/// `NiTimeController::SetTarget` (`00a6d450`), all `thiscall`.
const BASE_REGISTER_STREAMABLES: u32 = 0x00a3_8fe0;
const BASE_IS_EQUAL: u32 = 0x00a3_9030;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a3_90a0;
const BASE_SET_TARGET: u32 = 0x00a6_d450;
/// `NiObjectNET::ms_RTTI` (`011f4304`).
const NI_OBJECT_NET_RTTI: u32 = 0x011f_4304;
/// `00a62660` (`thiscall` on an extra data object, no stack word; the
/// engine map's name for it is a folded body's): the address of the extra
/// data's name handle.
const EXTRA_DATA_NAME_ADDRESS: u32 = 0x00a6_2660;
/// `NiObjectNET`'s extra-data array (Xbox PDB `m_ppkExtra` +0x10, its 16-bit
/// count `m_usExtraDataSize` +0x14; PC offsets as the code uses them).
const OBJECT_EXTRA_DATA: u32 = 0x10;
const OBJECT_EXTRA_DATA_COUNT: u32 = 0x14;
/// Virtual slots (Xbox PDB names): `NiObject::GetRTTI` +0x08,
/// `NiObject::IsEqual` +0x5C and `NiStream::RegisterFixedString` +0x20.
const SLOT_GET_RTTI: u32 = 0x08;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_REGISTER_FIXED_STRING: u32 = 0x20;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name) and
/// `NiGetViewerString_ov11` (engine map; `00a7d5e0`, `cdecl`: label, fixed
/// string); the viewer-string array's grow (`0096ad30`) and store
/// (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FIXED: u32 = 0x00a7_d5e0;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"Extra Data Name"`.
const LABEL_EXTRA_DATA_NAME: u32 = 0x0109_ab00;

/// Appends `text` to the viewer-string array at `strings` (grown by its
/// step when full), through a stack word holding it.
fn add_viewer_string(e: &mut Engine, strings: u32, text: u32) {
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

// Translated from 00a57100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::RegisterStreamables` (Xbox PDB): false when the
/// base `RegisterStreamables` answers false; otherwise the stream's
/// `RegisterFixedString` (slot +0x20) of the extra data name, and true.
pub fn ni_extra_data_controller_register_streamables(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    stream: Ptr,
) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let name = this.addr().wrapping_add(0x38);
    e.vcall(stream.addr(), SLOT_REGISTER_FIXED_STRING, &args![name]);
    true
}

// Translated from 00a571e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::IsEqual` (Xbox PDB): the base `IsEqual`, either
/// both without extra data or both with extra data the first's `IsEqual`
/// (slot +0x5C) finds equal, and the same extra data name handle.
pub fn ni_extra_data_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    other: Ptr,
) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let other_controller = Ptr::<NiExtraDataController>::new(other.addr());
    let extra = e.get(this, NiExtraDataController::m_spExtraData);
    let other_extra = e.get(other_controller, NiExtraDataController::m_spExtraData);
    if extra != 0 {
        if other_extra == 0 {
            return false;
        }
    } else if other_extra != 0 {
        return false;
    }
    if extra != 0 {
        let other_extra = e.get(other_controller, NiExtraDataController::m_spExtraData);
        // Never null here (checked above).
        if other_extra != 0 && !e.vcall(extra, SLOT_IS_EQUAL, &args![other_extra]).bool() {
            return false;
        }
    }
    e.get(other_controller, NiExtraDataController::m_kExtraDataName)
        == e.get(this, NiExtraDataController::m_kExtraDataName)
}

// Translated from 00a57330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::SetTarget` (Xbox PDB): releases and clears the
/// extra data, runs `NiTimeController::SetTarget`, then, with a target,
/// looks the extra data up again ([`fn_00a57240`]).
pub fn ni_extra_data_controller_set_target(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    target: Ptr,
) {
    let extra = e.get(this, NiExtraDataController::m_spExtraData);
    if extra != 0 {
        release(e, extra);
        e.set(this, NiExtraDataController::m_spExtraData, 0);
    }
    e.call(BASE_SET_TARGET, &args![this, target]);
    if e.get(this, NiExtraDataController::m_pkTarget) != 0 {
        fn_00a57240(e, this);
    }
}

// Translated from 00a57380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::InterpTargetIsCorrectType` (Xbox PDB): false for
/// a null target, a target that is not an `NiObjectNET` (its `GetRTTI` and
/// the base records walked for `011f4304`) or a controller whose extra data
/// name is the empty string; otherwise whether one of the target's extra
/// data (the array at +0x10, 16-bit count at +0x14) has this name (the
/// handle at the address `00a62660` answers).
pub fn ni_extra_data_controller_interp_target_is_correct_type(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    target: Ptr,
) -> bool {
    let target = target.addr();
    if target == 0 {
        return false;
    }
    let mut rtti = e.vcall(target, SLOT_GET_RTTI, &args![]).u32();
    loop {
        if rtti == 0 {
            return false;
        }
        if rtti == NI_OBJECT_NET_RTTI {
            break;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    if e.get(this, NiExtraDataController::m_kExtraDataName) == e.global::<u32>(EMPTY_FIXED_STRING) {
        return false;
    }
    let mut index = 0u32;
    if e.mem.u16(target.wrapping_add(OBJECT_EXTRA_DATA_COUNT)) == 0 {
        return false;
    }
    loop {
        let array = e.mem.u32(target.wrapping_add(OBJECT_EXTRA_DATA));
        let extra = e
            .mem
            .u32(array.wrapping_add(u32::from(index as u16).wrapping_mul(4)));
        let name = e.call(EXTRA_DATA_NAME_ADDRESS, &args![extra]).u32();
        if e.mem.u32(name) == e.get(this, NiExtraDataController::m_kExtraDataName) {
            return true;
        }
        index = index.wrapping_add(1);
        if index >= u32::from(e.mem.u16(target.wrapping_add(OBJECT_EXTRA_DATA_COUNT))) {
            return false;
        }
    }
}

// Translated from 00a57420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::GetViewerStrings` (Xbox PDB): the base
/// `GetViewerStrings`, then the class name (the `NiRTTI` record's first word
/// through `NiGetViewerString`) and the extra data name
/// (`NiGetViewerString_ov11("Extra Data Name", name)`).
pub fn ni_extra_data_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiExtraDataController>,
    strings: Ptr,
) {
    let strings = strings.addr();
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_EXTRA_DATA_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    let extra_name = e.get(this, NiExtraDataController::m_kExtraDataName);
    let text = e
        .call(
            NI_GET_VIEWER_STRING_FIXED,
            &args![LABEL_EXTRA_DATA_NAME, extra_name],
        )
        .u32();
    add_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a57290, ni_extra_data_controller_get_rtti() -> Ptr),
        entry!(
            0x00a5_72a0,
            ni_extra_data_controller_constructor(
                Ptr<NiExtraDataController>,
            ) -> Ptr<NiExtraDataController>
        ),
        entry!(0x00a5_7240, fn_00a57240(Ptr<NiExtraDataController>)),
        entry!(
            0x00a5_7400,
            ni_extra_data_controller_post_link_object(Ptr<NiExtraDataController>, Ptr)
        ),
        entry!(
            0x00a5_7150,
            ni_extra_data_controller_process_clone(Ptr<NiExtraDataController>, Ptr)
        ),
        entry!(
            0x00a5_72d0,
            ni_extra_data_controller_destructor(Ptr<NiExtraDataController>)
        ),
        entry!(
            0x00a5_7100,
            ni_extra_data_controller_register_streamables(Ptr<NiExtraDataController>, Ptr) -> bool
        ),
        entry!(
            0x00a5_71e0,
            ni_extra_data_controller_is_equal(Ptr<NiExtraDataController>, Ptr) -> bool
        ),
        entry!(
            0x00a5_7330,
            ni_extra_data_controller_set_target(Ptr<NiExtraDataController>, Ptr)
        ),
        entry!(
            0x00a5_7380,
            ni_extra_data_controller_interp_target_is_correct_type(
                Ptr<NiExtraDataController>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00a5_7420,
            ni_extra_data_controller_get_viewer_strings(Ptr<NiExtraDataController>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57290, &args![]).u32(),
            NI_EXTRA_DATA_CONTROLLER_RTTI
        );
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

    const DELETE_THIS: u32 = 0x0310_0000;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x0800u32);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    fn object(e: &mut Engine, vtable: u32, count: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn constructor_starts_with_the_empty_name() {
        let (mut e, _) = engine();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, 0x1234);
        assert_eq!(e.call(0x00a5_72a0, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), 0x0800);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert!(calls(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn extra_data_lookup_replaces_the_pointer() {
        let (mut e, vtable) = engine();
        let old = object(&mut e, vtable, 1);
        let found = object(&mut e, vtable, 0);
        e.register_double(GET_EXTRA_DATA, move |_, _| found.into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.mem.set_u32(this + 0x3c, old);
        e.call(0x00a5_7240, &args![this]);
        assert_eq!(calls(&e, GET_EXTRA_DATA), vec![vec![0x7000, this + 0x38]]);
        assert_eq!(e.mem.u32(this + 0x3c), found);
        assert_eq!(e.mem.u32(found + 4), 1);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        // The same object again: nothing is counted.
        e.call(0x00a5_7240, &args![this]);
        assert_eq!(e.mem.u32(found + 4), 1);
    }

    #[test]
    fn post_link_object_looks_up_only_with_a_target() {
        let (mut e, _) = engine();
        e.register(BASE_POST_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_EXTRA_DATA, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call(0x00a5_7400, &args![this, 0x5000u32]);
        assert!(calls(&e, GET_EXTRA_DATA).is_empty());
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.call(0x00a5_7400, &args![this, 0x5000u32]);
        assert_eq!(calls(&e, GET_EXTRA_DATA).len(), 1);
        assert_eq!(
            calls(&e, BASE_POST_LINK_OBJECT),
            vec![vec![this, 0x5000]; 2]
        );
    }

    #[test]
    fn process_clone_assigns_the_cloned_extra_data() {
        let (mut e, _) = engine();
        e.register(BASE_PROCESS_CLONE, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        let clone = e.mem.alloc(0x40);
        e.mem.set_u32(clone + 0x2c, 0x7100);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = if a[1] == 0x6600 { 0x6611 } else { clone };
            e.mem.set_u32(a[2], found);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.mem.set_u32(this + 0x3c, 0x6600);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x9900);
        e.call(0x00a5_7150, &args![this, cloning]);
        assert_eq!(calls(&e, BASE_PROCESS_CLONE), vec![vec![this, cloning]]);
        let lookups = calls(&e, CLONE_MAP_LOOKUP);
        assert_eq!((lookups[0][0], lookups[0][1]), (0x9900, this));
        assert_eq!((lookups[1][0], lookups[1][1]), (0x9900, 0x6600));
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![clone + 0x3c, 0x6611]]);
    }

    #[test]
    fn process_clone_stops_without_targets() {
        let (mut e, _) = engine();
        e.register(BASE_PROCESS_CLONE, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |_, _| Ret::default());
        let clone = e.mem.alloc(0x40);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], clone);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        let cloning = e.mem.alloc(4);
        e.call(0x00a5_7150, &args![this, cloning]);
        assert_eq!(calls(&e, CLONE_MAP_LOOKUP).len(), 1);
        // A target here but none on the clone.
        e.mem.set_u32(this + 0x2c, 0x7000);
        e.call(0x00a5_7150, &args![this, cloning]);
        assert_eq!(calls(&e, CLONE_MAP_LOOKUP).len(), 2);
        assert!(calls(&e, POINTER_ASSIGN).is_empty());
    }

    #[test]
    fn destructor_releases_the_members() {
        let (mut e, vtable) = engine();
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        let extra = object(&mut e, vtable, 1);
        let interpolator = object(&mut e, vtable, 2);
        let name = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, interpolator);
        e.mem.set_u32(this + 0x38, name + 8);
        e.mem.set_u32(this + 0x3c, extra);
        e.call(0x00a5_72d0, &args![this]);
        assert_eq!(e.mem.u32(this), EXTRA_DATA_CONTROLLER_VTABLE);
        assert_eq!(
            calls(&e, INTERLOCKED_DECREMENT),
            vec![vec![extra + 4], vec![name], vec![interpolator + 4]]
        );
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![extra]]);
        assert_eq!(calls(&e, BASE_DESTRUCTOR), vec![vec![this]]);
        // The empty name is not released.
        e.mem.set_u32(this + 0x38, 0x0800);
        e.mem.set_u32(this + 0x34, 0);
        e.mem.set_u32(this + 0x3c, 0);
        e.call(0x00a5_72d0, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 3);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    const REGISTER_FIXED_STRING: u32 = 0x0360_0000;
    const DATA_IS_EQUAL: u32 = 0x0360_0010;
    const GET_RTTI: u32 = 0x0360_0020;
    const DELETE_THIS: u32 = 0x0360_0030;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(REGISTER_FIXED_STRING, |_, _| Ret::default());
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x0800u32);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem.set_u32(vtable + SLOT_GET_RTTI, GET_RTTI);
        e.mem.set_u32(vtable + SLOT_IS_EQUAL, DATA_IS_EQUAL);
        e.mem
            .set_u32(vtable + SLOT_REGISTER_FIXED_STRING, REGISTER_FIXED_STRING);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn register_streamables_registers_the_name() {
        let mut e = engine();
        let mut answers = vec![0u32, 1];
        e.register_double(BASE_REGISTER_STREAMABLES, move |_, _| {
            answers.remove(0).into_ret()
        });
        let stream = object(&mut e, 1);
        let this = e.mem.alloc(0x40);
        assert!(!e.call(0x00a5_7100, &args![this, stream]).bool());
        assert!(calls(&e, REGISTER_FIXED_STRING).is_empty());
        assert!(e.call(0x00a5_7100, &args![this, stream]).bool());
        assert_eq!(
            calls(&e, REGISTER_FIXED_STRING),
            vec![vec![stream, this + 0x38]]
        );
    }

    #[test]
    fn is_equal_compares_base_extra_data_and_name() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let mut answers = vec![1u32, 0];
        e.register_double(DATA_IS_EQUAL, move |_, _| answers.remove(0).into_ret());
        let this = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 1);
        assert!(e.call(0x00a5_71e0, &args![this, other]).bool());
        e.mem.set_u32(other + 0x38, 0x900);
        assert!(!e.call(0x00a5_71e0, &args![this, other]).bool());
        e.mem.set_u32(this + 0x38, 0x900);
        let extra = object(&mut e, 1);
        let other_extra = object(&mut e, 1);
        e.mem.set_u32(other + 0x3c, other_extra);
        assert!(!e.call(0x00a5_71e0, &args![this, other]).bool());
        e.mem.set_u32(this + 0x3c, extra);
        e.mem.set_u32(other + 0x3c, 0);
        assert!(!e.call(0x00a5_71e0, &args![this, other]).bool());
        e.mem.set_u32(other + 0x3c, other_extra);
        assert!(e.call(0x00a5_71e0, &args![this, other]).bool());
        assert!(!e.call(0x00a5_71e0, &args![this, other]).bool());
        assert_eq!(calls(&e, DATA_IS_EQUAL), vec![vec![extra, other_extra]; 2]);
        e.mem.set_u32(this + 8, 0);
        assert!(!e.call(0x00a5_71e0, &args![this, other]).bool());
    }

    #[test]
    fn set_target_drops_the_extra_data_and_looks_it_up_again() {
        let mut e = engine();
        e.register(BASE_SET_TARGET, |e, a| {
            e.mem.set_u32(a[0] + 0x2c, a[1]);
            Ret::default()
        });
        let found = object(&mut e, 0);
        e.register_double(GET_EXTRA_DATA, move |_, _| found.into_ret());
        let this = e.mem.alloc(0x40);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x3c, old);
        e.call(0x00a5_7330, &args![this, 0x7000u32]);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(calls(&e, BASE_SET_TARGET), vec![vec![this, 0x7000]]);
        assert_eq!(calls(&e, GET_EXTRA_DATA), vec![vec![0x7000, this + 0x38]]);
        assert_eq!(e.mem.u32(this + 0x3c), found);
        // A null target: cleared, no lookup.
        e.call(0x00a5_7330, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(calls(&e, GET_EXTRA_DATA).len(), 1);
    }

    #[test]
    fn interp_target_is_correct_type_finds_the_named_extra_data() {
        let mut e = engine();
        e.map(NI_OBJECT_NET_RTTI, 8);
        let derived = e.mem.alloc(8);
        e.mem.set_u32(derived + 4, NI_OBJECT_NET_RTTI);
        e.register_double(GET_RTTI, move |e, a| e.mem.u32(a[0] + 0x30).into_ret());
        e.register(EXTRA_DATA_NAME_ADDRESS, |_, a| (a[0] + 8).into_ret());
        let target = object(&mut e, 1);
        e.mem.set_u32(target + 0x30, derived);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first + 8, 0x910);
        e.mem.set_u32(second + 8, 0x920);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 4, second);
        e.mem.set_u32(target + OBJECT_EXTRA_DATA, array);
        e.mem.set_u16(target + OBJECT_EXTRA_DATA_COUNT, 2);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x38, 0x920);
        assert!(e.call(0x00a5_7380, &args![this, target]).bool());
        assert_eq!(
            calls(&e, EXTRA_DATA_NAME_ADDRESS),
            vec![vec![first], vec![second]]
        );
        // No match, the empty name, no extra data, not an NiObjectNET, null.
        e.mem.set_u32(this + 0x38, 0x930);
        assert!(!e.call(0x00a5_7380, &args![this, target]).bool());
        e.mem.set_u32(this + 0x38, 0x0800);
        assert!(!e.call(0x00a5_7380, &args![this, target]).bool());
        e.mem.set_u32(this + 0x38, 0x920);
        e.mem.set_u16(target + OBJECT_EXTRA_DATA_COUNT, 0);
        assert!(!e.call(0x00a5_7380, &args![this, target]).bool());
        let unrelated = e.mem.alloc(8);
        e.mem.set_u32(target + 0x30, unrelated);
        assert!(!e.call(0x00a5_7380, &args![this, target]).bool());
        assert!(!e.call(0x00a5_7380, &args![this, 0u32]).bool());
        assert_eq!(calls(&e, EXTRA_DATA_NAME_ADDRESS).len(), 4);
    }

    #[test]
    fn get_viewer_strings_adds_the_name_and_the_extra_data_name() {
        let mut e = engine();
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x7100u32.into_ret());
        e.register(NI_GET_VIEWER_STRING_FIXED, |_, _| 0x7200u32.into_ret());
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7100 + 0x100 * a[1]);
            let size = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, size);
            Ret::default()
        });
        e.map(NI_EXTRA_DATA_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_EXTRA_DATA_CONTROLLER_RTTI, 0x5151);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x38, 0x940);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.call(0x00a5_7420, &args![this, strings]);
        assert_eq!(
            calls(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![LABEL_EXTRA_DATA_NAME, 0x940]]
        );
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 2);
        assert!(calls(&e, STRINGS_GROW).is_empty());
    }
}
