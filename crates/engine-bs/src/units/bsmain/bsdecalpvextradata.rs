//! `BSMain/bsdecalpvextradata.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a9af00` (named `NiTextKeyExtraData::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A9AF00: u32 = 0x00a9_af00;

// Translated from 00c644b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSDecalPlacementVectorExtraData` vtable word 010c38fc,
/// right after its `LoadBinary` (010c38f8). Forwards `this` and the stream to `00a9af00`, a
/// one-instruction jump to `NiExtraData::LinkObject` (`00a7b3b0`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c644b0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A9AF00, &args![this, stream]);
}

// Translated from 00c64600 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043d4 without reading the receiver. The map names this body `BSDecalPlacementVectorExtraData::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c64600(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43d4
}

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00aa1460`: the sized delete (`cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The class's constructor (`00c63ab0`; `thiscall`: value).
const DECAL_PLACEMENT_VECTOR_CONSTRUCTOR: u32 = 0x00c6_3ab0;
/// The decal placement group constructor (`00c64640`; `thiscall`).
const GROUP_CONSTRUCTOR: u32 = 0x00c6_4640;
/// The skinned-object list's clear (`00c64710`; `thiscall`).
const LIST_CLEAR: u32 = 0x00c6_4710;
/// The base class's member copy (`00a99c40`; `thiscall` on the source: clone, cloning
/// process) and `LoadBinary` (`00a99d30`; `thiscall`: stream).
const BASE_COPY_MEMBERS: u32 = 0x00a9_9c40;
const BASE_LOAD_BINARY: u32 = 0x00a9_9d30;
/// The `NiExtraData` destructor (`00a7b300`; `thiscall`).
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// The function the create function calls on the new object with the address `011f49e0`
/// (`00a5b950`; `thiscall`, one stack word).
const CREATE_SETUP: u32 = 0x00a5_b950;
const CREATE_SETUP_ARGUMENT: u32 = 0x011f_49e0;
/// Stream reads (`cdecl`): one 16-bit word (`00a30370`: stream, destination) and a run
/// of floats (`00a41770`: stream, destination, count).
const READ_WORD16: u32 = 0x00a3_0370;
const READ_FLOATS: u32 = 0x00a4_1770;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free (`00aa4060`),
/// both `thiscall` with one stack word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The virtual tables: the class's (`010c38ac`) and the base extra data class's
/// (`010a0d24`).
const DECAL_PLACEMENT_VECTOR_VTABLE: u32 = 0x010c_38ac;
const BASE_VTABLE: u32 = 0x010a_0d24;
/// The size of a decal placement group (0xC4 bytes).
const GROUP_SIZE: u32 = 0xc4;

layout! {
    /// `BSDecalPlacementVectorExtraData` (Xbox PDB), 0x40 bytes.
    pub struct BSDecalPlacementVectorExtraData: 0x40 {
        /// `usGroupCount` (Xbox PDB).
        0x10 usGroupCount: u16,
        /// `pDecalPlacementGroups` (Xbox PDB): eight group pointers from here.
        0x14 pDecalPlacementGroups: u32,
    }
}

/// A new decal placement group: 0xC4 bytes from the memory manager, constructed
/// (`00c64640`) when the allocation succeeded; null otherwise.
fn new_group(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, GROUP_SIZE])
        .u32();
    if block == 0 {
        return 0;
    }
    e.call(GROUP_CONSTRUCTOR, &args![block]).u32()
}

// Translated from 00c63b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c63b40 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `BSDecalPlacementVectorExtraData`'s destructor (unnamed in the engine map; `thiscall`):
/// stores the class's virtual table, frees every set group pointer of the eight
/// (`00aa4060`) and clears it, empties the skinned-object list (`00c64710`), then the base
/// class's virtual table and the `NiExtraData` destructor (`00a7b300`).
pub fn fn_00c63b40(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, DECAL_PLACEMENT_VECTOR_VTABLE);
    for index in 0..8 {
        let slot = object + 0x14 + 4 * index;
        let group = e.mem.u32(slot);
        if group != 0 {
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, group]);
        }
        e.mem.set_u32(slot, 0);
    }
    e.call(LIST_CLEAR, &args![object + 0x34]);
    e.mem.set_u32(object, BASE_VTABLE);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
}

