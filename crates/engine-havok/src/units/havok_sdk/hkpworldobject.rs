//! `Havok SDK/hkpworldobject.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable pointer `hkpWorldObject` stores at +0.
const WORLD_OBJECT_VTABLE: u32 = 0x010c_6f64;
/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`, cdecl `(allocator, array, elementSize)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject::addReference` (`00c90510`, `this`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkpCollidable::hkpCollidable` (Xbox PDB, `00cd2d80`, `this`, `RET 0xC`: three words).
const COLLIDABLE_CONSTRUCT: u32 = 0x00cd_2d80;
/// `hkStringPtr::hkStringPtr` (Xbox PDB, `00cb0a60`, `this`) and the constructor taking the
/// finish-loading flag (`00cb0a70`, `this`, `RET 4`; unconfirmed name).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_FINISH: u32 = 0x00cb_0a70;
/// `hkStatisticsCollector::getAllocatedSize` (Xbox PDB, `00d06f80`, cdecl `(pointer, size)`) and
/// `addNormalChunk` (Xbox PDB, `00d06fb0`, `this` = the collector, `(name, pointer, size,
/// flag)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
const ADD_NORMAL_CHUNK: u32 = 0x00d0_6fb0;
/// The strings `"Shape"`, `"CollAgtPtr"`, `"Properties"` and `"ChildShapeAabbs"`.
const SHAPE_TEXT: u32 = 0x0105_c390;
const COLLISION_AGENT_TEXT: u32 = 0x010c_b2d8;
const PROPERTIES_TEXT: u32 = 0x0102_0e44;
const CHILD_AABBS_TEXT: u32 = 0x010c_b2c8;

// Translated from 00cd2930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::setShape` (Xbox PDB), thiscall with one stack word, neither read: answers
/// 1.
pub fn hkp_world_object_set_shape(_e: &mut Engine, _this: Ptr, _unused_0: u32) -> u32 {
    1
}

// Translated from 00cd29f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(value)`: stores the word at `this + 0x88` (the one `00cd2a00` reads and the
/// tree-data setter `00d08400` writes).
pub fn fn_00cd29f0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x88, value);
}

// Translated from 00cd2a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the word at `this + 0x88` (the one `00cd29f0` stores).
pub fn fn_00cd2a00(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x88)
}

// Translated from 00cd2a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::removeProperty` (Xbox PDB), thiscall `(out, key)`, `RET 8`: the properties
/// are 16-byte entries in the array at +0x78 (count at +0x7C): the key at +0, two values at +8
/// and +0xC. Finds the first entry with the key; the values of that entry go to `out[0]` and
/// `out[1]`, the count drops by one and the entries after it move down one place (two words
/// at a time). Without such an entry `out` receives two zeros.
pub fn hkp_world_object_remove_property(e: &mut Engine, this: Ptr, out: Ptr, key: u32) {
    let me = this.addr();
    let count = e.mem.i32(me + 0x7c);
    if count > 0 {
        let array = e.mem.u32(me + 0x78);
        let mut index = 0i32;
        let mut element = array;
        loop {
            if e.mem.u32(element) == key {
                let offset = (index as u32) << 4;
                let first_value = e.mem.u32(array.wrapping_add(offset) + 8);
                let second_value = e.mem.u32(array.wrapping_add(offset) + 0xc);
                let new_count = e.mem.u32(me + 0x7c).wrapping_sub(1);
                e.mem.set_u32(me + 0x7c, new_count);
                let bytes = new_count.wrapping_sub(index as u32) << 4;
                let base = e.mem.u32(me + 0x78);
                let mut target = offset.wrapping_add(base);
                if (bytes as i32) > 0 {
                    let mut left = (bytes.wrapping_sub(1) >> 3) + 1;
                    while left != 0 {
                        let low = e.mem.u32(target + 0x10);
                        e.mem.set_u32(target, low);
                        let high = e.mem.u32(target + 0x14);
                        e.mem.set_u32(target + 4, high);
                        target += 8;
                        left -= 1;
                    }
                }
                e.mem.set_u32(out.addr(), first_value);
                e.mem.set_u32(out.addr() + 4, second_value);
                return;
            }
            index += 1;
            element += 0x10;
            if index >= count {
                break;
            }
        }
    }
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, 0);
}

