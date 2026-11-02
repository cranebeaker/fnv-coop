//! `Havok SDK/hkapose.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaPose` (Xbox PDB name) is read off the code, as the class is not in the Xbox PDB's type
//! list: `+0` the skeleton (`+0xc` the 16-bit parent index of every bone, `+0x18` an array of
//! 8-byte bone records whose byte at `+4` marks a bone that has a reference pose, `+0x1c` the
//! bone count, `+0x24` the reference pose of 0x30-byte transforms and `+0x34` the number of float
//! slots), `+4` an `hkArray` of the local-space transforms (data, size `+8`, capacity word
//! `+0xc`), `+0x10` an `hkArray` of the model-space transforms (size `+0x14`, capacity word
//! `+0x18`), `+0x1c` an `hkArray` of one flag word per bone (size `+0x20`, capacity word `+0x24`),
//! `+0x28` a byte that says the model space is up to date, `+0x29` a byte that says the same of
//! the local space and `+0x2c` an `hkArray` of float slots (size `+0x30`, capacity word `+0x34`).
//! A transform is 0x30 bytes: the translation, the rotation quaternion and the scale, each a
//! 16-byte vector. The flag word of a bone has bit 1 when its local transform is stale, bit 2
//! when its model transform is stale and bits 4 and 8 as marks of the walks over the hierarchy.
//! Nothing here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkakeyframehierarchyutility::{
    add4, mul4, rotate_and_move, rotate_half, sub4,
};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, dot3, load_bits, load_vec, maxps, minps, refined_reciprocal, shufps, store_vec,
    unpckhps, Vec4,
};

/// The memory router (`00c85750`): its field `+0x10` is the allocator, which
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) takes.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The top bit of a capacity word marks storage that is not owned; the rest is the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// `hkString::memCpy` (`00cb1260`, cdecl `(destination, source, size)`).
const MEM_COPY: u32 = 0x00cb_1260;
/// `hkQuaternion::setMul` (`00c66320`, `this` = the result, `(a, b)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// What `accessSyncedPoseModelSpace` ends with (`00cd9fe0`, `this` only, no name in the Xbox
/// PDB): it answers a pointer.
const POSE_MODEL_SPACE_CHANGED: u32 = 0x00cd_9fe0;
/// This unit's own routines, which the translations call the way the code does.
const COPY_LOCAL_TRANSFORMS: u32 = 0x00cd_9f10;
const COPY_MODEL_TRANSFORMS: u32 = 0x00cd_9f60;
const GET_LOCAL_TRANSFORMS: u32 = 0x00cd_9fb0;
const CALCULATE_BONE_MODEL_SPACE: u32 = 0x00cd_a180;
const SYNC_MODEL_SPACE: u32 = 0x00cd_a340;
const SYNC_LOCAL_SPACE: u32 = 0x00cd_a690;
/// The SSE constants: four times -0.5 (`010c71e0`), four times 1.0 (`010c7170`), the four sign
/// bits (`010c44d0`), the sign bits of the three lowest lanes (`010c4c00`) and the all-ones mask
/// of the three lowest lanes (`010c4c10`); and the floats the bounding box starts from
/// (`0104da48` for the minimum, `0104e148` for the maximum).
const NEG_HALF_LANES: u32 = 0x010c_71e0;
const ONE_LANES: u32 = 0x010c_7170;
const SIGN_BITS: u32 = 0x010c_44d0;
const SIGN_BITS_XYZ: u32 = 0x010c_4c00;
const MASK_XYZ: u32 = 0x010c_4c10;
const BOX_MINIMUM_START: u32 = 0x0104_da48;
const BOX_MAXIMUM_START: u32 = 0x0104_e148;

/// The address of element `index` of an array of 0x30-byte transforms at `base`.
fn element(base: u32, index: i32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(0x30))
}

/// The number of bones: the skeleton's field `+0x1c`.
fn bone_count(e: &Engine, pose: u32) -> i32 {
    e.mem.u32(e.mem.u32(pose) + 0x1c) as i32
}

/// The address of the flag word of `index`.
fn flag_address(e: &Engine, pose: u32, index: i32) -> u32 {
    e.mem
        .u32(pose + 0x1c)
        .wrapping_add((index as u32).wrapping_mul(4))
}

/// The 16-bit parent index of `index`, as the code reads it (`MOVZX`).
fn parent_word(e: &Engine, pose: u32, index: i32) -> u16 {
    let parents = e.mem.u32(e.mem.u32(pose) + 0xc);
    e.mem
        .u16(parents.wrapping_add((index as u32).wrapping_mul(2)))
}

/// `flags[index] &= mask` (the whole word).
fn clear_flags(e: &mut Engine, pose: u32, index: i32, mask: u32) {
    let at = flag_address(e, pose, index);
    let word = e.mem.u32(at);
    e.mem.set_u32(at, word & mask);
}

/// `flags[index] |= bits` (the whole word).
fn set_flags(e: &mut Engine, pose: u32, index: i32, bits: u32) {
    let at = flag_address(e, pose, index);
    let word = e.mem.u32(at);
    e.mem.set_u32(at, word | bits);
}

/// A 3-vector (`MOVAPS`) copy of a whole transform.
fn copy_transform(e: &mut Engine, from: u32, to: u32) {
    for offset in [0, 0x10, 0x20] {
        copy_vec(e, from + offset, to + offset);
    }
}

/// The model transform of `index` from its parent's (a bone whose parent index is not `-1`) and
/// its local one: the parent's translation plus the local translation turned by the parent's
/// rotation, `setMul` of the parent's and the local rotation, the product of the two scales.
fn compose_model(e: &mut Engine, pose: u32, index: i32, parent: i32) {
    let local = element(e.mem.u32(pose + 4), index);
    let model = e.mem.u32(pose + 0x10);
    let parent_block = element(model, parent);
    let moved = rotate_and_move(
        e,
        load_vec(e, parent_block + 0x10),
        load_vec(e, local),
        load_vec(e, parent_block),
    );
    let own = element(model, index);
    store_vec(e, own, moved);
    e.call(
        QUATERNION_SET_MUL,
        &args![own + 0x10, parent_block + 0x10, local + 0x10],
    );
    let scale = mul4(load_vec(e, parent_block + 0x20), load_vec(e, local + 0x20));
    store_vec(e, own + 0x20, scale);
}

fn xor_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

