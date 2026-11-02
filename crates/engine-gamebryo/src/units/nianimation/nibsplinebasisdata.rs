//! `NiAnimation/nibsplinebasisdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineBasisData::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_BASIS_DATA_RTTI: u32 = 0x011f_403c;

// Translated from 00a52af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_basis_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_BASIS_DATA_RTTI)
}

// Translated from 00a52c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a5d6e0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f403c`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_b_spline_basis_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
    const CLASS_NAME_POINTER: u32 = 0x011f_403c;
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

/// The `NiBSplineBasisData` vtable (RTTI `.?AVNiBSplineBasisData@@`).
const B_SPLINE_BASIS_DATA_VTABLE: u32 = 0x0109_9f24;
/// `00aa13e0` (`cdecl`, one word): allocates; `00aa1460` (`cdecl`, `(block, size)`): the
/// sized delete.
const ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a5d3a0` / `00a5d3d0` (`thiscall`, unit `niobject.cpp`): the `NiObject` constructor
/// and destructor bodies; `00a5d450` (`thiscall`, `RET 8`): its member copy;
/// `NiObject::LoadBinary` (`00a5d610`, `thiscall`, `RET 4`).
const OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
const OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// The float a new basis starts its `+0x1c` field with (`01012054`, read as the game does).
const INITIAL_PARAMETER: u32 = 0x0101_2054;

// Translated from 00a62660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::GetDegree3Basis` (Xbox PDB; `ECX` only): the address of the
/// basis inside the object, `this + 8`.
pub fn ni_b_spline_basis_data_get_degree3_basis(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(8))
}

// Translated from 00a52b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra's project (read from the bytes; the class factory of
/// `NiBSplineBasisData`, `cdecl`, no argument): allocates 0x28 bytes (`00aa13e0`); a null
/// block answers null. Otherwise the `NiObject` constructor runs on it, the vtable
/// `01099f24` is set, the float at `01012054` is copied to `+0x1c` through the x87 stack,
/// and the words at `+0x08` and `+0x20` become 0 and the one at `+0x24` 3. Answers the
/// object.
pub fn fn_00a52b60(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(OBJECT_CONSTRUCTOR, &args![block]);
    let parameter = x87_quiet_bits(e.mem.u32(INITIAL_PARAMETER));
    e.mem.set_u32(block, B_SPLINE_BASIS_DATA_VTABLE);
    e.mem.set_u32(block + 0x1c, parameter);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0x20, 0);
    e.mem.set_u32(block + 0x24, 3);
    Ptr::new(block)
}

// Translated from 00a52b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::CreateClone` (Xbox PDB; `thiscall`, `RET 4`): a new object built
/// as `00a52b60` builds it (null when the allocation fails), the `NiObject` member copy
/// (`00a5d450`) from `this` to it with `cloning`, then the eight words at `+0x08..+0x28`
/// of `this` copied over (`REP MOVSD`; written even to a null clone, as the exe does).
/// Answers the clone.
pub fn ni_b_spline_basis_data_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let clone = fn_00a52b60(e).addr();
    e.call(OBJECT_COPY_MEMBERS, &args![this, clone, cloning]);
    for word in 0..8 {
        let at = 8 + 4 * word;
        let value = e.mem.u32(this.addr().wrapping_add(at));
        e.mem.set_u32(clone.wrapping_add(at), value);
    }
    Ptr::new(clone)
}

// Translated from 00a52be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// sets the vtable, runs the `NiObject` destructor body (`00a5d3d0`) and, when bit 0 of
/// `flags` is set, the sized delete of 0x28 bytes. Answers `this`.
pub fn ni_b_spline_basis_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), B_SPLINE_BASIS_DATA_VTABLE);
    e.call(OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x28u32]);
    }
    this
}

// Translated from 00a52c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`):
/// `NiObject::LoadBinary`, then the stream's reader (`stream + 0x24c`) reads one 4-byte
/// word into `+0x08` through the function pointer at its `+8` (`cdecl`
/// `(reader, this + 8, 4, &4, 1)`, the count slot being the stack slot of the stream
/// argument); the answer is added to the reader's word at `+4`.
pub fn ni_b_spline_basis_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    let reader = e.mem.u32(stream.addr() + 0x24c);
    let read_function = e.mem.u32(reader + 8);
    let destination = this.addr().wrapping_add(8);
    let read = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(
            read_function,
            &args![reader, destination, 4u32, count, 1u32],
        )
        .u32()
    });
    let total = e.mem.u32(reader + 4).wrapping_add(read);
    e.mem.set_u32(reader + 4, total);
}

/// `NiObject::IsEqual` (`00a5d470`, engine map; `thiscall`, one word) and
/// the base `SaveBinary` (`00a89af0`, `thiscall`, one word).
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;

// Translated from 00a52bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::IsEqual` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the other object): `NiObject::IsEqual` and the
/// same word at +0x08 (the basis's control point count).
pub fn ni_b_spline_basis_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.mem.u32(other.addr().wrapping_add(8)) == e.mem.u32(this.addr().wrapping_add(8))
}

