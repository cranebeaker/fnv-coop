//! `Havok SDK/hkxattributeholder.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator whose slot `+0x10`
/// (`bufFree(block, size)`) gives blocks back.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve(allocator, array, capacity, element size)` (`00c908c0`, cdecl).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkString::strCasecmp(a, b)` (`00cb0fe0`, cdecl).
const STRING_CASECMP: u32 = 0x00cb_0fe0;
/// `hkxAttributeGroup`: the destructor (`00dfd840`) and the assignment (`00e5cce0`, one stack
/// word); `hkStringPtr::hkStringPtr()` (`00cb0a60`).
const GROUP_DESTRUCT: u32 = 0x00df_d840;
const GROUP_ASSIGN: u32 = 0x00e5_cce0;
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
/// The reference pointer copy (`00d47750`, `thiscall` on the destination, one stack word: the
/// source's address; adds a reference and answers the destination), `hkRefVariant::getClass`
/// (`00d477a0`, `thiscall` on the variant) and `hkClass::getName` (`00d46d40`, `thiscall`).
const REF_PTR_COPY: u32 = 0x00d4_7750;
const REF_VARIANT_GET_CLASS: u32 = 0x00d4_77a0;
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
/// The holder's vtable (`010e310c`) and the base class's (`0102e388`) written by the
/// constructor and destructor.
const HOLDER_VTABLE: u32 = 0x010e_310c;
const BASE_VTABLE: u32 = 0x0102_e388;

fn holder_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00e3e950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttributeHolder::hkxAttributeHolder` (Xbox PDB), the copy constructor, `thiscall`,
/// `RET 4` (`other`): sets the 16-bit word at `+6` to 1 and the vtable, clears the array of
/// attribute groups at `+8` (data, size at `+0xc`, capacity word at `+0x10` = `0x80000000`)
/// and copies `other`'s groups into it: with `common` the smaller of the (empty) own size and
/// `other`'s count `n`, the array is reserved for `n` entries of 0x10 bytes when the masked
/// capacity is below `n` (twice the capacity or `n`, whichever is larger); an own size above
/// `n` would have its extra groups destroyed (`00dfd840`) and the first `common` groups
/// assigned (`00e5cce0`); the groups from `common` to `n` are constructed (name
/// `hkStringPtr`, empty array with capacity word `0x80000000`) and assigned from `other`'s.
/// The size is stored last. Returns `this`.
pub fn hkx_attribute_holder_hkx_attribute_holder(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, HOLDER_VTABLE);
    let array = base + 8;
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let allocator = holder_allocator(e);
    let own_size = e.mem.i32(array + 4);
    let count = e.mem.i32(other.addr() + 0xc);
    let common = if count > own_size { own_size } else { count };
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
    }
    let mut extra = e
        .mem
        .u32(array)
        .wrapping_add((count as u32).wrapping_shl(4));
    let mut surplus = own_size.wrapping_sub(count);
    while surplus > 0 {
        e.call(GROUP_DESTRUCT, &args![extra]);
        extra = extra.wrapping_add(0x10);
        surplus -= 1;
    }
    let other_data = e.mem.u32(other.addr() + 8);
    let own_data = e.mem.u32(array);
    let mut at = own_data;
    let mut left = common;
    while left > 0 {
        let source = at.wrapping_add(other_data.wrapping_sub(own_data));
        e.call(GROUP_ASSIGN, &args![at, source]);
        at = at.wrapping_add(0x10);
        left -= 1;
    }
    let offset = (common as u32).wrapping_shl(4);
    let source_base = e.mem.u32(other.addr() + 8).wrapping_add(offset);
    let mut at = e.mem.u32(array).wrapping_add(offset);
    let mut left = count.wrapping_sub(common);
    let delta = source_base.wrapping_sub(at);
    while left > 0 {
        if at != 0 {
            e.call(STRING_PTR_CONSTRUCT, &args![at]);
            e.mem.set_u32(at + 4, 0);
            e.mem.set_u32(at + 8, 0);
            e.mem.set_u32(at + 0xc, 0x8000_0000);
            e.call(GROUP_ASSIGN, &args![at, at.wrapping_add(delta)]);
        }
        at = at.wrapping_add(0x10);
        left -= 1;
    }
    e.mem.set_u32(array + 4, count as u32);
    this
}

// Translated from 00e3eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttributeHolder::~hkxAttributeHolder` (Xbox PDB), `thiscall`, no stack words: sets the
/// holder vtable, destroys the attribute groups (`00dfd840`, 0x10 bytes each, the count at
/// `+0xc` read after the memory router's allocator), clears the size, gives the array
/// back (`bufFree(data, (capacity & 0x3fffffff) * 16)`) unless its capacity word at `+0x10`
/// is negative, clears the data pointer, sets the capacity word to `0x80000000` and the vtable
/// to the base class's.
pub fn hkx_attribute_holder_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, HOLDER_VTABLE);
    let allocator = holder_allocator(e);
    let count = e.mem.i32(base + 0xc);
    let mut group = e.mem.u32(base + 8);
    let mut left = count;
    while left > 0 {
        e.call(GROUP_DESTRUCT, &args![group]);
        group = group.wrapping_add(0x10);
        left -= 1;
    }
    let capacity = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0xc, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 8);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_shl(4)],
        );
    }
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0x8000_0000);
    e.mem.set_u32(base, BASE_VTABLE);
}