// Translated from 00cd2b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::calcContentStatistics` (Xbox PDB), thiscall `(collector)`, `RET 8`: reports
/// to the statistics collector. The collector's virtual function at +0xC gets (`"Shape"`, the
/// word at +0x10, 0). If the capacity word at +0x68 is not negative, or the collector's byte
/// at +0x10 is set, its function at +0x14 gets (3, `"CollAgtPtr"`, the pointer at +0x60, the
/// count at +0x64 times 8, the allocated size `00d06f80` answers for the pointer and the
/// capacity masked with 0x3fffffff times 8). The same for the properties (+0x78 data, +0x7C
/// count times 16, +0x80 capacity, `"Properties"`). With child shape boxes at +0x54 (count
/// 16-bit at +0x52) two chunks are added with `addNormalChunk` (`00d06fb0`) under
/// `"ChildShapeAabbs"`: the boxes at 32 bytes each and the array at +0x58 at 4 bytes each.
/// The second stack word (`RET 8`; callers pass the class record) is not read.
pub fn hkp_world_object_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let me = this.addr();
    let sink = collector.addr();
    let shape = e.mem.u32(me + 0x10);
    e.vcall(sink, 0xc, &args![SHAPE_TEXT, shape, 0u32]);
    let agents = e.mem.i32(me + 0x68);
    if agents >= 0 || e.mem.u8(sink + 0x10) != 0 {
        let data = e.mem.u32(me + 0x60);
        let reserved = (agents as u32 & 0x3fff_ffff) << 3;
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, reserved]).u32();
        let data = e.mem.u32(me + 0x60);
        let used = e.mem.u32(me + 0x64) << 3;
        e.vcall(
            sink,
            0x14,
            &args![3u32, COLLISION_AGENT_TEXT, data, used, allocated],
        );
    }
    let capacity = e.mem.i32(me + 0x80);
    if capacity >= 0 || e.mem.u8(sink + 0x10) != 0 {
        let data = e.mem.u32(me + 0x78);
        let reserved = (capacity as u32 & 0x3fff_ffff) << 4;
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, reserved]).u32();
        let data = e.mem.u32(me + 0x78);
        let used = e.mem.u32(me + 0x7c) << 4;
        e.vcall(
            sink,
            0x14,
            &args![3u32, PROPERTIES_TEXT, data, used, allocated],
        );
    }
    let boxes = e.mem.u32(me + 0x54);
    if boxes != 0 {
        let count = u32::from(e.mem.u16(me + 0x52));
        e.call(
            ADD_NORMAL_CHUNK,
            &args![collector, CHILD_AABBS_TEXT, boxes, count << 5, 0u32],
        );
        let array = e.mem.u32(me + 0x58);
        let count = u32::from(e.mem.u16(me + 0x52));
        e.call(
            ADD_NORMAL_CHUNK,
            &args![collector, CHILD_AABBS_TEXT, array, count << 2, 0u32],
        );
    }
}

// Translated from 00cd2be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::addProperty` (Xbox PDB), thiscall `(key, first, second)`, `RET 0xC`: a
/// property is a 16-byte entry in the array at +0x78 (count +0x7C, capacity word +0x80 with
/// the low 30 bits the capacity): the key at +0, the values at +8 and +0xC. If an entry with
/// the key exists nothing happens. Otherwise, when the count equals the capacity, the array is
/// grown (`00c90940` with the router's allocator and element size 0x10), and the entry is
/// appended.
pub fn hkp_world_object_add_property(e: &mut Engine, this: Ptr, key: u32, first: u32, second: u32) {
    let me = this.addr();
    let count = e.mem.i32(me + 0x7c);
    if count > 0 {
        let mut element = e.mem.u32(me + 0x78);
        let mut index = 0i32;
        loop {
            if e.mem.u32(element) == key {
                return;
            }
            index += 1;
            element += 0x10;
            if index >= count {
                break;
            }
        }
    }
    let array = me + 0x78;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
    }
    let used = e.mem.u32(array + 4);
    let entry = (used << 4).wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, used + 1);
    e.mem.set_u32(entry + 8, first);
    e.mem.set_u32(entry + 0xc, second);
    e.mem.set_u32(entry, key);
}

