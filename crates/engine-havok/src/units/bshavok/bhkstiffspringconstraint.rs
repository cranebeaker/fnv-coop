//! `BSHavok/bhkstiffspringconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkStiffSpringConstraint` (0x10 bytes, vtable `010c52bc`, RTTI-confirmed: a
//! `bhkConstraint`, vtable `0108febc`) keeps the Havok object at `+8` and `pInfo` at `+0xC`.
//! `pInfo` points at the `pData` field (`+4`) of an `hkStiffSpringConstraintCinfo` (0x14
//! bytes, vtable `010c5398`, RTTI-confirmed), so the cinfo itself is `pInfo - 4`. Its
//! `pData` is the 0x50-byte `hkpStiffSpringConstraintData` (constructor `00d4f0f0`): two
//! vectors at `+0x20` and `+0x30` and a `float` at `+0x44`.
//!
//! The functions that allocate Havok objects raise the per-thread allocation word at
//! `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
//! exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie check of the
//! functions marked `Translation incomplete` are not modeled.

use super::bhkconvextransformshape::mulps_lane;
use super::bhklimitedhingeconstraint::{
    allocate_sized, enter_scope, increment, stream_transfer, ALLOCATE, POOL_ALLOCATE, POOL_OBJECT,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// The vtables of `bhkConstraint`, `bhkStiffSpringConstraint` and
/// `hkStiffSpringConstraintCinfo`, with the instance counters of the first two.
const VTABLE_CONSTRAINT: u32 = 0x0108_febc;
const VTABLE_STIFF_SPRING: u32 = 0x010c_52bc;
const VTABLE_CINFO: u32 = 0x010c_5398;
const COUNT_CONSTRAINT: u32 = 0x0126_8188;
const COUNT_STIFF_SPRING: u32 = 0x0126_85f4;
/// Size of the data block and of the cinfo allocation.
const DATA_SIZE: u32 = 0x50;
const CINFO_SIZE: u32 = 0x14;
/// `hkpStiffSpringConstraintData` constructor (`00d4f0f0`, `ECX` = the block, returns it).
const DATA_CONSTRUCT: u32 = 0x00d4_f0f0;
/// `bhkSerializable::Kill` (`00c876a0`), `CloneCritLock` (`00c876f0`) and
/// `CloneCritUnlock` (`00c87700`); the `bhkRefObject` constructor (`00c87c30`);
/// `bhkConstraint`'s destructor (`00c89640`), `CopyMembers` (`00c89730`, `(clone,
/// process_info)`) and `CopyCinfo` (`00c89c80`, one word).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const CONSTRAINT_DESTRUCT: u32 = 0x00c8_9640;
const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
const CONSTRAINT_COPY_CINFO: u32 = 0x00c8_9c80;
/// `hkConstraintCinfo::CloneData` (`00c8a1b0`, `(this, target, info)`), `LoadBinary`
/// (`00c8a230`) and `SaveBinary` (`00c8a280`).
const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;

/// Constructs a `bhkStiffSpringConstraint` in `object`: the `bhkRefObject` constructor
/// (`00c87c30`), the `bhkConstraint` vtable (count `01268188` up by one), `+0xC` cleared,
/// then the class vtable (count `012685f4` up by one).
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_CONSTRAINT);
    e.mem.set_u32(object + 0xc, 0);
    increment(e, COUNT_CONSTRAINT);
    e.mem.set_u32(object, VTABLE_STIFF_SPRING);
    increment(e, COUNT_STIFF_SPRING);
}