// Translated from 00e5c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `name`): the index of the first entry of
/// the (name, value) array at `+4` (data) and `+8` (size), 8 bytes an entry, whose name
/// (the `hkStringPtr` word with its low bit cleared) equals `name` case-insensitively
/// (`hkString::strCasecmp`, `00cb0fe0`); -1 when there is none. The body reads an attribute
/// group's layout.
pub fn fn_00e5c490(e: &mut Engine, this: Ptr, name: u32) -> i32 {
    let base = this.addr();
    let mut index = 0i32;
    while index < e.mem.i32(base + 8) {
        let data = e.mem.u32(base + 4);
        let entry_name = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(8))) & 0xffff_fffe;
        if e.call(STRING_CASECMP, &args![entry_name, name]).u32() == 0 {
            return index;
        }
        index += 1;
    }
    -1
}

// Translated from 00e5c4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttributeHolder::findAttributeVariantByName` (Xbox PDB, the engine map's name; the body
/// reads an attribute group's layout), `thiscall`, `RET 8` (`out`, `name`): `*out` becomes the
/// value reference of the entry `00e5c490` finds, with a reference added (`00c90510`) when it is
/// not null. When there is no entry the value comes from a temporary reference pointer copied
/// (`00d47750`) from a null one, so `*out` is null, and the temporary (kept in the stack
/// word the exe reuses) is released if it holds something. Returns `out`.
pub fn hkx_attribute_holder_find_attribute_variant_by_name(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    name: u32,
) -> Ptr {
    let index = fn_00e5c490(e, this, name);
    // The exe's frame (`SUB ESP,0x8`, two pushes) taken with the arguments: the null source at
    // +0x8 (00e5c4e7); the temporary is the `name` argument's own slot, +0x18 past the return
    // address at +0x10 (00e5c4f8).
    e.with_stack(0x1c, |e, frame| {
        let frame = frame.addr();
        let null_source = frame + 8;
        e.mem.set_u32(null_source, 0);
        e.mem.set_u32(null_source + 4, 0);
        let temporary = frame + 0x18;
        e.mem.set_u32(frame + 0x14, out.addr());
        e.mem.set_u32(temporary, name);
        let slot;
        if index >= 0 {
            slot = e
                .mem
                .u32(this.addr() + 4)
                .wrapping_add((index as u32).wrapping_mul(8))
                .wrapping_add(4);
        } else {
            slot = e.call(REF_PTR_COPY, &args![temporary, null_source]).u32();
        }
        let value = e.mem.u32(slot);
        if value != 0 {
            e.call(ADD_REFERENCE, &args![value]);
        }
        let value = e.mem.u32(slot);
        e.mem.set_u32(out.addr(), value);
        if index < 0 {
            let held = e.mem.u32(temporary);
            if held != 0 {
                e.call(REMOVE_REFERENCE, &args![held]);
            }
        }
    });
    out
}