// Translated from 00c64610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDecalPlacementVectorExtraData::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`,
/// `RET 4`): the destructor ([`fn_00c63b40`]) and, when bit 0 of `flags` is set, the sized
/// delete of 0x40 bytes. Answers `this`.
pub fn bs_decal_placement_vector_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c63b40(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

// Translated from 00c640f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c640f0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `BSDecalPlacementVectorExtraData::CopyMembers` (unnamed in the engine map; `thiscall` on
/// the source, `RET 8`: clone, cloning process): the base copy (`00a99c40`), the group
/// count, and for each group a new group (memory manager, `00c64640`) with the source's
/// 16-bit point count and its points copied from both arrays (`+0x4` and `+0x64`, 12 bytes
/// each). Every count is read again from memory for each step, as the exe does.
pub fn bs_decal_placement_vector_extra_data_copy_members(
    e: &mut Engine,
    this: Ptr<BSDecalPlacementVectorExtraData>,
    clone: Ptr<BSDecalPlacementVectorExtraData>,
    cloning: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    let count = e.get(this, BSDecalPlacementVectorExtraData::usGroupCount);
    e.set(clone, BSDecalPlacementVectorExtraData::usGroupCount, count);
    let mut index = 0i32;
    while index < i32::from(e.get(this, BSDecalPlacementVectorExtraData::usGroupCount)) {
        let offset = 0x14 + 4 * index as u32;
        let group = new_group(e);
        e.mem.set_u32(clone.addr() + offset, group);
        let source = e.mem.u32(this.addr() + offset);
        let target = e.mem.u32(clone.addr() + offset);
        let points = e.mem.u16(source);
        e.mem.set_u16(target, points);
        let mut point = 0i32;
        while point < i32::from(e.mem.u16(e.mem.u32(this.addr() + offset))) {
            let source = e.mem.u32(this.addr() + offset);
            let target = e.mem.u32(clone.addr() + offset);
            for base in [0x4u32, 0x64] {
                let at = base.wrapping_add((point as u32).wrapping_mul(12));
                for word in 0..3 {
                    let value = e.mem.u32(source.wrapping_add(at + 4 * word));
                    e.mem.set_u32(target.wrapping_add(at + 4 * word), value);
                }
            }
            point += 1;
        }
        index += 1;
    }
}

// Translated from 00c64060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c64060 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `BSDecalPlacementVectorExtraData::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning
/// process): a new 0x40-byte object built by the constructor (`00c63ab0`, value 0; null
/// when the allocation fails), filled by
/// [`bs_decal_placement_vector_extra_data_copy_members`]. Answers the clone.
pub fn bs_decal_placement_vector_extra_data_create_clone(
    e: &mut Engine,
    this: Ptr<BSDecalPlacementVectorExtraData>,
    cloning: Ptr,
) -> Ptr<BSDecalPlacementVectorExtraData> {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(DECAL_PLACEMENT_VECTOR_CONSTRUCTOR, &args![block, 0u32])
            .u32()
    };
    let clone = Ptr::new(clone);
    bs_decal_placement_vector_extra_data_copy_members(e, this, clone, cloning);
    clone
}

// Translated from 00c64270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c64270 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `BSDecalPlacementVectorExtraData::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream):
/// the base `LoadBinary` (`00a99d30`), the 16-bit group count (`00a30370`), then for each
/// group a new group (memory manager, `00c64640`), its 16-bit point count and both point
/// arrays (`00a41770`, three floats per point, at `+0x4` and `+0x64`).
pub fn bs_decal_placement_vector_extra_data_load_binary(
    e: &mut Engine,
    this: Ptr<BSDecalPlacementVectorExtraData>,
    stream: Ptr,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.call(READ_WORD16, &args![stream, this.addr() + 0x10]);
    let mut index = 0i32;
    while index < i32::from(e.get(this, BSDecalPlacementVectorExtraData::usGroupCount)) {
        let offset = 0x14 + 4 * index as u32;
        let group = new_group(e);
        e.mem.set_u32(this.addr() + offset, group);
        let group = e.mem.u32(this.addr() + offset);
        e.call(READ_WORD16, &args![stream, group]);
        let group = e.mem.u32(this.addr() + offset);
        let count = u32::from(e.mem.u16(group));
        e.call(
            READ_FLOATS,
            &args![stream, group.wrapping_add(4), count.wrapping_mul(3)],
        );
        let group = e.mem.u32(this.addr() + offset);
        let count = u32::from(e.mem.u16(group));
        e.call(
            READ_FLOATS,
            &args![stream, group.wrapping_add(0x64), count.wrapping_mul(3)],
        );
        index += 1;
    }
}

