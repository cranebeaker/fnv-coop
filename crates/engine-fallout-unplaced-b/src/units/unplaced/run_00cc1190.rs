//! `(unplaced)/run_00cc1190` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cc1710 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00cc1520`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00cc1710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_1520;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00cc1280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x0126_8590.
pub fn return_constant_00cc1280(_e: &mut Engine) -> u32 {
    0x0126_8590
}

/// `00d08260` (thiscall on `first + 0x10`: the address of an `hkArray` it fills).
const COLLECT_ENTRIES: u32 = 0x00d0_8260;
/// The thread's memory router (`00c85750`, unit `hkmemoryrouter.cpp`; no arguments).
const MEMORY_ROUTER_INSTANCE: u32 = 0x00c8_5750;
/// The object allocator `00aa13e0` (cdecl: size).
const OBJECT_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObject`'s constructor body (`00a5d3a0`, unit `niobject.cpp`; thiscall).
const NI_OBJECT_CTOR: u32 = 0x00a5_d3a0;
/// `NiObject`'s destructor body (`00a5d3d0`, unit `niobject.cpp`; thiscall).
const NI_OBJECT_DTOR: u32 = 0x00a5_d3d0;
/// `NiObject::RegisterStreamables` (Xbox PDB, `00a5d2f0`).
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// The base `SaveBinary` body `00a89af0` (unit NiMain; thiscall: the stream).
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// `NiStream::SaveFixedString` (Xbox PDB, `00a641f0`; thiscall: the string's address).
const STREAM_SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// The transform writer `00cf09a0` (unit `bhkstream.obj`; cdecl: stream, transform).
const STREAM_SAVE_TRANSFORM: u32 = 0x00cf_09a0;
/// The CRT's `eh vector destructor iterator` (`00ec5fce`, stdcall: array, element size,
/// count, destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The `NiFixedString` element destructor `00ec5fce` runs over the bone names (`004381b0`).
const FIXED_STRING_DTOR: u32 = 0x0043_81b0;
/// The `MemoryManager` singleton and its deallocation (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `bhkPoseArray`'s vtable (`010c93fc`).
const POSE_ARRAY_VTABLE: u32 = 0x010c_93fc;
/// The word of the thread's TLS block (`e.tls() + 0x2b4`) the destructor sets to 0x10 while
/// it frees, and restores.
const TLS_CONTEXT: u32 = 0x2b4;

/// `bhkPoseArray` (Xbox PDB, 0x18 bytes): `uiNumBones` +8, `BoneNameA` +0xc, `uiNumPose`
/// +0x10, `PoseA` +0x14 (offsets as the PC code uses them).
const POSE_ARRAY_NUM_BONES: u32 = 0x08;
const POSE_ARRAY_BONE_NAMES: u32 = 0x0c;
const POSE_ARRAY_NUM_POSES: u32 = 0x10;
const POSE_ARRAY_POSES: u32 = 0x14;

/// The inlined `strcmp(a, b) == 0` of `00cc1360`/`00cc1420`: bytes compared until one
/// differs or both end.
fn c_strings_equal(e: &Engine, a: u32, b: u32) -> bool {
    let mut offset = 0u32;
    loop {
        let byte = e.mem.u8(a.wrapping_add(offset));
        if byte != e.mem.u8(b.wrapping_add(offset)) {
            return false;
        }
        if byte == 0 {
            return true;
        }
        offset += 1;
    }
}

/// The index after the first of the pose array's bone names equal to `name` (the inner
/// search of `00cc1360`/`00cc1420`: it stops on the pass after the match), or `None`.
fn find_bone(e: &Engine, this: u32, name: u32) -> Option<u32> {
    let count = e.mem.u32(this + POSE_ARRAY_NUM_BONES);
    let mut found = false;
    let mut index = 0u32;
    while index < count {
        if found {
            break;
        }
        let names = e.mem.u32(this + POSE_ARRAY_BONE_NAMES);
        let bone = e.mem.u32(names.wrapping_add(index.wrapping_mul(4)));
        if c_strings_equal(e, name, bone) {
            found = true;
        }
        index += 1;
    }
    found.then_some(index)
}

// Translated from 00cc1190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (nothing references it; cdecl, two words). Fills a local
/// `hkArray` (data, size, capacity `0x80000000`; 8-byte elements) through `00d08260` on
/// `first + 0x10`, then for each element (the bound re-read each pass) takes the entry its
/// first word points at and the function at `[second] + entry[1] * 0x50 + 0x16c8`; when
/// that is set, calls it (cdecl, 7 words) with the entry, `entry + 0x30` for an entry type
/// byte of 4 (else `entry + 0x20`), the entry's two objects (+0x10, +0x14), `second`, the
/// entry's word at +8, and a word read from the first object's sub-object (at its signed
/// byte +0x10 offset; when that sub-object's byte +0xe8 is 5, from the second object's the
/// same way) at +0xcc. Last the local array is freed through the thread's memory router's
/// allocator (vtable +0x10, `(capacity & 0x3fffffff) * 8` bytes) unless its capacity word
/// is negative.
pub fn fn_00cc1190(e: &mut Engine, first: Ptr, second: Ptr) {
    let second = second.addr();
    e.with_stack(12, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.call(COLLECT_ENTRIES, &args![first.addr() + 0x10, array]);
        let mut index: i32 = 0;
        if e.mem.i32(array + 4) > 0 {
            loop {
                let data = e.mem.u32(array);
                let entry = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(8)));
                let kind = u32::from(e.mem.u8(entry + 1));
                let table = e.mem.u32(second);
                let function = e
                    .mem
                    .u32(table.wrapping_add(kind * 0x50).wrapping_add(0x16c8));
                if function != 0 {
                    let object_a = e.mem.u32(entry + 0x10);
                    let sub_a = object_a.wrapping_add(e.mem.i8(object_a + 0x10) as u32);
                    let object_b = e.mem.u32(entry + 0x14);
                    let word = if e.mem.u8(sub_a + 0xe8) == 5 {
                        let sub_b = object_b.wrapping_add(e.mem.i8(object_b + 0x10) as u32);
                        e.mem.u32(sub_b + 0xcc)
                    } else {
                        e.mem.u32(sub_a + 0xcc)
                    };
                    let target = if e.mem.u8(entry) == 4 {
                        entry + 0x30
                    } else {
                        entry + 0x20
                    };
                    let extra = e.mem.u32(entry + 8);
                    e.call(
                        function,
                        &args![entry, target, object_a, object_b, second, extra, word],
                    );
                }
                index += 1;
                if index >= e.mem.i32(array + 4) {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER_INSTANCE, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.i32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if capacity >= 0 {
            let bytes = (capacity as u32 & 0x3fff_ffff).wrapping_mul(8);
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00cc1290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1290 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f33a7f, state variable) is not translated
/// Unnamed in the engine map: the `bhkPoseArray` create function: 0x18 bytes from
/// `00aa13e0`, `NiObject`'s constructor (`00a5d3a0`), the class's vtable and its four
/// fields cleared; null when the allocation failed.
pub fn bhk_pose_array_create(e: &mut Engine) -> Ptr {
    let block = e.call(OBJECT_ALLOCATE, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(NI_OBJECT_CTOR, &args![block]);
    e.mem.set_u32(block, POSE_ARRAY_VTABLE);
    for field in [0x08u32, 0x0c, 0x10, 0x14] {
        e.mem.set_u32(block + field, 0);
    }
    Ptr::new(block)
}

// Translated from 00cc1310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPoseArray::RegisterStreamables` (unnamed in the engine map): `NiObject`'s part;
/// when it answers true, each bone name (`BoneNameA[i]`, by address; count and array
/// re-read) goes through the stream's vtable slot +0x20. Passes on the base's byte.
pub fn bhk_pose_array_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> u8 {
    let this = this.addr();
    let registered = e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .u8();
    if registered != 0 {
        let mut index = 0u32;
        while index < e.mem.u32(this + POSE_ARRAY_NUM_BONES) {
            let names = e.mem.u32(this + POSE_ARRAY_BONE_NAMES);
            e.vcall(
                stream.addr(),
                0x20,
                &args![names.wrapping_add(index.wrapping_mul(4))],
            );
            index += 1;
        }
    }
    registered
}

