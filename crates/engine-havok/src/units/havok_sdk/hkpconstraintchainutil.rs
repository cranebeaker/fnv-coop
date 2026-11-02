//! `Havok SDK/hkpconstraintchainutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::removeReference` (`00c905b0`, `this`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpPoweredChainData` constructor (`00e3b8f0`, `this`) and its method that takes one
/// constraint's description (`00e3b850`, `this`, six stack words).
const POWERED_CHAIN_DATA_CONSTRUCT: u32 = 0x00e3_b8f0;
const ADD_CONSTRAINT_INFO: u32 = 0x00e3_b850;
/// The chain instance's constructor (`00dd0530`, `this`, the data) and its `add entity`
/// method (`00dd04d0`, `this`, the entity); both are translated in
/// `hkpconstraintchaininstance.rs`.
const CHAIN_INSTANCE_CONSTRUCT: u32 = 0x00dd_0530;
const CHAIN_ADD_ENTITY: u32 = 0x00dd_04d0;
/// `hkpConstraintUtils::getConstraintPivots` (`00e5ad40`) and `getConstraintMotors`
/// (`00e5ae60`), cdecl.
const GET_CONSTRAINT_PIVOTS: u32 = 0x00e5_ad40;
const GET_CONSTRAINT_MOTORS: u32 = 0x00e5_ae60;
/// The product of two quaternions stored at `+0x160` of the two entities (`00c74b70`, `this` =
/// the result, then the two quaternions; the function name is not known).
const QUATERNION_PRODUCT: u32 = 0x00c7_4b70;
/// The report builder: constructor (`00cb1600`, `this`, buffer, size), text append (`00cc01c0`,
/// `this`, text) and destructor (`00cc00e0`, `this`); the global at `01268360` holds the sink
/// object whose slot `0xc` takes `(1, id, text, file, line)`; `010ecc38` is the file name
/// `.\Constraint\Chain\hkpConstraintChainUtil.cpp`.
const REPORT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const REPORT_BUILDER_TEXT: u32 = 0x00cc_01c0;
const REPORT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
const REPORT_SINK: u32 = 0x0126_8360;
const REPORT_FILE: u32 = 0x010e_cc38;

/// A new block from the memory router's allocator (slot 4) of `size` bytes with its size
/// word (`u16 +4`) set to `size`.
fn allocate_sized(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// The report this unit writes: a builder over a 0x200-byte buffer holding `message`, sent to
/// the sink at `01268360` (slot `0xc`) with `(1, 0xabbad88d or 0xabba88d3, text, file, line)`,
/// then destroyed.
fn report(e: &mut Engine, message: u32, id: u32, line: u32) {
    e.with_stack(0x20c, |e, block| {
        let builder = block.addr();
        let buffer = builder + 0xc;
        e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
        e.call(REPORT_BUILDER_TEXT, &args![builder, message]);
        let sink = e.global::<u32>(REPORT_SINK);
        e.vcall(sink, 0xc, &args![1u32, id, buffer, REPORT_FILE, line]);
        e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
    });
}

/// The last entity the chain instance has collected (`[[chain + 0x38] + [chain + 0x3c] * 4 - 4]`).
fn last_entity(e: &Engine, chain: u32) -> u32 {
    let entities = e.mem.u32(chain + 0x38);
    let count = e.mem.u32(chain + 0x3c);
    e.mem
        .u32(entities.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4))
}

/// Which of the three failures of `build_powered_chain` was met.
enum ChainFault {
    UnsupportedConstraint,
    NotOrdered,
    NoMotors,
}