// Translated from 00cd2c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor taking the finish-loading flag (thiscall `(flag)`, `RET 4`; the Xbox PDB name
/// is not confirmed): stores the vtable pointer. Only with a non-zero flag, the collidable
/// part at +0x10 is reset (16-bit words at +0x50 and +0x52, the words at +0x54, +0x58, +0x40
/// and +0x30 set to 0, 0, 0 and 1, and the byte at +0x29 set to 0xEC) before the arrays at
/// +0x60 (data 0, count 0, capacity word 0x80000000) are emptied; the id words are set
/// (+0x6C is 0xFFFFFFF1, the 16-bit +0x70 is 0x8000), `00cb0a70(flag)` runs on the string at
/// +0x74 and, with a non-zero flag, the byte at +0x20 is set to 0xF0. Answers `this`.
pub fn fn_00cd2c50(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, WORLD_OBJECT_VTABLE);
    let part = me + 0x10;
    if flag != 0 {
        e.mem.set_u16(part + 0x40, 0);
        e.mem.set_u16(part + 0x42, 0);
        e.mem.set_u32(part + 0x44, 0);
        e.mem.set_u32(part + 0x48, 0);
        e.mem.set_u32(part + 0x20, 1);
        e.mem.set_u32(part + 0x30, 0);
        // `DL = BL - AL` where `AL` is the low byte of `part + 0x14`: the byte is -0x14.
        e.mem.set_u8(part + 0x19, 0xec);
    }
    e.mem.set_u32(part + 0x50, 0);
    e.mem.set_u32(part + 0x54, 0);
    e.mem.set_u32(part + 0x58, 0x8000_0000);
    e.mem.set_u32(me + 0x6c, 0xffff_fff1);
    e.mem.set_u16(me + 0x70, 0x8000);
    e.call(STRING_PTR_FINISH, &args![me + 0x74, flag]);
    if flag != 0 {
        // `CL = AL - BL` with `AL` the low byte of `this` and `BL` that of `this + 0x10`: -0x10.
        e.mem.set_u8(part + 0x10, 0xf0);
    }
    this
}

// Translated from 00cd2ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::hkpWorldObject` (Xbox PDB), thiscall `(shape, second)`, `RET 8`: sets the
/// 16-bit word at +6 to 1, the vtable pointer, the words at +8 and +0xC to 0, constructs the
/// `hkpCollidable` at +0x10 (`00cd2d80` with `shape`, 0, `second`), and sets the arrays and
/// ids: +0x60 and +0x64 to 0, +0x68 to 0x80000000, +0x6C to 0xFFFFFFF1, the 16-bit +0x70 to
/// 0x8000, the string at +0x74 constructed (`00cb0a60`), +0x78 and +0x7C to 0, +0x80 to
/// 0x80000000, +0x84 to 0, +0x88 to 0xFFFFFFFF and the byte at +0x20 to 0xF0 (`this` minus
/// `this + 0x10`). A non-zero `shape` gets a reference (`00c90510`). Answers `this`.
pub fn hkp_world_object_construct(e: &mut Engine, this: Ptr, shape: Ptr, second: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, WORLD_OBJECT_VTABLE);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 0);
    e.call(COLLIDABLE_CONSTRUCT, &args![me + 0x10, shape, 0u32, second]);
    e.mem.set_u32(me + 0x60, 0);
    e.mem.set_u32(me + 0x64, 0);
    e.mem.set_u32(me + 0x68, 0x8000_0000);
    e.mem.set_u32(me + 0x6c, 0xffff_fff1);
    e.mem.set_u16(me + 0x70, 0x8000);
    e.call(STRING_PTR_CONSTRUCT, &args![me + 0x74]);
    e.mem.set_u32(me + 0x78, 0);
    e.mem.set_u32(me + 0x7c, 0);
    e.mem.set_u32(me + 0x80, 0x8000_0000);
    e.mem.set_u32(me + 0x84, 0);
    e.mem.set_u32(me + 0x88, 0xffff_ffff);
    e.mem.set_u8(me + 0x20, 0xf0);
    if shape.addr() != 0 {
        e.call(ADD_REFERENCE, &args![shape]);
    }
    this
}

