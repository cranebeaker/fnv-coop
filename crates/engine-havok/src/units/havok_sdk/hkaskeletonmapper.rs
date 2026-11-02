//! `Havok SDK/hkaskeletonmapper.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The mapper (`hkaSkeletonMapper`, Xbox PDB) is read off the code: `+0` the vtable, `+4` the 16-bit
//! size of the block, `+6` the 16-bit reference count, and from `+0x10` its mapping data (the
//! offsets below are from the mapper, `-0x10` from the data): `+0x10` and `+0x14` the two skeletons
//! (reference counted; `+0xc` the 16-bit parent index of every bone, `+0x18` the bone records of 8
//! bytes whose byte at `+4` says the translation is locked, `+0x1c` the bone count, `+0x24` the
//! reference pose of 0x30-byte transforms), `+0x18` an `hkArray` of simple mappings (data, size
//! `+0x1c`, capacity word `+0x20`; 0x40 bytes each: the 16-bit bone of the first skeleton at `+0`,
//! the 16-bit bone of the second at `+2`, a transform of the second in the first at `+0x10`, made
//! of the translation, the rotation at `+0x20` and the scale at `+0x30`), `+0x24` an `hkArray` of
//! chain mappings (size `+0x28`, capacity word `+0x2c`; 0x70 bytes each: the first and last bone
//! of the chain in the first skeleton (16 bits each, `+0` and `+2`), the first and last bone of
//! the chain in the second (`+4`, `+6`), the transform of the start at `+0x10` and the one of the
//! end at `+0x40`, each translation, rotation, scale), `+0x30` an `hkArray` of 16-bit bones of
//! the second skeleton that are not mapped (size `+0x34`, capacity word `+0x38`), a transform at
//! `+0x40` (translation, rotation `+0x50`, scale `+0x60`), a byte at `+0x70` (the unmapped bones
//! are kept local) and a dword at `+0x74` (the kind of mapping). A transform of a pose is 0x30
//! bytes: the translation, the rotation quaternion and the scale, each a 16-byte vector. Nothing
//! here has an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkakeyframehierarchyutility::{
    add4, mul4, rotate_and_move, rotate_half, splat, sub4,
};
use crate::units::havok_sdk::hkaragdollinstance::inverse_transform;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, dot3, load_vec, profile_record, store_vec, Vec4, PROFILE_SLOT, TLS_GET_VALUE,
    ZERO_F32,
};
use crate::x87::rsqrt_estimate;

/// The memory router (`0x00c8_5750`): its field `+0` is the bump cursor of the stack-like arena
/// and its field `+0x10` the allocator, whose virtual functions at `+0xc` (allocate through a size
/// in memory, which is rounded in place) and `+0x10` (free `(data, bytes)`) this unit uses.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::addReference` and `removeReference` (`00c90510`, `00c905b0`, `ECX` = the
/// object), `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element
/// size)`) and `_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkQuaternion::setMul` (`00c66320`, `this` = the result, `(a, b)`), the unnamed quaternion
/// product `00c74ac0` (`this` = the result, `(a, b)`), the unnamed in-place product `00ce0ca0`
/// (`ECX` = the quaternion, `(other)`) and the unnamed routine that sets a quaternion to the
/// rotation from one vector to another (`00cd9d10`, `ECX` = the result, `(from, to)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_PRODUCT: u32 = 0x00c7_4ac0;
const QUATERNION_MULTIPLY_IN_PLACE: u32 = 0x00ce_0ca0;
const QUATERNION_BETWEEN: u32 = 0x00cd_9d10;
/// The constructor and the destructor of the mapping data (`00ce5310`, `00ce5370`, `ECX` = the
/// data), `hkaSkeletonUtils::getBoneChain` (`00cde3d0`, cdecl `(flag, skeleton, first, last,
/// array)`, answers `EAX` = `flag`), and the two routines that enforce the locked translations
/// (`00cde1d0` `(skeleton, pose)` and `00cde2d0` `(skeleton, reference, pose)`, cdecl).
const DATA_CONSTRUCTOR: u32 = 0x00ce_5310;
const DATA_DESTRUCTOR: u32 = 0x00ce_5370;
const GET_BONE_CHAIN: u32 = 0x00cd_e3d0;
const LOCK_TRANSLATIONS: u32 = 0x00cd_e1d0;
const LOCK_TRANSLATIONS_FROM_REFERENCE: u32 = 0x00cd_e2d0;
/// The `hkOstream` routines `checkMapping` writes its messages with: the constructor (`00cb1600`,
/// `ECX` = the stream, `(buffer, size)`), the text (`00cc01c0`), pointer (`00cc05a0`) and float
/// (`00cc0320`) insertions (`ECX` = the stream, one argument, answer the stream) and the
/// destructor (`00cc00e0`, `ECX` = the stream).
const STREAM_CONSTRUCTOR: u32 = 0x00cb_1600;
const STREAM_TEXT: u32 = 0x00cc_01c0;
const STREAM_POINTER: u32 = 0x00cc_05a0;
const STREAM_FLOAT: u32 = 0x00cc_0320;
const STREAM_DESTRUCTOR: u32 = 0x00cc_00e0;
/// This unit's own routines, which the translations call the way the code does.
const NORMALISE_PAIR: u32 = 0x00ce_2a50;
const PLACE_BONE: u32 = 0x00ce_2ca0;
const PLACE_POSE: u32 = 0x00ce_2d10;
const PLACE_POSE_THUNK: u32 = 0x00ce_2f30;
const PLACE_POSE_INVERSE: u32 = 0x00ce_2f40;
const ASSIGN_DATA: u32 = 0x00ce_4ae0;
/// The vtable the constructor writes and the one the destructor leaves.
const VTABLE: u32 = 0x010c_b864;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The object that reports problems (`01268360`; its virtual function at `+0xc` takes `(severity,
/// code, text, file, line)`) and the file name it is told (`010cb8d0`).
const REPORTER: u32 = 0x0126_8360;
const REPORT_FILE: u32 = 0x010c_b8d0;
/// The vectors of four 3.0 (`010c71c0`) and four 0.5 (`010c71d0`), the `float` 0.01 (`01013ea4`)
/// and the texts `checkMapping` writes (`010c4d7c`, `010c4d88`, `010c4d90`, `010c4db0`,
/// `010cb870`).
const THREE_LANES: u32 = 0x010c_71c0;
const HALF_LANES: u32 = 0x010c_71d0;
const ALIGNED_TOLERANCE: u32 = 0x0101_3ea4;
const TEXT_BONES_A: u32 = 0x010c_4d7c;
const TEXT_AND_B: u32 = 0x010c_4d88;
const TEXT_NOT_ALIGNED: u32 = 0x010c_4d90;
const TEXT_CLOSE: u32 = 0x010c_4db0;
const TEXT_NOT_LOCKED: u32 = 0x010c_b870;
/// The profiling labels: the begin records of `mapPoseLocalSpace` (`LtMapPoseLocalSpace`) and
/// `mapPose` (`LtMapPoseModelSpace`), whose records are 16 bytes with `"Stinit"` (`010c6794`) in
/// the fourth dword, then `"StsimpleMappings"`, `"StchainMappings"`, `"StunmappedBones"` and
/// `"StenforceConstraints"`, and the shared end label (`01076f38`).
const LABEL_MAP_POSE_LOCAL_SPACE: u32 = 0x010c_b914;
const LABEL_MAP_POSE: u32 = 0x010c_b950;
const LABEL_INIT: u32 = 0x010c_6794;
const LABEL_SIMPLE_MAPPINGS: u32 = 0x010c_b900;
const LABEL_CHAIN_MAPPINGS: u32 = 0x010c_b8f0;
const LABEL_UNMAPPED_BONES: u32 = 0x010c_b940;
const LABEL_ENFORCE_CONSTRAINTS: u32 = 0x010c_b928;
const LABEL_END: u32 = 0x0107_6f38;
/// The bit that marks the storage of an `hkArray` as not owned, and the capacity mask.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The size of the `hkOstream` of `checkMapping` with its 0x200-byte buffer at `+0xc`.
const STREAM_BLOCK: u32 = 0x20c;

/// The 16-bit word at `addr`, sign extended (`MOVSX`).
fn word(e: &Engine, addr: u32) -> i32 {
    i32::from(e.mem.u16(addr) as i16)
}

/// The address of record `index` (signed) of `size` bytes each, from `base`.
fn record(base: u32, index: i32, size: u32) -> u32 {
    base.wrapping_add((index as u32).wrapping_mul(size))
}

/// The address of transform `index` of a pose (0x30 bytes each).
fn transform(base: u32, index: i32) -> u32 {
    record(base, index, 0x30)
}

/// The address of the memory router.
fn router(e: &mut Engine) -> u32 {
    e.call(MEMORY_ROUTER, &args![]).u32()
}

/// The allocator the memory router points at.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = router(e);
    e.mem.u32(router + 0x10)
}

/// Takes `bytes` (rounded up to 0x80) of the router's arena: moves its cursor and answers where it
/// was.
fn bump_cursor(e: &mut Engine, bytes: u32) -> u32 {
    let router = router(e);
    let start = e.mem.u32(router);
    let size = bytes.wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(start));
    start
}

