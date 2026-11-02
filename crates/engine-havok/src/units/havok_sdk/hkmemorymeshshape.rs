//! `Havok SDK/hkmemorymeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no `hkMemoryMeshShape`; the PC code shows these fields: the vtable at +0,
//! the 16-bit size at +4 and the 16-bit reference count at +6, then three `hkArray`s
//! (`data`, `size`, `capacityAndFlags`, 12 bytes each): the sections (`0x20` bytes an element)
//! at +8, the 16-bit indices at +0x14, the 32-bit indices at +0x20, and an `hkStringPtr` at
//! +0x2c. The top bit of a capacity word (`0x80000000`) says the array does not own its memory;
//! the rest (`0x3fffffff`) is the capacity.

#[allow(unused_imports)]
use crate::prelude::*;

/// The shape's vtable (`010e929c`) and the base class's (`0102e388`, written last by the
/// destructor).
const SHAPE_VTABLE: u32 = 0x010e_929c;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The memory router (`00c85750`): its field `+0x10` is the allocator the arrays are freed
/// through (slot `0x10`, `(pointer, bytes)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The function the destructor runs on the first and second words of every section
/// (`ECX` the word, no stack words; the word is not tested for null first). Unnamed in the
/// engine map (it is in `hkreferencedobject.cpp`).
const RELEASE_OBJECT: u32 = 0x00c9_05b0;
/// `hkStringPtr::~hkStringPtr` (`00cb0c00`, `ECX` the string).
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkMeshPrimitiveUtil::calculateNumIndices` (`00e60cb0`, cdecl `(primitive type, count)`).
const CALCULATE_NUM_INDICES: u32 = 0x00e6_0cb0;
/// The names `calcContentStatistics` gives its chunks: `indices16`, `indices32`, `sections`,
/// and the two the sections' objects are reported under.
const NAME_INDICES16: u32 = 0x010c_fdf8;
const NAME_INDICES32: u32 = 0x010c_fdec;
const NAME_SECTIONS: u32 = 0x010e_9228;
const NAME_FIRST_OBJECT: u32 = 0x010d_c4a0;
const NAME_SECOND_OBJECT: u32 = 0x010c_da78;
/// The top bit of an `hkArray`'s capacity word and the mask of the capacity itself.
const DONT_DEALLOCATE: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00e4a020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryMeshShape::unlockSection` (Xbox PDB), `thiscall` with one stack word: does nothing.
pub fn hk_memory_mesh_shape_unlock_section(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00e4a030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryMeshShape::getNumSections` (Xbox PDB): the size of the sections array (`+0xc`).
pub fn hk_memory_mesh_shape_get_num_sections(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 0xc)
}

// Translated from 00e4a040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryMeshShape::lockSection` (Xbox PDB), `(index, flags, out)`: fills the record `out`
/// from the section `index` (the element of the sections array at `index * 0x20`, `s`). Byte
/// `+0` is the byte at `s + 8`; `+4` the word `s + 0xc`; `+8` the answer of
/// `calculateNumIndices` (`00e60cb0`) for that byte (as a word) and that word; `+0xc` the word
/// `s + 0x18`; `+0x14` the byte `s + 0x10`; `+0x10` the word `s + 0x1c`; `+0x1c` the word at
/// `s` when `flags & 2` is set, else 0; `+0x18` is 0, or the word `s + 0x14` when `flags & 1`
/// is set and the byte `s + 0x10` is not 0; `+0x20` the word `s + 4`; `+0x24` the index.
pub fn hk_memory_mesh_shape_lock_section(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    flags: u32,
    out: Ptr,
) {
    let sections = e.mem.u32(this.addr() + 8);
    let section = (index << 5).wrapping_add(sections);
    let out = out.addr();
    let primitive_type = e.mem.u8(section + 8);
    e.mem.set_u8(out, primitive_type);
    let count = e.mem.u32(section + 0xc);
    e.mem.set_u32(out + 4, count);
    let primitive_type = u32::from(e.mem.u8(section + 8));
    let count = e.mem.u32(section + 0xc);
    let indices = e
        .call(CALCULATE_NUM_INDICES, &args![primitive_type, count])
        .u32();
    e.mem.set_u32(out + 8, indices);
    let word = e.mem.u32(section + 0x18);
    e.mem.set_u32(out + 0xc, word);
    let flag_byte = e.mem.u8(section + 0x10);
    e.mem.set_u8(out + 0x14, flag_byte);
    let word = e.mem.u32(section + 0x1c);
    e.mem.set_u32(out + 0x10, word);
    let first = if flags & 2 != 0 {
        e.mem.u32(section)
    } else {
        0
    };
    e.mem.set_u32(out + 0x1c, first);
    e.mem.set_u32(out + 0x18, 0);
    if flags & 1 != 0 && e.mem.u8(section + 0x10) != 0 {
        let word = e.mem.u32(section + 0x14);
        e.mem.set_u32(out + 0x18, word);
    }
    let word = e.mem.u32(section + 4);
    e.mem.set_u32(out + 0x24, index);
    e.mem.set_u32(out + 0x20, word);
}

/// One array chunk of `calcContentStatistics`: unless the array does not own its memory and
/// the collector's byte `+0x10` is 0, asks `getAllocatedSize` (`00d06f80`) about the array's
/// data and calls slot `0x14` of the collector with `(3, name, data, used bytes, allocated)`.
fn report_array(
    e: &mut Engine,
    collector: Ptr,
    name: u32,
    data: u32,
    capacity: u32,
    size: u32,
    element_shift: u32,
) {
    if capacity as i32 >= 0 || e.mem.u8(collector.addr() + 0x10) != 0 {
        let bytes = (capacity & CAPACITY_MASK) << element_shift;
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, name, data, size << element_shift, allocated],
        );
    }
}