/// The step `syncLocalSpace`, `accessBoneLocalSpace` and `00cdad80` share: the local transform of
/// `index` from its model transform and its parent's (`parent` is the 16-bit word read from the
/// skeleton; `0xffff` is no parent and copies the model transform). With a parent, whose model
/// transform comes from `calculateBoneModelSpace` when its flag has bit 2 and straight from the
/// array otherwise: the translation is the model translation turned by the inverse of the
/// parent's rotation (the rotation with the three lowest sign bits flipped, `010c4c00`) plus the
/// negated double (sign bits `010c44d0`) of a first half the exe computes from the parent's own
/// translation `t` and its RAW rotation `q` (`w` its lane 3) in all four lanes:
/// `(t x q) * w + ((w * w - 0.5) * t + dot(t, q) * q)`; the rotation is `setMul` of that inverse
/// and the model rotation; the scale is the model scale times the reciprocal of the parent's
/// scale (`RCPPS` and one Newton step, lanes 0 to 2, lane 3 zero). Ends by clearing bit 1 of the
/// flag word.
fn update_local(e: &mut Engine, pose: u32, index: i32, parent: u16) {
    let model = e.mem.u32(pose + 0x10);
    if parent == 0xffff {
        let local = element(e.mem.u32(pose + 4), index);
        copy_transform(e, element(model, index), local);
    } else {
        let parent = i32::from(parent as i16);
        let parent_block = if e.mem.u8(flag_address(e, pose, parent)) & 2 == 0 {
            element(model, parent)
        } else {
            e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, parent])
                .u32()
        };
        let parent_translation = load_vec(e, parent_block);
        let raw_rotation = load_vec(e, parent_block + 0x10);
        // The first half (00cda70c-00cda79c here; 00cda977-00cdaa0b and 00cdabaf-00cdac3f in
        // accessBoneLocalSpace, 00cdae77-00cdaf0b in 00cdad80, and the same code inlined in
        // 00c7aa60 at 00c7ab81-00c7ac15) works on the parent's RAW rotation `q`
        // (`MOVAPS XMM0,[EAX+0x10]`) in all four lanes, and the cross term is `t x q`
        // (`t.c9 * q.d2 - t.d2 * q.c9`, 00cda72f-00cda751). That turns the parent's
        // translation by the INVERSE rotation; turning it by `q` moved a child bone by about
        // twice its parent's model position (Doc's Bip01 Head ~150 units off Neck1 under look
        // IK). It is not `rotate_half` of the inverse bit for bit: the dot (`z + (y + x)` of
        // `t * q`, 00cda716-00cda72b) and the along term (`splat(dot) * q`, 00cda778 /
        // 00cda77c) take the raw `q`, so lane 3 is `+dot * q.w`, and a dot that sums to +0
        // gives zeros whose sign follows `q`, not the inverse. `q` only becomes the inverse at
        // 00cda79f / 00cda7a3.
        let w = [raw_rotation[3]; 4];
        let cross = sub4(
            mul4(
                shufps(parent_translation, parent_translation, 0xc9),
                shufps(raw_rotation, raw_rotation, 0xd2),
            ),
            mul4(
                shufps(parent_translation, parent_translation, 0xd2),
                shufps(raw_rotation, raw_rotation, 0xc9),
            ),
        );
        let dot = dot3(parent_translation, raw_rotation);
        let half = add4(
            mul4(cross, w),
            add4(
                mul4(
                    add4(mul4(w, w), load_vec(e, NEG_HALF_LANES)),
                    parent_translation,
                ),
                mul4([dot; 4], raw_rotation),
            ),
        );
        let negated = xor_bits(add4(half, half), load_bits(e, SIGN_BITS));
        let inverse_rotation = xor_bits(raw_rotation, load_bits(e, SIGN_BITS_XYZ));
        let scale = load_vec(e, parent_block + 0x20);
        let ones = load_vec(e, ONE_LANES);
        let lanes = shufps(scale, unpckhps(scale, ones), 0xc4);
        let mask = load_bits(e, MASK_XYZ);
        let inverse_scale: Vec4 = std::array::from_fn(|lane| {
            f32::from_bits(refined_reciprocal(lanes[lane]).to_bits() & mask[lane])
        });
        let own = element(model, index);
        let local = element(e.mem.u32(pose + 4), index);
        let half = rotate_half(e, inverse_rotation, load_vec(e, own));
        e.with_stack(0x10, |e, spill| {
            store_vec(e, spill.addr(), inverse_rotation);
            store_vec(e, local, add4(add4(half, half), negated));
            e.call(
                QUATERNION_SET_MUL,
                &args![local + 0x10, spill.addr(), own + 0x10],
            );
        });
        let scaled = mul4(load_vec(e, own + 0x20), inverse_scale);
        store_vec(e, local + 0x20, scaled);
    }
    clear_flags(e, pose, index, 0xffff_fffe);
}

/// The common start of `accessBoneLocalSpace` and `00cdad80`: marks `bone` with bit 4, then for
/// every later bone whose parent index is not `-1` and whose parent carries bit 4, brings its
/// local transform up to date when its bit 1 is set (`update_local`), marks it with bit 4 and
/// clears the model-space byte; finally every later bone with bit 4 swaps it for bit 2.
fn mark_later_bones(e: &mut Engine, pose: u32, bone: i32) {
    let count = bone_count(e, pose);
    set_flags(e, pose, bone, 4);
    let mut index = bone.wrapping_add(1);
    while index < count {
        let parent = parent_word(e, pose, index);
        if parent != 0xffff && e.mem.u8(flag_address(e, pose, i32::from(parent as i16))) & 4 != 0 {
            if e.mem.u8(flag_address(e, pose, index)) & 1 != 0 {
                update_local(e, pose, index, parent);
            }
            set_flags(e, pose, index, 4);
            e.mem.set_u8(pose + 0x28, 0);
        }
        index += 1;
    }
    let mut index = bone.wrapping_add(1);
    while index < count {
        let at = flag_address(e, pose, index);
        let word = e.mem.u32(at);
        if word & 4 != 0 {
            e.mem.set_u32(at, (word & 0xffff_fffb) | 2);
        }
        index += 1;
    }
}

// Translated from 00cd9f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose, `(source)`: copies `bone count * 0x30` bytes from the pointer
/// stored at `source` over the local transforms (`hkString::memCpy`), marks every bone's model
/// transform stale (flag word 2), clears the model-space byte and sets the local-space one.
pub fn fn_00cd9f10(e: &mut Engine, this: Ptr, source: Ptr) {
    let pose = this.addr();
    let count = bone_count(e, pose);
    let from = e.mem.u32(source.addr());
    let local = e.mem.u32(pose + 4);
    e.call(
        MEM_COPY,
        &args![local, from, (count as u32).wrapping_mul(0x30)],
    );
    let mut index = 0;
    while index < count {
        let at = flag_address(e, pose, index);
        e.mem.set_u32(at, 2);
        index += 1;
    }
    e.mem.set_u8(pose + 0x29, 1);
    e.mem.set_u8(pose + 0x28, 0);
}

// Translated from 00cd9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose, `(source)`: the same for the model transforms: copies over them,
/// marks every bone's local transform stale (flag word 1), clears the local-space byte and sets
/// the model-space one.
pub fn fn_00cd9f60(e: &mut Engine, this: Ptr, source: Ptr) {
    let pose = this.addr();
    let count = bone_count(e, pose);
    let from = e.mem.u32(source.addr());
    let model = e.mem.u32(pose + 0x10);
    e.call(
        MEM_COPY,
        &args![model, from, (count as u32).wrapping_mul(0x30)],
    );
    let mut index = 0;
    while index < count {
        let at = flag_address(e, pose, index);
        e.mem.set_u32(at, 1);
        index += 1;
    }
    e.mem.set_u8(pose + 0x29, 0);
    e.mem.set_u8(pose + 0x28, 1);
}

// Translated from 00cd9fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose: marks every bone's model transform stale (flag word 2), clears the
/// model-space byte, sets the local-space one and answers `pose + 4` (the address of the local
/// transform array).
pub fn fn_00cd9fb0(e: &mut Engine, this: Ptr) -> u32 {
    let pose = this.addr();
    let count = bone_count(e, pose);
    let mut index = 0;
    while index < count {
        let at = flag_address(e, pose, index);
        e.mem.set_u32(at, 2);
        index += 1;
    }
    e.mem.set_u8(pose + 0x28, 0);
    e.mem.set_u8(pose + 0x29, 1);
    pose + 4
}

// Translated from 00cda110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::setToReferencePose` (Xbox PDB), `ECX` = the pose: copies the skeleton's reference
/// pose (`+0x24`) over the local transforms, marks every model transform stale (flag word 2),
/// clears the model-space byte, sets the local-space one and zeroes the float slots.
pub fn hka_pose_set_to_reference_pose(e: &mut Engine, this: Ptr) {
    let pose = this.addr();
    let skeleton = e.mem.u32(pose);
    let count = e.mem.u32(skeleton + 0x1c);
    let reference = e.mem.u32(skeleton + 0x24);
    let local = e.mem.u32(pose + 4);
    e.call(MEM_COPY, &args![local, reference, count.wrapping_mul(0x30)]);
    let mut index = 0;
    while index < bone_count(e, pose) {
        let at = flag_address(e, pose, index);
        e.mem.set_u32(at, 2);
        index += 1;
    }
    e.mem.set_u8(pose + 0x28, 0);
    e.mem.set_u8(pose + 0x29, 1);
    let slots = e.mem.u32(skeleton + 0x34) as i32;
    if slots > 0 {
        let floats = e.mem.u32(pose + 0x2c);
        let mut index = 0;
        while index < e.mem.u32(skeleton + 0x34) as i32 {
            e.mem.set_f32(floats.wrapping_add(4 * index as u32), 0.0);
            index += 1;
        }
    }
}

// Translated from 00cda180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::calculateBoneModelSpace` (Xbox PDB), `ECX` = the pose, `(bone)`: brings the model
/// transform of `bone` up to date and answers its address. Walks up the parents while the bone's
/// flag word has bit 2, marking each walked bone with 8; a root (parent index `-1`) copies its
/// local transform and clears its bit 2. Then, from the bone where the walk stopped up to
/// `bone`, every bone marked 8 is composed from its parent's model transform and its own local
/// one (`setMul` for the rotation) and loses bits 2 and 8.
pub fn hka_pose_calculate_bone_model_space(e: &mut Engine, this: Ptr, bone: i32) -> u32 {
    let pose = this.addr();
    let mut index = bone;
    while e.mem.u8(flag_address(e, pose, index)) & 2 != 0 {
        let parent = parent_word(e, pose, index);
        if parent == 0xffff {
            let local = element(e.mem.u32(pose + 4), index);
            let model = element(e.mem.u32(pose + 0x10), index);
            copy_transform(e, local, model);
            clear_flags(e, pose, index, 0xffff_fffd);
            break;
        }
        set_flags(e, pose, index, 8);
        index = i32::from(parent as i16);
    }
    index = index.wrapping_add(1);
    while index <= bone {
        if e.mem.u8(flag_address(e, pose, index)) & 8 != 0 {
            let parent = i32::from(parent_word(e, pose, index) as i16);
            compose_model(e, pose, index, parent);
            clear_flags(e, pose, index, 0xffff_fff5);
        }
        index += 1;
    }
    element(e.mem.u32(pose + 0x10), bone)
}

