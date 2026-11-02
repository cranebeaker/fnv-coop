//! `Havok SDK/run_00e4aaa0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`): cdecl, `(allocator, array, count, element size)`.
const RESERVE: u32 = 0x00c9_08c0;
/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`): cdecl, `(allocator, array, element size)`.
const RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkMemUtil::memMove` (Xbox PDB, `00d2cc70`): cdecl, `(destination, source, byte count)`.
const MEM_MOVE: u32 = 0x00d2_cc70;
/// `00e61530` (`hkxMeshSection::appendGeometry`, Xbox PDB name): thiscall on the section,
/// `(geometry, section index)`, `RET 8`.
const APPEND_GEOMETRY: u32 = 0x00e6_1530;
/// `00d5a4c0`: thiscall on a 16-byte element array, `(allocator, index, items, count)`, `RET 0x10`.
const INSERT_VERTICES: u32 = 0x00d5_a4c0;
/// `00e4ae00`: thiscall on a 16-byte element array, `(allocator, index, items, count)`, `RET 0x10`.
const INSERT_TRIANGLES: u32 = 0x00e4_ae00;
/// `hkGeometry::~hkGeometry` (Xbox PDB, `00cce530`): thiscall, `this` only.
const GEOMETRY_DESTRUCTOR: u32 = 0x00cc_e530;
/// `00e61480` (`hkxMeshSection::collectVertexPositions`, Xbox PDB name): thiscall on the
/// section, one word, `RET 4`.
const COLLECT_VERTEX_POSITIONS: u32 = 0x00e6_1480;

/// The heap allocator object of the memory router (`00c85750`, then `+0x10`).
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    e.mem.u32(router.wrapping_add(0x10))
}

// Translated from 00e4aaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x14`: replaces `remove` elements of 16 bytes at `index` of the `hkArray` at
/// `this` (`data`, `size`, `capacity | flags`) by the `insert` elements at `items`. The new size
/// is `size - remove + insert`; when the capacity bits are below it the array is reserved
/// (`00c908c0`, with the larger of twice the capacity and the new size). The elements after the
/// replaced ones (`size - index - remove`) are moved with `hkMemUtil::memMove` (`00d2cc70`) to
/// `index + insert`, then the new elements are copied in and the size stored.
pub fn fn_00e4aaa0(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    remove: i32,
    items: u32,
    insert: i32,
) {
    let this = this.addr();
    let size = e.mem.i32(this.wrapping_add(4));
    let new_size = size.wrapping_sub(remove).wrapping_add(insert);
    let tail = size.wrapping_sub(index).wrapping_sub(remove);
    let capacity = (e.mem.u32(this.wrapping_add(8)) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(RESERVE, &args![allocator, this, wanted, 0x10u32]);
    }
    let data = e.mem.u32(this);
    let source = (remove.wrapping_add(index) as u32)
        .wrapping_shl(4)
        .wrapping_add(data);
    let destination = (index.wrapping_add(insert) as u32)
        .wrapping_shl(4)
        .wrapping_add(data);
    e.call(
        MEM_MOVE,
        &args![destination, source, (tail as u32).wrapping_shl(4)],
    );
    let mut cursor = (index as u32).wrapping_shl(4).wrapping_add(e.mem.u32(this));
    for element in 0..insert.max(0) as u32 {
        let from = items.wrapping_add(16 * element);
        for word in 0..4u32 {
            let value = e.mem.u32(from.wrapping_add(4 * word));
            e.mem.set_u32(cursor.wrapping_add(4 * word), value);
        }
        cursor = cursor.wrapping_add(16);
    }
    e.mem.set_i32(this.wrapping_add(4), new_size);
}