// Translated from 00cc1360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called from `InitPoseMatching`): whether every bone of
/// `skeleton` (its bones at +0x18, 8 bytes each with the name pointer's bit 0 masked off,
/// count at +0x1c read once) has a name equal (inlined `strcmp`) to one of the pose array's
/// bone names. True for a skeleton without bones; the search stops at the first bone with
/// no match.
pub fn bhk_pose_array_matches_skeleton(e: &mut Engine, this: Ptr, skeleton: Ptr) -> bool {
    let count = e.mem.i32(skeleton.addr() + 0x1c);
    let mut matches = true;
    let mut index: i32 = 0;
    if count > 0 {
        loop {
            if !matches {
                break;
            }
            let bones = e.mem.u32(skeleton.addr() + 0x18);
            let name = e
                .mem
                .u32(bones.wrapping_add((index as u32).wrapping_mul(8)))
                & 0xffff_fffe;
            if find_bone(e, this.addr(), name).is_none() {
                matches = false;
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    matches
}

// Translated from 00cc1420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (called from `DoPoseMatching`): for each bone of `skeleton`
/// (count at +0x1c read once, names as in `00cc1360`) whose name is one of the pose array's
/// bone names, copies that bone's 0x30-byte transform from pose `pose` (`PoseA[pose]`, its
/// transforms at +4) into the 0x30-byte slot of `output` with the skeleton bone's index;
/// slots of unmatched bones are left alone.
pub fn bhk_pose_array_copy_pose(e: &mut Engine, this: Ptr, output: Ptr, skeleton: Ptr, pose: u32) {
    let this = this.addr();
    let count = e.mem.i32(skeleton.addr() + 0x1c);
    if count <= 0 {
        return;
    }
    let mut slot = output.addr();
    let mut index: i32 = 0;
    loop {
        let bones = e.mem.u32(skeleton.addr() + 0x18);
        let name = e
            .mem
            .u32(bones.wrapping_add((index as u32).wrapping_mul(8)))
            & 0xffff_fffe;
        if let Some(after) = find_bone(e, this, name) {
            let poses = e.mem.u32(this + POSE_ARRAY_POSES);
            let entry = e.mem.u32(poses.wrapping_add(pose.wrapping_mul(4)));
            let transforms = e.mem.u32(entry + 4);
            let source = transforms
                .wrapping_add(after.wrapping_mul(0x30))
                .wrapping_sub(0x30);
            let bytes = e.mem.bytes(source, 0x30);
            e.mem.write(slot, &bytes);
        }
        slot = slot.wrapping_add(0x30);
        index += 1;
        if index >= count {
            break;
        }
    }
}

// Translated from 00cc1520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1520 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f33ab0, state variable) is not translated
/// The `bhkPoseArray` destructor (unnamed in the engine map): the class's vtable; with the
/// thread's TLS word +0x2b4 set to 0x10 meanwhile (restored after), the bone names
/// (`BoneNameA`, a counted `new[]` array: count 4 bytes before) destroyed through the CRT's
/// vector iterator with `004381b0` and their block freed, then each non-null pose (count
/// and array re-read) and the pose array freed through the memory manager; last
/// `NiObject`'s destructor (`00a5d3d0`).
pub fn bhk_pose_array_dtor(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, POSE_ARRAY_VTABLE);
    let context = e.tls() + TLS_CONTEXT;
    let saved = e.mem.u32(context);
    e.mem.set_u32(context, 0x10);
    let names = e.mem.u32(this + POSE_ARRAY_BONE_NAMES);
    if names != 0 {
        let count = e.mem.u32(names.wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![names, 4u32, count, FIXED_STRING_DTOR],
        );
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, names.wrapping_sub(4)]);
    }
    if e.mem.u32(this + POSE_ARRAY_POSES) != 0 {
        let mut index = 0u32;
        while index < e.mem.u32(this + POSE_ARRAY_NUM_POSES) {
            let poses = e.mem.u32(this + POSE_ARRAY_POSES);
            let pose = e.mem.u32(poses.wrapping_add(index.wrapping_mul(4)));
            if pose != 0 {
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, pose]);
            }
            index += 1;
        }
        let poses = e.mem.u32(this + POSE_ARRAY_POSES);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, poses]);
    }
    e.mem.set_u32(context, saved);
    e.call(NI_OBJECT_DTOR, &args![this]);
}