// Translated from 00a52c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasisData::SaveBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): the base `SaveBinary` (`00a89af0`),
/// then the word at +0x08 written through the stream's writer (`stream +
/// 0x250`, its function at +0xC, `cdecl (writer, this + 8, 4, &4, 1)`, the
/// count slot being the stack slot of the stream argument); the answer is
/// added to the writer's word at +4.
pub fn ni_b_spline_basis_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(OBJECT_SAVE_BINARY, &args![this, stream]);
    let writer = e.mem.u32(stream.addr() + 0x250);
    let write_function = e.mem.u32(writer + 0xc);
    let source = this.addr().wrapping_add(8);
    let written = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(write_function, &args![writer, source, 4u32, count, 1u32])
            .u32()
    });
    let total = e.mem.u32(writer + 4).wrapping_add(written);
    e.mem.set_u32(writer + 4, total);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_2bb0, ni_b_spline_basis_data_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a5_2c50, ni_b_spline_basis_data_save_binary(Ptr, Ptr)),
        entry!(0x00a52af0, ni_b_spline_basis_data_get_rtti() -> Ptr),
        entry!(
            0x00a52c90,
            ni_b_spline_basis_data_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a6_2660, ni_b_spline_basis_data_get_degree3_basis(Ptr) -> Ptr),
        entry!(0x00a5_2b60, fn_00a52b60() -> Ptr),
        entry!(0x00a5_2b00, ni_b_spline_basis_data_create_clone(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_2be0,
            ni_b_spline_basis_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a5_2c10, ni_b_spline_basis_data_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_b_spline_basis_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a52af0, &args![]).u32(),
            NI_B_SPLINE_BASIS_DATA_RTTI
        );
    }

    #[test]
    fn ni_b_spline_basis_data_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00a5_d6e0;
        const NAME_POINTER: u32 = 0x011f_403c;
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
            ni_b_spline_basis_data_get_viewer_strings(&mut e, this, Ptr::new(strings));
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
mod basis_data_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_u32(INITIAL_PARAMETER, 0xbf80_0000);
        e.register(OBJECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_degree3_basis_is_inside_the_object() {
        let mut e = engine();
        assert_eq!(e.call(0x00a6_2660, &args![0x5000u32]).u32(), 0x5008);
    }

    #[test]
    fn the_factory_builds_a_basis_with_its_defaults() {
        let mut e = engine();
        let object = e.call(0x00a5_2b60, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, OBJECT_CONSTRUCTOR), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), B_SPLINE_BASIS_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 0x1c), 0xbf80_0000);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        assert_eq!(e.mem.u32(object + 0x24), 3);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a5_2b60, &args![]).u32(), 0);
    }

    #[test]
    fn a_clone_copies_the_eight_words() {
        let mut e = engine();
        let this = e.mem.alloc(0x28);
        for word in 0..8 {
            e.mem.set_u32(this + 8 + 4 * word, 0x100 + word);
        }
        let clone = e.call(0x00a5_2b00, &args![this, 9u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS),
            vec![vec![this, clone, 9]]
        );
        for word in 0..8 {
            assert_eq!(e.mem.u32(clone + 8 + 4 * word), 0x100 + word);
        }
        assert_eq!(e.mem.u32(clone), B_SPLINE_BASIS_DATA_VTABLE);
    }

    #[test]
    fn the_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x28);
        assert_eq!(e.call(0x00a5_2be0, &args![this, 2u32]).u32(), this);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a5_2be0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x28]]);
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR), vec![vec![this]; 2]);
        assert_eq!(e.mem.u32(this), B_SPLINE_BASIS_DATA_VTABLE);
    }

    #[test]
    fn loading_reads_one_word_through_the_reader() {
        let mut e = engine();
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x77);
            4u32.into_ret()
        });
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 4, 0x10);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x28);
        e.call(0x00a5_2c10, &args![this, stream]);
        assert_eq!(calls_to(&e, OBJECT_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + 8), 0x77);
        assert_eq!(e.mem.u32(reader + 4), 0x14);
        let reads = calls_to(&e, READ);
        assert_eq!(
            (reads[0][0], reads[0][1], reads[0][2], reads[0][4]),
            (reader, this + 8, 4, 1)
        );
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

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
    fn is_equal_compares_the_control_point_count() {
        let mut e = Engine::new();
        e.register(OBJECT_IS_EQUAL, |_, _| word(1));
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x28);
        let other = e.mem.alloc(0x28);
        e.mem.set_u32(this + 8, 7);
        e.mem.set_u32(other + 8, 7);
        assert!(e.call(0x00a5_2bb0, &args![this, other]).bool());
        assert_eq!(calls(&e, OBJECT_IS_EQUAL), vec![vec![this, other]]);
        e.mem.set_u32(other + 8, 8);
        assert!(!e.call(0x00a5_2bb0, &args![this, other]).bool());
        e.mem.set_u32(other + 8, 7);
        e.register(OBJECT_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_2bb0, &args![this, other]).bool());
    }

    #[test]
    fn save_binary_writes_the_control_point_count() {
        const WRITE: u32 = 0x0300_0200;
        let mut e = Engine::new();
        e.register(OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            word(4)
        });
        e.call_log = Some(vec![]);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 4, 0x10);
        e.mem.set_u32(writer + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, writer);
        let this = e.mem.alloc(0x28);
        e.call(0x00a5_2c50, &args![this, stream]);
        assert_eq!(calls(&e, OBJECT_SAVE_BINARY), vec![vec![this, stream]]);
        assert_eq!(calls(&e, WRITE)[0][..2], [writer, this + 8]);
        assert_eq!(e.mem.u32(writer + 4), 0x14);
    }
}
