//! `Havok SDK/hkadetectragdollpenetration.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{
    allocator, load_vec, max_ps, min_ps, release_array, Vec4, ARRAY_RESERVE_MORE,
};

/// The vtable the destructors of this family leave in the object (`0102e388`).
const BASE_VTABLE: u32 = 0x0102_e388;
/// `hkaPose::calculateBoneModelSpace` (Xbox PDB; `00cda180`, thiscall on the pose, one word:
/// the bone index; returns a pointer to the transform).
const CALCULATE_BONE_MODEL_SPACE: u32 = 0x00cd_a180;
/// `hkAabb::isValid` (Xbox PDB; `00da0b30`, thiscall on the box, one word: a byte the answer is
/// written to; returns a pointer to that byte), `hkpAabbPhantom::setAabb` (Xbox PDB;
/// `00cf24a0`, thiscall on the phantom, one word: the box) and
/// `hkaSkeletonUtils::getDescendants` (Xbox PDB; `00cde450`, cdecl `(skeleton, bone, out
/// array, 0)`).
const AABB_IS_VALID: u32 = 0x00da_0b30;
const PHANTOM_SET_AABB: u32 = 0x00cf_24a0;
const GET_DESCENDANTS: u32 = 0x00cd_e450;
/// The floats a box starts as: every lane of the minimum (`0104da48`, the largest float) and of
/// the maximum (`0104e148`, its negative).
const BOX_START_MINIMUM: u32 = 0x0104_da48;
const BOX_START_MAXIMUM: u32 = 0x0104_e148;

/// Copies 16 bytes (a `MOVAPS` load and store: the bits are kept as they are).
fn copy_16_bytes(e: &mut Engine, from: u32, to: u32) {
    for word in 0..4 {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

fn store_lanes(e: &mut Engine, at: u32, lanes: Vec4) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * lane as u32, value.to_bits());
    }
}

/// The model space transform of `bone` in `pose` (the pose has the transforms, 0x30 bytes
/// each, behind +0x10 and a flag word per bone behind +0x1c): the stored transform when bit 1
/// of the bone's flag is clear, otherwise the result of `hkaPose::calculateBoneModelSpace`.
fn bone_transform(e: &mut Engine, pose: u32, bone: i32) -> u32 {
    let flags = e.mem.u32(pose + 0x1c);
    if e.mem.u8(flags.wrapping_add((bone as u32).wrapping_mul(4))) & 2 == 0 {
        (bone as u32)
            .wrapping_mul(0x30)
            .wrapping_add(e.mem.u32(pose + 0x10))
    } else {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, bone]).u32()
    }
}

/// The address of the state word of `bone` (the object has the array behind +0x18).
fn state_slot(e: &Engine, this: u32, bone: i32) -> u32 {
    e.mem
        .u32(this + 0x18)
        .wrapping_add((bone as u32).wrapping_mul(4))
}

// Translated from 00d1f9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x30-byte hit record (thiscall, no stack words; no PDB name): sets the
/// 16-bit words at +0 and +2 to 0 and 1 and clears the 16 bytes at +0x10 and at +0x20; the
/// other bytes are not written. Returns `this`.
pub fn fn_00d1f9f0(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me, 0);
    e.mem.set_u16(me + 2, 1);
    for word in 0..4 {
        e.mem.set_u32(me + 0x10 + 4 * word, 0);
        e.mem.set_u32(me + 0x20 + 4 * word, 0);
    }
    this
}