// Translated from 00e4ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: runs `00e61480` (collect vertex positions) with `argument` on each of the
/// `[+0xc]` sections of the pointer array at `*(this + 8)`, in order.
pub fn fn_00e4ab50(e: &mut Engine, this: Ptr, argument: u32) {
    let this = this.addr();
    let mut index = 0u32;
    while (index as i32) < e.mem.i32(this.wrapping_add(0xc)) {
        let sections = e.mem.u32(this.wrapping_add(8));
        let section = e.mem.u32(sections.wrapping_add(index.wrapping_mul(4)));
        e.call(COLLECT_VERTEX_POSITIONS, &args![section, argument]);
        index += 1;
    }
}

/// The section id of `key` in the `hkArray` `ids` (`data`, `size`, `capacity | flags`), adding the
/// key at the end (growing through `00c90940` when full) when it is not there.
fn section_id_of(e: &mut Engine, ids: u32, key: u32) -> u32 {
    let size = e.mem.i32(ids.wrapping_add(4));
    if size > 0 {
        let data = e.mem.u32(ids);
        for index in 0..size as u32 {
            if e.mem.u32(data.wrapping_add(4 * index)) == key {
                return index;
            }
        }
    }
    let allocator = heap_allocator(e);
    let capacity = e.mem.u32(ids.wrapping_add(8)) & 0x3fff_ffff;
    if e.mem.u32(ids.wrapping_add(4)) == capacity {
        e.call(RESERVE_MORE, &args![allocator, ids, 4u32]);
    }
    let count = e.mem.u32(ids.wrapping_add(4));
    let data = e.mem.u32(ids);
    e.mem.set_u32(data.wrapping_add(count.wrapping_mul(4)), key);
    let count = e.mem.u32(ids.wrapping_add(4));
    e.mem.set_u32(ids.wrapping_add(4), count.wrapping_add(1));
    size as u32
}

// Translated from 00e4ab80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: appends the geometry of every section of this scene to `target`. `ids` is an
/// optional `hkArray` of section keys (the word at `+0x18` of each section): the section's index
/// in it (added when missing) is passed to the geometry call, `-1` without one. For each of the
/// `[+0xc]` sections of the pointer array at `*(this + 8)`, an `hkGeometry` on the stack
/// (vertex `hkArray` at `+0`, triangle `hkArray` at `+0xc`, both empty and unowned) is filled by
/// the section (`00e61530`, with the geometry and the id), every triangle's three vertex indices
/// (words 0 to 2 of each 16-byte triangle) get `target[+4]` added (the vertex count so far), the
/// vertices are appended to `target` (`00d5a4c0`, at index `target[+4]`) and the triangles to the
/// array at `target + 0xc` (`00e4ae00`, at index `target[+0x10]`), and the geometry is destroyed
/// (`00cce530`).
pub fn fn_00e4ab80(e: &mut Engine, this: Ptr, target: u32, ids: u32) {
    let this = this.addr();
    let mut section_id = u32::MAX;
    if e.mem.i32(this.wrapping_add(0xc)) <= 0 {
        return;
    }
    let mut index = 0i32;
    loop {
        let offset = e.mem.u32(target.wrapping_add(4));
        if ids != 0 {
            let sections = e.mem.u32(this.wrapping_add(8));
            let section = e
                .mem
                .u32(sections.wrapping_add((index as u32).wrapping_mul(4)));
            let key = e.mem.u32(section.wrapping_add(0x18));
            section_id = section_id_of(e, ids, key);
        }
        e.with_stack(0x20, |e, geometry| {
            let geometry = geometry.addr();
            for array in [geometry, geometry.wrapping_add(0xc)] {
                e.mem.set_u32(array, 0);
                e.mem.set_u32(array.wrapping_add(4), 0);
                e.mem.set_u32(array.wrapping_add(8), 0x8000_0000);
            }
            let sections = e.mem.u32(this.wrapping_add(8));
            let section = e
                .mem
                .u32(sections.wrapping_add((index as u32).wrapping_mul(4)));
            e.call(APPEND_GEOMETRY, &args![section, geometry, section_id]);
            let triangles = geometry.wrapping_add(0xc);
            let mut triangle = 0u32;
            while (triangle as i32) < e.mem.i32(triangles.wrapping_add(4)) {
                for word in [0u32, 4, 8] {
                    let data = e.mem.u32(triangles);
                    let at = data
                        .wrapping_add(triangle.wrapping_mul(16))
                        .wrapping_add(word);
                    let value = e.mem.u32(at).wrapping_add(offset);
                    e.mem.set_u32(at, value);
                }
                triangle += 1;
            }
            let vertex_count = e.mem.u32(geometry.wrapping_add(4));
            let vertex_data = e.mem.u32(geometry);
            let allocator = heap_allocator(e);
            e.call(
                INSERT_VERTICES,
                &args![target, allocator, offset, vertex_data, vertex_count],
            );
            let triangle_count = e.mem.u32(triangles.wrapping_add(4));
            let triangle_data = e.mem.u32(triangles);
            let at_end = e.mem.u32(target.wrapping_add(0x10));
            let allocator = heap_allocator(e);
            e.call(
                INSERT_TRIANGLES,
                &args![
                    target.wrapping_add(0xc),
                    allocator,
                    at_end,
                    triangle_data,
                    triangle_count
                ],
            );
            e.call(GEOMETRY_DESTRUCTOR, &args![geometry]);
        });
        index += 1;
        if index >= e.mem.i32(this.wrapping_add(0xc)) {
            break;
        }
    }
}

