//! `NiMain/nivertweightsextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a89c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertWeightsExtraData::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a7b580`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f5b78`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_vert_weights_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
    const CLASS_NAME_POINTER: u32 = 0x011f_5b78;
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

/// `NiVertWeightsExtraData`'s vtable (its RTTI locator names `.?AVNiVertWeightsExtraData@@`).
const NI_VERT_WEIGHTS_EXTRA_DATA_VTABLE: u32 = 0x0109_ee9c;
/// The class's `NiRTTI` record (`011f5b78`).
const NI_VERT_WEIGHTS_EXTRA_DATA_RTTI: u32 = 0x011f_5b78;
/// The object size.
const NI_VERT_WEIGHTS_EXTRA_DATA_SIZE: u32 = 0xc;
/// `00aa13e0` (`cdecl`, the size): `NiNew`'s allocator; `00aa1460` (`cdecl`, the block and
/// its size): its release.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// `NiExtraData` (unit `niextradata.cpp`): `IsEqual` (`00a7b3d0`, `thiscall`, the other
/// object), `LoadBinary` (`00a7b4a0`, `thiscall`, the stream), and three functions the engine
/// map leaves unnamed: the constructor (`00a7b2e0`, `thiscall`), the destructor body
/// (`00a7b300`, `thiscall`) and the member copy (`00a7b330`, `thiscall`, clone and cloning
/// process).
const EXTRA_DATA_IS_EQUAL: u32 = 0x00a7_b3d0;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;

// Translated from 00a89b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertWeightsExtraData::IsEqual` (Xbox PDB): `NiExtraData::IsEqual` (`00a7b3d0`) on the
/// same words; the low byte of the answer becomes 1 when it is not 0 (`SETNZ AL`), the upper
/// bytes stay the base's.
pub fn ni_vert_weights_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    let answer = e.call(EXTRA_DATA_IS_EQUAL, &args![this, other]).u32();
    (answer & 0xffff_ff00) | u32::from(answer as u8 != 0)
}

// Translated from 00a89b20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiVertWeightsExtraData::GetRTTI` (Xbox PDB): the class's `NiRTTI` record at `011f5b78`.
pub fn ni_vert_weights_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_VERT_WEIGHTS_EXTRA_DATA_RTTI)
}

// Translated from 00a89b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertWeightsExtraData::CreateClone` (Xbox PDB): a new 0xc-byte object (`00aa13e0`, the
/// `NiExtraData` constructor `00a7b2e0`, the class's vtable), then the base's member copy
/// (`00a7b330`) from `this` into it, run with a null clone when the allocation failed.
/// Answers the clone (null on failure).
pub fn ni_vert_weights_extra_data_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning_process: Ptr,
) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_VERT_WEIGHTS_EXTRA_DATA_SIZE])
        .u32();
    if block != 0 {
        e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_VERT_WEIGHTS_EXTRA_DATA_VTABLE);
    }
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, block, cloning_process],
    );
    Ptr::new(block)
}

// Translated from 00a89b80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiVertWeightsExtraData`): a new 0xc-byte object (`00aa13e0`, the
/// `NiExtraData` constructor `00a7b2e0`, the class's vtable), or null when the allocation
/// fails.
pub fn fn_00a89b80(e: &mut Engine) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_VERT_WEIGHTS_EXTRA_DATA_SIZE])
        .u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_VERT_WEIGHTS_EXTRA_DATA_VTABLE);
    Ptr::new(block)
}

// Translated from 00a89bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertWeightsExtraData::_scalar_deleting_destructor_` (Xbox PDB): the class's vtable,
/// the `NiExtraData` destructor body (`00a7b300`), then, when bit 0 of the flags is set, the
/// 0xc-byte block released (`00aa1460`). Answers `this`.
pub fn ni_vert_weights_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem
        .set_u32(this.addr(), NI_VERT_WEIGHTS_EXTRA_DATA_VTABLE);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, NI_VERT_WEIGHTS_EXTRA_DATA_SIZE]);
    }
    this
}

