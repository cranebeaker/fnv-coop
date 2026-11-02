//! `NiAnimation/nifloatcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a57bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a6d830`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f4230`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_float_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
    const CLASS_NAME_POINTER: u32 = 0x011f_4230;
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

/// `NiFloatController` (Xbox PDB, 0x3C bytes; the PC code uses the same
/// offset): `m_spFloatData` +0x38 (`NiFloatData`: `m_uiNumKeys` +0x08,
/// `m_pkKeys` +0x0C, `m_eType` +0x10, `m_ucKeySize` +0x14).
const FLOAT_DATA: u32 = 0x38;
/// The class's vtable (RTTI `.?AVNiFloatController@@`).
const NI_FLOAT_CONTROLLER_VTABLE: u32 = 0x0109_857c;
/// `NiTimeController` functions this class calls (`thiscall`, Xbox PDB
/// names by the engine map): `IsEqual` (`00a6cc80`), the member copy
/// (`00a6cfe0`: destination, cloning process; unnamed in the map),
/// `ProcessClone` (`00a6d0c0`), `RegisterStreamables` (`00a6d0f0`),
/// `LinkObject` (`00a6d500`), `LoadBinary` (`00a6d5e0`) and `SaveBinary`
/// (`00a6d730`); the base destructor `00a6d290` the destructor ends in.
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a6_d0f0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const BASE_DESTRUCT: u32 = 0x00a6_d290;
/// `00a645b0` (`thiscall` on the stream, no stack word): reads one link ID
/// (4 bytes) and appends it to the stream's link-ID list.
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`; `thiscall` on the stream).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Virtual slots (Xbox PDB names): `NiRefObject::DeleteThis` +0x04,
/// `NiObject::CreateClone` +0x48, `RegisterStreamables` +0x54, `IsEqual`
/// +0x5C, `ProcessClone` +0x68, and `NiStream::SaveLinkID` +0x30.
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_CREATE_CLONE: u32 = 0x48;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_PROCESS_CLONE: u32 = 0x68;
const SLOT_SAVE_LINK_ID: u32 = 0x30;

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

/// The inline `NiPointer::operator=` for the float data of `controller`:
/// when `new` differs, the old one (if set) is released, `new` stored and
/// (if set) counted.
fn assign_float_data(e: &mut Engine, controller: u32, new: u32) {
    let slot = controller.wrapping_add(FLOAT_DATA);
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

// Translated from 00a57970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::LoadBinary` (Xbox PDB): `NiTimeController::LoadBinary`,
/// then the float data's link ID (`00a645b0`).
pub fn ni_float_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00a57990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiFloatController` destructor (unnamed in the engine map; by the
/// vtable it stores): the vtable, the float data released and cleared, the
/// (now null) pointer released again when set, then the base destructor
/// (`00a6d290`, a jump).
pub fn fn_00a57990(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, NI_FLOAT_CONTROLLER_VTABLE);
    let data = e.mem.u32(object.wrapping_add(FLOAT_DATA));
    if data != 0 {
        release(e, data);
        e.mem.set_u32(object.wrapping_add(FLOAT_DATA), 0);
    }
    // The `NiPointer` member's own destructor: the pointer is null by now.
    let data = e.mem.u32(object.wrapping_add(FLOAT_DATA));
    if data != 0 {
        release(e, data);
    }
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00a579f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::GetAnim` (Xbox PDB; `RET 0xC`: addresses of the key
/// count, key type and key size): the float data's keys, with its count,
/// type and key size (a byte) written out; without float data all three
/// are cleared and 0 is returned.
pub fn ni_float_controller_get_anim(
    e: &mut Engine,
    this: Ptr,
    num_keys: Ptr,
    key_type: Ptr,
    key_size: Ptr,
) -> u32 {
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    if data == 0 {
        e.mem.set_u32(num_keys.addr(), 0);
        e.mem.set_u32(key_type.addr(), 0);
        e.mem.set_u8(key_size.addr(), 0);
        return 0;
    }
    let count = e.mem.u32(data.wrapping_add(8));
    e.mem.set_u32(num_keys.addr(), count);
    let kind = e.mem.u32(data.wrapping_add(0x10));
    e.mem.set_u32(key_type.addr(), kind);
    let size = e.mem.u8(data.wrapping_add(0x14));
    e.mem.set_u8(key_size.addr(), size);
    e.mem.u32(data.wrapping_add(0xc))
}

// Translated from 00a57a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::ProcessClone` (Xbox PDB): `NiTimeController::
/// ProcessClone`, then the float data's `ProcessClone` (slot +0x68) when
/// there is float data.
pub fn ni_float_controller_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, cloning]);
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    if data != 0 {
        e.vcall(data, SLOT_PROCESS_CLONE, &args![cloning]);
    }
}

// Translated from 00a57a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::LinkObject` (Xbox PDB): `NiTimeController::
/// LinkObject`, then the next linked object (`NiStream::
/// GetObjectFromLinkID`) becomes the float data (counted, the old one
/// released, when they differ).
pub fn ni_float_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let data = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_float_data(e, this.addr(), data);
}