// Translated from 00d1fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDetectRagdollPenetration::detectPenetration` (Xbox PDB; thiscall, three stack words:
/// `found`, `pose`, `hits`). The object has the skeleton behind +8 (its parent bone indices,
/// 16-bit, behind +0xc of that), the ray cast interface behind +0xc, the phantom behind +0x10,
/// the bone count at +0x14, an array of state words per bone (0 not tracked, 1, 2) behind
/// +0x18 with its length at +0x1c and a byte at +0x24; `hits` is an array of 0x30-byte
/// records (data, size, capacity word).
///
/// Sets the size of `hits` to 0. When the byte at +0x24 is clear: first builds the box that
/// holds the model space translations of every tracked bone and its parent (bones 1 up, the
/// counter is a 16-bit value) and, when `hkAabb::isValid` accepts it, hands it to
/// `hkpAabbPhantom::setAabb`; then for every tracked bone casts a ray through the ray cast
/// interface's second function from its parent's translation to its own and, when it reports a
/// hit, appends a record (parent index, bone number, the two result vectors) to `hits`
/// (growing it with `00c90940` when full). Then clears all state words. With no records it
/// sets the byte at +0x24 and writes 0 to `found`. Otherwise marks the parent of each record
/// with 1 (unless it already is 2) and all its descendants (from `getDescendants`) with 2 and
/// writes 1 to `found`.
///
/// The records' bytes +4 to +0xf and the scratch layout follow the exe's stack frame, so the
/// frame is modelled at the same offsets: a record is built where the box minimum was, so its
/// bytes +4 to +0xf are lanes 1 to 3 of the minimum.
/// Answers `found` (`EAX` at both `RET`s; callers test the flag through it, 00ca1763).
pub fn hka_detect_ragdoll_penetration_detect_penetration(
    e: &mut Engine,
    this: Ptr,
    found: Ptr,
    pose: Ptr,
    hits: Ptr,
) -> Ptr {
    let me = this.addr();
    let pose = pose.addr();
    let hits = hits.addr();
    e.with_stack(0xb0, |e, frame| {
        let frame = frame.addr();
        // Frame layout (offsets from the exe's stack pointer after its prologue): +0x1b the
        // answer byte of the box check and the ray cast, +0x1c the loop counter, +0x20 the
        // descendants array, +0x3c the parent index, +0x40 the box minimum and then the hit
        // record, +0x50 the box maximum, +0x70/+0x80/+0x90/+0xa0 the ray cast vectors.
        e.mem.set_u32(hits + 4, 0);
        if e.mem.u8(me + 0x24) == 0 {
            let start_minimum = e.mem.f32(BOX_START_MINIMUM);
            let start_maximum = e.mem.f32(BOX_START_MAXIMUM);
            store_lanes(e, frame + 0x40, [start_minimum; 4]);
            store_lanes(e, frame + 0x50, [start_maximum; 4]);
            let bone_count = e.mem.i32(me + 0x14);
            let mut counter = 1u32;
            if bone_count > 1 {
                let mut bone = 1i32;
                loop {
                    let skeleton = e.mem.u32(me + 8);
                    let parents = e.mem.u32(skeleton + 0xc);
                    let parent = i32::from(
                        e.mem
                            .u16(parents.wrapping_add((bone as u32).wrapping_mul(2)))
                            as i16,
                    );
                    let slot = state_slot(e, me, parent);
                    if e.mem.u32(slot) != 0 {
                        let parent_at = bone_transform(e, pose, parent);
                        let parent_position = load_vec(e, parent_at);
                        let bone_at = bone_transform(e, pose, bone);
                        let bone_position = load_vec(e, bone_at);
                        let minimum = load_vec(e, frame + 0x40);
                        let maximum = load_vec(e, frame + 0x50);
                        let minimum = min_ps(min_ps(minimum, parent_position), bone_position);
                        let maximum = max_ps(max_ps(maximum, parent_position), bone_position);
                        store_lanes(e, frame + 0x40, minimum);
                        store_lanes(e, frame + 0x50, maximum);
                    }
                    counter = counter.wrapping_add(1);
                    bone = i32::from(counter as i16);
                    if bone >= bone_count {
                        break;
                    }
                }
            }
            let valid = e
                .call(AABB_IS_VALID, &args![frame + 0x40, frame + 0x1b])
                .u32();
            if e.mem.u8(valid) != 0 {
                let phantom = e.mem.u32(me + 0x10);
                e.call(PHANTOM_SET_AABB, &args![phantom, frame + 0x40]);
            }
            counter = 1;
            if bone_count > 1 {
                let mut bone = 1i32;
                loop {
                    let skeleton = e.mem.u32(me + 8);
                    let parents = e.mem.u32(skeleton + 0xc);
                    let parent_word = e
                        .mem
                        .u16(parents.wrapping_add((bone as u32).wrapping_mul(2)));
                    e.mem.set_u32(frame + 0x3c, u32::from(parent_word));
                    let parent = i32::from(parent_word as i16);
                    let slot = state_slot(e, me, parent);
                    if e.mem.u32(slot) != 0 {
                        let parent_at = bone_transform(e, pose, parent);
                        copy_16_bytes(e, parent_at, frame + 0xa0);
                        let bone_at = bone_transform(e, pose, bone);
                        copy_16_bytes(e, bone_at, frame + 0x70);
                        let ray_caster = e.mem.u32(me + 0xc);
                        let phantom = e.mem.u32(me + 0x10);
                        e.vcall(
                            ray_caster,
                            4,
                            &args![
                                frame + 0x1b,
                                phantom,
                                frame + 0xa0,
                                frame + 0x70,
                                frame + 0x90,
                                frame + 0x80
                            ],
                        );
                        if e.mem.u8(frame + 0x1b) != 0 {
                            fn_00d1f9f0(e, Ptr::new(frame + 0x40));
                            copy_16_bytes(e, frame + 0x90, frame + 0x50);
                            let index_word = e.mem.u16(frame + 0x3c);
                            e.mem.set_u16(frame + 0x40, index_word);
                            e.mem.set_u16(frame + 0x42, counter as u16);
                            copy_16_bytes(e, frame + 0x80, frame + 0x60);
                            let alloc = allocator(e);
                            let capacity = e.mem.u32(hits + 8) & 0x3fff_ffff;
                            if e.mem.u32(hits + 4) == capacity {
                                e.call(ARRAY_RESERVE_MORE, &args![alloc, hits, 0x30u32]);
                            }
                            let size = e.mem.u32(hits + 4);
                            let target = size.wrapping_mul(0x30).wrapping_add(e.mem.u32(hits));
                            // A record is not copied to a null slot (the exe's address sum is
                            // zero).
                            if target != 0 {
                                for word in 0..12 {
                                    let value = e.mem.u32(frame + 0x40 + 4 * word);
                                    e.mem.set_u32(target.wrapping_add(4 * word), value);
                                }
                            }
                            let size = e.mem.u32(hits + 4);
                            e.mem.set_u32(hits + 4, size.wrapping_add(1));
                        }
                    }
                    counter = counter.wrapping_add(1);
                    bone = i32::from(counter as i16);
                    if bone >= bone_count {
                        break;
                    }
                }
            }
        }
        let state_count = e.mem.i32(me + 0x1c);
        let mut index = 0i32;
        if state_count > 0 {
            loop {
                let slot = state_slot(e, me, index);
                e.mem.set_u32(slot, 0);
                index += 1;
                if index >= e.mem.i32(me + 0x1c) {
                    break;
                }
            }
        }
        let record_count = e.mem.i32(hits + 4);
        if record_count == 0 {
            e.mem.set_u8(me + 0x24, 1);
            e.mem.set_u8(found.addr(), 0);
            return;
        }
        if record_count > 0 {
            let mut record = 0i32;
            let mut offset = 0u32;
            loop {
                let records = e.mem.u32(hits);
                let bone_word = e.mem.u16(records.wrapping_add(offset));
                let bone = i32::from(bone_word as i16);
                let slot = state_slot(e, me, bone);
                if e.mem.u32(slot) != 2 {
                    e.mem.set_u32(slot, 1);
                }
                e.mem.set_u32(frame + 0x20, 0);
                e.mem.set_u32(frame + 0x24, 0);
                e.mem.set_u32(frame + 0x28, 0x8000_0000);
                let skeleton = e.mem.u32(me + 8);
                e.call(
                    GET_DESCENDANTS,
                    &args![skeleton, u32::from(bone_word), frame + 0x20, 0u32],
                );
                let descendant_count = e.mem.i32(frame + 0x24);
                if descendant_count > 0 {
                    let mut descendant = 0u32;
                    loop {
                        let data = e.mem.u32(frame + 0x20);
                        let bone = i32::from(e.mem.u16(data.wrapping_add(2 * descendant)) as i16);
                        let slot = state_slot(e, me, bone);
                        if e.mem.u32(slot) != 2 {
                            e.mem.set_u32(slot, 2);
                        }
                        descendant += 1;
                        if descendant as i32 >= e.mem.i32(frame + 0x24) {
                            break;
                        }
                    }
                }
                release_array(e, frame + 0x20, 2);
                record += 1;
                offset = offset.wrapping_add(0x30);
                if record >= e.mem.i32(hits + 4) {
                    break;
                }
            }
        }
        e.mem.set_u8(found.addr(), 1);
    });
    found
}

