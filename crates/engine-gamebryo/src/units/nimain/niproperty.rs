//! `NiMain/niproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a5d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiProperty::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a5ca80`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f4420`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_ca80;
    const CLASS_NAME_POINTER: u32 = 0x011f_4420;
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

/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// `NiStream::m_pkIstr` (Xbox PDB) +0x24C: the `NiBinaryStream*` read from.
const STREAM_INPUT: u32 = 0x24c;
/// `NiStream::m_usNiPropertyFlags` (Xbox PDB) +0x288.
const STREAM_PROPERTY_FLAGS: u32 = 0x288;
/// `NiBinaryStream::m_uiAbsoluteCurrentPos` (Xbox PDB) +4.
const BINARY_STREAM_POSITION: u32 = 0x4;
/// `NiBinaryStream::m_pfnRead` (Xbox PDB) +8: a `cdecl` function pointer
/// `(stream, buffer, size, &component size, component count)` returning the
/// number of bytes read.
const BINARY_STREAM_READ: u32 = 0x8;
/// The first file version that no longer stores the property flags here.
const FLAGS_IN_SUBCLASS_VERSION: u32 = 0x0a00_0102;
/// `NiObjectNET::LoadBinary` (`00a5c080`, `RET 4`).
const NI_OBJECT_NET_LOAD_BINARY: u32 = 0x00a5_c080;
/// `NiObjectNET::m_spControllers` (Xbox PDB) +0xC: the first controller.
const OBJECT_CONTROLLERS: u32 = 0xc;
/// `NiTimeController::m_spNext` (Xbox PDB) +0x30.
const CONTROLLER_NEXT: u32 = 0x30;
/// The controllers' vtable slot +0x94 (`NiTimeController::Update` in the
/// Xbox PDB's vtable order).
const CONTROLLER_UPDATE_SLOT: u32 = 0x94;

// Translated from 00a5d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiProperty::Update` (Xbox PDB): calls vtable slot +0x94 (`Update`) of
/// every controller in the property's controller chain (`+0xC`, next at
/// +0x30) with `time`.
pub fn ni_property_update(e: &mut Engine, this: Ptr, time: u32) {
    let mut controller = e.mem.u32(this.addr().wrapping_add(OBJECT_CONTROLLERS));
    while controller != 0 {
        e.vcall(controller, CONTROLLER_UPDATE_SLOT, &args![time]);
        controller = e.mem.u32(controller.wrapping_add(CONTROLLER_NEXT));
    }
}

// Translated from 00a5d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiProperty::LoadBinary` (Xbox PDB): `NiObjectNET::LoadBinary`, then, for
/// files before version `0x0A000102`, reads the 2-byte property flags through
/// the input stream's `m_pfnRead` (`(istr, buffer, 2, &component size = 2,
/// 1)`), advances the stream's position by the bytes read and keeps the
/// flags in the stream's `m_usNiPropertyFlags` (+0x288). The buffer is the
/// stack word of the stream argument (its low half is kept), the component
/// size a local.
pub fn ni_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_NET_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) >= FLAGS_IN_SUBCLASS_VERSION {
        return;
    }
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    let flags = e.with_stack(8, |e, frame| {
        let buffer = frame.addr();
        let component_size = buffer + 4;
        e.mem.set_u32(buffer, stream.addr());
        e.mem.set_u32(component_size, 2);
        let got = e
            .call(read, &args![input, buffer, 2u32, component_size, 1u32])
            .u32();
        let position = input.wrapping_add(BINARY_STREAM_POSITION);
        let advanced = e.mem.u32(position).wrapping_add(got);
        e.mem.set_u32(position, advanced);
        e.mem.u16(buffer)
    });
    e.mem
        .set_u16(stream.addr().wrapping_add(STREAM_PROPERTY_FLAGS), flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5d850, ni_property_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a5d7c0, ni_property_update(Ptr, u32)),
        entry!(0x00a5d7f0, ni_property_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_property_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_ca80;
        const NAME_POINTER: u32 = 0x011f_4420;
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
            ni_property_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
mod b014_tests {
    use super::*;

    /// A stand-in `m_pfnRead` (at a made-up address): writes `0xBEEF` into
    /// the buffer and answers 2 bytes read.
    const FAKE_READ: u32 = 0x7100_0000;

    #[test]
    fn ni_property_update_runs_every_controller_in_the_chain() {
        let mut e = Engine::new();
        e.put_vtable(0x7000_0000, &[0; 0x98 / 4]);
        e.mem.set_u32(0x7000_0000 + 0x94, 0x7200_0000);
        e.register(0x7200_0000, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        let first = e.mem.alloc(0x34);
        let second = e.mem.alloc(0x34);
        e.mem.set_u32(first, 0x7000_0000);
        e.mem.set_u32(second, 0x7000_0000);
        e.mem.set_u32(first + 0x30, second);
        e.mem.set_u32(this.addr() + 0xc, first);
        e.call_log = Some(vec![]);
        e.call(0x00a5d7c0, &args![this, 0x4120_0000u32]);
        let calls: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == 0x7200_0000)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            calls,
            vec![vec![first, 0x4120_0000], vec![second, 0x4120_0000]]
        );
        // No controllers: nothing runs.
        let empty: Ptr = Ptr::new(e.mem.alloc(0x18));
        e.call_log = Some(vec![]);
        e.call(0x00a5d7c0, &args![empty, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn ni_property_load_binary_reads_the_flags_only_for_old_files() {
        for (version, read) in [(0x0a00_0101u32, true), (0x0a00_0102, false)] {
            let mut e = Engine::new();
            e.register(0x00a5_c080, |_, _| Ret::default());
            e.register(FAKE_READ, |e, a| {
                assert_eq!(e.mem.u32(a[3]), 2);
                assert_eq!(&a[2..], [2, a[3], 1]);
                e.mem.set_u16(a[1], 0xbeef);
                Ret {
                    eax: 2,
                    ..Ret::default()
                }
            });
            let this: Ptr = Ptr::new(e.mem.alloc(0x18));
            let stream = e.mem.alloc(0x5c4);
            let input = e.mem.alloc(0x10);
            e.mem.set_u32(input + 4, 10);
            e.mem.set_u32(input + 8, FAKE_READ);
            e.mem.set_u32(stream + 0xd8, version);
            e.mem.set_u32(stream + 0x24c, input);
            e.call_log = Some(vec![]);
            e.call(0x00a5d7f0, &args![this, stream]);
            let log = e.call_log.take().unwrap();
            let parent = log.iter().find(|c| c.0 == 0x00a5_c080).unwrap();
            assert_eq!(parent.1, [this.addr(), stream]);
            if read {
                assert_eq!(e.mem.u16(stream + 0x288), 0xbeef);
                assert_eq!(e.mem.u32(input + 4), 12);
                let call = log.iter().find(|c| c.0 == FAKE_READ).unwrap();
                assert_eq!(call.1[0], input);
            } else {
                assert_eq!(e.mem.u16(stream + 0x288), 0);
                assert!(log.iter().all(|c| c.0 != FAKE_READ));
            }
        }
    }
}
