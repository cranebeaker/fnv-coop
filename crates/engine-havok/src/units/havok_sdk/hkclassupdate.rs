//! `Havok SDK/hkclassupdate.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMapBase<unsigned long,unsigned long,...>::getWithDefault` (`00cb1930`, `this` = the map,
/// `(key, default)`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
/// The map's insert (`00cb1870`, `this` = the map, `(allocator, key, value)`).
const MAP_INSERT: u32 = 0x00cb_1870;
/// The memory router (`00c85750`): its field `+0x10` is the allocator the map inserts with.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkString::strCmp` (`00cb0fa0`, cdecl `(first, second)`; zero when equal).
const STR_CMP: u32 = 0x00cb_0fa0;
/// `hkClass::getName` (`00d46d40`, `this` = the class record).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
/// `hkClass::getParent` (the second overload, `00d46d60`, `this` = the class record).
const CLASS_GET_PARENT_OV2: u32 = 0x00d4_6d60;
/// `hkClass::getMember` (`00d46f20`, `this` = the class record, `(index)`).
const CLASS_GET_MEMBER: u32 = 0x00d4_6f20;
/// `hkClass::getNumDeclaredMembers` (`00d46f70`, `this` = the class record).
const CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00d4_6f70;
/// `hkClass::getDeclaredMember` (`00d46f80`, `this` = the class record, `(index)`).
const CLASS_GET_DECLARED_MEMBER: u32 = 0x00d4_6f80;
/// `hkClass::getMemberByName` (`00d46fe0`, `this` = the class record, `(name)`).
const CLASS_GET_MEMBER_BY_NAME: u32 = 0x00d4_6fe0;
/// `hkClassMember::getStructClass` (`00d86910`, `this` = the member record).
const MEMBER_GET_STRUCT_CLASS: u32 = 0x00d8_6910;
/// `hkClassMember::getClass` (`00d86920`, `this` = the member record).
const MEMBER_GET_CLASS: u32 = 0x00d8_6920;
/// The exe's class record that describes the class records themselves (identity not
/// confirmed); `hasVtable` and `numImplementedInterfaces` are looked up in it.
const CLASS_RECORD_CLASS: u32 = 0x0126_ed90;
/// The exe's class record that describes the member records (identity not confirmed).
const MEMBER_RECORD_CLASS: u32 = 0x0126_ca58;
/// The member name `hasVtable` in the exe's strings.
const NAME_HAS_VTABLE: u32 = 0x010e_a4cc;
/// The member name `numImplementedInterfaces` in the exe's strings.
const NAME_NUM_IMPLEMENTED_INTERFACES: u32 = 0x010e_5210;
/// The class name `hkpConstraintInstance` in the exe's strings.
const NAME_CONSTRAINT_INSTANCE: u32 = 0x010d_27bc;
/// The member callback `walkMembers` gets for the first update step (identity not confirmed).
const FIRST_STEP_CALLBACK: u32 = 0x00e0_4a60;
/// The member callback `walkMembers` gets for the second update step (identity not confirmed).
const SECOND_STEP_CALLBACK: u32 = 0x00e0_4a80;

/// Records `class` as visited with `bits` in `map`: inserts through the memory router's
/// allocator (`+0x10` of the router `00c85750` answers).
fn map_insert(e: &mut Engine, map: u32, class: u32, bits: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.call(MAP_INSERT, &args![map, allocator, class, bits]);
}

// Translated from 00e04af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::walkMembers` (Xbox PDB; cdecl `(class, map, flag, callback)`):
/// visits a class once per `flag` bit: when the map's word for the class (0 if absent) has no
/// bit of `flag`, sets them there and, for each declared member, first walks the member's
/// class (`hkClassMember::getClass`, `00d86920`) when it has one and then calls the cdecl
/// function `callback(member)`; it then continues with the parent class (`getParent`,
/// `00d46d60`) the same way, and stops at the first class that already has a bit or when
/// there is no parent.
pub fn walk_members(e: &mut Engine, class: u32, map: u32, flag: u32, callback: u32) {
    let mut class = class;
    let mut bits = e.call(MAP_GET_WITH_DEFAULT, &args![map, class, 0u32]).u32();
    if flag & bits != 0 {
        return;
    }
    loop {
        map_insert(e, map, class, bits | flag);
        let mut index = 0i32;
        if e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32() > 0 {
            loop {
                let member = e
                    .call(CLASS_GET_DECLARED_MEMBER, &args![class, index])
                    .u32();
                let member_class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
                if member_class != 0 {
                    walk_members(e, member_class, map, flag, callback);
                }
                e.call(callback, &args![member]);
                index = index.wrapping_add(1);
                if index >= e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32() {
                    break;
                }
            }
        }
        if e.call(CLASS_GET_PARENT_OV2, &args![class]).u32() == 0 {
            return;
        }
        class = e.call(CLASS_GET_PARENT_OV2, &args![class]).u32();
        bits = e.call(MAP_GET_WITH_DEFAULT, &args![map, class, 0u32]).u32();
        if flag & bits != 0 {
            return;
        }
    }
}