/// Gives the arena back down to `start`.
fn restore_cursor(e: &mut Engine, start: u32) {
    let router = router(e);
    e.mem.set_u32(router, start);
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends the begin record of a function to the thread's record stream while its cursor (`+4`) is
/// below its limit (`+0xC`): a record of 16 bytes, `label`, the time stamp, and `"Stinit"` in the
/// fourth dword.
fn profile_begin(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 0xc, LABEL_INIT);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// `child` put on `parent`, into `out` (three transforms of 0x30 bytes): the translation is the
/// child's turned by the parent's rotation and moved by the parent's, the rotation is `setMul` of
/// the parent's and the child's (the result written before the scales are read) and the scale the
/// product of the two.
fn place_on(e: &mut Engine, parent: u32, child: u32, out: u32) {
    let moved = rotate_and_move(
        e,
        load_vec(e, parent + 0x10),
        load_vec(e, child),
        load_vec(e, parent),
    );
    store_vec(e, out, moved);
    e.call(
        QUATERNION_SET_MUL,
        &args![out + 0x10, parent + 0x10, child + 0x10],
    );
    let scale = mul4(load_vec(e, parent + 0x20), load_vec(e, child + 0x20));
    store_vec(e, out + 0x20, scale);
}

// Translated from 00ce29e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the mapper (the class is unconfirmed), `ECX` = the mapper: writes the vtable of
/// the full class, destroys the mapping data at `+0x10` (`00ce5370`) and leaves the vtable of the
/// base.
pub fn fn_00ce29e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(DATA_DESTRUCTOR, &args![this.addr() + 0x10]);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00ce2a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = `first`, `EAX` = `second` (two vectors): scales each to the unit length of its
/// first three lanes, without a guard for a zero length. Both squared lengths (`z + (y + x)`) go
/// into the lanes `(first, first, first, second)` of one vector, whose `RSQRTPS` estimate `r` is
/// refined once, `(3 - (r * x) * r) * (r * 0.5)`; `first` is stored multiplied by lane 0 and then
/// `second` (read again after that store) by lane 3. `EAX` is left as it came.
pub fn fn_00ce2a50(e: &mut Engine, first: Ptr, second: Ptr) -> Ptr {
    let (first_at, second_at) = (first.addr(), second.addr());
    let a = load_vec(e, first_at);
    let b = load_vec(e, second_at);
    let squared_a = dot3(a, a);
    let squared_b = dot3(b, b);
    let lanes = [squared_a, squared_a, squared_a, squared_b];
    let three = load_vec(e, THREE_LANES);
    let half = load_vec(e, HALF_LANES);
    let scale: Vec4 = std::array::from_fn(|lane| {
        let estimate = rsqrt_estimate(lanes[lane]);
        (three[lane] - (estimate * lanes[lane]) * estimate) * (estimate * half[lane])
    });
    store_vec(e, first_at, mul4(splat(scale[0]), a));
    let b = load_vec(e, second_at);
    store_vec(e, second_at, mul4(splat(scale[3]), b));
    second
}

/// Runs `body(stream, buffer)` between the construction and the destruction of an `hkOstream`
/// over a 0x200-byte buffer (`00cb1600`, `00cc00e0`).
fn with_stream(e: &mut Engine, body: impl FnOnce(&mut Engine, u32, u32)) {
    e.with_stack(STREAM_BLOCK, |e, block| {
        let stream = block.addr();
        let buffer = stream + 0xc;
        e.call(STREAM_CONSTRUCTOR, &args![stream, buffer, 0x200u32]);
        body(e, stream, buffer);
        e.call(STREAM_DESTRUCTOR, &args![stream]);
    });
}

/// Tells the reporter (virtual function `+0xc` of the object at `01268360`) about `text`.
fn report(e: &mut Engine, code: u32, text: u32, line: u32) {
    let reporter: u32 = e.global(REPORTER);
    e.vcall(reporter, 0xc, &args![1u32, code, text, REPORT_FILE, line]);
}

// Translated from 00ce2ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapper::checkMapping` (Xbox PDB), stack `(data)` = the mapping data (the mapper
/// from `+0x10`), plain `RET`. Unless the dword at `+0x64` is set it walks the simple mappings
/// (the count is read again after every one): when the bone of the second skeleton has its
/// translation locked and the length of the mapping's translation (`z * z + (y * y + x * x)` of
/// the first three lanes, square root) is above the float at `01013ea4` (the `x87` compare: not
/// for a length that is not a number), the message "Bones A:" / " and B:" / " are not fully
/// aligned (error :" with the two bone records, the length and the closing text goes to the
/// reporter with code `abbaefbf`. Then, again unless the dword is set, it looks for a bone of the
/// second skeleton with a locked translation and, when there is none, reports `010cb870` with
/// code `abba77a5`.
pub fn hka_skeleton_mapper_check_mapping(e: &mut Engine, data: Ptr) {
    let data = data.addr();
    if e.mem.u32(data + 0x64) == 0 && e.mem.i32(data + 0xc) > 0 {
        let mut offset = 0u32;
        let mut done = 0i32;
        loop {
            let entry = e.mem.u32(data + 8).wrapping_add(offset);
            let bones_a = e.mem.u32(e.mem.u32(data) + 0x18);
            let bone_a = record(bones_a, word(e, entry), 8);
            let bones_b = e.mem.u32(e.mem.u32(data + 4) + 0x18);
            let bone_b = record(bones_b, word(e, entry + 2), 8);
            if e.mem.u8(bone_b + 4) != 0 {
                let translation = load_vec(e, entry + 0x10);
                let length = dot3(translation, translation).sqrt();
                if e.mem.f32(ALIGNED_TOLERANCE) < length {
                    with_stream(e, |e, stream, buffer| {
                        let out = e.call(STREAM_TEXT, &args![stream, TEXT_BONES_A]).u32();
                        let out = e.call(STREAM_POINTER, &args![out, bone_a]).u32();
                        let out = e.call(STREAM_TEXT, &args![out, TEXT_AND_B]).u32();
                        let out = e.call(STREAM_POINTER, &args![out, bone_b]).u32();
                        let out = e.call(STREAM_TEXT, &args![out, TEXT_NOT_ALIGNED]).u32();
                        let out = e.call(STREAM_FLOAT, &args![out, x87_quiet(length)]).u32();
                        e.call(STREAM_TEXT, &args![out, TEXT_CLOSE]);
                        report(e, 0xabba_efbf, buffer, 0x1f3);
                    });
                }
            }
            offset = offset.wrapping_add(0x40);
            done += 1;
            if done >= e.mem.i32(data + 0xc) {
                break;
            }
        }
    }
    if e.mem.u32(data + 0x64) == 0 {
        let skeleton = e.mem.u32(data + 4);
        let count = e.mem.i32(skeleton + 0x1c);
        let bones = e.mem.u32(skeleton + 0x18).wrapping_add(4);
        for index in 0..count.max(0) {
            if e.mem.u8(record(bones, index, 8)) != 0 {
                return;
            }
        }
        with_stream(e, |e, stream, buffer| {
            e.call(STREAM_TEXT, &args![stream, TEXT_NOT_LOCKED]);
            report(e, 0xabba_77a5, buffer, 0x209);
        });
    }
}

// Translated from 00ce2ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, stack `(_unused_1, out, a, b)`, plain `RET` (the first word is never read; the caller
/// passes the dword at `+0x74` of the mapper): `out` becomes `a` moved onto the translation of
/// `b`: the translation is the scale of `a` times the translation of `b` plus the translation of
/// `a`, the rotation is `setMul` of the rotations of `a` and `b` and the scale is the scale of
/// `b` (read after the product).
pub fn fn_00ce2ca0(e: &mut Engine, _unused_1: u32, out: Ptr, a: Ptr, b: Ptr) {
    let (out, a, b) = (out.addr(), a.addr(), b.addr());
    let moved = add4(mul4(load_vec(e, a + 0x20), load_vec(e, b)), load_vec(e, a));
    store_vec(e, out, moved);
    e.call(QUATERNION_SET_MUL, &args![out + 0x10, a + 0x10, b + 0x10]);
    copy_vec(e, b + 0x20, out + 0x20);
}

// Translated from 00ce2d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, stack `(a, b, parents, count, out)`, plain `RET`. Without `parents` (a null pointer),
/// for each of the `count` (signed, nothing for a count that is not above zero) transforms of
/// `b`: `out[i]` is `b[i]` put on `a[i]`. With `parents`, an array of 16-bit indices, for every
/// `i` whose index is not negative (as a 16-bit number): `out[index]` is `b[i]` put on
/// `a[index]`; the rotation of `a[index]` is copied to a scratch quaternion first, as is its scale
/// (so `out` may be `a`).
pub fn fn_00ce2d10(e: &mut Engine, a: Ptr, b: Ptr, parents: Ptr, count: i32, out: Ptr) {
    let (a, b, parents, out) = (a.addr(), b.addr(), parents.addr(), out.addr());
    if parents == 0 {
        for index in 0..count.max(0) {
            place_on(
                e,
                transform(a, index),
                transform(b, index),
                transform(out, index),
            );
        }
        return;
    }
    e.with_stack(0x10, |e, scratch| {
        let scratch = scratch.addr();
        for i in 0..count.max(0) {
            let entry = e.mem.u16(parents + 2 * i as u32);
            if (entry as i16) < 0 {
                continue;
            }
            let index = i32::from(entry as i16);
            let parent = transform(a, index);
            let child = transform(b, i);
            let target = transform(out, index);
            let rotation = load_vec(e, parent + 0x10);
            let scale = load_vec(e, parent + 0x20);
            let translation = load_vec(e, parent);
            let half = rotate_half(e, rotation, load_vec(e, child));
            store_vec(e, target, add4(add4(half, half), translation));
            store_vec(e, scratch, rotation);
            e.call(
                QUATERNION_SET_MUL,
                &args![target + 0x10, scratch, child + 0x10],
            );
            let scaled = mul4(load_vec(e, child + 0x20), scale);
            store_vec(e, target + 0x20, scaled);
        }
    });
}

// Translated from 00ce2f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, stack `(a, b, parents, count, out)`, plain `RET`: a thunk, `JMP 00ce2d10` with the
/// same arguments.
pub fn fn_00ce2f30(e: &mut Engine, a: Ptr, b: Ptr, parents: Ptr, count: i32, out: Ptr) {
    e.call(PLACE_POSE, &args![a, b, parents, count, out]);
}

// Translated from 00ce2f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, stack `(a, b, parents, count, out)`, plain `RET`: `00ce2d10` with the inverses of the
/// `count` transforms of `b` instead of `b`. The inverses are built in the router's arena (its
/// cursor is moved past `count * 0x30` bytes, rounded up to 0x80, and put back in the end) with
/// the vectors of the inverse of a transform (`inverse_transform`: translation, rotation, scale).
pub fn fn_00ce2f40(e: &mut Engine, a: Ptr, b: Ptr, parents: Ptr, count: i32, out: Ptr) {
    let start = bump_cursor(e, (count as u32).wrapping_mul(0x30));
    for index in 0..count.max(0) {
        let [translation, rotation, scale] = inverse_transform(e, transform(b.addr(), index));
        let target = transform(start, index);
        store_vec(e, target, translation);
        store_vec(e, target + 0x10, rotation);
        store_vec(e, target + 0x20, scale);
    }
    e.call(PLACE_POSE, &args![a, start, parents, count, out]);
    restore_cursor(e, start);
}

// Translated from 00ce4a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the mapper (the class is unconfirmed), `ECX` = the mapper, stack `(source)` =
/// the mapping data to copy, `RET 4`: the reference count (16 bits at `+6`) becomes 1, the vtable
/// the one of the full class, the mapping data at `+0x10` is constructed (`00ce5310`) and
/// assigned from `source` (`00ce4ae0`). Returns `this`.
pub fn fn_00ce4a20(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, VTABLE);
    e.call(DATA_CONSTRUCTOR, &args![me + 0x10]);
    e.call(ASSIGN_DATA, &args![me + 0x10, source]);
    this
}

/// Copies a chain mapping (the four 16-bit bones, then the six vectors from `+0x10` on).
fn copy_chain_record(e: &mut Engine, to: u32, from: u32) {
    for offset in [0, 2, 4, 6] {
        let value = e.mem.u16(from + offset);
        e.mem.set_u16(to + offset, value);
    }
    for offset in (0x10..0x70).step_by(0x10) {
        copy_vec(e, from + offset, to + offset);
    }
}

/// Copies a simple mapping (the two 16-bit bones, then the three vectors from `+0x10` on).
fn copy_simple_record(e: &mut Engine, to: u32, from: u32) {
    for offset in [0, 2] {
        let value = e.mem.u16(from + offset);
        e.mem.set_u16(to + offset, value);
    }
    for offset in (0x10..0x40).step_by(0x10) {
        copy_vec(e, from + offset, to + offset);
    }
}

// Translated from 00ce4a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, `ECX` = the destination, stack `(source)`, `RET 4`: copies a chain mapping of 0x70
/// bytes (the four 16-bit words at `+0`, `+2`, `+4`, `+6` and the six vectors from `+0x10` to
/// `+0x6f`; `+8` to `+0xf` are not copied). Returns the destination.
pub fn fn_00ce4a50(e: &mut Engine, to: Ptr, from: Ptr) -> Ptr {
    copy_chain_record(e, to.addr(), from.addr());
    to
}

/// Makes the `hkArray` at `this + field` of `size`-byte records a copy of the one at `other +
/// field` (records copied by `copy`): the common part (the smaller of the two sizes) over the old
/// records, the rest constructed in place (skipped for a null address), after `_reserve` made room
/// when the capacity is below the other's size (to the larger of that and twice the capacity).
fn assign_records(
    e: &mut Engine,
    this: u32,
    other: u32,
    field: u32,
    size: u32,
    copy: fn(&mut Engine, u32, u32),
) {
    let array = this + field;
    let allocator = router_allocator(e);
    let other_size = e.mem.i32(other + field + 4);
    let common = e.mem.i32(array + 4).min(other_size);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < other_size {
        let wanted = other_size.max(capacity.wrapping_mul(2));
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, size]);
    }
    let from = e.mem.u32(other + field);
    let to = e.mem.u32(array);
    for index in 0..common.max(0) {
        copy(e, record(to, index, size), record(from, index, size));
    }
    for index in common..other_size {
        let target = record(to, index, size);
        if target != 0 {
            copy(e, target, record(from, index, size));
        }
    }
    e.mem.set_u32(array + 4, other_size as u32);
}

// Translated from 00ce4ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assignment of the mapping data, `ECX` = the data, stack `(other)`, `RET 4`; answers `this`.
/// Both skeletons are taken over with a reference added to the other's (`00c90510`) and the old
/// one's removed (`00c905b0`) when there is one; the arrays of simple mappings (`+8`, records of
/// 0x40 bytes) and of chain mappings (`+0x14`, records of 0x70) are assigned (`assign_records`);
/// the array of 16-bit words at `+0x20` gets, when its capacity is below the other's size, its
/// storage freed (virtual `+0x10` of the allocator, only for owned storage) and a new one
/// allocated through virtual `+0xc` with the size in bytes in memory (rounded in place; the new
/// capacity is half of that, rounded toward zero) and has the words copied; the three vectors at
/// `+0x30`, `+0x40` and `+0x50`, the byte at `+0x60` and the dword at `+0x64` are copied.
pub fn fn_00ce4ae0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (me, other) = (this.addr(), other.addr());
    for field in [0, 4] {
        let incoming = e.mem.u32(other + field);
        if incoming != 0 {
            e.call(ADD_REFERENCE, &args![incoming]);
        }
        let old = e.mem.u32(me + field);
        if old != 0 {
            e.call(REMOVE_REFERENCE, &args![old]);
        }
        let incoming = e.mem.u32(other + field);
        e.mem.set_u32(me + field, incoming);
    }
    assign_records(e, me, other, 8, 0x40, copy_simple_record);
    assign_records(e, me, other, 0x14, 0x70, copy_chain_record);
    let allocator = router_allocator(e);
    let capacity_word = e.mem.u32(me + 0x28);
    let capacity = capacity_word & CAPACITY_MASK;
    let other_size = e.mem.i32(other + 0x24);
    if (capacity as i32) < other_size {
        if capacity_word & NOT_OWNED == 0 {
            let data = e.mem.u32(me + 0x20);
            e.vcall(allocator, 0x10, &args![data, capacity.wrapping_mul(2)]);
        }
        e.with_stack(4, |e, bytes| {
            e.mem
                .set_u32(bytes.addr(), (other_size as u32).wrapping_mul(2));
            let data = e.vcall(allocator, 0xc, &args![bytes]).u32();
            e.mem.set_u32(me + 0x20, data);
            let rounded = e.mem.i32(bytes.addr());
            e.mem
                .set_u32(me + 0x28, (rounded.wrapping_sub(rounded >> 31) >> 1) as u32);
        });
    }
    let count = e.mem.i32(other + 0x24);
    e.mem.set_u32(me + 0x24, count as u32);
    let to = e.mem.u32(me + 0x20);
    let from = e.mem.u32(other + 0x20);
    for index in 0..count.max(0) as u32 {
        let value = e.mem.u16(from + 2 * index);
        e.mem.set_u16(to + 2 * index, value);
    }
    for offset in [0x30, 0x40, 0x50] {
        copy_vec(e, other + offset, me + offset);
    }
    let flag = e.mem.u8(other + 0x60);
    e.mem.set_u8(me + 0x60, flag);
    let kind = e.mem.u32(other + 0x64);
    e.mem.set_u32(me + 0x64, kind);
    this
}