// Translated from 00d1fe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDetectRagdollPenetration::_scalar_deleting_destructor_` (Xbox PDB; thiscall, one stack
/// word `flags`): empties the array of 32-bit values at +0x18 (data, size +0x1c, capacity word
/// +0x20) through the allocator, leaves the base vtable and, when bit 0 of `flags` is set,
/// returns the block to the memory router's allocator (function at +8) with the 16-bit size
/// kept at +4. Returns `this`.
pub fn hka_detect_ragdoll_penetration_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let me = this.addr();
    release_array(e, me + 0x18, 4);
    e.mem.set_u32(me, BASE_VTABLE);
    if flags & 1 != 0 {
        let alloc = allocator(e);
        let size = u32::from(e.mem.u16(me + 4));
        e.vcall(alloc, 8, &args![me, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_f9f0, fn_00d1f9f0(Ptr) -> Ptr),
        entry!(
            0x00d1_fb00,
            hka_detect_ragdoll_penetration_detect_penetration(Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00d1_fe70, hka_detect_ragdoll_penetration_scalar_deleting_destructor(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const MEMORY_ROUTER: u32 = 0x00c8_5750;
    /// The allocator object's two virtual functions (free a block, free an array) and the
    /// ray cast interface's second virtual function.
    const FREE_BLOCK: u32 = 0x00cc_0008;
    const FREE_ARRAY: u32 = 0x00cc_0010;
    const RAY_CAST: u32 = 0x00cc_0020;
    const PHANTOM: u32 = 0x5555_0000;

    type Positions = Vec<[f32; 4]>;

    /// What the scenario holds: the parent of every bone (index 0 is the root and unused),
    /// the model space translation of every bone, the state word of every bone, whether
    /// each ray cast (in call order) hits, the descendants `getDescendants` reports per
    /// bone, whether the box is valid, the bones whose transform the pose has to compute.
    struct Scenario {
        parents: Vec<i16>,
        positions: Positions,
        states: Vec<u32>,
        rays: Vec<bool>,
        descendants: Vec<(u32, Vec<i16>)>,
        box_valid: bool,
        computed: Vec<usize>,
    }

    struct World {
        e: Engine,
        log: Log,
        alloc: u32,
        this: u32,
        pose: u32,
        hits: u32,
        found: u32,
        state: u32,
    }

    fn scenario(parents: &[i16], positions: &[[f32; 4]]) -> Scenario {
        Scenario {
            parents: parents.to_vec(),
            positions: positions.to_vec(),
            states: vec![1; parents.len()],
            rays: Vec::new(),
            descendants: Vec::new(),
            box_valid: false,
            computed: Vec::new(),
        }
    }

    fn build(s: &Scenario) -> World {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(BOX_START_MINIMUM, 3.0e38);
        e.mem.set_f32(BOX_START_MAXIMUM, -3.0e38);
        let log: Log = Rc::default();
        let count = s.parents.len() as u32;
        // The allocator and the memory router.
        let vtable = e.mem.alloc(0x20);
        let alloc = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(alloc, vtable);
        e.mem.set_u32(vtable + 8, FREE_BLOCK);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [FREE_BLOCK, FREE_ARRAY] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        // The skeleton, the ray cast interface and the detector itself.
        let parents = e.mem.alloc(2 * count);
        for (index, parent) in s.parents.iter().enumerate() {
            e.mem.set_u16(parents + 2 * index as u32, *parent as u16);
        }
        let skeleton = e.mem.alloc(0x20);
        e.mem.set_u32(skeleton + 0xc, parents);
        let ray_table = e.mem.alloc(0x10);
        e.mem.set_u32(ray_table + 4, RAY_CAST);
        let ray_caster = e.mem.alloc(0x10);
        e.mem.set_u32(ray_caster, ray_table);
        let state = e.mem.alloc(4 * count);
        for (index, value) in s.states.iter().enumerate() {
            e.mem.set_u32(state + 4 * index as u32, *value);
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, skeleton);
        e.mem.set_u32(this + 0xc, ray_caster);
        e.mem.set_u32(this + 0x10, PHANTOM);
        e.mem.set_u32(this + 0x14, count);
        e.mem.set_u32(this + 0x18, state);
        e.mem.set_u32(this + 0x1c, count);
        // The pose: stored transforms, the flags and the transforms it computes.
        let transforms = e.mem.alloc(0x30 * count);
        let computed_transforms = e.mem.alloc(0x30 * count);
        let flags = e.mem.alloc(4 * count);
        for (index, position) in s.positions.iter().enumerate() {
            let at = if s.computed.contains(&index) {
                e.mem.set_u8(flags + 4 * index as u32, 2);
                computed_transforms
            } else {
                transforms
            };
            for (lane, value) in position.iter().enumerate() {
                e.mem
                    .set_f32(at + 0x30 * index as u32 + 4 * lane as u32, *value);
            }
        }
        let pose = e.mem.alloc(0x30);
        e.mem.set_u32(pose + 0x10, transforms);
        e.mem.set_u32(pose + 0x1c, flags);
        let hits = e.mem.alloc(0x10);
        e.mem.set_u32(hits + 4, 7);
        e.mem.set_u32(hits + 8, 0x8000_0000);
        let found = e.mem.alloc(4);
        e.mem.set_u8(found, 0xcc);
        // Doubles for the functions the detector calls.
        {
            let log = Rc::clone(&log);
            e.register_double(CALCULATE_BONE_MODEL_SPACE, move |_, a| {
                log.borrow_mut()
                    .push((CALCULATE_BONE_MODEL_SPACE, a.to_vec()));
                Ret {
                    eax: computed_transforms + 0x30 * a[1],
                    ..Ret::default()
                }
            });
        }
        {
            let log = Rc::clone(&log);
            let valid = s.box_valid;
            e.register_double(AABB_IS_VALID, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..8).map(|word| e.mem.u32(a[0] + 4 * word)));
                log.borrow_mut().push((AABB_IS_VALID, words));
                e.mem.set_u8(a[1], u8::from(valid));
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        }
        {
            let log = Rc::clone(&log);
            e.register_double(PHANTOM_SET_AABB, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..8).map(|word| e.mem.u32(a[1] + 4 * word)));
                log.borrow_mut().push((PHANTOM_SET_AABB, words));
                Ret::default()
            });
        }
        {
            let log = Rc::clone(&log);
            let rays = s.rays.clone();
            let mut next = 0usize;
            e.register_double(RAY_CAST, move |e, a| {
                // a: this, found byte, phantom, from, to, second vector, first vector.
                let mut words = a.to_vec();
                words.extend((0..4).map(|word| e.mem.u32(a[3] + 4 * word)));
                words.extend((0..4).map(|word| e.mem.u32(a[4] + 4 * word)));
                log.borrow_mut().push((RAY_CAST, words));
                let hit = rays.get(next).copied().unwrap_or(false);
                for lane in 0..4 {
                    e.mem
                        .set_f32(a[5] + 4 * lane, 100.0 + 10.0 * next as f32 + lane as f32);
                    e.mem
                        .set_f32(a[6] + 4 * lane, 200.0 + 10.0 * next as f32 + lane as f32);
                }
                e.mem.set_u8(a[1], u8::from(hit));
                next += 1;
                Ret::default()
            });
        }
        {
            let log = Rc::clone(&log);
            let descendants = s.descendants.clone();
            e.register_double(GET_DESCENDANTS, move |e, a| {
                log.borrow_mut().push((GET_DESCENDANTS, a.to_vec()));
                let list: Vec<i16> = descendants
                    .iter()
                    .find(|(bone, _)| *bone == a[1])
                    .map(|(_, list)| list.clone())
                    .unwrap_or_default();
                let data = e.mem.alloc(2 * list.len() as u32 + 2);
                for (index, bone) in list.iter().enumerate() {
                    e.mem.set_u16(data + 2 * index as u32, *bone as u16);
                }
                e.mem.set_u32(a[2], data);
                e.mem.set_u32(a[2] + 4, list.len() as u32);
                e.mem.set_u32(a[2] + 8, list.len() as u32);
                Ret::default()
            });
        }
        {
            let log = Rc::clone(&log);
            e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
                log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
                let (array, element) = (a[1], a[2]);
                let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff).max(2) * 2;
                let data = e.mem.alloc(capacity * element);
                let old = e.mem.u32(array);
                for byte in 0..e.mem.u32(array + 4) * element {
                    let value = e.mem.u8(old + byte);
                    e.mem.set_u8(data + byte, value);
                }
                e.mem.set_u32(array, data);
                e.mem.set_u32(array + 8, capacity);
                Ret::default()
            });
        }
        World {
            e,
            log,
            alloc,
            this,
            pose,
            hits,
            found,
            state,
        }
    }

    impl World {
        fn run(&mut self) {
            hka_detect_ragdoll_penetration_detect_penetration(
                &mut self.e,
                Ptr::new(self.this),
                Ptr::new(self.found),
                Ptr::new(self.pose),
                Ptr::new(self.hits),
            );
        }

        fn states(&self, count: u32) -> Vec<u32> {
            (0..count)
                .map(|index| self.e.mem.u32(self.state + 4 * index))
                .collect()
        }

        fn calls_to(&self, address: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|(called, _)| *called == address)
                .map(|(_, words)| words.clone())
                .collect()
        }

        fn record_bytes(&self, index: u32) -> Vec<u8> {
            let data = self.e.mem.u32(self.hits);
            (0..0x30)
                .map(|byte| self.e.mem.u8(data + 0x30 * index + byte))
                .collect()
        }
    }

    fn bits(lanes: [f32; 4]) -> Vec<u32> {
        lanes.iter().map(|value| value.to_bits()).collect()
    }

    const POSITIONS: [[f32; 4]; 4] = [
        [1.0, 2.0, 3.0, 0.0],
        [0.5, 5.0, -1.0, 0.0],
        [-2.0, 4.0, 8.0, 0.0],
        [3.0, -6.0, 2.0, 0.0],
    ];

    #[test]
    fn a_set_flag_skips_the_search_and_reports_nothing() {
        let mut s = scenario(&[0, 0, 1, 1], &POSITIONS);
        s.states = vec![1, 2, 1, 2];
        let mut world = build(&s);
        world.e.mem.set_u8(world.this + 0x24, 1);
        world.run();
        assert!(world.log.borrow().is_empty());
        assert_eq!(world.e.mem.u32(world.hits + 4), 0);
        assert_eq!(world.states(4), vec![0; 4]);
        assert_eq!(world.e.mem.u8(world.this + 0x24), 1);
        assert_eq!(world.e.mem.u8(world.found), 0);
    }

    #[test]
    fn detect_penetration_answers_its_flag_pointer() {
        let mut s = scenario(&[0, 0, 1, 1], &POSITIONS);
        s.states = vec![1, 2, 1, 2];
        let mut world = build(&s);
        world.e.mem.set_u8(world.this + 0x24, 1);
        // EAX is `found` at both RETs; the caller tests the flag through it (00ca1763).
        let answered = world
            .e
            .call(
                0x00d1_fb00,
                &args![world.this, world.found, world.pose, world.hits],
            )
            .u32();
        assert_eq!(answered, world.found);
    }

    #[test]
    fn without_hits_the_flag_is_set_and_nothing_is_found() {
        let mut s = scenario(&[0, 0, 1, 1], &POSITIONS);
        s.rays = vec![false, false, false];
        let mut world = build(&s);
        world.run();
        assert_eq!(world.calls_to(RAY_CAST).len(), 3);
        assert!(world.calls_to(PHANTOM_SET_AABB).is_empty());
        assert!(world.calls_to(GET_DESCENDANTS).is_empty());
        assert!(world.calls_to(ARRAY_RESERVE_MORE).is_empty());
        assert_eq!(world.e.mem.u32(world.hits + 4), 0);
        assert_eq!(world.e.mem.u8(world.this + 0x24), 1);
        assert_eq!(world.e.mem.u8(world.found), 0);
        assert_eq!(world.states(4), vec![0; 4]);
    }

    #[test]
    fn the_box_covers_every_tracked_bone_and_its_parent() {
        let mut s = scenario(&[0, 0, 1, 1], &POSITIONS);
        s.states = vec![1, 0, 1, 1];
        s.box_valid = true;
        let mut world = build(&s);
        world.run();
        // The parent of bones 2 and 3 (bone 1) is not tracked: only bone 1 and its parent (0)
        // are in the box.
        let check = world.calls_to(AABB_IS_VALID);
        assert_eq!(check.len(), 1);
        let box_at = check[0][0];
        let mut expected = bits([0.5, 2.0, -1.0, 0.0]);
        expected.extend(bits([1.0, 5.0, 3.0, 0.0]));
        assert_eq!(check[0][2..].to_vec(), expected);
        let set = world.calls_to(PHANTOM_SET_AABB);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][0], PHANTOM);
        assert_eq!(set[0][1], box_at);
        assert_eq!(set[0][2..].to_vec(), expected);
        // One ray: the bones whose parent is untracked are skipped.
        assert_eq!(world.calls_to(RAY_CAST).len(), 1);
    }

    #[test]
    fn hits_become_records_and_mark_the_states() {
        let mut s = scenario(&[0, 0, 1, 1], &POSITIONS);
        s.rays = vec![true, false, true];
        s.descendants = vec![(0, vec![1, 2, 3]), (1, vec![2, 3])];
        s.box_valid = true;
        let mut world = build(&s);
        world.run();
        assert_eq!(world.e.mem.u8(world.found), 1);
        assert_eq!(world.e.mem.u8(world.this + 0x24), 0);
        assert_eq!(world.states(4), vec![1, 2, 2, 2]);
        assert_eq!(world.e.mem.u32(world.hits + 4), 2);
        // The ray casts run from the parent's translation to the bone's.
        let rays = world.calls_to(RAY_CAST);
        assert_eq!(rays.len(), 3);
        assert_eq!(rays[0][2], PHANTOM);
        assert_eq!(rays[0][7..11].to_vec(), bits(POSITIONS[0]));
        assert_eq!(rays[0][11..15].to_vec(), bits(POSITIONS[1]));
        assert_eq!(rays[1][7..11].to_vec(), bits(POSITIONS[1]));
        assert_eq!(rays[1][11..15].to_vec(), bits(POSITIONS[2]));
        assert_eq!(rays[2][7..11].to_vec(), bits(POSITIONS[1]));
        assert_eq!(rays[2][11..15].to_vec(), bits(POSITIONS[3]));
        // The hit array was empty and not owning storage: it grows once, by one record size.
        let growth = world.calls_to(ARRAY_RESERVE_MORE);
        assert_eq!(growth, vec![vec![world.alloc, world.hits, 0x30]]);
        // Record 0: parent 0, bone 1; record 1: parent 1, bone 3.
        let minimum = {
            let check = world.calls_to(AABB_IS_VALID);
            check[0][2..6].to_vec()
        };
        for (index, (parent, bone, call)) in [(0u16, 1u16, 0u32), (1, 3, 2)].into_iter().enumerate()
        {
            let record = world.record_bytes(index as u32);
            assert_eq!(u16::from_le_bytes([record[0], record[1]]), parent);
            assert_eq!(u16::from_le_bytes([record[2], record[3]]), bone);
            let word = |at: usize| {
                u32::from_le_bytes([record[at], record[at + 1], record[at + 2], record[at + 3]])
            };
            // Bytes 4 to 0xf are lanes 1 to 3 of the box minimum the record was built over.
            assert_eq!([word(4), word(8), word(12)].to_vec(), minimum[1..].to_vec());
            for lane in 0..4usize {
                assert_eq!(
                    f32::from_bits(word(0x10 + 4 * lane)),
                    100.0 + 10.0 * call as f32 + lane as f32
                );
                assert_eq!(
                    f32::from_bits(word(0x20 + 4 * lane)),
                    200.0 + 10.0 * call as f32 + lane as f32
                );
            }
        }
        // The descendant arrays are released with the capacity in bytes.
        let freed = world.calls_to(FREE_ARRAY);
        assert_eq!(freed.len(), 2);
        assert_eq!(freed[0][0], world.alloc);
        assert_eq!(freed[0][2], 6);
        assert_eq!(freed[1][2], 4);
        let asked = world.calls_to(GET_DESCENDANTS);
        assert_eq!(asked.len(), 2);
        assert_eq!((asked[0][1], asked[1][1]), (0, 1));
        assert_eq!(asked[0][3], 0);
    }

    #[test]
    fn a_hit_array_with_room_does_not_grow() {
        let mut s = scenario(&[0, 0], &POSITIONS[..2]);
        s.rays = vec![true];
        let mut world = build(&s);
        let storage = world.e.mem.alloc(0x30 * 3);
        world.e.mem.set_u32(world.hits, storage);
        world.e.mem.set_u32(world.hits + 8, 3);
        world.run();
        assert!(world.calls_to(ARRAY_RESERVE_MORE).is_empty());
        assert_eq!(world.e.mem.u32(world.hits + 4), 1);
        assert_eq!(world.e.mem.u32(world.hits), storage);
        assert_eq!(world.e.mem.u16(storage + 2), 1);
        assert_eq!(world.e.mem.u8(world.found), 1);
    }

    #[test]
    fn transforms_flagged_in_the_pose_are_computed() {
        let mut s = scenario(&[0, 0, 1], &POSITIONS[..3]);
        s.computed = vec![1];
        s.rays = vec![false, false];
        let mut world = build(&s);
        world.run();
        let computed = world.calls_to(CALCULATE_BONE_MODEL_SPACE);
        // Bone 1 is asked for as the bone (twice: box and rays, parent 0 is stored) and as
        // the parent of bone 2 (twice).
        assert_eq!(computed.len(), 4);
        assert!(computed.iter().all(|call| call == &vec![world.pose, 1]));
        let rays = world.calls_to(RAY_CAST);
        assert_eq!(rays[0][11..15].to_vec(), bits(POSITIONS[1]));
        assert_eq!(rays[1][7..11].to_vec(), bits(POSITIONS[1]));
    }

    #[test]
    fn a_single_bone_leaves_the_box_at_its_start_value() {
        let s = scenario(&[0], &POSITIONS[..1]);
        let mut world = build(&s);
        world.e.mem.set_f32(BOX_START_MINIMUM, 3.0e38);
        world.e.mem.set_f32(BOX_START_MAXIMUM, -3.0e38);
        world.run();
        let check = world.calls_to(AABB_IS_VALID);
        assert_eq!(check.len(), 1);
        let mut expected = bits([3.0e38; 4]);
        expected.extend(bits([-3.0e38; 4]));
        assert_eq!(check[0][2..].to_vec(), expected);
        assert!(world.calls_to(RAY_CAST).is_empty());
        assert_eq!(world.e.mem.u8(world.found), 0);
        assert_eq!(world.e.mem.u8(world.this + 0x24), 1);
    }

    #[test]
    fn the_record_constructor_sets_words_and_clears_two_vectors() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let record = e.mem.alloc(0x30);
        for byte in 0..0x30 {
            e.mem.set_u8(record + byte, 0xaa);
        }
        let result = fn_00d1f9f0(&mut e, Ptr::new(record));
        assert_eq!(result, Ptr::new(record));
        assert_eq!(e.mem.u16(record), 0);
        assert_eq!(e.mem.u16(record + 2), 1);
        for byte in 4..0x10 {
            assert_eq!(e.mem.u8(record + byte), 0xaa);
        }
        for byte in 0x10..0x30 {
            assert_eq!(e.mem.u8(record + byte), 0);
        }
    }

    #[test]
    fn the_destructor_frees_the_array_and_leaves_the_vtable() {
        let s = scenario(&[0], &POSITIONS[..1]);
        let mut world = build(&s);
        let this = world.e.mem.alloc(0x30);
        let data = world.e.mem.alloc(20);
        world.e.mem.set_u32(this + 0x18, data);
        world.e.mem.set_u32(this + 0x1c, 3);
        world.e.mem.set_u32(this + 0x20, 5);
        world.e.mem.set_u16(this + 4, 0x2c);
        let result = hka_detect_ragdoll_penetration_scalar_deleting_destructor(
            &mut world.e,
            Ptr::new(this),
            1,
        );
        assert_eq!(result, Ptr::new(this));
        assert_eq!(
            *world.log.borrow(),
            vec![
                (FREE_ARRAY, vec![world.alloc, data, 20]),
                (FREE_BLOCK, vec![world.alloc, this, 0x2c]),
            ]
        );
        assert_eq!(world.e.mem.u32(this + 0x18), 0);
        assert_eq!(world.e.mem.u32(this + 0x1c), 0);
        assert_eq!(world.e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(world.e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn the_destructor_keeps_the_block_without_bit_zero_and_unowned_storage() {
        let s = scenario(&[0], &POSITIONS[..1]);
        let mut world = build(&s);
        let this = world.e.mem.alloc(0x30);
        world.e.mem.set_u32(this + 0x18, 0x1234);
        world.e.mem.set_u32(this + 0x1c, 3);
        world.e.mem.set_u32(this + 0x20, 0x8000_0005);
        hka_detect_ragdoll_penetration_scalar_deleting_destructor(
            &mut world.e,
            Ptr::new(this),
            0xfe,
        );
        assert!(world.log.borrow().is_empty());
        assert_eq!(world.e.mem.u32(this + 0x18), 0);
        assert_eq!(world.e.mem.u32(this), BASE_VTABLE);
    }
}
