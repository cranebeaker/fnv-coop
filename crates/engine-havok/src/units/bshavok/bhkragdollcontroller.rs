//! `BSHavok/bhkragdollcontroller.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// CRT x87 square root (`00ec6040`, takes and answers `ST0`) and the CRT `memmove` (`00ec7230`,
/// cdecl `(destination, source, bytes)`).
const X87_SQRT: u32 = 0x00ec_6040;
const MEMMOVE: u32 = 0x00ec_7230;
/// `MemoryManager::Allocate` (`00aa3e40`, thiscall `(size)`) and `MemoryManager::Deallocate`
/// (`00aa4060`, thiscall `(block)`) on the manager `011f6238`.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// Per-thread word (`TLS + 0x2b4`) that the functions with an unwinding frame set to 0x10.
const SCOPE_WORD: u32 = 0x2b4;
/// `NiPoint3` copy (`00458620`, cdecl `(destination, source)`).
const POINT_COPY: u32 = 0x0045_8620;
/// Float factor the NI to Havok conversion (`00c74dd0`) and the grab IK (`00c751f0`) scale
/// positions with (`01267c20`).
const SCALE_FACTOR: u32 = 0x0126_7c20;
/// The empty lock the capsule lookup calls twice (`00a29680`, a bare `RET`).
const EMPTY_LOCK: u32 = 0x00a2_9680;
/// Constant vectors: the ones vector (`010ec7f0`) and the two vectors `00c74ed0` copies out of the
/// data section (`01267e30`, `012683b0`).
const ONES_VECTOR: u32 = 0x010e_c7f0;
const CONSTANT_VECTOR_A: u32 = 0x0126_7e30;
const CONSTANT_VECTOR_B: u32 = 0x0126_83b0;

// Translated from 00c7e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c7_dc40`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c7e970(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c7_dc40, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c7a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollShareData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c76240`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x20 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_share_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c7_6240;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x20u32]);
    }
    this
}

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// Reads four floats (a `MOVAPS` load).
fn load4(e: &Engine, address: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(address + offset))
}

/// Writes four floats (a `MOVAPS` store).
fn store4(e: &mut Engine, address: u32, lanes: [f32; 4]) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

/// Copies 16 bytes (a `MOVAPS` load and store).
fn copy16(e: &mut Engine, from: u32, to: u32) {
    for offset in [0, 4, 8, 12] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

/// An `FLD dword` / `FSTP dword` copy of a float: a signalling NaN comes out quiet.
fn copy_quiet(e: &mut Engine, from: u32, to: u32) {
    let bits = e.mem.u32(from);
    e.mem.set_u32(to, x87_quiet_bits(bits));
}

// Translated from 00a38560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::Normalize` (Xbox PDB): divides the four floats at `this` by the square root of
/// their sum of squares (summed as `((y*y + x*x) + z*z) + w*w`, narrowed to a float after the
/// sum and after the root; the CRT root is `00ec6040`). The reciprocal is a float too.
pub fn ni_quaternion_normalize(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let [x, y, z, w] = load4(e, base).map(f64::from);
    let sum = (((y * y + x * x) + z * z) + w * w) as f32;
    let root = e.call(X87_SQRT, &args![f64::from(sum)]).f32();
    let inverse = (1.0 / f64::from(root)) as f32;
    for lane in 0..4u32 {
        let value = e.mem.f32(base + 4 * lane);
        e.mem.set_f32(
            base + 4 * lane,
            (f64::from(inverse) * f64::from(value)) as f32,
        );
    }
}

// Translated from 00c747b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot `0xb4` of `this` with the address of a scratch byte; answers the result minus 4
/// when it is not zero, else 0.
pub fn fn_00c747b0(e: &mut Engine, this: Ptr) -> u32 {
    let result = e.with_stack(4, |e, scratch| {
        e.vcall(this.addr(), 0xb4, &args![scratch]).u32()
    });
    if result != 0 {
        result.wrapping_sub(4)
    } else {
        0
    }
}

// Translated from 00c74910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 16-bit `value` for `key` in a chained hash map (the table pointer at `+8`, the count
/// at `+0xc`; a node is `{next, key, value}`). Slot 4 hashes the key to the bucket index, slot 8
/// compares two keys, slot `0x14` makes a node and slot `0xc` fills it in. An existing node
/// just takes the new value; a new node goes to the front of its bucket.
pub fn fn_00c74910(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let index = e.vcall(map, 4, &args![key]).u32();
    let table = e.mem.u32(map + 8);
    let mut node = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, 8, &args![key, node_key]).bool() {
            e.mem.set_u16(node + 8, value as u16);
            return;
        }
        node = e.mem.u32(node);
    }
    let fresh = e.vcall(map, 0x14, &args![]).u32();
    e.vcall(map, 0xc, &args![fresh, key, value]);
    let table = e.mem.u32(map + 8);
    let head = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    e.mem.set_u32(fresh, head);
    let table = e.mem.u32(map + 8);
    e.mem
        .set_u32(table.wrapping_add(index.wrapping_mul(4)), fresh);
    let count = e.mem.u32(map + 0xc);
    e.mem.set_u32(map + 0xc, count.wrapping_add(1));
}

// Translated from 00c749f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `count` elements of 0x38 bytes from `source` to `destination` (`memmove`, one element
/// at a time: from the front when the destination is below the source, from the back when it is
/// above; nothing when they are equal).
pub fn fn_00c749f0(e: &mut Engine, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        for index in 0..count {
            let step = index.wrapping_mul(0x38);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(step),
                    source.wrapping_add(step),
                    0x38u32
                ],
            );
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let step = (index as u32).wrapping_mul(0x38);
            e.call(
                MEMMOVE,
                &args![
                    destination.wrapping_add(step),
                    source.wrapping_add(step),
                    0x38u32
                ],
            );
            index -= 1;
        }
    }
}

// Translated from 00c74b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c74b70 - the /GS stack-cookie check (00ec408c) is not modeled.
/// Quaternion product `a * b` into `out` (SSE): with `a` and `b` as `(x, y, z, w)`, the vector
/// part is `a.w*b + cross(b, a)... - b.w*a` lane by lane as the code orders it, `(b.y*a.z -
/// b.z*a.y, b.z*a.x - b.x*a.z, b.x*a.y - b.y*a.x) + a.w*b - b.w*a`, and the last lane is the
/// sum of the products `b_i*a_i`, added as `(p3 + p1) + (p2 + p0)`.
pub fn fn_00c74b70(e: &mut Engine, out: Ptr, a: Ptr, b: Ptr) {
    let a = load4(e, a.addr());
    let b = load4(e, b.addr());
    let minuend = [b[1] * a[2], b[2] * a[0], b[0] * a[1]];
    let subtrahend = [b[2] * a[1], b[0] * a[2], b[1] * a[0]];
    let products = [b[0] * a[0], b[1] * a[1], b[2] * a[2], b[3] * a[3]];
    let dot = (products[3] + products[1]) + (products[2] + products[0]);
    let mut result = [0.0f32; 4];
    for lane in 0..3 {
        let cross = minuend[lane] - subtrahend[lane];
        result[lane] = (a[3] * b[lane] + cross) - b[3] * a[lane];
    }
    result[3] = dot;
    store4(e, out.addr(), result);
}

// Translated from 00c74d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c74d50 - the /GS stack-cookie check (00ec408c) is not modeled.
/// Converts the transform at `source` to the Havok form at `destination`: builds the quaternion
/// `(source+0x1c, +0x10, +0x14, +0x18)` (each a quiet float copy), normalises it
/// (`ni_quaternion_normalize`), has `004f0180` (thiscall on the quaternion, `(destination)`)
/// convert it, copies the point at `source` to `destination + 0x24` (`00458620`) and copies
/// the float `source + 0x20` to `destination + 0x30` (quiet copy). Answers `destination`.
pub fn fn_00c74d50(e: &mut Engine, destination: Ptr, source: Ptr) -> Ptr {
    const CONVERT: u32 = 0x004f_0180;
    let from = source.addr();
    let to = destination.addr();
    e.with_stack(0x10, |e, quaternion| {
        let q = quaternion.addr();
        copy_quiet(e, from + 0x1c, q);
        copy_quiet(e, from + 0x10, q + 4);
        copy_quiet(e, from + 0x14, q + 8);
        copy_quiet(e, from + 0x18, q + 0xc);
        ni_quaternion_normalize(e, quaternion);
        e.call(CONVERT, &args![quaternion, destination]);
    });
    e.call(POINT_COPY, &args![to + 0x24, source]);
    copy_quiet(e, from + 0x20, to + 0x30);
    destination
}

// Translated from 00c74dd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c74dd0 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `NI2HK_ov2` (Xbox PDB): converts the transform at `source` for Havok into `destination`: a
/// temporary quaternion from `a6df40` (thiscall on the temporary, `(source)`) goes to
/// `destination + 0x10` as `(y, z, w, x)` (quiet copies) and is normalised by `005611c0`
/// (thiscall on it). Then the position `(source+0x24, +0x28, +0x2c)` times the float at
/// `01267c20` (each product taken in `f64` and narrowed) and `0.0` go to `destination`, and the
/// float at `source + 0x30` fills the four lanes at `destination + 0x20`. Answers `destination`.
pub fn ni2hk_ov2(e: &mut Engine, destination: Ptr, source: Ptr) -> Ptr {
    const TO_QUATERNION: u32 = 0x00a6_df40;
    const NORMALISE: u32 = 0x0056_11c0;
    let from = source.addr();
    let to = destination.addr();
    e.with_stack(0x10, |e, quaternion| {
        let q = quaternion.addr();
        e.call(TO_QUATERNION, &args![quaternion, source]);
        copy_quiet(e, q + 4, to + 0x10);
        copy_quiet(e, q + 8, to + 0x14);
        copy_quiet(e, q + 0xc, to + 0x18);
        copy_quiet(e, q, to + 0x1c);
    });
    e.call(NORMALISE, &args![to + 0x10]);
    let factor: f32 = e.global(SCALE_FACTOR);
    let scaled = [0x24, 0x28, 0x2c]
        .map(|offset| (f64::from(e.mem.f32(from + offset)) * f64::from(factor)) as f32);
    store4(e, to, [scaled[0], scaled[1], scaled[2], 0.0]);
    let fill = e.mem.u32(from + 0x30);
    for offset in [0x20, 0x24, 0x28, 0x2c] {
        e.mem.set_u32(to + offset, fill);
    }
    destination
}

// Translated from 00c74e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRaycastInterface::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable `010c4b60`
/// and, when bit 0 of `flags` is set, hands the block to `MemoryManager::Deallocate`
/// (`00aa4060`). Answers `this`.
pub fn hka_raycast_interface_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_4b60);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c74ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `size + 0x10` bytes (`MemoryManager::Allocate`, `00aa3e40`) and answers the block
/// moved up to the next 16-byte boundary (by 1 to 16 bytes); the byte before the answer holds
/// that distance.
pub fn fn_00c74ea0(e: &mut Engine, size: u32) -> u32 {
    let raw = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, size.wrapping_add(0x10)],
        )
        .u32();
    let shift = 0x10 - (raw as u8 & 0xf);
    let aligned = raw.wrapping_add(u32::from(shift));
    e.mem.set_u8(aligned.wrapping_sub(1), shift);
    aligned
}

// Translated from 00c74ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises the 0x82-byte record at `this` (register `ECX`): five 16-bit values of `0xffff`
/// at `+0..+9`, the vector `01267e30` at `+0x10`, the float `01012054` at `+0x20` (a quiet
/// copy), `1.0` at `+0x24`, `+0x28`, `+0x2c` and `+0x30`, then the vectors `01267e30`,
/// `012683b0`, `01267e30`, `012683b0` at `+0x40`, `+0x50`, `+0x60`, `+0x70`, the byte `1` at
/// `+0x80` and `0` at `+0x81`. Answers `this`.
pub fn fn_00c74ed0(e: &mut Engine, this: Ptr) -> Ptr {
    let record = this.addr();
    for offset in [0, 2, 4, 6, 8] {
        e.mem.set_u16(record + offset, 0xffff);
    }
    copy16(e, CONSTANT_VECTOR_A, record + 0x10);
    copy_quiet(e, 0x0101_2054, record + 0x20);
    for offset in [0x24, 0x28, 0x2c, 0x30] {
        e.mem.set_f32(record + offset, 1.0);
    }
    copy16(e, CONSTANT_VECTOR_A, record + 0x40);
    copy16(e, CONSTANT_VECTOR_B, record + 0x50);
    copy16(e, CONSTANT_VECTOR_A, record + 0x60);
    copy16(e, CONSTANT_VECTOR_B, record + 0x70);
    e.mem.set_u8(record + 0x80, 1);
    e.mem.set_u8(record + 0x81, 0);
    this
}

// Translated from 00c74fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the settings records at `this` (register `ECX`) and `other` differ: the counts at
/// `+0x48`, the 16-bit arrays at `+0x44`, the byte at `+0`, the floats at `+4` to `+0x30` (every
/// float must compare equal; NaN differs), the dwords at `+0x38`, `+0x3c` and the float at `+0x34`.
pub fn fn_00c74fb0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let a = this.addr();
    let b = other.addr();
    let count = e.mem.u32(a + 0x48);
    if count != e.mem.u32(b + 0x48) {
        return true;
    }
    if count != 0 {
        let first = e.mem.u32(a + 0x44);
        let second = e.mem.u32(b + 0x44);
        for index in 0..count {
            let offset = index.wrapping_mul(2);
            if e.mem.u16(first.wrapping_add(offset)) != e.mem.u16(second.wrapping_add(offset)) {
                return true;
            }
        }
    }
    if e.mem.u8(a) != e.mem.u8(b) {
        return true;
    }
    let floats_equal = |e: &Engine, offset: u32| e.mem.f32(a + offset) == e.mem.f32(b + offset);
    for offset in (4..=0x30).step_by(4) {
        if !floats_equal(e, offset) {
            return true;
        }
    }
    if e.mem.u32(a + 0x38) != e.mem.u32(b + 0x38) || e.mem.u32(a + 0x3c) != e.mem.u32(b + 0x3c) {
        return true;
    }
    !floats_equal(e, 0x34)
}

// Translated from 00c75100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c75100 - the FS:[0] exception-unwinding frame is not modeled.
/// With the allocation-scope word at 0x10, calls slot 0 of the object whose pointer is stored
/// at `this` (register `ECX`; nothing when it is null) with the argument 1 (the deleting
/// destructor), then restores the word.
pub fn fn_00c75100(e: &mut Engine, this: Ptr) {
    let (word, saved) = enter_scope(e);
    let object = e.mem.u32(this.addr());
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c75180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::GetCapsuleBoneShapeInfo` (Xbox PDB): follows `00c802d0(handle)` (cdecl)
/// to its `+0x10`, the object at that one's `+8` (with the empty lock `00a29680` taken twice on
/// the former) and the chain `[object + 0x10] -> [+8]`. When the chain reaches an object,
/// answers slot `0xb4` of it called with the address of a word holding `handle`; else 0.
pub fn bhk_ragdoll_controller_get_capsule_bone_shape_info(e: &mut Engine, handle: u32) -> u32 {
    let holder = 'chain: {
        if handle == 0 {
            break 'chain 0;
        }
        let found = e.call(0x00c8_02d0, &args![handle]).u32();
        if found == 0 {
            break 'chain 0;
        }
        let shape = e.mem.u32(found + 0x10);
        if shape == 0 {
            break 'chain 0;
        }
        let object = e.mem.u32(shape + 8);
        if object == 0 {
            break 'chain 0;
        }
        e.call(EMPTY_LOCK, &args![shape]);
        let slot = object.wrapping_add(0x10);
        e.call(EMPTY_LOCK, &args![shape]);
        if slot == 0 {
            break 'chain 0;
        }
        let inner = e.mem.u32(slot);
        if inner == 0 {
            break 'chain 0;
        }
        e.mem.u32(inner + 8)
    };
    if holder == 0 {
        return 0;
    }
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), handle);
        e.vcall(holder, 0xb4, &args![argument]).u32()
    })
}

/// The pose helpers `UpdateRagdollRigidBodies` uses: `hkaPose::getSyncedPoseModelSpace` (Ghidra
/// name, `00cda8d0`, thiscall, answers a pointer), `hkaSkeletonMapper::mapPose` (Ghidra name,
/// `00ce3a00`, cdecl `(mapper, source, local, result, 2)`),
/// `hkaSkeletonUtils::transformModelPoseToLocalPose` (Ghidra name, `00cddef0`, cdecl
/// `(count, parents, model, local)`), `hkaRagdollRigidBodyController::driveToPose` (Ghidra name,
/// `00cdb7c0`, thiscall `(strength, pose, transform, scratch)`) and `00ce1ad0` (thiscall on the
/// constraint list, `(model pose, transform)`).
const GET_SYNCED_POSE_MODEL_SPACE: u32 = 0x00cd_a8d0;
const MAP_POSE: u32 = 0x00ce_3a00;
const MODEL_TO_LOCAL_POSE: u32 = 0x00cd_def0;
const RIGID_BODY_DRIVE_TO_POSE: u32 = 0x00cd_b7c0;
const APPLY_MODEL_POSE: u32 = 0x00ce_1ad0;
/// The drive strength float (`011afe64`) and the table of per-type limit floats (`010c7948`,
/// indexed by the byte at a body's `+0x19c`).
const DRIVE_STRENGTH: u32 = 0x011a_fe64;
const LIMIT_TABLE: u32 = 0x010c_7948;

// Translated from 00c7d030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7d030 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollController::UpdateRagdollRigidBodies` (Ghidra name), thiscall `(source)`: two
/// pose blocks (`count * 0x30` bytes rounded up to 128, count from the constraint list's
/// `+0x2c` object) are taken from the allocation cursor. A set byte `+0x44` forces the mode
/// word `+0xc` to 1 for the call (the old value comes back at the end and the byte is
/// cleared); a set byte `+4` stops the motors (`00c7a380`) and clears itself. Without `source`
/// the local pose is turned into a model pose (`00cddcf0`), mapped onto the second pose
/// through the mapper of `[this + 0x2a4]` (`00ce3a00`) and, unless the mode is 1, turned
/// back into the first block (`00cddef0`); with `source` its `count` 0x30-byte poses are
/// copied into the first block. Mode 1 applies the second block (`00ce1ad0`). Mode 2 reinitialises
/// the rigid body controller first if the byte `+0x50` is set, and when the drive strength
/// `011afe64` is above 0 drives the controller to the pose (`00cdb7c0`) using a scratch array
/// of `count` floats, then applies the model pose and sets `+0x50` when some body's squared
/// vector `+0x1b0` is above the squared limit from the table at `010c7948`. Mode 3 starts the
/// motors and drives them to the pose, then clears the mode.
pub fn bhk_ragdoll_controller_update_ragdoll_rigid_bodies(e: &mut Engine, this: Ptr, source: u32) {
    let base = this.addr();
    let list = e.mem.u32(base + 0x48);
    let skeleton = e.mem.u32(list + 0x2c);
    let count = e.mem.i32(skeleton + 0x1c);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let rounded = (count as u32).wrapping_mul(0x30).wrapping_add(0x7f) & 0xffff_ff80;
    let first = e.mem.u32(router);
    e.mem.set_u32(router, first.wrapping_add(rounded));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let second = e.mem.u32(router);
    e.mem.set_u32(router, second.wrapping_add(rounded));
    let saved_mode = e.mem.u32(base + 0xc);
    if e.mem.u8(base + 0x44) != 0 {
        e.mem.set_u32(base + 0xc, 1);
    }
    if e.mem.u8(base + 4) != 0 {
        bhk_ragdoll_powered_constraint_controller_stop_motors(e, list);
        e.mem.set_u8(base + 4, 0);
    }
    if source == 0 {
        let parents = e.mem.u32(skeleton + 0xc);
        let local = e.mem.u32(skeleton + 0x24);
        e.call(
            LOCAL_TO_MODEL_POSE,
            &args![count as u32, parents, local, second],
        );
        let local = e.mem.u32(skeleton + 0x24);
        let synced_owner = e.mem.u32(base + 0x88);
        let synced = e
            .call(GET_SYNCED_POSE_MODEL_SPACE, &args![synced_owner])
            .u32();
        let synced_pose = e.mem.u32(synced);
        let owner = e.mem.u32(base + 0x2a4);
        let mapper = e.mem.u32(owner + 0x10);
        e.call(MAP_POSE, &args![mapper, synced_pose, local, second, 2u32]);
        if e.mem.u32(base + 0xc) != 1 {
            let parents = e.mem.u32(skeleton + 0xc);
            e.call(
                MODEL_TO_LOCAL_POSE,
                &args![count as u32, parents, second, first],
            );
        }
    } else if count > 0 {
        for index in 0..count as u32 {
            copy16(e, source + 0x30 * index, first + 0x30 * index);
            copy16(e, source + 0x30 * index + 0x10, first + 0x30 * index + 0x10);
            copy16(e, source + 0x30 * index + 0x20, first + 0x30 * index + 0x20);
        }
    }
    match e.mem.u32(base + 0xc) {
        1 => {
            e.call(APPLY_MODEL_POSE, &args![list, second, base + 0x10]);
        }
        2 => drive_rigid_bodies(e, this, list, count, first, second),
        3 => {
            bhk_ragdoll_powered_constraint_controller_start_motors(e, list);
            bhk_ragdoll_powered_constraint_controller_drive_to_pose(e, list, first);
            e.mem.set_u32(base + 0xc, 0);
        }
        _ => {}
    }
    if e.mem.u8(base + 0x44) != 0 {
        e.mem.set_u32(base + 0xc, saved_mode);
        e.mem.set_u8(base + 0x44, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, second);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, first);
}

/// Mode 2 of `00c7d030`: see there.
fn drive_rigid_bodies(e: &mut Engine, this: Ptr, list: u32, count: i32, first: u32, second: u32) {
    let base = this.addr();
    if e.mem.u8(base + 0x50) != 0 {
        let controller = e.mem.u32(base + 0x4c);
        e.call(RIGID_BODY_REINITIALIZE, &args![controller]);
        e.mem.set_u8(base + 0x50, 0);
    }
    let strength_bits = x87_quiet_bits(e.mem.u32(DRIVE_STRENGTH));
    let drives = 0.0 < f32::from_bits(strength_bits);
    if !drives {
        return;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let (scratch, flags) = if count == 0 {
        (0, 0x8000_0000)
    } else {
        let (scratch, size) = e.with_stack(4, |e, size_ptr| {
            let size = size_ptr.addr();
            e.mem.set_i32(size, count.wrapping_mul(4));
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.i32(size))
        });
        let capacity = size.wrapping_add((size >> 31) & 3) >> 2;
        (
            scratch,
            if capacity == 0 {
                0x8000_0000
            } else {
                capacity as u32
            },
        )
    };
    let controller = e.mem.u32(base + 0x4c);
    e.call(
        RIGID_BODY_DRIVE_TO_POSE,
        &args![controller, strength_bits, first, base + 0x10, scratch],
    );
    let mut exceeded = false;
    let mut index = 0u32;
    let bodies_object = e.mem.u32(list + 0x2c);
    if e.mem.i32(bodies_object + 0x1c) > 0 {
        loop {
            if exceeded {
                break;
            }
            let table = e.mem.u32(list + 0x20);
            let slot = e.mem.i32(table.wrapping_add(index.wrapping_mul(4)));
            if slot >= 0 {
                let bodies = e.mem.u32(list + 8);
                let body = e
                    .mem
                    .u32(bodies.wrapping_add((slot as u32).wrapping_mul(4)));
                if body != 0 {
                    let [x, y, z, _] = load4(e, body + 0x1b0);
                    let (x2, y2, z2) = (x * x, y * y, z * z);
                    let kind = u32::from(e.mem.u8(body + 0x19c));
                    let limit = x87_quiet(e.mem.f32(LIMIT_TABLE + 4 * kind));
                    let limit_squared = (f64::from(limit) * f64::from(limit)) as f32;
                    let sum = z2 + (y2 + x2);
                    exceeded = limit_squared < sum;
                }
            }
            index += 1;
            let bodies_object = e.mem.u32(list + 0x2c);
            if index as i32 >= e.mem.i32(bodies_object + 0x1c) {
                break;
            }
        }
    }
    if exceeded {
        e.call(APPLY_MODEL_POSE, &args![list, second, base + 0x10]);
        e.mem.set_u8(base + 0x50, 1);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if flags & 0x8000_0000 == 0 {
        e.vcall(allocator, 0x10, &args![scratch, (flags & 0x3fff_ffff) * 4]);
    }
}

/// Callees of `DoRagdollAnim` outside this file: `00c7aa60` (thiscall `(transform)`, no name in
/// the Ghidra database) and the owner transform fetch `0062c250` (thiscall on `[[world + 0x18]
/// + 0x68]`, `(record, world + 0x34)`).
const UPDATE_SYNC_TRANSFORM: u32 = 0x00c7_aa60;
const WORLD_TRANSFORM_FETCH: u32 = 0x0062_c250;

// Translated from 00c7d630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7d630 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollController::DoRagdollAnim` (Ghidra name), thiscall; the one stack argument is
/// never read. When the byte `+0x43` is set but none of the bytes `+0xb3`, `+0x222`, `+0x21e`
/// and `+0x28e` is, the bytes `+0x43` and `+0x41` are cleared. With the switch byte
/// `01267c28` and the byte `+0x41` set, the owner's world transform (`0062c250`, thiscall on
/// `[[world + 0x18] + 0x68]`, `(record, world + 0x34)`) is multiplied with the matrix at
/// `+0x60` (`0043f8d0`), nine dwords of the product go to the record and `1.0` to its `+0x30`,
/// and the record is converted into the Havok transform at `+0x10`. Unless the byte `+0x40` is
/// set, the local pose is taken (`00c79680`), the debug lines updated with 0 (`00c7cb50`) and
/// the optional foot IK (`01267c34` and one of the bytes `+0x222`/`+0x223`), grab IK
/// (`01267c40` and `+0x21e`) and the `00c7aa60` pass (`01267c4c` and `+0xb3`) run on the
/// transform at `+0x10`. Two pose blocks (`count * 0x30` bytes rounded up to 128, count from
/// the constraint list's `+0x2c` object) are then taken from the allocation cursor; with
/// `01267c64` and the byte `+0x256` the pose matching (`00c7ba50`) runs and, when it sets its
/// flag, the first block is the pose source for `UpdateRagdollRigidBodies` (otherwise 0). The
/// debug lines are updated with 1, then, unless `+0x40` is set, the feedback (`00c7c870`, with
/// `01267c58` and `+0x28e`), the debug lines with 2 and `SetSGPoseLocalSpace` (`00c79a50`) run.
/// The cursor goes back to the first block.
pub fn bhk_ragdoll_controller_do_ragdoll_anim(e: &mut Engine, this: Ptr, _unused: u32) {
    let base = this.addr();
    if e.mem.u8(base + 0x43) != 0
        && e.mem.u8(base + 0xb3) == 0
        && e.mem.u8(base + 0x222) == 0
        && e.mem.u8(base + 0x21e) == 0
        && e.mem.u8(base + 0x28e) == 0
    {
        e.mem.set_u8(base + 0x43, 0);
        e.mem.set_u8(base + 0x41, 0);
    }
    if e.mem.u8(0x0126_7c28) == 0 || e.mem.u8(base + 0x41) == 0 {
        return;
    }
    let world = e.mem.u32(base + 0x58);
    let owner = e.mem.u32(world + 0x18);
    let transform = base + 0x10;
    e.with_stack(0x34, |e, record| {
        let record = record.addr();
        e.call(
            WORLD_TRANSFORM_FETCH,
            &args![owner + 0x68, record, world + 0x34],
        );
        e.with_stack(0x24, |e, result| {
            let product = e
                .call(MATRIX_MULTIPLY, &args![base + 0x60, result, record])
                .u32();
            for index in 0..9 {
                let value = e.mem.u32(product + 4 * index);
                e.mem.set_u32(record + 4 * index, value);
            }
        });
        e.mem.set_f32(record + 0x30, 1.0);
        ni2hk_ov2(e, Ptr::new(transform), Ptr::new(record));
    });
    if e.mem.u8(base + 0x40) == 0 {
        bhk_ragdoll_controller_get_sg_pose_local_space(e, this);
        bhk_ragdoll_controller_update_debug_lines(e, this, 0);
        if e.mem.u8(0x0126_7c34) != 0
            && (e.mem.u8(base + 0x222) != 0 || e.mem.u8(base + 0x223) != 0)
        {
            bhk_ragdoll_controller_do_foot_ik(e, this, transform);
        }
        if e.mem.u8(0x0126_7c40) != 0 && e.mem.u8(base + 0x21e) != 0 {
            bhk_ragdoll_controller_do_grab_ik(e, this, transform);
        }
        if e.mem.u8(0x0126_7c4c) != 0 && e.mem.u8(base + 0xb3) != 0 {
            e.call(UPDATE_SYNC_TRANSFORM, &args![base, transform]);
        }
    }
    let list = e.mem.u32(base + 0x48);
    let skeleton = e.mem.u32(list + 0x2c);
    let count = e.mem.u32(skeleton + 0x1c);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let rounded = count.wrapping_mul(0x30).wrapping_add(0x7f) & 0xffff_ff80;
    let cursor = e.mem.u32(router);
    e.mem.set_u32(router, cursor.wrapping_add(rounded));
    let source = if e.mem.u8(0x0126_7c64) != 0 && e.mem.u8(base + 0x256) != 0 {
        e.with_stack(4, |e, flag| {
            let flag = flag.addr();
            e.mem.set_u8(flag, 0);
            bhk_ragdoll_controller_do_pose_matching(e, this, transform, cursor, flag);
            if e.mem.u8(flag) != 0 {
                cursor
            } else {
                0
            }
        })
    } else {
        0
    };
    bhk_ragdoll_controller_update_ragdoll_rigid_bodies(e, this, source);
    bhk_ragdoll_controller_update_debug_lines(e, this, 1);
    if e.mem.u8(base + 0x40) == 0 {
        if e.mem.u8(0x0126_7c58) != 0 && e.mem.u8(base + 0x28e) != 0 {
            bhk_ragdoll_controller_do_ragdoll_feedback(e, this);
        }
        bhk_ragdoll_controller_update_debug_lines(e, this, 2);
        bhk_ragdoll_controller_set_sg_pose_local_space(e, this);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, cursor);
}

/// Four packed floats of an XMM register.
type Lanes = [f32; 4];

/// `MULPS`.
fn mulps(a: Lanes, b: Lanes) -> Lanes {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

/// `ADDPS`.
fn addps(a: Lanes, b: Lanes) -> Lanes {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

/// `SUBPS`.
fn subps(a: Lanes, b: Lanes) -> Lanes {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// `SHUFPS destination, source, selector`.
fn shufps(destination: Lanes, source: Lanes, selector: u8) -> Lanes {
    let pick = |lanes: Lanes, shift: u8| lanes[usize::from((selector >> shift) & 3)];
    [
        pick(destination, 0),
        pick(destination, 2),
        pick(source, 4),
        pick(source, 6),
    ]
}

/// `UNPCKHPS destination, source`.
fn unpckhps(destination: Lanes, source: Lanes) -> Lanes {
    [destination[2], source[2], destination[3], source[3]]
}

/// `XORPS` with a mask register.
fn xorps(a: Lanes, mask: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| f32::from_bits(a[lane].to_bits() ^ mask[lane].to_bits()))
}

/// `ANDPS` with a mask register.
fn andps(a: Lanes, mask: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| f32::from_bits(a[lane].to_bits() & mask[lane].to_bits()))
}

/// `RCPPS`: the host CPU's per-lane reciprocal estimate on x86.
#[cfg(target_arch = "x86_64")]
fn rcpps(a: Lanes) -> Lanes {
    use std::arch::x86_64::{_mm_loadu_ps, _mm_rcp_ps, _mm_storeu_ps};
    let mut out = [0.0f32; 4];
    // SAFETY: SSE is part of the x86_64 baseline and both arrays hold four floats.
    unsafe { _mm_storeu_ps(out.as_mut_ptr(), _mm_rcp_ps(_mm_loadu_ps(a.as_ptr()))) };
    out
}

#[cfg(not(target_arch = "x86_64"))]
fn rcpps(a: Lanes) -> Lanes {
    a.map(|lane| 1.0 / lane)
}

/// The dot product `x2 + (x1 + x0)` of the three lowest lanes of a product (`SHUFPS`/`ADDSS`).
fn dot_xyz(product: Lanes) -> f32 {
    product[2] + (product[1] + product[0])
}

/// Rotation of `vector` by the quaternion `quaternion` the way the pose code does it: with
/// `cross` the product of `vector` and `quaternion` in the exe's lane order, the result
/// `cross * w + ((w * w + -0.5) * vector + dot * quaternion)` doubled. `negated_cross` selects
/// the second pose pass where the subtraction runs the other way round.
fn rotate_lanes(vector: Lanes, quaternion: Lanes, half: Lanes, negated_cross: bool) -> Lanes {
    let dot = dot_xyz(mulps(vector, quaternion));
    let forward = mulps(
        shufps(vector, vector, 0xc9),
        shufps(quaternion, quaternion, 0xd2),
    );
    let backward = mulps(
        shufps(vector, vector, 0xd2),
        shufps(quaternion, quaternion, 0xc9),
    );
    let cross = if negated_cross {
        subps(backward, forward)
    } else {
        subps(forward, backward)
    };
    let w = shufps(quaternion, quaternion, 0xff);
    let by_w = mulps(cross, w);
    let scaled = mulps(addps(mulps(w, w), half), vector);
    let along = mulps([dot; 4], quaternion);
    let sum = addps(scaled, along);
    let total = addps(by_w, sum);
    addps(total, total)
}

/// Reciprocal of the scale lanes `(x, y, z, 1.0)` refined once, lane 3 cleared:
/// `(2r - (r * x) * r) & mask` with `r = RCPPS(x)`.
fn reciprocal_scale(scale: Lanes, ones: Lanes, mask: Lanes) -> Lanes {
    let unpacked = unpckhps(scale, ones);
    let divisor = shufps(scale, unpacked, 0xc4);
    let estimate = rcpps(divisor);
    let product = mulps(mulps(estimate, divisor), estimate);
    andps(subps(addps(estimate, estimate), product), mask)
}

/// The helpers `GetSGPoseLocalSpace` calls: `hkaPose::calculateBoneModelSpace` (Ghidra name,
/// `00cda180`, thiscall on the pose, `(bone)`, answers the transform), `hkaPose::syncModelSpace`
/// (Ghidra name, `00cda340`, thiscall), `hkaPose::getSyncedPoseLocalSpace` (Ghidra name,
/// `00cdb290`, thiscall, answers a pointer to the pointer of the local transforms),
/// `hkQuaternion::setMul` (Ghidra name, `00c66320`, thiscall on the result, `(a, b)`) and
/// `hkString::memCpy` (Ghidra name, `00cb1260`, cdecl `(destination, source, size)`).
const CALCULATE_BONE_MODEL_SPACE: u32 = 0x00cd_a180;
const SYNC_MODEL_SPACE: u32 = 0x00cd_a340;
const GET_SYNCED_POSE_LOCAL_SPACE: u32 = 0x00cd_b290;
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const MEMCPY: u32 = 0x00cb_1260;
/// The SSE constants of the pose code: four times -0.5 (`010c71e0`), four times 1.0
/// (`010c7170`), four sign bits (`010c44d0`), the sign bits of the three lowest lanes
/// (`010c4c00`) and the all-ones mask of the three lowest lanes (`010c4c10`).
const NEG_HALF_LANES: u32 = 0x010c_71e0;
const ONE_LANES: u32 = 0x010c_7170;
const SIGN_LANES: u32 = 0x010c_44d0;
const SIGN_XYZ_LANES: u32 = 0x010c_4c00;
const MASK_XYZ_LANES: u32 = 0x010c_4c10;

/// `hkaPose::syncLocalSpace` (Ghidra name, `00cda690`, thiscall on the pose).
const SYNC_LOCAL_SPACE: u32 = 0x00cd_a690;

/// The five SSE constant vectors the pose conversions read from the exe.
struct PoseConstants {
    half: Lanes,
    ones: Lanes,
    sign: Lanes,
    sign_xyz: Lanes,
    mask_xyz: Lanes,
}

fn load_pose_constants(e: &Engine) -> PoseConstants {
    PoseConstants {
        half: load4(e, NEG_HALF_LANES),
        ones: load4(e, ONE_LANES),
        sign: load4(e, SIGN_LANES),
        sign_xyz: load4(e, SIGN_XYZ_LANES),
        mask_xyz: load4(e, MASK_XYZ_LANES),
    }
}

/// The transform both pose conversions apply to a bone whose parent is `parent` (not `-1`):
/// the parent's transform is the local entry (`pose + 0x10` array), or, when the parent's flag
/// word has bit 2, `hkaPose::calculateBoneModelSpace` (`00cda180`). The translation of the
/// bone's local transform is rotated by the conjugate of the parent's rotation and added to the
/// negated doubled rotation of the parent's translation; the rotation is `00c66320` of the
/// conjugate and the local rotation; the scale is the local scale times the refined reciprocal
/// of the parent's scale. The results go to the entry at `offset` of the array at `pose + 4`.
fn compose_with_parent(
    e: &mut Engine,
    pose: u32,
    parent: u16,
    constants: &PoseConstants,
    local: u32,
    offset: u32,
) {
    let flags = e.mem.u32(pose + 0x1c);
    let parent_index = i32::from(parent as i16) as u32;
    let transform = if e.mem.u8(flags.wrapping_add(parent_index.wrapping_mul(4))) & 2 == 0 {
        local.wrapping_add(parent_index.wrapping_mul(0x30))
    } else {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, parent_index])
            .u32()
    };
    let translation = load4(e, transform);
    let rotation = load4(e, transform + 0x10);
    let parent_scale = load4(e, transform + 0x20);
    let doubled = rotate_lanes(translation, rotation, constants.half, false);
    let negated = xorps(doubled, constants.sign);
    let rotation = load4(e, transform + 0x10);
    let conjugate = xorps(rotation, constants.sign_xyz);
    let output = e.mem.u32(pose + 4) + offset;
    let inverse = reciprocal_scale(parent_scale, constants.ones, constants.mask_xyz);
    let local_translation = load4(e, local + offset);
    let result = rotate_lanes(local_translation, conjugate, constants.half, true);
    store4(e, output, addps(result, negated));
    e.with_stack(0x10, |e, quaternion| {
        let quaternion = quaternion.addr();
        store4(e, quaternion, conjugate);
        e.call(
            QUATERNION_SET_MUL,
            &args![output + 0x10, quaternion, local + offset + 0x10],
        );
    });
    let local_scale = load4(e, local + offset + 0x20);
    store4(e, output + 0x20, mulps(local_scale, inverse));
}

// Translated from 00c79680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c79680 - the /GS cookie check (00ec408c) is not modeled.
/// `bhkRagdollController::GetSGPoseLocalSpace` (Ghidra name), register `ECX` (fastcall): for
/// each of the `count` bodies (count from `[[this + 0x2a4] + 0xc] + 0x1c`) the Havok transform
/// of the body at `[this + 0xa4]` is converted back (`00c74dd0`, from the body's `+0x34`) and its
/// first four lanes scaled by the float at `[[this + 0x58] + 0x98]` (loaded with `FLD`/`FSTP`).
/// In the pose at `[this + 0x88]` (skeleton at `+0`, parents shorts at skeleton `+0xc`, count at
/// `+0x1c`, flag words at `+0x1c` (1, 2, 4), local transforms at `+0x10`, output transforms at
/// `+4`, dirty byte at `+0x28`) the bone's flag gets bit 4, then every later bone whose parent
/// is flagged 4 and which itself is flagged 1 is composed with its parent (see
/// `compose_with_parent`; a parent of `-1` would copy the local entry, but such bones were
/// already skipped by the same test). Such bones lose flag 1 and gain flag 4; the flagged-4 bones after the current
/// one become flagged 2; the scaled transform is stored for the bone, its flag becomes 2 and
/// the dirty byte is cleared. The pose is synchronised (`00cda340`) and the local transforms
/// (`00cdb290`) copied to `[this + 0x94]` (`count * 0x30` bytes).
pub fn bhk_ragdoll_controller_get_sg_pose_local_space(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let world = e.mem.u32(base + 0x58);
    let scale = f32::from_bits(x87_quiet_bits(e.mem.u32(world + 0x98)));
    let source = e.mem.u32(base + 0x2a4);
    let source = e.mem.u32(source + 0xc);
    let count = e.mem.i32(source + 0x1c);
    let constants = load_pose_constants(e);
    if count > 0 {
        for index in 0..count as u32 {
            let bodies = e.mem.u32(base + 0xa4);
            let body = e.mem.u32(bodies.wrapping_add(4 * index));
            let (converted, rest_one, rest_two) = e.with_stack(0x30, |e, record| {
                let record = record.addr();
                ni2hk_ov2(e, Ptr::new(record), Ptr::new(body + 0x34));
                (
                    load4(e, record),
                    load4(e, record + 0x10),
                    load4(e, record + 0x20),
                )
            });
            let pose = e.mem.u32(base + 0x88);
            let skeleton = e.mem.u32(pose);
            let bones = e.mem.i32(skeleton + 0x1c);
            let flags = e.mem.u32(pose + 0x1c);
            let word = e.mem.u32(flags + 4 * index) | 4;
            e.mem.set_u32(flags + 4 * index, word);
            let first = mulps([scale; 4], converted);
            let mut bone = index + 1;
            if (bone as i32) < bones {
                let mut offset = 0x30 * index + 0x30;
                loop {
                    let skeleton = e.mem.u32(pose);
                    let parents = e.mem.u32(skeleton + 0xc);
                    let parent = e.mem.u16(parents + 2 * bone);
                    let flags = e.mem.u32(pose + 0x1c);
                    let parent_index = i32::from(parent as i16) as u32;
                    if parent != 0xffff
                        && e.mem.u8(flags.wrapping_add(parent_index.wrapping_mul(4))) & 4 != 0
                    {
                        if e.mem.u8(flags + 4 * bone) & 1 != 0 {
                            let local = e.mem.u32(pose + 0x10);
                            if parent == 0xffff {
                                let output = e.mem.u32(pose + 4) + offset;
                                copy16(e, local + offset, output);
                                copy16(e, local + offset + 0x10, output + 0x10);
                                copy16(e, local + offset + 0x20, output + 0x20);
                            } else {
                                compose_with_parent(e, pose, parent, &constants, local, offset);
                            }
                            let flags = e.mem.u32(pose + 0x1c);
                            let word = e.mem.u32(flags + 4 * bone) & 0xffff_fffe;
                            e.mem.set_u32(flags + 4 * bone, word);
                        }
                        let flags = e.mem.u32(pose + 0x1c);
                        let word = e.mem.u32(flags + 4 * bone) | 4;
                        e.mem.set_u32(flags + 4 * bone, word);
                        e.mem.set_u8(pose + 0x28, 0);
                    }
                    offset += 0x30;
                    bone += 1;
                    if (bone as i32) >= bones {
                        break;
                    }
                }
            }
            let mut later = index + 1;
            while (later as i32) < bones {
                let flags = e.mem.u32(pose + 0x1c);
                let word = e.mem.u32(flags + 4 * later);
                if word & 4 != 0 {
                    e.mem.set_u32(flags + 4 * later, (word & 0xffff_fffb) | 2);
                }
                later += 1;
            }
            let output = e.mem.u32(pose + 4) + 0x30 * index;
            store4(e, output, first);
            store4(e, output + 0x10, rest_one);
            store4(e, output + 0x20, rest_two);
            let flags = e.mem.u32(pose + 0x1c);
            e.mem.set_u32(flags + 4 * index, 2);
            e.mem.set_u8(pose + 0x28, 0);
        }
    }
    let pose = e.mem.u32(base + 0x88);
    e.call(SYNC_MODEL_SPACE, &args![pose]);
    let pose = e.mem.u32(base + 0x88);
    let synced = e.call(GET_SYNCED_POSE_LOCAL_SPACE, &args![pose]).u32();
    let transforms = e.mem.u32(synced);
    let destination = e.mem.u32(base + 0x94);
    e.call(
        MEMCPY,
        &args![destination, transforms, (count as u32).wrapping_mul(0x30)],
    );
}

// Translated from 00c79a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c79a50 - the /GS cookie check (00ec408c) is not modeled.
/// `bhkRagdollController::SetSGPoseLocalSpace` (Ghidra name), register `ECX` (fastcall): the
/// pose at `[this + 0x88]` is synchronised in local space (`hkaPose::syncLocalSpace`,
/// `00cda690`) and the inverse of the world scale `1.0 / [[this + 0x58] + 0x98]` computed with
/// the x87 unit. Then for every body (count from `[[this + 0x2a4] + 0xc] + 0x1c`, bodies at
/// `[this + 0xa4]`) whose bone has flag bit 1 the bone's output entry is composed with its
/// parent (`compose_with_parent`; a bone without parent copies its local entry) and loses the
/// flag. Whatever the flag, the output entry is then converted to a transform (`00c74d50`): the
/// rotation (nine dwords) goes to the body's `+0x34`, the translation times the inverse scale
/// to `+0x58`/`+0x5c`/`+0x60` and, when the object at the body's `+0x1c` exists, its virtual
/// function `+0x3c` answers a record whose byte `+0x14` is set.
pub fn bhk_ragdoll_controller_set_sg_pose_local_space(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let source = e.mem.u32(base + 0x2a4);
    let source = e.mem.u32(source + 0xc);
    let count = e.mem.i32(source + 0x1c);
    let pose = e.mem.u32(base + 0x88);
    e.call(SYNC_LOCAL_SPACE, &args![pose]);
    let world = e.mem.u32(base + 0x58);
    let scale = x87_quiet(e.mem.f32(world + 0x98));
    let inverse_scale = (1.0f64 / f64::from(scale)) as f32;
    if count <= 0 {
        return;
    }
    let constants = load_pose_constants(e);
    for index in 0..count as u32 {
        let offset = 0x30 * index;
        let bodies = e.mem.u32(base + 0xa4);
        let body = e.mem.u32(bodies + 4 * index);
        let pose = e.mem.u32(base + 0x88);
        let flags = e.mem.u32(pose + 0x1c);
        if e.mem.u8(flags + 4 * index) & 1 != 0 {
            let skeleton = e.mem.u32(pose);
            let parents = e.mem.u32(skeleton + 0xc);
            let parent = e.mem.u16(parents + 2 * index);
            let local = e.mem.u32(pose + 0x10);
            if parent == 0xffff {
                let output = e.mem.u32(pose + 4) + offset;
                copy16(e, local + offset, output);
                copy16(e, local + offset + 0x10, output + 0x10);
                copy16(e, local + offset + 0x20, output + 0x20);
            } else {
                compose_with_parent(e, pose, parent, &constants, local, offset);
            }
            let flags = e.mem.u32(pose + 0x1c);
            let word = e.mem.u32(flags + 4 * index) & 0xffff_fffe;
            e.mem.set_u32(flags + 4 * index, word);
        }
        let output = e.mem.u32(pose + 4) + offset;
        e.with_stack(0x34, |e, transform| {
            let transform = transform.addr();
            fn_00c74d50(e, Ptr::new(transform), Ptr::new(output));
            let x = x87_quiet(e.mem.f32(transform + 0x24));
            let y = x87_quiet(e.mem.f32(transform + 0x28));
            let z = x87_quiet(e.mem.f32(transform + 0x2c));
            let factor = f64::from(inverse_scale);
            e.mem.set_f32(body + 0x58, (f64::from(x) * factor) as f32);
            e.mem.set_f32(body + 0x5c, (f64::from(y) * factor) as f32);
            e.mem.set_f32(body + 0x60, (factor * f64::from(z)) as f32);
            for word in 0..9 {
                let value = e.mem.u32(transform + 4 * word);
                e.mem.set_u32(body + 0x34 + 4 * word, value);
            }
        });
        let object = e.mem.u32(body + 0x1c);
        if object != 0 {
            let record = e.vcall(object, 0x3c, &args![]).u32();
            if record != 0 {
                e.mem.set_u8(record + 0x14, 1);
            }
        }
    }
}

/// The helpers `DoRagdollFeedback` calls: `hkQuaternion::setSlerp` (Ghidra name `setSlerp`,
/// `00cb2850`, thiscall on the result, `(a, b, t)`), `hkaPose::accessSyncedPoseModelSpace`
/// (Ghidra name, `00cda4b0`, thiscall on the pose, answers a pointer to the pointer of the model
/// transforms) and `00cd9fe0` (thiscall on the pose, no name in the Ghidra database).
const SET_SLERP: u32 = 0x00cb_2850;
const ACCESS_SYNCED_POSE_MODEL_SPACE: u32 = 0x00cd_a4b0;
const POSE_MODEL_SPACE_CHANGED: u32 = 0x00cd_9fe0;

// Translated from 00c7c870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7c870 - the FS:[0] exception-unwinding frame and the /GS cookie
// check (00ec408c) are not modeled.
/// `bhkRagdollController::DoRagdollFeedback` (Ghidra name), register `ECX`: when the 64-bit
/// timer (`00aa4d80`) minus the start `+0x2a8` is above the signed limit `+0x2b0` the feedback
/// is switched off (`00c7b6a0` with 0) and nothing else happens. Otherwise three blocks of
/// `count * 0x30` bytes rounded up to 128 are taken from the allocation cursor (count from
/// the constraint list's `+0x2c` object; the second block is given back and taken again, so
/// the second and third blocks follow each other): the list's pose model space for the
/// transform at `+0x10` (`00ce18f0`) is turned into a local pose in the first block
/// (`00cddef0`); the skeleton's local pose (`+0x24`) is turned into model space in the third block
/// (`00cddcf0`), mapped with the synced model pose of the pose at `+0x88` through the mapper
/// `[[this + 0x2a4] + 0x10]` (`00ce3a00`) and turned back into local space in the second
/// block. For each of the `[[this + 0x48] + 0xc]` bodies listed (as a 16-bit value, searched
/// linearly in the array at `[[this + 0x2a4] + 0x18]` `+0x44`, size `+0x48`) the rotation of
/// the second block's entry is replaced by the spherical interpolation (`00cb2850`) from the
/// first block's rotation with the weight `+0x290` (loaded with `FLD`/`FSTP`). The second
/// block is converted to model space (third block), mapped through the mapper
/// `[[this + 0x2a4] + 0x14]` onto the model pose of the pose (`00cda4b0`) with the synced
/// local pose (`00cdb290`) and the pose is told it changed (`00cd9fe0`). The cursor goes back
/// to the first block.
pub fn bhk_ragdoll_controller_do_ragdoll_feedback(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let now = e.call(GET_TIMER, &args![]).u64() as i64;
    let started =
        (u64::from(e.mem.u32(base + 0x2ac)) << 32 | u64::from(e.mem.u32(base + 0x2a8))) as i64;
    let limit =
        (u64::from(e.mem.u32(base + 0x2b4)) << 32 | u64::from(e.mem.u32(base + 0x2b0))) as i64;
    if now.wrapping_sub(started) > limit {
        bhk_ragdoll_controller_set_ragdoll_feedback_active(e, this, 0);
        return;
    }
    let list = e.mem.u32(base + 0x48);
    let skeleton = e.mem.u32(list + 0x2c);
    let count = e.mem.u32(skeleton + 0x1c);
    let rounded = count.wrapping_mul(0x30).wrapping_add(0x7f) & 0xffff_ff80;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let first = e.mem.u32(router);
    e.mem.set_u32(router, first.wrapping_add(rounded));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let second = e.mem.u32(router);
    e.mem.set_u32(router, second.wrapping_add(rounded));
    e.call(GET_POSE_MODEL_SPACE, &args![list, second, base + 0x10]);
    let parents = e.mem.u32(skeleton + 0xc);
    e.call(MODEL_TO_LOCAL_POSE, &args![count, parents, second, first]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, second);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let first_pose_block = e.mem.u32(router);
    e.mem
        .set_u32(router, first_pose_block.wrapping_add(rounded));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let model_block = e.mem.u32(router);
    e.mem.set_u32(router, model_block.wrapping_add(rounded));
    let parents = e.mem.u32(skeleton + 0xc);
    let skeleton_local = e.mem.u32(skeleton + 0x24);
    e.call(
        LOCAL_TO_MODEL_POSE,
        &args![count, parents, skeleton_local, model_block],
    );
    let skeleton_local = e.mem.u32(skeleton + 0x24);
    let pose = e.mem.u32(base + 0x88);
    let synced = e.call(GET_SYNCED_POSE_MODEL_SPACE, &args![pose]).u32();
    let owner = e.mem.u32(base + 0x2a4);
    let mapper = e.mem.u32(owner + 0x10);
    let synced_pose = e.mem.u32(synced);
    e.call(
        MAP_POSE,
        &args![mapper, synced_pose, skeleton_local, model_block, 2u32],
    );
    let parents = e.mem.u32(skeleton + 0xc);
    e.call(
        MODEL_TO_LOCAL_POSE,
        &args![count, parents, model_block, first_pose_block],
    );
    let list = e.mem.u32(base + 0x48);
    let bodies = e.mem.i32(list + 0xc);
    for body in 0..bodies.max(0) {
        let owner = e.mem.u32(base + 0x2a4);
        let table = e.mem.u32(owner + 0x18);
        let size = e.mem.u32(table + 0x48);
        let mut found = None;
        let mut index = 0u32;
        while index < size {
            let data = e.mem.u32(table + 0x44);
            if data != 0 && e.mem.u16(data + 2 * index) == body as u16 {
                found = Some(index);
                break;
            }
            index += 1;
        }
        if let Some(index) = found {
            if index != 0xffff_ffff {
                let weight = x87_quiet_bits(e.mem.u32(base + 0x290));
                let rotation = 0x10 + 0x30 * body as u32;
                let from = first + rotation;
                let to = first_pose_block + rotation;
                e.with_stack(0x10, |e, result| {
                    let result = result.addr();
                    e.call(SET_SLERP, &args![result, from, to, weight]);
                    copy16(e, result, to);
                });
            }
        }
    }
    let parents = e.mem.u32(skeleton + 0xc);
    e.call(
        LOCAL_TO_MODEL_POSE,
        &args![count, parents, first_pose_block, model_block],
    );
    let pose = e.mem.u32(base + 0x88);
    let access = e.call(ACCESS_SYNCED_POSE_MODEL_SPACE, &args![pose]).u32();
    let model_pose = e.mem.u32(access);
    let pose = e.mem.u32(base + 0x88);
    let synced = e.call(GET_SYNCED_POSE_LOCAL_SPACE, &args![pose]).u32();
    let local_pose = e.mem.u32(synced);
    let owner = e.mem.u32(base + 0x2a4);
    let mapper = e.mem.u32(owner + 0x14);
    e.call(
        MAP_POSE,
        &args![mapper, model_block, local_pose, model_pose, 2u32],
    );
    let pose = e.mem.u32(base + 0x88);
    e.call(POSE_MODEL_SPACE_CHANGED, &args![pose]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, model_block);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, first_pose_block);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, first);
}

/// The data the debug lines read from the exe: the three dwords (`011f426c`) a fresh line
/// object's vertex colour starts from, the float `01016264` (the default colour weight), and
/// the four colour dwords `011a9c10` used for the bodies whose motion type is 4 or 5.
const LINE_BASE_COLOR: u32 = 0x011f_426c;
const LINE_DEFAULT_WEIGHT: u32 = 0x0101_6264;
const LINE_ACTIVE_COLOR: u32 = 0x011a_9c10;

/// The part of `UpdateDebugLines` it repeats for every line object (`NiAVObject`, virtual slot
/// `0x78` sets up its data): sets bit `0x800` of the flag word `+0x30`; without data at `+0x20`
/// takes 0x10 bytes from `MemoryManager::Allocate`, hands them to the object's virtual slot
/// `0x78`, copies the three dwords at `011f426c` to the data and clears its fourth float; the
/// float at data `+0xc` becomes 1.0.
fn prepare_line_object(e: &mut Engine, object: u32) {
    let flags = e.mem.u32(object + 0x30);
    e.mem.set_u32(object + 0x30, flags | 0x800);
    if e.mem.u32(object + 0x20) == 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        e.vcall(object, 0x78, &args![block]);
        let data = e.mem.u32(object + 0x20);
        let first = e.mem.u32(LINE_BASE_COLOR);
        e.mem.set_u32(data, first);
        let second = e.mem.u32(LINE_BASE_COLOR + 4);
        e.mem.set_u32(data + 4, second);
        let third = e.mem.u32(LINE_BASE_COLOR + 8);
        e.mem.set_u32(data + 0xc, 0);
        e.mem.set_u32(data + 8, third);
    }
    let data = e.mem.u32(object + 0x20);
    e.mem.set_f32(data + 0xc, 1.0);
}

// Translated from 00c7cb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7cb50 - the FS:[0] exception-unwinding frame and the /GS cookie
// check (00ec408c) are not modeled.
/// `bhkRagdollController::UpdateDebugLines` (Ghidra name), thiscall `(stage)`: nothing happens
/// without the object at `+0x84`. Its entry `stage` of the array at `+0xa0` answers (virtual slot
/// `0xc`) the object whose own array at `+0xa0` holds one line object per listed bone; when
/// the entry's flag bit 0 is set that is all. The objects at `+0x58`, `+0x84` and the answered
/// one are prepared (`prepare_line_object`). The bone count comes from the skeleton at
/// `[[this + 0x2a4] + 8]` (stage 1) or `+0xc`; a block of that many 0x30-byte transforms is
/// taken from the allocator slot `0xc` (its capacity is the answered size divided by 0x30), the
/// list's pose model space for the transform at `+0x10` is written into it (`00ce18f0`) and
/// the poses are that block (stage 1) or the synced model space of the pose at `+0x88`
/// (`00cda4b0`). For every bone whose parent is not `-1` and which has a line object, the
/// object is prepared and, for stage 1, coloured: the default colour `(0, w, w, w)` with `w`
/// from `01016264`, or the four dwords at `011a9c10` when the list entry of the bone (through
/// `00c76220`) is of motion type 4 or 5 (`00c761f0`). The parent's translation lanes and the
/// bone's, both multiplied by `1.0 / [[this + 0x58] + 0x98]`, are converted with `00458620` into
/// the two vertices of the line. The block is freed at the end.
pub fn bhk_ragdoll_controller_update_debug_lines(e: &mut Engine, this: Ptr, stage: u32) {
    let base = this.addr();
    let world = e.mem.u32(base + 0x58);
    let scale = x87_quiet(e.mem.f32(world + 0x98));
    let inverse_scale = (1.0f64 / f64::from(scale)) as f32;
    let owner = e.mem.u32(base + 0x84);
    if owner == 0 {
        return;
    }
    let entries = e.mem.u32(owner + 0xa0);
    let entry = e.mem.u32(entries.wrapping_add(stage.wrapping_mul(4)));
    let lines = e.vcall(entry, 0xc, &args![]).u32();
    if e.mem.u8(entry + 0x30) & 1 != 0 {
        return;
    }
    let first = e.mem.u32(base + 0x58);
    prepare_line_object(e, first);
    let second = e.mem.u32(base + 0x84);
    prepare_line_object(e, second);
    prepare_line_object(e, lines);
    let mappers = e.mem.u32(base + 0x2a4);
    let skeleton = if stage == 1 {
        e.mem.u32(mappers + 8)
    } else {
        e.mem.u32(mappers + 0xc)
    };
    let count = e.mem.u32(skeleton + 0x1c);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let (block, flags) = if count != 0 {
        let (block, bytes) = e.with_stack(4, |e, requested| {
            let requested = requested.addr();
            e.mem.set_u32(requested, count.wrapping_mul(0x30));
            let block = e.vcall(allocator, 0xc, &args![requested]).u32();
            (block, e.mem.i32(requested))
        });
        let capacity = bytes / 0x30;
        (
            block,
            if capacity == 0 {
                0x8000_0000
            } else {
                capacity as u32
            },
        )
    } else {
        (0, 0x8000_0000)
    };
    let list = e.mem.u32(base + 0x48);
    e.call(GET_POSE_MODEL_SPACE, &args![list, block, base + 0x10]);
    let access = if stage == 1 {
        0
    } else {
        let pose = e.mem.u32(base + 0x88);
        e.call(ACCESS_SYNCED_POSE_MODEL_SPACE, &args![pose]).u32()
    };
    let mut line_offset = 0u32;
    for bone in 0..count {
        let parents = e.mem.u32(skeleton + 0xc);
        let parent = e.mem.i16(parents + 2 * bone);
        if parent == -1 {
            continue;
        }
        let line_objects = e.mem.u32(lines + 0xa0);
        let line = e.mem.u32(line_objects.wrapping_add(line_offset));
        if line == 0 {
            continue;
        }
        prepare_line_object(e, line);
        if stage == 1 {
            let weight = x87_quiet_bits(e.mem.u32(LINE_DEFAULT_WEIGHT));
            let mut color = [0, weight, weight, weight];
            let list = e.mem.u32(base + 0x48);
            let table = e.mem.u32(list + 0x20);
            let slot = e.mem.i32(table.wrapping_add(bone.wrapping_mul(4)));
            let entries = e.mem.u32(list + 8);
            if slot >= 0
                && e.mem
                    .u32(entries.wrapping_add((slot as u32).wrapping_mul(4)))
                    != 0
            {
                let body = fn_00c76220(e, Ptr::new(list), bone);
                let active = e.with_stack(4, |e, out| {
                    let out = out.addr();
                    fn_00c761f0(e, Ptr::new(body), Ptr::new(out));
                    e.mem.u8(out) != 0
                });
                if active {
                    color = [0, 4, 8, 12].map(|offset| e.mem.u32(LINE_ACTIVE_COLOR + offset));
                }
            }
            let geometry = e.mem.u32(line + 0xb8);
            let target = e.mem.u32(geometry + 0x28);
            for (lane, value) in color.into_iter().enumerate() {
                e.mem.set_u32(target + 4 * lane as u32, value);
            }
            let geometry = e.mem.u32(line + 0xb8);
            let target = e.mem.u32(geometry + 0x28);
            for (lane, value) in color.into_iter().enumerate() {
                e.mem.set_u32(target + 0x10 + 4 * lane as u32, value);
            }
        }
        let poses = if stage == 1 { block } else { e.mem.u32(access) };
        let parents = e.mem.u32(skeleton + 0xc);
        let parent = i32::from(e.mem.i16(parents + 2 * bone));
        let parent_offset = (parent as u32).wrapping_mul(0x30);
        let parent_lanes = load4(e, poses.wrapping_add(parent_offset));
        let from = mulps([inverse_scale; 4], parent_lanes);
        let own_lanes = load4(e, poses.wrapping_add(0x30 * bone));
        let to = mulps(own_lanes, [inverse_scale; 4]);
        let vertices = e.with_stack(0x40, |e, scratch| {
            let scratch = scratch.addr();
            store4(e, scratch, from);
            store4(e, scratch + 0x10, to);
            e.call(POINT_COPY, &args![scratch + 0x20, scratch]);
            e.call(POINT_COPY, &args![scratch + 0x2c, scratch + 0x10]);
            [0, 4, 8, 12, 16, 20].map(|offset| e.mem.u32(scratch + 0x20 + offset))
        });
        let geometry = e.mem.u32(line + 0xb8);
        let data = e.mem.u32(geometry + 0x20);
        e.mem.set_u32(data, vertices[0]);
        e.mem.set_u32(data + 4, vertices[1]);
        e.mem.set_u32(data + 0xc, vertices[3]);
        e.mem.set_u32(data + 8, vertices[2]);
        e.mem.set_u32(data + 0x10, vertices[4]);
        e.mem.set_u32(data + 0x14, vertices[5]);
        line_offset += 4;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if flags & 0x8000_0000 == 0 {
        e.vcall(
            allocator,
            0x10,
            &args![block, (flags & 0x3fff_ffff).wrapping_mul(0x30)],
        );
    }
}

/// What `CheckMappersForWarnings` calls: the dynamic cast `00653290` (thiscall on an object,
/// `(type)`; answers the object or 0) with the type record `012683c8`, the text builder set up
/// by `00cb1600` (thiscall on a 12-byte builder, `(buffer, size)`), its appenders `00cc01c0`
/// (text), `00cc05a0` (a pointer; Ghidra name `operator_shl_ov6`) and `00cc0320` (a float;
/// Ghidra name `operator_shl_ov5`), each thiscall `(value)` answering the builder, the builder's
/// destructor `00cc00e0` (thiscall), and the logger object `[01268360]` whose virtual slot `0xc`
/// takes `(1, 0xbadbadba, text, file, line)`. The texts are "Bones A:" (`010c4d7c`),
/// " and B:" (`010c4d88`), " are not fully aligned (error :" (`010c4d90`) and ")." (`010c4db0`),
/// the file name is at `010c4d38`.
const CAST_OBJECT: u32 = 0x0065_3290;
const MAPPED_TYPE: u32 = 0x0126_83c8;
const BUILDER_INIT: u32 = 0x00cb_1600;
const BUILDER_TEXT: u32 = 0x00cc_01c0;
const BUILDER_POINTER: u32 = 0x00cc_05a0;
const BUILDER_FLOAT: u32 = 0x00cc_0320;
const BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
const LOGGER: u32 = 0x0126_8360;
const ALIGNMENT_THRESHOLD: u32 = 0x0101_3ea4;
const TEXT_BONES_A: u32 = 0x010c_4d7c;
const TEXT_AND_B: u32 = 0x010c_4d88;
const TEXT_NOT_ALIGNED: u32 = 0x010c_4d90;
const TEXT_CLOSE: u32 = 0x010c_4db0;
const WARNING_FILE: u32 = 0x010c_4d38;

/// The part of `CheckMappersForWarnings` it repeats: when the byte `+4` of the second bone
/// record is set, the object found for the pair is 0 and the length of the lanes `+0x10` of
/// the mapping record (`z * z + (y * y + x * x)` rooted with `SQRTSS`) is above the float
/// threshold `01013ea4` (`FCOMP`: only an ordered "threshold below length" counts), a warning
/// is logged: the text "Bones A:" first-record " and B:" second-record " are not fully aligned
/// (error :" length ")." built in a 0x200-byte buffer and sent to the logger with the line
/// number `line`. Answers whether it logged.
fn warn_if_misaligned(
    e: &mut Engine,
    record: u32,
    first: u32,
    second: u32,
    found: u32,
    line: u32,
) -> bool {
    if e.mem.u8(second + 4) == 0 || found != 0 {
        return false;
    }
    let lanes = load4(e, record + 0x10);
    let squares = mulps(lanes, lanes);
    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
    let threshold = x87_quiet(e.mem.f32(ALIGNMENT_THRESHOLD));
    let above = f64::from(threshold) < f64::from(length);
    if !above {
        return false;
    }
    e.with_stack(0x20c, |e, builder| {
        let builder = builder.addr();
        let buffer = builder + 0xc;
        e.call(BUILDER_INIT, &args![builder, buffer, 0x200u32]);
        let step = e.call(BUILDER_TEXT, &args![builder, TEXT_BONES_A]).u32();
        let step = e.call(BUILDER_POINTER, &args![step, first]).u32();
        let step = e.call(BUILDER_TEXT, &args![step, TEXT_AND_B]).u32();
        let step = e.call(BUILDER_POINTER, &args![step, second]).u32();
        let step = e.call(BUILDER_TEXT, &args![step, TEXT_NOT_ALIGNED]).u32();
        let bits = x87_quiet_bits(length.to_bits());
        let step = e.call(BUILDER_FLOAT, &args![step, bits]).u32();
        e.call(BUILDER_TEXT, &args![step, TEXT_CLOSE]);
        let logger = e.mem.u32(LOGGER);
        e.vcall(
            logger,
            0xc,
            &args![1u32, 0xbadb_adbau32, buffer, WARNING_FILE, line],
        );
        e.call(BUILDER_DESTRUCT, &args![builder]);
    });
    true
}

// Translated from 00c79e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c79e20 - the FS:[0] exception-unwinding frame and the /GS cookie
// check (00ec408c) are not modeled.
/// `bhkRagdollController::CheckMappersForWarnings` (Ghidra name), register `ECX`: answers 1
/// when there is no mapper object at `+0x2a4` or one of its two mappers (`+0x10`, `+0x14`) is
/// missing, else whether a warning was logged. Each mapper holds its mapping records (0x40
/// bytes each, count at `+0x1c`, array at `+0x18`; two signed 16-bit bone indices at `+0` and
/// `+2` into the bone arrays of the mapper's two skeletons, 8-byte entries at `[skeleton +
/// 0x18]`). For the first mapper the controller's list at `+0x48` is asked for the entry of
/// the second index (slot table `+0x20`, entries `+8`) and the cast (`00653290`, type
/// `012683c8`) of the object at that entry's `+0xc` is the found object; the second mapper
/// uses the first index and walks the type chain of the object (virtual slot 8, parent at
/// `+4`) itself. Records that qualify are reported by `warn_if_misaligned` (lines `0xc1f` and
/// `0xc39`).
pub fn bhk_ragdoll_controller_check_mappers_for_warnings(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let mappers = e.mem.u32(base + 0x2a4);
    if mappers == 0 || e.mem.u32(mappers + 0x10) == 0 || e.mem.u32(mappers + 0x14) == 0 {
        return true;
    }
    let mut warned = false;
    let first = e.mem.u32(mappers + 0x10);
    let count = e.mem.i32(first + 0x1c);
    if count > 0 {
        let mut index = 0i32;
        let mut offset = 0u32;
        loop {
            let mappers = e.mem.u32(base + 0x2a4);
            let mapper = e.mem.u32(mappers + 0x10);
            let record = e.mem.u32(mapper + 0x18).wrapping_add(offset);
            let from = e.mem.u32(mapper + 0x10);
            let first_bones = e.mem.u32(from + 0x18);
            let word_a = i32::from(e.mem.i16(record));
            let word_b = i32::from(e.mem.i16(record + 2));
            let first_bone = first_bones.wrapping_add((word_a as u32).wrapping_mul(8));
            let to = e.mem.u32(mapper + 0x14);
            let second_bones = e.mem.u32(to + 0x18);
            let second_bone = second_bones.wrapping_add((word_b as u32).wrapping_mul(8));
            let list = e.mem.u32(base + 0x48);
            let table = e.mem.u32(list + 0x20);
            let slot = e
                .mem
                .i32(table.wrapping_add((word_b as u32).wrapping_mul(4)));
            let mut found = 0;
            if slot >= 0 {
                let entries = e.mem.u32(list + 8);
                let entry = e
                    .mem
                    .u32(entries.wrapping_add((slot as u32).wrapping_mul(4)));
                if entry != 0 {
                    let object = e.mem.u32(entry + 0xc);
                    if object != 0 {
                        found = e.call(CAST_OBJECT, &args![object, MAPPED_TYPE]).u32();
                    }
                }
            }
            if warn_if_misaligned(e, record, first_bone, second_bone, found, 0xc1f) {
                warned = true;
            }
            let mappers = e.mem.u32(base + 0x2a4);
            let mapper = e.mem.u32(mappers + 0x10);
            offset += 0x40;
            index += 1;
            if index >= e.mem.i32(mapper + 0x1c) {
                break;
            }
        }
    }
    let mappers = e.mem.u32(base + 0x2a4);
    let second = e.mem.u32(mappers + 0x14);
    if e.mem.i32(second + 0x1c) > 0 {
        let mut index = 0i32;
        let mut offset = 0u32;
        loop {
            let mappers = e.mem.u32(base + 0x2a4);
            let mapper = e.mem.u32(mappers + 0x14);
            let record = e.mem.u32(mapper + 0x18).wrapping_add(offset);
            let from = e.mem.u32(mapper + 0x10);
            let first_bones = e.mem.u32(from + 0x18);
            let to = e.mem.u32(mapper + 0x14);
            let second_bones = e.mem.u32(to + 0x18);
            let word_a = i32::from(e.mem.i16(record));
            let word_b = i32::from(e.mem.i16(record + 2));
            let first_bone = first_bones.wrapping_add((word_a as u32).wrapping_mul(8));
            let second_bone = second_bones.wrapping_add((word_b as u32).wrapping_mul(8));
            let list = e.mem.u32(base + 0x48);
            let table = e.mem.u32(list + 0x20);
            let slot = e
                .mem
                .i32(table.wrapping_add((word_a as u32).wrapping_mul(4)));
            let mut found = 0;
            if slot >= 0 {
                let entries = e.mem.u32(list + 8);
                let entry = e
                    .mem
                    .u32(entries.wrapping_add((slot as u32).wrapping_mul(4)));
                if entry != 0 {
                    let object = e.mem.u32(entry + 0xc);
                    if object != 0 {
                        let mut chain = e.vcall(object, 8, &args![]).u32();
                        while chain != 0 {
                            if chain == MAPPED_TYPE {
                                found = object;
                                break;
                            }
                            chain = e.mem.u32(chain + 4);
                        }
                    }
                }
            }
            if warn_if_misaligned(e, record, first_bone, second_bone, found, 0xc39) {
                warned = true;
            }
            let mappers = e.mem.u32(base + 0x2a4);
            let mapper = e.mem.u32(mappers + 0x14);
            offset += 0x40;
            index += 1;
            if index >= e.mem.i32(mapper + 0x1c) {
                break;
            }
        }
    }
    warned
}

/// What the foot IK setup calls: the dynamic cast with a type record, `00653270` (cdecl
/// `(type, object)`, answers the object or 0), the helper `005e4ab0` (thiscall; the same body as
/// `00c747b0`: slot `0xb4` of the object called with a scratch byte, answers the result minus 4
/// or 0), the setting float pointer `00403e20` (answers the address of a game setting's value),
/// the CRT cosine `00ec9f70` (`ST0` in and out) and the `bhkRagdollController::GetCapsule...`
/// type records `0126831c`, `01268310` and `0126864c`.
const CAST_TYPED: u32 = 0x0065_3270;
const CAPSULE_HOLDER: u32 = 0x005e_4ab0;
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
const CRT_COSINE: u32 = 0x00ec_9f70;
const TYPE_FIRST: u32 = 0x0126_831c;
const TYPE_SECOND: u32 = 0x0126_8310;
const TYPE_THIRD: u32 = 0x0126_864c;
const SETTING_FOOT_A: u32 = 0x0126_7d98;
const SETTING_FOOT_B: u32 = 0x0126_7de0;

/// The first child of `node`: slot `0xc` of the node has to answer an object and the 16-bit child
/// count at `+0xa6` has to be above 0 (checked on `node` itself), the child is the first entry
/// of the array at `+0xa0`.
fn first_child(e: &mut Engine, node: u32) -> u32 {
    let group = e.vcall(node, 0xc, &args![]).u32();
    if group != 0 && e.mem.u16(node + 0xa6) != 0 {
        let children = e.mem.u32(node + 0xa0);
        e.mem.u32(children)
    } else {
        0
    }
}

/// Looks `name` (a pointer to a string; 0 for the null string `[0109b220]`, otherwise a counted
/// copy through `00a5b690`) up under `world` with `00c4b470` and drops the count again.
fn find_object_by_name(e: &mut Engine, world: u32, name: u32) -> u32 {
    let copy = if name == 0 {
        e.mem.u32(NULL_STRING)
    } else {
        e.call(STRING_COPY, &args![name]).u32()
    };
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, copy);
        let found = e.call(GET_OBJECT_BY_NAME, &args![world, local, 1u32]).u32();
        let held = e.mem.u32(local);
        if held != e.mem.u32(NULL_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![held.wrapping_sub(8)]);
        }
        found
    })
}

/// The shape behind the object stored at `address` as the foot IK setup finds it: the cast to the
/// type `0126831c` gives the shape `[00c747b0-like(cast) + 4]`; without it the cast to
/// `01268310` is followed through `005e4ab0` and `+4` to the object at `+0xc`, which counts when
/// its virtual slot `0x20` answers 2. Answers 0 when nothing qualifies.
fn foot_shape(e: &mut Engine, address: u32) -> u32 {
    let object = e.mem.u32(address);
    let cast = e.call(CAST_TYPED, &args![TYPE_FIRST, object]).u32();
    if cast != 0 {
        let holder = e.call(CAPSULE_HOLDER, &args![cast]).u32();
        return e.mem.u32(holder + 4);
    }
    let object = e.mem.u32(address);
    let cast = e.call(CAST_TYPED, &args![TYPE_SECOND, object]).u32();
    if cast != 0 {
        let holder = e.call(CAPSULE_HOLDER, &args![cast]).u32();
        let inner = e.mem.u32(holder + 4);
        let shape = if inner != 0 {
            e.mem.u32(inner + 0xc)
        } else {
            0
        };
        if shape != 0 && e.vcall(shape, 0x20, &args![]).u32() == 2 {
            return shape;
        }
    }
    0
}

/// The collision body pointer address of `node` for the foot IK: the object at the node's
/// `+0x1c` cast to the type `012043f8`, its `+0x10` and that plus `0x14`; 0 when any step is 0.
fn foot_shape_address(e: &mut Engine, node: u32) -> u32 {
    let object = e.mem.u32(node + 0x1c);
    if object == 0 {
        return 0;
    }
    let cast = e.call(CAST_OBJECT, &args![object, 0x0120_43f8u32]).u32();
    if cast == 0 {
        return 0;
    }
    let inner = e.mem.u32(cast + 0x10);
    if inner == 0 {
        return 0;
    }
    inner + 0x14
}

// Translated from 00c77ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c77ac0 - the FS:[0] exception-unwinding frame and the /GS cookie
// check (00ec408c) are not modeled.
/// No name in the Xbox PDB or the Ghidra database; thiscall `(record, setup)`, answers 1 when it
/// filled the foot record `record` and 0 when a lookup failed. The first name of `setup`
/// (`[[setup + 4]]`) is looked up under the world `[this + 0x58]` (`find_object_by_name`); that
/// node, its first child and the child's first child (`first_child`) give three infos through
/// `GetCapsuleBoneShapeInfo` (`00c75180`), and the second and third node a collision address
/// each (`foot_shape_address`). The shape of the first address (`foot_shape`) supplies two limit
/// cosines (`cos` of `(90 - v * 180/pi - 10)` degrees for its float `+0xc8`, negated into
/// `record + 0x64`, and of `(v * 180/pi + 90 - 20)` for `+0xc4` into `record + 0x68`, each step
/// rounded to `f32` and the angle taken in radians) and the vector at `+0x20`, which goes
/// normalised into `record + 0x10` (`RSQRTSS` is the host CPU's estimate, refined once, 0 for an
/// all-zero sum). The shape of the second address (its `+0x50` vector, or the `+0x20` vector of the
/// `00c747b0` object of the cast to `0126864c`) only has to exist; its first lane plus the setting
/// float `01267d98` (`00403e20`) is the offset. The difference of the lane groups at `+0x10` and
/// `+0x20` of the third info goes to `record + 0x20` unless its length is below the float
/// `01016248`; the setting `01267de0` negated is `record + 0x60`, and `record + 0x54`, `+0x58` and
/// `+0x5c` come from the x87 sums of the second and third infos with the double `01011590`.
/// (The code also measures the lane group lengths of the first two infos into a local it never
/// reads again; that is not reproduced.)
pub fn fn_00c77ac0(e: &mut Engine, this: Ptr, record: u32, setup: u32) -> bool {
    let base = this.addr();
    let names = e.mem.u32(setup + 4);
    let name = e.mem.u32(names);
    let world = e.mem.u32(base + 0x58);
    let top = find_object_by_name(e, world, name);
    let mut second = 0;
    if top != 0 {
        second = first_child(e, top);
    }
    let mut third = 0;
    if second != 0 {
        third = first_child(e, second);
    }
    if top == 0 || second == 0 || third == 0 {
        return false;
    }
    let info_top = bhk_ragdoll_controller_get_capsule_bone_shape_info(e, top);
    let info_second = bhk_ragdoll_controller_get_capsule_bone_shape_info(e, second);
    let info_third = bhk_ragdoll_controller_get_capsule_bone_shape_info(e, third);
    let first_address = foot_shape_address(e, second);
    let second_address = foot_shape_address(e, third);
    if info_top == 0
        || info_second == 0
        || info_third == 0
        || first_address == 0
        || second_address == 0
    {
        return false;
    }
    let shape = foot_shape(e, first_address);
    if shape == 0 {
        return false;
    }
    let to_degrees = e.mem.f64(0x0102_f248);
    let ninety = e.mem.f64(0x0106_6808);
    let ten = e.mem.f64(0x0102_0758);
    let to_radians = e.mem.f64(0x0102_3128);
    let twenty = e.mem.f64(0x0102_fc70);
    let upper = x87_quiet(e.mem.f32(shape + 0xc8));
    let angle = ((ninety - f64::from(upper) * to_degrees) - ten) as f32;
    let angle = (f64::from(angle) * to_radians) as f32;
    let first_cosine = e.call(CRT_COSINE, &args![f64::from(angle)]).f32();
    let lower = x87_quiet(e.mem.f32(shape + 0xc4));
    let angle = ((f64::from(lower) * to_degrees + ninety) - twenty) as f32;
    let angle = (f64::from(angle) * to_radians) as f32;
    let second_cosine = e.call(CRT_COSINE, &args![f64::from(angle)]).f32();
    let first_vector = load4(e, shape + 0x20);
    let other = foot_shape(e, second_address);
    let second_vector = if other != 0 {
        load4(e, other + 0x50)
    } else {
        let object = e.mem.u32(second_address);
        let cast = e.call(CAST_TYPED, &args![TYPE_THIRD, object]).u32();
        let holder = if cast != 0 {
            let answer = fn_00c747b0(e, Ptr::new(cast));
            e.mem.u32(answer + 4)
        } else {
            0
        };
        if holder == 0 {
            return false;
        }
        load4(e, holder + 0x20)
    };
    let from = load4(e, info_third + 0x10);
    let to = load4(e, info_third + 0x20);
    let difference = subps(from, to);
    let setting = e.call(SETTING_FLOAT_POINTER, &args![SETTING_FOOT_A]).u32();
    let setting_value = x87_quiet(e.mem.f32(setting));
    let offset = (f64::from(setting_value) + f64::from(x87_quiet(second_vector[0]))) as f32;
    store4(e, record + 0x10, first_vector);
    let vector = load4(e, record + 0x10);
    let unit = refined_normal(e, vector);
    store4(e, record + 0x10, unit);
    let squares = mulps(difference, difference);
    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
    let limit = x87_quiet(e.mem.f32(0x0101_6248));
    let kept = if f64::from(limit) > f64::from(length) {
        [0.0f32; 4]
    } else {
        difference
    };
    store4(e, record + 0x20, kept);
    let setting = e.call(SETTING_FLOAT_POINTER, &args![SETTING_FOOT_B]).u32();
    let value = x87_quiet(e.mem.f32(setting));
    e.mem.set_f32(record + 0x60, -value);
    let spread = f64::from(x87_quiet(e.mem.f32(info_second + 0x10)))
        - f64::from(x87_quiet(e.mem.f32(info_second + 0x20)));
    let position = f64::from(x87_quiet(e.mem.f32(info_third + 0x10)));
    let doubled = e.mem.f64(0x0101_1590);
    let total = (position * doubled + spread) + f64::from(offset);
    let stored = total as f32;
    e.mem.set_f32(record + 0x5c, stored);
    let position_bits = x87_quiet_bits(e.mem.u32(info_third + 0x10));
    e.mem.set_u32(record + 0x54, position_bits);
    let product = doubled * f64::from(offset);
    e.mem
        .set_f32(record + 0x58, (f64::from(stored) - product) as f32);
    e.mem.set_f32(record + 0x64, -first_cosine);
    e.mem.set_f32(record + 0x68, second_cosine);
    true
}

/// What `InitFootIK` calls: the 0x78-byte foot record constructor `00cd8920` (thiscall; it sets
/// the three bone index words at `+4`, `+6` and `+8` to `0xffff` and fills the rest), the
/// generic array `_reserve` (Ghidra name, `00c908c0`, cdecl `(allocator, array, capacity,
/// element size)`), and the foot element constructor `00cd8820` (thiscall on a 0xf0-byte block,
/// `(record)`, answers the block). `011f49e0` holds the name pointer used for collision objects
/// without a name, `010c4dc8` is the double added to the weight.
const FOOT_RECORD_INIT: u32 = 0x00cd_8920;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const FOOT_ELEMENT_INIT: u32 = 0x00cd_8820;
const DEFAULT_NAME: u32 = 0x011f_49e0;
const WEIGHT_OFFSET: u32 = 0x010c_4dc8;
const FOOT_RECORD_SIZE: u32 = 0x78;

/// The name pointer of a collision object: `[[object + 8] + 8]`, or the pointer stored at
/// `011f49e0` without an object.
fn collision_name(e: &mut Engine, object: u32) -> u32 {
    if object != 0 {
        let inner = e.mem.u32(object + 8);
        e.mem.u32(inner + 8)
    } else {
        e.mem.u32(DEFAULT_NAME)
    }
}

// Translated from 00c7c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7c3a0 - the FS:[0] exception-unwinding frame and the /GS cookie
// check (00ec408c) are not modeled.
/// `bhkRagdollController::InitFootIK` (Ghidra name), thiscall `(setup)`: nothing happens without
/// the count `[setup + 8]` or with the byte `+0x42` clear. A foot record is constructed on the
/// stack (`00cd8920`) and filled by `00c77ac0(record, setup)`; when that fails nothing else
/// happens. The record then gets the skeleton `[[this + 0x2a4] + 0xc]` at `+0`, the vector
/// `(fill, fill, third, fill)` from `01011d78`/`01012068` at `+0x30` and `+0x40` and a quiet copy
/// of the float `01267d90` at `+0x50`. The array of 0xd0-byte elements at `+0x22c` is grown
/// (`_reserve`, doubling or to the count) to the count, its new elements get a null object
/// pointer, `0.0` at `+4` and `0xffff` at `+0xc0`, and the size becomes the count. For every name
/// of `[setup + 4]`: the node named so under `[this + 0x58]`, its first child and that one's first
/// child (`first_child`) give three collision objects (`00c802d0`); the weight from
/// `GetCapsuleBoneShapeInfo(third node)` (`+4`, 0 without info) plus the double `010c4dc8` becomes
/// the float at `+0x244`. When the three collision objects differ and the second and third have
/// names, the three names are looked up in the skeleton (`00cdd390`) into the record's words
/// `+4`, `+6`, `+8`; when none is `0xffff` a 0xf0-byte block (allocator slot 4, word `+4`
/// set to 0xf0) is built from the record (`00cd8820`) and becomes the element's object. Every
/// mapping record of the first mapper (`[[this + 0x2a4] + 0x10]`, 0x40 bytes) whose first word equals the
/// record's word `+8` sets the element's word `+0xc0` to its second word. In the end the byte
/// `+0x21f` says whether every element has an object; if not all objects are destroyed
/// (`00c905b0`) and the size is cleared. (The shorter `r1 = constant + weight` sum the code
/// stores in a local it never reads is not reproduced.)
pub fn bhk_ragdoll_controller_init_foot_ik(e: &mut Engine, this: Ptr, setup: u32) {
    let base = this.addr();
    let count = e.mem.u32(setup + 8);
    if count == 0 || e.mem.u8(base + 0x42) == 0 {
        return;
    }
    e.with_stack(FOOT_RECORD_SIZE, |e, record| {
        let record = record.addr();
        e.call(FOOT_RECORD_INIT, &args![record]);
        let succeeded = fn_00c77ac0(e, this, record, setup);
        let mappers = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(mappers + 0xc);
        if !succeeded {
            return;
        }
        let weight_bits = x87_quiet_bits(e.mem.u32(0x0126_7d90));
        let fill = e.mem.u32(0x0101_1d78);
        let third = e.mem.u32(0x0101_2068);
        e.mem.set_u32(record + 0x50, weight_bits);
        e.mem.set_u32(record, skeleton);
        for lanes_at in [0x30, 0x40] {
            for (lane, value) in [fill, fill, third, fill].into_iter().enumerate() {
                e.mem.set_u32(record + lanes_at + 4 * lane as u32, value);
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let array = base + 0x22c;
        let mut capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        if (capacity as i32) < (count as i32) {
            capacity = capacity.wrapping_add(capacity);
            if (count as i32) >= (capacity as i32) {
                capacity = count;
            }
            e.call(ARRAY_RESERVE, &args![allocator, array, capacity, 0xd0u32]);
        }
        let size = e.mem.u32(array + 4);
        let mut missing = count.wrapping_sub(size) as i32;
        let mut element = size.wrapping_mul(0xd0).wrapping_add(e.mem.u32(array));
        while missing > 0 {
            if element != 0 {
                e.mem.set_f32(element + 4, 0.0);
                e.mem.set_u32(element, 0);
                e.mem.set_u16(element + 0xc0, 0xffff);
            }
            element = element.wrapping_add(0xd0);
            missing -= 1;
        }
        e.mem.set_u32(base + 0x230, count);
        let mut offset = 0u32;
        for index in 0..count {
            let names = e.mem.u32(setup + 4);
            let name = e.mem.u32(names + 4 * index);
            let world = e.mem.u32(base + 0x58);
            let found = find_object_by_name(e, world, name);
            let mut child = 0;
            if found != 0 {
                child = first_child(e, found);
            }
            let mut grandchild = 0;
            if child != 0 {
                grandchild = first_child(e, child);
            }
            let mut first_collision = 0;
            if found != 0 {
                first_collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![found]).u32();
            }
            let mut second_collision = 0;
            if child != 0 {
                second_collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![child]).u32();
            }
            let mut third_collision = 0;
            if grandchild != 0 {
                third_collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![grandchild]).u32();
            }
            let info = bhk_ragdoll_controller_get_capsule_bone_shape_info(e, grandchild);
            let weight = if info != 0 {
                x87_quiet(e.mem.f32(info + 4))
            } else {
                0.0
            };
            let weight_offset = e.mem.f64(WEIGHT_OFFSET);
            let adjusted = (f64::from(weight) + weight_offset) as f32;
            e.mem.set_f32(base + 0x244, adjusted);
            let first_name = collision_name(e, first_collision);
            let second_name = collision_name(e, second_collision);
            let third_name = collision_name(e, third_collision);
            if second_name != 0
                && third_name != 0
                && first_collision != second_collision
                && first_collision != third_collision
                && third_collision != second_collision
            {
                let lookups = [first_name, second_name, third_name].map(|name| {
                    e.call(FIND_BONE_WITH_NAME, &args![skeleton, name, 0u32])
                        .u32() as u16
                });
                for (lane, value) in lookups.into_iter().enumerate() {
                    e.mem.set_u16(record + 4 + 2 * lane as u32, value);
                }
                if lookups.iter().all(|&value| value != 0xffff) {
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let allocator = e.mem.u32(router + 0x10);
                    let block = e.vcall(allocator, 4, &args![0xf0u32]).u32();
                    e.mem.set_u16(block + 4, 0xf0);
                    let object = e.call(FOOT_ELEMENT_INIT, &args![block, record]).u32();
                    let data = e.mem.u32(base + 0x22c);
                    e.mem.set_u32(data + offset, object);
                }
            }
            let mappers = e.mem.u32(base + 0x2a4);
            let mapper = e.mem.u32(mappers + 0x10);
            if mapper != 0 && e.mem.i32(mapper + 0x1c) > 0 {
                let mut position = 0i32;
                let mut at = 0u32;
                loop {
                    let mappers = e.mem.u32(base + 0x2a4);
                    let mapper = e.mem.u32(mappers + 0x10);
                    let records = e.mem.u32(mapper + 0x18);
                    let wanted = e.mem.u16(record + 8);
                    let current = records.wrapping_add(at);
                    if e.mem.u16(current) == wanted {
                        let data = e.mem.u32(base + 0x22c);
                        let target = e.mem.u16(current + 2);
                        e.mem.set_u16(data + offset + 0xc0, target);
                    }
                    let mappers = e.mem.u32(base + 0x2a4);
                    let mapper = e.mem.u32(mappers + 0x10);
                    position += 1;
                    at += 0x40;
                    if position >= e.mem.i32(mapper + 0x1c) {
                        break;
                    }
                }
            }
            offset += 0xd0;
        }
        let complete = (0..count).all(|index| {
            let data = e.mem.u32(base + 0x22c);
            e.mem.u32(data + 0xd0 * index) != 0
        });
        if !complete {
            for index in 0..count {
                let data = e.mem.u32(base + 0x22c);
                let object = e.mem.u32(data + 0xd0 * index);
                if object != 0 {
                    e.call(ELEMENT_DESTRUCT, &args![object]);
                }
            }
            e.mem.set_u32(base + 0x230, 0);
        }
        e.mem.set_u8(base + 0x21f, u8::from(complete));
    });
}

/// What `castRay` calls and reads: `SetMultiLayerCollision` (Ghidra name, `00c82870`, cdecl
/// `(layer, mask low, mask high)`), `setTransformedPos_ov2` (Ghidra name, `00c7f7a0`, thiscall on
/// the result vector, `(transform, vector)`), the world's ray cast `castRay` (Ghidra name,
/// `00cc26c0`, thiscall on `[this + 0x90]`, `(input, output)`), the float `01012054` (the "no hit"
/// fraction), the tolerance `01013ea4`, the absolute-value mask `01017d10`, the vector `010ec7f0`
/// of ones, the vector `01267e30` copied into the output record, the doubles `01018a90` (the
/// limit for the rotated vector's height) and `01011590`, and the floats `01011d78`, `01012068`
/// (the `(fill, fill, third, fill)` vector) and `01017718`, `01016248` (the constants of the
/// refined reciprocal root).
const SET_MULTI_LAYER_COLLISION: u32 = 0x00c8_2870;
const TRANSFORM_POINT: u32 = 0x00c7_f7a0;
const WORLD_CAST_RAY: u32 = 0x00cc_26c0;
const NO_HIT_FRACTION: u32 = 0x0101_2054;
const RAY_TOLERANCE: u32 = 0x0101_3ea4;
const ABS_MASK: u32 = 0x0101_7d10;
const ONE_VECTOR: u32 = 0x010e_c7f0;
const RAY_OUTPUT_VECTOR: u32 = 0x0126_7e30;
const HEIGHT_LIMIT: u32 = 0x0101_8a90;
const HEIGHT_SHIFT: u32 = 0x0101_1590;
const ROOT_THREE: u32 = 0x0101_7718;
const ROOT_HALF: u32 = 0x0101_6248;

/// The three lowest lanes of `vector` scaled to unit length with the refined reciprocal root the
/// SSE code uses: with `s = z*z + (y*y + x*x)` and `r = RSQRTSS(s)` the scale is
/// `(half * r) * (three - (s * r) * r)`, 0 when `s` equals the float `01011d78`; all four lanes
/// are multiplied by it.
fn refined_normal(e: &Engine, vector: Lanes) -> Lanes {
    let sum = dot_xyz(mulps(vector, vector));
    let fill = e.mem.f32(VECTOR_FILL);
    let three = e.mem.f32(ROOT_THREE);
    let half = e.mem.f32(ROOT_HALF);
    let estimate = crate::x87::rsqrt_estimate(sum);
    let refined = (half * estimate) * (three - (sum * estimate) * estimate);
    let scale = if sum == fill { 0.0 } else { refined };
    mulps([scale; 4], vector)
}

/// The cache record of the next ray: the controller's array of 0xd0-byte elements at `+0x22c`
/// holds in element `[this + 0x23a]` the 0x40-byte records, record `[this + 0x238]` being the
/// one meant here (`+0x10` start, `+0x20` end, `+0x30` normal, and the fraction in the first
/// float of the next record).
fn ray_cache_record(e: &Engine, base: u32) -> u32 {
    let element = u32::from(e.mem.u16(base + 0x23a)).wrapping_mul(0xd0);
    let record = u32::from(e.mem.u16(base + 0x238)) << 6;
    e.mem
        .u32(base + 0x22c)
        .wrapping_add(record.wrapping_add(element))
}

/// The transform of bone 0 of `pose`: the local transform array entry (`+0x10`), or
/// `hkaPose::calculateBoneModelSpace(pose, 0)` when bit 1 of the bone's flag dword is set.
fn root_bone_transform(e: &mut Engine, pose: u32) -> u32 {
    let flags = e.mem.u32(pose + 0x1c);
    if e.mem.u8(flags) & 2 == 0 {
        e.mem.u32(pose + 0x10)
    } else {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, 0u32]).u32()
    }
}

/// The ray-cast world the controller shoots into: `[[[[this + 0x48] + 8] + 0] + 8] + 0x350`,
/// 0 when a link is null.
fn ray_world(e: &Engine, base: u32) -> u32 {
    let list = e.mem.u32(base + 0x48);
    if list == 0 {
        return 0;
    }
    let entries = e.mem.u32(list + 8);
    let first = e.mem.u32(entries);
    if first == 0 {
        return 0;
    }
    let inner = e.mem.u32(first + 8);
    if inner == 0 {
        return 0;
    }
    e.mem.u32(inner + 0x350)
}

// Translated from 00c77460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c77460 - the /GS cookie check (00ec408c) is not modeled; the
// ray records handed to the world's cast are built on the stack and only the fields the code
// writes are set (the others are whatever the stack held).
/// `bhkRagdollController::castRay` (Ghidra name), thiscall `(hit, from, to, fraction, normal)`;
/// `hit` is a byte, `from`/`to`/`normal` are 16-byte vectors and `fraction` a float. A ray cache
/// (`ray_cache_record`) is consulted first: when the fraction stored for the current record is
/// not the "no hit" float `01012054` and the cached start and end differ from `from` and `to` by
/// at most the tolerance `01013ea4` in each of the three lowest lanes, the cached normal is
/// copied to `normal` and the fraction is rebuilt from the height of the interpolated cached
/// point. Otherwise the ray is cast: `SetMultiLayerCollision(0x27, 0x2002200e, 0)` is called and
/// the input (start, end, filter `[this + 0x5c] << 16 | 0x27`) goes either to the virtual
/// function `0xc8` of the world found through the list at `+0x48` (byte `+0x224` clear), or to
/// `castRay` of the object at `+0x90` (when set; nothing happens without it). A hit stores its
/// fraction in `fraction` and its normalised normal (the refined reciprocal root) in `normal`
/// and answers 1 in `hit`. For a hit the root bone's transform and the transform at `+0x10`
/// give a reference height (`setTransformedPos_ov2`); with `[this + 0x230]` equal to 2 and the
/// rotated `(fill, fill, third, fill)` vector's height above the limit `01018a90`, a hit above
/// the reference height minus the double `01011590` gets its fraction recomputed from the
/// height of the point; with byte `+0x240` clear the normal becomes the vector `(fill, fill,
/// third, fill)`. The start, end, normal and fraction are stored in the cache record, and a miss
/// stores the "no hit" float; the record index `[this + 0x238]` grows by one.
/// Answers `hit` (the `hkBool` returned through the hidden pointer, Xbox PDB `castRay` returns
/// `?AVhkBool`; `EAX` is `hit`, reloaded at 00c77a9b, at the RET 00c77ab5 on every path). No exe
/// reader of the hkaRaycastInterface slot +0x8 was found.
pub fn bhk_ragdoll_controller_cast_ray(
    e: &mut Engine,
    this: Ptr,
    hit: u32,
    from: u32,
    to: u32,
    fraction: u32,
    normal: u32,
) -> u32 {
    let base = this.addr();
    let no_hit = x87_quiet(e.mem.f32(NO_HIT_FRACTION));
    let record = ray_cache_record(e, base);
    let cached = e.mem.f32(record + 0x40);
    let mut found = false;
    let mut reuse = false;
    if f64::from(no_hit) != f64::from(cached) {
        let tolerance = e.mem.f32(RAY_TOLERANCE);
        let mask = load4(e, ABS_MASK);
        let within = |a: Lanes, b: Lanes| {
            andps(subps(a, b), mask)[..3]
                .iter()
                .all(|&lane| lane <= tolerance)
        };
        let cached_from = load4(e, record + 0x10);
        let start = load4(e, from);
        if within(cached_from, start) {
            let cached_to = load4(e, record + 0x20);
            let end = load4(e, to);
            if within(cached_to, end) {
                reuse = true;
            }
        }
    }
    if reuse {
        copy16(e, record + 0x30, normal);
        let fraction_cached = e.mem.f32(record + 0x40);
        let cached_from = load4(e, record + 0x10);
        let cached_to = load4(e, record + 0x20);
        let ones = load4(e, ONE_VECTOR);
        let along = [fraction_cached; 4];
        let point = addps(
            mulps(subps(ones, along), cached_from),
            mulps(cached_to, along),
        );
        let start_height = x87_quiet(e.mem.f32(from + 8));
        let numerator = f64::from(start_height) - f64::from(point[2]);
        let start_height = x87_quiet(e.mem.f32(from + 8));
        let denominator = f64::from(start_height) - f64::from(e.mem.f32(to + 8));
        e.mem.set_f32(fraction, (numerator / denominator) as f32);
        found = true;
    } else {
        e.call(
            SET_MULTI_LAYER_COLLISION,
            &args![0x27u32, 0x2002_200eu32, 0u32],
        );
        let filter = (e.mem.u32(base + 0x5c) << 16) | 0x27;
        let start = load4(e, from);
        let end = load4(e, to);
        let cast = e.with_stack(0x1a4, |e, frame| {
            let frame = frame.addr();
            if e.mem.u8(base + 0x224) == 0 {
                e.mem.set_f32(frame + 0xd0, 1.0);
                copy16(e, RAY_OUTPUT_VECTOR, frame + 0x120);
                e.mem.set_u8(frame + 0xb0, 0);
                e.mem.set_u32(frame + 0xd4, 0xffff_ffff);
                e.mem.set_u32(frame + 0x100, 0);
                e.mem.set_u32(frame + 0xe0, 0xffff_ffff);
                e.mem.set_u32(frame + 0x110, 0);
                for offset in [0x130, 0x134, 0x138] {
                    e.mem.set_u32(frame + offset, 0);
                }
                e.mem.set_u8(frame + 0x13c, 0);
                e.mem.set_u32(frame + 0xb4, filter);
                store4(e, frame + 0x90, start);
                store4(e, frame + 0xa0, end);
                let world = ray_world(e, base);
                let answer = e.vcall(world, 0xc8, &args![frame + 0x90]).u32();
                (answer as u8 != 0).then(|| (load4(e, frame + 0xc0), e.mem.f32(frame + 0xd0)))
            } else {
                let caster = e.mem.u32(base + 0x90);
                if caster == 0 {
                    return None;
                }
                e.mem.set_u32(frame + 0x84, filter);
                e.mem.set_f32(frame + 0x150, 1.0);
                e.mem.set_u32(frame + 0x154, 0xffff_ffff);
                e.mem.set_u32(frame + 0x160, 0xffff_ffff);
                e.mem.set_u8(frame + 0x80, 0);
                e.mem.set_u32(frame + 0x180, 0);
                e.mem.set_u32(frame + 0x190, 0);
                store4(e, frame + 0x60, start);
                store4(e, frame + 0x70, end);
                e.call(WORLD_CAST_RAY, &args![caster, frame + 0x60, frame + 0x140]);
                (e.mem.u32(frame + 0x190) != 0)
                    .then(|| (load4(e, frame + 0x140), e.mem.f32(frame + 0x150)))
            }
        });
        if let Some((vector, hit_fraction)) = cast {
            e.mem.set_f32(fraction, x87_quiet(hit_fraction));
            let unit = refined_normal(e, vector);
            store4(e, normal, unit);
            found = true;
        }
    }
    if !found {
        let record = ray_cache_record(e, base);
        let next = record + 0x40;
        e.mem
            .set_u32(next, x87_quiet_bits(e.mem.u32(NO_HIT_FRACTION)));
        advance_ray_cache(e, base, hit, found);
        return hit;
    }
    let along = [x87_quiet(e.mem.f32(fraction)); 4];
    let ones = load4(e, ONE_VECTOR);
    let start = load4(e, from);
    let end = load4(e, to);
    let point = addps(mulps(subps(ones, along), start), mulps(end, along));
    let pose = e.mem.u32(base + 0x88);
    let root = root_bone_transform(e, pose);
    let transformed = e.with_stack(0x10, |e, result| {
        let result = result.addr();
        e.call(TRANSFORM_POINT, &args![result, base + 0x10, root]);
        e.mem.u32(result + 8)
    });
    let vector_bits = [
        e.mem.u32(VECTOR_FILL),
        e.mem.u32(VECTOR_FILL),
        e.mem.u32(VECTOR_THIRD),
        e.mem.u32(VECTOR_FILL),
    ];
    let vector = vector_bits.map(f32::from_bits);
    let pose = e.mem.u32(base + 0x88);
    let root = root_bone_transform(e, pose);
    let rotation = load4(e, root + 0x10);
    let half = load4(e, NEG_HALF_LANES);
    let rotated = rotate_lanes(vector, rotation, half, true);
    if e.mem.u32(base + 0x230) == 2 {
        let height = x87_quiet(rotated[2]).abs();
        let limit = e.mem.f64(HEIGHT_LIMIT);
        if f64::from(height) > limit {
            let point_height = x87_quiet(point[2]);
            let reference = x87_quiet(f32::from_bits(transformed));
            let shift = e.mem.f64(HEIGHT_SHIFT);
            let lowered = f64::from(reference) - shift;
            if lowered < f64::from(point_height) {
                let start_height = f64::from(x87_quiet(e.mem.f32(from + 8)));
                let numerator = (start_height - f64::from(point_height)) + shift;
                let start_height = f64::from(x87_quiet(e.mem.f32(from + 8)));
                let denominator = start_height - f64::from(e.mem.f32(to + 8));
                e.mem.set_f32(fraction, (numerator / denominator) as f32);
            }
        }
    }
    if e.mem.u8(base + 0x240) == 0 {
        for (lane, bits) in vector_bits.into_iter().enumerate() {
            e.mem.set_u32(normal + 4 * lane as u32, bits);
        }
    }
    let record = ray_cache_record(e, base);
    copy16(e, from, record + 0x10);
    let record = ray_cache_record(e, base);
    copy16(e, to, record + 0x20);
    let record = ray_cache_record(e, base);
    copy16(e, normal, record + 0x30);
    let next = ray_cache_record(e, base) + 0x40;
    e.mem.set_u32(next, x87_quiet_bits(e.mem.u32(fraction)));
    advance_ray_cache(e, base, hit, found);
    hit
}

/// The end of `castRay`: the record index `[this + 0x238]` is incremented (16 bits) and the byte
/// `hit` gets whether the ray hit.
fn advance_ray_cache(e: &mut Engine, base: u32, hit: u32, found: bool) {
    let index = e.mem.u16(base + 0x238);
    e.mem.set_u16(base + 0x238, index.wrapping_add(1));
    e.mem.set_u8(hit, u8::from(found));
}

// Translated from 00c751f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c751f0 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRagdollController::DoGrabIK` (Xbox PDB): steps the two grab entries at `this + 0x1b0`
/// (0x30 bytes each: target object `+0`, blend `+4`, weight `+8`, active byte `+0xc`, held byte
/// `+0xd`, vectors `+0x10` and `+0x20`; the 16-bit bone triples are at `this + 0x210 + 6*i`).
///
/// For an active entry that is not held, the target position (the floats at `target + 0x8c`
/// times the float at `01267c20`, each product taken in `f64`) and `0.0` make the vector `v`;
/// when the weight is above 0 the stored vectors blend towards `v` and `(0, 1, 0, 0)` by the
/// blend factor `s` (`(ones - s) * old + new * s`, SSE, `ones` is the vector at `010ec7f0`),
/// else they are replaced by them. The weight moves towards 1 (held entries: 0) by the rate
/// float `01267dfc`: `weight += (goal - weight) * rate`. A held entry whose new weight is
/// below the float `01013ea4` is released (held and active cleared, weight 0). Every other
/// active entry solves: `00c7f840` (thiscall on a 16-byte result, `(transform, vector)`)
/// transforms the first vector, a 0x82-byte record built by `00c74ed0` is filled in (the
/// bone triple, the weight at `+0x28` and `+0x2c`, `(0, 0, 1, 0)` at `+0x10`, the result at
/// `+0x40`) and `00cdbae0(flag, record, [this + 0x88])` (cdecl) runs. When neither entry is
/// active any more the byte at `this + 0x21e` is cleared.
pub fn bhk_ragdoll_controller_do_grab_ik(e: &mut Engine, this: Ptr, transform: u32) {
    const TRANSFORM_VECTOR: u32 = 0x00c7_f840;
    const SOLVE: u32 = 0x00cd_bae0;
    const RATE: u32 = 0x0126_7dfc;
    const RELEASE_BELOW: u32 = 0x0101_3ea4;
    let base = this.addr();
    let mut factor: f32 = e.global(SCALE_FACTOR);
    for index in 0..2u32 {
        let entry = base + 0x1b0 + 0x30 * index;
        let triple = base + 0x210 + 6 * index;
        if e.mem.u8(entry + 0xc) == 0 {
            continue;
        }
        let held = e.mem.u8(entry + 0xd);
        let goal = if held == 0 {
            let target = e.mem.u32(entry);
            let scale = f64::from(factor);
            let [x, y, z] = [0x8c, 0x90, 0x94].map(|offset| f64::from(e.mem.f32(target + offset)));
            let vector = [
                (scale * x) as f32,
                (y * scale) as f32,
                (z * scale) as f32,
                0.0,
            ];
            let towards = [0.0, 1.0, 0.0, 0.0];
            if 0.0 < e.mem.f32(entry + 8) {
                let blend = e.mem.f32(entry + 4);
                let ones = load4(e, ONES_VECTOR);
                for (offset, new) in [(0x10, vector), (0x20, towards)] {
                    let old = load4(e, entry + offset);
                    let mixed = [0, 1, 2, 3]
                        .map(|lane| (ones[lane] - blend) * old[lane] + new[lane] * blend);
                    store4(e, entry + offset, mixed);
                }
            } else {
                store4(e, entry + 0x10, vector);
                store4(e, entry + 0x20, towards);
            }
            1.0f32
        } else {
            0.0f32
        };
        let weight = e.mem.f32(entry + 8);
        let rate: f32 = e.global(RATE);
        let moved =
            ((f64::from(goal) - f64::from(weight)) * f64::from(rate) + f64::from(weight)) as f32;
        e.mem.set_f32(entry + 8, moved);
        let release_below: f32 = e.global(RELEASE_BELOW);
        if moved < release_below && held != 0 {
            e.mem.set_u8(entry + 0xd, 0);
            e.mem.set_f32(entry + 8, 0.0);
            e.mem.set_u8(entry + 0xc, 0);
            continue;
        }
        e.with_stack(0x10, |e, result| {
            e.call(TRANSFORM_VECTOR, &args![result, transform, entry + 0x10]);
            e.with_stack(0x90, |e, record| {
                let r = record.addr();
                fn_00c74ed0(e, record);
                copy_quiet(e, entry + 8, r + 0x28);
                copy_quiet(e, entry + 8, r + 0x2c);
                let bones = [0, 2, 4].map(|offset| e.mem.u16(triple + offset));
                for (offset, bone) in [0, 2, 4].into_iter().zip(bones) {
                    e.mem.set_u16(r + offset, bone);
                }
                store4(e, r + 0x10, [0.0, 0.0, 1.0, 0.0]);
                copy16(e, result.addr(), r + 0x40);
                let skeleton = e.mem.u32(base + 0x88);
                e.with_stack(4, |e, flag| {
                    e.call(SOLVE, &args![flag, record, skeleton]);
                });
            });
        });
        factor = e.global(SCALE_FACTOR);
    }
    if e.mem.u8(base + 0x1bc) == 0 && e.mem.u8(base + 0x1ec) == 0 {
        e.mem.set_u8(base + 0x21e, 0);
    }
}

// Translated from 00c75460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::InitGrabIK` (Xbox PDB), register `ECX`: with the byte at `this + 0x42`
/// set and a skeleton (`[[this + 0x2a4] + 0xc]`), looks up six bone indices with `00cdd390`
/// (cdecl `(skeleton, name, 0)`; the names at `010203c4`, `010c4c50`, `010c4c40`, `010203a0`,
/// `010c4c30`, `010c4c20`) into the 16-bit slots `this + 0x210..0x21a`; if any of the first three
/// or of the last three is `0xffff` the answer is cleared. Then clears the two grab entries'
/// target, blend, weight and active fields (`+0`, `+4`, `+8`, `+0xc` of each entry at
/// `this + 0x1b0`, 0x30 bytes apart). Answers the byte at `this + 0x42` (0 when a lookup failed).
pub fn bhk_ragdoll_controller_init_grab_ik(e: &mut Engine, this: Ptr) -> u8 {
    const FIND_BONE: u32 = 0x00cd_d390;
    const NAMES: [u32; 6] = [
        0x0102_03c4,
        0x010c_4c50,
        0x010c_4c40,
        0x0102_03a0,
        0x010c_4c30,
        0x010c_4c20,
    ];
    let base = this.addr();
    let mut flag = e.mem.u8(base + 0x42);
    if flag != 0 {
        let owner = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(owner + 0xc);
        if skeleton != 0 {
            for (index, name) in NAMES.into_iter().enumerate() {
                let slot = e.call(FIND_BONE, &args![skeleton, name, 0u32]).u32() as u16;
                e.mem.set_u16(base + 0x210 + 2 * index as u32, slot);
            }
            let missing = |e: &Engine, first: u32| {
                [0, 2, 4]
                    .into_iter()
                    .any(|offset| e.mem.u16(base + first + offset) == 0xffff)
            };
            if missing(e, 0x210) {
                flag = 0;
            }
            if missing(e, 0x216) {
                flag = 0;
            }
        }
    }
    for index in 0..2u32 {
        let entry = base + 0x1b0 + 0x30 * index;
        e.mem.set_u32(entry, 0);
        e.mem.set_f32(entry + 4, 0.0);
        e.mem.set_f32(entry + 8, 0.0);
        e.mem.set_u8(entry + 0xc, 0);
    }
    flag
}

// Translated from 00c757b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c757b0 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRagdollController::GetEyeOffset` (Xbox PDB), register `ECX`: with the byte at
/// `this + 0x190` set, takes the 16 bytes `00cda8e0([this + 0xf0], 1)` points at, transforms
/// them by the head bone (the 16-bit index at `this + 0x144`, a 0x30-byte record of the
/// skeleton at `[this + 0x88]`, or `00cda180(skeleton, index)` when bit 1 of the index's flag
/// dword is set) and by the transform at `this + 0x10` (both through `00c7f7a0`, thiscall on
/// the vector, `(transform, vector)`), copies the vector's point to `out` (`00458620`) and
/// answers true; else false.
pub fn bhk_ragdoll_controller_get_eye_offset(e: &mut Engine, this: Ptr, out: Ptr) -> bool {
    const EYE_VECTOR: u32 = 0x00cd_a8e0;
    const BONE_RECORD: u32 = 0x00cd_a180;
    const TRANSFORM_VECTOR: u32 = 0x00c7_f7a0;
    let base = this.addr();
    if e.mem.u8(base + 0x190) == 0 {
        return false;
    }
    let owner = e.mem.u32(base + 0xf0);
    let source = e.call(EYE_VECTOR, &args![owner, 1u32]).u32();
    e.with_stack(0x10, |e, vector| {
        copy16(e, source, vector.addr());
        let index = i32::from(e.mem.i16(base + 0x144)) as u32;
        let skeleton = e.mem.u32(base + 0x88);
        let flags = e.mem.u32(skeleton + 0x1c);
        let bone = if e.mem.u8(flags.wrapping_add(index.wrapping_mul(4))) & 2 == 0 {
            let records = e.mem.u32(skeleton + 0x10);
            index.wrapping_mul(0x30).wrapping_add(records)
        } else {
            e.call(BONE_RECORD, &args![skeleton, index]).u32()
        };
        e.call(TRANSFORM_VECTOR, &args![vector, bone, vector]);
        e.call(TRANSFORM_VECTOR, &args![vector, base + 0x10, vector]);
        e.call(POINT_COPY, &args![out, vector]);
    });
    true
}

/// What `00c7e3b0` calls and uses: `aa1070`/`aa10f0` (the cdecl allocate `(size)` and free
/// `(block)`), the CRT `memset` (`00ec61c0`, cdecl), `hkStringPtr::hkStringPtr` (Ghidra name,
/// `00cb0a60`, thiscall), `~hkStringPtr` (Ghidra name, `00cb0c00`, thiscall), `00cb0c10`
/// (Ghidra name `operator_`, thiscall `(text)`), `strDup` (Ghidra name, `00cb1410`, cdecl),
/// `setToReferencePose` (Ghidra name, `00cda110`, thiscall on the pose), the hash map clear
/// `00b601a0` (thiscall; the code runs it once for the derived and once for the base vtable)
/// and the vtables from the RTTI of the objects built on the stack: `010c4ce4`
/// (`BSSimpleArray<bhkRagdollController::bhkSkeletonBone *>`, the slot at `010c4cec` frees its
/// storage), `010c4cf8`/`010c4d18` (`NiTMapBase` and `NiTMap<NiAVObject *, short>`) and `010c4dec`
/// (`hkaSkeleton`).
const ARRAY_ALLOCATE: u32 = 0x00aa_1070;
const ARRAY_FREE: u32 = 0x00aa_10f0;
const CRT_MEMSET: u32 = 0x00ec_61c0;
const HK_STRING_CONSTRUCT: u32 = 0x00cb_0a60;
const HK_STRING_DESTRUCT: u32 = 0x00cb_0c00;
const HK_STRING_ASSIGN: u32 = 0x00cb_0c10;
const STRING_DUPLICATE: u32 = 0x00cb_1410;
const SET_TO_REFERENCE_POSE: u32 = 0x00cd_a110;
const MAP_CLEAR: u32 = 0x00b6_01a0;
const BONE_ARRAY_VTABLE: u32 = 0x010c_4ce4;
const BONE_ARRAY_FREE_SLOT: u32 = 0x010c_4cec;
const NODE_MAP_BASE_VTABLE: u32 = 0x010c_4cf8;
const NODE_MAP_VTABLE: u32 = 0x010c_4d18;
const HKA_SKELETON_VTABLE: u32 = 0x010c_4dec;

/// Makes room for `wanted` elements of `element_size` bytes in the Havok array at `array` (data
/// `+0`, size `+4`, capacity and flags `+8`): when the capacity (`& 0x3fffffff`, signed
/// compare) is below `wanted`, the reserve function `00c908c0` of the router's allocator is
/// called with the larger of twice the capacity and `wanted`.
fn grow_array(e: &mut Engine, array: u32, wanted: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let mut capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < (wanted as i32) {
        capacity = capacity.wrapping_add(capacity);
        if (wanted as i32) >= (capacity as i32) {
            capacity = wanted;
        }
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, capacity, element_size],
        );
    }
}

/// An allocation of `size` bytes from the router's allocator (virtual slot 4).
fn allocator_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

// Translated from 00c7e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7e3b0 - the FS:[0] exception-unwinding frame is not modeled (the
// code has no /GS cookie check).
/// Register `ECX`, no name in the Xbox PDB or the Ghidra database. Without the node at `+0x58`
/// nothing happens. The collision object of that node (`00c802d0`) gives its node (`+8`), which
/// `fn_00c7b4f0` walks into a `BSSimpleArray` of bone records (0x60 bytes: name at `+0`, the
/// converted transform at `+0x10`, parent number at `+0x40`) and an `NiTMap` from node to number.
/// When `[this + 0x2a4]` has no skeleton at `+0xc`, one is built (an `hkaSkeleton` of 0x48 bytes
/// with five empty arrays): its bone array (8-byte entries: `hkStringPtr` name and a lock byte)
/// is grown to the record count, entries beyond the old size are constructed and ones above the
/// new count destroyed, its parent array (2-byte entries) and reference pose array (0x30-byte
/// entries) are grown; then each record gives its bone the name (`strDup` of the record's
/// text), the lock byte 1, its `+0x10` transform as reference pose and its `+0x40` word as parent.
/// The records' aligned blocks are given back (`MemoryManager::Deallocate` of the block less the
/// pad byte before it), as is the record array's storage. A 0x38-byte pose object is then built
/// on the skeleton (arrays at `+4`, `+0x10` and `+0x1c` sized to the bone count, the one at
/// `+0x2c` to the skeleton's `+0x34` count, zero-filled), stored in `[this + 0x88]` and reset
/// (`setToReferencePose`), and the controller's array of 0x30-byte transforms at `+0x94` is
/// sized to the bone count. The map is cleared twice (`00b601a0`) and its table freed
/// (`aa10f0`).
pub fn fn_00c7e3b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let root = e.mem.u32(base + 0x58);
    if root == 0 {
        return;
    }
    e.with_stack(0x20, |e, frame| {
        let records = frame.addr();
        let map = records + 0x10;
        let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![root]).u32();
        let node = e.mem.u32(collision + 8);
        e.mem.set_u32(records, BONE_ARRAY_VTABLE);
        e.mem.set_u32(records + 4, 0);
        e.mem.set_u32(records + 8, 0);
        e.mem.set_u32(records + 0xc, 0);
        e.mem.set_u32(map, NODE_MAP_BASE_VTABLE);
        e.mem.set_u32(map + 4, 0x25);
        e.mem.set_u32(map + 0xc, 0);
        let table = e.call(ARRAY_ALLOCATE, &args![0x94u32]).u32();
        e.mem.set_u32(map + 8, table);
        let table_bytes = e.mem.u32(map + 4).wrapping_mul(4);
        e.call(CRT_MEMSET, &args![table, 0u32, table_bytes]);
        e.mem.set_u32(map, NODE_MAP_VTABLE);
        fn_00c7b4f0(e, this, node, records, map);
        let mappers = e.mem.u32(base + 0x2a4);
        if e.mem.u32(mappers + 0xc) == 0 {
            build_skeleton(e, base, records);
        }
        let used = e.mem.u32(records + 8);
        for index in 0..used {
            let data = e.mem.u32(records + 4);
            let record = e.mem.u32(data + 4 * index);
            if record != 0 {
                let pad = u32::from(e.mem.u8(record - 1));
                e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, record - pad]);
            }
        }
        let data = e.mem.u32(records + 4);
        if data != 0 {
            e.vcall(records, 8, &args![data]);
            e.mem.set_u32(records + 4, 0);
            e.mem.set_u32(records + 0xc, 0);
            e.mem.set_u32(records + 8, 0);
        }
        let pose = build_pose(e, base);
        e.mem.set_u32(base + 0x88, pose);
        e.call(SET_TO_REFERENCE_POSE, &args![pose]);
        let mappers = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(mappers + 0xc);
        let bones = e.mem.u32(skeleton + 0x1c);
        grow_array(e, base + 0x94, bones, 0x30);
        e.mem.set_u32(base + 0x98, bones);
        e.mem.set_u32(map, NODE_MAP_VTABLE);
        e.call(MAP_CLEAR, &args![map]);
        e.mem.set_u32(map, NODE_MAP_BASE_VTABLE);
        e.call(MAP_CLEAR, &args![map]);
        let table = e.mem.u32(map + 8);
        e.call(ARRAY_FREE, &args![table]);
        let data = e.mem.u32(records + 4);
        e.mem.set_u32(records, BONE_ARRAY_VTABLE);
        if data != 0 {
            let free = e.mem.u32(BONE_ARRAY_FREE_SLOT);
            e.call(free, &args![records, data]);
        }
    });
}

/// The skeleton part of `00c7e3b0`: see there. `records` is the array of bone records (data
/// `+4`, count `+8`).
fn build_skeleton(e: &mut Engine, base: u32, records: u32) {
    let skeleton = allocator_block(e, 0x48);
    e.mem.set_u16(skeleton + 4, 0x48);
    e.mem.set_u16(skeleton + 6, 1);
    e.mem.set_u32(skeleton, HKA_SKELETON_VTABLE);
    e.call(HK_STRING_CONSTRUCT, &args![skeleton + 8]);
    for array in 0..5 {
        let at = skeleton + 0xc + 0xc * array;
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, 0x8000_0000);
    }
    let mappers = e.mem.u32(base + 0x2a4);
    e.mem.set_u32(mappers + 0xc, skeleton);
    let mappers = e.mem.u32(base + 0x2a4);
    let skeleton = e.mem.u32(mappers + 0xc);
    let count = e.mem.u32(records + 8);
    let bones = skeleton + 0x18;
    grow_array(e, bones, count, 8);
    let size = e.mem.u32(bones + 4);
    let data = e.mem.u32(bones);
    let surplus = size.wrapping_sub(count) as i32;
    if surplus > 0 {
        for index in 0..surplus as u32 {
            e.call(HK_STRING_DESTRUCT, &args![data + 8 * count + 8 * index]);
        }
    }
    let size = e.mem.u32(bones + 4);
    let data = e.mem.u32(bones);
    let missing = count.wrapping_sub(size) as i32;
    if missing > 0 {
        for index in 0..missing as u32 {
            let bone = data + 8 * size + 8 * index;
            if bone != 0 {
                e.call(HK_STRING_CONSTRUCT, &args![bone]);
                e.mem.set_u8(bone + 4, 0);
            }
        }
    }
    e.mem.set_u32(bones + 4, count);
    let mappers = e.mem.u32(base + 0x2a4);
    let skeleton = e.mem.u32(mappers + 0xc);
    grow_array(e, skeleton + 0xc, count, 2);
    e.mem.set_u32(skeleton + 0x10, count);
    let mappers = e.mem.u32(base + 0x2a4);
    let skeleton = e.mem.u32(mappers + 0xc);
    grow_array(e, skeleton + 0x24, count, 0x30);
    e.mem.set_u32(skeleton + 0x28, count);
    for index in 0..count {
        let mappers = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(mappers + 0xc);
        let bone = e.mem.u32(skeleton + 0x18) + 8 * index;
        let items = e.mem.u32(records + 4);
        let record = e.mem.u32(items + 4 * index);
        let name = e.mem.u32(record);
        let text = e.call(STRING_DUPLICATE, &args![name]).u32();
        e.call(HK_STRING_ASSIGN, &args![bone, text]);
        let mappers = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(mappers + 0xc);
        let bones = e.mem.u32(skeleton + 0x18);
        e.mem.set_u8(bones + 8 * index + 4, 1);
        let mappers = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(mappers + 0xc);
        let poses = e.mem.u32(skeleton + 0x24);
        let items = e.mem.u32(records + 4);
        let record = e.mem.u32(items + 4 * index);
        let pose = poses + 0x30 * index;
        copy16(e, record + 0x10, pose);
        copy16(e, record + 0x20, pose + 0x10);
        copy16(e, record + 0x30, pose + 0x20);
        let mappers = e.mem.u32(base + 0x2a4);
        let items = e.mem.u32(records + 4);
        let record = e.mem.u32(items + 4 * index);
        let skeleton = e.mem.u32(mappers + 0xc);
        let parents = e.mem.u32(skeleton + 0xc);
        let parent = e.mem.u16(record + 0x40);
        e.mem.set_u16(parents + 2 * index, parent);
    }
}

/// The pose part of `00c7e3b0`: a 0x38-byte block (answers 0 without one) holding the skeleton
/// `[[this + 0x2a4] + 0xc]` at `+0`, four empty arrays (at `+4`, `+0x10`, `+0x1c`, `+0x2c`;
/// the bytes `+0x28` and `+0x29` are cleared) and the arrays sized as `00c7e3b0` says.
fn build_pose(e: &mut Engine, base: u32) -> u32 {
    let pose = allocator_block(e, 0x38);
    if pose == 0 {
        return 0;
    }
    let mappers = e.mem.u32(base + 0x2a4);
    let skeleton = e.mem.u32(mappers + 0xc);
    e.mem.set_u32(pose, skeleton);
    e.mem.set_u32(pose + 4, 0);
    e.mem.set_u32(pose + 8, 0);
    e.mem.set_u32(pose + 0xc, 0x8000_0000);
    e.mem.set_u32(pose + 0x10, 0);
    e.mem.set_u32(pose + 0x14, 0);
    e.mem.set_u32(pose + 0x18, 0x8000_0000);
    e.mem.set_u32(pose + 0x1c, 0);
    e.mem.set_u32(pose + 0x20, 0);
    e.mem.set_u32(pose + 0x24, 0x8000_0000);
    e.mem.set_u8(pose + 0x28, 0);
    e.mem.set_u8(pose + 0x29, 0);
    e.mem.set_u32(pose + 0x2c, 0);
    e.mem.set_u32(pose + 0x30, 0);
    e.mem.set_u32(pose + 0x34, 0x8000_0000);
    let skeleton = e.mem.u32(pose);
    let count = e.mem.u32(skeleton + 0x1c);
    grow_array(e, pose + 0x10, count, 0x30);
    e.mem.set_u32(pose + 0x14, count);
    grow_array(e, pose + 4, count, 0x30);
    e.mem.set_u32(pose + 8, count);
    grow_array(e, pose + 0x1c, count, 4);
    e.mem.set_u32(pose + 0x20, count);
    let skeleton = e.mem.u32(pose);
    let rounded = e.mem.u32(skeleton + 0x34).wrapping_add(3) & 0xffff_fffc;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(pose + 0x34) & 0x3fff_ffff;
    if (capacity as i32) < (rounded as i32) {
        e.call(ARRAY_RESERVE, &args![allocator, pose + 0x2c, rounded, 4u32]);
    }
    let skeleton = e.mem.u32(pose);
    let slots = e.mem.u32(skeleton + 0x34);
    grow_array(e, pose + 0x2c, slots, 4);
    let size = e.mem.u32(pose + 0x30);
    let data = e.mem.u32(pose + 0x2c);
    let missing = slots.wrapping_sub(size) as i32;
    if missing > 0 {
        for index in 0..missing as u32 {
            e.mem.set_u32(data + 4 * size + 4 * index, 0);
        }
    }
    e.mem.set_u32(pose + 0x30, slots);
    pose
}

/// What `DoFootIK` calls and reads. Functions: `hkaFootPlacementIkSolver::Input::Input` (Ghidra
/// name, `00cd8a10`, thiscall on the input record), the foot solver `00cd8d90` (no name in the
/// Ghidra database; thiscall on the foot object, `(input, output, pose)`, the output an 8-byte
/// record whose first float is read), the timer `00aa12f0` (cdecl, answers a float), `hkQuaternion::decomposeRestAxis`
/// (Ghidra name, `00cb2ca0`, thiscall on the quaternion, `(rest axis, swing, angle)`),
/// `hkQuaternion::setAxisAngle` (Ghidra name, `00cb2450`, thiscall on the result, `(axis,
/// angle)`) and the vector normaliser `005611c0` (thiscall, in place). Data: the floats
/// `01267d48`, `01267d54`, `01267d60`, `01267d6c`, `01267d78` copied into the input record, the
/// floats `01267dc0` and `01267dd8` (height scale and limit), the double `01017b70` and the
/// float `01267c1c` (time accumulator), the blend factor `01016248`, the vector `010c7160`, the
/// floats `01267da8`, `01267db4`, `0102caf8`, `0101712c`, the double `010217d8` and the dword
/// `01267dcc` (number of steps of the decay).
const FOOT_INPUT_CONSTRUCT: u32 = 0x00cd_8a10;
const FOOT_SOLVE: u32 = 0x00cd_8d90;
const FOOT_TIMER: u32 = 0x00aa_12f0;
const DECOMPOSE_REST_AXIS: u32 = 0x00cb_2ca0;
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
const NORMALISE_VECTOR: u32 = 0x0056_11c0;
const FOOT_INPUT_FLOATS: [(u32, u32); 4] = [
    (0x0126_7d54, 0x6c),
    (0x0126_7d60, 0x70),
    (0x0126_7d6c, 0x78),
    (0x0126_7d78, 0x74),
];
const FOOT_INPUT_FIRST: u32 = 0x0126_7d48;
const FOOT_HEIGHT_SCALE: u32 = 0x0126_7dc0;
const FOOT_HEIGHT_LIMIT: u32 = 0x0126_7dd8;
const FOOT_TIME_SCALE: u32 = 0x0101_7b70;
const FOOT_TIME_TOTAL: u32 = 0x0126_7c1c;
const BLEND_FACTOR: u32 = 0x0101_6248;
const NEGATIVE_BLEND_BASE: u32 = 0x010c_7160;
const SELECTOR_DEFAULT: u32 = 0x0126_7da8;
const SMOOTHING_SCALE: u32 = 0x0126_7db4;
const OFFSET_LOW: u32 = 0x0102_caf8;
const OFFSET_HIGH: u32 = 0x0101_712c;
const OFFSET_DECAY: u32 = 0x0102_17d8;
const DECAY_STEPS: u32 = 0x0126_7dcc;

/// The bone at `offset` (bone number times 0x30) gets its output transform: the composition
/// with `parent` (`compose_with_parent`), or for a parent of `-1` a copy of the local entry.
fn compose_or_copy(
    e: &mut Engine,
    pose: u32,
    parent: u16,
    constants: &PoseConstants,
    local: u32,
    offset: u32,
) {
    if parent == 0xffff {
        let output = e.mem.u32(pose + 4).wrapping_add(offset);
        for lane_offset in [0, 0x10, 0x20] {
            copy16(
                e,
                local.wrapping_add(offset).wrapping_add(lane_offset),
                output.wrapping_add(lane_offset),
            );
        }
    } else {
        compose_with_parent(e, pose, parent, constants, local, offset);
    }
}

/// The output transform (`pose + 4` array, 0x30 bytes per bone) of `bone` (a sign-extended
/// index) after bringing it up to date: with bit 0 of the bone's flag clear it is as it is.
/// Otherwise the bone gets composed with its parent (`compose_with_parent`; a parent of `-1`
/// copies the local entry), and bit 0 is cleared. This is the code `DoFootIK` has inline
/// three times for the foot bone and once in the loop over the descendants.
fn synced_bone_output(e: &mut Engine, pose: u32, bone: u32, constants: &PoseConstants) -> u32 {
    let offset = bone.wrapping_mul(0x30);
    let flags = e.mem.u32(pose + 0x1c);
    if e.mem.u8(flags.wrapping_add(bone.wrapping_mul(4))) & 1 != 0 {
        let skeleton = e.mem.u32(pose);
        let parents = e.mem.u32(skeleton + 0xc);
        let parent = e.mem.u16(parents.wrapping_add(bone.wrapping_mul(2)));
        let local = e.mem.u32(pose + 0x10);
        compose_or_copy(e, pose, parent, constants, local, offset);
        let flags = e.mem.u32(pose + 0x1c);
        let at = flags.wrapping_add(bone.wrapping_mul(4));
        let word = e.mem.u32(at) & 0xffff_fffe;
        e.mem.set_u32(at, word);
    }
    e.mem.u32(pose + 4).wrapping_add(offset)
}

/// The bones after `bone` of `pose` once the transform of `bone` is known: the flag of `bone`
/// gets bit 2 (value 4); then every later bone whose parent is not `-1` and flagged 4 is
/// brought up to date if it is flagged 1 (`synced_bone_output`'s composition, then flag 1
/// cleared), gets flag 4 and clears the byte at `pose + 0x28`; at last the later bones flagged
/// 4 become flagged 2 instead. (`GetSGPoseLocalSpace` has the same loops inline.)
fn flag_descendants(e: &mut Engine, pose: u32, bone: u32, constants: &PoseConstants) {
    let skeleton = e.mem.u32(pose);
    let count = e.mem.i32(skeleton + 0x1c);
    let flags = e.mem.u32(pose + 0x1c);
    let at = flags.wrapping_add(bone.wrapping_mul(4));
    let word = e.mem.u32(at) | 4;
    e.mem.set_u32(at, word);
    let first = bone.wrapping_add(1);
    if (first as i32) < count {
        let mut current = first;
        let mut offset = first.wrapping_mul(0x30);
        loop {
            let skeleton = e.mem.u32(pose);
            let parents = e.mem.u32(skeleton + 0xc);
            let parent = e.mem.u16(parents.wrapping_add(current.wrapping_mul(2)));
            let flags = e.mem.u32(pose + 0x1c);
            let parent_index = i32::from(parent as i16) as u32;
            if parent != 0xffff
                && e.mem.u8(flags.wrapping_add(parent_index.wrapping_mul(4))) & 4 != 0
            {
                if e.mem.u8(flags.wrapping_add(current.wrapping_mul(4))) & 1 != 0 {
                    let local = e.mem.u32(pose + 0x10);
                    compose_or_copy(e, pose, parent, constants, local, offset);
                    let flags = e.mem.u32(pose + 0x1c);
                    let at = flags.wrapping_add(current.wrapping_mul(4));
                    let word = e.mem.u32(at) & 0xffff_fffe;
                    e.mem.set_u32(at, word);
                }
                let flags = e.mem.u32(pose + 0x1c);
                let at = flags.wrapping_add(current.wrapping_mul(4));
                let word = e.mem.u32(at) | 4;
                e.mem.set_u32(at, word);
                e.mem.set_u8(pose + 0x28, 0);
            }
            offset = offset.wrapping_add(0x30);
            current = current.wrapping_add(1);
            if (current as i32) >= count {
                break;
            }
        }
    }
    let mut later = first;
    while (later as i32) < count {
        let flags = e.mem.u32(pose + 0x1c);
        let at = flags.wrapping_add(later.wrapping_mul(4));
        let word = e.mem.u32(at);
        if word & 4 != 0 {
            e.mem.set_u32(at, (word & 0xffff_fffb) | 2);
        }
        later = later.wrapping_add(1);
    }
}

/// The three vectors of the transform at `address`.
fn load_transform(e: &Engine, address: u32) -> [Lanes; 3] {
    [
        load4(e, address),
        load4(e, address + 0x10),
        load4(e, address + 0x20),
    ]
}

/// Stores the three vectors of a transform at `address`.
fn store_transform(e: &mut Engine, address: u32, transform: &[Lanes; 3]) {
    for (index, lanes) in transform.iter().enumerate() {
        store4(e, address + 0x10 * index as u32, *lanes);
    }
}

// Translated from 00c76350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c76350 - the /GS cookie check (00ec408c) is not modeled (the code
// has no FS:[0] frame); the stack slots the solver `00cd8d90` may leave unwritten are zero here.
/// `bhkRagdollController::DoFootIK` (Ghidra name), thiscall `(transform)`, `transform` being
/// three vectors. An input record for the foot solver is built on the stack (`Input::Input`);
/// the transform goes to its `+0x30`, the floats at `+0x68`, `+0x6c`, `+0x70`, `+0x78` and
/// `+0x74` come from `01267d48` (1.0 when byte `+0x223` is set), `01267d54`, `01267d60`,
/// `01267d6c` and `01267d78` (quiet copies), the byte `+0x60` is 1 when byte `+0x223` is clear
/// and byte `+0x221` clear, and the controller goes to `+0x64`. The root bone's transform
/// (`root_bone_transform`) has its height replaced: with `h` the old height, `o` the float at
/// `+0x23c`, `s = h + o` and `a = scale * o + h` (floats), `a` is pulled to `limit + s` when
/// `a - s` is above `limit` and to `s - limit` when it is below `-limit` (`limit` from
/// `01267dd8`); `a` becomes the root's height, and the vector `(fill, fill, a - h, fill)` is
/// added to the transform at the input's `+0x30`. The root bone's local transform is written
/// back, its flag set to 1 and its descendants flagged (`flag_descendants`).
///
/// For each of the `[this + 0x230]` feet (0xd0-byte records at `[this + 0x22c]` starting with
/// the foot object, which holds the bone index at `+0x18`, a saved vector at `+0x30` and the
/// weight `[this + 0x244]` at `+0x80`): the vector at `+0x30` is saved and, with byte `+0x241`
/// clear, zeroed; the foot bone's transform is brought up to date and remembered; the bone's
/// model space transform (or local one) is the input's first transform; the foot index
/// goes to `+0x23a`, the ray record index `+0x238` is cleared and the solver `00cd8d90` runs
/// between two timer readings, the time being accumulated at `01267c1c`; its float goes to
/// `+4` of the record. The bone's transform is brought up to date again; with byte `+0x240`
/// set the record's transform at `+0x90` is replaced by it when byte `+0x242` is set, a
/// rotation constraint with kind 2 (`+0xc0` selects the entry of the lists at `[this + 0x48]`;
/// kind 13 is unwrapped) limits the rotation (`decomposeRestAxis`, `setAxisAngle` with the rest
/// axis `((byte data + 0xc3) + 2) * 16`) and the transform is blended with the record's one
/// (`1 - f` against `f`, `f` the blend factor `01016248`, the rotations by their sign-corrected
/// dot product and then normalised) and stored in the record. The foot bone's output transform
/// becomes this (or the remembered one), its flag 2, its descendants are flagged, and the saved
/// vector is put back. After the feet: byte `+0x242` is cleared; with the weights of the first
/// two records `a`, `b` (`low`, `high` their minimum and maximum), `selector` 1.0 (byte `+0x243`)
/// or `01267da8`, a clear byte `+0x221` adds `((1 - selector) * high + selector * low) *
/// 01267db4` to `+0x23c`; that float is then kept above the float at `0102caf8` and below the
/// one at `0101712c`. With byte `+0x221` set it is scaled by the double `010217d8` and the step
/// counter `+0x228` is incremented, and when the counter equals the dword `01267dcc` it and the
/// bytes `+0x221` and `+0x222` are cleared and byte `+0x242` is set. Byte `+0x223` is cleared.
pub fn bhk_ragdoll_controller_do_foot_ik(e: &mut Engine, this: Ptr, transform: u32) {
    let base = this.addr();
    let constants = load_pose_constants(e);
    e.with_stack(0x90, |e, input| {
        let input = input.addr();
        e.call(FOOT_INPUT_CONSTRUCT, &args![input]);
        for lane_offset in [0, 0x10, 0x20] {
            copy16(e, transform + lane_offset, input + 0x30 + lane_offset);
        }
        let flag = e.mem.u8(base + 0x223);
        let first = if flag == 0 {
            x87_quiet_bits(e.mem.u32(FOOT_INPUT_FIRST))
        } else {
            1.0f32.to_bits()
        };
        e.mem.set_u32(input + 0x68, first);
        for (source, field) in FOOT_INPUT_FLOATS {
            let bits = x87_quiet_bits(e.mem.u32(source));
            e.mem.set_u32(input + field, bits);
        }
        let idle = if flag == 0 {
            u8::from(e.mem.u8(base + 0x221) == 0)
        } else {
            0
        };
        e.mem.set_u8(input + 0x60, idle);
        e.mem.set_u32(input + 0x64, base);
        let pose = e.mem.u32(base + 0x88);
        let root = root_bone_transform(e, pose);
        let mut saved = load_transform(e, root);
        let height = x87_quiet(saved[0][2]);
        let offset = e.mem.f32(base + 0x23c);
        let shifted = (f64::from(height) + f64::from(offset)) as f32;
        let scale = e.mem.f32(FOOT_HEIGHT_SCALE);
        let mut lifted = (f64::from(scale) * f64::from(offset) + f64::from(height)) as f32;
        let delta = (f64::from(lifted) - f64::from(shifted)) as f32;
        let limit = x87_quiet(e.mem.f32(FOOT_HEIGHT_LIMIT));
        if limit < delta {
            lifted = (f64::from(limit) + f64::from(shifted)) as f32;
        }
        if -limit > delta {
            lifted = (f64::from(shifted) - f64::from(limit)) as f32;
        }
        let rise = (f64::from(lifted) - f64::from(height)) as f32;
        saved[0][2] = f32::from_bits(x87_quiet_bits(lifted.to_bits()));
        let fill = e.mem.f32(VECTOR_FILL);
        let target = addps([fill, fill, rise, fill], load4(e, input + 0x30));
        store4(e, input + 0x30, target);
        flag_descendants(e, pose, 0, &constants);
        let local = e.mem.u32(pose + 0x10);
        store_transform(e, local, &saved);
        let flags = e.mem.u32(pose + 0x1c);
        e.mem.set_u32(flags, 1);
        e.mem.set_u8(pose + 0x29, 0);
        let mut counter = 0u32;
        if e.mem.i32(base + 0x230) > 0 {
            let mut record = 0u32;
            loop {
                do_foot_ik_foot(e, base, input, record, counter, &constants);
                counter += 1;
                record += 0xd0;
                if counter as i32 >= e.mem.i32(base + 0x230) {
                    break;
                }
            }
        }
    });
    if e.mem.u8(base + 0x242) != 0 {
        e.mem.set_u8(base + 0x242, 0);
    }
    let selector = if e.mem.u8(base + 0x243) != 0 {
        1.0
    } else {
        x87_quiet(e.mem.f32(SELECTOR_DEFAULT))
    };
    let records = e.mem.u32(base + 0x22c);
    let first = x87_quiet(e.mem.f32(records + 4));
    let second = x87_quiet(e.mem.f32(records + 0xd4));
    let low = if first < second { first } else { second };
    let high = if second < first { first } else { second };
    let tracking = e.mem.u8(base + 0x221);
    if tracking == 0 {
        let mixed = ((1.0 - f64::from(selector)) * f64::from(high)
            + f64::from(low) * f64::from(selector)) as f32;
        let smoothing = e.mem.f32(SMOOTHING_SCALE);
        let scaled = (f64::from(mixed) * f64::from(smoothing)) as f32;
        let current = e.mem.f32(base + 0x23c);
        e.mem.set_f32(
            base + 0x23c,
            (f64::from(scaled) + f64::from(current)) as f32,
        );
    }
    let value = x87_quiet(e.mem.f32(base + 0x23c));
    let lower = e.mem.f32(OFFSET_LOW);
    let limited = if value > lower { value } else { lower };
    let upper = e.mem.f32(OFFSET_HIGH);
    let limited = if limited < upper { limited } else { upper };
    e.mem.set_f32(base + 0x23c, limited);
    if tracking == 0 {
        e.mem.set_u8(base + 0x223, 0);
        return;
    }
    let decayed = (f64::from(limited) * e.mem.f64(OFFSET_DECAY)) as f32;
    let steps = e.mem.u32(base + 0x228).wrapping_add(1);
    e.mem.set_u32(base + 0x228, steps);
    e.mem.set_f32(base + 0x23c, decayed);
    e.mem.set_u8(base + 0x223, 0);
    if e.mem.u32(DECAY_STEPS) == steps {
        e.mem.set_u32(base + 0x228, 0);
        e.mem.set_u8(base + 0x221, 0);
        e.mem.set_u8(base + 0x222, 0);
        e.mem.set_u8(base + 0x242, 1);
    }
}

/// One foot of `DoFootIK`: `record` is the byte offset of the foot's 0xd0-byte record in the
/// array at `[this + 0x22c]`, `counter` the foot's number, `input` the solver's input record.
fn do_foot_ik_foot(
    e: &mut Engine,
    base: u32,
    input: u32,
    record: u32,
    counter: u32,
    constants: &PoseConstants,
) {
    let records = e.mem.u32(base + 0x22c);
    let object = e.mem.u32(records.wrapping_add(record));
    let kept = load4(e, object + 0x30);
    let weight = x87_quiet_bits(e.mem.u32(base + 0x244));
    e.mem.set_u32(object + 0x80, weight);
    if e.mem.u8(base + 0x241) == 0 {
        let records = e.mem.u32(base + 0x22c);
        let object = e.mem.u32(records.wrapping_add(record));
        store4(e, object + 0x30, [0.0; 4]);
    }
    let records = e.mem.u32(base + 0x22c);
    let object = e.mem.u32(records.wrapping_add(record));
    let bone = i32::from(e.mem.i16(object + 0x18)) as u32;
    let pose = e.mem.u32(base + 0x88);
    let output = synced_bone_output(e, pose, bone, constants);
    let mut current = load_transform(e, output);
    let records = e.mem.u32(base + 0x22c);
    let object = e.mem.u32(records.wrapping_add(record));
    let again = i32::from(e.mem.i16(object + 0x18)) as u32;
    let pose = e.mem.u32(base + 0x88);
    let flags = e.mem.u32(pose + 0x1c);
    let source = if e.mem.u8(flags.wrapping_add(again.wrapping_mul(4))) & 2 == 0 {
        again
            .wrapping_mul(0x30)
            .wrapping_add(e.mem.u32(pose + 0x10))
    } else {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, again])
            .u32()
    };
    for lane_offset in [0, 0x10, 0x20] {
        copy16(e, source + lane_offset, input + lane_offset);
    }
    e.mem.set_u16(base + 0x23a, counter as u16);
    e.mem.set_u16(base + 0x238, 0);
    let started = e.call(FOOT_TIMER, &args![]).f32();
    let result = e.with_stack(8, |e, output| {
        let output = output.addr();
        let pose = e.mem.u32(base + 0x88);
        let records = e.mem.u32(base + 0x22c);
        let solver = e.mem.u32(records.wrapping_add(record));
        e.call(FOOT_SOLVE, &args![solver, input, output, pose]);
        e.mem.u32(output)
    });
    let finished = e.call(FOOT_TIMER, &args![]).f32();
    let records = e.mem.u32(base + 0x22c);
    e.mem.set_u32(
        records.wrapping_add(record).wrapping_add(4),
        x87_quiet_bits(result),
    );
    let total = e.mem.f32(FOOT_TIME_TOTAL);
    let scale = e.mem.f64(FOOT_TIME_SCALE);
    let elapsed = (f64::from(finished) - f64::from(started)) * scale;
    e.mem
        .set_f32(FOOT_TIME_TOTAL, (elapsed + f64::from(total)) as f32);
    let pose = e.mem.u32(base + 0x88);
    let output = synced_bone_output(e, pose, bone, constants);
    let mut solved = load_transform(e, output);
    if e.mem.u8(base + 0x240) != 0 {
        let records = e.mem.u32(base + 0x22c);
        let stored = records.wrapping_add(record).wrapping_add(0x90);
        if e.mem.u8(base + 0x242) != 0 {
            store_transform(e, stored, &solved);
        }
        let constraint = e.mem.u16(stored + 0x30);
        if constraint != 0xffff {
            let owner = e.mem.u32(base + 0x48);
            let table = e.mem.u32(owner + 0x20);
            let slot = e
                .mem
                .u32(table.wrapping_add((i32::from(constraint as i16) as u32).wrapping_mul(4)))
                .wrapping_sub(1);
            let entry = if (slot as i32) < 0 {
                0
            } else {
                let array = e.mem.u32(owner + 0x14);
                e.mem.u32(array.wrapping_add(slot.wrapping_mul(4)))
            };
            let mut data = e.mem.u32(entry + 0xc);
            if e.vcall(data, DATA_KIND_SLOT, &args![]).u32() == 0xd {
                data = e.call(WRAPPED_DATA, &args![data]).u32();
            }
            if e.vcall(data, DATA_KIND_SLOT, &args![]).u32() == 2 {
                let axis = load4(e, data + ((u32::from(e.mem.u8(data + 0xc3)) + 2) << 4));
                let limited = e.with_stack(0x50, |e, frame| {
                    let frame = frame.addr();
                    store4(e, frame, solved[1]);
                    store4(e, frame + 0x10, axis);
                    e.call(
                        DECOMPOSE_REST_AXIS,
                        &args![frame, frame + 0x10, frame + 0x20, frame + 0x40],
                    );
                    let angle = x87_quiet_bits(e.mem.u32(frame + 0x40));
                    e.call(SET_AXIS_ANGLE, &args![frame, frame + 0x10, angle]);
                    load4(e, frame)
                });
                solved[1] = limited;
            }
        }
        let records = e.mem.u32(base + 0x22c);
        let stored = records.wrapping_add(record).wrapping_add(0x90);
        let blend = e.mem.f32(BLEND_FACTOR);
        let ones = load4(e, ONE_VECTOR);
        let splat = [blend; 4];
        let remaining = subps(ones, splat);
        let old = load_transform(e, stored);
        let third = addps(mulps(remaining, solved[2]), mulps(old[2], splat));
        let first = addps(mulps(remaining, solved[0]), mulps(splat, old[0]));
        let products = mulps(solved[1], old[1]);
        let pairs = addps(shufps(products, products, 0x4e), products);
        let dot = addps(shufps(pairs, pairs, 0xb1), pairs)[0];
        let complement = (1.0 - f64::from(x87_quiet(blend))) as f32;
        let chosen = if dot < 0.0 {
            let base_vector = load4(e, NEGATIVE_BLEND_BASE);
            x87_quiet(base_vector[0] - blend)
        } else {
            x87_quiet(blend)
        };
        let mixed = addps(
            mulps([complement; 4], solved[1]),
            mulps([chosen; 4], old[1]),
        );
        let rotation = e.with_stack(0x10, |e, vector| {
            let vector = vector.addr();
            store4(e, vector, mixed);
            e.call(NORMALISE_VECTOR, &args![vector]);
            load4(e, vector)
        });
        current = [first, rotation, third];
        let records = e.mem.u32(base + 0x22c);
        store_transform(e, records.wrapping_add(record).wrapping_add(0x90), &current);
    }
    let pose = e.mem.u32(base + 0x88);
    flag_descendants(e, pose, bone, constants);
    let output = e.mem.u32(pose + 4).wrapping_add(bone.wrapping_mul(0x30));
    store_transform(e, output, &current);
    let flags = e.mem.u32(pose + 0x1c);
    e.mem.set_u32(flags.wrapping_add(bone.wrapping_mul(4)), 2);
    e.mem.set_u8(pose + 0x28, 0);
    let records = e.mem.u32(base + 0x22c);
    let object = e.mem.u32(records.wrapping_add(record));
    store4(e, object + 0x30, kept);
}

/// A scratch array of the setup (`data`, `size`, capacity word, `backup`) as `Init` builds it on
/// its stack: empty, then given a 0x100-byte chunk from the allocation cursor (data and backup
/// are the chunk, the capacity word is `0x80000040`).
fn take_scratch_array(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let chunk = e.mem.u32(router);
    e.mem.set_u32(router, chunk.wrapping_add(0x100));
    e.mem.set_u32(array, chunk);
    e.mem.set_u32(array + 8, 0x8000_0040);
    e.mem.set_u32(array + 0xc, chunk);
}

/// The destructor of such an array: an array still in its chunk is emptied, the cursor goes
/// back to the chunk, and a block the array grew into (capacity word without bit 31) is given
/// back through the allocator's slot `0x10` with `(data, capacity * 4)`.
fn release_scratch_array(e: &mut Engine, array: u32) {
    let backup = e.mem.u32(array + 0xc);
    if e.mem.u32(array) == backup {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, backup);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Allocates one object from the allocator of the memory router (slot 4, `(size)`).
fn allocate_from_router(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

// Translated from 00c7e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7e9a0 - the FS:[0] exception-unwinding frame and the /GS cookie
// check (00ec408c) are not modeled.
/// `bhkRagdollController::Init` (Ghidra name), thiscall `(node, name)`. The name string at
/// `+0x2a0` becomes a counted copy of `name` (`00a5b690`; the old string released with the
/// interlocked decrement at `fdf1c4`); with `name` 0 the string held at `node + 8` is taken over
/// instead. The share data object for the name is looked up with `00e92760` (thiscall on
/// `011b0300`, `(name, this + 0x2a4)`); when none exists a 0x20-byte object (`00a5d3a0`, vtable
/// `010c4b74`, fields `+8`, `+0xc`, `+0x10`, `+0x14`, `+0x1c` zero) is allocated (`00aa13e0`),
/// stored as the counted pointer at `+0x2a4` and registered with `00b582b0` (thiscall on
/// `011b0300`, `(name, pointer)`). Without a collision object for `node` (`00c802d0`) nothing
/// else happens. Otherwise `+0x58` becomes the world from the collision object's node
/// (`[[collision + 8] + 0x18]`), the bone nodes are collected into two scratch arrays taken
/// from the allocation cursor (`fn_00c7b140`), reordered for the ragdoll (`00cea030`, answer 1
/// ends the setup), and, when the share data has no skeleton (`+8`), built (`00ceb2c0`, stored
/// at `+8`) with the Havok transforms of the skeleton's `+0x24` set from the node's
/// transform and the one composed with the collision object's rotation (`00653290`, `00cb2d90`,
/// `00624ad0`, `0062c250`). With a skeleton the ragdoll instance is created (`00cea810`, `+0x48`),
/// the bone containing the string at `0101f604` gives `+0x2b8`, the rigid body controller
/// (`00cdb400`, `+0x4c`) gets twelve quiet floats from `01267c7c` (stride 0xc) and byte `+0x42`
/// is set, and the grab IK setup runs (`+0x21c`). `fn_00c7e3b0` runs, and when the share data
/// has neither mapping (`+0x10`, `+0x14`) the bytes at `+4` of the structures `[[+0xc] + 0x18]`
/// and `[[+8] + 0x18]` are cleared and the two mappings are created (`00ce5310`, `00ce61c0`,
/// `00ce4a20`, `00ce5370`) and the mappers checked (`00c79e20`). The scratch arrays are released.
///
/// Stack layout kept: the arrays at `+0x2c` and `+0x1c` (second one released first), the
/// transform at `+0x3c`, its copy at `+0x70`, the parameters at `+0xa4` (the Havok buffer and
/// the transform result reuse that area) and the mapping buffers at `+0x104` and `+0x174`.
pub fn bhk_ragdoll_controller_init(e: &mut Engine, this: Ptr, node: u32, name: u32) {
    const COLLISION_CAST_TYPE: u32 = 0x0126_83c8;
    const HEAD_BONE_NAME: u32 = 0x0101_f604;
    const ALLOCATE_OBJECT: u32 = 0x00aa_13e0;
    const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
    const REGISTER_SHARE: u32 = 0x00b5_82b0;
    const REORDER_FOR_RAGDOLL: u32 = 0x00ce_a030;
    const CONSTRUCT_SKELETON: u32 = 0x00ce_b2c0;
    const CREATE_INSTANCE: u32 = 0x00ce_a810;
    const MAKE_IDENTITY: u32 = 0x00a8_6bf0;
    const QUATERNION_TO_MATRIX: u32 = 0x00cb_2d90;
    const COMPOSE_ROTATION: u32 = 0x0062_4ad0;
    const NODE_TRANSFORM_FETCH: u32 = 0x0062_c250;
    const STRING_SEARCH: u32 = 0x00ec_7750;
    const RIGID_BODY_CONTROLLER_CONSTRUCT: u32 = 0x00cd_b400;
    const MAPPING_BUFFER_CONSTRUCT: u32 = 0x00ce_5310;
    const MAPPING_BUFFER_DESTRUCT: u32 = 0x00ce_5370;
    const CREATE_MAPPING: u32 = 0x00ce_61c0;
    const MAPPING_CONSTRUCT: u32 = 0x00ce_4a20;
    let base = this.addr();
    // The name string.
    let previous = e.mem.u32(base + 0x2a0);
    if previous != name {
        let copy = e.call(STRING_COPY, &args![name]).u32();
        e.mem.set_u32(base + 0x2a0, copy);
        if previous != e.mem.u32(NULL_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![previous.wrapping_sub(8)]);
        }
    }
    if name == 0 {
        let held = e.mem.u32(node + 8);
        if e.mem.u32(base + 0x2a0) != held {
            if held != e.mem.u32(NULL_STRING) {
                e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_sub(8)]);
            }
            let current = e.mem.u32(base + 0x2a0);
            if current != e.mem.u32(NULL_STRING) {
                e.call(INTERLOCKED_DECREMENT, &args![current.wrapping_sub(8)]);
            }
            e.mem.set_u32(base + 0x2a0, held);
        }
    }
    // The share data for the name.
    let share_slot = base + 0x2a4;
    let name_word = e.mem.u32(base + 0x2a0);
    e.call(
        POSE_LOADER_LOAD,
        &args![0x011b_0300u32, name_word, share_slot],
    );
    if e.mem.u32(share_slot) == 0 {
        let object = e.call(ALLOCATE_OBJECT, &args![0x20u32]).u32();
        let object = if object != 0 {
            e.call(OBJECT_CONSTRUCT, &args![object]);
            e.mem.set_u32(object, 0x010c_4b74);
            for offset in [8, 0xc, 0x10, 0x14, 0x1c] {
                e.mem.set_u32(object + offset, 0);
            }
            object
        } else {
            0
        };
        let old = e.mem.u32(share_slot);
        if old != object {
            if old != 0 {
                release_reference(e, old);
            }
            e.mem.set_u32(share_slot, object);
            if object != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
            }
        }
        let pointer = e.mem.u32(share_slot);
        if pointer != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![pointer + 4]);
        }
        let name_word = e.mem.u32(base + 0x2a0);
        e.call(REGISTER_SHARE, &args![0x011b_0300u32, name_word, pointer]);
    }
    // The collision object of the node.
    if e.call(FIND_NEXT_COLLISION_OBJECT, &args![node]).u32() == 0 {
        return;
    }
    let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![node]).u32();
    let collision_node = e.mem.u32(collision + 8);
    let world = e.mem.u32(collision_node + 0x18);
    e.mem.set_u32(base + 0x58, world);
    e.with_stack(0x200, |e, frame| {
        let frame = frame.addr();
        let first = frame + 0x2c;
        let second = frame + 0x1c;
        take_scratch_array(e, first);
        take_scratch_array(e, second);
        fn_00c7b140(e, collision_node, first, second);
        if e.call(REORDER_FOR_RAGDOLL, &args![first, second]).u32() != 1 {
            let share = e.mem.u32(share_slot);
            if e.mem.u32(share + 8) == 0 {
                let skeleton = e.call(CONSTRUCT_SKELETON, &args![first, second]).u32();
                e.mem.set_u32(share + 8, skeleton);
                let skeleton_transform = e.mem.u32(skeleton + 0x24);
                ni2hk_ov2(
                    e,
                    Ptr::new(skeleton_transform),
                    Ptr::new(collision_node + 0x34),
                );
                let object = e.mem.u32(collision + 0x10);
                let cast = if object == 0 {
                    0
                } else {
                    e.call(CAST_OBJECT, &args![object, COLLISION_CAST_TYPE])
                        .u32()
                };
                let transform = frame + 0x3c;
                let buffer = frame + 0xa4;
                e.call(MAKE_IDENTITY, &args![transform]);
                if cast != 0 {
                    for offset in [0, 4, 8, 0xc] {
                        let word = e.mem.u32(cast + 0x30 + offset);
                        e.mem.set_u32(buffer + 0x30 + offset, word);
                    }
                    e.call(QUATERNION_TO_MATRIX, &args![buffer, cast + 0x20]);
                    e.call(COMPOSE_ROTATION, &args![transform, buffer]);
                }
                let result = e
                    .call(
                        NODE_TRANSFORM_FETCH,
                        &args![collision_node + 0x34, buffer, transform],
                    )
                    .u32();
                let copy = frame + 0x70;
                for index in 0..13 {
                    let word = e.mem.u32(result + 4 * index);
                    e.mem.set_u32(copy + 4 * index, word);
                }
                let share = e.mem.u32(share_slot);
                let skeleton = e.mem.u32(share + 8);
                let skeleton_transform = e.mem.u32(skeleton + 0x24);
                ni2hk_ov2(e, Ptr::new(skeleton_transform), Ptr::new(copy));
            }
            let share = e.mem.u32(share_slot);
            let skeleton = e.mem.u32(share + 8);
            if skeleton != 0 {
                let instance = e
                    .call(CREATE_INSTANCE, &args![skeleton, first, second])
                    .u32();
                e.mem.set_u32(base + 0x48, instance);
                let mut index = 0u32;
                while (index as i32) < e.mem.u32(skeleton + 0x1c) as i32 {
                    let bones = e.mem.u32(skeleton + 0x18);
                    let bone_name =
                        e.mem.u32(bones.wrapping_add(index.wrapping_mul(8))) & 0xffff_fffe;
                    if e.call(STRING_SEARCH, &args![bone_name, HEAD_BONE_NAME])
                        .u32()
                        != 0
                    {
                        e.mem.set_u32(base + 0x2b8, index);
                    }
                    index = index.wrapping_add(1);
                }
                if e.mem.u32(base + 0x48) != 0 {
                    let block = allocate_from_router(e, 0x48);
                    let controller = if block != 0 {
                        let instance = e.mem.u32(base + 0x48);
                        e.call(RIGID_BODY_CONTROLLER_CONSTRUCT, &args![block, instance])
                            .u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(base + 0x4c, controller);
                    let target = e.mem.u32(controller);
                    for index in 0..12 {
                        let source = 0x0126_7c7c + 0xc * index;
                        copy_quiet(e, source, target + 4 * index);
                    }
                    e.mem.set_u8(base + 0x42, 1);
                    let grab = bhk_ragdoll_controller_init_grab_ik(e, this);
                    e.mem.set_u8(base + 0x21c, grab);
                }
            }
            fn_00c7e3b0(e, this);
            let share = e.mem.u32(share_slot);
            if e.mem.u32(share + 0x10) == 0 && e.mem.u32(share + 0x14) == 0 {
                let structure = e.mem.u32(e.mem.u32(share + 0xc) + 0x18);
                e.mem.set_u8(structure + 4, 0);
                let structure = e.mem.u32(e.mem.u32(e.mem.u32(share_slot) + 8) + 0x18);
                e.mem.set_u8(structure + 4, 0);
                let params = frame + 0xa4;
                let first_buffer = frame + 0x174;
                let second_buffer = frame + 0x104;
                e.call(MAPPING_BUFFER_CONSTRUCT, &args![first_buffer]);
                e.call(MAPPING_BUFFER_CONSTRUCT, &args![second_buffer]);
                fn_00c7bfd0(e, Ptr::new(params));
                let share = e.mem.u32(share_slot);
                let word = e.mem.u32(share + 0xc);
                e.mem.set_u32(params, word);
                let word = e.mem.u32(share + 8);
                e.mem.set_u32(params + 4, word);
                e.mem.set_u8(params + 0x39, 0);
                e.call(CREATE_MAPPING, &args![params, first_buffer, second_buffer]);
                fn_00c7c2b0(e, Ptr::new(params));
                bhk_ragdoll_controller_check_mappers_for_warnings(e, this);
                let block = allocate_from_router(e, 0x80);
                e.mem.set_u16(block + 4, 0x80);
                let mapping = e.call(MAPPING_CONSTRUCT, &args![block, first_buffer]).u32();
                let share = e.mem.u32(share_slot);
                e.mem.set_u32(share + 0x10, mapping);
                let block = allocate_from_router(e, 0x80);
                e.mem.set_u16(block + 4, 0x80);
                let mapping = e
                    .call(MAPPING_CONSTRUCT, &args![block, second_buffer])
                    .u32();
                let share = e.mem.u32(share_slot);
                e.mem.set_u32(share + 0x14, mapping);
                e.call(MAPPING_BUFFER_DESTRUCT, &args![second_buffer]);
                e.call(MAPPING_BUFFER_DESTRUCT, &args![first_buffer]);
            }
        }
        release_scratch_array(e, second);
        release_scratch_array(e, first);
    });
}

/// `x87 FISTP qword` with truncation (the control word rounds toward zero): the integer, or the
/// "integer indefinite" value `i64::MIN` for NaN and for values that do not fit.
fn fistp_truncate_i64(value: f64) -> i64 {
    if value.is_nan() || value >= i64::MAX as f64 || value < i64::MIN as f64 {
        i64::MIN
    } else {
        value.trunc() as i64
    }
}

/// `FILD dword` of a value the code treats as unsigned: a negative signed value gets the float
/// at `01099378` added (2^32).
fn fild_unsigned(e: &Engine, value: u32) -> f64 {
    let signed = value as i32;
    let mut result = f64::from(signed);
    if signed < 0 {
        result += f64::from(e.global::<f32>(0x0109_9378));
    }
    result
}

/// The two counters `00c75910` and `00c759f0` set in the controller: `+0x29c` from the
/// settings field `+0x38` and `+0x298` from `+0x3c`, each the double at `010290b0` times the
/// field taken as unsigned, truncated to 64 bits (the low half is stored).
fn store_scaled_counters(e: &mut Engine, controller: u32, settings: u32) {
    let factor: f64 = e.global(0x0102_90b0);
    let first = e.mem.u32(settings + 0x38);
    let first = fistp_truncate_i64(factor * fild_unsigned(e, first));
    e.mem.set_u32(controller + 0x29c, first as u32);
    let second = e.mem.u32(settings + 0x3c);
    let second = fistp_truncate_i64(factor * fild_unsigned(e, second));
    e.mem.set_u32(controller + 0x298, second as u32);
}

// Translated from 00c75850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::IsPoweredRagdoll` (Ghidra name), register `ECX`: with the constraint list
/// at `+0x48` (a pointer list: array `+0x14`, signed count `+0x18`), answers whether the list
/// exists and every constraint passes. Each constraint's data (`[entry + 0xc]`) is asked for its
/// type (slot `0x20`, once, twice or three times as the tests go on): 13 and 12 fail; type 2
/// fails when its dword at `+0xb0` is 0; type 7 needs the three getters `00cdea20`, `00cdea80`
/// and `00cdeaa0` (thiscall on the data) to answer non-zero; other types pass.
pub fn bhk_ragdoll_controller_is_powered_ragdoll(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let list = e.mem.u32(base + 0x48);
    if list == 0 {
        return false;
    }
    let count = e.mem.i32(list + 0x18);
    let mut ok = true;
    let mut index = 0;
    while index < count {
        if !ok {
            return false;
        }
        let array = e.mem.u32(list + 0x14);
        let entry = e
            .mem
            .u32(array.wrapping_add((index as u32).wrapping_mul(4)));
        let data = e.mem.u32(entry + 0xc);
        if e.vcall(data, 0x20, &args![]).u32() == 0xd || e.vcall(data, 0x20, &args![]).u32() == 0xc
        {
            ok = false;
        } else {
            match e.vcall(data, 0x20, &args![]).u32() {
                2 => {
                    if e.mem.u32(data + 0xb0) == 0 {
                        ok = false;
                    }
                }
                7 => {
                    for getter in [0x00cd_ea20, 0x00cd_ea80, 0x00cd_eaa0] {
                        if e.call(getter, &args![data]).u32() == 0 {
                            ok = false;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        index += 1;
    }
    ok && e.mem.u32(base + 0x48) != 0
}

// Translated from 00c75910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: copies the settings block `[[this + 0x2a4] + 0x18]` (nothing when it is
/// null) into the target record `[[this + 0x4c]]`, scaling: `+0xc` gets the float `+0x10` times
/// the double at `010290d0`, `+0x10` the float `+0xc` times the double at `0101ffa0` and `+0x1c`
/// the float `+0x18` times the double at `01012060` (each product narrowed to a float). It then
/// stores the scaled counters of `store_scaled_counters` into the controller.
pub fn fn_00c75910(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let owner = e.mem.u32(base + 0x2a4);
    let settings = e.mem.u32(owner + 0x18);
    if settings == 0 {
        return;
    }
    let target = e.mem.u32(e.mem.u32(base + 0x4c));
    for (to, from, factor) in [
        (0xc, 0x10, 0x0102_90d0),
        (0x10, 0xc, 0x0101_ffa0),
        (0x1c, 0x18, 0x0101_2060),
    ] {
        let scale: f64 = e.global(factor);
        let value = f64::from(e.mem.f32(settings + from));
        e.mem.set_f32(target + to, (value * scale) as f32);
    }
    store_scaled_counters(e, base, settings);
}

// Translated from 00c759f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: like `00c75910` but without the scaling of the three floats: it copies
/// the settings floats `+0x10`, `+0xc`, `+0x18` to the target record's `+0xc`, `+0x10`, `+0x1c`
/// (quiet copies), the float `+4` to `this + 0x290`, and stores the scaled counters.
pub fn fn_00c759f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let owner = e.mem.u32(base + 0x2a4);
    let settings = e.mem.u32(owner + 0x18);
    if settings == 0 {
        return;
    }
    let target = e.mem.u32(e.mem.u32(base + 0x4c));
    copy_quiet(e, settings + 0x10, target + 0xc);
    copy_quiet(e, settings + 0xc, target + 0x10);
    copy_quiet(e, settings + 0x18, target + 0x1c);
    copy_quiet(e, settings + 4, base + 0x290);
    store_scaled_counters(e, base, settings);
}

// Translated from 00c75ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`, argument the settings block: stores it at `[this + 0x2a4] + 0x18`, sets the
/// byte `this + 0x28c` to 1, copies its first byte to `this + 0x28d` and copies twelve of its
/// floats (quiet copies) to the target record `[[this + 0x4c]]`: record offset from settings
/// offset `0 <- 8, 4 <- 0x1c, 8 <- 0x14, 0xc <- 0x10, 0x10 <- 0xc, 0x14 <- 0x30, 0x18 <- 0x34,
/// 0x1c <- 0x18, 0x20 <- 0x20, 0x24 <- 0x24, 0x28 <- 0x28, 0x2c <- 0x2c`.
pub fn fn_00c75ad0(e: &mut Engine, this: Ptr, settings: Ptr) {
    let base = this.addr();
    let block = settings.addr();
    let owner = e.mem.u32(base + 0x2a4);
    e.mem.set_u32(owner + 0x18, block);
    e.mem.set_u8(base + 0x28c, 1);
    let first = e.mem.u8(block);
    e.mem.set_u8(base + 0x28d, first);
    let target = e.mem.u32(e.mem.u32(base + 0x4c));
    for (to, from) in [
        (0, 8),
        (4, 0x1c),
        (8, 0x14),
        (0xc, 0x10),
        (0x10, 0xc),
        (0x14, 0x30),
        (0x18, 0x34),
        (0x1c, 0x18),
        (0x20, 0x20),
        (0x24, 0x24),
        (0x28, 0x28),
        (0x2c, 0x2c),
    ] {
        copy_quiet(e, block + from, target + to);
    }
}

// Translated from 00c75b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: with `inverse = 1 / [[this + 0x58] + 0x98]` (a float), for each of the
/// `[[[this + 0x2a4] + 0xc] + 0x1c]` (signed count) bones, converts the 0x30-byte transform
/// `[this + 0x94] + 0x30*i` into a 0x34-byte block with `00c74d50`, scales the block's point
/// (`+0x24`, `+0x28`, `+0x2c`) by `inverse` (products in `f64`, narrowed) and writes the
/// scaled point to `bone + 0x58..0x60` and the first nine dwords of the block to
/// `bone + 0x34`, where `bone` is entry `i` of the pointer array at `[this + 0xa4]`.
pub fn fn_00c75b40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let source = e.mem.u32(base + 0x58);
    let divisor = f64::from(e.mem.f32(source + 0x98));
    let owner = e.mem.u32(base + 0x2a4);
    let skeleton = e.mem.u32(owner + 0xc);
    let inverse = (1.0 / divisor) as f32;
    let count = e.mem.i32(skeleton + 0x1c);
    let mut index = 0i32;
    while index < count {
        let transform = e
            .mem
            .u32(base + 0x94)
            .wrapping_add((index as u32).wrapping_mul(0x30));
        let table = e.mem.u32(base + 0xa4);
        let bone = e
            .mem
            .u32(table.wrapping_add((index as u32).wrapping_mul(4)));
        e.with_stack(0x34, |e, block| {
            let b = block.addr();
            fn_00c74d50(e, block, Ptr::new(transform));
            let scaled = [0x24, 0x28, 0x2c]
                .map(|offset| (f64::from(e.mem.f32(b + offset)) * f64::from(inverse)) as f32);
            for (offset, value) in [0x24, 0x28, 0x2c].into_iter().zip(scaled) {
                e.mem.set_f32(b + offset, value);
            }
            for (offset, value) in [0x58, 0x5c, 0x60].into_iter().zip(scaled) {
                e.mem.set_u32(bone + offset, value.to_bits());
            }
            for word in 0..9 {
                let value = e.mem.u32(b + 4 * word);
                e.mem.set_u32(bone + 0x34 + 4 * word, value);
            }
        });
        index += 1;
    }
}

// Translated from 00c75c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::ResetPose` (Ghidra name), cdecl `(world, array, flag, mode)`: for each of
/// the `array`'s elements (count `+8`, base `+4`, 0x38 bytes each), copies the element's three
/// dwords at `+0x28` to the object at the element's `+0` (`+0x58`) and its nine dwords at `+4` to
/// the object's `+0x34`. Then `00a59c60` (thiscall on `world`, argument the address of a zeroed
/// 9-byte record), `00c6a350(world, mode, 1, 0, 1)` when `mode` is neither 4 nor 0, and
/// `00c80330(flag, world)` when the low byte of `flag` is not 0.
pub fn bhk_ragdoll_controller_reset_pose(
    e: &mut Engine,
    world: u32,
    array: u32,
    flag: u32,
    mode: u32,
) {
    let count = e.mem.u32(array + 8);
    for index in 0..count {
        let element = e.mem.u32(array + 4).wrapping_add(index.wrapping_mul(0x38));
        let object = e.mem.u32(element);
        for word in 0..3 {
            let value = e.mem.u32(element + 0x28 + 4 * word);
            e.mem.set_u32(object + 0x58 + 4 * word, value);
        }
        for word in 0..9 {
            let value = e.mem.u32(element + 4 + 4 * word);
            e.mem.set_u32(object + 0x34 + 4 * word, value);
        }
    }
    e.with_stack(0x10, |e, record| {
        e.call(0x00a5_9c60, &args![world, record]);
    });
    if mode != 4 && mode != 0 {
        e.call(0x00c6_a350, &args![world, mode, 1u32, 0u32, 1u32]);
    }
    if flag & 0xff != 0 {
        e.call(0x00c8_0330, &args![flag, world]);
    }
}

/// The map's remove-all (`00b601a0`, thiscall, no arguments) and the heap free (`00aa10f0`,
/// cdecl `(block)`) the map destructors call; `kernel32` `InterlockedIncrement` and
/// `InterlockedDecrement` import slots.
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
const HEAP_FREE: u32 = 0x00aa_10f0;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// The destructor body of a map whose vtable is `vtable`: stores it, empties the map and frees
/// its storage block (`[this + 8]`).
fn map_destruct(e: &mut Engine, this: u32, vtable: u32) {
    e.mem.set_u32(this, vtable);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let storage = e.mem.u32(this + 8);
    e.call(HEAP_FREE, &args![storage]);
}

/// The destructor of a derived map: the derived vtable and `remove all`, then the base body.
fn derived_map_destruct(e: &mut Engine, this: u32, derived: u32, base: u32) {
    e.mem.set_u32(this, derived);
    e.call(MAP_REMOVE_ALL, &args![this]);
    map_destruct(e, this, base);
}

/// The scalar deleting form: the body, then `MemoryManager::Deallocate` when bit 0 of `flags`.
fn map_delete(e: &mut Engine, this: Ptr, vtable: u32, flags: u32) -> Ptr {
    map_destruct(e, this.addr(), vtable);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// Drops one reference of an `NiRefObject` (count at `+4`): at zero, slot 4 (the destroy).
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The body of `00c75ce0` and `00c75e20`: `[this + 4] = key`; the held reference at `+8` is
/// replaced by `value` (the old one released, the new one counted); then the extra reference
/// the caller's by-value pointer held on `value` is dropped.
fn replace_reference(e: &mut Engine, this: u32, key: u32, value: u32) {
    e.mem.set_u32(this + 4, key);
    let old = e.mem.u32(this + 8);
    if old != value {
        if old != 0 {
            release_reference(e, old);
        }
        e.mem.set_u32(this + 8, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
        }
    }
    if value != 0 {
        release_reference(e, value);
    }
}

// Translated from 00c75cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMapBase<char const*, NiPointer<bhkPoseArray>>` (RTTI vtable
/// `010c4c64`): empties the map and frees its storage.
pub fn fn_00c75cc0(e: &mut Engine, this: Ptr) {
    map_destruct(e, this.addr(), 0x010c_4c64);
}

// Translated from 00c75ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c75ce0 - the FS:[0] exception-unwinding frame is not modeled.
/// A map vtable slot with the stack words `(item, key, value)` on a map item holding an
/// `NiPointer` at `+8`: see `replace_reference`. The item class is the one of the pose-array
/// map (unconfirmed).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn fn_00c75ce0(e: &mut Engine, _this: u32, item: Ptr, key: u32, value: u32) {
    replace_reference(e, item.addr(), key, value);
}

// Translated from 00c75d90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c75d90 - the FS:[0] exception-unwinding frame is not modeled.
/// Destructor of `NiTMap<char const*, NiPointer<bhkPoseArray>>` (RTTI vtable `010c4c84`),
/// register `ECX`: runs the base map's destructor body under its own vtable `010c4c64`.
pub fn fn_00c75d90(e: &mut Engine, this: Ptr) {
    derived_map_destruct(e, this.addr(), 0x010c_4c84, 0x010c_4c64);
}

// Translated from 00c75e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMapBase<char const*, NiPointer<bhkRagdollShareData>>` (RTTI vtable
/// `010c4ca4`), register `ECX`.
pub fn fn_00c75e00(e: &mut Engine, this: Ptr) {
    map_destruct(e, this.addr(), 0x010c_4ca4);
}

// Translated from 00c75e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c75e20 - the FS:[0] exception-unwinding frame is not modeled.
/// Map vtable slot like `00c75ce0` (`this` unread, stack words `(item, key, value)`), for the
/// share-data map's item.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn fn_00c75e20(e: &mut Engine, _this: u32, item: Ptr, key: u32, value: u32) {
    replace_reference(e, item.addr(), key, value);
}

// Translated from 00c75ed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c75ed0 - the FS:[0] exception-unwinding frame is not modeled.
/// Destructor of `NiTMap<char const*, NiPointer<bhkRagdollShareData>>` (RTTI vtable
/// `010c4cc4`), register `ECX`: runs the base map's body under vtable `010c4ca4`.
pub fn fn_00c75ed0(e: &mut Engine, this: Ptr) {
    derived_map_destruct(e, this.addr(), 0x010c_4cc4, 0x010c_4ca4);
}

// Translated from 00c75f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `count * 4` bytes (`MemoryManager::Allocate`, `00aa3e40`); the one stack argument
/// is the callee's to pop (`thiscall`).
/// `this` (`ECX`, the array) is not read, but it is a `BSSimpleArray` vtable slot and is called
/// with `ECX` set.
pub fn fn_00c75f40(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(
        MEMORY_ALLOCATE,
        &args![MEMORY_MANAGER, count.wrapping_mul(4)],
    )
    .u32()
}

// Translated from 00c75f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A map vtable slot with the stack words `(item, key, value)`: stores the dword `key` at `+4`
/// and the 16-bit `value` at `+8` (a map item of `NiAVObject*` to `short`, unconfirmed).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn fn_00c75f60(e: &mut Engine, _this: u32, item: Ptr, key: u32, value: u32) {
    e.mem.set_u32(item.addr() + 4, key);
    e.mem.set_u16(item.addr() + 8, value as u16);
}

// Translated from 00c75f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `NiTMapBase<NiAVObject*, short>` (RTTI vtable `010c4cf8`), register `ECX`.
pub fn fn_00c75f80(e: &mut Engine, this: Ptr) {
    map_destruct(e, this.addr(), 0x010c_4cf8);
}

// Translated from 00c75fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c75fa0 - the FS:[0] exception-unwinding frame is not modeled.
/// Destructor of `NiTMap<NiAVObject*, short>` (RTTI vtable `010c4d18`), register `ECX`: runs
/// the base map's body under vtable `010c4cf8`.
pub fn fn_00c75fa0(e: &mut Engine, this: Ptr) {
    derived_map_destruct(e, this.addr(), 0x010c_4d18, 0x010c_4cf8);
}

// Translated from 00c76010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiAVObject*, short>::DeleteItem` (Ghidra name), a map vtable slot with the
/// stack word `(item)`: clears the 16-bit value at `+8` and returns the item to
/// `MemoryManager::Deallocate` (`00aa4060`).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn fn_00c76010(e: &mut Engine, _this: u32, item: Ptr) {
    e.mem.set_u16(item.addr() + 8, 0);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, item]);
}

// Translated from 00c76030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the pose-array map base (vtable `010c4c64`), register `ECX`,
/// bit 0 of `flags` deletes the block. Answers `this`.
pub fn fn_00c76030(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    map_delete(e, this, 0x010c_4c64, flags)
}

// Translated from 00c76070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the share-data map base (vtable `010c4ca4`), register `ECX`.
pub fn fn_00c76070(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    map_delete(e, this, 0x010c_4ca4, flags)
}

// Translated from 00c760b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiAVObject*`-to-`short` map base (vtable `010c4cf8`),
/// register `ECX`.
pub fn fn_00c760b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    map_delete(e, this, 0x010c_4cf8, flags)
}

// Translated from 00c760f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c760f0 - the FS:[0] exception-unwinding frame is not modeled.
/// Stdcall `(base, count)`: constructs `count` elements of 0x38 bytes at `base` (the dword at
/// `+0` cleared, `00a86bf0` run as a thiscall on `+4`); named
/// `BSSimpleArray<bhkRagdollController::BoneTransf>::Add` in Ghidra.
pub fn fn_00c760f0(e: &mut Engine, base: u32, count: u32) {
    for index in 0..count {
        let element = base.wrapping_add(index.wrapping_mul(0x38));
        if element != 0 {
            e.mem.set_u32(element, 0);
            e.call(0x00a8_6bf0, &args![element + 4]);
        }
    }
}

// Translated from 00c76180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(count, used)` on an array object (data pointer `+4`, capacity `+0xc`, allocator
/// slots `4` = allocate `(count)`, `8` = free `(block)`, `0xc` = resize `(block, count)`):
/// without data, allocates `count` and records it as the capacity; with data, when `used`
/// equals the capacity resizes in place (`slot 0xc`), else allocates, moves `used` elements of
/// 0x38 bytes over with `00c749f0` and frees the old block.
pub fn fn_00c76180(e: &mut Engine, this: Ptr, count: u32, used: u32) {
    let array = this.addr();
    let data = e.mem.u32(array + 4);
    if data == 0 {
        let block = e.vcall(array, 4, &args![count]).u32();
        e.mem.set_u32(array + 0xc, count);
        e.mem.set_u32(array + 4, block);
    } else if used == e.mem.u32(array + 0xc) {
        let block = e.vcall(array, 0xc, &args![data, count]).u32();
        e.mem.set_u32(array + 4, block);
    } else {
        let block = e.vcall(array, 4, &args![count]).u32();
        let old = e.mem.u32(array + 4);
        fn_00c749f0(e, block, old, used);
        let old = e.mem.u32(array + 4);
        e.vcall(array, 8, &args![old]);
        e.mem.set_u32(array + 4, block);
    }
}

// Translated from 00c761f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(out)`: writes the byte 1 to `out` when the byte at `this + 0xe8` is 4 or 5,
/// else 0, and answers `out`.
pub fn fn_00c761f0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let kind = e.mem.u8(this.addr() + 0xe8);
    e.mem.set_u8(out.addr(), u8::from(kind == 5 || kind == 4));
    out
}

// Translated from 00c76220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(index)`: the entry `[this + 8][ [this + 0x20][index] ]`, or 0 when the index
/// table's (signed) entry is negative.
pub fn fn_00c76220(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let table = e.mem.u32(this.addr() + 0x20);
    let slot = e.mem.i32(table.wrapping_add(index.wrapping_mul(4)));
    if slot < 0 {
        return 0;
    }
    let entries = e.mem.u32(this.addr() + 8);
    e.mem
        .u32(entries.wrapping_add((slot as u32).wrapping_mul(4)))
}

// Translated from 00c76240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c76240 - the FS:[0] exception-unwinding frame is not modeled.
/// Destructor body of `bhkRagdollShareData` (RTTI vtable `010c4b74`, role inferred from its
/// use by `00c7a8b0`), register `ECX`: releases the references at `+8`, `+0x10` and `+0x14`
/// (`00c905b0`), deletes the object at `+0xc` (slot 0 with 1), frees the aligned block at `+0x1c`
/// (`00c75100` on it, then `MemoryManager::Deallocate` of the block start, the byte before the
/// pointer being the distance) and runs the base destructor `00a5d3d0`.
pub fn bhk_ragdoll_share_data_destruct(e: &mut Engine, this: Ptr) {
    const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
    const BASE_DESTRUCT: u32 = 0x00a5_d3d0;
    let object = this.addr();
    e.mem.set_u32(object, 0x010c_4b74);
    for offset in [8, 0xc, 0x10, 0x14] {
        let held = e.mem.u32(object + offset);
        if held == 0 {
            continue;
        }
        if offset == 0xc {
            e.vcall(held, 0, &args![1u32]);
        } else {
            e.call(REMOVE_REFERENCE, &args![held]);
        }
    }
    let block = e.mem.u32(object + 0x1c);
    if block != 0 {
        fn_00c75100(e, Ptr::new(block));
        let distance = u32::from(e.mem.u8(block.wrapping_sub(1)));
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, block.wrapping_sub(distance)],
        );
    }
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00c762f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: the entry `[this + 0x48][...]`: with the list at `+0x48`, the (signed)
/// index `[[list + 0x20] + [this + 0x2b8]*4]` selects `[list + 8][index]`; 0 when there is no
/// list or the index is negative.
pub fn fn_00c762f0(e: &mut Engine, this: Ptr) -> u32 {
    let list = e.mem.u32(this.addr() + 0x48);
    if list == 0 {
        return 0;
    }
    let position = e.mem.u32(this.addr() + 0x2b8);
    let table = e.mem.u32(list + 0x20);
    let slot = e.mem.i32(table.wrapping_add(position.wrapping_mul(4)));
    if slot < 0 {
        return 0;
    }
    let entries = e.mem.u32(list + 8);
    e.mem
        .u32(entries.wrapping_add((slot as u32).wrapping_mul(4)))
}

// Translated from 00c76320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: follows `[this + 0x48] -> +8 -> [0] -> +8` and answers the dword at `+0x350`
/// there; 0 when any link is null.
pub fn fn_00c76320(e: &mut Engine, this: Ptr) -> u32 {
    let list = e.mem.u32(this.addr() + 0x48);
    if list == 0 {
        return 0;
    }
    let entries = e.mem.u32(list + 8);
    let first = e.mem.u32(entries);
    if first == 0 {
        return 0;
    }
    let inner = e.mem.u32(first + 8);
    if inner == 0 {
        return 0;
    }
    e.mem.u32(inner + 0x350)
}

/// The constraint-data kind getter every motor function asks (slot `0x20`), the wrapped-data
/// getter it follows for kind 13 (`00cdf6c0`, thiscall) and the two kind numbers it acts on.
const DATA_KIND_SLOT: u32 = 0x20;
const WRAPPED_DATA: u32 = 0x00cd_f6c0;

// Translated from 00c78090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: answers whether any of the `[this + 0x230]` (signed count) 0xd0-byte
/// records at `[this + 0x22c]` names a bone whose flag word (`+0x30`) has bit 1 clear: each
/// record starts with a pointer to a block holding three signed 16-bit bone indices at `+0x18`,
/// `+0x16` and `+0x14`, looked up in the pointer array at `[this + 0xa4]` (size `[this + 0xa8]`,
/// an index at or past it gives no bone). The search stops at the first record that has one.
pub fn fn_00c78090(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let count = e.mem.i32(base + 0x230);
    let mut found = false;
    let mut index = 0i32;
    while index < count {
        if found {
            break;
        }
        let records = e.mem.u32(base + 0x22c);
        let block = e
            .mem
            .u32(records.wrapping_add((index as u32).wrapping_mul(0xd0)));
        let size = e.mem.u32(base + 0xa8);
        let bones = [0x18, 0x16, 0x14].map(|offset| {
            let slot = i32::from(e.mem.i16(block + offset)) as u32;
            if size > slot {
                let table = e.mem.u32(base + 0xa4);
                e.mem.u32(table.wrapping_add(slot.wrapping_mul(4)))
            } else {
                0
            }
        });
        found = bones
            .into_iter()
            .any(|bone| bone != 0 && (e.mem.u32(bone + 0x30) >> 1) & 1 == 0);
        index += 1;
    }
    found
}

// Translated from 00c79d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(a, b, c)` (three booleans): sets or clears bit 0 of the flag word `+0x30` of the
/// three entries `[[this + 0x84] + 0xa0][0..3]`, set when the argument is 0 and cleared else.
/// Then asks the object `[this + 0x58]` for its handler (slot `0xc`); with one, when any
/// argument is set and `[[this + 0x84] + 0x18]` is 0 calls the handler's slot `0xdc` with
/// `([this + 0x84], 1)`, and when all three are 0 calls its slot `0xe8` with `[this + 0x84]`.
pub fn fn_00c79d60(e: &mut Engine, this: Ptr, a: u8, b: u8, c: u8) {
    let base = this.addr();
    let object = e.mem.u32(base + 0x84);
    for (slot, argument) in [(0, a), (4, b), (8, c)] {
        let entries = e.mem.u32(object + 0xa0);
        let entry = e.mem.u32(entries + slot);
        let flags = e.mem.u32(entry + 0x30);
        let flags = if argument == 0 { flags | 1 } else { flags & !1 };
        e.mem.set_u32(entry + 0x30, flags);
    }
    let owner = e.mem.u32(base + 0x58);
    let handler = e.vcall(owner, 0xc, &args![]).u32();
    if handler == 0 {
        return;
    }
    if a == 0 && b == 0 && c == 0 {
        let object = e.mem.u32(base + 0x84);
        e.vcall(handler, 0xe8, &args![object]);
    } else {
        let object = e.mem.u32(base + 0x84);
        if e.mem.u32(object + 0x18) == 0 {
            e.vcall(handler, 0xdc, &args![object, 1u32]);
        }
    }
}

// Translated from 00c7a230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::UpdateConstraintsWorld` (Ghidra name), thiscall `(add)`: with the
/// constraint list at `+0x48` (entries `+0x14`, signed count `+0x18`, world holders `+8`),
/// gets the world `[[[list + 8] + 0] + 8]`'s `+0x350` object; nothing happens without one, nor
/// when the first constraint's holder is set but `add` is set, or unset but `add` is not.
/// With `add` set every entry whose `+8` is 0 goes to `00c6b320` (thiscall on the world
/// object, `(entry)`); without it every entry whose `+8` is set goes to `00c68d50`.
pub fn bhk_ragdoll_controller_update_constraints_world(e: &mut Engine, this: Ptr, add: u8) {
    let list = e.mem.u32(this.addr() + 0x48);
    let array = e.mem.u32(list + 0x14);
    let first = e.mem.u32(array);
    let holder = if first != 0 { e.mem.u32(first + 8) } else { 0 };
    let worlds = e.mem.u32(list + 8);
    let world = e.mem.u32(worlds);
    if world == 0 {
        return;
    }
    let world_object = e.mem.u32(world + 8);
    if world_object == 0 {
        return;
    }
    if add != 0 {
        if holder != 0 {
            return;
        }
    } else if holder == 0 {
        return;
    }
    let target = e.mem.u32(world_object + 0x350);
    let count = e.mem.i32(list + 0x18);
    for index in 0..count {
        let array = e.mem.u32(list + 0x14);
        let entry = e
            .mem
            .u32(array.wrapping_add((index as u32).wrapping_mul(4)));
        let held = e.mem.u32(entry + 8) != 0;
        if add != 0 && !held {
            e.call(0x00c6_b320, &args![target, entry]);
        } else if add == 0 && held {
            e.call(0x00c6_8d50, &args![target, entry]);
        }
    }
}

/// The constraint data of motor entry `index` of the controller's list, or 0 when its slot
/// table entry is not positive; kind 13 data answers its wrapped data (`00cdf6c0`).
fn motor_constraint(e: &mut Engine, controller: u32, index: u32) -> (u32, u32) {
    let table = e.mem.u32(controller + 0x20);
    let slot = e
        .mem
        .i32(table.wrapping_add(index.wrapping_mul(4)))
        .wrapping_sub(1);
    if slot < 0 {
        return (0, 0);
    }
    let array = e.mem.u32(controller + 0x14);
    let entry = e.mem.u32(array.wrapping_add((slot as u32).wrapping_mul(4)));
    if entry == 0 {
        return (0, 0);
    }
    let mut data = e.mem.u32(entry + 0xc);
    if e.vcall(data, DATA_KIND_SLOT, &args![]).u32() == 0xd {
        data = e.call(WRAPPED_DATA, &args![data]).u32();
    }
    (entry, data)
}

// Translated from 00c7a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::bhkRagdollPoweredConstraintController::startMotors` (Xbox PDB), cdecl
/// `(controller)`: for each of the `[[controller + 0x2c] + 0x1c]` entries asks the constraint data
/// for its kind (`motor_constraint` rules, data may be 0 after the wrapped lookup): kind 2 runs
/// `00cb8fe0(data; entry, 1)` (thiscall) and zeroes the float `data + 0xac`; kind 7 runs
/// `00cdf290(data; entry, 1)` and `00cdf230(data; 0x01268370)`. The flag argument is a pushed
/// dword whose low byte is 1; the callees read the byte.
pub fn bhk_ragdoll_powered_constraint_controller_start_motors(e: &mut Engine, controller: u32) {
    let motors = e.mem.u32(controller + 0x2c);
    let count = e.mem.i32(motors + 0x1c);
    for index in 0..count {
        let (entry, data) = motor_constraint(e, controller, index as u32);
        if entry == 0 || data == 0 {
            continue;
        }
        match e.vcall(data, DATA_KIND_SLOT, &args![]).u32() {
            2 => {
                e.call(0x00cb_8fe0, &args![data, entry, 1u32]);
                e.mem.set_f32(data + 0xac, 0.0);
            }
            7 => {
                e.call(0x00cd_f290, &args![data, entry, 1u32]);
                e.call(0x00cd_f230, &args![data, 0x0126_8370u32]);
            }
            _ => {}
        }
    }
}

// Translated from 00c7a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::bhkRagdollPoweredConstraintController::stopMotors` (Xbox PDB), cdecl
/// `(controller)`: like `startMotors` with the flag 0 and without the follow-up calls.
pub fn bhk_ragdoll_powered_constraint_controller_stop_motors(e: &mut Engine, controller: u32) {
    let motors = e.mem.u32(controller + 0x2c);
    let count = e.mem.i32(motors + 0x1c);
    for index in 0..count {
        let (entry, data) = motor_constraint(e, controller, index as u32);
        if entry == 0 || data == 0 {
            continue;
        }
        match e.vcall(data, DATA_KIND_SLOT, &args![]).u32() {
            2 => {
                e.call(0x00cb_8fe0, &args![data, entry, 0u32]);
            }
            7 => {
                e.call(0x00cd_f290, &args![data, entry, 0u32]);
            }
            _ => {}
        }
    }
}

// Translated from 00c7a410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7a410 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRagdollController::bhkRagdollPoweredConstraintController::driveToPose` (Xbox PDB), cdecl
/// `(controller, pose)`: for each motor entry (as in `startMotors`, but the wrapped data is
/// used without a null check) with the pose cursor `pose + 0x10 + 0x30*i`: kind 2 builds two
/// quaternions from the data's `+0x20` and `+0x60` (`00cb26e0`), multiplies the first by the cursor
/// (`00c66320`), the second by that product (`fn_00c74b70`), has `00cb2ca0` decompose the result about
/// the rest axis `(1, 0, 0, 0)` and stores the resulting angle at `data + 0xac`; kind 7 builds a
/// target from the cursor (`00cb2d90`) and hands it to `00cdf230`.
pub fn bhk_ragdoll_powered_constraint_controller_drive_to_pose(
    e: &mut Engine,
    controller: u32,
    pose: u32,
) {
    let motors = e.mem.u32(controller + 0x2c);
    let count = e.mem.i32(motors + 0x1c);
    let mut cursor = pose.wrapping_add(0x10);
    for index in 0..count {
        let table = e.mem.u32(controller + 0x20);
        let slot = e
            .mem
            .i32(table.wrapping_add((index as u32).wrapping_mul(4)))
            .wrapping_sub(1);
        let array = e.mem.u32(controller + 0x14);
        let entry = if slot >= 0 {
            e.mem.u32(array.wrapping_add((slot as u32).wrapping_mul(4)))
        } else {
            0
        };
        if entry != 0 {
            let mut data = e.mem.u32(entry + 0xc);
            if e.vcall(data, DATA_KIND_SLOT, &args![]).u32() == 0xd {
                data = e.call(WRAPPED_DATA, &args![data]).u32();
            }
            e.with_stack(0xb0, |e, frame| {
                let f = frame.addr();
                match e.vcall(data, DATA_KIND_SLOT, &args![]).u32() {
                    2 => {
                        e.call(0x00cb_26e0, &args![f + 0x54, data + 0x20]);
                        e.call(0x00cb_26e0, &args![f + 0x44, data + 0x60]);
                        e.call(0x00c6_6320, &args![f + 0x14, cursor, f + 0x54]);
                        fn_00c74b70(
                            e,
                            Ptr::new(f + 0x34),
                            Ptr::new(f + 0x44),
                            Ptr::new(f + 0x14),
                        );
                        store4(e, f + 0x24, [1.0, 0.0, 0.0, 0.0]);
                        e.call(0x00cb_2ca0, &args![f + 0x34, f + 0x24, f + 0x64, f + 0xc]);
                        copy_quiet(e, f + 0xc, data + 0xac);
                    }
                    7 => {
                        e.call(0x00cb_2d90, &args![f + 0x74, cursor]);
                        e.call(0x00cd_f230, &args![data, f + 0x74]);
                    }
                    _ => {}
                }
            });
        }
        cursor = cursor.wrapping_add(0x30);
    }
}

// Translated from 00c7a570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<char const*, NiPointer<bhkPoseArray>>::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor `00c75d90`, then `MemoryManager::Deallocate` when bit 0 of `flags` is set.
/// Answers `this`.
pub fn ni_t_map_pose_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c75d90(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c7a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<char const*, NiPointer<bhkRagdollShareData>>::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor `00c75ed0`, then the deallocation when bit 0 of `flags` is set.
pub fn ni_t_map_ragdoll_share_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c75ed0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c7a5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<NiAVObject*, short>::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// `00c75fa0`, then the deallocation when bit 0 of `flags` is set.
pub fn ni_t_map_ni_av_object_short_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c75fa0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c7a5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<bhkRagdollController::BoneTransf>::_Allocate` (Xbox PDB): allocates
/// `count * 0x38` bytes (`MemoryManager::Allocate`); stack `(count)`.
/// `this` (`ECX`, the array) is not read, but it is a `BSSimpleArray` vtable slot and is called
/// with `ECX` set.
pub fn bs_simple_array_bone_transf_allocate(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(
        MEMORY_ALLOCATE,
        &args![MEMORY_MANAGER, count.wrapping_mul(0x38)],
    )
    .u32()
}

// Translated from 00c7a600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<bhkRagdollController::BoneTransf>::_Reallocate` (Xbox PDB): stack
/// `(block, count)`: resizes the block to `count * 0x38` bytes (`MemoryManager::Reallocate`,
/// `00aa4150`, `(block, size)`) and answers its result.
/// `this` (`ECX`, the array) is not read, but it is a `BSSimpleArray` vtable slot and is called
/// with `ECX` set.
pub fn bs_simple_array_bone_transf_reallocate(
    e: &mut Engine,
    _this: Ptr,
    block: u32,
    count: u32,
) -> u32 {
    e.call(
        0x00aa_4150,
        &args![MEMORY_MANAGER, block, count.wrapping_mul(0x38)],
    )
    .u32()
}

/// The destructor body of a `BSSimpleArray` whose vtable is `vtable` and whose free function is
/// the vtable's slot 2 at `vtable + 8`: stores the vtable and, when there is data (`+4`),
/// frees it through that slot (called directly with `ECX` still the array, `(block)` on the
/// stack) and clears data, capacity (`+0xc`) and size (`+8`).
fn simple_array_destruct(e: &mut Engine, this: u32, vtable: u32) {
    let data = e.mem.u32(this + 4);
    e.mem.set_u32(this, vtable);
    if data != 0 {
        let free = e.mem.u32(vtable + 8);
        e.call(free, &args![this, data]);
        e.mem.set_u32(this + 4, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0);
    }
}

// Translated from 00c7a630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSSimpleArray<bhkRagdollController::BoneTransf>` (RTTI vtable
/// `010c4db8`), register `ECX`.
pub fn fn_00c7a630(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this.addr(), 0x010c_4db8);
}

// Translated from 00c7a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSSimpleArray<bhkRagdollController::bhkSkeletonBone*>` (RTTI vtable
/// `010c4ce4`), register `ECX`.
pub fn fn_00c7a660(e: &mut Engine, this: Ptr) {
    simple_array_destruct(e, this.addr(), 0x010c_4ce4);
}

// Translated from 00c7a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<bhkRagdollController::BoneTransf>::_scalar_deleting_destructor_` (Xbox PDB):
/// the destructor body, then `MemoryManager::Deallocate` when bit 0 of `flags` is set.
pub fn bs_simple_array_bone_transf_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c7a630(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c7a6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<bhkRagdollController::bhkSkeletonBone*>::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor body, then the deallocation when bit 0 of `flags` is set.
pub fn bs_simple_array_bhk_skeleton_bone_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c7a660(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c7a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: appends a slot to the array of 4-byte entries and answers its index (the old
/// size). Without room (size `+8` equals capacity `+0xc`): with no capacity it allocates 4
/// entries (slot 4 of the array's allocator, `(4)`) and sets the capacity to 4; else grows by
/// doubling up to 0x400 entries and by 0x400 beyond, through `00c76180(capacity; used)`.
pub fn fn_00c7a730(e: &mut Engine, this: Ptr) -> u32 {
    let array = this.addr();
    let size = e.mem.u32(array + 8);
    let capacity = e.mem.u32(array + 0xc);
    if size == capacity {
        if capacity == 0 {
            let block = e.vcall(array, 4, &args![4u32]).u32();
            let used = e.mem.u32(array + 8);
            e.mem.set_u32(array + 8, used.wrapping_add(1));
            e.mem.set_u32(array + 4, block);
            e.mem.set_u32(array + 0xc, 4);
            return e.mem.u32(array + 8).wrapping_sub(1);
        }
        let grown = if capacity > 0x400 {
            capacity.wrapping_add(0x400)
        } else {
            capacity.wrapping_add(capacity)
        };
        fn_00c76180(e, this, grown, size);
        e.mem.set_u32(array + 0xc, grown);
    }
    let used = e.mem.u32(array + 8).wrapping_add(1);
    e.mem.set_u32(array + 8, used);
    used.wrapping_sub(1)
}

// Translated from 00c7a790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`, argument `source`: assigns a settings record (the layout `00c74fb0`
/// compares): resizes the array of 16-bit values at `+0x40` (data `+0x44`, size `+0x48`,
/// capacity `+0x4c`, allocator slots `4` allocate `(count)` and `8` free `(block)`; `005ef6a0`
/// resizes `(count, used)`) to the source's size, copies the values, the byte at `+0`, the
/// floats `+4..+0x34` (quiet copies) and the dwords `+0x38` and `+0x3c`.
pub fn fn_00c7a790(e: &mut Engine, this: Ptr, source: Ptr) {
    const RESIZE: u32 = 0x005e_f6a0;
    let destination = this.addr();
    let from = source.addr();
    let array = destination + 0x40;
    let wanted = e.mem.u32(from + 0x48);
    if wanted == 0 {
        let data = e.mem.u32(array + 4);
        if data != 0 {
            e.vcall(array, 8, &args![data]);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 0xc, 0);
            e.mem.set_u32(array + 8, 0);
        }
    } else {
        let capacity = e.mem.u32(array + 0xc);
        if wanted > capacity {
            if capacity == 0 {
                let block = e.vcall(array, 4, &args![wanted]).u32();
                e.mem.set_u32(array + 4, block);
            } else {
                let used = e.mem.u32(array + 8);
                e.call(RESIZE, &args![array, wanted, used]);
            }
            e.mem.set_u32(array + 0xc, wanted);
            e.mem.set_u32(array + 8, wanted);
        } else if wanted >= e.mem.u32(array + 8) {
            e.mem.set_u32(array + 8, wanted);
        } else {
            e.mem.set_u32(array + 8, wanted);
            if wanted <= capacity >> 2 {
                e.call(RESIZE, &args![array, wanted, wanted]);
                e.mem.set_u32(array + 0xc, wanted);
            }
        }
    }
    let size = e.mem.u32(from + 0x48);
    for index in 0..size {
        let values = e.mem.u32(from + 0x44);
        let value = e.mem.u16(values.wrapping_add(index.wrapping_mul(2)));
        let target = e.mem.u32(destination + 0x44);
        e.mem
            .set_u16(target.wrapping_add(index.wrapping_mul(2)), value);
    }
    let byte = e.mem.u8(from);
    e.mem.set_u8(destination, byte);
    for offset in (4..=0x34).step_by(4) {
        copy_quiet(e, from + offset, destination + offset);
    }
    for offset in [0x38, 0x3c] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(destination + offset, word);
    }
}

// Translated from 00c7a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::EnableRagdollAnim` (Xbox PDB), register `ECX`: unless the byte
/// `+0x41` is set, calls `00cdb3e0` (thiscall) on the object at `+0x4c` when there is one. Then
/// `+0x41 = +0x42`, `+0x43 = 0`, `+0x222 = (+0x220 != 0)` (with `+0x223 = 1` when it was set
/// and no longer is) and `+0x21e = (+0x21d != 0)`. Unless `+0x28e` is set, and when `+0x42` was
/// set and the constraint list (`+0x48`) has a world object, every constraint entry whose `+8`
/// is set goes to `00c68d50` (thiscall on the world's `+0x350` object, `(entry)`).
pub fn bhk_ragdoll_controller_enable_ragdoll_anim(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x41) == 0 {
        let object = e.mem.u32(base + 0x4c);
        if object != 0 {
            e.call(0x00cd_b3e0, &args![object]);
        }
    }
    let previous = e.mem.u8(base + 0x222);
    let enabled = e.mem.u8(base + 0x42);
    let now = u8::from(e.mem.u8(base + 0x220) != 0);
    e.mem.set_u8(base + 0x41, enabled);
    e.mem.set_u8(base + 0x43, 0);
    e.mem.set_u8(base + 0x222, now);
    if previous != 0 && now == 0 {
        e.mem.set_u8(base + 0x223, 1);
    }
    let flag = u8::from(e.mem.u8(base + 0x21d) != 0);
    e.mem.set_u8(base + 0x21e, flag);
    if e.mem.u8(base + 0x28e) != 0 || enabled == 0 {
        return;
    }
    let list = e.mem.u32(base + 0x48);
    let array = e.mem.u32(list + 0x14);
    let first = e.mem.u32(array);
    let holder = if first != 0 { e.mem.u32(first + 8) } else { 0 };
    let worlds = e.mem.u32(list + 8);
    let world = e.mem.u32(worlds);
    if world == 0 {
        return;
    }
    let world_object = e.mem.u32(world + 8);
    if world_object == 0 || holder == 0 {
        return;
    }
    let target = e.mem.u32(world_object + 0x350);
    let count = e.mem.i32(list + 0x18);
    for index in 0..count {
        let array = e.mem.u32(list + 0x14);
        let entry = e
            .mem
            .u32(array.wrapping_add((index as u32).wrapping_mul(4)));
        if e.mem.u32(entry + 8) != 0 {
            e.call(0x00c6_8d50, &args![target, entry]);
        }
    }
}

// Translated from 00c7a990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(node, first, second)`: with a node, appends its dword `+8` to the Havok array
/// `first` (data `+0`, size `+4`, capacity-and-flags `+8` masked with `0x3fffffff`; a full array
/// first grows through `00c90940(allocator, array, 4)` with the allocator `[00c85750() + 0x10]`).
/// Then walks the list headed at `node + 0x14` (items `[0]`, next `[4]`): for each item whose
/// run-time type (slot 8 of the item) is, or derives from, the type at `0126818c`
/// (the chain is the dword at `+4`), appends the item's `+8` dword to `second`.
pub fn fn_00c7a990(e: &mut Engine, node: u32, first: u32, second: u32) {
    const GROW: u32 = 0x00c9_0940;
    const WANTED_TYPE: u32 = 0x0126_818c;
    fn push_back(e: &mut Engine, array: u32, value: u32) {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        if e.mem.u32(array + 4) == capacity {
            let allocator = e.mem.u32(router + 0x10);
            e.call(GROW, &args![allocator, array, 4u32]);
        }
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        e.mem
            .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
    }
    if node == 0 {
        return;
    }
    let value = e.mem.u32(node + 8);
    push_back(e, first, value);
    let mut link = node + 0x14;
    while link != 0 {
        let item = e.mem.u32(link);
        if item == 0 {
            break;
        }
        let mut kind = e.vcall(item, 8, &args![]).u32();
        let mut found = false;
        while kind != 0 {
            if kind == WANTED_TYPE {
                found = true;
                break;
            }
            kind = e.mem.u32(kind + 4);
        }
        if found {
            let value = e.mem.u32(item + 8);
            push_back(e, second, value);
        }
        link = e.mem.u32(link + 4);
    }
}
/// Empties a Havok array whose data pointer is at `array`, size at `+4` and capacity and flags
/// word at `+8`, with the allocator already fetched (slot `0x10` of `allocator` frees
/// `(block, bytes)`): the size is cleared, the block freed unless the flags word has bit 31
/// (not owned), then the data pointer is cleared and the flags set to `0x80000000`. `element`
/// is the element size.
fn free_havok_array(e: &mut Engine, allocator: u32, array: u32, element: u32) {
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if flags & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (flags & 0x3fff_ffff).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// `free_havok_array` with the allocator `[00c85750() + 0x10]` fetched just before.
fn release_havok_array(e: &mut Engine, array: u32, element: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    free_havok_array(e, allocator, array, element);
}

// Translated from 00c7b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: empties the Havok array of 0xd0-byte elements at `this` (see
/// `release_havok_array`).
pub fn fn_00c7b0b0(e: &mut Engine, this: Ptr) {
    release_havok_array(e, this.addr(), 0xd0);
}

// Translated from 00c7b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: empties the Havok array of 4-byte elements at `this`.
pub fn fn_00c7b0f0(e: &mut Engine, this: Ptr) {
    release_havok_array(e, this.addr(), 4);
}

// Translated from 00c7b140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(node, first, second)`: walks the scene graph under `node`. The cast function
/// `00653290` (thiscall on an object, `(type)`; answers the object or 0) is applied to `[node + 0x1c]`
/// with the type `01267e64` and to the result with `0126817c` (kept) and, from the result's
/// `+0x10`, with `012681c0`; when that last cast succeeds and the `0126817c` one failed,
/// `fn_00c7a990(cast, first, second)` collects it. Then `node` itself is cast to `011f4428`
/// (a node type with a child array: pointer `+0xa0`, 16-bit count `+0xa6`) and the function
/// recurses into each child.
pub fn fn_00c7b140(e: &mut Engine, node: u32, first: u32, second: u32) {
    const CAST: u32 = 0x0065_3290;
    if node == 0 {
        return;
    }
    let object = e.mem.u32(node + 0x1c);
    if object != 0 {
        let outer = e.call(CAST, &args![object, 0x0126_7e64u32]).u32();
        if outer != 0 {
            let flagged = e.call(CAST, &args![outer, 0x0126_817cu32]).u32();
            let inner = e.mem.u32(outer + 0x10);
            if inner != 0 {
                let target = e.call(CAST, &args![inner, 0x0126_81c0u32]).u32();
                if target != 0 && flagged == 0 {
                    fn_00c7a990(e, target, first, second);
                }
            }
        }
    }
    let group = e.call(CAST, &args![node, 0x011f_4428u32]).u32();
    if group != 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(group + 0xa6)) {
            let children = e.mem.u32(group + 0xa0);
            let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
            fn_00c7b140(e, child, first, second);
            index += 1;
        }
    }
}

// Translated from 00c7ba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::EnableRagdollFeedback` (Xbox PDB), thiscall `(enable)`: with the byte
/// `+0x28c` set and `enable` set, just sets `+0x28d`. Otherwise, when `+0x28e` is set,
/// `SetRagdollFeedbackActive(this, 0)`, then clears `+0x28d`.
pub fn bhk_ragdoll_controller_enable_ragdoll_feedback(e: &mut Engine, this: Ptr, enable: u8) {
    let base = this.addr();
    if e.mem.u8(base + 0x28c) != 0 && enable != 0 {
        e.mem.set_u8(base + 0x28d, 1);
        return;
    }
    if e.mem.u8(base + 0x28e) != 0 {
        bhk_ragdoll_controller_set_ragdoll_feedback_active(e, this, 0);
    }
    e.mem.set_u8(base + 0x28d, 0);
}

// Translated from 00c7be20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::GetRBForBones` (Xbox PDB), thiscall `(out)`: with the constraint list
/// at `+0x48`, makes the Havok array `out` (data `+0`, size `+4`, capacity-and-flags `+8`) hold
/// the list's `[list + 0xc]` dwords from `[list + 8]`: when the capacity is below the count it
/// first grows through `00c908c0(allocator, out, max(2 * capacity, count), 4)` (cdecl).
pub fn bhk_ragdoll_controller_get_rb_for_bones(e: &mut Engine, this: Ptr, out: Ptr) {
    let list = e.mem.u32(this.addr() + 0x48);
    if list == 0 {
        return;
    }
    let count = e.mem.i32(list + 0xc);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let array = out.addr();
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(0x00c9_08c0, &args![allocator, array, wanted as u32, 4u32]);
    }
    e.mem.set_u32(array + 4, count as u32);
    let list = e.mem.u32(this.addr() + 0x48);
    let count = e.mem.i32(list + 0xc);
    let source = e.mem.u32(list + 8);
    let data = e.mem.u32(array);
    for index in 0..count.max(0) as u32 {
        let value = e.mem.u32(source.wrapping_add(4 * index));
        e.mem.set_u32(data.wrapping_add(4 * index), value);
    }
}

// Translated from 00c7be90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates 12 bytes (`MemoryManager::Allocate`, `00aa3e40`), clears the dword at `+8` and
/// answers the block; 0 when the allocation fails.
pub fn fn_00c7be90(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    if block != 0 {
        e.mem.set_u32(block + 8, 0);
    }
    block
}

// Translated from 00c7beb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<char const*, NiPointer<bhkPoseArray>>::DeleteItem` (Xbox PDB), a map vtable slot with
/// the stack word `(item)`: releases the reference held at `+8` (decrement at `+4`; at zero slot
/// 4 of the object), clears it, then the inlined destructor releases it again (nothing left)
/// and the item goes to `MemoryManager::Deallocate`.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn fn_00c7beb0(e: &mut Engine, _this: u32, item: u32) {
    let held = e.mem.u32(item + 8);
    if held != 0 {
        release_reference(e, held);
        e.mem.set_u32(item + 8, 0);
    }
    if item == 0 {
        return;
    }
    let held = e.mem.u32(item + 8);
    if held != 0 {
        release_reference(e, held);
    }
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, item]);
}

// Translated from 00c7bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: empties the Havok array of 8-byte elements at `this`.
pub fn fn_00c7bf10(e: &mut Engine, this: Ptr) {
    release_havok_array(e, this.addr(), 8);
}

// Translated from 00c7bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: empties the Havok array of 2-byte elements at `this`.
pub fn fn_00c7bf50(e: &mut Engine, this: Ptr) {
    release_havok_array(e, this.addr(), 2);
}

// Translated from 00c7bf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: empties the Havok array of 0x30-byte elements at `this`.
pub fn fn_00c7bf90(e: &mut Engine, this: Ptr) {
    release_havok_array(e, this.addr(), 0x30);
}

// Translated from 00c7bfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register `ECX`: initialises the 0x54-byte record at `this`: dwords `+0`, `+4`, `+8` zero, floats
/// `+0xc` = 0 and `+0x10` = the float at `01078a5c` (quiet copy), `+0x14`, `+0x18` zero, `+0x1c`,
/// `+0x28` and `+0x34` = `0x80000000`, `+0x20`, `+0x24`, `+0x2c`, `+0x30` zero, bytes `+0x38` and
/// `+0x39` = 1, `+0x3c` zero, floats `+0x4c` and `+0x50` = 1.0, `+0x40`, `+0x44`, `+0x48`
/// zero. Answers `this`.
pub fn fn_00c7bfd0(e: &mut Engine, this: Ptr) -> Ptr {
    let record = this.addr();
    e.mem.set_f32(record + 0xc, 0.0);
    for offset in [0, 4, 8] {
        e.mem.set_u32(record + offset, 0);
    }
    copy_quiet(e, 0x0107_8a5c, record + 0x10);
    for offset in [0x14, 0x18] {
        e.mem.set_u32(record + offset, 0);
    }
    for offset in [0x1c, 0x28, 0x34] {
        e.mem.set_u32(record + offset, 0x8000_0000);
    }
    for offset in [0x20, 0x24, 0x2c, 0x30] {
        e.mem.set_u32(record + offset, 0);
    }
    e.mem.set_u8(record + 0x38, 1);
    e.mem.set_u8(record + 0x39, 1);
    e.mem.set_u32(record + 0x3c, 0);
    e.mem.set_f32(record + 0x4c, 1.0);
    e.mem.set_f32(record + 0x50, 1.0);
    for offset in [0x40, 0x44, 0x48] {
        e.mem.set_u32(record + offset, 0);
    }
    this
}

// Translated from 00c7c030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7c030 - the FS:[0] exception-unwinding frame is not modeled.
/// Register `ECX`: empties the four Havok arrays at `this + 0x2c` (4-byte elements),
/// `this + 0x1c` (4), `this + 0x10` (0x30) and `this + 4` (0x30), in that order.
pub fn fn_00c7c030(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_havok_array(e, base + 0x2c, 4);
    release_havok_array(e, base + 0x1c, 4);
    release_havok_array(e, base + 0x10, 0x30);
    release_havok_array(e, base + 4, 0x30);
}

// Translated from 00c7c150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::DisableRagdollAnim` (Xbox PDB), thiscall `(add_to_world)`: nothing
/// unless the byte `+0x42` is set. Then clears `+0x41`, `+0x43`, `+0x222` (setting `+0x223`
/// when it was set), `+0x21e` and `+0xb3`, fills the vectors at `+0x120` and `+0x170` from
/// `010c71b0`, sets the bytes `+0x180` and `+0x130`, calls `SetRagdollFeedbackActive(this, 0)`
/// when `+0x28d` is set, and clears the active byte (`+0x14`) of the object that slot `0x3c`
/// of each bone's `+0x1c` object gives. Then it resets `+0xb2`, the floats
/// and bytes at `+0x1b8`, `+0x1bc`, `+0x1e8`, `+0x1ec`, `+0x221`, `+0x290` (1.0),
/// `+0x228`, `+0x28f` and `+0x294`; and when `add_to_world` is set adds the unheld constraints to the
/// world.
pub fn bhk_ragdoll_controller_disable_ragdoll_anim(e: &mut Engine, this: Ptr, add_to_world: u8) {
    let base = this.addr();
    if e.mem.u8(base + 0x42) == 0 {
        return;
    }
    let previous = e.mem.u8(base + 0x222);
    e.mem.set_u8(base + 0x41, 0);
    e.mem.set_u8(base + 0x43, 0);
    e.mem.set_u8(base + 0x222, 0);
    if previous != 0 {
        e.mem.set_u8(base + 0x223, 1);
    }
    e.mem.set_u8(base + 0x21e, 0);
    e.mem.set_u8(base + 0xb3, 0);
    copy16(e, 0x010c_71b0, base + 0x120);
    copy16(e, 0x010c_71b0, base + 0x170);
    e.mem.set_u8(base + 0x180, 1);
    e.mem.set_u8(base + 0x130, 1);
    if e.mem.u8(base + 0x28d) != 0 {
        bhk_ragdoll_controller_set_ragdoll_feedback_active(e, this, 0);
    }
    let owner = e.mem.u32(base + 0x2a4);
    if owner != 0 {
        let skeleton = e.mem.u32(owner + 0xc);
        let count = e.mem.i32(skeleton + 0x1c);
        for index in 0..count {
            let bones = e.mem.u32(base + 0xa4);
            let bone = e
                .mem
                .u32(bones.wrapping_add((index as u32).wrapping_mul(4)));
            let object = e.mem.u32(bone + 0x1c);
            if object != 0 {
                let found = e.vcall(object, 0x3c, &args![]).u32();
                if found != 0 {
                    e.mem.set_u8(found + 0x14, 0);
                }
            }
        }
    }
    e.mem.set_u8(base + 0xb2, 0);
    e.mem.set_f32(base + 0x1e8, 0.0);
    e.mem.set_u8(base + 0x1ec, 0);
    e.mem.set_f32(base + 0x1b8, 0.0);
    e.mem.set_u8(base + 0x1bc, 0);
    e.mem.set_u8(base + 0x221, 0);
    e.mem.set_f32(base + 0x290, 1.0);
    e.mem.set_u32(base + 0x228, 0);
    e.mem.set_u8(base + 0x28f, 0);
    e.mem.set_u32(base + 0x294, 0);
    if add_to_world != 0 {
        bhk_ragdoll_controller_update_constraints_world(e, this, 1);
    }
}

// Translated from 00c7c2b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7c2b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkaSkeletonMapperUtils::Params::~Params` (Xbox PDB name from the Ghidra database), register `ECX`:
/// empties the three Havok arrays at `this + 0x2c`, `this + 0x20` and `this + 0x14`, all of
/// 8-byte elements.
pub fn fn_00c7c2b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_havok_array(e, base + 0x2c, 8);
    release_havok_array(e, base + 0x20, 8);
    release_havok_array(e, base + 0x14, 8);
}

/// `BSPrecisionTimer::Reset` (Xbox PDB, `00448e30`, thiscall `(seconds)`), `BSPrecisionTimer::GetTimer`
/// (`00aa4d80`, 64 bits in `EDX:EAX`) and the `ST0` to 64-bit integer conversion `00ec62f6`.
const TIMER_RESET: u32 = 0x0044_8e30;
const GET_TIMER: u32 = 0x00aa_4d80;
const FLOAT_TO_INT64: u32 = 0x00ec_62f6;
/// The CRT `memset` (`00cb1280`, cdecl `(destination, value, bytes)`).
const MEMSET: u32 = 0x00cb_1280;
/// `00c8cce0` (cdecl, takes the address of a dword) answers whether a motion type is 1, 2, 3 or 6.
const MOTION_TYPE_CHECK: u32 = 0x00c8_cce0;
/// `hkpRigidBody::setMotionType` (Ghidra name, `00ca8420`, thiscall of three arguments),
/// `00ca8630` (thiscall of one) and `hkpEntity::activate` (Ghidra name, `00c9c1d0`).
const SET_MOTION_TYPE: u32 = 0x00ca_8420;
const MOTION_TYPE_FOLLOWUP: u32 = 0x00ca_8630;
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// The timer's seconds float (`011b02e8`), its tick factor (`011ac39c`), and the tolerance
/// (`01017d00`) the pose comparison uses.
const TIMER_SECONDS: u32 = 0x011b_02e8;
const TICK_FACTOR: u32 = 0x011a_c39c;
const POSE_TOLERANCE: u32 = 0x0101_7d00;

/// `CMPLEPS (|a - b|) <= tolerance` on four lanes and `MOVMSKPS`: the bit mask of the lanes that
/// pass (a NaN lane fails).
fn within_tolerance_mask(e: &Engine, first: u32, second: u32, tolerance: f32) -> u32 {
    let a = load4(e, first);
    let b = load4(e, second);
    let mut mask = 0;
    for lane in 0..4 {
        let difference = f32::from_bits((a[lane] - b[lane]).to_bits() & 0x7fff_ffff);
        if difference <= tolerance {
            mask |= 1 << lane;
        }
    }
    mask
}

// Translated from 00c7b6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7b6a0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollController::SetRagdollFeedbackActive` (Ghidra name), thiscall `(active)`: nothing
/// unless the byte `+0x28d` is set and either `active` or the byte `+0x28e` is. When both
/// `+0x28e` and `active` are set it only restarts the timer at `+0x2a8` (`00448e30` with the
/// float at `011b02e8`). Otherwise it runs `UpdateConstraintsWorld(active)` and the timer fields
/// (`+0x2a8` the current timer, `+0x2b0` the float at `011b02e8` times the one at `011ac39c`
/// converted to 64 bits, both zero when `active` is clear). With two scratch blocks taken from
/// the allocation cursor `[00c85750()]` (each `count * 16` bytes rounded up to 128, count from the
/// constraint list's `+0x2c` object's `+0x1c`) and cleared, the first holds, per constraint whose
/// body's motion type byte `+0xe8` passes `00c8cce0`, the body's vector at `+0x1b0`, the
/// second the one at `+0x1c0`. A second pass finds each body's index in the 16-bit table of its
/// owner's skeleton object (`[[this + 0x2a4] + 0x18]`, count `+0x48`, data `+0x44`) and
/// switches the motion type (to 4 when `active` is set, to 1 when not) and then compares the
/// stored vectors with the body's: a lane differing by more than `01017d00` makes the body
/// `activate` and receive the vector through slot `0x40` (`+0x1b0` vector) or `0x44`
/// (`+0x1c0` vector) of its object at `+0xe0`. Last it stores `active` in `+0x28e` and gives the
/// allocation cursor back.
pub fn bhk_ragdoll_controller_set_ragdoll_feedback_active(e: &mut Engine, this: Ptr, active: u8) {
    let base = this.addr();
    if e.mem.u8(base + 0x28d) == 0 {
        return;
    }
    if active == 0 && e.mem.u8(base + 0x28e) == 0 {
        return;
    }
    if e.mem.u8(base + 0x28e) != 0 && active != 0 {
        let seconds = x87_quiet_bits(e.mem.u32(TIMER_SECONDS));
        e.call(TIMER_RESET, &args![base + 0x2a8, seconds]);
        return;
    }
    bhk_ragdoll_controller_update_constraints_world(e, this, active);
    let ticks_low = if active != 0 {
        let seconds = f32::from_bits(x87_quiet_bits(e.mem.u32(TIMER_SECONDS)));
        let timer = e.call(GET_TIMER, &args![]);
        e.mem.set_u32(base + 0x2a8, timer.eax);
        e.mem.set_u32(base + 0x2ac, timer.edx);
        let factor = e.global::<f32>(TICK_FACTOR);
        let ticks = e.call(
            FLOAT_TO_INT64,
            &args![f64::from(factor) * f64::from(seconds)],
        );
        e.mem.set_u32(base + 0x2b4, ticks.edx);
        ticks.eax
    } else {
        e.mem.set_u32(base + 0x2b4, 0);
        0
    };
    e.mem.set_u32(base + 0x2b0, ticks_low);
    let list = e.mem.u32(base + 0x48);
    let bodies_object = e.mem.u32(list + 0x2c);
    let count = e.mem.i32(bodies_object + 0x1c);
    let bytes = (count as u32).wrapping_shl(4);
    let rounded = bytes.wrapping_add(0x7f) & 0xffff_ff80;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let stored = e.mem.u32(router);
    e.mem.set_u32(router, stored.wrapping_add(rounded));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let current = e.mem.u32(router);
    e.mem.set_u32(router, current.wrapping_add(rounded));
    // `stored` holds the body vectors, `current` the second vectors.
    e.call(MEMSET, &args![stored, 0u32, bytes]);
    e.call(MEMSET, &args![current, 0u32, bytes]);
    for index in 0..count {
        let list = e.mem.u32(base + 0x48);
        let slots = e.mem.u32(list + 0x20);
        let slot = e
            .mem
            .i32(slots.wrapping_add((index as u32).wrapping_mul(4)));
        if slot < 0 {
            continue;
        }
        let bodies = e.mem.u32(list + 8);
        let body = e
            .mem
            .u32(bodies.wrapping_add((slot as u32).wrapping_mul(4)));
        if body == 0 {
            continue;
        }
        let kind = u32::from(e.mem.u8(body + 0xe8));
        let passes = e.with_stack(4, |e, scratch_ptr| {
            let scratch = scratch_ptr.addr();
            e.mem.set_u32(scratch, kind);
            e.call(MOTION_TYPE_CHECK, &args![scratch]).u32() & 0xff != 0
        });
        if passes {
            let offset = 16 * index as u32;
            copy16(e, body + 0x1b0, stored.wrapping_add(offset));
            copy16(e, body + 0x1c0, current.wrapping_add(offset));
        }
    }
    for index in 0..count {
        let list = e.mem.u32(base + 0x48);
        let slots = e.mem.u32(list + 0x20);
        let slot = e
            .mem
            .i32(slots.wrapping_add((index as u32).wrapping_mul(4)));
        if slot < 0 {
            continue;
        }
        let bodies = e.mem.u32(list + 8);
        let body = e
            .mem
            .u32(bodies.wrapping_add((slot as u32).wrapping_mul(4)));
        if body == 0 {
            continue;
        }
        let owner = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(owner + 0x18);
        let table_count = e.mem.u32(skeleton + 0x48);
        let table = e.mem.u32(skeleton + 0x44);
        let mut found = None;
        let mut position = 0u32;
        while position < table_count {
            if table != 0 && e.mem.u16(table.wrapping_add(position * 2)) == index as u16 {
                found = Some(position);
                break;
            }
            position += 1;
        }
        if let Some(position) = found {
            if position != 0xffff_ffff {
                let kind = e.mem.u8(body + 0xe8);
                let motion = e.mem.u8(body + 0x2a);
                if active != 0 {
                    if kind == 4 || motion != 4 {
                        e.call(SET_MOTION_TYPE, &args![body, 1u32, 1u32, 0u32]);
                        e.mem.set_u8(body + 0x2a, 4);
                        e.call(MOTION_TYPE_FOLLOWUP, &args![body, 0u32]);
                    }
                } else if kind != 4 || motion != 1 {
                    e.call(SET_MOTION_TYPE, &args![body, 4u32, 1u32, 0u32]);
                    e.mem.set_u8(body + 0x2a, 1);
                    e.call(MOTION_TYPE_FOLLOWUP, &args![body, 1u32]);
                }
            }
        }
        let offset = 16 * index as u32;
        let tolerance = e.global::<f32>(POSE_TOLERANCE);
        let first = stored.wrapping_add(offset);
        if within_tolerance_mask(e, first, body + 0x1b0, tolerance) != 0xf {
            e.call(ENTITY_ACTIVATE, &args![body]);
            e.vcall(body + 0xe0, 0x40, &args![first]);
        }
        let second = current.wrapping_add(offset);
        if within_tolerance_mask(e, second, body + 0x1c0, tolerance) != 0xf {
            e.call(ENTITY_ACTIVATE, &args![body]);
            e.vcall(body + 0xe0, 0x44, &args![second]);
        }
    }
    e.mem.set_u8(base + 0x28e, active);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, current);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, stored);
}

/// `NiFixedString`-style copy (`00a5b690`, cdecl `(string)`, answers the interned string with
/// its count raised) and `hkaSkeletonUtils::findBoneWithName` (Ghidra name, `00cdd390`, cdecl
/// `(skeleton, name, 0)`, 16-bit index in `AX`).
const STRING_COPY: u32 = 0x00a5_b690;
const FIND_BONE_WITH_NAME: u32 = 0x00cd_d390;
/// The pose-matching helpers: `00ce14b0` (constructor, thiscall `(bone, bone, bone, vector)`),
/// `00e92760` (thiscall on `011b02f0`, `(name, destination)`), the model-pose transform
/// `00cddcf0` (cdecl `(count, parents, local, model)`), `addCandidatePose` (`00ce1420`, thiscall
/// `(pose, 0, 0.0f)`) and the finishing call `00cc1360` (thiscall `(skeleton)`, answers a byte).
const POSE_MATCHING_NEW: u32 = 0x00ce_14b0;
const POSE_LOADER: u32 = 0x011b_02f0;
const POSE_LOADER_LOAD: u32 = 0x00e9_2760;
const LOCAL_TO_MODEL_POSE: u32 = 0x00cd_dcf0;
const ADD_CANDIDATE_POSE: u32 = 0x00ce_1420;
const POSE_FINISH: u32 = 0x00cc_1360;
/// The bone names the pose matching looks up: `Bip01 NonAccum`, `Bip01 Head` and
/// `Bip01 R UpperArm`.
const NAME_NON_ACCUM: u32 = 0x0106_4f70;
const NAME_HEAD: u32 = 0x0101_f63c;
const NAME_R_UPPER_ARM: u32 = 0x0102_03a0;
/// Two float constants (`01011d78`, `01012068`) the up vector is built from, and the null
/// string value (`0109b220`).
const VECTOR_FILL: u32 = 0x0101_1d78;
const VECTOR_THIRD: u32 = 0x0101_2068;
const NULL_STRING: u32 = 0x0109_b220;

// Translated from 00c7b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7b1f0 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkRagdollController::InitPoseMatching` (Ghidra name), thiscall `(name, root, head, arm)`
/// (the three bones are 16-bit indices, `0xffff` meaning look them up): nothing unless the byte
/// `+0x42` is set. The name replaces the string at `+0x24c` (copied through `00a5b690`; the old
/// one's count at `-8` is dropped unless it is the null string). The bones not given are found
/// by `Bip01 NonAccum`, `Bip01 Head` and `Bip01 R UpperArm` in the skeleton
/// `[[this + 0x2a4] + 8]`. When the controller is powered or a bone is missing, a 0x30-byte
/// pose matching object is made (`00ce14b0`) into `+0x248`, the loader fills the list at `+0x250`
/// from the name, and each of its poses is copied into scratch blocks taken from the allocation
/// cursor, turned to a model pose and added as a candidate; finally `00cc1360` sets the byte
/// at `+0x254`.
pub fn bhk_ragdoll_controller_init_pose_matching(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    root: u16,
    head: u16,
    arm: u16,
) {
    let base = this.addr();
    if e.mem.u8(base + 0x42) == 0 {
        return;
    }
    let previous = e.mem.u32(base + 0x24c);
    if previous != name {
        let copy = e.call(STRING_COPY, &args![name]).u32();
        e.mem.set_u32(base + 0x24c, copy);
        if previous != e.mem.u32(NULL_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![previous.wrapping_sub(8)]);
        }
    }
    let owner = e.mem.u32(base + 0x2a4);
    let skeleton = e.mem.u32(owner + 8);
    let root = if root == 0xffff {
        u32::from(
            e.call(FIND_BONE_WITH_NAME, &args![skeleton, NAME_NON_ACCUM, 0u32])
                .u32() as u16,
        )
    } else {
        u32::from(root)
    };
    let head = if head == 0xffff {
        u32::from(
            e.call(FIND_BONE_WITH_NAME, &args![skeleton, NAME_HEAD, 0u32])
                .u32() as u16,
        )
    } else {
        u32::from(head)
    };
    let arm = if arm == 0xffff {
        e.call(
            FIND_BONE_WITH_NAME,
            &args![skeleton, NAME_R_UPPER_ARM, 0u32],
        )
        .u32() as u16
    } else {
        arm
    };
    let fill = e.mem.u32(VECTOR_FILL);
    let third = e.mem.u32(VECTOR_THIRD);
    let powered = bhk_ragdoll_controller_is_powered_ragdoll(e, this);
    if !(powered || root as u16 == 0xffff || head as u16 == 0xffff || arm == 0xffff) {
        return;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let object = e.vcall(allocator, 4, &args![0x30u32]).u32();
    let matching = if object != 0 {
        e.with_stack(0x10, |e, vector_ptr| {
            let vector = vector_ptr.addr();
            for (lane, bits) in [fill, fill, third, fill].into_iter().enumerate() {
                e.mem.set_u32(vector + 4 * lane as u32, bits);
            }
            e.call(
                POSE_MATCHING_NEW,
                &args![object, root, head, u32::from(arm), vector],
            )
            .u32()
        })
    } else {
        0
    };
    e.mem.set_u32(base + 0x248, matching);
    e.call(POSE_LOADER_LOAD, &args![POSE_LOADER, name, base + 0x250]);
    let candidates = e.mem.u32(base + 0x250);
    if candidates == 0 {
        return;
    }
    let mut index = 0u32;
    if e.mem.u32(candidates + 0x10) != 0 {
        loop {
            let candidates = e.mem.u32(base + 0x250);
            let list = e.mem.u32(candidates + 0x14);
            let item = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
            let count = e.mem.i32(skeleton + 0x1c);
            let rounded = (count as u32).wrapping_mul(0x30).wrapping_add(0x7f) & 0xffff_ff80;
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let local = e.mem.u32(router);
            e.mem.set_u32(router, local.wrapping_add(rounded));
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let model = e.mem.u32(router);
            e.mem.set_u32(router, model.wrapping_add(rounded));
            for pose in 0..count.max(0) as u32 {
                let source = e.mem.u32(item + 4).wrapping_add(0x30 * pose);
                copy16(e, source, local.wrapping_add(0x30 * pose));
                copy16(e, source + 0x10, local.wrapping_add(0x30 * pose + 0x10));
                copy16(e, source + 0x20, local.wrapping_add(0x30 * pose + 0x20));
            }
            let parents = e.mem.u32(skeleton + 0xc);
            e.call(
                LOCAL_TO_MODEL_POSE,
                &args![count as u32, parents, local, model],
            );
            let matching = e.mem.u32(base + 0x248);
            e.call(ADD_CANDIDATE_POSE, &args![matching, model, 0u32, 0u32]);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            e.mem.set_u32(router, model);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            e.mem.set_u32(router, local);
            index += 1;
            let candidates = e.mem.u32(base + 0x250);
            if index >= e.mem.u32(candidates + 0x10) {
                break;
            }
        }
    }
    let candidates = e.mem.u32(base + 0x250);
    let finished = e.call(POSE_FINISH, &args![candidates, skeleton]).u32() as u8;
    e.mem.set_u8(base + 0x254, finished);
}

/// The key-to-index lookup `00c74990` (thiscall on the hash map, `(key, result)`), the
/// array append `00b5c360` (thiscall, answers the new index) for the array object at `x`, the
/// one `00761540` for the controller's array at `+0xa0`, and `bhkUtilFunctions::FindNextCollisionObject`
/// (Ghidra name, `00c802d0`, cdecl `(node)`).
const MAP_LOOKUP: u32 = 0x00c7_4990;
const ARRAY_APPEND: u32 = 0x00b5_c360;
const NODE_ARRAY_APPEND: u32 = 0x0076_1540;
const FIND_NEXT_COLLISION_OBJECT: u32 = 0x00c8_02d0;
/// The type descriptor (`0126817c`) whose derived objects the walk does not enter.
const SKIPPED_TYPE: u32 = 0x0126_817c;

// Translated from 00c7b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(node, records, map)`, no name in the Xbox PDB or the Ghidra database: records one
/// scene graph node. A 0x60-byte block comes from `MemoryManager::Allocate` (aligned up to 16
/// with the pad in the byte before it), gets `[node + 8]` at `+0`, `ni2hk_ov2` of `node + 0x34`
/// at `+0x10` and the 16-bit result of `map.lookup(node.[+0x18])` (0xffff when missing) at
/// `+0x40`; when it was found the vector at `+0x10` is scaled by the float `[[this + 0x58] + 0x98]`.
/// The map then gets `node -> [records + 8]` (16-bit), the block is appended to the array at
/// `records + 4`, and `node` to the one at `this + 0xa0`. Then each child of the node's slot `0xc`
/// object (pointer `+0xa0`, 16-bit count `+0xa6`) whose collision object (`00c802d0`, its slot
/// `0x3c` object) exists and whose type chain does not contain `0126817c` is recorded the same way.
pub fn fn_00c7b4f0(e: &mut Engine, this: Ptr, node: u32, records: u32, map: u32) {
    let controller = this.addr();
    let raw = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x60u32])
        .u32();
    let pad = (0x10u32.wrapping_sub(raw & 0xf)) as u8;
    let block = raw.wrapping_add(u32::from(pad));
    e.mem.set_u8(block.wrapping_sub(1), pad);
    let width = e.mem.u32(node + 8);
    e.mem.set_u32(block, width);
    let vector = block + 0x10;
    ni2hk_ov2(e, Ptr::new(vector), Ptr::new(node + 0x34));
    let key = e.mem.u32(node + 0x18);
    let found = e.with_stack(4, |e, out_ptr| {
        let out = out_ptr.addr();
        e.mem.set_u32(out, 0xffff_ffff);
        e.call(MAP_LOOKUP, &args![map, key, out]);
        e.mem.u32(out)
    });
    e.mem.set_u16(block + 0x40, found as u16);
    if found as u16 != 0xffff {
        let owner = e.mem.u32(controller + 0x58);
        let scale = e.mem.f32(owner + 0x98);
        for lane in 0..4 {
            let value = e.mem.f32(vector + 4 * lane);
            e.mem.set_f32(vector + 4 * lane, scale * value);
        }
    }
    let number = u32::from(e.mem.u16(records + 8));
    fn_00c74910(e, Ptr::new(map), node, number);
    let slot = e.call(ARRAY_APPEND, &args![records]).u32();
    let data = e.mem.u32(records + 4);
    e.mem
        .set_u32(data.wrapping_add(slot.wrapping_mul(4)), block);
    let nodes = controller + 0xa0;
    let slot = e.call(NODE_ARRAY_APPEND, &args![nodes]).u32();
    let data = e.mem.u32(nodes + 4);
    e.mem.set_u32(data.wrapping_add(slot.wrapping_mul(4)), node);
    let group = e.vcall(node, 0xc, &args![]).u32();
    if group == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(group + 0xa6)) {
        let children = e.mem.u32(group + 0xa0);
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        let mut object = 0;
        let mut flagged = 0;
        if child != 0 {
            let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![child]).u32();
            if collision != 0 {
                object = e.vcall(collision, 0x3c, &args![]).u32();
                if object != 0 {
                    let mut kind = e.vcall(object, 8, &args![]).u32();
                    while kind != 0 {
                        if kind == SKIPPED_TYPE {
                            flagged = object;
                            break;
                        }
                        kind = e.mem.u32(kind + 4);
                    }
                }
            }
        }
        if child != 0 && object != 0 && flagged == 0 {
            fn_00c7b4f0(e, this, child, records, map);
        }
        index += 1;
    }
}

/// `hkaRagdollInstance::getPoseModelSpace` (Ghidra name, `00ce18f0`, thiscall `(destination,
/// pose)`), `hkaPoseMatchingUtility::findBestCandidatePoseIndex` (Ghidra name, `00ce11f0`, thiscall
/// `(pose, score)`), the candidate apply call `00cc1420` (thiscall `(target, object, index)`) and
/// the score limit float `011b02e4`.
const GET_POSE_MODEL_SPACE: u32 = 0x00ce_18f0;
const FIND_BEST_CANDIDATE: u32 = 0x00ce_11f0;
const APPLY_CANDIDATE: u32 = 0x00cc_1420;
const SCORE_LIMIT: u32 = 0x011b_02e4;

// Translated from 00c7ba50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7ba50 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollController::DoPoseMatching` (Ghidra name), thiscall `(pose, target, flag)`: the
/// distance between the vector at `+0x270` and the one at `[this + 0x58] + 0x8c` (squares
/// summed `((dy*dy + dx*dx) + dz*dz)` through `f64`, narrowed, rooted by the CRT root `00ec6040`)
/// and the timer decide whether to run: the time since `+0x260` is above the limit at `+0x268`,
/// or the byte `+0x284` is set and the float at `+0x27c` is below the distance, or the index at
/// `+0x288` is not -1. A stored index is applied (`00cc1420`) and cleared; otherwise the
/// vector is stored, the model space pose (`00ce18f0`) is taken into a block from the
/// allocation cursor and the best candidate index found (`00ce11f0`, score into `+8`). A score
/// below `011b02e4` with an index that differs from `+0x258` (and is not -1) is applied; then
/// `+0xc` is 3 and the byte `*flag` is 1, otherwise an unset flag resets `+0xc`, and sets the byte at
/// `+4`. Last the timer fields `+0x260` (now) and `+0x268` (the float at `+0x280` times the
/// one at `011ac39c`, as 64 bits) are renewed.
pub fn bhk_ragdoll_controller_do_pose_matching(
    e: &mut Engine,
    this: Ptr,
    pose: u32,
    target: u32,
    flag: u32,
) {
    let base = this.addr();
    let world = e.mem.u32(base + 0x58);
    let dx = (f64::from(e.mem.f32(base + 0x270)) - f64::from(e.mem.f32(world + 0x8c))) as f32;
    let dy = (f64::from(e.mem.f32(base + 0x274)) - f64::from(e.mem.f32(world + 0x90))) as f32;
    let dz = (f64::from(e.mem.f32(base + 0x278)) - f64::from(e.mem.f32(world + 0x94))) as f32;
    let (dx, dy, dz) = (f64::from(dx), f64::from(dy), f64::from(dz));
    let sum = ((dy * dy + dx * dx) + dz * dz) as f32;
    let distance = x87_quiet(e.call(X87_SQRT, &args![f64::from(sum)]).f32());
    let now = e.call(GET_TIMER, &args![]);
    let elapsed = (u64::from(now.edx) << 32 | u64::from(now.eax))
        .wrapping_sub(u64::from(e.mem.u32(base + 0x264)) << 32 | u64::from(e.mem.u32(base + 0x260)))
        as i64;
    let limit =
        (u64::from(e.mem.u32(base + 0x26c)) << 32 | u64::from(e.mem.u32(base + 0x268))) as i64;
    let run = elapsed > limit
        || (e.mem.u8(base + 0x284) != 0 && e.mem.f32(base + 0x27c) < distance)
        || e.mem.i32(base + 0x288) != -1;
    if !run {
        return;
    }
    let stored = e.mem.i32(base + 0x288);
    let objects = e.mem.u32(base + 0x48);
    let object = e.mem.u32(objects + 0x2c);
    if stored != -1 {
        let applier = e.mem.u32(base + 0x250);
        e.call(
            APPLY_CANDIDATE,
            &args![applier, target, object, stored as u32],
        );
        e.mem.set_u32(base + 0xc, 3);
        e.mem.set_u8(flag, 1);
        let chosen = e.mem.u32(base + 0x288);
        e.mem.set_u32(base + 0x258, chosen);
        e.mem.set_u32(base + 0x288, 0xffff_ffff);
    } else {
        for offset in [0, 4, 8] {
            let value = e.mem.u32(world + 0x8c + offset);
            e.mem.set_u32(base + 0x270 + offset, value);
        }
        let count = e.mem.i32(object + 0x1c);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let block = e.mem.u32(router);
        let size = (count as u32).wrapping_mul(0x30).wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, block.wrapping_add(size));
        e.call(GET_POSE_MODEL_SPACE, &args![objects, block, pose]);
        let matching = e.mem.u32(base + 0x248);
        let index = e
            .call(FIND_BEST_CANDIDATE, &args![matching, block, base + 8])
            .u32();
        let score = e.mem.f32(base + 8);
        let limit = e.mem.f32(SCORE_LIMIT);
        let mut applies = false;
        if limit > score && index != e.mem.u32(base + 0x258) {
            e.mem.set_u32(base + 0x258, index);
            applies = index != 0xffff_ffff;
        }
        if applies {
            let applier = e.mem.u32(base + 0x250);
            e.call(APPLY_CANDIDATE, &args![applier, target, object, index]);
            e.mem.set_u32(base + 0xc, 3);
            e.mem.set_u8(flag, 1);
        } else if e.mem.u8(flag) == 0 {
            e.mem.set_u32(base + 0xc, 0);
            e.mem.set_u8(flag, 0);
            e.mem.set_u8(base + 4, 1);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, block);
    }
    let interval = x87_quiet(e.mem.f32(base + 0x280));
    let now = e.call(GET_TIMER, &args![]);
    e.mem.set_u32(base + 0x260, now.eax);
    e.mem.set_u32(base + 0x264, now.edx);
    let factor = e.global::<f32>(TICK_FACTOR);
    let ticks = e.call(
        FLOAT_TO_INT64,
        &args![f64::from(factor) * f64::from(interval)],
    );
    e.mem.set_u32(base + 0x268, ticks.eax);
    e.mem.set_u32(base + 0x26c, ticks.edx);
}

/// `NiFixedString::operator==` against a C string (`00a305e0`, thiscall `(text)`, answers a
/// byte) and `BSUtilities::GetObjectByName` (Ghidra name, `00c4b470`, cdecl `(root, &name, 1)`).
const FIXED_STRING_EQUALS: u32 = 0x00a3_05e0;
const GET_OBJECT_BY_NAME: u32 = 0x00c4_b470;

// Translated from 00c7bcb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7bcb0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollController::CopyPoseTo3DRec` (Ghidra name), cdecl `(root, node, transforms)`:
/// takes a counted copy of the node's name string (`[node + 8]`, the count at `-8` raised
/// unless it is the null string `[0109b220]`), and looks the node up under `root` by that name
/// (`00c4b470`). A node found gets its 14-dword record (the pointer, then its `+0x34`
/// transform block) appended to the array `transforms` (0x38-byte elements, `00c7a730` gives the
/// index and `00c760f0` constructs it), and takes over the transform block `+0x34..+0x58` and
/// the vector `+0x58..+0x64` of the source node, which is the node's `+0x18` object when the
/// name equals `Bip01 NonAccum`. Then the same runs for each child of the node's slot `0xc` object
/// (pointer `+0xa0`, 16-bit count `+0xa6`), and the name's count is dropped again.
pub fn bhk_ragdoll_controller_copy_pose_to_3d_rec(
    e: &mut Engine,
    root: u32,
    node: u32,
    transforms: Ptr,
) {
    e.with_stack(4, |e, name_ptr| {
        let name = name_ptr.addr();
        let held = e.mem.u32(node + 8);
        if held != e.mem.u32(NULL_STRING) {
            e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_sub(8)]);
        }
        let kept = e.mem.u32(node + 8);
        e.mem.set_u32(name, kept);
        let mut source = node;
        let equal = e
            .call(FIXED_STRING_EQUALS, &args![name, NAME_NON_ACCUM])
            .u32() as u8;
        if equal != 0 {
            source = e.mem.u32(node + 0x18);
        }
        let found = e.call(GET_OBJECT_BY_NAME, &args![root, name, 1u32]).u32();
        if found != 0 {
            let mut record = vec![found];
            for index in 0..13 {
                record.push(e.mem.u32(found + 0x34 + 4 * index));
            }
            let slot = fn_00c7a730(e, transforms);
            let element = e
                .mem
                .u32(transforms.addr() + 4)
                .wrapping_add(slot.wrapping_mul(0x38));
            fn_00c760f0(e, element, 1);
            let element = e
                .mem
                .u32(transforms.addr() + 4)
                .wrapping_add(slot.wrapping_mul(0x38));
            for (index, value) in record.into_iter().enumerate() {
                e.mem.set_u32(element + 4 * index as u32, value);
            }
            for index in 0..9 {
                let value = e.mem.u32(source + 0x34 + 4 * index);
                e.mem.set_u32(found + 0x34 + 4 * index, value);
            }
            for offset in [0x58, 0x5c, 0x60] {
                let value = e.mem.u32(source + offset);
                e.mem.set_u32(found + offset, value);
            }
        }
        let group = e.vcall(node, 0xc, &args![]).u32();
        if group != 0 {
            let mut index = 0u32;
            while index < u32::from(e.mem.u16(group + 0xa6)) {
                let children = e.mem.u32(group + 0xa0);
                let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
                if child != 0 {
                    bhk_ragdoll_controller_copy_pose_to_3d_rec(e, root, child, transforms);
                }
                index += 1;
            }
        }
        let held = e.mem.u32(name);
        if held != e.mem.u32(NULL_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![held.wrapping_sub(8)]);
        }
    });
}

/// `bhkUtilFunctions::SetCOUseVel` (Ghidra name, `00c80330`, cdecl `(0, object)`),
/// `bhkWorld::SetMotion` (Ghidra name, `00c6a350`, cdecl `(object, 4, 1, 0, 1)`) and the update
/// call `00a59c60` (thiscall on the object, `(&update_data)`; the update data is a float
/// followed by five bytes).
const SET_CO_USE_VEL: u32 = 0x00c8_0330;
const SET_MOTION: u32 = 0x00c6_a350;
const UPDATE_OBJECT: u32 = 0x00a5_9c60;

// Translated from 00c7d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::CopyPoseTo3D` (Ghidra name), cdecl `(object, node, transforms, moved,
/// motion)`: the collision object of `object` (`00c802d0`) decides `*moved`: it is 1 when the
/// object exists, has bit 5 and not bit 6 of its 16-bit flags at `+0xc` set. A moved object
/// gets `SetCOUseVel(0, object)`. An existing collision object also stores the motion type
/// byte (`+0xe8`) of the body at `[[co + 0x10] + 8]` into `*motion` (5 when there is none).
/// A motion type other than 4 is set to 4 (`00c6a350`). With both `object` and `node`, the
/// pose is copied (`bhk_ragdoll_controller_copy_pose_to_3d_rec`) and `object` is updated
/// through `00a59c60` with a zeroed update record.
pub fn bhk_ragdoll_controller_copy_pose_to_3d(
    e: &mut Engine,
    object: u32,
    node: u32,
    transforms: Ptr,
    moved: u32,
    motion: u32,
) {
    let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![object]).u32();
    let flag = if collision != 0 {
        let flags = e.mem.u16(collision + 0xc);
        u8::from(flags & 0x20 != 0 && (flags as u8 >> 6) & 1 == 0)
    } else {
        0
    };
    e.mem.set_u8(moved, flag);
    if flag != 0 {
        e.call(SET_CO_USE_VEL, &args![0u32, object]);
    }
    if collision != 0 {
        let body = e.mem.u32(collision + 0x10);
        let kind = if body != 0 {
            let body = e.mem.u32(body + 8);
            if body != 0 {
                u32::from(e.mem.u8(body + 0xe8))
            } else {
                5
            }
        } else {
            5
        };
        e.mem.set_u32(motion, kind);
    }
    if e.mem.u32(motion) != 4 {
        e.call(SET_MOTION, &args![object, 4u32, 1u32, 0u32, 1u32]);
    }
    if object != 0 && node != 0 {
        bhk_ragdoll_controller_copy_pose_to_3d_rec(e, object, node, transforms);
        e.with_stack(0xc, |e, record_ptr| {
            let record = record_ptr.addr();
            e.mem.set_f32(record, 0.0);
            for offset in 4..9 {
                e.mem.set_u8(record + offset, 0);
            }
            e.call(UPDATE_OBJECT, &args![object, record]);
        });
    }
}

/// Destructor tail of the local Havok array that `00c7d480` and `00c7d520` share (register
/// `ECX`; data `+0`, size `+4`, flags `+8`, saved allocation cursor `+0xc`; 4-byte elements):
/// when the saved cursor `+0xc` equals the data pointer the size is cleared, the allocation
/// cursor `[00c85750()]` is set back to it and the array is released.
fn stack_array_destruct(e: &mut Engine, this: u32) {
    let saved = e.mem.u32(this + 0xc);
    if saved == e.mem.u32(this) {
        e.mem.set_u32(this + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved);
    release_havok_array(e, this, 4);
}

// Translated from 00c7d480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7d480 - the FS:[0] exception-unwinding frame is not modeled.
/// Register `ECX`: destructor of a local Havok array of 4-byte elements (no name in the
/// Xbox PDB or the Ghidra database); see `stack_array_destruct`.
pub fn fn_00c7d480(e: &mut Engine, this: Ptr) {
    stack_array_destruct(e, this.addr());
}

// Translated from 00c7d520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7d520 - the FS:[0] exception-unwinding frame is not modeled.
/// Register `ECX`: the same local Havok array destructor as `00c7d480` (a separate copy with
/// its own unwinding handler); see `stack_array_destruct`.
pub fn fn_00c7d520(e: &mut Engine, this: Ptr) {
    stack_array_destruct(e, this.addr());
}

// Translated from 00c7d810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::SyncRagdollTo3D` (Ghidra name), register `ECX`: a transform record
/// (13 dwords) is copied from `[this + 0x58] + 0x68`, its rotation (9 dwords) replaced by the
/// matrix `0043f8d0` (thiscall on `this + 0x60`, `(result, record)`) gives and its scale set to
/// 1.0, and `ni2hk_ov2` converts it into `this + 0x10`. Then `GetSGPoseLocalSpace` (`00c79680`)
/// runs, `UpdateRagdollRigidBodies(0)` (`00c7d030`) with the word `+0xc` temporarily set to 1,
/// and the object at `+0x4c`, when there is one, is reinitialised (`00cdb3e0`).
pub fn bhk_ragdoll_controller_sync_ragdoll_to_3d(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let world = e.mem.u32(base + 0x58);
    e.with_stack(0x34, |e, record_ptr| {
        let record = record_ptr.addr();
        for index in 0..13 {
            let value = e.mem.u32(world + 0x68 + 4 * index);
            e.mem.set_u32(record + 4 * index, value);
        }
        e.with_stack(0x24, |e, result| {
            let product = e
                .call(MATRIX_MULTIPLY, &args![base + 0x60, result, record])
                .u32();
            for index in 0..9 {
                let value = e.mem.u32(product + 4 * index);
                e.mem.set_u32(record + 4 * index, value);
            }
        });
        e.mem.set_f32(record + 0x30, 1.0);
        ni2hk_ov2(e, Ptr::new(base + 0x10), Ptr::new(record));
    });
    bhk_ragdoll_controller_get_sg_pose_local_space(e, this);
    let saved = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 0xc, 1);
    bhk_ragdoll_controller_update_ragdoll_rigid_bodies(e, this, 0);
    e.mem.set_u32(base + 0xc, saved);
    let controller = e.mem.u32(base + 0x4c);
    if controller != 0 {
        e.call(RIGID_BODY_REINITIALIZE, &args![controller]);
    }
}

/// `NiMatrix3` product `0043f8d0` (thiscall on the matrix, `(result, transform)`, answers the
/// result), and
/// `hkaRagdollRigidBodyController::reinitialize` (Ghidra name, `00cdb3e0`).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const RIGID_BODY_REINITIALIZE: u32 = 0x00cd_b3e0;

/// The destructors the controller destructor and the skeleton destructor call: the pose
/// array element destructor `00c905b0` (thiscall), `00cdb6f0` (the rigid body controller
/// destructor, thiscall), the string pointer destructor `00cb0c00`
/// (`hkStringPtr::~hkStringPtr`, Ghidra name) and the pose loader's release `00e91fd0` (thiscall on
/// the loader, `(name)`).
const ELEMENT_DESTRUCT: u32 = 0x00c9_05b0;
const RIGID_BODY_CONTROLLER_DESTRUCT: u32 = 0x00cd_b6f0;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const POSE_LOADER_RELEASE: u32 = 0x00e9_1fd0;
/// The second pose loader object (`011b0300`), the vtable of the base class the array at
/// `+0xa0` reverts to (`01071aec`), and the controller's vtables during destruction.
const SECOND_LOADER: u32 = 0x011b_0300;
const ARRAY_BASE_VTABLE: u32 = 0x0107_1aec;
const CONTROLLER_DERIVED_VTABLE: u32 = 0x010c_4ddc;
const CONTROLLER_BASE_VTABLE: u32 = 0x010c_4b60;
const SKELETON_VTABLE: u32 = 0x0102_e388;

// Translated from 00c7d900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7d900 - the FS:[0] exception-unwinding frame is not modeled.
/// Destructor body of the ragdoll controller (no name in the Xbox PDB or the Ghidra database;
/// register `ECX`). It sets the vtable `010c4ddc`, destroys the `[this + 0x230]` elements of
/// 0xd0 bytes at `+0x22c` (`00c905b0` on the first dword of each) and clears the count,
/// destroys the objects at `+0x4c` (`00cdb6f0` and a 0x48-byte free through allocator slot 8),
/// `+0x48` (`00c905b0`), `+0x54` and `+0x88` (`00c7c030` and a 0x38-byte free). The pose loader
/// `011b02f0` is asked for the name `+0x24c` into `+0x250` and, when that object's count at
/// `+4` is 2, releases the name (`00e91fd0`); the same for the object at `+0x2a4` against the
/// loader `011b0300` and the name `+0x2a0`. The references at `+0x2a4` and `+0x250` are
/// dropped and cleared, the Havok array object at `+0x248` (32-byte elements, 0x30 bytes
/// itself) is freed, the references are dropped again, the name counts at `-8` of `+0x2a0` and
/// `+0x24c` are lowered (unless they are the null string), the arrays at `+0x22c` (0xd0-byte
/// elements) and `+0x94` (0x30) are released, the member at `+0xa0` (vtable `01071aec`) frees
/// its storage through slot 8, the reference at `+0x84` is dropped and the vtable becomes
/// `010c4b60`.
pub fn fn_00c7d900(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, CONTROLLER_DERIVED_VTABLE);
    let mut remaining = e.mem.i32(base + 0x230);
    if remaining > 0 {
        let mut offset = 0u32;
        while remaining != 0 {
            let table = e.mem.u32(base + 0x22c);
            let element = e.mem.u32(table.wrapping_add(offset));
            e.call(ELEMENT_DESTRUCT, &args![element]);
            offset = offset.wrapping_add(0xd0);
            remaining = remaining.wrapping_sub(1);
        }
    }
    e.mem.set_u32(base + 0x230, 0);
    let rigid_bodies = e.mem.u32(base + 0x4c);
    if rigid_bodies != 0 {
        e.call(RIGID_BODY_CONTROLLER_DESTRUCT, &args![rigid_bodies]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![rigid_bodies, 0x48u32]);
    }
    let list = e.mem.u32(base + 0x48);
    if list != 0 {
        e.call(ELEMENT_DESTRUCT, &args![list]);
    }
    for field in [0x54, 0x88] {
        let object = e.mem.u32(base + field);
        if object != 0 {
            fn_00c7c030(e, Ptr::new(object));
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![object, 0x38u32]);
        }
    }
    let name = e.mem.u32(base + 0x24c);
    e.call(POSE_LOADER_LOAD, &args![POSE_LOADER, name, base + 0x250]);
    let candidates = e.mem.u32(base + 0x250);
    if candidates != 0 && e.mem.u32(candidates + 4) == 2 {
        let name = e.mem.u32(base + 0x24c);
        e.call(POSE_LOADER_RELEASE, &args![POSE_LOADER, name]);
    }
    let owner = e.mem.u32(base + 0x2a4);
    if owner != 0 && e.mem.u32(owner + 4) == 2 {
        let name = e.mem.u32(base + 0x2a0);
        e.call(POSE_LOADER_RELEASE, &args![SECOND_LOADER, name]);
    }
    let owner = e.mem.u32(base + 0x2a4);
    if owner != 0 {
        release_reference(e, owner);
        e.mem.set_u32(base + 0x2a4, 0);
    }
    let candidates = e.mem.u32(base + 0x250);
    if candidates != 0 {
        release_reference(e, candidates);
        e.mem.set_u32(base + 0x250, 0);
    }
    let matching = e.mem.u32(base + 0x248);
    if matching != 0 {
        release_havok_array(e, matching, 0x20);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![matching, 0x30u32]);
    }
    let owner = e.mem.u32(base + 0x2a4);
    if owner != 0 {
        release_reference(e, owner);
    }
    let name = e.mem.u32(base + 0x2a0);
    if name != e.mem.u32(NULL_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    let candidates = e.mem.u32(base + 0x250);
    if candidates != 0 {
        release_reference(e, candidates);
    }
    let name = e.mem.u32(base + 0x24c);
    if name != e.mem.u32(NULL_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    release_havok_array(e, base + 0x22c, 0xd0);
    let member = base + 0xa0;
    let storage = e.mem.u32(member + 4);
    e.mem.set_u32(member, ARRAY_BASE_VTABLE);
    if storage != 0 {
        e.vcall(member, 8, &args![storage]);
        e.mem.set_u32(member + 4, 0);
        e.mem.set_u32(member + 0xc, 0);
        e.mem.set_u32(member + 8, 0);
    }
    release_havok_array(e, base + 0x94, 0x30);
    let reference = e.mem.u32(base + 0x84);
    if reference != 0 {
        release_reference(e, reference);
    }
    e.mem.set_u32(base, CONTROLLER_BASE_VTABLE);
}

// Translated from 00c7dc40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7dc40 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkaSkeleton::~hkaSkeleton` (Ghidra name), register `ECX`: releases the Havok arrays of the
/// skeleton one by one. The array at `+0x3c` (8-byte elements, size `+0x40`) first gets
/// `00c905b0` on the pointer of each element, which is cleared; the one at `+0x30` (4 bytes,
/// size `+0x34`) and the one at `+0x18` (8 bytes, size `+0x1c`) get `hkStringPtr::~hkStringPtr`
/// (`00cb0c00`) on each element's address; the arrays at `+0x24` (0x30 bytes) and `+0xc` (2
/// bytes) are plain. Then the string pointer at `+8` is destroyed and the vtable becomes
/// `0102e388`. The allocator is fetched once before the first array and again before each
/// following one.
pub fn hka_skeleton_destruct(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.i32(base + 0x40);
    let data = e.mem.u32(base + 0x3c);
    for index in 0..count.max(0) as u32 {
        let slot = data.wrapping_add(index.wrapping_mul(8));
        let element = e.mem.u32(slot);
        if element != 0 {
            e.call(ELEMENT_DESTRUCT, &args![element]);
        }
        e.mem.set_u32(slot, 0);
    }
    free_havok_array(e, allocator, base + 0x3c, 8);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.i32(base + 0x34);
    let data = e.mem.u32(base + 0x30);
    for index in 0..count.max(0) as u32 {
        e.call(
            STRING_PTR_DESTRUCT,
            &args![data.wrapping_add(index.wrapping_mul(4))],
        );
    }
    free_havok_array(e, allocator, base + 0x30, 4);
    release_havok_array(e, base + 0x24, 0x30);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let count = e.mem.i32(base + 0x1c);
    let data = e.mem.u32(base + 0x18);
    for index in 0..count.max(0) as u32 {
        e.call(
            STRING_PTR_DESTRUCT,
            &args![data.wrapping_add(index.wrapping_mul(8))],
        );
    }
    free_havok_array(e, allocator, base + 0x18, 8);
    release_havok_array(e, base + 0xc, 2);
    e.call(STRING_PTR_DESTRUCT, &args![base + 8]);
    e.mem.set_u32(base, SKELETON_VTABLE);
}

// Translated from 00c7de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollController::_scalar_deleting_destructor_` (Ghidra name), thiscall `(flags)`:
/// runs the destructor body (`fn_00c7d900`) and, when bit 0 of `flags` is set and `this` is not
/// 0, frees the block through `MemoryManager::Deallocate` at `this` minus the pad byte
/// stored before it. Answers `this`.
pub fn bhk_ragdoll_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00c7d900(e, this);
    if flags & 1 != 0 && this.addr() != 0 {
        let pad = u32::from(e.mem.u8(this.addr().wrapping_sub(1)));
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, this.addr().wrapping_sub(pad)],
        );
    }
    this
}

/// `_CIacos` (`00eca1d0`, the argument in `ST0`, a leading `f64`), the floats `1.0` (`01012068`)
/// and pi (`0102b3c8`) of the arc-cosine range check, the vector of absolute-value masks
/// (`01017d10`) and the float `0.001` (`01017d00`) of the quaternion comparison, and the
/// angle limit floats `01267d24` and `01267d30` (in degrees) with the `double` degrees-to-radians
/// factor (`01023128`).
const CRT_ARC_COSINE: u32 = 0x00ec_a1d0;
const ARC_COSINE_ONE: u32 = 0x0101_2068;
const ARC_COSINE_PI: u32 = 0x0102_b3c8;
const QUATERNION_ABS_MASK: u32 = 0x0101_7d10;
const QUATERNION_TOLERANCE: u32 = 0x0101_7d00;
const ANGLE_LIMIT_SOFT: u32 = 0x0126_7d24;
const ANGLE_LIMIT_HARD: u32 = 0x0126_7d30;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// `hkpEntity`-side pose helper `00cda8e0` (thiscall on the object, one word: a bone index,
/// answers a pointer to a transform).
const POSE_BONE_TRANSFORM: u32 = 0x00cd_a8e0;

/// The arc cosine of the float with the bits `x` as `hkMath::acos` and `getAngle` compute it:
/// `|x|` is stored as a float and compared with 1.0; below 1.0 (or when either is a NaN) the CRT
/// `acos` of `x` is taken, otherwise the answer is 0.0 for a positive `x` and pi for any other.
/// Answers the bits of the float result.
fn arc_cosine_bits(e: &mut Engine, x: u32) -> u32 {
    let loaded = x87_load32(x);
    let magnitude = x87_store32(loaded.abs());
    let one = x87_m32(e.mem.u32(ARC_COSINE_ONE));
    if !(x87_load32(magnitude) >= one) {
        x87_store32(e.call(CRT_ARC_COSINE, &args![loaded]).f64())
    } else if 0.0 < loaded {
        0.0f32.to_bits()
    } else {
        x87_store32(x87_load32(e.mem.u32(ARC_COSINE_PI)))
    }
}

// Translated from 00c74a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMath::acos` (the engine map has no name; `hkquaternion.rs` calls it so), cdecl with one
/// float stack word, the float answer in `ST0`: the arc cosine of `x`, but exactly 0 or pi
/// (instead of a NaN) when `|x|` is 1 or more (see `arc_cosine_bits`).
pub fn fn_00c74a70(e: &mut Engine, x: f32) -> f32 {
    f32::from_bits(arc_cosine_bits(e, x.to_bits()))
}

// Translated from 00c74c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Quaternion angle (the engine map has no name; `hkquaternion.rs` calls it `getAngle`):
/// thiscall on the quaternion, answers twice the arc cosine of `|w|` (the float at `+0xc`, taken
/// absolute and stored as a float first) in `ST0`, with the same range handling as `00c74a70`.
pub fn fn_00c74c20(e: &mut Engine, this: Ptr) -> f32 {
    let w = x87_load32(e.mem.u32(this.addr() + 0xc));
    let magnitude = x87_store32(w.abs());
    let arc = arc_cosine_bits(e, magnitude);
    let doubled = x87_add(x87_load32(arc), x87_load32(arc));
    f32::from_bits(x87_store32(doubled))
}

// Translated from 00c74ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c74ac0 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// Quaternion product with the inverse of the second (no engine-map name; `RET 8`: `this` is
/// the result, the stack words are the quaternions `a` and `b`, each four floats): with `v` the
/// vector parts and `w` the fourth lanes, the result lanes 0 to 2 are
/// `b.w * a.v + (b.v x a.v - a.w * b.v)` and lane 3 is the sum of the lane products of `a` and
/// `b` (summed `(p3 + p1) + (p2 + p0)`), which is `a.w * b.w + a.v . b.v`.
pub fn fn_00c74ac0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let a = load4(e, a.addr());
    let b = load4(e, b.addr());
    let cross = subps(
        mulps(shufps(b, b, 0xc9), shufps(a, a, 0xd2)),
        mulps(shufps(b, b, 0xd2), shufps(a, a, 0xc9)),
    );
    let vector_part = subps(cross, mulps(shufps(a, a, 0xff), b));
    let products = mulps(b, a);
    let by_b_w = mulps(shufps(b, b, 0xff), a);
    let folded = addps(shufps(products, products, 0x4e), products);
    let sum = addps(shufps(folded, folded, 0xb1), folded);
    let vector = addps(by_b_w, vector_part);
    let unpacked = unpckhps(vector, shufps(sum, sum, 0x00));
    store4(e, this.addr(), shufps(vector, unpacked, 0xc4));
}

// Translated from 00c74c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Normalise with a non-negative fourth lane (no engine-map name; `RET 4`: `this` is the
/// quaternion, the stack word the output): the four floats scaled to unit length with the refined
/// reciprocal root (`refined_normal`), all negated when the fourth lane of the input is below
/// zero, stored at the output.
pub fn fn_00c74c90(e: &mut Engine, this: Ptr, out: Ptr) {
    let vector = load4(e, this.addr());
    let scaled = refined_normal(e, vector);
    let result = if vector[3] < 0.0 {
        xorps(scaled, load4(e, SIGN_LANES))
    } else {
        scaled
    };
    store4(e, out.addr(), result);
}

// Translated from 00c75580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall with one byte stack word `enable` (no engine-map name). The byte at `+0xb3` becomes
/// 1 when the byte at `+0x43` is 0, the byte at `+0xb1` is set and `enable` is non-zero, 0 when
/// `+0x43` is 0 otherwise, and stays as it is (the byte written back unchanged) when `+0x43`
/// is set. When `enable` is zero the two
/// quaternions at `+0x120` and `+0x170` are set to the identity vector at `010c71b0` and the
/// bytes at `+0x180` and `+0x130` to 1.
pub fn fn_00c75580(e: &mut Engine, this: Ptr, enable: u8) {
    let base = this.addr();
    let flag = if e.mem.u8(base + 0x43) == 0 {
        u8::from(e.mem.u8(base + 0xb1) != 0 && enable != 0)
    } else {
        e.mem.u8(base + 0xb3)
    };
    e.mem.set_u8(base + 0xb3, flag);
    if enable == 0 {
        let identity = load4(e, 0x010c_71b0);
        store4(e, base + 0x120, identity);
        store4(e, base + 0x170, load4(e, 0x010c_71b0));
        e.mem.set_u8(base + 0x180, 1);
        e.mem.set_u8(base + 0x130, 1);
    }
}

// Translated from 00c755e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c755e0 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// Turns one bone of the controller towards a target quaternion (no engine-map name; `RET 8`:
/// thiscall with `index` and the target `target`, four floats). The bone record is `this +
/// index * 0x50`, its current quaternion at `+0x120`. When all of the three lowest lanes of
/// `target - current` are within `0.001` nothing happens and 0.0 is answered. Otherwise the
/// turn `target * current^-1` is made (`00c74ac0`), its angle taken (`00c74c20`) and limited to
/// the soft or hard limit in degrees (`01267d24` or `01267d30`, chosen by the byte at `+0xb2`,
/// converted to radians) by rebuilding it with that angle about its normalised axis
/// (`00c74c90`, `refined_normal`, `setAxisAngle`). The new current quaternion is the turn times
/// the old one (`setMul`), normalised (`005611c0`), stored at `+0x120` and in the pose transform
/// the bone's object (the word at `+0xf0` of the record) gives for the signed 16-bit bone index
/// at `+0xf4` of the record (`00cda8e0`, at `+0x10`). Answers the angle used.
pub fn fn_00c755e0(e: &mut Engine, this: Ptr, index: u32, target: Ptr) -> f32 {
    let this = this.addr();
    let entry = this.wrapping_add(index.wrapping_mul(5) << 4);
    let current_at = entry.wrapping_add(0x120);
    let difference = subps(load4(e, target.addr()), load4(e, current_at));
    let absolute = andps(difference, load4(e, QUATERNION_ABS_MASK));
    let tolerance = e.mem.f32(QUATERNION_TOLERANCE);
    if (0..3).all(|lane| absolute[lane] <= tolerance) {
        return 0.0;
    }
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x44`, three pushes): 0x50 bytes from a
    // 16-byte boundary; the turn at +0x30 (00c75650) and the axis / result at +0x20 (00c7569c).
    e.with_stack(0x60, |e, raw| {
        let frame = (raw.addr() + 0xf) & !0xf;
        turn_bone_body(e, this, entry, target, frame + 0x30, frame + 0x20)
    })
}

/// [`fn_00c755e0`] past its early exit, in its frame: `turn` and `axis` are its two locals.
fn turn_bone_body(e: &mut Engine, this: u32, entry: u32, target: Ptr, turn: u32, axis: u32) -> f32 {
    let current_at = entry.wrapping_add(0x120);
    fn_00c74ac0(e, Ptr::new(turn), target, Ptr::new(current_at));
    let mut angle = x87_store32(f64::from(fn_00c74c20(e, Ptr::new(turn))));
    let limit_at = if e.mem.u8(this + 0xb2) == 0 {
        ANGLE_LIMIT_SOFT
    } else {
        ANGLE_LIMIT_HARD
    };
    let limit_degrees = x87_store32(x87_load32(e.mem.u32(limit_at)));
    let radians: f64 = e.global(DEGREES_TO_RADIANS);
    let limit = x87_mul(x87_load32(limit_degrees), radians);
    if x87_load32(angle) > limit {
        angle = x87_store32(limit);
        fn_00c74c90(e, Ptr::new(turn), Ptr::new(axis));
        let normalised = refined_normal(e, load4(e, axis));
        store4(e, axis, normalised);
        e.call(SET_AXIS_ANGLE, &args![turn, axis, angle]);
    }
    e.call(QUATERNION_SET_MUL, &args![axis, turn, current_at]);
    e.call(NORMALISE_VECTOR, &args![axis]);
    let bone = i32::from(e.mem.u16(entry + 0xf4) as i16) as u32;
    let object = e.mem.u32(entry.wrapping_add(0xf0));
    let transform = e.call(POSE_BONE_TRANSFORM, &args![object, bone]).u32();
    let result = load4(e, axis);
    store4(e, transform + 0x10, result);
    store4(e, current_at, result);
    f32::from_bits(angle)
}

/// `hkVector4` helpers of the target adjustment (`hkvector4.rs`): `00c7f840` (the inverse of a
/// transform, `this` = the result, `(transform, vector)`) and `00c7f7a0` (`this` = the result,
/// `(transform, vector)`).
const TRANSFORM_POINT_INVERSE: u32 = 0x00c7_f840;
/// The floats of the target adjustment: the clamp distance `011b05a8`, and the two offsets
/// `0101712c` and `0102caf8` the second case adds to the first coordinate.
const TARGET_CLAMP_DISTANCE: u32 = 0x011b_05a8;
const TARGET_OFFSET_POSITIVE: u32 = 0x0101_712c;
const TARGET_OFFSET_NEGATIVE: u32 = 0x0102_caf8;

/// The model-space transform of the controller's bone (the signed 16-bit index at `+0x144`) in the
/// pose at `+0x88`: the entry of the pose's array (stride 0x30, array pointer at +0x10) when the
/// flag bit 1 of the bone's flags (array at `+0x1c`) is clear, otherwise what
/// `calculateBoneModelSpace` (`00cda180`) makes of it.
fn target_bone_transform(e: &mut Engine, base: u32) -> u32 {
    let bone = i32::from(e.mem.u16(base + 0x144) as i16) as u32;
    let pose = e.mem.u32(base + 0x88);
    let flags = e.mem.u32(pose + 0x1c);
    if e.mem.u8(flags.wrapping_add(bone.wrapping_mul(4))) & 2 == 0 {
        let transforms = e.mem.u32(pose + 0x10);
        bone.wrapping_mul(3)
            .wrapping_shl(4)
            .wrapping_add(transforms)
    } else {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, bone]).u32()
    }
}

/// A zero vector plus the vector the controller's object gives for bone 1 (`00cda8e0`), when the
/// byte at `+0x190` is set (`ADDPS` on a zero register, so a `-0.0` lane comes out `+0.0`).
fn target_bone_offset(e: &mut Engine, base: u32) -> Lanes {
    let zero = [0.0f32; 4];
    if e.mem.u8(base + 0x190) == 0 {
        return zero;
    }
    let object = e.mem.u32(base + 0xf0);
    let vector = e.call(POSE_BONE_TRANSFORM, &args![object, 1u32]).u32();
    addps(zero, load4(e, vector))
}

// Translated from 00c78160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c78160 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// `bhkRagdollController::AdjustCurrentTarget` (Xbox PDB), plain thiscall. With `T1` the
/// transform at `this + 0x10` and `target` the vector at `this + 0xd0`, `P = T1^-1 * target`
/// (`00c7f840`) is normalised (`refined_normal`) and the byte at `+0x1a4` set to whether its second
/// coordinate is below zero. With the byte at `+0xb2` set the target becomes `T1 * (T0 * offset)
/// + 3 * R2`, with `T0` the bone transform (`target_bone_transform`), `offset` the bone 1
/// vector or zero (`target_bone_offset`) and `R2` the vector at `+0x150` rotated by the bone's
/// quaternion (`+0x10` of its transform) and then by the quaternion at `+0x20`. With the byte
/// clear and the flag set it becomes `T1 * ((T0 * offset) + (L, 0, 0, 0))`, with `L` the float at
/// `0102caf8` when `P`'s first coordinate is below zero and the float at `0101712c` otherwise; with
/// both clear the target stays. Finally the target is brought within the distance of the float at
/// `011b05a8` of the point `T1 * (T0 * offset)`: when farther, it becomes that point plus that
/// distance along the unit direction to the target.
pub fn bhk_ragdoll_controller_adjust_current_target(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x74`, three pushes): 0x80 bytes from a
    // 16-byte boundary; the first inverse point at +0x60 (00c78184), the second at +0x50
    // (00c78430) and the offset carried through the transforms at +0x40 (00c78407).
    e.with_stack(0x90, |e, raw| {
        let frame = (raw.addr() + 0xf) & !0xf;
        adjust_current_target_body(e, base, frame + 0x60, frame + 0x50, frame + 0x40);
    });
}

/// [`bhk_ragdoll_controller_adjust_current_target`] in its frame: `point` and `second_point`
/// are the outputs of the two inverse transforms, `offset_at` the forward transforms' vector.
fn adjust_current_target_body(
    e: &mut Engine,
    base: u32,
    point: u32,
    second_point: u32,
    offset_at: u32,
) {
    let (transform, target) = (base + 0x10, base + 0xd0);
    e.call(TRANSFORM_POINT_INVERSE, &args![point, transform, target]);
    let normalised = refined_normal(e, load4(e, point));
    store4(e, point, normalised);
    let along_y = dot_xyz(mulps([0.0, 1.0, 0.0, 0.0], normalised));
    let below = along_y < 0.0;
    e.mem.set_u8(base + 0x1a4, below as u8);
    let half = load4(e, NEG_HALF_LANES);
    let hard = e.mem.u8(base + 0xb2) != 0;
    if hard {
        let bone_transform = target_bone_transform(e, base);
        let bone_quaternion = load4(e, bone_transform + 0x10);
        let first = rotate_lanes(load4(e, base + 0x150), bone_quaternion, half, true);
        let second = rotate_lanes(first, load4(e, base + 0x20), half, true);
        let three = e.mem.f32(ROOT_THREE);
        let scaled = mulps([three; 4], second);
        let offset = target_bone_offset(e, base);
        store4(e, offset_at, offset);
        let bone_transform = target_bone_transform(e, base);
        e.call(
            TRANSFORM_POINT,
            &args![offset_at, bone_transform, offset_at],
        );
        e.call(TRANSFORM_POINT, &args![offset_at, transform, offset_at]);
        let moved = addps(load4(e, offset_at), scaled);
        store4(e, target, moved);
    } else if below {
        e.call(
            TRANSFORM_POINT_INVERSE,
            &args![second_point, transform, target],
        );
        let positive = e.mem.u32(TARGET_OFFSET_POSITIVE);
        let negative = e.mem.u32(TARGET_OFFSET_NEGATIVE);
        // `FLDZ` / `FCOMP`: only an ordered "below zero" first coordinate takes the second float.
        let adjustment = if 0.0 > x87_m32(e.mem.u32(second_point)) {
            x87_quiet_bits(negative)
        } else {
            x87_quiet_bits(positive)
        };
        let offset = target_bone_offset(e, base);
        store4(e, offset_at, offset);
        let bone_transform = target_bone_transform(e, base);
        e.call(
            TRANSFORM_POINT,
            &args![offset_at, bone_transform, offset_at],
        );
        let first = x87_store32(x87_add(
            x87_load32(e.mem.u32(offset_at)),
            x87_m32(adjustment),
        ));
        e.mem.set_u32(offset_at, first);
        e.call(TRANSFORM_POINT, &args![offset_at, transform, offset_at]);
        let moved = load4(e, offset_at);
        store4(e, target, moved);
    }
    // The point the target is held near: the bone offset carried through both transforms.
    let offset = target_bone_offset(e, base);
    store4(e, offset_at, offset);
    let bone_transform = target_bone_transform(e, base);
    e.call(
        TRANSFORM_POINT,
        &args![offset_at, bone_transform, offset_at],
    );
    e.call(TRANSFORM_POINT, &args![offset_at, transform, offset_at]);
    let anchor = load4(e, offset_at);
    let direction = subps(load4(e, target), anchor);
    let length = dot_xyz(mulps(direction, direction)).sqrt();
    let limit = x87_m32(e.mem.u32(TARGET_CLAMP_DISTANCE));
    // `FCOMPP`: only an ordered "limit below length" clamps.
    if limit < x87_load32(length.to_bits()) {
        let unit = refined_normal(e, direction);
        let clamp = e.mem.f32(TARGET_CLAMP_DISTANCE);
        store4(e, target, addps(mulps(unit, [clamp; 4]), anchor));
    }
}

/// The model-space transform of `bone` in `pose` (array at `+0x10`, stride 0x30; flags at
/// `+0x1c`): the array entry when bit 1 of the bone's flags is clear, otherwise what
/// `calculateBoneModelSpace` (`00cda180`) makes of it.
fn pose_bone_model_transform(e: &mut Engine, pose: u32, bone: u32) -> u32 {
    let flags = e.mem.u32(pose + 0x1c);
    if e.mem.u8(flags.wrapping_add(bone.wrapping_mul(4))) & 2 == 0 {
        let transforms = e.mem.u32(pose + 0x10);
        bone.wrapping_mul(0x30).wrapping_add(transforms)
    } else {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![pose, bone]).u32()
    }
}

// Translated from 00c79340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c79340 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// `bhkRagdollController::InitBoneParams` (Xbox PDB), thiscall `(pose, name, index)`, `RET 0xc`.
/// The bone named `name` is looked up in the skeleton at `[this + 0x2a4] + 0xc` (`00cdd390`, with
/// 0) and its 16-bit index stored at `+0xf4` of the record `this + index * 0x50`; the index of its
/// parent (the skeleton's parent array, word per bone) goes to `+0xf6`. When either is `0xffff`
/// nothing else happens but the byte at `+0xb2` being cleared. Otherwise the 0x34 bytes at
/// `[this + 0x58] + 0x68` are multiplied by the matrix at `this + 0x60` (`0043f8d0`), the
/// product's nine words replace the first nine of the copy, its last float becomes 1.0 and
/// `NI2HK_ov2` converts it (into a scratch result nothing reads). The vector `(0, 1, 0, 0)` (the
/// floats `01011d78` and `01012068`) is then turned by the quaternion at `+0x10` of the model
/// transform of the parent bone (`pose_bone_model_transform`) into `+0x110` of the record, and
/// by that of the bone itself into `+0x100`, which is normalised (`refined_normal`). For bone
/// record 1 the two vectors at `+0x10` and `+0x20` of the object at `[[this + 0x2a4] + 0x1c]`
/// go to `this + 0x160` and `this + 0x150` when that object exists. The word at `+0x88` goes to
/// the first word of the record's 0x50 block at `this + (index * 5 + 0xf) * 0x10`. Answers (in
/// `AL`, which the decompiler drops) whether both indices were found.
pub fn bhk_ragdoll_controller_init_bone_params(
    e: &mut Engine,
    this: Ptr,
    pose: u32,
    name: u32,
    index: u32,
) -> bool {
    let base = this.addr();
    let owner = e.mem.u32(base + 0x2a4);
    let skeleton = e.mem.u32(owner + 0xc);
    let found = e
        .call(FIND_BONE_WITH_NAME, &args![skeleton, name, 0u32])
        .u32() as u16;
    let entry = index.wrapping_mul(0x50).wrapping_add(base);
    e.mem.set_u16(entry + 0xf4, found);
    let mut valid = true;
    if found != 0xffff {
        let owner = e.mem.u32(base + 0x2a4);
        let skeleton = e.mem.u32(owner + 0xc);
        let parents = e.mem.u32(skeleton + 0xc);
        let signed = i32::from(found as i16) as u32;
        let parent = e.mem.u16(parents.wrapping_add(signed.wrapping_mul(2)));
        e.mem.set_u16(entry + 0xf6, parent);
        if parent == 0xffff {
            valid = false;
        }
    } else {
        valid = false;
    }
    e.mem.set_u8(base + 0xb2, 0);
    if !valid {
        return false;
    }
    let world = e.mem.u32(base + 0x58);
    e.with_stack(0x34, |e, record| {
        let record = record.addr();
        for word in 0..13 {
            let value = e.mem.u32(world + 0x68 + 4 * word);
            e.mem.set_u32(record + 4 * word, value);
        }
        e.with_stack(0x24, |e, result| {
            let product = e
                .call(MATRIX_MULTIPLY, &args![base + 0x60, result, record])
                .u32();
            for word in 0..9 {
                let value = e.mem.u32(product + 4 * word);
                e.mem.set_u32(record + 4 * word, value);
            }
        });
        e.mem.set_f32(record + 0x30, 1.0);
        e.with_stack(0x3c, |e, scratch| {
            ni2hk_ov2(e, scratch, Ptr::new(record));
        });
    });
    let fill = e.mem.f32(VECTOR_FILL);
    let one = e.mem.f32(VECTOR_THIRD);
    let vector = [fill, one, fill, fill];
    let bone = i32::from(e.mem.u16(entry + 0xf4) as i16) as u32;
    let bone_transform = pose_bone_model_transform(e, pose, bone);
    let parent = i32::from(e.mem.u16(entry + 0xf6) as i16) as u32;
    let parent_transform = pose_bone_model_transform(e, pose, parent);
    let half = load4(e, NEG_HALF_LANES);
    let parent_quaternion = load4(e, parent_transform + 0x10);
    let turned = rotate_lanes(vector, parent_quaternion, half, false);
    store4(e, entry + 0x110, turned);
    let bone_quaternion = load4(e, bone_transform + 0x10);
    let turned = rotate_lanes(vector, bone_quaternion, half, false);
    store4(e, entry + 0x100, turned);
    let stored = load4(e, entry + 0x100);
    let normalised = refined_normal(e, stored);
    store4(e, entry + 0x100, normalised);
    let owner = e.mem.u32(base + 0x2a4);
    let extra = e.mem.u32(owner + 0x1c);
    if extra != 0 && index == 1 {
        copy16(e, extra + 0x10, base + 0x160);
        let owner = e.mem.u32(base + 0x2a4);
        let extra = e.mem.u32(owner + 0x1c);
        copy16(e, extra + 0x20, base + 0x150);
    }
    let pose_word = e.mem.u32(base + 0x88);
    let block = index.wrapping_mul(5).wrapping_add(0xf) << 4;
    e.mem.set_u32(block.wrapping_add(base), pose_word);
    true
}

/// `NiTransform::MakeIdentity` (`00a86bf0`, thiscall on the transform) and the two floats
/// `01050c98` and `01050c48` the look-at skeleton puts into the translation of its second
/// reference pose.
const NI_TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
const LOOK_IK_POSE_X: u32 = 0x0105_0c98;
const LOOK_IK_POSE_Y: u32 = 0x0105_0c48;

/// The skeleton `InitLookIK` builds for the controller: the share object at
/// `[this + 0x2a4] + 0x1c` (a 0x30-byte block from `00c74ea0` whose first word is cleared and
/// then holds the skeleton) is created, with an `hkaSkeleton` of 0x48 bytes (five empty arrays)
/// given two bones (the 8-byte names, the 2-byte parents and the 0x30-byte reference poses).
/// The parents become 0xffff and 0, the first reference pose the identity and the second one a
/// translation of (`01050c98`, `01050c48`, 0). The two vectors at `+0x160` and `+0x150` of the
/// controller go to `+0x10` and `+0x20` of the share object.
fn build_look_ik_skeleton(e: &mut Engine, base: u32) {
    let cell = fn_00c74ea0(e, 0x30);
    let cell = if cell != 0 {
        e.mem.set_u32(cell, 0);
        cell
    } else {
        0
    };
    let mappers = e.mem.u32(base + 0x2a4);
    e.mem.set_u32(mappers + 0x1c, cell);
    let skeleton = allocator_block(e, 0x48);
    e.mem.set_u16(skeleton + 4, 0x48);
    e.mem.set_u16(skeleton + 6, 1);
    e.mem.set_u32(skeleton, HKA_SKELETON_VTABLE);
    e.call(HK_STRING_CONSTRUCT, &args![skeleton + 8]);
    for array in 0..5 {
        let at = skeleton + 0xc + 0xc * array;
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, 0x8000_0000);
    }
    let mappers = e.mem.u32(base + 0x2a4);
    let cell = e.mem.u32(mappers + 0x1c);
    e.mem.set_u32(cell, skeleton);
    let skeleton = e.mem.u32(e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c));
    let bones = skeleton + 0x18;
    grow_array(e, bones, 2, 8);
    let size = e.mem.u32(bones + 4);
    let data = e.mem.u32(bones);
    let surplus = size.wrapping_sub(2) as i32;
    if surplus > 0 {
        for index in 0..surplus as u32 {
            e.call(HK_STRING_DESTRUCT, &args![data + 0x10 + 8 * index]);
        }
    }
    let size = e.mem.u32(bones + 4);
    let data = e.mem.u32(bones);
    let missing = 2u32.wrapping_sub(size) as i32;
    if missing > 0 {
        for index in 0..missing as u32 {
            let bone = data + 8 * size + 8 * index;
            if bone != 0 {
                e.call(HK_STRING_CONSTRUCT, &args![bone]);
                e.mem.set_u8(bone + 4, 0);
            }
        }
    }
    e.mem.set_u32(bones + 4, 2);
    let skeleton = e.mem.u32(e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c));
    grow_array(e, skeleton + 0xc, 2, 2);
    e.mem.set_u32(skeleton + 0x10, 2);
    let skeleton = e.mem.u32(e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c));
    grow_array(e, skeleton + 0x24, 2, 0x30);
    e.mem.set_u32(skeleton + 0x28, 2);
    let skeleton = e.mem.u32(e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c));
    let parents = e.mem.u32(skeleton + 0xc);
    e.mem.set_u16(parents, 0xffff);
    let skeleton = e.mem.u32(e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c));
    let parents = e.mem.u32(skeleton + 0xc);
    e.mem.set_u16(parents + 2, 0);
    e.with_stack(0x34, |e, transform| {
        let transform = transform.addr();
        e.call(NI_TRANSFORM_MAKE_IDENTITY, &args![transform]);
        let skeleton = e.mem.u32(e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c));
        let poses = e.mem.u32(skeleton + 0x24);
        ni2hk_ov2(e, Ptr::new(poses), Ptr::new(transform));
        copy_quiet(e, LOOK_IK_POSE_X, transform + 0x24);
        copy_quiet(e, LOOK_IK_POSE_Y, transform + 0x28);
        e.mem.set_u32(transform + 0x2c, 0);
        let skeleton = e.mem.u32(e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c));
        let poses = e.mem.u32(skeleton + 0x24);
        ni2hk_ov2(e, Ptr::new(poses + 0x30), Ptr::new(transform));
    });
    let cell = e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c);
    copy16(e, base + 0x160, cell + 0x10);
    let cell = e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c);
    copy16(e, base + 0x150, cell + 0x20);
}

/// The pose object `InitLookIK` builds on the skeleton `[[this + 0x2a4] + 0x1c]` points to: the
/// same 0x38-byte object `build_pose` makes, sized to that skeleton (answers 0 without a block).
fn build_look_ik_pose(e: &mut Engine, base: u32) -> u32 {
    let pose = allocator_block(e, 0x38);
    if pose == 0 {
        return 0;
    }
    let cell = e.mem.u32(e.mem.u32(base + 0x2a4) + 0x1c);
    let skeleton = e.mem.u32(cell);
    e.mem.set_u32(pose, skeleton);
    e.mem.set_u32(pose + 4, 0);
    e.mem.set_u32(pose + 8, 0);
    e.mem.set_u32(pose + 0xc, 0x8000_0000);
    e.mem.set_u32(pose + 0x10, 0);
    e.mem.set_u32(pose + 0x14, 0);
    e.mem.set_u32(pose + 0x18, 0x8000_0000);
    e.mem.set_u32(pose + 0x1c, 0);
    e.mem.set_u32(pose + 0x20, 0);
    e.mem.set_u32(pose + 0x24, 0x8000_0000);
    e.mem.set_u8(pose + 0x28, 0);
    e.mem.set_u8(pose + 0x29, 0);
    e.mem.set_u32(pose + 0x2c, 0);
    e.mem.set_u32(pose + 0x30, 0);
    e.mem.set_u32(pose + 0x34, 0x8000_0000);
    let count = e.mem.u32(e.mem.u32(pose) + 0x1c);
    grow_array(e, pose + 0x10, count, 0x30);
    e.mem.set_u32(pose + 0x14, count);
    grow_array(e, pose + 4, count, 0x30);
    e.mem.set_u32(pose + 8, count);
    grow_array(e, pose + 0x1c, count, 4);
    e.mem.set_u32(pose + 0x20, count);
    let rounded = e.mem.u32(e.mem.u32(pose) + 0x34).wrapping_add(3) & 0xffff_fffc;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(pose + 0x34) & 0x3fff_ffff;
    if (capacity as i32) < (rounded as i32) {
        e.call(ARRAY_RESERVE, &args![allocator, pose + 0x2c, rounded, 4u32]);
    }
    let slots = e.mem.u32(e.mem.u32(pose) + 0x34);
    grow_array(e, pose + 0x2c, slots, 4);
    let size = e.mem.u32(pose + 0x30);
    let data = e.mem.u32(pose + 0x2c);
    let missing = slots.wrapping_sub(size) as i32;
    if missing > 0 {
        for index in 0..missing as u32 {
            e.mem.set_u32(data + 4 * size + 4 * index, 0);
        }
    }
    e.mem.set_u32(pose + 0x30, slots);
    pose
}

// Translated from 00c7de60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7de60 - the FS:[0] exception-unwinding frame is not modeled (the
// saved cookie word is pushed but never checked).
/// `bhkRagdollController::InitLookIK` (Xbox PDB), thiscall `(name)`, `RET 4`. Without the byte
/// at `+0x42` nothing happens. `InitBoneParams` (`00c79340`) runs for bone record 1 on the pose
/// at `+0x88`; when it answers true the dword `+0x194` is set to 1 and, with the byte at `+0x190`
/// set, the look-at objects are made: the skeleton and its share object when
/// `[[this + 0x2a4] + 0x1c]` is still empty (`build_look_ik_skeleton`), then a pose object
/// (`build_look_ik_pose`) kept at `+0x54`, reset (`setToReferencePose`) and synced
/// (`syncModelSpace`). The record at `+0xf0` gets that pose, the bone index 1 (`+0xf4`), the
/// parent index 0 (`+0xf6`) and the two vectors `(0, 1, 0, 0)` (`+0x100` and `+0x110`, from the
/// floats `01011d78` and `01012068`); `+0x194` is cleared again. Finally the byte at `+0xb0`
/// takes the answer of `InitBoneParams`.
pub fn bhk_ragdoll_controller_init_look_ik(e: &mut Engine, this: Ptr, name: u32) {
    let base = this.addr();
    if e.mem.u8(base + 0x42) == 0 {
        return;
    }
    let pose = e.mem.u32(base + 0x88);
    let found = e.call(0x00c7_9340, &args![base, pose, name, 1u32]).u32() as u8;
    if found != 0 {
        e.mem.set_u32(base + 0x194, 1);
        if e.mem.u8(base + 0x190) != 0 {
            let mappers = e.mem.u32(base + 0x2a4);
            if e.mem.u32(mappers + 0x1c) == 0 {
                build_look_ik_skeleton(e, base);
            }
            let pose = build_look_ik_pose(e, base);
            e.mem.set_u32(base + 0x54, pose);
            e.call(SET_TO_REFERENCE_POSE, &args![pose]);
            let pose = e.mem.u32(base + 0x54);
            e.call(SYNC_MODEL_SPACE, &args![pose]);
            let fill = e.mem.u32(VECTOR_FILL);
            let one = e.mem.u32(VECTOR_THIRD);
            e.mem.set_u16(base + 0xf4, 1);
            let pose = e.mem.u32(base + 0x54);
            for offset in [0x100, 0x110] {
                for (lane, bits) in [fill, one, fill, fill].into_iter().enumerate() {
                    e.mem.set_u32(base + offset + 4 * lane as u32, bits);
                }
            }
            e.mem.set_u16(base + 0xf6, 0);
            e.mem.set_u32(base + 0xf0, pose);
            e.mem.set_u32(base + 0x194, 0);
        }
    }
    e.mem.set_u8(base + 0xb0, found);
}

/// What the look-at update calls and reads. Functions: `hkaPose::accessBoneModelSpace` (Ghidra
/// name, `00cdad80`, thiscall on the pose, `(bone, flag)`), the look-at solver `00ce0290`
/// (`hkaLookAtIkSolver::solve`, cdecl `(result, input, target, 1.0, bone info, 0)`, answers the
/// pointer to its result byte) and the quaternion check `00cb24c0` (thiscall on the quaternion,
/// `(result byte)`, answers that pointer). Data: the vector `01267e30` the controller's
/// quaternion at `+0x120` is compared with, the vector `010c71b0` (0, 0, 0, 1) and the float
/// `01267d3c` (a limit in degrees).
const LOOK_AT_ACCESS_MODEL_SPACE: u32 = 0x00cd_ad80;
const LOOK_AT_SOLVE: u32 = 0x00ce_0290;
const QUATERNION_CHECK: u32 = 0x00cb_24c0;
const LOOK_AT_REFERENCE: u32 = 0x0126_7e30;
const UNIT_QUATERNION: u32 = 0x010c_71b0;
const LOOK_AT_ANGLE_LIMIT: u32 = 0x0126_7d3c;

// Translated from 00c78610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c78610 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// The look-at update of one bone record (no Xbox PDB or Ghidra name; thiscall, `RET 8`) with
/// `index` the record (0x50 bytes at `this + index * 0x50`, the pose at the first word of the
/// block `this + (index * 5 + 0xf) * 0x10`) and `transform` the hkQsTransform of the controller.
/// The target point is `transform^-1 * [this + 0xd0]` (`00c7f840`). With the byte `+0x190` set and
/// `index` 1 it is pulled towards the second pose's bone (the bone `+0x144` of the pose at
/// `+0x140`): `p` being the local vector `00cda8e0` gives for bone 1 of the object at `+0xf0`, `u`
/// the unit vector of `(d x p) x d` with `d` the vector at `+0x150`, and `t = (u . p) / |u|`;
/// when `t` is above the distance of the target from that bone, the target is moved to that
/// distance along the unit direction. The second pose's vector becomes the second vector of the
/// solver input. The record's bone `+0xf4` is brought up to date (`synced_bone_output`) and its
/// transform kept; the quaternion at `+0x120` is replaced by the bone's when its three lowest lanes
/// are within the tolerance `01017d00` of the vector `01267e30`. The solver then runs, answering
/// whether it changed the bone. With the byte `+0x130` set the bone's quaternion goes through
/// `00c755e0` and the angle it answers is kept. When the bone's model transform fails the
/// quaternion check, the kept transform is put back, the bone's flag becomes 2 and its descendants
/// are flagged (`flag_descendants`). The byte `+0x130` is set. With the byte `+0xb2` set and the
/// angle below the limit `01267d3c` (degrees) the quaternion is reset to (0, 0, 0, 1), and the bytes
/// `+0xb2` and `+0xb3` cleared, or with `index` 1 and the byte `+0x190` set the byte `+0x131` of
/// the record numbered by the dword `+0x194` cleared and the dword decremented. Answers whether
/// the solver found nothing to do (the decompiler drops the value, a `SETZ AL`). The solver's
/// setup is four vectors: the record's `+0x100`, the second pose's vector (or zero), the record's
/// `+0x110` rotated by the model rotation of the bone `+0xf6` and normalised (the reference
/// direction), and the record's float `+0xf8` (the largest angle); the solver reads all four
/// (00ce0290). Leaving the last two out let the kept bone transform fill them, and the head
/// turned to the floor.
pub fn fn_00c78610(e: &mut Engine, this: Ptr, index: u32, transform: u32) -> bool {
    e.with_stack(0x100, |e, frame| {
        look_at_update(e, this.addr(), index, transform, frame.addr())
    })
}

/// The body of `fn_00c78610` over the stack frame `frame`: the target at `+0`, the solver's
/// setup (0x40 bytes, the exe's `ESP+0x104` block after the prologue) at `+0x10`, the kept bone
/// transform at `+0x50`, its copy after the solver at `+0x80`, the solver's result byte at `+0xb0`
/// and the quaternion check's at `+0xb4`.
fn look_at_update(e: &mut Engine, base: u32, index: u32, transform: u32, frame: u32) -> bool {
    let entry = index.wrapping_mul(0x50).wrapping_add(base);
    let block = index
        .wrapping_mul(5)
        .wrapping_add(0xf)
        .wrapping_shl(4)
        .wrapping_add(base);
    let (target, input, kept, copy) = (frame, frame + 0x10, frame + 0x50, frame + 0x80);
    let (solved_byte, check_byte) = (frame + 0xb0, frame + 0xb4);
    let constants = load_pose_constants(e);
    // 00c7863b..00c78649: the setup's largest angle (`+0x30`) is the record's float `+0xf8`
    // through the x87 (`FLD`/`FSTP`).
    let angle = x87_store32(x87_load32(e.mem.u32(entry + 0xf8)));
    e.mem.set_u32(input + 0x30, angle);
    store4(e, input + 0x10, [0.0; 4]);
    copy16(e, entry + 0x100, input);
    let pose = e.mem.u32(block);
    let parent = i32::from(e.mem.u16(entry + 0xf6) as i16) as u32;
    let parent_model = pose_bone_model_transform(e, pose, parent);
    // 00c786a3..00c787bd: the setup's reference direction (`+0x20`) is the record's vector
    // `+0x110` rotated by that bone's model rotation and scaled to unit length.
    let rotated = rotate_lanes(
        load4(e, entry + 0x110),
        load4(e, parent_model + 0x10),
        constants.half,
        true,
    );
    let reference_direction = refined_normal(e, rotated);
    store4(e, input + 0x20, reference_direction);
    e.call(
        TRANSFORM_POINT_INVERSE,
        &args![target, transform, base + 0xd0],
    );
    if e.mem.u8(base + 0x190) != 0 && index == 1 {
        let object = e.mem.u32(base + 0xf0);
        let vector = e.call(POSE_BONE_TRANSFORM, &args![object, 1u32]).u32();
        let p = load4(e, vector);
        let d = load4(e, base + 0x150);
        let cross = subps(
            mulps(shufps(p, p, 0xd2), shufps(d, d, 0xc9)),
            mulps(shufps(p, p, 0xc9), shufps(d, d, 0xd2)),
        );
        let side = refined_normal(e, cross);
        let second = subps(
            mulps(shufps(side, side, 0xc9), shufps(d, d, 0xd2)),
            mulps(shufps(side, side, 0xd2), shufps(d, d, 0xc9)),
        );
        let unit = refined_normal(e, second);
        let squares = mulps(unit, unit);
        let object = e.mem.u32(base + 0xf0);
        let vector = e.call(POSE_BONE_TRANSFORM, &args![object, 1u32]).u32();
        let p = load4(e, vector);
        let along = dot_xyz(mulps(unit, p));
        let reach = along / dot_xyz(squares).sqrt();
        let other_pose = e.mem.u32(base + 0x140);
        let other_bone = i32::from(e.mem.u16(base + 0x144) as i16) as u32;
        let other = pose_bone_model_transform(e, other_pose, other_bone);
        let anchor = load4(e, other);
        let point = load4(e, target);
        let difference = subps(point, anchor);
        let distance = dot_xyz(mulps(difference, difference)).sqrt();
        if reach > distance {
            let direction = refined_normal(e, difference);
            let moved = addps(mulps(direction, [reach; 4]), point);
            store4(e, target, moved);
        }
        let object = e.mem.u32(base + 0xf0);
        let vector = e.call(POSE_BONE_TRANSFORM, &args![object, 1u32]).u32();
        copy16(e, vector, input + 0x10);
    }
    let pose = e.mem.u32(block);
    let bone = i32::from(e.mem.u16(entry + 0xf4) as i16) as u32;
    let output = synced_bone_output(e, pose, bone, &constants);
    let reference = load4(e, LOOK_AT_REFERENCE);
    let current = load4(e, entry + 0x120);
    let difference = andps(subps(current, reference), load4(e, QUATERNION_ABS_MASK));
    let tolerance = e.mem.f32(QUATERNION_TOLERANCE);
    for offset in [0, 0x10, 0x20] {
        copy16(e, output + offset, kept + offset);
    }
    if (0..3).all(|lane| difference[lane] <= tolerance) {
        copy16(e, output + 0x10, entry + 0x120);
    }
    let pose = e.mem.u32(block);
    let bone = i32::from(e.mem.u16(entry + 0xf4) as i16) as u32;
    let bone_info = e
        .call(LOOK_AT_ACCESS_MODEL_SPACE, &args![pose, bone, 1u32])
        .u32();
    let answer = e
        .call(
            LOOK_AT_SOLVE,
            &args![
                solved_byte,
                input,
                target,
                1.0f32.to_bits(),
                bone_info,
                0u32
            ],
        )
        .u32();
    let solved = e.mem.u8(answer) != 0;
    let pose = e.mem.u32(block);
    let bone = i32::from(e.mem.u16(entry + 0xf4) as i16) as u32;
    let output = synced_bone_output(e, pose, bone, &constants);
    for offset in [0, 0x10, 0x20] {
        copy16(e, output + offset, copy + offset);
    }
    let angle = if e.mem.u8(entry + 0x130) == 0 {
        0.0f32
    } else {
        e.call(0x00c7_55e0, &args![base, index, copy + 0x10]).f32()
    };
    let pose = e.mem.u32(block);
    let bone = i32::from(e.mem.u16(entry + 0xf4) as i16) as u32;
    let model = pose_bone_model_transform(e, pose, bone);
    let ok = e
        .call(QUATERNION_CHECK, &args![model + 0x10, check_byte])
        .u32();
    if e.mem.u8(ok) == 0 {
        let pose = e.mem.u32(block);
        flag_descendants(e, pose, bone, &constants);
        let output = e.mem.u32(pose + 4).wrapping_add(bone.wrapping_mul(0x30));
        for offset in [0, 0x10, 0x20] {
            copy16(e, kept + offset, output + offset);
        }
        let flags = e.mem.u32(pose + 0x1c);
        e.mem.set_u32(flags.wrapping_add(bone.wrapping_mul(4)), 2);
        e.mem.set_u8(pose + 0x28, 0);
    }
    e.mem.set_u8(entry + 0x130, 1);
    if e.mem.u8(base + 0xb2) != 0 {
        let radians: f64 = e.global(DEGREES_TO_RADIANS);
        let limit = x87_mul(x87_load32(e.mem.u32(LOOK_AT_ANGLE_LIMIT)), radians);
        // `FCOMPP`: only an ordered "angle below the limit" resets.
        if limit > x87_load32(angle.to_bits()) {
            copy16(e, UNIT_QUATERNION, entry + 0x120);
            if index == 0 || e.mem.u8(base + 0x190) == 0 {
                e.mem.set_u8(base + 0xb2, 0);
                e.mem.set_u8(base + 0xb3, 0);
            } else {
                let count = e.mem.u32(base + 0x194);
                e.mem
                    .set_u8(count.wrapping_mul(0x50).wrapping_add(base) + 0x131, 0);
                e.mem.set_u32(base + 0x194, count.wrapping_sub(1));
            }
        }
    }
    !solved
}

/// The two look-at helpers of this file that `DoLookAtIK` calls, by address (as `look_at_update`
/// calls `00c755e0`): `AdjustCurrentTarget` (`00c78160`) and the bone record update
/// `fn_00c78610`.
const ADJUST_CURRENT_TARGET: u32 = 0x00c7_8160;
const LOOK_AT_UPDATE: u32 = 0x00c7_8610;
/// The scalars `DoLookAtIK` builds its three axes from (`0.0` at `01011d78`, `1.0` at
/// `01012068`, `-1.0` at `01012054`, read with `MOVSS`) and the `double` the arc cosines are
/// subtracted from (`01030f38`, a quarter turn).
const AXIS_ZERO: u32 = 0x0101_1d78;
const AXIS_ONE: u32 = 0x0101_2068;
const AXIS_MINUS_ONE: u32 = 0x0101_2054;
const QUARTER_TURN: u32 = 0x0103_0f38;

/// The angle between `axis` and a unit vector, as the exe takes it: the dot product goes
/// through `FLD`, the CRT `acos` (`00eca1d0`) and an `FSTP`, and is subtracted from the quarter
/// turn with `FSUBR` and rounded to a float again.
fn quarter_turn_minus_arc_cosine(e: &mut Engine, dot: f32) -> u32 {
    let loaded = x87_load32(dot.to_bits());
    let angle = x87_store32(e.call(CRT_ARC_COSINE, &args![loaded]).f64());
    x87_store32(x87_sub(e.mem.f64(QUARTER_TURN), x87_load32(angle)))
}

// Translated from 00c7aa60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7aa60 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// `bhkRagdollController::DoLookAtIK` (Xbox PDB name per the engine map), thiscall
/// `(transform)`, `RET 4`. Runs `AdjustCurrentTarget`; when the byte `+0x181` is set or the
/// byte `+0x190` clear, sets the word `+0x194` to 1 and runs the bone record update for record
/// 1 with `transform`. The model transform of the bone `+0x144` in the pose at `+0x88`
/// (`hkaPose::accessBoneModelSpace`, `00cdad80`, with 1) has its rotation multiplied by the
/// quaternion at `+0xc0` (`00c66320`). The bone gets flag 4; every later bone whose parent is
/// flagged 4 is then handled as `GetSGPoseLocalSpace` does (bones also flagged 1 are composed
/// with their parent, see `compose_with_parent`, and lose flag 1; all of them gain flag 4 and
/// clear the pose's byte `+0x28`), and the later bones flagged 4 become flagged 2. The kept
/// transform is copied to the bone's entry of the array at `pose + 0x10`, the bone's flag word
/// becomes 1 and the pose's byte `+0x29` is cleared. With the byte `+0x190` set the bone's
/// model transform (`target_bone_transform`) is composed with `transform` (translation `t +
/// rotate(bone translation)`, rotation `00c66320(transform rotation, bone rotation)`, scale the
/// product) on the stack and the bone record update runs for record 0 with it; the byte
/// `+0x181` becomes whether it answered true or the word `+0x194` is not zero. The up axis `(0,
/// 1, 0, 0)` is rotated by the rotation of bone 1 of the pose at `+0x54` (its array entry, or
/// `calculateBoneModelSpace` when the bone has flag 2); the quarter turn minus the arc cosine
/// of its dot product with `(0, 0, -1)` goes to `+0x19c`, with `(1, 0, 0)` to `+0x1a0`. When
/// the limit `+0x138` is below the absolute value of the second angle, or the limit `+0x134`
/// below that of the first (ordered compares), the byte `+0x181` is set. Last the pose at
/// `+0x54` is set to its reference pose (`00cda110`).
pub fn bhk_ragdoll_controller_do_look_at_ik(e: &mut Engine, this: Ptr, transform: u32) {
    let base = this.addr();
    e.call(ADJUST_CURRENT_TARGET, &args![base]);
    if e.mem.u8(base + 0x181) != 0 || e.mem.u8(base + 0x190) == 0 {
        e.mem.set_u32(base + 0x194, 1);
        e.call(LOOK_AT_UPDATE, &args![base, 1u32, transform]);
    }
    let pose = e.mem.u32(base + 0x88);
    let bone = i32::from(e.mem.u16(base + 0x144) as i16) as u32;
    let kept = e
        .call(LOOK_AT_ACCESS_MODEL_SPACE, &args![pose, bone, 1u32])
        .u32();
    e.call(
        QUATERNION_SET_MUL,
        &args![kept + 0x10, kept + 0x10, base + 0xc0],
    );
    let root = i32::from(e.mem.u16(base + 0x144) as i16);
    let pose = e.mem.u32(base + 0x88);
    let skeleton = e.mem.u32(pose);
    let count = e.mem.i32(skeleton + 0x1c);
    let flag_of = |e: &Engine, bone: i32| {
        e.mem
            .u32(pose + 0x1c)
            .wrapping_add((bone as u32).wrapping_mul(4))
    };
    let root_flag = flag_of(e, root);
    e.mem.set_u32(root_flag, e.mem.u32(root_flag) | 4);
    let constants = load_pose_constants(e);
    let mut bone = root.wrapping_add(1);
    if bone < count {
        let mut offset = (bone as u32).wrapping_mul(0x30);
        loop {
            let skeleton = e.mem.u32(pose);
            let parents = e.mem.u32(skeleton + 0xc);
            let parent = e
                .mem
                .u16(parents.wrapping_add((bone as u32).wrapping_mul(2)));
            if parent != 0xffff && e.mem.u8(flag_of(e, i32::from(parent as i16))) & 4 != 0 {
                if e.mem.u8(flag_of(e, bone)) & 1 != 0 {
                    let local = e.mem.u32(pose + 0x10);
                    if parent == 0xffff {
                        // Unreachable (the test above already skipped a parent of -1), but in
                        // the exe: the local entry is copied as it is.
                        let output = e.mem.u32(pose + 4).wrapping_add(offset);
                        for part in [0, 0x10, 0x20] {
                            copy16(e, local.wrapping_add(offset) + part, output + part);
                        }
                    } else {
                        compose_with_parent(e, pose, parent, &constants, local, offset);
                    }
                    let flag = flag_of(e, bone);
                    e.mem.set_u32(flag, e.mem.u32(flag) & 0xffff_fffe);
                }
                let flag = flag_of(e, bone);
                e.mem.set_u32(flag, e.mem.u32(flag) | 4);
                e.mem.set_u8(pose + 0x28, 0);
            }
            offset = offset.wrapping_add(0x30);
            bone = bone.wrapping_add(1);
            if bone >= count {
                break;
            }
        }
    }
    let mut later = root.wrapping_add(1);
    while later < count {
        let flag = flag_of(e, later);
        let word = e.mem.u32(flag);
        if word & 4 != 0 {
            e.mem.set_u32(flag, (word & 0xffff_fffb) | 2);
        }
        later = later.wrapping_add(1);
    }
    let entry = (root as u32)
        .wrapping_mul(0x30)
        .wrapping_add(e.mem.u32(pose + 0x10));
    for part in [0, 0x10, 0x20] {
        copy16(e, kept + part, entry + part);
    }
    let flag = flag_of(e, root);
    e.mem.set_u32(flag, 1);
    e.mem.set_u8(pose + 0x29, 0);
    if e.mem.u8(base + 0x190) == 0 {
        return;
    }
    let model = target_bone_transform(e, base);
    let moved = e.with_stack(0x30, |e, composed| {
        let composed = composed.addr();
        let rotated = rotate_lanes(
            load4(e, model),
            load4(e, transform + 0x10),
            constants.half,
            true,
        );
        store4(e, composed, addps(rotated, load4(e, transform)));
        e.call(
            QUATERNION_SET_MUL,
            &args![composed + 0x10, transform + 0x10, model + 0x10],
        );
        let scale = mulps(load4(e, transform + 0x20), load4(e, model + 0x20));
        store4(e, composed + 0x20, scale);
        e.call(LOOK_AT_UPDATE, &args![base, 0u32, composed]).u32() as u8 != 0
    });
    let looking = moved || e.mem.u32(base + 0x194) != 0;
    let (zero, one, minus_one) = (
        f32::from_bits(e.mem.u32(AXIS_ZERO)),
        f32::from_bits(e.mem.u32(AXIS_ONE)),
        f32::from_bits(e.mem.u32(AXIS_MINUS_ONE)),
    );
    e.mem.set_u8(base + 0x181, looking as u8);
    let forward = [one, zero, zero, zero];
    let up = [zero, one, zero, zero];
    let down = [zero, zero, minus_one, zero];
    let other_pose = e.mem.u32(base + 0x54);
    let other_flags = e.mem.u32(other_pose + 0x1c);
    let bone_one = if e.mem.u8(other_flags + 4) & 2 == 0 {
        e.mem.u32(other_pose + 0x10) + 0x30
    } else {
        e.call(CALCULATE_BONE_MODEL_SPACE, &args![other_pose, 1u32])
            .u32()
    };
    let axis = rotate_lanes(up, load4(e, bone_one + 0x10), constants.half, true);
    let first = quarter_turn_minus_arc_cosine(e, dot_xyz(mulps(axis, down)));
    e.mem.set_u32(base + 0x19c, x87_store32(x87_load32(first)));
    let second = quarter_turn_minus_arc_cosine(e, dot_xyz(mulps(axis, forward)));
    e.mem.set_u32(base + 0x1a0, x87_store32(x87_load32(second)));
    // `FABS`, `FSTP`, `FLD` and `FCOMPP` against the limit: only an ordered "limit below the
    // angle" sets the byte.
    let second_size = x87_store32(x87_load32(second).abs());
    if x87_load32(e.mem.u32(base + 0x138)) < x87_load32(second_size) {
        e.mem.set_u8(base + 0x181, 1);
    }
    let first_size = x87_store32(x87_load32(first).abs());
    if x87_load32(e.mem.u32(base + 0x134)) < x87_load32(first_size) {
        e.mem.set_u8(base + 0x181, 1);
    }
    let other_pose = e.mem.u32(base + 0x54);
    e.call(SET_TO_REFERENCE_POSE, &args![other_pose]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c7_9340, bhk_ragdoll_controller_init_bone_params(Ptr, u32, u32, u32) -> bool),
        entry!(0x00c7_de60, bhk_ragdoll_controller_init_look_ik(Ptr, u32)),
        entry!(0x00c7_8610, fn_00c78610(Ptr, u32, u32) -> bool),
        entry!(0x00c7_aa60, bhk_ragdoll_controller_do_look_at_ik(Ptr, u32)),
        entry!(
            0x00c7_8160,
            bhk_ragdoll_controller_adjust_current_target(Ptr)
        ),
        entry!(0x00c7_4a70, fn_00c74a70(f32) -> f32),
        entry!(0x00c7_4ac0, fn_00c74ac0(Ptr, Ptr, Ptr)),
        entry!(0x00c7_4c20, fn_00c74c20(Ptr) -> f32),
        entry!(0x00c7_4c90, fn_00c74c90(Ptr, Ptr)),
        entry!(0x00c7_5580, fn_00c75580(Ptr, u8)),
        entry!(0x00c7_55e0, fn_00c755e0(Ptr, u32, Ptr) -> f32),
        entry!(0x00c7_e970, deleting_destructor_00c7e970(Ptr, u8) -> Ptr),
        entry!(0x00c7a8b0, bhk_ragdoll_share_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a38560, ni_quaternion_normalize(Ptr)),
        entry!(0x00c747b0, fn_00c747b0(Ptr) -> u32),
        entry!(0x00c74910, fn_00c74910(Ptr, u32, u32)),
        entry!(0x00c749f0, fn_00c749f0(u32, u32, u32)),
        entry!(0x00c74b70, fn_00c74b70(Ptr, Ptr, Ptr)),
        entry!(0x00c74d50, fn_00c74d50(Ptr, Ptr) -> Ptr),
        entry!(0x00c74dd0, ni2hk_ov2(Ptr, Ptr) -> Ptr),
        entry!(0x00c74e70, hka_raycast_interface_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00c74ea0, fn_00c74ea0(u32) -> u32),
        entry!(0x00c74ed0, fn_00c74ed0(Ptr) -> Ptr),
        entry!(0x00c74fb0, fn_00c74fb0(Ptr, Ptr) -> bool),
        entry!(0x00c75100, fn_00c75100(Ptr)),
        entry!(0x00c75180, bhk_ragdoll_controller_get_capsule_bone_shape_info(u32) -> u32),
        entry!(0x00c751f0, bhk_ragdoll_controller_do_grab_ik(Ptr, u32)),
        entry!(0x00c75460, bhk_ragdoll_controller_init_grab_ik(Ptr) -> u8),
        entry!(0x00c757b0, bhk_ragdoll_controller_get_eye_offset(Ptr, Ptr) -> bool),
        entry!(0x00c75850, bhk_ragdoll_controller_is_powered_ragdoll(Ptr) -> bool),
        entry!(0x00c75910, fn_00c75910(Ptr)),
        entry!(0x00c759f0, fn_00c759f0(Ptr)),
        entry!(0x00c75ad0, fn_00c75ad0(Ptr, Ptr)),
        entry!(0x00c75b40, fn_00c75b40(Ptr)),
        entry!(
            0x00c75c00,
            bhk_ragdoll_controller_reset_pose(u32, u32, u32, u32)
        ),
        entry!(0x00c75cc0, fn_00c75cc0(Ptr)),
        entry!(0x00c75ce0, fn_00c75ce0(u32, Ptr, u32, u32)),
        entry!(0x00c75d90, fn_00c75d90(Ptr)),
        entry!(0x00c75e00, fn_00c75e00(Ptr)),
        entry!(0x00c75e20, fn_00c75e20(u32, Ptr, u32, u32)),
        entry!(0x00c75ed0, fn_00c75ed0(Ptr)),
        entry!(0x00c75f40, fn_00c75f40(Ptr, u32) -> u32),
        entry!(0x00c75f60, fn_00c75f60(u32, Ptr, u32, u32)),
        entry!(0x00c75f80, fn_00c75f80(Ptr)),
        entry!(0x00c75fa0, fn_00c75fa0(Ptr)),
        entry!(0x00c76010, fn_00c76010(u32, Ptr)),
        entry!(0x00c76030, fn_00c76030(Ptr, u32) -> Ptr),
        entry!(0x00c76070, fn_00c76070(Ptr, u32) -> Ptr),
        entry!(0x00c760b0, fn_00c760b0(Ptr, u32) -> Ptr),
        entry!(0x00c760f0, fn_00c760f0(u32, u32)),
        entry!(0x00c76180, fn_00c76180(Ptr, u32, u32)),
        entry!(0x00c761f0, fn_00c761f0(Ptr, Ptr) -> Ptr),
        entry!(0x00c76220, fn_00c76220(Ptr, u32) -> u32),
        entry!(0x00c76240, bhk_ragdoll_share_data_destruct(Ptr)),
        entry!(0x00c762f0, fn_00c762f0(Ptr) -> u32),
        entry!(0x00c76320, fn_00c76320(Ptr) -> u32),
        entry!(0x00c78090, fn_00c78090(Ptr) -> bool),
        entry!(0x00c79d60, fn_00c79d60(Ptr, u8, u8, u8)),
        entry!(
            0x00c7a230,
            bhk_ragdoll_controller_update_constraints_world(Ptr, u8)
        ),
        entry!(
            0x00c7a2d0,
            bhk_ragdoll_powered_constraint_controller_start_motors(u32)
        ),
        entry!(
            0x00c7a380,
            bhk_ragdoll_powered_constraint_controller_stop_motors(u32)
        ),
        entry!(
            0x00c7a410,
            bhk_ragdoll_powered_constraint_controller_drive_to_pose(u32, u32)
        ),
        entry!(0x00c7a570, ni_t_map_pose_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c7a590, ni_t_map_ragdoll_share_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c7a5b0, ni_t_map_ni_av_object_short_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c7a5d0, bs_simple_array_bone_transf_allocate(Ptr, u32) -> u32),
        entry!(0x00c7a600, bs_simple_array_bone_transf_reallocate(Ptr, u32, u32) -> u32),
        entry!(0x00c7a630, fn_00c7a630(Ptr)),
        entry!(0x00c7a660, fn_00c7a660(Ptr)),
        entry!(0x00c7a690, bs_simple_array_bone_transf_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c7a6e0, bs_simple_array_bhk_skeleton_bone_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c7a730, fn_00c7a730(Ptr) -> u32),
        entry!(0x00c7a790, fn_00c7a790(Ptr, Ptr)),
        entry!(0x00c7a8d0, bhk_ragdoll_controller_enable_ragdoll_anim(Ptr)),
        entry!(0x00c7a990, fn_00c7a990(u32, u32, u32)),
        entry!(0x00c7b0b0, fn_00c7b0b0(Ptr)),
        entry!(0x00c7b0f0, fn_00c7b0f0(Ptr)),
        entry!(0x00c7b140, fn_00c7b140(u32, u32, u32)),
        entry!(
            0x00c7b1f0,
            bhk_ragdoll_controller_init_pose_matching(Ptr, u32, u16, u16, u16)
        ),
        entry!(0x00c7b4f0, fn_00c7b4f0(Ptr, u32, u32, u32)),
        entry!(
            0x00c7b6a0,
            bhk_ragdoll_controller_set_ragdoll_feedback_active(Ptr, u8)
        ),
        entry!(
            0x00c7ba10,
            bhk_ragdoll_controller_enable_ragdoll_feedback(Ptr, u8)
        ),
        entry!(
            0x00c7ba50,
            bhk_ragdoll_controller_do_pose_matching(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00c7bcb0,
            bhk_ragdoll_controller_copy_pose_to_3d_rec(u32, u32, Ptr)
        ),
        entry!(
            0x00c7be20,
            bhk_ragdoll_controller_get_rb_for_bones(Ptr, Ptr)
        ),
        entry!(0x00c7be90, fn_00c7be90() -> u32),
        entry!(0x00c7beb0, fn_00c7beb0(u32, u32)),
        entry!(0x00c7bf10, fn_00c7bf10(Ptr)),
        entry!(0x00c7bf50, fn_00c7bf50(Ptr)),
        entry!(0x00c7bf90, fn_00c7bf90(Ptr)),
        entry!(0x00c7bfd0, fn_00c7bfd0(Ptr) -> Ptr),
        entry!(0x00c7c030, fn_00c7c030(Ptr)),
        entry!(
            0x00c7c150,
            bhk_ragdoll_controller_disable_ragdoll_anim(Ptr, u8)
        ),
        entry!(0x00c7c2b0, fn_00c7c2b0(Ptr)),
        entry!(
            0x00c7d030,
            bhk_ragdoll_controller_update_ragdoll_rigid_bodies(Ptr, u32)
        ),
        entry!(
            0x00c7d3b0,
            bhk_ragdoll_controller_copy_pose_to_3d(u32, u32, Ptr, u32, u32)
        ),
        entry!(0x00c7d480, fn_00c7d480(Ptr)),
        entry!(0x00c7d520, fn_00c7d520(Ptr)),
        entry!(0x00c7d810, bhk_ragdoll_controller_sync_ragdoll_to_3d(Ptr)),
        entry!(0x00c7d900, fn_00c7d900(Ptr)),
        entry!(0x00c7dc40, hka_skeleton_destruct(Ptr)),
        entry!(0x00c7de30, bhk_ragdoll_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c79680,
            bhk_ragdoll_controller_get_sg_pose_local_space(Ptr)
        ),
        entry!(
            0x00c79a50,
            bhk_ragdoll_controller_set_sg_pose_local_space(Ptr)
        ),
        entry!(0x00c7c870, bhk_ragdoll_controller_do_ragdoll_feedback(Ptr)),
        entry!(
            0x00c7cb50,
            bhk_ragdoll_controller_update_debug_lines(Ptr, u32)
        ),
        entry!(0x00c7d630, bhk_ragdoll_controller_do_ragdoll_anim(Ptr, u32)),
        entry!(0x00c79e20, bhk_ragdoll_controller_check_mappers_for_warnings(Ptr) -> bool),
        entry!(0x00c77ac0, fn_00c77ac0(Ptr, u32, u32) -> bool),
        entry!(0x00c7c3a0, bhk_ragdoll_controller_init_foot_ik(Ptr, u32)),
        entry!(
            0x00c77460,
            bhk_ragdoll_controller_cast_ray(Ptr, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00c7e3b0, fn_00c7e3b0(Ptr)),
        entry!(0x00c76350, bhk_ragdoll_controller_do_foot_ik(Ptr, u32)),
        entry!(0x00c7e9a0, bhk_ragdoll_controller_init(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00c7e970_frees_only_when_bit_zero_is_set() {
        check(0x00c7_e970, 0x00c7_dc40);
    }

    #[test]
    fn bhk_ragdoll_share_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c7_6240, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c7a8b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c7a8b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x20]]);
        assert_eq!(calls_to(&e, 0x00c7_6240), vec![vec![this]; 2]);
    }

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A logging double over `address` that answers `eax`.
    fn recorder(e: &mut Engine, log: &CallLog, address: u32, eax: u32) {
        let kept = Rc::clone(log);
        e.register_double(address, move |_, a| {
            kept.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The CRT square root double: `ST0 = sqrt(argument)`, logged.
    fn install_sqrt(e: &mut Engine, log: &CallLog) {
        let kept = Rc::clone(log);
        e.register_double(X87_SQRT, move |_, a| {
            kept.borrow_mut().push((X87_SQRT, a.to_vec()));
            st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt())
        });
    }

    fn put_floats(e: &mut Engine, address: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * index as u32, *value);
        }
    }

    fn floats(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(address + 4 * i)).collect()
    }

    #[test]
    fn ni_quaternion_normalize_scales_by_the_float_reciprocal_of_the_root() {
        let mut e = engine();
        let log = CallLog::default();
        install_sqrt(&mut e, &log);
        let this = e.mem.alloc(0x10);
        put_floats(&mut e, this, &[1.0, 2.0, 3.0, 4.0]);
        e.call(0x00a38560, &args![this]);
        assert_eq!(calls_to(&log, X87_SQRT), vec![args![30.0f64]]);
        let root = 30.0f32.sqrt();
        for (got, want) in floats(&e, this, 4).into_iter().zip([1.0, 2.0, 3.0, 4.0]) {
            assert!((got - want / root).abs() < 1e-6, "{got} {want}");
        }
    }

    #[test]
    fn fn_00c747b0_answers_the_slot_result_minus_four_or_zero() {
        let mut e = engine();
        let log = CallLog::default();
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xb4, 0x0900_00b4);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        recorder(&mut e, &log, 0x0900_00b4, 0x1004);
        assert_eq!(e.call(0x00c747b0, &args![this]).u32(), 0x1000);
        let call = &calls_to(&log, 0x0900_00b4)[0];
        assert_eq!((call[0], call.len()), (this, 2));
        recorder(&mut e, &log, 0x0900_00b4, 0);
        assert_eq!(e.call(0x00c747b0, &args![this]).u32(), 0);
    }

    /// A hash map object whose slots are doubles: 4 hashes to `key & 7`, 8 compares keys, `0x14`
    /// hands out a fresh node, `0xc` stores the key and value in the node. Returns the map and
    /// the log of `(slot, args)`.
    fn hash_map(e: &mut Engine) -> (u32, CallLog) {
        let log = CallLog::default();
        let vtable = e.mem.alloc(0x40);
        for (slot, double) in [
            (4, 0x0900_0004),
            (8, 0x0900_0008),
            (0xc, 0x0900_000c),
            (0x14, 0x0900_0014),
        ] {
            e.mem.set_u32(vtable + slot, double);
        }
        let kept = Rc::clone(&log);
        e.register_double(0x0900_0004, move |_, a| {
            kept.borrow_mut().push((4, a.to_vec()));
            Ret {
                eax: a[1] & 7,
                ..Ret::default()
            }
        });
        let kept = Rc::clone(&log);
        e.register_double(0x0900_0008, move |_, a| {
            kept.borrow_mut().push((8, a.to_vec()));
            Ret {
                eax: u32::from(a[1] == a[2]),
                ..Ret::default()
            }
        });
        let kept = Rc::clone(&log);
        e.register_double(0x0900_0014, move |e, a| {
            kept.borrow_mut().push((0x14, a.to_vec()));
            Ret {
                eax: e.mem.alloc(0x10),
                ..Ret::default()
            }
        });
        let kept = Rc::clone(&log);
        e.register_double(0x0900_000c, move |e, a| {
            kept.borrow_mut().push((0xc, a.to_vec()));
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u16(a[1] + 8, a[3] as u16);
            Ret::default()
        });
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, vtable);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(map + 8, table);
        (map, log)
    }

    #[test]
    fn fn_00c74910_inserts_at_the_front_of_the_bucket_or_updates_a_node() {
        let mut e = engine();
        let (map, log) = hash_map(&mut e);
        e.call(0x00c74910, &args![map, 10u32, 0x1111u32]);
        e.call(0x00c74910, &args![map, 18u32, 0x2222u32]);
        let table = e.mem.u32(map + 8);
        let first = e.mem.u32(table + 8);
        let second = e.mem.u32(first);
        assert_eq!((e.mem.u32(first + 4), e.mem.u16(first + 8)), (18, 0x2222));
        assert_eq!((e.mem.u32(second + 4), e.mem.u16(second + 8)), (10, 0x1111));
        assert_eq!((e.mem.u32(second), e.mem.u32(map + 0xc)), (0, 2));
        log.borrow_mut().clear();
        e.call(0x00c74910, &args![map, 10u32, 0x3333u32]);
        assert_eq!(e.mem.u16(second + 8), 0x3333);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        let slots: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(slots, vec![4, 8, 8]);
        assert_eq!(log.borrow()[1].1, vec![map, 10, 18]);
    }

    /// A `memmove` double that moves the bytes and logs `(destination, source, bytes)`.
    fn install_memmove(e: &mut Engine) -> CallLog {
        let log = CallLog::default();
        let kept = Rc::clone(&log);
        e.register_double(MEMMOVE, move |e, a| {
            kept.borrow_mut().push((MEMMOVE, a.to_vec()));
            let data = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &data);
            Ret::default()
        });
        log
    }

    #[test]
    fn fn_00c749f0_moves_elements_front_to_back_or_back_to_front() {
        let mut e = engine();
        let log = install_memmove(&mut e);
        let block = e.mem.alloc(0x38 * 5);
        for element in 0..4u32 {
            e.mem.set_u32(block + 0x38 * element, 100 + element);
        }
        e.call(0x00c749f0, &args![block, block + 0x38, 3u32]);
        let moved: Vec<u32> = (0..4).map(|i| e.mem.u32(block + 0x38 * i)).collect();
        assert_eq!(moved, vec![101, 102, 103, 103]);
        let sources: Vec<u32> = calls_to(&log, MEMMOVE).iter().map(|c| c[1]).collect();
        assert_eq!(sources, vec![block + 0x38, block + 0x70, block + 0xa8]);
        log.borrow_mut().clear();
        e.call(0x00c749f0, &args![block + 0x38, block, 3u32]);
        let moved: Vec<u32> = (0..4).map(|i| e.mem.u32(block + 0x38 * i)).collect();
        assert_eq!(moved, vec![101, 101, 102, 103]);
        let destinations: Vec<u32> = calls_to(&log, MEMMOVE).iter().map(|c| c[0]).collect();
        assert_eq!(destinations, vec![block + 0xa8, block + 0x70, block + 0x38]);
    }

    #[test]
    fn fn_00c749f0_does_nothing_for_zero_elements_or_equal_addresses() {
        let mut e = engine();
        let log = install_memmove(&mut e);
        let block = e.mem.alloc(0x100);
        e.call(0x00c749f0, &args![block, block + 0x38, 0u32]);
        e.call(0x00c749f0, &args![block, block, 3u32]);
        assert!(log.borrow().is_empty());
    }

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    /// What `foot_rig` built: the controller, the setup block, the foot record, the collision
    /// objects and capsule infos of the three nodes and the log of the cosine arguments.
    struct FootRig {
        this: u32,
        setup: u32,
        record: u32,
        collisions: [u32; 3],
        infos: [u32; 3],
        cosines: Rc<RefCell<Vec<f64>>>,
    }

    /// A node object whose slot `0xc` answers 1 and whose first child is `child` (when given).
    fn foot_node(e: &mut Engine, child: Option<u32>) -> u32 {
        let node = object_with_vtable(e, &[(0xc, 0x0900_100c)]);
        if let Some(child) = child {
            let children = e.mem.alloc(0x10);
            e.mem.set_u32(children, child);
            e.mem.set_u32(node + 0xa0, children);
            e.mem.set_u16(node + 0xa6, 1);
        }
        node
    }

    /// A foot IK setup with `names` bone names: three nodes (top, second, third) found by any
    /// name, their collision objects (named `0xa0`, `0xb0`, `0xc0`), capsule infos, shapes and
    /// the doubles of everything `00c77ac0` calls. The second node's shape has the vector
    /// `(0, 3, 4, 0)` at `+0x20` and the limit angles 0, the third node's the float 0.25 at `+0x50`;
    /// the settings are 1.5 and 0.5, the cosines 0.25 and 0.75.
    fn foot_rig(e: &mut Engine, log: &CallLog, names: u32) -> FootRig {
        const RADIANS: f64 = std::f64::consts::PI / 180.0;
        for (address, value) in [
            (0x0102_f248u32, 180.0 / std::f64::consts::PI),
            (0x0106_6808, 90.0),
            (0x0102_0758, 10.0),
            (0x0102_3128, RADIANS),
            (0x0102_fc70, 20.0),
            (0x0101_1590, 2.0),
        ] {
            e.mem.set_f64(address, value);
        }
        e.mem.set_f32(0x0101_6248, 0.5);
        e.mem.set_f32(0x0101_7718, 3.0);
        recorder(e, log, 0x0900_100c, 1);
        let third = foot_node(e, None);
        let second = foot_node(e, Some(third));
        let top = foot_node(e, Some(second));
        recorder(e, log, GET_OBJECT_BY_NAME, top);
        recorder(e, log, STRING_COPY, 0x7000_0000);
        recorder(e, log, INTERLOCKED_DECREMENT, 0);
        let shape = e.mem.alloc(0x40);
        let capsule = e.mem.alloc(0x40);
        let inner = e.mem.alloc(0x40);
        let holder = object_with_vtable(e, &[(0xb4, 0x0900_10b4)]);
        e.mem.set_u32(shape + 8, capsule);
        e.mem.set_u32(capsule + 0x10, inner);
        e.mem.set_u32(inner + 8, holder);
        recorder(e, log, EMPTY_LOCK, 0);
        let nodes = [top, second, third];
        let collisions = [e.mem.alloc(0x40), e.mem.alloc(0x40), e.mem.alloc(0x40)];
        let infos = [e.mem.alloc(0x40), e.mem.alloc(0x40), e.mem.alloc(0x40)];
        for (index, name) in [0xa0u32, 0xb0, 0xc0].into_iter().enumerate() {
            let named = e.mem.alloc(0x10);
            e.mem.set_u32(named + 8, name);
            e.mem.set_u32(collisions[index] + 8, named);
            e.mem.set_u32(collisions[index] + 0x10, shape);
        }
        e.register_double(FIND_NEXT_COLLISION_OBJECT, move |_, a| Ret {
            eax: nodes
                .iter()
                .position(|&node| node == a[0])
                .map_or(0, |index| collisions[index]),
            ..Ret::default()
        });
        e.register_double(0x0900_10b4, move |e, a| {
            let handle = e.mem.u32(a[1]);
            Ret {
                eax: nodes
                    .iter()
                    .position(|&node| node == handle)
                    .map_or(0, |index| infos[index]),
                ..Ret::default()
            }
        });
        put_floats(e, infos[1] + 0x10, &[3.0]);
        put_floats(e, infos[1] + 0x20, &[1.0]);
        put_floats(e, infos[2] + 4, &[0.5]);
        put_floats(e, infos[2] + 0x10, &[2.0, 0.0, 0.0, 0.0]);
        let shapes = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        put_floats(e, shapes[0] + 0x20, &[0.0, 3.0, 4.0, 0.0]);
        put_floats(e, shapes[1] + 0x50, &[0.25, 0.0, 0.0, 0.0]);
        let sources = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        let mut casts = [0; 2];
        for (index, shape_address) in shapes.into_iter().enumerate() {
            let held = e.mem.alloc(0x10);
            e.mem.set_u32(held + 4, shape_address);
            let body = e.mem.alloc(0x40);
            e.mem.set_u32(body + 0x14, held);
            casts[index] = e.mem.alloc(0x40);
            e.mem.set_u32(casts[index] + 0x10, body);
            e.mem.set_u32(nodes[index + 1] + 0x1c, sources[index]);
        }
        e.register_double(0x0065_3290, move |_, a| Ret {
            eax: sources
                .iter()
                .position(|&source| source == a[0])
                .map_or(0, |index| casts[index]),
            ..Ret::default()
        });
        e.register_double(CAST_TYPED, |_, a| Ret {
            eax: if a[0] == TYPE_FIRST { a[1] } else { 0 },
            ..Ret::default()
        });
        e.register_double(CAPSULE_HOLDER, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let settings = [e.mem.alloc(4), e.mem.alloc(4)];
        e.mem.set_f32(settings[0], 1.5);
        e.mem.set_f32(settings[1], 0.5);
        e.register_double(SETTING_FLOAT_POINTER, move |_, a| Ret {
            eax: settings[usize::from(a[0] != SETTING_FOOT_A)],
            ..Ret::default()
        });
        let cosines = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&cosines);
        e.register_double(CRT_COSINE, move |_, a| {
            let mut seen = kept.borrow_mut();
            seen.push(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32));
            st0([0.25, 0.75][(seen.len() - 1) % 2])
        });
        let this = e.mem.alloc(0x300);
        e.mem.set_u32(this + 0x58, 0x5000);
        e.mem.set_u8(this + 0x42, 1);
        let setup = e.mem.alloc(0x20);
        let name_array = e.mem.alloc(4 * names.max(1));
        for index in 0..names {
            e.mem.set_u32(name_array + 4 * index, 0x8000 + 8 * index);
        }
        e.mem.set_u32(setup + 4, name_array);
        e.mem.set_u32(setup + 8, names);
        FootRig {
            this,
            setup,
            record: e.mem.alloc(0x78),
            collisions,
            infos,
            cosines,
        }
    }

    #[test]
    fn fn_00c77ac0_fills_the_foot_record_from_the_capsule_shapes() {
        const RADIANS: f64 = std::f64::consts::PI / 180.0;
        let mut e = engine();
        let log = CallLog::default();
        let rig = foot_rig(&mut e, &log, 1);
        let call_args = args![rig.this, rig.record, rig.setup];
        assert!(e.call(0x00c7_7ac0, &call_args).bool());
        let record = rig.record;
        assert_eq!(
            *rig.cosines.borrow(),
            vec![
                f64::from((80.0 * RADIANS) as f32),
                f64::from((70.0 * RADIANS) as f32)
            ]
        );
        assert_eq!(e.mem.f32(record + 0x64), -0.25);
        assert_eq!(e.mem.f32(record + 0x68), 0.75);
        assert_eq!(e.mem.f32(record + 0x60), -0.5);
        assert_eq!(e.mem.f32(record + 0x5c), 7.75);
        assert_eq!(e.mem.f32(record + 0x58), 4.25);
        assert_eq!(e.mem.f32(record + 0x54), 2.0);
        let direction = floats(&e, record + 0x10, 4);
        for (got, want) in direction.into_iter().zip([0.0, 0.6, 0.8, 0.0]) {
            assert!((got - want).abs() < 1e-3, "{got} against {want}");
        }
        assert_eq!(floats(&e, record + 0x20, 4), vec![2.0, 0.0, 0.0, 0.0]);
        assert_eq!(calls_to(&log, STRING_COPY), vec![vec![0x8000]]);
        assert_eq!(
            calls_to(&log, INTERLOCKED_DECREMENT),
            vec![vec![0x6fff_fff8]]
        );
        e.mem.set_f32(rig.infos[2] + 0x10, 0.1);
        assert!(e.call(0x00c7_7ac0, &call_args).bool());
        assert_eq!(floats(&e, record + 0x20, 4), vec![0.0; 4]);
    }

    #[test]
    fn fn_00c77ac0_fails_without_the_nodes_or_infos() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = foot_rig(&mut e, &log, 1);
        let call_args = args![rig.this, rig.record, rig.setup];
        e.mem.set_u32(rig.collisions[2] + 0x10, 0);
        assert!(!e.call(0x00c7_7ac0, &call_args).bool());
        recorder(&mut e, &log, GET_OBJECT_BY_NAME, 0);
        assert!(!e.call(0x00c7_7ac0, &call_args).bool());
        assert_eq!(floats(&e, rig.record + 0x58, 3), vec![0.0; 3]);
    }

    /// The doubles `InitFootIK` calls besides those of `00c77ac0`: the mapper holding the
    /// skeleton `0x7e7e` and two mapping records (the second maps bone 5 to 0x77), the record
    /// and element constructors, the array reserve, the allocator and the bone lookup, which
    /// answers `0xffff` after `lookups` calls. Returns the dwords of the foot record at each
    /// element construction and the allocator.
    fn install_foot_ik(e: &mut Engine, log: &CallLog, rig: &FootRig, lookups: u32) -> (Words, u32) {
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0xc, 0x7e7e);
        let mapper = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x80);
        e.mem.set_u16(records, 9);
        e.mem.set_u16(records + 0x40, 5);
        e.mem.set_u16(records + 0x42, 0x77);
        e.mem.set_u32(mapper + 0x18, records);
        e.mem.set_i32(mapper + 0x1c, 2);
        e.mem.set_u32(owner + 0x10, mapper);
        e.mem.set_u32(rig.this + 0x2a4, owner);
        e.mem.set_f64(WEIGHT_OFFSET, 0.25);
        e.mem.set_f32(0x0101_2068, 1.0);
        e.mem.set_f32(0x0126_7d90, 0.1);
        e.register_double(FOOT_RECORD_INIT, |e, a| {
            for lane in 0..3 {
                e.mem.set_u16(a[0] + 4 + 2 * lane, 0xffff);
            }
            Ret::default()
        });
        let cursor = e.mem.alloc(0x10);
        let (_, allocator) = install_router(e, log, cursor);
        let kept = Rc::clone(log);
        e.register_double(0x0900_a004, move |e, a| {
            kept.borrow_mut().push((0x0900_a004, a.to_vec()));
            Ret {
                eax: e.mem.alloc(0x100),
                ..Ret::default()
            }
        });
        let kept = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            kept.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let data = e.mem.alloc(a[2] * 0xd0);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let seen = Words::default();
        let kept_words = Rc::clone(&seen);
        let kept = Rc::clone(log);
        e.register_double(FOOT_ELEMENT_INIT, move |e, a| {
            kept.borrow_mut().push((FOOT_ELEMENT_INIT, a.to_vec()));
            let words = (0..30).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            kept_words.borrow_mut().push(words);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let count = Rc::new(std::cell::Cell::new(0u32));
        let counted = Rc::clone(&count);
        let kept = Rc::clone(log);
        e.register_double(FIND_BONE_WITH_NAME, move |_, a| {
            kept.borrow_mut().push((FIND_BONE_WITH_NAME, a.to_vec()));
            counted.set(counted.get() + 1);
            let bone = match a[1] {
                _ if counted.get() > lookups => 0xffff,
                0xa0 => 3,
                0xb0 => 4,
                _ => 5,
            };
            Ret {
                eax: bone,
                ..Ret::default()
            }
        });
        recorder(e, log, ELEMENT_DESTRUCT, 0);
        (seen, allocator)
    }

    #[test]
    fn init_foot_ik_builds_one_element_per_foot_name() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = foot_rig(&mut e, &log, 2);
        let (seen, allocator) = install_foot_ik(&mut e, &log, &rig, 99);
        e.call(0x00c7_c3a0, &args![rig.this, rig.setup]);
        let this = rig.this;
        assert_eq!((e.mem.u32(this + 0x230), e.mem.u8(this + 0x21f)), (2, 1));
        assert_eq!(e.mem.f32(this + 0x244), 0.75);
        let data = e.mem.u32(this + 0x22c);
        let objects: Vec<u32> = (0..2).map(|index| e.mem.u32(data + 0xd0 * index)).collect();
        assert!(objects.iter().all(|&object| object != 0));
        assert_ne!(objects[0], objects[1]);
        for (index, object) in objects.iter().enumerate() {
            assert_eq!(e.mem.u16(data + 0xd0 * index as u32 + 0xc0), 0x77);
            assert_eq!(e.mem.u16(object + 4), 0xf0);
            assert_eq!(e.mem.f32(data + 0xd0 * index as u32 + 4), 0.0);
        }
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![vec![allocator, this + 0x22c, 2, 0xd0]]
        );
        assert_eq!(calls_to(&log, 0x0900_a004), vec![vec![allocator, 0xf0]; 2]);
        let lookups: Vec<Vec<u32>> = [0xa0, 0xb0, 0xc0, 0xa0, 0xb0, 0xc0]
            .into_iter()
            .map(|name| vec![0x7e7e, name, 0])
            .collect();
        assert_eq!(calls_to(&log, FIND_BONE_WITH_NAME), lookups);
        let weight = 0.1f32.to_bits();
        let third = 1.0f32.to_bits();
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        for words in seen.iter() {
            assert_eq!(words[0], 0x7e7e);
            assert_eq!(words[1], 3 | 4 << 16);
            assert_eq!(words[2], 5);
            assert_eq!(words[12..20], [0, 0, third, 0, 0, 0, third, 0]);
            assert_eq!(words[20], weight);
        }
        assert!(calls_to(&log, ELEMENT_DESTRUCT).is_empty());
    }

    #[test]
    fn init_foot_ik_destroys_the_elements_when_one_foot_fails() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = foot_rig(&mut e, &log, 2);
        install_foot_ik(&mut e, &log, &rig, 3);
        e.call(0x00c7_c3a0, &args![rig.this, rig.setup]);
        let this = rig.this;
        let data = e.mem.u32(this + 0x22c);
        let first = e.mem.u32(data);
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(data + 0xd0), 0);
        assert_eq!(calls_to(&log, ELEMENT_DESTRUCT), vec![vec![first]]);
        assert_eq!((e.mem.u32(this + 0x230), e.mem.u8(this + 0x21f)), (0, 0));
    }

    #[test]
    fn init_foot_ik_stops_without_names_flag_or_record() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = foot_rig(&mut e, &log, 1);
        let record_init = Rc::new(std::cell::Cell::new(0u32));
        let counted = Rc::clone(&record_init);
        e.register_double(FOOT_RECORD_INIT, move |_, _| {
            counted.set(counted.get() + 1);
            Ret::default()
        });
        e.mem.set_u8(rig.this + 0x42, 0);
        e.call(0x00c7_c3a0, &args![rig.this, rig.setup]);
        assert_eq!(record_init.get(), 0);
        e.mem.set_u8(rig.this + 0x42, 1);
        e.mem.set_u32(rig.setup + 8, 0);
        e.call(0x00c7_c3a0, &args![rig.this, rig.setup]);
        assert_eq!(record_init.get(), 0);
        e.mem.set_u32(rig.setup + 8, 1);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(rig.this + 0x2a4, owner);
        recorder(&mut e, &log, GET_OBJECT_BY_NAME, 0);
        e.call(0x00c7_c3a0, &args![rig.this, rig.setup]);
        assert_eq!(record_init.get(), 1);
        assert_eq!(e.mem.u32(rig.this + 0x230), 0);
        assert_eq!(e.mem.u8(rig.this + 0x21f), 0);
    }

    const WORLD_CAST_DOUBLE: u32 = 0x0900_d0c8;

    /// A ray hit as the cast doubles answer it: the normal and the fraction.
    type RayHit = ([f32; 4], f32);

    /// The controller and buffers `castRay` works on.
    struct CastRig {
        this: u32,
        from: u32,
        to: u32,
        fraction: u32,
        normal: u32,
        hit: u32,
        cache: u32,
        pose: u32,
    }

    /// What a cast double was handed: the filter dword at `+0x24`, then the start (`+0`) and end
    /// (`+0x10`) vectors as raw words.
    fn ray_words(e: &Engine, input: u32) -> Vec<u32> {
        let mut words = vec![e.mem.u32(input + 0x24)];
        words.extend((0..4).map(|lane| e.mem.u32(input + 4 * lane)));
        words.extend((0..4).map(|lane| e.mem.u32(input + 0x10 + 4 * lane)));
        words
    }

    /// A controller for `castRay` with the constants the code reads, a ray from (0, 0, 10) to
    /// the origin, the root bone's transform (identity rotation; `setTransformedPos_ov2` answers
    /// the height `reference`), the cache record holding "no hit", and both ray casts as doubles
    /// answering `answer` (the world's slot `0xc8` and `castRay` of the object at `+0x90`). The
    /// flag byte of the root bone is `flags`.
    fn cast_rig(
        e: &mut Engine,
        log: &CallLog,
        reference: f32,
        flags: u8,
        answer: Option<RayHit>,
    ) -> CastRig {
        e.mem.set_f32(NO_HIT_FRACTION, 2.0);
        e.mem.set_f32(RAY_TOLERANCE, 0.01);
        e.mem.set_f64(HEIGHT_LIMIT, 0.5);
        e.mem.set_f64(HEIGHT_SHIFT, 2.0);
        e.mem.set_f32(VECTOR_FILL, 0.0);
        e.mem.set_f32(VECTOR_THIRD, 1.0);
        e.mem.set_f32(ROOT_THREE, 3.0);
        e.mem.set_f32(ROOT_HALF, 0.5);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_f32(ONE_VECTOR + 4 * lane, 1.0);
            e.mem.set_f32(NEG_HALF_LANES + 4 * lane, -0.5);
        }
        recorder(e, log, SET_MULTI_LAYER_COLLISION, 0);
        let kept = Rc::clone(log);
        e.register_double(TRANSFORM_POINT, move |e, a| {
            kept.borrow_mut().push((TRANSFORM_POINT, a.to_vec()));
            put_floats(e, a[0], &[0.0, 0.0, reference, 0.0]);
            Ret::default()
        });
        let kept = Rc::clone(log);
        e.register_double(WORLD_CAST_DOUBLE, move |e, a| {
            let mut words = vec![a[0]];
            words.extend(ray_words(e, a[1]));
            kept.borrow_mut().push((WORLD_CAST_DOUBLE, words));
            match answer {
                Some((vector, fraction)) => {
                    put_floats(e, a[1] + 0x30, &vector);
                    put_floats(e, a[1] + 0x40, &[fraction]);
                    Ret {
                        eax: 1,
                        ..Ret::default()
                    }
                }
                None => Ret::default(),
            }
        });
        let kept = Rc::clone(log);
        e.register_double(WORLD_CAST_RAY, move |e, a| {
            let mut words = vec![a[0]];
            words.extend(ray_words(e, a[1]));
            kept.borrow_mut().push((WORLD_CAST_RAY, words));
            if let Some((vector, fraction)) = answer {
                put_floats(e, a[2], &vector);
                put_floats(e, a[2] + 0x10, &[fraction]);
                e.mem.set_u32(a[2] + 0x50, 1);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x300);
        let world = object_with_vtable(e, &[(0xc8, WORLD_CAST_DOUBLE)]);
        let inner = e.mem.alloc(0x400);
        e.mem.set_u32(inner + 0x350, world);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 8, inner);
        let entries = e.mem.alloc(0x10);
        e.mem.set_u32(entries, first);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list + 8, entries);
        e.mem.set_u32(this + 0x48, list);
        e.mem.set_u32(this + 0x5c, 3);
        e.mem.set_u8(this + 0x240, 1);
        let cache = e.mem.alloc(0x200);
        e.mem.set_u32(this + 0x22c, cache);
        e.mem.set_f32(cache + 0x40, 2.0);
        let flag_byte = e.mem.alloc(4);
        e.mem.set_u8(flag_byte, flags);
        let transform = e.mem.alloc(0x40);
        put_floats(e, transform + 0x10, &[0.0, 0.0, 0.0, 1.0]);
        let pose = e.mem.alloc(0x40);
        e.mem.set_u32(pose + 0x10, transform);
        e.mem.set_u32(pose + 0x1c, flag_byte);
        e.mem.set_u32(this + 0x88, pose);
        recorder(e, log, CALCULATE_BONE_MODEL_SPACE, transform);
        let from = e.mem.alloc(0x10);
        put_floats(e, from, &[0.0, 0.0, 10.0, 0.0]);
        let to = e.mem.alloc(0x10);
        let fraction = e.mem.alloc(4);
        e.mem.set_f32(fraction, 9.0);
        let normal = e.mem.alloc(0x10);
        put_floats(e, normal, &[7.0; 4]);
        CastRig {
            this,
            from,
            to,
            fraction,
            normal,
            hit: e.mem.alloc(4),
            cache,
            pose,
        }
    }

    fn cast_ray(e: &mut Engine, rig: &CastRig) {
        let call_args = args![
            rig.this,
            rig.hit,
            rig.from,
            rig.to,
            rig.fraction,
            rig.normal
        ];
        e.call(0x00c7_7460, &call_args);
    }

    #[test]
    fn cast_ray_answers_the_hit_pointer() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = cast_rig(&mut e, &log, 0.0, 0, None);
        put_floats(&mut e, rig.cache + 0x10, &[0.005, 0.0, 10.0, 99.0]);
        put_floats(&mut e, rig.cache + 0x20, &[0.0; 4]);
        put_floats(&mut e, rig.cache + 0x30, &[0.0, 0.6, 0.8, 0.0]);
        e.mem.set_f32(rig.cache + 0x40, 0.25);
        let call_args = args![
            rig.this,
            rig.hit,
            rig.from,
            rig.to,
            rig.fraction,
            rig.normal
        ];
        // EAX is `hit` (00c77a9b) at the RET 00c77ab5.
        assert_eq!(e.call(0x00c7_7460, &call_args).u32(), rig.hit);
    }

    #[test]
    fn cast_ray_reuses_the_cached_ray_within_the_tolerance() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = cast_rig(&mut e, &log, 0.0, 0, None);
        put_floats(&mut e, rig.cache + 0x10, &[0.005, 0.0, 10.0, 99.0]);
        put_floats(&mut e, rig.cache + 0x20, &[0.0; 4]);
        put_floats(&mut e, rig.cache + 0x30, &[0.0, 0.6, 0.8, 0.0]);
        e.mem.set_f32(rig.cache + 0x40, 0.25);
        cast_ray(&mut e, &rig);
        assert!(calls_to(&log, SET_MULTI_LAYER_COLLISION).is_empty());
        assert!(calls_to(&log, WORLD_CAST_DOUBLE).is_empty());
        assert_eq!(e.mem.f32(rig.fraction), 0.25);
        assert_eq!(floats(&e, rig.normal, 4), vec![0.0, 0.6, 0.8, 0.0]);
        assert_eq!(e.mem.u8(rig.hit), 1);
        assert_eq!(e.mem.u16(rig.this + 0x238), 1);
        assert_eq!(floats(&e, rig.cache + 0x10, 4), vec![0.0, 0.0, 10.0, 0.0]);
        assert_eq!(e.mem.f32(rig.cache + 0x40), 0.25);
        let transforms = calls_to(&log, TRANSFORM_POINT);
        assert_eq!(transforms.len(), 1);
        assert_eq!(
            transforms[0][1..],
            [rig.this + 0x10, e.mem.u32(rig.pose + 0x10)]
        );
    }

    #[test]
    fn cast_ray_casts_into_the_world_and_stores_the_hit_in_the_cache() {
        let mut e = engine();
        let log = CallLog::default();
        let answer = Some(([0.0, 3.0, 4.0, 0.0], 0.5));
        let rig = cast_rig(&mut e, &log, 0.0, 0, answer);
        put_floats(&mut e, rig.cache + 0x10, &[0.0, 0.0, 10.0, 0.0]);
        cast_ray(&mut e, &rig);
        assert_eq!(
            calls_to(&log, SET_MULTI_LAYER_COLLISION),
            vec![args![0x27u32, 0x2002_200eu32, 0u32]]
        );
        let world_calls = calls_to(&log, WORLD_CAST_DOUBLE);
        assert_eq!(world_calls.len(), 1);
        assert_eq!(world_calls[0][1], 0x3_0027);
        assert_eq!(world_calls[0][2..6], [0, 0, 10.0f32.to_bits(), 0]);
        assert!(calls_to(&log, WORLD_CAST_RAY).is_empty());
        assert_eq!(e.mem.f32(rig.fraction), 0.5);
        for (got, want) in floats(&e, rig.normal, 4)
            .into_iter()
            .zip([0.0, 0.6, 0.8, 0.0])
        {
            assert!((got - want).abs() < 1e-3, "{got} against {want}");
        }
        assert_eq!(e.mem.u8(rig.hit), 1);
        assert_eq!(e.mem.u16(rig.this + 0x238), 1);
        assert_eq!(e.mem.f32(rig.cache + 0x40), 0.5);
        assert_eq!(floats(&e, rig.cache + 0x30, 4), floats(&e, rig.normal, 4));
        assert_eq!(calls_to(&log, TRANSFORM_POINT).len(), 1);
    }

    #[test]
    fn cast_ray_stores_no_hit_when_the_world_misses_or_the_start_moved() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = cast_rig(&mut e, &log, 0.0, 0, None);
        put_floats(&mut e, rig.cache + 0x10, &[0.02, 0.0, 10.0, 0.0]);
        e.mem.set_f32(rig.cache + 0x40, 0.25);
        cast_ray(&mut e, &rig);
        assert_eq!(calls_to(&log, WORLD_CAST_DOUBLE).len(), 1);
        assert_eq!(e.mem.u8(rig.hit), 0);
        assert_eq!(e.mem.u16(rig.this + 0x238), 1);
        assert_eq!(e.mem.f32(rig.fraction), 9.0);
        assert_eq!(floats(&e, rig.normal, 4), vec![7.0; 4]);
        assert_eq!(e.mem.f32(rig.cache + 0x40), 2.0);
        assert!(calls_to(&log, TRANSFORM_POINT).is_empty());
    }

    #[test]
    fn cast_ray_uses_the_object_ray_cast_and_the_up_normal_when_asked() {
        let mut e = engine();
        let log = CallLog::default();
        let answer = Some(([0.0, 0.0, 2.0, 0.0], 0.5));
        let rig = cast_rig(&mut e, &log, 0.0, 2, answer);
        let this = rig.this;
        e.mem.set_u8(this + 0x224, 1);
        e.mem.set_u8(this + 0x240, 0);
        cast_ray(&mut e, &rig);
        assert!(calls_to(&log, WORLD_CAST_DOUBLE).is_empty());
        assert!(calls_to(&log, WORLD_CAST_RAY).is_empty());
        assert_eq!(e.mem.u8(rig.hit), 0);
        e.mem.set_u32(this + 0x90, 0x7a7a);
        e.mem.set_u16(this + 0x238, 0);
        cast_ray(&mut e, &rig);
        let casts = calls_to(&log, WORLD_CAST_RAY);
        assert_eq!(casts.len(), 1);
        assert_eq!(casts[0][..2], [0x7a7a, 0x3_0027]);
        assert_eq!(casts[0][2..], [0, 0, 10.0f32.to_bits(), 0, 0, 0, 0, 0]);
        assert_eq!(e.mem.f32(rig.fraction), 0.5);
        assert_eq!(floats(&e, rig.normal, 4), vec![0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.u8(rig.hit), 1);
        let bone_calls = calls_to(&log, CALCULATE_BONE_MODEL_SPACE);
        assert_eq!(bone_calls, vec![args![rig.pose, 0u32]; 2]);
    }

    #[test]
    fn cast_ray_lowers_the_fraction_of_a_hit_above_the_reference_height() {
        let mut e = engine();
        let log = CallLog::default();
        let answer = Some(([0.0, 0.0, 1.0, 0.0], 0.5));
        let rig = cast_rig(&mut e, &log, 5.0, 0, answer);
        e.mem.set_u32(rig.this + 0x230, 2);
        cast_ray(&mut e, &rig);
        assert_eq!(e.mem.f32(rig.fraction), (7.0f64 / 10.0) as f32);
        assert_eq!(e.mem.f32(rig.cache + 0x40), (7.0f64 / 10.0) as f32);
        e.register_double(TRANSFORM_POINT, |e, a| {
            put_floats(e, a[0], &[0.0, 0.0, 8.0, 0.0]);
            Ret::default()
        });
        e.mem.set_u16(rig.this + 0x238, 0);
        e.mem.set_f32(rig.cache + 0x40, 2.0);
        cast_ray(&mut e, &rig);
        assert_eq!(e.mem.f32(rig.fraction), 0.5);
        e.register_double(TRANSFORM_POINT, |e, a| {
            put_floats(e, a[0], &[0.0, 0.0, 5.0, 0.0]);
            Ret::default()
        });
        e.mem.set_f64(HEIGHT_LIMIT, 1.5);
        e.mem.set_u16(rig.this + 0x238, 0);
        e.mem.set_f32(rig.cache + 0x40, 2.0);
        cast_ray(&mut e, &rig);
        assert_eq!(e.mem.f32(rig.fraction), 0.5);
    }

    const RECORDS_FREE_DOUBLE: u32 = 0x0900_e008;

    /// What `rig_00c7e3b0` built: the controller, the mapper holding the skeleton, the allocator
    /// and the raw blocks the bone records were allocated at.
    struct SkeletonRig {
        this: u32,
        mapper: u32,
        allocator: u32,
        raws: Rc<RefCell<Vec<u32>>>,
    }

    /// A controller with the root node `0x5000` whose collision object belongs to one node
    /// (name word `0x77`, position (2, 4, 6), no children) and the doubles `00c7e3b0` and the
    /// bone walk call: allocation of records, array append, the map's virtual functions at the
    /// vtable `010c4d18`, the array reserve and allocator, the string functions and the rest.
    fn rig_00c7e3b0(e: &mut Engine, log: &CallLog) -> SkeletonRig {
        install_ni2hk(e);
        let (map, _) = hash_map(e);
        let map_vtable = e.mem.u32(map);
        for slot in (0..0x40).step_by(4) {
            let function = e.mem.u32(map_vtable + slot);
            e.mem.set_u32(NODE_MAP_VTABLE + slot, function);
        }
        e.mem.set_u32(BONE_ARRAY_FREE_SLOT, RECORDS_FREE_DOUBLE);
        recorder(e, log, RECORDS_FREE_DOUBLE, 0);
        let raws = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&raws);
        e.register_double(MEMORY_ALLOCATE, move |e, _| {
            let raw = ((e.mem.alloc(0x90) + 0xf) & !0xf) + 3;
            kept.borrow_mut().push(raw);
            Ret {
                eax: raw,
                ..Ret::default()
            }
        });
        recorder(e, log, MEMORY_DEALLOCATE, 0);
        e.register_double(ARRAY_APPEND, |e, a| {
            if e.mem.u32(a[0] + 4) == 0 {
                let data = e.mem.alloc(0x40);
                e.mem.set_u32(a[0] + 4, data);
            }
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count + 1);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        e.register_double(NODE_ARRAY_APPEND, |_, _| Ret::default());
        e.register_double(MAP_LOOKUP, |_, _| Ret::default());
        let kept = Rc::clone(log);
        e.register_double(ARRAY_ALLOCATE, move |e, a| {
            let table = e.mem.alloc(a[0]);
            kept.borrow_mut().push((ARRAY_ALLOCATE, vec![a[0], table]));
            Ret {
                eax: table,
                ..Ret::default()
            }
        });
        for address in [
            CRT_MEMSET,
            ARRAY_FREE,
            MAP_CLEAR,
            SET_TO_REFERENCE_POSE,
            HK_STRING_CONSTRUCT,
            HK_STRING_DESTRUCT,
            HK_STRING_ASSIGN,
        ] {
            recorder(e, log, address, 0);
        }
        e.register_double(STRING_DUPLICATE, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        let cursor = e.mem.alloc(0x10);
        let (_, allocator) = install_router(e, log, cursor);
        let kept = Rc::clone(log);
        e.register_double(0x0900_a004, move |e, a| {
            kept.borrow_mut().push((0x0900_a004, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let kept = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            kept.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        recorder(e, log, 0x0900_e00c, 0);
        let node = object_with_vtable(e, &[(0xc, 0x0900_e00c)]);
        e.mem.set_u32(node + 8, 0x77);
        e.mem.set_u32(node + 0x18, 9);
        put_floats(e, node + 0x58, &[2.0, 4.0, 6.0]);
        let collision = e.mem.alloc(0x20);
        e.mem.set_u32(collision + 8, node);
        e.register_double(FIND_NEXT_COLLISION_OBJECT, move |_, a| Ret {
            eax: if a[0] == 0x5000 { collision } else { 0 },
            ..Ret::default()
        });
        let this = e.mem.alloc(0x300);
        e.mem.set_u32(this + 0x58, 0x5000);
        let node_data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xa4, node_data);
        let mapper = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2a4, mapper);
        SkeletonRig {
            this,
            mapper,
            allocator,
            raws,
        }
    }

    #[test]
    fn fn_00c7e3b0_builds_the_skeleton_and_the_pose_from_the_bone_records() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = rig_00c7e3b0(&mut e, &log);
        let this = rig.this;
        e.mem.set_u32(this + 0x58, 0);
        e.call(0x00c7_e3b0, &args![this]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(this + 0x58, 0x5000);
        e.call(0x00c7_e3b0, &args![this]);
        let skeleton = e.mem.u32(rig.mapper + 0xc);
        let pose = e.mem.u32(this + 0x88);
        assert_ne!((skeleton, pose), (0, 0));
        assert_eq!(e.mem.u32(skeleton), HKA_SKELETON_VTABLE);
        assert_eq!(
            (e.mem.u16(skeleton + 4), e.mem.u16(skeleton + 6)),
            (0x48, 1)
        );
        assert_eq!(
            calls_to(&log, 0x0900_a004),
            vec![vec![rig.allocator, 0x48], vec![rig.allocator, 0x38]]
        );
        let bones = e.mem.u32(skeleton + 0x18);
        assert_eq!(e.mem.u32(skeleton + 0x1c), 1);
        assert_eq!(e.mem.u8(bones + 4), 1);
        assert_eq!(
            calls_to(&log, HK_STRING_CONSTRUCT),
            vec![vec![skeleton + 8], vec![bones]]
        );
        assert_eq!(
            calls_to(&log, HK_STRING_ASSIGN),
            vec![vec![bones, 0x77 + 0x1000]]
        );
        assert!(calls_to(&log, HK_STRING_DESTRUCT).is_empty());
        assert_eq!(e.mem.u16(e.mem.u32(skeleton + 0xc)), 0xffff);
        assert_eq!(
            floats(&e, e.mem.u32(skeleton + 0x24), 4),
            vec![1.0, 2.0, 3.0, 0.0]
        );
        assert_eq!(e.mem.u32(skeleton + 0x10), 1);
        assert_eq!(e.mem.u32(skeleton + 0x28), 1);
        let reserved = |array: u32, count: u32, size: u32| vec![rig.allocator, array, count, size];
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![
                reserved(skeleton + 0x18, 1, 8),
                reserved(skeleton + 0xc, 1, 2),
                reserved(skeleton + 0x24, 1, 0x30),
                reserved(pose + 0x10, 1, 0x30),
                reserved(pose + 4, 1, 0x30),
                reserved(pose + 0x1c, 1, 4),
                reserved(this + 0x94, 1, 0x30),
            ]
        );
        assert_eq!(e.mem.u32(pose), skeleton);
        assert_eq!((e.mem.u32(pose + 8), e.mem.u32(pose + 0x14)), (1, 1));
        assert_eq!((e.mem.u32(pose + 0x20), e.mem.u32(this + 0x98)), (1, 1));
        assert_eq!(calls_to(&log, SET_TO_REFERENCE_POSE), vec![vec![pose]]);
        let raws = rig.raws.borrow();
        assert_eq!(raws.len(), 1);
        assert_eq!(
            calls_to(&log, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, raws[0]]]
        );
        let tables = calls_to(&log, ARRAY_ALLOCATE);
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0][0], 0x94);
        let clears = calls_to(&log, MAP_CLEAR);
        assert_eq!(clears.len(), 2);
        assert_eq!(clears[0], clears[1]);
        let map = clears[0][0];
        assert_eq!(
            calls_to(&log, CRT_MEMSET),
            vec![vec![tables[0][1], 0, 0x94]]
        );
        assert_eq!(calls_to(&log, ARRAY_FREE), vec![vec![tables[0][1]]]);
        let records_free = calls_to(&log, RECORDS_FREE_DOUBLE);
        assert_eq!(records_free.len(), 1);
        assert_eq!(records_free[0][0], map - 0x10);
    }

    #[test]
    fn fn_00c7e3b0_keeps_an_existing_skeleton() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = rig_00c7e3b0(&mut e, &log);
        let this = rig.this;
        let skeleton = e.mem.alloc(0x80);
        e.mem.set_u32(skeleton + 0x1c, 3);
        e.mem.set_u32(skeleton + 0x34, 5);
        e.mem.set_u32(rig.mapper + 0xc, skeleton);
        e.call(0x00c7_e3b0, &args![this]);
        assert_eq!(e.mem.u32(rig.mapper + 0xc), skeleton);
        assert!(calls_to(&log, HK_STRING_CONSTRUCT).is_empty());
        let pose = e.mem.u32(this + 0x88);
        assert_eq!(calls_to(&log, 0x0900_a004), vec![vec![rig.allocator, 0x38]]);
        let reserved = |array: u32, count: u32, size: u32| vec![rig.allocator, array, count, size];
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![
                reserved(pose + 0x10, 3, 0x30),
                reserved(pose + 4, 3, 0x30),
                reserved(pose + 0x1c, 3, 4),
                reserved(pose + 0x2c, 8, 4),
                reserved(this + 0x94, 3, 0x30),
            ]
        );
        assert_eq!((e.mem.u32(pose), e.mem.u32(pose + 0x30)), (skeleton, 5));
        assert_eq!((e.mem.u32(pose + 0x20), e.mem.u32(this + 0x98)), (3, 3));
        assert_eq!(calls_to(&log, SET_TO_REFERENCE_POSE), vec![vec![pose]]);
    }

    /// The controller and objects `do_foot_ik` works on.
    struct IkRig {
        this: u32,
        pose: u32,
        records: u32,
        objects: [u32; 2],
        model: u32,
    }

    /// What the foot solver double saw: its arguments, the 32 dwords of the input record, the
    /// ray indices `+0x23a`/`+0x238` of the controller and the saved vector of the foot object.
    struct IkCall {
        args: Vec<u32>,
        input: Vec<u32>,
        indices: (u16, u16),
        object_vector: Vec<f32>,
        bone_vector: Vec<f32>,
    }

    type IkCalls = Rc<RefCell<Vec<IkCall>>>;

    /// A controller with `feet` foot records (bones 1 and 2 of a three-bone chain with parents
    /// `(-1, 0, 1)` and the given flag words), the constants of `DoFootIK`, the doubles of the
    /// pose code and the timer, which answers half a second more with every call.
    fn ik_rig(e: &mut Engine, log: &CallLog, feet: u32, flags: &[u32]) -> IkRig {
        set_pose_constants(e);
        for (address, value) in [
            (0x0126_7d48u32, 0.125f32),
            (0x0126_7d54, 0.25),
            (0x0126_7d60, 0.5),
            (0x0126_7d6c, 0.75),
            (0x0126_7d78, 0.875),
            (FOOT_HEIGHT_SCALE, 0.5),
            (FOOT_HEIGHT_LIMIT, 1.0),
            (FOOT_TIME_TOTAL, 10.0),
            (BLEND_FACTOR, 0.5),
            (SELECTOR_DEFAULT, 0.5),
            (SMOOTHING_SCALE, 2.0),
            (OFFSET_LOW, -5.0),
            (OFFSET_HIGH, 5.0),
        ] {
            e.mem.set_f32(address, value);
        }
        put_floats(e, ONE_VECTOR, &[1.0; 4]);
        put_floats(e, NEGATIVE_BLEND_BASE, &[0.0; 4]);
        e.mem.set_f64(FOOT_TIME_SCALE, 1000.0);
        e.mem.set_f64(OFFSET_DECAY, 0.8);
        e.mem.set_u32(DECAY_STEPS, 5);
        recorder(e, log, FOOT_INPUT_CONSTRUCT, 0);
        let calls = Rc::new(std::cell::Cell::new(0u32));
        e.register_double(FOOT_TIMER, move |_, _| {
            calls.set(calls.get() + 1);
            st0(f64::from(calls.get()) * 0.5)
        });
        let pose = pose_setup(e, &[-1, 0, 1], flags);
        let local = e.mem.u32(pose + 0x10);
        let output = e.mem.u32(pose + 4);
        for bone in 0..3 {
            put_floats(e, local + 0x30 * bone + 0x20, &[2.0, 2.0, 2.0, 1.0]);
            put_floats(e, output + 0x30 * bone, &[3.0, 3.0, 3.0, 0.0]);
            put_floats(e, output + 0x30 * bone + 0x20, &[2.0, 2.0, 2.0, 2.0]);
        }
        put_floats(e, local, &[1.0, 2.0, 2.0, 0.0]);
        let model = e.mem.alloc(0x30);
        put_floats(e, model, &[7.0, 8.0, 9.0, 0.0]);
        put_floats(e, model + 0x10, &[0.0, 0.0, 0.0, 1.0]);
        recorder(e, log, CALCULATE_BONE_MODEL_SPACE, model);
        let this = e.mem.alloc(0x300);
        e.mem.set_u32(this + 0x88, pose);
        let records = e.mem.alloc(0x1a0);
        let objects = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        for (index, object) in objects.into_iter().enumerate() {
            e.mem.set_u32(records + 0xd0 * index as u32, object);
            e.mem.set_i16(object + 0x18, index as i16 + 1);
            put_floats(e, object + 0x30, &[9.0 + index as f32; 4]);
        }
        e.mem.set_u32(this + 0x22c, records);
        e.mem.set_u32(this + 0x230, feet);
        e.mem.set_f32(this + 0x23c, 1.0);
        e.mem.set_f32(this + 0x244, 0.5);
        IkRig {
            this,
            pose,
            records,
            objects,
            model,
        }
    }

    /// The foot solver double: it logs what it saw, answers the floats `outputs` one per call and
    /// replaces the first vector of the foot bone's output transform with `rewrite`.
    fn install_foot_solver(
        e: &mut Engine,
        rig: &IkRig,
        outputs: [f32; 2],
        rewrite: Option<Lanes>,
    ) -> IkCalls {
        let calls = IkCalls::default();
        let kept = Rc::clone(&calls);
        let this = rig.this;
        e.register_double(FOOT_SOLVE, move |e, a| {
            let number = kept.borrow().len();
            let bone = u32::from(e.mem.u16(a[0] + 0x18));
            kept.borrow_mut().push(IkCall {
                args: a.to_vec(),
                input: (0..32).map(|word| e.mem.u32(a[1] + 4 * word)).collect(),
                indices: (e.mem.u16(this + 0x23a), e.mem.u16(this + 0x238)),
                object_vector: floats(e, a[0] + 0x30, 4),
                bone_vector: floats(e, e.mem.u32(a[3] + 4) + 0x30 * bone, 4),
            });
            e.mem.set_f32(a[2], outputs[number]);
            if let Some(lanes) = rewrite {
                let output = e.mem.u32(a[3] + 4);
                store4(e, output + 0x30 * bone, lanes);
            }
            Ret::default()
        });
        calls
    }

    #[test]
    fn do_foot_ik_lifts_the_root_runs_the_solver_per_foot_and_blends_the_offset() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = ik_rig(&mut e, &log, 2, &[0, 0, 1]);
        let products = install_set_mul(&mut e);
        let calls = install_foot_solver(&mut e, &rig, [0.75, 0.25], Some([5.0, 5.0, 5.0, 0.0]));
        let transform = e.mem.alloc(0x30);
        put_floats(&mut e, transform, &[1.0, 2.0, 3.0, 4.0]);
        let this = rig.this;
        e.call(0x00c7_6350, &args![this, transform]);
        let local = e.mem.u32(rig.pose + 0x10);
        let output = e.mem.u32(rig.pose + 4);
        assert_eq!(floats(&e, local, 4), vec![1.0, 2.0, 2.5, 0.0]);
        let flags = e.mem.u32(rig.pose + 0x1c);
        let words: Vec<u32> = (0..3).map(|bone| e.mem.u32(flags + 4 * bone)).collect();
        assert_eq!(words, vec![1, 2, 2]);
        assert_eq!(e.mem.u8(rig.pose + 0x28), 0);
        assert_eq!(products.borrow().len(), 1);
        assert_eq!(products.borrow()[0].0[0], output + 0x60 + 0x10);
        assert_eq!(
            calls_to(&log, CALCULATE_BONE_MODEL_SPACE),
            vec![args![rig.pose, 1u32], args![rig.pose, 2u32]]
        );
        let calls = calls.borrow();
        assert_eq!(calls.len(), 2);
        for (index, call) in calls.iter().enumerate() {
            assert_eq!(call.args[0], rig.objects[index]);
            assert_eq!((call.args[1] != 0, call.args[2] != 0), (true, true));
            assert_eq!(call.args[3], rig.pose);
            assert_eq!(call.indices, (index as u16, 0));
            assert_eq!(call.object_vector, vec![0.0; 4]);
            let model: Vec<u32> = floats(&e, rig.model, 3)
                .into_iter()
                .map(f32::to_bits)
                .collect();
            assert_eq!(call.input[..3], model[..]);
            let lifted = [1.0f32, 2.0, 3.5, 4.0].map(f32::to_bits);
            assert_eq!(call.input[12..16], lifted);
            assert_eq!(call.input[0x18], 1);
            assert_eq!(call.input[0x19], this);
            let wanted = [0.125f32, 0.25, 0.5, 0.875, 0.75].map(f32::to_bits);
            assert_eq!(call.input[0x1a..0x1f], wanted);
        }
        for (index, object) in rig.objects.into_iter().enumerate() {
            assert_eq!(floats(&e, object + 0x30, 4), vec![9.0 + index as f32; 4]);
            assert_eq!(e.mem.f32(object + 0x80), 0.5);
        }
        assert_eq!(e.mem.f32(rig.records + 4), 0.75);
        assert_eq!(e.mem.f32(rig.records + 0xd4), 0.25);
        assert_eq!(calls[0].bone_vector, vec![3.0, 3.0, 3.0, 0.0]);
        for (index, call) in calls.iter().enumerate() {
            let bone = index as u32 + 1;
            assert_eq!(floats(&e, output + 0x30 * bone, 4), call.bone_vector);
        }
        assert_eq!(e.mem.f32(FOOT_TIME_TOTAL), 1010.0);
        assert_eq!(e.mem.f32(this + 0x23c), 2.0);
        assert_eq!(e.mem.u8(this + 0x223), 0);
    }

    #[test]
    fn do_foot_ik_limits_and_blends_the_foot_rotation() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = ik_rig(&mut e, &log, 1, &[0, 0, 0]);
        let this = rig.this;
        let output = e.mem.u32(rig.pose + 4);
        put_floats(&mut e, output + 0x30 + 0x10, &[1.0, 0.0, 0.0, 0.0]);
        let stored = rig.records + 0x90;
        put_floats(&mut e, stored, &[10.0; 4]);
        put_floats(&mut e, stored + 0x10, &[0.0, 0.0, 1.0, 0.0]);
        put_floats(&mut e, stored + 0x20, &[4.0; 4]);
        e.mem.set_u16(stored + 0x30, 1);
        e.mem.set_u8(this + 0x240, 1);
        let owner = e.mem.alloc(0x40);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table + 4, 1);
        let entries = e.mem.alloc(8);
        let wrapper = object_with_vtable(&mut e, &[(0x20, 0x0900_f020)]);
        let data = object_with_vtable(&mut e, &[(0x20, 0x0900_f021)]);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entries, entry);
        e.mem.set_u32(entry + 0xc, wrapper);
        e.mem.set_u32(owner + 0x20, table);
        e.mem.set_u32(owner + 0x14, entries);
        e.mem.set_u32(this + 0x48, owner);
        recorder(&mut e, &log, 0x0900_f020, 0xd);
        recorder(&mut e, &log, 0x0900_f021, 2);
        recorder(&mut e, &log, WRAPPED_DATA, data);
        put_floats(&mut e, data + 0x20, &[0.0, 1.0, 0.0, 0.0]);
        let seen = Rc::new(RefCell::new(Vec::<Vec<f32>>::new()));
        let kept = Rc::clone(&seen);
        let kept_log = Rc::clone(&log);
        e.register_double(DECOMPOSE_REST_AXIS, move |e, a| {
            kept_log
                .borrow_mut()
                .push((DECOMPOSE_REST_AXIS, a.to_vec()));
            kept.borrow_mut().push(floats(e, a[0], 4));
            kept.borrow_mut().push(floats(e, a[1], 4));
            e.mem.set_f32(a[3], 0.75);
            Ret::default()
        });
        let kept_log = Rc::clone(&log);
        e.register_double(SET_AXIS_ANGLE, move |e, a| {
            kept_log.borrow_mut().push((SET_AXIS_ANGLE, a.to_vec()));
            put_floats(e, a[0], &[0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        let kept_log = Rc::clone(&log);
        e.register_double(NORMALISE_VECTOR, move |e, a| {
            kept_log.borrow_mut().push((NORMALISE_VECTOR, a.to_vec()));
            let lanes = floats(e, a[0], 4);
            let length = lanes.iter().map(|lane| lane * lane).sum::<f32>().sqrt();
            for (index, lane) in lanes.into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * index as u32, lane / length);
            }
            Ret::default()
        });
        install_foot_solver(&mut e, &rig, [0.5, 0.0], Some([2.0; 4]));
        let transform = e.mem.alloc(0x30);
        e.call(0x00c7_6350, &args![this, transform]);
        let decompose = calls_to(&log, DECOMPOSE_REST_AXIS);
        assert_eq!(decompose.len(), 1);
        let frame = decompose[0][0];
        assert_eq!(
            decompose[0][1..],
            [frame + 0x10, frame + 0x20, frame + 0x40]
        );
        assert_eq!(
            *seen.borrow(),
            vec![vec![1.0, 0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0, 0.0]]
        );
        let angle = 0.75f32.to_bits();
        let set = calls_to(&log, SET_AXIS_ANGLE);
        assert_eq!(set, vec![vec![decompose[0][0], decompose[0][1], angle]]);
        assert_eq!(calls_to(&log, NORMALISE_VECTOR).len(), 1);
        assert_eq!(calls_to(&log, WRAPPED_DATA), vec![vec![wrapper]]);
        let root = 0.5f32.sqrt();
        let rotation = floats(&e, output + 0x30 + 0x10, 4);
        for (got, want) in rotation.iter().zip([0.0, 0.0, root, root]) {
            assert!((got - want).abs() < 1e-6, "{rotation:?}");
        }
        assert_eq!(floats(&e, output + 0x30, 4), vec![6.0; 4]);
        assert_eq!(floats(&e, output + 0x30 + 0x20, 4), vec![3.0; 4]);
        assert_eq!(floats(&e, stored, 4), vec![6.0; 4]);
        assert_eq!(floats(&e, stored + 0x20, 4), vec![3.0; 4]);
        assert_eq!(floats(&e, stored + 0x10, 4), rotation);
        put_floats(&mut e, stored + 0x10, &[0.0, 0.0, 0.0, -1.0]);
        put_floats(&mut e, output + 0x30 + 0x10, &[0.0, 0.0, 0.0, 1.0]);
        e.mem.set_u16(stored + 0x30, 0xffff);
        e.call(0x00c7_6350, &args![this, transform]);
        assert_eq!(calls_to(&log, DECOMPOSE_REST_AXIS).len(), 1);
        assert_eq!(floats(&e, stored + 0x10, 4), vec![0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn do_foot_ik_limits_the_offset_and_counts_down_the_decay() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = ik_rig(&mut e, &log, 0, &[0, 0, 0]);
        let this = rig.this;
        let transform = e.mem.alloc(0x30);
        e.mem.set_f32(this + 0x23c, 7.0);
        e.mem.set_u8(this + 0x221, 1);
        e.mem.set_u8(this + 0x223, 1);
        e.mem.set_u8(this + 0x242, 1);
        e.call(0x00c7_6350, &args![this, transform]);
        assert_eq!(e.mem.f32(this + 0x23c), 4.0);
        assert_eq!(e.mem.u32(this + 0x228), 1);
        assert_eq!((e.mem.u8(this + 0x221), e.mem.u8(this + 0x223)), (1, 0));
        assert_eq!(e.mem.u8(this + 0x242), 0);
        assert!(calls_to(&log, FOOT_INPUT_CONSTRUCT).len() == 1);
        e.mem.set_u32(DECAY_STEPS, 2);
        e.mem.set_f32(this + 0x23c, -9.0);
        e.mem.set_u8(this + 0x222, 1);
        e.call(0x00c7_6350, &args![this, transform]);
        assert_eq!(e.mem.f32(this + 0x23c), -4.0);
        assert_eq!(e.mem.u32(this + 0x228), 0);
        let bytes: Vec<u8> = [0x221, 0x222, 0x223, 0x242]
            .into_iter()
            .map(|offset| e.mem.u8(this + offset))
            .collect();
        assert_eq!(bytes, vec![0, 0, 0, 1]);
        e.mem.set_f32(this + 0x23c, -9.0);
        e.mem.set_u8(this + 0x243, 1);
        e.call(0x00c7_6350, &args![this, transform]);
        assert_eq!(e.mem.f32(this + 0x23c), -5.0);
    }

    const INIT_NODE: u32 = 0x5000;
    const INIT_NAME: u32 = 0x8000;
    const INIT_CURSOR: u32 = 0x0001_0000;
    const INIT_CAST_TYPE: u32 = 0x0126_83c8;
    const INIT_HEAD_NAME: u32 = 0x0101_f604;
    const SHARE_REGISTRY: u32 = 0x011b_0300;

    type Captures = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A logging double over `address` whose answer comes from `answer`.
    fn answering(
        e: &mut Engine,
        log: &CallLog,
        address: u32,
        mut answer: impl FnMut(&mut Engine, &[u32]) -> u32 + 'static,
    ) {
        let kept = Rc::clone(log);
        e.register_double(address, move |e, a| {
            kept.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer(e, a),
                ..Ret::default()
            }
        });
    }

    /// What `bhk_ragdoll_controller_init` works on and what the doubles saw.
    struct InitRig {
        this: u32,
        share: u32,
        collision_node: u32,
        mapper_skeleton: u32,
        mapper_structure: u32,
        built: u32,
        built_bones: u32,
        instance: u32,
        controller: u32,
        controller_floats: u32,
        holder: u32,
        cast_object: u32,
        router: u32,
        allocator: u32,
        arrays: Captures,
        params: Captures,
        conversions: Captures,
        quaternions: Captures,
    }

    /// A controller (name word `0x8000`, share data at `+0x2a4` holding the mapper skeleton at
    /// `+0xc`) for the node `0x5000`, whose collision object (answered by `00c802d0`) has a node
    /// without world and children, and a cast object when `with_cast`. The doubles answer the
    /// built skeleton (three bones, the names `0x6001`, `0x6009`, `0x6011`, of which the second
    /// and third contain the head name), the instance, the rigid body controller, the transform
    /// fetch (position 8, 10, 12, fill 7) and `reorder` for the reordering.
    fn init_rig(e: &mut Engine, log: &CallLog, reorder: u32, with_cast: bool) -> InitRig {
        install_ni2hk(e);
        e.mem.set_u32(NULL_STRING, 0x7000);
        for address in [
            POSE_LOADER_LOAD,
            0x00b5_82b0,
            0x00a5_d3a0,
            0x00a8_6bf0,
            0x0062_4ad0,
            0x00ce_5310,
            0x00ce_5370,
        ] {
            recorder(e, log, address, 0);
        }
        recorder(e, log, INTERLOCKED_INCREMENT, 1);
        recorder(e, log, INTERLOCKED_DECREMENT, 1);
        answering(e, log, STRING_COPY, |_, a| a[0] + 0x1000);
        let (router, allocator) = install_router(e, log, INIT_CURSOR);
        answering(e, log, 0x0900_a004, |e, a| e.mem.alloc(a[1]));
        answering(e, log, 0x00aa_13e0, |e, a| {
            let object = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(object + offset, 0xdead_beef);
            }
            object
        });
        let conversions = Captures::default();
        let kept = Rc::clone(&conversions);
        e.register_double(0x00a6_df40, move |e, a| {
            kept.borrow_mut().push(vec![
                a[0],
                a[1],
                e.mem.u32(a[1] + 0x24),
                e.mem.u32(a[1] + 0x30),
            ]);
            Ret::default()
        });
        let collision_node = e.mem.alloc(0x80);
        put_floats(e, collision_node + 0x34 + 0x24, &[2.0, 4.0, 6.0]);
        e.mem.set_f32(collision_node + 0x34 + 0x30, 3.0);
        let holder = e.mem.alloc(0x10);
        let cast_object = e.mem.alloc(0x60);
        for offset in 0..8 {
            e.mem
                .set_u32(cast_object + 0x20 + 4 * offset, 0x1111_1111 * (offset + 1));
        }
        let collision = e.mem.alloc(0x20);
        e.mem.set_u32(collision + 8, collision_node);
        e.mem
            .set_u32(collision + 0x10, if with_cast { holder } else { 0 });
        answering(e, log, FIND_NEXT_COLLISION_OBJECT, move |_, a| {
            if a[0] == INIT_NODE {
                collision
            } else {
                0
            }
        });
        answering(e, log, CAST_OBJECT, move |_, a| {
            if a[1] == INIT_CAST_TYPE {
                cast_object
            } else {
                0
            }
        });
        let quaternions = Captures::default();
        let kept = Rc::clone(&quaternions);
        e.register_double(0x00cb_2d90, move |e, a| {
            let mut words = vec![a[0], a[1]];
            words.extend((0..4).map(|i| e.mem.u32(a[0] + 0x30 + 4 * i)));
            kept.borrow_mut().push(words);
            Ret::default()
        });
        let result = e.mem.alloc(0x40);
        put_floats(e, result + 0x24, &[8.0, 10.0, 12.0]);
        e.mem.set_f32(result + 0x30, 7.0);
        answering(e, log, 0x0062_c250, move |_, _| result);
        let arrays = Captures::default();
        let kept = Rc::clone(&arrays);
        let log_reorder = Rc::clone(log);
        e.register_double(0x00ce_a030, move |e, a| {
            log_reorder.borrow_mut().push((0x00ce_a030, a.to_vec()));
            let mut words = vec![a[0], a[1]];
            for array in [a[0], a[1]] {
                words.extend((0..4).map(|i| e.mem.u32(array + 4 * i)));
            }
            kept.borrow_mut().push(words);
            Ret {
                eax: reorder,
                ..Ret::default()
            }
        });
        let built = e.mem.alloc(0x40);
        let built_bones = e.mem.alloc(0x20);
        for (index, name) in [0x6001, 0x6009, 0x6011].into_iter().enumerate() {
            e.mem.set_u32(built_bones + 8 * index as u32, name);
        }
        e.mem.set_u8(built_bones + 4, 1);
        e.mem.set_u32(built + 0x18, built_bones);
        e.mem.set_u32(built + 0x1c, 3);
        let transform = e.mem.alloc(0x40);
        e.mem.set_u32(built + 0x24, transform);
        answering(e, log, 0x00ce_b2c0, move |_, _| built);
        let instance = e.mem.alloc(0x10);
        answering(e, log, 0x00ce_a810, move |_, _| instance);
        answering(e, log, 0x00ec_7750, |_, a| {
            u32::from(a[1] == INIT_HEAD_NAME && (a[0] == 0x6008 || a[0] == 0x6010))
        });
        let controller = e.mem.alloc(0x10);
        let controller_floats = e.mem.alloc(0x30);
        e.mem.set_u32(controller, controller_floats);
        answering(e, log, 0x00cd_b400, move |_, _| controller);
        for index in 0..12 {
            let bits = if index == 1 {
                0x7f80_0001
            } else {
                0x3f80_0000 + index
            };
            e.mem.set_u32(0x0126_7c7c + 0xc * index, bits);
        }
        answering(e, log, 0x00cd_d390, |_, _| 3);
        let params = Captures::default();
        let kept = Rc::clone(&params);
        let log_mapping = Rc::clone(log);
        e.register_double(0x00ce_61c0, move |e, a| {
            log_mapping.borrow_mut().push((0x00ce_61c0, a.to_vec()));
            kept.borrow_mut().push(vec![
                e.mem.u32(a[0]),
                e.mem.u32(a[0] + 4),
                u32::from(e.mem.u8(a[0] + 0x39)),
            ]);
            Ret::default()
        });
        answering(e, log, 0x00ce_4a20, |_, a| a[0] + 0x10);
        let mapper_skeleton = e.mem.alloc(0x40);
        let mapper_structure = e.mem.alloc(0x10);
        e.mem.set_u8(mapper_structure + 4, 1);
        e.mem.set_u32(mapper_skeleton + 0x18, mapper_structure);
        let share = e.mem.alloc(0x40);
        e.mem.set_u32(share + 0xc, mapper_skeleton);
        let this = e.mem.alloc(0x300);
        e.mem.set_u32(this + 0x2a0, INIT_NAME);
        e.mem.set_u32(this + 0x2a4, share);
        InitRig {
            this,
            share,
            collision_node,
            mapper_skeleton,
            mapper_structure,
            built,
            built_bones,
            instance,
            controller,
            controller_floats,
            holder,
            cast_object,
            router,
            allocator,
            arrays,
            params,
            conversions,
            quaternions,
        }
    }

    /// The whole setup: skeleton built from the reordered arrays and converted twice, instance,
    /// head bone, rigid body controller with its twelve quiet floats, grab IK flag, and the two
    /// mappings built from parameters and buffers on the stack.
    #[test]
    fn bhk_ragdoll_controller_init_builds_the_skeleton_instance_and_mappings() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = init_rig(&mut e, &log, 0, true);
        e.call(0x00c7_e9a0, &args![rig.this, INIT_NODE, INIT_NAME]);
        let this = rig.this;
        assert_eq!(
            calls_to(&log, POSE_LOADER_LOAD),
            vec![vec![SHARE_REGISTRY, INIT_NAME, this + 0x2a4]]
        );
        assert!(calls_to(&log, 0x00b5_82b0).is_empty());
        let arrays = rig.arrays.borrow();
        assert_eq!(arrays.len(), 1);
        let (first, second) = (arrays[0][0], arrays[0][1]);
        assert_eq!(first - second, 0x10);
        assert_eq!(
            arrays[0][2..],
            [
                INIT_CURSOR,
                0,
                0x8000_0040,
                INIT_CURSOR,
                INIT_CURSOR + 0x100,
                0,
                0x8000_0040,
                INIT_CURSOR + 0x100
            ]
        );
        assert_eq!(calls_to(&log, 0x00ce_b2c0), vec![vec![first, second]]);
        assert_eq!(
            calls_to(&log, 0x00ce_a810),
            vec![vec![rig.built, first, second]]
        );
        assert_eq!(e.mem.u32(rig.share + 8), rig.built);
        let transform = first + 0x10;
        let buffer = first + 0x78;
        let copy = first + 0x44;
        let conversions = rig.conversions.borrow();
        assert_eq!(conversions.len(), 2);
        assert_eq!(
            conversions[0][1..],
            [
                rig.collision_node + 0x34,
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]
        );
        assert_eq!(
            conversions[1][1..],
            [copy, 8.0f32.to_bits(), 7.0f32.to_bits()]
        );
        let skeleton_transform = e.mem.u32(rig.built + 0x24);
        assert_eq!(floats(&e, skeleton_transform, 3), vec![4.0, 5.0, 6.0]);
        assert_eq!(
            floats(&e, skeleton_transform + 0x20, 4),
            vec![7.0, 7.0, 7.0, 7.0]
        );
        assert_eq!(
            calls_to(&log, CAST_OBJECT)
                .into_iter()
                .filter(|a| a[1] == INIT_CAST_TYPE)
                .collect::<Vec<_>>(),
            vec![vec![rig.holder, INIT_CAST_TYPE]]
        );
        assert_eq!(calls_to(&log, 0x00a8_6bf0), vec![vec![transform]]);
        let quaternions = rig.quaternions.borrow();
        assert_eq!(quaternions.len(), 1);
        assert_eq!(quaternions[0][..2], [buffer, rig.cast_object + 0x20]);
        assert_eq!(
            quaternions[0][2..],
            [0x5555_5555, 0x6666_6666, 0x7777_7777, 0x8888_8888]
        );
        assert_eq!(calls_to(&log, 0x0062_4ad0), vec![vec![transform, buffer]]);
        assert_eq!(
            calls_to(&log, 0x0062_c250),
            vec![vec![rig.collision_node + 0x34, buffer, transform]]
        );
        assert_eq!(e.mem.u32(this + 0x48), rig.instance);
        assert_eq!(
            calls_to(&log, 0x00ec_7750),
            vec![
                vec![0x6000, INIT_HEAD_NAME],
                vec![0x6008, INIT_HEAD_NAME],
                vec![0x6010, INIT_HEAD_NAME]
            ]
        );
        assert_eq!(e.mem.u32(this + 0x2b8), 2);
        assert_eq!(
            calls_to(&log, 0x0900_a004),
            vec![
                vec![rig.allocator, 0x48],
                vec![rig.allocator, 0x80],
                vec![rig.allocator, 0x80]
            ]
        );
        let constructed = calls_to(&log, 0x00cd_b400);
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0][1], rig.instance);
        assert_eq!(e.mem.u32(this + 0x4c), rig.controller);
        let copied = floats(&e, rig.controller_floats, 12)
            .into_iter()
            .map(f32::to_bits)
            .collect::<Vec<_>>();
        let wanted: Vec<u32> = (0..12)
            .map(|index| {
                if index == 1 {
                    0x7fc0_0001
                } else {
                    0x3f80_0000 + index
                }
            })
            .collect();
        assert_eq!(copied, wanted);
        assert_eq!(e.mem.u8(this + 0x42), 1);
        assert_eq!(e.mem.u8(this + 0x21c), 1);
        assert_eq!(calls_to(&log, 0x00cd_d390).len(), 6);
        assert_eq!(e.mem.u8(rig.mapper_structure + 4), 0);
        assert_eq!(e.mem.u8(rig.built_bones + 4), 0);
        let (params, first_buffer, second_buffer) = (first + 0x78, first + 0x148, first + 0xd8);
        assert_eq!(
            calls_to(&log, 0x00ce_5310),
            vec![vec![first_buffer], vec![second_buffer]]
        );
        assert_eq!(
            calls_to(&log, 0x00ce_61c0),
            vec![vec![params, first_buffer, second_buffer]]
        );
        assert_eq!(
            rig.params.borrow()[..],
            [vec![rig.mapper_skeleton, rig.built, 0]]
        );
        let (first_mapping, second_mapping) =
            (e.mem.u32(rig.share + 0x10), e.mem.u32(rig.share + 0x14));
        assert_eq!(
            calls_to(&log, 0x00ce_4a20),
            vec![
                vec![first_mapping - 0x10, first_buffer],
                vec![second_mapping - 0x10, second_buffer]
            ]
        );
        assert_eq!(e.mem.u16(first_mapping - 0x10 + 4), 0x80);
        assert_eq!(e.mem.u16(second_mapping - 0x10 + 4), 0x80);
        assert_eq!(
            calls_to(&log, 0x00ce_5370),
            vec![vec![second_buffer], vec![first_buffer]]
        );
        assert!(calls_to(&log, 0x0900_a010).is_empty());
        assert_eq!(e.mem.u32(rig.router), INIT_CURSOR);
    }

    /// With the reordering answering 1 nothing is built and the arrays, which the reordering
    /// grew past their chunks, are given back (second one first) with their cursor restored.
    #[test]
    fn bhk_ragdoll_controller_init_releases_grown_arrays_when_reordering_answers_one() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = init_rig(&mut e, &log, 1, true);
        let kept = Rc::clone(&log);
        e.register_double(0x00ce_a030, move |e, a| {
            kept.borrow_mut().push((0x00ce_a030, a.to_vec()));
            e.mem.set_u32(a[0], 0xd000);
            e.mem.set_u32(a[0] + 8, 5);
            e.mem.set_u32(a[1], 0xe000);
            e.mem.set_u32(a[1] + 8, 0x4000_0003);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call(0x00c7_e9a0, &args![rig.this, INIT_NODE, INIT_NAME]);
        assert_eq!(calls_to(&log, 0x00ce_a030).len(), 1);
        for address in [
            0x00ce_b2c0,
            0x00ce_a810,
            0x00ce_5310,
            0x00ce_61c0,
            0x00cd_b400,
        ] {
            assert!(calls_to(&log, address).is_empty());
        }
        assert_eq!(e.mem.u32(rig.share + 8), 0);
        assert_eq!(
            calls_to(&log, 0x0900_a010),
            vec![
                vec![rig.allocator, 0xe000, 12],
                vec![rig.allocator, 0xd000, 20]
            ]
        );
        assert_eq!(e.mem.u32(rig.router), INIT_CURSOR);
    }

    /// An existing skeleton is kept, an instance of 0 builds no controller, and existing
    /// mappings are not created again.
    #[test]
    fn bhk_ragdoll_controller_init_keeps_existing_data() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = init_rig(&mut e, &log, 0, false);
        let existing = e.mem.alloc(0x40);
        let bones = e.mem.alloc(0x10);
        e.mem.set_u32(existing + 0x18, bones);
        e.mem.set_u32(rig.share + 8, existing);
        e.mem.set_u32(rig.share + 0x14, 0x1234);
        answering(&mut e, &log, 0x00ce_a810, |_, _| 0);
        e.call(0x00c7_e9a0, &args![rig.this, INIT_NODE, INIT_NAME]);
        assert!(calls_to(&log, 0x00ce_b2c0).is_empty());
        assert!(calls_to(&log, 0x0062_c250).is_empty());
        assert_eq!(e.mem.u32(rig.share + 8), existing);
        let arrays = rig.arrays.borrow();
        assert_eq!(
            calls_to(&log, 0x00ce_a810),
            vec![vec![existing, arrays[0][0], arrays[0][1]]]
        );
        assert_eq!(e.mem.u32(rig.this + 0x48), 0);
        assert_eq!(e.mem.u32(rig.this + 0x4c), 0);
        assert_eq!(e.mem.u8(rig.this + 0x42), 0);
        assert!(calls_to(&log, 0x00cd_d390).is_empty());
        assert!(calls_to(&log, 0x0900_a004).is_empty());
        assert!(calls_to(&log, 0x00ce_61c0).is_empty());
        assert!(calls_to(&log, 0x00ce_5310).is_empty());
        assert_eq!(e.mem.u32(rig.router), INIT_CURSOR);
    }

    /// Without a cast object the collision node's transform is fetched against the identity.
    #[test]
    fn bhk_ragdoll_controller_init_skips_the_rotation_without_a_cast_object() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = init_rig(&mut e, &log, 0, false);
        e.call(0x00c7_e9a0, &args![rig.this, INIT_NODE, INIT_NAME]);
        let arrays = rig.arrays.borrow();
        let transform = arrays[0][0] + 0x10;
        let buffer = arrays[0][0] + 0x78;
        assert!(rig.quaternions.borrow().is_empty());
        assert!(calls_to(&log, 0x0062_4ad0).is_empty());
        assert!(calls_to(&log, CAST_OBJECT)
            .iter()
            .all(|a| a[1] != INIT_CAST_TYPE));
        assert_eq!(calls_to(&log, 0x00a8_6bf0), vec![vec![transform]]);
        assert_eq!(
            calls_to(&log, 0x0062_c250),
            vec![vec![rig.collision_node + 0x34, buffer, transform]]
        );
        assert_eq!(rig.conversions.borrow().len(), 2);
    }

    /// A new name is copied and the old string released unless it is the null string; with the
    /// name 0 the string held by the node is taken over (counted unless it is the null string,
    /// the copy just made released); without a collision object nothing else happens.
    #[test]
    fn bhk_ragdoll_controller_init_replaces_the_name_string() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = init_rig(&mut e, &log, 0, true);
        let this = rig.this;
        let node = e.mem.alloc(0x20);
        for (previous, name, held, copies, releases, counts, stored) in [
            (
                0x6000,
                0x8100,
                0,
                vec![vec![0x8100]],
                vec![vec![0x5ff8]],
                vec![],
                0x9100,
            ),
            (
                0x7000,
                0x8200,
                0,
                vec![vec![0x8200]],
                vec![],
                vec![],
                0x9200,
            ),
            (
                0x6000,
                0,
                0x9000,
                vec![vec![0]],
                vec![vec![0x5ff8], vec![0xff8]],
                vec![vec![0x8ff8]],
                0x9000,
            ),
            (0, 0, 0x7000, vec![], vec![vec![0xfffffff8]], vec![], 0x7000),
            (0, 0, 0, vec![], vec![], vec![], 0),
        ] {
            log.borrow_mut().clear();
            e.mem.set_u32(this + 0x2a0, previous);
            e.mem.set_u32(node + 8, held);
            e.call(0x00c7_e9a0, &args![this, node, name]);
            assert_eq!(calls_to(&log, STRING_COPY), copies);
            assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT), releases);
            assert_eq!(calls_to(&log, INTERLOCKED_INCREMENT), counts);
            assert_eq!(e.mem.u32(this + 0x2a0), stored);
            assert_eq!(
                calls_to(&log, POSE_LOADER_LOAD),
                vec![vec![SHARE_REGISTRY, stored, this + 0x2a4]]
            );
            assert_eq!(calls_to(&log, FIND_NEXT_COLLISION_OBJECT), vec![vec![node]]);
            assert!(calls_to(&log, 0x00ce_a030).is_empty());
        }
    }

    /// Without share data an object is allocated, built, counted for the slot and for the
    /// by-value pointer and registered; when the allocation fails the pointer stays 0.
    #[test]
    fn bhk_ragdoll_controller_init_allocates_the_share_data() {
        let mut e = engine();
        let log = CallLog::default();
        let rig = init_rig(&mut e, &log, 0, true);
        let this = rig.this;
        e.mem.set_u32(this + 0x2a4, 0);
        e.call(0x00c7_e9a0, &args![this, 0x6000u32, INIT_NAME]);
        let object = e.mem.u32(this + 0x2a4);
        assert_ne!(object, 0);
        assert_eq!(calls_to(&log, 0x00aa_13e0), vec![vec![0x20]]);
        assert_eq!(calls_to(&log, 0x00a5_d3a0), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), 0x010c_4b74);
        for offset in [8, 0xc, 0x10, 0x14, 0x1c] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(e.mem.u32(object + 0x18), 0xdead_beef);
        assert_eq!(
            calls_to(&log, INTERLOCKED_INCREMENT),
            vec![vec![object + 4], vec![object + 4]]
        );
        assert_eq!(
            calls_to(&log, 0x00b5_82b0),
            vec![vec![SHARE_REGISTRY, INIT_NAME, object]]
        );
        log.borrow_mut().clear();
        answering(&mut e, &log, 0x00aa_13e0, |_, _| 0);
        e.mem.set_u32(this + 0x2a4, 0);
        e.call(0x00c7_e9a0, &args![this, 0x6000u32, INIT_NAME]);
        assert_eq!(e.mem.u32(this + 0x2a4), 0);
        assert!(calls_to(&log, 0x00a5_d3a0).is_empty());
        assert!(calls_to(&log, INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(
            calls_to(&log, 0x00b5_82b0),
            vec![vec![SHARE_REGISTRY, INIT_NAME, 0]]
        );
    }

    #[test]
    fn fn_00c74b70_multiplies_two_quaternions_lane_by_lane() {
        let mut e = engine();
        let (out, a, b) = (e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10));
        put_floats(&mut e, a, &[1.0, 2.0, 3.0, 4.0]);
        put_floats(&mut e, b, &[5.0, 6.0, 7.0, 8.0]);
        e.call(0x00c74b70, &args![out, a, b]);
        assert_eq!(floats(&e, out, 4), vec![16.0, 0.0, 8.0, 70.0]);
        assert_eq!(floats(&e, a, 4), vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn fn_00c74d50_converts_the_transform_through_a_normalised_quaternion() {
        let mut e = engine();
        let log = CallLog::default();
        install_sqrt(&mut e, &log);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&seen);
        e.register_double(0x004f_0180, move |e, a| {
            kept.borrow_mut().push((a.to_vec(), floats(e, a[0], 4)));
            Ret::default()
        });
        recorder(&mut e, &log, POINT_COPY, 0);
        let (destination, source) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        put_floats(&mut e, source, &[0.0; 12]);
        put_floats(&mut e, source + 0x10, &[0.0, 0.0, 0.0, 2.0]);
        e.mem.set_u32(source + 0x20, 0x7f80_0001);
        let got = e.call(0x00c74d50, &args![destination, source]).u32();
        assert_eq!(got, destination);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[1], destination);
        assert_eq!(seen[0].1, vec![1.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            calls_to(&log, POINT_COPY),
            vec![vec![destination + 0x24, source]]
        );
        assert_eq!(e.mem.u32(destination + 0x30), 0x7fc0_0001);
    }

    #[test]
    fn ni2hk_ov2_reorders_the_quaternion_and_scales_the_position() {
        let mut e = engine();
        e.set_global(SCALE_FACTOR, 0.5f32);
        let lanes = [
            1.0f32.to_bits(),
            0x7f80_0001,
            3.0f32.to_bits(),
            4.0f32.to_bits(),
        ];
        e.register_double(0x00a6_df40, move |e, a| {
            for (index, bits) in lanes.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * index as u32, bits);
            }
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&seen);
        e.register_double(0x0056_11c0, move |e, a| {
            kept.borrow_mut().push((a[0], floats(e, a[0], 4)));
            Ret::default()
        });
        let (destination, source) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        put_floats(&mut e, source + 0x24, &[2.0, 4.0, 6.0]);
        e.mem.set_u32(source + 0x30, 0x7f80_0001);
        assert_eq!(
            e.call(0x00c74dd0, &args![destination, source]).u32(),
            destination
        );
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(seen.borrow()[0].0, destination + 0x10);
        assert_eq!(e.mem.u32(destination + 0x10), 0x7fc0_0001);
        assert_eq!(floats(&e, destination + 0x14, 3), vec![3.0, 4.0, 1.0]);
        assert_eq!(floats(&e, destination, 4), vec![1.0, 2.0, 3.0, 0.0]);
        for offset in [0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(destination + offset), 0x7f80_0001);
        }
    }

    #[test]
    fn hka_raycast_interface_scalar_deleting_destructor_frees_only_on_bit_zero() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, MEMORY_DEALLOCATE, 0);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00c74e70, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_4b60);
        assert!(log.borrow().is_empty());
        assert_eq!(e.call(0x00c74e70, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&log, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00c74ea0_aligns_up_and_stores_the_distance_below() {
        let mut e = engine();
        let log = CallLog::default();
        let base = (e.mem.alloc(0x80) + 0xf) & !0xf;
        recorder(&mut e, &log, MEMORY_ALLOCATE, base + 7);
        assert_eq!(e.call(0x00c74ea0, &args![0x30u32]).u32(), base + 0x10);
        assert_eq!(e.mem.u8(base + 0xf), 9);
        assert_eq!(
            calls_to(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x40]]
        );
        recorder(&mut e, &log, MEMORY_ALLOCATE, base);
        assert_eq!(e.call(0x00c74ea0, &args![0x30u32]).u32(), base + 0x10);
        assert_eq!(e.mem.u8(base + 0xf), 0x10);
    }

    #[test]
    fn fn_00c74ed0_fills_the_record_with_defaults() {
        let mut e = engine();
        for (index, bits) in [0x11u32, 0x22, 0x33, 0x44].into_iter().enumerate() {
            e.mem.set_u32(CONSTANT_VECTOR_A + 4 * index as u32, bits);
            e.mem
                .set_u32(CONSTANT_VECTOR_B + 4 * index as u32, bits + 0x100);
        }
        e.mem.set_u32(0x0101_2054, 0x7f80_0001);
        let this = e.mem.alloc(0x90);
        assert_eq!(e.call(0x00c74ed0, &args![this]).u32(), this);
        for offset in [0, 2, 4, 6, 8] {
            assert_eq!(e.mem.u16(this + offset), 0xffff);
        }
        let words = |e: &Engine, offset: u32| {
            (0..4)
                .map(|i| e.mem.u32(this + offset + 4 * i))
                .collect::<Vec<_>>()
        };
        assert_eq!(words(&e, 0x10), vec![0x11, 0x22, 0x33, 0x44]);
        assert_eq!(words(&e, 0x40), vec![0x11, 0x22, 0x33, 0x44]);
        assert_eq!(words(&e, 0x50), vec![0x111, 0x122, 0x133, 0x144]);
        assert_eq!(words(&e, 0x60), vec![0x11, 0x22, 0x33, 0x44]);
        assert_eq!(words(&e, 0x70), vec![0x111, 0x122, 0x133, 0x144]);
        assert_eq!(e.mem.u32(this + 0x20), 0x7fc0_0001);
        assert_eq!(floats(&e, this + 0x24, 4), vec![1.0; 4]);
        assert_eq!((e.mem.u8(this + 0x80), e.mem.u8(this + 0x81)), (1, 0));
    }

    /// Two settings records with the same contents: one 16-bit entry and floats `1.0, 2.0, ...`.
    fn settings_pair(e: &mut Engine) -> (u32, u32) {
        let (a, b) = (e.mem.alloc(0x50), e.mem.alloc(0x50));
        for record in [a, b] {
            let entries = e.mem.alloc(8);
            e.mem.set_u16(entries, 5);
            e.mem.set_u16(entries + 2, 6);
            e.mem.set_u32(record + 0x44, entries);
            e.mem.set_u32(record + 0x48, 2);
            e.mem.set_u8(record, 7);
            for index in 1..=12u32 {
                e.mem.set_f32(record + 4 * index, index as f32);
            }
            e.mem.set_u32(record + 0x38, 0x38);
            e.mem.set_u32(record + 0x3c, 0x3c);
            e.mem.set_f32(record + 0x34, 52.0);
        }
        (a, b)
    }

    #[test]
    fn fn_00c74fb0_reports_any_difference_between_the_records() {
        let mut e = engine();
        let (a, b) = settings_pair(&mut e);
        assert!(!e.call(0x00c74fb0, &args![a, b]).bool());
        e.mem.set_u32(b + 0x48, 3);
        assert!(e.call(0x00c74fb0, &args![a, b]).bool());
        e.mem.set_u32(b + 0x48, 2);
        let entries = e.mem.u32(b + 0x44);
        e.mem.set_u16(entries + 2, 9);
        assert!(e.call(0x00c74fb0, &args![a, b]).bool());
        e.mem.set_u16(entries + 2, 6);
        e.mem.set_u8(b, 8);
        assert!(e.call(0x00c74fb0, &args![a, b]).bool());
        e.mem.set_u8(b, 7);
        for offset in (4..=0x30).step_by(4) {
            e.mem.set_f32(b + offset, 99.0);
            assert!(e.call(0x00c74fb0, &args![a, b]).bool(), "{offset:#x}");
            e.mem.set_f32(b + offset, (offset / 4) as f32);
        }
        for offset in [0x38, 0x3c] {
            e.mem.set_u32(b + offset, 1);
            assert!(e.call(0x00c74fb0, &args![a, b]).bool());
            e.mem.set_u32(b + offset, offset);
        }
        e.mem.set_f32(b + 0x34, 53.0);
        assert!(e.call(0x00c74fb0, &args![a, b]).bool());
        e.mem.set_f32(b + 0x34, 52.0);
        assert!(!e.call(0x00c74fb0, &args![a, b]).bool());
        e.mem.set_u32(a + 0x34, 0x7fc0_0000);
        e.mem.set_u32(b + 0x34, 0x7fc0_0000);
        assert!(e.call(0x00c74fb0, &args![a, b]).bool());
    }

    #[test]
    fn fn_00c74fb0_with_no_entries_compares_the_rest() {
        let mut e = engine();
        let (a, b) = settings_pair(&mut e);
        e.mem.set_u32(a + 0x48, 0);
        e.mem.set_u32(b + 0x48, 0);
        e.mem.set_u32(a + 0x44, 0);
        e.mem.set_u32(b + 0x44, 0);
        assert!(!e.call(0x00c74fb0, &args![a, b]).bool());
    }

    #[test]
    fn fn_00c75100_calls_slot_zero_with_one_inside_the_scope() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&seen);
        e.register_double(0x0900_0000, move |e, a| {
            let word = e.tls() + SCOPE_WORD;
            kept.borrow_mut().push((a.to_vec(), e.mem.u32(word)));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0000);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let this = e.mem.alloc(0x10);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 7);
        e.call(0x00c75100, &args![this]);
        assert!(seen.borrow().is_empty());
        assert_eq!(e.mem.u32(word), 7);
        e.mem.set_u32(this, object);
        e.call(0x00c75100, &args![this]);
        assert_eq!(*seen.borrow(), vec![(vec![object, 1], 0x10)]);
        assert_eq!(e.mem.u32(word), 7);
    }

    #[test]
    fn capsule_bone_shape_info_follows_the_chain_to_slot_0xb4() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, 0x00c8_02d0, 0);
        recorder(&mut e, &log, EMPTY_LOCK, 0);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&seen);
        e.register_double(0x0900_00b4, move |e, a| {
            kept.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xb4, 0x0900_00b4);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder, vtable);
        let (found, shape, object, inner) = (
            e.mem.alloc(0x20),
            e.mem.alloc(0x20),
            e.mem.alloc(0x20),
            e.mem.alloc(0x10),
        );
        e.mem.set_u32(found + 0x10, shape);
        e.mem.set_u32(shape + 8, object);
        e.mem.set_u32(object + 0x10, inner);
        e.mem.set_u32(inner + 8, holder);
        assert_eq!(e.call(0x00c75180, &args![0u32]).u32(), 0);
        assert!(log.borrow().is_empty());
        assert_eq!(e.call(0x00c75180, &args![0x55u32]).u32(), 0);
        recorder(&mut e, &log, 0x00c8_02d0, found);
        assert_eq!(e.call(0x00c75180, &args![0x55u32]).u32(), 0x77);
        assert_eq!(*seen.borrow(), vec![(holder, 0x55)]);
        assert_eq!(calls_to(&log, EMPTY_LOCK), vec![vec![shape]; 2]);
        e.mem.set_u32(inner + 8, 0);
        assert_eq!(e.call(0x00c75180, &args![0x55u32]).u32(), 0);
        e.mem.set_u32(shape + 8, 0);
        assert_eq!(e.call(0x00c75180, &args![0x55u32]).u32(), 0);
    }

    type Snapshots = Rc<RefCell<Vec<Vec<u8>>>>;

    /// Fills a grab entry: target, blend, weight, active and held bytes.
    fn set_entry(e: &mut Engine, entry: u32, target: u32, blend: f32, weight: f32, flags: [u8; 2]) {
        e.mem.set_u32(entry, target);
        e.mem.set_f32(entry + 4, blend);
        e.mem.set_f32(entry + 8, weight);
        e.mem.set_u8(entry + 0xc, flags[0]);
        e.mem.set_u8(entry + 0xd, flags[1]);
    }

    /// The grab IK setup: the float globals, a controller block with a skeleton pointer and a
    /// target object, and doubles for the transform (`00c7f840`, answers `(7, 8, 9, 10)`) and
    /// the solver (`00cdbae0`, logs the 0x90 bytes of its record).
    fn grab_setup(e: &mut Engine) -> (u32, CallLog, Snapshots) {
        let log = CallLog::default();
        e.set_global(SCALE_FACTOR, 0.5f32);
        e.set_global(0x0126_7dfc, 0.5f32);
        e.set_global(0x0101_3ea4, 0.1f32);
        put_floats(e, ONES_VECTOR, &[1.0; 4]);
        put_floats(e, CONSTANT_VECTOR_A, &[0.0; 4]);
        put_floats(e, CONSTANT_VECTOR_B, &[0.0; 4]);
        e.mem.set_f32(0x0101_2054, 0.0);
        e.register_double(0x00c7_f840, {
            let kept = Rc::clone(&log);
            move |e, a| {
                kept.borrow_mut().push((0x00c7_f840, a.to_vec()));
                put_floats(e, a[0], &[7.0, 8.0, 9.0, 10.0]);
                Ret::default()
            }
        });
        let records = Rc::new(RefCell::new(Vec::new()));
        e.register_double(0x00cd_bae0, {
            let kept = Rc::clone(&log);
            let records = Rc::clone(&records);
            move |e, a| {
                kept.borrow_mut().push((0x00cd_bae0, a.to_vec()));
                records.borrow_mut().push(e.mem.bytes(a[1], 0x90));
                Ret::default()
            }
        });
        let this = e.mem.alloc(0x300);
        e.mem.set_u32(this + 0x88, 0x5151);
        for (index, bone) in [1u16, 2, 3, 4, 5, 6].into_iter().enumerate() {
            e.mem.set_u16(this + 0x210 + 2 * index as u32, bone);
        }
        e.mem.set_u8(this + 0x21e, 0x55);
        (this, log, records)
    }

    #[test]
    fn do_grab_ik_replaces_the_vectors_and_solves_with_the_record() {
        let mut e = engine();
        let (this, log, records) = grab_setup(&mut e);
        let target = e.mem.alloc(0x100);
        put_floats(&mut e, target + 0x8c, &[2.0, 4.0, 6.0]);
        set_entry(&mut e, this + 0x1b0, target, 0.25, 0.0, [1, 0]);
        e.call(0x00c751f0, &args![this, 0x1234u32]);
        let entry = this + 0x1b0;
        assert_eq!(floats(&e, entry + 0x10, 4), vec![1.0, 2.0, 3.0, 0.0]);
        assert_eq!(floats(&e, entry + 0x20, 4), vec![0.0, 1.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(entry + 8), 0.5);
        let transforms = calls_to(&log, 0x00c7_f840);
        assert_eq!(transforms.len(), 1);
        assert_eq!(transforms[0][1..], [0x1234, entry + 0x10]);
        let solves = calls_to(&log, 0x00cd_bae0);
        assert_eq!(solves.len(), 1);
        assert_eq!(solves[0][2], 0x5151);
        let record = &records.borrow()[0];
        let word =
            |offset: usize| u32::from_le_bytes(record[offset..offset + 4].try_into().unwrap());
        assert_eq!(
            [word(0) as u16, (word(0) >> 16) as u16, word(4) as u16],
            [1, 2, 3]
        );
        assert_eq!(word(0x28), 0.5f32.to_bits());
        assert_eq!(word(0x2c), 0.5f32.to_bits());
        assert_eq!(f32::from_bits(word(0x18)), 1.0);
        assert_eq!(f32::from_bits(word(0x40)), 7.0);
        assert_eq!(f32::from_bits(word(0x4c)), 10.0);
        assert_eq!(e.mem.u8(this + 0x21e), 0x55);
    }

    #[test]
    fn do_grab_ik_blends_towards_the_new_vectors_when_the_weight_is_positive() {
        let mut e = engine();
        let (this, _, _) = grab_setup(&mut e);
        let target = e.mem.alloc(0x100);
        put_floats(&mut e, target + 0x8c, &[2.0, 4.0, 6.0]);
        let entry = this + 0x1b0;
        set_entry(&mut e, entry, target, 0.25, 0.5, [1, 0]);
        put_floats(&mut e, entry + 0x10, &[10.0; 4]);
        put_floats(&mut e, entry + 0x20, &[4.0; 4]);
        e.call(0x00c751f0, &args![this, 0x1234u32]);
        assert_eq!(floats(&e, entry + 0x10, 4), vec![7.75, 8.0, 8.25, 7.5]);
        assert_eq!(floats(&e, entry + 0x20, 4), vec![3.0, 3.25, 3.0, 3.0]);
        assert_eq!(e.mem.f32(entry + 8), 0.75);
    }

    #[test]
    fn do_grab_ik_releases_a_held_entry_whose_weight_falls_below_the_limit() {
        let mut e = engine();
        let (this, log, _) = grab_setup(&mut e);
        set_entry(&mut e, this + 0x1b0, 0, 0.0, 0.05, [1, 1]);
        e.call(0x00c751f0, &args![this, 0x1234u32]);
        let entry = this + 0x1b0;
        assert_eq!((e.mem.u8(entry + 0xc), e.mem.u8(entry + 0xd)), (0, 0));
        assert_eq!(e.mem.f32(entry + 8), 0.0);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u8(this + 0x21e), 0);
    }

    #[test]
    fn do_grab_ik_keeps_solving_a_held_entry_above_the_limit() {
        let mut e = engine();
        let (this, log, _) = grab_setup(&mut e);
        let entry = this + 0x1b0;
        set_entry(&mut e, entry, 0, 0.0, 0.8, [1, 1]);
        e.call(0x00c751f0, &args![this, 0x1234u32]);
        assert_eq!(e.mem.f32(entry + 8), 0.4);
        assert_eq!(calls_to(&log, 0x00cd_bae0).len(), 1);
        assert_eq!(e.mem.u8(entry + 0xd), 1);
        assert_eq!(e.mem.u8(this + 0x21e), 0x55);
    }

    #[test]
    fn do_grab_ik_only_steps_the_active_entries() {
        let mut e = engine();
        let (this, log, _) = grab_setup(&mut e);
        e.call(0x00c751f0, &args![this, 0x1234u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u8(this + 0x21e), 0);
        e.mem.set_u8(this + 0x21e, 0x66);
        e.mem.set_u8(this + 0x1ec, 1);
        let target = e.mem.alloc(0x100);
        set_entry(&mut e, this + 0x1e0, target, 0.0, 0.0, [1, 0]);
        e.call(0x00c751f0, &args![this, 0x1234u32]);
        let solves = calls_to(&log, 0x00cd_bae0);
        assert_eq!(solves.len(), 1);
        assert_eq!(e.mem.u8(this + 0x21e), 0x66);
        let transforms = calls_to(&log, 0x00c7_f840);
        assert_eq!(transforms[0][2], this + 0x1e0 + 0x10);
    }

    /// The bone lookup double: answers the position of the name in the six names, from 1.
    fn install_bone_lookup(e: &mut Engine, log: &CallLog, missing: Option<u32>) {
        let names = [
            0x0102_03c4u32,
            0x010c_4c50,
            0x010c_4c40,
            0x0102_03a0,
            0x010c_4c30,
            0x010c_4c20,
        ];
        let kept = Rc::clone(log);
        e.register_double(0x00cd_d390, move |_, a| {
            kept.borrow_mut().push((0x00cd_d390, a.to_vec()));
            let position = names.iter().position(|n| *n == a[1]).unwrap() as u32;
            Ret {
                eax: if missing == Some(position) {
                    0xffff_ffff
                } else {
                    position + 1
                },
                ..Ret::default()
            }
        });
    }

    fn grab_ik_controller(e: &mut Engine, flag: u8) -> u32 {
        let this = e.mem.alloc(0x300);
        let owner = e.mem.alloc(0x20);
        let skeleton = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u32(owner + 0xc, skeleton);
        e.mem.set_u8(this + 0x42, flag);
        for entry in [this + 0x1b0, this + 0x1e0] {
            set_entry(e, entry, 0x99, 1.5, 2.5, [1, 1]);
        }
        this
    }

    #[test]
    fn init_grab_ik_looks_the_six_bones_up_and_clears_the_entries() {
        let mut e = engine();
        let log = CallLog::default();
        install_bone_lookup(&mut e, &log, None);
        let this = grab_ik_controller(&mut e, 1);
        let skeleton = e.mem.u32(e.mem.u32(this + 0x2a4) + 0xc);
        assert_eq!(e.call(0x00c75460, &args![this]).u32() as u8, 1);
        let names: Vec<u32> = calls_to(&log, 0x00cd_d390).iter().map(|c| c[1]).collect();
        assert_eq!(
            names,
            vec![
                0x0102_03c4,
                0x010c_4c50,
                0x010c_4c40,
                0x0102_03a0,
                0x010c_4c30,
                0x010c_4c20
            ]
        );
        assert!(calls_to(&log, 0x00cd_d390)
            .iter()
            .all(|c| c[0] == skeleton && c[2] == 0));
        let bones: Vec<u16> = (0..6).map(|i| e.mem.u16(this + 0x210 + 2 * i)).collect();
        assert_eq!(bones, vec![1, 2, 3, 4, 5, 6]);
        for entry in [this + 0x1b0, this + 0x1e0] {
            assert_eq!(e.mem.u32(entry), 0);
            assert_eq!((e.mem.f32(entry + 4), e.mem.f32(entry + 8)), (0.0, 0.0));
            assert_eq!(e.mem.u8(entry + 0xc), 0);
            assert_eq!(e.mem.u8(entry + 0xd), 1);
        }
    }

    #[test]
    fn init_grab_ik_clears_the_flag_when_a_bone_is_missing_in_either_group() {
        for missing in 0..6 {
            let mut e = engine();
            let log = CallLog::default();
            install_bone_lookup(&mut e, &log, Some(missing));
            let this = grab_ik_controller(&mut e, 1);
            assert_eq!(e.call(0x00c75460, &args![this]).u32() as u8, 0, "{missing}");
            assert_eq!(e.mem.u16(this + 0x210 + 2 * missing), 0xffff);
        }
    }

    #[test]
    fn init_grab_ik_without_the_flag_or_a_skeleton_skips_the_lookups() {
        let mut e = engine();
        let log = CallLog::default();
        install_bone_lookup(&mut e, &log, None);
        let this = grab_ik_controller(&mut e, 0);
        assert_eq!(e.call(0x00c75460, &args![this]).u32() as u8, 0);
        e.mem.set_u8(this + 0x42, 3);
        let owner = e.mem.u32(this + 0x2a4);
        e.mem.set_u32(owner + 0xc, 0);
        assert_eq!(e.call(0x00c75460, &args![this]).u32() as u8, 3);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u8(this + 0x1bc), 0);
    }

    #[test]
    fn get_eye_offset_transforms_the_eye_vector_by_the_bone_and_the_root() {
        let mut e = engine();
        let log = CallLog::default();
        let eye = e.mem.alloc(0x10);
        put_floats(&mut e, eye, &[1.0, 2.0, 3.0, 4.0]);
        recorder(&mut e, &log, 0x00cd_a8e0, eye);
        recorder(&mut e, &log, 0x00cd_a180, 0xb0e);
        recorder(&mut e, &log, POINT_COPY, 0);
        let copies = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&copies);
        e.register_double(0x00c7_f7a0, move |e, a| {
            kept.borrow_mut().push((a.to_vec(), floats(e, a[0], 4)));
            Ret::default()
        });
        let this = e.mem.alloc(0x200);
        let skeleton = e.mem.alloc(0x40);
        let flags = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xf0, 0x4242);
        e.mem.set_u32(this + 0x88, skeleton);
        e.mem.set_u32(skeleton + 0x1c, flags);
        e.mem.set_u32(skeleton + 0x10, records);
        e.mem.set_u16(this + 0x144, 2);
        assert!(!e.call(0x00c757b0, &args![this, out]).bool());
        assert!(log.borrow().is_empty());
        e.mem.set_u8(this + 0x190, 1);
        assert!(e.call(0x00c757b0, &args![this, out]).bool());
        assert_eq!(calls_to(&log, 0x00cd_a8e0), vec![vec![0x4242, 1]]);
        assert!(calls_to(&log, 0x00cd_a180).is_empty());
        let copies = copies.borrow();
        let vector = copies[0].0[0];
        assert_eq!(copies[0].0[1..], [records + 0x60, vector]);
        assert_eq!(copies[0].1, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(copies[1].0[1..], [this + 0x10, vector]);
        assert_eq!(calls_to(&log, POINT_COPY), vec![vec![out, vector]]);
    }

    #[test]
    fn get_eye_offset_asks_the_skeleton_for_a_flagged_bone_and_sign_extends_the_index() {
        let mut e = engine();
        let log = CallLog::default();
        let eye = e.mem.alloc(0x10);
        recorder(&mut e, &log, 0x00cd_a8e0, eye);
        recorder(&mut e, &log, 0x00cd_a180, 0xb0e);
        recorder(&mut e, &log, POINT_COPY, 0);
        recorder(&mut e, &log, 0x00c7_f7a0, 0);
        let this = e.mem.alloc(0x200);
        let skeleton = e.mem.alloc(0x40);
        let flags = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x88, skeleton);
        e.mem.set_u32(skeleton + 0x1c, flags + 0x20);
        e.mem.set_u8(this + 0x190, 1);
        e.mem.set_i16(this + 0x144, -2);
        e.mem.set_u8(flags + 0x20 - 8, 2);
        assert!(e.call(0x00c757b0, &args![this, out]).bool());
        assert_eq!(
            calls_to(&log, 0x00cd_a180),
            vec![vec![skeleton, -2i32 as u32]]
        );
        assert_eq!(calls_to(&log, 0x00c7_f7a0)[0][1], 0xb0e);
    }

    /// An object (0x400 bytes) whose vtable (0x100 bytes) has the given `(slot byte offset,
    /// function)` entries.
    fn object_with_vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, function) in slots {
            e.mem.set_u32(vtable + slot, *function);
        }
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A controller whose constraint list holds one entry with a data object answering the
    /// type `kind` to slot 0x20; returns the controller and the data object, and counts the
    /// slot calls in `log`.
    fn powered_setup(e: &mut Engine, log: &CallLog, kind: u32) -> (u32, u32) {
        recorder(e, log, 0x0900_0020, kind);
        let data = object_with_vtable(e, &[(0x20, 0x0900_0020)]);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 0xc, data);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, entry);
        let list = e.mem.alloc(0x40);
        e.mem.set_u32(list + 0x14, array);
        e.mem.set_i32(list + 0x18, 1);
        let this = e.mem.alloc(0x300);
        e.mem.set_u32(this + 0x48, list);
        (this, data)
    }

    #[test]
    fn is_powered_ragdoll_needs_a_list_and_checks_every_constraint_type() {
        let mut e = engine();
        let log = CallLog::default();
        let this = e.mem.alloc(0x300);
        assert!(!e.call(0x00c75850, &args![this]).bool());
        let list = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x48, list);
        assert!(e.call(0x00c75850, &args![this]).bool());
        for (kind, passes, calls) in [(1, true, 3), (0xd, false, 1), (0xc, false, 2), (9, true, 3)]
        {
            log.borrow_mut().clear();
            let (this, _) = powered_setup(&mut e, &log, kind);
            assert_eq!(e.call(0x00c75850, &args![this]).bool(), passes, "{kind}");
            assert_eq!(calls_to(&log, 0x0900_0020).len(), calls, "{kind}");
        }
    }

    #[test]
    fn is_powered_ragdoll_type_two_needs_the_dword_at_0xb0() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, data) = powered_setup(&mut e, &log, 2);
        assert!(!e.call(0x00c75850, &args![this]).bool());
        e.mem.set_u32(data + 0xb0, 1);
        assert!(e.call(0x00c75850, &args![this]).bool());
    }

    #[test]
    fn is_powered_ragdoll_type_seven_needs_the_three_getters_in_turn() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, data) = powered_setup(&mut e, &log, 7);
        for getter in [0x00cd_ea20, 0x00cd_ea80, 0x00cd_eaa0] {
            recorder(&mut e, &log, getter, 1);
        }
        assert!(e.call(0x00c75850, &args![this]).bool());
        assert_eq!(calls_to(&log, 0x00cd_eaa0), vec![vec![data]]);
        for (zero, ran) in [(0x00cd_ea20, 1), (0x00cd_ea80, 2), (0x00cd_eaa0, 3)] {
            log.borrow_mut().clear();
            recorder(&mut e, &log, zero, 0);
            assert!(!e.call(0x00c75850, &args![this]).bool());
            let getters = [0x00cd_ea20, 0x00cd_ea80, 0x00cd_eaa0];
            let seen = getters
                .iter()
                .filter(|g| !calls_to(&log, **g).is_empty())
                .count();
            assert_eq!(seen, ran);
            recorder(&mut e, &log, zero, 1);
        }
    }

    /// A controller with a settings block, a target record and the float globals the two
    /// settings functions read.
    fn settings_setup(e: &mut Engine) -> (u32, u32, u32) {
        e.set_global(0x0102_90d0, 2.0f64);
        e.set_global(0x0101_ffa0, 3.0f64);
        e.set_global(0x0101_2060, 0.5f64);
        e.set_global(0x0102_90b0, 10.0f64);
        e.set_global(0x0109_9378, 4294967296.0f32);
        let this = e.mem.alloc(0x300);
        let owner = e.mem.alloc(0x40);
        let settings = e.mem.alloc(0x60);
        let target = e.mem.alloc(0x40);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u32(owner + 0x18, settings);
        e.mem.set_u32(this + 0x4c, holder);
        e.mem.set_u32(holder, target);
        put_floats(e, settings + 0xc, &[2.0, 1.5]);
        e.mem.set_f32(settings + 0x18, 4.0);
        e.mem.set_u32(settings + 0x38, 7);
        e.mem.set_u32(settings + 0x3c, 0xffff_fff0);
        (this, settings, target)
    }

    #[test]
    fn fn_00c75910_scales_three_floats_and_the_two_counters() {
        let mut e = engine();
        let (this, settings, target) = settings_setup(&mut e);
        e.call(0x00c75910, &args![this]);
        assert_eq!(e.mem.f32(target + 0xc), 3.0);
        assert_eq!(e.mem.f32(target + 0x10), 6.0);
        assert_eq!(e.mem.f32(target + 0x1c), 2.0);
        assert_eq!(e.mem.u32(this + 0x29c), 70);
        assert_eq!(e.mem.u32(this + 0x298), 0xffff_ff60);
        e.set_global(0x0102_90b0, 1.0e19f64);
        e.call(0x00c75910, &args![this]);
        assert_eq!(e.mem.u32(this + 0x29c), 0);
        let owner = e.mem.u32(this + 0x2a4);
        e.mem.set_u32(owner + 0x18, 0);
        e.mem.set_u32(this + 0x29c, 5);
        e.call(0x00c75910, &args![this]);
        assert_eq!(e.mem.u32(this + 0x29c), 5);
        assert_eq!(e.mem.f32(settings + 0xc), 2.0);
    }

    #[test]
    fn fn_00c759f0_copies_the_floats_quietly_and_scales_the_counters() {
        let mut e = engine();
        let (this, settings, target) = settings_setup(&mut e);
        e.mem.set_u32(settings + 0x10, 0x7f80_0001);
        e.mem.set_f32(settings + 4, 9.0);
        e.call(0x00c759f0, &args![this]);
        assert_eq!(e.mem.u32(target + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.f32(target + 0x10), 2.0);
        assert_eq!(e.mem.f32(target + 0x1c), 4.0);
        assert_eq!(e.mem.f32(this + 0x290), 9.0);
        assert_eq!(e.mem.u32(this + 0x29c), 70);
        assert_eq!(e.mem.u32(this + 0x298), 0xffff_ff60);
        let owner = e.mem.u32(this + 0x2a4);
        e.mem.set_u32(owner + 0x18, 0);
        e.mem.set_f32(this + 0x290, 1.0);
        e.call(0x00c759f0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x290), 1.0);
    }

    #[test]
    fn fn_00c75ad0_installs_the_settings_and_copies_twelve_floats() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        let owner = e.mem.alloc(0x40);
        let settings = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u32(this + 0x4c, holder);
        e.mem.set_u32(holder, target);
        e.mem.set_u8(settings, 0x5a);
        for offset in (4..0x38).step_by(4) {
            e.mem.set_f32(settings + offset, offset as f32);
        }
        e.mem.set_u32(settings + 0x1c, 0x7f80_0001);
        e.call(0x00c75ad0, &args![this, settings]);
        assert_eq!(e.mem.u32(owner + 0x18), settings);
        assert_eq!((e.mem.u8(this + 0x28c), e.mem.u8(this + 0x28d)), (1, 0x5a));
        let copied = floats(&e, target, 12);
        assert_eq!(copied[0], 8.0);
        assert_eq!(e.mem.u32(target + 4), 0x7fc0_0001);
        assert_eq!(copied[2], 20.0);
        assert_eq!(copied[3], 16.0);
        assert_eq!(copied[4], 12.0);
        assert_eq!(copied[5], 48.0);
        assert_eq!(copied[6], 52.0);
        assert_eq!(copied[7], 24.0);
        assert_eq!(copied[8..], [32.0, 36.0, 40.0, 44.0]);
    }

    #[test]
    fn fn_00c75b40_converts_each_bone_transform_and_scales_its_point() {
        let mut e = engine();
        let log = CallLog::default();
        install_sqrt(&mut e, &log);
        let this = e.mem.alloc(0x300);
        let source = e.mem.alloc(0xa0);
        let owner = e.mem.alloc(0x20);
        let skeleton = e.mem.alloc(0x40);
        let transforms = e.mem.alloc(0x60);
        let table = e.mem.alloc(0x10);
        let bones = [e.mem.alloc(0x80), e.mem.alloc(0x80)];
        e.mem.set_f32(source + 0x98, 2.0);
        e.mem.set_u32(this + 0x58, source);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u32(owner + 0xc, skeleton);
        e.mem.set_i32(skeleton + 0x1c, 2);
        e.mem.set_u32(this + 0x94, transforms);
        e.mem.set_u32(this + 0xa4, table);
        e.mem.set_u32(table, bones[0]);
        e.mem.set_u32(table + 4, bones[1]);
        e.mem.set_f32(transforms + 0x1c, 1.0);
        e.mem.set_f32(transforms + 0x4c, 1.0);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&calls);
        e.register_double(0x004f_0180, move |e, a| {
            let call = kept.borrow().len() as u32;
            kept.borrow_mut().push(a.to_vec());
            for word in 0..9 {
                e.mem.set_u32(a[1] + 4 * word, 0x1000 * (call + 1) + word);
            }
            Ret::default()
        });
        e.register_double(POINT_COPY, |e, a| {
            put_floats(e, a[0], &[4.0, 6.0, 8.0]);
            Ret::default()
        });
        e.call(0x00c75b40, &args![this]);
        assert_eq!(calls.borrow().len(), 2);
        for (index, bone) in bones.into_iter().enumerate() {
            assert_eq!(floats(&e, bone + 0x58, 3), vec![2.0, 3.0, 4.0]);
            let words: Vec<u32> = (0..9).map(|i| e.mem.u32(bone + 0x34 + 4 * i)).collect();
            let want: Vec<u32> = (0..9).map(|i| 0x1000 * (index as u32 + 1) + i).collect();
            assert_eq!(words, want);
        }
    }

    #[test]
    fn reset_pose_copies_the_element_data_and_applies_the_motion_and_flag() {
        let mut e = engine();
        let log = CallLog::default();
        for address in [0x00a5_9c60, 0x00c6_a350, 0x00c8_0330] {
            recorder(&mut e, &log, address, 0);
        }
        let objects = [e.mem.alloc(0x80), e.mem.alloc(0x80)];
        let elements = e.mem.alloc(0x38 * 2);
        for (index, object) in objects.into_iter().enumerate() {
            let element = elements + 0x38 * index as u32;
            e.mem.set_u32(element, object);
            for word in 0..9 {
                e.mem
                    .set_u32(element + 4 + 4 * word, 0x100 * (index as u32 + 1) + word);
            }
            for word in 0..3 {
                e.mem.set_u32(
                    element + 0x28 + 4 * word,
                    0x900 + 0x10 * index as u32 + word,
                );
            }
        }
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, elements);
        e.mem.set_u32(array + 8, 2);
        let world = 0x7777u32;
        e.call(0x00c75c00, &args![world, array, 0x100u32, 4u32]);
        for (index, object) in objects.into_iter().enumerate() {
            let index = index as u32;
            assert_eq!(e.mem.u32(object + 0x34), 0x100 * (index + 1));
            assert_eq!(e.mem.u32(object + 0x34 + 32), 0x100 * (index + 1) + 8);
            assert_eq!(e.mem.u32(object + 0x58), 0x900 + 0x10 * index);
            assert_eq!(e.mem.u32(object + 0x60), 0x902 + 0x10 * index);
        }
        let record = calls_to(&log, 0x00a5_9c60);
        assert_eq!((record.len(), record[0][0]), (1, world));
        assert!(calls_to(&log, 0x00c6_a350).is_empty());
        assert!(calls_to(&log, 0x00c8_0330).is_empty());
        e.call(0x00c75c00, &args![world, array, 0x101u32, 2u32]);
        assert_eq!(calls_to(&log, 0x00c6_a350), vec![vec![world, 2, 1, 0, 1]]);
        assert_eq!(calls_to(&log, 0x00c8_0330), vec![vec![0x101, world]]);
        e.call(0x00c75c00, &args![world, array, 0u32, 0u32]);
        assert_eq!(calls_to(&log, 0x00c6_a350).len(), 1);
    }

    /// Doubles for what the map destructors call, logged.
    fn map_doubles(e: &mut Engine, log: &CallLog) {
        recorder(e, log, MAP_REMOVE_ALL, 0);
        recorder(e, log, HEAP_FREE, 0);
        recorder(e, log, MEMORY_DEALLOCATE, 0);
    }

    #[test]
    fn map_destructor_bodies_store_their_vtable_empty_the_map_and_free_the_storage() {
        for (address, vtable) in [
            (0x00c7_5cc0, 0x010c_4c64),
            (0x00c7_5e00, 0x010c_4ca4),
            (0x00c7_5f80, 0x010c_4cf8),
        ] {
            let mut e = engine();
            let log = CallLog::default();
            map_doubles(&mut e, &log);
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 8, 0x1234);
            e.call(address, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![this]]);
            assert_eq!(calls_to(&log, HEAP_FREE), vec![vec![0x1234]]);
        }
    }

    #[test]
    fn derived_map_destructors_run_both_levels() {
        for (address, derived, base) in [
            (0x00c7_5d90, 0x010c_4c84, 0x010c_4c64),
            (0x00c7_5ed0, 0x010c_4cc4, 0x010c_4ca4),
            (0x00c7_5fa0, 0x010c_4d18, 0x010c_4cf8),
        ] {
            let mut e = engine();
            let seen = Rc::new(RefCell::new(Vec::new()));
            let kept = Rc::clone(&seen);
            e.register_double(MAP_REMOVE_ALL, move |e, a| {
                kept.borrow_mut().push(e.mem.u32(a[0]));
                Ret::default()
            });
            let log = CallLog::default();
            recorder(&mut e, &log, HEAP_FREE, 0);
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 8, 0x4321);
            e.call(address, &args![this]);
            assert_eq!(*seen.borrow(), vec![derived, base]);
            assert_eq!(e.mem.u32(this), base);
            assert_eq!(calls_to(&log, HEAP_FREE), vec![vec![0x4321]]);
        }
    }

    #[test]
    fn map_scalar_deleting_destructors_delete_only_on_bit_zero() {
        for (address, vtable) in [
            (0x00c7_6030, 0x010c_4c64),
            (0x00c7_6070, 0x010c_4ca4),
            (0x00c7_60b0, 0x010c_4cf8),
        ] {
            let mut e = engine();
            let log = CallLog::default();
            map_doubles(&mut e, &log);
            let this = e.mem.alloc(0x20);
            assert_eq!(e.call(address, &args![this, 0xfeu32]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert!(calls_to(&log, MEMORY_DEALLOCATE).is_empty());
            assert_eq!(e.call(address, &args![this, 1u32]).u32(), this);
            assert_eq!(
                calls_to(&log, MEMORY_DEALLOCATE),
                vec![vec![MEMORY_MANAGER, this]]
            );
        }
    }

    /// Doubles for the reference counting the item functions do: `decrement` answers the
    /// new count, slot 4 of any object logs a destroy.
    fn reference_setup(e: &mut Engine, log: &CallLog, decrement: u32) -> (u32, u32, u32) {
        recorder(e, log, INTERLOCKED_INCREMENT, 1);
        recorder(e, log, INTERLOCKED_DECREMENT, decrement);
        recorder(e, log, 0x0900_0004, 0);
        let item = e.mem.alloc(0x10);
        let old = object_with_vtable(e, &[(4, 0x0900_0004)]);
        let new = object_with_vtable(e, &[(4, 0x0900_0004)]);
        (item, old, new)
    }

    #[test]
    fn item_pointer_setters_swap_the_reference_and_drop_the_callers_extra_one() {
        for address in [0x00c7_5ce0, 0x00c7_5e20] {
            let mut e = engine();
            let log = CallLog::default();
            let (item, old, new) = reference_setup(&mut e, &log, 0);
            e.mem.set_u32(item + 8, old);
            e.call(address, &args![VCALL_THIS, item, 0x42u32, new]);
            assert_eq!((e.mem.u32(item + 4), e.mem.u32(item + 8)), (0x42, new));
            assert_eq!(
                calls_to(&log, INTERLOCKED_DECREMENT),
                vec![vec![old + 4], vec![new + 4]]
            );
            assert_eq!(calls_to(&log, INTERLOCKED_INCREMENT), vec![vec![new + 4]]);
            assert_eq!(calls_to(&log, 0x0900_0004), vec![vec![old], vec![new]]);
        }
    }

    #[test]
    fn item_pointer_setter_without_a_change_only_drops_the_extra_reference() {
        let mut e = engine();
        let log = CallLog::default();
        let (item, _, new) = reference_setup(&mut e, &log, 5);
        e.mem.set_u32(item + 8, new);
        e.call(0x00c7_5ce0, &args![VCALL_THIS, item, 7u32, new]);
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT), vec![vec![new + 4]]);
        assert!(calls_to(&log, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to(&log, 0x0900_0004).is_empty());
        assert_eq!(e.mem.u32(item + 4), 7);
    }

    #[test]
    fn item_pointer_setter_accepts_null_old_and_new_values() {
        let mut e = engine();
        let log = CallLog::default();
        let (item, old, _) = reference_setup(&mut e, &log, 1);
        e.mem.set_u32(item + 8, old);
        e.call(0x00c7_5ce0, &args![VCALL_THIS, item, 1u32, 0u32]);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT), vec![vec![old + 4]]);
        assert!(calls_to(&log, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to(&log, 0x0900_0004).is_empty());
    }

    #[test]
    fn small_stdcall_helpers_store_and_allocate() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, MEMORY_ALLOCATE, 0x3333);
        recorder(&mut e, &log, MEMORY_DEALLOCATE, 0);
        assert_eq!(e.call(0x00c7_5f40, &args![0x5555u32, 5u32]).u32(), 0x3333);
        assert_eq!(
            calls_to(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 20]]
        );
        let item = e.mem.alloc(0x10);
        e.call(
            0x00c7_5f60,
            &args![VCALL_THIS, item, 0xaabbu32, 0x1_2345u32],
        );
        assert_eq!((e.mem.u32(item + 4), e.mem.u16(item + 8)), (0xaabb, 0x2345));
        e.call(0x00c7_6010, &args![VCALL_THIS, item]);
        assert_eq!(e.mem.u16(item + 8), 0);
        assert_eq!(
            calls_to(&log, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, item]]
        );
    }

    #[test]
    fn fn_00c760f0_constructs_each_element() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, 0x00a8_6bf0, 0);
        let base = e.mem.alloc(0x38 * 3);
        for index in 0..3 {
            e.mem.set_u32(base + 0x38 * index, 0xdead);
        }
        e.call(0x00c7_60f0, &args![base, 3u32]);
        assert_eq!(
            calls_to(&log, 0x00a8_6bf0),
            vec![vec![base + 4], vec![base + 0x3c], vec![base + 0x74]]
        );
        assert_eq!(e.mem.u32(base + 0x38), 0);
        log.borrow_mut().clear();
        e.call(0x00c7_60f0, &args![base, 0u32]);
        assert!(log.borrow().is_empty());
    }

    /// An array object whose allocator slots log: 4 hands out `fresh`, 8 frees, 0xc resizes
    /// and answers `resized`.
    fn array_setup(e: &mut Engine, log: &CallLog, fresh: u32, resized: u32) -> u32 {
        recorder(e, log, 0x0900_0004, fresh);
        recorder(e, log, 0x0900_0008, 0);
        recorder(e, log, 0x0900_000c, resized);
        object_with_vtable(e, &[(4, 0x0900_0004), (8, 0x0900_0008), (0xc, 0x0900_000c)])
    }

    #[test]
    fn fn_00c76180_allocates_resizes_or_moves_the_elements() {
        let mut e = engine();
        let log = CallLog::default();
        let moves = install_memmove(&mut e);
        let array = array_setup(&mut e, &log, 0x5000, 0x6000);
        e.call(0x00c7_6180, &args![array, 4u32, 0u32]);
        assert_eq!((e.mem.u32(array + 4), e.mem.u32(array + 0xc)), (0x5000, 4));
        assert_eq!(calls_to(&log, 0x0900_0004), vec![vec![array, 4]]);
        e.call(0x00c7_6180, &args![array, 8u32, 4u32]);
        assert_eq!(e.mem.u32(array + 4), 0x6000);
        assert_eq!(calls_to(&log, 0x0900_000c), vec![vec![array, 0x5000, 8]]);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        let old = e.mem.alloc(0x38 * 2);
        let fresh = e.mem.alloc(0x38 * 2);
        recorder(&mut e, &log, 0x0900_0004, fresh);
        e.mem.set_u32(array + 4, old);
        e.mem.set_u32(old + 0x38, 0x77);
        e.call(0x00c7_6180, &args![array, 8u32, 2u32]);
        assert_eq!(e.mem.u32(array + 4), fresh);
        assert_eq!(e.mem.u32(fresh + 0x38), 0x77);
        assert_eq!(calls_to(&log, 0x0900_0008), vec![vec![array, old]]);
        assert_eq!(calls_to(&moves, MEMMOVE).len(), 2);
    }

    #[test]
    fn fn_00c761f0_flags_the_kinds_four_and_five() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let out = e.mem.alloc(4);
        for (kind, want) in [(3, 0), (4, 1), (5, 1), (6, 0)] {
            e.mem.set_u8(this + 0xe8, kind);
            e.mem.set_u8(out, 9);
            e.call(0x00c7_61f0, &args![this, out]);
            assert_eq!(e.mem.u8(out), want, "{kind}");
        }
    }

    #[test]
    fn fn_00c76220_and_fn_00c762f0_index_through_the_table_or_answer_zero() {
        let mut e = engine();
        let list = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x20);
        let entries = e.mem.alloc(0x20);
        e.mem.set_u32(list + 0x20, table);
        e.mem.set_u32(list + 8, entries);
        e.mem.set_i32(table, 2);
        e.mem.set_i32(table + 4, -1);
        e.mem.set_u32(entries + 8, 0x5150);
        assert_eq!(e.call(0x00c7_6220, &args![list, 0u32]).u32(), 0x5150);
        assert_eq!(e.call(0x00c7_6220, &args![list, 1u32]).u32(), 0);
        let this = e.mem.alloc(0x300);
        assert_eq!(e.call(0x00c7_62f0, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x48, list);
        assert_eq!(e.call(0x00c7_62f0, &args![this]).u32(), 0x5150);
        e.mem.set_u32(this + 0x2b8, 1);
        assert_eq!(e.call(0x00c7_62f0, &args![this]).u32(), 0);
    }

    #[test]
    fn fn_00c76320_follows_the_chain_to_the_dword_at_0x350() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        let (list, entries, first, inner) = (
            e.mem.alloc(0x20),
            e.mem.alloc(0x20),
            e.mem.alloc(0x20),
            e.mem.alloc(0x400),
        );
        e.mem.set_u32(inner + 0x350, 0x1357);
        assert_eq!(e.call(0x00c7_6320, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x48, list);
        e.mem.set_u32(list + 8, entries);
        assert_eq!(e.call(0x00c7_6320, &args![this]).u32(), 0);
        e.mem.set_u32(entries, first);
        assert_eq!(e.call(0x00c7_6320, &args![this]).u32(), 0);
        e.mem.set_u32(first + 8, inner);
        assert_eq!(e.call(0x00c7_6320, &args![this]).u32(), 0x1357);
    }

    #[test]
    fn share_data_destructor_releases_what_it_holds_and_runs_the_base() {
        let mut e = engine();
        let log = CallLog::default();
        for address in [0x00c9_05b0, 0x00a5_d3d0, MEMORY_DEALLOCATE] {
            recorder(&mut e, &log, address, 0);
        }
        recorder(&mut e, &log, 0x0900_0000, 0);
        let this = e.mem.alloc(0x40);
        let deleted = object_with_vtable(&mut e, &[(0, 0x0900_0000)]);
        let holder_object = object_with_vtable(&mut e, &[(0, 0x0900_0000)]);
        let block = e.mem.alloc(0x40);
        e.mem.set_u8(block + 0xf, 9);
        e.mem.set_u32(this + 8, 0x11);
        e.mem.set_u32(this + 0xc, deleted);
        e.mem.set_u32(this + 0x10, 0x22);
        e.mem.set_u32(this + 0x14, 0x33);
        e.mem.set_u32(block + 0x10, holder_object);
        e.mem.set_u32(this + 0x1c, block + 0x10);
        e.call(0x00c7_6240, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010c_4b74);
        assert_eq!(
            calls_to(&log, 0x00c9_05b0),
            vec![vec![0x11], vec![0x22], vec![0x33]]
        );
        assert_eq!(
            calls_to(&log, 0x0900_0000),
            vec![vec![deleted, 1], vec![holder_object, 1]]
        );
        assert_eq!(
            calls_to(&log, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block + 0x10 - 9]]
        );
        assert_eq!(calls_to(&log, 0x00a5_d3d0), vec![vec![this]]);
    }

    #[test]
    fn fn_00c78090_finds_a_record_with_an_enabled_bone() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        let table = e.mem.alloc(0x40);
        let bones: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x40)).collect();
        for (index, bone) in bones.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *bone);
            e.mem.set_u32(bone + 0x30, 2);
        }
        let records = e.mem.alloc(0xd0 * 2);
        let blocks = [e.mem.alloc(0x40), e.mem.alloc(0x40)];
        e.mem.set_u32(records, blocks[0]);
        e.mem.set_u32(records + 0xd0, blocks[1]);
        for block in blocks {
            e.mem.set_i16(block + 0x18, 0);
            e.mem.set_i16(block + 0x16, 1);
            e.mem.set_i16(block + 0x14, 2);
        }
        e.mem.set_i16(blocks[0] + 0x16, 0);
        e.mem.set_i16(blocks[0] + 0x14, 0);
        e.mem.set_u32(this + 0xa4, table);
        e.mem.set_u32(this + 0xa8, 3);
        e.mem.set_u32(this + 0x22c, records);
        assert!(!e.call(0x00c78090, &args![this]).bool());
        e.mem.set_i32(this + 0x230, 2);
        assert!(!e.call(0x00c78090, &args![this]).bool());
        e.mem.set_u32(bones[1] + 0x30, 0);
        e.mem.set_i16(blocks[1] + 0x16, 5);
        assert!(!e.call(0x00c78090, &args![this]).bool());
        e.mem.set_i16(blocks[1] + 0x16, 1);
        assert!(e.call(0x00c78090, &args![this]).bool());
        e.mem.set_u32(bones[1] + 0x30, 2);
        e.mem.set_u32(bones[2] + 0x30, 0x10);
        assert!(e.call(0x00c78090, &args![this]).bool());
        e.mem.set_i16(blocks[0] + 0x14, -1);
        e.mem.set_i16(blocks[1] + 0x14, -1);
        assert!(!e.call(0x00c78090, &args![this]).bool());
    }

    #[test]
    fn fn_00c79d60_sets_the_flag_bits_and_notifies_the_handler() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, 0x0900_00dc, 0);
        recorder(&mut e, &log, 0x0900_00e8, 0);
        let handler = object_with_vtable(&mut e, &[(0xdc, 0x0900_00dc), (0xe8, 0x0900_00e8)]);
        recorder(&mut e, &log, 0x0900_000c, handler);
        let owner = object_with_vtable(&mut e, &[(0xc, 0x0900_000c)]);
        let this = e.mem.alloc(0x300);
        let object = e.mem.alloc(0x100);
        let entries = e.mem.alloc(0x10);
        let flagged: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x40)).collect();
        for (index, entry) in flagged.iter().enumerate() {
            e.mem.set_u32(entries + 4 * index as u32, *entry);
            e.mem.set_u32(entry + 0x30, 0x10 | 1);
        }
        e.mem.set_u32(object + 0xa0, entries);
        e.mem.set_u32(this + 0x84, object);
        e.mem.set_u32(this + 0x58, owner);
        e.call(0x00c79d60, &args![this, 1u32, 0u32, 1u32]);
        let words: Vec<u32> = flagged.iter().map(|f| e.mem.u32(f + 0x30)).collect();
        assert_eq!(words, vec![0x10, 0x11, 0x10]);
        assert_eq!(calls_to(&log, 0x0900_00dc), vec![vec![handler, object, 1]]);
        assert!(calls_to(&log, 0x0900_00e8).is_empty());
    }

    #[test]
    fn fn_00c79d60_with_everything_zero_calls_slot_0xe8_and_without_a_handler_nothing() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, 0x0900_00dc, 0);
        recorder(&mut e, &log, 0x0900_00e8, 0);
        let handler = object_with_vtable(&mut e, &[(0xdc, 0x0900_00dc), (0xe8, 0x0900_00e8)]);
        let result = Rc::new(std::cell::Cell::new(handler));
        let given = Rc::clone(&result);
        e.register_double(0x0900_000c, move |_, _| Ret {
            eax: given.get(),
            ..Ret::default()
        });
        let owner = object_with_vtable(&mut e, &[(0xc, 0x0900_000c)]);
        let this = e.mem.alloc(0x300);
        let object = e.mem.alloc(0x100);
        let entries = e.mem.alloc(0x10);
        for index in 0..3 {
            let entry = e.mem.alloc(0x40);
            e.mem.set_u32(entries + 4 * index, entry);
        }
        e.mem.set_u32(object + 0xa0, entries);
        e.mem.set_u32(this + 0x84, object);
        e.mem.set_u32(this + 0x58, owner);
        e.call(0x00c79d60, &args![this, 0u32, 0u32, 0u32]);
        assert_eq!(calls_to(&log, 0x0900_00e8), vec![vec![handler, object]]);
        assert_eq!(e.mem.u32(e.mem.u32(entries) + 0x30), 1);
        e.mem.set_u32(object + 0x18, 5);
        e.call(0x00c79d60, &args![this, 0u32, 1u32, 0u32]);
        assert!(calls_to(&log, 0x0900_00dc).is_empty());
        result.set(0);
        e.call(0x00c79d60, &args![this, 1u32, 1u32, 1u32]);
        assert!(calls_to(&log, 0x0900_00dc).is_empty());
        assert_eq!(calls_to(&log, 0x0900_00e8).len(), 1);
    }

    /// A constraint list for `00c7a230`/`00c7a8d0`: two entries, the first holder object, a world
    /// object whose `+0x350` is `0x9999`. Returns the controller and the entries.
    fn world_setup(e: &mut Engine, log: &CallLog, holder_set: bool) -> (u32, [u32; 2]) {
        recorder(e, log, 0x00c6_b320, 0);
        recorder(e, log, 0x00c6_8d50, 0);
        let this = e.mem.alloc(0x300);
        let list = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x10);
        let entries = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        e.mem.set_u32(array, entries[0]);
        e.mem.set_u32(array + 4, entries[1]);
        if holder_set {
            e.mem.set_u32(entries[0] + 8, 0x4000);
        }
        e.mem.set_u32(list + 0x14, array);
        e.mem.set_i32(list + 0x18, 2);
        let worlds = e.mem.alloc(0x10);
        let world = e.mem.alloc(0x20);
        let world_object = e.mem.alloc(0x400);
        e.mem.set_u32(world_object + 0x350, 0x9999);
        e.mem.set_u32(world + 8, world_object);
        e.mem.set_u32(worlds, world);
        e.mem.set_u32(list + 8, worlds);
        e.mem.set_u32(this + 0x48, list);
        (this, entries)
    }

    #[test]
    fn update_constraints_world_adds_unheld_entries_or_removes_held_ones() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, entries) = world_setup(&mut e, &log, false);
        e.call(0x00c7a230, &args![this, 1u32]);
        assert_eq!(
            calls_to(&log, 0x00c6_b320),
            vec![vec![0x9999, entries[0]], vec![0x9999, entries[1]]]
        );
        assert!(calls_to(&log, 0x00c6_8d50).is_empty());
        log.borrow_mut().clear();
        e.call(0x00c7a230, &args![this, 0u32]);
        assert!(log.borrow().is_empty());
        let (this, entries) = world_setup(&mut e, &log, true);
        e.call(0x00c7a230, &args![this, 1u32]);
        assert!(log.borrow().is_empty());
        e.call(0x00c7a230, &args![this, 0u32]);
        assert_eq!(calls_to(&log, 0x00c6_8d50), vec![vec![0x9999, entries[0]]]);
    }

    #[test]
    fn update_constraints_world_needs_a_world() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, _) = world_setup(&mut e, &log, false);
        let list = e.mem.u32(this + 0x48);
        let worlds = e.mem.u32(list + 8);
        let world = e.mem.u32(worlds);
        e.mem.set_u32(world + 8, 0);
        e.call(0x00c7a230, &args![this, 1u32]);
        e.mem.set_u32(worlds, 0);
        e.call(0x00c7a230, &args![this, 1u32]);
        assert!(log.borrow().is_empty());
    }

    /// A motor controller: `kinds` constraint data objects (each answering its kind to slot
    /// 0x20 through its own double), entries 1.. mapped by the table; returns the controller,
    /// the entries and the data objects.
    fn motor_setup(e: &mut Engine, log: &CallLog, kinds: &[u32]) -> (u32, Vec<u32>, Vec<u32>) {
        let controller = e.mem.alloc(0x40);
        let motors = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x200);
        e.mem.set_u32(controller + 0x2c, motors);
        e.mem.set_i32(motors + 0x1c, kinds.len() as i32);
        e.mem.set_u32(controller + 0x20, table);
        e.mem.set_u32(controller + 0x14, array);
        let mut entries = Vec::new();
        let mut datas = Vec::new();
        for (index, kind) in kinds.iter().enumerate() {
            let double = 0x0900_1000 + index as u32;
            recorder(e, log, double, *kind);
            let data = object_with_vtable(e, &[(0x20, double)]);
            let entry = e.mem.alloc(0x20);
            e.mem.set_u32(entry + 0xc, data);
            e.mem.set_u32(array + 4 * index as u32, entry);
            e.mem.set_i32(table + 4 * index as u32, index as i32 + 1);
            entries.push(entry);
            datas.push(data);
        }
        (controller, entries, datas)
    }

    #[test]
    fn start_motors_runs_the_kind_two_and_seven_setups() {
        let mut e = engine();
        let log = CallLog::default();
        for address in [0x00cb_8fe0, 0x00cd_f290, 0x00cd_f230] {
            recorder(&mut e, &log, address, 0);
        }
        let (controller, entries, datas) = motor_setup(&mut e, &log, &[2, 7, 1]);
        e.mem.set_f32(datas[0] + 0xac, 5.0);
        e.call(0x00c7_a2d0, &args![controller]);
        assert_eq!(
            calls_to(&log, 0x00cb_8fe0),
            vec![vec![datas[0], entries[0], 1]]
        );
        assert_eq!(
            calls_to(&log, 0x00cd_f290),
            vec![vec![datas[1], entries[1], 1]]
        );
        assert_eq!(
            calls_to(&log, 0x00cd_f230),
            vec![vec![datas[1], 0x0126_8370]]
        );
        assert_eq!(e.mem.f32(datas[0] + 0xac), 0.0);
    }

    #[test]
    fn stop_motors_runs_the_kind_two_and_seven_calls_with_zero() {
        let mut e = engine();
        let log = CallLog::default();
        for address in [0x00cb_8fe0, 0x00cd_f290, 0x00cd_f230] {
            recorder(&mut e, &log, address, 0);
        }
        let (controller, entries, datas) = motor_setup(&mut e, &log, &[7, 2, 3]);
        e.mem.set_f32(datas[1] + 0xac, 5.0);
        e.call(0x00c7_a380, &args![controller]);
        assert_eq!(
            calls_to(&log, 0x00cd_f290),
            vec![vec![datas[0], entries[0], 0]]
        );
        assert_eq!(
            calls_to(&log, 0x00cb_8fe0),
            vec![vec![datas[1], entries[1], 0]]
        );
        assert!(calls_to(&log, 0x00cd_f230).is_empty());
        assert_eq!(e.mem.f32(datas[1] + 0xac), 5.0);
    }

    #[test]
    fn motors_follow_wrapped_data_and_skip_missing_entries() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, 0x00cb_8fe0, 0);
        let (controller, entries, datas) = motor_setup(&mut e, &log, &[0xd, 2, 2]);
        recorder(&mut e, &log, 0x0900_2000, 2);
        let wrapped = object_with_vtable(&mut e, &[(0x20, 0x0900_2000)]);
        recorder(&mut e, &log, WRAPPED_DATA, wrapped);
        let table = e.mem.u32(controller + 0x20);
        e.mem.set_i32(table + 4, 0);
        e.mem.set_i32(table + 8, 99);
        e.call(0x00c7_a2d0, &args![controller]);
        assert_eq!(calls_to(&log, WRAPPED_DATA), vec![vec![datas[0]]]);
        assert_eq!(
            calls_to(&log, 0x00cb_8fe0),
            vec![vec![wrapped, entries[0], 1]]
        );
    }

    #[test]
    fn drive_to_pose_decomposes_kind_two_and_targets_kind_seven() {
        let mut e = engine();
        let log = CallLog::default();
        for address in [0x00cb_26e0, 0x00c6_6320, 0x00cb_2d90, 0x00cd_f230] {
            recorder(&mut e, &log, address, 0);
        }
        let axes = Rc::new(RefCell::new(Vec::new()));
        e.register_double(0x00cb_2ca0, {
            let kept = Rc::clone(&log);
            let axes = Rc::clone(&axes);
            move |e, a| {
                kept.borrow_mut().push((0x00cb_2ca0, a.to_vec()));
                axes.borrow_mut().push(floats(e, a[1], 4));
                e.mem.set_f32(a[3], 0.75);
                Ret::default()
            }
        });
        let (controller, _, datas) = motor_setup(&mut e, &log, &[2, 7]);
        let pose = e.mem.alloc(0x100);
        e.mem.set_u32(datas[0] + 0xac, 0x7f80_0001);
        e.call(0x00c7_a410, &args![controller, pose]);
        assert_eq!(e.mem.f32(datas[0] + 0xac), 0.75);
        let quaternions = calls_to(&log, 0x00cb_26e0);
        assert_eq!(quaternions.len(), 2);
        let (first, second) = (quaternions[0][0], quaternions[1][0]);
        assert_eq!(
            (quaternions[0][1], quaternions[1][1]),
            (datas[0] + 0x20, datas[0] + 0x60)
        );
        let product = calls_to(&log, 0x00c6_6320);
        assert_eq!(product.len(), 1);
        assert_eq!(product[0][1..], [pose + 0x10, first]);
        let decompose = calls_to(&log, 0x00cb_2ca0);
        assert_eq!(decompose.len(), 1);
        assert_eq!(decompose[0][1], product[0][0] + 0x10);
        assert_eq!(second, first - 0x10);
        assert_eq!(*axes.borrow(), vec![vec![1.0, 0.0, 0.0, 0.0]]);
        let target = calls_to(&log, 0x00cb_2d90);
        assert_eq!(target.len(), 1);
        assert_eq!(target[0][1], pose + 0x40);
        assert_eq!(
            calls_to(&log, 0x00cd_f230),
            vec![vec![datas[1], target[0][0]]]
        );
    }

    #[test]
    fn deleting_destructors_of_the_maps_and_arrays_free_only_on_bit_zero() {
        let mut e = engine();
        let log = CallLog::default();
        for address in [MAP_REMOVE_ALL, HEAP_FREE, MEMORY_DEALLOCATE, 0x0900_00f0] {
            recorder(&mut e, &log, address, 0);
        }
        e.mem.set_u32(0x010c_4dc0, 0x0900_00f0);
        e.mem.set_u32(0x010c_4cec, 0x0900_00f0);
        for (address, vtable) in [
            (0x00c7_a570, 0x010c_4c64),
            (0x00c7_a590, 0x010c_4ca4),
            (0x00c7_a5b0, 0x010c_4cf8),
            (0x00c7_a690, 0x010c_4db8),
            (0x00c7_a6e0, 0x010c_4ce4),
        ] {
            log.borrow_mut().clear();
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 4, 0);
            assert_eq!(e.call(address, &args![this, 0xfeu32]).u32(), this);
            assert!(calls_to(&log, MEMORY_DEALLOCATE).is_empty(), "{address:x}");
            assert_eq!(e.call(address, &args![this, 1u32]).u32(), this);
            assert_eq!(calls_to(&log, MEMORY_DEALLOCATE).len(), 1, "{address:x}");
            assert_eq!(e.mem.u32(this), vtable);
        }
    }

    #[test]
    fn simple_array_destructors_free_the_data_through_the_vtable_slot_and_clear_the_array() {
        for (address, vtable) in [(0x00c7_a630, 0x010c_4db8u32), (0x00c7_a660, 0x010c_4ce4)] {
            let mut e = engine();
            let log = CallLog::default();
            recorder(&mut e, &log, 0x0900_00f0, 0);
            e.mem.set_u32(vtable + 8, 0x0900_00f0);
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 4, 0);
            e.call(address, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            assert!(log.borrow().is_empty());
            e.mem.set_u32(this + 4, 0x7000);
            e.mem.set_u32(this + 8, 3);
            e.mem.set_u32(this + 0xc, 4);
            e.call(address, &args![this]);
            assert_eq!(calls_to(&log, 0x0900_00f0), vec![vec![this, 0x7000]]);
            assert_eq!(
                (
                    e.mem.u32(this + 4),
                    e.mem.u32(this + 8),
                    e.mem.u32(this + 0xc)
                ),
                (0, 0, 0)
            );
        }
    }

    #[test]
    fn bone_transform_array_allocation_scales_by_0x38() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, MEMORY_ALLOCATE, 0x5555);
        recorder(&mut e, &log, 0x00aa_4150, 0x6666);
        assert_eq!(e.call(0x00c7_a5d0, &args![0x7777u32, 3u32]).u32(), 0x5555);
        assert_eq!(
            calls_to(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xa8]]
        );
        assert_eq!(
            e.call(0x00c7_a600, &args![0x5555u32, 0x1234u32, 2u32])
                .u32(),
            0x6666
        );
        assert_eq!(
            calls_to(&log, 0x00aa_4150),
            vec![vec![MEMORY_MANAGER, 0x1234, 0x70]]
        );
    }

    #[test]
    fn fn_00c7a730_appends_a_slot_growing_the_array_when_full() {
        let mut e = engine();
        let log = CallLog::default();
        let array = array_setup(&mut e, &log, 0x5000, 0x6000);
        assert_eq!(e.call(0x00c7_a730, &args![array]).u32(), 0);
        assert_eq!(calls_to(&log, 0x0900_0004), vec![vec![array, 4]]);
        assert_eq!(
            (
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc),
                e.mem.u32(array + 4)
            ),
            (1, 4, 0x5000)
        );
        assert_eq!(e.call(0x00c7_a730, &args![array]).u32(), 1);
        assert_eq!(e.mem.u32(array + 8), 2);
        e.mem.set_u32(array + 8, 4);
        assert_eq!(e.call(0x00c7_a730, &args![array]).u32(), 4);
        assert_eq!((e.mem.u32(array + 8), e.mem.u32(array + 0xc)), (5, 8));
        assert_eq!(calls_to(&log, 0x0900_0004).len(), 1);
        assert_eq!(calls_to(&log, 0x0900_000c), vec![vec![array, 0x5000, 8]]);
        e.mem.set_u32(array + 8, 0x800);
        e.mem.set_u32(array + 0xc, 0x800);
        e.call(0x00c7_a730, &args![array]);
        assert_eq!(e.mem.u32(array + 0xc), 0xc00);
        e.mem.set_u32(array + 8, 0x400);
        e.mem.set_u32(array + 0xc, 0x400);
        e.call(0x00c7_a730, &args![array]);
        assert_eq!(e.mem.u32(array + 0xc), 0x800);
    }

    /// A settings record (the layout `00c74fb0` compares) whose value array is driven by an
    /// allocator object with logging slots.
    fn assign_setup(e: &mut Engine, log: &CallLog) -> (u32, u32) {
        recorder(e, log, 0x0900_0004, 0x5000);
        recorder(e, log, 0x0900_0008, 0);
        recorder(e, log, 0x005e_f6a0, 0);
        let destination = object_with_vtable(e, &[(4, 0x0900_0004), (8, 0x0900_0008)]);
        let vtable = e.mem.u32(destination);
        let record = e.mem.alloc(0x100);
        e.mem.set_u32(record + 0x40, vtable);
        let source = e.mem.alloc(0x100);
        let values = e.mem.alloc(0x10);
        for (index, value) in [7u16, 8, 9].into_iter().enumerate() {
            e.mem.set_u16(values + 2 * index as u32, value);
        }
        e.mem.set_u32(source + 0x44, values);
        e.mem.set_u32(source + 0x48, 3);
        e.mem.set_u8(source, 0x21);
        for index in 1..=13u32 {
            e.mem.set_f32(source + 4 * index, index as f32);
        }
        e.mem.set_u32(source + 0x38, 0x38);
        e.mem.set_u32(source + 0x3c, 0x3c);
        e.mem.set_u32(source + 0x4c, 3);
        (record, source)
    }

    #[test]
    fn fn_00c7a790_grows_the_array_and_copies_every_field() {
        let mut e = engine();
        let log = CallLog::default();
        let (record, source) = assign_setup(&mut e, &log);
        let data = e.mem.alloc(0x10);
        recorder(&mut e, &log, 0x0900_0004, data);
        e.mem.set_u32(source + 4, 0x7fc0_0001);
        e.mem.set_u32(source + 8, 0x7f80_0001);
        e.call(0x00c7a790, &args![record, source]);
        assert_eq!(calls_to(&log, 0x0900_0004), vec![vec![record + 0x40, 3]]);
        assert_eq!(e.mem.u32(record + 0x44), data);
        assert_eq!((e.mem.u32(record + 0x48), e.mem.u32(record + 0x4c)), (3, 3));
        let copied: Vec<u16> = (0..3).map(|i| e.mem.u16(data + 2 * i)).collect();
        assert_eq!(copied, vec![7, 8, 9]);
        assert_eq!(e.mem.u8(record), 0x21);
        assert_eq!(e.mem.u32(record + 8), 0x7fc0_0001);
        assert_eq!(e.mem.f32(record + 0x34), 13.0);
        assert_eq!(
            (e.mem.u32(record + 0x38), e.mem.u32(record + 0x3c)),
            (0x38, 0x3c)
        );
    }

    #[test]
    fn fn_00c7a790_resizes_shrinks_and_empties_the_array() {
        let mut e = engine();
        let log = CallLog::default();
        let (record, source) = assign_setup(&mut e, &log);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(record + 0x44, data);
        e.mem.set_u32(record + 0x48, 2);
        e.mem.set_u32(record + 0x4c, 2);
        e.call(0x00c7a790, &args![record, source]);
        assert_eq!(calls_to(&log, 0x005e_f6a0), vec![vec![record + 0x40, 3, 2]]);
        assert_eq!((e.mem.u32(record + 0x48), e.mem.u32(record + 0x4c)), (3, 3));
        log.borrow_mut().clear();
        e.mem.set_u32(source + 0x48, 2);
        e.call(0x00c7a790, &args![record, source]);
        assert!(log.borrow().is_empty());
        assert_eq!((e.mem.u32(record + 0x48), e.mem.u32(record + 0x4c)), (2, 3));
        e.mem.set_u32(record + 0x4c, 16);
        e.mem.set_u32(record + 0x48, 8);
        e.call(0x00c7a790, &args![record, source]);
        assert_eq!(calls_to(&log, 0x005e_f6a0), vec![vec![record + 0x40, 2, 2]]);
        assert_eq!((e.mem.u32(record + 0x48), e.mem.u32(record + 0x4c)), (2, 2));
        e.mem.set_u32(record + 0x4c, 16);
        e.mem.set_u32(record + 0x48, 1);
        e.call(0x00c7a790, &args![record, source]);
        assert_eq!(e.mem.u32(record + 0x48), 2);
        assert_eq!(e.mem.u32(record + 0x4c), 16);
        e.mem.set_u32(source + 0x48, 0);
        e.call(0x00c7a790, &args![record, source]);
        assert_eq!(calls_to(&log, 0x0900_0008), vec![vec![record + 0x40, data]]);
        assert_eq!(
            (
                e.mem.u32(record + 0x44),
                e.mem.u32(record + 0x48),
                e.mem.u32(record + 0x4c)
            ),
            (0, 0, 0)
        );
    }

    #[test]
    fn enable_ragdoll_anim_updates_the_flags_and_re_adds_held_constraints() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, 0x00cd_b3e0, 0);
        let (this, entries) = world_setup(&mut e, &log, true);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x4c, object);
        e.mem.set_u8(this + 0x42, 1);
        e.mem.set_u8(this + 0x222, 1);
        e.mem.set_u8(this + 0x21d, 7);
        e.mem.set_u8(this + 0x43, 5);
        e.call(0x00c7a8d0, &args![this]);
        assert_eq!(calls_to(&log, 0x00cd_b3e0), vec![vec![object]]);
        assert_eq!(e.mem.u8(this + 0x41), 1);
        assert_eq!(e.mem.u8(this + 0x43), 0);
        assert_eq!((e.mem.u8(this + 0x222), e.mem.u8(this + 0x223)), (0, 1));
        assert_eq!(e.mem.u8(this + 0x21e), 1);
        assert_eq!(calls_to(&log, 0x00c6_8d50), vec![vec![0x9999, entries[0]]]);
        log.borrow_mut().clear();
        e.call(0x00c7a8d0, &args![this]);
        assert!(calls_to(&log, 0x00cd_b3e0).is_empty());
        e.mem.set_u8(this + 0x28e, 1);
        e.call(0x00c7a8d0, &args![this]);
        assert_eq!(calls_to(&log, 0x00c6_8d50).len(), 1);
        e.mem.set_u8(this + 0x28e, 0);
        e.mem.set_u8(this + 0x42, 0);
        e.mem.set_u8(this + 0x220, 3);
        e.call(0x00c7a8d0, &args![this]);
        assert_eq!(calls_to(&log, 0x00c6_8d50).len(), 1);
        assert_eq!((e.mem.u8(this + 0x41), e.mem.u8(this + 0x222)), (0, 1));
    }

    #[test]
    fn fn_00c7a990_collects_the_node_value_and_the_items_of_the_wanted_type() {
        let mut e = engine();
        let log = CallLog::default();
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa11c);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x00c9_0940, {
            let kept = Rc::clone(&log);
            move |e, a| {
                kept.borrow_mut().push((0x00c9_0940, a.to_vec()));
                e.mem.set_u32(a[1] + 8, 0x10);
                Ret::default()
            }
        });
        let derived = e.mem.alloc(0x10);
        e.mem.set_u32(derived + 4, 0x0126_818c);
        let other = e.mem.alloc(0x10);
        let kinds = [derived, other, 0x0126_818c, 0];
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 8, 0x42);
        let mut cell = node + 0x14;
        for (index, kind) in kinds.into_iter().enumerate() {
            let double = 0x0900_3000 + index as u32;
            recorder(&mut e, &log, double, kind);
            let item = object_with_vtable(&mut e, &[(8, double)]);
            e.mem.set_u32(item + 8, 0x100 + index as u32);
            e.mem.set_u32(cell, item);
            let next = if index + 1 < 4 { e.mem.alloc(8) } else { 0 };
            e.mem.set_u32(cell + 4, next);
            cell = next;
        }
        let arrays = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        for (array, capacity) in arrays.into_iter().zip([4, 1]) {
            let data = e.mem.alloc(0x40);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity | 0x8000_0000);
        }
        e.call(0x00c7a990, &args![node, arrays[0], arrays[1]]);
        let values = |e: &Engine, array: u32| -> Vec<u32> {
            let data = e.mem.u32(array);
            (0..e.mem.u32(array + 4))
                .map(|i| e.mem.u32(data + 4 * i))
                .collect()
        };
        assert_eq!(values(&e, arrays[0]), vec![0x42]);
        assert_eq!(values(&e, arrays[1]), vec![0x100, 0x102]);
        assert_eq!(
            calls_to(&log, 0x00c9_0940),
            vec![vec![0xa11c, arrays[1], 4]]
        );
        e.call(0x00c7a990, &args![0u32, arrays[0], arrays[1]]);
        assert_eq!(values(&e, arrays[0]), vec![0x42]);
    }

    const ALLOCATOR_FREE: u32 = 0x0900_a010;
    const ALLOCATOR_FREE_SIZED: u32 = 0x0900_a008;

    /// The memory router double: it answers a block whose first dword is the allocation cursor
    /// and whose `+0x10` is an allocator object with logged doubles on slots 4, 8, 0xc and 0x10
    /// (each answering `0x2222_0000`). Returns the router block and the allocator.
    fn install_router(e: &mut Engine, log: &CallLog, cursor: u32) -> (u32, u32) {
        let slots = [
            (4, 0x0900_a004),
            (8, ALLOCATOR_FREE_SIZED),
            (0xc, 0x0900_a00c),
            (0x10, ALLOCATOR_FREE),
        ];
        for (_, double) in slots {
            recorder(e, log, double, 0x2222_0000);
        }
        let allocator = object_with_vtable(e, &slots);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, cursor);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (router, allocator)
    }

    /// Runs the array release at `address` over an owned array of `element`-byte elements and
    /// over one the flags mark as not owned.
    fn check_array_release(address: u32, element: u32) {
        let mut e = engine();
        let log = CallLog::default();
        let (_, allocator) = install_router(&mut e, &log, 0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0xd000);
        e.mem.set_u32(this + 4, 3);
        e.mem.set_u32(this + 8, 5 | 0x4000_0000);
        e.call(address, &args![this]);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, 0xd000, 5 * element]]
        );
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0x8000_0000]);
        e.mem.set_u32(this, 0xd000);
        e.mem.set_u32(this + 4, 3);
        e.mem.set_u32(this + 8, 0x8000_0005);
        e.call(address, &args![this]);
        assert_eq!(calls_to(&log, ALLOCATOR_FREE).len(), 1);
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn fn_00c7b0b0_empties_the_array_of_0xd0_byte_elements() {
        check_array_release(0x00c7b0b0, 0xd0);
    }

    #[test]
    fn fn_00c7b0f0_empties_the_array_of_4_byte_elements() {
        check_array_release(0x00c7b0f0, 4);
    }

    #[test]
    fn fn_00c7bf10_empties_the_array_of_8_byte_elements() {
        check_array_release(0x00c7bf10, 8);
    }

    #[test]
    fn fn_00c7bf50_empties_the_array_of_2_byte_elements() {
        check_array_release(0x00c7bf50, 2);
    }

    #[test]
    fn fn_00c7bf90_empties_the_array_of_0x30_byte_elements() {
        check_array_release(0x00c7bf90, 0x30);
    }

    #[test]
    fn fn_00c7b140_collects_the_node_found_through_the_casts_and_recurses_into_the_children() {
        let mut e = engine();
        let log = CallLog::default();
        install_router(&mut e, &log, 0);
        let node = e.mem.alloc(0x20);
        let holder = e.mem.alloc(0x20);
        let outer = e.mem.alloc(0x20);
        let inner = e.mem.alloc(0x20);
        let target = e.mem.alloc(0x40);
        let group = e.mem.alloc(0x100);
        let children = e.mem.alloc(0x10);
        let leaves = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        e.mem.set_u32(node + 0x1c, holder);
        e.mem.set_u32(outer + 0x10, inner);
        e.mem.set_u32(target + 8, 0x42);
        e.mem.set_u32(group + 0xa0, children);
        e.mem.set_u16(group + 0xa6, 2);
        for (index, leaf) in leaves.iter().enumerate() {
            e.mem.set_u32(children + 4 * index as u32, *leaf);
        }
        let casts = vec![
            ((holder, 0x0126_7e64), outer),
            ((outer, 0x0126_817c), 0),
            ((inner, 0x0126_81c0), target),
            ((node, 0x011f_4428), group),
        ];
        let kept = Rc::clone(&log);
        e.register_double(0x0065_3290, move |_, a| {
            kept.borrow_mut().push((0x0065_3290, a.to_vec()));
            let found = casts.iter().find(|c| c.0 == (a[0], a[1]));
            Ret {
                eax: found.map_or(0, |c| c.1),
                ..Ret::default()
            }
        });
        let arrays = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        for array in arrays {
            let data = e.mem.alloc(0x40);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, 4 | 0x8000_0000);
        }
        e.call(0x00c7b140, &args![node, arrays[0], arrays[1]]);
        let first = e.mem.u32(arrays[0]);
        assert_eq!((e.mem.u32(arrays[0] + 4), e.mem.u32(first)), (1, 0x42));
        assert_eq!(e.mem.u32(arrays[1] + 4), 0);
        let types: Vec<(u32, u32)> = calls_to(&log, 0x0065_3290)
            .into_iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(
            types,
            vec![
                (holder, 0x0126_7e64),
                (outer, 0x0126_817c),
                (inner, 0x0126_81c0),
                (node, 0x011f_4428),
                (leaves[0], 0x011f_4428),
                (leaves[1], 0x011f_4428),
            ]
        );
        log.borrow_mut().clear();
        e.call(0x00c7b140, &args![0u32, arrays[0], arrays[1]]);
        assert!(calls_to(&log, 0x0065_3290).is_empty());
    }

    #[test]
    fn enable_ragdoll_feedback_only_marks_the_request_while_the_flag_is_on() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        e.mem.set_u8(this + 0x28c, 1);
        e.call(0x00c7ba10, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x28d), 1);
        e.mem.set_u8(this + 0x28d, 0);
        e.call(0x00c7ba10, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x28d), 0);
        e.mem.set_u8(this + 0x28c, 0);
        e.mem.set_u8(this + 0x28e, 1);
        e.call(0x00c7ba10, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x28d), 0);
    }

    /// A constraint list of `count` dwords for `GetRBForBones` and an output array of the given
    /// capacity; returns the controller, the list's source dwords and the array.
    fn rb_setup(e: &mut Engine, count: u32, capacity: u32) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x100);
        let list = e.mem.alloc(0x20);
        let source = e.mem.alloc(0x40);
        for index in 0..count {
            e.mem.set_u32(source + 4 * index, 0x100 + index);
        }
        e.mem.set_u32(list + 8, source);
        e.mem.set_u32(list + 0xc, count);
        e.mem.set_u32(this + 0x48, list);
        let out = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(out, data);
        e.mem.set_u32(out + 8, capacity);
        (this, source, out)
    }

    #[test]
    fn get_rb_for_bones_copies_the_list_growing_the_array_when_it_is_too_small() {
        let mut e = engine();
        let log = CallLog::default();
        let (_, allocator) = install_router(&mut e, &log, 0);
        let kept = Rc::clone(&log);
        e.register_double(0x00c9_08c0, move |e, a| {
            kept.borrow_mut().push((0x00c9_08c0, a.to_vec()));
            let data = e.mem.alloc(0x40);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        for (count, capacity, wanted) in [(3, 8, None), (3, 2, Some(4)), (5, 2, Some(5))] {
            let (this, _, out) = rb_setup(&mut e, count, capacity);
            log.borrow_mut().clear();
            e.call(0x00c7be20, &args![this, out]);
            let grown = calls_to(&log, 0x00c9_08c0);
            match wanted {
                None => assert!(grown.is_empty()),
                Some(size) => assert_eq!(grown, vec![vec![allocator, out, size, 4]]),
            }
            let data = e.mem.u32(out);
            let values: Vec<u32> = (0..count).map(|i| e.mem.u32(data + 4 * i)).collect();
            assert_eq!(e.mem.u32(out + 4), count);
            assert_eq!(values, (0..count).map(|i| 0x100 + i).collect::<Vec<_>>());
        }
        let (this, _, out) = rb_setup(&mut e, 3, 8);
        e.mem.set_u32(this + 0x48, 0);
        log.borrow_mut().clear();
        e.call(0x00c7be20, &args![this, out]);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00c7be90_allocates_twelve_bytes_and_clears_the_last_dword() {
        let mut e = engine();
        let log = CallLog::default();
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 8, 0xff);
        recorder(&mut e, &log, MEMORY_ALLOCATE, block);
        assert_eq!(e.call(0x00c7be90, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(
            calls_to(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xc]]
        );
        recorder(&mut e, &log, MEMORY_ALLOCATE, 0);
        assert_eq!(e.call(0x00c7be90, &args![]).u32(), 0);
    }

    #[test]
    fn fn_00c7beb0_releases_the_held_reference_and_deallocates_the_item() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, INTERLOCKED_DECREMENT, 0);
        recorder(&mut e, &log, MEMORY_DEALLOCATE, 0);
        recorder(&mut e, &log, 0x0900_0004, 0);
        let held = object_with_vtable(&mut e, &[(4, 0x0900_0004)]);
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 8, held);
        e.call(0x00c7beb0, &args![VCALL_THIS, item]);
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT), vec![vec![held + 4]]);
        assert_eq!(calls_to(&log, 0x0900_0004), vec![vec![held]]);
        assert_eq!(
            calls_to(&log, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, item]]
        );
        assert_eq!(e.mem.u32(item + 8), 0);
        log.borrow_mut().clear();
        e.call(0x00c7beb0, &args![VCALL_THIS, item]);
        assert!(calls_to(&log, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls_to(&log, MEMORY_DEALLOCATE).len(), 1);
    }

    #[test]
    fn fn_00c7bfd0_initialises_the_record_with_a_quiet_copy_of_the_float() {
        let mut e = engine();
        let record = e.mem.alloc(0x60);
        for offset in 0..0x15 {
            e.mem.set_u32(record + 4 * offset, 0xdead_beef);
        }
        e.mem.set_u32(0x0107_8a5c, 0x7f80_0001);
        assert_eq!(e.call(0x00c7bfd0, &args![record]).u32(), record);
        let words: Vec<u32> = (0..0x15).map(|i| e.mem.u32(record + 4 * i)).collect();
        let mut want = vec![0u32; 0x15];
        want[4] = 0x7fc0_0001;
        for index in [7, 10, 13] {
            want[index] = 0x8000_0000;
        }
        want[19] = 1.0f32.to_bits();
        want[20] = 1.0f32.to_bits();
        assert_eq!(words[..14], want[..14]);
        assert_eq!(words[15..], want[15..]);
        assert_eq!(e.mem.u8(record + 0x38), 1);
        assert_eq!(e.mem.u8(record + 0x39), 1);
        assert_eq!(e.mem.u32(record + 0x3c), 0);
    }

    #[test]
    fn fn_00c7c030_empties_its_four_arrays_in_order() {
        let mut e = engine();
        let log = CallLog::default();
        let (_, allocator) = install_router(&mut e, &log, 0);
        let this = e.mem.alloc(0x40);
        for (base, data) in [(0x2c, 0xa000), (0x1c, 0xb000), (0x10, 0xc000), (4, 0xd000)] {
            e.mem.set_u32(this + base, data);
            e.mem.set_u32(this + base + 4, 2);
            e.mem.set_u32(this + base + 8, 3);
        }
        e.call(0x00c7c030, &args![this]);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![
                vec![allocator, 0xa000, 12],
                vec![allocator, 0xb000, 12],
                vec![allocator, 0xc000, 0x90],
                vec![allocator, 0xd000, 0x90],
            ]
        );
        for base in [0x2c, 0x1c, 0x10, 4] {
            let words: Vec<u32> = (0..3).map(|i| e.mem.u32(this + base + 4 * i)).collect();
            assert_eq!(words, vec![0, 0, 0x8000_0000]);
        }
    }

    #[test]
    fn fn_00c7c2b0_empties_its_three_arrays_of_eight_byte_elements() {
        let mut e = engine();
        let log = CallLog::default();
        let (_, allocator) = install_router(&mut e, &log, 0);
        let this = e.mem.alloc(0x40);
        for (base, data) in [(0x2c, 0xa000), (0x20, 0xb000), (0x14, 0xc000)] {
            e.mem.set_u32(this + base, data);
            e.mem.set_u32(this + base + 4, 2);
            e.mem.set_u32(this + base + 8, 3);
        }
        e.call(0x00c7c2b0, &args![this]);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![
                vec![allocator, 0xa000, 24],
                vec![allocator, 0xb000, 24],
                vec![allocator, 0xc000, 24],
            ]
        );
    }

    #[test]
    fn disable_ragdoll_anim_resets_the_state_and_clears_the_bone_flags() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, entries) = world_setup(&mut e, &log, false);
        e.mem.set_u32(0x010c_71b0, 0x3f80_0000);
        let flagged = e.mem.alloc(0x20);
        e.mem.set_u8(flagged + 0x14, 1);
        recorder(&mut e, &log, 0x0900_003c, flagged);
        let holder = object_with_vtable(&mut e, &[(0x3c, 0x0900_003c)]);
        let bone = e.mem.alloc(0x40);
        e.mem.set_u32(bone + 0x1c, holder);
        let bones = e.mem.alloc(0x10);
        e.mem.set_u32(bones, bone);
        let second_bone = e.mem.alloc(0x40);
        e.mem.set_u32(bones + 4, second_bone);
        let skeleton = e.mem.alloc(0x40);
        e.mem.set_u32(skeleton + 0x1c, 2);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0xc, skeleton);
        e.mem.set_u32(this + 0xa4, bones);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u8(this + 0x42, 1);
        e.mem.set_u8(this + 0x222, 1);
        for offset in [0x41, 0x43, 0x21e, 0xb3, 0x221, 0xb2, 0x28f] {
            e.mem.set_u8(this + offset, 1);
        }
        e.mem.set_f32(this + 0x1e8, 5.0);
        e.mem.set_f32(this + 0x1b8, 5.0);
        e.mem.set_f32(this + 0x290, 5.0);
        e.mem.set_u32(this + 0x228, 7);
        e.mem.set_u32(this + 0x294, 7);
        e.call(0x00c7c150, &args![this, 1u32]);
        assert_eq!(e.mem.u8(flagged + 0x14), 0);
        assert_eq!(calls_to(&log, 0x0900_003c), vec![vec![holder]]);
        for offset in [0x41, 0x43, 0x222, 0x21e, 0xb3, 0x221, 0xb2, 0x28f] {
            assert_eq!(e.mem.u8(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x223), 1);
        assert_eq!((e.mem.u8(this + 0x180), e.mem.u8(this + 0x130)), (1, 1));
        assert_eq!(e.mem.u32(this + 0x120), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x170), 0x3f80_0000);
        assert_eq!(e.mem.f32(this + 0x290), 1.0);
        assert_eq!(
            (e.mem.f32(this + 0x1e8), e.mem.f32(this + 0x1b8)),
            (0.0, 0.0)
        );
        assert_eq!((e.mem.u32(this + 0x228), e.mem.u32(this + 0x294)), (0, 0));
        assert_eq!(
            calls_to(&log, 0x00c6_b320),
            vec![vec![0x9999, entries[0]], vec![0x9999, entries[1]]]
        );
        log.borrow_mut().clear();
        e.mem.set_u8(this + 0x42, 0);
        e.mem.set_u8(this + 0x222, 1);
        e.call(0x00c7c150, &args![this, 1u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u8(this + 0x222), 1);
    }

    /// The doubles under `ni2hk_ov2`: the quaternion conversion and the normalisation do
    /// nothing, the position factor is 0.5.
    fn install_ni2hk(e: &mut Engine) {
        e.set_global(SCALE_FACTOR, 0.5f32);
        e.register_double(0x00a6_df40, |_, _| Ret::default());
        e.register_double(0x0056_11c0, |_, _| Ret::default());
    }

    /// A collision object holder whose slot 0x3c answers an object whose slot 8 answers
    /// `kind` (both through the doubles `double + 1` and `double`).
    fn collision_object(e: &mut Engine, double: u32, kind: u32) -> u32 {
        let log = CallLog::default();
        recorder(e, &log, double, kind);
        let object = object_with_vtable(e, &[(8, double)]);
        recorder(e, &log, double + 1, object);
        object_with_vtable(e, &[(0x3c, double + 1)])
    }

    /// The timer doubles: `GetTimer` answers `now` as a 64-bit value, the float to 64-bit
    /// conversion truncates.
    fn install_timer(e: &mut Engine, now: u64) {
        e.register_double(GET_TIMER, move |_, _| Ret {
            eax: now as u32,
            edx: (now >> 32) as u32,
            ..Ret::default()
        });
        e.register_double(FLOAT_TO_INT64, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                eax: value as i64 as u32,
                edx: (value as i64 >> 32) as u32,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn set_ragdoll_feedback_active_needs_the_feedback_flag() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, TIMER_RESET, 0);
        let this = e.mem.alloc(0x300);
        e.call(0x00c7b6a0, &args![this, 1u32]);
        e.mem.set_u8(this + 0x28d, 1);
        e.call(0x00c7b6a0, &args![this, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u8(this + 0x28e), 0);
    }

    #[test]
    fn set_ragdoll_feedback_active_restarts_the_timer_when_it_is_already_on() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, TIMER_RESET, 0);
        e.mem.set_f32(TIMER_SECONDS, 3000.0);
        let this = e.mem.alloc(0x300);
        e.mem.set_u8(this + 0x28d, 1);
        e.mem.set_u8(this + 0x28e, 1);
        e.call(0x00c7b6a0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&log, TIMER_RESET),
            vec![vec![this + 0x2a8, 3000.0f32.to_bits()]]
        );
        assert_eq!(log.borrow().len(), 1);
    }

    /// A controller for `00c7b6a0`: the world list of `world_setup` extended with two bodies
    /// (slots 1 and 2 of the body table, an entry with a negative slot between), the skeleton
    /// index table `[0, 2]`, the doubles for the helpers and the allocation cursor.
    /// Returns the controller, the bodies, the first block's address and the router block.
    fn feedback_setup(e: &mut Engine, log: &CallLog) -> (u32, [u32; 2], u32, u32) {
        let (this, _) = world_setup(e, log, false);
        let list = e.mem.u32(this + 0x48);
        let bodies = [e.mem.alloc(0x400), e.mem.alloc(0x400)];
        let worlds = e.mem.u32(list + 8);
        e.mem.set_u32(worlds + 4, bodies[0]);
        e.mem.set_u32(worlds + 8, bodies[1]);
        let slots = e.mem.alloc(0x10);
        e.mem.set_i32(slots, 1);
        e.mem.set_i32(slots + 4, -1);
        e.mem.set_i32(slots + 8, 2);
        e.mem.set_u32(list + 0x20, slots);
        let count_holder = e.mem.alloc(0x40);
        e.mem.set_u32(count_holder + 0x1c, 3);
        e.mem.set_u32(list + 0x2c, count_holder);
        let skeleton = e.mem.alloc(0x80);
        let table = e.mem.alloc(0x10);
        e.mem.set_u16(table, 0);
        e.mem.set_u16(table + 2, 2);
        e.mem.set_u32(skeleton + 0x44, table);
        e.mem.set_u32(skeleton + 0x48, 2);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x18, skeleton);
        e.mem.set_u32(this + 0x2a4, owner);
        for (index, body) in bodies.iter().enumerate() {
            let kind = [2u8, 5][index];
            e.mem.set_u8(body + 0xe8, kind);
            e.mem.set_u8(body + 0x2a, [1u8, 4][index]);
            put_floats(e, body + 0x1b0, &[1.0, 2.0, 3.0, 4.0]);
            put_floats(e, body + 0x1c0, &[5.0, 6.0, 7.0, 8.0]);
            let vtable = e.mem.alloc(0x100);
            e.mem.set_u32(vtable + 0x40, 0x0900_0040);
            e.mem.set_u32(vtable + 0x44, 0x0900_0044);
            e.mem.set_u32(body + 0xe0, vtable);
        }
        recorder(e, log, 0x0900_0040, 0);
        recorder(e, log, 0x0900_0044, 0);
        let cursor = e.mem.alloc(0x400);
        let (router, _) = install_router(e, log, cursor);
        for double in [
            SET_MOTION_TYPE,
            MOTION_TYPE_FOLLOWUP,
            ENTITY_ACTIVATE,
            MEMSET,
            TIMER_RESET,
        ] {
            recorder(e, log, double, 0);
        }
        e.register_double(MOTION_TYPE_CHECK, |e, a| Ret {
            eax: u32::from((1..=3).contains(&e.mem.u32(a[0])) || e.mem.u32(a[0]) == 6),
            ..Ret::default()
        });
        install_timer(e, 100);
        e.set_global(TICK_FACTOR, 2.0f32);
        e.mem.set_f32(TIMER_SECONDS, 3000.0);
        e.set_global(POSE_TOLERANCE, 0.001f32);
        e.mem.set_u8(this + 0x28d, 1);
        (this, bodies, cursor, router)
    }

    #[test]
    fn set_ragdoll_feedback_active_stores_the_poses_and_switches_the_motion_types() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, bodies, cursor, router) = feedback_setup(&mut e, &log);
        e.call(0x00c7b6a0, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x28e), 1);
        assert_eq!(e.mem.u32(router), cursor);
        assert_eq!(
            calls_to(&log, 0x00c6_b320).len(),
            2,
            "the constraints went to the world"
        );
        assert_eq!((e.mem.u32(this + 0x2a8), e.mem.u32(this + 0x2ac)), (100, 0));
        assert_eq!(
            (e.mem.u32(this + 0x2b0), e.mem.u32(this + 0x2b4)),
            (6000, 0)
        );
        let first_block = floats(&e, cursor, 4);
        assert_eq!(first_block, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(floats(&e, cursor + 0x80, 4), vec![5.0, 6.0, 7.0, 8.0]);
        assert_eq!(floats(&e, cursor + 0x20, 4), vec![0.0; 4]);
        assert_eq!(
            calls_to(&log, MEMSET),
            vec![vec![cursor, 0, 0x30], vec![cursor + 0x80, 0, 0x30]]
        );
        assert_eq!(
            calls_to(&log, SET_MOTION_TYPE),
            vec![vec![bodies[0], 1, 1, 0]]
        );
        assert_eq!(
            calls_to(&log, MOTION_TYPE_FOLLOWUP),
            vec![vec![bodies[0], 0]]
        );
        assert_eq!(e.mem.u8(bodies[0] + 0x2a), 4);
        assert_eq!(
            calls_to(&log, ENTITY_ACTIVATE),
            vec![vec![bodies[1]], vec![bodies[1]]]
        );
        assert_eq!(
            calls_to(&log, 0x0900_0040),
            vec![vec![bodies[1] + 0xe0, cursor + 0x20]]
        );
        assert_eq!(
            calls_to(&log, 0x0900_0044),
            vec![vec![bodies[1] + 0xe0, cursor + 0xa0]]
        );
    }

    #[test]
    fn set_ragdoll_feedback_active_switches_back_when_it_is_turned_off() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, bodies, _, _) = feedback_setup(&mut e, &log);
        e.mem.set_u8(this + 0x28e, 1);
        e.mem.set_u32(this + 0x2b0, 9);
        e.mem.set_u32(this + 0x2b4, 9);
        e.call(0x00c7b6a0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x28e), 0);
        assert_eq!((e.mem.u32(this + 0x2b0), e.mem.u32(this + 0x2b4)), (0, 0));
        assert!(calls_to(&log, 0x00c6_b320).is_empty());
        assert_eq!(
            calls_to(&log, SET_MOTION_TYPE),
            vec![vec![bodies[0], 4, 1, 0], vec![bodies[1], 4, 1, 0]]
        );
        assert_eq!(
            calls_to(&log, MOTION_TYPE_FOLLOWUP),
            vec![vec![bodies[0], 1], vec![bodies[1], 1]]
        );
        assert_eq!(
            (e.mem.u8(bodies[0] + 0x2a), e.mem.u8(bodies[1] + 0x2a)),
            (1, 1)
        );
    }

    /// A controller for `00c7b1f0` with the doubles of its helpers. Returns the controller, the
    /// skeleton, the candidate pose list object and the log of the single snapshot of the
    /// local block `00cddcf0` got.
    fn pose_matching_setup(
        e: &mut Engine,
        log: &CallLog,
    ) -> (u32, u32, u32, Rc<RefCell<Vec<Vec<u32>>>>) {
        let this = e.mem.alloc(0x300);
        e.mem.set_u8(this + 0x42, 1);
        let skeleton = e.mem.alloc(0x40);
        e.mem.set_u32(skeleton + 0x1c, 2);
        e.mem.set_u32(skeleton + 0xc, 0x5555);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 8, skeleton);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u32(this + 0x24c, 0x7000);
        recorder(e, log, STRING_COPY, 0x6100);
        recorder(e, log, INTERLOCKED_DECREMENT, 0);
        let kept = Rc::clone(log);
        e.register_double(FIND_BONE_WITH_NAME, move |_, a| {
            kept.borrow_mut().push((FIND_BONE_WITH_NAME, a.to_vec()));
            let bone = match a[1] {
                NAME_NON_ACCUM => 3,
                NAME_HEAD => 5,
                _ => 0xffff,
            };
            Ret {
                eax: 0x1234_0000 | bone,
                ..Ret::default()
            }
        });
        let candidates = e.mem.alloc(0x40);
        let items = e.mem.alloc(0x10);
        for index in 0..2u32 {
            let item = e.mem.alloc(0x20);
            let poses = e.mem.alloc(0x60);
            for word in 0..24 {
                e.mem.set_u32(poses + 4 * word, 0x1000 * (index + 1) + word);
            }
            e.mem.set_u32(item + 4, poses);
            e.mem.set_u32(items + 4 * index, item);
        }
        e.mem.set_u32(candidates + 0x10, 2);
        e.mem.set_u32(candidates + 0x14, items);
        e.register_double(POSE_LOADER_LOAD, move |e, a| {
            e.mem.set_u32(a[2], candidates);
            Ret::default()
        });
        let kept = Rc::clone(log);
        e.register_double(POSE_MATCHING_NEW, move |e, a| {
            let mut seen = a.to_vec();
            seen.extend((0..4).map(|i| e.mem.u32(a[4] + 4 * i)));
            kept.borrow_mut().push((POSE_MATCHING_NEW, seen));
            Ret {
                eax: 0x3333_0000,
                ..Ret::default()
            }
        });
        let snapshots = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&snapshots);
        e.register_double(LOCAL_TO_MODEL_POSE, move |e, a| {
            kept.borrow_mut()
                .push((0..24).map(|i| e.mem.u32(a[2] + 4 * i)).collect());
            Ret::default()
        });
        recorder(e, log, ADD_CANDIDATE_POSE, 0);
        recorder(e, log, POSE_FINISH, 1);
        (this, skeleton, candidates, snapshots)
    }

    #[test]
    fn init_pose_matching_builds_the_matching_object_and_adds_each_candidate_pose() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, skeleton, candidates, snapshots) = pose_matching_setup(&mut e, &log);
        let cursor = e.mem.alloc(0x400);
        let (router, allocator) = install_router(&mut e, &log, cursor);
        e.mem.set_u32(0x0101_1d78, 0);
        e.mem.set_u32(0x0101_2068, 1.0f32.to_bits());
        e.call(
            0x00c7b1f0,
            &args![this, 0x6000u32, 0xffffu32, 4u32, 0xffffu32],
        );
        assert_eq!(calls_to(&log, STRING_COPY), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(this + 0x24c), 0x6100);
        assert_eq!(
            calls_to(&log, INTERLOCKED_DECREMENT),
            vec![vec![0x7000 - 8]]
        );
        assert_eq!(
            calls_to(&log, FIND_BONE_WITH_NAME),
            vec![
                vec![skeleton, NAME_NON_ACCUM, 0],
                vec![skeleton, NAME_R_UPPER_ARM, 0]
            ]
        );
        assert_eq!(calls_to(&log, 0x0900_a004), vec![vec![allocator, 0x30]]);
        let created = calls_to(&log, POSE_MATCHING_NEW);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][..4], [0x2222_0000, 3, 4, 0xffff]);
        let up: Vec<u32> = [0.0f32, 0.0, 1.0, 0.0]
            .iter()
            .map(|f| f.to_bits())
            .collect();
        assert_eq!(created[0][5..], up[..]);
        assert_eq!(e.mem.u32(this + 0x248), 0x3333_0000);
        assert_eq!(e.mem.u32(this + 0x250), candidates);
        let adds = calls_to(&log, ADD_CANDIDATE_POSE);
        assert_eq!(adds.len(), 2);
        assert_eq!(adds[0], vec![0x3333_0000, cursor + 0x80, 0, 0]);
        assert_eq!(adds[1], vec![0x3333_0000, cursor + 0x80, 0, 0]);
        let snapshots = snapshots.borrow();
        assert_eq!(snapshots.len(), 2);
        assert_eq!(
            snapshots[0],
            (0..24).map(|w| 0x1000 + w).collect::<Vec<_>>()
        );
        assert_eq!(
            snapshots[1],
            (0..24).map(|w| 0x2000 + w).collect::<Vec<_>>()
        );
        assert_eq!(
            calls_to(&log, POSE_FINISH),
            vec![vec![candidates, skeleton]]
        );
        assert_eq!(e.mem.u8(this + 0x254), 1);
        assert_eq!(e.mem.u32(router), cursor);
    }

    #[test]
    fn init_pose_matching_does_nothing_when_all_bones_are_given_and_it_is_not_powered() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, _, _, snapshots) = pose_matching_setup(&mut e, &log);
        e.mem.set_u32(this + 0x24c, 0x6000);
        e.call(0x00c7b1f0, &args![this, 0x6000u32, 1u32, 2u32, 3u32]);
        assert!(calls_to(&log, STRING_COPY).is_empty());
        assert!(calls_to(&log, FIND_BONE_WITH_NAME).is_empty());
        assert!(calls_to(&log, POSE_MATCHING_NEW).is_empty());
        assert!(snapshots.borrow().is_empty());
        e.mem.set_u8(this + 0x42, 0);
        e.call(0x00c7b1f0, &args![this, 0x6001u32, 0xffffu32, 2u32, 3u32]);
        assert!(calls_to(&log, STRING_COPY).is_empty());
    }

    #[test]
    fn fn_00c7b4f0_records_the_node_and_the_children_that_have_a_collision_object() {
        let mut e = engine();
        let log = CallLog::default();
        install_ni2hk(&mut e);
        let (map, _) = hash_map(&mut e);
        let this = e.mem.alloc(0x300);
        let world = e.mem.alloc(0x100);
        e.mem.set_f32(world + 0x98, 3.0);
        e.mem.set_u32(this + 0x58, world);
        let records = e.mem.alloc(0x20);
        let record_data = e.mem.alloc(0x40);
        e.mem.set_u32(records + 4, record_data);
        e.mem.set_u16(records + 8, 123);
        let node_data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xa4, node_data);
        let count = Rc::new(std::cell::Cell::new(0u32));
        let kept = Rc::clone(&count);
        e.register_double(ARRAY_APPEND, move |_, _| {
            kept.set(kept.get() + 1);
            Ret {
                eax: kept.get() - 1,
                ..Ret::default()
            }
        });
        let kept = Rc::new(std::cell::Cell::new(0u32));
        let counter = Rc::clone(&kept);
        e.register_double(NODE_ARRAY_APPEND, move |_, _| {
            counter.set(counter.get() + 1);
            Ret {
                eax: counter.get() - 1,
                ..Ret::default()
            }
        });
        e.register_double(MAP_LOOKUP, |e, a| {
            if a[1] == 9 {
                e.mem.set_u32(a[2], 5);
            }
            Ret::default()
        });
        e.register_double(MEMORY_ALLOCATE, |e, _| Ret {
            eax: ((e.mem.alloc(0x90) + 0xf) & !0xf) + 3,
            ..Ret::default()
        });
        recorder(&mut e, &log, 0x0900_c001, 0);
        let leaf_vtable = [(0xc, 0x0900_c001)];
        let children: Vec<u32> = (0..3)
            .map(|_| object_with_vtable(&mut e, &leaf_vtable))
            .collect();
        let walked = children[0];
        let skipped = children[1];
        let plain_type = e.mem.alloc(0x10);
        let skipped_type = e.mem.alloc(0x10);
        e.mem.set_u32(skipped_type + 4, SKIPPED_TYPE);
        let walked_collision = collision_object(&mut e, 0x0900_c100, plain_type);
        let skipped_collision = collision_object(&mut e, 0x0900_c200, skipped_type);
        e.register_double(FIND_NEXT_COLLISION_OBJECT, move |_, a| Ret {
            eax: match a[0] {
                x if x == walked => walked_collision,
                x if x == skipped => skipped_collision,
                _ => 0,
            },
            ..Ret::default()
        });
        let group = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x10);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *child);
        }
        e.mem.set_u32(group + 0xa0, table);
        e.mem.set_u16(group + 0xa6, 3);
        recorder(&mut e, &log, 0x0900_c002, group);
        let node = object_with_vtable(&mut e, &[(0xc, 0x0900_c002)]);
        e.mem.set_u32(node + 8, 0x77);
        e.mem.set_u32(node + 0x18, 9);
        put_floats(&mut e, node + 0x58, &[2.0, 4.0, 6.0]);
        for child in &children {
            e.mem.set_u32(child + 0x18, 10);
            put_floats(&mut e, child + 0x58, &[2.0, 4.0, 6.0]);
        }
        e.call(0x00c7b4f0, &args![this, node, records, map]);
        let blocks: Vec<u32> = (0..2).map(|i| e.mem.u32(record_data + 4 * i)).collect();
        assert_eq!(e.mem.u32(record_data + 8), 0);
        assert_eq!(blocks[0] & 0xf, 0);
        assert_eq!(e.mem.u8(blocks[0] - 1), 13);
        assert_eq!(e.mem.u32(blocks[0]), 0x77);
        assert_eq!(floats(&e, blocks[0] + 0x10, 4), vec![3.0, 6.0, 9.0, 0.0]);
        assert_eq!(e.mem.u16(blocks[0] + 0x40), 5);
        assert_eq!(floats(&e, blocks[1] + 0x10, 4), vec![1.0, 2.0, 3.0, 0.0]);
        assert_eq!(e.mem.u16(blocks[1] + 0x40), 0xffff);
        assert_eq!(e.mem.u32(node_data), node);
        assert_eq!(e.mem.u32(node_data + 4), walked);
        assert_eq!(e.mem.u32(node_data + 8), 0);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        assert_eq!(count.get(), 2);
    }

    /// A controller for `00c7ba50` with the distance (3, 4, 0) from the stored vector, the
    /// timer, the helper doubles and the allocation cursor; returns the controller, the flag
    /// byte, the cursor, the objects block and the router block.
    fn do_pose_matching_setup(e: &mut Engine, log: &CallLog) -> (u32, u32, u32, u32, u32) {
        let this = e.mem.alloc(0x300);
        let world = e.mem.alloc(0x100);
        put_floats(e, world + 0x8c, &[3.0, 4.0, 0.0]);
        e.mem.set_u32(this + 0x58, world);
        let objects = e.mem.alloc(0x40);
        let counts = e.mem.alloc(0x40);
        e.mem.set_u32(counts + 0x1c, 2);
        e.mem.set_u32(objects + 0x2c, counts);
        e.mem.set_u32(this + 0x48, objects);
        e.mem.set_u32(this + 0x250, 0xa11c);
        e.mem.set_u32(this + 0x248, 0xb00b);
        e.mem.set_u32(this + 0x288, 0xffff_ffff);
        e.mem.set_u32(this + 0x258, 0xffff_ffff);
        e.mem.set_u32(this + 0x26c, 0);
        e.mem.set_u32(this + 0x268, 1000);
        e.mem.set_f32(this + 0x280, 4.0);
        e.mem.set_f32(this + 0x27c, 2.0);
        install_sqrt(e, log);
        install_timer(e, 100);
        e.set_global(TICK_FACTOR, 2.0f32);
        e.mem.set_f32(SCORE_LIMIT, 1.0);
        let cursor = e.mem.alloc(0x400);
        let (router, _) = install_router(e, log, cursor);
        recorder(e, log, GET_POSE_MODEL_SPACE, 0);
        recorder(e, log, APPLY_CANDIDATE, 0);
        e.register_double(FIND_BEST_CANDIDATE, |e, a| {
            e.mem.set_f32(a[2], 0.5);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let flag = e.mem.alloc(0x10);
        (this, flag, cursor, objects, router)
    }

    #[test]
    fn do_pose_matching_waits_until_the_timer_or_the_distance_asks_for_a_run() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, flag, _, _, _) = do_pose_matching_setup(&mut e, &log);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        assert!(calls_to(&log, APPLY_CANDIDATE).is_empty());
        assert!(calls_to(&log, GET_POSE_MODEL_SPACE).is_empty());
        assert_eq!(calls_to(&log, X87_SQRT), vec![args![25.0f64]]);
        e.mem.set_u8(this + 0x284, 1);
        e.mem.set_f32(this + 0x27c, 5.0);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        assert!(calls_to(&log, GET_POSE_MODEL_SPACE).is_empty());
        e.mem.set_f32(this + 0x27c, 4.9);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        assert_eq!(calls_to(&log, GET_POSE_MODEL_SPACE).len(), 1);
    }

    #[test]
    fn do_pose_matching_applies_the_best_candidate_and_renews_the_timer() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, flag, cursor, objects, router) = do_pose_matching_setup(&mut e, &log);
        e.mem.set_u32(this + 0x268, 50);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        assert_eq!(
            calls_to(&log, GET_POSE_MODEL_SPACE),
            vec![vec![objects, cursor, 0x1111]]
        );
        let counts = e.mem.u32(objects + 0x2c);
        assert_eq!(
            calls_to(&log, APPLY_CANDIDATE),
            vec![vec![0xa11c, 0x2222, counts, 4]]
        );
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u8(flag)), (3, 1));
        assert_eq!(e.mem.f32(this + 8), 0.5);
        assert_eq!(e.mem.u32(this + 0x258), 4);
        assert_eq!(floats(&e, this + 0x270, 3), vec![3.0, 4.0, 0.0]);
        assert_eq!((e.mem.u32(this + 0x260), e.mem.u32(this + 0x264)), (100, 0));
        assert_eq!((e.mem.u32(this + 0x268), e.mem.u32(this + 0x26c)), (8, 0));
        assert_eq!(e.mem.u32(router), cursor);
        e.mem.set_u32(this + 0x260, 0);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        assert_eq!(calls_to(&log, APPLY_CANDIDATE).len(), 1);
        assert_eq!(e.mem.u32(this + 0xc), 3);
    }

    #[test]
    fn do_pose_matching_without_a_new_best_candidate_resets_the_mode() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, flag, _, _, _) = do_pose_matching_setup(&mut e, &log);
        e.mem.set_u32(this + 0x268, 50);
        e.mem.set_f32(SCORE_LIMIT, 0.25);
        e.mem.set_u32(this + 0xc, 3);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        assert!(calls_to(&log, APPLY_CANDIDATE).is_empty());
        assert_eq!(
            (e.mem.u32(this + 0xc), e.mem.u8(flag), e.mem.u8(this + 4)),
            (0, 0, 1)
        );
        e.mem.set_u8(flag, 1);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u8(this + 4, 0);
        e.mem.set_u32(this + 0x260, 0);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u8(this + 4)), (3, 0));
    }

    #[test]
    fn do_pose_matching_applies_a_stored_index_first() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, flag, _, objects, _) = do_pose_matching_setup(&mut e, &log);
        e.mem.set_u32(this + 0x288, 7);
        e.call(0x00c7ba50, &args![this, 0x1111u32, 0x2222u32, flag]);
        let counts = e.mem.u32(objects + 0x2c);
        assert_eq!(
            calls_to(&log, APPLY_CANDIDATE),
            vec![vec![0xa11c, 0x2222, counts, 7]]
        );
        assert!(calls_to(&log, GET_POSE_MODEL_SPACE).is_empty());
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u8(flag)), (3, 1));
        assert_eq!(
            (e.mem.u32(this + 0x258), e.mem.u32(this + 0x288)),
            (7, 0xffff_ffff)
        );
        assert_eq!((e.mem.u32(this + 0x268), e.mem.u32(this + 0x260)), (8, 100));
    }

    #[test]
    fn copy_pose_to_3d_rec_copies_the_transforms_of_each_found_node() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, INTERLOCKED_INCREMENT, 1);
        recorder(&mut e, &log, INTERLOCKED_DECREMENT, 1);
        recorder(&mut e, &log, 0x00a8_6bf0, 0);
        recorder(&mut e, &log, 0x0900_c001, 0);
        let equal = Rc::new(std::cell::Cell::new(0u32));
        let kept = Rc::clone(&equal);
        e.register_double(FIXED_STRING_EQUALS, move |_, _| Ret {
            eax: kept.get(),
            ..Ret::default()
        });
        let found = e.mem.alloc(0x100);
        for word in 0..0x20 {
            e.mem.set_u32(found + 0x34 + 4 * word, 0x4000 + word);
        }
        e.register_double(GET_OBJECT_BY_NAME, move |_, a| Ret {
            eax: if a[0] == 0xc0de { found } else { 0 },
            ..Ret::default()
        });
        let source = e.mem.alloc(0x100);
        for word in 0..0x20 {
            e.mem.set_u32(source + 0x34 + 4 * word, 0x8000 + word);
        }
        let node = object_with_vtable(&mut e, &[(0xc, 0x0900_c001)]);
        e.mem.set_u32(node + 8, 0x500);
        for word in 0..0x20 {
            e.mem.set_u32(node + 0x34 + 4 * word, 0xa000 + word);
        }
        e.mem.set_u32(node + 0x18, source);
        let transforms = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x38 * 4);
        e.mem.set_u32(transforms + 4, elements);
        e.mem.set_u32(transforms + 0xc, 4);
        e.call(0x00c7bcb0, &args![0xc0deu32, node, transforms]);
        assert_eq!(calls_to(&log, INTERLOCKED_INCREMENT), vec![vec![0x500 - 8]]);
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT), vec![vec![0x500 - 8]]);
        assert_eq!(e.mem.u32(transforms + 8), 1);
        let record: Vec<u32> = (0..14).map(|i| e.mem.u32(elements + 4 * i)).collect();
        let mut want = vec![found];
        want.extend((0..13).map(|w| 0x4000 + w));
        assert_eq!(record, want);
        assert_eq!(e.mem.u32(found + 0x34), 0xa000);
        equal.set(1);
        e.call(0x00c7bcb0, &args![0xc0deu32, node, transforms]);
        assert_eq!(e.mem.u32(transforms + 8), 2);
        let nine: Vec<u32> = (0..9).map(|i| e.mem.u32(found + 0x34 + 4 * i)).collect();
        assert_eq!(nine, (0..9).map(|w| 0x8000 + w).collect::<Vec<_>>());
        let vector: Vec<u32> = (0..3).map(|i| e.mem.u32(found + 0x58 + 4 * i)).collect();
        assert_eq!(vector, (0..3).map(|w| 0x8000 + 9 + w).collect::<Vec<_>>());
        assert_eq!(e.mem.u32(elements + 0x38), found);
        assert_eq!(e.mem.u32(elements + 0x38 + 4), 0xa000);
    }

    #[test]
    fn copy_pose_to_3d_reports_a_moved_object_and_forces_the_motion_type() {
        let mut e = engine();
        let log = CallLog::default();
        recorder(&mut e, &log, SET_CO_USE_VEL, 0);
        recorder(&mut e, &log, SET_MOTION, 0);
        recorder(&mut e, &log, UPDATE_OBJECT, 0);
        recorder(&mut e, &log, INTERLOCKED_INCREMENT, 1);
        recorder(&mut e, &log, INTERLOCKED_DECREMENT, 1);
        recorder(&mut e, &log, FIXED_STRING_EQUALS, 0);
        recorder(&mut e, &log, GET_OBJECT_BY_NAME, 0);
        recorder(&mut e, &log, 0x0900_c001, 0);
        let collision = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x40);
        let inner = e.mem.alloc(0x100);
        e.mem.set_u32(collision + 0x10, body);
        e.mem.set_u32(body + 8, inner);
        e.mem.set_u8(inner + 0xe8, 4);
        e.register_double(FIND_NEXT_COLLISION_OBJECT, move |_, a| Ret {
            eax: if a[0] == 0x1000 { collision } else { 0 },
            ..Ret::default()
        });
        let moved = e.mem.alloc(0x10);
        let motion = e.mem.alloc(0x10);
        let transforms = e.mem.alloc(0x10);
        e.mem.set_u16(collision + 0xc, 0x20);
        e.call(
            0x00c7d3b0,
            &args![0x1000u32, 0u32, transforms, moved, motion],
        );
        assert_eq!((e.mem.u8(moved), e.mem.u32(motion)), (1, 4));
        assert_eq!(calls_to(&log, SET_CO_USE_VEL), vec![vec![0, 0x1000]]);
        assert!(calls_to(&log, SET_MOTION).is_empty());
        assert!(calls_to(&log, UPDATE_OBJECT).is_empty());
        e.mem.set_u16(collision + 0xc, 0x60);
        e.mem.set_u32(collision + 0x10, 0);
        e.call(
            0x00c7d3b0,
            &args![0x1000u32, 0u32, transforms, moved, motion],
        );
        assert_eq!((e.mem.u8(moved), e.mem.u32(motion)), (0, 5));
        assert_eq!(calls_to(&log, SET_CO_USE_VEL).len(), 1);
        assert_eq!(calls_to(&log, SET_MOTION), vec![vec![0x1000, 4, 1, 0, 1]]);
        e.mem.set_u32(motion, 4);
        e.mem.set_u8(moved, 7);
        e.call(
            0x00c7d3b0,
            &args![0x2000u32, 0u32, transforms, moved, motion],
        );
        assert_eq!((e.mem.u8(moved), e.mem.u32(motion)), (0, 4));
        assert_eq!(calls_to(&log, SET_MOTION).len(), 1);
        let node = object_with_vtable(&mut e, &[(0xc, 0x0900_c001)]);
        e.call(
            0x00c7d3b0,
            &args![0x1000u32, node, transforms, moved, motion],
        );
        let updates = calls_to(&log, UPDATE_OBJECT);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0][0], 0x1000);
    }

    fn check_stack_array_destruct(address: u32) {
        let mut e = engine();
        let log = CallLog::default();
        let (router, allocator) = install_router(&mut e, &log, 0x1234);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0xd000);
        e.mem.set_u32(this + 4, 2);
        e.mem.set_u32(this + 8, 3);
        e.mem.set_u32(this + 0xc, 0xd000);
        e.call(address, &args![this]);
        assert_eq!(e.mem.u32(router), 0xd000);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, 0xd000, 12]]
        );
        let words: Vec<u32> = (0..3).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0x8000_0000]);
        e.mem.set_u32(this, 0xe000);
        e.mem.set_u32(this + 4, 2);
        e.mem.set_u32(this + 8, 0x8000_0003);
        e.mem.set_u32(this + 0xc, 0xf000);
        e.call(address, &args![this]);
        assert_eq!(e.mem.u32(router), 0xf000);
        assert_eq!(calls_to(&log, ALLOCATOR_FREE).len(), 1);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn fn_00c7d480_releases_the_stack_array_and_restores_the_allocation_cursor() {
        check_stack_array_destruct(0x00c7d480);
    }

    #[test]
    fn fn_00c7d520_releases_the_stack_array_and_restores_the_allocation_cursor() {
        check_stack_array_destruct(0x00c7d520);
    }

    #[test]
    fn fn_00c7d900_destroys_the_members_in_order() {
        let mut e = engine();
        let log = CallLog::default();
        let (_, allocator) = install_router(&mut e, &log, 0);
        for double in [
            ELEMENT_DESTRUCT,
            RIGID_BODY_CONTROLLER_DESTRUCT,
            POSE_LOADER_RELEASE,
            0x0900_0b01,
        ] {
            recorder(&mut e, &log, double, 0);
        }
        recorder(&mut e, &log, INTERLOCKED_DECREMENT, 1);
        e.mem.set_u32(0x0107_1af4, 0x0900_0b01);
        let this = e.mem.alloc(0x300);
        let table = e.mem.alloc(0x1a0);
        let elements = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        e.mem.set_u32(table, elements[0]);
        e.mem.set_u32(table + 0xd0, elements[1]);
        e.mem.set_u32(this + 0x22c, table);
        e.mem.set_u32(this + 0x230, 2);
        e.mem.set_u32(this + 0x234, 3);
        let rigid_bodies = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x4c, rigid_bodies);
        let list = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x48, list);
        let mut empties = Vec::new();
        for field in [0x54, 0x88] {
            let object = e.mem.alloc(0x40);
            for flags in [0xc, 0x18, 0x24, 0x34] {
                e.mem.set_u32(object + flags, 0x8000_0000);
            }
            e.mem.set_u32(this + field, object);
            empties.push(object);
        }
        let candidates = e.mem.alloc(0x20);
        e.mem.set_u32(candidates + 4, 2);
        e.register_double(POSE_LOADER_LOAD, move |e, a| {
            e.mem.set_u32(a[2], candidates);
            Ret::default()
        });
        e.mem.set_u32(this + 0x24c, 0x6000);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 4, 2);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u32(this + 0x2a0, 0x6100);
        let matching = e.mem.alloc(0x20);
        e.mem.set_u32(matching, 0x9000);
        e.mem.set_u32(matching + 4, 5);
        e.mem.set_u32(matching + 8, 2);
        e.mem.set_u32(this + 0x248, matching);
        e.mem.set_u32(this + 0xa4, 0x7000);
        e.mem.set_u32(this + 0xa8, 4);
        e.mem.set_u32(this + 0xac, 4);
        e.mem.set_u32(this + 0x94, 0xe000);
        e.mem.set_u32(this + 0x98, 1);
        e.mem.set_u32(this + 0x9c, 2);
        let reference = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x84, reference);
        e.call(0x00c7d900, &args![this]);
        assert_eq!(e.mem.u32(this), CONTROLLER_BASE_VTABLE);
        assert_eq!(
            calls_to(&log, ELEMENT_DESTRUCT),
            vec![vec![elements[0]], vec![elements[1]], vec![list]]
        );
        assert_eq!(
            calls_to(&log, RIGID_BODY_CONTROLLER_DESTRUCT),
            vec![vec![rigid_bodies]]
        );
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE_SIZED),
            vec![
                vec![allocator, rigid_bodies, 0x48],
                vec![allocator, empties[0], 0x38],
                vec![allocator, empties[1], 0x38],
                vec![allocator, matching, 0x30],
            ]
        );
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![
                vec![allocator, 0x9000, 0x40],
                vec![allocator, table, 0x270],
                vec![allocator, 0xe000, 0x60],
            ]
        );
        assert_eq!(
            calls_to(&log, POSE_LOADER_LOAD).len(),
            0,
            "the loader double does not log"
        );
        assert_eq!(
            calls_to(&log, POSE_LOADER_RELEASE),
            vec![vec![POSE_LOADER, 0x6000], vec![SECOND_LOADER, 0x6100]]
        );
        assert_eq!(
            calls_to(&log, INTERLOCKED_DECREMENT),
            vec![
                vec![owner + 4],
                vec![candidates + 4],
                vec![0x6100 - 8],
                vec![0x6000 - 8],
                vec![reference + 4],
            ]
        );
        assert_eq!(calls_to(&log, 0x0900_0b01), vec![vec![this + 0xa0, 0x7000]]);
        assert_eq!(e.mem.u32(this + 0xa0), ARRAY_BASE_VTABLE);
        assert_eq!(
            (
                e.mem.u32(this + 0xa4),
                e.mem.u32(this + 0xa8),
                e.mem.u32(this + 0xac)
            ),
            (0, 0, 0)
        );
        assert_eq!((e.mem.u32(this + 0x2a4), e.mem.u32(this + 0x250)), (0, 0));
        assert_eq!(
            (
                e.mem.u32(this + 0x22c),
                e.mem.u32(this + 0x230),
                e.mem.u32(this + 0x234)
            ),
            (0, 0, 0x8000_0000)
        );
        assert_eq!(
            (e.mem.u32(this + 0x94), e.mem.u32(this + 0x9c)),
            (0, 0x8000_0000)
        );
    }

    #[test]
    fn hka_skeleton_destruct_releases_its_arrays_one_by_one() {
        let mut e = engine();
        let log = CallLog::default();
        let (_, allocator) = install_router(&mut e, &log, 0);
        recorder(&mut e, &log, ELEMENT_DESTRUCT, 0);
        recorder(&mut e, &log, STRING_PTR_DESTRUCT, 0);
        let this = e.mem.alloc(0x60);
        let pointers = e.mem.alloc(0x20);
        let kept = e.mem.alloc(0x10);
        e.mem.set_u32(pointers, kept);
        e.mem.set_u32(pointers + 8, 0);
        for (base, data, size, flags) in [
            (0x3c, pointers, 2, 3),
            (0x30, 0xaa00, 2, 2),
            (0x24, 0xbb00, 1, 1),
            (0x18, 0xcc00, 2, 4),
            (0xc, 0xdd00, 1, 5),
        ] {
            e.mem.set_u32(this + base, data);
            e.mem.set_u32(this + base + 4, size);
            e.mem.set_u32(this + base + 8, flags);
        }
        e.call(0x00c7dc40, &args![this]);
        assert_eq!(e.mem.u32(this), SKELETON_VTABLE);
        assert_eq!(calls_to(&log, ELEMENT_DESTRUCT), vec![vec![kept]]);
        assert_eq!(e.mem.u32(pointers), 0);
        assert_eq!(
            calls_to(&log, STRING_PTR_DESTRUCT),
            vec![
                vec![0xaa00],
                vec![0xaa04],
                vec![0xcc00],
                vec![0xcc08],
                vec![this + 8]
            ]
        );
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![
                vec![allocator, pointers, 24],
                vec![allocator, 0xaa00, 8],
                vec![allocator, 0xbb00, 0x30],
                vec![allocator, 0xcc00, 32],
                vec![allocator, 0xdd00, 10],
            ]
        );
        for base in [0x3c, 0x30, 0x24, 0x18, 0xc] {
            let words: Vec<u32> = (0..3).map(|i| e.mem.u32(this + base + 4 * i)).collect();
            assert_eq!(words, vec![0, 0, 0x8000_0000]);
        }
    }

    #[test]
    fn scalar_deleting_destructor_of_the_controller_frees_only_on_bit_zero() {
        let mut e = engine();
        let log = CallLog::default();
        install_router(&mut e, &log, 0);
        recorder(&mut e, &log, POSE_LOADER_LOAD, 0);
        recorder(&mut e, &log, MEMORY_DEALLOCATE, 0);
        let block = e.mem.alloc(0x400);
        let this = block + 8;
        e.mem.set_u8(this - 1, 8);
        assert_eq!(e.call(0x00c7de30, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), CONTROLLER_BASE_VTABLE);
        assert!(calls_to(&log, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00c7de30, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&log, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
    }

    /// A controller for `00c7d030`: two bones, the blocks from a cursor, one body in the table
    /// (slot 0, the second entry unused) and the helper doubles. Returns the controller, the
    /// list, the cursor, the router block and the source pose buffer.
    fn rigid_body_setup(e: &mut Engine, log: &CallLog, mode: u32) -> (u32, u32, u32, u32, u32) {
        let this = e.mem.alloc(0x300);
        let list = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        let local = e.mem.alloc(0x80);
        e.mem.set_u32(skeleton + 0x1c, 2);
        e.mem.set_u32(skeleton + 0xc, 0x5555);
        e.mem.set_u32(skeleton + 0x24, local);
        e.mem.set_u32(list + 0x2c, skeleton);
        let bodies = e.mem.alloc(0x10);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(bodies, body);
        e.mem.set_u32(list + 8, bodies);
        let slots = e.mem.alloc(0x10);
        e.mem.set_i32(slots, 0);
        e.mem.set_i32(slots + 4, -1);
        e.mem.set_u32(list + 0x20, slots);
        e.mem.set_u32(this + 0x48, list);
        e.mem.set_u32(this + 0xc, mode);
        e.mem.set_u32(this + 0x4c, 0xc0de);
        e.mem.set_u32(this + 0x88, 0xa5a5);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x10, 0x3a3a);
        e.mem.set_u32(this + 0x2a4, owner);
        let cursor = e.mem.alloc(0x400);
        let (router, _) = install_router(e, log, cursor);
        let cell = e.mem.alloc(0x10);
        e.mem.set_u32(cell, 0x7777);
        recorder(e, log, GET_SYNCED_POSE_MODEL_SPACE, cell);
        for double in [
            LOCAL_TO_MODEL_POSE,
            MAP_POSE,
            MODEL_TO_LOCAL_POSE,
            RIGID_BODY_DRIVE_TO_POSE,
            APPLY_MODEL_POSE,
            RIGID_BODY_REINITIALIZE,
        ] {
            recorder(e, log, double, 0);
        }
        let source = e.mem.alloc(0x80);
        (this, list, cursor, router, source)
    }

    #[test]
    fn update_ragdoll_rigid_bodies_maps_the_local_pose_and_applies_it_in_mode_one() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, list, cursor, router, _) = rigid_body_setup(&mut e, &log, 1);
        e.call(0x00c7d030, &args![this, 0u32]);
        let second = cursor + 0x80;
        let skeleton = e.mem.u32(list + 0x2c);
        let local = e.mem.u32(skeleton + 0x24);
        assert_eq!(
            calls_to(&log, LOCAL_TO_MODEL_POSE),
            vec![vec![2, 0x5555, local, second]]
        );
        assert_eq!(
            calls_to(&log, GET_SYNCED_POSE_MODEL_SPACE),
            vec![vec![0xa5a5]]
        );
        assert_eq!(
            calls_to(&log, MAP_POSE),
            vec![vec![0x3a3a, 0x7777, local, second, 2]]
        );
        assert!(calls_to(&log, MODEL_TO_LOCAL_POSE).is_empty());
        assert_eq!(
            calls_to(&log, APPLY_MODEL_POSE),
            vec![vec![list, second, this + 0x10]]
        );
        assert_eq!(e.mem.u32(router), cursor);
        e.mem.set_u32(this + 0xc, 0);
        e.call(0x00c7d030, &args![this, 0u32]);
        assert_eq!(
            calls_to(&log, MODEL_TO_LOCAL_POSE),
            vec![vec![2, 0x5555, second, cursor]]
        );
        assert_eq!(calls_to(&log, APPLY_MODEL_POSE).len(), 1);
    }

    #[test]
    fn update_ragdoll_rigid_bodies_copies_a_given_pose_and_handles_the_flags() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, _, cursor, _, source) = rigid_body_setup(&mut e, &log, 3);
        for word in 0..24 {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.mem.set_u8(this + 4, 1);
        e.mem.set_u8(this + 0x44, 1);
        e.call(0x00c7d030, &args![this, source]);
        let copy: Vec<u32> = (0..24).map(|i| e.mem.u32(cursor + 4 * i)).collect();
        assert_eq!(copy, (0..24).map(|w| 0x100 + w).collect::<Vec<_>>());
        assert!(calls_to(&log, LOCAL_TO_MODEL_POSE).is_empty());
        assert_eq!((e.mem.u8(this + 4), e.mem.u8(this + 0x44)), (0, 0));
        assert_eq!(calls_to(&log, APPLY_MODEL_POSE).len(), 1, "forced mode 1");
        assert_eq!(e.mem.u32(this + 0xc), 3);
        e.mem.set_u8(this + 0x44, 0);
        e.call(0x00c7d030, &args![this, source]);
        assert_eq!(
            e.mem.u32(this + 0xc),
            0,
            "mode 3 starts the motors and clears the mode"
        );
        assert_eq!(calls_to(&log, APPLY_MODEL_POSE).len(), 1);
    }

    #[test]
    fn update_ragdoll_rigid_bodies_mode_two_drives_the_controller_and_checks_the_bodies() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, list, cursor, router, _) = rigid_body_setup(&mut e, &log, 2);
        let body = e.mem.u32(e.mem.u32(list + 8));
        e.mem.set_f32(LIMIT_TABLE + 4, 0.01);
        e.mem.set_u8(body + 0x19c, 1);
        e.mem.set_u8(this + 0x50, 1);
        e.mem.set_f32(DRIVE_STRENGTH, 0.0);
        e.call(0x00c7d030, &args![this, 0u32]);
        assert_eq!(calls_to(&log, RIGID_BODY_REINITIALIZE), vec![vec![0xc0de]]);
        assert_eq!(e.mem.u8(this + 0x50), 0);
        assert!(calls_to(&log, RIGID_BODY_DRIVE_TO_POSE).is_empty());
        e.mem.set_f32(DRIVE_STRENGTH, 0.5);
        e.call(0x00c7d030, &args![this, 0u32]);
        assert_eq!(
            calls_to(&log, RIGID_BODY_DRIVE_TO_POSE),
            vec![vec![
                0xc0de,
                0.5f32.to_bits(),
                cursor,
                this + 0x10,
                0x2222_0000
            ]]
        );
        assert_eq!(calls_to(&log, 0x0900_a00c).len(), 1);
        assert!(calls_to(&log, APPLY_MODEL_POSE).is_empty());
        assert_eq!(e.mem.u8(this + 0x50), 0);
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE).last().map(|c| (c[1], c[2])),
            Some((0x2222_0000, 8))
        );
        put_floats(&mut e, body + 0x1b0, &[0.5, 0.5, 0.5, 0.0]);
        e.call(0x00c7d030, &args![this, 0u32]);
        assert_eq!(
            calls_to(&log, APPLY_MODEL_POSE),
            vec![vec![list, cursor + 0x80, this + 0x10]]
        );
        assert_eq!(e.mem.u8(this + 0x50), 1);
        assert_eq!(e.mem.u32(router), cursor);
    }

    /// The SSE constants of the pose code with the values the exe stores in them.
    fn set_pose_constants(e: &mut Engine) {
        put_floats(e, NEG_HALF_LANES, &[-0.5; 4]);
        put_floats(e, ONE_LANES, &[1.0; 4]);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_LANES + 4 * lane, 0x8000_0000);
        }
        for lane in 0..3 {
            e.mem.set_u32(SIGN_XYZ_LANES + 4 * lane, 0x8000_0000);
            e.mem.set_u32(MASK_XYZ_LANES + 4 * lane, 0xffff_ffff);
        }
    }

    /// A pose whose skeleton has the given parent indices and flag words: output and local
    /// transform arrays (0x30 bytes per bone) with identity rotations, and the dirty byte set.
    fn pose_setup(e: &mut Engine, parents: &[i16], flags: &[u32]) -> u32 {
        let bones = parents.len() as u32;
        let pose = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        let parent_array = e.mem.alloc(2 * bones);
        let flag_array = e.mem.alloc(4 * bones);
        let output = e.mem.alloc(0x30 * bones);
        let local = e.mem.alloc(0x30 * bones);
        for (index, parent) in parents.iter().enumerate() {
            let index = index as u32;
            e.mem.set_i16(parent_array + 2 * index, *parent);
            e.mem.set_u32(flag_array + 4 * index, flags[index as usize]);
            for array in [output, local] {
                put_floats(e, array + 0x30 * index + 0x10, &[0.0, 0.0, 0.0, 1.0]);
            }
        }
        e.mem.set_u32(skeleton + 0xc, parent_array);
        e.mem.set_u32(skeleton + 0x1c, bones);
        e.mem.set_u32(pose, skeleton);
        e.mem.set_u32(pose + 4, output);
        e.mem.set_u32(pose + 0x10, local);
        e.mem.set_u32(pose + 0x1c, flag_array);
        e.mem.set_u8(pose + 0x28, 1);
        pose
    }

    /// A body whose Havok transform (at `+0x34`) has the translation `position` and the scale.
    fn body_setup(e: &mut Engine, position: [f32; 3], scale: f32) -> u32 {
        let body = e.mem.alloc(0x100);
        put_floats(e, body + 0x58, &position);
        e.mem.set_f32(body + 0x64, scale);
        body
    }

    /// A controller for the pose conversions: the pose at `+0x88`, the bodies at `+0xa4`, their
    /// number in the object behind `[[+0x2a4] + 0xc]` and the world scale.
    fn pose_controller(e: &mut Engine, pose: u32, bodies: &[u32], world_scale: f32) -> u32 {
        let this = e.mem.alloc(0x300);
        let world = e.mem.alloc(0x100);
        e.mem.set_f32(world + 0x98, world_scale);
        e.mem.set_u32(this + 0x58, world);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x1c, bodies.len() as u32);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0xc, source);
        e.mem.set_u32(this + 0x2a4, owner);
        let table = e.mem.alloc(4 * bodies.len() as u32);
        for (index, body) in bodies.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *body);
        }
        e.mem.set_u32(this + 0xa4, table);
        e.mem.set_u32(this + 0x88, pose);
        this
    }

    type Mul = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>>;

    /// The `hkQuaternion::setMul` double: logs its arguments and the four dwords of its second.
    fn install_set_mul(e: &mut Engine) -> Mul {
        let seen = Mul::default();
        let kept = Rc::clone(&seen);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            let bits = (0..4).map(|lane| e.mem.u32(a[1] + 4 * lane)).collect();
            kept.borrow_mut().push((a.to_vec(), bits));
            Ret::default()
        });
        seen
    }

    #[test]
    fn get_sg_pose_local_space_converts_the_bodies_and_composes_flagged_children() {
        let mut e = engine();
        let log = CallLog::default();
        install_ni2hk(&mut e);
        set_pose_constants(&mut e);
        recorder(&mut e, &log, SYNC_MODEL_SPACE, 0);
        recorder(&mut e, &log, MEMCPY, 0);
        let transforms = e.mem.alloc(0x80);
        let cell = e.mem.alloc(0x10);
        e.mem.set_u32(cell, transforms);
        recorder(&mut e, &log, GET_SYNCED_POSE_LOCAL_SPACE, cell);
        let products = install_set_mul(&mut e);
        let pose = pose_setup(&mut e, &[-1, 0], &[0, 1]);
        let output = e.mem.u32(pose + 4);
        let local = e.mem.u32(pose + 0x10);
        put_floats(&mut e, local, &[1.0, 2.0, 3.0, 0.0]);
        put_floats(&mut e, local + 0x20, &[2.0, 2.0, 2.0, 1.0]);
        put_floats(&mut e, local + 0x30, &[5.0, 5.0, 5.0, 0.0]);
        put_floats(&mut e, local + 0x50, &[3.0, 3.0, 3.0, 1.0]);
        let body = body_setup(&mut e, [2.0, 4.0, 6.0], 3.0);
        let this = pose_controller(&mut e, pose, &[body], 2.0);
        let destination = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x94, destination);
        e.call(0x00c79680, &args![this]);
        assert_eq!(floats(&e, output, 4), vec![2.0, 4.0, 6.0, 0.0]);
        assert_eq!(floats(&e, output + 0x20, 4), vec![3.0; 4]);
        let child = output + 0x30;
        assert_eq!(floats(&e, child, 4), vec![4.0, 3.0, 2.0, 0.0]);
        let scale = floats(&e, child + 0x20, 4);
        assert!(scale[..3].iter().all(|lane| (lane - 1.5).abs() < 1e-4));
        assert_eq!(scale[3], 0.0);
        let products = products.borrow();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].0[0], child + 0x10);
        assert_eq!(products[0].0[2], local + 0x30 + 0x10);
        assert_eq!(
            products[0].1,
            vec![0x8000_0000; 3]
                .into_iter()
                .chain([1.0f32.to_bits()])
                .collect::<Vec<_>>()
        );
        let flags = e.mem.u32(pose + 0x1c);
        assert_eq!((e.mem.u32(flags), e.mem.u32(flags + 4)), (2, 2));
        assert_eq!(e.mem.u8(pose + 0x28), 0);
        assert_eq!(calls_to(&log, SYNC_MODEL_SPACE), vec![vec![pose]]);
        assert_eq!(
            calls_to(&log, MEMCPY),
            vec![vec![destination, transforms, 0x30]]
        );
    }

    #[test]
    fn set_sg_pose_local_space_composes_flagged_bones_and_stores_the_body_transforms() {
        let mut e = engine();
        let log = CallLog::default();
        set_pose_constants(&mut e);
        install_sqrt(&mut e, &log);
        install_set_mul(&mut e);
        recorder(&mut e, &log, SYNC_LOCAL_SPACE, 0);
        e.register_double(0x004f_0180, |e, a| {
            for word in 0..9 {
                e.mem.set_u32(a[1] + 4 * word, 0x100 + word);
            }
            Ret::default()
        });
        e.register_double(POINT_COPY, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        let pose = pose_setup(&mut e, &[-1, 0], &[1, 1]);
        let output = e.mem.u32(pose + 4);
        let local = e.mem.u32(pose + 0x10);
        put_floats(&mut e, local, &[1.0, 2.0, 3.0, 0.0]);
        put_floats(&mut e, local + 0x20, &[2.0, 2.0, 2.0, 1.0]);
        put_floats(&mut e, local + 0x30, &[5.0, 5.0, 5.0, 0.0]);
        put_floats(&mut e, local + 0x50, &[3.0, 3.0, 3.0, 1.0]);
        let bodies = [
            body_setup(&mut e, [0.0; 3], 1.0),
            body_setup(&mut e, [0.0; 3], 1.0),
        ];
        let record = e.mem.alloc(0x40);
        recorder(&mut e, &log, 0x0900_003c, record);
        let object = object_with_vtable(&mut e, &[(0x3c, 0x0900_003c)]);
        e.mem.set_u32(bodies[0] + 0x1c, object);
        let this = pose_controller(&mut e, pose, &bodies, 2.0);
        e.call(0x00c79a50, &args![this]);
        assert_eq!(calls_to(&log, SYNC_LOCAL_SPACE), vec![vec![pose]]);
        assert_eq!(floats(&e, output, 3), vec![1.0, 2.0, 3.0]);
        assert_eq!(floats(&e, output + 0x30, 4), vec![4.0, 3.0, 2.0, 0.0]);
        let translations: Vec<Vec<f32>> = bodies
            .iter()
            .map(|body| floats(&e, body + 0x58, 3))
            .collect();
        assert_eq!(translations, vec![vec![0.5, 1.0, 1.5], vec![2.0, 1.5, 1.0]]);
        for body in bodies {
            let rotation: Vec<u32> = (0..9)
                .map(|word| e.mem.u32(body + 0x34 + 4 * word))
                .collect();
            assert_eq!(
                rotation,
                (0..9).map(|word| 0x100 + word).collect::<Vec<_>>()
            );
        }
        let flags = e.mem.u32(pose + 0x1c);
        assert_eq!((e.mem.u32(flags), e.mem.u32(flags + 4)), (0, 0));
        assert_eq!(calls_to(&log, 0x0900_003c).len(), 1);
        assert_eq!(e.mem.u8(record + 0x14), 1);
    }

    #[test]
    fn do_ragdoll_feedback_blends_the_listed_bodies_and_maps_the_poses() {
        let mut e = engine();
        let log = CallLog::default();
        let cursor = e.mem.alloc(0x800);
        let (router, _) = install_router(&mut e, &log, cursor);
        install_timer(&mut e, 100);
        for double in [
            GET_POSE_MODEL_SPACE,
            MODEL_TO_LOCAL_POSE,
            LOCAL_TO_MODEL_POSE,
            MAP_POSE,
            POSE_MODEL_SPACE_CHANGED,
        ] {
            recorder(&mut e, &log, double, 0);
        }
        let cells: Vec<u32> = [0x1111_0000u32, 0x2222_1000, 0x3333_2000]
            .iter()
            .map(|&value| {
                let cell = e.mem.alloc(8);
                e.mem.set_u32(cell, value);
                cell
            })
            .collect();
        recorder(&mut e, &log, GET_SYNCED_POSE_MODEL_SPACE, cells[0]);
        recorder(&mut e, &log, ACCESS_SYNCED_POSE_MODEL_SPACE, cells[1]);
        recorder(&mut e, &log, GET_SYNCED_POSE_LOCAL_SPACE, cells[2]);
        let slerps = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&slerps);
        e.register_double(SET_SLERP, move |e, a| {
            kept.borrow_mut().push(a.to_vec());
            for lane in 0..4 {
                e.mem.set_f32(a[0] + 4 * lane, 9.0 + lane as f32);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x300);
        let list = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        e.mem.set_u32(skeleton + 0x1c, 3);
        e.mem.set_u32(skeleton + 0xc, 0x5555);
        e.mem.set_u32(skeleton + 0x24, 0x6666);
        e.mem.set_u32(list + 0xc, 2);
        e.mem.set_u32(list + 0x2c, skeleton);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x10, 0x3a3a);
        e.mem.set_u32(owner + 0x14, 0x3b3b);
        let table = e.mem.alloc(0x60);
        let data = e.mem.alloc(0x10);
        e.mem.set_u16(data, 7);
        e.mem.set_u16(data + 2, 1);
        e.mem.set_u32(table + 0x44, data);
        e.mem.set_u32(table + 0x48, 2);
        e.mem.set_u32(owner + 0x18, table);
        e.mem.set_u32(this + 0x48, list);
        e.mem.set_u32(this + 0x88, 0xa5a5);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_f32(this + 0x290, 0.25);
        e.mem.set_u32(this + 0x2b0, 1000);
        e.call(0x00c7c870, &args![this]);
        let (first, second, third) = (cursor, cursor + 0x100, cursor + 0x200);
        assert_eq!(
            calls_to(&log, GET_POSE_MODEL_SPACE),
            vec![vec![list, second, this + 0x10]]
        );
        assert_eq!(
            calls_to(&log, MODEL_TO_LOCAL_POSE),
            vec![
                vec![3, 0x5555, second, first],
                vec![3, 0x5555, third, second]
            ]
        );
        assert_eq!(
            calls_to(&log, LOCAL_TO_MODEL_POSE),
            vec![
                vec![3, 0x5555, 0x6666, third],
                vec![3, 0x5555, second, third]
            ]
        );
        assert_eq!(
            calls_to(&log, MAP_POSE),
            vec![
                vec![0x3a3a, 0x1111_0000, 0x6666, third, 2],
                vec![0x3b3b, third, 0x3333_2000, 0x2222_1000, 2]
            ]
        );
        assert_eq!(calls_to(&log, POSE_MODEL_SPACE_CHANGED), vec![vec![0xa5a5]]);
        let slerps = slerps.borrow();
        assert_eq!(slerps.len(), 1);
        assert_eq!(
            slerps[0][1..],
            [first + 0x40, second + 0x40, 0.25f32.to_bits()]
        );
        assert_eq!(floats(&e, second + 0x40, 4), vec![9.0, 10.0, 11.0, 12.0]);
        assert_eq!(e.mem.u32(router), cursor);
    }

    #[test]
    fn do_ragdoll_feedback_switches_the_feedback_off_when_the_time_is_up() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, bodies, _, _) = feedback_setup(&mut e, &log);
        e.mem.set_u8(this + 0x28e, 1);
        e.mem.set_u32(this + 0x2b0, 9);
        e.call(0x00c7c870, &args![this]);
        assert_eq!(e.mem.u8(this + 0x28e), 0);
        assert_eq!(
            calls_to(&log, SET_MOTION_TYPE),
            vec![vec![bodies[0], 4, 1, 0], vec![bodies[1], 4, 1, 0]]
        );
        assert!(calls_to(&log, GET_POSE_MODEL_SPACE).is_empty());
    }

    /// A line object with its data block present and, when `geometry` is set, the geometry the
    /// debug lines write their colours (`+0x28`) and vertices (`+0x20`) into.
    fn line_object(e: &mut Engine, geometry: bool) -> u32 {
        let line = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(line + 0x20, data);
        if geometry {
            let block = e.mem.alloc(0x40);
            let vertices = e.mem.alloc(0x20);
            let colors = e.mem.alloc(0x20);
            e.mem.set_u32(block + 0x20, vertices);
            e.mem.set_u32(block + 0x28, colors);
            e.mem.set_u32(line + 0xb8, block);
        }
        line
    }

    /// What `debug_lines_setup` built: the controller, the line objects (world, owner, group,
    /// bone line), the list, the body of the list, the entry, the pose block and the allocator.
    struct DebugLines {
        this: u32,
        objects: [u32; 4],
        list: u32,
        body: u32,
        entry: u32,
        block: u32,
        allocator: u32,
    }

    /// A controller for `00c7cb50` (stage 1): two bones with the parents `[-1, 0]`, one line
    /// object for the second bone, a pose block holding the bones at `(10, 20, 30)` and
    /// `(40, 50, 60)`, the world scale 2, a body table whose bone 1 slot is negative and the
    /// router/allocator doubles.
    fn debug_lines_setup(e: &mut Engine, log: &CallLog) -> DebugLines {
        let this = e.mem.alloc(0x300);
        let objects = [
            line_object(e, false),
            line_object(e, false),
            line_object(e, false),
            line_object(e, true),
        ];
        e.mem.set_f32(objects[0] + 0x98, 2.0);
        e.mem.set_u32(this + 0x58, objects[0]);
        e.mem.set_u32(this + 0x84, objects[1]);
        let entry = object_with_vtable(e, &[(0xc, 0x0900_000c)]);
        recorder(e, log, 0x0900_000c, objects[2]);
        let entries = e.mem.alloc(0x10);
        e.mem.set_u32(entries + 4, entry);
        e.mem.set_u32(objects[1] + 0xa0, entries);
        let line_array = e.mem.alloc(0x10);
        e.mem.set_u32(line_array, objects[3]);
        e.mem.set_u32(objects[2] + 0xa0, line_array);
        let skeleton = e.mem.alloc(0x40);
        let parents = e.mem.alloc(0x10);
        e.mem.set_i16(parents, -1);
        e.mem.set_i16(parents + 2, 0);
        e.mem.set_u32(skeleton + 0xc, parents);
        e.mem.set_u32(skeleton + 0x1c, 2);
        let mappers = e.mem.alloc(0x40);
        e.mem.set_u32(mappers + 8, skeleton);
        e.mem.set_u32(this + 0x2a4, mappers);
        let list = e.mem.alloc(0x40);
        let slots = e.mem.alloc(0x10);
        e.mem.set_i32(slots, -1);
        e.mem.set_i32(slots + 4, -1);
        e.mem.set_u32(list + 0x20, slots);
        let bodies = e.mem.alloc(0x10);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(bodies, body);
        e.mem.set_u32(list + 8, bodies);
        e.mem.set_u32(this + 0x48, list);
        let block = e.mem.alloc(0x60);
        put_floats(e, block, &[10.0, 20.0, 30.0, 0.0]);
        put_floats(e, block + 0x30, &[40.0, 50.0, 60.0, 0.0]);
        let cursor = e.mem.alloc(0x100);
        let (_, allocator) = install_router(e, log, cursor);
        recorder(e, log, 0x0900_a00c, block);
        recorder(e, log, GET_POSE_MODEL_SPACE, 0);
        e.register_double(POINT_COPY, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.mem.set_f32(LINE_DEFAULT_WEIGHT, 0.5);
        for (lane, value) in [0x11u32, 0x22, 0x33, 0x44].into_iter().enumerate() {
            e.mem.set_u32(LINE_ACTIVE_COLOR + 4 * lane as u32, value);
        }
        DebugLines {
            this,
            objects,
            list,
            body,
            entry,
            block,
            allocator,
        }
    }

    #[test]
    fn update_debug_lines_does_nothing_without_the_owner_or_when_the_entry_is_flagged() {
        let mut e = engine();
        let log = CallLog::default();
        let DebugLines {
            this,
            objects,
            entry,
            ..
        } = debug_lines_setup(&mut e, &log);
        e.mem.set_u32(this + 0x84, 0);
        e.call(0x00c7cb50, &args![this, 1u32]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(this + 0x84, objects[1]);
        e.mem.set_u8(entry + 0x30, 1);
        e.call(0x00c7cb50, &args![this, 1u32]);
        assert_eq!(calls_to(&log, 0x0900_000c).len(), 1);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(objects[0] + 0x30), 0);
    }

    #[test]
    fn update_debug_lines_places_the_bone_lines_and_colours_them() {
        let mut e = engine();
        let log = CallLog::default();
        let DebugLines {
            this,
            objects,
            list,
            body,
            block,
            allocator,
            ..
        } = debug_lines_setup(&mut e, &log);
        e.call(0x00c7cb50, &args![this, 1u32]);
        for object in objects {
            assert_eq!(e.mem.u32(object + 0x30) & 0x800, 0x800);
            assert_eq!(e.mem.f32(e.mem.u32(object + 0x20) + 0xc), 1.0);
        }
        assert_eq!(
            calls_to(&log, GET_POSE_MODEL_SPACE),
            vec![vec![list, block, this + 0x10]]
        );
        let geometry = e.mem.u32(objects[3] + 0xb8);
        let vertices = e.mem.u32(geometry + 0x20);
        let colors = e.mem.u32(geometry + 0x28);
        assert_eq!(
            floats(&e, vertices, 6),
            vec![5.0, 10.0, 15.0, 20.0, 25.0, 30.0]
        );
        let weight = 0.5f32.to_bits();
        let default: Vec<u32> = (0..8).map(|word| e.mem.u32(colors + 4 * word)).collect();
        assert_eq!(
            default,
            vec![0, weight, weight, weight, 0, weight, weight, weight]
        );
        assert_eq!(
            calls_to(&log, ALLOCATOR_FREE),
            vec![vec![allocator, block, 0x60]]
        );
        let slots = e.mem.u32(list + 0x20);
        let entries = e.mem.u32(list + 8);
        e.mem.set_i32(slots + 4, 0);
        e.mem.set_u32(entries, body);
        e.mem.set_u8(body + 0xe8, 4);
        e.call(0x00c7cb50, &args![this, 1u32]);
        let active: Vec<u32> = (0..8).map(|word| e.mem.u32(colors + 4 * word)).collect();
        assert_eq!(active, vec![0x11, 0x22, 0x33, 0x44, 0x11, 0x22, 0x33, 0x44]);
        assert_eq!(calls_to(&log, ALLOCATOR_FREE).len(), 2);
    }

    #[test]
    fn do_ragdoll_anim_clears_the_flags_when_nothing_needs_the_pose() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        e.mem.set_u8(this + 0x43, 1);
        e.mem.set_u8(this + 0x41, 1);
        e.mem.set_u8(this + 0xb3, 1);
        e.call(0x00c7d630, &args![this, 0u32]);
        assert_eq!((e.mem.u8(this + 0x43), e.mem.u8(this + 0x41)), (1, 1));
        e.mem.set_u8(this + 0xb3, 0);
        e.call(0x00c7d630, &args![this, 0u32]);
        assert_eq!((e.mem.u8(this + 0x43), e.mem.u8(this + 0x41)), (0, 0));
    }

    #[test]
    fn do_ragdoll_anim_builds_the_transform_and_runs_the_enabled_passes() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, list, cursor, router, _) = rigid_body_setup(&mut e, &log, 0);
        install_ni2hk(&mut e);
        let captured = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&captured);
        e.register_double(0x00a6_df40, move |e, a| {
            let words: Vec<u32> = (0..13).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            kept.borrow_mut().push(words);
            Ret::default()
        });
        let product = e.mem.alloc(0x30);
        for word in 0..9 {
            e.mem.set_u32(product + 4 * word, 0x200 + word);
        }
        recorder(&mut e, &log, MATRIX_MULTIPLY, product);
        recorder(&mut e, &log, WORLD_TRANSFORM_FETCH, 0);
        let world = e.mem.alloc(0x100);
        let owner = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x18, owner);
        e.mem.set_u32(this + 0x58, world);
        e.mem.set_u8(0x0126_7c28, 1);
        e.mem.set_u8(this + 0x41, 1);
        e.mem.set_u8(this + 0x40, 1);
        e.call(0x00c7d630, &args![this, 0u32]);
        let fetches = calls_to(&log, WORLD_TRANSFORM_FETCH);
        assert_eq!(fetches.len(), 1);
        assert_eq!((fetches[0][0], fetches[0][2]), (owner + 0x68, world + 0x34));
        let multiplies = calls_to(&log, MATRIX_MULTIPLY);
        assert_eq!(multiplies.len(), 1);
        assert_eq!(multiplies[0][0], this + 0x60);
        assert_eq!(multiplies[0][2], fetches[0][1]);
        let words = captured.borrow();
        assert_eq!(words.len(), 1);
        assert_eq!(
            words[0][..9],
            (0..9).map(|word| 0x200 + word).collect::<Vec<_>>()
        );
        assert_eq!(words[0][12], 1.0f32.to_bits());
        let skeleton = e.mem.u32(list + 0x2c);
        let local = e.mem.u32(skeleton + 0x24);
        assert_eq!(
            calls_to(&log, LOCAL_TO_MODEL_POSE),
            vec![vec![2, 0x5555, local, cursor + 0x100]]
        );
        assert_eq!(
            calls_to(&log, MODEL_TO_LOCAL_POSE),
            vec![vec![2, 0x5555, cursor + 0x100, cursor + 0x80]]
        );
        assert_eq!(e.mem.u32(router), cursor);
        assert!(calls_to(&log, SYNC_MODEL_SPACE).is_empty());
    }

    #[test]
    fn do_ragdoll_anim_runs_the_local_pose_passes_when_the_controller_is_not_frozen() {
        let mut e = engine();
        let log = CallLog::default();
        let (this, _, cursor, router, _) = rigid_body_setup(&mut e, &log, 0);
        install_ni2hk(&mut e);
        recorder(&mut e, &log, WORLD_TRANSFORM_FETCH, 0);
        recorder(&mut e, &log, SYNC_MODEL_SPACE, 0);
        recorder(&mut e, &log, SYNC_LOCAL_SPACE, 0);
        recorder(&mut e, &log, MEMCPY, 0);
        recorder(&mut e, &log, UPDATE_SYNC_TRANSFORM, 0);
        let product = e.mem.alloc(0x30);
        recorder(&mut e, &log, MATRIX_MULTIPLY, product);
        let transforms = e.mem.alloc(0x10);
        let cell = e.mem.alloc(0x10);
        e.mem.set_u32(cell, transforms);
        recorder(&mut e, &log, GET_SYNCED_POSE_LOCAL_SPACE, cell);
        let world = e.mem.alloc(0x100);
        let owner = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0x18, owner);
        e.mem.set_u32(this + 0x58, world);
        let source = e.mem.alloc(0x40);
        let mappers = e.mem.u32(this + 0x2a4);
        e.mem.set_u32(mappers + 0xc, source);
        e.mem.set_u8(0x0126_7c28, 1);
        e.mem.set_u8(0x0126_7c4c, 1);
        e.mem.set_u8(this + 0x41, 1);
        e.mem.set_u8(this + 0xb3, 1);
        e.call(0x00c7d630, &args![this, 0u32]);
        assert_eq!(calls_to(&log, SYNC_MODEL_SPACE), vec![vec![0xa5a5]]);
        assert_eq!(calls_to(&log, SYNC_LOCAL_SPACE), vec![vec![0xa5a5]]);
        assert_eq!(
            calls_to(&log, UPDATE_SYNC_TRANSFORM),
            vec![vec![this, this + 0x10]]
        );
        assert_eq!(calls_to(&log, MEMCPY).len(), 1);
        assert_eq!(e.mem.u32(router), cursor);
    }

    /// What `mapper_setup` built: the controller, the two mappers' record blocks, the body
    /// slot table, the body entries and the addresses of the bones the warning names.
    struct Mappers {
        this: u32,
        records: [u32; 2],
        table: u32,
        entries: u32,
        bones: [u32; 2],
    }

    /// A controller for `00c79e20` with two mappers holding `counts` records each. Every record
    /// is `(bone 1, bone 2)` with the lanes `(0.3, 0.4, 0, 0)`; the second bone has its flag byte
    /// set; all body slots are `-1`; the builder, logger and cast doubles are installed.
    fn mapper_setup(e: &mut Engine, log: &CallLog, counts: [i32; 2]) -> Mappers {
        let this = e.mem.alloc(0x300);
        let list = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x10);
        for slot in 0..4 {
            e.mem.set_i32(table + 4 * slot, -1);
        }
        let entries = e.mem.alloc(0x10);
        e.mem.set_u32(list + 0x20, table);
        e.mem.set_u32(list + 8, entries);
        e.mem.set_u32(this + 0x48, list);
        let skeletons = [e.mem.alloc(0x40), e.mem.alloc(0x40)];
        let bones = [e.mem.alloc(0x40), e.mem.alloc(0x40)];
        for (skeleton, bone) in skeletons.iter().zip(bones) {
            e.mem.set_u32(skeleton + 0x18, bone);
        }
        e.mem.set_u8(bones[1] + 2 * 8 + 4, 1);
        let owner = e.mem.alloc(0x40);
        let mut records = [0; 2];
        for (index, count) in counts.into_iter().enumerate() {
            let mapper = e.mem.alloc(0x40);
            e.mem.set_u32(mapper + 0x10, skeletons[0]);
            e.mem.set_u32(mapper + 0x14, skeletons[1]);
            records[index] = e.mem.alloc(0x40);
            e.mem.set_i16(records[index], 1);
            e.mem.set_i16(records[index] + 2, 2);
            put_floats(e, records[index] + 0x10, &[0.3, 0.4, 0.0, 0.0]);
            e.mem.set_u32(mapper + 0x18, records[index]);
            e.mem.set_i32(mapper + 0x1c, count);
            e.mem.set_u32(owner + 0x10 + 4 * index as u32, mapper);
        }
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_f32(0x0101_3ea4, 0.01);
        let logger = object_with_vtable(e, &[(0xc, 0x0900_000c)]);
        e.mem.set_u32(0x0126_8360, logger);
        for double in [0x0900_000c, 0x00cb_1600, 0x00cc_00e0] {
            recorder(e, log, double, 0);
        }
        for double in [0x00cc_01c0, 0x00cc_05a0, 0x00cc_0320] {
            recorder(e, log, double, 0x77);
        }
        recorder(e, log, 0x0065_3290, 0x55);
        Mappers {
            this,
            records,
            table,
            entries,
            bones: [bones[0] + 8, bones[1] + 16],
        }
    }

    #[test]
    fn check_mappers_for_warnings_answers_true_without_mappers() {
        let mut e = engine();
        let this = e.mem.alloc(0x300);
        assert!(e.call(0x00c7_9e20, &args![this]).bool());
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2a4, owner);
        assert!(e.call(0x00c7_9e20, &args![this]).bool());
        e.mem.set_u32(owner + 0x10, 0x1000);
        assert!(e.call(0x00c7_9e20, &args![this]).bool());
    }

    #[test]
    fn check_mappers_for_warnings_logs_misaligned_first_mapper_records() {
        let mut e = engine();
        let log = CallLog::default();
        let Mappers {
            this,
            records,
            table,
            entries,
            bones,
        } = mapper_setup(&mut e, &log, [1, 0]);
        assert!(e.call(0x00c7_9e20, &args![this]).bool());
        let init = calls_to(&log, 0x00cb_1600);
        assert_eq!(init.len(), 1);
        let (builder, buffer) = (init[0][0], init[0][1]);
        assert_eq!((buffer, init[0][2]), (builder + 0xc, 0x200));
        assert_eq!(
            calls_to(&log, 0x00cc_01c0),
            vec![
                vec![builder, 0x010c_4d7c],
                vec![0x77, 0x010c_4d88],
                vec![0x77, 0x010c_4d90],
                vec![0x77, 0x010c_4db0],
            ]
        );
        assert_eq!(
            calls_to(&log, 0x00cc_05a0),
            vec![vec![0x77, bones[0]], vec![0x77, bones[1]]]
        );
        let squares = [0.3f32 * 0.3, 0.4 * 0.4, 0.0];
        let length = (squares[2] + (squares[1] + squares[0])).sqrt();
        assert_eq!(
            calls_to(&log, 0x00cc_0320),
            vec![vec![0x77, length.to_bits()]]
        );
        let logger = e.mem.u32(0x0126_8360);
        assert_eq!(
            calls_to(&log, 0x0900_000c),
            vec![vec![logger, 1, 0xbadb_adba, buffer, 0x010c_4d38, 0xc1f]]
        );
        assert_eq!(calls_to(&log, 0x00cc_00e0), vec![vec![builder]]);
        assert!(calls_to(&log, 0x0065_3290).is_empty());
        put_floats(&mut e, records[0] + 0x10, &[0.005, 0.0, 0.0, 0.0]);
        log.borrow_mut().clear();
        assert!(!e.call(0x00c7_9e20, &args![this]).bool());
        assert!(log.borrow().is_empty());
        put_floats(&mut e, records[0] + 0x10, &[0.3, 0.4, 0.0, 0.0]);
        e.mem.set_i32(table + 8, 0);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 0xc, 0x4242);
        e.mem.set_u32(entries, entry);
        assert!(!e.call(0x00c7_9e20, &args![this]).bool());
        assert_eq!(calls_to(&log, 0x0065_3290), vec![vec![0x4242, 0x0126_83c8]]);
        assert!(calls_to(&log, 0x00cb_1600).is_empty());
    }

    #[test]
    fn check_mappers_for_warnings_walks_the_type_chain_for_the_second_mapper() {
        let mut e = engine();
        let log = CallLog::default();
        let Mappers {
            this,
            table,
            entries,
            ..
        } = mapper_setup(&mut e, &log, [0, 1]);
        let chain = e.mem.alloc(0x10);
        recorder(&mut e, &log, 0x0900_0008, chain);
        let object = object_with_vtable(&mut e, &[(8, 0x0900_0008)]);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 0xc, object);
        e.mem.set_u32(entries, entry);
        e.mem.set_i32(table + 4, 0);
        assert!(e.call(0x00c7_9e20, &args![this]).bool());
        let logger = e.mem.u32(0x0126_8360);
        let logged = calls_to(&log, 0x0900_000c);
        assert_eq!(logged.len(), 1);
        assert_eq!((logged[0][0], logged[0][5]), (logger, 0xc39));
        log.borrow_mut().clear();
        e.mem.set_u32(chain + 4, 0x0126_83c8);
        assert!(!e.call(0x00c7_9e20, &args![this]).bool());
        assert_eq!(calls_to(&log, 0x0900_0008), vec![vec![object]]);
        assert!(calls_to(&log, 0x00cb_1600).is_empty());
    }
}

#[cfg(test)]
mod quaternion_limit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const BONE_OBJECT: u32 = 0x0f00_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ARC_COSINE_ONE, 1.0f32);
        e.set_global(ARC_COSINE_PI, std::f32::consts::PI);
        e.set_global(QUATERNION_TOLERANCE, 0.001f32);
        e.set_global(ROOT_THREE, 3.0f32);
        e.set_global(ROOT_HALF, 0.5f32);
        e.set_global(VECTOR_FILL, 0.0f32);
        e.set_global(DEGREES_TO_RADIANS, 0.017453292519943295f64);
        e.set_global(ANGLE_LIMIT_SOFT, 10.0f32);
        e.set_global(ANGLE_LIMIT_HARD, 90.0f32);
        for lane in 0..4 {
            e.mem.set_u32(QUATERNION_ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_u32(SIGN_LANES + 4 * lane, 0x8000_0000);
        }
        for (lane, value) in [0.0f32, 0.0, 0.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(0x010c_71b0 + 4 * lane as u32, value);
        }
        e.register(CRT_ARC_COSINE, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).acos(),
            ..Ret::default()
        });
        e
    }

    fn vec4(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let address = e.mem.alloc(0x10);
        store4(e, address, lanes);
        address
    }

    fn close(actual: [f32; 4], expected: [f32; 4]) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-5,
                "{actual:?} against {expected:?}"
            );
        }
    }

    #[test]
    fn arc_cosine_returns_exact_limits_instead_of_nan() {
        let mut e = engine();
        let acos = |e: &mut Engine, x: f32| e.call(0x00c7_4a70, &args![x]).f32();
        assert!((acos(&mut e, 0.5) - std::f32::consts::FRAC_PI_3).abs() < 1e-6);
        assert_eq!(acos(&mut e, 1.5), 0.0);
        assert_eq!(acos(&mut e, 1.0), 0.0);
        assert_eq!(acos(&mut e, -1.0), std::f32::consts::PI);
        assert_eq!(acos(&mut e, -3.0), std::f32::consts::PI);
        // A NaN goes to the CRT function (and stays a NaN).
        assert!(acos(&mut e, f32::NAN).is_nan());
    }

    #[test]
    fn the_quaternion_angle_is_twice_the_arc_cosine_of_the_absolute_fourth_lane() {
        let mut e = engine();
        let q = vec4(&mut e, [0.0, 0.0, 0.8, -0.5]);
        let angle = e.call(0x00c7_4c20, &args![q]).f32();
        assert!((angle - 2.0943952).abs() < 1e-6);
        let q = vec4(&mut e, [0.0, 0.0, 0.0, -1.5]);
        assert_eq!(e.call(0x00c7_4c20, &args![q]).f32(), 0.0);
    }

    #[test]
    fn the_inverse_product_of_two_quaternions() {
        let mut e = engine();
        let a = vec4(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = vec4(&mut e, [0.5, -1.0, 2.0, 3.0]);
        let out = vec4(&mut e, [9.0; 4]);
        e.call(0x00c7_4ac0, &args![out, a, b]);
        close(load4(&e, out), [-6.0, 10.5, 3.0, 16.5]);
    }

    #[test]
    fn normalising_flips_the_sign_when_the_fourth_lane_is_negative() {
        let mut e = engine();
        let v = vec4(&mut e, [0.0, 0.0, 3.0, -4.0]);
        let out = vec4(&mut e, [9.0; 4]);
        e.call(0x00c7_4c90, &args![v, out]);
        // The length is that of the three lowest lanes (3): the vector (0, 0, 1, -4/3) is negated.
        close(load4(&e, out), [-0.0, -0.0, -1.0, 4.0 / 3.0]);
        let v = vec4(&mut e, [0.0, 0.0, 3.0, 4.0]);
        e.call(0x00c7_4c90, &args![v, out]);
        close(load4(&e, out), [0.0, 0.0, 1.0, 4.0 / 3.0]);
        let v = vec4(&mut e, [0.0; 4]);
        e.call(0x00c7_4c90, &args![v, out]);
        assert_eq!(load4(&e, out), [0.0; 4]);
    }

    #[test]
    fn the_enable_byte_sets_the_flag_and_resets_the_quaternions_when_clear() {
        let mut e = engine();
        let this = e.mem.alloc(0x200);
        e.mem.set_u8(this + 0xb1, 1);
        e.call(0x00c7_5580, &args![this, 1u8]);
        assert_eq!(e.mem.u8(this + 0xb3), 1);
        assert_eq!(e.mem.u8(this + 0x180), 0);
        e.mem.set_u8(this + 0x43, 1);
        e.mem.set_u8(this + 0xb3, 1);
        // With +0x43 set the flag keeps its value; a clear enable byte resets the quaternions.
        e.call(0x00c7_5580, &args![this, 0u8]);
        assert_eq!(e.mem.u8(this + 0xb3), 1);
        assert_eq!(load4(&e, this + 0x120), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(load4(&e, this + 0x170), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!((e.mem.u8(this + 0x180), e.mem.u8(this + 0x130)), (1, 1));
        e.mem.set_u8(this + 0x43, 0);
        e.call(0x00c7_5580, &args![this, 0u8]);
        assert_eq!(e.mem.u8(this + 0xb3), 0);
    }

    /// A controller with the bone record of `index` holding the identity quaternion, and the doubles
    /// for the callees of `00c755e0`.
    fn turning(index: u32) -> (Engine, Seen, u32, u32) {
        let mut e = engine();
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let this = e.mem.alloc(0x400);
        let entry = this + index * 0x50;
        store4(&mut e, entry + 0x120, [0.0, 0.0, 0.0, 1.0]);
        e.mem.set_u32(entry + 0xf0, BONE_OBJECT);
        e.mem.set_u16(entry + 0xf4, 0xfffe);
        let transform = e.mem.alloc(0x40);
        let log = Rc::clone(&seen);
        e.register_double(POSE_BONE_TRANSFORM, move |_, a| {
            log.borrow_mut().push((POSE_BONE_TRANSFORM, a.to_vec()));
            transform.into_ret()
        });
        for address in [SET_AXIS_ANGLE, NORMALISE_VECTOR] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..4).map(|lane| {
                    e.mem
                        .u32(a[1 - (address == NORMALISE_VECTOR) as usize] + 4 * lane)
                }));
                log.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        // The product copies the turn (first operand): current stays the identity.
        let log = Rc::clone(&seen);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            log.borrow_mut().push((QUATERNION_SET_MUL, a.to_vec()));
            let lanes = load4(e, a[1]);
            store4(e, a[0], lanes);
            Ret::default()
        });
        (e, seen, this, transform)
    }

    fn calls(seen: &Seen, address: u32) -> Vec<Vec<u32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn a_target_close_to_the_current_quaternion_changes_nothing() {
        let (mut e, seen, this, _) = turning(1);
        let target = vec4(&mut e, [0.0005, 0.0, 0.0, 1.0]);
        assert_eq!(e.call(0x00c7_55e0, &args![this, 1u32, target]).f32(), 0.0);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn a_turn_within_the_limit_is_applied_as_it_is() {
        let (mut e, seen, this, transform) = turning(1);
        // A turn of 0.1 rad about z: below the 10 degree soft limit.
        let half = 0.05f32;
        let target = vec4(&mut e, [0.0, 0.0, half.sin(), half.cos()]);
        let angle = e.call(0x00c7_55e0, &args![this, 1u32, target]).f32();
        assert!((angle - 0.1).abs() < 1e-5);
        assert!(calls(&seen, SET_AXIS_ANGLE).is_empty());
        let entry = this + 0x50;
        close(load4(&e, entry + 0x120), [0.0, 0.0, half.sin(), half.cos()]);
        assert_eq!(load4(&e, transform + 0x10), load4(&e, entry + 0x120));
        // The bone index is the sign-extended 16-bit word at +0xf4 of the record.
        assert_eq!(
            calls(&seen, POSE_BONE_TRANSFORM),
            vec![vec![BONE_OBJECT, 0xffff_fffe]]
        );
        assert_eq!(calls(&seen, NORMALISE_VECTOR).len(), 1);
    }

    #[test]
    fn a_larger_turn_is_limited_to_the_soft_or_the_hard_angle() {
        let (mut e, seen, this, _) = turning(0);
        let half = 0.25f32;
        let target = vec4(&mut e, [0.0, 0.0, half.sin(), half.cos()]);
        let limit = (10.0f64 * 0.017453292519943295) as f32;
        let angle = e.call(0x00c7_55e0, &args![this, 0u32, target]).f32();
        assert_eq!(angle, limit);
        let rebuilt = &calls(&seen, SET_AXIS_ANGLE)[0];
        assert_eq!(f32::from_bits(rebuilt[2]), limit);
        // The axis is the turn's vector part scaled to unit length (the fourth lane follows).
        assert!((f32::from_bits(rebuilt[3 + 2]) - 1.0).abs() < 1e-3);
        // The hard limit applies when the byte at +0xb2 is set: 90 degrees is not reached.
        let (mut e, seen, this, _) = turning(0);
        e.mem.set_u8(this + 0xb2, 1);
        let target = vec4(&mut e, [0.0, 0.0, half.sin(), half.cos()]);
        let angle = e.call(0x00c7_55e0, &args![this, 0u32, target]).f32();
        assert!((angle - 0.5).abs() < 1e-5);
        assert!(calls(&seen, SET_AXIS_ANGLE).is_empty());
    }

    #[test]
    fn the_turn_and_the_axis_sit_in_one_aligned_frame_at_the_exe_offsets() {
        let (mut e, seen, this, _) = turning(0);
        let half = 0.25f32;
        let target = vec4(&mut e, [0.0, 0.0, half.sin(), half.cos()]);
        e.call(0x00c7_55e0, &args![this, 0u32, target]);
        // The turn at frame +0x30 (16-byte aligned), the axis / product at +0x20.
        let rebuilt = &calls(&seen, SET_AXIS_ANGLE)[0];
        let (turn, axis) = (rebuilt[0], rebuilt[1]);
        assert_eq!(turn & 0xf, 0);
        assert_eq!(axis, turn - 0x10);
        assert_eq!(
            calls(&seen, QUATERNION_SET_MUL),
            vec![vec![axis, turn, this + 0x120]]
        );
        assert_eq!(calls(&seen, NORMALISE_VECTOR)[0][0], axis);
        // The frame is given back: the next run uses the same addresses.
        store4(&mut e, this + 0x120, [0.0, 0.0, 0.0, 1.0]);
        e.call(0x00c7_55e0, &args![this, 0u32, target]);
        assert_eq!(calls(&seen, SET_AXIS_ANGLE)[1][0], turn);
    }
}

#[cfg(test)]
mod adjust_target_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Scene {
        e: Engine,
        seen: Seen,
        this: u32,
        bone_transform: u32,
    }

    fn scene(inverse_point: [f32; 4]) -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ROOT_THREE, 3.0f32);
        e.set_global(ROOT_HALF, 0.5f32);
        e.set_global(VECTOR_FILL, 0.0f32);
        e.set_global(TARGET_CLAMP_DISTANCE, 10.0f32);
        e.set_global(TARGET_OFFSET_POSITIVE, 0.5f32);
        e.set_global(TARGET_OFFSET_NEGATIVE, -0.25f32);
        for lane in 0..4 {
            e.mem.set_f32(NEG_HALF_LANES + 4 * lane, -0.5);
        }
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let this = e.mem.alloc(0x400);
        let pose = e.mem.alloc(0x40);
        let transforms = e.mem.alloc(0x100);
        let flags = e.mem.alloc(0x20);
        e.mem.set_u32(pose + 0x10, transforms);
        e.mem.set_u32(pose + 0x1c, flags);
        e.mem.set_u32(this + 0x88, pose);
        e.mem.set_u16(this + 0x144, 1);
        let bone_transform = transforms + 0x30;
        store4(&mut e, bone_transform + 0x10, [0.0, 0.0, 0.0, 1.0]);
        store4(&mut e, this + 0x20, [0.0, 0.0, 0.0, 1.0]);
        // The inverse transform of the target gives `inverse_point`.
        let log = Rc::clone(&seen);
        e.register_double(TRANSFORM_POINT_INVERSE, move |e, a| {
            log.borrow_mut().push((TRANSFORM_POINT_INVERSE, a.to_vec()));
            store4(e, a[0], inverse_point);
            Ret::default()
        });
        // Every forward transform is the identity: the result is the vector.
        let log = Rc::clone(&seen);
        e.register_double(TRANSFORM_POINT, move |e, a| {
            log.borrow_mut().push((TRANSFORM_POINT, a.to_vec()));
            let lanes = load4(e, a[2]);
            store4(e, a[0], lanes);
            Ret::default()
        });
        Scene {
            e,
            seen,
            this,
            bone_transform,
        }
    }

    fn calls(seen: &Seen, address: u32) -> Vec<Vec<u32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn a_target_near_the_anchor_is_left_alone() {
        let mut s = scene([0.0, 3.0, 0.0, 0.0]);
        store4(&mut s.e, s.this + 0xd0, [1.0, 2.0, 3.0, 0.0]);
        s.e.call(0x00c7_8160, &args![s.this]);
        assert_eq!(s.e.mem.u8(s.this + 0x1a4), 0);
        assert_eq!(load4(&s.e, s.this + 0xd0), [1.0, 2.0, 3.0, 0.0]);
        let inverse = calls(&s.seen, TRANSFORM_POINT_INVERSE);
        assert_eq!(
            inverse,
            vec![vec![inverse[0][0], s.this + 0x10, s.this + 0xd0]]
        );
        // The anchor (the zero offset through the two transforms) is made from the bone
        // transform (the entry at index 1 of the pose's array) and then the controller's.
        let forward = calls(&s.seen, TRANSFORM_POINT);
        assert_eq!(forward.len(), 2);
        assert_eq!(forward[0][1], s.bone_transform);
        assert_eq!(forward[1][1], s.this + 0x10);
    }

    #[test]
    fn a_far_target_is_pulled_in_to_the_clamp_distance() {
        let mut s = scene([0.0, 3.0, 0.0, 0.0]);
        store4(&mut s.e, s.this + 0xd0, [0.0, 0.0, 30.0, 0.0]);
        s.e.call(0x00c7_8160, &args![s.this]);
        let target = load4(&s.e, s.this + 0xd0);
        // Ten along the unit direction to the target, from the anchor (the origin).
        assert!((target[2] - 10.0).abs() < 1e-3, "{target:?}");
        assert!(target[0].abs() < 1e-6 && target[1].abs() < 1e-6);
    }

    #[test]
    fn a_normalised_point_below_the_plane_sets_the_flag_and_adds_the_offset() {
        // The inverse point (-1, -1, 0): normalised y is negative; x is negative too.
        let mut s = scene([-1.0, -1.0, 0.0, 0.0]);
        store4(&mut s.e, s.this + 0xd0, [9.0, 9.0, 9.0, 0.0]);
        s.e.call(0x00c7_8160, &args![s.this]);
        assert_eq!(s.e.mem.u8(s.this + 0x1a4), 1);
        // The inverse transform was asked twice; with x below zero the negative float
        // (-0.25) is added to the first coordinate of the transformed offset (0).
        assert_eq!(calls(&s.seen, TRANSFORM_POINT_INVERSE).len(), 2);
        assert_eq!(load4(&s.e, s.this + 0xd0), [-0.25, 0.0, 0.0, 0.0]);
        // A non-negative x takes the positive float instead.
        let mut s = scene([1.0, -1.0, 0.0, 0.0]);
        store4(&mut s.e, s.this + 0xd0, [9.0, 9.0, 9.0, 0.0]);
        s.e.call(0x00c7_8160, &args![s.this]);
        assert_eq!(load4(&s.e, s.this + 0xd0), [0.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn the_hard_mode_adds_three_times_the_rotated_vector() {
        let mut s = scene([0.0, 1.0, 0.0, 0.0]);
        s.e.mem.set_u8(s.this + 0xb2, 1);
        store4(&mut s.e, s.this + 0x150, [1.0, 0.0, 0.0, 0.0]);
        store4(&mut s.e, s.this + 0xd0, [0.0, 0.0, 0.0, 0.0]);
        s.e.call(0x00c7_8160, &args![s.this]);
        // Identity quaternions: rotated vector (1, 0, 0), times 3, onto the zero offset.
        assert_eq!(load4(&s.e, s.this + 0xd0), [3.0, 0.0, 0.0, 0.0]);
        // The inverse transform is asked once only.
        assert_eq!(calls(&s.seen, TRANSFORM_POINT_INVERSE).len(), 1);
    }

    #[test]
    fn the_bone_one_vector_and_the_calculated_transform_are_used_when_asked_for() {
        let mut s = scene([0.0, 1.0, 0.0, 0.0]);
        // The bone's flags ask for `calculateBoneModelSpace`, and bone 1's vector is added.
        let pose = s.e.mem.u32(s.this + 0x88);
        let flags = s.e.mem.u32(pose + 0x1c);
        s.e.mem.set_u32(flags + 4, 2);
        s.e.mem.set_u8(s.this + 0x190, 1);
        s.e.mem.set_u32(s.this + 0xf0, 0x7777);
        let vector = s.e.mem.alloc(0x10);
        store4(&mut s.e, vector, [0.0, 4.0, 0.0, 0.0]);
        let log = Rc::clone(&s.seen);
        s.e.register_double(POSE_BONE_TRANSFORM, move |_, a| {
            log.borrow_mut().push((POSE_BONE_TRANSFORM, a.to_vec()));
            vector.into_ret()
        });
        let calculated = s.e.mem.alloc(0x40);
        let log = Rc::clone(&s.seen);
        s.e.register_double(CALCULATE_BONE_MODEL_SPACE, move |_, a| {
            log.borrow_mut()
                .push((CALCULATE_BONE_MODEL_SPACE, a.to_vec()));
            calculated.into_ret()
        });
        store4(&mut s.e, s.this + 0xd0, [0.0, 4.0, 0.0, 0.0]);
        s.e.call(0x00c7_8160, &args![s.this]);
        assert_eq!(
            calls(&s.seen, CALCULATE_BONE_MODEL_SPACE),
            vec![vec![pose, 1]]
        );
        assert_eq!(calls(&s.seen, POSE_BONE_TRANSFORM), vec![vec![0x7777, 1]]);
        assert_eq!(calls(&s.seen, TRANSFORM_POINT)[0][1], calculated);
        // The target (0, 4, 0) is at the anchor (0, 4, 0): unchanged.
        assert_eq!(load4(&s.e, s.this + 0xd0), [0.0, 4.0, 0.0, 0.0]);
    }

    #[test]
    fn the_locals_sit_in_one_aligned_frame_at_the_exe_offsets() {
        let mut s = scene([-1.0, -1.0, 0.0, 0.0]);
        store4(&mut s.e, s.this + 0xd0, [9.0, 9.0, 9.0, 0.0]);
        s.e.call(0x00c7_8160, &args![s.this]);
        let inverse = calls(&s.seen, TRANSFORM_POINT_INVERSE);
        let forward = calls(&s.seen, TRANSFORM_POINT);
        let first = inverse[0][0];
        // Frame +0x60 (16-byte aligned), the second inverse point at +0x50, the forward
        // transforms' vector at +0x40 (output and input of each).
        assert_eq!(first & 0xf, 0);
        assert_eq!(inverse[1][0], first - 0x10);
        for call in &forward {
            assert_eq!(call[0], first - 0x20);
            assert_eq!(call[2], first - 0x20);
        }
        // The frame is given back: the next run uses the same addresses.
        s.seen.borrow_mut().clear();
        s.e.call(0x00c7_8160, &args![s.this]);
        assert_eq!(calls(&s.seen, TRANSFORM_POINT_INVERSE)[0][0], first);
    }
}

#[cfg(test)]
mod init_bone_params_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const TO_QUATERNION: u32 = 0x00a6_df40;
    const NORMALISE: u32 = 0x0056_11c0;

    struct Scene {
        e: Engine,
        seen: Seen,
        this: u32,
        pose: u32,
        skeleton: u32,
        transforms: u32,
        flags: u32,
        world: u32,
    }

    /// A controller whose skeleton finds `found` for any name and gives it the parent `parent`;
    /// the pose has four bones with identity quaternions.
    fn scene(found: u32, parent: u16) -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(VECTOR_FILL, 0.0f32);
        e.set_global(VECTOR_THIRD, 1.0f32);
        e.set_global(ROOT_THREE, 3.0f32);
        e.set_global(ROOT_HALF, 0.5f32);
        e.set_global(SCALE_FACTOR, 1.0f32);
        for lane in 0..4 {
            e.mem.set_f32(NEG_HALF_LANES + 4 * lane, -0.5);
        }
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let this = e.mem.alloc(0x400);
        let owner = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        let parents = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x2a4, owner);
        e.mem.set_u32(owner + 0xc, skeleton);
        e.mem.set_u32(skeleton + 0xc, parents);
        if found != 0xffff {
            e.mem.set_u16(parents + 2 * found, parent);
        }
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x58, world);
        for word in 0..13 {
            e.mem.set_u32(world + 0x68 + 4 * word, 100 + word);
        }
        e.mem.set_u32(this + 0x88, 0xcafe);
        let pose = e.mem.alloc(0x40);
        let transforms = e.mem.alloc(0x30 * 4);
        let flags = e.mem.alloc(0x20);
        e.mem.set_u32(pose + 0x10, transforms);
        e.mem.set_u32(pose + 0x1c, flags);
        for bone in 0..4 {
            store4(
                &mut e,
                transforms + 0x30 * bone + 0x10,
                [0.0, 0.0, 0.0, 1.0],
            );
        }
        let log = Rc::clone(&seen);
        e.register_double(FIND_BONE_WITH_NAME, move |_, a| {
            log.borrow_mut().push((FIND_BONE_WITH_NAME, a.to_vec()));
            // The upper half of the answer is whatever EAX held.
            (0xdead_0000 | found).into_ret()
        });
        let product = e.mem.alloc(0x30);
        for word in 0..9 {
            e.mem.set_u32(product + 4 * word, 200 + word);
        }
        let log = Rc::clone(&seen);
        e.register_double(MATRIX_MULTIPLY, move |_, a| {
            log.borrow_mut().push((MATRIX_MULTIPLY, a.to_vec()));
            product.into_ret()
        });
        let log = Rc::clone(&seen);
        e.register_double(TO_QUATERNION, move |e, a| {
            let words = (0..13).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            log.borrow_mut().push((TO_QUATERNION, words));
            Ret::default()
        });
        e.register_double(NORMALISE, |_, _| Ret::default());
        Scene {
            e,
            seen,
            this,
            pose,
            skeleton,
            transforms,
            flags,
            world,
        }
    }

    fn calls(seen: &Seen, address: u32) -> Vec<Vec<u32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn close(actual: [f32; 4], expected: [f32; 4], tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < tolerance,
                "{actual:?} against {expected:?}"
            );
        }
    }

    #[test]
    fn a_missing_bone_only_clears_the_byte() {
        let mut s = scene(0xffff, 0);
        s.e.mem.set_u8(s.this + 0xb2, 1);
        s.e.mem.set_u16(s.this + 0x50 + 0xf6, 0x7777);
        let answer =
            s.e.call(0x00c7_9340, &args![s.this, s.pose, 0x1234u32, 1u32]);
        assert_eq!(answer.u32() & 0xff, 0);
        assert_eq!(s.e.mem.u16(s.this + 0x50 + 0xf4), 0xffff);
        assert_eq!(s.e.mem.u16(s.this + 0x50 + 0xf6), 0x7777);
        assert_eq!(s.e.mem.u8(s.this + 0xb2), 0);
        assert_eq!(
            calls(&s.seen, FIND_BONE_WITH_NAME),
            vec![vec![s.skeleton, 0x1234, 0]]
        );
        assert!(calls(&s.seen, MATRIX_MULTIPLY).is_empty());
        assert_eq!(s.e.mem.u32(s.this + (5 + 0xf) * 0x10), 0);
    }

    #[test]
    fn a_missing_parent_stores_the_index_and_stops() {
        let mut s = scene(2, 0xffff);
        s.e.mem.set_u8(s.this + 0xb2, 1);
        let answer =
            s.e.call(0x00c7_9340, &args![s.this, s.pose, 0x1234u32, 0u32]);
        assert_eq!(answer.u32() & 0xff, 0);
        assert_eq!(s.e.mem.u16(s.this + 0xf4), 2);
        assert_eq!(s.e.mem.u16(s.this + 0xf6), 0xffff);
        assert_eq!(s.e.mem.u8(s.this + 0xb2), 0);
        assert!(calls(&s.seen, MATRIX_MULTIPLY).is_empty());
        assert!(calls(&s.seen, TO_QUATERNION).is_empty());
        assert_eq!(s.e.mem.u32(s.this + 0xf0), 0);
    }

    #[test]
    fn the_bone_and_parent_axes_are_turned_by_their_quaternions() {
        let mut s = scene(2, 1);
        let entry = s.this + 2 * 0x50;
        // The bone turn is not of unit length: the result is (4, 3, 0, 0) before normalising.
        store4(
            &mut s.e,
            s.transforms + 0x60 + 0x10,
            [0.0, 0.0, std::f32::consts::SQRT_2, std::f32::consts::SQRT_2],
        );
        let half = std::f32::consts::FRAC_1_SQRT_2;
        store4(&mut s.e, s.transforms + 0x30 + 0x10, [0.0, 0.0, half, half]);
        let answer =
            s.e.call(0x00c7_9340, &args![s.this, s.pose, 0x1234u32, 2u32]);
        assert_eq!(answer.u32() & 0xff, 1);
        assert_eq!(s.e.mem.u16(entry + 0xf4), 2);
        assert_eq!(s.e.mem.u16(entry + 0xf6), 1);
        assert_eq!(s.e.mem.u8(s.this + 0xb2), 0);
        // The parent turn of 90 degrees takes (0, 1, 0) to (1, 0, 0) (the inverse rotation).
        close(load4(&s.e, entry + 0x110), [1.0, 0.0, 0.0, 0.0], 1e-5);
        close(load4(&s.e, entry + 0x100), [0.8, 0.6, 0.0, 0.0], 1e-3);
        // The record block starts with the word at +0x88.
        assert_eq!(s.e.mem.u32(s.this + (2 * 5 + 0xf) * 0x10), 0xcafe);
        // No bone is asked for from `calculateBoneModelSpace` with the flags clear.
        assert!(calls(&s.seen, CALCULATE_BONE_MODEL_SPACE).is_empty());
    }

    #[test]
    fn the_world_transform_is_multiplied_and_converted() {
        let mut s = scene(2, 1);
        s.e.call(0x00c7_9340, &args![s.this, s.pose, 0x1234u32, 0u32]);
        let multiplies = calls(&s.seen, MATRIX_MULTIPLY);
        assert_eq!(multiplies.len(), 1);
        assert_eq!(multiplies[0][0], s.this + 0x60);
        // The converter sees the 13 words copied from the world, the first nine replaced by
        // the product, and the last float set to one.
        let converted = calls(&s.seen, TO_QUATERNION);
        assert_eq!(converted.len(), 1);
        let mut expected: Vec<u32> = (0..9).map(|word| 200 + word).collect();
        expected.extend([109, 110, 111]);
        expected.push(1.0f32.to_bits());
        assert_eq!(converted[0], expected);
        assert_eq!(s.e.mem.u32(s.world + 0x68), 100);
    }

    #[test]
    fn a_flagged_parent_uses_the_calculated_transform() {
        let mut s = scene(2, 1);
        // The parent flags ask for `calculateBoneModelSpace`.
        s.e.mem.set_u32(s.flags + 4, 2);
        let calculated = s.e.mem.alloc(0x40);
        store4(&mut s.e, calculated + 0x10, [0.0, 0.0, 1.0, 0.0]);
        let log = Rc::clone(&s.seen);
        s.e.register_double(CALCULATE_BONE_MODEL_SPACE, move |_, a| {
            log.borrow_mut()
                .push((CALCULATE_BONE_MODEL_SPACE, a.to_vec()));
            calculated.into_ret()
        });
        s.e.call(0x00c7_9340, &args![s.this, s.pose, 0x1234u32, 0u32]);
        assert_eq!(
            calls(&s.seen, CALCULATE_BONE_MODEL_SPACE),
            vec![vec![s.pose, 1]]
        );
        // A half turn about z: w = 0, so (0, 1, 0) goes to 2 * (-0.5 * (0,1,0)) = (0, -1, 0).
        close(load4(&s.e, s.this + 0x110), [0.0, -1.0, 0.0, 0.0], 1e-6);
        // The bone itself (index 2, flags clear) is the identity: the axis stays.
        close(load4(&s.e, s.this + 0x100), [0.0, 1.0, 0.0, 0.0], 1e-3);
    }

    #[test]
    fn the_first_record_takes_the_vectors_of_the_extra_object() {
        let mut s = scene(2, 1);
        let extra = s.e.mem.alloc(0x40);
        store4(&mut s.e, extra + 0x10, [1.0, 2.0, 3.0, 4.0]);
        store4(&mut s.e, extra + 0x20, [5.0, 6.0, 7.0, 8.0]);
        let owner = s.e.mem.u32(s.this + 0x2a4);
        s.e.mem.set_u32(owner + 0x1c, extra);
        store4(&mut s.e, s.this + 0x160, [9.0; 4]);
        store4(&mut s.e, s.this + 0x150, [9.0; 4]);
        // Another record leaves them alone.
        s.e.call(0x00c7_9340, &args![s.this, s.pose, 0x1234u32, 0u32]);
        assert_eq!(load4(&s.e, s.this + 0x160), [9.0; 4]);
        assert_eq!(load4(&s.e, s.this + 0x150), [9.0; 4]);
        // Record 1 copies the two vectors; the second overwrites the record normalised
        // axis, which shares its place.
        s.e.call(0x00c7_9340, &args![s.this, s.pose, 0x1234u32, 1u32]);
        assert_eq!(load4(&s.e, s.this + 0x160), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load4(&s.e, s.this + 0x150), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(s.e.mem.u32(s.this + (5 + 0xf) * 0x10), 0xcafe);
    }
}

#[cfg(test)]
mod init_look_ik_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const TO_QUATERNION: u32 = 0x00a6_df40;
    const NORMALISE: u32 = 0x0056_11c0;
    const ALLOCATE_SLOT: u32 = 0x0900_0004;
    const INIT_BONE_PARAMS: u32 = 0x00c7_9340;

    struct Scene {
        e: Engine,
        seen: Seen,
        this: u32,
        mappers: u32,
        allocator: u32,
        answer: Rc<Cell<u32>>,
    }

    fn recorder(e: &mut Engine, seen: &Seen, address: u32) {
        let log = Rc::clone(seen);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    fn scene() -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(VECTOR_FILL, 0.0f32);
        e.set_global(VECTOR_THIRD, 1.0f32);
        e.set_global(SCALE_FACTOR, 1.0f32);
        e.set_global(LOOK_IK_POSE_X, 9.0f32);
        e.set_global(LOOK_IK_POSE_Y, 6.0f32);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        // The memory router, whose allocator hands out zeroed blocks through slot 4.
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        let log = Rc::clone(&seen);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            log.borrow_mut().push((ALLOCATE_SLOT, a.to_vec()));
            e.mem.alloc(a[1]).into_ret()
        });
        let log = Rc::clone(&seen);
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            log.borrow_mut().push((MEMORY_ALLOCATE, a.to_vec()));
            e.mem.alloc(a[1]).into_ret()
        });
        let log = Rc::clone(&seen);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        for address in [
            HK_STRING_CONSTRUCT,
            HK_STRING_DESTRUCT,
            SET_TO_REFERENCE_POSE,
            SYNC_MODEL_SPACE,
        ] {
            recorder(&mut e, &seen, address);
        }
        // `NiTransform::MakeIdentity`: the rotation, a zero translation and the scale 1.
        let log = Rc::clone(&seen);
        e.register_double(NI_TRANSFORM_MAKE_IDENTITY, move |e, a| {
            log.borrow_mut()
                .push((NI_TRANSFORM_MAKE_IDENTITY, a.to_vec()));
            for word in 0..13 {
                let value = match word {
                    0 | 4 | 8 | 12 => 1.0f32,
                    _ => 0.0,
                };
                e.mem.set_f32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        recorder(&mut e, &seen, TO_QUATERNION);
        recorder(&mut e, &seen, NORMALISE);
        let answer = Rc::new(Cell::new(1u32));
        let reply = Rc::clone(&answer);
        let log = Rc::clone(&seen);
        e.register_double(INIT_BONE_PARAMS, move |_, a| {
            log.borrow_mut().push((INIT_BONE_PARAMS, a.to_vec()));
            reply.get().into_ret()
        });
        let this = e.mem.alloc(0x300);
        let mappers = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2a4, mappers);
        e.mem.set_u8(this + 0x42, 1);
        e.mem.set_u8(this + 0x190, 1);
        e.mem.set_u32(this + 0x88, 0xbeef);
        for lane in 0..4 {
            e.mem.set_f32(this + 0x160 + 4 * lane, 1.0 + lane as f32);
            e.mem.set_f32(this + 0x150 + 4 * lane, 5.0 + lane as f32);
        }
        Scene {
            e,
            seen,
            this,
            mappers,
            allocator,
            answer,
        }
    }

    fn calls(seen: &Seen, address: u32) -> Vec<Vec<u32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn lanes(s: &Scene, address: u32) -> [f32; 4] {
        std::array::from_fn(|lane| s.e.mem.f32(address + 4 * lane as u32))
    }

    #[test]
    fn nothing_happens_without_the_enable_byte() {
        let mut s = scene();
        s.e.mem.set_u8(s.this + 0x42, 0);
        s.e.mem.set_u8(s.this + 0xb0, 0x55);
        s.e.call(0x00c7_de60, &args![s.this, 0x1234u32]);
        assert!(s.seen.borrow().is_empty());
        assert_eq!(s.e.mem.u8(s.this + 0xb0), 0x55);
    }

    #[test]
    fn a_failed_bone_lookup_only_stores_the_answer() {
        let mut s = scene();
        s.answer.set(0xaabb_cc00);
        s.e.mem.set_u8(s.this + 0xb0, 0x55);
        s.e.call(0x00c7_de60, &args![s.this, 0x1234u32]);
        assert_eq!(
            calls(&s.seen, INIT_BONE_PARAMS),
            vec![vec![s.this, 0xbeef, 0x1234, 1]]
        );
        assert_eq!(s.seen.borrow().len(), 1);
        assert_eq!(s.e.mem.u8(s.this + 0xb0), 0);
        assert_eq!(s.e.mem.u32(s.this + 0x194), 0);
    }

    #[test]
    fn without_the_look_byte_only_the_flag_and_the_answer_are_stored() {
        let mut s = scene();
        s.e.mem.set_u8(s.this + 0x190, 0);
        s.e.call(0x00c7_de60, &args![s.this, 0x1234u32]);
        assert_eq!(s.seen.borrow().len(), 1);
        assert_eq!(s.e.mem.u32(s.this + 0x194), 1);
        assert_eq!(s.e.mem.u8(s.this + 0xb0), 1);
        assert_eq!(s.e.mem.u32(s.mappers + 0x1c), 0);
    }

    #[test]
    fn the_two_bone_skeleton_and_the_pose_are_built() {
        let mut s = scene();
        s.e.call(0x00c7_de60, &args![s.this, 0x1234u32]);
        let cell = s.e.mem.u32(s.mappers + 0x1c);
        assert_ne!(cell, 0);
        // The 0x30-byte share object comes from the aligned allocator (0x10 more bytes).
        assert_eq!(
            calls(&s.seen, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x40]]
        );
        let skeleton = s.e.mem.u32(cell);
        assert_eq!(s.e.mem.u32(skeleton), HKA_SKELETON_VTABLE);
        assert_eq!(
            (s.e.mem.u16(skeleton + 4), s.e.mem.u16(skeleton + 6)),
            (0x48, 1)
        );
        let pose = s.e.mem.u32(s.this + 0x54);
        assert_eq!(
            calls(&s.seen, ALLOCATE_SLOT),
            vec![vec![s.allocator, 0x48], vec![s.allocator, 0x38]]
        );
        let bones = s.e.mem.u32(skeleton + 0x18);
        assert_eq!(s.e.mem.u32(skeleton + 0x1c), 2);
        assert_eq!((s.e.mem.u8(bones + 4), s.e.mem.u8(bones + 0xc)), (0, 0));
        assert_eq!(
            calls(&s.seen, HK_STRING_CONSTRUCT),
            vec![vec![skeleton + 8], vec![bones], vec![bones + 8]]
        );
        assert!(calls(&s.seen, HK_STRING_DESTRUCT).is_empty());
        let parents = s.e.mem.u32(skeleton + 0xc);
        assert_eq!(
            (s.e.mem.u16(parents), s.e.mem.u16(parents + 2)),
            (0xffff, 0)
        );
        assert_eq!(
            (s.e.mem.u32(skeleton + 0x10), s.e.mem.u32(skeleton + 0x28)),
            (2, 2)
        );
        // The identity transform makes the first reference pose; the second one has the
        // translation (9, 6, 0) from the two floats.
        let poses = s.e.mem.u32(skeleton + 0x24);
        assert_eq!(lanes(&s, poses), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(lanes(&s, poses + 0x30), [9.0, 6.0, 0.0, 0.0]);
        assert_eq!(lanes(&s, poses + 0x20), [1.0; 4]);
        assert_eq!(lanes(&s, poses + 0x50), [1.0; 4]);
        let reserved = |array: u32, count: u32, size: u32| vec![s.allocator, array, count, size];
        assert_eq!(
            calls(&s.seen, ARRAY_RESERVE),
            vec![
                reserved(skeleton + 0x18, 2, 8),
                reserved(skeleton + 0xc, 2, 2),
                reserved(skeleton + 0x24, 2, 0x30),
                reserved(pose + 0x10, 2, 0x30),
                reserved(pose + 4, 2, 0x30),
                reserved(pose + 0x1c, 2, 4),
            ]
        );
        // The vectors go to the share object.
        assert_eq!(lanes(&s, cell + 0x10), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(lanes(&s, cell + 0x20), [5.0, 6.0, 7.0, 8.0]);
        // The pose is reset and synced; the controller keeps it and the look-at record.
        assert_eq!(s.e.mem.u32(pose), skeleton);
        assert_eq!((s.e.mem.u32(pose + 8), s.e.mem.u32(pose + 0x14)), (2, 2));
        assert_eq!(calls(&s.seen, SET_TO_REFERENCE_POSE), vec![vec![pose]]);
        assert_eq!(calls(&s.seen, SYNC_MODEL_SPACE), vec![vec![pose]]);
        assert_eq!(s.e.mem.u32(s.this + 0xf0), pose);
        assert_eq!(
            (s.e.mem.u16(s.this + 0xf4), s.e.mem.u16(s.this + 0xf6)),
            (1, 0)
        );
        assert_eq!(lanes(&s, s.this + 0x100), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(lanes(&s, s.this + 0x110), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(s.e.mem.u32(s.this + 0x194), 0);
        assert_eq!(s.e.mem.u8(s.this + 0xb0), 1);
    }

    #[test]
    fn an_existing_share_object_keeps_its_skeleton() {
        let mut s = scene();
        let cell = s.e.mem.alloc(0x30);
        let skeleton = s.e.mem.alloc(0x80);
        s.e.mem.set_u32(cell, skeleton);
        s.e.mem.set_u32(skeleton + 0x1c, 3);
        s.e.mem.set_u32(skeleton + 0x34, 5);
        s.e.mem.set_u32(s.mappers + 0x1c, cell);
        s.e.call(0x00c7_de60, &args![s.this, 0x1234u32]);
        assert_eq!(s.e.mem.u32(s.mappers + 0x1c), cell);
        assert!(calls(&s.seen, MEMORY_ALLOCATE).is_empty());
        assert!(calls(&s.seen, HK_STRING_CONSTRUCT).is_empty());
        assert_eq!(calls(&s.seen, ALLOCATE_SLOT), vec![vec![s.allocator, 0x38]]);
        let pose = s.e.mem.u32(s.this + 0x54);
        let reserved = |array: u32, count: u32, size: u32| vec![s.allocator, array, count, size];
        assert_eq!(
            calls(&s.seen, ARRAY_RESERVE),
            vec![
                reserved(pose + 0x10, 3, 0x30),
                reserved(pose + 4, 3, 0x30),
                reserved(pose + 0x1c, 3, 4),
                reserved(pose + 0x2c, 8, 4),
            ]
        );
        assert_eq!((s.e.mem.u32(pose), s.e.mem.u32(pose + 0x30)), (skeleton, 5));
        assert_eq!(lanes(&s, cell + 0x10), [0.0; 4]);
        assert_eq!(s.e.mem.u32(s.this + 0xf0), pose);
    }
}

#[cfg(test)]
mod look_at_update_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const BONE_INFO: u32 = 0x5555;

    struct Scene {
        e: Engine,
        seen: Seen,
        this: u32,
        pose: u32,
        outputs: u32,
        locals: u32,
        flags: u32,
        solved: Rc<Cell<u8>>,
        scribble: Rc<Cell<bool>>,
        check: Rc<Cell<u8>>,
        angle: Rc<Cell<f64>>,
        vector: u32,
    }

    fn fill(e: &mut Engine, address: u32, words: [u32; 4]) {
        for (lane, word) in words.into_iter().enumerate() {
            e.mem.set_u32(address + 4 * lane as u32, word);
        }
    }

    /// A controller whose record `index` has the bone 1 with the parent bone 0, in a pose of
    /// three bones (flags all clear, identity quaternions in the output transforms).
    fn scene(index: u32, inverse_point: [f32; 4]) -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(VECTOR_FILL, 0.0f32);
        e.set_global(ROOT_THREE, 3.0f32);
        e.set_global(ROOT_HALF, 0.5f32);
        e.set_global(QUATERNION_TOLERANCE, 0.001f32);
        e.set_global(LOOK_AT_ANGLE_LIMIT, 10.0f32);
        e.set_global(DEGREES_TO_RADIANS, 0.017453292519943295f64);
        for lane in 0..4 {
            e.mem.set_f32(NEG_HALF_LANES + 4 * lane, -0.5);
            e.mem.set_f32(ONE_LANES + 4 * lane, 1.0);
            e.mem.set_u32(QUATERNION_ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        fill(&mut e, SIGN_LANES, [0x8000_0000; 4]);
        fill(
            &mut e,
            SIGN_XYZ_LANES,
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0],
        );
        fill(&mut e, MASK_XYZ_LANES, [u32::MAX, u32::MAX, u32::MAX, 0]);
        store4(&mut e, LOOK_AT_REFERENCE, [0.0, 0.0, 0.0, 1.0]);
        store4(&mut e, UNIT_QUATERNION, [0.0, 0.0, 0.0, 1.0]);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let this = e.mem.alloc(0x400);
        let entry = this + index * 0x50;
        let block = this + (index * 5 + 0xf) * 0x10;
        let pose = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        let parents = e.mem.alloc(0x10);
        let outputs = e.mem.alloc(0x30 * 4);
        let locals = e.mem.alloc(0x30 * 4);
        let flags = e.mem.alloc(0x20);
        e.mem.set_u32(pose, skeleton);
        e.mem.set_u32(pose + 4, outputs);
        e.mem.set_u32(pose + 0x10, locals);
        e.mem.set_u32(pose + 0x1c, flags);
        e.mem.set_u32(skeleton + 0xc, parents);
        e.mem.set_u32(skeleton + 0x1c, 3);
        e.mem.set_u16(parents, 0xffff);
        e.mem.set_u16(parents + 2, 0);
        e.mem.set_u16(parents + 4, 1);
        for bone in 0..3 {
            store4(&mut e, outputs + 0x30 * bone + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store4(&mut e, outputs + 0x30 * bone + 0x20, [1.0; 4]);
        }
        e.mem.set_u32(block, pose);
        e.mem.set_u16(entry + 0xf4, 1);
        e.mem.set_u16(entry + 0xf6, 0);
        store4(&mut e, entry + 0x100, [0.0, 1.0, 0.0, 0.0]);
        store4(&mut e, entry + 0x120, [0.0, 0.0, 0.0, 1.0]);
        let vector = e.mem.alloc(0x10);
        let solved = Rc::new(Cell::new(1u8));
        let scribble = Rc::new(Cell::new(false));
        let check = Rc::new(Cell::new(1u8));
        let angle = Rc::new(Cell::new(0.0f64));
        let log = Rc::clone(&seen);
        e.register_double(TRANSFORM_POINT_INVERSE, move |e, a| {
            log.borrow_mut().push((TRANSFORM_POINT_INVERSE, a.to_vec()));
            store4(e, a[0], inverse_point);
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(POSE_BONE_TRANSFORM, move |_, a| {
            log.borrow_mut().push((POSE_BONE_TRANSFORM, a.to_vec()));
            vector.into_ret()
        });
        let log = Rc::clone(&seen);
        e.register_double(LOOK_AT_ACCESS_MODEL_SPACE, move |_, a| {
            log.borrow_mut()
                .push((LOOK_AT_ACCESS_MODEL_SPACE, a.to_vec()));
            BONE_INFO.into_ret()
        });
        let log = Rc::clone(&seen);
        let (answer, damage) = (Rc::clone(&solved), Rc::clone(&scribble));
        e.register_double(LOOK_AT_SOLVE, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..13).map(|word| e.mem.u32(a[1] + 4 * word)));
            words.extend((0..4).map(|word| e.mem.u32(a[2] + 4 * word)));
            log.borrow_mut().push((LOOK_AT_SOLVE, words));
            e.mem.set_u8(a[0], answer.get());
            if damage.get() {
                e.mem.set_u32(outputs + 0x30, 0xdead);
            }
            a[0].into_ret()
        });
        let log = Rc::clone(&seen);
        let verdict = Rc::clone(&check);
        e.register_double(QUATERNION_CHECK, move |e, a| {
            log.borrow_mut().push((QUATERNION_CHECK, a.to_vec()));
            e.mem.set_u8(a[1], verdict.get());
            a[1].into_ret()
        });
        let log = Rc::clone(&seen);
        let turned = Rc::clone(&angle);
        e.register_double(0x00c7_55e0, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|word| e.mem.u32(a[2] + 4 * word)));
            log.borrow_mut().push((0x00c7_55e0, words));
            Ret {
                st0: turned.get(),
                ..Ret::default()
            }
        });
        Scene {
            e,
            seen,
            this,
            pose,
            outputs,
            locals,
            flags,
            solved,
            scribble,
            check,
            angle,
            vector,
        }
    }

    fn calls(seen: &Seen, address: u32) -> Vec<Vec<u32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn bits(lanes: [f32; 4]) -> Vec<u32> {
        lanes.iter().map(|lane| lane.to_bits()).collect()
    }

    fn close(actual: [f32; 4], expected: [f32; 4]) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-3,
                "{actual:?} against {expected:?}"
            );
        }
    }

    #[test]
    fn the_solver_gets_the_axis_the_target_and_the_bone_info() {
        let mut s = scene(0, [1.0, 2.0, 3.0, 0.0]);
        let transform = s.e.mem.alloc(0x30);
        let answer = s.e.call(0x00c7_8610, &args![s.this, 0u32, transform]).u32();
        // The solver changed something (answers 1): the update answers false.
        assert_eq!(answer & 0xff, 0);
        assert_eq!(
            calls(&s.seen, TRANSFORM_POINT_INVERSE)[0][1..],
            [transform, s.this + 0xd0]
        );
        assert_eq!(
            calls(&s.seen, LOOK_AT_ACCESS_MODEL_SPACE),
            vec![vec![s.pose, 1, 1]]
        );
        let solve = &calls(&s.seen, LOOK_AT_SOLVE)[0];
        assert_eq!(solve[1..].len(), 5 + 13 + 4);
        assert_eq!(solve[3..6], [1.0f32.to_bits(), BONE_INFO, 0]);
        // The input: the axis from the record, and the zero vector.
        assert_eq!(solve[6..10], bits([0.0, 1.0, 0.0, 0.0])[..]);
        assert_eq!(solve[10..14], [0; 4]);
        assert_eq!(solve[19..23], bits([1.0, 2.0, 3.0, 0.0])[..]);
    }

    /// The setup's third vector is the record's `+0x110` turned by the model rotation of the bone
    /// `+0xf6` and normalised, its fourth the record's float `+0xf8` (00c786a3..00c787bd,
    /// 00c7863b): the real game's first look of Doc's opening has (0.047, 0.951, -0.306) and
    /// 1.047 there (the test uses 1.25); the port had left both to the kept bone transform and Doc looked at the floor.
    #[test]
    fn the_solver_gets_the_reference_direction_and_the_largest_angle() {
        let mut s = scene(0, [1.0, 2.0, 3.0, 0.0]);
        let entry = s.this;
        s.e.mem.set_f32(entry + 0xf8, 1.25);
        store4(&mut s.e, entry + 0x110, [2.0, 0.0, 0.0, 0.0]);
        // Bone 0 (the record's `+0xf6`) turned a quarter about z: x goes to y.
        let half_root = std::f32::consts::FRAC_1_SQRT_2;
        store4(&mut s.e, s.locals + 0x10, [0.0, 0.0, half_root, half_root]);
        let transform = s.e.mem.alloc(0x30);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, transform]);
        let solve = &calls(&s.seen, LOOK_AT_SOLVE)[0];
        let direction = [solve[14], solve[15], solve[16], solve[17]].map(f32::from_bits);
        close(direction, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(solve[18], 1.25f32.to_bits());
    }

    #[test]
    fn the_solver_answer_is_kept() {
        let mut s = scene(0, [1.0, 2.0, 3.0, 0.0]);
        let transform = s.e.mem.alloc(0x30);
        s.solved.set(0);
        let answer = s.e.call(0x00c7_8610, &args![s.this, 0u32, transform]).u32();
        assert_eq!(answer & 0xff, 1);
        assert_eq!(s.e.mem.u8(s.this + 0x130), 1);
    }

    #[test]
    fn the_quaternion_is_replaced_only_when_it_is_close_to_the_reference() {
        let mut s = scene(0, [0.0; 4]);
        store4(&mut s.e, s.outputs + 0x30 + 0x10, [0.1, 0.2, 0.3, 0.9]);
        store4(&mut s.e, s.this + 0x120, [0.0, 0.0, 0.0005, 1.0]);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert_eq!(load4(&s.e, s.this + 0x120), [0.1, 0.2, 0.3, 0.9]);
        store4(&mut s.e, s.this + 0x120, [0.0, 0.0, 0.5, 1.0]);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert_eq!(load4(&s.e, s.this + 0x120), [0.0, 0.0, 0.5, 1.0]);
    }

    #[test]
    fn the_bone_angle_is_asked_for_once_the_record_has_been_used() {
        let mut s = scene(0, [0.0; 4]);
        store4(&mut s.e, s.outputs + 0x30 + 0x10, [0.1, 0.2, 0.3, 0.9]);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert!(calls(&s.seen, 0x00c7_55e0).is_empty());
        assert_eq!(s.e.mem.u8(s.this + 0x130), 1);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        let asked = &calls(&s.seen, 0x00c7_55e0);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][..2], [s.this, 0]);
        assert_eq!(asked[0][3..], bits([0.1, 0.2, 0.3, 0.9])[..]);
    }

    #[test]
    fn a_small_angle_resets_the_quaternion_and_clears_the_bytes() {
        let mut s = scene(0, [0.0; 4]);
        s.e.mem.set_u8(s.this + 0x130, 1);
        s.e.mem.set_u8(s.this + 0xb2, 1);
        s.e.mem.set_u8(s.this + 0xb3, 1);
        s.angle.set(0.1);
        store4(&mut s.e, s.this + 0x120, [0.0, 0.0, 0.5, 0.5]);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert_eq!(load4(&s.e, s.this + 0x120), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(
            (s.e.mem.u8(s.this + 0xb2), s.e.mem.u8(s.this + 0xb3)),
            (0, 0)
        );
        // A larger one (above 10 degrees) leaves them.
        s.e.mem.set_u8(s.this + 0xb2, 1);
        s.e.mem.set_u8(s.this + 0xb3, 1);
        s.angle.set(0.2);
        store4(&mut s.e, s.this + 0x120, [0.0, 0.0, 0.5, 0.5]);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert_eq!(load4(&s.e, s.this + 0x120), [0.0, 0.0, 0.5, 0.5]);
        assert_eq!(
            (s.e.mem.u8(s.this + 0xb2), s.e.mem.u8(s.this + 0xb3)),
            (1, 1)
        );
        // Without the byte at +0xb2 nothing is reset.
        s.e.mem.set_u8(s.this + 0xb2, 0);
        s.angle.set(0.0);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert_eq!(load4(&s.e, s.this + 0x120), [0.0, 0.0, 0.5, 0.5]);
    }

    #[test]
    fn the_second_record_takes_back_one_of_the_counted_records() {
        let mut s = scene(1, [0.0; 4]);
        let entry = s.this + 0x50;
        s.e.mem.set_u8(entry + 0x130, 1);
        s.e.mem.set_u8(s.this + 0xb2, 1);
        s.e.mem.set_u8(s.this + 0x190, 1);
        s.e.mem.set_u32(s.this + 0x194, 2);
        s.e.mem.set_u8(s.this + 2 * 0x50 + 0x131, 1);
        s.e.mem.set_u32(s.this + 0x150, 0);
        s.angle.set(0.0);
        store4(&mut s.e, entry + 0x120, [0.0, 0.0, 0.5, 0.5]);
        s.e.call(0x00c7_8610, &args![s.this, 1u32, 0u32]);
        assert_eq!(s.e.mem.u8(s.this + 2 * 0x50 + 0x131), 0);
        assert_eq!(s.e.mem.u32(s.this + 0x194), 1);
        assert_eq!(s.e.mem.u8(s.this + 0xb2), 1);
        assert_eq!(load4(&s.e, entry + 0x120), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn a_bad_quaternion_puts_the_transform_back_and_flags_the_descendants() {
        let mut s = scene(0, [0.0; 4]);
        s.check.set(0);
        s.scribble.set(true);
        store4(&mut s.e, s.outputs + 0x30, [7.0, 8.0, 9.0, 0.0]);
        store4(&mut s.e, s.outputs + 0x30 + 0x10, [0.1, 0.2, 0.3, 0.9]);
        s.e.mem.set_u8(s.pose + 0x28, 1);
        s.e.mem.set_u32(s.flags + 4, 0);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        // The solver wrote 0xdead into the first word; the kept transform is back.
        assert_eq!(load4(&s.e, s.outputs + 0x30), [7.0, 8.0, 9.0, 0.0]);
        assert_eq!(load4(&s.e, s.outputs + 0x40), [0.1, 0.2, 0.3, 0.9]);
        assert_eq!(s.e.mem.u32(s.flags + 4), 2);
        // Bone 2 (a child of bone 1) is flagged 4 and then 2, and the byte is cleared.
        assert_eq!(s.e.mem.u32(s.flags + 8), 2);
        assert_eq!(s.e.mem.u8(s.pose + 0x28), 0);
        // The check looked at the quaternion of the bone's transform.
        let checks = calls(&s.seen, QUATERNION_CHECK);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0][0], s.locals + 0x30 + 0x10);
        s.check.set(1);
        s.e.mem.set_u32(s.flags + 4, 0);
        s.e.mem.set_u32(s.outputs + 0x30, 0x1234);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert_eq!(s.e.mem.u32(s.flags + 4), 0);
    }

    /// A scene for the look-at pull: bone vector p = (0, 0, 1, 0), d = (1, 0, 0, 0); the
    /// other transform (bone 1 of the pose) has its translation at the origin.
    fn pulled(inverse_point: [f32; 4]) -> Scene {
        let mut s = scene(1, inverse_point);
        s.e.mem.set_u8(s.this + 0x190, 1);
        s.e.mem.set_u32(s.this + 0xf0, 0x7777);
        store4(&mut s.e, s.this + 0x150, [1.0, 0.0, 0.0, 0.0]);
        store4(&mut s.e, s.vector, [0.0, 0.0, 1.0, 0.0]);
        store4(&mut s.e, s.locals + 0x30, [0.0; 4]);
        s
    }

    #[test]
    fn a_target_nearer_than_the_reach_is_pushed_out_along_its_direction() {
        let mut s = pulled([0.0, 0.0, 0.5, 0.0]);
        s.e.call(0x00c7_8610, &args![s.this, 1u32, 0u32]);
        // u = (0, 0, 1), reach 1: the unit direction times 1 is added to the target.
        let solve = &calls(&s.seen, LOOK_AT_SOLVE)[0];
        let target: Vec<f32> = solve[19..23].iter().map(|w| f32::from_bits(*w)).collect();
        close(
            [target[0], target[1], target[2], target[3]],
            [0.0, 0.0, 1.5, 0.0],
        );
        // The vector of the bone is the second vector of the solver input.
        assert_eq!(solve[10..14], bits([0.0, 0.0, 1.0, 0.0])[..]);
        assert_eq!(
            calls(&s.seen, POSE_BONE_TRANSFORM),
            vec![vec![0x7777, 1]; 3]
        );
    }

    #[test]
    fn a_target_farther_than_the_reach_stays() {
        let mut s = pulled([0.0, 0.0, 5.0, 0.0]);
        s.e.call(0x00c7_8610, &args![s.this, 1u32, 0u32]);
        let solve = &calls(&s.seen, LOOK_AT_SOLVE)[0];
        assert_eq!(solve[19..23], bits([0.0, 0.0, 5.0, 0.0])[..]);
        assert_eq!(solve[10..14], bits([0.0, 0.0, 1.0, 0.0])[..]);
    }

    #[test]
    fn the_look_at_pull_needs_the_byte_and_the_second_record() {
        let mut s = pulled([0.0, 0.0, 0.5, 0.0]);
        s.e.mem.set_u8(s.this + 0x190, 0);
        s.e.call(0x00c7_8610, &args![s.this, 1u32, 0u32]);
        assert!(calls(&s.seen, POSE_BONE_TRANSFORM).is_empty());
        // The first record has its pose at the word the controller reads the object from.
        let mut s = scene(0, [0.0, 0.0, 0.5, 0.0]);
        s.e.mem.set_u8(s.this + 0x190, 1);
        s.e.call(0x00c7_8610, &args![s.this, 0u32, 0u32]);
        assert!(calls(&s.seen, POSE_BONE_TRANSFORM).is_empty());
    }
}

#[cfg(test)]
mod do_look_at_ik_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    struct Rig {
        e: Engine,
        log: CallLog,
        this: u32,
        pose: u32,
        kept: u32,
        transform: u32,
        other_pose: u32,
        moved: Rc<Cell<u32>>,
    }

    fn set_lanes(e: &mut Engine, address: u32, lanes: [f32; 4]) {
        store4(e, address, lanes);
    }

    /// A controller (bone 1 of a pose of four bones: parents -1, 0, 1, 0; bone 2 flagged 1,
    /// bone 3 flagged 4) with doubles for every callee: the bone record update answers what
    /// `moved` holds, `acos` answers its argument plus 0.5 and the quarter turn is 2.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        set_lanes(&mut e, NEG_HALF_LANES, [-0.5; 4]);
        set_lanes(&mut e, ONE_LANES, [1.0; 4]);
        for lane in 0..4u32 {
            e.mem.set_u32(SIGN_LANES + 4 * lane, 0x8000_0000);
            let xyz = lane < 3;
            e.mem
                .set_u32(SIGN_XYZ_LANES + 4 * lane, if xyz { 0x8000_0000 } else { 0 });
            e.mem
                .set_u32(MASK_XYZ_LANES + 4 * lane, if xyz { 0xffff_ffff } else { 0 });
        }
        e.mem.set_f32(AXIS_ZERO, 0.0);
        e.mem.set_f32(AXIS_ONE, 1.0);
        e.mem.set_f32(AXIS_MINUS_ONE, -1.0);
        e.mem.set_f64(QUARTER_TURN, 2.0);
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let moved = Rc::new(Cell::new(0u32));
        for address in [
            ADJUST_CURRENT_TARGET,
            QUATERNION_SET_MUL,
            SET_TO_REFERENCE_POSE,
        ] {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                let mut entry = vec![address];
                entry.extend_from_slice(a);
                log.borrow_mut().push(entry);
                Ret::default()
            });
        }
        e.register_double(LOOK_AT_UPDATE, {
            let log = Rc::clone(&log);
            let moved = Rc::clone(&moved);
            move |e, a| {
                let mut entry = vec![LOOK_AT_UPDATE, a[0], a[1], a[2]];
                if a[1] == 0 {
                    entry.extend((0..12).map(|word| e.mem.u32(a[2] + 4 * word)));
                }
                log.borrow_mut().push(entry);
                moved.get().into_ret()
            }
        });
        e.register_double(CRT_ARC_COSINE, {
            let log = Rc::clone(&log);
            move |_, a| {
                let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
                log.borrow_mut().push(vec![CRT_ARC_COSINE]);
                Ret {
                    st0: value + 0.5,
                    ..Ret::default()
                }
            }
        });
        let skeleton = e.mem.alloc(0x20);
        let parents = e.mem.alloc(8);
        for (bone, parent) in [0xffffu16, 0, 1, 0].into_iter().enumerate() {
            e.mem.set_u16(parents + 2 * bone as u32, parent);
        }
        e.mem.set_u32(skeleton + 0xc, parents);
        e.mem.set_u32(skeleton + 0x1c, 4);
        let flags = e.mem.alloc(0x10);
        e.mem.set_u32(flags + 8, 1);
        e.mem.set_u32(flags + 0xc, 4);
        let local = e.mem.alloc(4 * 0x30);
        let output = e.mem.alloc(4 * 0x30);
        // The parent of bone 2 (bone 1) is the identity; bone 2's local translation is (1, 2, 3).
        set_lanes(&mut e, local + 0x40, [0.0, 0.0, 0.0, 1.0]);
        set_lanes(&mut e, local + 0x50, [1.0; 4]);
        set_lanes(&mut e, local + 0x60, [1.0, 2.0, 3.0, 0.0]);
        let pose = e.mem.alloc(0x30);
        e.mem.set_u32(pose, skeleton);
        e.mem.set_u32(pose + 4, output);
        e.mem.set_u32(pose + 0x10, local);
        e.mem.set_u32(pose + 0x1c, flags);
        e.mem.set_u8(pose + 0x28, 1);
        e.mem.set_u8(pose + 0x29, 1);
        // The kept bone transform: translation (1, 0, 0), the identity, scale 2.
        let kept = e.mem.alloc(0x30);
        set_lanes(&mut e, kept, [1.0, 0.0, 0.0, 0.0]);
        set_lanes(&mut e, kept + 0x10, [0.0, 0.0, 0.0, 1.0]);
        set_lanes(&mut e, kept + 0x20, [2.0; 4]);
        e.register_double(LOOK_AT_ACCESS_MODEL_SPACE, {
            let log = Rc::clone(&log);
            move |_, a| {
                log.borrow_mut()
                    .push(vec![LOOK_AT_ACCESS_MODEL_SPACE, a[0], a[1], a[2]]);
                kept.into_ret()
            }
        });
        // The second pose: bone 1 turned by the (unnormalised) quaternion (1, 0, 0, 1).
        let other_flags = e.mem.alloc(8);
        let other_local = e.mem.alloc(0x60);
        set_lanes(&mut e, other_local + 0x40, [1.0, 0.0, 0.0, 1.0]);
        let other_pose = e.mem.alloc(0x30);
        e.mem.set_u32(other_pose + 0x10, other_local);
        e.mem.set_u32(other_pose + 0x1c, other_flags);
        let transform = e.mem.alloc(0x30);
        set_lanes(&mut e, transform, [10.0, 20.0, 30.0, 0.0]);
        set_lanes(&mut e, transform + 0x10, [0.0, 0.0, 0.0, 1.0]);
        set_lanes(&mut e, transform + 0x20, [3.0; 4]);
        let this = e.mem.alloc(0x2b0);
        e.mem.set_u32(this + 0x88, pose);
        e.mem.set_u32(this + 0x54, other_pose);
        e.mem.set_u16(this + 0x144, 1);
        e.mem.set_f32(this + 0x134, 10.0);
        e.mem.set_f32(this + 0x138, 10.0);
        Rig {
            e,
            log,
            this,
            pose,
            kept,
            transform,
            other_pose,
            moved,
        }
    }

    fn run(r: &mut Rig) {
        let (this, transform) = (r.this, r.transform);
        r.e.call(0x00c7_aa60, &args![this, transform]);
    }

    fn logged(r: &Rig, address: u32) -> Vec<Vec<u32>> {
        r.log
            .borrow()
            .iter()
            .filter(|entry| entry[0] == address)
            .map(|entry| entry[1..].to_vec())
            .collect()
    }

    fn flags(r: &Rig) -> Vec<u32> {
        let flags = r.e.mem.u32(r.pose + 0x1c);
        (0..4).map(|bone| r.e.mem.u32(flags + 4 * bone)).collect()
    }

    #[test]
    fn bhk_ragdoll_controller_do_look_at_ik_propagates_the_kept_bone() {
        let mut r = rig();
        run(&mut r);
        let (this, pose, kept, transform) = (r.this, r.pose, r.kept, r.transform);
        // Without `+0x190` the record 1 update runs first and the word `+0x194` is 1.
        assert_eq!(logged(&r, ADJUST_CURRENT_TARGET), vec![vec![this]]);
        assert_eq!(logged(&r, LOOK_AT_UPDATE), vec![vec![this, 1, transform]]);
        assert_eq!(r.e.mem.u32(this + 0x194), 1);
        assert_eq!(
            logged(&r, LOOK_AT_ACCESS_MODEL_SPACE),
            vec![vec![pose, 1, 1]]
        );
        let multiplications = logged(&r, QUATERNION_SET_MUL);
        assert_eq!(
            multiplications[0],
            vec![kept + 0x10, kept + 0x10, this + 0xc0]
        );
        // Bone 2 is composed with bone 1 (one more multiplication), bone 3 (parent 0) is not.
        assert_eq!(multiplications.len(), 2);
        let output = r.e.mem.u32(pose + 4);
        assert_eq!(load4(&r.e, output + 0x60), [1.0, 2.0, 3.0, 0.0]);
        // Root 1, the composed bone and the later flagged bone 2, bone 0 untouched.
        assert_eq!(flags(&r), vec![0, 1, 2, 2]);
        assert_eq!(r.e.mem.u8(pose + 0x28), 0);
        assert_eq!(r.e.mem.u8(pose + 0x29), 0);
        let local = r.e.mem.u32(pose + 0x10);
        for word in 0..12 {
            assert_eq!(
                r.e.mem.u32(local + 0x30 + 4 * word),
                r.e.mem.u32(kept + 4 * word)
            );
        }
        // Nothing of the second part ran.
        assert!(logged(&r, CRT_ARC_COSINE).is_empty());
        assert!(logged(&r, SET_TO_REFERENCE_POSE).is_empty());
    }

    #[test]
    fn bhk_ragdoll_controller_do_look_at_ik_measures_the_head_angles() {
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0x190, 1);
        run(&mut r);
        let (this, transform) = (r.this, r.transform);
        // Only the record 0 update runs, with the bone composed with `transform`.
        let updates = logged(&r, LOOK_AT_UPDATE);
        assert_eq!(updates.len(), 1);
        assert_eq!(&updates[0][..2], &[this, 0]);
        let composed: Vec<f32> = updates[0][3..].iter().map(|&w| f32::from_bits(w)).collect();
        assert_eq!(&composed[0..4], &[11.0, 20.0, 30.0, 0.0]);
        assert_eq!(&composed[8..12], &[6.0; 4]);
        let model = r.e.mem.u32(r.pose + 0x10) + 0x30;
        assert_eq!(
            *logged(&r, QUATERNION_SET_MUL).last().unwrap(),
            vec![updates[0][2] + 0x10, transform + 0x10, model + 0x10]
        );
        // The up axis turned by (1, 0, 0, 1) is (0, 1, 2): 2 - (-2 + 0.5) and 2 - (0 + 0.5).
        assert_eq!(r.e.mem.f32(this + 0x19c), 3.5);
        assert_eq!(r.e.mem.f32(this + 0x1a0), 1.5);
        assert_eq!(r.e.mem.u8(this + 0x181), 0);
        assert_eq!(logged(&r, SET_TO_REFERENCE_POSE), vec![vec![r.other_pose]]);
    }

    #[test]
    fn bhk_ragdoll_controller_do_look_at_ik_sets_the_looking_byte() {
        // The update answering true.
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0x190, 1);
        r.moved.set(1);
        run(&mut r);
        assert_eq!(r.e.mem.u8(r.this + 0x181), 1);
        // The word `+0x194` set (and `+0x181` set before: the record 1 update runs too).
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0x190, 1);
        r.e.mem.set_u8(r.this + 0x181, 1);
        run(&mut r);
        assert_eq!(logged(&r, LOOK_AT_UPDATE).len(), 2);
        assert_eq!(r.e.mem.u8(r.this + 0x181), 1);
        // The second angle above its limit, then the first.
        for (offset, limit) in [(0x138, 1.0f32), (0x134, 3.0)] {
            let mut r = rig();
            r.e.mem.set_u8(r.this + 0x190, 1);
            r.e.mem.set_f32(r.this + offset, limit);
            run(&mut r);
            assert_eq!(r.e.mem.u8(r.this + 0x181), 1);
        }
        // A NaN limit never sets it.
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0x190, 1);
        r.e.mem.set_u32(r.this + 0x138, 0x7fc0_0000);
        run(&mut r);
        assert_eq!(r.e.mem.u8(r.this + 0x181), 0);
        // Bone 1 of the second pose flagged 2: its transform comes from
        // `calculateBoneModelSpace`.
        let mut r = rig();
        r.e.mem.set_u8(r.this + 0x190, 1);
        let other_flags = r.e.mem.u32(r.other_pose + 0x1c);
        r.e.mem.set_u32(other_flags + 4, 2);
        let identity = r.e.mem.alloc(0x30);
        set_lanes(&mut r.e, identity + 0x10, [0.0, 0.0, 0.0, 1.0]);
        r.e.register_double(CALCULATE_BONE_MODEL_SPACE, move |_, _| identity.into_ret());
        run(&mut r);
        // The up axis unturned: both dot products 0.
        assert_eq!(r.e.mem.f32(r.this + 0x19c), 1.5);
        assert_eq!(r.e.mem.f32(r.this + 0x1a0), 1.5);
    }
}
