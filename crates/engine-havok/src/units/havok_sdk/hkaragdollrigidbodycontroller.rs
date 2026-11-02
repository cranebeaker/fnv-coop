//! `Havok SDK/hkaragdollrigidbodycontroller.cpp` (Xbox PDB source unit), subsystem `Havok SDK`:
//! its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaRagdollRigidBodyController` (Xbox PDB name) is read off the code: `+0` an `hkArray` of the
//! 0x30-byte control records (data, size `+4`, capacity word `+8`), `+0xc` an `hkArray` of 32-bit
//! bone indices (size `+0x10`, capacity word `+0x14`), `+0x18` the bone list that
//! `hkaKeyFrameHierarchyUtility` works on (`+0x18` the count, `+0x1c` the bone objects, `+0x20` the
//! 16-bit parent index of every bone, `+0x24` the indices to use, `+0x28` the factors), `+0x2c` an
//! `hkArray` of the utility's 0x40-byte states (size `+0x30`, capacity word `+0x34`), `+0x38` the
//! ragdoll instance (reference counted; `+0xc` its bone count, `+0x20` the 32-bit index of the
//! rigid body of every skeleton bone, `+0x2c` its skeleton) and `+0x3c` an `hkArray` of 16-bit
//! parent indices (size `+0x40`, capacity word `+0x44`). Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkapose::grow_array;
use crate::units::havok_sdk::hkpsphereshape::copy_vec;

/// The memory router (`00c85750`): its field `+0x10` is the allocator, whose virtual function at
/// `+0x10` frees a block `(data, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::addReference` and `removeReference` (`00c90510`, `00c905b0`, `ECX` = the
/// object).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkaKeyFrameHierarchyUtility::initialize` (`00d6b4b0`, cdecl `(list, out)`) and `applyKeyFrame`
/// (`00d6b8e0`, cdecl `(time_step, target, list, controls, errors)`).
const KEY_FRAME_INITIALIZE: u32 = 0x00d6_b4b0;
const KEY_FRAME_APPLY: u32 = 0x00d6_b8e0;
/// The unnamed routine that writes the default control record (`00cdb360`, `ECX` = the record).
const DEFAULT_CONTROL: u32 = 0x00cd_b360;
/// The bit that marks the storage of an `hkArray` as not owned, and the capacity mask.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The twelve words of the default control record, each float read from the exe's data the way
/// `FLD` / `FSTP` copies it (a signalling NaN comes out quiet): the rest are the constants 0.0
/// and 1.0. The record's meaning is that of the control records `applyKeyFrame` takes.
fn default_control(e: &Engine) -> [u32; 12] {
    let constant = |address: u32| x87_quiet_bits(e.mem.u32(address));
    let shared = constant(0x0101_e2bc);
    let pair = constant(0x0101_7870);
    [
        constant(0x0104_722c),
        0,
        0x3f80_0000,
        constant(0x0101_8180),
        constant(0x0101_81bc),
        constant(0x0104_f498),
        constant(0x0105_3154),
        shared,
        pair,
        pair,
        constant(0x0102_1c9c),
        shared,
    ]
}

/// The number of bones of the skeleton of the instance.
fn skeleton_count(e: &Engine, instance: u32) -> i32 {
    e.mem.u32(e.mem.u32(instance + 0x2c) + 0x1c) as i32
}

// Translated from 00cdb360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = a 0x30-byte control record: writes the default one (`default_control`) and
/// answers its address.
pub fn fn_00cdb360(e: &mut Engine, this: Ptr) -> Ptr {
    let record = this.addr();
    for (k, word) in default_control(e).into_iter().enumerate() {
        e.mem.set_u32(record + 4 * k as u32, word);
    }
    this
}

// Translated from 00cdb3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollRigidBodyController::reinitialize` (Xbox PDB), `ECX` = the controller:
/// `hkaKeyFrameHierarchyUtility::initialize` of the bone list into the state array.
pub fn hka_ragdoll_rigid_body_controller_reinitialize(e: &mut Engine, this: Ptr) {
    let states = e.mem.u32(this.addr() + 0x2c);
    e.call(KEY_FRAME_INITIALIZE, &args![this.addr() + 0x18, states]);
}