// Translated from 00a89be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertWeightsExtraData::LoadBinary` (Xbox PDB): only for files older than version
/// `0x05000010` (`NiStream` +0xd8): `NiExtraData::LoadBinary` (`00a7b4a0`), then a 4-byte
/// count (into the stream argument's own word) and that many 4-byte weights, each read into
/// the same local word and dropped. The byte-count word is the other local. Newer files
/// read nothing.
pub fn ni_vert_weights_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    if e.mem.u32(stream.addr() + 0xd8) >= 0x0500_0010 {
        return;
    }
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    // Frame: the byte-count local, the weight local, the return address word, the argument.
    e.with_stack(0x10, |e, frame| {
        let size = frame.addr();
        let weight = frame.addr() + 4;
        let argument = frame.addr() + 0xc;
        e.mem.set_u32(argument, stream.addr());
        let read = |e: &mut Engine, destination: u32| {
            let input = e.mem.u32(stream.addr() + 0x24c);
            let function = e.mem.u32(input + 8);
            e.mem.set_u32(size, 4);
            let done = e
                .call(function, &args![input, destination, 4u32, size, 1u32])
                .u32();
            let position = e.mem.u32(input + 4).wrapping_add(done);
            e.mem.set_u32(input + 4, position);
        };
        read(e, argument);
        let mut i = 0u32;
        while i < e.mem.u32(argument) {
            read(e, weight);
            i += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a89c70,
            ni_vert_weights_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a8_9b00, ni_vert_weights_extra_data_is_equal(Ptr, Ptr) -> u32),
        entry!(0x00a8_9b20, ni_vert_weights_extra_data_get_rtti() -> Ptr),
        entry!(0x00a8_9b30, ni_vert_weights_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_9b80, fn_00a89b80() -> Ptr),
        entry!(
            0x00a8_9bb0,
            ni_vert_weights_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a8_9be0,
            ni_vert_weights_extra_data_load_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_vert_weights_extra_data_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a7_b580;
        const NAME_POINTER: u32 = 0x011f_5b78;
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
            ni_vert_weights_extra_data_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
mod class_tests {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .replace(vec![])
            .unwrap()
            .into_iter()
            .skip(1)
            .collect()
    }

    #[test]
    fn is_equal_normalises_the_low_byte() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_IS_EQUAL, |_, _| 0x1234_5602u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a8_9b00, &args![0x10u32, 0x20u32]).u32(),
            0x1234_5601
        );
        assert_eq!(calls(&mut e), vec![(EXTRA_DATA_IS_EQUAL, vec![0x10, 0x20])]);
        e.register(EXTRA_DATA_IS_EQUAL, |_, _| 0x1234_5600u32.into_ret());
        assert_eq!(
            e.call(0x00a8_9b00, &args![0x10u32, 0x20u32]).u32(),
            0x1234_5600
        );
        assert_eq!(e.call(0x00a8_9b20, &args![]).u32(), 0x011f_5b78);
    }

    #[test]
    fn create_and_clone() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let object = e.call(0x00a8_9b80, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_VERT_WEIGHTS_EXTRA_DATA_VTABLE);
        assert_eq!(
            calls(&mut e),
            vec![
                (NI_ALLOCATE, vec![0xc]),
                (EXTRA_DATA_CONSTRUCT, vec![object])
            ]
        );
        let clone = e.call(0x00a8_9b30, &args![object, 0x55u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_VERT_WEIGHTS_EXTRA_DATA_VTABLE);
        assert_eq!(
            calls(&mut e),
            vec![
                (NI_ALLOCATE, vec![0xc]),
                (EXTRA_DATA_CONSTRUCT, vec![clone]),
                (EXTRA_DATA_COPY_MEMBERS, vec![object, clone, 0x55])
            ]
        );
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_9b80, &args![]).u32(), 0);
        assert_eq!(calls(&mut e), vec![(NI_ALLOCATE, vec![0xc])]);
        assert_eq!(e.call(0x00a8_9b30, &args![object, 0x55u32]).u32(), 0);
        assert_eq!(
            calls(&mut e),
            vec![
                (NI_ALLOCATE, vec![0xc]),
                (EXTRA_DATA_COPY_MEMBERS, vec![object, 0, 0x55])
            ]
        );
    }

    #[test]
    fn deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_9bb0, &args![this, 0x100u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_VERT_WEIGHTS_EXTRA_DATA_VTABLE);
        assert_eq!(calls(&mut e), vec![(EXTRA_DATA_DESTRUCT, vec![this])]);
        e.call(0x00a8_9bb0, &args![this, 1u32]);
        assert_eq!(
            calls(&mut e),
            vec![
                (EXTRA_DATA_DESTRUCT, vec![this]),
                (NI_FREE_SIZED, vec![this, 0xc])
            ]
        );
    }

    #[test]
    fn load_binary_skips_the_weights_of_old_files() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        let stream = e.mem.alloc(0x260);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut answers = vec![3u32, 9, 9, 9].into_iter();
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[2], a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], answers.next().unwrap());
            4u32.into_ret()
        });
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(stream + 0xd8, 0x0500_000f);
        e.call_log = Some(vec![]);
        e.call(0x00a8_9be0, &args![this, stream]);
        let log = calls(&mut e);
        assert_eq!(log[0], (EXTRA_DATA_LOAD_BINARY, vec![this, stream]));
        assert_eq!(log.len(), 5);
        assert_eq!(log[2].1[1], log[3].1[1]);
        assert_ne!(log[1].1[1], log[2].1[1]);
        assert_eq!(e.mem.u32(input + 4), 16);
        e.mem.set_u32(stream + 0xd8, 0x0500_0010);
        e.call(0x00a8_9be0, &args![this, stream]);
        assert!(calls(&mut e).is_empty());
    }
}