/// One write of `00cc1610`: `word` (4 bytes, one component) through the write function at
/// +0xc of the stream's binary stream (+0x250; cdecl: binary stream, source, size, address
/// of a local holding 4, 1), its answer added to the binary stream's +4.
fn write_word(e: &mut Engine, stream: u32, source: u32, sizes: u32) {
    let binary = e.mem.u32(stream + 0x250);
    let write = e.mem.u32(binary + 0xc);
    e.mem.set_u32(sizes, 4);
    let written = e
        .call(write, &args![binary, source, 4u32, sizes, 1u32])
        .u32();
    let total = e.mem.u32(binary + 4);
    e.mem.set_u32(binary + 4, total.wrapping_add(written));
}

// Translated from 00cc1610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPoseArray::SaveBinary` (unnamed in the engine map): the base part (`00a89af0`),
/// `uiNumBones`, each bone name (`NiStream::SaveFixedString`, by address), `uiNumPose`,
/// then per pose its transform count (the pose's first word) and each 0x30-byte transform
/// (`00cf09a0`); counts written through the binary stream's write function (see
/// `write_word`), every bound re-read each pass.
pub fn bhk_pose_array_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let this = this.addr();
    let stream = stream.addr();
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, sizes| {
        let sizes = sizes.addr();
        write_word(e, stream, this + POSE_ARRAY_NUM_BONES, sizes);
        let mut index = 0u32;
        while index < e.mem.u32(this + POSE_ARRAY_NUM_BONES) {
            let names = e.mem.u32(this + POSE_ARRAY_BONE_NAMES);
            e.call(
                STREAM_SAVE_FIXED_STRING,
                &args![stream, names.wrapping_add(index.wrapping_mul(4))],
            );
            index += 1;
        }
        write_word(e, stream, this + POSE_ARRAY_NUM_POSES, sizes);
        let mut pose_index = 0u32;
        while pose_index < e.mem.u32(this + POSE_ARRAY_NUM_POSES) {
            let poses = e.mem.u32(this + POSE_ARRAY_POSES);
            let pose = e.mem.u32(poses.wrapping_add(pose_index.wrapping_mul(4)));
            write_word(e, stream, pose, sizes);
            let mut transform = 0u32;
            while transform < e.mem.u32(pose) {
                let transforms = e.mem.u32(pose + 4);
                e.call(
                    STREAM_SAVE_TRANSFORM,
                    &args![
                        stream,
                        transforms.wrapping_add(transform.wrapping_mul(0x30))
                    ],
                );
                transform += 1;
            }
            pose_index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc1280, return_constant_00cc1280() -> u32),
        entry!(0x00cc1710, fn_00cc1710(Ptr, u32) -> Ptr),
        entry!(0x00cc_1190, fn_00cc1190(Ptr, Ptr)),
        entry!(0x00cc_1290, bhk_pose_array_create() -> Ptr),
        entry!(0x00cc_1310, bhk_pose_array_register_streamables(Ptr, Ptr) -> u8),
        entry!(0x00cc_1360, bhk_pose_array_matches_skeleton(Ptr, Ptr) -> bool),
        entry!(0x00cc_1420, bhk_pose_array_copy_pose(Ptr, Ptr, Ptr, u32)),
        entry!(0x00cc_1520, bhk_pose_array_dtor(Ptr)),
        entry!(0x00cc_1610, bhk_pose_array_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(e: &Engine, addr: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .cloned()
            .collect()
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        p
    }

    /// A pose array with the given bone names and `poses` poses of `bones.len()`
    /// transforms (transform `j` of pose `p` filled with the byte `p * 16 + j + 1`).
    fn pose_array(e: &mut Engine, bones: &[&[u8]], poses: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        let names = e.mem.alloc(4 * bones.len().max(1) as u32);
        for (i, b) in bones.iter().enumerate() {
            let t = text(e, b);
            e.mem.set_u32(names + 4 * i as u32, t);
        }
        e.mem.set_u32(this + 8, bones.len() as u32);
        e.mem.set_u32(this + 0xc, names);
        let array = e.mem.alloc(4 * poses.max(1));
        for p in 0..poses {
            let pose = e.mem.alloc(8);
            let n = bones.len() as u32;
            let transforms = e.mem.alloc(0x30 * n.max(1));
            for j in 0..n {
                e.mem
                    .write(transforms + 0x30 * j, &[(p * 16 + j + 1) as u8; 0x30]);
            }
            e.mem.set_u32(pose, n);
            e.mem.set_u32(pose + 4, transforms);
            e.mem.set_u32(array + 4 * p, pose);
        }
        e.mem.set_u32(this + 0x10, poses);
        e.mem.set_u32(this + 0x14, array);
        this
    }

    fn skeleton(e: &mut Engine, bones: &[&[u8]]) -> u32 {
        let skeleton = e.mem.alloc(0x20);
        let array = e.mem.alloc(8 * bones.len().max(1) as u32);
        for (i, b) in bones.iter().enumerate() {
            let t = text(e, b);
            // Bit 0 of the stored pointer is a flag the code masks off.
            e.mem.set_u32(array + 8 * i as u32, t | 1);
        }
        e.mem.set_u32(skeleton + 0x18, array);
        e.mem.set_u32(skeleton + 0x1c, bones.len() as u32);
        skeleton
    }

    #[test]
    fn matches_skeleton_needs_every_bone_name() {
        let mut e = Engine::new();
        let this = pose_array(&mut e, &[b"Pelvis", b"Spine"], 0);
        for (bones, want) in [
            (vec![], true),
            (vec![&b"Spine"[..], b"Pelvis"], true),
            (vec![&b"Spine"[..], b"Head"], false),
            (vec![&b"Spin"[..]], false),
        ] {
            let skeleton = skeleton(&mut e, &bones);
            assert_eq!(
                e.call(0x00cc_1360, &args![this, skeleton]).bool(),
                want,
                "{bones:?}"
            );
        }
    }

    #[test]
    fn copy_pose_fills_the_matched_slots() {
        let mut e = Engine::new();
        let this = pose_array(&mut e, &[b"Pelvis", b"Spine"], 2);
        let skeleton = skeleton(&mut e, &[b"Spine", b"Head", b"Pelvis"]);
        let output = e.mem.alloc(0x90);
        e.mem.write(output, &[0xee; 0x90]);
        e.call(0x00cc_1420, &args![this, output, skeleton, 1u32]);
        // Pose 1: Spine is transform 1 (byte 0x12), Pelvis transform 0 (byte 0x11).
        assert_eq!(e.mem.bytes(output, 0x30), vec![0x12; 0x30]);
        assert_eq!(e.mem.bytes(output + 0x30, 0x30), vec![0xee; 0x30]);
        assert_eq!(e.mem.bytes(output + 0x60, 0x30), vec![0x11; 0x30]);
    }

    #[test]
    fn create_builds_an_empty_pose_array_or_null() {
        let mut e = Engine::new();
        e.register(OBJECT_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc_1290, &args![]).u32(), 0);
        assert!(calls_to(&e, NI_OBJECT_CTOR).is_empty());
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xff; 0x18]);
        e.register_double(OBJECT_ALLOCATE, move |_, a| {
            assert_eq!(a, [0x18]);
            block.into_ret()
        });
        e.register(NI_OBJECT_CTOR, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_1290, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), POSE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0xffff_ffff);
        assert_eq!(e.mem.bytes(block + 8, 0x10), vec![0; 0x10]);
    }

    #[test]
    fn register_streamables_registers_each_name_when_the_base_does() {
        for base in [0u8, 1] {
            let mut e = Engine::new();
            let this = pose_array(&mut e, &[b"A", b"B"], 0);
            let names = e.mem.u32(this + 0xc);
            e.register_double(NI_OBJECT_REGISTER_STREAMABLES, move |_, _| {
                (0x300 | u32::from(base)).into_ret()
            });
            let vtable = e.mem.alloc(0x24);
            e.mem.set_u32(vtable + 0x20, 0x0079_0020);
            e.register(0x0079_0020, |_, _| Ret::default());
            let stream = e.mem.alloc(4);
            e.mem.set_u32(stream, vtable);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00cc_1310, &args![this, stream]).u32(),
                u32::from(base)
            );
            let regs: Vec<u32> = calls_to(&e, 0x0079_0020).iter().map(|c| c.1[1]).collect();
            if base == 1 {
                assert_eq!(regs, vec![names, names + 4]);
            } else {
                assert!(regs.is_empty());
            }
        }
    }

    #[test]
    fn dtor_frees_names_poses_and_restores_the_tls_word() {
        let mut e = Engine::new();
        let this = pose_array(&mut e, &[], 2);
        let poses = e.mem.u32(this + 0x14);
        e.mem.set_u32(poses + 4, 0);
        let first = e.mem.u32(poses);
        let block = e.mem.alloc(12);
        e.mem.set_u32(block, 2);
        e.mem.set_u32(this + 0xc, block + 4);
        let context = e.tls() + TLS_CONTEXT;
        e.mem.set_u32(context, 7);
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(DEALLOCATE, |e, _| {
            let context = e.tls() + TLS_CONTEXT;
            assert_eq!(e.mem.u32(context), 0x10);
            Ret::default()
        });
        e.register(NI_OBJECT_DTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cc_1520, &args![this]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![(
                VECTOR_DESTRUCTOR_ITERATOR,
                vec![block + 4, 4, 2, FIXED_STRING_DTOR]
            )]
        );
        let freed: Vec<u32> = calls_to(&e, DEALLOCATE).iter().map(|c| c.1[1]).collect();
        assert_eq!(freed, vec![block, first, poses]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_DTOR),
            vec![(NI_OBJECT_DTOR, vec![this])]
        );
        assert_eq!(e.mem.u32(context), 7);
        assert_eq!(e.mem.u32(this), POSE_ARRAY_VTABLE);
    }

    #[test]
    fn save_binary_writes_counts_names_and_transforms() {
        let mut e = Engine::new();
        let this = pose_array(&mut e, &[b"A", b"B"], 1);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 0xc, 0x0079_0000);
        e.register(0x0079_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, binary);
        e.register(NI_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_FIXED_STRING, |_, _| Ret::default());
        e.register(STREAM_SAVE_TRANSFORM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cc_1610, &args![this, stream]);
        let pose = e.mem.u32(e.mem.u32(this + 0x14));
        let writes: Vec<u32> = calls_to(&e, 0x0079_0000).iter().map(|c| c.1[1]).collect();
        assert_eq!(writes, vec![this + 8, this + 0x10, pose]);
        assert_eq!(e.mem.u32(binary + 4), 12);
        assert_eq!(calls_to(&e, STREAM_SAVE_FIXED_STRING).len(), 2);
        let transforms = e.mem.u32(pose + 4);
        assert_eq!(
            calls_to(&e, STREAM_SAVE_TRANSFORM),
            vec![
                (STREAM_SAVE_TRANSFORM, vec![stream, transforms]),
                (STREAM_SAVE_TRANSFORM, vec![stream, transforms + 0x30])
            ]
        );
    }

    #[test]
    fn fn_00cc1190_calls_each_entrys_handler_and_frees_the_array() {
        let mut e = Engine::new();
        // Two entries: kind 1 with a handler (type byte 4, first sub-object byte 5 -> word
        // from the second object), kind 2 without one.
        let make_object = |e: &mut Engine, flag: u8, word: u32| {
            let object = e.mem.alloc(0x100);
            e.mem.set_i8(object + 0x10, 8);
            e.mem.set_u8(object + 8 + 0xe8, flag);
            e.mem.set_u32(object + 8 + 0xcc, word);
            object
        };
        let a = make_object(&mut e, 5, 0xaaaa);
        let b = make_object(&mut e, 0, 0xbbbb);
        let entry = e.mem.alloc(0x40);
        e.mem.set_u8(entry, 4);
        e.mem.set_u8(entry + 1, 1);
        e.mem.set_u32(entry + 8, 0x1234);
        e.mem.set_u32(entry + 0x10, a);
        e.mem.set_u32(entry + 0x14, b);
        let other = e.mem.alloc(0x40);
        e.mem.set_u8(other + 1, 2);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, entry);
        e.mem.set_u32(data + 8, other);
        e.register_double(COLLECT_ENTRIES, move |e, args| {
            e.mem.set_u32(args[1], data);
            e.mem.set_u32(args[1] + 4, 2);
            e.mem.set_u32(args[1] + 8, 2);
            Ret::default()
        });
        let table = e.mem.alloc(0x1800);
        e.mem.set_u32(table + 0x50 + 0x16c8, 0x0079_0000);
        e.register(0x0079_0000, |_, _| Ret::default());
        let second = e.mem.alloc(4);
        e.mem.set_u32(second, table);
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(vtable + 0x10, 0x0079_0010);
        e.register(0x0079_0010, |_, _| Ret::default());
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER_INSTANCE, move |_, _| router.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00cc_1190, &args![0x5000u32, second]);
        assert_eq!(calls_to(&e, COLLECT_ENTRIES)[0].1[0], 0x5010);
        assert_eq!(
            calls_to(&e, 0x0079_0000),
            vec![(
                0x0079_0000,
                vec![entry, entry + 0x30, a, b, second, 0x1234, 0xbbbb]
            )]
        );
        assert_eq!(
            calls_to(&e, 0x0079_0010),
            vec![(0x0079_0010, vec![allocator, data, 16])]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00cc1710_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_1520, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc1710, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc1710, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00cc_1520), vec![vec![this]; 2]);
    }

    #[test]
    fn return_constant_00cc1280_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_1280, &args![]).u32(), 0x0126_8590);
    }
}