// Translated from 00cc58c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc58c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkStiffSpringConstraint::~bhkStiffSpringConstraint` (Xbox PDB): sets the class
/// vtable, runs `bhkSerializable::Kill` (`00c876a0`), takes one off the instance counter
/// `012685f4` and runs the `bhkConstraint` destructor (`00c89640`).
pub fn bhk_stiff_spring_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_STIFF_SPRING);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_STIFF_SPRING);
    e.mem.set_u32(COUNT_STIFF_SPRING, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00cc5930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5930 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: allocates a 0x10-byte `bhkStiffSpringConstraint` (`00aa13e0`) and, when
/// that succeeded, constructs it. Returns the object, or 0. Takes no arguments.
pub fn fn_00cc5930(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

// Translated from 00cc59c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc59c0 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `hkStiffSpringConstraintCinfo::CloneData` (Xbox PDB), slot `4` of the cinfo vtable
/// (`this` is the cinfo, `+4` its data): copies the data block into `target`, which is
/// allocated (0x50 bytes, size word at `+4`) and constructed first (`00d4f0f0`) when 0.
/// The copy is the two 16-byte blocks `+0x20` and `+0x30` and the `float` at `+0x44`.
/// Unless the `float` at `+0x10` of `info` is exactly 1.0 (a NaN is not 1.0), the two
/// blocks are multiplied by it (`MULPS`, the scale in the destination operand).
/// `hkConstraintCinfo::CloneData` (`00c8a1b0`, `(this, target, info)`) finishes. The
/// allocation-scope word is 0x10 meanwhile.
pub fn hk_stiff_spring_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    info: u32,
) {
    let (word, saved) = enter_scope(e);
    let data = e.mem.u32(this.addr() + 4);
    let mut target = target;
    if target == 0 {
        let block = allocate_sized(e, DATA_SIZE);
        target = e.call(DATA_CONSTRUCT, &args![block]).u32();
    }
    for block in [0x20, 0x30] {
        let bytes = e.mem.bytes(data + block, 0x10);
        e.mem.write(target + block, &bytes);
    }
    let bits = x87_quiet_bits(e.mem.u32(data + 0x44));
    e.mem.set_u32(target + 0x44, bits);
    let scale = x87_quiet_bits(e.mem.u32(info + 0x10));
    if f32::from_bits(scale) != 1.0 {
        for offset in (0x20..0x40).step_by(4) {
            let lane = e.mem.u32(target + offset);
            e.mem.set_u32(target + offset, mulps_lane(scale, lane));
        }
    }
    e.call(CINFO_CLONE_DATA, &args![this, target, info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cc5ae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5ae0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkStiffSpringConstraintCinfo::CreateData` (Xbox PDB), slot `0xC` of the cinfo vtable
/// (`this` is the cinfo): when `+4` (the data) is empty, allocates 0x50 bytes (size word
/// at `+4`), constructs the data block there (`00d4f0f0`) and keeps what that returns.
/// Answers 0x50. The allocation-scope word is 0x10 meanwhile.
pub fn hk_stiff_spring_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    if e.mem.u32(this.addr() + 4) == 0 {
        let block = allocate_sized(e, DATA_SIZE);
        let data = e.call(DATA_CONSTRUCT, &args![block]).u32();
        e.mem.set_u32(this.addr() + 4, data);
    }
    e.mem.set_u32(word, saved);
    DATA_SIZE
}

// Translated from 00cc5b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5b90 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkStiffSpringConstraint::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the
/// clone lock (`00c876f0` / `00c87700`) allocates 0x10 bytes (`00aa13e0`), constructs a
/// `bhkStiffSpringConstraint` there when that succeeded and copies the members
/// (`00c89730`, `(clone, process_info)`). Answers the clone (0 when the allocation
/// failed). The allocation-scope word is 0x10 meanwhile.
pub fn bhk_stiff_spring_constraint_create_clone(
    e: &mut Engine,
    this: Ptr,
    process_info: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let clone = if block != 0 {
        construct(e, block);
        block
    } else {
        0
    };
    e.call(CONSTRAINT_COPY_MEMBERS, &args![this, clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cc5c70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5c70 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkStiffSpringConstraint::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created` (a
/// byte) is set to whether `pInfo` was empty. If so, a 0x14-byte cinfo is allocated from
/// the pool (`00aa3e40`) and initialised (vtable `010c5398`, `+4` data empty, `+8`
/// priority 1, `+0xC` and `+0x10` empty), `pInfo` takes its `+4` address (0 when the
/// allocation failed) and, when the Havok object at `+8` exists, `CopyCinfo` (`00c89c80`)
/// is given the cinfo. Answers `pInfo`. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_stiff_spring_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, CINFO_SIZE]).u32();
        let mut info = 0;
        if block != 0 {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, VTABLE_CINFO);
            e.mem.set_u32(block + 4, 0);
            info = block + 4;
        }
        e.mem.set_u32(me + 0xc, info);
        if e.mem.u32(me + 8) != 0 {
            let cinfo = if info != 0 { info - 4 } else { 0 };
            e.call(CONSTRAINT_COPY_CINFO, &args![this, cinfo]);
        }
    }
    let info = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00cc5d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStiffSpringConstraintCinfo::LoadBinary` (Xbox PDB), slot `8` of the cinfo vtable
/// (`this` is the cinfo, `stream` the `NiStream`, whose reader is at `+0x24C`): the base
/// `LoadBinary` (`00c8a230`), then reads 0x10 bytes to data `+0x20` and `+0x30` and four
/// bytes to `+0x44`, advancing the reader's position each time.
pub fn hk_stiff_spring_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in [(0x20, 0x10), (0x30, 0x10), (0x44, 4)] {
        let io = e.mem.u32(stream + 0x24c);
        stream_transfer(e, io, 8, data + offset, size, true);
    }
}

// Translated from 00cc5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStiffSpringConstraintCinfo::SaveBinary` (Xbox PDB), slot `0xC` of the cinfo vtable
/// (`this` is the cinfo, `stream` the `NiStream`, whose writer is at `+0x250`): the base
/// `SaveBinary` (`00c8a280`), then writes 0x10 bytes from data `+0x20` and `+0x30` and
/// four bytes from `+0x44`.
pub fn hk_stiff_spring_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in [(0x20, 0x10), (0x30, 0x10), (0x44, 4)] {
        let io = e.mem.u32(stream + 0x250);
        stream_transfer(e, io, 0xc, data + offset, size, true);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc58c0, bhk_stiff_spring_constraint_destruct(Ptr)),
        entry!(0x00cc5930, fn_00cc5930() -> u32),
        entry!(
            0x00cc59c0,
            hk_stiff_spring_constraint_cinfo_clone_data(Ptr, u32, u32)
        ),
        entry!(0x00cc5ae0, hk_stiff_spring_constraint_cinfo_create_data(Ptr) -> u32),
        entry!(0x00cc5b90, bhk_stiff_spring_constraint_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc5c70, bhk_stiff_spring_constraint_create_cinfo(Ptr, Ptr) -> u32),
        entry!(
            0x00cc5d40,
            hk_stiff_spring_constraint_cinfo_load_binary(Ptr, u32)
        ),
        entry!(
            0x00cc5dd0,
            hk_stiff_spring_constraint_cinfo_save_binary(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    #[test]
    fn destruct_sets_the_vtable_kills_and_counts_down() {
        let mut e = engine();
        e.set_global(COUNT_STIFF_SPRING, 5u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, CONSTRAINT_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_58c0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_STIFF_SPRING);
        assert_eq!(e.global::<u32>(COUNT_STIFF_SPRING), 4);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT]);
        assert_eq!(calls_to(&log, CONSTRAINT_DESTRUCT), [[this]]);
    }

    #[test]
    fn fn_00cc5930_builds_the_object_and_counts_it() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 7);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        assert_eq!(e.call(0x00cc_5930, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_STIFF_SPRING);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 1);
        assert_eq!(e.global::<u32>(COUNT_STIFF_SPRING), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_5930, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn clone_data_copies_the_blocks_and_scales_them_unless_the_scale_is_one() {
        let (mut e, word, block) = engine_with_allocator();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Calls = Rc::default();
        record(&mut e, CINFO_CLONE_DATA, 0, &log);
        let built = e.mem.alloc(0x50);
        e.register_double(DATA_CONSTRUCT, move |_, a| {
            assert_eq!(a[0], block);
            ret(built)
        });
        let data = e.mem.alloc(0x50);
        for lane in 0..8 {
            e.mem.set_f32(data + 0x20 + 4 * lane, 1.0 + lane as f32);
        }
        // A signalling NaN comes out quiet through FLD / FSTP.
        e.mem.set_u32(data + 0x44, 0x7fa0_0000);
        let cinfo = e.mem.alloc(0x10);
        e.mem.set_u32(cinfo + 4, data);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, 1.0);
        // No target: one is allocated (0x50 bytes) and constructed.
        e.call(0x00cc_59c0, &args![cinfo, 0u32, info]);
        assert_eq!(e.mem.u16(block + 4), 0x50);
        assert_eq!(calls_to(&log, CINFO_CLONE_DATA), [[cinfo, built, info]]);
        for lane in 0..8 {
            assert_eq!(e.mem.f32(built + 0x20 + 4 * lane), 1.0 + lane as f32);
        }
        assert_eq!(e.mem.u32(built + 0x44), 0x7fe0_0000);
        assert_eq!(e.mem.u32(word), 0x77);
        // A given target is used as it is, and a scale other than 1.0 multiplies the lanes.
        let target = e.mem.alloc(0x50);
        e.mem.set_f32(info + 0x10, 3.0);
        e.call(0x00cc_59c0, &args![cinfo, target, info]);
        assert_eq!(calls_to(&log, CINFO_CLONE_DATA)[1], [cinfo, target, info]);
        for lane in 0..8 {
            assert_eq!(
                e.mem.f32(target + 0x20 + 4 * lane),
                3.0 * (1.0 + lane as f32)
            );
        }
        // A NaN scale is not 1.0 either; a NaN lane wins over the scale.
        e.mem.set_u32(info + 0x10, 0x7fc0_1234);
        e.call(0x00cc_59c0, &args![cinfo, target, info]);
        assert_eq!(e.mem.u32(target + 0x20), 0x7fc0_1234);
        e.mem.set_f32(info + 0x10, 3.0);
        e.mem.set_u32(data + 0x24, 0x7fc0_4321);
        e.call(0x00cc_59c0, &args![cinfo, target, info]);
        assert_eq!(e.mem.u32(target + 0x24), 0x7fc0_4321);
    }

    #[test]
    fn create_data_builds_the_data_block_only_when_there_is_none() {
        let (mut e, word, block) = engine_with_allocator();
        let built = e.mem.alloc(0x50);
        e.register_double(DATA_CONSTRUCT, move |_, a| {
            assert_eq!(a[0], block);
            ret(built)
        });
        let cinfo = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cc_5ae0, &args![cinfo]).u32(), 0x50);
        assert_eq!(e.mem.u32(cinfo + 4), built);
        assert_eq!(e.mem.u16(block + 4), 0x50);
        assert_eq!(e.mem.u32(word), 0x77);
        // Present data is kept (the constructor double would fail on another block).
        e.mem.set_u32(cinfo + 4, 0x1234);
        assert_eq!(e.call(0x00cc_5ae0, &args![cinfo]).u32(), 0x50);
        assert_eq!(e.mem.u32(cinfo + 4), 0x1234);
    }

    #[test]
    fn create_clone_constructs_and_copies_inside_the_clone_lock() {
        let mut e = engine();
        let clone = e.mem.alloc(0x10);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, CONSTRAINT_COPY_MEMBERS, 0, &log);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        assert_eq!(e.call(0x00cc_5b90, &args![0x5000u32, 0x99u32]).u32(), clone);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                CONSTRAINT_COPY_MEMBERS,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(e.mem.u32(clone), VTABLE_STIFF_SPRING);
        assert_eq!(
            calls_to(&log, CONSTRAINT_COPY_MEMBERS),
            [[0x5000, clone, 0x99]]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        // A failed allocation copies into nothing and answers 0.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_5b90, &args![0x5000u32, 0x98u32]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(
            calls_to(&log, CONSTRAINT_COPY_MEMBERS)[1],
            [0x5000, 0, 0x98]
        );
    }

    #[test]
    fn create_cinfo_builds_the_cinfo_once_and_copies_from_the_havok_object() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        let log: Calls = Rc::default();
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, CONSTRAINT_COPY_CINFO, 0, &log);
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        // No Havok object: nothing is copied.
        assert_eq!(e.call(0x00cc_5c70, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x14]]);
        assert_eq!(e.mem.u32(block), VTABLE_CINFO);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0xc), block + 4);
        assert!(calls_to(&log, CONSTRAINT_COPY_CINFO).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // The second time it is kept.
        assert_eq!(e.call(0x00cc_5c70, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // With a Havok object, the cinfo is handed to CopyCinfo.
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0x7000);
        e.call(0x00cc_5c70, &args![this, created]);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_CINFO), [[this, block]]);
        // A failed allocation leaves pInfo empty and CopyCinfo gets 0.
        e.mem.set_u32(this + 0xc, 0);
        record(&mut e, POOL_ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_5c70, &args![this, created]).u32(), 0);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_CINFO)[1], [this, 0]);
    }

    #[test]
    fn load_and_save_binary_move_the_three_fields_through_the_stream() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, CINFO_LOAD_BINARY, 0, &log);
        record(&mut e, CINFO_SAVE_BINARY, 0, &log);
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x24c);
        let data = e.mem.alloc(0x50);
        let cinfo = e.mem.alloc(0x10);
        e.mem.set_u32(cinfo + 4, data);
        e.call(0x00cc_5d40, &args![cinfo, stream]);
        assert_eq!(calls_to(&log, CINFO_LOAD_BINARY), [[cinfo, stream]]);
        let reads = calls_to(&log, 0x0900_0008);
        let wanted = [(0x20, 0x10), (0x30, 0x10), (0x44, 4)];
        assert_eq!(reads.len(), 3);
        for (read, (offset, size)) in reads.iter().zip(wanted) {
            assert_eq!(read[0], io);
            assert_eq!(read[1], data + offset);
            assert_eq!(read[2], size);
            assert_eq!(read[4], 1);
        }
        assert_eq!(e.mem.u8(data + 0x20), 0x10);
        assert_eq!(e.mem.u8(data + 0x44 + 3), 0x13);
        assert_eq!(e.mem.u32(io + 4), 0x24);
        e.call(0x00cc_5dd0, &args![cinfo, stream]);
        assert_eq!(calls_to(&log, CINFO_SAVE_BINARY), [[cinfo, stream]]);
        let writes = calls_to(&log, 0x0900_000c);
        assert_eq!(writes.len(), 3);
        for (write, (offset, size)) in writes.iter().zip(wanted) {
            assert_eq!(write[1], data + offset);
            assert_eq!(write[2], size);
        }
        assert_eq!(e.mem.u32(io + 4), 0x48);
    }
}