// Translated from 00d08400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldObject::setTreeData` (Xbox PDB), cdecl `(collidable, value)`: only the low 16 bits
/// of `value` are read. The collidable pointer plus the signed byte at its +0x10 is the world
/// object; `00cd29f0` stores `value` in its +0x88 word.
pub fn hkp_world_object_set_tree_data(e: &mut Engine, collidable: Ptr, value: u16) {
    let offset = i32::from(e.mem.i8(collidable.addr() + 0x10));
    let object = collidable.addr().wrapping_add(offset as u32);
    fn_00cd29f0(e, Ptr::new(object), u32::from(value));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_2930, hkp_world_object_set_shape(Ptr, u32) -> u32),
        entry!(0x00cd_29f0, fn_00cd29f0(Ptr, u32)),
        entry!(0x00cd_2a00, fn_00cd2a00(Ptr) -> u32),
        entry!(0x00cd_2a50, hkp_world_object_remove_property(Ptr, Ptr, u32)),
        entry!(
            0x00cd_2b00,
            hkp_world_object_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(
            0x00cd_2be0,
            hkp_world_object_add_property(Ptr, u32, u32, u32)
        ),
        entry!(0x00cd_2c50, fn_00cd2c50(Ptr, u32) -> Ptr),
        entry!(
            0x00cd_2ce0,
            hkp_world_object_construct(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(0x00d0_8400, hkp_world_object_set_tree_data(Ptr, u16)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn log_double(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A property array of 16-byte entries `(key, a, b)` at +0x78 of a new object, with room
    /// for `room` entries.
    fn object_with_properties(e: &mut Engine, entries: &[(u32, u32, u32)], room: u32) -> u32 {
        let object = e.mem.alloc(0x100);
        let data = e.mem.alloc(16 * room.max(1));
        for (i, &(key, a, b)) in entries.iter().enumerate() {
            let at = data + 16 * i as u32;
            e.mem.set_u32(at, key);
            e.mem.set_u32(at + 8, a);
            e.mem.set_u32(at + 0xc, b);
        }
        e.mem.set_u32(object + 0x78, data);
        e.mem.set_u32(object + 0x7c, entries.len() as u32);
        e.mem.set_u32(object + 0x80, room);
        object
    }

    #[test]
    fn set_shape_answers_one_and_the_word_accessors_use_the_user_data_slot() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00cd_2930, &args![this, 0x1234u32]).u32(), 1);
        e.call(0x00cd_29f0, &args![this, 0xcafe_f00du32]);
        assert_eq!(e.mem.u32(this + 0x88), 0xcafe_f00d);
        assert_eq!(e.call(0x00cd_2a00, &args![this]).u32(), 0xcafe_f00d);
    }

    #[test]
    fn remove_property_hands_back_the_values_and_closes_the_gap() {
        let mut e = engine();
        let object = object_with_properties(
            &mut e,
            &[(1, 10, 11), (2, 20, 21), (3, 30, 31), (4, 40, 41)],
            4,
        );
        let out = e.mem.alloc(8);
        e.call(0x00cd_2a50, &args![object, out, 2u32]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (20, 21));
        assert_eq!(e.mem.u32(object + 0x7c), 3);
        let data = e.mem.u32(object + 0x78);
        let keys: Vec<(u32, u32, u32)> = (0..3)
            .map(|i| {
                (
                    e.mem.u32(data + 16 * i),
                    e.mem.u32(data + 16 * i + 8),
                    e.mem.u32(data + 16 * i + 0xc),
                )
            })
            .collect();
        assert_eq!(keys, vec![(1, 10, 11), (3, 30, 31), (4, 40, 41)]);
        // The last entry: nothing to move.
        e.call(0x00cd_2a50, &args![object, out, 4u32]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (40, 41));
        assert_eq!(e.mem.u32(object + 0x7c), 2);
        // Not found, and an empty list: zeros.
        e.call(0x00cd_2a50, &args![object, out, 99u32]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
        assert_eq!(e.mem.u32(object + 0x7c), 2);
        e.mem.set_u32(object + 0x7c, 0);
        e.mem.set_u32(out, 5);
        e.call(0x00cd_2a50, &args![object, out, 1u32]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
    }

    #[test]
    fn add_property_appends_unless_the_key_exists_and_grows_a_full_array() {
        let mut e = engine();
        let log: Log = Rc::default();
        let allocator = e.mem.alloc(0x40);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        // The grow function doubles the room to 4 entries in place.
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let old = e.mem.u32(a[1]);
            let bigger = e.mem.alloc(16 * 4);
            let bytes = e.mem.bytes(old, 16 * 2);
            e.mem.write(bigger, &bytes);
            e.mem.set_u32(a[1], bigger);
            e.mem.set_u32(a[1] + 8, 0x8000_0000 | 4);
            Ret::default()
        });
        let object = object_with_properties(&mut e, &[(1, 10, 11)], 2);
        // A key that exists changes nothing.
        e.call(0x00cd_2be0, &args![object, 1u32, 99u32, 98u32]);
        assert_eq!(e.mem.u32(object + 0x7c), 1);
        assert!(calls(&log, MEMORY_ROUTER).is_empty());
        // A new key is appended; room for it, so no growth.
        e.call(0x00cd_2be0, &args![object, 2u32, 20u32, 21u32]);
        assert_eq!(e.mem.u32(object + 0x7c), 2);
        assert!(calls(&log, ARRAY_RESERVE_MORE).is_empty());
        let data = e.mem.u32(object + 0x78);
        assert_eq!(
            (
                e.mem.u32(data + 16),
                e.mem.u32(data + 24),
                e.mem.u32(data + 28)
            ),
            (2, 20, 21)
        );
        // The array is full now: it is grown with the allocator, then the entry is appended.
        e.call(0x00cd_2be0, &args![object, 3u32, 30u32, 31u32]);
        assert_eq!(
            calls(&log, ARRAY_RESERVE_MORE),
            vec![vec![allocator, object + 0x78, 0x10]]
        );
        assert_eq!(e.mem.u32(object + 0x7c), 3);
        let data = e.mem.u32(object + 0x78);
        assert_eq!(
            (
                e.mem.u32(data),
                e.mem.u32(data + 32),
                e.mem.u32(data + 40),
                e.mem.u32(data + 44)
            ),
            (1, 3, 30, 31)
        );
        // An empty list needs no search.
        let empty = object_with_properties(&mut e, &[], 1);
        e.call(0x00cd_2be0, &args![empty, 7u32, 70u32, 71u32]);
        assert_eq!(e.mem.u32(empty + 0x7c), 1);
    }

    #[test]
    fn content_statistics_reports_the_shape_the_arrays_and_the_child_boxes() {
        let mut e = engine();
        let log: Log = Rc::default();
        let (name_hook, size_hook) = (0x0900_0001, 0x0900_0002);
        log_double(&mut e, &log, name_hook, 0);
        log_double(&mut e, &log, size_hook, 0);
        log_double(&mut e, &log, GET_ALLOCATED_SIZE, 0x333);
        log_double(&mut e, &log, ADD_NORMAL_CHUNK, 0);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0xc, name_hook);
        e.mem.set_u32(table + 0x14, size_hook);
        let collector = e.mem.alloc(0x40);
        e.mem.set_u32(collector, table);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x10, 0x5000);
        // Negative capacities and the collector's byte clear: only the shape is reported.
        e.mem.set_u32(this + 0x68, 0x8000_0000);
        e.mem.set_u32(this + 0x80, 0x8000_0000);
        e.call(0x00cd_2b00, &args![this, collector, 0u32]);
        assert_eq!(
            calls(&log, name_hook),
            vec![vec![collector, SHAPE_TEXT, 0x5000, 0]]
        );
        assert!(calls(&log, size_hook).is_empty());
        assert!(calls(&log, GET_ALLOCATED_SIZE).is_empty());
        assert!(calls(&log, ADD_NORMAL_CHUNK).is_empty());
        // The collector's byte set: both arrays are reported even with negative capacities
        // (masked to 0).
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.mem.set_u32(this + 0x60, 0x6000);
        e.mem.set_u32(this + 0x64, 3);
        e.mem.set_u32(this + 0x78, 0x7000);
        e.mem.set_u32(this + 0x7c, 5);
        e.call(0x00cd_2b00, &args![this, collector, 0u32]);
        assert_eq!(
            calls(&log, size_hook),
            vec![
                vec![collector, 3, COLLISION_AGENT_TEXT, 0x6000, 24, 0x333],
                vec![collector, 3, PROPERTIES_TEXT, 0x7000, 80, 0x333]
            ]
        );
        assert_eq!(
            calls(&log, GET_ALLOCATED_SIZE),
            vec![vec![0x6000, 0], vec![0x7000, 0]]
        );
        // Non-negative capacities are masked and scaled; child boxes add two chunks.
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 0);
        e.mem.set_u32(this + 0x68, 0x4000_0006);
        e.mem.set_u32(this + 0x80, 9);
        e.mem.set_u32(this + 0x54, 0x8000);
        e.mem.set_u16(this + 0x52, 2);
        e.mem.set_u32(this + 0x58, 0x9000);
        e.call(0x00cd_2b00, &args![this, collector, 0u32]);
        assert_eq!(
            calls(&log, GET_ALLOCATED_SIZE),
            vec![vec![0x6000, 48], vec![0x7000, 144]]
        );
        assert_eq!(
            calls(&log, ADD_NORMAL_CHUNK),
            vec![
                vec![collector, CHILD_AABBS_TEXT, 0x8000, 64, 0],
                vec![collector, CHILD_AABBS_TEXT, 0x9000, 8, 0]
            ]
        );
    }

    #[test]
    fn finish_constructor_resets_the_collidable_only_with_the_flag() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, &log, STRING_PTR_FINISH, 0);
        let this = e.mem.alloc(0x100);
        for offset in [0x50, 0x52, 0x54, 0x58, 0x40, 0x30, 0x29, 0x20] {
            e.mem.set_u8(this + offset, 0x77);
        }
        let got = e.call(0x00cd_2c50, &args![this, 0u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), WORLD_OBJECT_VTABLE);
        assert_eq!(e.mem.u8(this + 0x29), 0x77);
        assert_eq!(e.mem.u8(this + 0x20), 0x77);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x6c), 0xffff_fff1);
        assert_eq!(e.mem.u16(this + 0x70), 0x8000);
        assert_eq!(calls(&log, STRING_PTR_FINISH), vec![vec![this + 0x74, 0]]);
        log.borrow_mut().clear();
        e.call(0x00cd_2c50, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x29), 0xec);
        assert_eq!(e.mem.u8(this + 0x20), 0xf0);
        assert_eq!(e.mem.u32(this + 0x30), 1);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(e.mem.u32(this + 0x54), 0);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(calls(&log, STRING_PTR_FINISH), vec![vec![this + 0x74, 1]]);
    }

    #[test]
    fn constructor_builds_the_collidable_and_references_the_shape() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, &log, COLLIDABLE_CONSTRUCT, 0);
        log_double(&mut e, &log, STRING_PTR_CONSTRUCT, 0);
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x80, 0x1234);
        let got = e.call(0x00cd_2ce0, &args![this, 0x5000u32, 7u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls(&log, COLLIDABLE_CONSTRUCT),
            vec![vec![this + 0x10, 0x5000, 0, 7]]
        );
        assert_eq!(calls(&log, STRING_PTR_CONSTRUCT), vec![vec![this + 0x74]]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x5000]]);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), WORLD_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x6c), 0xffff_fff1);
        assert_eq!(e.mem.u16(this + 0x70), 0x8000);
        assert_eq!(e.mem.u32(this + 0x78), 0);
        assert_eq!(e.mem.u32(this + 0x7c), 0);
        assert_eq!(e.mem.u32(this + 0x80), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert_eq!(e.mem.u32(this + 0x88), 0xffff_ffff);
        assert_eq!(e.mem.u8(this + 0x20), 0xf0);
        // Without a shape there is no reference.
        log.borrow_mut().clear();
        e.call(0x00cd_2ce0, &args![this, 0u32, 7u32]);
        assert!(calls(&log, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn set_tree_data_writes_through_the_signed_offset_byte() {
        let mut e = engine();
        let object = e.mem.alloc(0x100);
        let collidable = object + 0x10;
        // The collidable's byte at +0x10 is -0x10: the world object starts there.
        e.mem.set_u8(collidable + 0x10, 0xf0);
        e.call(0x00d0_8400, &args![collidable, 0xffff_1234u32]);
        assert_eq!(e.mem.u32(object + 0x88), 0x1234);
    }
}