// Translated from 00cdb400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollRigidBodyController::hkaRagdollRigidBodyController` (Xbox PDB), `ECX` = the
/// controller, `(instance)`: the constructor. Empties the arrays, keeps `instance` (and adds a
/// reference to it), makes room for one control record and fills it with the default one
/// (`00cdb360`). Then sizes the parent index array to the instance's bone count, with `-1` in
/// every new entry: when that count is the skeleton's own, the skeleton's parent indices are
/// copied; otherwise every skeleton bone that has a rigid body (index not negative in the
/// instance's array) gets, as its entry, the rigid body index of the nearest ancestor that has
/// one (the last index read when the walk ends at a root). Last, the bone list gets its count,
/// the instance's `+8` and the parent indices, the state array gets one 0x40-byte entry per bone
/// and `initialize` runs. Answers the controller.
pub fn hka_ragdoll_rigid_body_controller_constructor(
    e: &mut Engine,
    this: Ptr,
    instance: Ptr,
) -> Ptr {
    let controller = this.addr();
    let instance = instance.addr();
    let parents = controller + 0x3c;
    for (offset, word) in [
        (0x8, NOT_OWNED),
        (0x0, 0),
        (0x4, 0),
        (0xc, 0),
        (0x10, 0),
        (0x14, NOT_OWNED),
        (0x24, 0),
        (0x28, 0),
        (0x2c, 0),
        (0x30, 0),
        (0x34, NOT_OWNED),
        (0x38, instance),
        (0x3c, 0),
        (0x40, 0),
        (0x44, NOT_OWNED),
    ] {
        e.mem.set_u32(controller + offset, word);
    }
    e.call(ADD_REFERENCE, &args![instance]);
    grow_array(e, controller, 1, 0x30);
    let missing = 1i32.wrapping_sub(e.mem.u32(controller + 4) as i32);
    if missing > 0 {
        let words = default_control(e);
        let mut record = e
            .mem
            .u32(controller)
            .wrapping_add(e.mem.u32(controller + 4).wrapping_mul(0x30));
        for _ in 0..missing {
            if record != 0 {
                for (k, word) in words.into_iter().enumerate() {
                    e.mem.set_u32(record + 4 * k as u32, word);
                }
            }
            record = record.wrapping_add(0x30);
        }
    }
    e.mem.set_u32(controller + 4, 1);
    e.with_stack(0x30, |e, record| {
        let defaults = e.call(DEFAULT_CONTROL, &args![record]).u32();
        let first = e.mem.u32(controller);
        for k in 0..12 {
            let word = e.mem.u32(defaults + 4 * k);
            e.mem.set_u32(first + 4 * k, word);
        }
    });
    let count = e.mem.u32(instance + 0xc) as i32;
    let same = count == skeleton_count(e, instance);
    grow_array(e, parents, count, 2);
    let size = e.mem.u32(parents + 4) as i32;
    let missing = count.wrapping_sub(size);
    if missing > 0 {
        let mut at = e
            .mem
            .u32(parents)
            .wrapping_add((size as u32).wrapping_mul(2));
        for _ in 0..missing {
            e.mem.set_u16(at, 0xffff);
            at = at.wrapping_add(2);
        }
    }
    e.mem.set_u32(parents + 4, count as u32);
    let mut bone = 0i32;
    if same {
        while bone < skeleton_count(e, instance) {
            let skeleton_parents = e.mem.u32(e.mem.u32(instance + 0x2c) + 0xc);
            let word = e.mem.u16(skeleton_parents.wrapping_add(2 * bone as u32));
            let at = e.mem.u32(parents).wrapping_add(2 * bone as u32);
            e.mem.set_u16(at, word);
            bone += 1;
        }
    } else {
        while bone < skeleton_count(e, instance) {
            let bodies = e.mem.u32(instance + 0x20);
            let own = e.mem.i32(bodies.wrapping_add(4 * bone as u32));
            if own >= 0 {
                let skeleton_parents = e.mem.u32(e.mem.u32(instance + 0x2c) + 0xc);
                let mut found = -1i32;
                let mut walk = bone;
                loop {
                    walk = i32::from(
                        e.mem
                            .i16(skeleton_parents.wrapping_add((walk as u32).wrapping_mul(2))),
                    );
                    if walk < 0 {
                        break;
                    }
                    found = e
                        .mem
                        .i32(bodies.wrapping_add((walk as u32).wrapping_mul(4)));
                    if found >= 0 {
                        break;
                    }
                }
                let at = e
                    .mem
                    .u32(parents)
                    .wrapping_add((own as u32).wrapping_mul(2));
                e.mem.set_u16(at, found as u16);
            }
            bone += 1;
        }
    }
    e.mem.set_u32(controller + 0x18, count as u32);
    let list_bones = e.mem.u32(instance + 8);
    e.mem.set_u32(controller + 0x1c, list_bones);
    let parent_data = e.mem.u32(parents);
    e.mem.set_u32(controller + 0x20, parent_data);
    let states = controller + 0x2c;
    grow_array(e, states, count, 0x40);
    e.mem.set_u32(states + 4, count as u32);
    let state_data = e.mem.u32(states);
    e.call(KEY_FRAME_INITIALIZE, &args![controller + 0x18, state_data]);
    this
}

