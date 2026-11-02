//! `Havok SDK/hkrootlevelcontainer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkRootLevelContainer` is an `hkArray<NamedVariant>` (data pointer at +0, size at +4); a
//! `NamedVariant` is 0xC bytes: `m_name` (`hkStringPtr`, +0), `m_className` (`hkStringPtr`, +4)
//! and `m_variant` (`hkRefVariant`, +8, the object pointer). The class is not in the Xbox PDB's
//! type list; the layout is read from the code.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkRefVariant::getClass` (Xbox PDB name), `thiscall`: the variant's class, or 0.
const REF_VARIANT_GET_CLASS: u32 = 0x00d4_77a0;
/// `hkClass::getName` (Xbox PDB name), `thiscall`: the class's name string.
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
/// `hkString::strCmp` (Xbox PDB name), `cdecl`, two strings: 0 when they are equal.
const STRING_COMPARE: u32 = 0x00cb_0fa0;
/// `hkStringPtr::hkStringPtr` (Xbox PDB name), `thiscall`, no stack arguments.
const STRING_PTR_CONSTRUCTOR: u32 = 0x00cb_0a60;
/// `hkStringPtr::operator=` (Xbox PDB name), `thiscall`, one string word.
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// `hkRefVariant::set` (Xbox PDB name), `thiscall`: object and class.
const REF_VARIANT_SET: u32 = 0x00d4_7770;

/// Size of one `NamedVariant` in the array.
const NAMED_VARIANT_SIZE: u32 = 0xc;

/// Index of the first element the search looks at, as the first loop of the two searches
/// computes it: the loop stops at once for a null `after`, at the end of the array, or one
/// past the element whose `m_variant` is `after`.
fn first_index_after(e: &Engine, this: u32, after: u32) -> i32 {
    let mut index: i32 = 0;
    let mut offset: u32 = 0;
    loop {
        if after == 0 || e.mem.i32(this + 4) <= index {
            break;
        }
        let element = e.mem.u32(this).wrapping_add(offset);
        index += 1;
        offset = offset.wrapping_add(NAMED_VARIANT_SIZE);
        if e.mem.u32(element + 8) == after {
            break;
        }
    }
    index
}

/// The class name of the element at `element`: the name of the variant's class when it has one
/// (the class is asked twice, as the code does), else the element's `m_className` string with
/// the flag bit cleared.
fn element_class_name(e: &mut Engine, element: u32) -> u32 {
    let variant = element + 8;
    if e.call(REF_VARIANT_GET_CLASS, &args![variant]).u32() != 0 {
        let class = e.call(REF_VARIANT_GET_CLASS, &args![variant]).u32();
        e.call(CLASS_GET_NAME, &args![class]).u32()
    } else {
        e.mem.u32(element + 4) & 0xffff_fffe
    }
}

// Translated from 00d48080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRootLevelContainer::findObjectByType` (Xbox PDB), `thiscall`, `RET 8`: the object (`m_variant`)
/// of the first named variant, starting after the one whose object is `after` when that is not
/// null, whose class name equals `name`; 0 when there is none. The class name is the variant's
/// class's name, or the stored `m_className` when it has no class; an element without a name is
/// skipped.
pub fn hk_root_level_container_find_object_by_type(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    after: u32,
) -> u32 {
    let container = this.addr();
    let mut index = first_index_after(e, container, after);
    if index >= e.mem.i32(container + 4) {
        return 0;
    }
    let mut offset = (index as u32).wrapping_mul(NAMED_VARIANT_SIZE);
    loop {
        let element = e.mem.u32(container).wrapping_add(offset);
        if element_class_name(e, element) != 0 {
            let element = e.mem.u32(container).wrapping_add(offset);
            let class_name = element_class_name(e, element);
            if e.call(STRING_COMPARE, &args![name, class_name]).u32() == 0 {
                let data = e.mem.u32(container);
                return e.mem.u32(data + 8 + (index as u32).wrapping_mul(12));
            }
        }
        index += 1;
        offset = offset.wrapping_add(NAMED_VARIANT_SIZE);
        if index >= e.mem.i32(container + 4) {
            return 0;
        }
    }
}

