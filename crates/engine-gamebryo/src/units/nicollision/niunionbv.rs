//! `NiCollision/niunionbv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;
use crate::units::nicollision::nicollisiondata::{
    add_viewer_string, c_string_length, stream_transfer,
};

/// `00a90180`: `thiscall(array, new capacity)`, the array `SetSize`.
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
/// `00ba3aa0`: `thiscall(array, index, pointer to the value)`, the array `SetAt`.
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;

// Translated from 00a64330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `Add` body of an `NiTArray`-shaped array of pointers (layout [`NiTArray`]; `thiscall`,
/// one stack word). The engine map names this address
/// `NiUnionBV::AddBoundingVolume`, which is the template instance that identical
/// code folding put the name on: the body works on the array itself (`this` is the
/// array, with its pointer at +4 and 16-bit counts from +8), not on an
/// `NiUnionBV`, whose array sits at +4, so it is named by its address.
///
/// `element` points at the value to add. A null value is refused (result
/// `0xffffffff`). Otherwise the first empty slot below the used size takes the value
/// (the non-null count goes up) and its index is returned. With no empty slot, the
/// index is the used size: when that equals or exceeds the capacity the array is
/// first grown by its grow-by amount (`SetSize` `00a90180`, capacity
/// `used size + grow-by`), then `SetAt` (`00ba3aa0`) stores the value there.
pub fn fn_00a64330(e: &mut Engine, this: Ptr<NiTArray>, element: Ptr) -> u32 {
    let value = e.mem.u32(element.addr());
    if value == 0 {
        return 0xffff_ffff;
    }
    let size = e.get(this, NiTArray::m_usSize);
    let mut index = 0u16;
    if size != 0 {
        loop {
            let base = e.get(this, NiTArray::m_pBase);
            let slot = base.wrapping_add(index as u32 * 4);
            if e.mem.u32(slot) == 0 {
                e.mem.set_u32(slot, value);
                let present = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, present.wrapping_add(1));
                return index as u32;
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiTArray::m_usSize) {
                break;
            }
        }
    }
    let used = e.get(this, NiTArray::m_usSize) as u32;
    if used >= e.get(this, NiTArray::m_usMaxSize) as u32 {
        let grow = e.get(this, NiTArray::m_usGrowBy) as u32;
        e.call(ARRAY_SET_SIZE, &args![this, grow.wrapping_add(used)]);
    }
    e.call(ARRAY_SET_AT, &args![this, used, element]);
    used
}

layout! {
    /// `NiUnionBV` (Xbox PDB), 0x18 bytes: the vtable, `m_kBoundingVolume` (an
    /// `NiTPrimitiveArray<NiBoundingVolume*>`, laid out as [`NiTArray`]: its own vtable at +4,
    /// the elements at +8, 16-bit capacity, size, non-null count and grow-by from +0xc) and
    /// `m_uiWhichObjectIntersect`.
    pub struct NiUnionBV: 0x18 {
        /// `m_kBoundingVolume` (Xbox PDB): the array's vtable.
        0x04 m_kBoundingVolume_vtable: u32,
        /// `m_kBoundingVolume.m_pBase` (Xbox PDB): the child volumes.
        0x08 m_kBoundingVolume_pBase: u32,
        /// `m_kBoundingVolume.m_usMaxSize` (Xbox PDB).
        0x0C m_kBoundingVolume_usMaxSize: u16,
        /// `m_kBoundingVolume.m_usSize` (Xbox PDB).
        0x0E m_kBoundingVolume_usSize: u16,
        /// `m_kBoundingVolume.m_usESize` (Xbox PDB).
        0x10 m_kBoundingVolume_usESize: u16,
        /// `m_kBoundingVolume.m_usGrowBy` (Xbox PDB).
        0x12 m_kBoundingVolume_usGrowBy: u16,
        /// `m_uiWhichObjectIntersect` (Xbox PDB): the index of the child that intersected.
        0x14 m_uiWhichObjectIntersect: u32,
    }
}

/// `NiUnionBV`'s vtable (`010f1778`, RTTI `.?AVNiUnionBV@@`).
const NI_UNION_BV_VTABLE: u32 = 0x010f_1778;
/// The vtable of `NiTArray<NiBoundingVolume*, NiTMallocInterface<...>>` (`010f1770`, from its
/// RTTI), whose only slot is the scalar deleting destructor `00e9ec90`.
const BOUNDING_VOLUME_ARRAY_VTABLE: u32 = 0x010f_1770;
/// The vtable of `NiTPrimitiveArray<NiBoundingVolume*>` (`010f17ac`, from its RTTI).
const BOUNDING_VOLUME_PRIMITIVE_ARRAY_VTABLE: u32 = 0x010f_17ac;
/// `NiBoundingVolume`'s vtable (`010f16f4`, RTTI `.?AVNiBoundingVolume@@`).
const NI_BOUNDING_VOLUME_VTABLE: u32 = 0x010f_16f4;
/// The memory manager instance (`this` of its allocate and deallocate).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, thiscall on [`MEMORY_MANAGER`], size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager::Deallocate` (`00aa4060`, thiscall on [`MEMORY_MANAGER`], block).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The allocator `00aa1070` (cdecl, size): returns the block.
const ALLOCATE: u32 = 0x00aa_1070;
/// The deallocator `00aa10f0` (cdecl, block), unnamed in the engine map.
const FREE: u32 = 0x00aa_10f0;
/// `NiSprintf` (`00a305b0`, cdecl: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiBoundingVolume::SaveBinary` (`00e9cb10`, thiscall, the stream).
const NI_BOUNDING_VOLUME_SAVE_BINARY: u32 = 0x00e9_cb10;
/// `NiBoundingVolume`'s `LoadBinary` part (`00a89af0`, thiscall, the stream).
const NI_BOUNDING_VOLUME_LOAD_BINARY: u32 = 0x00a8_9af0;
/// `NiBoundingVolume::CreateFromStream` (`00e9cab0`, cdecl, the stream).
const NI_BOUNDING_VOLUME_CREATE_FROM_STREAM: u32 = 0x00e9_cab0;
/// Unnamed tests of `NiCollision/niboundingvolume.cpp` (cdecl) the union versions forward each
/// child to: `00e9c8c0` takes five words (the time, two volumes, two velocities), `00e9ca10`
/// seven, `00e9c950` ten and `00e9ca50` twelve, the time first and the volume second.
const VOLUME_TEST_00E9C8C0: u32 = 0x00e9_c8c0;
const VOLUME_TEST_00E9CA10: u32 = 0x00e9_ca10;
const VOLUME_TEST_00E9C950: u32 = 0x00e9_c950;
const VOLUME_TEST_00E9CA50: u32 = 0x00e9_ca50;
/// `"%s = UNION_BV"` and `"   child %i"`.
const UNION_BV_FORMAT: u32 = 0x010f_17bc;
const CHILD_FORMAT: u32 = 0x010f_17b0;