// Translated from 00a57ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::RegisterStreamables` (Xbox PDB): false when
/// `NiTimeController::RegisterStreamables` answers false; otherwise the
/// float data's `RegisterStreamables` (slot +0x54, with no null check) and
/// true.
pub fn ni_float_controller_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(TIME_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    e.vcall(data, SLOT_REGISTER_STREAMABLES, &args![stream]);
    true
}

// Translated from 00a57b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::SaveBinary` (Xbox PDB): `NiTimeController::
/// SaveBinary`, then the stream's `SaveLinkID` (slot +0x30) of the float
/// data.
pub fn ni_float_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![data]);
}

// Translated from 00a57b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::IsEqual` (Xbox PDB): `NiTimeController::IsEqual`,
/// then the float data's `IsEqual` (slot +0x5C, with no null check) on the
/// other's float data.
pub fn ni_float_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    let other_data = e.mem.u32(other.addr().wrapping_add(FLOAT_DATA));
    e.vcall(data, SLOT_IS_EQUAL, &args![other_data]).bool()
}

// Translated from 00a57b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: destination, cloning
/// process): `NiFloatController`'s member copy. The `NiTimeController` copy
/// (`00a6cfe0`), then, with float data, its clone (`CreateClone`, slot
/// +0x48) becomes the destination's float data (counted, the old one
/// released, when they differ).
pub fn fn_00a57b50(e: &mut Engine, this: Ptr, destination: Ptr, cloning: Ptr) {
    e.call(
        TIME_CONTROLLER_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let data = e.mem.u32(this.addr().wrapping_add(FLOAT_DATA));
    if data == 0 {
        return;
    }
    let clone = e.vcall(data, SLOT_CREATE_CLONE, &args![cloning]).u32();
    assign_float_data(e, destination.addr(), clone);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a57bb0, ni_float_controller_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a5_7970, ni_float_controller_load_binary(Ptr, Ptr)),
        entry!(0x00a5_7990, fn_00a57990(Ptr)),
        entry!(
            0x00a5_79f0,
            ni_float_controller_get_anim(Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x00a5_7a40, ni_float_controller_process_clone(Ptr, Ptr)),
        entry!(0x00a5_7a70, ni_float_controller_link_object(Ptr, Ptr)),
        entry!(
            0x00a5_7ad0,
            ni_float_controller_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a5_7b00, ni_float_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a5_7b20, ni_float_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a5_7b50, fn_00a57b50(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_float_controller_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a6_d830;
        const NAME_POINTER: u32 = 0x011f_4230;
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
            ni_float_controller_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
mod n003_tests {
    use super::*;

    const DELETE_THIS: u32 = 0x0380_0000;
    const CREATE_CLONE: u32 = 0x0380_0010;
    const REGISTER: u32 = 0x0380_0020;
    const IS_EQUAL: u32 = 0x0380_0030;
    const PROCESS_CLONE: u32 = 0x0380_0040;
    const SAVE_LINK_ID: u32 = 0x0380_0050;

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
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        for address in [DELETE_THIS, REGISTER, PROCESS_CLONE, SAVE_LINK_ID] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem.set_u32(vtable + SLOT_CREATE_CLONE, CREATE_CLONE);
        e.mem.set_u32(vtable + SLOT_REGISTER_STREAMABLES, REGISTER);
        e.mem.set_u32(vtable + SLOT_IS_EQUAL, IS_EQUAL);
        e.mem.set_u32(vtable + SLOT_PROCESS_CLONE, PROCESS_CLONE);
        e.mem.set_u32(vtable + SLOT_SAVE_LINK_ID, SAVE_LINK_ID);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn load_binary_reads_the_data_link() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        e.call(0x00a5_7970, &args![0x3000u32, 0x8000u32]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_LOAD_BINARY),
            vec![vec![0x3000, 0x8000]]
        );
        assert_eq!(calls(&e, STREAM_READ_LINK_ID), vec![vec![0x8000]]);
    }

    #[test]
    fn destructor_releases_the_data() {
        let mut e = engine();
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        let data = object(&mut e, 1);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.call(0x00a5_7990, &args![this]);
        assert_eq!(e.mem.u32(this), NI_FLOAT_CONTROLLER_VTABLE);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![data]]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 1);
        assert_eq!(e.mem.u32(this + FLOAT_DATA), 0);
        assert_eq!(calls(&e, BASE_DESTRUCT), vec![vec![this]]);
        e.call(0x00a5_7990, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn get_anim_reports_the_data_or_clears_the_outputs() {
        let mut e = engine();
        let this = e.mem.alloc(0x3c);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + 8, 4);
        e.mem.set_u32(data + 0xc, 0x7000);
        e.mem.set_u32(data + 0x10, 2);
        e.mem.set_u8(data + 0x14, 0x14);
        e.mem.set_u32(this + FLOAT_DATA, data);
        let outputs = e.mem.alloc(12);
        e.mem.write(outputs, &[0xaa; 12]);
        let words = args![this, outputs, outputs + 4, outputs + 8];
        assert_eq!(e.call(0x00a5_79f0, &words).u32(), 0x7000);
        assert_eq!(e.mem.u32(outputs), 4);
        assert_eq!(e.mem.u32(outputs + 4), 2);
        assert_eq!(e.mem.bytes(outputs + 8, 2), vec![0x14, 0xaa]);
        e.mem.set_u32(this + FLOAT_DATA, 0);
        assert_eq!(e.call(0x00a5_79f0, &words).u32(), 0);
        assert_eq!(
            e.mem.bytes(outputs, 10),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0xaa]
        );
    }

    #[test]
    fn process_clone_passes_on_to_the_data() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_PROCESS_CLONE, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        e.call(0x00a5_7a40, &args![this, 0x9000u32]);
        assert!(calls(&e, PROCESS_CLONE).is_empty());
        let data = object(&mut e, 1);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.call(0x00a5_7a40, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, PROCESS_CLONE), vec![vec![data, 0x9000]]);
        assert_eq!(calls(&e, TIME_CONTROLLER_PROCESS_CLONE).len(), 2);
    }

    #[test]
    fn link_object_assigns_the_linked_data() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        let data = object(&mut e, 0);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| data.into_ret());
        let this = e.mem.alloc(0x3c);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + FLOAT_DATA, old);
        e.call(0x00a5_7a70, &args![this, 0x8000u32]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_LINK_OBJECT),
            vec![vec![this, 0x8000]]
        );
        assert_eq!(
            calls(&e, STREAM_GET_OBJECT_FROM_LINK_ID),
            vec![vec![0x8000]]
        );
        assert_eq!(e.mem.u32(this + FLOAT_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        e.call(0x00a5_7a70, &args![this, 0x8000u32]);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn register_streamables_registers_the_data_after_the_base() {
        let mut e = engine();
        let mut answers = vec![0u32, 1];
        e.register_double(TIME_CONTROLLER_REGISTER_STREAMABLES, move |_, _| {
            answers.remove(0).into_ret()
        });
        let this = e.mem.alloc(0x3c);
        let data = object(&mut e, 1);
        e.mem.set_u32(this + FLOAT_DATA, data);
        assert!(!e.call(0x00a5_7ad0, &args![this, 0x8000u32]).bool());
        assert!(calls(&e, REGISTER).is_empty());
        assert!(e.call(0x00a5_7ad0, &args![this, 0x8000u32]).bool());
        assert_eq!(calls(&e, REGISTER), vec![vec![data, 0x8000]]);
    }

    #[test]
    fn save_binary_links_the_data() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        let stream = object(&mut e, 1);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + FLOAT_DATA, 0x6000);
        e.call(0x00a5_7b00, &args![this, stream]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls(&e, SAVE_LINK_ID), vec![vec![stream, 0x6000]]);
    }

    #[test]
    fn is_equal_asks_the_data() {
        let mut e = engine();
        let mut bases = vec![0u32, 1, 1];
        e.register_double(TIME_CONTROLLER_IS_EQUAL, move |_, _| {
            bases.remove(0).into_ret()
        });
        let mut answers = vec![0x100u32, 1];
        e.register_double(IS_EQUAL, move |_, _| answers.remove(0).into_ret());
        let this = e.mem.alloc(0x3c);
        let other = e.mem.alloc(0x3c);
        let data = object(&mut e, 1);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.mem.set_u32(other + FLOAT_DATA, 0x6100);
        assert!(!e.call(0x00a5_7b20, &args![this, other]).bool());
        // `SETNZ` on `AL`: 0x100 has a zero low byte.
        assert!(!e.call(0x00a5_7b20, &args![this, other]).bool());
        assert!(e.call(0x00a5_7b20, &args![this, other]).bool());
        assert_eq!(calls(&e, IS_EQUAL), vec![vec![data, 0x6100]; 2]);
    }

    #[test]
    fn copy_members_clones_the_data_into_the_destination() {
        let mut e = engine();
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let clone = object(&mut e, 0);
        e.register_double(CREATE_CLONE, move |_, _| clone.into_ret());
        let this = e.mem.alloc(0x3c);
        let destination = e.mem.alloc(0x3c);
        e.call(0x00a5_7b50, &args![this, destination, 0x9000u32]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this, destination, 0x9000]]
        );
        assert!(calls(&e, CREATE_CLONE).is_empty());
        let data = object(&mut e, 1);
        e.mem.set_u32(this + FLOAT_DATA, data);
        e.call(0x00a5_7b50, &args![this, destination, 0x9000u32]);
        assert_eq!(calls(&e, CREATE_CLONE), vec![vec![data, 0x9000]]);
        assert_eq!(e.mem.u32(destination + FLOAT_DATA), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
    }
}