/// Frees the storage of the `hkArray` at `array` through the allocator of the memory router, when
/// the array owns it (`bytes` is the size of the block, from the capacity word), and leaves the
/// array empty and not owning anything. The size is zeroed first.
fn release_array(e: &mut Engine, array: u32, bytes: impl Fn(u32) -> u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if NOT_OWNED & capacity == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, bytes(capacity & CAPACITY_MASK)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

// Translated from 00cdb6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the destructor), `ECX` = the controller: drops the reference to the instance, then
/// frees and empties the four arrays (parent indices, states, bone indices, control records,
/// in that order).
pub fn fn_00cdb6f0(e: &mut Engine, this: Ptr) {
    let controller = this.addr();
    let instance = e.mem.u32(controller + 0x38);
    e.call(REMOVE_REFERENCE, &args![instance]);
    release_array(e, controller + 0x3c, |capacity| capacity.wrapping_mul(2));
    release_array(e, controller + 0x2c, |capacity| capacity.wrapping_shl(6));
    release_array(e, controller + 0xc, |capacity| capacity.wrapping_mul(4));
    release_array(e, controller, |capacity| capacity.wrapping_mul(0x30));
}

// Translated from 00cdb7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollRigidBodyController::driveToPose` (Xbox PDB), `ECX` = the controller,
/// `(time_step, poses, transform, errors)`: runs `applyKeyFrame` of the bone list with the control
/// records and a target made of the 0x30-byte `transform`, the transforms to aim at and the state
/// array. The bone indices (`+0x24` of the controller) become the index array when it has as many
/// entries as the list has bones, none otherwise. When the instance's bone count (`+0xc`) is the
/// skeleton's, the transforms are `poses` itself. Otherwise they are built in memory taken from
/// the router's stack (`count * 0x30` rounded up to 0x80): the transform of every skeleton bone
/// of `poses` that has a rigid body (index not negative in the instance's array) goes to that
/// index; afterwards the router's stack is given back, and the block would be freed if the
/// capacity word `count | 0x80000000` could ever be positive.
pub fn hka_ragdoll_rigid_body_controller_drive_to_pose(
    e: &mut Engine,
    this: Ptr,
    time_step: f32,
    poses: Ptr,
    transform: Ptr,
    errors: Ptr,
) {
    let controller = this.addr();
    let poses = poses.addr();
    let states = e.mem.u32(controller + 0x2c);
    e.with_stack(0x38, |e, target| {
        let target = target.addr();
        for offset in [0, 0x10, 0x20] {
            copy_vec(e, transform.addr() + offset, target + offset);
        }
        e.mem.set_u32(target + 0x34, states);
        let list = controller + 0x18;
        let count = e.mem.u32(list);
        let indices = if e.mem.u32(controller + 0x10) == count {
            e.mem.u32(controller + 0xc)
        } else {
            0
        };
        e.mem.set_u32(controller + 0x24, indices);
        let instance = e.mem.u32(controller + 0x38);
        let time = x87_quiet(time_step);
        if e.mem.u32(instance + 0xc) as i32 == skeleton_count(e, instance) {
            e.mem.set_u32(target + 0x30, poses);
            let controls = e.mem.u32(controller);
            e.call(
                KEY_FRAME_APPLY,
                &args![time, target, list, controls, errors],
            );
        } else {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let base = e.mem.u32(router);
            let size = count.wrapping_mul(0x30).wrapping_add(0x7f) & 0xffff_ff80;
            e.mem.set_u32(router, size.wrapping_add(base));
            let mut bone = 0u32;
            while (bone as i32) < skeleton_count(e, instance) {
                let bodies = e.mem.u32(instance + 0x20);
                let body = e.mem.i32(bodies.wrapping_add(4 * bone));
                if body >= 0 {
                    let from = poses.wrapping_add(bone.wrapping_mul(0x30));
                    let to = base.wrapping_add((body as u32).wrapping_mul(0x30));
                    for offset in [0, 0x10, 0x20] {
                        copy_vec(e, from + offset, to + offset);
                    }
                }
                bone += 1;
            }
            e.mem.set_u32(target + 0x30, base);
            let controls = e.mem.u32(controller);
            e.call(
                KEY_FRAME_APPLY,
                &args![time, target, list, controls, errors],
            );
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            e.mem.set_u32(router, base);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            if ((count | NOT_OWNED) as i32) >= 0 {
                let allocator = e.mem.u32(router + 0x10);
                let bytes = (count & CAPACITY_MASK).wrapping_mul(0x30);
                e.vcall(allocator, 0x10, &args![base, bytes]);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_b360, fn_00cdb360(Ptr) -> Ptr),
        entry!(
            0x00cd_b3e0,
            hka_ragdoll_rigid_body_controller_reinitialize(Ptr)
        ),
        entry!(0x00cd_b400, hka_ragdoll_rigid_body_controller_constructor(Ptr, Ptr) -> Ptr),
        entry!(0x00cd_b6f0, fn_00cdb6f0(Ptr)),
        entry!(
            0x00cd_b7c0,
            hka_ragdoll_rigid_body_controller_drive_to_pose(Ptr, f32, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const FREE: u32 = 0x00aa_0010;
    const CONSTANTS: [(u32, f32); 6] = [
        (0x0104_722c, 0.17),
        (0x0101_8180, 0.6),
        (0x0101_81bc, 0.05),
        (0x0104_f498, 1.4),
        (0x0105_3154, 1.8),
        (0x0101_e2bc, 0.1),
    ];

    fn entry(address: u32, words: &[u32]) -> (u32, Vec<u32>) {
        (address, words.to_vec())
    }

    /// The default record the constants make.
    fn expected_default() -> [u32; 12] {
        [
            0.17f32.to_bits(),
            0,
            1.0f32.to_bits(),
            0.6f32.to_bits(),
            0.05f32.to_bits(),
            1.4f32.to_bits(),
            1.8f32.to_bits(),
            0.1f32.to_bits(),
            0.3f32.to_bits(),
            0.3f32.to_bits(),
            0.02f32.to_bits(),
            0.1f32.to_bits(),
        ]
    }

    fn regrow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let data = e.mem.alloc(capacity * element);
        let size = e.mem.u32(array + 4);
        if size > 0 {
            let old = e.mem.bytes(e.mem.u32(array), size * element);
            e.mem.write(data, &old);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    /// An engine with the constants of the default record in memory, the doubles of the router
    /// (its allocator frees through `FREE`), `_reserve`, the reference counting, the two key
    /// frame routines and the free of the allocator, all logging their words.
    fn engine() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for (address, value) in CONSTANTS {
            e.mem.set_f32(address, value);
        }
        e.mem.set_f32(0x0101_7870, 0.3);
        e.mem.set_f32(0x0102_1c9c, 0.02);
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(vtable + 0x10, FREE);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log: Log = Rc::default();
        for address in [ADD_REFERENCE, REMOVE_REFERENCE, KEY_FRAME_INITIALIZE, FREE] {
            let l = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                l.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let l = Rc::clone(&log);
        e.register_double(0x00c9_08c0, move |e, a| {
            l.borrow_mut().push((0x00c9_08c0, a.to_vec()));
            regrow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        (e, log, allocator)
    }

    #[test]
    fn the_default_record_is_built_from_the_exe_constants() {
        let (mut e, _, _) = engine();
        let record = e.mem.alloc(0x30);
        let answer = e.call(0x00cd_b360, &args![record]).u32();
        assert_eq!(answer, record);
        let words: Vec<u32> = (0..12).map(|k| e.mem.u32(record + 4 * k)).collect();
        assert_eq!(words, expected_default());
        // A signalling NaN comes out quiet.
        e.mem.set_u32(0x0104_722c, 0x7f80_0001);
        e.call(0x00cd_b360, &args![record]);
        assert_eq!(e.mem.u32(record), 0x7fc0_0001);
    }

    #[test]
    fn reinitialize_runs_the_utility_on_the_bone_list_and_the_states() {
        let (mut e, log, _) = engine();
        let controller = e.mem.alloc(0x50);
        e.mem.set_u32(controller + 0x2c, 0x1234);
        e.call(0x00cd_b3e0, &args![controller]);
        assert_eq!(
            log.borrow()[..],
            [entry(KEY_FRAME_INITIALIZE, &[controller + 0x18, 0x1234])]
        );
    }

    #[test]
    fn the_destructor_frees_the_owned_arrays_and_empties_all_four() {
        let (mut e, log, allocator) = engine();
        let controller = e.mem.alloc(0x50);
        e.mem.set_u32(controller + 0x38, 0x4242);
        // Parents (2 bytes), states (0x40), bone indices (4) and records (0x30) with their data
        // pointers and capacity words; the bone index array does not own its storage.
        let arrays = [
            (0x3c, 0x1000, 5u32),
            (0x2c, 0x2000, 3),
            (0xc, 0x3000, 0x8000_0004),
            (0x0, 0x4000, 2),
        ];
        for (offset, data, capacity) in arrays {
            e.mem.set_u32(controller + offset, data);
            e.mem.set_u32(controller + offset + 4, 7);
            e.mem.set_u32(controller + offset + 8, capacity);
        }
        e.call(0x00cd_b6f0, &args![controller]);
        assert_eq!(
            log.borrow()[..],
            [
                entry(REMOVE_REFERENCE, &[0x4242]),
                entry(FREE, &[allocator, 0x1000, 10]),
                entry(FREE, &[allocator, 0x2000, 0xc0]),
                entry(FREE, &[allocator, 0x4000, 0x60]),
            ]
        );
        for (offset, _, _) in arrays {
            let words: Vec<u32> = (0..3)
                .map(|k| e.mem.u32(controller + offset + 4 * k))
                .collect();
            assert_eq!(words, [0, 0, 0x8000_0000]);
        }
    }

    /// Logs the words `applyKeyFrame` is given, the router's stack cursor and the 14 words of
    /// the target (the transform, the transforms pointer and the state pointer), and then
    /// `scratch` words from where the target says the transforms are.
    fn register_apply(e: &mut Engine, log: &Log, router: u32, scratch: u32) {
        let l = Rc::clone(log);
        e.register_double(KEY_FRAME_APPLY, move |e, a| {
            let mut seen = vec![a[0], a[2], a[3], a[4], e.mem.u32(router)];
            seen.extend((0..14).map(|k| e.mem.u32(a[1] + 4 * k)));
            let transforms = e.mem.u32(a[1] + 0x30);
            seen.extend((0..scratch).map(|k| e.mem.u32(transforms + 4 * k)));
            l.borrow_mut().push((KEY_FRAME_APPLY, seen));
            Ret::default()
        });
    }

    struct Drive {
        controller: u32,
        instance: u32,
        transform: u32,
        router: u32,
    }

    fn drive_rig(e: &mut Engine, allocator: u32, instance_bones: u32) -> Drive {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let controller = e.mem.alloc(0x50);
        let instance = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0xc, instance_bones);
        e.mem.set_u32(instance + 0x2c, skeleton);
        e.mem.set_u32(skeleton + 0x1c, 3);
        e.mem.set_u32(controller, 0x7001);
        e.mem.set_u32(controller + 0xc, 0x6000);
        e.mem.set_u32(controller + 0x10, 2);
        e.mem.set_u32(controller + 0x18, 2);
        e.mem.set_u32(controller + 0x2c, 0x7002);
        e.mem.set_u32(controller + 0x38, instance);
        let transform = e.mem.alloc(0x30);
        for k in 0..12 {
            e.mem.set_u32(transform + 4 * k, 0x100 + k);
        }
        Drive {
            controller,
            instance,
            transform,
            router,
        }
    }

    #[test]
    fn driving_with_matching_bone_counts_uses_the_poses_as_they_are() {
        let (mut e, log, allocator) = engine();
        let rig = drive_rig(&mut e, allocator, 3);
        register_apply(&mut e, &log, rig.router, 0);
        e.call(
            0x00cd_b7c0,
            &args![rig.controller, 0.5f32, 0x5000u32, rig.transform, 0x7003u32],
        );
        let mut want = vec![0.5f32.to_bits(), rig.controller + 0x18, 0x7001, 0x7003, 0];
        want.extend((0..12).map(|k| 0x100 + k));
        want.extend([0x5000, 0x7002]);
        assert_eq!(log.borrow()[..], [(KEY_FRAME_APPLY, want)]);
        // As many indices as bones: they are passed on, otherwise none.
        assert_eq!(e.mem.u32(rig.controller + 0x24), 0x6000);
        e.mem.set_u32(rig.controller + 0x10, 3);
        e.call(
            0x00cd_b7c0,
            &args![rig.controller, 0.5f32, 0x5000u32, rig.transform, 0x7003u32],
        );
        assert_eq!(e.mem.u32(rig.controller + 0x24), 0);
    }

    #[test]
    fn driving_with_other_bone_counts_rearranges_the_poses_on_the_router_stack() {
        let (mut e, log, allocator) = engine();
        let rig = drive_rig(&mut e, allocator, 2);
        let arena = e.mem.alloc(0x400);
        e.mem.set_u32(rig.router, arena);
        // The rigid body of every skeleton bone: bone 0 is the second, bone 1 has none, bone 2
        // the first.
        let bodies = e.mem.alloc(12);
        for (k, body) in [1i32, -1, 0].into_iter().enumerate() {
            e.mem.set_i32(bodies + 4 * k as u32, body);
        }
        e.mem.set_u32(rig.instance + 0x20, bodies);
        let poses = e.mem.alloc(0x90);
        for k in 0..36 {
            e.mem.set_u32(poses + 4 * k, 0x200 + k);
        }
        register_apply(&mut e, &log, rig.router, 24);
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(
            0x00cd_b7c0,
            &args![rig.controller, signalling, poses, rig.transform, 0x7003u32],
        );
        let mut want = vec![
            0x7fc0_0001,
            rig.controller + 0x18,
            0x7001,
            0x7003,
            arena + 0x80,
        ];
        want.extend((0..12).map(|k| 0x100 + k));
        want.extend([arena, 0x7002]);
        want.extend((0..12).map(|k| 0x218 + k));
        want.extend((0..12).map(|k| 0x200 + k));
        assert_eq!(log.borrow()[..], [(KEY_FRAME_APPLY, want)]);
        assert_eq!(e.mem.u32(rig.router), arena);
    }

    struct Build {
        controller: u32,
        instance: u32,
    }

    /// An instance of `bones` bones over a skeleton whose parent indices are `skeleton_parents`,
    /// with `bodies` as the rigid body index of every skeleton bone.
    fn build_rig(e: &mut Engine, bones: u32, skeleton_parents: &[u16], bodies: &[i32]) -> Build {
        let controller = e.mem.alloc(0x50);
        let instance = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        let parents = e.mem.alloc(2 * skeleton_parents.len() as u32);
        for (k, parent) in skeleton_parents.iter().enumerate() {
            e.mem.set_u16(parents + 2 * k as u32, *parent);
        }
        let body_array = e.mem.alloc(4 * skeleton_parents.len() as u32);
        for (k, body) in bodies.iter().enumerate() {
            e.mem.set_i32(body_array + 4 * k as u32, *body);
        }
        e.mem.set_u32(skeleton + 0xc, parents);
        e.mem
            .set_u32(skeleton + 0x1c, skeleton_parents.len() as u32);
        e.mem.set_u32(instance + 8, 0x1234);
        e.mem.set_u32(instance + 0xc, bones);
        e.mem.set_u32(instance + 0x20, body_array);
        e.mem.set_u32(instance + 0x2c, skeleton);
        Build {
            controller,
            instance,
        }
    }

    fn parents_of(e: &Engine, controller: u32, count: u32) -> Vec<u16> {
        let data = e.mem.u32(controller + 0x3c);
        (0..count).map(|k| e.mem.u16(data + 2 * k)).collect()
    }

    #[test]
    fn construction_over_the_skeleton_copies_its_parent_indices() {
        let (mut e, log, allocator) = engine();
        let rig = build_rig(&mut e, 3, &[0xffff, 0, 1], &[]);
        let answer = e
            .call(0x00cd_b400, &args![rig.controller, rig.instance])
            .u32();
        assert_eq!(answer, rig.controller);
        let c = rig.controller;
        let states = e.mem.u32(c + 0x2c);
        assert_eq!(
            log.borrow()[..],
            [
                entry(ADD_REFERENCE, &[rig.instance]),
                entry(0x00c9_08c0, &[allocator, c, 1, 0x30]),
                entry(0x00c9_08c0, &[allocator, c + 0x3c, 3, 2]),
                entry(0x00c9_08c0, &[allocator, c + 0x2c, 3, 0x40]),
                entry(KEY_FRAME_INITIALIZE, &[c + 0x18, states]),
            ]
        );
        let first: Vec<u32> = (0..12).map(|k| e.mem.u32(e.mem.u32(c) + 4 * k)).collect();
        assert_eq!(first, expected_default());
        assert_eq!(e.mem.u32(c + 4), 1);
        assert_eq!(parents_of(&e, c, 3), [0xffff, 0, 1]);
        assert_eq!(e.mem.u32(c + 0x40), 3);
        assert_eq!(e.mem.u32(c + 0x38), rig.instance);
        assert_eq!(e.mem.u32(c + 0x18), 3);
        assert_eq!(e.mem.u32(c + 0x1c), 0x1234);
        assert_eq!(e.mem.u32(c + 0x20), e.mem.u32(c + 0x3c));
        assert_eq!(e.mem.u32(c + 0x30), 3);
        assert_eq!((e.mem.u32(c + 0xc), e.mem.u32(c + 0x10)), (0, 0));
    }

    #[test]
    fn construction_over_other_bones_maps_the_rigid_bodies_to_their_nearest_parent() {
        let (mut e, log, allocator) = engine();
        // Four skeleton bones in a chain, two rigid bodies: bone 1 has body 0 and bone 3 has
        // body 1; bone 1's nearest ancestor with a body is none, bone 3's is bone 1 (body 0).
        let rig = build_rig(&mut e, 2, &[0xffff, 0, 1, 2], &[-1, 0, -1, 1]);
        e.call(0x00cd_b400, &args![rig.controller, rig.instance]);
        let c = rig.controller;
        assert_eq!(parents_of(&e, c, 2), [0xffff, 0]);
        assert_eq!(e.mem.u32(c + 0x40), 2);
        assert_eq!(e.mem.u32(c + 0x18), 2);
        let states = e.mem.u32(c + 0x2c);
        assert_eq!(
            log.borrow()[..],
            [
                entry(ADD_REFERENCE, &[rig.instance]),
                entry(0x00c9_08c0, &[allocator, c, 1, 0x30]),
                entry(0x00c9_08c0, &[allocator, c + 0x3c, 2, 2]),
                entry(0x00c9_08c0, &[allocator, c + 0x2c, 2, 0x40]),
                entry(KEY_FRAME_INITIALIZE, &[c + 0x18, states]),
            ]
        );
    }

    #[test]
    fn construction_leaves_minus_one_where_the_walk_ends_at_a_root() {
        let (mut e, _, _) = engine();
        // Bone 1 has a body but its parent, bone 0, has none and is a root: the entry is the
        // last index read, -1, and the entries nothing maps to stay -1 too.
        let rig = build_rig(&mut e, 3, &[0xffff, 0], &[-1, 1]);
        e.call(0x00cd_b400, &args![rig.controller, rig.instance]);
        assert_eq!(parents_of(&e, rig.controller, 3), [0xffff, 0xffff, 0xffff]);
    }
}
