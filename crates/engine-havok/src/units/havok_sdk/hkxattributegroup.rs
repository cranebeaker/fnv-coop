//! `Havok SDK/hkxattributegroup.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator whose slot `+0x10`
/// (`bufFree(block, size)`) gives blocks back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve(allocator, array, capacity, element size)` (`00c908c0`, cdecl).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`), `thiscall`
/// on the object, no stack words.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkStringPtr`: the destructor (`00cb0c00`), the copy constructor (`00cb0bd0`, one stack word:
/// the source) and the assignment from another `hkStringPtr` (`00cb0c30`, one stack word),
/// all `thiscall`.
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_COPY_CONSTRUCT: u32 = 0x00cb_0bd0;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c30;
/// `hkxAttribute::hkxAttribute` (`00e3f8e0`).
const ATTRIBUTE_CONSTRUCT: u32 = 0x00e3_f8e0;
/// `hkMemUtil::memMove(destination, source, byte count)` (`00d2cc70`, cdecl).
const MEM_MOVE: u32 = 0x00d2_cc70;
/// `hkxAttributeHolder::findAttributeObjectByName` (`00e5c540`, `thiscall`, `RET 8`: the name
/// and a class record).
const FIND_ATTRIBUTE_OBJECT_BY_NAME: u32 = 0x00e5_c540;
/// The class records the two finders pass: the quaternion's (`0126ea80`) and the matrix's
/// (`0126eac8`).
const QUATERNION_CLASS: u32 = 0x0126_ea80;
const MATRIX_CLASS: u32 = 0x0126_eac8;
/// The error text pieces and the error handler: `hkOstream::hkOstream(buffer, size)`
/// (`00cb1600`, `thiscall`, `RET 8`), `operator<<(const char *)` (`00cc01c0`, `RET 4`, answers
/// the stream), `operator<<(const hkStringPtr &)` (`00cc05a0`, `RET 4`), the stream destructor
/// (`00cc00e0`) and the error handler object (the pointer at `01268360`; slot `+0xc` takes
/// `(severity 1, id, text, file, line)`).
const OSTREAM_CONSTRUCT: u32 = 0x00cb_1600;
const OSTREAM_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_STRING_PTR: u32 = 0x00cc_05a0;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_HANDLER: u32 = 0x0126_8360;
/// The text `"Quaternion attribute "`, `"Matrix attribute "`, `" not found in "`,
/// `" attribute group"` and the file name `".\Attributes\hkxAttributeGroup.cpp"`.
const TEXT_QUATERNION_ATTRIBUTE: u32 = 0x010e_cd5c;
const TEXT_MATRIX_ATTRIBUTE: u32 = 0x010e_cd74;
const TEXT_NOT_FOUND_IN: u32 = 0x010e_ccfc;
const TEXT_ATTRIBUTE_GROUP: u32 = 0x010e_cd0c;
const SOURCE_FILE_GROUP: u32 = 0x010e_ccc8;
/// The error id of both messages.
const ERROR_ID_ATTRIBUTE_MISSING: u32 = 0xabba_ab81;
/// The size of the message buffer and the layout of the stack block holding the `hkOstream`
/// (12 bytes) and the buffer after it.
const MESSAGE_SIZE: u32 = 0x200;

fn group_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Reports `"<kind> attribute <name> not found in <group name> attribute group"` (error id
/// `0xabbaab81`, the file `hkxAttributeGroup.cpp`, `line`) through the error handler: an
/// `hkOstream` over a 0x200-byte buffer, the three texts and the group's name
/// (`operator<<(hkStringPtr)` on `this`, whose first field is its name), each call taking the
/// stream the previous one answered.
fn report_attribute_missing(e: &mut Engine, this: Ptr, kind: u32, name: u32, line: u32) {
    e.with_stack(0xc + MESSAGE_SIZE, |e, stream| {
        let buffer = stream.addr() + 0xc;
        e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, MESSAGE_SIZE]);
        let s = e.call(OSTREAM_TEXT, &args![stream, kind]).u32();
        let s = e.call(OSTREAM_TEXT, &args![s, name]).u32();
        let s = e.call(OSTREAM_TEXT, &args![s, TEXT_NOT_FOUND_IN]).u32();
        let s = e.call(OSTREAM_STRING_PTR, &args![s, this]).u32();
        e.call(OSTREAM_TEXT, &args![s, TEXT_ATTRIBUTE_GROUP]);
        let handler: u32 = e.global(ERROR_HANDLER);
        e.vcall(
            handler,
            0xc,
            &args![
                1u32,
                ERROR_ID_ATTRIBUTE_MISSING,
                buffer,
                SOURCE_FILE_GROUP,
                line
            ],
        );
        e.call(OSTREAM_DESTRUCT, &args![stream]);
    });
}