// Translated from 00e04bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::updateClassVersion1Inplace` (Xbox PDB; cdecl `(class, map)`):
/// visits a class once (bit 1 of the map's word for it): looks `hasVtable` up in the exe's
/// class record `0126ed90` (`hkClass::getMemberByName`, `00d46fe0`) and, taking the member's
/// 16-bit offset (`+0x12`), zeroes that dword in the class and in every parent below the
/// topmost class; when the byte there in the topmost class is not 0 it adds 1 to the dword
/// at the offset of `numImplementedInterfaces` in that class; it zeroes the `hasVtable`
/// dword of the topmost class, runs itself on the class of every declared member that has
/// one (`hkClassMember::getStructClass`, `00d86910`), and goes on with the parent class the
/// same way, stopping at a class whose bit is already set.
pub fn update_class_version1_inplace(e: &mut Engine, class: u32, map: u32) {
    let mut class = class;
    let mut bits = e.call(MAP_GET_WITH_DEFAULT, &args![map, class, 0u32]).u32();
    while bits & 1 == 0 {
        map_insert(e, map, class, bits | 1);
        let has_vtable = e
            .call(
                CLASS_GET_MEMBER_BY_NAME,
                &args![CLASS_RECORD_CLASS, NAME_HAS_VTABLE],
            )
            .u32();
        let offset = u32::from(e.mem.u16(has_vtable + 0x12));
        let mut top = class;
        if e.call(CLASS_GET_PARENT_OV2, &args![class]).u32() != 0 {
            loop {
                e.mem.set_u32(top.wrapping_add(offset), 0);
                top = e.call(CLASS_GET_PARENT_OV2, &args![top]).u32();
                if e.call(CLASS_GET_PARENT_OV2, &args![top]).u32() == 0 {
                    break;
                }
            }
        }
        if e.mem.u8(top.wrapping_add(offset)) != 0 {
            let interfaces = e
                .call(
                    CLASS_GET_MEMBER_BY_NAME,
                    &args![CLASS_RECORD_CLASS, NAME_NUM_IMPLEMENTED_INTERFACES],
                )
                .u32();
            let at = u32::from(e.mem.u16(interfaces + 0x12)).wrapping_add(top);
            let count = e.mem.u32(at);
            e.mem.set_u32(at, count.wrapping_add(1));
        }
        e.mem.set_u32(top.wrapping_add(offset), 0);
        let mut index = 0i32;
        if e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32() > 0 {
            loop {
                let member = e
                    .call(CLASS_GET_DECLARED_MEMBER, &args![class, index])
                    .u32();
                if e.mem.u32(member + 4) != 0 {
                    let member_class = e.call(MEMBER_GET_STRUCT_CLASS, &args![member]).u32();
                    update_class_version1_inplace(e, member_class, map);
                }
                index = index.wrapping_add(1);
                if index >= e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32() {
                    break;
                }
            }
        }
        if e.call(CLASS_GET_PARENT_OV2, &args![class]).u32() == 0 {
            return;
        }
        class = e.call(CLASS_GET_PARENT_OV2, &args![class]).u32();
        bits = e.call(MAP_GET_WITH_DEFAULT, &args![map, class, 0u32]).u32();
    }
}

// Translated from 00e04cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::updateConstraintInstance300` (Xbox PDB). Takes its inputs in
/// registers: `class` in `EAX` and `map` in `EDI` (nothing on the stack, `RET`). Visits a class
/// once (bit 4 of the map's word for it); when the class is named `hkpConstraintInstance`
/// (`hkString::strCmp` of `hkClass::getName` against the string at `010d27bc`), writes the byte
/// 0x19 into the class's declared member 2, at the 16-bit offset (`+0x12`) of member 4 of the
/// exe's member-describing class record `0126ca58` (`hkClass::getMember`, `00d46f20`).
pub fn update_constraint_instance300(e: &mut Engine, class: u32, map: u32) {
    let bits = e.call(MAP_GET_WITH_DEFAULT, &args![map, class, 0u32]).u32();
    if bits & 4 != 0 {
        return;
    }
    map_insert(e, map, class, bits | 4);
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    if e.call(STR_CMP, &args![name, NAME_CONSTRAINT_INSTANCE])
        .i32()
        != 0
    {
        return;
    }
    let member = e.call(CLASS_GET_DECLARED_MEMBER, &args![class, 2i32]).u32();
    let described = e
        .call(CLASS_GET_MEMBER, &args![MEMBER_RECORD_CLASS, 4i32])
        .u32();
    let offset = u32::from(e.mem.u16(described + 0x12));
    e.mem.set_u8(offset.wrapping_add(member), 0x19);
}