// Translated from 00e4a0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryMeshShape::calcContentStatistics` (Xbox PDB), `(collector, class)` (the class word
/// is not read). Reports the 16-bit indices, the 32-bit indices and the sections as chunks
/// (slot `0x14` of the collector, `(3, name, data, size * element size, allocated bytes)`;
/// each unless the array does not own its memory and the collector's byte `+0x10` is 0), then
/// for each section (the count read once, the array read again for each) calls slot `0xc` of
/// the collector with the name at `010dc4a0`, the section's first word and 0, and again with
/// the name at `010cda78`, the section's second word and 0.
pub fn hk_memory_mesh_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let base = this.addr();
    let (data, size, capacity) = (
        e.mem.u32(base + 0x14),
        e.mem.u32(base + 0x18),
        e.mem.u32(base + 0x1c),
    );
    report_array(e, collector, NAME_INDICES16, data, capacity, size, 1);
    let (data, size, capacity) = (
        e.mem.u32(base + 0x20),
        e.mem.u32(base + 0x24),
        e.mem.u32(base + 0x28),
    );
    report_array(e, collector, NAME_INDICES32, data, capacity, size, 2);
    let (data, size, capacity) = (
        e.mem.u32(base + 8),
        e.mem.u32(base + 0xc),
        e.mem.u32(base + 0x10),
    );
    report_array(e, collector, NAME_SECTIONS, data, capacity, size, 5);
    let count = e.mem.i32(base + 0xc);
    if count > 0 {
        let mut offset = 0u32;
        for _ in 0..count {
            let section = e.mem.u32(base + 8).wrapping_add(offset);
            let first = e.mem.u32(section);
            e.vcall(
                collector.addr(),
                0xc,
                &args![NAME_FIRST_OBJECT, first, 0u32],
            );
            let second = e.mem.u32(section + 4);
            e.vcall(
                collector.addr(),
                0xc,
                &args![NAME_SECOND_OBJECT, second, 0u32],
            );
            offset = offset.wrapping_add(0x20);
        }
    }
}

/// Frees one of the shape's arrays the way the destructor does: asks the memory router
/// (`00c85750`) for its allocator (`+0x10`), clears the array's size word and, unless the array
/// does not own its memory, frees its data through slot `0x10` with the bytes the capacity
/// covers; then clears the data pointer and marks the capacity as not owning.
fn free_array(e: &mut Engine, this: Ptr, array: u32, element_shift: u32) {
    let base = this.addr() + array;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 8);
    e.mem.set_u32(base + 4, 0);
    if capacity & DONT_DEALLOCATE == 0 {
        let bytes = (capacity & CAPACITY_MASK) << element_shift;
        let data = e.mem.u32(base);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 8, DONT_DEALLOCATE);
}