// Translated from 00e5c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `name`): `findAttributeObjectByName`
/// (`00e5c540`) with the class record at `0126ea80` (the quaternion's: the group's
/// `getQuaternionValue` uses it). Returns what that returns.
pub fn fn_00e5c670(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    e.call(
        FIND_ATTRIBUTE_OBJECT_BY_NAME,
        &args![this, name, QUATERNION_CLASS],
    )
    .u32()
}

// Translated from 00e5c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `name`): `findAttributeObjectByName`
/// (`00e5c540`) with the class record at `0126eac8` (the matrix's). Returns what that returns.
pub fn fn_00e5c690(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    e.call(
        FIND_ATTRIBUTE_OBJECT_BY_NAME,
        &args![this, name, MATRIX_CLASS],
    )
    .u32()
}

// Translated from 00e5cb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttributeGroup::getQuaternionValue` (Xbox PDB), `thiscall`, `RET 0xc` (`name`,
/// `report`, `out`): looks the attribute up (`00e5c670`); found, the 16 bytes at the object's
/// `+8` are copied to `out` and 0 is returned. Not found, the error `"Quaternion attribute
/// <name> not found in <group> attribute group"` is reported when the `report` byte is not
/// zero, and 1 is returned.
pub fn hkx_attribute_group_get_quaternion_value(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    report: u8,
    out: Ptr,
) -> u32 {
    let attribute = fn_00e5c670(e, this, name);
    if attribute != 0 {
        let value = e.mem.u32(attribute + 8);
        copy_16(e, value, out.addr());
        return 0;
    }
    if report != 0 {
        report_attribute_missing(e, this, TEXT_QUATERNION_ATTRIBUTE, name, 0xb3);
    }
    1
}