// Translated from 00e04d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkClass::updateMetadataInplace` (Xbox PDB; cdecl `(class, map, version)`): for version 1
/// runs [`update_class_version1_inplace`] and [`update_constraint_instance300`] on the class;
/// for versions below 4 (version 1 included) walks the members with flag 2 and the callback
/// at `00e04a60`; for versions below 5 walks them with flag 8 and the callback at `00e04a80`.
pub fn hk_class_update_metadata_inplace(e: &mut Engine, class: u32, map: u32, version: i32) {
    if version == 1 {
        update_class_version1_inplace(e, class, map);
        update_constraint_instance300(e, class, map);
    }
    // Version 1 is below 4: the exe falls from its version 1 steps into this walk.
    if version < 4 {
        walk_members(e, class, map, 2, FIRST_STEP_CALLBACK);
    }
    if version < 5 {
        walk_members(e, class, map, 8, SECOND_STEP_CALLBACK);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e04af0, walk_members(u32, u32, u32, u32)),
        entry!(0x00e04bb0, update_class_version1_inplace(u32, u32)),
        entry!(0x00e04cc0, update_constraint_instance300(u32, u32)),
        entry!(0x00e04d30, hk_class_update_metadata_inplace(u32, u32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type MapWords = Rc<RefCell<HashMap<(u32, u32), u32>>>;

    const ALLOCATOR: u32 = 0x5a5a_0000;
    const MAP: u32 = 0x0300_0000;

    /// A fake word map keyed by (map, key), the memory router, `strCmp` on C strings, and the
    /// class accessors the update code calls, all reading the class records the tests build
    /// (parent `+4`, name `+0`, member array `+0x18`, member count `+0x1C`).
    fn engine() -> (Engine, MapWords) {
        let mut e = Engine::new();
        let words: MapWords = Rc::default();
        let reads = Rc::clone(&words);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: reads.borrow().get(&(a[0], a[1])).copied().unwrap_or(a[2]),
            ..Ret::default()
        });
        let writes = Rc::clone(&words);
        e.register_double(MAP_INSERT, move |_, a| {
            assert_eq!(a[1], ALLOCATOR, "inserted through the router's allocator");
            writes.borrow_mut().insert((a[0], a[2]), a[3]);
            Ret::default()
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(STR_CMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            Ret {
                eax: first.cmp(&second) as i32 as u32,
                ..Ret::default()
            }
        });
        e.register(CLASS_GET_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CLASS_GET_PARENT_OV2, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(CLASS_GET_NUM_DECLARED_MEMBERS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        for address in [CLASS_GET_DECLARED_MEMBER, CLASS_GET_MEMBER] {
            e.register(address, |e, a| Ret {
                eax: e.mem.u32(a[0] + 0x18) + a[1] * 0x18,
                ..Ret::default()
            });
        }
        e.register(CLASS_GET_MEMBER_BY_NAME, |e, a| {
            let wanted = e.mem.cstr(a[1]);
            let members = e.mem.u32(a[0] + 0x18);
            let count = e.mem.u32(a[0] + 0x1c);
            let found = (0..count)
                .map(|i| members + 0x18 * i)
                .find(|&m| e.mem.cstr(e.mem.u32(m)) == wanted);
            Ret {
                eax: found.unwrap_or(0),
                ..Ret::default()
            }
        });
        e.register(MEMBER_GET_CLASS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(MEMBER_GET_STRUCT_CLASS, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        (e, words)
    }

    fn text(e: &mut Engine, s: &str) -> u32 {
        let a = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(a, s.as_bytes());
        a
    }

    fn class(e: &mut Engine, name: &str, parent: u32) -> u32 {
        let c = e.mem.alloc(0x40);
        let n = text(e, name);
        e.mem.set_u32(c, n);
        e.mem.set_u32(c + 4, parent);
        c
    }

    /// Gives the class declared members with the given names and classes.
    fn members(e: &mut Engine, class: u32, list: &[(&str, u32)]) -> u32 {
        let base = e.mem.alloc(0x18 * list.len().max(1) as u32);
        for (i, (name, member_class)) in list.iter().enumerate() {
            let n = text(e, name);
            e.mem.set_u32(base + 0x18 * i as u32, n);
            e.mem.set_u32(base + 0x18 * i as u32 + 4, *member_class);
        }
        e.mem.set_u32(class + 0x18, base);
        e.mem.set_u32(class + 0x1c, list.len() as u32);
        base
    }

    /// A callback double that records the member it is called with.
    fn recording_callback(e: &mut Engine, address: u32) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a[0]));
            Ret::default()
        });
        seen
    }

    #[test]
    fn walk_members_visits_member_classes_then_the_parent() {
        let (mut e, words) = engine();
        let callback = 0x0900_0021;
        let seen = recording_callback(&mut e, callback);
        let inner = class(&mut e, "Inner", 0);
        let inner_members = members(&mut e, inner, &[("i0", 0)]);
        let parent = class(&mut e, "Parent", 0);
        let parent_members = members(&mut e, parent, &[("p0", inner), ("p1", 0)]);
        let c = class(&mut e, "Child", parent);
        let child_members = members(&mut e, c, &[("c0", 0)]);
        e.call_log = Some(Vec::new());
        e.call(0x00e04af0, &args![c, MAP, 2u32, callback]);
        let order: Vec<u32> = seen.borrow().iter().map(|(_, m)| *m).collect();
        assert_eq!(
            order,
            vec![
                child_members,
                inner_members,
                parent_members,
                parent_members + 0x18
            ]
        );
        for visited in [c, parent, inner] {
            assert_eq!(words.borrow()[&(MAP, visited)], 2, "class {visited:08x}");
        }
        // A second walk with the same flag does nothing; another flag adds its bit.
        e.call(0x00e04af0, &args![c, MAP, 2u32, callback]);
        assert_eq!(seen.borrow().len(), 4);
        e.call(0x00e04af0, &args![c, MAP, 8u32, callback]);
        assert_eq!(seen.borrow().len(), 8);
        assert_eq!(words.borrow()[&(MAP, c)], 10);
    }

    #[test]
    fn walk_members_stops_at_a_parent_that_was_already_visited() {
        let (mut e, words) = engine();
        let callback = 0x0900_0021;
        let seen = recording_callback(&mut e, callback);
        let parent = class(&mut e, "Parent", 0);
        members(&mut e, parent, &[("p0", 0)]);
        let c = class(&mut e, "Child", parent);
        let child_members = members(&mut e, c, &[("c0", 0)]);
        words.borrow_mut().insert((MAP, parent), 2 | 4);
        e.call(0x00e04af0, &args![c, MAP, 2u32, callback]);
        assert_eq!(*seen.borrow(), vec![(callback, child_members)]);
        assert_eq!(words.borrow()[&(MAP, c)], 2);
        assert_eq!(words.borrow()[&(MAP, parent)], 6);
    }

    /// The exe's two describing class records, with the members the updates look up: the
    /// class record describing classes has `hasVtable` at +0x2C and
    /// `numImplementedInterfaces` at +0x0C; the one describing members has five members whose
    /// fifth is at offset 0x0D.
    fn describing_classes(e: &mut Engine) {
        e.map(0x0126_c000, 0x3000);
        e.map(0x010e_5000, 0x1000);
        e.map(0x010e_a000, 0x1000);
        e.map(0x010d_2000, 0x1000);
        e.mem.set_cstr(NAME_HAS_VTABLE, b"hasVtable");
        e.mem
            .set_cstr(NAME_NUM_IMPLEMENTED_INTERFACES, b"numImplementedInterfaces");
        e.mem
            .set_cstr(NAME_CONSTRAINT_INSTANCE, b"hkpConstraintInstance");
        let record = members(
            e,
            CLASS_RECORD_CLASS,
            &[("hasVtable", 0), ("numImplementedInterfaces", 0)],
        );
        // The names above live on the heap, the class record's own array is at 0x0126ed90.
        e.mem.set_u16(record + 0x12, 0x2c);
        e.mem.set_u16(record + 0x18 + 0x12, 0x0c);
        let described = members(
            e,
            MEMBER_RECORD_CLASS,
            &[("a", 0), ("b", 0), ("c", 0), ("d", 0), ("offset", 0)],
        );
        e.mem.set_u16(described + 4 * 0x18 + 0x12, 0x0d);
    }

    #[test]
    fn version_one_update_clears_vtable_flags_and_counts_the_interface() {
        let (mut e, words) = engine();
        describing_classes(&mut e);
        let root = class(&mut e, "Root", 0);
        let middle = class(&mut e, "Middle", root);
        let c = class(&mut e, "C", middle);
        let nested = class(&mut e, "Nested", 0);
        members(&mut e, c, &[("m", nested)]);
        for class in [root, middle, c, nested] {
            e.mem.set_u32(class + 0x2c, 0x0101_0101);
        }
        e.mem.set_u32(root + 0x2c, 1);
        e.mem.set_u32(root + 0xc, 5);
        e.call(0x00e04bb0, &args![c, MAP]);
        // The topmost class had a vtable: one more interface; every flag dword is cleared.
        assert_eq!(e.mem.u32(root + 0xc), 6);
        for class in [root, middle, c, nested] {
            assert_eq!(e.mem.u32(class + 0x2c), 0, "class {class:08x}");
        }
        for class in [root, middle, c, nested] {
            assert_eq!(words.borrow()[&(MAP, class)], 1);
        }
        // Visited classes are skipped: nothing changes the second time.
        e.mem.set_u32(root + 0x2c, 1);
        e.call(0x00e04bb0, &args![c, MAP]);
        assert_eq!(e.mem.u32(root + 0xc), 6);
        assert_eq!(e.mem.u32(root + 0x2c), 1);
    }

    #[test]
    fn version_one_update_without_a_vtable_leaves_the_interface_count() {
        let (mut e, _) = engine();
        describing_classes(&mut e);
        let root = class(&mut e, "Root", 0);
        e.mem.set_u32(root + 0xc, 5);
        e.mem.set_u32(root + 0x2c, 0x0100_0000);
        e.call(0x00e04bb0, &args![root, MAP]);
        assert_eq!(e.mem.u32(root + 0xc), 5);
        assert_eq!(e.mem.u32(root + 0x2c), 0);
    }

    #[test]
    fn constraint_instance_update_patches_the_named_class_once() {
        let (mut e, words) = engine();
        describing_classes(&mut e);
        let target = class(&mut e, "hkpConstraintInstance", 0);
        let target_members = members(&mut e, target, &[("a", 0), ("b", 0), ("c", 0)]);
        let other = class(&mut e, "hkpOther", 0);
        let other_members = members(&mut e, other, &[("a", 0), ("b", 0), ("c", 0)]);
        e.call(0x00e04cc0, &args![target, MAP]);
        assert_eq!(e.mem.u8(target_members + 2 * 0x18 + 0x0d), 0x19);
        assert_eq!(words.borrow()[&(MAP, target)], 4);
        e.call(0x00e04cc0, &args![other, MAP]);
        assert_eq!(e.mem.u8(other_members + 2 * 0x18 + 0x0d), 0);
        assert_eq!(words.borrow()[&(MAP, other)], 4);
        // Already visited: not touched again.
        e.mem.set_u8(target_members + 2 * 0x18 + 0x0d, 0);
        e.call(0x00e04cc0, &args![target, MAP]);
        assert_eq!(e.mem.u8(target_members + 2 * 0x18 + 0x0d), 0);
    }

    #[test]
    fn metadata_update_runs_the_steps_the_version_calls_for() {
        for (version, expected) in [
            (1, vec![FIRST_STEP_CALLBACK, SECOND_STEP_CALLBACK]),
            (2, vec![FIRST_STEP_CALLBACK, SECOND_STEP_CALLBACK]),
            (3, vec![FIRST_STEP_CALLBACK, SECOND_STEP_CALLBACK]),
            (4, vec![SECOND_STEP_CALLBACK]),
            (5, vec![]),
            (9, vec![]),
        ] {
            let (mut e, words) = engine();
            describing_classes(&mut e);
            let first = recording_callback(&mut e, FIRST_STEP_CALLBACK);
            let second = recording_callback(&mut e, SECOND_STEP_CALLBACK);
            let c = class(&mut e, "C", 0);
            let c_members = members(&mut e, c, &[("m", 0)]);
            e.call(0x00e04d30, &args![c, MAP, version]);
            let mut called: Vec<u32> = first.borrow().iter().map(|(a, _)| *a).collect();
            called.extend(second.borrow().iter().map(|(a, _)| *a));
            assert_eq!(called, expected, "version {version}");
            for (_, member) in first.borrow().iter().chain(second.borrow().iter()) {
                assert_eq!(*member, c_members);
            }
            // Only version 1 runs the two class updates (bits 1 and 4 of the class word).
            let bits = words.borrow().get(&(MAP, c)).copied().unwrap_or(0);
            let steps = match version {
                1 => 1 | 4 | 2 | 8,
                2 | 3 => 2 | 8,
                4 => 8,
                _ => 0,
            };
            assert_eq!(bits, steps, "version {version}");
        }
    }
}