/// Resizes the `hkArray` of 16-bit words at `array` to `count` words, 0xffff in the new ones: when
/// its capacity is below `count`, `_reserve` (to the larger of `count` and twice the capacity)
/// first; the words from the old size on are filled.
fn resize_words(e: &mut Engine, array: u32, count: i32) {
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < count {
        let wanted = count.max(capacity.wrapping_mul(2));
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 2u32]);
    }
    let size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    for index in size..count.max(size) {
        e.mem.set_u16(record(data, index, 2), 0xffff);
    }
    e.mem.set_u32(array + 4, count as u32);
}

/// Appends `value` to the `hkArray` of 16-bit words at `array`, `_reserveMore` first when it is
/// full (the size equals the capacity).
fn push_word(e: &mut Engine, array: u32, value: u16) {
    let allocator = router_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_MASK {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 2u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u16(data + 2 * size, value);
    e.mem.set_u32(array + 4, size + 1);
}

/// Empties the `hkArray` of 16-bit words at `array` (the 12 bytes of data, size, capacity word):
/// size zero, the storage freed through the allocator when it is owned (virtual `+0x10`, `(data,
/// bytes)`), data zero and the capacity word that of an empty array that owns nothing.
fn release_words(e: &mut Engine, array: u32) {
    let allocator = router_allocator(e);
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity_word & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity_word & CAPACITY_MASK).wrapping_mul(2)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

// Translated from 00ce35f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapper::initializeBindingMaps` (Xbox PDB), `ECX` = the mapper, stack `(bones_a,
/// slots_a, slots_b, bones_b)`, `RET 0x10`. `bones_a` is a list of bones of the first skeleton
/// (16-bit indices at the pointer in its `+0x10`, count at `+0x14`); `slots_a`, `slots_b` and
/// `bones_b` are `hkArray`s of 16-bit words. `slots_a` becomes one word per bone of the first
/// skeleton: the position of the bone in `bones_a`, 0xffff for none (the counters are 16-bit
/// numbers, as in the code). `bones_b` collects the bones of the second skeleton that the
/// mappings reach: the bone of every simple mapping whose first bone is in the list, then for
/// every chain mapping whose bones of the first chain (`getBoneChain` of the first skeleton from
/// the first to the last bone of the chain, then the second chain on the second skeleton) all are
/// in the list, the bones of the second chain; a chain with a bone outside the list, or a chain
/// `getBoneChain` fails on, adds nothing. `slots_b` becomes one word per bone of the second
/// skeleton, the position in `bones_b`, 0xffff for none. The two chain arrays are local and free
/// their storage when it is owned.
pub fn hka_skeleton_mapper_initialize_binding_maps(
    e: &mut Engine,
    this: Ptr,
    bones_a: Ptr,
    slots_a: Ptr,
    slots_b: Ptr,
    bones_b: Ptr,
) {
    let (me, bones_a, slots_a, slots_b, bones_b) = (
        this.addr(),
        bones_a.addr(),
        slots_a.addr(),
        slots_b.addr(),
        bones_b.addr(),
    );
    let skeleton_a = e.mem.u32(me + 0x10);
    let skeleton_b = e.mem.u32(me + 0x14);
    e.mem.set_u32(slots_a + 4, 0);
    let count_a = e.mem.i32(skeleton_a + 0x1c);
    resize_words(e, slots_a, count_a);
    let mut position = 0i16;
    if e.mem.i32(bones_a + 0x14) > 0 {
        loop {
            let bone = word(
                e,
                e.mem.u32(bones_a + 0x10) + 2 * i32::from(position) as u32,
            );
            let target = e.mem.u32(slots_a);
            e.mem.set_u16(record(target, bone, 2), position as u16);
            position = position.wrapping_add(1);
            if i32::from(position) >= e.mem.i32(bones_a + 0x14) {
                break;
            }
        }
    }
    e.mem.set_u32(bones_b + 4, 0);
    e.mem.set_u32(slots_b + 4, 0);
    let count_b = e.mem.i32(skeleton_b + 0x1c);
    resize_words(e, slots_b, count_b);
    let mut index = 0i16;
    if e.mem.i32(me + 0x1c) > 0 {
        loop {
            let entry = e
                .mem
                .u32(me + 0x18)
                .wrapping_add((i32::from(index) as u32).wrapping_mul(0x40));
            let bone = word(e, entry);
            let slot = e.mem.u16(record(e.mem.u32(slots_a), bone, 2)) as i16;
            if slot >= 0 {
                let mapped = e.mem.u16(entry + 2);
                push_word(e, bones_b, mapped);
            }
            index = index.wrapping_add(1);
            if i32::from(index) >= e.mem.i32(me + 0x1c) {
                break;
            }
        }
    }
    let mut chain = 0i16;
    if e.mem.i32(me + 0x28) > 0 {
        e.with_stack(0x20, |e, frame| {
            let frame = frame.addr();
            let (list_a, list_b, flag_a, flag_b) = (frame, frame + 0xc, frame + 0x18, frame + 0x1c);
            loop {
                let entry = e
                    .mem
                    .u32(me + 0x24)
                    .wrapping_add((i32::from(chain) as u32).wrapping_mul(0x70));
                for list in [list_a, list_b] {
                    e.mem.set_u32(list, 0);
                    e.mem.set_u32(list + 4, 0);
                    e.mem.set_u32(list + 8, NOT_OWNED);
                }
                let first = u32::from(e.mem.u16(entry));
                let last = u32::from(e.mem.u16(entry + 2));
                let flag = e
                    .call(
                        GET_BONE_CHAIN,
                        &args![flag_a, skeleton_a, first, last, list_a],
                    )
                    .u32();
                if e.mem.u8(flag) != 0 {
                    let entry = e
                        .mem
                        .u32(me + 0x24)
                        .wrapping_add((i32::from(chain) as u32).wrapping_mul(0x70));
                    let first = u32::from(e.mem.u16(entry + 4));
                    let last = u32::from(e.mem.u16(entry + 6));
                    let flag = e
                        .call(
                            GET_BONE_CHAIN,
                            &args![flag_b, skeleton_b, first, last, list_b],
                        )
                        .u32();
                    if e.mem.u8(flag) != 0 {
                        let size_a = e.mem.i32(list_a + 4);
                        let data_a = e.mem.u32(list_a);
                        let slots = e.mem.u32(slots_a);
                        let outside = (0..size_a.max(0)).any(|at| {
                            let bone = word(e, data_a + 2 * at as u32);
                            (e.mem.u16(record(slots, bone, 2)) as i16) < 0
                        });
                        if !outside {
                            for at in 0..e.mem.i32(list_b + 4).max(0) {
                                let bone = e.mem.u16(e.mem.u32(list_b) + 2 * at as u32);
                                push_word(e, bones_b, bone);
                            }
                        }
                    }
                }
                release_words(e, list_b);
                release_words(e, list_a);
                chain = chain.wrapping_add(1);
                if i32::from(chain) >= e.mem.i32(me + 0x28) {
                    break;
                }
            }
        });
    }
    let count_b = e.mem.i32(skeleton_b + 0x1c);
    resize_words(e, slots_b, count_b);
    let mut position = 0i16;
    if e.mem.i32(bones_b + 4) > 0 {
        loop {
            let bone = word(e, e.mem.u32(bones_b) + 2 * i32::from(position) as u32);
            let target = e.mem.u32(slots_b);
            e.mem.set_u16(record(target, bone, 2), position as u16);
            position = position.wrapping_add(1);
            if i32::from(position) >= e.mem.i32(bones_b + 4) {
                break;
            }
        }
    }
}

/// The 16-bit word at element `index` of the array of 16-bit words at `array`, sign extended.
fn pick(e: &Engine, array: u32, index: i32) -> i32 {
    word(e, record(array, index, 2))
}

/// The 16-bit word at element `index` of the array at `array`, as it is.
fn pick_raw(e: &Engine, array: u32, index: i32) -> u16 {
    e.mem.u16(record(array, index, 2))
}

// Translated from 00ce3090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapper::mapPoseLocalSpace` (Xbox PDB), stack `(mapper, pose_a, pose_b, map_a,
/// map_b, convert)` (`convert` a byte), plain `RET`. Maps the local pose of the first skeleton,
/// `pose_a`, to the local pose of the second, `pose_b`. `map_a` and `map_b`, both given or the
/// mapping is not partial, are arrays of 16-bit indices that say where a bone of each skeleton is
/// in the poses (negative for none; a bone whose `map_a` entry is negative is skipped). With
/// `convert` the poses are taken in the other space: the router's arena (its cursor moved past
/// the bone count of the first skeleton times 0x30, rounded up to 0x80, and put back in the end)
/// holds `pose_a` put on the reference pose of the first skeleton (`00ce2f30` with `map_a`) and
/// the result `pose_b` is brought back by the inverse of the reference pose of the second
/// (`00ce2f40` with `map_b`). Every simple mapping is placed with `00ce2ca0`. Every chain mapping
/// adds up the lengths of the translations along the first chain (from the last bone up to,
/// not including, the first; a vector turned and moved by each bone on the way; with a partial
/// mapping the chain fails at a bone with a negative index, and so does the chain with a first
/// bone whose index is negative) and scales the translations of the second chain by that length
/// times the mapping's last scale, accumulating the turned and moved vector the same way; then
/// places the first bone of the second chain (`00ce2ca0`), normalises the two vectors
/// (`00ce2a50`), turns the first bone by the rotation from one to the other (`00cd9d10`,
/// `00c74ac0` with the mapping's last translation, `00ce0ca0`) and sets the last bone's rotation
/// to the product (`setMul`) of the mapping's last rotation and the first skeleton's. Its profile
/// records are `LtMapPoseLocalSpace` (16 bytes), `StsimpleMappings`, `StchainMappings` and the
/// end label.
pub fn hka_skeleton_mapper_map_pose_local_space(
    e: &mut Engine,
    this: Ptr,
    pose_a: Ptr,
    pose_b: Ptr,
    map_a: Ptr,
    map_b: Ptr,
    convert: u8,
) {
    let me = this.addr();
    let (pose_a, pose_b, map_a, map_b) = (pose_a.addr(), pose_b.addr(), map_a.addr(), map_b.addr());
    profile_begin(e, LABEL_MAP_POSE_LOCAL_SPACE);
    profile_record(e, LABEL_SIMPLE_MAPPINGS);
    let partial = map_a != 0 && map_b != 0;
    let skeleton_a = e.mem.u32(me + 0x10);
    let count_a = e.mem.u32(skeleton_a + 0x1c);
    let start = bump_cursor(e, count_a.wrapping_mul(0x30));
    let source = if convert != 0 {
        let skeleton_a = e.mem.u32(me + 0x10);
        let count = e.mem.u32(skeleton_a + 0x1c);
        let reference = e.mem.u32(skeleton_a + 0x24);
        e.call(
            PLACE_POSE_THUNK,
            &args![pose_a, reference, map_a, count, start],
        );
        start
    } else {
        pose_a
    };
    let simple_count = e.mem.i32(me + 0x1c);
    for index in 0..simple_count.max(0) {
        let entry = e
            .mem
            .u32(me + 0x18)
            .wrapping_add((index as u32).wrapping_mul(0x40));
        let mut bone_a = word(e, entry);
        let mut bone_b = word(e, entry + 2);
        if partial {
            bone_a = pick(e, map_a, bone_a);
            if bone_a < 0 {
                continue;
            }
            bone_b = pick(e, map_b, bone_b);
        }
        let unused = e.mem.u32(me + 0x74);
        e.call(
            PLACE_BONE,
            &args![
                unused,
                transform(pose_b, bone_b),
                entry + 0x10,
                transform(source, bone_a)
            ],
        );
    }
    profile_record(e, LABEL_CHAIN_MAPPINGS);
    let chain_count = e.mem.i32(me + 0x28);
    if chain_count > 0 {
        e.with_stack(0x50, |e, frame| {
            let frame = frame.addr();
            let (vector_b, vector_a) = (frame, frame + 0x10);
            let (rotation, copy, product) = (frame + 0x20, frame + 0x30, frame + 0x40);
            for index in 0..chain_count {
                let entry = e
                    .mem
                    .u32(me + 0x24)
                    .wrapping_add((index as u32).wrapping_mul(0x70));
                let first_a = e.mem.u16(entry);
                let last_a = e.mem.u16(entry + 2);
                let first_b = e.mem.u16(entry + 4);
                let last_b = e.mem.u16(entry + 6);
                let mut length: f32 = e.global(ZERO_F32);
                let mut ok = true;
                let mut walked_a: Vec4 = [0.0; 4];
                if (last_a as i16) > (first_a as i16) {
                    let mut current = last_a;
                    let mut vector: Vec4 = [0.0; 4];
                    loop {
                        let bone = if partial {
                            pick_raw(e, map_a, i32::from(current as i16))
                        } else {
                            current
                        };
                        if (bone as i16) < 0 {
                            ok = false;
                            break;
                        }
                        let pose = transform(source, i32::from(bone as i16));
                        let translation = load_vec(e, pose);
                        vector = rotate_and_move(e, load_vec(e, pose + 0x10), vector, translation);
                        let skeleton_a = e.mem.u32(me + 0x10);
                        let parents = e.mem.u32(skeleton_a + 0xc);
                        current = pick_raw(e, parents, i32::from(current as i16));
                        length += dot3(translation, translation).sqrt();
                        if (current as i16) <= (first_a as i16) {
                            break;
                        }
                    }
                    walked_a = vector;
                }
                if partial && !(ok && pick(e, map_a, i32::from(first_a as i16)) >= 0) {
                    continue;
                }
                let scale = mul4(splat(length), load_vec(e, entry + 0x60));
                let mut walked_b: Vec4 = [0.0; 4];
                if (last_b as i16) > (first_b as i16) {
                    let mut current = last_b;
                    loop {
                        let bone = if partial {
                            pick(e, map_b, i32::from(current as i16))
                        } else {
                            i32::from(current as i16)
                        };
                        let pose = transform(pose_b, bone);
                        let scaled = mul4(scale, load_vec(e, pose));
                        store_vec(e, pose, scaled);
                        let rotation_of_bone = load_vec(e, pose + 0x10);
                        walked_b =
                            rotate_and_move(e, rotation_of_bone, walked_b, load_vec(e, pose));
                        let skeleton_b = e.mem.u32(me + 0x14);
                        let parents = e.mem.u32(skeleton_b + 0xc);
                        current = pick_raw(e, parents, i32::from(current as i16));
                        if (current as i16) <= (first_b as i16) {
                            break;
                        }
                    }
                }
                let mut start_a = i32::from(first_a as i16);
                let mut end_a = i32::from(last_a as i16);
                let mut start_b = i32::from(first_b as i16);
                let mut end_b = i32::from(last_b as i16);
                if partial {
                    start_a = pick(e, map_a, start_a);
                    end_a = pick(e, map_a, end_a);
                    end_b = pick(e, map_b, end_b);
                    start_b = pick(e, map_b, start_b);
                }
                let placed = transform(pose_b, start_b);
                let unused = e.mem.u32(me + 0x74);
                e.call(
                    PLACE_BONE,
                    &args![unused, placed, entry + 0x10, transform(source, start_a)],
                );
                store_vec(e, vector_b, walked_b);
                store_vec(e, vector_a, walked_a);
                e.call(NORMALISE_PAIR, &args![vector_a, vector_b]);
                e.call(QUATERNION_BETWEEN, &args![rotation, vector_b, vector_a]);
                copy_vec(e, entry + 0x40, copy);
                e.call(QUATERNION_PRODUCT, &args![product, rotation, copy]);
                e.call(QUATERNION_MULTIPLY_IN_PLACE, &args![placed + 0x10, product]);
                e.call(
                    QUATERNION_SET_MUL,
                    &args![
                        transform(pose_b, end_b) + 0x10,
                        entry + 0x50,
                        transform(source, end_a) + 0x10
                    ],
                );
            }
        });
    }
    if convert != 0 {
        let skeleton_b = e.mem.u32(me + 0x14);
        let count = e.mem.u32(skeleton_b + 0x1c);
        let reference = e.mem.u32(skeleton_b + 0x24);
        e.call(
            PLACE_POSE_INVERSE,
            &args![pose_b, reference, map_b, count, pose_b],
        );
    }
    profile_record(e, LABEL_END);
    restore_cursor(e, start);
}

// Translated from 00ce3a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapper::mapPose` (Xbox PDB), stack `(mapper, pose_a, local_pose_b, pose_b,
/// constraint_mode)`, plain `RET`: maps the model-space pose of the first skeleton, `pose_a`, to
/// `pose_b` of the second (`local_pose_b` the local pose of the second, whose translations the chains
/// and the unmapped bones keep). Every simple mapping puts the mapping's transform on the bone of
/// `pose_a` (`place_on`, with `setMul` for the rotation). Every chain mapping places its first
/// bone the same way, then takes a list of the bones of the second chain (`getBoneChain` of the
/// second skeleton, in the arena: the cursor moves past twice the chain length rounded up to
/// 0x80 and goes back in the end; the list is freed through the allocator when it owns its
/// storage), accumulates the translation of the last bone of the chain through the bones in
/// between (each scaled, turned and moved), turns the first bone by the rotation from that
/// direction to the direction of the mapping's end point (the end translation put on the last
/// bone of the first chain; two normalised vectors by `00ce2a50`, `00cd9d10`, `setMul`; the
/// product of the last bone's rotation with the mapping's end rotation, `00c66320`, is not
/// used), and puts every bone of the list after the first on its predecessor with the local
/// pose. Then, when the byte at `+0x70` is set, every unmapped bone (the 16-bit list at `+0x30`)
/// is put on its parent in `pose_b`, or copied from `local_pose_b` when it has none (parent index
/// 0xffff). `constraint_mode` 1 calls `00cde1d0(skeleton, pose_b)`, 2 calls
/// `00cde2d0(skeleton, local_pose_b, pose_b)`. Profile records: `LtMapPoseModelSpace` (16 bytes),
/// `StsimpleMappings`, `StchainMappings`, `StunmappedBones`, `StenforceConstraints` and the end
/// label.
pub fn hka_skeleton_mapper_map_pose(
    e: &mut Engine,
    this: Ptr,
    pose_a: Ptr,
    local_pose_b: Ptr,
    pose_b: Ptr,
    constraint_mode: u32,
) {
    let me = this.addr();
    let (pose_a, local_pose_b, pose_b) = (pose_a.addr(), local_pose_b.addr(), pose_b.addr());
    profile_begin(e, LABEL_MAP_POSE);
    profile_record(e, LABEL_SIMPLE_MAPPINGS);
    let simple_count = e.mem.i32(me + 0x1c);
    for index in 0..simple_count.max(0) {
        let entry = e
            .mem
            .u32(me + 0x18)
            .wrapping_add((index as u32).wrapping_mul(0x40));
        let parent = transform(pose_a, word(e, entry));
        let out = transform(pose_b, word(e, entry + 2));
        place_on(e, parent, entry + 0x10, out);
    }
    profile_record(e, LABEL_CHAIN_MAPPINGS);
    let chain_count = e.mem.i32(me + 0x28);
    if chain_count > 0 {
        e.with_stack(0x70, |e, frame| {
            let frame = frame.addr();
            let (list, flag) = (frame, frame + 0xc);
            let (first_vector, second_vector) = (frame + 0x10, frame + 0x20);
            let (rotation, copy, product, spare) =
                (frame + 0x30, frame + 0x40, frame + 0x50, frame + 0x60);
            for index in 0..chain_count {
                let entry = e
                    .mem
                    .u32(me + 0x24)
                    .wrapping_add((index as u32).wrapping_mul(0x70));
                let start_a = word(e, entry);
                let end_a = word(e, entry + 2);
                let first_b = e.mem.u16(entry + 4);
                let last_b = e.mem.u16(entry + 6);
                let start_b = i32::from(first_b as i16);
                let end_b = i32::from(last_b as i16);
                let placed = transform(pose_b, start_b);
                place_on(e, transform(pose_a, start_a), entry + 0x10, placed);
                let length = end_b.wrapping_sub(start_b).wrapping_add(1);
                e.mem.set_u32(list, 0);
                e.mem.set_u32(list + 4, 0);
                e.mem.set_u32(list + 8, NOT_OWNED);
                let begin = bump_cursor(e, (length as u32).wrapping_mul(2));
                e.mem.set_u32(list, begin);
                e.mem.set_u32(list + 8, length as u32 | NOT_OWNED);
                let skeleton_b = e.mem.u32(me + 0x14);
                e.call(
                    GET_BONE_CHAIN,
                    &args![
                        flag,
                        skeleton_b,
                        u32::from(first_b),
                        u32::from(last_b),
                        list
                    ],
                );
                let mut vector = load_vec(e, transform(local_pose_b, end_b));
                let mut at = e.mem.i32(list + 4).wrapping_sub(2);
                while at >= 1 {
                    let bone = transform(local_pose_b, pick(e, e.mem.u32(list), at));
                    let scaled = mul4(load_vec(e, bone + 0x20), vector);
                    vector =
                        rotate_and_move(e, load_vec(e, bone + 0x10), scaled, load_vec(e, bone));
                    at -= 1;
                }
                let scaled = mul4(load_vec(e, placed + 0x20), vector);
                let half = rotate_half(e, load_vec(e, placed + 0x10), scaled);
                let end_pose = transform(pose_a, end_a);
                let target = rotate_and_move(
                    e,
                    load_vec(e, end_pose + 0x10),
                    load_vec(e, entry + 0x40),
                    load_vec(e, end_pose),
                );
                e.call(
                    QUATERNION_SET_MUL,
                    &args![spare, end_pose + 0x10, entry + 0x50],
                );
                let origin = load_vec(e, placed);
                let first = sub4(add4(add4(half, half), origin), origin);
                store_vec(e, first_vector, first);
                store_vec(e, second_vector, sub4(target, origin));
                e.call(NORMALISE_PAIR, &args![first_vector, second_vector]);
                e.call(
                    QUATERNION_BETWEEN,
                    &args![rotation, first_vector, second_vector],
                );
                copy_vec(e, placed + 0x10, copy);
                e.call(QUATERNION_SET_MUL, &args![product, rotation, copy]);
                copy_vec(e, product, placed + 0x10);
                let mut at = 1i32;
                while at < e.mem.i32(list + 4).wrapping_sub(1) {
                    let data = e.mem.u32(list);
                    let parent = pick(e, data, at - 1);
                    let child = pick(e, data, at);
                    place_on(
                        e,
                        transform(pose_b, parent),
                        transform(local_pose_b, child),
                        transform(pose_b, child),
                    );
                    at += 1;
                }
                if begin == e.mem.u32(list) {
                    e.mem.set_u32(list + 4, 0);
                }
                restore_cursor(e, begin);
                release_words(e, list);
            }
        });
    }
    profile_record(e, LABEL_UNMAPPED_BONES);
    let keep_local = e.mem.u8(me + 0x70) != 0;
    let unmapped_count = e.mem.i32(me + 0x34);
    if keep_local {
        for index in 0..unmapped_count.max(0) {
            let bone = pick(e, e.mem.u32(me + 0x30), index);
            let skeleton_b = e.mem.u32(me + 0x14);
            let parent = pick_raw(e, e.mem.u32(skeleton_b + 0xc), bone);
            let local = transform(local_pose_b, bone);
            let out = transform(pose_b, bone);
            if parent == 0xffff {
                for offset in [0, 0x10, 0x20] {
                    copy_vec(e, local + offset, out + offset);
                }
            } else {
                let parent = transform(pose_b, i32::from(parent as i16));
                place_on(e, parent, local, out);
            }
        }
    }
    profile_record(e, LABEL_ENFORCE_CONSTRAINTS);
    let skeleton_b = e.mem.u32(me + 0x14);
    match constraint_mode {
        1 => {
            e.call(LOCK_TRANSLATIONS, &args![skeleton_b, pose_b]);
        }
        2 => {
            e.call(
                LOCK_TRANSLATIONS_FROM_REFERENCE,
                &args![skeleton_b, local_pose_b, pose_b],
            );
        }
        _ => {}
    }
    profile_record(e, LABEL_END);
}

// Translated from 00ce4ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00ce_29e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ce4ab0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00ce_29e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_29e0, fn_00ce29e0(Ptr)),
        entry!(0x00ce_2a50, fn_00ce2a50(Ptr, Ptr) -> Ptr),
        entry!(0x00ce_2ae0, hka_skeleton_mapper_check_mapping(Ptr)),
        entry!(0x00ce_2ca0, fn_00ce2ca0(u32, Ptr, Ptr, Ptr)),
        entry!(0x00ce_2d10, fn_00ce2d10(Ptr, Ptr, Ptr, i32, Ptr)),
        entry!(0x00ce_2f30, fn_00ce2f30(Ptr, Ptr, Ptr, i32, Ptr)),
        entry!(0x00ce_2f40, fn_00ce2f40(Ptr, Ptr, Ptr, i32, Ptr)),
        entry!(
            0x00ce_3090,
            hka_skeleton_mapper_map_pose_local_space(Ptr, Ptr, Ptr, Ptr, Ptr, u8)
        ),
        entry!(
            0x00ce_35f0,
            hka_skeleton_mapper_initialize_binding_maps(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00ce_3a00,
            hka_skeleton_mapper_map_pose(Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00ce_4a20, fn_00ce4a20(Ptr, Ptr) -> Ptr),
        entry!(0x00ce_4a50, fn_00ce4a50(Ptr, Ptr) -> Ptr),
        entry!(0x00ce_4ab0, deleting_destructor_00ce4ab0(Ptr, u8) -> Ptr),
        entry!(0x00ce_4ae0, fn_00ce4ae0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::store_bits;
    use std::cell::RefCell;
    use std::f32::consts::FRAC_1_SQRT_2;
    use std::rc::Rc;

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
    fn deleting_destructor_00ce4ab0_frees_only_when_bit_zero_is_set() {
        check(0x00ce_4ab0, 0x00ce_29e0);
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Transform = [Vec4; 3];

    const SIZED_ALLOCATE_DOUBLE: u32 = 0x0900_000c;
    const FREE_DOUBLE: u32 = 0x0900_0010;
    const REPORT_DOUBLE: u32 = 0x0900_0030;
    const IDENTITY: Vec4 = [0.0, 0.0, 0.0, 1.0];

    fn new_log() -> Log {
        Rc::new(RefCell::new(Vec::new()))
    }

    fn close(a: Vec4, b: Vec4, tolerance: f32) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() <= tolerance)
    }

    /// An engine with the exe constants the translations read: -0.5, 3.0, 0.5 and 1.0 in every
    /// lane, the sign masks, the keep-three-lanes mask and the 0.01 of `checkMapping`.
    fn exe() -> Engine {
        let mut e = engine();
        store_vec(&mut e, 0x010c_71e0, [-0.5; 4]);
        store_vec(&mut e, THREE_LANES, [3.0; 4]);
        store_vec(&mut e, HALF_LANES, [0.5; 4]);
        store_vec(&mut e, 0x010c_7170, [1.0; 4]);
        store_bits(&mut e, 0x010c_44d0, [0x8000_0000; 4]);
        store_bits(
            &mut e,
            0x010c_4c00,
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0],
        );
        store_bits(
            &mut e,
            0x010c_4c10,
            [0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0],
        );
        e.mem.set_f32(ALIGNED_TOLERANCE, 0.01);
        e
    }

    fn put_transform(e: &mut Engine, at: u32, transform: Transform) {
        for (k, v) in transform.into_iter().enumerate() {
            store_vec(e, at + 0x10 * k as u32, v);
        }
    }

    fn get_transform(e: &Engine, at: u32) -> Transform {
        std::array::from_fn(|k| load_vec(e, at + 0x10 * k as u32))
    }

    /// A pose: the transforms one after the other.
    fn pose(e: &mut Engine, transforms: &[Transform]) -> u32 {
        let at = e.mem.alloc(0x30 * transforms.len().max(1) as u32);
        for (k, transform) in transforms.iter().enumerate() {
            put_transform(e, at + 0x30 * k as u32, *transform);
        }
        at
    }

    /// An array of 16-bit words.
    fn words16(e: &mut Engine, values: &[u16]) -> u32 {
        let at = e.mem.alloc(2 * values.len().max(1) as u32);
        for (k, value) in values.iter().enumerate() {
            e.mem.set_u16(at + 2 * k as u32, *value);
        }
        at
    }

    /// A double that logs the words it is called with.
    fn log_calls(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    /// `setMul` as the sum of its two rotations.
    fn add_quaternions(e: &mut Engine) {
        e.register_double(QUATERNION_SET_MUL, |e, a| {
            let sum = add4(load_vec(e, a[1]), load_vec(e, a[2]));
            store_vec(e, a[0], sum);
            Ret::default()
        });
    }

    /// A memory router whose arena starts at the second answer and whose allocator frees through a
    /// logging double and allocates through a size in memory (rounded up to 16).
    fn heap(e: &mut Engine, log: &Log) -> (u32, u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SIZED_ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let arena = e.mem.alloc(0x1000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, arena);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(SIZED_ALLOCATE_DOUBLE, move |e, a| {
            let size = e.mem.u32(a[1]);
            l.borrow_mut()
                .push((SIZED_ALLOCATE_DOUBLE, vec![a[0], size]));
            let rounded = (size + 15) & !15;
            e.mem.set_u32(a[1], rounded);
            Ret {
                eax: e.mem.alloc(rounded),
                ..Ret::default()
            }
        });
        log_calls(e, log, FREE_DOUBLE);
        (router, arena)
    }

    /// Moves the records of the array at `array` into new storage for `capacity` records of `size`
    /// bytes, owned.
    fn grow(e: &mut Engine, array: u32, capacity: u32, size: u32) {
        let new = e.mem.alloc(capacity * size);
        let old = e.mem.u32(array);
        for k in 0..e.mem.u32(array + 4) * size {
            let byte = e.mem.u8(old + k);
            e.mem.set_u8(new + k, byte);
        }
        e.mem.set_u32(array, new);
        e.mem.set_u32(array + 8, capacity);
    }

    /// `_reserve` and `_reserveMore` as allocations of new storage.
    fn array_doubles(e: &mut Engine, log: &Log) {
        let l = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            grow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        let l = Rc::clone(log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            l.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let capacity = e.mem.u32(a[1] + 8) & CAPACITY_MASK;
            grow(e, a[1], (capacity * 2).max(4), a[2]);
            Ret::default()
        });
    }

    /// A skeleton: the parents of its bones, the flags that say which bones have a locked
    /// translation, no reference pose.
    fn skeleton(e: &mut Engine, parents: &[u16], locked: &[u8]) -> u32 {
        let at = e.mem.alloc(0x30);
        let list = words16(e, parents);
        e.mem.set_u32(at + 0xc, list);
        let bones = e.mem.alloc(8 * parents.len().max(1) as u32);
        for (k, flag) in locked.iter().enumerate() {
            e.mem.set_u8(bones + 8 * k as u32 + 4, *flag);
        }
        e.mem.set_u32(at + 0x18, bones);
        e.mem.set_u32(at + 0x1c, parents.len() as u32);
        at
    }

    fn bone_record(e: &Engine, skeleton: u32, bone: u32) -> u32 {
        e.mem.u32(skeleton + 0x18) + 8 * bone
    }

    // ---- the destructor and the vector pair ----

    #[test]
    fn the_destructor_destroys_the_data_between_the_two_vtables() {
        let mut e = exe();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let l = Rc::clone(&seen);
        e.register_double(DATA_DESTRUCTOR, move |e, a| {
            l.borrow_mut().push((a[0], e.mem.u32(a[0] - 0x10)));
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.call(0x00ce_29e0, &args![this]);
        assert_eq!(*seen.borrow(), vec![(this + 0x10, VTABLE)]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn two_vectors_are_scaled_to_unit_length_by_their_first_three_lanes() {
        let mut e = exe();
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        store_vec(&mut e, first, [3.0, 4.0, 0.0, 10.0]);
        store_vec(&mut e, second, [0.0, 0.0, 2.0, 5.0]);
        let got = e.call(0x00ce_2a50, &args![first, second]).u32();
        assert_eq!(got, second);
        // Lane 3 is scaled by the same factor as the others (0.2 and 0.5).
        assert!(close(load_vec(&e, first), [0.6, 0.8, 0.0, 2.0], 1.0e-4));
        assert!(close(load_vec(&e, second), [0.0, 0.0, 1.0, 2.5], 1.0e-4));
    }

    // ---- checkMapping ----

    fn stream_doubles(e: &mut Engine, log: &Log) {
        let l = Rc::clone(log);
        e.register_double(STREAM_CONSTRUCTOR, move |_, a| {
            l.borrow_mut().push((STREAM_CONSTRUCTOR, vec![a[2]]));
            Ret::default()
        });
        for address in [STREAM_TEXT, STREAM_POINTER, STREAM_FLOAT] {
            let l = Rc::clone(log);
            e.register_double(address, move |_, a| {
                l.borrow_mut().push((address, vec![a[1]]));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let l = Rc::clone(log);
        e.register_double(STREAM_DESTRUCTOR, move |_, _| {
            l.borrow_mut().push((STREAM_DESTRUCTOR, vec![]));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, REPORT_DOUBLE);
        let reporter = e.mem.alloc(0x10);
        e.mem.set_u32(reporter, vtable);
        e.set_global(REPORTER, reporter);
        let l = Rc::clone(log);
        e.register_double(REPORT_DOUBLE, move |_, a| {
            l.borrow_mut()
                .push((REPORT_DOUBLE, vec![a[1], a[2], a[4], a[5]]));
            Ret::default()
        });
    }

    /// The mapping data `checkMapping` reads: the two skeletons, the simple mappings (the two
    /// bones and the translation) and the dword at `+0x64`.
    fn check_data(
        e: &mut Engine,
        skeletons: (u32, u32),
        entries: &[(i16, i16, Vec4)],
        kind: u32,
    ) -> u32 {
        let data = e.mem.alloc(0x70);
        let records = e.mem.alloc(0x40 * entries.len().max(1) as u32);
        for (k, (a, b, translation)) in entries.iter().enumerate() {
            let at = records + 0x40 * k as u32;
            e.mem.set_u16(at, *a as u16);
            e.mem.set_u16(at + 2, *b as u16);
            store_vec(e, at + 0x10, *translation);
        }
        e.mem.set_u32(data, skeletons.0);
        e.mem.set_u32(data + 4, skeletons.1);
        e.mem.set_u32(data + 8, records);
        e.mem.set_u32(data + 0xc, entries.len() as u32);
        e.mem.set_u32(data + 0x64, kind);
        data
    }

    #[test]
    fn a_locked_bone_that_is_not_aligned_is_reported_with_both_bones_and_the_length() {
        let mut e = exe();
        let log = new_log();
        stream_doubles(&mut e, &log);
        let a = skeleton(&mut e, &[0xffff, 0], &[0, 0]);
        let b = skeleton(&mut e, &[0xffff, 0], &[1, 0]);
        let entries = [
            (0, 0, [3.0, 4.0, 0.0, 9.0]),
            // The second skeleton's bone has no locked translation.
            (1, 1, [30.0, 0.0, 0.0, 0.0]),
            // Locked, but the length (0.001) is below the limit of 0.01.
            (1, 0, [0.001, 0.0, 0.0, 0.0]),
        ];
        let data = check_data(&mut e, (a, b), &entries, 0);
        e.call(0x00ce_2ae0, &args![data]);
        let bone_a = bone_record(&e, a, 0);
        let bone_b = bone_record(&e, b, 0);
        assert_eq!(
            *log.borrow(),
            vec![
                (STREAM_CONSTRUCTOR, vec![0x200]),
                (STREAM_TEXT, vec![TEXT_BONES_A]),
                (STREAM_POINTER, vec![bone_a]),
                (STREAM_TEXT, vec![TEXT_AND_B]),
                (STREAM_POINTER, vec![bone_b]),
                (STREAM_TEXT, vec![TEXT_NOT_ALIGNED]),
                (STREAM_FLOAT, vec![5.0f32.to_bits()]),
                (STREAM_TEXT, vec![TEXT_CLOSE]),
                (REPORT_DOUBLE, vec![1, 0xabba_efbf, REPORT_FILE, 0x1f3]),
                (STREAM_DESTRUCTOR, vec![]),
            ]
        );
    }

    #[test]
    fn a_translation_that_is_not_a_number_is_never_reported() {
        let mut e = exe();
        let log = new_log();
        stream_doubles(&mut e, &log);
        let a = skeleton(&mut e, &[0xffff], &[0]);
        let b = skeleton(&mut e, &[0xffff], &[1]);
        let entries = [(0, 0, [f32::NAN, 0.0, 0.0, 0.0])];
        let data = check_data(&mut e, (a, b), &entries, 0);
        e.call(0x00ce_2ae0, &args![data]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn nothing_is_checked_when_the_kind_is_set() {
        let mut e = exe();
        let log = new_log();
        stream_doubles(&mut e, &log);
        let a = skeleton(&mut e, &[0xffff], &[0]);
        let b = skeleton(&mut e, &[0xffff], &[0]);
        let entries = [(0, 0, [30.0, 0.0, 0.0, 0.0])];
        let data = check_data(&mut e, (a, b), &entries, 2);
        e.call(0x00ce_2ae0, &args![data]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn a_second_skeleton_without_a_locked_translation_is_reported() {
        let mut e = exe();
        let log = new_log();
        stream_doubles(&mut e, &log);
        let a = skeleton(&mut e, &[0xffff], &[0]);
        let b = skeleton(&mut e, &[0xffff, 0], &[0, 0]);
        let data = check_data(&mut e, (a, b), &[], 0);
        e.call(0x00ce_2ae0, &args![data]);
        assert_eq!(
            *log.borrow(),
            vec![
                (STREAM_CONSTRUCTOR, vec![0x200]),
                (STREAM_TEXT, vec![TEXT_NOT_LOCKED]),
                (REPORT_DOUBLE, vec![1, 0xabba_77a5, REPORT_FILE, 0x209]),
                (STREAM_DESTRUCTOR, vec![]),
            ]
        );
        // An empty second skeleton is reported as well; one locked bone is enough to be quiet.
        log.borrow_mut().clear();
        let c = skeleton(&mut e, &[0xffff, 0], &[0, 1]);
        let data = check_data(&mut e, (a, c), &[], 0);
        e.call(0x00ce_2ae0, &args![data]);
        assert!(log.borrow().is_empty());
    }

    // ---- the placing routines ----

    #[test]
    fn a_bone_is_placed_on_the_translation_of_the_other_with_the_scale_of_the_other() {
        let mut e = exe();
        add_quaternions(&mut e);
        let a = pose(
            &mut e,
            &[[
                [10.0, 20.0, 30.0, 40.0],
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 3.0, 4.0, 1.0],
            ]],
        );
        let b = pose(
            &mut e,
            &[[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 9.0, 9.0, 9.0],
            ]],
        );
        let out = pose(&mut e, &[[[0.0; 4]; 3]]);
        e.call(0x00ce_2ca0, &args![0xdead_beefu32, out, a, b]);
        let got = get_transform(&e, out);
        assert_eq!(got[0], [12.0, 26.0, 42.0, 44.0]);
        assert_eq!(got[1], [6.0, 8.0, 10.0, 12.0]);
        assert_eq!(got[2], [9.0; 4]);
    }

    /// A transform with the identity rotation.
    fn at(translation: Vec4, scale: f32) -> Transform {
        [translation, IDENTITY, [scale; 4]]
    }

    #[test]
    fn without_parents_every_transform_is_put_on_the_one_of_the_same_index() {
        let mut e = exe();
        add_quaternions(&mut e);
        let a = pose(
            &mut e,
            &[
                at([1.0, 2.0, 3.0, 4.0], 2.0),
                at([10.0, 20.0, 30.0, 40.0], 3.0),
            ],
        );
        let b = pose(
            &mut e,
            &[at([5.0, 6.0, 7.0, 8.0], 4.0), at([1.0, 1.0, 1.0, 1.0], 2.0)],
        );
        let out = pose(&mut e, &[[[0.0; 4]; 3]; 2]);
        e.call(0x00ce_2d10, &args![a, b, 0u32, 2i32, out]);
        assert_eq!(
            get_transform(&e, out),
            [[6.0, 8.0, 10.0, 12.0], [0.0, 0.0, 0.0, 2.0], [8.0; 4]]
        );
        assert_eq!(
            get_transform(&e, out + 0x30),
            [[11.0, 21.0, 31.0, 41.0], [0.0, 0.0, 0.0, 2.0], [6.0; 4]]
        );
    }

    #[test]
    fn with_parents_a_transform_is_put_on_the_one_its_index_names() {
        let mut e = exe();
        add_quaternions(&mut e);
        let a = pose(
            &mut e,
            &[
                at([1.0, 2.0, 3.0, 4.0], 2.0),
                at([10.0, 20.0, 30.0, 40.0], 3.0),
            ],
        );
        let b = pose(
            &mut e,
            &[
                at([1.0, 1.0, 1.0, 1.0], 1.0),
                at([100.0; 4], 5.0),
                at([2.0, 3.0, 4.0, 5.0], 7.0),
            ],
        );
        // The second index is negative (as a 16-bit number): that transform is skipped.
        let parents = words16(&mut e, &[1, 0xffff, 0]);
        let out = pose(&mut e, &[[[0.0; 4]; 3]; 2]);
        e.call(0x00ce_2d10, &args![a, b, parents, 3i32, out]);
        assert_eq!(
            get_transform(&e, out + 0x30),
            [[11.0, 21.0, 31.0, 41.0], [0.0, 0.0, 0.0, 2.0], [3.0; 4]]
        );
        assert_eq!(
            get_transform(&e, out),
            [[3.0, 5.0, 7.0, 9.0], [0.0, 0.0, 0.0, 2.0], [14.0; 4]]
        );
    }

    #[test]
    fn with_parents_the_result_may_be_put_over_the_first_pose() {
        let build = |e: &mut Engine| {
            let a = pose(
                e,
                &[
                    at([1.0, 2.0, 3.0, 4.0], 2.0),
                    at([10.0, 20.0, 30.0, 40.0], 3.0),
                ],
            );
            let b = pose(
                e,
                &[at([5.0, 6.0, 7.0, 8.0], 4.0), at([1.0, 1.0, 1.0, 1.0], 2.0)],
            );
            let parents = words16(e, &[0, 1]);
            (a, b, parents)
        };
        let mut e = exe();
        add_quaternions(&mut e);
        let (a, b, parents) = build(&mut e);
        let out = pose(&mut e, &[[[0.0; 4]; 3]; 2]);
        e.call(0x00ce_2d10, &args![a, b, parents, 2i32, out]);
        let (a, b, parents) = build(&mut e);
        e.call(0x00ce_2d10, &args![a, b, parents, 2i32, a]);
        for k in 0..2 {
            assert_eq!(
                get_transform(&e, a + 0x30 * k),
                get_transform(&e, out + 0x30 * k)
            );
        }
    }

    #[test]
    fn a_count_that_is_not_above_zero_places_nothing() {
        let mut e = exe();
        add_quaternions(&mut e);
        let a = pose(&mut e, &[at([1.0; 4], 2.0)]);
        let b = pose(&mut e, &[at([5.0; 4], 4.0)]);
        let parents = words16(&mut e, &[0]);
        let out = pose(&mut e, &[[[7.0; 4]; 3]]);
        for (parents, count) in [(0, 0), (0, -3), (parents, 0), (parents, -1)] {
            e.call(0x00ce_2d10, &args![a, b, parents, count, out]);
        }
        assert_eq!(get_transform(&e, out), [[7.0; 4]; 3]);
    }

    #[test]
    fn the_thunk_calls_the_placing_routine_with_the_same_arguments() {
        let mut e = exe();
        let log = new_log();
        log_calls(&mut e, &log, PLACE_POSE);
        e.call(0x00ce_2f30, &args![1u32, 2u32, 3u32, 4i32, 5u32]);
        assert_eq!(*log.borrow(), vec![(PLACE_POSE, vec![1, 2, 3, 4, 5])]);
    }

    #[test]
    fn the_inverse_variant_builds_the_inverses_in_the_arena_and_gives_it_back() {
        let mut e = exe();
        let log = new_log();
        let (router, arena) = heap(&mut e, &log);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let cursors = Rc::new(RefCell::new(Vec::new()));
        let inverses = Rc::new(RefCell::new(Vec::new()));
        let (c, k, i) = (Rc::clone(&calls), Rc::clone(&cursors), Rc::clone(&inverses));
        e.register_double(PLACE_POSE, move |e, a| {
            c.borrow_mut().push(a.to_vec());
            k.borrow_mut().push(e.mem.u32(router));
            for index in 0..2 {
                i.borrow_mut().push(get_transform(e, a[1] + 0x30 * index));
            }
            Ret::default()
        });
        let b = pose(
            &mut e,
            &[
                [[1.0, 2.0, 3.0, 4.0], IDENTITY, [2.0, 4.0, 8.0, 3.0]],
                at([0.0; 4], 1.0),
            ],
        );
        e.call(0x00ce_2f40, &args![0x111u32, b, 0x222u32, 2i32, 0x333u32]);
        assert_eq!(*calls.borrow(), vec![vec![0x111, arena, 0x222, 2, 0x333]]);
        // Two transforms take 0x60 bytes, rounded up to 0x80.
        assert_eq!(*cursors.borrow(), vec![arena + 0x80]);
        assert_eq!(e.mem.u32(router), arena);
        let inverses = inverses.borrow();
        assert_eq!(inverses[0][0], [-1.0, -2.0, -3.0, -4.0]);
        assert_eq!(inverses[0][1], IDENTITY);
        assert!(close(inverses[0][2], [0.5, 0.25, 0.125, 0.0], 1.0e-5));
        assert_eq!(inverses[1][0], [0.0; 4]);
        assert_eq!(inverses[1][1], IDENTITY);
        assert!(close(inverses[1][2], [1.0, 1.0, 1.0, 0.0], 1.0e-5));
    }

    // ---- the mapping data ----

    #[test]
    fn the_constructor_sets_the_count_the_vtable_and_assigns_the_data() {
        let mut e = exe();
        let log = new_log();
        log_calls(&mut e, &log, DATA_CONSTRUCTOR);
        log_calls(&mut e, &log, ASSIGN_DATA);
        let this = e.mem.alloc(0x80);
        let source = e.mem.alloc(0x70);
        let got = e.call(0x00ce_4a20, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            *log.borrow(),
            vec![
                (DATA_CONSTRUCTOR, vec![this + 0x10]),
                (ASSIGN_DATA, vec![this + 0x10, source]),
            ]
        );
    }

    #[test]
    fn a_chain_mapping_is_copied_except_the_dwords_at_eight() {
        let mut e = exe();
        let to = e.mem.alloc(0x80);
        let from = e.mem.alloc(0x80);
        for k in 0..0x80 {
            e.mem.set_u8(to + k, 0xaa);
            e.mem.set_u8(from + k, k as u8);
        }
        let got = e.call(0x00ce_4a50, &args![to, from]).u32();
        assert_eq!(got, to);
        for k in 0..0x80 {
            let expected = match k {
                0..=7 | 0x10..=0x6f => k as u8,
                _ => 0xaa,
            };
            assert_eq!(e.mem.u8(to + k), expected, "byte {k:#x}");
        }
    }

    /// Mapping data that owns nothing.
    fn new_data(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x70);
        for field in [0x10, 0x1c, 0x28] {
            e.mem.set_u32(data + field, NOT_OWNED);
        }
        data
    }

    /// Gives the array at `array` storage for `capacity` records of `size` bytes (owned) with `count`
    /// of them holding bytes that count up from `seed`.
    fn fill_array(e: &mut Engine, array: u32, size: u32, count: u32, capacity: u32, seed: u8) {
        let data = e.mem.alloc(capacity.max(1) * size);
        for k in 0..count * size {
            e.mem.set_u8(data + k, seed.wrapping_add(k as u8));
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, count);
        e.mem.set_u32(array + 8, capacity);
    }

    /// Source data: two skeleton objects, two simple mappings, one chain mapping, three words, the
    /// three vectors, the byte and the dword.
    fn source_data(e: &mut Engine) -> (u32, u32, u32) {
        let data = new_data(e);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 4, second);
        fill_array(e, data + 8, 0x40, 2, 2, 1);
        fill_array(e, data + 0x14, 0x70, 1, 1, 50);
        let words = words16(e, &[7, 8, 9]);
        e.mem.set_u32(data + 0x20, words);
        e.mem.set_u32(data + 0x24, 3);
        e.mem.set_u32(data + 0x28, 3);
        store_vec(e, data + 0x30, [1.0, 2.0, 3.0, 4.0]);
        store_vec(e, data + 0x40, [5.0, 6.0, 7.0, 8.0]);
        store_vec(e, data + 0x50, [9.0, 10.0, 11.0, 12.0]);
        e.mem.set_u8(data + 0x60, 1);
        e.mem.set_u32(data + 0x64, 0x77);
        (data, first, second)
    }

    /// The bytes of the record that the assignment copies (`copied` the byte ranges).
    fn same_bytes(e: &Engine, from: u32, to: u32, copied: &[std::ops::Range<u32>]) {
        for range in copied {
            for k in range.clone() {
                assert_eq!(e.mem.u8(to + k), e.mem.u8(from + k), "byte {k:#x}");
            }
        }
    }

    #[test]
    fn assigning_to_empty_data_takes_the_skeletons_makes_room_and_copies_everything() {
        let mut e = exe();
        let log = new_log();
        let (router, _) = heap(&mut e, &log);
        let allocator = e.mem.u32(router + 0x10);
        array_doubles(&mut e, &log);
        log_calls(&mut e, &log, ADD_REFERENCE);
        log_calls(&mut e, &log, REMOVE_REFERENCE);
        let (source, first, second) = source_data(&mut e);
        let dest = new_data(&mut e);
        let (old_first, old_second) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        e.mem.set_u32(dest, old_first);
        e.mem.set_u32(dest + 4, old_second);
        let got = e.call(ASSIGN_DATA, &args![dest, source]).u32();
        assert_eq!(got, dest);
        assert_eq!(
            *log.borrow(),
            vec![
                (ADD_REFERENCE, vec![first]),
                (REMOVE_REFERENCE, vec![old_first]),
                (ADD_REFERENCE, vec![second]),
                (REMOVE_REFERENCE, vec![old_second]),
                (ARRAY_RESERVE, vec![allocator, dest + 8, 2, 0x40]),
                (ARRAY_RESERVE, vec![allocator, dest + 0x14, 1, 0x70]),
                (SIZED_ALLOCATE_DOUBLE, vec![allocator, 6]),
            ]
        );
        assert_eq!(e.mem.u32(dest), first);
        assert_eq!(e.mem.u32(dest + 4), second);
        assert_eq!(e.mem.u32(dest + 0xc), 2);
        for k in 0..2 {
            same_bytes(
                &e,
                e.mem.u32(source + 8) + 0x40 * k,
                e.mem.u32(dest + 8) + 0x40 * k,
                &[0..4, 0x10..0x40],
            );
        }
        assert_eq!(e.mem.u32(dest + 0x18), 1);
        same_bytes(
            &e,
            e.mem.u32(source + 0x14),
            e.mem.u32(dest + 0x14),
            &[0..8, 0x10..0x70],
        );
        // Six bytes rounded up to 16: a capacity of 8 words.
        assert_eq!(e.mem.u32(dest + 0x24), 3);
        assert_eq!(e.mem.u32(dest + 0x28), 8);
        let words = e.mem.u32(dest + 0x20);
        assert_eq!(
            (0..3).map(|k| e.mem.u16(words + 2 * k)).collect::<Vec<_>>(),
            vec![7, 8, 9]
        );
        assert_eq!(load_vec(&e, dest + 0x30), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, dest + 0x40), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vec(&e, dest + 0x50), [9.0, 10.0, 11.0, 12.0]);
        assert_eq!(e.mem.u8(dest + 0x60), 1);
        assert_eq!(e.mem.u32(dest + 0x64), 0x77);
    }

    #[test]
    fn assigning_into_room_that_is_big_enough_keeps_the_storage_and_resizes() {
        let mut e = exe();
        let log = new_log();
        let (router, _) = heap(&mut e, &log);
        let allocator = e.mem.u32(router + 0x10);
        array_doubles(&mut e, &log);
        log_calls(&mut e, &log, ADD_REFERENCE);
        log_calls(&mut e, &log, REMOVE_REFERENCE);
        let (source, _, _) = source_data(&mut e);
        // The source has no skeletons this time, and one simple mapping and two chain mappings.
        e.mem.set_u32(source, 0);
        e.mem.set_u32(source + 4, 0);
        e.mem.set_u32(source + 0xc, 1);
        fill_array(&mut e, source + 0x14, 0x70, 2, 2, 90);
        let dest = new_data(&mut e);
        fill_array(&mut e, dest + 8, 0x40, 3, 4, 200);
        fill_array(&mut e, dest + 0x14, 0x70, 0, 2, 220);
        let words = words16(&mut e, &[1, 2]);
        e.mem.set_u32(dest + 0x20, words);
        e.mem.set_u32(dest + 0x24, 2);
        e.mem.set_u32(dest + 0x28, 2);
        let simple_storage = e.mem.u32(dest + 8);
        let chain_storage = e.mem.u32(dest + 0x14);
        // Five words are needed; the storage of two words is owned, so it is freed first.
        e.mem.set_u32(source + 0x24, 5);
        let long = words16(&mut e, &[1, 2, 3, 4, 5]);
        e.mem.set_u32(source + 0x20, long);
        e.call(ASSIGN_DATA, &args![dest, source]);
        assert_eq!(
            *log.borrow(),
            vec![
                (FREE_DOUBLE, vec![allocator, words, 4]),
                (SIZED_ALLOCATE_DOUBLE, vec![allocator, 10]),
            ]
        );
        assert_eq!(e.mem.u32(dest + 8), simple_storage);
        assert_eq!(e.mem.u32(dest + 0xc), 1);
        assert_eq!(e.mem.u32(dest + 0x14), chain_storage);
        assert_eq!(e.mem.u32(dest + 0x18), 2);
        for k in 0..2 {
            same_bytes(
                &e,
                e.mem.u32(source + 0x14) + 0x70 * k,
                chain_storage + 0x70 * k,
                &[0..8, 0x10..0x70],
            );
        }
        same_bytes(
            &e,
            e.mem.u32(source + 8),
            simple_storage,
            &[0..4, 0x10..0x40],
        );
        assert_eq!(e.mem.u32(dest + 0x28), 8);
        let words = e.mem.u32(dest + 0x20);
        assert_eq!(
            (0..5).map(|k| e.mem.u16(words + 2 * k)).collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5]
        );
    }

    // ---- the mapper and its poses ----

    /// A mapper object with the two skeletons and empty arrays; the dword at `+0x74` is 0x1234.
    fn mapper(e: &mut Engine, skeleton_a: u32, skeleton_b: u32) -> u32 {
        let me = e.mem.alloc(0x80);
        e.mem.set_u32(me + 0x10, skeleton_a);
        e.mem.set_u32(me + 0x14, skeleton_b);
        for field in [0x20, 0x2c, 0x38] {
            e.mem.set_u32(me + field, NOT_OWNED);
        }
        e.mem.set_u32(me + 0x74, 0x1234);
        me
    }

    /// The simple mappings of a mapper: the two bones and the transform.
    fn put_simple(e: &mut Engine, me: u32, entries: &[(i16, i16, Transform)]) {
        let data = e.mem.alloc(0x40 * entries.len().max(1) as u32);
        for (k, (a, b, transform)) in entries.iter().enumerate() {
            let at = data + 0x40 * k as u32;
            e.mem.set_u16(at, *a as u16);
            e.mem.set_u16(at + 2, *b as u16);
            put_transform(e, at + 0x10, *transform);
        }
        e.mem.set_u32(me + 0x18, data);
        e.mem.set_u32(me + 0x1c, entries.len() as u32);
        e.mem.set_u32(me + 0x20, entries.len() as u32);
    }

    /// The chain mappings of a mapper: the four bones (first and last of the first skeleton, first
    /// and last of the second), the transform of the start and the one of the end.
    fn put_chains(e: &mut Engine, me: u32, entries: &[([u16; 4], Transform, Transform)]) {
        let data = e.mem.alloc(0x70 * entries.len().max(1) as u32);
        for (k, (bones, start, end)) in entries.iter().enumerate() {
            let at = data + 0x70 * k as u32;
            for (n, bone) in bones.iter().enumerate() {
                e.mem.set_u16(at + 2 * n as u32, *bone);
            }
            put_transform(e, at + 0x10, *start);
            put_transform(e, at + 0x40, *end);
        }
        e.mem.set_u32(me + 0x24, data);
        e.mem.set_u32(me + 0x28, entries.len() as u32);
        e.mem.set_u32(me + 0x2c, entries.len() as u32);
    }

    /// The profile record stream: answers where its records start and the stream itself.
    fn profile_stream(e: &mut Engine) -> (u32, u32) {
        let buffer = e.mem.alloc(0x400);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, buffer);
        e.mem.set_u32(stream + 0xc, buffer + 0x400);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (buffer, stream)
    }

    /// The labels of the records: the first record is 16 bytes, the others 12.
    fn profile_labels(e: &Engine, buffer: u32, stream: u32) -> Vec<u32> {
        let end = e.mem.u32(stream + 4);
        let mut labels = Vec::new();
        let mut at = buffer;
        while at < end {
            labels.push(e.mem.u32(at));
            at += if labels.len() == 1 { 0x10 } else { 0xc };
        }
        labels
    }

    /// `getBoneChain` as the exe does it: the bones from the last up (while their index is above
    /// zero and not the first) are put in front of the list, then the first; the flag says the
    /// last index reached was not negative. Logs the skeleton, the two bones and the list's data
    /// and capacity word on the way in.
    fn chain_double(e: &mut Engine, log: &Log) {
        let l = Rc::clone(log);
        e.register_double(GET_BONE_CHAIN, move |e, a| {
            let (flag, skeleton, first, last, array) = (a[0], a[1], a[2] as u16, a[3] as u16, a[4]);
            l.borrow_mut().push((
                GET_BONE_CHAIN,
                vec![skeleton, a[2], a[3], e.mem.u32(array), e.mem.u32(array + 8)],
            ));
            let parents = e.mem.u32(skeleton + 0xc);
            let mut chain = Vec::new();
            let mut current = last;
            while (current as i16) > 0 && current != first {
                chain.insert(0, current);
                current = e.mem.u16(parents + 2 * u32::from(current));
            }
            chain.insert(0, first);
            let count = chain.len() as u32;
            if e.mem.u32(array + 8) & CAPACITY_MASK < count {
                let data = e.mem.alloc(2 * count);
                e.mem.set_u32(array, data);
                e.mem.set_u32(array + 8, count);
            }
            let data = e.mem.u32(array);
            for (k, bone) in chain.iter().enumerate() {
                e.mem.set_u16(data + 2 * k as u32, *bone);
            }
            e.mem.set_u32(array + 4, count);
            e.mem.set_u8(flag, u8::from((current as i16) >= 0));
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
    }

    /// The 16-bit words of an `hkArray`.
    fn array_words(e: &Engine, array: u32) -> Vec<u16> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|k| e.mem.u16(data + 2 * k))
            .collect()
    }

    fn empty_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 8, NOT_OWNED);
        array
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    // ---- initializeBindingMaps ----

    #[test]
    fn the_binding_maps_follow_the_simple_mappings_and_the_chains_inside_the_list() {
        let mut e = exe();
        let log = new_log();
        let (router, _) = heap(&mut e, &log);
        let allocator = e.mem.u32(router + 0x10);
        array_doubles(&mut e, &log);
        chain_double(&mut e, &log);
        let a = skeleton(&mut e, &[0xffff, 0, 1, 2], &[0; 4]);
        let b = skeleton(&mut e, &[0xffff, 0, 1], &[0; 3]);
        let me = mapper(&mut e, a, b);
        put_simple(
            &mut e,
            me,
            &[
                (1, 1, at([0.0; 4], 1.0)),
                (3, 2, at([0.0; 4], 1.0)),
                (0, 0, at([0.0; 4], 1.0)),
            ],
        );
        put_chains(
            &mut e,
            me,
            &[
                ([0, 2, 0, 2], at([0.0; 4], 1.0), at([0.0; 4], 1.0)),
                ([1, 3, 1, 2], at([0.0; 4], 1.0), at([0.0; 4], 1.0)),
            ],
        );
        // The list of the bones of the first skeleton: 0, 1 and 2 (the count at +0x14).
        let list = e.mem.alloc(0x20);
        let bones = words16(&mut e, &[0, 1, 2]);
        e.mem.set_u32(list + 0x10, bones);
        e.mem.set_u32(list + 0x14, 3);
        let (slots_a, slots_b, bones_b) = (
            empty_array(&mut e),
            empty_array(&mut e),
            empty_array(&mut e),
        );
        e.call(0x00ce_35f0, &args![me, list, slots_a, slots_b, bones_b]);
        assert_eq!(array_words(&e, slots_a), vec![0, 1, 2, 0xffff]);
        // The mappings of bones 1 and 0, then the second chain's bones 0, 1 and 2.
        assert_eq!(array_words(&e, bones_b), vec![1, 0, 0, 1, 2]);
        // The positions: the last one in the list wins.
        assert_eq!(array_words(&e, slots_b), vec![2, 3, 4]);
        assert_eq!(
            calls_to(&log, ARRAY_RESERVE),
            vec![
                vec![allocator, slots_a, 4, 2],
                vec![allocator, slots_b, 3, 2]
            ]
        );
        // The list of bones is full at 0 and at 4.
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 2);
        let not_owned = NOT_OWNED;
        assert_eq!(
            calls_to(&log, GET_BONE_CHAIN),
            vec![
                vec![a, 0, 2, 0, not_owned],
                vec![b, 0, 2, 0, not_owned],
                vec![a, 1, 3, 0, not_owned],
                vec![b, 1, 2, 0, not_owned],
            ]
        );
        // The chain lists are freed second one first: three words (6 bytes), three, two, three.
        let freed: Vec<u32> = calls_to(&log, FREE_DOUBLE)
            .iter()
            .map(|words| words[2])
            .collect();
        assert_eq!(freed, vec![6, 6, 4, 6]);
    }

    #[test]
    fn a_chain_that_does_not_reach_its_first_bone_adds_nothing() {
        let mut e = exe();
        let log = new_log();
        heap(&mut e, &log);
        array_doubles(&mut e, &log);
        chain_double(&mut e, &log);
        let a = skeleton(&mut e, &[0xffff, 0xffff], &[0; 2]);
        let b = skeleton(&mut e, &[0xffff, 0], &[0; 2]);
        let me = mapper(&mut e, a, b);
        put_chains(
            &mut e,
            me,
            &[([0, 1, 0, 1], at([0.0; 4], 1.0), at([0.0; 4], 1.0))],
        );
        let list = e.mem.alloc(0x20);
        let bones = words16(&mut e, &[0, 1]);
        e.mem.set_u32(list + 0x10, bones);
        e.mem.set_u32(list + 0x14, 2);
        let (slots_a, slots_b, bones_b) = (
            empty_array(&mut e),
            empty_array(&mut e),
            empty_array(&mut e),
        );
        e.call(0x00ce_35f0, &args![me, list, slots_a, slots_b, bones_b]);
        assert_eq!(array_words(&e, slots_a), vec![0, 1]);
        assert!(array_words(&e, bones_b).is_empty());
        assert_eq!(array_words(&e, slots_b), vec![0xffff, 0xffff]);
        // Only the first skeleton was asked; its list (two words) was freed.
        assert_eq!(calls_to(&log, GET_BONE_CHAIN).len(), 1);
        let freed: Vec<u32> = calls_to(&log, FREE_DOUBLE)
            .iter()
            .map(|words| words[2])
            .collect();
        assert_eq!(freed, vec![4]);
    }

    // ---- mapPoseLocalSpace ----

    #[test]
    fn local_space_mapping_places_every_simple_mapping() {
        let mut e = exe();
        let log = new_log();
        let (router, arena) = heap(&mut e, &log);
        let (buffer, stream) = profile_stream(&mut e);
        log_calls(&mut e, &log, PLACE_BONE);
        let a = skeleton(&mut e, &[0xffff, 0], &[0, 0]);
        let b = skeleton(&mut e, &[0xffff, 0], &[0, 0]);
        let me = mapper(&mut e, a, b);
        put_simple(
            &mut e,
            me,
            &[(0, 1, at([1.0; 4], 1.0)), (1, 0, at([2.0; 4], 1.0))],
        );
        let pose_a = pose(&mut e, &[at([0.0; 4], 1.0); 2]);
        let pose_b = pose(&mut e, &[at([0.0; 4], 1.0); 2]);
        e.call(0x00ce_3090, &args![me, pose_a, pose_b, 0u32, 0u32, 0u8]);
        let entries = e.mem.u32(me + 0x18);
        assert_eq!(
            *log.borrow(),
            vec![
                (
                    PLACE_BONE,
                    vec![0x1234, pose_b + 0x30, entries + 0x10, pose_a]
                ),
                (
                    PLACE_BONE,
                    vec![0x1234, pose_b, entries + 0x50, pose_a + 0x30]
                ),
            ]
        );
        assert_eq!(e.mem.u32(router), arena);
        assert_eq!(
            profile_labels(&e, buffer, stream),
            vec![
                LABEL_MAP_POSE_LOCAL_SPACE,
                LABEL_SIMPLE_MAPPINGS,
                LABEL_CHAIN_MAPPINGS,
                LABEL_END
            ]
        );
        assert_eq!(e.mem.u32(buffer + 0xc), LABEL_INIT);
    }

    #[test]
    fn local_space_mapping_goes_through_the_maps_and_converts_around_the_mappings() {
        let mut e = exe();
        let log = new_log();
        profile_stream(&mut e);
        let (router, arena) = heap(&mut e, &log);
        let cursors = Rc::new(RefCell::new(Vec::new()));
        let c = Rc::clone(&cursors);
        let l = Rc::clone(&log);
        e.register_double(PLACE_POSE_THUNK, move |e, a| {
            l.borrow_mut().push((PLACE_POSE_THUNK, a.to_vec()));
            c.borrow_mut().push(e.mem.u32(router));
            Ret::default()
        });
        log_calls(&mut e, &log, PLACE_POSE_INVERSE);
        log_calls(&mut e, &log, PLACE_BONE);
        let a = skeleton(&mut e, &[0xffff, 0], &[0, 0]);
        let b = skeleton(&mut e, &[0xffff, 0], &[0, 0]);
        let reference_a = pose(&mut e, &[at([0.0; 4], 1.0); 2]);
        let reference_b = pose(&mut e, &[at([0.0; 4], 1.0); 2]);
        e.mem.set_u32(a + 0x24, reference_a);
        e.mem.set_u32(b + 0x24, reference_b);
        let me = mapper(&mut e, a, b);
        put_simple(
            &mut e,
            me,
            &[(0, 1, at([1.0; 4], 1.0)), (1, 0, at([2.0; 4], 1.0))],
        );
        let pose_a = pose(&mut e, &[at([0.0; 4], 1.0); 2]);
        let pose_b = pose(&mut e, &[at([0.0; 4], 1.0); 2]);
        // The first bone of the first skeleton is at 1 in the poses, the second has none.
        let map_a = words16(&mut e, &[1, 0xffff]);
        let map_b = words16(&mut e, &[1, 0]);
        e.call(0x00ce_3090, &args![me, pose_a, pose_b, map_a, map_b, 1u8]);
        let entries = e.mem.u32(me + 0x18);
        assert_eq!(
            *log.borrow(),
            vec![
                (PLACE_POSE_THUNK, vec![pose_a, reference_a, map_a, 2, arena]),
                (
                    PLACE_BONE,
                    vec![0x1234, pose_b, entries + 0x10, arena + 0x30]
                ),
                (
                    PLACE_POSE_INVERSE,
                    vec![pose_b, reference_b, map_b, 2, pose_b]
                ),
            ]
        );
        // Two transforms take 0x60 bytes, rounded up to 0x80, and are given back in the end.
        assert_eq!(*cursors.borrow(), vec![arena + 0x80]);
        assert_eq!(e.mem.u32(router), arena);
    }

    /// The doubles of the chain tests: the pair normaliser stays real; the quaternion routines
    /// log what they get.
    struct ChainLogs {
        between: Rc<RefCell<Vec<(Vec4, Vec4)>>>,
        product: Rc<RefCell<Vec<(Vec4, Vec4)>>>,
    }

    fn chain_logs(e: &mut Engine, log: &Log) -> ChainLogs {
        let between = Rc::new(RefCell::new(Vec::new()));
        let b = Rc::clone(&between);
        e.register_double(QUATERNION_BETWEEN, move |e, a| {
            b.borrow_mut().push((load_vec(e, a[1]), load_vec(e, a[2])));
            store_vec(e, a[0], IDENTITY);
            Ret::default()
        });
        let product = Rc::new(RefCell::new(Vec::new()));
        let p = Rc::clone(&product);
        e.register_double(QUATERNION_PRODUCT, move |e, a| {
            p.borrow_mut().push((load_vec(e, a[1]), load_vec(e, a[2])));
            store_vec(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        log_calls(e, log, QUATERNION_MULTIPLY_IN_PLACE);
        log_calls(e, log, QUATERNION_SET_MUL);
        log_calls(e, log, PLACE_BONE);
        ChainLogs { between, product }
    }

    /// A chain from bone 0 to bone 2 of both skeletons; the first skeleton's pose has translations
    /// 4 up at bone 1 and 3 across at bone 2 (length 7), the second's 1 up and 1 across.
    fn chain_rig(e: &mut Engine) -> (u32, u32, u32) {
        let a = skeleton(e, &[0xffff, 0, 1], &[0; 3]);
        let b = skeleton(e, &[0xffff, 0, 1], &[0; 3]);
        let me = mapper(e, a, b);
        let start = at([1.0, 0.0, 0.0, 0.0], 1.0);
        let end = [[0.5; 4], IDENTITY, [1.0; 4]];
        put_chains(e, me, &[([0, 2, 0, 2], start, end)]);
        let pose_a = pose(
            e,
            &[
                at([0.0; 4], 1.0),
                at([0.0, 4.0, 0.0, 0.0], 1.0),
                at([3.0, 0.0, 0.0, 0.0], 1.0),
            ],
        );
        let pose_b = pose(
            e,
            &[
                at([0.0; 4], 1.0),
                at([0.0, 1.0, 0.0, 0.0], 1.0),
                at([1.0, 0.0, 0.0, 0.0], 1.0),
            ],
        );
        (me, pose_a, pose_b)
    }

    #[test]
    fn a_chain_scales_the_second_translations_by_the_length_of_the_first_and_turns_the_start() {
        let mut e = exe();
        let log = new_log();
        profile_stream(&mut e);
        let logs = chain_logs(&mut e, &log);
        let (me, pose_a, pose_b) = chain_rig(&mut e);
        e.call(0x00ce_3090, &args![me, pose_a, pose_b, 0u32, 0u32, 0u8]);
        // The second chain's translations are scaled by 7 (the length) times the mapping's scale.
        assert_eq!(load_vec(&e, pose_b + 0x30), [0.0, 7.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, pose_b + 0x60), [7.0, 0.0, 0.0, 0.0]);
        // The walked vectors are (7, 7) in the second chain and (3, 4) in the first, normalised.
        let between = logs.between.borrow();
        assert_eq!(between.len(), 1);
        assert!(close(
            between[0].0,
            [FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0, 0.0],
            1.0e-3
        ));
        assert!(close(between[0].1, [0.6, 0.8, 0.0, 0.0], 1.0e-3));
        // The product is taken with the mapping's end translation.
        assert_eq!(logs.product.borrow()[0].1, [0.5; 4]);
        let entry = e.mem.u32(me + 0x24);
        let in_place = calls_to(&log, QUATERNION_MULTIPLY_IN_PLACE);
        assert_eq!(in_place.len(), 1);
        assert_eq!(in_place[0][0], pose_b + 0x10);
        assert_eq!(
            calls_to(&log, PLACE_BONE),
            vec![vec![0x1234, pose_b, entry + 0x10, pose_a]]
        );
        assert_eq!(
            calls_to(&log, QUATERNION_SET_MUL),
            vec![vec![
                pose_b + 0x60 + 0x10,
                entry + 0x50,
                pose_a + 0x60 + 0x10
            ]]
        );
    }

    #[test]
    fn a_partial_mapping_skips_a_chain_with_a_bone_that_has_no_place() {
        for first in [[0xffffu16, 1, 2], [0, 0xffff, 2]] {
            let mut e = exe();
            let log = new_log();
            profile_stream(&mut e);
            chain_logs(&mut e, &log);
            let (me, pose_a, pose_b) = chain_rig(&mut e);
            let map_a = words16(&mut e, &first);
            let map_b = words16(&mut e, &[0, 1, 2]);
            e.call(0x00ce_3090, &args![me, pose_a, pose_b, map_a, map_b, 0u8]);
            assert!(log.borrow().is_empty(), "{first:?}");
            assert_eq!(load_vec(&e, pose_b + 0x30), [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, pose_b + 0x60), [1.0, 0.0, 0.0, 0.0]);
        }
    }

    // ---- mapPose ----

    #[test]
    fn model_space_mapping_places_the_simple_mappings_copies_unmapped_roots_and_enforces_locks() {
        for mode in [0u32, 1, 2, 3] {
            let mut e = exe();
            let log = new_log();
            let (buffer, stream) = profile_stream(&mut e);
            log_calls(&mut e, &log, QUATERNION_SET_MUL);
            log_calls(&mut e, &log, LOCK_TRANSLATIONS);
            log_calls(&mut e, &log, LOCK_TRANSLATIONS_FROM_REFERENCE);
            let a = skeleton(&mut e, &[0xffff], &[0]);
            let b = skeleton(&mut e, &[0xffff, 0, 0xffff], &[0; 3]);
            let me = mapper(&mut e, a, b);
            put_simple(&mut e, me, &[(0, 1, at([1.0, 0.0, 0.0, 0.0], 3.0))]);
            let unmapped = words16(&mut e, &[2]);
            e.mem.set_u32(me + 0x30, unmapped);
            e.mem.set_u32(me + 0x34, 1);
            e.mem.set_u8(me + 0x70, 1);
            let pose_a = pose(&mut e, &[at([10.0, 0.0, 0.0, 0.0], 2.0)]);
            let local_pose_b = pose(
                &mut e,
                &[
                    at([0.0; 4], 1.0),
                    at([0.0; 4], 1.0),
                    at([7.0, 8.0, 9.0, 0.0], 5.0),
                ],
            );
            let pose_b = pose(&mut e, &[at([0.0; 4], 1.0); 3]);
            e.call(0x00ce_3a00, &args![me, pose_a, local_pose_b, pose_b, mode]);
            let placed = get_transform(&e, pose_b + 0x30);
            assert!(close(placed[0], [11.0, 0.0, 0.0, 0.0], 1.0e-4));
            assert!(close(placed[2], [6.0; 4], 1.0e-4));
            assert_eq!(
                get_transform(&e, pose_b + 0x60),
                at([7.0, 8.0, 9.0, 0.0], 5.0)
            );
            let entry = e.mem.u32(me + 0x18);
            assert_eq!(
                calls_to(&log, QUATERNION_SET_MUL),
                vec![vec![
                    pose_b + 0x30 + 0x10,
                    pose_a + 0x10,
                    entry + 0x10 + 0x10
                ]]
            );
            assert_eq!(
                calls_to(&log, LOCK_TRANSLATIONS).len(),
                usize::from(mode == 1)
            );
            assert_eq!(
                calls_to(&log, LOCK_TRANSLATIONS_FROM_REFERENCE).len(),
                usize::from(mode == 2)
            );
            if mode == 1 {
                assert_eq!(calls_to(&log, LOCK_TRANSLATIONS), vec![vec![b, pose_b]]);
            }
            if mode == 2 {
                assert_eq!(
                    calls_to(&log, LOCK_TRANSLATIONS_FROM_REFERENCE),
                    vec![vec![b, local_pose_b, pose_b]]
                );
            }
            assert_eq!(
                profile_labels(&e, buffer, stream),
                vec![
                    LABEL_MAP_POSE,
                    LABEL_SIMPLE_MAPPINGS,
                    LABEL_CHAIN_MAPPINGS,
                    LABEL_UNMAPPED_BONES,
                    LABEL_ENFORCE_CONSTRAINTS,
                    LABEL_END
                ]
            );
        }
    }
}