// Translated from 00e4ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4` (the stack word is not read): stores the vtable `010ec1dc` and returns
/// `this`.
pub fn fn_00e4ad00(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let this = this.addr();
    e.mem.set_u32(this, 0x010e_c1dc);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_aaa0, fn_00e4aaa0(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00e4_ab50, fn_00e4ab50(Ptr, u32)),
        entry!(0x00e4_ab80, fn_00e4ab80(Ptr, u32, u32)),
        entry!(0x00e4_ad00, fn_00e4ad00(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e.mem.set_u32(0x0126_0010, 0x4444);
        e
    }

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let address = *address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    /// An `hkArray` of 16-byte elements whose element `k` has the words `k * 10 + 1 ..= 4`.
    fn array16(e: &mut Engine, size: u32, capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(16 * capacity.max(size) + 16);
        for element in 0..size {
            for word in 0..4 {
                e.mem
                    .set_u32(data + 16 * element + 4 * word, element * 10 + word + 1);
            }
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    fn element_words(e: &Engine, array: u32, element: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..4)
            .map(|word| e.mem.u32(data + 16 * element + 4 * word))
            .collect()
    }

    fn mem_move(e: &mut Engine) {
        e.register(MEM_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    #[test]
    fn insert_shifts_the_tail_and_copies_the_new_elements() {
        let mut e = engine();
        mem_move(&mut e);
        let reserved = record(&mut e, &[RESERVE]);
        let array = array16(&mut e, 3, 8);
        let items = array16(&mut e, 2, 2);
        let new_items = e.mem.u32(items);
        // Insert two elements at index 1, removing none.
        e.call(
            0x00e4_aaa0,
            &args![array, 0x99u32, 1i32, 0i32, new_items, 2i32],
        );
        assert_eq!(e.mem.u32(array + 4), 5);
        assert_eq!(element_words(&e, array, 0), vec![1, 2, 3, 4]);
        assert_eq!(element_words(&e, array, 1), vec![1, 2, 3, 4]);
        assert_eq!(element_words(&e, array, 2), vec![11, 12, 13, 14]);
        assert_eq!(element_words(&e, array, 3), vec![11, 12, 13, 14]);
        assert_eq!(element_words(&e, array, 4), vec![21, 22, 23, 24]);
        assert!(reserved.borrow().is_empty());
    }

    #[test]
    fn replacing_removes_the_old_elements_and_grows_when_needed() {
        let mut e = engine();
        mem_move(&mut e);
        let reserved = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&reserved);
        e.register_double(RESERVE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let data = e.mem.alloc(16 * a[2] + 16);
            let old = e.mem.u32(a[1]);
            let bytes = e.mem.bytes(old, 48);
            e.mem.write(data, &bytes);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let array = array16(&mut e, 3, 3);
        let items = array16(&mut e, 3, 3);
        let new_items = e.mem.u32(items);
        // Replace element 1 by three elements: the new size 5 exceeds the capacity 3.
        e.call(
            0x00e4_aaa0,
            &args![array, 0x99u32, 1i32, 1i32, new_items, 3i32],
        );
        assert_eq!(*reserved.borrow(), vec![vec![0x99, array, 6, 0x10]]);
        assert_eq!(e.mem.u32(array + 4), 5);
        assert_eq!(element_words(&e, array, 0), vec![1, 2, 3, 4]);
        assert_eq!(element_words(&e, array, 1), vec![1, 2, 3, 4]);
        assert_eq!(element_words(&e, array, 3), vec![21, 22, 23, 24]);
        assert_eq!(element_words(&e, array, 4), vec![21, 22, 23, 24]);
    }

    #[test]
    fn removing_without_insert_only_moves_the_tail() {
        let mut e = engine();
        mem_move(&mut e);
        let array = array16(&mut e, 3, 3);
        e.call(0x00e4_aaa0, &args![array, 0x99u32, 0i32, 1i32, 0u32, 0i32]);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(element_words(&e, array, 0), vec![11, 12, 13, 14]);
        assert_eq!(element_words(&e, array, 1), vec![21, 22, 23, 24]);
    }

    #[test]
    fn every_section_gets_the_collect_call_with_the_argument() {
        let mut e = engine();
        let calls = record(&mut e, &[COLLECT_VERTEX_POSITIONS]);
        let this = e.mem.alloc(0x20);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 0x1000);
        e.mem.set_u32(list + 4, 0x2000);
        e.mem.set_u32(this + 8, list);
        e.mem.set_u32(this + 0xc, 2);
        e.call(0x00e4_ab50, &args![this, 0x77u32]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (COLLECT_VERTEX_POSITIONS, vec![0x1000, 0x77]),
                (COLLECT_VERTEX_POSITIONS, vec![0x2000, 0x77])
            ]
        );
    }

    /// A scene with `keys.len()` sections (the key at `+0x18`), the append double giving each
    /// one vertex array of one vertex and one triangle `(0, 1, 2, material)`.
    fn scene(e: &mut Engine, keys: &[u32]) -> u32 {
        let this = e.mem.alloc(0x20);
        let list = e.mem.alloc(4 * keys.len() as u32 + 4);
        for (index, key) in keys.iter().enumerate() {
            let section = e.mem.alloc(0x30);
            e.mem.set_u32(section + 0x18, *key);
            e.mem.set_u32(list + 4 * index as u32, section);
        }
        e.mem.set_u32(this + 8, list);
        e.mem.set_u32(this + 0xc, keys.len() as u32);
        this
    }

    fn geometry_double(e: &mut Engine) -> Calls {
        let calls = record(e, &[GEOMETRY_DESTRUCTOR]);
        let log = Rc::clone(&calls);
        e.register_double(APPEND_GEOMETRY, move |e, a| {
            log.borrow_mut().push((APPEND_GEOMETRY, a.to_vec()));
            let vertices = e.mem.alloc(16);
            let triangles = e.mem.alloc(16);
            for (offset, value) in [(0u32, 0u32), (4, 1), (8, 2), (12, 7)] {
                e.mem.set_u32(triangles + offset, value);
            }
            e.mem.set_u32(a[1], vertices);
            e.mem.set_u32(a[1] + 4, 1);
            e.mem.set_u32(a[1] + 0xc, triangles);
            e.mem.set_u32(a[1] + 0x10, 1);
            Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(INSERT_VERTICES, move |_, a| {
            log.borrow_mut().push((INSERT_VERTICES, a.to_vec()));
            Ret::default()
        });
        let log = Rc::clone(&calls);
        e.register_double(INSERT_TRIANGLES, move |e, a| {
            let words: Vec<u32> = (0..4).map(|word| e.mem.u32(a[3] + 4 * word)).collect();
            let mut seen = a.to_vec();
            seen.extend(words);
            log.borrow_mut().push((INSERT_TRIANGLES, seen));
            Ret::default()
        });
        calls
    }

    #[test]
    fn geometry_is_appended_with_the_vertex_offset_and_section_ids() {
        let mut e = engine();
        let calls = geometry_double(&mut e);
        let this = scene(&mut e, &[50, 60]);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(target + 4, 10);
        e.mem.set_u32(target + 0x10, 3);
        let ids = e.mem.alloc(12);
        let data = e.mem.alloc(16);
        e.mem.set_u32(ids, data);
        e.mem.set_u32(ids + 8, 4);
        e.mem.set_u32(data, 60);
        e.mem.set_u32(ids + 4, 1);
        e.call(0x00e4_ab80, &args![this, target, ids]);
        let calls = calls.borrow();
        let appended: Vec<&(u32, Vec<u32>)> =
            calls.iter().filter(|c| c.0 == APPEND_GEOMETRY).collect();
        // Key 50 is not in the id array and is added at index 1; key 60 is at index 0.
        assert_eq!(appended[0].1[2], 1);
        assert_eq!(appended[1].1[2], 0);
        assert_eq!(e.mem.u32(ids + 4), 2);
        assert_eq!(e.mem.u32(data + 4), 50);
        // The triangle indices got the vertex count (10) added; the double does not append, so
        // the count stays 10 for both sections.
        let triangles: Vec<&(u32, Vec<u32>)> =
            calls.iter().filter(|c| c.0 == INSERT_TRIANGLES).collect();
        assert_eq!(triangles.len(), 2);
        assert_eq!(triangles[0].1[0], target + 0xc);
        assert_eq!(triangles[0].1[1], 0x4444);
        assert_eq!(triangles[0].1[2], 3);
        assert_eq!(&triangles[0].1[5..], &[10, 11, 12, 7]);
        let vertices: Vec<&(u32, Vec<u32>)> =
            calls.iter().filter(|c| c.0 == INSERT_VERTICES).collect();
        assert_eq!(vertices[0].1[0], target);
        assert_eq!(vertices[0].1[2], 10);
        assert_eq!(vertices[0].1[4], 1);
        assert_eq!(
            calls.iter().filter(|c| c.0 == GEOMETRY_DESTRUCTOR).count(),
            2
        );
    }

    #[test]
    fn without_ids_the_section_id_is_minus_one_and_no_sections_do_nothing() {
        let mut e = engine();
        let calls = geometry_double(&mut e);
        let this = scene(&mut e, &[50]);
        let target = e.mem.alloc(0x20);
        e.call(0x00e4_ab80, &args![this, target, 0u32]);
        let appended: Vec<u32> = calls
            .borrow()
            .iter()
            .filter(|c| c.0 == APPEND_GEOMETRY)
            .map(|c| c.1[2])
            .collect();
        assert_eq!(appended, vec![u32::MAX]);
        let empty = scene(&mut e, &[]);
        let before = calls.borrow().len();
        e.call(0x00e4_ab80, &args![empty, target, 0u32]);
        assert_eq!(calls.borrow().len(), before);
    }

    #[test]
    fn full_id_array_grows_before_the_key_is_added() {
        let mut e = engine();
        geometry_double(&mut e);
        let grown = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&grown);
        e.register_double(RESERVE_MORE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let data = e.mem.alloc(32);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let this = scene(&mut e, &[50]);
        let target = e.mem.alloc(0x20);
        let ids = e.mem.alloc(12);
        e.call(0x00e4_ab80, &args![this, target, ids]);
        assert_eq!(*grown.borrow(), vec![vec![0x4444, ids, 4]]);
        assert_eq!(e.mem.u32(ids + 4), 1);
        let data = e.mem.u32(ids);
        assert_eq!(e.mem.u32(data), 50);
    }

    #[test]
    fn vtable_only_function_returns_this() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00e4_ad00, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_c1dc);
    }
}