// Translated from 00c64530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c64530 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// The class's create function (unnamed in the engine map; plain `RET`): a new 0x40-byte
/// object built by the constructor (`00c63ab0`, value 0; null when the allocation fails),
/// then `00a5b950` called on it with the address `011f49e0` (also on a null object).
/// Answers the object.
pub fn fn_00c64530(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(DECAL_PLACEMENT_VECTOR_CONSTRUCTOR, &args![block, 0u32])
            .u32()
    };
    e.call(CREATE_SETUP, &args![object, CREATE_SETUP_ARGUMENT]);
    Ptr::new(object)
}

/// `NiFloatExtraData::SaveBinary` (`00a9a4e0`), `00a742a0` (unnamed,
/// `NiMain/nistringextradata.cpp`), `NiBinaryExtraData::RegisterStreamables`
/// (`00a89ae0`) and `NiIntegerExtraData::GetViewerStrings` (`00a99d70`):
/// the engine map's names for the bodies this class's slots call;
/// `thiscall`, one word each.
const BASE_SAVE_BINARY: u32 = 0x00a9_a4e0;
const BASE_IS_EQUAL: u32 = 0x00a7_42a0;
const BASE_REGISTER_STREAMABLES: u32 = 0x00a8_9ae0;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a9_9d70;
/// `NiStreamSaveBinary<unsigned short>` (`00a303a0`, `cdecl`, `(stream,
/// &value)`) and `00a417b0` (unnamed, `cdecl`, `(stream, floats, count)`:
/// writes `count` floats).
const SAVE_WORD16: u32 = 0x00a3_03a0;
const SAVE_FLOATS: u32 = 0x00a4_17b0;
/// `NiGetViewerString` (`00a7d330`, the class name) and the viewer-string
/// array's `Add` (`0096a610`, `thiscall` on the array, the string's
/// address); the word the class name is read from (`011f4428`, which other
/// units read as `NiNode`'s `NiRTTI` record).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_ADD: u32 = 0x0096_a610;
const CLASS_NAME_RECORD: u32 = 0x011f_4428;

// Translated from 00c643d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDecalPlacementVectorExtraData::SaveBinary` (Xbox PDB): the base part
/// (`00a9a4e0`), the group count (`00a303a0` with +0x10), then for each
/// group (index below the 16-bit count, signed) its vertex count
/// (`00a303a0` with the group's first word), its positions (`00a417b0`:
/// group +4, three floats per vertex) and its normals (group +0x64, the
/// same count).
pub fn bs_decal_placement_vector_extra_data_save_binary(
    e: &mut Engine,
    this: Ptr<BSDecalPlacementVectorExtraData>,
    stream: Ptr,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.call(SAVE_WORD16, &args![stream, this.addr() + 0x10]);
    let mut index = 0i32;
    while index < i32::from(e.get(this, BSDecalPlacementVectorExtraData::usGroupCount)) {
        let slot = this.addr() + 0x14 + 4 * index as u32;
        let group = e.mem.u32(slot);
        e.call(SAVE_WORD16, &args![stream, group]);
        let group = e.mem.u32(slot);
        let vertices = u32::from(e.mem.u16(group));
        e.call(
            SAVE_FLOATS,
            &args![stream, group + 4, vertices.wrapping_mul(3)],
        );
        let group = e.mem.u32(slot);
        let vertices = u32::from(e.mem.u16(group));
        e.call(
            SAVE_FLOATS,
            &args![stream, group + 0x64, vertices.wrapping_mul(3)],
        );
        index += 1;
    }
}

// Translated from 00c644d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the class vtable's `IsEqual` slot (word
/// `010c3908`). Answers `00a742a0`'s AL as it is.
pub fn fn_00c644d0(e: &mut Engine, this: Ptr, other: Ptr) -> u8 {
    e.call(BASE_IS_EQUAL, &args![this, other]).u8()
}