// Translated from 00cda340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::syncModelSpace` (Xbox PDB), `ECX` = the pose: unless the model-space byte is set,
/// brings the model transform of every bone whose flag word has bit 2 up to date (a root copies
/// its local transform, any other is composed from its parent's model transform, which comes
/// first in the array), clears the bit, and sets the byte.
pub fn hka_pose_sync_model_space(e: &mut Engine, this: Ptr) {
    let pose = this.addr();
    if e.mem.u8(pose + 0x28) != 0 {
        return;
    }
    let count = bone_count(e, pose);
    let mut index = 0;
    while index < count {
        if e.mem.u8(flag_address(e, pose, index)) & 2 != 0 {
            let parent = parent_word(e, pose, index);
            if parent == 0xffff {
                let local = element(e.mem.u32(pose + 4), index);
                let model = element(e.mem.u32(pose + 0x10), index);
                copy_transform(e, local, model);
            } else {
                compose_model(e, pose, index, i32::from(parent as i16));
            }
            clear_flags(e, pose, index, 0xffff_fffd);
        }
        index += 1;
    }
    e.mem.set_u8(pose + 0x28, 1);
}

// Translated from 00cda4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::accessSyncedPoseModelSpace` (Xbox PDB), `ECX` = the pose: `syncModelSpace`, then
/// `00cd9fe0` (which answers the pointer this returns).
pub fn hka_pose_access_synced_pose_model_space(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SYNC_MODEL_SPACE, &args![this]);
    e.call(POSE_MODEL_SPACE_CHANGED, &args![this]).u32()
}

// Translated from 00cda4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose: `syncModelSpace`, then for every bone: when the bone's record in
/// the skeleton (`+0x18`, byte at `+4`) is set, its model translation becomes the skeleton's
/// reference translation (`+0x24`) of the bone, turned by the parent's model rotation and moved
/// by the parent's model translation (a root takes it as it is), and the flag word gets bits 1
/// and 4; when it is not set and the parent index is not `-1` and the parent's word has bit 4,
/// the bone's word gets bit 1. Finally the first `+0x20`-count flag words lose bits 4, 8 and
/// 0x10.
pub fn fn_00cda4c0(e: &mut Engine, this: Ptr) {
    let pose = this.addr();
    let count = bone_count(e, pose);
    e.call(SYNC_MODEL_SPACE, &args![this]);
    let mut index = 0;
    while index < count {
        let skeleton = e.mem.u32(pose);
        let records = e.mem.u32(skeleton + 0x18);
        let marked = e
            .mem
            .u8(records.wrapping_add((index as u32).wrapping_mul(8)) + 4)
            != 0;
        let parent = parent_word(e, pose, index);
        if marked {
            let reference = element(e.mem.u32(skeleton + 0x24), index);
            let model = e.mem.u32(pose + 0x10);
            let own = element(model, index);
            if parent == 0xffff {
                copy_vec(e, reference, own);
            } else {
                let parent_block = element(model, i32::from(parent as i16));
                let half = rotate_half(e, load_vec(e, parent_block + 0x10), load_vec(e, reference));
                store_vec(e, own, add4(half, half));
                let moved = add4(load_vec(e, parent_block), load_vec(e, own));
                store_vec(e, own, moved);
            }
            set_flags(e, pose, index, 1);
            set_flags(e, pose, index, 4);
        } else if parent != 0xffff
            && e.mem.u8(flag_address(e, pose, i32::from(parent as i16))) & 4 != 0
        {
            set_flags(e, pose, index, 1);
        }
        index += 1;
    }
    let flag_count = e.mem.u32(pose + 0x20) as i32;
    let mut index = 0;
    while index < flag_count {
        clear_flags(e, pose, index, 0xffff_ffe3);
        index += 1;
    }
}

// Translated from 00cda610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose, `(out)`: the bounding box of the model translations. Starts the
/// two vectors at `out` (the minimum) and `out + 0x10` (the maximum) from the floats at
/// `0104da48` and `0104e148` (each in all four lanes), runs `syncModelSpace` and folds the
/// translation of every bone in with `MINPS` and `MAXPS`.
pub fn fn_00cda610(e: &mut Engine, this: Ptr, out: Ptr) {
    let pose = this.addr();
    let out = out.addr();
    let minimum = e.mem.f32(BOX_MINIMUM_START);
    store_vec(e, out, [minimum; 4]);
    let maximum = e.mem.f32(BOX_MAXIMUM_START);
    store_vec(e, out + 0x10, [maximum; 4]);
    let count = bone_count(e, pose);
    e.call(SYNC_MODEL_SPACE, &args![this]);
    let mut index = 0;
    while index < count {
        let translation = load_vec(e, element(e.mem.u32(pose + 0x10), index));
        let lowest = load_vec(e, out);
        let folded: Vec4 = std::array::from_fn(|lane| minps(lowest[lane], translation[lane]));
        store_vec(e, out, folded);
        let highest = load_vec(e, out + 0x10);
        let translation = load_vec(e, element(e.mem.u32(pose + 0x10), index));
        let folded: Vec4 = std::array::from_fn(|lane| maxps(highest[lane], translation[lane]));
        store_vec(e, out + 0x10, folded);
        index += 1;
    }
}

// Translated from 00cda690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::syncLocalSpace` (Xbox PDB), `ECX` = the pose: unless the local-space byte is set,
/// brings the local transform of every bone whose flag word has bit 1 up to date from the model
/// transforms (`update_local`) and sets the byte.
pub fn hka_pose_sync_local_space(e: &mut Engine, this: Ptr) {
    let pose = this.addr();
    if e.mem.u8(pose + 0x29) != 0 {
        return;
    }
    let count = bone_count(e, pose);
    let mut index = 0;
    while index < count {
        if e.mem.u8(flag_address(e, pose, index)) & 1 != 0 {
            let parent = parent_word(e, pose, index);
            update_local(e, pose, index, parent);
        }
        index += 1;
    }
    e.mem.set_u8(pose + 0x29, 1);
}

// Translated from 00cda8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::getSyncedPoseModelSpace` (Xbox PDB), `ECX` = the pose: `syncModelSpace`, then
/// answers `pose + 0x10` (the address of the model transform array).
pub fn hka_pose_get_synced_pose_model_space(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SYNC_MODEL_SPACE, &args![this]);
    this.addr() + 0x10
}

// Translated from 00cda8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::accessBoneLocalSpace` (Xbox PDB), `ECX` = the pose, `(bone)`: makes the local
/// transform of `bone` the truth and answers its address. Marks the later bones that depend on
/// it (`mark_later_bones`), brings its own local transform up to date when its flag word has bit
/// 1, sets its flag word to 2 (the model transform is stale) and clears the model-space byte.
pub fn hka_pose_access_bone_local_space(e: &mut Engine, this: Ptr, bone: i32) -> u32 {
    let pose = this.addr();
    mark_later_bones(e, pose, bone);
    if e.mem.u8(flag_address(e, pose, bone)) & 1 != 0 {
        let parent = parent_word(e, pose, bone);
        update_local(e, pose, bone, parent);
    }
    let at = flag_address(e, pose, bone);
    e.mem.set_u32(at, 2);
    e.mem.set_u8(pose + 0x28, 0);
    element(e.mem.u32(pose + 4), bone)
}

// Translated from 00cdad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose, `(bone, keep_children)`: makes the model transform of `bone` the
/// truth and answers its address. With `keep_children` zero, every later bone whose parent is
/// `bone` gets its model transform brought up to date (when its word has bit 2) and its word set
/// to 1 (the local transform is stale), clearing the local-space byte. Otherwise the later bones
/// are walked as in `accessBoneLocalSpace` (`mark_later_bones`). Then `bone` itself is brought
/// up to date when its word has bit 2, its word is set to 1 and the local-space byte cleared.
pub fn fn_00cdad80(e: &mut Engine, this: Ptr, bone: i32, keep_children: u32) -> u32 {
    let pose = this.addr();
    if keep_children == 0 {
        let count = bone_count(e, pose);
        let mut index = bone.wrapping_add(1);
        while index < count {
            if i32::from(parent_word(e, pose, index) as i16) == bone {
                if e.mem.u8(flag_address(e, pose, index)) & 2 != 0 {
                    e.call(CALCULATE_BONE_MODEL_SPACE, &args![this, index]);
                }
                let at = flag_address(e, pose, index);
                e.mem.set_u32(at, 1);
                e.mem.set_u8(pose + 0x29, 0);
            }
            index += 1;
        }
    } else {
        mark_later_bones(e, pose, bone);
    }
    if e.mem.u8(flag_address(e, pose, bone)) & 2 != 0 {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![this, bone]);
    }
    let at = flag_address(e, pose, bone);
    e.mem.set_u32(at, 1);
    e.mem.set_u8(pose + 0x29, 0);
    element(e.mem.u32(pose + 0x10), bone)
}