/// The child volume at `index` of the union `union`.
fn child(e: &Engine, union: Ptr<NiUnionBV>, index: u32) -> u32 {
    let base = e.get(union, NiUnionBV::m_kBoundingVolume_pBase);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// The union's child count (`m_usSize`).
fn child_count(e: &Engine, union: Ptr<NiUnionBV>) -> u32 {
    e.get(union, NiUnionBV::m_kBoundingVolume_usSize) as u32
}

/// The union's array `m_kBoundingVolume` (at +4), as the array functions take it.
fn volume_array(union: Ptr<NiUnionBV>) -> u32 {
    union.addr().wrapping_add(4)
}

// Translated from 00e9e860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: empties the union. Every non-null child is deleted through its
/// scalar deleting destructor (virtual slot +8, flag 1), then every slot below the size is
/// cleared and the size and the non-null count are set to 0.
pub fn fn_00e9e860(e: &mut Engine, this: Ptr<NiUnionBV>) {
    let mut index = 0u32;
    while index < child_count(e, this) {
        let volume = child(e, this, index);
        if volume != 0 {
            e.vcall(volume, 8, &args![1u32]);
        }
        index = index.wrapping_add(1);
    }
    let mut index = 0u16;
    while (index as u32) < child_count(e, this) {
        let base = e.get(this, NiUnionBV::m_kBoundingVolume_pBase);
        e.mem.set_u32(base.wrapping_add(index as u32 * 4), 0);
        index = index.wrapping_add(1);
    }
    e.set(this, NiUnionBV::m_kBoundingVolume_usSize, 0);
    e.set(this, NiUnionBV::m_kBoundingVolume_usESize, 0);
}

// Translated from 00e9e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::operator==` (Xbox PDB; the vtable's slot +0x24): the other volume's `Type`
/// (virtual slot +0xc) must be 4 and its child count this one's; then each child pair is asked
/// `operator!=` (the child's virtual slot +0x28, the other's child as the argument), and any
/// true answer makes the result false.
pub fn ni_union_bv_operator_(e: &mut Engine, this: Ptr<NiUnionBV>, other: Ptr<NiUnionBV>) -> bool {
    if e.vcall(other.addr(), 0xc, &args![]).u32() != 4 {
        return false;
    }
    if e.get(this, NiUnionBV::m_kBoundingVolume_usSize)
        != e.get(other, NiUnionBV::m_kBoundingVolume_usSize)
    {
        return false;
    }
    let mut index = 0u32;
    while index < child_count(e, this) {
        let mine = child(e, this, index);
        let theirs = child(e, other, index);
        if e.vcall(mine, 0x28, &args![theirs]).bool() {
            return false;
        }
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 00e9e930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::UpdateWorldData` (Xbox PDB): each child's `UpdateWorldData` (virtual slot +0x14)
/// with the model union's child of the same index and the transform.
pub fn ni_union_bv_update_world_data(
    e: &mut Engine,
    this: Ptr<NiUnionBV>,
    model: Ptr<NiUnionBV>,
    transform: Ptr,
) {
    let mut index = 0u32;
    while index < child_count(e, this) {
        let volume = child(e, this, index);
        let model_child = child(e, model, index);
        e.vcall(volume, 0x14, &args![model_child, transform]);
        index = index.wrapping_add(1);
    }
}

// Translated from 00e9e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a union-against-volume test (cdecl, five words). Each child of
/// `union` in turn goes to `00e9c8c0` as `(time, child, other, union_velocity, other_velocity)`
/// (the time as `FLD`/`FSTP` leave it); the first that answers true is stored in
/// `m_uiWhichObjectIntersect` and the answer is true. Otherwise false.
pub fn fn_00e9e980(
    e: &mut Engine,
    time: f32,
    union: Ptr<NiUnionBV>,
    other: Ptr,
    union_velocity: Ptr,
    other_velocity: Ptr,
) -> bool {
    let mut index = 0u32;
    while index < child_count(e, union) {
        let volume = child(e, union, index);
        let time_bits = x87_quiet_bits(time.to_bits());
        let hit = e
            .call(
                VOLUME_TEST_00E9C8C0,
                &args![time_bits, volume, other, union_velocity, other_velocity],
            )
            .bool();
        if hit {
            e.set(union, NiUnionBV::m_uiWhichObjectIntersect, index);
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00e9e9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): the union-against-union test
/// (cdecl, five words: the time, the first union, its velocity, the second union, its velocity).
/// Every child of the first against every child of the second goes to `00e9c8c0`; the first pair
/// that answers true stores each child's index in its union's `m_uiWhichObjectIntersect` and the
/// answer is true. Otherwise false.
pub fn fn_00e9e9e0(
    e: &mut Engine,
    time: f32,
    first: Ptr<NiUnionBV>,
    first_velocity: Ptr,
    second: Ptr<NiUnionBV>,
    second_velocity: Ptr,
) -> bool {
    let mut outer = 0u32;
    while outer < child_count(e, first) {
        let first_child = child(e, first, outer);
        let mut inner = 0u32;
        while inner < child_count(e, second) {
            let second_child = child(e, second, inner);
            let time_bits = x87_quiet_bits(time.to_bits());
            let hit = e
                .call(
                    VOLUME_TEST_00E9C8C0,
                    &args![
                        time_bits,
                        first_child,
                        first_velocity,
                        second_child,
                        second_velocity
                    ],
                )
                .bool();
            if hit {
                e.set(first, NiUnionBV::m_uiWhichObjectIntersect, outer);
                e.set(second, NiUnionBV::m_uiWhichObjectIntersect, inner);
                return true;
            }
            inner = inner.wrapping_add(1);
        }
        outer = outer.wrapping_add(1);
    }
    false
}

// Translated from 00e9ea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a union test of seven words (cdecl). Each child of `union` goes to
/// `00e9ca10` with the same words, the child in place of the union; the first that answers true
/// is stored in `m_uiWhichObjectIntersect` and the answer is true. Otherwise false.
pub fn fn_00e9ea70(
    e: &mut Engine,
    time: f32,
    union: Ptr<NiUnionBV>,
    word2: u32,
    word3: u32,
    word4: u32,
    word5: u32,
    word6: u32,
) -> bool {
    let mut index = 0u32;
    while index < child_count(e, union) {
        let volume = child(e, union, index);
        let time_bits = x87_quiet_bits(time.to_bits());
        let hit = e
            .call(
                VOLUME_TEST_00E9CA10,
                &args![time_bits, volume, word2, word3, word4, word5, word6],
            )
            .bool();
        if hit {
            e.set(union, NiUnionBV::m_uiWhichObjectIntersect, index);
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00e9eae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a union test of ten words (cdecl). Each child of `union` goes to
/// `00e9c950` with the same words, the child in place of the union; the first that answers true
/// is stored in `m_uiWhichObjectIntersect` and the answer is true. Otherwise false.
pub fn fn_00e9eae0(
    e: &mut Engine,
    time: f32,
    union: Ptr<NiUnionBV>,
    word2: u32,
    word3: u32,
    word4: u32,
    word5: u32,
    word6: u32,
    word7: u32,
    word8: u32,
    word9: u32,
) -> bool {
    let mut index = 0u32;
    while index < child_count(e, union) {
        let volume = child(e, union, index);
        let time_bits = x87_quiet_bits(time.to_bits());
        let hit = e
            .call(
                VOLUME_TEST_00E9C950,
                &args![time_bits, volume, word2, word3, word4, word5, word6, word7, word8, word9],
            )
            .bool();
        if hit {
            e.set(union, NiUnionBV::m_uiWhichObjectIntersect, index);
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00e9eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the union-against-union version of [`fn_00e9eae0`] (cdecl, ten
/// words: the time, the first union, a word, the second union, six more words). Every child of
/// the first against every child of the second goes to `00e9c950` in their unions' places; the
/// first pair that answers true stores each child's index in its union's
/// `m_uiWhichObjectIntersect` and the answer is true. Otherwise false.
pub fn fn_00e9eb60(
    e: &mut Engine,
    time: f32,
    first: Ptr<NiUnionBV>,
    word2: u32,
    second: Ptr<NiUnionBV>,
    word4: u32,
    word5: u32,
    word6: u32,
    word7: u32,
    word8: u32,
    word9: u32,
) -> bool {
    let mut outer = 0u32;
    while outer < child_count(e, first) {
        let first_child = child(e, first, outer);
        let mut inner = 0u32;
        while inner < child_count(e, second) {
            let second_child = child(e, second, inner);
            let time_bits = x87_quiet_bits(time.to_bits());
            let hit = e
                .call(
                    VOLUME_TEST_00E9C950,
                    &args![
                        time_bits,
                        first_child,
                        word2,
                        second_child,
                        word4,
                        word5,
                        word6,
                        word7,
                        word8,
                        word9
                    ],
                )
                .bool();
            if hit {
                e.set(first, NiUnionBV::m_uiWhichObjectIntersect, outer);
                e.set(second, NiUnionBV::m_uiWhichObjectIntersect, inner);
                return true;
            }
            inner = inner.wrapping_add(1);
        }
        outer = outer.wrapping_add(1);
    }
    false
}

// Translated from 00e9ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a union test of twelve words (cdecl). Each child of `union` goes
/// to `00e9ca50` with the same words, the child in place of the union; the first that answers
/// true is stored in `m_uiWhichObjectIntersect` and the answer is true. Otherwise false.
pub fn fn_00e9ec10(
    e: &mut Engine,
    time: f32,
    union: Ptr<NiUnionBV>,
    word2: u32,
    word3: u32,
    word4: u32,
    word5: u32,
    word6: u32,
    word7: u32,
    word8: u32,
    word9: u32,
    word10: u32,
    word11: u32,
) -> bool {
    let mut index = 0u32;
    while index < child_count(e, union) {
        let volume = child(e, union, index);
        let time_bits = x87_quiet_bits(time.to_bits());
        let hit = e
            .call(
                VOLUME_TEST_00E9CA50,
                &args![
                    time_bits, volume, word2, word3, word4, word5, word6, word7, word8, word9,
                    word10, word11
                ],
            )
            .bool();
        if hit {
            e.set(union, NiUnionBV::m_uiWhichObjectIntersect, index);
            return true;
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00e9ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor of
/// `NiTArray<NiBoundingVolume*, NiTMallocInterface<...>>` (the class from its vtable's RTTI).
/// Sets the vtable `010f1770`, frees the elements (`00aa10f0`, the word at +4) and, when bit 0 of
/// `flags` is set, gives the array back to the memory manager. Answers `this`.
pub fn fn_00e9ec90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), BOUNDING_VOLUME_ARRAY_VTABLE);
    let elements = e.mem.u32(this.addr().wrapping_add(4));
    e.call(FREE, &args![elements]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00e9ecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::Type` (Xbox PDB; the vtable's slot +0xc): 4.
pub fn ni_union_bv_type(_e: &mut Engine, _this: Ptr<NiUnionBV>) -> u32 {
    4
}

// Translated from 00e9ecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::Copy` (Xbox PDB): only from another union (its `Type`, virtual slot +0xc, is 4).
/// Empties this union ([`fn_00e9e860`]), sizes its array to the other's child count (`SetSize`
/// `00a90180`) and, for each of the other's children, has the child `Create` an empty volume of
/// its kind (virtual slot +0x18), `Copy`s the child into it (the new volume's slot +0x20) and
/// stores it at the same index (`SetAt` `00ba3aa0`).
pub fn ni_union_bv_copy(e: &mut Engine, this: Ptr<NiUnionBV>, other: Ptr<NiUnionBV>) {
    if e.vcall(other.addr(), 0xc, &args![]).u32() != 4 {
        return;
    }
    fn_00e9e860(e, this);
    let count = e.get(other, NiUnionBV::m_kBoundingVolume_usSize) as u32;
    e.call(ARRAY_SET_SIZE, &args![volume_array(this), count]);
    e.with_stack(4, |e, created_slot| {
        let mut index = 0u32;
        while index < child_count(e, other) {
            let source = child(e, other, index);
            let created = e.vcall(source, 0x18, &args![]).u32();
            e.mem.set_u32(created_slot.addr(), created);
            e.vcall(created, 0x20, &args![source]);
            e.call(
                ARRAY_SET_AT,
                &args![volume_array(this), index, created_slot],
            );
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00e9ed50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::SaveBinary` (Xbox PDB): `NiBoundingVolume::SaveBinary`, the child count as a
/// 4-byte item (the stream's write function at +0xc of its writer at +0x250), then each
/// non-null child's `SaveBinary` (virtual slot +4).
pub fn ni_union_bv_save_binary(e: &mut Engine, this: Ptr<NiUnionBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let value = child_count(e, this);
        e.mem.set_u32(count.addr(), value);
        let writer = e.mem.u32(stream.addr().wrapping_add(0x250));
        stream_transfer(e, writer, 0xc, count.addr(), 4);
    });
    let mut index = 0u32;
    while index < child_count(e, this) {
        let volume = child(e, this, index);
        if volume != 0 {
            e.vcall(volume, 4, &args![stream]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00e9edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiUnionBV` constructor. Sets the vtable `010f1778` and builds
/// an empty array with room for 2 (`00aa1070` gives the 8-byte element block), grow-by 1, then
/// the array's vtable `010f17ac`; `m_uiWhichObjectIntersect` is 0. Answers `this`.
pub fn fn_00e9edd0(e: &mut Engine, this: Ptr<NiUnionBV>) -> Ptr<NiUnionBV> {
    e.mem.set_u32(this.addr(), NI_UNION_BV_VTABLE);
    e.set(this, NiUnionBV::m_kBoundingVolume_usMaxSize, 2);
    e.set(
        this,
        NiUnionBV::m_kBoundingVolume_vtable,
        BOUNDING_VOLUME_ARRAY_VTABLE,
    );
    e.set(this, NiUnionBV::m_kBoundingVolume_usGrowBy, 1);
    e.set(this, NiUnionBV::m_kBoundingVolume_usSize, 0);
    e.set(this, NiUnionBV::m_kBoundingVolume_usESize, 0);
    let elements = e.call(ALLOCATE, &args![8u32]).u32();
    e.set(this, NiUnionBV::m_kBoundingVolume_pBase, elements);
    e.set(
        this,
        NiUnionBV::m_kBoundingVolume_vtable,
        BOUNDING_VOLUME_PRIMITIVE_ARRAY_VTABLE,
    );
    e.set(this, NiUnionBV::m_uiWhichObjectIntersect, 0);
    this
}

// Translated from 00e9ee20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::_scalar_deleting_destructor_` (Xbox PDB): sets the vtable `010f1778`, empties the
/// union ([`fn_00e9e860`]), sets the array's vtable `010f1770` and frees its elements
/// (`00aa10f0`), sets `NiBoundingVolume`'s vtable `010f16f4` and, when bit 0 of `flags` is set,
/// gives the object back to the memory manager. Answers `this`.
pub fn ni_union_bv_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiUnionBV>,
    flags: u32,
) -> Ptr<NiUnionBV> {
    e.mem.set_u32(this.addr(), NI_UNION_BV_VTABLE);
    fn_00e9e860(e, this);
    let elements = e.get(this, NiUnionBV::m_kBoundingVolume_pBase);
    e.set(
        this,
        NiUnionBV::m_kBoundingVolume_vtable,
        BOUNDING_VOLUME_ARRAY_VTABLE,
    );
    e.call(FREE, &args![elements]);
    e.mem.set_u32(this.addr(), NI_BOUNDING_VOLUME_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00e9ee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiUnionBV` copy constructor. Sets the vtable `010f1778` and an
/// empty array with no elements and grow-by 1 (its vtable `010f17ac`), sizes the array to the
/// other's child count (`SetSize` `00a90180`) and stores a `Clone` (virtual slot +0x1c) of each
/// non-null child of `other` at the same index (`SetAt` `00ba3aa0`). `m_uiWhichObjectIntersect`
/// is not written. Answers `this`.
pub fn fn_00e9ee60(e: &mut Engine, this: Ptr<NiUnionBV>, other: Ptr<NiUnionBV>) -> Ptr<NiUnionBV> {
    e.mem.set_u32(this.addr(), NI_UNION_BV_VTABLE);
    e.set(this, NiUnionBV::m_kBoundingVolume_usGrowBy, 1);
    e.set(this, NiUnionBV::m_kBoundingVolume_usMaxSize, 0);
    e.set(this, NiUnionBV::m_kBoundingVolume_usSize, 0);
    e.set(this, NiUnionBV::m_kBoundingVolume_usESize, 0);
    e.set(this, NiUnionBV::m_kBoundingVolume_pBase, 0);
    e.set(
        this,
        NiUnionBV::m_kBoundingVolume_vtable,
        BOUNDING_VOLUME_PRIMITIVE_ARRAY_VTABLE,
    );
    let count = e.get(other, NiUnionBV::m_kBoundingVolume_usSize) as u32;
    e.call(ARRAY_SET_SIZE, &args![volume_array(this), count]);
    e.with_stack(4, |e, clone_slot| {
        let mut index = 0u32;
        while index < child_count(e, other) {
            let source = child(e, other, index);
            if source != 0 {
                let clone = e.vcall(source, 0x1c, &args![]).u32();
                e.mem.set_u32(clone_slot.addr(), clone);
                e.call(ARRAY_SET_AT, &args![volume_array(this), index, clone_slot]);
            }
            index = index.wrapping_add(1);
        }
    });
    this
}

// Translated from 00e9eef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::Create` (Xbox PDB): a new union (0x18 bytes from the memory manager, built by
/// [`fn_00e9edd0`]; null when the allocation fails, and the code goes on with it) whose array is
/// sized to this one's child count (`SetSize` `00a90180`, on `new + 4`) and filled, index by
/// index, with what each of this union's children `Create`s (virtual slot +0x18). The array's
/// inlined `SetAt` keeps the counts: an index at or past the size makes the size `index + 1`
/// and counts a non-null value; below the size, a non-null value over a null slot counts one
/// more and a null value over a non-null slot one less. Answers the new union.
pub fn ni_union_bv_create(e: &mut Engine, this: Ptr<NiUnionBV>) -> Ptr<NiUnionBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x18u32])
        .u32();
    let created: Ptr<NiUnionBV> = if block != 0 {
        fn_00e9edd0(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    let count = e.get(this, NiUnionBV::m_kBoundingVolume_usSize) as u32;
    e.call(ARRAY_SET_SIZE, &args![volume_array(created), count]);
    let mut index = 0u32;
    while index < child_count(e, this) {
        let source = child(e, this, index);
        let value = e.vcall(source, 0x18, &args![]).u32();
        let base = e.get(created, NiUnionBV::m_kBoundingVolume_pBase);
        let slot = base.wrapping_add(index.wrapping_mul(4));
        if index >= e.get(created, NiUnionBV::m_kBoundingVolume_usSize) as u32 {
            e.set(
                created,
                NiUnionBV::m_kBoundingVolume_usSize,
                index.wrapping_add(1) as u16,
            );
            if value != 0 {
                let present = e.get(created, NiUnionBV::m_kBoundingVolume_usESize);
                e.set(
                    created,
                    NiUnionBV::m_kBoundingVolume_usESize,
                    present.wrapping_add(1),
                );
            }
        } else if value != 0 {
            if e.mem.u32(slot) == 0 {
                let present = e.get(created, NiUnionBV::m_kBoundingVolume_usESize);
                e.set(
                    created,
                    NiUnionBV::m_kBoundingVolume_usESize,
                    present.wrapping_add(1),
                );
            }
        } else if e.mem.u32(slot) != 0 {
            let present = e.get(created, NiUnionBV::m_kBoundingVolume_usESize);
            e.set(
                created,
                NiUnionBV::m_kBoundingVolume_usESize,
                present.wrapping_add(0xffff),
            );
        }
        let base = e.get(created, NiUnionBV::m_kBoundingVolume_pBase);
        e.mem
            .set_u32(base.wrapping_add(index.wrapping_mul(4)), value);
        index = index.wrapping_add(1);
    }
    created
}

// Translated from 00e9efb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::Clone` (Xbox PDB): 0x18 bytes from the memory manager, built as a copy of this
/// union ([`fn_00e9ee60`]). Answers the copy, or null.
pub fn ni_union_bv_clone(e: &mut Engine, this: Ptr<NiUnionBV>) -> Ptr<NiUnionBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x18u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00e9ee60(e, Ptr::new(block), this)
}

// Translated from 00e9efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra never created): the union's stream loader
/// (cdecl, the stream). Makes a union (0x18 bytes from the memory manager, [`fn_00e9edd0`]) and
/// runs its `LoadBinary` (virtual slot 0) on the stream. When the allocation fails the code still
/// makes the virtual call through the null pointer (it reads the vtable at address 0). Answers
/// the union.
pub fn fn_00e9efe0(e: &mut Engine, stream: Ptr) -> Ptr<NiUnionBV> {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x18u32])
        .u32();
    let union: Ptr<NiUnionBV> = if block != 0 {
        fn_00e9edd0(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    e.vcall(union.addr(), 0, &args![stream]);
    union
}

// Translated from 00e9f020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::LoadBinary` (Xbox PDB): the base volume's part (`00a89af0`), then a 4-byte child
/// count (the stream's read function at +8 of its reader at +0x24c) and that many volumes made
/// by `NiBoundingVolume::CreateFromStream`. Each non-null volume not already among the children
/// (the size is read once, before the search) is added ([`fn_00a64330`] on the array at +4).
pub fn ni_union_bv_load_binary(e: &mut Engine, this: Ptr<NiUnionBV>, stream: Ptr) {
    e.call(NI_BOUNDING_VOLUME_LOAD_BINARY, &args![this, stream]);
    e.with_stack(8, |e, locals| {
        let count_slot = locals.addr();
        let volume_slot = count_slot.wrapping_add(4);
        let reader = e.mem.u32(stream.addr().wrapping_add(0x24c));
        stream_transfer(e, reader, 8, count_slot, 4);
        let mut read = 0u32;
        while read < e.mem.u32(count_slot) {
            let volume = e
                .call(NI_BOUNDING_VOLUME_CREATE_FROM_STREAM, &args![stream])
                .u32();
            e.mem.set_u32(volume_slot, volume);
            if volume != 0 {
                let size = child_count(e, this);
                let base = e.get(this, NiUnionBV::m_kBoundingVolume_pBase);
                let mut found = false;
                let mut index = 0u32;
                while index < size {
                    if e.mem.u32(base.wrapping_add(index.wrapping_mul(4))) == volume {
                        found = true;
                        break;
                    }
                    index = index.wrapping_add(1);
                }
                if !found {
                    fn_00a64330(e, Ptr::new(volume_array(this)), Ptr::new(volume_slot));
                }
            }
            read = read.wrapping_add(1);
        }
    });
}

// Translated from 00e9f0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUnionBV::AddViewerStrings` (Xbox PDB): appends `"<name> = UNION_BV"` (a block of
/// `strlen(name) + 0xc` bytes from `00aa1070`, formatted by `NiSprintf`) to the strings array,
/// then for each child formats `"   child <index>"` into a fresh 0xd-byte block and has the
/// child add its own strings under that name (virtual slot +0x2c: the name, the strings). The
/// child is read only below the size (null otherwise, which the loop never reaches).
pub fn ni_union_bv_add_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiUnionBV>,
    name: Ptr,
    strings: Ptr,
) {
    let size = c_string_length(e, name.addr()).wrapping_add(0xc);
    let line = e.call(ALLOCATE, &args![size]).u32();
    e.call(NI_SPRINTF, &args![line, size, UNION_BV_FORMAT, name]);
    add_viewer_string(e, strings, line);
    let mut index = 0u32;
    while index < child_count(e, this) {
        let child_name = e.call(ALLOCATE, &args![0xdu32]).u32();
        e.call(NI_SPRINTF, &args![child_name, 0xdu32, CHILD_FORMAT, index]);
        let volume = if index < child_count(e, this) {
            child(e, this, index)
        } else {
            0
        };
        e.vcall(volume, 0x2c, &args![child_name, strings]);
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_4330, fn_00a64330(Ptr<NiTArray>, Ptr) -> u32),
        entry!(0x00e9_e860, fn_00e9e860(Ptr<NiUnionBV>)),
        entry!(
            0x00e9_e8c0,
            ni_union_bv_operator_(Ptr<NiUnionBV>, Ptr<NiUnionBV>) -> bool
        ),
        entry!(
            0x00e9_e930,
            ni_union_bv_update_world_data(Ptr<NiUnionBV>, Ptr<NiUnionBV>, Ptr)
        ),
        entry!(
            0x00e9_e980,
            fn_00e9e980(f32, Ptr<NiUnionBV>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00e9_e9e0,
            fn_00e9e9e0(f32, Ptr<NiUnionBV>, Ptr, Ptr<NiUnionBV>, Ptr) -> bool
        ),
        entry!(
            0x00e9_ea70,
            fn_00e9ea70(f32, Ptr<NiUnionBV>, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e9_eae0,
            fn_00e9eae0(f32, Ptr<NiUnionBV>, u32, u32, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e9_eb60,
            fn_00e9eb60(
                f32,
                Ptr<NiUnionBV>,
                u32,
                Ptr<NiUnionBV>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> bool
        ),
        entry!(
            0x00e9_ec10,
            fn_00e9ec10(
                f32,
                Ptr<NiUnionBV>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> bool
        ),
        entry!(0x00e9_ec90, fn_00e9ec90(Ptr, u32) -> Ptr),
        entry!(0x00e9_ecc0, ni_union_bv_type(Ptr<NiUnionBV>) -> u32),
        entry!(
            0x00e9_ecd0,
            ni_union_bv_copy(Ptr<NiUnionBV>, Ptr<NiUnionBV>)
        ),
        entry!(0x00e9_ed50, ni_union_bv_save_binary(Ptr<NiUnionBV>, Ptr)),
        entry!(0x00e9_edd0, fn_00e9edd0(Ptr<NiUnionBV>) -> Ptr<NiUnionBV>),
        entry!(
            0x00e9_ee20,
            ni_union_bv_scalar_deleting_destructor(Ptr<NiUnionBV>, u32) -> Ptr<NiUnionBV>
        ),
        entry!(
            0x00e9_ee60,
            fn_00e9ee60(Ptr<NiUnionBV>, Ptr<NiUnionBV>) -> Ptr<NiUnionBV>
        ),
        entry!(
            0x00e9_eef0,
            ni_union_bv_create(Ptr<NiUnionBV>) -> Ptr<NiUnionBV>
        ),
        entry!(
            0x00e9_efb0,
            ni_union_bv_clone(Ptr<NiUnionBV>) -> Ptr<NiUnionBV>
        ),
        entry!(0x00e9_efe0, fn_00e9efe0(Ptr) -> Ptr<NiUnionBV>),
        entry!(0x00e9_f020, ni_union_bv_load_binary(Ptr<NiUnionBV>, Ptr)),
        entry!(
            0x00e9_f0b0,
            ni_union_bv_add_viewer_strings(Ptr<NiUnionBV>, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nicollision::nicollisiondata::{VIEWER_STRINGS_GROW, VIEWER_STRINGS_SET_AT};

    /// An array with room for `max` pointers holding `items`, grow-by 4.
    fn fixture(max: u16, items: &[u32]) -> (Engine, Ptr<NiTArray>, u32) {
        let mut e = Engine::new();
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(max as u32 * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, max);
        e.set(array, NiTArray::m_usSize, items.len() as u16);
        let present = items.iter().filter(|item| **item != 0).count();
        e.set(array, NiTArray::m_usESize, present as u16);
        e.set(array, NiTArray::m_usGrowBy, 4);
        (e, array, base)
    }

    fn value(e: &mut Engine, word: u32) -> u32 {
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, word);
        holder
    }

    #[test]
    fn a_null_value_is_refused() {
        let (mut e, array, _) = fixture(4, &[1]);
        let holder = value(&mut e, 0);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), u32::MAX);
        assert_eq!(e.get(array, NiTArray::m_usESize), 1);
    }

    #[test]
    fn the_first_empty_slot_takes_the_value() {
        let (mut e, array, base) = fixture(4, &[0x10, 0, 0x30]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.mem.u32(base + 4), 0x77);
        assert_eq!(e.get(array, NiTArray::m_usESize), 3);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
    }

    #[test]
    fn a_full_array_with_room_stores_at_the_used_size_through_set_at() {
        let (mut e, array, _) = fixture(4, &[0x10, 0x20]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 2);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ARRAY_SET_SIZE));
        let stores: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_SET_AT).collect();
        assert_eq!(stores.len(), 1);
        assert_eq!(stores[0].1, args![array, 2u32, holder]);
    }

    #[test]
    fn a_full_array_without_room_grows_first() {
        let (mut e, array, _) = fixture(2, &[0x10, 0x20]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 2);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|call| call.0)
            .filter(|address| *address != 0x00a6_4330)
            .collect();
        assert_eq!(order, vec![ARRAY_SET_SIZE, ARRAY_SET_AT]);
        let grows: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_SET_SIZE).collect();
        assert_eq!(grows[0].1, args![array, 6u32]);
    }

    #[test]
    fn an_empty_array_goes_straight_to_set_at() {
        let (mut e, array, _) = fixture(0, &[]);
        let holder = value(&mut e, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_4330, &args![array, holder]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let grows: Vec<_> = log.iter().filter(|call| call.0 == ARRAY_SET_SIZE).collect();
        assert_eq!(grows[0].1, args![array, 4u32]);
    }

    /// The fake vtable every test volume uses: slot `s` calls `SLOT_BASE + s`.
    const CHILD_VTABLE: u32 = 0x0200_0000;
    const SLOT_BASE: u32 = 0x0300_0000;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The argument words of every logged call to `addr`.
    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// An engine with the fake vtable mapped and the call log on.
    fn union_engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..12).map(|i| SLOT_BASE + 4 * i).collect();
        e.put_vtable(CHILD_VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    /// A volume with the fake vtable; its word at +4 is `id` (doubles answer from it).
    fn volume(e: &mut Engine, id: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, CHILD_VTABLE);
        e.mem.set_u32(block + 4, id);
        block
    }

    /// A union with the fake vtable (so its own `Type` goes to `SLOT_BASE + 0xc`) holding
    /// `children` (room for eight).
    fn union(e: &mut Engine, children: &[u32]) -> Ptr<NiUnionBV> {
        let this = e.new_object::<NiUnionBV>();
        e.mem.set_u32(this.addr(), CHILD_VTABLE);
        let base = e.mem.alloc(0x20);
        for (i, volume) in children.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *volume);
        }
        e.set(this, NiUnionBV::m_kBoundingVolume_pBase, base);
        e.set(this, NiUnionBV::m_kBoundingVolume_usMaxSize, 8);
        e.set(
            this,
            NiUnionBV::m_kBoundingVolume_usSize,
            children.len() as u16,
        );
        let present = children.iter().filter(|v| **v != 0).count() as u16;
        e.set(this, NiUnionBV::m_kBoundingVolume_usESize, present);
        e.set(this, NiUnionBV::m_kBoundingVolume_usGrowBy, 1);
        this
    }

    /// A double that answers the word at `this + 4` (the volume's id).
    fn answer_id(e: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        }
    }

    /// A double that answers true when its second word (the volume) has id 2.
    fn second_word_is_id_two(e: &mut Engine, a: &[u32]) -> Ret {
        Ret {
            eax: (e.mem.u32(a[1] + 4) == 2) as u32,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_00e9e860_deletes_every_child_and_empties_the_array() {
        let mut e = union_engine();
        e.register(SLOT_BASE + 8, nothing);
        let first = volume(&mut e, 1);
        let second = volume(&mut e, 2);
        let this = union(&mut e, &[first, 0, second]);
        e.call(0x00e9_e860, &args![this]);
        assert_eq!(
            logged(&e, SLOT_BASE + 8),
            vec![vec![first, 1], vec![second, 1]]
        );
        let base = e.get(this, NiUnionBV::m_kBoundingVolume_pBase);
        assert_eq!([e.mem.u32(base), e.mem.u32(base + 8)], [0, 0]);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usSize), 0);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usESize), 0);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usMaxSize), 8);
    }

    #[test]
    fn operator_equal_needs_a_union_of_the_same_size_with_no_child_unequal() {
        let mut e = union_engine();
        // `Type` answers the id; `operator!=` answers whether the ids differ.
        e.register(SLOT_BASE + 0xc, answer_id);
        e.register(SLOT_BASE + 0x28, |e, a| Ret {
            eax: (e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)) as u32,
            ..Ret::default()
        });
        let a = volume(&mut e, 7);
        let b = volume(&mut e, 8);
        let a2 = volume(&mut e, 7);
        let b2 = volume(&mut e, 8);
        let this = union(&mut e, &[a, b]);
        let other = union(&mut e, &[a2, b2]);
        e.mem.set_u32(other.addr() + 4, 4);
        assert!(e.call(0x00e9_e8c0, &args![this, other]).bool());
        assert_eq!(logged(&e, SLOT_BASE + 0x28), vec![vec![a, a2], vec![b, b2]]);
        // A child differs.
        e.mem.set_u32(b2 + 4, 9);
        assert!(!e.call(0x00e9_e8c0, &args![this, other]).bool());
        // Another size.
        e.mem.set_u32(b2 + 4, 8);
        e.set(other, NiUnionBV::m_kBoundingVolume_usSize, 1);
        assert!(!e.call(0x00e9_e8c0, &args![this, other]).bool());
        // Another type: nothing compared.
        e.set(other, NiUnionBV::m_kBoundingVolume_usSize, 2);
        e.mem.set_u32(other.addr() + 4, 3);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_e8c0, &args![this, other]).bool());
        assert!(logged(&e, SLOT_BASE + 0x28).is_empty());
    }

    #[test]
    fn update_world_data_pairs_each_child_with_the_model_child() {
        let mut e = union_engine();
        e.register(SLOT_BASE + 0x14, nothing);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let ma = volume(&mut e, 3);
        let mb = volume(&mut e, 4);
        let this = union(&mut e, &[a, b]);
        let model = union(&mut e, &[ma, mb]);
        e.call(0x00e9_e930, &args![this, model, 0x7000u32]);
        assert_eq!(
            logged(&e, SLOT_BASE + 0x14),
            vec![vec![a, ma, 0x7000], vec![b, mb, 0x7000]]
        );
    }

    #[test]
    fn fn_00e9e980_stops_at_the_first_child_that_intersects() {
        let mut e = union_engine();
        e.register(VOLUME_TEST_00E9C8C0, second_word_is_id_two);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let c = volume(&mut e, 2);
        let this = union(&mut e, &[a, b, c]);
        let time = f32::from_bits(0x7f80_0001);
        assert!(e
            .call(0x00e9_e980, &args![time, this, 0x10u32, 0x11u32, 0x12u32])
            .bool());
        assert_eq!(e.get(this, NiUnionBV::m_uiWhichObjectIntersect), 1);
        assert_eq!(
            logged(&e, VOLUME_TEST_00E9C8C0),
            vec![
                vec![0x7fc0_0001, a, 0x10, 0x11, 0x12],
                vec![0x7fc0_0001, b, 0x10, 0x11, 0x12]
            ]
        );
        let none = union(&mut e, &[a]);
        e.set(none, NiUnionBV::m_uiWhichObjectIntersect, 9);
        assert!(!e
            .call(0x00e9_e980, &args![1.0f32, none, 0x10u32, 0x11u32, 0x12u32])
            .bool());
        assert_eq!(e.get(none, NiUnionBV::m_uiWhichObjectIntersect), 9);
    }

    #[test]
    fn fn_00e9e9e0_tries_every_pair_and_records_both_indices() {
        let mut e = union_engine();
        // True when the first volume has id 2 and the second id 5.
        e.register(VOLUME_TEST_00E9C8C0, |e, a| Ret {
            eax: (e.mem.u32(a[1] + 4) == 2 && e.mem.u32(a[3] + 4) == 5) as u32,
            ..Ret::default()
        });
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let x = volume(&mut e, 4);
        let y = volume(&mut e, 5);
        let first = union(&mut e, &[a, b]);
        let second = union(&mut e, &[x, y]);
        assert!(e
            .call(0x00e9_e9e0, &args![2.0f32, first, 0x20u32, second, 0x21u32])
            .bool());
        assert_eq!(e.get(first, NiUnionBV::m_uiWhichObjectIntersect), 1);
        assert_eq!(e.get(second, NiUnionBV::m_uiWhichObjectIntersect), 1);
        let two = 2.0f32.to_bits();
        assert_eq!(
            logged(&e, VOLUME_TEST_00E9C8C0),
            vec![
                vec![two, a, 0x20, x, 0x21],
                vec![two, a, 0x20, y, 0x21],
                vec![two, b, 0x20, x, 0x21],
                vec![two, b, 0x20, y, 0x21]
            ]
        );
        let empty = union(&mut e, &[]);
        assert!(!e
            .call(0x00e9_e9e0, &args![2.0f32, first, 0x20u32, empty, 0x21u32])
            .bool());
    }

    #[test]
    fn fn_00e9ea70_forwards_its_seven_words() {
        let mut e = union_engine();
        e.register(VOLUME_TEST_00E9CA10, second_word_is_id_two);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let this = union(&mut e, &[a, b]);
        let words = args![1.5f32, this, 2u32, 3u32, 4u32, 5u32, 6u32];
        assert!(e.call(0x00e9_ea70, &words).bool());
        assert_eq!(e.get(this, NiUnionBV::m_uiWhichObjectIntersect), 1);
        let calls = logged(&e, VOLUME_TEST_00E9CA10);
        assert_eq!(calls[1], args![1.5f32, b, 2u32, 3u32, 4u32, 5u32, 6u32]);
        let other = union(&mut e, &[a]);
        let words = args![1.5f32, other, 2u32, 3u32, 4u32, 5u32, 6u32];
        assert!(!e.call(0x00e9_ea70, &words).bool());
    }

    #[test]
    fn fn_00e9eae0_forwards_its_ten_words() {
        let mut e = union_engine();
        e.register(VOLUME_TEST_00E9C950, second_word_is_id_two);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let this = union(&mut e, &[a, b]);
        let words = args![1.5f32, this, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32];
        assert!(e.call(0x00e9_eae0, &words).bool());
        assert_eq!(e.get(this, NiUnionBV::m_uiWhichObjectIntersect), 1);
        let calls = logged(&e, VOLUME_TEST_00E9C950);
        assert_eq!(
            calls[1],
            args![1.5f32, b, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32]
        );
        let other = union(&mut e, &[a]);
        let words = args![1.5f32, other, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32];
        assert!(!e.call(0x00e9_eae0, &words).bool());
    }

    #[test]
    fn fn_00e9eb60_puts_both_children_in_their_unions_places() {
        let mut e = union_engine();
        e.register(VOLUME_TEST_00E9C950, |e, a| Ret {
            eax: (e.mem.u32(a[1] + 4) == 2 && e.mem.u32(a[3] + 4) == 4) as u32,
            ..Ret::default()
        });
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let x = volume(&mut e, 3);
        let y = volume(&mut e, 4);
        let first = union(&mut e, &[a, b]);
        let second = union(&mut e, &[x, y]);
        let words = args![1.5f32, first, 2u32, second, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32];
        assert!(e.call(0x00e9_eb60, &words).bool());
        assert_eq!(e.get(first, NiUnionBV::m_uiWhichObjectIntersect), 1);
        assert_eq!(e.get(second, NiUnionBV::m_uiWhichObjectIntersect), 1);
        let calls = logged(&e, VOLUME_TEST_00E9C950);
        assert_eq!(calls.len(), 4);
        assert_eq!(
            calls[3],
            args![1.5f32, b, 2u32, y, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32]
        );
        e.mem.set_u32(y + 4, 0);
        assert!(!e.call(0x00e9_eb60, &words).bool());
    }

    #[test]
    fn fn_00e9ec10_forwards_its_twelve_words() {
        let mut e = union_engine();
        e.register(VOLUME_TEST_00E9CA50, second_word_is_id_two);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let this = union(&mut e, &[a, b]);
        let words =
            args![1.5f32, this, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32];
        assert!(e.call(0x00e9_ec10, &words).bool());
        assert_eq!(e.get(this, NiUnionBV::m_uiWhichObjectIntersect), 1);
        let calls = logged(&e, VOLUME_TEST_00E9CA50);
        assert_eq!(
            calls[1],
            args![1.5f32, b, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32]
        );
        let other = union(&mut e, &[a]);
        let words =
            args![1.5f32, other, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32];
        assert!(!e.call(0x00e9_ec10, &words).bool());
    }

    #[test]
    fn fn_00e9ec90_frees_the_elements_and_deletes_on_flag_bit_zero() {
        let mut e = union_engine();
        e.register(FREE, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x4444);
        assert_eq!(e.call(0x00e9_ec90, &args![array, 0u32]).u32(), array);
        assert_eq!(e.mem.u32(array), BOUNDING_VOLUME_ARRAY_VTABLE);
        assert_eq!(logged(&e, FREE), vec![vec![0x4444]]);
        assert!(logged(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00e9_ec90, &args![array, 3u32]);
        assert_eq!(
            logged(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }

    #[test]
    fn type_is_four() {
        let mut e = union_engine();
        assert_eq!(e.call(0x00e9_ecc0, &args![0x1234u32]).u32(), 4);
    }

    #[test]
    fn copy_rebuilds_the_children_from_another_union_only() {
        let mut e = union_engine();
        e.register(SLOT_BASE + 0xc, answer_id);
        e.register(SLOT_BASE + 8, nothing);
        e.register(SLOT_BASE + 0x20, nothing);
        e.register(ARRAY_SET_SIZE, nothing);
        // `SetAt` stores the value its third word points at.
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            Ret::default()
        });
        // `Create` answers a fresh volume with the id plus 100.
        e.register(SLOT_BASE + 0x18, |e, a| {
            let id = e.mem.u32(a[0] + 4);
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, CHILD_VTABLE);
            e.mem.set_u32(block + 4, id + 100);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let old = volume(&mut e, 9);
        let this = union(&mut e, &[old]);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let other = union(&mut e, &[a, b]);
        e.mem.set_u32(other.addr() + 4, 4);
        e.call(0x00e9_ecd0, &args![this, other]);
        assert_eq!(logged(&e, SLOT_BASE + 8), vec![vec![old, 1]]);
        assert_eq!(logged(&e, ARRAY_SET_SIZE), vec![vec![this.addr() + 4, 2]]);
        let copies = logged(&e, SLOT_BASE + 0x20);
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0][1], a);
        assert_eq!(copies[1][1], b);
        let base = e.get(this, NiUnionBV::m_kBoundingVolume_pBase);
        assert_eq!(e.mem.u32(base), copies[0][0]);
        assert_eq!(e.mem.u32(e.mem.u32(base + 4) + 4), 102);
        let stores = logged(&e, ARRAY_SET_AT);
        assert_eq!(stores[1][1], 1);
        // Not a union: nothing happens.
        e.mem.set_u32(other.addr() + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_ecd0, &args![this, other]);
        assert!(logged(&e, SLOT_BASE + 8).is_empty());
        assert!(logged(&e, ARRAY_SET_SIZE).is_empty());
    }

    #[test]
    fn save_binary_writes_the_count_then_each_child() {
        let mut e = union_engine();
        e.register(NI_BOUNDING_VOLUME_SAVE_BINARY, nothing);
        e.register(SLOT_BASE + 4, nothing);
        e.register(0x0500_0000, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(0x0600_0000, value);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        e.map(0x0600_0000, 4);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 0xc, 0x0500_0000);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x250, writer);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let this = union(&mut e, &[a, 0, b]);
        e.call(0x00e9_ed50, &args![this, stream]);
        assert_eq!(
            logged(&e, NI_BOUNDING_VOLUME_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(e.mem.u32(0x0600_0000), 3);
        let write = &logged(&e, 0x0500_0000)[0];
        assert_eq!((write[0], write[2], write[4]), (writer, 4, 1));
        assert_eq!(e.mem.u32(writer + 4), 4);
        assert_eq!(
            logged(&e, SLOT_BASE + 4),
            vec![vec![a, stream], vec![b, stream]]
        );
    }

    #[test]
    fn the_constructor_builds_an_empty_array_with_room_for_two() {
        let mut e = union_engine();
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x8888,
            ..Ret::default()
        });
        let this = e.new_object::<NiUnionBV>();
        e.set(this, NiUnionBV::m_uiWhichObjectIntersect, 5);
        e.set(this, NiUnionBV::m_kBoundingVolume_usSize, 5);
        assert_eq!(e.call(0x00e9_edd0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_UNION_BV_VTABLE);
        assert_eq!(
            e.get(this, NiUnionBV::m_kBoundingVolume_vtable),
            BOUNDING_VOLUME_PRIMITIVE_ARRAY_VTABLE
        );
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_pBase), 0x8888);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usMaxSize), 2);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usSize), 0);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usESize), 0);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usGrowBy), 1);
        assert_eq!(e.get(this, NiUnionBV::m_uiWhichObjectIntersect), 0);
        assert_eq!(logged(&e, ALLOCATE), vec![vec![8]]);
    }

    #[test]
    fn the_scalar_deleting_destructor_empties_frees_and_deletes_on_bit_zero() {
        let mut e = union_engine();
        e.register(SLOT_BASE + 8, nothing);
        e.register(FREE, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        let a = volume(&mut e, 1);
        let this = union(&mut e, &[a]);
        let base = e.get(this, NiUnionBV::m_kBoundingVolume_pBase);
        assert_eq!(e.call(0x00e9_ee20, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(logged(&e, SLOT_BASE + 8), vec![vec![a, 1]]);
        assert_eq!(logged(&e, FREE), vec![vec![base]]);
        assert_eq!(e.mem.u32(this.addr()), NI_BOUNDING_VOLUME_VTABLE);
        assert_eq!(
            e.get(this, NiUnionBV::m_kBoundingVolume_vtable),
            BOUNDING_VOLUME_ARRAY_VTABLE
        );
        assert!(logged(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00e9_ee20, &args![this, 1u32]);
        assert_eq!(
            logged(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this.addr()]]
        );
    }

    #[test]
    fn the_copy_constructor_stores_clones_of_the_non_null_children() {
        let mut e = union_engine();
        e.register(ARRAY_SET_SIZE, nothing);
        e.register(ARRAY_SET_AT, |e, a| Ret {
            eax: e.mem.u32(a[2]),
            ..Ret::default()
        });
        e.register(SLOT_BASE + 0x1c, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4) + 0x1000,
            ..Ret::default()
        });
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let other = union(&mut e, &[a, 0, b]);
        let this = e.new_object::<NiUnionBV>();
        e.set(this, NiUnionBV::m_uiWhichObjectIntersect, 7);
        assert_eq!(e.call(0x00e9_ee60, &args![this, other]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_UNION_BV_VTABLE);
        assert_eq!(
            e.get(this, NiUnionBV::m_kBoundingVolume_vtable),
            BOUNDING_VOLUME_PRIMITIVE_ARRAY_VTABLE
        );
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usGrowBy), 1);
        assert_eq!(e.get(this, NiUnionBV::m_uiWhichObjectIntersect), 7);
        assert_eq!(logged(&e, ARRAY_SET_SIZE), vec![vec![this.addr() + 4, 3]]);
        let stores = logged(&e, ARRAY_SET_AT);
        assert_eq!(stores.len(), 2);
        assert_eq!((stores[0][1], stores[1][1]), (0, 2));
        assert_eq!(logged(&e, SLOT_BASE + 0x1c), vec![vec![a], vec![b]]);
    }

    #[test]
    fn create_fills_a_new_union_with_what_each_child_creates() {
        let mut e = union_engine();
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        // The element block: room for eight, the second and third slots already holding a
        // value.
        e.register(ALLOCATE, |e, _| {
            let block = e.mem.alloc(0x20);
            e.mem.set_u32(block + 4, 0x5555);
            e.mem.set_u32(block + 8, 0x6666);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        // `SetSize` here makes the size 3 with the two values counted.
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 0xa, 3);
            e.mem.set_u16(a[0] + 0xc, 2);
            Ret::default()
        });
        // `Create` answers the id (0 for a null volume).
        e.register(SLOT_BASE + 0x18, answer_id);
        let first = volume(&mut e, 0x11);
        let second = volume(&mut e, 0);
        let third = volume(&mut e, 0x33);
        let fourth = volume(&mut e, 0x44);
        let fifth = volume(&mut e, 0);
        let this = union(&mut e, &[first, second, third, fourth, fifth]);
        let created: Ptr<NiUnionBV> = Ptr::new(e.call(0x00e9_eef0, &args![this]).u32());
        assert_eq!(e.mem.u32(created.addr()), NI_UNION_BV_VTABLE);
        assert_eq!(
            logged(&e, ARRAY_SET_SIZE),
            vec![vec![created.addr() + 4, 5]]
        );
        let base = e.get(created, NiUnionBV::m_kBoundingVolume_pBase);
        let stored: Vec<u32> = (0..5).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(stored, vec![0x11, 0, 0x33, 0x44, 0]);
        // From 2: index 0 a value over empty (+1); 1 null over a value (-1); 2 a value over a
        // value (no change); 3 a value past the size (+1, size 4); 4 null past the size (size 5).
        assert_eq!(e.get(created, NiUnionBV::m_kBoundingVolume_usESize), 3);
        assert_eq!(e.get(created, NiUnionBV::m_kBoundingVolume_usSize), 5);
    }

    #[test]
    fn clone_copies_into_a_new_block_or_answers_null() {
        let mut e = union_engine();
        e.register(ARRAY_SET_SIZE, nothing);
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let this = union(&mut e, &[]);
        let copy = e.call(0x00e9_efb0, &args![this]).u32();
        assert_ne!(copy, 0);
        assert_eq!(e.mem.u32(copy), NI_UNION_BV_VTABLE);
        assert_eq!(logged(&e, ARRAY_SET_SIZE), vec![vec![copy + 4, 0]]);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00e9_efb0, &args![this]).u32(), 0);
    }

    #[test]
    fn fn_00e9efe0_makes_a_union_and_loads_it() {
        let mut e = union_engine();
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.map(NI_UNION_BV_VTABLE & !0xfff, 0x1000);
        e.mem.set_u32(NI_UNION_BV_VTABLE, 0x0500_0010);
        e.register(0x0500_0010, nothing);
        let union = e.call(0x00e9_efe0, &args![0x9000u32]).u32();
        assert_ne!(union, 0);
        assert_eq!(logged(&e, 0x0500_0010), vec![vec![union, 0x9000]]);
    }

    #[test]
    fn load_binary_adds_each_new_volume_once() {
        let mut e = union_engine();
        e.register(NI_BOUNDING_VOLUME_LOAD_BINARY, nothing);
        e.register(ARRAY_SET_SIZE, nothing);
        // `SetAt` stores at the index and grows the size and the non-null count.
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            let present = e.mem.u16(a[0] + 0xc);
            e.mem.set_u16(a[0] + 0xc, present + 1);
            Ret::default()
        });
        // The read gives the count 4.
        e.register(0x0500_0000, |e, a| {
            e.mem.set_u32(a[1], 4);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let reader = e.mem.alloc(0x10);
        e.mem.set_u32(reader + 8, 0x0500_0000);
        let stream = e.mem.alloc(0x400);
        e.mem.set_u32(stream + 0x24c, reader);
        let existing = volume(&mut e, 1);
        let fresh = volume(&mut e, 2);
        // The stream gives the existing child, null, a new volume and the new one again: the
        // answers sit from +0x304, the turn counter at +0x300.
        e.register(NI_BOUNDING_VOLUME_CREATE_FROM_STREAM, |e, a| {
            let turn = e.mem.u32(a[0] + 0x300);
            e.mem.set_u32(a[0] + 0x300, turn + 1);
            Ret {
                eax: e.mem.u32(a[0] + 0x304 + 4 * turn),
                ..Ret::default()
            }
        });
        for (turn, answer) in [existing, 0, fresh, fresh].iter().enumerate() {
            e.mem.set_u32(stream + 0x304 + 4 * turn as u32, *answer);
        }
        let this = union(&mut e, &[existing]);
        e.call(0x00e9_f020, &args![this, stream]);
        assert_eq!(
            logged(&e, NI_BOUNDING_VOLUME_LOAD_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(logged(&e, NI_BOUNDING_VOLUME_CREATE_FROM_STREAM).len(), 4);
        let stores = logged(&e, ARRAY_SET_AT);
        assert_eq!(stores.len(), 1);
        assert_eq!(&stores[0][..2], &[this.addr() + 4, 1]);
        let base = e.get(this, NiUnionBV::m_kBoundingVolume_pBase);
        assert_eq!(e.mem.u32(base + 4), fresh);
        assert_eq!(e.get(this, NiUnionBV::m_kBoundingVolume_usSize), 2);
    }

    #[test]
    fn add_viewer_strings_names_the_union_then_each_child() {
        let mut e = union_engine();
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_SPRINTF, nothing);
        e.register(VIEWER_STRINGS_GROW, nothing);
        e.register(VIEWER_STRINGS_SET_AT, nothing);
        e.register(SLOT_BASE + 0x2c, nothing);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"abc\0");
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        let a = volume(&mut e, 1);
        let b = volume(&mut e, 2);
        let this = union(&mut e, &[a, b]);
        e.call(0x00e9_f0b0, &args![this, name, strings]);
        let lines = logged(&e, NI_SPRINTF);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0][1..], [0xf, UNION_BV_FORMAT, name]);
        assert_eq!(lines[1][1..], [0xd, CHILD_FORMAT, 0]);
        assert_eq!(lines[2][1..], [0xd, CHILD_FORMAT, 1]);
        assert_eq!(logged(&e, VIEWER_STRINGS_SET_AT).len(), 1);
        assert_eq!(
            logged(&e, SLOT_BASE + 0x2c),
            vec![vec![a, lines[1][0], strings], vec![b, lines[2][0], strings]]
        );
    }
}
