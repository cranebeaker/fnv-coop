//! `Havok SDK/run_00d65f70` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_00d64650::remove_record;

/// The shape display viewer's own methods (thiscall on the viewer, one argument): `00d65970`
/// takes a world in (`hkpShapeDisplayViewer`'s `worldAdded` shape of function: listeners,
/// record, entities), `00d655e0` finds the index of a world's record in the array at `+0x30`
/// (or -1) and `00d65750` takes down the display state of the record at an index.
const WORLD_ADDED: u32 = 0x00d6_5970;
const FIND_WORLD_RECORD: u32 = 0x00d6_55e0;
const TAKE_DOWN_WORLD_RECORD: u32 = 0x00d6_5750;
/// `hkMapBase<unsigned long, unsigned long, hkMapOperations<unsigned long>>::clearAndDeallocate`
/// (Xbox PDB, `00cb1a90`, thiscall on the map, one argument: the allocator), and `00a29680`
/// (called with `ECX` = the map and no stack words: its body is a plain `RET`, the debug
/// print of the exe; see `bscullingprocess`).
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const EMPTY_DESTRUCTOR: u32 = 0x00a2_9680;
/// The memory router (`00c85750`, no arguments; its `+0x10` is the allocator) and
/// `hkpWorldViewerBase`'s destructor (Xbox PDB, `00ddcc60`, thiscall, no arguments; it answers its
/// `EAX`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const WORLD_VIEWER_DESTRUCT: u32 = 0x00dd_cc60;
/// The vtable pointers of the shape display viewer (RTTI: `hkpShapeDisplayViewer`): the live
/// ones at `+0`, `+8`, `+0x20`, `+0x28` and `+0x2c`, and the two listener base class ones the
/// destructor leaves (`0102f13c`: `hkpEntityListener`; `010ccf7c`: the post-simulation
/// listener's).
const VTABLE_VIEWER: u32 = 0x010d_1b04;
const VTABLE_PROCESS: u32 = 0x010d_1ae4;
const VTABLE_FILTER: u32 = 0x010d_1ad4;
const VTABLE_ENTITY_LISTENER: u32 = 0x010d_1ab8;
const VTABLE_POST_SIMULATION_LISTENER: u32 = 0x010d_1aa8;
const ENTITY_LISTENER_BASE_VTABLE: u32 = 0x0102_f13c;
const POST_SIMULATION_LISTENER_BASE_VTABLE: u32 = 0x010c_cf7c;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d65f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00d65970` on `this - 8` once for each world of the container at `[this + 0x1c]` (a
/// count at `+0x70`, an array of world pointers at `+0x6c`; both read again after every call).
/// Does nothing without a container or without worlds.
pub fn fn_00d65f70(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut container = e.mem.u32(at.wrapping_add(0x1c));
    if container == 0 || e.mem.i32(container.wrapping_add(0x70)) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
        e.call(WORLD_ADDED, &args![at.wrapping_sub(8), world]);
        container = e.mem.u32(at.wrapping_add(0x1c));
        index = index.wrapping_add(1);
        if (index as i32) >= e.mem.i32(container.wrapping_add(0x70)) {
            break;
        }
    }
}

// Translated from 00d65fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` and jumps to `00d65970` with the same stack word.
pub fn fn_00d65fb0(e: &mut Engine, this: Ptr, world: Ptr) {
    e.call(WORLD_ADDED, &args![this.addr().wrapping_sub(0x20), world]);
}

// Translated from 00d65fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes down the record at `index` of the array at `this + 0x30`: the same compiled code as
/// `00d64800` (see `run_00d64650`) with `00d65750` as the take-down routine.
pub fn fn_00d65fc0(e: &mut Engine, this: Ptr, index: i32) {
    remove_record(e, this, index, TAKE_DOWN_WORLD_RECORD);
}

