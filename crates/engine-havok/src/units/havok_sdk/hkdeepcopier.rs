//! `Havok SDK/hkdeepcopier.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemoryRouter::easyAlloc(allocator, size)` (cdecl).
const EASY_ALLOC: u32 = 0x00c8_5690;
/// `hkString::memCpy(dst, src, size)` (cdecl, Xbox PDB).
const MEM_COPY: u32 = 0x00cb_1260;
/// Grows an `hkArray` (cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkMapBase::reserve` (`this` = the map, then `(allocator, count)`, `RET 8`).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// Inserts a key and value into a map (`this`, then `(allocator, key, value)`, `RET 0xC`).
const MAP_INSERT: u32 = 0x00cb_1870;
/// `hkMapBase::getWithDefault` (`this`, then `(key, default)`, `RET 8`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
/// `hkMapBase::findKey` (`this`, then `(key)`, `RET 4`): the slot of the key, or the slot mask
/// plus one when it is absent.
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
/// `hkMapBase::clearAndDeallocate` (`this`, then `(allocator)`, `RET 4`).
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
/// The destructor of the map, which does nothing.
const NO_OP_DESTRUCTOR: u32 = 0x00a2_9680;
/// `hkStructureLayout` constructor taking the layout rules word (`RET 4`).
const LAYOUT_CONSTRUCT: u32 = 0x00dc_1680;
/// The `hkOstream` constructor over a byte array (`RET 4`).
const OSTREAM_CONSTRUCT: u32 = 0x00cc_0630;
/// The `hkOstream` destructor.
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The `hkObjectCopier` constructor (`this`, then `(layout out, layout in, argument)`,
/// `RET 0xC`).
const COPIER_CONSTRUCT: u32 = 0x00e0_3120;
/// `hkObjectCopier::~hkObjectCopier` (Xbox PDB).
const COPIER_DESTRUCT: u32 = 0x00e0_3160;
/// `hkObjectCopier::copyObject` (Xbox PDB; `this`, then `(object, class, stream, class,
/// relocations)`, `RET 0x14`).
const COPIER_COPY_OBJECT: u32 = 0x00e0_4920;
/// `hkRelocationInfo::~hkRelocationInfo` (Xbox PDB).
const RELOCATION_INFO_DESTRUCT: u32 = 0x00e0_28e0;
/// `hkClass::hasVtable` (Xbox PDB; `this` = the class, then `(out byte)`, answers the out
/// pointer).
const CLASS_HAS_VTABLE: u32 = 0x00d4_70f0;
/// `hkClass::getName` (Xbox PDB; `this` = the class).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
/// The host layout rules word handed to both structure layouts.
const HOST_LAYOUT_RULES: u32 = 0x011b_7b88;

// Frame of the translated function, as offsets from the stack pointer after the prologue
// (the saved registers are at `+0` to `+0x10`, the return address at `+0xB0`).
/// The work list: `hkArray` of `(object, class)` pairs of 8 bytes.
const WORK_LIST: u32 = 0x10;
/// The `hkArray` of 12-byte records `(offset in the output, object, class name)`.
const PAIRS: u32 = 0x1c;
/// The input layout (a dword, dead after the copier is built) and then the loop counter.
const LAYOUT_IN_AND_COUNTER: u32 = 0x28;
/// The `hkArray` of output bytes (its size is the number of bytes written).
const BYTES: u32 = 0x2c;
/// The byte `hkClass::hasVtable` writes.
const HAS_VTABLE_OUT: u32 = 0x3b;
/// The map from object to its offset in the output.
const OFFSET_MAP: u32 = 0x3c;
/// The word of the map after its size and capacity, compared with `findKey`.
const OFFSET_MAP_LIMIT: u32 = 0x44;
/// The output layout (a dword, dead after the copier is built), later a scratch dword.
const LAYOUT_OUT_AND_SCRATCH: u32 = 0x48;
/// The `hkRelocationInfo`: the local relocations (8-byte records `(offset, target)`).
const RELOCATIONS: u32 = 0x4c;
/// The `hkRelocationInfo`: the global references (16-byte records `(offset, key, value, _)`).
const REFERENCES: u32 = 0x58;
/// The `hkOstream` over the byte array.
const STREAM: u32 = 0x80;
/// The `hkObjectCopier`.
const COPIER: u32 = 0x8c;
/// The class name of the current object.
const CLASS_NAME: u32 = 0xac;

/// The allocator of the memory router (its field `+0x10`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Pushes `(object, class)` onto the work list the way the exe inlines it: the array grows
/// (element size 8) only when its size equals its capacity.
fn push_work_item(e: &mut Engine, list: u32, object: u32, class: u32) {
    let allocator = allocator(e);
    let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
    if e.mem.u32(list + 4) == capacity {
        e.call(ARRAY_GROW, &args![allocator, list, 8u32]);
    }
    let size = e.mem.u32(list + 4);
    let slot = e.mem.u32(list).wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(slot, object);
    e.mem.set_u32(slot + 4, class);
    e.mem.set_u32(list + 4, size.wrapping_add(1));
}

/// Appends the record `(offset, object, name)` of 12 bytes to the pair array. The slot is only
/// written when its address is not null.
fn push_pair(e: &mut Engine, list: u32, offset: u32, object: u32, name: u32) {
    let allocator = allocator(e);
    let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
    if e.mem.u32(list + 4) == capacity {
        e.call(ARRAY_GROW, &args![allocator, list, 0xcu32]);
    }
    let size = e.mem.u32(list + 4);
    let slot = e.mem.u32(list).wrapping_add(size.wrapping_mul(12));
    if slot != 0 {
        e.mem.set_u32(slot, offset);
        e.mem.set_u32(slot + 4, object);
        e.mem.set_u32(slot + 8, name);
    }
    let size = e.mem.u32(list + 4);
    e.mem.set_u32(list + 4, size.wrapping_add(1));
}

/// Empties an `hkArray` of the frame: the size becomes 0 and storage that is owned (bit 31 of
/// the capacity word clear) is given back through the allocator's slot `+0x10` with its byte
/// size. `reset` then also clears the data pointer and marks the capacity "not owned".
fn release_array(e: &mut Engine, list: u32, element_size: u32, reset: bool) {
    let allocator = allocator(e);
    let capacity_word = e.mem.u32(list + 8);
    e.mem.set_u32(list + 4, 0);
    if capacity_word & 0x8000_0000 == 0 {
        let bytes = (capacity_word & 0x3fff_ffff).wrapping_mul(element_size);
        let data = e.mem.u32(list);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    if reset {
        e.mem.set_u32(list, 0);
        e.mem.set_u32(list + 8, 0x8000_0000);
    }
}

/// Runs the destructors of the locals in the order the exe does, on both of its exits.
fn release_locals(e: &mut Engine, frame: u32) {
    e.call(COPIER_DESTRUCT, &args![frame + COPIER]);
    release_array(e, frame + PAIRS, 12, true);
    let allocator = allocator(e);
    e.call(
        MAP_CLEAR_AND_DEALLOCATE,
        &args![frame + OFFSET_MAP, allocator],
    );
    e.call(NO_OP_DESTRUCTOR, &args![frame + OFFSET_MAP]);
    release_array(e, frame + WORK_LIST, 8, true);
    e.call(RELOCATION_INFO_DESTRUCT, &args![frame + RELOCATIONS]);
    e.call(OSTREAM_DESTRUCT, &args![frame + STREAM]);
    release_array(e, frame + BYTES, 1, false);
}

// Translated from 00e4afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDeepCopier::deepCopy` (Xbox PDB; cdecl, six stack arguments): copies `object` of
/// `class` and everything it references into one new block of the router's allocator.
///
/// A work list starts with `(object, class)`. For each entry the current output size is
/// recorded as the object's offset in a map, the class is replaced by the one the
/// `class_resolver` (virtual `+0x10`, given the object) names when `hkClass::hasVtable`
/// says so, a record `(offset, object, class name)` is kept, and the object copier writes the
/// object into the byte stream and its relocations. Each new global reference `(key, value)`
/// with both words set is first looked up in `copy_map` (when given: a hit replaces the key by
/// the existing copy) and otherwise queued when its key has no offset yet.
///
/// With nothing written the locals are released and 0 is answered. Otherwise the bytes are
/// copied into a block of the written size, local relocations store `target + block` at
/// `block + offset`, each reference stores the block address of its key (or the key itself
/// when it has no offset), `copy_map` gets `object -> block + offset` for every record, and
/// `sink` (virtual `+0x10`) is told `(block + offset, class name)` for every record.
/// `copier_argument` goes unchanged to the object copier's constructor. Answers the block.
pub fn hk_deep_copier_deep_copy(
    e: &mut Engine,
    object: Ptr,
    class: Ptr,
    copy_map: Ptr,
    class_resolver: Ptr,
    sink: Ptr,
    copier_argument: u32,
) -> Ptr {
    e.with_stack(0xb0, |e, frame| {
        let f = frame.addr();
        // The byte array and the stream over it.
        e.mem.set_u32(f + BYTES, 0);
        e.mem.set_u32(f + BYTES + 4, 0);
        e.mem.set_u32(f + BYTES + 8, 0x8000_0000);
        e.call(OSTREAM_CONSTRUCT, &args![f + STREAM, f + BYTES]);
        // The relocation info: four empty arrays and a cleared word.
        for array in [RELOCATIONS, REFERENCES, 0x64, 0x70] {
            e.mem.set_u32(f + array, 0);
            e.mem.set_u32(f + array + 4, 0);
            e.mem.set_u32(f + array + 8, 0x8000_0000);
        }
        e.mem.set_u32(f + 0x7c, 0);
        // The work list, starting with the object and its class.
        e.mem.set_u32(f + WORK_LIST, 0);
        e.mem.set_u32(f + WORK_LIST + 4, 0);
        e.mem.set_u32(f + WORK_LIST + 8, 0x8000_0000);
        push_work_item(e, f + WORK_LIST, object.addr(), class.addr());
        // The offset map.
        e.mem.set_u32(f + OFFSET_MAP, 0);
        e.mem.set_u32(f + OFFSET_MAP + 4, 0);
        e.mem.set_u32(f + OFFSET_MAP_LIMIT, 0xffff_ffff);
        let pool = allocator(e);
        e.call(MAP_RESERVE, &args![f + OFFSET_MAP, pool, 0u32]);
        // The pair array, the two layouts and the object copier.
        e.mem.set_u32(f + PAIRS, 0);
        e.mem.set_u32(f + PAIRS + 4, 0);
        e.mem.set_u32(f + PAIRS + 8, 0x8000_0000);
        e.call(
            LAYOUT_CONSTRUCT,
            &args![f + LAYOUT_IN_AND_COUNTER, HOST_LAYOUT_RULES],
        );
        e.call(
            LAYOUT_CONSTRUCT,
            &args![f + LAYOUT_OUT_AND_SCRATCH, HOST_LAYOUT_RULES],
        );
        e.call(
            COPIER_CONSTRUCT,
            &args![
                f + COPIER,
                f + LAYOUT_OUT_AND_SCRATCH,
                f + LAYOUT_IN_AND_COUNTER,
                copier_argument
            ],
        );
        // The work list loop; the counter lives in the (dead) input layout slot.
        e.mem.set_u32(f + LAYOUT_IN_AND_COUNTER, 0);
        if (e.mem.u32(f + WORK_LIST + 4) as i32) > 0 {
            let mut index = 0u32;
            loop {
                let list = e.mem.u32(f + WORK_LIST);
                let item = list.wrapping_add(index.wrapping_mul(8));
                let item_object = e.mem.u32(item);
                let mut item_class = e.mem.u32(item + 4);
                let offset = e.mem.u32(f + BYTES + 4);
                let pool = allocator(e);
                e.call(
                    MAP_INSERT,
                    &args![f + OFFSET_MAP, pool, item_object, offset],
                );
                let first_reference = e.mem.u32(f + REFERENCES + 4);
                let flag = e
                    .call(CLASS_HAS_VTABLE, &args![item_class, f + HAS_VTABLE_OUT])
                    .u32();
                if e.mem.u8(flag) != 0 {
                    item_class = e
                        .vcall(class_resolver.addr(), 0x10, &args![item_object])
                        .u32();
                }
                let written = e.mem.u32(f + BYTES + 4);
                let name = e.call(CLASS_GET_NAME, &args![item_class]).u32();
                e.mem.set_u32(f + CLASS_NAME, name);
                push_pair(e, f + PAIRS, written, item_object, name);
                let stream = e.mem.u32(f + STREAM + 8);
                e.call(
                    COPIER_COPY_OBJECT,
                    &args![
                        f + COPIER,
                        item_object,
                        item_class,
                        stream,
                        item_class,
                        f + RELOCATIONS
                    ],
                );
                // The references the copy added.
                let mut position = first_reference as i32;
                let mut at = first_reference << 4;
                if position < e.mem.u32(f + REFERENCES + 4) as i32 {
                    loop {
                        let records = e.mem.u32(f + REFERENCES);
                        let key = e.mem.u32(records.wrapping_add(at).wrapping_add(4));
                        let value = e.mem.u32(records.wrapping_add(at).wrapping_add(8));
                        if key != 0 && value != 0 {
                            let mut existing = 0;
                            if !copy_map.is_null() {
                                existing = e
                                    .call(MAP_GET_WITH_DEFAULT, &args![copy_map, key, 0u32])
                                    .u32();
                            }
                            if existing != 0 {
                                let records = e.mem.u32(f + REFERENCES);
                                e.mem
                                    .set_u32(records.wrapping_add(at).wrapping_add(4), existing);
                            } else {
                                let slot = e.call(MAP_FIND_KEY, &args![f + OFFSET_MAP, key]).i32();
                                if slot > e.mem.u32(f + OFFSET_MAP_LIMIT) as i32 {
                                    push_work_item(e, f + WORK_LIST, key, value);
                                }
                            }
                        }
                        position = position.wrapping_add(1);
                        at = at.wrapping_add(0x10);
                        if position >= e.mem.u32(f + REFERENCES + 4) as i32 {
                            break;
                        }
                    }
                }
                index = e.mem.u32(f + LAYOUT_IN_AND_COUNTER).wrapping_add(1);
                e.mem.set_u32(f + LAYOUT_IN_AND_COUNTER, index);
                if index as i32 >= e.mem.u32(f + WORK_LIST + 4) as i32 {
                    break;
                }
            }
        }
        let size = e.mem.u32(f + BYTES + 4);
        if size == 0 {
            release_locals(e, f);
            return Ptr::new(0);
        }
        // The output block, the bytes in it, then the pointers.
        let pool = allocator(e);
        let block = e.call(EASY_ALLOC, &args![pool, size]).u32();
        let data = e.mem.u32(f + BYTES);
        let size = e.mem.u32(f + BYTES + 4);
        e.call(MEM_COPY, &args![block, data, size]);
        // Local relocations: the pointer stored is the block address of the target.
        if (e.mem.u32(f + RELOCATIONS + 4) as i32) > 0 {
            let mut index = 0u32;
            loop {
                let records = e.mem.u32(f + RELOCATIONS);
                let at = records.wrapping_add(index.wrapping_mul(8));
                let target = e.mem.u32(at + 4);
                let patch = e.mem.u32(at);
                e.mem
                    .set_u32(patch.wrapping_add(block), target.wrapping_add(block));
                index = index.wrapping_add(1);
                if index as i32 >= e.mem.u32(f + RELOCATIONS + 4) as i32 {
                    break;
                }
            }
        }
        // References: the block address of the copied key, or the key as it is.
        if (e.mem.u32(f + REFERENCES + 4) as i32) > 0 {
            let mut index = 0u32;
            loop {
                let at = index.wrapping_mul(0x10);
                let records = e.mem.u32(f + REFERENCES);
                let mut stored = e.mem.u32(records.wrapping_add(at).wrapping_add(4));
                let found = e
                    .call(
                        MAP_GET_WITH_DEFAULT,
                        &args![f + OFFSET_MAP, stored, 0xffff_ffffu32],
                    )
                    .u32();
                if found != 0xffff_ffff {
                    stored = found.wrapping_add(block);
                }
                let records = e.mem.u32(f + REFERENCES);
                let patch = e.mem.u32(records.wrapping_add(at));
                e.mem.set_u32(patch.wrapping_add(block), stored);
                index = index.wrapping_add(1);
                if index as i32 >= e.mem.u32(f + REFERENCES + 4) as i32 {
                    break;
                }
            }
        }
        // The caller's map of copies: every object now has an address.
        if !copy_map.is_null() && (e.mem.u32(f + PAIRS + 4) as i32) > 0 {
            let mut index = 0u32;
            loop {
                let at = index.wrapping_mul(12);
                let pairs = e.mem.u32(f + PAIRS);
                let offset = e.mem.u32(pairs.wrapping_add(at));
                let pair_object = e.mem.u32(pairs.wrapping_add(at).wrapping_add(4));
                let address = offset.wrapping_add(block);
                e.mem.set_u32(f + LAYOUT_OUT_AND_SCRATCH, pair_object);
                let pool = allocator(e);
                let key = e.mem.u32(f + LAYOUT_OUT_AND_SCRATCH);
                e.call(MAP_INSERT, &args![copy_map, pool, key, address]);
                index = index.wrapping_add(1);
                if index as i32 >= e.mem.u32(f + PAIRS + 4) as i32 {
                    break;
                }
            }
        }
        // The sink hears about every object and its class name.
        if !sink.is_null() && (e.mem.u32(f + PAIRS + 4) as i32) > 0 {
            let mut index = 0u32;
            loop {
                let at = index.wrapping_mul(12);
                let pairs = e.mem.u32(f + PAIRS);
                let name = e.mem.u32(pairs.wrapping_add(at).wrapping_add(8));
                let offset = e.mem.u32(pairs.wrapping_add(at));
                e.vcall(sink.addr(), 0x10, &args![offset.wrapping_add(block), name]);
                index = index.wrapping_add(1);
                if index as i32 >= e.mem.u32(f + PAIRS + 4) as i32 {
                    break;
                }
            }
        }
        release_locals(e, f);
        Ptr::new(block)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e4_afe0, hk_deep_copier_deep_copy(Ptr, Ptr, Ptr, Ptr, Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    /// The allocator's `free(data, bytes)` (slot `+0x10`), the class resolver's and the
    /// sink's virtual functions.
    const FREE_BLOCK: u32 = 0x0900_0010;
    const RESOLVE_CLASS: u32 = 0x0900_0110;
    const SINK_ENTRY: u32 = 0x0900_0210;

    type Calls = Vec<Vec<u32>>;
    /// The entries of every fake map, by the address of the map.
    type Maps = Rc<RefCell<HashMap<u32, Vec<(u32, u32)>>>>;
    /// What a copied object writes: its byte count, its local relocations `(offset, target)`
    /// and its references `(offset, key, value)`.
    #[derive(Clone, Default)]
    struct Written {
        bytes: u32,
        locals: Vec<[u32; 2]>,
        references: Vec<[u32; 3]>,
    }
    type Script = Rc<RefCell<HashMap<u32, Written>>>;

    struct Rig {
        e: Engine,
        maps: Maps,
        script: Script,
        classes_with_vtable: Rc<RefCell<HashSet<u32>>>,
        /// `(layout word, argument)` the copier held at each `copyObject`.
        copier_state: Rc<RefCell<Vec<(u32, u32)>>>,
        allocator: u32,
    }

    /// Appends a record of `words` to a frame array, allocating it when it is empty.
    fn push_record(e: &mut Engine, array: u32, words: &[u32]) {
        let bytes = words.len() as u32 * 4;
        if e.mem.u32(array + 8) & 0x8000_0000 != 0 {
            let data = e.mem.alloc(bytes * 16);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, 16);
        }
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(data + size * bytes + i as u32 * 4, *word);
        }
        e.mem.set_u32(array + 4, size + 1);
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        let maps: Maps = Rc::default();
        let script: Script = Rc::default();
        let classes_with_vtable: Rc<RefCell<HashSet<u32>>> = Rc::default();
        let copier_state: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_BLOCK);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [
            FREE_BLOCK,
            COPIER_DESTRUCT,
            RELOCATION_INFO_DESTRUCT,
            OSTREAM_DESTRUCT,
            NO_OP_DESTRUCTOR,
            MAP_CLEAR_AND_DEALLOCATE,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(ARRAY_GROW, |e, a| {
            let (array, element) = (a[1], a[2]);
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            let size = e.mem.u32(array + 4);
            let new_capacity = (capacity * 2).max(4);
            let data = e.mem.alloc(new_capacity * element);
            let old = e.mem.u32(array);
            for i in 0..size * element {
                let byte = e.mem.u8(old + i);
                e.mem.set_u8(data + i, byte);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, new_capacity);
            Ret::default()
        });
        e.register_double(EASY_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register_double(MAP_RESERVE, |e, a| {
            e.mem.set_u32(a[0] + 8, 7);
            Ret::default()
        });
        let sink = Rc::clone(&maps);
        e.register_double(MAP_INSERT, move |_, a| {
            let mut maps = sink.borrow_mut();
            let entries = maps.entry(a[0]).or_default();
            match entries.iter_mut().find(|(key, _)| *key == a[2]) {
                Some(entry) => entry.1 = a[3],
                None => entries.push((a[2], a[3])),
            }
            Ret::default()
        });
        let source = Rc::clone(&maps);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| {
            let maps = source.borrow();
            let found = maps
                .get(&a[0])
                .and_then(|entries| entries.iter().find(|(key, _)| *key == a[1]));
            Ret {
                eax: found.map_or(a[2], |entry| entry.1),
                ..Ret::default()
            }
        });
        let source = Rc::clone(&maps);
        e.register_double(MAP_FIND_KEY, move |e, a| {
            let present = source
                .borrow()
                .get(&a[0])
                .is_some_and(|entries| entries.iter().any(|(key, _)| *key == a[1]));
            Ret {
                eax: if present { 0 } else { e.mem.u32(a[0] + 8) + 1 },
                ..Ret::default()
            }
        });
        e.register_double(LAYOUT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(OSTREAM_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 8, 0x5eed_0001);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(COPIER_CONSTRUCT, |e, a| {
            let layout = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], layout);
            e.mem.set_u32(a[0] + 4, a[3]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let classes = Rc::clone(&classes_with_vtable);
        e.register_double(CLASS_HAS_VTABLE, move |e, a| {
            e.mem
                .set_u8(a[1], u8::from(classes.borrow().contains(&a[0])));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(CLASS_GET_NAME, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register_double(RESOLVE_CLASS, |_, a| Ret {
            eax: a[1] + 0x200,
            ..Ret::default()
        });
        e.register_double(SINK_ENTRY, |_, _| Ret::default());
        let scripted = Rc::clone(&script);
        let state = Rc::clone(&copier_state);
        e.register_double(COPIER_COPY_OBJECT, move |e, a| {
            let (this, object, relocations) = (a[0], a[1], a[5]);
            state
                .borrow_mut()
                .push((e.mem.u32(this), e.mem.u32(this + 4)));
            let copy = scripted
                .borrow()
                .get(&object)
                .cloned()
                .expect("scripted object");
            let buffer = relocations - 0x20;
            let size = e.mem.u32(buffer + 4);
            if copy.bytes != 0 && e.mem.u32(buffer + 8) & 0x8000_0000 != 0 {
                let data = e.mem.alloc(0x100);
                e.mem.set_u32(buffer, data);
                e.mem.set_u32(buffer + 8, 0x100);
            }
            let data = e.mem.u32(buffer);
            for i in 0..copy.bytes {
                e.mem.set_u8(data + size + i, object as u8);
            }
            e.mem.set_u32(buffer + 4, size + copy.bytes);
            for local in &copy.locals {
                push_record(e, relocations, local);
            }
            for reference in &copy.references {
                push_record(
                    e,
                    relocations + 0xc,
                    &[reference[0], reference[1], reference[2], 0],
                );
            }
            Ret::default()
        });
        e.call_log = Some(Vec::new());
        Rig {
            e,
            maps,
            script,
            classes_with_vtable,
            copier_state,
            allocator,
        }
    }

    impl Rig {
        fn script(&self, object: u32, copy: Written) {
            self.script.borrow_mut().insert(object, copy);
        }

        fn copy(
            &mut self,
            object: u32,
            class: u32,
            copy_map: u32,
            resolver: u32,
            sink: u32,
        ) -> u32 {
            self.e
                .call(
                    0x00e4_afe0,
                    &args![object, class, copy_map, resolver, sink, 0xa5a5u32],
                )
                .u32()
        }

        fn calls_to(&self, address: u32) -> Calls {
            self.e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, words)| words.clone())
                .collect()
        }

        /// An object whose virtual `+0x10` is `function`.
        fn object_with_slot(&mut self, function: u32) -> u32 {
            let vtable = self.e.mem.alloc(0x20);
            self.e.mem.set_u32(vtable + 0x10, function);
            let object = self.e.mem.alloc(8);
            self.e.mem.set_u32(object, vtable);
            object
        }
    }

    #[test]
    fn nothing_written_answers_zero_and_releases_the_locals() {
        let mut r = rig();
        r.script(0x1011, Written::default());
        assert_eq!(r.copy(0x1011, 0x2000, 0, 0, 0), 0);
        assert!(r.calls_to(EASY_ALLOC).is_empty());
        assert!(r.calls_to(MEM_COPY).is_empty());
        // The work list and the pair array grew once each; both are given back, the pairs
        // first (4 records of 12 bytes, 4 work items of 8 bytes); the byte array was never
        // allocated.
        let frees = r.calls_to(FREE_BLOCK);
        let bytes: Vec<u32> = frees.iter().map(|call| call[2]).collect();
        assert_eq!(bytes, [48, 32]);
        assert!(frees.iter().all(|call| call[0] == r.allocator));
        // The destructors run in the exe's order.
        let order: Vec<u32> =
            r.e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|(address, _)| *address)
                .filter(|address| {
                    [
                        COPIER_DESTRUCT,
                        MAP_CLEAR_AND_DEALLOCATE,
                        NO_OP_DESTRUCTOR,
                        RELOCATION_INFO_DESTRUCT,
                        OSTREAM_DESTRUCT,
                    ]
                    .contains(address)
                })
                .collect();
        assert_eq!(
            order,
            [
                COPIER_DESTRUCT,
                MAP_CLEAR_AND_DEALLOCATE,
                NO_OP_DESTRUCTOR,
                RELOCATION_INFO_DESTRUCT,
                OSTREAM_DESTRUCT
            ]
        );
    }

    #[test]
    fn locals_are_set_up_the_way_the_exe_does() {
        let mut r = rig();
        r.script(0x1011, Written::default());
        r.copy(0x1011, 0x2000, 0, 0, 0);
        let ostream = r.calls_to(OSTREAM_CONSTRUCT);
        let copier = r.calls_to(COPIER_CONSTRUCT);
        let layouts = r.calls_to(LAYOUT_CONSTRUCT);
        let reserve = r.calls_to(MAP_RESERVE);
        assert_eq!((ostream.len(), copier.len(), layouts.len()), (1, 1, 2));
        let this = copier[0][0];
        // Frame offsets: copier 0x8c, input layout 0x28, output layout 0x48, stream 0x80,
        // bytes 0x2c, map 0x3c.
        assert_eq!(copier[0][1..], [this - 0x44, this - 0x64, 0xa5a5]);
        assert_eq!(layouts[0], [this - 0x64, HOST_LAYOUT_RULES]);
        assert_eq!(layouts[1], [this - 0x44, HOST_LAYOUT_RULES]);
        assert_eq!(ostream[0], [this - 0xc, this - 0x60]);
        assert_eq!(reserve, [[this - 0x50, r.allocator, 0]]);
        // The copier held the copy of the layout word and the argument.
        assert_eq!(*r.copier_state.borrow(), [(HOST_LAYOUT_RULES, 0xa5a5)]);
    }

    #[test]
    fn the_first_object_gets_offset_zero_and_its_class_name() {
        let mut r = rig();
        r.script(0x1011, Written::default());
        r.copy(0x1011, 0x2000, 0, 0, 0);
        let inserts = r.calls_to(MAP_INSERT);
        assert_eq!(inserts.len(), 1);
        assert_eq!(inserts[0][1..], [r.allocator, 0x1011, 0]);
        assert_eq!(r.calls_to(CLASS_GET_NAME), [[0x2000]]);
        let copied = r.calls_to(COPIER_COPY_OBJECT);
        assert_eq!(copied.len(), 1);
        let this = copied[0][0];
        // Object, class, the stream word the stream constructor made, class, the relocations.
        assert_eq!(
            copied[0][1..],
            [0x1011, 0x2000, 0x5eed_0001, 0x2000, this - 0x40]
        );
        // No class resolution without a vtable.
        assert_eq!(r.calls_to(RESOLVE_CLASS), Calls::new());
    }

    #[test]
    fn written_bytes_are_copied_into_a_block_and_local_relocations_are_patched() {
        let mut r = rig();
        r.script(
            0x1011,
            Written {
                bytes: 0x20,
                locals: vec![[0x8, 0x10], [0x0, 0x18]],
                references: vec![],
            },
        );
        let block = r.copy(0x1011, 0x2000, 0, 0, 0);
        assert_ne!(block, 0);
        assert_eq!(r.calls_to(EASY_ALLOC), [[r.allocator, 0x20]]);
        let copied = r.calls_to(MEM_COPY);
        assert_eq!(copied.len(), 1);
        assert_eq!((copied[0][0], copied[0][2]), (block, 0x20));
        assert!((0x10..0x20).all(|offset| r.e.mem.u8(block + offset) == 0x11));
        // The relocation targets are block relative.
        assert_eq!(r.e.mem.u32(block + 8), 0x10 + block);
        assert_eq!(r.e.mem.u32(block), 0x18 + block);
        // The byte array (capacity 0x100, owned) is freed last, without a reset of the size.
        let bytes: Vec<u32> = r.calls_to(FREE_BLOCK).iter().map(|c| c[2]).collect();
        assert_eq!(bytes, [48, 32, 0x100]);
    }

    #[test]
    fn references_to_new_objects_are_queued_and_patched_with_their_offsets() {
        let mut r = rig();
        r.script(
            0x1011,
            Written {
                bytes: 0x10,
                locals: vec![],
                references: vec![[4, 0x3022, 0x4000]],
            },
        );
        // The second object refers back to the first, which already has an offset.
        r.script(
            0x3022,
            Written {
                bytes: 8,
                locals: vec![],
                references: vec![[0x10, 0x1011, 0x2000]],
            },
        );
        let block = r.copy(0x1011, 0x2000, 0, 0, 0);
        let copied = r.calls_to(COPIER_COPY_OBJECT);
        assert_eq!(copied.len(), 2);
        assert_eq!(copied[1][1..3], [0x3022, 0x4000]);
        let inserts = r.calls_to(MAP_INSERT);
        let entries: Vec<[u32; 2]> = inserts.iter().map(|c| [c[2], c[3]]).collect();
        assert_eq!(entries, [[0x1011, 0], [0x3022, 0x10]]);
        assert_eq!(r.calls_to(EASY_ALLOC), [[r.allocator, 0x18]]);
        // Reference words hold the block address of the key's offset: +4 of the first object
        // is the second (offset 0x10), +0 of the second (offset 0x10) is the first (0).
        assert_eq!(r.e.mem.u32(block + 4), block + 0x10);
        assert_eq!(r.e.mem.u32(block + 0x10), block);
    }

    #[test]
    fn a_copy_map_hit_replaces_the_key_and_queues_nothing() {
        let mut r = rig();
        let copy_map = r.e.mem.alloc(0x10);
        r.maps
            .borrow_mut()
            .insert(copy_map, vec![(0x3022, 0x7777_0000)]);
        r.script(
            0x1011,
            Written {
                bytes: 0x10,
                locals: vec![],
                references: vec![[4, 0x3022, 0x4000]],
            },
        );
        let block = r.copy(0x1011, 0x2000, copy_map, 0, 0);
        assert_eq!(r.calls_to(COPIER_COPY_OBJECT).len(), 1);
        assert_eq!(r.calls_to(MAP_FIND_KEY), Calls::new());
        // The existing copy is stored as it is: it has no offset in the new block.
        assert_eq!(r.e.mem.u32(block + 4), 0x7777_0000);
        // The new object is entered into the copy map with its block address.
        let inserts = r.calls_to(MAP_INSERT);
        let into_copy_map: Vec<&Vec<u32>> =
            inserts.iter().filter(|call| call[0] == copy_map).collect();
        assert_eq!(into_copy_map.len(), 1);
        assert_eq!(into_copy_map[0][1..], [r.allocator, 0x1011, block]);
        assert_eq!(r.calls_to(MAP_GET_WITH_DEFAULT)[0], [copy_map, 0x3022, 0]);
    }

    #[test]
    fn a_copy_map_miss_falls_back_to_the_offset_map() {
        let mut r = rig();
        let copy_map = r.e.mem.alloc(0x10);
        r.script(
            0x1011,
            Written {
                bytes: 4,
                locals: vec![],
                references: vec![[0, 0x3022, 0x4000]],
            },
        );
        r.script(
            0x3022,
            Written {
                bytes: 4,
                ..Written::default()
            },
        );
        let block = r.copy(0x1011, 0x2000, copy_map, 0, 0);
        assert_eq!(r.calls_to(COPIER_COPY_OBJECT).len(), 2);
        assert_eq!(r.e.mem.u32(block), block + 4);
        // Both objects are entered into the copy map.
        let inserted: Vec<[u32; 2]> = r
            .calls_to(MAP_INSERT)
            .iter()
            .filter(|call| call[0] == copy_map)
            .map(|call| [call[2], call[3]])
            .collect();
        assert_eq!(inserted, [[0x1011, block], [0x3022, block + 4]]);
    }

    #[test]
    fn references_with_a_null_key_or_value_are_skipped() {
        let mut r = rig();
        r.script(
            0x1011,
            Written {
                bytes: 8,
                locals: vec![],
                references: vec![[0, 0, 0x4000], [4, 0x3022, 0]],
            },
        );
        let block = r.copy(0x1011, 0x2000, 0, 0, 0);
        assert_eq!(r.calls_to(MAP_FIND_KEY), Calls::new());
        assert_eq!(r.calls_to(COPIER_COPY_OBJECT).len(), 1);
        // The final pass still stores the (unmapped) keys as they are.
        assert_eq!(r.e.mem.u32(block), 0);
        assert_eq!(r.e.mem.u32(block + 4), 0x3022);
    }

    #[test]
    fn a_class_with_a_vtable_is_resolved_through_the_resolver_and_reported_to_the_sink() {
        let mut r = rig();
        r.classes_with_vtable.borrow_mut().insert(0x2000);
        let resolver = r.object_with_slot(RESOLVE_CLASS);
        let sink = r.object_with_slot(SINK_ENTRY);
        r.script(
            0x1011,
            Written {
                bytes: 8,
                locals: vec![],
                references: vec![[0, 0x3022, 0x2000]],
            },
        );
        r.script(
            0x3022,
            Written {
                bytes: 8,
                ..Written::default()
            },
        );
        let block = r.copy(0x1011, 0x2000, 0, resolver, sink);
        // Both objects use class 0x2000, which has a vtable.
        assert_eq!(
            r.calls_to(RESOLVE_CLASS),
            [[resolver, 0x1011], [resolver, 0x3022]]
        );
        let copied = r.calls_to(COPIER_COPY_OBJECT);
        assert_eq!(copied[0][2], 0x1011 + 0x200);
        assert_eq!(copied[0][4], 0x1011 + 0x200);
        assert_eq!(copied[1][2], 0x3022 + 0x200);
        // The names come from the resolved classes; the sink gets address and name.
        assert_eq!(
            r.calls_to(SINK_ENTRY),
            [
                [sink, block, 0x1011 + 0x200 + 0x100],
                [sink, block + 8, 0x3022 + 0x200 + 0x100]
            ]
        );
    }

    #[test]
    fn without_a_sink_or_a_copy_map_no_virtual_call_is_made() {
        let mut r = rig();
        r.script(
            0x1011,
            Written {
                bytes: 4,
                ..Written::default()
            },
        );
        r.copy(0x1011, 0x2000, 0, 0, 0);
        assert_eq!(r.calls_to(SINK_ENTRY), Calls::new());
        assert_eq!(r.calls_to(MAP_INSERT).len(), 1);
    }

    #[test]
    fn long_chains_grow_the_work_list_and_the_pair_array() {
        let mut r = rig();
        for i in 1..=5u32 {
            let object = i * 0x1000 + 0x11;
            let next = (i + 1) * 0x1000 + 0x11;
            let references = if i < 5 {
                vec![[(i - 1) * 4, next, 0x2000]]
            } else {
                vec![]
            };
            r.script(
                object,
                Written {
                    bytes: 4,
                    locals: vec![],
                    references,
                },
            );
        }
        let block = r.copy(0x1011, 0x2000, 0, 0, 0);
        assert_eq!(r.calls_to(COPIER_COPY_OBJECT).len(), 5);
        let grows = r.calls_to(ARRAY_GROW);
        let work: Vec<&Vec<u32>> = grows.iter().filter(|call| call[2] == 8).collect();
        let pairs: Vec<&Vec<u32>> = grows.iter().filter(|call| call[2] == 0xc).collect();
        assert_eq!((work.len(), pairs.len()), (2, 2));
        // Each object's reference word is the block address of the next one.
        for i in 0..4u32 {
            assert_eq!(r.e.mem.u32(block + i * 4), block + (i + 1) * 4);
        }
        let bytes: Vec<u32> = r.calls_to(FREE_BLOCK).iter().map(|c| c[2]).collect();
        assert_eq!(bytes, [96, 64, 0x100]);
    }

    #[test]
    fn a_key_referenced_twice_before_its_turn_is_copied_twice_and_keeps_the_later_offset() {
        let mut r = rig();
        r.script(
            0x1011,
            Written {
                bytes: 0x10,
                locals: vec![],
                references: vec![[0, 0x3022, 0x2000], [4, 0x3022, 0x2000]],
            },
        );
        r.script(
            0x3022,
            Written {
                bytes: 4,
                ..Written::default()
            },
        );
        r.copy(0x1011, 0x2000, 0, 0, 0);
        // The key has no offset until its turn, so both references queue it; the second copy
        // enters it into the map again, with its later offset.
        let copied = r.calls_to(COPIER_COPY_OBJECT);
        assert_eq!(copied.len(), 3);
        let maps = r.maps.borrow();
        let offsets: Vec<(u32, u32)> = maps.values().flatten().copied().collect();
        assert_eq!(offsets, [(0x1011, 0), (0x3022, 0x14)]);
    }
}
