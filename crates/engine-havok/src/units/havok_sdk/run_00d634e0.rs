//! `Havok SDK/run_00d634e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpWorld` methods of `hkpworld.cpp` (thiscall on the world, one argument, the listener):
/// `00c937a0` pushes it on the array at world `+0x11c`, `00c93920` (Xbox PDB name
/// `hkpWorld::addWorldPostSimulationListener`) on the one at `+0x164`.
const WORLD_ADD_LISTENER: u32 = 0x00c9_37a0;
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
/// `hkpWorld` method `00c929e0` (thiscall, no arguments): builds a 0x44-byte referenced object
/// (reference count word at `+4`) holding the world's entities (data at `+0x20`, size at
/// `+0x24`) and answers it.
const WORLD_ENTITIES: u32 = 0x00c9_29e0;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`), both
/// thiscall on the object.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The memory router (`00c85750`, no arguments; its `+0x10` is the allocator) and
/// `hkArrayUtil::_reserveMore`'s `00c90940` (cdecl: the allocator, the array, the element size).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The global holding an object whose virtual methods at `+0x10` and `+0x14` (no arguments)
/// each answer a lookup object with a virtual method at `+0x10` (name or id in, class out);
/// the string `"hkpVehicleInstance"` (`010d17d0`) and the class test `00d46d80` (thiscall on the
/// class, `(result byte pointer, other class)`; writes 1 into the byte when the class named
/// like `this` is `other` or one of its parents, else 0, and answers the pointer).
const TYPE_REGISTRY: u32 = 0x0126_8b08;
const VEHICLE_CLASS_NAME: u32 = 0x010d_17d0;
const CLASS_DERIVES: u32 = 0x00d4_6d80;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d634e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the method at vtable offset 4 of the sub-object at `this + 0x18` once for each world
/// of the container at `[this + 0x1c]` (a count at `+0x70`, an array of world pointers at
/// `+0x6c`; both read again after every call). Does nothing without a container or without
/// worlds.
pub fn fn_00d634e0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut container = e.mem.u32(at.wrapping_add(0x1c));
    if container == 0 || e.mem.i32(container.wrapping_add(0x70)) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
        e.vcall(at.wrapping_add(0x18), 4, &args![world]);
        container = e.mem.u32(at.wrapping_add(0x1c));
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(container.wrapping_add(0x70)) {
            break;
        }
    }
}

// Translated from 00d63520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of a sub-object at `+0x20` (`this - 0x20` is the whole object) taking a world: hands
/// the listener `this + 8` (0 when the whole object would be null) to the world's array at
/// `+0x11c` (`00c937a0`) and `this + 0xc` (or 0) to the post-simulation listeners
/// (`00c93920`), then takes the world's entity list (`00c929e0`), calls the method at vtable
/// offset 4 of the sub-object `this + 8` with each of its `[list + 0x24]` entries (the data
/// pointer at `[list + 0x20]`) and releases the list (`00c905b0`).
pub fn fn_00d63520(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let whole = at.wrapping_sub(0x20);
    let first = if whole != 0 { at.wrapping_add(8) } else { 0 };
    e.call(WORLD_ADD_LISTENER, &args![world, first]);
    let second = if whole != 0 { at.wrapping_add(0xc) } else { 0 };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, second]);
    let list = e.call(WORLD_ENTITIES, &args![world]).u32();
    let mut index = 0u32;
    if e.mem.i32(list.wrapping_add(0x24)) > 0 {
        loop {
            let data = e.mem.u32(list.wrapping_add(0x20));
            let entity = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
            e.vcall(at.wrapping_add(8), 4, &args![entity]);
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(list.wrapping_add(0x24)) {
                break;
            }
        }
    }
    e.call(REMOVE_REFERENCE, &args![list]);
}

// Translated from 00d63590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `object` from the array of objects at `this + 8` (data, size at `+0xc`): the first
/// match is replaced by the last element and the size is lowered, then the object's reference
/// is released (`00c905b0`). Nothing happens when the array is empty or holds no match.
pub fn fn_00d63590(e: &mut Engine, this: Ptr, object: u32) {
    let at = this.addr();
    let size = e.mem.i32(at.wrapping_add(0xc));
    if size <= 0 {
        return;
    }
    let data = e.mem.u32(at.wrapping_add(8));
    let mut index = 0i32;
    loop {
        if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == object {
            break;
        }
        index = index.wrapping_add(1);
        if index >= size {
            return;
        }
    }
    // The search cannot end on -1; the exe tests it all the same.
    if index == -1 {
        return;
    }
    let last = e.mem.u32(at.wrapping_add(0xc)).wrapping_sub(1);
    e.mem.set_u32(at.wrapping_add(0xc), last);
    if last as i32 != index {
        let data = e.mem.u32(at.wrapping_add(8));
        let moved = e.mem.u32(data.wrapping_add(last.wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), moved);
    }
    e.call(REMOVE_REFERENCE, &args![object]);
}

// Translated from 00d635e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps `entity` (and takes a reference to it, `00c90510`) in the array of objects at
/// `this + 8` (data, size `+0xc`, capacity word `+0x10`) when its class is `hkpVehicleInstance`
/// or derived from it: both classes are looked up through the object at `0126_8b08` (its
/// method at `+0x10` answers a lookup object whose method at `+0x10` takes the string
/// `"hkpVehicleInstance"`; its method at `+0x14` the same for `entity`), and both must exist.
/// The test is `00d46d80`, whose result byte the exe has written into the argument's own
/// stack slot.
pub fn fn_00d635e0(e: &mut Engine, this: Ptr, entity: u32) {
    let registry: u32 = e.global(TYPE_REGISTRY);
    let by_name = e.vcall(registry, 0x10, &args![]).u32();
    let vehicle_class = e.vcall(by_name, 0x10, &args![VEHICLE_CLASS_NAME]).u32();
    let by_object = e.vcall(registry, 0x14, &args![]).u32();
    let entity_class = e.vcall(by_object, 0x10, &args![entity]).u32();
    if vehicle_class == 0 || entity_class == 0 {
        return;
    }
    let derived = e.with_stack(4, |e, slot| {
        let result = e
            .call(CLASS_DERIVES, &args![vehicle_class, slot, entity_class])
            .u32();
        e.mem.u8(result)
    });
    if derived == 0 {
        return;
    }
    e.call(ADD_REFERENCE, &args![entity]);
    let array = this.addr().wrapping_add(8);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array.wrapping_add(8)) & CAPACITY_MASK;
    if e.mem.u32(array.wrapping_add(4)) == capacity {
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array.wrapping_add(4));
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), entity);
    e.mem.set_u32(array.wrapping_add(4), size.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_34e0, fn_00d634e0(Ptr)),
        entry!(0x00d6_3520, fn_00d63520(Ptr, Ptr)),
        entry!(0x00d6_3590, fn_00d63590(Ptr, u32)),
        entry!(0x00d6_35e0, fn_00d635e0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod vehicle_viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let seen = Rc::clone(log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_00d634e0_calls_the_slot_for_each_world() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, 0x0900_0011, 0);
        e.map(0x0900_0000, 0x100);
        e.mem.set_u32(0x0900_000c, 0x0900_0011);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x0900_0008);
        let container = e.mem.alloc(0x80);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 0x1111);
        e.mem.set_u32(list + 4, 0x2222);
        e.mem.set_u32(container + 0x70, 2);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x1c, container);
        e.call(0x00d6_34e0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x0900_0011, vec![this + 0x18, 0x1111]),
                (0x0900_0011, vec![this + 0x18, 0x2222]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(container + 0x70, 0);
        e.call(0x00d6_34e0, &args![this]);
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00d6_34e0, &args![this]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00d63520_registers_the_listeners_and_replays_the_entities() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        e.map(0x0900_0000, 0x100);
        // The sub-object at this + 8 has its vtable at 0x09000010 (slot 4 at 0x09000014).
        e.mem.set_u32(0x0900_0014, 0x0900_0021);
        record(&mut e, &log, 0x0900_0021, 0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 0x0900_0010);
        let list = e.mem.alloc(0x40);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0xaaaa);
        e.mem.set_u32(data + 4, 0xbbbb);
        e.mem.set_u32(list + 0x20, data);
        e.mem.set_u32(list + 0x24, 2);
        record(&mut e, &log, WORLD_ADD_LISTENER, 0);
        record(&mut e, &log, WORLD_ADD_POST_SIMULATION_LISTENER, 0);
        record(&mut e, &log, WORLD_ENTITIES, list);
        record(&mut e, &log, REMOVE_REFERENCE, 0);
        e.call(0x00d6_3520, &args![this, 0x7000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_ADD_LISTENER, vec![0x7000, this + 8]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x7000, this + 0xc]),
                (WORLD_ENTITIES, vec![0x7000]),
                (0x0900_0021, vec![this + 8, 0xaaaa]),
                (0x0900_0021, vec![this + 8, 0xbbbb]),
                (REMOVE_REFERENCE, vec![list]),
            ]
        );
        // `this - 0x20` null: both listeners are 0; an empty list calls nothing.
        log.borrow_mut().clear();
        e.mem.set_u32(list + 0x24, 0);
        e.call(0x00d6_3520, &args![0x20u32, 0x7000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_ADD_LISTENER, vec![0x7000, 0]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x7000, 0]),
                (WORLD_ENTITIES, vec![0x7000]),
                (REMOVE_REFERENCE, vec![list]),
            ]
        );
    }

    fn object_array(e: &mut Engine, values: &[u32]) -> u32 {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, values.len() as u32);
        e.mem.set_u32(this + 0x10, values.len() as u32 + 1);
        this
    }

    fn items(e: &Engine, this: u32) -> Vec<u32> {
        let data = e.mem.u32(this + 8);
        (0..e.mem.u32(this + 0xc))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00d63590_swaps_the_last_element_into_the_hole() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, REMOVE_REFERENCE, 0);
        let this = object_array(&mut e, &[10, 20, 30]);
        e.call(0x00d6_3590, &args![this, 10u32]);
        assert_eq!(items(&e, this), vec![30, 20]);
        // The last element is removed by lowering the size only.
        e.call(0x00d6_3590, &args![this, 20u32]);
        assert_eq!(items(&e, this), vec![30]);
        // No match: nothing changes and no reference is released.
        e.call(0x00d6_3590, &args![this, 99u32]);
        assert_eq!(items(&e, this), vec![30]);
        assert_eq!(
            *log.borrow(),
            vec![(REMOVE_REFERENCE, vec![10]), (REMOVE_REFERENCE, vec![20])]
        );
        // An empty array is left alone.
        let empty = object_array(&mut e, &[]);
        e.call(0x00d6_3590, &args![empty, 10u32]);
        assert_eq!(log.borrow().len(), 2);
    }

    const REGISTRY: u32 = 0x0900_0100;
    const REGISTRY_VTABLE: u32 = 0x0900_0120;
    const BY_NAME: u32 = 0x0900_0140;
    const BY_NAME_VTABLE: u32 = 0x0900_0160;
    const BY_OBJECT: u32 = 0x0900_0180;
    const BY_OBJECT_VTABLE: u32 = 0x0900_01a0;
    /// Three words the doubles read: the vehicle class, the entity class, the test result byte.
    const CONTROL: u32 = 0x0900_0200;

    /// The registry and its lookup objects, with doubles that read their answers from the
    /// control words, and the class test double writing the result byte.
    fn registry(vehicle_class: u32, entity_class: u32, derived: u8) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0900_0000, 0x1000);
        e.map(0x0126_8000, 0x1000);
        e.mem.set_u32(CONTROL, vehicle_class);
        e.mem.set_u32(CONTROL + 4, entity_class);
        e.mem.set_u32(CONTROL + 8, u32::from(derived));
        e.set_global(TYPE_REGISTRY, REGISTRY);
        e.mem.set_u32(REGISTRY, REGISTRY_VTABLE);
        e.mem.set_u32(REGISTRY_VTABLE + 0x10, 0x0900_0301);
        e.mem.set_u32(REGISTRY_VTABLE + 0x14, 0x0900_0302);
        e.mem.set_u32(BY_NAME, BY_NAME_VTABLE);
        e.mem.set_u32(BY_NAME_VTABLE + 0x10, 0x0900_0303);
        e.mem.set_u32(BY_OBJECT, BY_OBJECT_VTABLE);
        e.mem.set_u32(BY_OBJECT_VTABLE + 0x10, 0x0900_0304);
        e.register(0x0900_0301, |_, _| Ret {
            eax: BY_NAME,
            ..Ret::default()
        });
        e.register(0x0900_0302, |_, _| Ret {
            eax: BY_OBJECT,
            ..Ret::default()
        });
        e.register(0x0900_0303, |e, a| {
            assert_eq!(a, &[BY_NAME, VEHICLE_CLASS_NAME][..]);
            Ret {
                eax: e.mem.u32(CONTROL),
                ..Ret::default()
            }
        });
        e.register(0x0900_0304, |e, a| {
            assert_eq!(a[0], BY_OBJECT);
            Ret {
                eax: e.mem.u32(CONTROL + 4),
                ..Ret::default()
            }
        });
        let log: Log = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(CLASS_DERIVES, move |e, a| {
            seen.borrow_mut().push((CLASS_DERIVES, a.to_vec()));
            e.mem.set_u8(a[1], e.mem.u32(CONTROL + 8) as u8);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        record(&mut e, &log, ADD_REFERENCE, 0);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x5555);
        record(&mut e, &log, MEMORY_ROUTER, router);
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            seen.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let array = a[1];
            let data = e.mem.alloc(64);
            let size = e.mem.u32(array + 4);
            for i in 0..size {
                let word = e.mem.u32(e.mem.u32(array) + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, 16);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        (e, log, this)
    }

    #[test]
    fn fn_00d635e0_keeps_an_entity_of_a_derived_class_and_takes_a_reference() {
        let (mut e, log, this) = registry(0xc1, 0xe1, 1);
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 0x10, 0);
        e.call(0x00d6_35e0, &args![this, 0x4321u32]);
        let log = log.borrow();
        assert_eq!(log[0].0, CLASS_DERIVES);
        assert_eq!(log[0].1[0], 0xc1);
        assert_eq!(log[0].1[2], 0xe1);
        assert_eq!(log[1], (ADD_REFERENCE, vec![0x4321]));
        // The array was empty with no capacity: it grew through the router's allocator.
        assert_eq!(log[3], (ARRAY_RESERVE_MORE, vec![0x5555, this + 8, 4]));
        drop(log);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(e.mem.u32(this + 8)), 0x4321);
    }

    #[test]
    fn fn_00d635e0_appends_without_growing_when_there_is_room() {
        let (mut e, log, this) = registry(0xc1, 0xe1, 1);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, 7);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u32(this + 0x10, 4);
        e.call(0x00d6_35e0, &args![this, 0x4321u32]);
        assert!(log.borrow().iter().all(|(a, _)| *a != ARRAY_RESERVE_MORE));
        assert_eq!(e.mem.u32(data + 4), 0x4321);
        assert_eq!(e.mem.u32(this + 0xc), 2);
    }

    #[test]
    fn fn_00d635e0_ignores_other_classes_and_missing_ones() {
        // The class test says no: no reference, nothing kept.
        let (mut e, log, this) = registry(0xc1, 0xe1, 0);
        e.call(0x00d6_35e0, &args![this, 0x4321u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(log.borrow().iter().all(|(a, _)| *a != ADD_REFERENCE));
        // No vehicle class, or no class for the entity: the test is not run.
        for (vehicle, entity) in [(0, 0xe1), (0xc1, 0)] {
            let (mut e, log, this) = registry(vehicle, entity, 1);
            e.call(0x00d6_35e0, &args![this, 0x4321u32]);
            assert!(log.borrow().is_empty());
            assert_eq!(e.mem.u32(this + 0xc), 0);
        }
    }
}