// Translated from 00e5cc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttributeGroup::getMatrixValue` (Xbox PDB), `thiscall`, `RET 0xc` (`name`, `report`,
/// `out`): like `getQuaternionValue` with the matrix class record (`00e5c690`), copying the 64
/// bytes at the object's `+8` and reporting `"Matrix attribute ..."` (line 0xc4).
pub fn hkx_attribute_group_get_matrix_value(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    report: u8,
    out: Ptr,
) -> u32 {
    let attribute = fn_00e5c690(e, this, name);
    if attribute != 0 {
        let value = e.mem.u32(attribute + 8);
        for row in 0..4 {
            copy_16(e, value + 0x10 * row, out.addr() + 0x10 * row);
        }
        return 0;
    }
    if report != 0 {
        report_attribute_missing(e, this, TEXT_MATRIX_ATTRIBUTE, name, 0xc4);
    }
    1
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_16(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00e5cce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttributeGroup::operator=` (Xbox PDB), `thiscall`, `RET 4` (`other`): copies the name
/// (`hkStringPtr` assignment, `00cb0c30`), then empties the attribute array (data at `+4`,
/// size at `+8`, capacity word at `+0xc`; the entries are a name `hkStringPtr` and a value
/// reference, 8 bytes each): the capacity is brought to twice the masked capacity when that
/// is negative (never, the mask clears the sign), every entry's value is released
/// (`00c905b0`) and set to null and its name destroyed, and, for a negative size, entries are
/// constructed (`00e3f8e0`, never for a size that is not negative); then the size is zero.
/// Finally the entries of `other` are inserted at index 0 (`00e5cec0`). Returns `this`.
pub fn hkx_attribute_group_assign(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(STRING_PTR_ASSIGN, &args![this, other]);
    let array = this.addr() + 4;
    let allocator = group_allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < 0 {
        // The masked capacity is never negative: the exe's `JGE` always skips this.
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if doubled <= 0 { 0 } else { doubled };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 8u32]);
    }
    let size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    let mut entry = data;
    let mut left = size;
    while left > 0 {
        let value = e.mem.u32(entry + 4);
        if value != 0 {
            e.call(REMOVE_REFERENCE, &args![value]);
        }
        e.mem.set_u32(entry + 4, 0);
        e.call(STRING_PTR_DESTRUCT, &args![entry]);
        entry = entry.wrapping_add(8);
        left -= 1;
    }
    let size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    let mut missing = size.wrapping_neg();
    let mut at = data.wrapping_add((size as u32).wrapping_mul(8));
    while missing > 0 {
        if at != 0 {
            e.call(ATTRIBUTE_CONSTRUCT, &args![at]);
        }
        at = at.wrapping_add(8);
        missing -= 1;
    }
    e.mem.set_u32(array + 4, 0);
    let other_size = e.mem.u32(other.addr() + 8);
    if other_size != 0 {
        let other_data = e.mem.u32(other.addr() + 4);
        let allocator = group_allocator(e);
        fn_00e5cec0(
            e,
            Ptr::new(array),
            allocator,
            0,
            other_data,
            other_size as i32,
        );
    }
    this
}

// Translated from 00e5cdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayBase<hkxAttribute>::_spliceInto` (Xbox PDB), `thiscall` on the array (data `+0`,
/// size `+4`, capacity word `+8`; 8-byte entries), `RET 0x14` (`allocator`, `index`,
/// `delete_count`, `source`, `insert_count`): the new size is `size - delete_count +
/// insert_count`; when the masked capacity is below it the array is reserved for the larger
/// of twice the capacity and the new size (`00c908c0`, element size 8). The `delete_count`
/// entries from `index` have their value released (`00c905b0`) and nulled and their name
/// destroyed; the tail is moved (`memMove`, `(size - index - delete_count) * 8` bytes) to
/// `index + insert_count`; the `insert_count` entries of `source` are copied into the gap
/// (name copy-constructed, value reference added with `00c90510` when not null, stored).
/// The size is stored last.
pub fn hk_array_base_hkx_attribute_splice_into(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    delete_count: i32,
    source: u32,
    insert_count: i32,
) {
    let array = this.addr();
    let size = e.mem.i32(array + 4);
    let tail = size.wrapping_sub(index).wrapping_sub(delete_count);
    let new_size = size.wrapping_sub(delete_count).wrapping_add(insert_count);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 8u32]);
    }
    let first = e
        .mem
        .u32(array)
        .wrapping_add((index as u32).wrapping_mul(8));
    let mut entry = first.wrapping_add(4);
    let mut left = delete_count;
    while left > 0 {
        let value = e.mem.u32(entry);
        if value != 0 {
            e.call(REMOVE_REFERENCE, &args![value]);
        }
        e.mem.set_u32(entry, 0);
        e.call(STRING_PTR_DESTRUCT, &args![entry.wrapping_sub(4)]);
        entry = entry.wrapping_add(8);
        left -= 1;
    }
    let data = e.mem.u32(array);
    let destination = data.wrapping_add((index.wrapping_add(insert_count) as u32).wrapping_mul(8));
    let from = data.wrapping_add((index.wrapping_add(delete_count) as u32).wrapping_mul(8));
    e.call(
        MEM_MOVE,
        &args![destination, from, (tail as u32).wrapping_mul(8)],
    );
    let mut at = e
        .mem
        .u32(array)
        .wrapping_add((index as u32).wrapping_mul(8));
    let mut from_entry = source;
    let mut left = insert_count;
    while left > 0 {
        if at != 0 {
            e.call(STRING_PTR_COPY_CONSTRUCT, &args![at, from_entry]);
            let value = e.mem.u32(from_entry + 4);
            if value != 0 {
                e.call(ADD_REFERENCE, &args![value]);
            }
            let value = e.mem.u32(from_entry + 4);
            e.mem.set_u32(at + 4, value);
        }
        at = at.wrapping_add(8);
        from_entry = from_entry.wrapping_add(8);
        left -= 1;
    }
    e.mem.set_u32(array + 4, new_size as u32);
}