// Translated from 00e4a460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryMeshShape::~hkMemoryMeshShape` (Xbox PDB): sets the shape's vtable and, for each
/// section (the count read once, the array read again for each; none when the count is not
/// above 0), runs `00c905b0` on the section's first word (without a null test) and on its second
/// word when that is not null. Then runs `hkStringPtr::~hkStringPtr` (`00cb0c00`) on the string
/// at `+0x2c` and empties the three arrays, 32-bit indices first, then 16-bit indices, then
/// sections: each takes the memory router's allocator (`00c85750`, `+0x10`), sets its size
/// word to 0, frees its data (slot `0x10`, `(data, capacity * element size)`) unless the top
/// bit of its capacity word is set, then sets the data pointer to 0 and the capacity word to
/// `0x80000000`. Ends by setting the base class's vtable (`0102e388`).
pub fn hk_memory_mesh_shape_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.mem.i32(base + 0xc);
    e.mem.set_u32(base, SHAPE_VTABLE);
    if count > 0 {
        let mut offset = 0u32;
        for _ in 0..count {
            let section = e.mem.u32(base + 8).wrapping_add(offset);
            let first = e.mem.u32(section);
            e.call(RELEASE_OBJECT, &args![first]);
            let second = e.mem.u32(section + 4);
            if second != 0 {
                e.call(RELEASE_OBJECT, &args![second]);
            }
            offset = offset.wrapping_add(0x20);
        }
    }
    e.call(STRING_PTR_DESTRUCTOR, &args![base + 0x2c]);
    free_array(e, this, 0x20, 2);
    free_array(e, this, 0x14, 1);
    free_array(e, this, 8, 5);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4a020, hk_memory_mesh_shape_unlock_section(Ptr, u32)),
        entry!(0x00e4a030, hk_memory_mesh_shape_get_num_sections(Ptr) -> i32),
        entry!(
            0x00e4a040,
            hk_memory_mesh_shape_lock_section(Ptr, u32, u32, Ptr)
        ),
        entry!(
            0x00e4a0c0,
            hk_memory_mesh_shape_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00e4a460, hk_memory_mesh_shape_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    /// A shape with two sections (objects 0x1000/0x1001 and 0/0x1003) and the three arrays.
    fn shape(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        let sections = e.mem.alloc(0x40);
        for (i, (first, second)) in [(0x1000u32, 0x1001u32), (0, 0x1003)]
            .into_iter()
            .enumerate()
        {
            let section = sections + 0x20 * i as u32;
            e.mem.set_u32(section, first);
            e.mem.set_u32(section + 4, second);
            e.mem.set_u8(section + 8, 3);
            e.mem.set_u32(section + 0xc, 5 + i as u32);
            e.mem.set_u8(section + 0x10, 1);
            e.mem.set_u32(section + 0x14, 0x2000 + i as u32);
            e.mem.set_u32(section + 0x18, 0x3000 + i as u32);
            e.mem.set_u32(section + 0x1c, 0x4000 + i as u32);
        }
        e.mem.set_u32(this + 8, sections);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 4);
        let indices16 = e.mem.alloc(16);
        e.mem.set_u32(this + 0x14, indices16);
        e.mem.set_u32(this + 0x18, 6);
        e.mem.set_u32(this + 0x1c, 8);
        let indices32 = e.mem.alloc(32);
        e.mem.set_u32(this + 0x20, indices32);
        e.mem.set_u32(this + 0x24, 3);
        e.mem.set_u32(this + 0x28, 0x10);
        this
    }

    #[test]
    fn unlock_does_nothing_and_get_num_sections_reads_the_size() {
        let mut e = Engine::new();
        let this = shape(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00e4_a020, &args![this, 7u32]);
        assert_eq!(e.call_log.clone().unwrap().len(), 1);
        assert_eq!(e.call(0x00e4_a030, &args![this]).i32(), 2);
    }

    #[test]
    fn lock_section_fills_the_record_from_the_section() {
        let mut e = Engine::new();
        let this = shape(&mut e);
        e.register(CALCULATE_NUM_INDICES, |_, a| Ret {
            eax: a[0] * 100 + a[1],
            ..Ret::default()
        });
        let out = e.mem.alloc(0x28);
        e.mem.write(out, &[0xcd; 0x28]);
        let sections = e.mem.u32(this + 8);
        e.call(0x00e4_a040, &args![this, 0u32, 3u32, out]);
        // the two bytes are written alone: the rest of their words is untouched
        assert_eq!(e.mem.u8(out), 3);
        assert_eq!(e.mem.u8(out + 1), 0xcd);
        assert_eq!(e.mem.u32(out + 4), 5);
        assert_eq!(e.mem.u32(out + 8), 305);
        assert_eq!(e.mem.u32(out + 0xc), 0x3000);
        assert_eq!(e.mem.u32(out + 0x10), 0x4000);
        assert_eq!(e.mem.u8(out + 0x14), 1);
        assert_eq!(e.mem.u8(out + 0x15), 0xcd);
        // flags 3: the first word of the section (bit 1) and the +0x14 word (bit 0, byte set)
        assert_eq!(e.mem.u32(out + 0x1c), 0x1000);
        assert_eq!(e.mem.u32(out + 0x18), 0x2000);
        assert_eq!(e.mem.u32(out + 0x20), 0x1001);
        assert_eq!(e.mem.u32(out + 0x24), 0);
        // the second section, no flags: both are 0
        e.call(0x00e4_a040, &args![this, 1u32, 0u32, out]);
        assert_eq!(e.mem.u32(out + 0x1c), 0);
        assert_eq!(e.mem.u32(out + 0x18), 0);
        assert_eq!(e.mem.u32(out + 8), 306);
        assert_eq!(e.mem.u32(out + 0x20), 0x1003);
        assert_eq!(e.mem.u32(out + 0x24), 1);
        // bit 0 with the section's byte clear gives 0
        e.mem.set_u8(sections + 0x10, 0);
        e.call(0x00e4_a040, &args![this, 0u32, 1u32, out]);
        assert_eq!(e.mem.u32(out + 0x18), 0);
        assert_eq!(e.mem.u8(out + 0x14), 0);
    }

    /// A collector whose slots `0xc` and `0x14` do nothing and whose allocated size for
    /// `(pointer, bytes)` is `bytes + 1`.
    fn install_collector(e: &mut Engine) -> u32 {
        e.put_vtable(
            0x4400_0000,
            &[0, 0, 0, 0x4500_000c, 0x4500_0010, 0x4500_0014],
        );
        let collector = e.mem.alloc(0x14);
        e.mem.set_u32(collector, 0x4400_0000);
        e.register(GET_ALLOCATED_SIZE, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(0x4500_000c, |_, _| Ret::default());
        e.register(0x4500_0014, |_, _| Ret::default());
        collector
    }

    #[test]
    fn statistics_report_each_owned_array_and_every_section_object() {
        let mut e = Engine::new();
        let this = shape(&mut e);
        let collector = install_collector(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00e4_a0c0, &args![this, collector, 0u32]);
        let indices16 = e.mem.u32(this + 0x14);
        let indices32 = e.mem.u32(this + 0x20);
        let sections = e.mem.u32(this + 8);
        assert_eq!(
            calls_to(&e, 0x4500_0014),
            vec![
                vec![collector, 3, NAME_INDICES16, indices16, 12, 17],
                vec![collector, 3, NAME_INDICES32, indices32, 12, 65],
                vec![collector, 3, NAME_SECTIONS, sections, 64, 129],
            ]
        );
        assert_eq!(
            calls_to(&e, GET_ALLOCATED_SIZE),
            vec![
                vec![indices16, 16],
                vec![indices32, 64],
                vec![sections, 128]
            ]
        );
        assert_eq!(
            calls_to(&e, 0x4500_000c),
            vec![
                vec![collector, NAME_FIRST_OBJECT, 0x1000, 0],
                vec![collector, NAME_SECOND_OBJECT, 0x1001, 0],
                vec![collector, NAME_FIRST_OBJECT, 0, 0],
                vec![collector, NAME_SECOND_OBJECT, 0x1003, 0],
            ]
        );
    }

    #[test]
    fn statistics_skip_borrowed_arrays_unless_the_collector_wants_them() {
        let mut e = Engine::new();
        let this = shape(&mut e);
        let collector = install_collector(&mut e);
        e.mem.set_u32(this + 0x1c, 0x8000_0008);
        e.mem.set_u32(this + 0x28, 0x8000_0010);
        e.call_log = Some(vec![]);
        e.call(0x00e4_a0c0, &args![this, collector, 0u32]);
        assert_eq!(calls_to(&e, 0x4500_0014).len(), 1);
        // the collector asks for them: they are reported with the capacity bits only
        e.mem.set_u8(collector + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e4_a0c0, &args![this, collector, 0u32]);
        let sizes = calls_to(&e, GET_ALLOCATED_SIZE);
        assert_eq!(sizes.len(), 3);
        assert_eq!(sizes[0][1], 16);
        // an empty section array reports no objects
        e.mem.set_u32(this + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e4_a0c0, &args![this, collector, 0u32]);
        assert!(calls_to(&e, 0x4500_000c).is_empty());
    }

    #[test]
    fn the_destructor_releases_objects_and_frees_the_owned_arrays() {
        let mut e = Engine::new();
        let this = shape(&mut e);
        e.put_vtable(0x4400_0000, &[0, 0, 0, 0, 0x4500_0010]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_0000);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x4500_0010, |_, _| Ret::default());
        e.register(RELEASE_OBJECT, |_, _| Ret::default());
        e.register(STRING_PTR_DESTRUCTOR, |_, _| Ret::default());
        let indices32 = e.mem.u32(this + 0x20);
        let sections = e.mem.u32(this + 8);
        // the 16-bit indices are borrowed: not freed
        e.mem.set_u32(this + 0x1c, 0x8000_0008);
        e.call_log = Some(vec![]);
        e.call(0x00e4_a460, &args![this]);
        assert_eq!(
            calls_to(&e, RELEASE_OBJECT),
            vec![vec![0x1000], vec![0x1001], vec![0], vec![0x1003]]
        );
        assert_eq!(calls_to(&e, STRING_PTR_DESTRUCTOR), vec![vec![this + 0x2c]]);
        // the 32-bit indices, then the sections, with the bytes their capacities cover
        assert_eq!(
            calls_to(&e, 0x4500_0010),
            vec![
                vec![allocator, indices32, 0x40],
                vec![allocator, sections, 0x80]
            ]
        );
        for array in [8, 0x14, 0x20] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}