// Translated from 00e5c540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAttributeHolder::findAttributeObjectByName` (Xbox PDB), `thiscall`, `RET 8` (`name`,
/// `class_record`): the object of the variant `findAttributeVariantByName` (`00e5c4d0`) finds
/// for `name`, or 0 when there is none. With a class record given, the variant's class name
/// (`hkRefVariant::getClass` `00d477a0`, then `hkClass::getName` `00d46d40`) must equal the
/// record's name case-insensitively; when it does not the object is released
/// (`00c905b0`) and 0 is returned. The reference the variant lookup added is released
/// (`00c905b0`) before the object is returned.
pub fn hkx_attribute_holder_find_attribute_object_by_name(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    class_record: u32,
) -> u32 {
    // The exe's frame (one push) taken with the arguments: the variant is the `name` argument's
    // own slot, +0x8 past the return address at +0x4 (00e5c546).
    e.with_stack(0x10, |e, frame| {
        let variant = frame.addr() + 8;
        e.mem.set_u32(variant, name);
        e.mem.set_u32(variant + 4, class_record);
        hkx_attribute_holder_find_attribute_variant_by_name(e, this, Ptr::new(variant), name);
        let object = e.mem.u32(variant);
        if object == 0 {
            0
        } else {
            let mut matches = true;
            if class_record != 0 {
                let class = e.call(REF_VARIANT_GET_CLASS, &args![variant]).u32();
                let variant_name = e.call(CLASS_GET_NAME, &args![class]).u32();
                let record_name = e.call(CLASS_GET_NAME, &args![class_record]).u32();
                matches = e
                    .call(STRING_CASECMP, &args![record_name, variant_name])
                    .u32()
                    == 0;
            }
            let held = e.mem.u32(variant);
            if held != 0 {
                e.call(REMOVE_REFERENCE, &args![held]);
            }
            if matches {
                held
            } else {
                0
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3e950, hkx_attribute_holder_hkx_attribute_holder(Ptr, Ptr) -> Ptr),
        entry!(0x00e3eaa0, hkx_attribute_holder_destructor(Ptr)),
        entry!(0x00e5c490, fn_00e5c490(Ptr, u32) -> i32),
        entry!(0x00e5c4d0, hkx_attribute_holder_find_attribute_variant_by_name(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00e5c540, hkx_attribute_holder_find_attribute_object_by_name(Ptr, u32, u32) -> u32),
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

    fn put_text(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text);
        at
    }

    /// A case-insensitive `strCasecmp` double over C strings in memory.
    fn install_casecmp(e: &mut Engine) {
        e.register(STRING_CASECMP, |e, a| {
            let left: Vec<u8> = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right: Vec<u8> = e.mem.cstr(a[1]).to_ascii_lowercase();
            Ret {
                eax: u32::from(left != right),
                ..Ret::default()
            }
        });
    }

    /// An attribute group (name at +0, array at +4 data, +8 size) with the attributes
    /// `(name, value)`; the name words carry the flag bit 0.
    fn group_with(e: &mut Engine, attributes: &[(&str, u32)]) -> u32 {
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(8 * attributes.len() as u32 + 8);
        for (index, (name, value)) in attributes.iter().enumerate() {
            let text = put_text(e, name.as_bytes());
            e.mem.set_u32(data + 8 * index as u32, text | 1);
            e.mem.set_u32(data + 8 * index as u32 + 4, *value);
        }
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, attributes.len() as u32);
        this
    }

    #[test]
    fn the_name_search_ignores_case_and_the_flag_bit() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        let this = group_with(&mut e, &[("Alpha", 1), ("Beta", 2)]);
        let wanted = put_text(&mut e, b"BETA");
        assert_eq!(e.call(0x00e5_c490, &args![this, wanted]).i32(), 1);
        let missing = put_text(&mut e, b"gamma");
        assert_eq!(e.call(0x00e5_c490, &args![this, missing]).i32(), -1);
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x00e5_c490, &args![this, wanted]).i32(), -1);
    }

    /// The reference pointer copy double: adds the reference and stores the source's word.
    fn install_ref_copy(e: &mut Engine, events: &Events) {
        let kept = events.clone();
        e.register_double(REF_PTR_COPY, move |e, a| {
            kept.borrow_mut().push(("copy", a.to_vec()));
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn the_variant_lookup_adds_a_reference_to_what_it_finds() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        tag_calls(&mut e, &events, ADD_REFERENCE, "add", 0);
        tag_calls(&mut e, &events, REMOVE_REFERENCE, "remove", 0);
        install_ref_copy(&mut e, &events);
        let this = group_with(&mut e, &[("a", 0x1111), ("b", 0x2222)]);
        let out = e.mem.alloc(4);
        let name = put_text(&mut e, b"B");
        assert_eq!(e.call(0x00e5_c4d0, &args![this, out, name]).u32(), out);
        assert_eq!(e.mem.u32(out), 0x2222);
        assert_eq!(*events.borrow(), vec![("add", vec![0x2222])]);
    }

    #[test]
    fn a_missing_variant_is_null_through_a_temporary_reference() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        tag_calls(&mut e, &events, ADD_REFERENCE, "add", 0);
        tag_calls(&mut e, &events, REMOVE_REFERENCE, "remove", 0);
        install_ref_copy(&mut e, &events);
        let this = group_with(&mut e, &[("a", 0x1111)]);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x5555);
        let name = put_text(&mut e, b"zz");
        let frame = e.next_stack_local(0x1c);
        assert_eq!(e.call(0x00e5_c4d0, &args![this, out, name]).u32(), out);
        assert_eq!(e.mem.u32(out), 0);
        let events = events.borrow();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0, "copy");
        // The destination is a temporary the exe keeps on its stack, the source a null word:
        // the `name` argument's slot (+0x18) and +0x8 of the exe's frame.
        assert_eq!(events[0].1, vec![frame + 0x18, frame + 8]);
    }

    /// `hkRefVariant::getClass` answers the class record stored at the variant's object
    /// (the object's first word); `hkClass::getName` answers the record's first word.
    fn install_class_doubles(e: &mut Engine) {
        e.register(REF_VARIANT_GET_CLASS, |e, a| {
            let object = e.mem.u32(a[0]);
            Ret {
                eax: e.mem.u32(object),
                ..Ret::default()
            }
        });
        e.register(CLASS_GET_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
    }

    #[test]
    fn the_object_lookup_checks_the_class_name() {
        let mut e = Engine::new();
        install_casecmp(&mut e);
        install_class_doubles(&mut e);
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        tag_calls(&mut e, &events, ADD_REFERENCE, "add", 0);
        tag_calls(&mut e, &events, REMOVE_REFERENCE, "remove", 0);
        install_ref_copy(&mut e, &events);
        // The attribute's object: its first word points at a class record whose first word is
        // the class name.
        let class_name = put_text(&mut e, b"hkQuaternion");
        let class = e.mem.alloc(8);
        e.mem.set_u32(class, class_name);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, class);
        let this = group_with(&mut e, &[("rot", object)]);
        let name = put_text(&mut e, b"ROT");
        // The wanted class record (same name, different case).
        let wanted_name = put_text(&mut e, b"HKQUATERNION");
        let wanted = e.mem.alloc(8);
        e.mem.set_u32(wanted, wanted_name);
        // The variant is the `name` argument's own slot, +0x8 of the exe's 0x10 bytes (the next
        // stack local; its bytes stay on the stack after the call).
        let frame = e.next_stack_local(0x10);
        assert_eq!(
            e.call(0x00e5_c540, &args![this, name, wanted]).u32(),
            object
        );
        assert_eq!(e.mem.u32(frame + 8), object);
        assert_eq!(
            *events.borrow(),
            vec![("add", vec![object]), ("remove", vec![object])]
        );
        // A class record with another name: released and null.
        events.borrow_mut().clear();
        let other_name = put_text(&mut e, b"hkMatrix");
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, other_name);
        assert_eq!(e.call(0x00e5_c540, &args![this, name, other]).u32(), 0);
        assert_eq!(
            *events.borrow(),
            vec![("add", vec![object]), ("remove", vec![object])]
        );
        // No class record: whatever is found is returned.
        events.borrow_mut().clear();
        assert_eq!(e.call(0x00e5_c540, &args![this, name, 0u32]).u32(), object);
        assert_eq!(events.borrow().len(), 2);
        // No attribute of that name: null, and nothing is released.
        events.borrow_mut().clear();
        let missing = put_text(&mut e, b"absent");
        assert_eq!(e.call(0x00e5_c540, &args![this, missing, wanted]).u32(), 0);
        assert!(events.borrow().iter().all(|(tag, _)| *tag == "copy"));
    }

    #[test]
    fn the_copy_constructor_builds_and_assigns_each_group() {
        let mut e = Engine::new();
        install_memory(&mut e);
        let events: Events = Rc::new(RefCell::new(Vec::new()));
        tag_calls(&mut e, &events, STRING_PTR_CONSTRUCT, "construct", 0);
        tag_calls(&mut e, &events, GROUP_ASSIGN, "assign", 0);
        tag_calls(&mut e, &events, GROUP_DESTRUCT, "destroy", 0);
        let kept = events.clone();
        e.register_double(ARRAY_RESERVE, move |e, a| {
            kept.borrow_mut().push(("reserve", a.to_vec()));
            let data = e.mem.alloc(0x10 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let other = e.mem.alloc(0x20);
        let groups = e.mem.alloc(0x20);
        e.mem.set_u32(other + 8, groups);
        e.mem.set_u32(other + 0xc, 2);
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xee; 0x20]);
        assert_eq!(e.call(0x00e3_e950, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), HOLDER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let data = e.mem.u32(this + 8);
        assert_eq!(
            *events.borrow(),
            vec![
                ("reserve", vec![ALLOCATOR_OBJECT, this + 8, 2, 0x10]),
                ("construct", vec![data]),
                ("assign", vec![data, groups]),
                ("construct", vec![data + 0x10]),
                ("assign", vec![data + 0x10, groups + 0x10]),
            ]
        );
        // The new groups start empty: no data, no size, the flag in the capacity word.
        for group in [data, data + 0x10] {
            assert_eq!(e.mem.u32(group + 4), 0);
            assert_eq!(e.mem.u32(group + 8), 0);
            assert_eq!(e.mem.u32(group + 0xc), 0x8000_0000);
        }
        // An empty source needs no reserve and no groups.
        events.borrow_mut().clear();
        e.mem.set_u32(other + 0xc, 0);
        e.call(0x00e3_e950, &args![this, other]);
        assert!(events.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn the_destructor_destroys_the_groups_and_gives_the_array_back() {
        let mut e = Engine::new();
        let (frees, _) = install_memory(&mut e);
        let groups = log_calls(&mut e, GROUP_DESTRUCT, 0);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 0x4000_0003);
        e.call(0x00e3_eaa0, &args![this]);
        assert_eq!(*groups.borrow(), vec![vec![data], vec![data + 0x10]]);
        assert_eq!(*frees.borrow(), vec![vec![data, 48]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        // A flagged capacity is not given back.
        e.mem.set_u32(this + 0x10, 0x8000_0003);
        e.mem.set_u32(this + 8, data);
        e.call(0x00e3_eaa0, &args![this]);
        assert_eq!(frees.borrow().len(), 1);
    }
}