// Translated from 00cdb0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose: `syncLocalSpace`, then `00cd9fb0` (which answers `pose + 4`).
pub fn fn_00cdb0a0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SYNC_LOCAL_SPACE, &args![this]);
    e.call(GET_LOCAL_TRANSFORMS, &args![this]).u32()
}

/// Makes room for `wanted` entries of `element` bytes in the `hkArray` at `array` (data, size,
/// capacity word): the router is asked for its allocator, and when the capacity is smaller the
/// larger of `wanted` and twice the capacity is reserved through `_reserve` (signed compares).
pub(crate) fn grow_array(e: &mut Engine, array: u32, wanted: i32, element: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let reserve = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![allocator, array, reserve, element]);
    }
}

// Translated from 00cdb150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the pose, `(mode, _unused_2, source)`: sizes the pose for its skeleton. Grows
/// the model transforms, the local transforms (0x30 bytes each) and the flag words (4 bytes
/// each) to the bone count, setting their sizes to it; when `mode` is 1 runs `00cd9f10`
/// (`source`) else `00cd9f60` (`source`); then grows the float slot array to the slot count
/// rounded up to a multiple of 4 and again to the slot count itself, zero-fills the new slots
/// and sets its size to the slot count.
pub fn fn_00cdb150(e: &mut Engine, this: Ptr, mode: u32, _unused_2: u32, source: Ptr) {
    let pose = this.addr();
    let count = bone_count(e, pose);
    grow_array(e, pose + 0x10, count, 0x30);
    e.mem.set_u32(pose + 0x14, count as u32);
    grow_array(e, pose + 4, count, 0x30);
    e.mem.set_u32(pose + 8, count as u32);
    grow_array(e, pose + 0x1c, count, 4);
    e.mem.set_u32(pose + 0x20, count as u32);
    if mode == 1 {
        e.call(COPY_LOCAL_TRANSFORMS, &args![this, source]);
    } else {
        e.call(COPY_MODEL_TRANSFORMS, &args![this, source]);
    }
    let slots = pose + 0x2c;
    let rounded = (e.mem.u32(e.mem.u32(pose) + 0x34) as i32).wrapping_add(3) & !3;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if ((e.mem.u32(slots + 8) & CAPACITY_MASK) as i32) < rounded {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE, &args![allocator, slots, rounded, 4u32]);
    }
    let wanted = e.mem.u32(e.mem.u32(pose) + 0x34) as i32;
    grow_array(e, slots, wanted, 4);
    let size = e.mem.u32(slots + 4) as i32;
    let data = e.mem.u32(slots);
    let mut at = data.wrapping_add((size as u32).wrapping_mul(4));
    if wanted.wrapping_sub(size) > 0 {
        for _ in 0..wanted.wrapping_sub(size) {
            e.mem.set_u32(at, 0);
            at = at.wrapping_add(4);
        }
    }
    e.mem.set_u32(slots + 4, wanted as u32);
}

