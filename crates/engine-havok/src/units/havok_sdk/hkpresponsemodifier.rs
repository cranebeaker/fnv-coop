//! `Havok SDK/hkpresponsemodifier.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! All three functions are cdecl and write a profiling record on entry and one on exit
//! (`profile_record`). None has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, profile_record, store_bits, PROFILE_END, SIGN_MASK,
};

/// The memory router (`00c85750`): its `+0x10` is the allocator (slot 4 allocates).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The lookup of a modifier of a given type on an instance (`00d277c0`, cdecl: instance, type);
/// answers the modifier or 0.
const FIND_MODIFIER: u32 = 0x00d2_77c0;
/// Adds a modifier block to an instance (`00d27960`, cdecl: instance, owner, block).
const ADD_MODIFIER: u32 = 0x00d2_7960;
/// Removes the modifier of a type from an instance (`00d27690`, cdecl: instance, owner, type).
const REMOVE_MODIFIER: u32 = 0x00d2_7690;

/// The profiling labels of the three functions.
const LABEL_SET_SURFACE_VELOCITY: u32 = 0x010c_b7dc;
const LABEL_DISABLE_CONSTRAINT: u32 = 0x010c_b80c;
const LABEL_ENABLE_CONSTRAINT: u32 = 0x010c_b81c;

/// The modifier types: the surface velocity (block of 0x30 bytes) and the disable marker
/// (block of 0x20 bytes).
const TYPE_SURFACE_VELOCITY: u32 = 0x1a;
const TYPE_DISABLE: u32 = 0x1b;

/// A modifier block of `size` bytes with the type in its first 16 bits and the size at `+0x10`
/// (16 bits), from the memory router's allocator (slot 4); 0 when the allocator answers 0.
fn allocate_modifier(e: &mut Engine, kind: u32, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    if block != 0 {
        e.mem.set_u16(block, kind as u16);
        e.mem.set_u16(block + 0x10, size as u16);
    }
    block
}