// Translated from 00e5cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: `allocator`, `index`, `source`,
/// `count`): `_spliceInto` (`00e5cdb0`) with nothing deleted: inserts `count` entries of
/// `source` at `index`.
pub fn fn_00e5cec0(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32, count: i32) {
    hk_array_base_hkx_attribute_splice_into(e, this, allocator, index, 0, source, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e5c670, fn_00e5c670(Ptr, u32) -> u32),
        entry!(0x00e5c690, fn_00e5c690(Ptr, u32) -> u32),
        entry!(0x00e5cb40, hkx_attribute_group_get_quaternion_value(Ptr, u32, u8, Ptr) -> u32),
        entry!(0x00e5cc00, hkx_attribute_group_get_matrix_value(Ptr, u32, u8, Ptr) -> u32),
        entry!(0x00e5cce0, hkx_attribute_group_assign(Ptr, Ptr) -> Ptr),
        entry!(
            0x00e5cdb0,
            hk_array_base_hkx_attribute_splice_into(Ptr, u32, i32, i32, u32, i32)
        ),
        entry!(0x00e5cec0, fn_00e5cec0(Ptr, u32, i32, u32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Registers a double at `addr` that logs its argument words and answers `answer`.
    fn log_calls(e: &mut Engine, addr: u32, answer: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let kept = log.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    const ALLOCATOR_OBJECT: u32 = 0x0060_0000;
    const ALLOCATOR_TABLE: u32 = 0x0060_1000;
    const DELETE_DOUBLE: u32 = 0x0060_2008;
    const FREE_DOUBLE: u32 = 0x0060_2010;

    /// A memory router (`00c85750`) whose allocator logs `[block, size]` for slot `+0x10`
    /// (`bufFree`, the first log) and for slot `+8` (the second log).
    fn install_memory(e: &mut Engine) -> (Log, Log) {
        e.map(ALLOCATOR_OBJECT, 0x100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR_OBJECT);
        e.mem.set_u32(ALLOCATOR_OBJECT, ALLOCATOR_TABLE);
        let mut table = vec![0u32; 5];
        table[2] = DELETE_DOUBLE;
        table[4] = FREE_DOUBLE;
        e.put_vtable(ALLOCATOR_TABLE, &table);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees: Log = Rc::new(RefCell::new(Vec::new()));
        let deletes: Log = Rc::new(RefCell::new(Vec::new()));
        let log = frees.clone();
        e.register_double(FREE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        let log = deletes.clone();
        e.register_double(DELETE_DOUBLE, move |_, a| {
            log.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        (frees, deletes)
    }

    type Events = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// Registers a double at `addr` that appends `(tag, arguments)` to `events` and answers
    /// `answer`.
    fn tag_calls(e: &mut Engine, events: &Events, addr: u32, tag: &'static str, answer: u32) {
        let kept = events.clone();
        e.register_double(addr, move |_, a| {
            kept.borrow_mut().push((tag, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    /// The string and reference doubles the array code uses: the copy constructor copies the
    /// word, `memMove` moves bytes.
    fn install_array_doubles(e: &mut Engine, events: &Events) {
        tag_calls(e, events, STRING_PTR_DESTRUCT, "destroy", 0);
        tag_calls(e, events, REMOVE_REFERENCE, "remove", 0);
        tag_calls(e, events, ADD_REFERENCE, "add", 0);
        let kept = events.clone();
        e.register_double(STRING_PTR_COPY_CONSTRUCT, move |e, a| {
            kept.borrow_mut().push(("copy", a.to_vec()));
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        let kept = events.clone();
        e.register_double(MEM_MOVE, move |e, a| {
            kept.borrow_mut().push(("move", a.to_vec()));
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    /// An attribute object whose value pointer (at +8) points at 64 words 0x100 + i.
    fn attribute_with_value(e: &mut Engine) -> (u32, u32) {
        let object = e.mem.alloc(16);
        let value = e.mem.alloc(64);
        for word in 0..16u32 {
            e.mem.set_u32(value + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(object + 8, value);
        (object, value)
    }

    #[test]
    fn the_two_finders_pass_their_class_records() {
        let mut e = Engine::new();
        let log = log_calls(&mut e, FIND_ATTRIBUTE_OBJECT_BY_NAME, 0x4444);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00e5_c670, &args![this, 0x5000u32]).u32(), 0x4444);
        assert_eq!(e.call(0x00e5_c690, &args![this, 0x5001u32]).u32(), 0x4444);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![this, 0x5000, 0x0126_ea80],
                vec![this, 0x5001, 0x0126_eac8]
            ]
        );
    }

    #[test]
    fn a_found_quaternion_is_copied_without_a_message() {
        let mut e = Engine::new();
        let (object, value) = attribute_with_value(&mut e);
        e.register_double(FIND_ATTRIBUTE_OBJECT_BY_NAME, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        let constructed = log_calls(&mut e, OSTREAM_CONSTRUCT, 0);
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x40);
        e.mem.write(out, &[0xee; 0x40]);
        assert_eq!(
            e.call(0x00e5_cb40, &args![this, 0x5000u32, 1u8, out]).u32(),
            0
        );
        for word in 0..4u32 {
            assert_eq!(e.mem.u32(out + 4 * word), e.mem.u32(value + 4 * word));
        }
        // Only 16 bytes are copied.
        assert_eq!(e.mem.u32(out + 0x10), 0xeeee_eeee);
        assert!(constructed.borrow().is_empty());
    }

    #[test]
    fn a_found_matrix_copies_64_bytes() {
        let mut e = Engine::new();
        let (object, value) = attribute_with_value(&mut e);
        e.register_double(FIND_ATTRIBUTE_OBJECT_BY_NAME, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00e5_cc00, &args![this, 0x5000u32, 0u8, out]).u32(),
            0
        );
        assert_eq!(e.mem.bytes(out, 0x40), e.mem.bytes(value, 0x40));
    }

    /// Doubles for the message: every call is logged, each stream call answers the stream
    /// plus one, and the error handler (the object at `01268360`) logs its five words.
    fn install_message_doubles(e: &mut Engine, events: &Events) {
        tag_calls(e, events, OSTREAM_CONSTRUCT, "construct", 0);
        for (address, tag) in [(OSTREAM_TEXT, "text"), (OSTREAM_STRING_PTR, "string")] {
            let kept = events.clone();
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((tag, a.to_vec()));
                Ret {
                    eax: a[0] + 1,
                    ..Ret::default()
                }
            });
        }
        tag_calls(e, events, OSTREAM_DESTRUCT, "destruct", 0);
        e.map(ERROR_HANDLER, 4);
        let handler = e.mem.alloc(8);
        e.mem.set_u32(ERROR_HANDLER, handler);
        e.put_vtable(0x0061_0000, &[0, 0, 0, 0x0061_1000]);
        e.mem.set_u32(handler, 0x0061_0000);
        tag_calls(e, events, 0x0061_1000, "report", 0);
    }

    #[test]
    fn a_missing_quaternion_is_reported_when_asked() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_message_doubles(&mut e, &events);
        e.register(FIND_ATTRIBUTE_OBJECT_BY_NAME, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x40);
        // Quiet: no message.
        assert_eq!(
            e.call(0x00e5_cb40, &args![this, 0x5000u32, 0u8, out]).u32(),
            1
        );
        assert!(events.borrow().is_empty());
        assert_eq!(
            e.call(0x00e5_cb40, &args![this, 0x5000u32, 1u8, out]).u32(),
            1
        );
        let events = events.borrow();
        let tags: Vec<&str> = events.iter().map(|(tag, _)| *tag).collect();
        assert_eq!(
            tags,
            vec![
                "construct",
                "text",
                "text",
                "text",
                "string",
                "text",
                "report",
                "destruct"
            ]
        );
        let stream = events[0].1[0];
        let buffer = stream + 0xc;
        assert_eq!(events[0].1, vec![stream, buffer, 0x200]);
        // Each call takes the stream the previous one answered.
        assert_eq!(events[1].1, vec![stream, TEXT_QUATERNION_ATTRIBUTE]);
        assert_eq!(events[2].1, vec![stream + 1, 0x5000]);
        assert_eq!(events[3].1, vec![stream + 2, TEXT_NOT_FOUND_IN]);
        assert_eq!(events[4].1, vec![stream + 3, this]);
        assert_eq!(events[5].1[1..], [TEXT_ATTRIBUTE_GROUP]);
        assert_eq!(events[5].1[0], stream + 4);
        let handler = e.mem.u32(ERROR_HANDLER);
        assert_eq!(
            events[6].1,
            vec![handler, 1, 0xabba_ab81, buffer, SOURCE_FILE_GROUP, 0xb3]
        );
        assert_eq!(events[7].1, vec![stream]);
    }

    #[test]
    fn a_missing_matrix_names_the_matrix_and_the_other_line() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_message_doubles(&mut e, &events);
        e.register(FIND_ATTRIBUTE_OBJECT_BY_NAME, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x40);
        assert_eq!(
            e.call(0x00e5_cc00, &args![this, 0x5000u32, 2u8, out]).u32(),
            1
        );
        let events = events.borrow();
        assert_eq!(events[1].1[1], TEXT_MATRIX_ATTRIBUTE);
        assert_eq!(events[6].1[5], 0xc4);
    }

    #[test]
    fn assignment_copies_the_name_empties_the_array_and_inserts_the_other_entries() {
        let mut e = Engine::new();
        install_memory(&mut e);
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_array_doubles(&mut e, &events);
        tag_calls(&mut e, &events, STRING_PTR_ASSIGN, "assign", 0);
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data, 0x91);
        e.mem.set_u32(data + 4, 0xa1);
        e.mem.set_u32(data + 8, 0x92);
        e.mem.set_u32(data + 12, 0xa2);
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, 2);
        e.mem.set_u32(this + 0xc, 4);
        let other = e.mem.alloc(0x10);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0xb1);
        e.mem.set_u32(source + 4, 0xc1);
        e.mem.set_u32(other + 4, source);
        e.mem.set_u32(other + 8, 1);
        assert_eq!(e.call(0x00e5_cce0, &args![this, other]).u32(), this);
        assert_eq!(
            *events.borrow(),
            vec![
                ("assign", vec![this, other]),
                ("remove", vec![0xa1]),
                ("destroy", vec![data]),
                ("remove", vec![0xa2]),
                ("destroy", vec![data + 8]),
                ("move", vec![data + 8, data, 0]),
                ("copy", vec![data, source]),
                ("add", vec![0xc1]),
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(data), 0xb1);
        assert_eq!(e.mem.u32(data + 4), 0xc1);
        // An empty other leaves an empty array and does not insert.
        events.borrow_mut().clear();
        e.mem.set_u32(other + 8, 0);
        e.call(0x00e5_cce0, &args![this, other]);
        let tags: Vec<&str> = events.borrow().iter().map(|(tag, _)| *tag).collect();
        assert_eq!(tags, vec!["assign", "remove", "destroy"]);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn splice_reserves_removes_moves_the_tail_and_inserts() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_array_doubles(&mut e, &events);
        let kept = events.clone();
        e.register_double(ARRAY_RESERVE, move |e, a| {
            kept.borrow_mut().push(("reserve", a.to_vec()));
            let old_size = e.mem.u32(a[1] + 4);
            let old = e.mem.u32(a[1]);
            let new = e.mem.alloc(8 * a[2]);
            let bytes = e.mem.bytes(old, 8 * old_size);
            e.mem.write(new, &bytes);
            e.mem.set_u32(a[1], new);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(24);
        for (index, (name, value)) in [(0x10u32, 0x20u32), (0x11, 0x21), (0x12, 0x22)]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(data + 8 * index as u32, *name);
            e.mem.set_u32(data + 8 * index as u32 + 4, *value);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 0x4000_0003);
        let source = e.mem.alloc(16);
        e.mem.set_u32(source, 0x30);
        e.mem.set_u32(source + 4, 0x40);
        e.mem.set_u32(source + 8, 0x31);
        e.mem.set_u32(source + 12, 0x41);
        e.call(
            0x00e5_cdb0,
            &args![array, 0xa11cu32, 1i32, 1i32, source, 2i32],
        );
        // New size 4 is above the capacity 3: reserve for max(6, 4) = 6 entries of 8 bytes.
        let new_data = e.mem.u32(array);
        assert_ne!(new_data, data);
        assert_eq!(
            *events.borrow(),
            vec![
                ("reserve", vec![0xa11c, array, 6, 8]),
                ("remove", vec![0x21]),
                ("destroy", vec![new_data + 8]),
                ("move", vec![new_data + 24, new_data + 16, 8]),
                ("copy", vec![new_data + 8, source]),
                ("add", vec![0x40]),
                ("copy", vec![new_data + 16, source + 8]),
                ("add", vec![0x41]),
            ]
        );
        assert_eq!(e.mem.u32(array + 4), 4);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(new_data + 4 * i)).collect();
        assert_eq!(words, vec![0x10, 0x20, 0x30, 0x40, 0x31, 0x41, 0x12, 0x22]);
    }

    #[test]
    fn splice_with_room_does_not_reserve_and_the_insert_form_deletes_nothing() {
        let mut e = Engine::new();
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        install_array_doubles(&mut e, &events);
        tag_calls(&mut e, &events, ARRAY_RESERVE, "reserve", 0);
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(32);
        e.mem.set_u32(data, 0x10);
        e.mem.set_u32(data + 4, 0x20);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 4);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x30);
        e.mem.set_u32(source + 4, 0);
        e.call(0x00e5_cec0, &args![array, 0xa11cu32, 1i32, source, 1i32]);
        // A null value is stored without a reference being added.
        let tags: Vec<&str> = events.borrow().iter().map(|(tag, _)| *tag).collect();
        assert_eq!(tags, vec!["move", "copy"]);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(e.mem.u32(data + 8), 0x30);
        assert_eq!(e.mem.u32(data + 12), 0);
        assert_eq!(e.mem.u32(data), 0x10);
    }
}