// Translated from 00d66080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the shape display viewer (`ECX` = `this`): puts its vtable pointers
/// back (`+0`, `+8`, `+0x20`, `+0x28`, `+0x2c`), takes down every record of the array at
/// `this + 0x30` from the last to the first (`00d65fc0`), clears three maps (at `+0x54`, `+0x48`
/// and `+0x3c`: `clearAndDeallocate` with the router's allocator, then `00a29680` on the map),
/// empties the array of records (freeing its data through the allocator, slot `0x10`, when the
/// capacity word at `+0x38` is not negative), stores the listener base vtables (`+0x2c`:
/// post-simulation listener, `+0x28`: entity listener) and ends in the `hkpWorldViewerBase`
/// destructor (`00ddcc60`), whose `EAX` it answers.
pub fn fn_00d66080(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    let mut index = e.mem.u32(at.wrapping_add(0x34)).wrapping_sub(1);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at.wrapping_add(8), VTABLE_PROCESS);
    e.mem.set_u32(at.wrapping_add(0x20), VTABLE_FILTER);
    e.mem.set_u32(at.wrapping_add(0x28), VTABLE_ENTITY_LISTENER);
    e.mem
        .set_u32(at.wrapping_add(0x2c), VTABLE_POST_SIMULATION_LISTENER);
    while (index as i32) >= 0 {
        fn_00d65fc0(e, this, index as i32);
        index = index.wrapping_sub(1);
    }
    for map in [0x54u32, 0x48, 0x3c] {
        let map = at.wrapping_add(map);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(EMPTY_DESTRUCTOR, &args![map]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let capacity = e.mem.u32(at.wrapping_add(0x38));
    e.mem.set_u32(at.wrapping_add(0x34), 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(at.wrapping_add(0x30));
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(at.wrapping_add(0x30), 0);
    e.mem.set_u32(at.wrapping_add(0x38), 0x8000_0000);
    e.mem
        .set_u32(at.wrapping_add(0x2c), POST_SIMULATION_LISTENER_BASE_VTABLE);
    e.mem
        .set_u32(at.wrapping_add(0x28), ENTITY_LISTENER_BASE_VTABLE);
    e.call(WORLD_VIEWER_DESTRUCT, &args![this]).u32()
}

// Translated from 00d66160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of a sub-object at `+0x20` (`this - 0x20` is the whole viewer) taking a world: finds
/// the world's record (`00d655e0` on the whole viewer) and, when the index is not negative,
/// takes it down (`00d65fc0`).
pub fn fn_00d66160(e: &mut Engine, this: Ptr, world: Ptr) {
    let whole = Ptr::new(this.addr().wrapping_sub(0x20));
    let index = e.call(FIND_WORLD_RECORD, &args![whole, world]).i32();
    if index >= 0 {
        fn_00d65fc0(e, whole, index);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_5f70, fn_00d65f70(Ptr)),
        entry!(0x00d6_5fb0, fn_00d65fb0(Ptr, Ptr)),
        entry!(0x00d6_5fc0, fn_00d65fc0(Ptr, i32)),
        entry!(0x00d6_6080, fn_00d66080(Ptr) -> u32),
        entry!(0x00d6_6160, fn_00d66160(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod shape_viewer_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ROUTER: u32 = 0x0900_0100;
    const ALLOCATOR: u32 = 0x0900_0120;
    const ALLOCATOR_VTABLE: u32 = 0x0900_0140;
    const FREE_BLOCK: u32 = 0x0900_0202;
    const FREE_ARRAY: u32 = 0x0900_0203;
    /// The world functions the shared record removal calls (`hkpWorld`: remove from the arrays
    /// at `+0x128` and `+0x164`).
    const WORLD_REMOVE_ISLAND_LISTENER: u32 = 0x00c9_17a0;
    const WORLD_REMOVE_POST_SIMULATION_LISTENER: u32 = 0x00c9_18e0;

    struct Rig {
        e: Engine,
        log: Log,
        found: Rc<Cell<i32>>,
    }

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

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0900_0000, 0x1000);
        let log: Log = Rc::default();
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 8, FREE_BLOCK);
        e.mem.set_u32(ALLOCATOR_VTABLE + 0x10, FREE_ARRAY);
        record(&mut e, &log, MEMORY_ROUTER, ROUTER);
        for address in [
            FREE_BLOCK,
            FREE_ARRAY,
            WORLD_ADDED,
            TAKE_DOWN_WORLD_RECORD,
            MAP_CLEAR_AND_DEALLOCATE,
            EMPTY_DESTRUCTOR,
            WORLD_REMOVE_ISLAND_LISTENER,
            WORLD_REMOVE_POST_SIMULATION_LISTENER,
        ] {
            record(&mut e, &log, address, 0);
        }
        record(&mut e, &log, WORLD_VIEWER_DESTRUCT, 0x99);
        let found = Rc::new(Cell::new(-1));
        let seen = Rc::clone(&log);
        let index = Rc::clone(&found);
        e.register_double(FIND_WORLD_RECORD, move |_, a| {
            seen.borrow_mut().push((FIND_WORLD_RECORD, a.to_vec()));
            Ret {
                eax: index.get() as u32,
                ..Ret::default()
            }
        });
        Rig { e, log, found }
    }

    fn calls(rig: &Rig) -> Vec<(u32, Vec<u32>)> {
        rig.log
            .borrow()
            .iter()
            .filter(|(a, _)| *a != MEMORY_ROUTER)
            .cloned()
            .collect()
    }

    /// A viewer whose array at `+0x30` holds records for `worlds` (each with 2 words of owned
    /// storage), and capacity word `+0x38` of `worlds.len() + 1` words.
    fn viewer_with_records(e: &mut Engine, worlds: &[u32]) -> (u32, Vec<(u32, u32)>) {
        let this = e.mem.alloc(0x60);
        let data = e.mem.alloc(4 * worlds.len() as u32 + 4);
        let mut records = Vec::new();
        for (i, world) in worlds.iter().enumerate() {
            let node = e.mem.alloc(0x10);
            let block = e.mem.alloc(8);
            e.mem.set_u32(node, *world);
            e.mem.set_u32(node + 4, block);
            e.mem.set_u32(node + 8, 2);
            e.mem.set_u32(node + 0xc, 2);
            e.mem.set_u32(data + 4 * i as u32, node);
            records.push((node, block));
        }
        e.mem.set_u32(this + 0x30, data);
        e.mem.set_u32(this + 0x34, worlds.len() as u32);
        e.mem.set_u32(this + 0x38, worlds.len() as u32 + 1);
        (this, records)
    }

    #[test]
    fn fn_00d65f70_runs_00d65970_for_each_world_on_this_minus_8() {
        let mut rig = rig();
        let this = rig.e.mem.alloc(0x60) + 8;
        let list = rig.e.mem.alloc(8);
        rig.e.mem.set_u32(list, 0x1111);
        rig.e.mem.set_u32(list + 4, 0x2222);
        let container = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(container + 0x70, 2);
        rig.e.mem.set_u32(container + 0x6c, list);
        rig.e.mem.set_u32(this + 0x1c, container);
        rig.e.call(0x00d6_5f70, &args![this]);
        assert_eq!(
            calls(&rig),
            vec![
                (WORLD_ADDED, vec![this - 8, 0x1111]),
                (WORLD_ADDED, vec![this - 8, 0x2222]),
            ]
        );
        rig.log.borrow_mut().clear();
        rig.e.mem.set_u32(container + 0x70, 0);
        rig.e.call(0x00d6_5f70, &args![this]);
        rig.e.mem.set_u32(this + 0x1c, 0);
        rig.e.call(0x00d6_5f70, &args![this]);
        assert!(calls(&rig).is_empty());
    }

    #[test]
    fn fn_00d65fb0_runs_00d65970_on_this_minus_0x20() {
        let mut rig = rig();
        rig.e.call(0x00d6_5fb0, &args![0x2020u32, 0x5000u32]);
        assert_eq!(calls(&rig), vec![(WORLD_ADDED, vec![0x2000, 0x5000])]);
    }

    #[test]
    fn fn_00d65fc0_takes_a_record_down_through_00d65750() {
        let mut rig = rig();
        let (this, records) = viewer_with_records(&mut rig.e, &[0xa1, 0xa2]);
        rig.e.call(0x00d6_5fc0, &args![this, 0u32]);
        assert_eq!(
            calls(&rig),
            vec![
                (WORLD_REMOVE_ISLAND_LISTENER, vec![0xa1, this + 0x28]),
                (
                    WORLD_REMOVE_POST_SIMULATION_LISTENER,
                    vec![0xa1, this + 0x2c]
                ),
                (TAKE_DOWN_WORLD_RECORD, vec![this, 0]),
                (FREE_ARRAY, vec![ALLOCATOR, records[0].1, 8]),
                (FREE_BLOCK, vec![ALLOCATOR, records[0].0, 0x10]),
            ]
        );
        assert_eq!(rig.e.mem.u32(this + 0x34), 1);
        assert_eq!(rig.e.mem.u32(rig.e.mem.u32(this + 0x30)), records[1].0);
    }

    #[test]
    fn fn_00d66080_takes_every_record_down_clears_the_maps_and_the_array() {
        let mut rig = rig();
        let (this, _) = viewer_with_records(&mut rig.e, &[0xa1, 0xa2]);
        let data = rig.e.mem.u32(this + 0x30);
        let got = rig.e.call(0x00d6_6080, &args![this]).u32();
        assert_eq!(got, 0x99);
        let seen = calls(&rig);
        let order: Vec<u32> = seen
            .iter()
            .filter(|(a, _)| *a == TAKE_DOWN_WORLD_RECORD)
            .map(|(_, a)| a[1])
            .collect();
        assert_eq!(order, vec![1, 0]);
        // The three maps, in the exe's order, after the records.
        let tail: Vec<(u32, Vec<u32>)> = seen
            .iter()
            .skip_while(|(a, _)| *a != MAP_CLEAR_AND_DEALLOCATE)
            .cloned()
            .collect();
        assert_eq!(
            tail,
            vec![
                (MAP_CLEAR_AND_DEALLOCATE, vec![this + 0x54, ALLOCATOR]),
                (EMPTY_DESTRUCTOR, vec![this + 0x54]),
                (MAP_CLEAR_AND_DEALLOCATE, vec![this + 0x48, ALLOCATOR]),
                (EMPTY_DESTRUCTOR, vec![this + 0x48]),
                (MAP_CLEAR_AND_DEALLOCATE, vec![this + 0x3c, ALLOCATOR]),
                (EMPTY_DESTRUCTOR, vec![this + 0x3c]),
                (FREE_ARRAY, vec![ALLOCATOR, data, 12]),
                (WORLD_VIEWER_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(rig.e.mem.u32(this), VTABLE_VIEWER);
        assert_eq!(rig.e.mem.u32(this + 8), VTABLE_PROCESS);
        assert_eq!(rig.e.mem.u32(this + 0x20), VTABLE_FILTER);
        assert_eq!(rig.e.mem.u32(this + 0x28), ENTITY_LISTENER_BASE_VTABLE);
        assert_eq!(
            rig.e.mem.u32(this + 0x2c),
            POST_SIMULATION_LISTENER_BASE_VTABLE
        );
        assert_eq!(rig.e.mem.u32(this + 0x30), 0);
        assert_eq!(rig.e.mem.u32(this + 0x34), 0);
        assert_eq!(rig.e.mem.u32(this + 0x38), 0x8000_0000);
    }

    #[test]
    fn fn_00d66160_takes_down_the_record_of_a_known_world() {
        let mut rig = rig();
        let (base, _) = viewer_with_records(&mut rig.e, &[0xa1, 0xa2]);
        rig.found.set(1);
        rig.e.call(0x00d6_6160, &args![base + 0x20, 0xa2u32]);
        let seen = calls(&rig);
        assert_eq!(seen[0], (FIND_WORLD_RECORD, vec![base, 0xa2]));
        assert!(seen.contains(&(TAKE_DOWN_WORLD_RECORD, vec![base, 1])));
        rig.log.borrow_mut().clear();
        rig.found.set(-1);
        rig.e.call(0x00d6_6160, &args![base + 0x20, 0xa9u32]);
        assert_eq!(calls(&rig), vec![(FIND_WORLD_RECORD, vec![base, 0xa9])]);
    }
}
