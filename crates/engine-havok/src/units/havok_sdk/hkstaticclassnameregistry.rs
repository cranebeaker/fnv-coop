//! `Havok SDK/hkstaticclassnameregistry.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkStaticClassNameRegistry` keeps its name at `+8` and a pointer at `+0xc` to a
//! null-terminated array of `hkClass` pointers; `00db7a80` (the map's `recomputeClassMemberOffsets`,
//! which takes the registry in `ECX`) prepares the classes once (flag at `+0x14`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00db7a80` (`thiscall`, `this` the registry, no stack words): recomputes the member offsets of
/// the registered classes the first time.
const RECOMPUTE_CLASS_MEMBER_OFFSETS: u32 = 0x00db_7a80;
/// `hkClass::getName` (Xbox PDB name, `thiscall`) and `hkString::strCmp` (cdecl, 0 when equal).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
const STRING_COMPARE: u32 = 0x00cb_0fa0;
/// The memory router (`00c85750`; its `+0x10` is the allocator) and `hkArrayUtil::_reserve` (cdecl:
/// allocator, array, count, element size).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

// Translated from 00db7a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStaticClassNameRegistry::getName` (Xbox PDB), `ECX` = `this`, no stack words: the registry's
/// name, the word at `+8`.
pub fn hk_static_class_name_registry_get_name(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

// Translated from 00db7ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStaticClassNameRegistry::getClassByName` (Xbox PDB), `thiscall`, `RET 4`: prepares the
/// classes (`00db7a80` on `this`), then compares `name` with each registered class's name
/// (`hkClass::getName`, `hkString::strCmp`) and answers the first class that matches, or 0.
pub fn hk_static_class_name_registry_get_class_by_name(
    e: &mut Engine,
    this: Ptr,
    name: u32,
) -> u32 {
    let base = this.addr();
    e.call(RECOMPUTE_CLASS_MEMBER_OFFSETS, &args![this]);
    let classes = e.mem.u32(base + 0xc);
    if e.mem.u32(classes) == 0 {
        return 0;
    }
    let mut classes = classes;
    let mut index: u32 = 0;
    loop {
        let class = e.mem.u32(classes.wrapping_add(index.wrapping_mul(4)));
        let class_name = e.call(CLASS_GET_NAME, &args![class]).u32();
        let different = e.call(STRING_COMPARE, &args![name, class_name]).u32();
        classes = e.mem.u32(base + 0xc);
        if different == 0 {
            return e.mem.u32(classes.wrapping_add(index.wrapping_mul(4)));
        }
        index += 1;
        if e.mem.u32(classes.wrapping_add(index.wrapping_mul(4))) == 0 {
            return 0;
        }
    }
}

// Translated from 00db7b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStaticClassNameRegistry::getClasses` (Xbox PDB), `thiscall`, `RET 4`: prepares the classes
/// (`00db7a80` on `this`), counts them (entries up to the null terminator), makes room in the
/// `hkArray` of class pointers at `out` (data, size at `+4`, capacity and flags at `+8`:
/// `_reserve(allocator, out, max(2 * capacity, size + count), 4)` when the capacity is below
/// `size + count`, the allocator being the memory router's `+0x10`), and appends every class
/// pointer after the existing ones.
pub fn hk_static_class_name_registry_get_classes(e: &mut Engine, this: Ptr, out: u32) {
    let base = this.addr();
    e.call(RECOMPUTE_CLASS_MEMBER_OFFSETS, &args![this]);
    let mut entry = e.mem.u32(base + 0xc);
    let mut count: i32 = 0;
    if e.mem.u32(entry) != 0 {
        loop {
            entry = entry.wrapping_add(4);
            count += 1;
            if e.mem.u32(entry) == 0 {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.i32(out + 4);
    let capacity = (e.mem.u32(out + 8) & 0x3fff_ffff) as i32;
    let total = count.wrapping_add(size);
    if capacity < total {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if total >= doubled { total } else { doubled };
        e.call(ARRAY_RESERVE, &args![allocator, out, wanted, 4u32]);
    }
    let new_size = e.mem.i32(out + 4).wrapping_add(count);
    e.mem.set_i32(out + 4, new_size);
    let data = e.mem.u32(out);
    let mut entry = e.mem.u32(base + 0xc);
    let destination = data.wrapping_add((size as u32).wrapping_mul(4));
    let mut index: u32 = 0;
    if e.mem.u32(entry) != 0 {
        loop {
            let class = e.mem.u32(entry);
            e.mem
                .set_u32(destination.wrapping_add(index.wrapping_mul(4)), class);
            entry = e.mem.u32(base + 0xc);
            index += 1;
            entry = entry.wrapping_add(index.wrapping_mul(4));
            if e.mem.u32(entry) == 0 {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db7a70, hk_static_class_name_registry_get_name(Ptr) -> u32),
        entry!(0x00db7ae0, hk_static_class_name_registry_get_class_by_name(Ptr, u32) -> u32),
        entry!(
            0x00db7b70,
            hk_static_class_name_registry_get_classes(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A registry (name word `0x4e41`) over classes named by the first word of each class object.
    /// Returns the registry and the class objects.
    fn registry(e: &mut Engine, names: &[&str]) -> (u32, Vec<u32>) {
        e.register(RECOMPUTE_CLASS_MEMBER_OFFSETS, |_, _| Ret::default());
        e.register(CLASS_GET_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(STRING_COMPARE, |e, a| Ret {
            eax: u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])),
            ..Ret::default()
        });
        let mut classes = Vec::new();
        for name in names {
            let text = e.mem.alloc(name.len() as u32 + 1);
            e.mem.set_cstr(text, name.as_bytes());
            let class = e.mem.alloc(8);
            e.mem.set_u32(class, text);
            classes.push(class);
        }
        let table = e.mem.alloc(4 * classes.len() as u32 + 4);
        for (i, class) in classes.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *class);
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0x4e41);
        e.mem.set_u32(this + 0xc, table);
        (this, classes)
    }

    #[test]
    fn get_name_is_the_word_at_8() {
        let mut e = Engine::new();
        let (this, _) = registry(&mut e, &[]);
        assert_eq!(e.call(0x00db_7a70, &args![this]).u32(), 0x4e41);
    }

    #[test]
    fn get_class_by_name_finds_the_first_match_after_preparing_the_classes() {
        let mut e = Engine::new();
        let (this, classes) = registry(&mut e, &["hkA", "hkB", "hkB", "hkC"]);
        let wanted = e.mem.alloc(8);
        e.mem.set_cstr(wanted, b"hkB");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00db_7ae0, &args![this, wanted]).u32(), classes[1]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (RECOMPUTE_CLASS_MEMBER_OFFSETS, vec![this]));
        // Only the classes up to the match are asked their names.
        assert_eq!(log.iter().filter(|c| c.0 == CLASS_GET_NAME).count(), 2);
        e.mem.set_cstr(wanted, b"hkC");
        assert_eq!(e.call(0x00db_7ae0, &args![this, wanted]).u32(), classes[3]);
        e.mem.set_cstr(wanted, b"hkZ");
        assert_eq!(e.call(0x00db_7ae0, &args![this, wanted]).u32(), 0);
    }

    #[test]
    fn get_class_by_name_answers_0_for_an_empty_registry() {
        let mut e = Engine::new();
        let (this, _) = registry(&mut e, &[]);
        let wanted = e.mem.alloc(8);
        e.mem.set_cstr(wanted, b"hkA");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00db_7ae0, &args![this, wanted]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != CLASS_GET_NAME));
    }

    /// A router whose allocator is `0xa11c`, and a `_reserve` double that gives the array a block of
    /// `count` words (copying the old words) and logs its arguments.
    fn install_reserve(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa11c);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log = Rc::new(RefCell::new(Vec::new()));
        let calls = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            calls.borrow_mut().push(a.to_vec());
            let (array, count) = (a[1], a[2]);
            let data = e.mem.alloc(count * 4);
            let size = e.mem.u32(array + 4);
            let old = e.mem.u32(array);
            for i in 0..size {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, count);
            Ret::default()
        });
        log
    }

    fn array_of(e: &mut Engine, words: &[u32], capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(4 * capacity.max(1));
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *word);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    #[test]
    fn get_classes_appends_every_class_growing_the_array_when_needed() {
        let mut e = Engine::new();
        let reserves = install_reserve(&mut e);
        let (this, classes) = registry(&mut e, &["hkA", "hkB", "hkC"]);
        // Capacity 4 holds the one existing word and the three classes: no growth.
        let out = array_of(&mut e, &[0x99], 4);
        e.call_log = Some(vec![]);
        e.call(0x00db_7b70, &args![this, out]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (RECOMPUTE_CLASS_MEMBER_OFFSETS, vec![this]));
        assert!(reserves.borrow().is_empty());
        assert_eq!(e.mem.u32(out + 4), 4);
        let data = e.mem.u32(out);
        assert_eq!(e.mem.u32(data), 0x99);
        for (i, class) in classes.iter().enumerate() {
            assert_eq!(e.mem.u32(data + 4 + 4 * i as u32), *class);
        }
        // Capacity 1 is below 4: grown to max(2, 4) = 4 words with the allocator `0xa11c`.
        let out = array_of(&mut e, &[0x99], 1);
        e.call(0x00db_7b70, &args![this, out]);
        assert_eq!(reserves.borrow().len(), 1);
        assert_eq!(reserves.borrow()[0], vec![0xa11c, out, 4, 4]);
        assert_eq!(e.mem.u32(out + 4), 4);
        let data = e.mem.u32(out);
        assert_eq!(e.mem.u32(data), 0x99);
        assert_eq!(e.mem.u32(data + 12), classes[2]);
        // Capacity 5 is below 8 (5 + 3): the doubled capacity 10 is asked for.
        let out = array_of(&mut e, &[1, 2, 3, 4, 5], 5);
        e.call(0x00db_7b70, &args![this, out]);
        assert_eq!(reserves.borrow()[1], vec![0xa11c, out, 10, 4]);
        assert_eq!(e.mem.u32(out + 4), 8);
    }

    #[test]
    fn get_classes_with_no_classes_adds_nothing() {
        let mut e = Engine::new();
        let reserves = install_reserve(&mut e);
        let (this, _) = registry(&mut e, &[]);
        let out = array_of(&mut e, &[7], 1);
        e.call(0x00db_7b70, &args![this, out]);
        assert!(reserves.borrow().is_empty());
        assert_eq!(e.mem.u32(out + 4), 1);
    }
}