// Translated from 00e5b510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainUtil::buildPoweredChain` (map name), cdecl `(constraints, copy_motors)`:
/// `constraints` is an array record (`[+0]` the array of constraint instances, `[+4]` the
/// signed count) and `copy_motors` a byte. Builds an `hkpPoweredChainData` (0x40 bytes, `00e3b8f0`) and
/// a chain instance around it (0x48 bytes, `00dd0530`), starts the chain with the first
/// constraint's first entity (its second one when the second constraint shares the first one),
/// and then, for each constraint: gets its pivots (`getConstraintPivots`; an unsupported
/// constraint is reported "Not supported types of constraints used to build a chain!", line
/// `0xb7`), swaps them when the constraint's first entity is not the chain's last, checks
/// that the constraint shares an entity with the chain's last ("Constraints are not ordered
/// properly ! Two consecutive constraint share no common hkpEntity.", line `0xc2`), forms the
/// quaternion product of the two entities' `+0x160` (`00c74b70`), gets the motors
/// (`getConstraintMotors`; none: "No motors extracted from the original constraint.", id
/// `0xabba88d3`, line `0xcf`; a missing cone or plane motor is replaced by the twist motor),
/// clones the three motors through their virtual slot `+0xc` when `copy_motors` is set, and adds
/// everything to the data (`00e3b850`). When the constraint's first entity is the chain's last the
/// byte at `+0x4c` of the data's last 0x50-byte record (`[data + 0x18]`, `[data + 0x1c]`
/// records) is set to 1 and the second entity joins the chain, otherwise the first one does
/// (`00dd04d0`). The data's reference is dropped at the end (`00c905b0`) and the chain
/// instance returned; on a fault the report is written, both references are dropped and the
/// answer is null.
pub fn build_powered_chain(e: &mut Engine, constraints: Ptr, copy_motors: u8) -> Ptr {
    let record = constraints.addr();
    let data_block = allocate_sized(e, 0x40);
    let data = e
        .call(POWERED_CHAIN_DATA_CONSTRUCT, &args![data_block])
        .u32();
    let chain_block = allocate_sized(e, 0x48);
    let chain = e
        .call(CHAIN_INSTANCE_CONSTRUCT, &args![chain_block, data])
        .u32();
    let count = e.mem.i32(record + 4);
    let array = e.mem.u32(record);
    let first = e.mem.u32(array);
    let mut start = e.mem.u32(first + 0x14);
    if count > 1 {
        let second = e.mem.u32(array + 4);
        if start == e.mem.u32(second + 0x14) || start == e.mem.u32(second + 0x18) {
            start = e.mem.u32(first + 0x18);
        }
    }
    e.call(CHAIN_ADD_ENTITY, &args![chain, start]);
    if e.mem.i32(record + 4) > 0 {
        let fault = e.with_stack(0x50, |e, frame| {
            let pivot_a = frame.addr();
            let pivot_b = frame.addr() + 0x10;
            let product = frame.addr() + 0x20;
            let motors = frame.addr() + 0x30;
            let mut index = 0u32;
            loop {
                let constraint = e.mem.u32(e.mem.u32(record) + 4 * index);
                if e.call(GET_CONSTRAINT_PIVOTS, &args![constraint, pivot_a, pivot_b])
                    .u32()
                    == 1
                {
                    return Some(ChainFault::UnsupportedConstraint);
                }
                let constraint = e.mem.u32(e.mem.u32(record) + 4 * index);
                let entity_a = e.mem.u32(constraint + 0x14);
                if entity_a != last_entity(e, chain) {
                    for word in 0..4 {
                        let first = e.mem.u32(pivot_a + 4 * word);
                        let second = e.mem.u32(pivot_b + 4 * word);
                        e.mem.set_u32(pivot_a + 4 * word, second);
                        e.mem.set_u32(pivot_b + 4 * word, first);
                    }
                    let entity_b = e.mem.u32(constraint + 0x18);
                    if entity_b != last_entity(e, chain) {
                        return Some(ChainFault::NotOrdered);
                    }
                }
                let entity_a = e.mem.u32(constraint + 0x14);
                let entity_b = e.mem.u32(constraint + 0x18);
                e.call(
                    QUATERNION_PRODUCT,
                    &args![product, entity_a + 0x160, entity_b + 0x160],
                );
                let constraint_data = e.mem.u32(constraint + 0xc);
                e.call(
                    GET_CONSTRAINT_MOTORS,
                    &args![constraint_data, motors, motors + 4, motors + 8],
                );
                let mut twist = e.mem.u32(motors);
                if twist == 0 {
                    return Some(ChainFault::NoMotors);
                }
                let mut cone = e.mem.u32(motors + 4);
                if cone == 0 {
                    cone = twist;
                }
                let mut plane = e.mem.u32(motors + 8);
                if plane == 0 {
                    plane = twist;
                }
                if copy_motors != 0 {
                    twist = e.vcall(twist, 0xc, &args![]).u32();
                    cone = e.vcall(cone, 0xc, &args![]).u32();
                    plane = e.vcall(plane, 0xc, &args![]).u32();
                }
                e.call(
                    ADD_CONSTRAINT_INFO,
                    &args![data, pivot_a, pivot_b, product, twist, cone, plane],
                );
                let constraint = e.mem.u32(e.mem.u32(record) + 4 * index);
                let mut next = e.mem.u32(constraint + 0x14);
                if next == last_entity(e, chain) {
                    let records = e.mem.u32(data + 0x18);
                    let used = e.mem.u32(data + 0x1c);
                    e.mem.set_u8(
                        records
                            .wrapping_add(used.wrapping_mul(0x50))
                            .wrapping_sub(4),
                        1,
                    );
                    let constraint = e.mem.u32(e.mem.u32(record) + 4 * index);
                    next = e.mem.u32(constraint + 0x18);
                }
                e.call(CHAIN_ADD_ENTITY, &args![chain, next]);
                index += 1;
                if index as i32 >= e.mem.i32(record + 4) {
                    return None;
                }
            }
        });
        if let Some(fault) = fault {
            let (message, id, line) = match fault {
                ChainFault::UnsupportedConstraint => (0x010e_cc68, 0xabba_d88d, 0xb7),
                ChainFault::NotOrdered => (0x010e_cbd8, 0xabba_d88d, 0xc2),
                ChainFault::NoMotors => (0x010e_cba0, 0xabba_88d3, 0xcf),
            };
            report(e, message, id, line);
            e.call(REMOVE_REFERENCE, &args![data]);
            e.call(REMOVE_REFERENCE, &args![chain]);
            return Ptr::NULL;
        }
    }
    e.call(REMOVE_REFERENCE, &args![data]);
    Ptr::new(chain)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_b510, build_powered_chain(Ptr, u8) -> Ptr)]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const MOTOR_CLONE_SLOT: u32 = 0x0900_0200;

    struct Rig {
        e: Engine,
        log: Calls,
        constraints: u32,
        chain: u32,
        data: u32,
        entities: [u32; 3],
    }

    /// An engine with the allocator, constructors, chain collection and report builder
    /// doubled; two constraints `(E1, E2)` and `(E2, E3)` (or the given pairs); the doubles
    /// log into one list.
    fn rig(pairs: [(usize, usize); 2], motors: [[u32; 3]; 2]) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        // The allocator hands out the data block, then the chain block.
        let data_block = e.mem.alloc(0x40);
        let chain_block = e.mem.alloc(0x48);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let blocks = Rc::new(RefCell::new(VecDeque::from([data_block, chain_block])));
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0004, move |_, a| {
            sink.borrow_mut().push((4, a[1..].to_vec()));
            Ret {
                eax: blocks.borrow_mut().pop_front().unwrap(),
                ..Ret::default()
            }
        });
        for address in [POWERED_CHAIN_DATA_CONSTRUCT, CHAIN_INSTANCE_CONSTRUCT] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        // The chain's entity list: the double of the add method appends to it.
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(chain_block + 0x38, list);
        let sink = Rc::clone(&log);
        e.register_double(CHAIN_ADD_ENTITY, move |e, a| {
            sink.borrow_mut().push((CHAIN_ADD_ENTITY, a.to_vec()));
            let count = e.mem.u32(a[0] + 0x3c);
            e.mem.set_u32(list + 4 * count, a[1]);
            e.mem.set_u32(a[0] + 0x3c, count + 1);
            Ret::default()
        });
        for address in [
            REMOVE_REFERENCE,
            QUATERNION_PRODUCT,
            REPORT_BUILDER_CONSTRUCT,
            REPORT_BUILDER_TEXT,
            REPORT_BUILDER_DESTRUCT,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        // The constraint info call logs the data, the first word of both pivots, the
        // quaternion's address and the three motors.
        let sink = Rc::clone(&log);
        e.register_double(ADD_CONSTRAINT_INFO, move |e, a| {
            sink.borrow_mut().push((
                ADD_CONSTRAINT_INFO,
                vec![
                    a[0],
                    e.mem.u32(a[1]),
                    e.mem.u32(a[2]),
                    a[3],
                    a[4],
                    a[5],
                    a[6],
                ],
            ));
            Ret::default()
        });
        // Powered chain data: the info array the last-record flag is set in.
        let infos = e.mem.alloc(0x200);
        e.mem.set_u32(data_block + 0x18, infos);
        e.mem.set_u32(data_block + 0x1c, 2);
        // The constraints.
        let entities = [e.mem.alloc(0x200), e.mem.alloc(0x200), e.mem.alloc(0x200)];
        let array = e.mem.alloc(8);
        for (slot, (a, b)) in pairs.into_iter().enumerate() {
            let constraint = e.mem.alloc(0x40);
            e.mem.set_u32(constraint + 0x14, entities[a]);
            e.mem.set_u32(constraint + 0x18, entities[b]);
            e.mem.set_u32(constraint + 0xc, 0xda70 + slot as u32);
            e.mem.set_u32(array + 4 * slot as u32, constraint);
        }
        let constraints = e.mem.alloc(8);
        e.mem.set_u32(constraints, array);
        e.mem.set_u32(constraints + 4, 2);
        // Pivots: the constraint's data word and 0x10 more in lane 0; motors from the table.
        let sink = Rc::clone(&log);
        e.register_double(GET_CONSTRAINT_PIVOTS, move |e, a| {
            sink.borrow_mut().push((GET_CONSTRAINT_PIVOTS, vec![a[0]]));
            let data_word = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], data_word);
            e.mem.set_u32(a[2], data_word + 0x10);
            Ret::default()
        });
        let queue = Rc::new(RefCell::new(VecDeque::from(motors)));
        let sink = Rc::clone(&log);
        e.register_double(GET_CONSTRAINT_MOTORS, move |e, a| {
            sink.borrow_mut().push((GET_CONSTRAINT_MOTORS, vec![a[0]]));
            let [twist, cone, plane] = queue.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[1], twist);
            e.mem.set_u32(a[2], cone);
            e.mem.set_u32(a[3], plane);
            Ret::default()
        });
        Rig {
            e,
            log,
            constraints,
            chain: chain_block,
            data: data_block,
            entities,
        }
    }

    /// A motor object whose clone slot answers `clone`.
    fn motor(e: &mut Engine, clone: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, MOTOR_CLONE_SLOT + clone);
        e.mem.set_u32(object, vtable);
        e.register_double(MOTOR_CLONE_SLOT + clone, move |_, _| Ret {
            eax: clone,
            ..Ret::default()
        });
        object
    }

    #[test]
    fn a_well_ordered_chain_collects_entities_and_marks_the_infos() {
        let mut rig = rig([(0, 1), (1, 2)], [[0x71, 0x72, 0x73], [0x74, 0, 0]]);
        let [e1, e2, e3] = rig.entities;
        let got = rig.e.call(0x00e5_b510, &args![rig.constraints, 0u32]).u32();
        assert_eq!(got, rig.chain);
        let log = rig.log.borrow();
        let wanted = |address: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, args)| args.clone())
                .collect()
        };
        assert_eq!(
            wanted(CHAIN_ADD_ENTITY),
            [
                vec![rig.chain, e1],
                vec![rig.chain, e2],
                vec![rig.chain, e3]
            ]
        );
        assert_eq!(
            wanted(QUATERNION_PRODUCT)
                .iter()
                .map(|args| args[1..].to_vec())
                .collect::<Vec<_>>(),
            [vec![e1 + 0x160, e2 + 0x160], vec![e2 + 0x160, e3 + 0x160]]
        );
        let infos = wanted(ADD_CONSTRAINT_INFO);
        assert_eq!(infos.len(), 2);
        // (data, pivot a, pivot b, quaternion, twist, cone, plane): the motors with the
        // missing ones replaced by the twist motor.
        assert_eq!(infos[0][0], rig.data);
        assert_eq!(infos[0][4..], [0x71, 0x72, 0x73]);
        assert_eq!(infos[1][4..], [0x74, 0x74, 0x74]);
        assert_eq!(wanted(REMOVE_REFERENCE), [vec![rig.data]]);
        // The flag byte is set at +0x4c of the last record each time (2 records: +0x9c).
        let infos_at = rig.e.mem.u32(rig.data + 0x18);
        assert_eq!(rig.e.mem.u8(infos_at + 0x9c), 1);
    }

    #[test]
    fn a_reversed_constraint_swaps_its_pivots_and_adds_its_first_entity() {
        // The second constraint is (E3, E2): its first entity is not the chain's last (E2).
        let mut rig = rig([(0, 1), (2, 1)], [[0x71, 0, 0], [0x74, 0, 0]]);
        let [e1, e2, e3] = rig.entities;
        let got = rig.e.call(0x00e5_b510, &args![rig.constraints, 0u32]).u32();
        assert_eq!(got, rig.chain);
        let log = rig.log.borrow();
        let added: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == CHAIN_ADD_ENTITY)
            .map(|(_, args)| args[1])
            .collect();
        assert_eq!(added, [e1, e2, e3]);
        let infos: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == ADD_CONSTRAINT_INFO)
            .map(|(_, args)| args)
            .collect();
        // The pivots' first words are the data word 0xda70 + slot for A and +0x10 for B; the
        // second constraint's are swapped.
        assert_eq!(infos[0][1..3], [0xda70, 0xda80]);
        assert_eq!(infos[1][1..3], [0xda81, 0xda71]);
    }

    #[test]
    fn the_motors_are_cloned_when_asked() {
        let mut rig = rig([(0, 1), (1, 2)], [[0, 0, 0], [0, 0, 0]]);
        let twist = motor(&mut rig.e, 0x501);
        let cone = motor(&mut rig.e, 0x502);
        let plane = motor(&mut rig.e, 0x503);
        let other = motor(&mut rig.e, 0x504);
        // The doubles are queued in the rig; replace the motor answer with real objects.
        let queue = Rc::new(RefCell::new(VecDeque::from([
            [twist, cone, plane],
            [other, 0, 0],
        ])));
        rig.e.register_double(GET_CONSTRAINT_MOTORS, move |e, a| {
            let [t, c, p] = queue.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[1], t);
            e.mem.set_u32(a[2], c);
            e.mem.set_u32(a[3], p);
            Ret::default()
        });
        rig.e.call(0x00e5_b510, &args![rig.constraints, 1u32]);
        let log = rig.log.borrow();
        let infos: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == ADD_CONSTRAINT_INFO)
            .map(|(_, args)| args[4..].to_vec())
            .collect();
        assert_eq!(
            infos,
            [vec![0x501, 0x502, 0x503], vec![0x504, 0x504, 0x504]]
        );
    }

    fn install_sink(e: &mut Engine, log: &Calls) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_00c0);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
        let seen = Rc::clone(log);
        e.register_double(0x0900_00c0, move |_, a| {
            seen.borrow_mut().push((0x0900_00c0, a[1..].to_vec()));
            Ret::default()
        });
    }

    #[test]
    fn an_unsupported_constraint_is_reported_and_both_references_dropped() {
        let mut rig = rig([(0, 1), (1, 2)], [[1, 1, 1], [1, 1, 1]]);
        install_sink(&mut rig.e, &rig.log);
        rig.e.register_double(GET_CONSTRAINT_PIVOTS, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let got = rig.e.call(0x00e5_b510, &args![rig.constraints, 0u32]).u32();
        assert_eq!(got, 0);
        let log = rig.log.borrow();
        let sink: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == 0x0900_00c0)
            .map(|(_, args)| args)
            .collect();
        assert_eq!(sink.len(), 1);
        assert_eq!(sink[0][1], 0xabba_d88d);
        assert_eq!(sink[0][3], REPORT_FILE);
        assert_eq!(sink[0][4], 0xb7);
        let removed: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == REMOVE_REFERENCE)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(removed, [rig.data, rig.chain]);
        let texts: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == REPORT_BUILDER_TEXT)
            .map(|(_, args)| args[1])
            .collect();
        assert_eq!(texts, [0x010e_cc68]);
    }

    #[test]
    fn constraints_that_do_not_chain_are_reported() {
        // The second constraint (E3, E3) shares no entity with the chain's last (E2).
        let mut rig = rig([(0, 1), (2, 2)], [[1, 1, 1], [1, 1, 1]]);
        install_sink(&mut rig.e, &rig.log);
        let got = rig.e.call(0x00e5_b510, &args![rig.constraints, 0u32]).u32();
        assert_eq!(got, 0);
        let log = rig.log.borrow();
        let sink: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == 0x0900_00c0)
            .map(|(_, args)| args)
            .collect();
        assert_eq!(sink[0][4], 0xc2);
        assert_eq!(sink[0][1], 0xabba_d88d);
    }

    #[test]
    fn a_constraint_without_motors_is_reported() {
        let mut rig = rig([(0, 1), (1, 2)], [[0, 0, 0], [1, 1, 1]]);
        install_sink(&mut rig.e, &rig.log);
        let got = rig.e.call(0x00e5_b510, &args![rig.constraints, 0u32]).u32();
        assert_eq!(got, 0);
        let log = rig.log.borrow();
        let sink: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == 0x0900_00c0)
            .map(|(_, args)| args)
            .collect();
        assert_eq!(sink[0][1], 0xabba_88d3);
        assert_eq!(sink[0][4], 0xcf);
    }
}