// Translated from 00cdb290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaPose::getSyncedPoseLocalSpace` (Xbox PDB), `ECX` = the pose: `syncLocalSpace`, then
/// answers `pose + 4` (the address of the local transform array).
pub fn hka_pose_get_synced_pose_local_space(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SYNC_LOCAL_SPACE, &args![this]);
    this.addr() + 4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_9f10, fn_00cd9f10(Ptr, Ptr)),
        entry!(0x00cd_9f60, fn_00cd9f60(Ptr, Ptr)),
        entry!(0x00cd_9fb0, fn_00cd9fb0(Ptr) -> u32),
        entry!(0x00cd_a110, hka_pose_set_to_reference_pose(Ptr)),
        entry!(0x00cd_a180, hka_pose_calculate_bone_model_space(Ptr, i32) -> u32),
        entry!(0x00cd_a340, hka_pose_sync_model_space(Ptr)),
        entry!(0x00cd_a4b0, hka_pose_access_synced_pose_model_space(Ptr) -> u32),
        entry!(0x00cd_a4c0, fn_00cda4c0(Ptr)),
        entry!(0x00cd_a610, fn_00cda610(Ptr, Ptr)),
        entry!(0x00cd_a690, hka_pose_sync_local_space(Ptr)),
        entry!(0x00cd_a8d0, hka_pose_get_synced_pose_model_space(Ptr) -> u32),
        entry!(0x00cd_a8e0, hka_pose_access_bone_local_space(Ptr, i32) -> u32),
        entry!(0x00cd_ad80, fn_00cdad80(Ptr, i32, u32) -> u32),
        entry!(0x00cd_b0a0, fn_00cdb0a0(Ptr) -> u32),
        entry!(0x00cd_b150, fn_00cdb150(Ptr, u32, u32, Ptr)),
        entry!(0x00cd_b290, hka_pose_get_synced_pose_local_space(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::store_bits;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOCATOR: u32 = 0x00a1_10c0;
    const CHANGED_ANSWER: u32 = 0x5555_0000;
    const NO_PARENT: u16 = 0xffff;

    fn entry(address: u32, words: &[u32]) -> (u32, Vec<u32>) {
        (address, words.to_vec())
    }

    /// Replaces the storage of an `hkArray` by a zero-filled block of `capacity` elements that
    /// holds the old elements.
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

    /// An engine with the SSE constants in memory (-0.5, 1.0, the sign bits of all lanes and of
    /// the three lowest, the mask of the three lowest, the two bounding box starts) and doubles
    /// for the memory router, `_reserve`, `memCpy`, `setMul` (which answers `a + b`) and
    /// `00cd9fe0`. The log records the calls of the last three and of `_reserve`.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        store_vec(&mut e, NEG_HALF_LANES, [-0.5; 4]);
        store_vec(&mut e, ONE_LANES, [1.0; 4]);
        store_bits(&mut e, SIGN_BITS, [0x8000_0000; 4]);
        store_bits(
            &mut e,
            SIGN_BITS_XYZ,
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0],
        );
        store_bits(&mut e, MASK_XYZ, [u32::MAX, u32::MAX, u32::MAX, 0]);
        e.mem.set_f32(BOX_MINIMUM_START, 1.0e30);
        e.mem.set_f32(BOX_MAXIMUM_START, -1.0e30);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, ALLOCATOR);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log: Log = Rc::default();
        let l = Rc::clone(&log);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            l.borrow_mut().push((QUATERNION_SET_MUL, a.to_vec()));
            let (p, q) = (load_vec(e, a[1]), load_vec(e, a[2]));
            store_vec(e, a[0], add4(p, q));
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(MEM_COPY, move |e, a| {
            l.borrow_mut().push((MEM_COPY, a.to_vec()));
            let data = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &data);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            regrow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        let l = Rc::clone(&log);
        e.register_double(POSE_MODEL_SPACE_CHANGED, move |_, a| {
            l.borrow_mut().push((POSE_MODEL_SPACE_CHANGED, a.to_vec()));
            Ret {
                eax: CHANGED_ANSWER,
                ..Ret::default()
            }
        });
        (e, log)
    }

    struct Rig {
        pose: u32,
        skeleton: u32,
        local: u32,
        model: u32,
        flags: u32,
        reference: u32,
        floats: u32,
    }

    /// A pose over a skeleton of `parents.len()` bones (`marked` is the byte at `+4` of every
    /// bone record) with `slots` float slots, every array exactly big enough, the bytes at
    /// `+0x28` and `+0x29` set to `synced`.
    fn make_rig(
        e: &mut Engine,
        parents: &[u16],
        marked: &[u8],
        slots: u32,
        synced: [u8; 2],
    ) -> Rig {
        let count = parents.len() as u32;
        let skeleton = e.mem.alloc(0x40);
        let pose = e.mem.alloc(0x40);
        let parent_array = e.mem.alloc(2 * count);
        let records = e.mem.alloc(8 * count);
        for (k, parent) in parents.iter().enumerate() {
            e.mem.set_u16(parent_array + 2 * k as u32, *parent);
            e.mem.set_u8(records + 8 * k as u32 + 4, marked[k]);
        }
        let reference = e.mem.alloc(0x30 * count);
        e.mem.set_u32(skeleton + 0xc, parent_array);
        e.mem.set_u32(skeleton + 0x18, records);
        e.mem.set_u32(skeleton + 0x1c, count);
        e.mem.set_u32(skeleton + 0x24, reference);
        e.mem.set_u32(skeleton + 0x34, slots);
        let local = e.mem.alloc(0x30 * count);
        let model = e.mem.alloc(0x30 * count);
        let flags = e.mem.alloc(4 * count);
        let floats = e.mem.alloc(4 * slots + 16);
        e.mem.set_u32(pose, skeleton);
        for (array, data) in [(0x4, local), (0x10, model), (0x1c, flags)] {
            e.mem.set_u32(pose + array, data);
            e.mem.set_u32(pose + array + 4, count);
            e.mem.set_u32(pose + array + 8, count);
        }
        e.mem.set_u32(pose + 0x2c, floats);
        e.mem.set_u32(pose + 0x30, slots);
        e.mem.set_u32(pose + 0x34, slots);
        e.mem.set_u8(pose + 0x28, synced[0]);
        e.mem.set_u8(pose + 0x29, synced[1]);
        Rig {
            pose,
            skeleton,
            local,
            model,
            flags,
            reference,
            floats,
        }
    }

    fn set_flags_of(e: &mut Engine, rig: &Rig, flags: &[u32]) {
        for (k, word) in flags.iter().enumerate() {
            e.mem.set_u32(rig.flags + 4 * k as u32, *word);
        }
    }

    fn flags_of(e: &Engine, rig: &Rig, count: u32) -> Vec<u32> {
        (0..count).map(|k| e.mem.u32(rig.flags + 4 * k)).collect()
    }

    fn put_transform(e: &mut Engine, base: u32, translation: Vec4, rotation: Vec4, scale: Vec4) {
        store_vec(e, base, translation);
        store_vec(e, base + 0x10, rotation);
        store_vec(e, base + 0x20, scale);
    }

    fn get_transform(e: &Engine, base: u32) -> [Vec4; 3] {
        [
            load_vec(e, base),
            load_vec(e, base + 0x10),
            load_vec(e, base + 0x20),
        ]
    }

    const IDENTITY: Vec4 = [0.0, 0.0, 0.0, 1.0];
    const NO_ROTATION: Vec4 = [0.0; 4];

    /// Three bones in a chain, root first, with the local transforms the model tests use. The
    /// rotations are set so that the double of `setMul` (`a + b`) makes every model rotation the
    /// identity.
    fn chain(e: &mut Engine, flags: &[u32; 3], synced: [u8; 2]) -> Rig {
        let rig = make_rig(e, &[NO_PARENT, 0, 1], &[0; 3], 0, synced);
        let locals = [
            ([1.0, 2.0, 3.0, 0.0], IDENTITY, [2.0, 2.0, 2.0, 1.0]),
            ([10.0, 20.0, 30.0, 0.0], NO_ROTATION, [3.0, 3.0, 3.0, 1.0]),
            (
                [100.0, 200.0, 300.0, 0.0],
                NO_ROTATION,
                [4.0, 4.0, 4.0, 1.0],
            ),
        ];
        for (k, (t, q, s)) in locals.into_iter().enumerate() {
            put_transform(e, rig.local + 0x30 * k as u32, t, q, s);
            put_transform(
                e,
                rig.model + 0x30 * k as u32,
                [-1.0; 4],
                [-2.0; 4],
                [-3.0; 4],
            );
        }
        set_flags_of(e, &rig, flags);
        rig
    }

    /// What the chain's model transforms are when all of them are brought up to date.
    fn chain_model() -> [[Vec4; 3]; 3] {
        [
            [[1.0, 2.0, 3.0, 0.0], IDENTITY, [2.0, 2.0, 2.0, 1.0]],
            [[11.0, 22.0, 33.0, 0.0], IDENTITY, [6.0, 6.0, 6.0, 1.0]],
            [
                [111.0, 222.0, 333.0, 0.0],
                IDENTITY,
                [24.0, 24.0, 24.0, 1.0],
            ],
        ]
    }

    fn untouched() -> [Vec4; 3] {
        [[-1.0; 4], [-2.0; 4], [-3.0; 4]]
    }

    fn set_mul(this: u32, a: u32, b: u32) -> (u32, Vec<u32>) {
        entry(QUATERNION_SET_MUL, &[this, a, b])
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= 4.0e-7 * b.abs()
    }

    #[test]
    fn copying_transforms_in_marks_the_other_space_stale() {
        let (mut e, log) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0, 1], &[0; 3], 0, [1, 0]);
        let data = e.mem.alloc(0x90);
        for k in 0..0x24u32 {
            e.mem.set_u32(data + 4 * k, 0x100 + k);
        }
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, data);
        set_flags_of(&mut e, &rig, &[7, 7, 7]);
        e.call(0x00cd_9f10, &args![rig.pose, source]);
        assert_eq!(e.mem.bytes(rig.local, 0x90), e.mem.bytes(data, 0x90));
        assert_eq!(flags_of(&e, &rig, 3), [2, 2, 2]);
        assert_eq!(
            (e.mem.u8(rig.pose + 0x28), e.mem.u8(rig.pose + 0x29)),
            (0, 1)
        );
        e.call(0x00cd_9f60, &args![rig.pose, source]);
        assert_eq!(e.mem.bytes(rig.model, 0x90), e.mem.bytes(data, 0x90));
        assert_eq!(flags_of(&e, &rig, 3), [1, 1, 1]);
        assert_eq!(
            (e.mem.u8(rig.pose + 0x28), e.mem.u8(rig.pose + 0x29)),
            (1, 0)
        );
        assert_eq!(
            log.borrow()[..],
            [
                entry(MEM_COPY, &[rig.local, data, 0x90]),
                entry(MEM_COPY, &[rig.model, data, 0x90]),
            ]
        );
    }

    #[test]
    fn the_local_transform_getter_marks_every_model_transform_stale() {
        let (mut e, _) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0], &[0; 2], 0, [1, 0]);
        set_flags_of(&mut e, &rig, &[1, 0]);
        let answer = e.call(0x00cd_9fb0, &args![rig.pose]).u32();
        assert_eq!(answer, rig.pose + 4);
        assert_eq!(flags_of(&e, &rig, 2), [2, 2]);
        assert_eq!(
            (e.mem.u8(rig.pose + 0x28), e.mem.u8(rig.pose + 0x29)),
            (0, 1)
        );
    }

    #[test]
    fn the_reference_pose_replaces_the_local_transforms_and_zeroes_the_slots() {
        let (mut e, log) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0], &[0; 2], 3, [1, 0]);
        for k in 0..0x18u32 {
            e.mem.set_u32(rig.reference + 4 * k, 0x200 + k);
        }
        for k in 0..4u32 {
            e.mem.set_f32(rig.floats + 4 * k, 7.0);
        }
        set_flags_of(&mut e, &rig, &[5, 6]);
        e.call(0x00cd_a110, &args![rig.pose]);
        assert_eq!(
            e.mem.bytes(rig.local, 0x60),
            e.mem.bytes(rig.reference, 0x60)
        );
        assert_eq!(flags_of(&e, &rig, 2), [2, 2]);
        assert_eq!(
            (e.mem.u8(rig.pose + 0x28), e.mem.u8(rig.pose + 0x29)),
            (0, 1)
        );
        let slots: Vec<f32> = (0..4).map(|k| e.mem.f32(rig.floats + 4 * k)).collect();
        assert_eq!(slots, [0.0, 0.0, 0.0, 7.0]);
        assert_eq!(
            log.borrow()[..],
            [entry(MEM_COPY, &[rig.local, rig.reference, 0x60])]
        );
        // No slots (or a negative count): they stay.
        for count in [0, u32::MAX] {
            e.mem.set_u32(rig.skeleton + 0x34, count);
            e.mem.set_f32(rig.floats, 7.0);
            e.call(0x00cd_a110, &args![rig.pose]);
            assert_eq!(e.mem.f32(rig.floats), 7.0);
        }
    }

    #[test]
    fn syncing_the_model_space_composes_the_stale_bones_once() {
        let (mut e, log) = engine();
        let rig = chain(&mut e, &[3, 2, 6], [0, 1]);
        e.call(0x00cd_a340, &args![rig.pose]);
        let expected = chain_model();
        for (k, want) in expected.iter().enumerate() {
            assert_eq!(get_transform(&e, rig.model + 0x30 * k as u32), *want);
        }
        assert_eq!(flags_of(&e, &rig, 3), [1, 0, 4]);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 1);
        let (m, l) = (rig.model, rig.local);
        assert_eq!(
            log.borrow()[..],
            [
                set_mul(m + 0x40, m + 0x10, l + 0x40),
                set_mul(m + 0x70, m + 0x40, l + 0x70),
            ]
        );
        // Already synced: nothing happens, whatever the flags say.
        set_flags_of(&mut e, &rig, &[2, 2, 2]);
        store_vec(&mut e, rig.local, [9.0; 4]);
        e.call(0x00cd_a340, &args![rig.pose]);
        assert_eq!(flags_of(&e, &rig, 3), [2, 2, 2]);
        assert_eq!(load_vec(&e, rig.model), [1.0, 2.0, 3.0, 0.0]);
        // A bone without bit 2 is skipped.
        e.mem.set_u8(rig.pose + 0x28, 0);
        set_flags_of(&mut e, &rig, &[0, 0, 0]);
        store_vec(&mut e, rig.model + 0x60, [5.0; 4]);
        e.call(0x00cd_a340, &args![rig.pose]);
        assert_eq!(load_vec(&e, rig.model + 0x60), [5.0; 4]);
    }

    #[test]
    fn a_bone_model_transform_walks_up_to_the_first_bone_that_is_current() {
        let (mut e, log) = engine();
        let rig = chain(&mut e, &[2, 2, 2], [0, 1]);
        let answer = e.call(0x00cd_a180, &args![rig.pose, 2u32]).u32();
        assert_eq!(answer, rig.model + 0x60);
        for (k, want) in chain_model().iter().enumerate() {
            assert_eq!(get_transform(&e, rig.model + 0x30 * k as u32), *want);
        }
        assert_eq!(flags_of(&e, &rig, 3), [0, 0, 0]);
        // Bone 1 current: only bone 2 is composed, from the stored model transform of bone 1.
        let (mut e, log2) = engine();
        let rig = chain(&mut e, &[2, 0, 2], [0, 1]);
        put_transform(
            &mut e,
            rig.model + 0x30,
            [11.0, 22.0, 33.0, 0.0],
            IDENTITY,
            [6.0; 4],
        );
        e.call(0x00cd_a180, &args![rig.pose, 2u32]);
        assert_eq!(
            get_transform(&e, rig.model + 0x60),
            [
                [111.0, 222.0, 333.0, 0.0],
                IDENTITY,
                [24.0, 24.0, 24.0, 6.0]
            ]
        );
        assert_eq!(get_transform(&e, rig.model), untouched());
        assert_eq!(flags_of(&e, &rig, 3), [2, 0, 0]);
        let (m, l) = (rig.model, rig.local);
        assert_eq!(log2.borrow()[..], [set_mul(m + 0x70, m + 0x40, l + 0x70)]);
        assert_eq!(log.borrow().len(), 2);
        // A bone that is current is answered as it is.
        let answer = e.call(0x00cd_a180, &args![rig.pose, 1u32]).u32();
        assert_eq!(answer, rig.model + 0x30);
        assert_eq!(log2.borrow().len(), 1);
    }

    #[test]
    fn the_model_space_accessors_sync_first() {
        let (mut e, log) = engine();
        let rig = chain(&mut e, &[2, 2, 2], [0, 1]);
        let answer = e.call(0x00cd_a4b0, &args![rig.pose]).u32();
        assert_eq!(answer, CHANGED_ANSWER);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 1);
        assert_eq!(flags_of(&e, &rig, 3), [0, 0, 0]);
        assert_eq!(
            log.borrow().last(),
            Some(&entry(POSE_MODEL_SPACE_CHANGED, &[rig.pose]))
        );
        let (mut e, _) = engine();
        let rig = chain(&mut e, &[2, 2, 2], [0, 1]);
        let answer = e.call(0x00cd_a8d0, &args![rig.pose]).u32();
        assert_eq!(answer, rig.pose + 0x10);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 1);
        assert_eq!(flags_of(&e, &rig, 3), [0, 0, 0]);
    }

    #[test]
    fn reference_translations_follow_the_parents_and_mark_the_bones() {
        let (mut e, _) = engine();
        let rig = make_rig(
            &mut e,
            &[NO_PARENT, 0, 1, NO_PARENT],
            &[1, 1, 0, 0],
            0,
            [1, 1],
        );
        store_vec(&mut e, rig.reference, [5.0, 6.0, 7.0, 8.0]);
        store_vec(&mut e, rig.reference + 0x30, [1.0, 1.0, 1.0, 0.0]);
        for k in 0..4u32 {
            put_transform(&mut e, rig.model + 0x30 * k, [-1.0; 4], IDENTITY, [1.0; 4]);
        }
        set_flags_of(&mut e, &rig, &[0, 0, 0x18, 4]);
        e.call(0x00cd_a4c0, &args![rig.pose]);
        assert_eq!(load_vec(&e, rig.model), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vec(&e, rig.model + 0x30), [6.0, 7.0, 8.0, 8.0]);
        assert_eq!(load_vec(&e, rig.model + 0x60), [-1.0; 4]);
        assert_eq!(flags_of(&e, &rig, 4), [1, 1, 1, 0]);
        // Out of date model transforms are brought up to date first.
        e.mem.set_u8(rig.pose + 0x28, 0);
        store_vec(&mut e, rig.local, [3.0, 3.0, 3.0, 3.0]);
        set_flags_of(&mut e, &rig, &[2, 0, 0, 0]);
        e.call(0x00cd_a4c0, &args![rig.pose]);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 1);
        assert_eq!(flags_of(&e, &rig, 4), [1, 1, 1, 0]);
    }

    #[test]
    fn the_bounding_box_folds_in_every_model_translation() {
        let (mut e, _) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT; 3], &[0; 3], 0, [1, 1]);
        let translations = [
            [1.0, 5.0, 3.0, 0.0],
            [-2.0, 7.0, 9.0, 0.0],
            [4.0, -1.0, 0.0, 1.0],
        ];
        for (k, t) in translations.into_iter().enumerate() {
            put_transform(&mut e, rig.model + 0x30 * k as u32, t, IDENTITY, [1.0; 4]);
        }
        let out = e.mem.alloc(0x20);
        e.call(0x00cd_a610, &args![rig.pose, out]);
        assert_eq!(load_vec(&e, out), [-2.0, -1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [4.0, 7.0, 9.0, 1.0]);
        // A pose without bones leaves the starting values.
        let (mut e, _) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT], &[0], 0, [1, 1]);
        e.mem.set_u32(rig.skeleton + 0x1c, 0);
        let out = e.mem.alloc(0x20);
        e.call(0x00cd_a610, &args![rig.pose, out]);
        assert_eq!(load_vec(&e, out), [1.0e30; 4]);
        assert_eq!(load_vec(&e, out + 0x10), [-1.0e30; 4]);
    }

    /// Two bones with a root whose scale is (2, 4, 8) and whose rotation is the identity.
    fn local_rig(e: &mut Engine, root_flags: u32) -> Rig {
        let rig = make_rig(e, &[NO_PARENT, 0], &[0; 2], 0, [1, 0]);
        put_transform(
            e,
            rig.model,
            [1.0, 2.0, 3.0, 0.0],
            IDENTITY,
            [2.0, 4.0, 8.0, 1.0],
        );
        put_transform(
            e,
            rig.model + 0x30,
            [11.0, 22.0, 33.0, 5.0],
            [0.5, 0.25, 0.0, 0.0],
            [6.0, 8.0, 16.0, 3.0],
        );
        put_transform(e, rig.local, [-1.0; 4], [-2.0; 4], [-3.0; 4]);
        put_transform(e, rig.local + 0x30, [-1.0; 4], [-2.0; 4], [-3.0; 4]);
        set_flags_of(e, &rig, &[root_flags, 1]);
        rig
    }

    fn check_local_child(e: &Engine, rig: &Rig) {
        let [t, q, s] = get_transform(e, rig.local + 0x30);
        assert_eq!(t, [10.0, 20.0, 30.0, 5.0]);
        assert_eq!(q, [0.5, 0.25, 0.0, 1.0]);
        for (got, want) in s.iter().zip([3.0, 2.0, 2.0]) {
            assert!(close(*got, want), "{got} {want}");
        }
        assert_eq!(s[3], 0.0);
    }

    #[test]
    fn syncing_the_local_space_inverts_the_parent() {
        let (mut e, log) = engine();
        let rig = local_rig(&mut e, 1);
        e.call(0x00cd_a690, &args![rig.pose]);
        assert_eq!(get_transform(&e, rig.local), get_transform(&e, rig.model));
        check_local_child(&e, &rig);
        assert_eq!(flags_of(&e, &rig, 2), [0, 0]);
        assert_eq!(e.mem.u8(rig.pose + 0x29), 1);
        let calls: Vec<(u32, u32)> = log.borrow().iter().map(|(_, a)| (a[0], a[2])).collect();
        assert_eq!(calls, [(rig.local + 0x40, rig.model + 0x40)]);
        // A parent whose model transform is stale is brought up to date first.
        let (mut e, _) = engine();
        let rig = local_rig(&mut e, 2);
        store_vec(&mut e, rig.local, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, rig.local + 0x10, IDENTITY);
        store_vec(&mut e, rig.local + 0x20, [2.0, 4.0, 8.0, 1.0]);
        for offset in [0u32, 0x10, 0x20] {
            store_vec(&mut e, rig.model + offset, [-9.0; 4]);
        }
        e.call(0x00cd_a690, &args![rig.pose]);
        assert_eq!(
            get_transform(&e, rig.model),
            [[1.0, 2.0, 3.0, 0.0], IDENTITY, [2.0, 4.0, 8.0, 1.0]]
        );
        check_local_child(&e, &rig);
        assert_eq!(flags_of(&e, &rig, 2), [0, 0]);
        // Already synced: nothing happens.
        let (mut e, _) = engine();
        let rig = local_rig(&mut e, 1);
        e.mem.set_u8(rig.pose + 0x29, 1);
        e.call(0x00cd_a690, &args![rig.pose]);
        assert_eq!(flags_of(&e, &rig, 2), [1, 1]);
    }

    /// The exe's local-from-model step turns the parent's translation by the parent's INVERSE
    /// rotation (cross term `t x q` in 00cda690 / 00cda8e0 / 00cdad80), so a child sitting at
    /// its parent's model position plus (5, 0, 0) turned by the parent comes back as (5, 0, 0)
    /// for a parent turned 90 degrees about x and standing 100 units up. Turning the parent's
    /// translation by the rotation itself gave (5, 200, 0): Doc's head 150 units off his neck.
    #[test]
    fn syncing_the_local_space_turns_the_parent_translation_back() {
        let (mut e, _) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0], &[0; 2], 0, [1, 0]);
        let s = std::f32::consts::FRAC_1_SQRT_2;
        put_transform(
            &mut e,
            rig.model,
            [0.0, 0.0, 100.0, 0.0],
            [s, 0.0, 0.0, s],
            [1.0, 1.0, 1.0, 1.0],
        );
        // The child: parent translation + q (5, 0, 0) = (5, 0, 100).
        put_transform(
            &mut e,
            rig.model + 0x30,
            [5.0, 0.0, 100.0, 0.0],
            [s, 0.0, 0.0, s],
            [1.0, 1.0, 1.0, 1.0],
        );
        set_flags_of(&mut e, &rig, &[1, 1]);
        e.call(0x00cd_a690, &args![rig.pose]);
        let [t, _, _] = get_transform(&e, rig.local + 0x30);
        for (got, want) in t[..3].iter().zip([5.0, 0.0, 0.0]) {
            assert!((got - want).abs() < 1e-3, "local translation {t:?}");
        }
        // The same step through accessBoneModelSpace with its children kept (00cdad80, 1),
        // the call DoLookAtIK makes on the head bone.
        let (mut e, _) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0], &[0; 2], 0, [1, 0]);
        put_transform(
            &mut e,
            rig.model,
            [0.0, 0.0, 100.0, 0.0],
            [s, 0.0, 0.0, s],
            [1.0, 1.0, 1.0, 1.0],
        );
        put_transform(
            &mut e,
            rig.model + 0x30,
            [5.0, 0.0, 100.0, 0.0],
            [s, 0.0, 0.0, s],
            [1.0, 1.0, 1.0, 1.0],
        );
        set_flags_of(&mut e, &rig, &[0, 1]);
        e.call(0x00cd_ad80, &args![rig.pose, 0u32, 1u32]);
        let [t, _, _] = get_transform(&e, rig.local + 0x30);
        for (got, want) in t[..3].iter().zip([5.0, 0.0, 0.0]) {
            assert!((got - want).abs() < 1e-3, "local translation {t:?}");
        }
    }

    /// The first half of the local-from-model step (the parent's own translation) works on the
    /// parent's raw rotation (00cda70c-00cda79c): lane 3 of it is `+dot(t, q) * q.w`, where the
    /// inverse rotation's sign flips would make it `-dot * q.w`. A parent at (1, 2, 3, 4) turned
    /// by (0.5, 0.5, 0.5, 0.5) gives a first half of (1, 1.5, 0.5, 0.5) (negated double
    /// (-2, -3, -1, -1)); a child at the parent's translation gives a second half doubled to
    /// (2, 3, 1, -5), so its local translation is (0, 0, 0, -6), exactly. With `-dot * q.w` lane
    /// 3 came out 0. The same through `syncLocalSpace`, `accessBoneLocalSpace` on the bone itself
    /// (00cdabaf-00cdac3f) and on its parent, which reaches it as a later bone
    /// (00cda977-00cdaa0b), and `00cdad80` with its children kept (00cdae77-00cdaf0b).
    #[test]
    fn the_parent_translation_w_lane_takes_the_raw_rotation_dot_term() {
        let rig_of = |e: &mut Engine, flags: [u32; 2]| {
            let rig = make_rig(e, &[NO_PARENT, 0], &[0; 2], 0, [1, 0]);
            put_transform(
                e,
                rig.model,
                [1.0, 2.0, 3.0, 4.0],
                [0.5, 0.5, 0.5, 0.5],
                [1.0, 1.0, 1.0, 1.0],
            );
            put_transform(
                e,
                rig.model + 0x30,
                [1.0, 2.0, 3.0, 4.0],
                [0.5, 0.5, 0.5, 0.5],
                [1.0, 1.0, 1.0, 1.0],
            );
            put_transform(e, rig.local + 0x30, [-9.0; 4], [-9.0; 4], [-9.0; 4]);
            set_flags_of(e, &rig, &flags);
            rig
        };
        let want: Vec4 = [0.0, 0.0, 0.0, -6.0];
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [1, 1]);
        e.call(0x00cd_a690, &args![rig.pose]);
        assert_eq!(load_vec(&e, rig.local + 0x30), want);
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [0, 1]);
        e.call(0x00cd_a8e0, &args![rig.pose, 1u32]);
        assert_eq!(load_vec(&e, rig.local + 0x30), want);
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [0, 1]);
        e.call(0x00cd_a8e0, &args![rig.pose, 0u32]);
        assert_eq!(load_vec(&e, rig.local + 0x30), want);
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [0, 1]);
        e.call(0x00cd_ad80, &args![rig.pose, 0u32, 1u32]);
        assert_eq!(load_vec(&e, rig.local + 0x30), want);
    }

    /// The first half's dot and along term take the parent's RAW rotation in lanes 0 to 2 as
    /// well (00cda716-00cda72b `z + (y + x)` of `t * q`, 00cda778 / 00cda77c `splat(dot) * q`),
    /// so a dot that sums to +0 leaves zeros signed by `q`. A parent at (0, 0, 0, 0) turned by
    /// (0.5, -0.5, 0.5, -0.5): `t * q` is (+0, -0, +0, -0), the dot +0, the first half
    /// (+0, -0, +0, -0) and its negated double (-0, +0, -0, +0). A child at (0, 0, 0, 0) under
    /// it: the inverse (-0.5, 0.5, -0.5, -0.5) gives a second half doubled to (-0, +0, +0, -0),
    /// so the local translation is (-0, +0, +0, +0) (bits 0x80000000, 0, 0, 0). `rotate_half` of
    /// the inverse made lane 0 of the first half -0 and stored +0 there. Checked by bits, as
    /// `assert_eq!` on f32 takes -0.0 for +0.0, through every inlined copy: `syncLocalSpace`
    /// (00cda70c), `accessBoneLocalSpace` on the parent (00cda977) and on the child itself
    /// (00cdabaf), and `00cdad80` with its children kept (00cdae77).
    #[test]
    fn a_first_half_that_sums_to_zero_keeps_the_raw_rotation_signs() {
        let rig_of = |e: &mut Engine, flags: [u32; 2]| {
            let rig = make_rig(e, &[NO_PARENT, 0], &[0; 2], 0, [1, 0]);
            let rotation: Vec4 = [0.5, -0.5, 0.5, -0.5];
            put_transform(e, rig.model, [0.0; 4], rotation, [1.0; 4]);
            put_transform(e, rig.model + 0x30, [0.0; 4], rotation, [1.0; 4]);
            put_transform(e, rig.local + 0x30, [-9.0; 4], [-9.0; 4], [-9.0; 4]);
            set_flags_of(e, &rig, &flags);
            rig
        };
        let want: [u32; 4] = [0x8000_0000, 0, 0, 0];
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [1, 1]);
        e.call(0x00cd_a690, &args![rig.pose]);
        assert_eq!(load_bits(&e, rig.local + 0x30), want);
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [0, 1]);
        e.call(0x00cd_a8e0, &args![rig.pose, 0u32]);
        assert_eq!(load_bits(&e, rig.local + 0x30), want);
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [0, 1]);
        e.call(0x00cd_a8e0, &args![rig.pose, 1u32]);
        assert_eq!(load_bits(&e, rig.local + 0x30), want);
        let (mut e, _) = engine();
        let rig = rig_of(&mut e, [0, 1]);
        e.call(0x00cd_ad80, &args![rig.pose, 0u32, 1u32]);
        assert_eq!(load_bits(&e, rig.local + 0x30), want);
    }

    #[test]
    fn the_local_space_accessors_sync_first() {
        let (mut e, _) = engine();
        let rig = local_rig(&mut e, 1);
        let answer = e.call(0x00cd_b290, &args![rig.pose]).u32();
        assert_eq!(answer, rig.pose + 4);
        assert_eq!(flags_of(&e, &rig, 2), [0, 0]);
        assert_eq!(e.mem.u8(rig.pose + 0x29), 1);
        let (mut e, _) = engine();
        let rig = local_rig(&mut e, 1);
        let answer = e.call(0x00cd_b0a0, &args![rig.pose]).u32();
        assert_eq!(answer, rig.pose + 4);
        // The getter that follows marks every model transform stale.
        assert_eq!(flags_of(&e, &rig, 2), [2, 2]);
        assert_eq!(
            (e.mem.u8(rig.pose + 0x28), e.mem.u8(rig.pose + 0x29)),
            (0, 1)
        );
        check_local_child(&e, &rig);
    }

    #[test]
    fn accessing_a_bone_local_transform_marks_its_descendants() {
        let (mut e, _) = engine();
        let rig = chain(&mut e, &[0, 0, 0], [1, 1]);
        let answer = e.call(0x00cd_a8e0, &args![rig.pose, 0u32]).u32();
        assert_eq!(answer, rig.local);
        assert_eq!(flags_of(&e, &rig, 3), [2, 2, 2]);
        assert_eq!(
            (e.mem.u8(rig.pose + 0x28), e.mem.u8(rig.pose + 0x29)),
            (0, 1)
        );
        // Stale local transforms of the bone and of its descendants are refreshed from the model
        // space first (the bone itself last), and a sibling that is not below it is left alone.
        let (mut e, log) = engine();
        let rig = chain(&mut e, &[0, 1, 1], [1, 1]);
        for k in 0..3u32 {
            put_transform(
                &mut e,
                rig.model + 0x30 * k,
                [1.0, 2.0, 3.0, 0.0],
                IDENTITY,
                [1.0; 4],
            );
        }
        let answer = e.call(0x00cd_a8e0, &args![rig.pose, 1u32]).u32();
        assert_eq!(answer, rig.local + 0x30);
        assert_eq!(flags_of(&e, &rig, 3), [0, 2, 2]);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 0);
        let calls: Vec<(u32, u32)> = log.borrow().iter().map(|(_, a)| (a[0], a[2])).collect();
        assert_eq!(
            calls,
            [
                (rig.local + 0x70, rig.model + 0x70),
                (rig.local + 0x40, rig.model + 0x40),
            ]
        );
    }

    #[test]
    fn accessing_a_bone_model_transform_marks_its_children_or_descendants() {
        // Without the flag only the direct children are marked, after being brought up to date.
        let (mut e, log) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0, 0, 1], &[0; 4], 0, [1, 1]);
        for k in 0..4u32 {
            put_transform(
                &mut e,
                rig.local + 0x30 * k,
                [1.0 + k as f32, 0.0, 0.0, 0.0],
                IDENTITY,
                [1.0; 4],
            );
        }
        put_transform(&mut e, rig.model, [0.0; 4], IDENTITY, [1.0; 4]);
        set_flags_of(&mut e, &rig, &[0, 2, 0, 0]);
        let answer = e.call(0x00cd_ad80, &args![rig.pose, 0u32, 0u32]).u32();
        assert_eq!(answer, rig.model);
        assert_eq!(flags_of(&e, &rig, 4), [1, 1, 1, 0]);
        assert_eq!(e.mem.u8(rig.pose + 0x29), 0);
        assert_eq!(load_vec(&e, rig.model + 0x30), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            log.borrow()[..],
            [set_mul(
                rig.model + 0x40,
                rig.model + 0x10,
                rig.local + 0x40
            )]
        );
        // With the flag the descendants are marked as for the local transform, and the bone is
        // brought up to date.
        let (mut e, _) = engine();
        let rig = chain(&mut e, &[2, 0, 0], [1, 1]);
        let answer = e.call(0x00cd_ad80, &args![rig.pose, 0u32, 1u32]).u32();
        assert_eq!(answer, rig.model);
        assert_eq!(flags_of(&e, &rig, 3), [1, 2, 2]);
        assert_eq!(
            (e.mem.u8(rig.pose + 0x28), e.mem.u8(rig.pose + 0x29)),
            (0, 0)
        );
        assert_eq!(load_vec(&e, rig.model), [1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn sizing_a_pose_grows_the_arrays_and_zero_fills_the_slots() {
        let (mut e, log) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0, 1], &[0; 3], 5, [1, 1]);
        // The bone arrays: one with room (nothing reserved), one too small (twice its capacity
        // is more than the count), one empty.
        e.mem.set_u32(rig.pose + 0x18, 6);
        e.mem.set_u32(rig.pose + 0x0c, 2);
        e.mem.set_u32(rig.pose + 0x24, 0);
        // The slots: room for eight, five wanted, two in use and the rest not zero.
        e.mem.set_u32(rig.pose + 0x34, 8);
        e.mem.set_u32(rig.pose + 0x30, 2);
        for k in 0..8u32 {
            e.mem.set_u32(rig.floats + 4 * k, 0x77);
        }
        let data = e.mem.alloc(0x90);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, data);
        e.call(0x00cd_b150, &args![rig.pose, 1u32, 0u32, source]);
        let model = e.mem.u32(rig.pose + 0x10);
        let local = e.mem.u32(rig.pose + 4);
        let flags = e.mem.u32(rig.pose + 0x1c);
        assert_eq!(model, rig.model);
        assert_ne!(local, rig.local);
        assert_ne!(flags, rig.flags);
        assert_eq!(
            [
                e.mem.u32(rig.pose + 0x14),
                e.mem.u32(rig.pose + 8),
                e.mem.u32(rig.pose + 0x20)
            ],
            [3, 3, 3]
        );
        assert_eq!(
            [
                e.mem.u32(rig.pose + 0x18),
                e.mem.u32(rig.pose + 0x0c),
                e.mem.u32(rig.pose + 0x24)
            ],
            [6, 4, 3]
        );
        let slots: Vec<u32> = (0..8).map(|k| e.mem.u32(rig.floats + 4 * k)).collect();
        assert_eq!(slots, [0x77, 0x77, 0, 0, 0, 0x77, 0x77, 0x77]);
        assert_eq!(e.mem.u32(rig.pose + 0x30), 5);
        assert_eq!(
            log.borrow()[..],
            [
                entry(ARRAY_RESERVE, &[ALLOCATOR, rig.pose + 4, 4, 0x30]),
                entry(ARRAY_RESERVE, &[ALLOCATOR, rig.pose + 0x1c, 3, 4]),
                entry(MEM_COPY, &[local, data, 0x90]),
            ]
        );
        assert_eq!(flags_of_pose(&e, rig.pose, 3), [2, 2, 2]);
        // Any other mode copies into the model transforms; slots are reserved when too few.
        let (mut e, log) = engine();
        let rig = make_rig(&mut e, &[NO_PARENT, 0, 1], &[0; 3], 5, [1, 1]);
        e.mem.set_u32(rig.pose + 0x34, 1);
        e.mem.set_u32(rig.pose + 0x30, 1);
        let data = e.mem.alloc(0x90);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, data);
        e.call(0x00cd_b150, &args![rig.pose, 0u32, 0u32, source]);
        assert_eq!(
            log.borrow()[..],
            [
                entry(MEM_COPY, &[rig.model, data, 0x90]),
                entry(ARRAY_RESERVE, &[ALLOCATOR, rig.pose + 0x2c, 8, 4]),
            ]
        );
        assert_eq!(flags_of_pose(&e, rig.pose, 3), [1, 1, 1]);
        assert_eq!(e.mem.u32(rig.pose + 0x30), 5);
    }

    fn flags_of_pose(e: &Engine, pose: u32, count: u32) -> Vec<u32> {
        (0..count)
            .map(|k| e.mem.u32(e.mem.u32(pose + 0x1c) + 4 * k))
            .collect()
    }
}