// Translated from 00d48150 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `hkrootlevelcontainer.cpp`). `thiscall`, `RET 8`: the same
/// search as `00d48080` but comparing `name` with the element's `m_name` string (the first word
/// of the element, flag bit cleared) and never asking the variant for its class. Returns the
/// object of the first match after `after`, or 0. An element whose name is null is skipped.
pub fn fn_00d48150(e: &mut Engine, this: Ptr, name: u32, after: u32) -> u32 {
    let container = this.addr();
    let mut index = first_index_after(e, container, after);
    if index >= e.mem.i32(container + 4) {
        return 0;
    }
    let mut offset = (index as u32).wrapping_mul(NAMED_VARIANT_SIZE);
    loop {
        let data = e.mem.u32(container);
        let element_name = e.mem.u32(data.wrapping_add(offset)) & 0xffff_fffe;
        if element_name != 0 && e.call(STRING_COMPARE, &args![name, element_name]).u32() == 0 {
            let data = e.mem.u32(container);
            return e.mem.u32(data + 8 + (index as u32).wrapping_mul(12));
        }
        index += 1;
        offset = offset.wrapping_add(NAMED_VARIANT_SIZE);
        if index >= e.mem.i32(container + 4) {
            return 0;
        }
    }
}

// Translated from 00d481d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRootLevelContainer::NamedVariant::NamedVariant` (Xbox PDB), `thiscall`, `RET 0xc`: constructs
/// the two string pointers (`m_name`, `m_className`), clears `m_variant`, assigns `name` to
/// `m_name`, sets the variant to `object` of `class`, then stores the variant's class name (or
/// null when it has no class) in `m_className`. Returns `this`.
pub fn hk_root_level_container_named_variant_named_variant(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    object: u32,
    class: u32,
) -> Ptr {
    let base = this.addr();
    let variant = base + 8;
    e.call(STRING_PTR_CONSTRUCTOR, &args![base]);
    e.call(STRING_PTR_CONSTRUCTOR, &args![base + 4]);
    e.mem.set_u32(variant, 0);
    e.call(STRING_PTR_ASSIGN, &args![base, name]);
    e.call(REF_VARIANT_SET, &args![variant, object, class]);
    if e.call(REF_VARIANT_GET_CLASS, &args![variant]).u32() != 0 {
        let variant_class = e.call(REF_VARIANT_GET_CLASS, &args![variant]).u32();
        let class_name = e.call(CLASS_GET_NAME, &args![variant_class]).u32();
        e.call(STRING_PTR_ASSIGN, &args![base + 4, class_name]);
    } else {
        e.call(STRING_PTR_ASSIGN, &args![base + 4, 0u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d48080, hk_root_level_container_find_object_by_type(Ptr, u32, u32) -> u32),
        entry!(0x00d48150, fn_00d48150(Ptr, u32, u32) -> u32),
        entry!(0x00d481d0, hk_root_level_container_named_variant_named_variant(Ptr, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine whose `strCmp` compares the two C strings byte for byte.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRING_COMPARE, |e, a| Ret {
            eax: u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])),
            ..Ret::default()
        });
        e
    }

    /// A container of named variants, `(m_name, m_className, m_variant)` each.
    fn container(e: &mut Engine, elements: &[(u32, u32, u32)]) -> u32 {
        let data = e.mem.alloc(0xc * elements.len().max(1) as u32);
        for (i, (name, class_name, object)) in elements.iter().enumerate() {
            let at = data + 0xc * i as u32;
            e.mem.set_u32(at, *name);
            e.mem.set_u32(at + 4, *class_name);
            e.mem.set_u32(at + 8, *object);
        }
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 4, elements.len() as u32);
        this
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(at, s);
        at
    }

    #[test]
    fn find_by_type_compares_the_stored_class_name_without_a_class() {
        let mut e = engine();
        e.register(REF_VARIANT_GET_CLASS, |_, _| Ret::default());
        let (n1, n2) = (text(&mut e, b"a"), text(&mut e, b"b"));
        let (c1, c2) = (text(&mut e, b"Alpha"), text(&mut e, b"Beta"));
        // The flag bit of a stored string is cleared before the compare.
        let this = container(
            &mut e,
            &[(n1, c1 | 1, 0x1111), (n2, 0, 0x2222), (n2, c2, 0x3333)],
        );
        let beta = text(&mut e, b"Beta");
        assert_eq!(e.call(0x00d4_8080, &args![this, beta, 0u32]).u32(), 0x3333);
        let missing = text(&mut e, b"Gamma");
        assert_eq!(e.call(0x00d4_8080, &args![this, missing, 0u32]).u32(), 0);
        let alpha = text(&mut e, b"Alpha");
        assert_eq!(e.call(0x00d4_8080, &args![this, alpha, 0u32]).u32(), 0x1111);
    }

    #[test]
    fn find_by_type_uses_the_variant_class_name_and_starts_after_the_given_object() {
        let mut e = engine();
        // The double reads the variant word (the object) as a pointer to the class.
        e.register(REF_VARIANT_GET_CLASS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CLASS_GET_NAME, |e, a| Ret {
            eax: e.mem.u32(e.mem.u32(a[0])),
            ..Ret::default()
        });
        let shape = text(&mut e, b"Shape");
        let stored = text(&mut e, b"Other");
        let this = container(&mut e, &[(0, stored, 0xa0), (0, stored, 0xb0)]);
        let data = e.mem.u32(this);
        e.call_log = Some(vec![]);
        // `getClass` returns the variant's own address and `getName` reads the string pointer
        // through the object, so each object is a cell holding the string.
        let cell0 = e.mem.alloc(4);
        e.mem.set_u32(cell0, shape);
        let cell1 = e.mem.alloc(4);
        e.mem.set_u32(cell1, shape);
        e.mem.set_u32(data + 8, cell0);
        e.mem.set_u32(data + 0xc + 8, cell1);
        assert_eq!(e.call(0x00d4_8080, &args![this, shape, 0u32]).u32(), cell0);
        // Starting after the first object finds the second.
        assert_eq!(e.call(0x00d4_8080, &args![this, shape, cell0]).u32(), cell1);
        // An `after` object that is not in the array leaves nothing to search.
        assert_eq!(e.call(0x00d4_8080, &args![this, shape, 0x9999u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        // The class is asked twice for each name the exe computes (the name is computed twice).
        assert_eq!(
            log.iter().filter(|c| c.0 == REF_VARIANT_GET_CLASS).count(),
            8
        );
    }

    #[test]
    fn find_by_type_skips_elements_without_a_class_name() {
        let mut e = engine();
        e.register(REF_VARIANT_GET_CLASS, |_, _| Ret::default());
        let n = text(&mut e, b"n");
        let wanted = text(&mut e, b"Wanted");
        // A stored word of just the flag bit, and a zero word, are null names: not compared.
        let this = container(&mut e, &[(n, 1, 0x10), (n, 0, 0x20), (n, wanted, 0x30)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_8080, &args![this, wanted, 0u32]).u32(), 0x30);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == STRING_COMPARE).count(), 1);
    }

    #[test]
    fn find_by_name_compares_the_element_names() {
        let mut e = engine();
        let (n1, n2) = (text(&mut e, b"physics"), text(&mut e, b"graphics"));
        let this = container(
            &mut e,
            &[(n1, 0, 0xa), (0, 0, 0xb), (n2 | 1, 0, 0xc), (n2, 0, 0xd)],
        );
        let graphics = text(&mut e, b"graphics");
        assert_eq!(e.call(0x00d4_8150, &args![this, graphics, 0u32]).u32(), 0xc);
        assert_eq!(
            e.call(0x00d4_8150, &args![this, graphics, 0xcu32]).u32(),
            0xd
        );
        assert_eq!(e.call(0x00d4_8150, &args![this, graphics, 0xdu32]).u32(), 0);
        let physics = text(&mut e, b"physics");
        assert_eq!(e.call(0x00d4_8150, &args![this, physics, 0u32]).u32(), 0xa);
        let none = text(&mut e, b"none");
        assert_eq!(e.call(0x00d4_8150, &args![this, none, 0u32]).u32(), 0);
    }

    #[test]
    fn named_variant_constructor_names_the_class_from_the_variant() {
        for has_class in [true, false] {
            let mut e = Engine::new();
            e.register(STRING_PTR_CONSTRUCTOR, |_, _| Ret::default());
            e.register(STRING_PTR_ASSIGN, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            e.register(REF_VARIANT_SET, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                e.mem.set_u32(a[0] + 0x100, a[2]);
                Ret::default()
            });
            e.register_double(REF_VARIANT_GET_CLASS, move |e, a| Ret {
                eax: if has_class {
                    e.mem.u32(a[0] + 0x100)
                } else {
                    0
                },
                ..Ret::default()
            });
            e.register(CLASS_GET_NAME, |_, a| Ret {
                eax: a[0] + 0x40,
                ..Ret::default()
            });
            let this = e.mem.alloc(0x200);
            e.call_log = Some(vec![]);
            let got = e.call(0x00d4_81d0, &args![this, 0x500u32, 0x600u32, 0x700u32]);
            assert_eq!(got.u32(), this);
            assert_eq!(e.mem.u32(this), 0x500);
            assert_eq!(e.mem.u32(this + 8), 0x600);
            assert_eq!(e.mem.u32(this + 4), if has_class { 0x740 } else { 0 });
            let log = e.call_log.take().unwrap();
            let ctor: Vec<_> = log
                .iter()
                .filter(|c| c.0 == STRING_PTR_CONSTRUCTOR)
                .collect();
            assert_eq!(ctor.len(), 2);
            assert_eq!(ctor[0].1, vec![this]);
            assert_eq!(ctor[1].1, vec![this + 4]);
        }
    }
}