// Translated from 00c64500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the class vtable's `RegisterStreamables` slot
/// (word `010c3900`). The answer in EAX is 0 when `00a89ae0` answered
/// nonzero in AL and 1 otherwise (`NEG`, `SBB`, `ADD 1`, as the exe
/// computes it).
pub fn fn_00c64500(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    let registered = e.call(BASE_REGISTER_STREAMABLES, &args![this, stream]).u8();
    u32::from(registered == 0)
}

// Translated from 00c645c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDecalPlacementVectorExtraData::GetViewerStrings` (Xbox PDB): the base
/// strings (`00a99d70`), then the name in the record at `011f4428` (not
/// this class's own `012043d4`), added through `0096a610`.
pub fn bs_decal_placement_vector_extra_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_RECORD);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_ADD, &args![strings, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c6_43d0,
            bs_decal_placement_vector_extra_data_save_binary(
                Ptr<BSDecalPlacementVectorExtraData>,
                Ptr,
            )
        ),
        entry!(0x00c6_44d0, fn_00c644d0(Ptr, Ptr) -> u8),
        entry!(0x00c6_4500, fn_00c64500(Ptr, Ptr) -> u32),
        entry!(
            0x00c6_45c0,
            bs_decal_placement_vector_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c6_4600, fn_00c64600(Ptr) -> u32),
        entry!(0x00c6_44b0, fn_00c644b0(Ptr, Ptr)),
        entry!(0x00c6_3b40, fn_00c63b40(Ptr)),
        entry!(0x00c6_4610, bs_decal_placement_vector_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c6_40f0,
            bs_decal_placement_vector_extra_data_copy_members(
                Ptr<BSDecalPlacementVectorExtraData>,
                Ptr<BSDecalPlacementVectorExtraData>,
                Ptr,
            )
        ),
        entry!(
            0x00c6_4060,
            bs_decal_placement_vector_extra_data_create_clone(
                Ptr<BSDecalPlacementVectorExtraData>,
                Ptr,
            ) -> Ptr<
                BSDecalPlacementVectorExtraData,
            >
        ),
        entry!(
            0x00c6_4270,
            bs_decal_placement_vector_extra_data_load_binary(
                Ptr<BSDecalPlacementVectorExtraData>,
                Ptr,
            )
        ),
        entry!(0x00c6_4530, fn_00c64530() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c644b0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A9AF00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_44b0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A9AF00)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c64600_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_4600, &args![0x1000u32]).u32(), 0x0120_43d4);
        assert_eq!(e.call(0x00c6_4600, &args![0u32]).u32(), 0x0120_43d4);
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(GROUP_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(LIST_CLEAR, |_, _| Ret::default());
        e.register(EXTRA_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(DECAL_PLACEMENT_VECTOR_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(CREATE_SETUP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn destructor_frees_the_groups_and_empties_the_list() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(this + 0x1c, 0x6000);
        e.call(0x00c6_4610, &args![this, 0u32]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, 0x5000], vec![MEMORY_MANAGER, 0x6000]]
        );
        assert_eq!((e.mem.u32(this + 0x14), e.mem.u32(this + 0x1c)), (0, 0));
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![this + 0x34]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(calls(&e, EXTRA_DATA_DESTRUCTOR), vec![vec![this]]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00c6_4610, &args![this, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x40]]);
    }

    fn source(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 0x10, 2);
        for (index, points) in [1u16, 2].iter().enumerate() {
            let group = e.mem.alloc(GROUP_SIZE);
            e.mem.set_u16(group, *points);
            for word in 0..48 {
                e.mem
                    .set_u32(group + 4 + 4 * word, 0x1000 * (index as u32 + 1) + word);
            }
            e.mem.set_u32(this + 0x14 + 4 * index as u32, group);
        }
        this
    }

    #[test]
    fn create_clone_copies_every_group() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        let this = source(&mut e);
        let clone = e.call(0x00c6_4060, &args![this, 0x700u32]).u32();
        assert_eq!(clone, block);
        assert_eq!(
            calls(&e, DECAL_PLACEMENT_VECTOR_CONSTRUCTOR),
            vec![vec![block, 0]]
        );
        assert_eq!(calls(&e, BASE_COPY_MEMBERS), vec![vec![this, block, 0x700]]);
        assert_eq!(e.mem.u16(block + 0x10), 2);
        for index in 0..2u32 {
            let from = e.mem.u32(this + 0x14 + 4 * index);
            let to = e.mem.u32(block + 0x14 + 4 * index);
            assert_ne!(from, to);
            let points = u32::from(e.mem.u16(from));
            assert_eq!(e.mem.u16(to), points as u16);
            for base in [4u32, 0x64] {
                for word in 0..3 * points {
                    let at = base + 4 * word;
                    assert_eq!(e.mem.u32(to + at), e.mem.u32(from + at));
                }
                // Past the copied points nothing is written.
                assert_eq!(e.mem.u32(to + base + 12 * points), 0);
            }
        }
        assert_eq!(calls(&e, MEMORY_MANAGER_ALLOCATE).len(), 2);
    }

    #[test]
    fn load_binary_reads_every_group() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        // The group count is 2; each group holds 3 points.
        e.register_double(READ_WORD16, move |e, a| {
            let value = if a[1] == this + 0x10 { 2 } else { 3 };
            e.mem.set_u16(a[1], value);
            Ret::default()
        });
        e.register(READ_FLOATS, |_, _| Ret::default());
        e.call(0x00c6_4270, &args![this, 0x900u32]);
        assert_eq!(calls(&e, BASE_LOAD_BINARY), vec![vec![this, 0x900]]);
        let count = e.mem.u16(this + 0x10);
        let groups: Vec<u32> = (0..u32::from(count))
            .map(|index| e.mem.u32(this + 0x14 + 4 * index))
            .collect();
        let floats = calls(&e, READ_FLOATS);
        assert_eq!(floats.len(), 2 * groups.len());
        for (index, group) in groups.iter().enumerate() {
            let points = u32::from(e.mem.u16(*group)) * 3;
            assert_eq!(floats[2 * index], vec![0x900, group + 4, points]);
            assert_eq!(floats[2 * index + 1], vec![0x900, group + 0x64, points]);
        }
    }

    #[test]
    fn create_function_sets_up_even_a_null_object() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00c6_4530, &args![]).u32(), block);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c6_4530, &args![]).u32(), 0);
        assert_eq!(
            calls(&e, CREATE_SETUP),
            vec![
                vec![block, CREATE_SETUP_ARGUMENT],
                vec![0, CREATE_SETUP_ARGUMENT]
            ]
        );
        assert_eq!(calls(&e, DECAL_PLACEMENT_VECTOR_CONSTRUCTOR).len(), 1);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn save_binary_writes_every_group() {
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_WORD16, |_, _| Ret::default());
        e.register(SAVE_FLOATS, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let first = e.mem.alloc(0xc4);
        let second = e.mem.alloc(0xc4);
        e.mem.set_u16(first, 2);
        e.mem.set_u16(second, 5);
        e.mem.set_u16(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, first);
        e.mem.set_u32(this + 0x18, second);
        e.call_log = Some(vec![]);
        e.call(0x00c6_43d0, &args![this, 0x2000u32]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, 0x2000]]);
        assert_eq!(
            calls_to(&e, SAVE_WORD16),
            vec![
                vec![0x2000, this + 0x10],
                vec![0x2000, first],
                vec![0x2000, second]
            ]
        );
        assert_eq!(
            calls_to(&e, SAVE_FLOATS),
            vec![
                vec![0x2000, first + 4, 6],
                vec![0x2000, first + 0x64, 6],
                vec![0x2000, second + 4, 15],
                vec![0x2000, second + 0x64, 15]
            ]
        );
    }

    #[test]
    fn is_equal_and_register_streamables_use_the_base_answers() {
        let mut e = Engine::new();
        e.register(BASE_IS_EQUAL, |_, a| word(a[1] & 0xff));
        assert_eq!(e.call(0x00c6_44d0, &args![0x1000u32, 0x2003u32]).u32(), 3);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(0x100));
        assert_eq!(e.call(0x00c6_4500, &args![0x1000u32, 0x2000u32]).u32(), 1);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(1));
        assert_eq!(e.call(0x00c6_4500, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn get_viewer_strings_adds_the_recorded_name() {
        let mut e = Engine::new();
        e.map(CLASS_NAME_RECORD, 4);
        e.set_global(CLASS_NAME_RECORD, 0x0555_0000u32);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(STRINGS_ADD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x7000_0001);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c6_45c0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0555_0000]]);
        assert_eq!(calls_to(&e, STRINGS_ADD)[0][0], 0x2000);
    }
}