// Translated from 00cdfcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpResponseModifier::setSurfaceVelocity` (engine map), cdecl with four words: `source`
/// answers the constraint instance through its slot `0x40` (nothing happens when it answers 0),
/// `entity` is compared with the instance's first entity (`+0x14`), `owner` is told through its
/// slot `0x18` that it is about to be written, and `velocity` is a 16-byte vector. The vector is
/// negated (sign mask `010c44d0`) when the instance's first entity is `entity`. The surface
/// velocity modifier (type `0x1a`) of the instance gets the vector at `+0x20`; without one a
/// 0x30-byte block is allocated, given the vector and added to the instance.
///
/// The code stores the vector into the new block without testing it; an allocator answering 0
/// faults at address `0x20` there, which is a panic here.
pub fn hkp_response_modifier_set_surface_velocity(
    e: &mut Engine,
    source: Ptr,
    entity: u32,
    owner: Ptr,
    velocity: Ptr,
) {
    let instance = e.vcall(source.addr(), 0x40, &args![]).u32();
    if instance == 0 {
        return;
    }
    e.vcall(owner.addr(), 0x18, &args![]);
    profile_record(e, LABEL_SET_SURFACE_VELOCITY);
    let mut vector = load_bits(e, velocity.addr());
    if e.mem.u32(instance + 0x14) == entity {
        let mask = load_bits(e, SIGN_MASK);
        for lane in 0..4 {
            vector[lane] ^= mask[lane];
        }
    }
    let modifier = e
        .call(FIND_MODIFIER, &args![instance, TYPE_SURFACE_VELOCITY])
        .u32();
    if modifier != 0 {
        store_bits(e, modifier + 0x20, vector);
    } else {
        let block = allocate_modifier(e, TYPE_SURFACE_VELOCITY, 0x30);
        if block == 0 {
            panic!("00cdfcd0: the new modifier block is null; the vector store at +0x20 faults");
        }
        store_bits(e, block + 0x20, vector);
        e.call(ADD_MODIFIER, &args![instance, owner, block]);
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00cdff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpResponseModifier::disableConstraint` (engine map), cdecl with two words: `owner` is told
/// through its slot `0x18` that it is about to be written; an instance without a modifier of
/// type `0x1b` gets a 0x20-byte one added (an allocator answering 0 adds the null block).
pub fn hkp_response_modifier_disable_constraint(e: &mut Engine, instance: Ptr, owner: Ptr) {
    e.vcall(owner.addr(), 0x18, &args![]);
    profile_record(e, LABEL_DISABLE_CONSTRAINT);
    let found = e.call(FIND_MODIFIER, &args![instance, TYPE_DISABLE]).u32();
    if found == 0 {
        let block = allocate_modifier(e, TYPE_DISABLE, 0x20);
        e.call(ADD_MODIFIER, &args![instance, owner, block]);
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00ce0020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpResponseModifier::enableConstraint` (engine map), cdecl with two words: `owner` is told
/// through its slot `0x18` that it is about to be written, the type `0x1b` modifier is removed
/// from the instance, and the solver results of the instance (`[[instance + 0x30] + 0x1c]`,
/// when not null) are cleared: the instance's constraint data (`+0xC`) fills in an 8-byte
/// record through its slot `0x24` (a byte flag of 1, the record) whose second word is the number
/// of results, and each result (two floats) is set to zero.
///
/// The flag word is pushed whole although only its low byte is set (the compiler shares the
/// stack slot with the record's second word); the callee takes the flag as a byte and writes
/// the record, so the three other bytes do not matter and are zero here.
pub fn hkp_response_modifier_enable_constraint(e: &mut Engine, instance: Ptr, owner: Ptr) {
    e.vcall(owner.addr(), 0x18, &args![]);
    profile_record(e, LABEL_ENABLE_CONSTRAINT);
    e.call(REMOVE_MODIFIER, &args![instance, owner, TYPE_DISABLE]);
    let internal = e.mem.u32(instance.addr() + 0x30);
    let results = e.mem.u32(internal + 0x1c);
    if results != 0 {
        let data = e.mem.u32(instance.addr() + 0xc);
        let count = e.with_stack(8, |e, record| {
            e.mem.set_u8(record.addr() + 4, 1);
            let flag = e.mem.u32(record.addr() + 4);
            e.vcall(data, 0x24, &args![flag, record]);
            e.mem.i32(record.addr() + 4)
        });
        for index in 0..count.max(0) as u32 {
            e.mem.set_f32(results + index * 8, 0.0);
            e.mem.set_f32(results + index * 8 + 4, 0.0);
        }
    }
    profile_record(e, PROFILE_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cd_fcd0,
            hkp_response_modifier_set_surface_velocity(Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00cd_ff60,
            hkp_response_modifier_disable_constraint(Ptr, Ptr)
        ),
        entry!(
            0x00ce_0020,
            hkp_response_modifier_enable_constraint(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use std::rc::Rc;

    /// The allocator function of the test router (its slot 4).
    const ALLOCATE: u32 = 0x0070_0004;
    /// Where the test objects' logging doubles live: the object's slot `n` is `base + n`.
    const SOURCE_BASE: u32 = 0x0071_0000;
    const OWNER_BASE: u32 = 0x0072_0000;
    /// The constraint data's slot `0x24` (the runtime info query).
    const RUNTIME_INFO: u32 = 0x0073_0024;

    /// The engine with the sign mask data, the router (whose allocator answers a filled block
    /// when `with_block`, else 0) and a monitor stream with room for `room` bytes. Returns the
    /// engine, the log, the stream, the first record and the block.
    fn setup(room: u32, with_block: bool) -> (Engine, Log, u32, u32, u32) {
        let mut e = engine();
        for lane in 0..4 {
            e.set_global(SIGN_MASK + 4 * lane, 0x8000_0000u32);
        }
        let log = new_log();
        let block = if with_block { e.mem.alloc(0x40) } else { 0 };
        if with_block {
            e.mem.write(block, &[0xee; 0x40]);
        }
        let allocator_table = e.mem.alloc(0x10);
        e.mem.set_u32(allocator_table + 4, ALLOCATE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        log_double(&mut e, &log, MEMORY_ROUTER, router);
        log_double(&mut e, &log, ALLOCATE, block);
        let (stream, records) = monitor(&mut e, room);
        (e, log, stream, records, block)
    }

    /// A source object whose slot `0x40` (a logging double at `SOURCE_BASE + 0x40`) answers
    /// `instance`.
    fn source_answering(e: &mut Engine, log: &Log, instance: u32) -> u32 {
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x40, SOURCE_BASE + 0x40);
        log_double(e, log, SOURCE_BASE + 0x40, instance);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, vtable);
        source
    }

    /// An instance whose first entity is `entity`, and a source answering it.
    fn source_for(e: &mut Engine, log: &Log, entity: u32) -> (u32, u32) {
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x14, entity);
        let source = source_answering(e, log, instance);
        (source, instance)
    }

    fn vector(e: &mut Engine, bits: [u32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        store_bits(e, at, bits);
        at
    }

    const VELOCITY: [u32; 4] = [0x3f80_0000, 0xc000_0000, 0x0000_0000, 0x7fc0_0001];
    const NEGATED: [u32; 4] = [0xbf80_0000, 0x4000_0000, 0x8000_0000, 0xffc0_0001];

    #[test]
    fn surface_velocity_without_an_instance_does_nothing_00cdfcd0() {
        let (mut e, log, stream, records, _) = setup(0x40, false);
        let source = source_answering(&mut e, &log, 0);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        let velocity = vector(&mut e, VELOCITY);
        e.call(0x00cd_fcd0, &args![source, 5u32, owner, velocity]);
        assert_eq!(calls(&log, SOURCE_BASE + 0x40).len(), 1);
        assert!(calls(&log, OWNER_BASE + 0x18).is_empty());
        assert!(calls(&log, FIND_MODIFIER).is_empty());
        assert!(profile_labels(&e, stream, records).is_empty());
    }

    #[test]
    fn surface_velocity_updates_the_existing_modifier_00cdfcd0() {
        for (entity, expected) in [(5u32, NEGATED), (6, VELOCITY)] {
            let (mut e, log, stream, records, _) = setup(0x40, false);
            let (source, instance) = source_for(&mut e, &log, 5);
            let owner = logging_object(&mut e, &log, OWNER_BASE);
            let modifier = e.mem.alloc(0x30);
            e.mem.write(modifier, &[0xee; 0x30]);
            log_double(&mut e, &log, FIND_MODIFIER, modifier);
            log_double(&mut e, &log, ADD_MODIFIER, 0);
            let velocity = vector(&mut e, VELOCITY);
            e.call(0x00cd_fcd0, &args![source, entity, owner, velocity]);
            assert_eq!(calls(&log, SOURCE_BASE + 0x40), vec![vec![source]]);
            assert_eq!(calls(&log, OWNER_BASE + 0x18), vec![vec![owner]]);
            assert_eq!(calls(&log, FIND_MODIFIER), vec![vec![instance, 0x1a]]);
            assert!(calls(&log, ADD_MODIFIER).is_empty());
            assert!(calls(&log, ALLOCATE).is_empty());
            assert_eq!(load_bits(&e, modifier + 0x20), expected);
            assert_eq!(e.mem.u32(modifier), 0xeeee_eeee);
            assert_eq!(e.mem.u32(modifier + 0x10), 0xeeee_eeee);
            // The caller's vector is left alone.
            assert_eq!(load_bits(&e, velocity), VELOCITY);
            assert_eq!(
                profile_labels(&e, stream, records),
                vec![LABEL_SET_SURFACE_VELOCITY, PROFILE_END]
            );
        }
    }

    #[test]
    fn surface_velocity_adds_a_new_modifier_00cdfcd0() {
        let (mut e, log, stream, records, block) = setup(0x40, true);
        let (source, instance) = source_for(&mut e, &log, 5);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        log_double(&mut e, &log, FIND_MODIFIER, 0);
        log_double(&mut e, &log, ADD_MODIFIER, 0);
        let velocity = vector(&mut e, VELOCITY);
        e.call(0x00cd_fcd0, &args![source, 5u32, owner, velocity]);
        assert_eq!(calls(&log, ALLOCATE).len(), 1);
        assert_eq!(calls(&log, ALLOCATE)[0][1], 0x30);
        assert_eq!(e.mem.u16(block), 0x1a);
        assert_eq!(e.mem.u16(block + 0x10), 0x30);
        assert_eq!(e.mem.u16(block + 2), 0xeeee);
        assert_eq!(load_bits(&e, block + 0x20), NEGATED);
        assert_eq!(
            calls(&log, ADD_MODIFIER),
            vec![vec![instance, owner, block]]
        );
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_SET_SURFACE_VELOCITY, PROFILE_END]
        );
    }

    #[test]
    #[should_panic(expected = "00cdfcd0")]
    fn surface_velocity_panics_when_the_allocator_answers_null_00cdfcd0() {
        let (mut e, log, _, _, _) = setup(0x40, false);
        let (source, _) = source_for(&mut e, &log, 5);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        log_double(&mut e, &log, FIND_MODIFIER, 0);
        log_double(&mut e, &log, ADD_MODIFIER, 0);
        let velocity = vector(&mut e, VELOCITY);
        e.call(0x00cd_fcd0, &args![source, 5u32, owner, velocity]);
    }

    #[test]
    fn disable_constraint_keeps_an_existing_modifier_00cdff60() {
        let (mut e, log, stream, records, _) = setup(0x40, false);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        let instance = e.mem.alloc(0x40);
        log_double(&mut e, &log, FIND_MODIFIER, 0x1234);
        log_double(&mut e, &log, ADD_MODIFIER, 0);
        e.call(0x00cd_ff60, &args![instance, owner]);
        assert_eq!(calls(&log, OWNER_BASE + 0x18), vec![vec![owner]]);
        assert_eq!(calls(&log, FIND_MODIFIER), vec![vec![instance, 0x1b]]);
        assert!(calls(&log, ADD_MODIFIER).is_empty());
        assert!(calls(&log, ALLOCATE).is_empty());
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_DISABLE_CONSTRAINT, PROFILE_END]
        );
    }

    #[test]
    fn disable_constraint_adds_a_marker_00cdff60() {
        let (mut e, log, stream, records, block) = setup(0x40, true);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        let instance = e.mem.alloc(0x40);
        log_double(&mut e, &log, FIND_MODIFIER, 0);
        log_double(&mut e, &log, ADD_MODIFIER, 0);
        e.call(0x00cd_ff60, &args![instance, owner]);
        assert_eq!(calls(&log, ALLOCATE).len(), 1);
        assert_eq!(calls(&log, ALLOCATE)[0][1], 0x20);
        assert_eq!(e.mem.u16(block), 0x1b);
        assert_eq!(e.mem.u16(block + 0x10), 0x20);
        assert_eq!(e.mem.u16(block + 2), 0xeeee);
        assert_eq!(
            calls(&log, ADD_MODIFIER),
            vec![vec![instance, owner, block]]
        );
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_DISABLE_CONSTRAINT, PROFILE_END]
        );
    }

    #[test]
    fn disable_constraint_adds_the_null_block_when_the_allocator_fails_00cdff60() {
        let (mut e, log, _, _, _) = setup(0x40, false);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        let instance = e.mem.alloc(0x40);
        log_double(&mut e, &log, FIND_MODIFIER, 0);
        log_double(&mut e, &log, ADD_MODIFIER, 0);
        e.call(0x00cd_ff60, &args![instance, owner]);
        assert_eq!(calls(&log, ADD_MODIFIER), vec![vec![instance, owner, 0]]);
    }

    /// An instance for `enableConstraint`: its solver results (`results`, 0 for none, with
    /// every word 0x7f7f7f7f) and a constraint data whose slot `0x24` answers `count`.
    fn enabled_instance(e: &mut Engine, log: &Log, results: u32, count: u32) -> u32 {
        let instance = e.mem.alloc(0x40);
        let internal = e.mem.alloc(0x40);
        e.mem.set_u32(internal + 0x1c, results);
        e.mem.set_u32(instance + 0x30, internal);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x24, RUNTIME_INFO);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, table);
        e.mem.set_u32(instance + 0xc, data);
        let log = Rc::clone(log);
        e.register_double(RUNTIME_INFO, move |e, a| {
            log.borrow_mut().push((RUNTIME_INFO, a.to_vec()));
            e.mem.set_u32(a[2] + 4, count);
            Ret::default()
        });
        instance
    }

    #[test]
    fn enable_constraint_clears_the_solver_results_00ce0020() {
        for (count, cleared) in [(3u32, 3u32), (0, 0), (0xffff_fffe, 0), (1, 1)] {
            let (mut e, log, stream, records, _) = setup(0x40, false);
            let owner = logging_object(&mut e, &log, OWNER_BASE);
            let results = e.mem.alloc(0x40);
            e.mem.write(results, &[0x7f; 0x40]);
            let instance = enabled_instance(&mut e, &log, results, count);
            log_double(&mut e, &log, REMOVE_MODIFIER, 0);
            e.call(0x00ce_0020, &args![instance, owner]);
            assert_eq!(calls(&log, OWNER_BASE + 0x18), vec![vec![owner]]);
            assert_eq!(
                calls(&log, REMOVE_MODIFIER),
                vec![vec![instance, owner, 0x1b]]
            );
            let data = e.mem.u32(instance + 0xc);
            let asked = calls(&log, RUNTIME_INFO);
            assert_eq!(asked.len(), 1);
            assert_eq!(asked[0][0], data);
            assert_eq!(asked[0][1], 1);
            for word in 0..16u32 {
                let expected = if word < cleared * 2 { 0 } else { 0x7f7f_7f7f };
                assert_eq!(e.mem.u32(results + word * 4), expected, "word {word}");
            }
            assert_eq!(
                profile_labels(&e, stream, records),
                vec![LABEL_ENABLE_CONSTRAINT, PROFILE_END]
            );
        }
    }

    #[test]
    fn enable_constraint_without_solver_results_asks_nothing_00ce0020() {
        let (mut e, log, stream, records, _) = setup(0x40, false);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        let instance = enabled_instance(&mut e, &log, 0, 4);
        log_double(&mut e, &log, REMOVE_MODIFIER, 0);
        e.call(0x00ce_0020, &args![instance, owner]);
        assert_eq!(calls(&log, REMOVE_MODIFIER).len(), 1);
        assert!(calls(&log, RUNTIME_INFO).is_empty());
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_ENABLE_CONSTRAINT, PROFILE_END]
        );
    }

    #[test]
    fn a_full_record_stream_takes_no_records_00ce0020() {
        let (mut e, log, stream, records, _) = setup(0, false);
        let owner = logging_object(&mut e, &log, OWNER_BASE);
        let instance = enabled_instance(&mut e, &log, 0, 4);
        log_double(&mut e, &log, REMOVE_MODIFIER, 0);
        e.call(0x00ce_0020, &args![instance, owner]);
        assert_eq!(e.mem.u32(stream + 4), records);
        assert_eq!(calls(&log, REMOVE_MODIFIER).len(), 1);
    }
}
