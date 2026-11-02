//! `NiMain/niskininstance.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Gamebryo allocator (`00aa13e0`, cdecl, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiAlloc` (`00aa1070`, cdecl, the size).
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiObject`'s constructor (`00a5d3a0`, `this` only).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiObject::LoadBinary` (`00a5d610`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiObject::CopyMembers` (`00a5d450`: `this`, the clone and the cloning
/// process).
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// The base `LinkObject` (`00a89af0`, a bare `RET 4`).
const NI_OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream` reader of a link id (`00a645b0`, the stream only).
const NI_STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`).
const NI_STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
/// `NiStream::GetObjectFromLinkID` (`00a63430`).
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`).
const NI_STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
/// The stream's file version (+0xD8) and the first version with a skin
/// partition link (`10.1.0.101`).
const STREAM_VERSION: u32 = 0xd8;
const VERSION_10_1_0_101: u32 = 0x0a01_0065;
/// `NiPointer<NiSkinPartition>::operator=` (`0066b0d0`: the holder and the
/// object).
const SKIN_PARTITION_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The clone map lookup (`00a53c30`: the map, the key and the address of
/// the result; answers whether it is there).
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// `InterlockedDecrement` / `InterlockedIncrement` (import slots).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiBound::Update` (`00a7fdd0`: the bound, the source bound and the
/// transform) and `NiBound::Merge` (`00a7f3f0`: the bound and the other).
const NI_BOUND_UPDATE: u32 = 0x00a7_fdd0;
const NI_BOUND_MERGE: u32 = 0x00a7_f3f0;
/// The bound moved by a transform (`00a7fea0`).
const NI_BOUND_TRANSFORM: u32 = 0x00a7_fea0;
/// The inverse of a transform (`008b42e0`: the transform and the result).
const TRANSFORM_INVERT: u32 = 0x008b_42e0;
/// The product of two transforms (`0062c250`: the first, the result and the
/// second; answers the result).
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
/// The `NiSkinInstance` vtable (its RTTI locator names
/// `.?AVNiSkinInstance@@`).
const NI_SKIN_INSTANCE_VTABLE: u32 = 0x0106_9b14;
/// `NiSkinInstance` (Xbox PDB field names): `m_spSkinData` +8,
/// `m_spSkinPartition` +0xC, `m_pkRootParent` +0x10, `m_ppkBones` +0x14,
/// `m_uiFrameID` +0x18.
const SKIN_DATA: u32 = 0x08;
const SKIN_PARTITION: u32 = 0x0c;
const ROOT_PARENT: u32 = 0x10;
const BONES: u32 = 0x14;
const FRAME_ID: u32 = 0x18;
/// `NiSkinData` on the PC: `m_spSkinPartition` +8, the root-parent-to-skin
/// transform +0xC, `m_pkBoneData` +0x40, `m_uiBones` +0x44; a `BoneData`
/// is 0x4C bytes with its bound at +0x34.
const DATA_PARTITION: u32 = 0x08;
const DATA_ROOT_TO_SKIN: u32 = 0x0c;
const DATA_BONE_DATA: u32 = 0x40;
const DATA_BONES: u32 = 0x44;
const BONE_DATA_SIZE: u32 = 0x4c;
const BONE_BOUND: u32 = 0x34;
/// `NiAVObject`'s world transform (+0x68; its translation +0x8C, scale
/// +0x98).
const WORLD_TRANSFORM: u32 = 0x68;
const WORLD_SCALE: u32 = 0x98;

/// Lets go of a reference-counted object: `InterlockedDecrement` of its
/// count (+4) and its virtual `+4` (`DeleteThis`) at 0.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

fn add_reference(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// `NiPointer::operator=` on the holder at `holder` with `value`.
fn assign(e: &mut Engine, holder: u32, value: u32) {
    let old = e.mem.u32(holder);
    if old == value {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(holder, value);
    if value != 0 {
        add_reference(e, value);
    }
}

// Translated from 00a866f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::CreateObject` (the class's stream factory; Ghidra made
/// no function of it): a new 0x34-byte object with the `NiObject`
/// constructor, the class's vtable, its pointers and counts cleared and
/// the frame id -1 (+0x2C is left as it is); null when the allocation
/// fails.
pub fn ni_skin_instance_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    construct_skin_instance(e, object);
    Ptr::new(object)
}

/// The constructor's stores, as `CreateObject` and `CreateClone` inline them.
fn construct_skin_instance(e: &mut Engine, object: u32) {
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_SKIN_INSTANCE_VTABLE);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x1c, 0x20, 0x24, 0x28, 0x30] {
        e.mem.set_u32(object + offset, 0);
    }
    e.mem.set_u32(object + FRAME_ID, 0xffff_ffff);
}

// Translated from 00a86340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::LoadBinary` (Xbox PDB): the `NiObject` part, the skin
/// data's link id, the skin partition's (from version 10.1.0.101), the root
/// parent's, then the bones' link ids.
pub fn ni_skin_instance_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_1_0_101 {
        e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
    }
    e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
    e.call(NI_STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
}

fn linked(e: &mut Engine, stream: Ptr) -> u32 {
    e.call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
        .u32()
}

// Translated from 00a86380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::LinkObject` (Xbox PDB): the skin data, the skin
/// partition (from version 10.1.0.101) and the root parent from their link
/// ids, then the bone array (`NiAlloc` of the link count) filled from the
/// next links.
pub fn ni_skin_instance_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(NI_OBJECT_LINK_OBJECT, &args![this, stream]);
    let data = linked(e, stream);
    assign(e, base + SKIN_DATA, data);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_1_0_101 {
        let partition = linked(e, stream);
        assign(e, base + SKIN_PARTITION, partition);
    }
    let root = linked(e, stream);
    e.mem.set_u32(base + ROOT_PARENT, root);
    let count = e
        .call(NI_STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream])
        .u32();
    let bones = e.call(NI_ALLOC, &args![count.wrapping_mul(4)]).u32();
    e.mem.set_u32(base + BONES, bones);
    for index in 0..count {
        let bone = linked(e, stream);
        let bones = e.mem.u32(base + BONES);
        e.mem.set_u32(bones + 4 * index, bone);
    }
}

// Translated from 00a86740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::PostLinkObject` (Xbox PDB): before version 10.1.0.101
/// the skin partition moves from the skin data to the instance (assigned,
/// then the data's pointer released and cleared).
pub fn ni_skin_instance_post_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_10_1_0_101 {
        return;
    }
    let data = e.mem.u32(base + SKIN_DATA);
    if data == 0 {
        return;
    }
    let partition = e.mem.u32(data + DATA_PARTITION);
    e.call(
        SKIN_PARTITION_POINTER_ASSIGN,
        &args![base + SKIN_PARTITION, partition],
    );
    let data = e.mem.u32(base + SKIN_DATA);
    let partition = e.mem.u32(data + DATA_PARTITION);
    if partition != 0 {
        release(e, partition);
        e.mem.set_u32(data + DATA_PARTITION, 0);
    }
}

// Translated from 00a86530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The skinned bound (unnamed in the engine map): the first bone's bound
/// placed by its world transform (`NiBound::Update`), merged with every
/// other bone's (moved by the translation only when the bone's world scale
/// is exactly 1.0, else `NiBound::Update`), then moved into skin space by
/// the root parent's inverse world transform times the data's
/// root-parent-to-skin transform (`00a7fea0`).
pub fn fn_00a86530(e: &mut Engine, this: Ptr, bound: Ptr) {
    let base = this.addr();
    let bones = e.mem.u32(base + BONES);
    let data = e.mem.u32(base + SKIN_DATA);
    let first_bone = e.mem.u32(bones);
    let bone_data = e.mem.u32(data + DATA_BONE_DATA);
    e.call(
        NI_BOUND_UPDATE,
        &args![bound, bone_data + BONE_BOUND, first_bone + WORLD_TRANSFORM],
    );
    let count = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
    e.with_stack(0x10, |e, local| {
        let local = local.addr();
        let mut source = bone_data.wrapping_add(BONE_DATA_SIZE + BONE_BOUND);
        let mut index = 1u32;
        while index < count {
            let bone = e.mem.u32(e.mem.u32(base + BONES) + 4 * index);
            let world = bone + WORLD_TRANSFORM;
            // FCOMP of 1.0 with the scale, C3 and C2 tested: equal.
            if 1.0 == x87_m32(e.mem.u32(bone + WORLD_SCALE)) {
                let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
                e.mem.set_u32(local + 0xc, radius);
                for axis in 0..3 {
                    let value = e.mem.u32(source + 4 * axis);
                    e.mem.set_u32(local + 4 * axis, value);
                }
                let mut moved = [0u32; 3];
                for (axis, slot) in moved.iter_mut().enumerate() {
                    let axis = axis as u32;
                    let offset = x87_load32(e.mem.u32(world + 0x24 + 4 * axis));
                    *slot = x87_store32(x87_add(offset, x87_m32(e.mem.u32(local + 4 * axis))));
                }
                for (axis, value) in moved.iter().enumerate() {
                    e.mem.set_u32(local + 4 * axis as u32, *value);
                }
            } else {
                e.call(NI_BOUND_UPDATE, &args![local, source, world]);
            }
            e.call(NI_BOUND_MERGE, &args![bound, local]);
            index += 1;
            source = source.wrapping_add(BONE_DATA_SIZE);
        }
    });
    e.with_stack(0x68, |e, transforms| {
        let inverse = transforms.addr();
        let product = transforms.addr() + 0x34;
        let root = e.mem.u32(base + ROOT_PARENT);
        e.call(TRANSFORM_INVERT, &args![root + WORLD_TRANSFORM, inverse]);
        let data = e.mem.u32(base + SKIN_DATA);
        let result = e
            .call(
                TRANSFORM_MULTIPLY,
                &args![data + DATA_ROOT_TO_SKIN, product, inverse],
            )
            .u32();
        let bytes = e.mem.bytes(result, 0x34);
        e.mem.write(inverse, &bytes);
        e.call(NI_BOUND_TRANSFORM, &args![bound, inverse]);
    });
}

// Translated from 00a86660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::CopyMembers` (by its use in `CreateClone`): the
/// `NiObject` part, then the clone shares the skin data and the skin
/// partition (reference-counted assignments).
pub fn fn_00a86660(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    let base = this.addr();
    let target = clone.addr();
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, clone, cloning]);
    for offset in [SKIN_DATA, SKIN_PARTITION] {
        let old = e.mem.u32(target + offset);
        if old == e.mem.u32(base + offset) {
            continue;
        }
        if old != 0 {
            release(e, old);
        }
        let value = e.mem.u32(base + offset);
        e.mem.set_u32(target + offset, value);
        if value != 0 {
            add_reference(e, value);
        }
    }
}

/// The clone map's entry for `key` written at `out`; whether there is one.
fn cloned(e: &mut Engine, cloning: Ptr, key: u32, out: u32) -> bool {
    let map = e.mem.u32(cloning.addr());
    e.call(CLONE_MAP_GET_AT, &args![map, key, out]).bool()
}

// Translated from 00a862d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the cloning process clones the root parent or any bone
/// (unnamed in the engine map; `CreateClone` clones only then).
pub fn fn_00a862d0(e: &mut Engine, this: Ptr, cloning: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, out| {
        let root = e.mem.u32(base + ROOT_PARENT);
        if cloned(e, cloning, root, out.addr()) {
            return true;
        }
        let count = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
        for index in 0..count {
            let bone = e.mem.u32(e.mem.u32(base + BONES) + 4 * index);
            if cloned(e, cloning, bone, out.addr()) {
                return true;
            }
        }
        false
    })
}

// Translated from 00a86830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::CreateClone` (Xbox PDB): `this` itself when nothing it
/// points at is cloned; otherwise a new instance (`CreateObject`'s stores,
/// null when the allocation fails, used as is) sharing the data, whose root
/// parent and bones are their clones where there is one.
pub fn ni_skin_instance_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let base = this.addr();
    if !fn_00a862d0(e, this, cloning) {
        return this;
    }
    let clone = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if clone != 0 {
        construct_skin_instance(e, clone);
    }
    fn_00a86660(e, this, Ptr::new(clone), cloning);
    e.with_stack(4, |e, out| {
        let out = out.addr();
        let root = e.mem.u32(base + ROOT_PARENT);
        let root = if cloned(e, cloning, root, out) {
            e.mem.u32(out)
        } else {
            e.mem.u32(base + ROOT_PARENT)
        };
        e.mem.set_u32(clone.wrapping_add(ROOT_PARENT), root);
        let count = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
        let bones = e.call(NI_ALLOC, &args![count.wrapping_mul(4)]).u32();
        e.mem.set_u32(clone.wrapping_add(BONES), bones);
        for index in 0..count {
            let bone = e.mem.u32(e.mem.u32(base + BONES) + 4 * index);
            let bone = if cloned(e, cloning, bone, out) {
                e.mem.u32(out)
            } else {
                e.mem.u32(e.mem.u32(base + BONES) + 4 * index)
            };
            let bones = e.mem.u32(clone.wrapping_add(BONES));
            e.mem.set_u32(bones + 4 * index, bone);
        }
    });
    Ptr::new(clone)
}

/// `NiSkinInstance::ms_RTTI` (`011f5b2c`, what the class's `GetRTTI` `00688590` answers).
const NI_SKIN_INSTANCE_RTTI: u32 = 0x011f_5b2c;
/// `NiObject::RegisterStreamables` (`00a5d2f0`), `NiObject::IsEqual` (`00a5d470`) and
/// `NiObject::GetViewerStrings` (`00a5d6e0`), `__thiscall` with one word.
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// The base `SaveBinary` (`00a89af0`, a bare `RET 4`; the address the file also calls for
/// `LinkObject`).
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// `NiObject`'s virtuals `RegisterStreamables` (+0x54), `IsEqual` (+0x5c) and
/// `GetViewerStrings` (+0x60), and `NiStream`'s +0x30 (one word: an object whose link id is
/// saved).
const REGISTER_STREAMABLES_SLOT: u32 = 0x54;
const IS_EQUAL_SLOT: u32 = 0x5c;
const GET_VIEWER_STRINGS_SLOT: u32 = 0x60;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
/// `NiStream`'s output stream (+0x250; write function +0xc: cdecl, stream, buffer, bytes,
/// component sizes, component count; position +4).
const STREAM_OUTPUT: u32 = 0x250;
/// `NiGetViewerString` (`00a7d330`), `NiGetViewerString_ov8` (`00a7d510`, label and an
/// unsigned word) and `NiGetViewerString_ov11` (`00a7d5e0`, label and a string), cdecl; the
/// string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`); `sprintf` (`00ec623a`),
/// `NiSprintf` (`00a305b0`), `qsort` (`00ec6f20`) with the comparison `00a86280`, `NiFree`
/// (`00aa10f0`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const SPRINTF: u32 = 0x00ec_623a;
const NI_SPRINTF: u32 = 0x00a3_05b0;
const QSORT: u32 = 0x00ec_6f20;
const COMPARE_NAMES_00A86280: u32 = 0x00a8_6280;
const NI_FREE: u32 = 0x00aa_10f0;
/// The labels and formats: `"Root Parent"`, `"Hardware partitions"`, `"Bone Count"`,
/// `"%s (%08X)"`, `"  %3d"` and `"<noname>"`.
const LABEL_ROOT_PARENT: u32 = 0x0109_e6e0;
const LABEL_HARDWARE_PARTITIONS: u32 = 0x0109_e6cc;
const LABEL_BONE_COUNT: u32 = 0x0109_e6c0;
const FORMAT_BONE: u32 = 0x0103_b570;
const FORMAT_INDEX: u32 = 0x0109_e6b8;
const NO_NAME: u32 = 0x0109_e6ac;
/// `NiObjectNET::m_kName` (+8): the name of the root parent and of each bone, and the skin
/// partition's partition count (`NiSkinPartition` +8).
const NAME: u32 = 0x08;
const PARTITION_COUNT: u32 = 0x08;

// Translated from 00a86460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::RegisterStreamables` (Xbox PDB): false when `NiObject`'s registration
/// (`00a5d2f0`) is; otherwise the skin data, the skin partition (when set), the root parent and
/// each bone (the data's bone count, read once) register themselves (virtual +0x54), and the
/// answer is true.
pub fn ni_skin_instance_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let base = this.addr();
    let data = e.mem.u32(base + SKIN_DATA);
    e.vcall(data, REGISTER_STREAMABLES_SLOT, &args![stream]);
    let partition = e.mem.u32(base + SKIN_PARTITION);
    if partition != 0 {
        e.vcall(partition, REGISTER_STREAMABLES_SLOT, &args![stream]);
    }
    let root = e.mem.u32(base + ROOT_PARENT);
    e.vcall(root, REGISTER_STREAMABLES_SLOT, &args![stream]);
    let bones = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
    for index in 0..bones {
        let bone = e.mem.u32(e.mem.u32(base + BONES).wrapping_add(index * 4));
        e.vcall(bone, REGISTER_STREAMABLES_SLOT, &args![stream]);
    }
    true
}

// Translated from 00a864d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::IsEqual` (Xbox PDB): `NiObject::IsEqual` (`00a5d470`), equal skin data (its
/// virtual `IsEqual`, +0x5c), an equal skin partition when this one has one, and no partition
/// on the other when this one has none.
pub fn ni_skin_instance_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let (a, b) = (this.addr(), other.addr());
    let data = e.mem.u32(a + SKIN_DATA);
    let their_data = e.mem.u32(b + SKIN_DATA);
    if !e.vcall(data, IS_EQUAL_SLOT, &args![their_data]).bool() {
        return false;
    }
    let partition = e.mem.u32(a + SKIN_PARTITION);
    if partition != 0 {
        let theirs = e.mem.u32(b + SKIN_PARTITION);
        if !e.vcall(partition, IS_EQUAL_SLOT, &args![theirs]).bool() {
            return false;
        }
    }
    e.mem.u32(a + SKIN_PARTITION) != 0 || e.mem.u32(b + SKIN_PARTITION) == 0
}

// Translated from 00a867a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::SaveBinary` (Xbox PDB): the base save (`00a89af0`), the link ids of the
/// skin data, the skin partition and the root parent (the stream's virtual +0x30), the data's
/// bone count (written from the stream argument's own stack word) and each bone's link id.
pub fn ni_skin_instance_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    for field in [SKIN_DATA, SKIN_PARTITION, ROOT_PARENT] {
        let object = e.mem.u32(base + field);
        e.vcall(stream.addr(), STREAM_SAVE_LINK_ID_SLOT, &args![object]);
    }
    let bones = e.mem.u32(e.mem.u32(base + SKIN_DATA) + DATA_BONES);
    let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    e.with_stack(8, |e, frame| {
        let (count, sizes) = (frame.addr(), frame.addr() + 4);
        e.mem.set_u32(count, bones);
        e.mem.set_u32(sizes, 4);
        let function = e.mem.u32(output + 0xc);
        let written = e
            .call(function, &args![output, count, 4u32, sizes, 1u32])
            .u32();
        let position = e.mem.u32(output + 4).wrapping_add(written);
        e.mem.set_u32(output + 4, position);
        let mut index = 0u32;
        while index < e.mem.u32(count) {
            let bone = e.mem.u32(e.mem.u32(base + BONES).wrapping_add(index * 4));
            e.vcall(stream.addr(), STREAM_SAVE_LINK_ID_SLOT, &args![bone]);
            index += 1;
        }
    });
}

/// The inlined `NiTArray<char *>::Add` through the stack word at `slot`: grown to its size
/// plus `m_usGrowBy` when full, then `SetAt` at the old size.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32, value: u32) {
    e.mem.set_u32(slot, value);
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let grow_by = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a86920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSkinInstance::GetViewerStrings` (Xbox PDB): `NiObject`'s strings (`00a5d6e0`), the class
/// name, `"Root Parent"` with its name when there is one, and with skin data:
/// `"Hardware partitions"` (the partition count, 0 without a partition), `"Bone Count"`, then
/// the bones as `"<name> (<address>)"` (printed into a 1000-byte stack buffer, copied into
/// blocks from `00aa1070`, sorted with `qsort` and `00a86280`), each appended as
/// `"  <n>"` (`NiSprintf` into a 10-byte buffer) with its text (`"<noname>"` for null) through
/// `SetAt` written out (the size and the non-null count kept as `NiTArray::SetAt` does). The
/// copies and their array are freed, and the skin partition adds its strings (its virtual
/// +0x60; read without a null test).
// Translation incomplete: 00a86920 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_skin_instance_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let base = this.addr();
    let strings = strings.addr();
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(0x404, |e, frame| {
        let slot = frame.addr();
        let index_text = slot + 0xc;
        let bone_text = slot + 0x18;
        let class_name = e.mem.u32(NI_SKIN_INSTANCE_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        add_viewer_string(e, strings, slot, text);
        let root = e.mem.u32(base + ROOT_PARENT);
        if root != 0 {
            let name = e.mem.u32(root + NAME);
            let text = e
                .call(NI_GET_VIEWER_STRING_TEXT, &args![LABEL_ROOT_PARENT, name])
                .u32();
            add_viewer_string(e, strings, slot, text);
        }
        let data = e.mem.u32(base + SKIN_DATA);
        if data == 0 {
            return;
        }
        let partition = e.mem.u32(base + SKIN_PARTITION);
        let partitions = if partition != 0 {
            e.mem.u32(partition + PARTITION_COUNT)
        } else {
            0
        };
        let text = e
            .call(
                NI_GET_VIEWER_STRING_UNSIGNED,
                &args![LABEL_HARDWARE_PARTITIONS, partitions],
            )
            .u32();
        add_viewer_string(e, strings, slot, text);
        let bones = e.mem.u32(data + DATA_BONES);
        let text = e
            .call(
                NI_GET_VIEWER_STRING_UNSIGNED,
                &args![LABEL_BONE_COUNT, bones],
            )
            .u32();
        add_viewer_string(e, strings, slot, text);
        let names = e.call(NI_ALLOC, &args![bones.wrapping_mul(4)]).u32();
        for index in 0..bones {
            let bone = e.mem.u32(e.mem.u32(base + BONES).wrapping_add(index * 4));
            let name = e.mem.u32(bone + NAME);
            e.call(SPRINTF, &args![bone_text, FORMAT_BONE, name, bone]);
            let text = e.mem.cstr(bone_text);
            let copy = e.call(NI_ALLOC, &args![text.len() as u32 + 1]).u32();
            e.mem.set_u32(names.wrapping_add(index * 4), copy);
            e.mem.set_cstr(copy, &text);
        }
        e.call(QSORT, &args![names, bones, 4u32, COMPARE_NAMES_00A86280]);
        for index in 0..bones {
            e.call(NI_SPRINTF, &args![index_text, 10u32, FORMAT_INDEX, index]);
            let mut name = e.mem.u32(names.wrapping_add(index * 4));
            if name == 0 {
                name = NO_NAME;
            }
            let text = e
                .call(NI_GET_VIEWER_STRING_TEXT, &args![index_text, name])
                .u32();
            let at = u32::from(e.mem.u16(strings + 0xa));
            if at >= u32::from(e.mem.u16(strings + 8)) {
                let grow_by = u32::from(e.mem.u16(strings + 0xe));
                e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + at]);
            }
            let element = e.mem.u32(strings + 4).wrapping_add(at * 4);
            if at >= u32::from(e.mem.u16(strings + 0xa)) {
                e.mem.set_u16(strings + 0xa, (at + 1) as u16);
                if text != 0 {
                    let used = e.mem.u16(strings + 0xc).wrapping_add(1);
                    e.mem.set_u16(strings + 0xc, used);
                }
            } else if text != 0 {
                if e.mem.u32(element) == 0 {
                    let used = e.mem.u16(strings + 0xc).wrapping_add(1);
                    e.mem.set_u16(strings + 0xc, used);
                }
            } else if e.mem.u32(element) != 0 {
                let used = e.mem.u16(strings + 0xc).wrapping_add(0xffff);
                e.mem.set_u16(strings + 0xc, used);
            }
            let element = e.mem.u32(strings + 4).wrapping_add(at * 4);
            e.mem.set_u32(element, text);
        }
        for index in 0..bones {
            let copy = e.mem.u32(names.wrapping_add(index * 4));
            e.call(NI_FREE, &args![copy]);
        }
        e.call(NI_FREE, &args![names]);
        let partition = e.mem.u32(base + SKIN_PARTITION);
        e.vcall(partition, GET_VIEWER_STRINGS_SLOT, &args![strings]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a866f0, ni_skin_instance_create_object() -> Ptr),
        entry!(0x00a86340, ni_skin_instance_load_binary(Ptr, Ptr)),
        entry!(0x00a86380, ni_skin_instance_link_object(Ptr, Ptr)),
        entry!(0x00a86740, ni_skin_instance_post_link_object(Ptr, Ptr)),
        entry!(0x00a86530, fn_00a86530(Ptr, Ptr)),
        entry!(0x00a86660, fn_00a86660(Ptr, Ptr, Ptr)),
        entry!(0x00a862d0, fn_00a862d0(Ptr, Ptr) -> bool),
        entry!(0x00a86830, ni_skin_instance_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a86460, ni_skin_instance_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a864d0, ni_skin_instance_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a867a0, ni_skin_instance_save_binary(Ptr, Ptr)),
        entry!(0x00a86920, ni_skin_instance_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn counting(e: &mut Engine) {
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    /// A reference-counted object with `count` references and a
    /// `DeleteThis` at slot +4 (`0x0620_0004`).
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0620_0004);
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn create_object_sets_the_start_values() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT, nothing);
        let object = e.call(0x00a8_66f0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(object + FRAME_ID), 0xffff_ffff);
        assert_eq!(e.mem.u32(object + 0x30), 0);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_66f0, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_reads_the_links_by_version() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, nothing);
        e.register(NI_STREAM_READ_LINK_ID, nothing);
        e.register(NI_STREAM_READ_MULTIPLE_LINK_IDS, nothing);
        let stream = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6340, &args![0x10u32, stream]);
        assert_eq!(calls_to(&e, NI_STREAM_READ_LINK_ID).len(), 2);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_10_1_0_101);
        e.call(0x00a8_6340, &args![0x10u32, stream]);
        assert_eq!(calls_to(&e, NI_STREAM_READ_LINK_ID).len(), 5);
        assert_eq!(calls_to(&e, NI_STREAM_READ_MULTIPLE_LINK_IDS).len(), 2);
    }

    #[test]
    fn link_object_assigns_the_links() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(NI_OBJECT_LINK_OBJECT, nothing);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(0x0620_0004, nothing);
        let data = counted(&mut e, 0);
        let partition = counted(&mut e, 0);
        let old = counted(&mut e, 1);
        let links = std::rc::Rc::new(std::cell::RefCell::new(vec![
            data, partition, 0x700, 0x801, 0x802,
        ]));
        let next = links.clone();
        e.register_double(NI_STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next.borrow_mut().remove(0).into_ret()
        });
        e.register(NI_STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| 2u32.into_ret());
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_10_1_0_101);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, old);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6380, &args![this, stream]);
        assert_eq!(e.mem.u32(this + SKIN_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(this + SKIN_PARTITION), partition);
        assert_eq!(calls_to(&e, 0x0620_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + ROOT_PARENT), 0x700);
        let bones = e.mem.u32(this + BONES);
        assert_eq!(e.mem.u32(bones), 0x801);
        assert_eq!(e.mem.u32(bones + 4), 0x802);
        assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![8]]);
    }

    #[test]
    fn post_link_object_moves_the_partition_for_old_files() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(SKIN_PARTITION_POINTER_ASSIGN, nothing);
        e.register(0x0620_0004, nothing);
        let partition = counted(&mut e, 1);
        let data = e.mem.alloc(0x48);
        e.mem.set_u32(data + DATA_PARTITION, partition);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_10_1_0_101);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6740, &args![this, stream]);
        assert!(calls_to(&e, SKIN_PARTITION_POINTER_ASSIGN).is_empty());
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0064);
        e.call(0x00a8_6740, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SKIN_PARTITION_POINTER_ASSIGN),
            vec![vec![this + SKIN_PARTITION, partition]]
        );
        assert_eq!(e.mem.u32(data + DATA_PARTITION), 0);
        assert_eq!(calls_to(&e, 0x0620_0004), vec![vec![partition]]);
    }

    #[test]
    fn skinned_bound_merges_every_bone() {
        let mut e = Engine::new();
        e.register(NI_BOUND_UPDATE, |e, a| {
            e.mem.set_f32(a[0], 100.0);
            Ret::default()
        });
        e.register(NI_BOUND_MERGE, nothing);
        e.register(TRANSFORM_INVERT, nothing);
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            a[1].into_ret()
        });
        e.register(NI_BOUND_TRANSFORM, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1234);
            Ret::default()
        });
        let bone_data = e.mem.alloc(0x4c * 3);
        // Bone 1's bound: centre (1, 2, 3), radius 4.
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(bone_data + 0x4c + 0x34 + 4 * i as u32, *v);
        }
        let data = e.mem.alloc(0x48);
        e.mem.set_u32(data + DATA_BONE_DATA, bone_data);
        e.mem.set_u32(data + DATA_BONES, 3);
        let bone = |e: &mut Engine, scale: f32| {
            let b = e.mem.alloc(0x9c);
            e.mem.set_f32(b + WORLD_SCALE, scale);
            for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
                e.mem.set_f32(b + 0x8c + 4 * i as u32, *v);
            }
            b
        };
        let bones = e.mem.alloc(12);
        let b0 = bone(&mut e, 1.0);
        let b1 = bone(&mut e, 1.0);
        let b2 = bone(&mut e, 2.0);
        for (i, b) in [b0, b1, b2].iter().enumerate() {
            e.mem.set_u32(bones + 4 * i as u32, *b);
        }
        let root = e.mem.alloc(0x9c);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        e.mem.set_u32(this + BONES, bones);
        e.mem.set_u32(this + ROOT_PARENT, root);
        let bound = e.mem.alloc(0x10);
        let merged = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = merged.clone();
        e.register_double(NI_BOUND_MERGE, move |e, a| {
            record.borrow_mut().push(e.mem.bytes(a[1], 16));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a8_6530, &args![this, bound]);
        let updates = calls_to(&e, NI_BOUND_UPDATE);
        assert_eq!(updates[0], vec![bound, bone_data + 0x34, b0 + 0x68]);
        assert_eq!(updates[1][1..], [bone_data + 0x4c * 2 + 0x34, b2 + 0x68]);
        let merged = merged.borrow();
        assert_eq!(merged.len(), 2);
        let floats: Vec<f32> = merged[0]
            .chunks(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        assert_eq!(floats, vec![11.0, 22.0, 33.0, 4.0]);
        assert_eq!(calls_to(&e, TRANSFORM_INVERT)[0][0], root + WORLD_TRANSFORM);
        assert_eq!(
            calls_to(&e, TRANSFORM_MULTIPLY)[0][0],
            data + DATA_ROOT_TO_SKIN
        );
        assert_eq!(calls_to(&e, NI_BOUND_TRANSFORM)[0][0], bound);
    }

    #[test]
    fn copy_members_shares_data_and_partition() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.register(0x0620_0004, nothing);
        let data = counted(&mut e, 1);
        let partition = counted(&mut e, 1);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        e.mem.set_u32(this + SKIN_PARTITION, partition);
        let clone = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6660, &args![this, clone, 0x99u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this, clone, 0x99]]
        );
        assert_eq!(e.mem.u32(clone + SKIN_DATA), data);
        assert_eq!(e.mem.u32(clone + SKIN_PARTITION), partition);
        assert_eq!(e.mem.u32(data + 4), 2);
        e.call(0x00a8_6660, &args![this, clone, 0x99u32]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    /// A clone map double: `key` maps to `key + 0x10000` for keys in `known`.
    fn clone_map(e: &mut Engine, known: Vec<u32>) {
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            if known.contains(&a[1]) {
                e.mem.set_u32(a[2], a[1] + 0x10000);
                true.into_ret()
            } else {
                false.into_ret()
            }
        });
    }

    fn instance(e: &mut Engine) -> (u32, u32) {
        let data = e.mem.alloc(0x48);
        e.mem.set_u32(data + DATA_BONES, 2);
        let bones = e.mem.alloc(8);
        e.mem.set_u32(bones, 0x801);
        e.mem.set_u32(bones + 4, 0x802);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + SKIN_DATA, data);
        e.mem.set_u32(this + BONES, bones);
        e.mem.set_u32(this + ROOT_PARENT, 0x700);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x5555);
        (this, cloning)
    }

    #[test]
    fn cloned_when_the_root_or_a_bone_is() {
        let mut e = Engine::new();
        let (this, cloning) = instance(&mut e);
        clone_map(&mut e, vec![]);
        assert!(!e.call(0x00a8_62d0, &args![this, cloning]).bool());
        clone_map(&mut e, vec![0x802]);
        assert!(e.call(0x00a8_62d0, &args![this, cloning]).bool());
        clone_map(&mut e, vec![0x700]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a8_62d0, &args![this, cloning]).bool());
        assert_eq!(calls_to(&e, CLONE_MAP_GET_AT).len(), 1);
        assert_eq!(calls_to(&e, CLONE_MAP_GET_AT)[0][..2], [0x5555, 0x700]);
    }

    #[test]
    fn create_clone_maps_root_and_bones() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_OBJECT_CONSTRUCT, nothing);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        let (this, cloning) = instance(&mut e);
        clone_map(&mut e, vec![]);
        assert_eq!(e.call(0x00a8_6830, &args![this, cloning]).u32(), this);
        clone_map(&mut e, vec![0x802]);
        let clone = e.call(0x00a8_6830, &args![this, cloning]).u32();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone), NI_SKIN_INSTANCE_VTABLE);
        assert_eq!(e.mem.u32(clone + ROOT_PARENT), 0x700);
        let bones = e.mem.u32(clone + BONES);
        assert_eq!(e.mem.u32(bones), 0x801);
        assert_eq!(e.mem.u32(bones + 4), 0x10802);
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    const OBJECT_DOUBLE: u32 = 0x0070_e000;
    const STREAM_DOUBLE: u32 = 0x0070_e010;
    const WRITE_DOUBLE: u32 = 0x0070_e020;

    /// An object whose virtual +0x54, +0x5c and +0x60 are the object double; its name at +8.
    fn object(e: &mut Engine, name: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for slot in [
            REGISTER_STREAMABLES_SLOT,
            IS_EQUAL_SLOT,
            GET_VIEWER_STRINGS_SLOT,
        ] {
            e.mem.set_u32(vtable + slot, OBJECT_DOUBLE);
        }
        e.mem.set_u32(object + NAME, name);
        object
    }

    /// A skin instance with skin data (`bones` bones), a partition, a root parent and bones.
    fn instance(e: &mut Engine, bones: u32, partition: bool) -> u32 {
        let skin = e.mem.alloc(0x20);
        let data = object(e, 0);
        let data_block = e.mem.alloc(0x48);
        e.mem.write(data_block, &e.mem.bytes(data, 0x20));
        e.mem.set_u32(data_block + DATA_BONES, bones);
        e.mem.set_u32(skin + SKIN_DATA, data_block);
        if partition {
            let p = object(e, 0);
            e.mem.set_u32(p + PARTITION_COUNT, 3);
            e.mem.set_u32(skin + SKIN_PARTITION, p);
        }
        let root = object(e, 0x5000);
        e.mem.set_u32(skin + ROOT_PARENT, root);
        let array = e.mem.alloc(4 * bones + 4);
        for i in 0..bones {
            let bone = object(e, 0x6000 + i);
            e.mem.set_u32(array + 4 * i, bone);
        }
        e.mem.set_u32(skin + BONES, array);
        skin
    }

    #[test]
    fn register_streamables_registers_every_part() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, a| {
            word(u32::from(a[1] != 0))
        });
        e.register(OBJECT_DOUBLE, |_, _| Ret::default());
        let skin = instance(&mut e, 2, false);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a8_6460, &args![skin, 0u32]).bool());
        assert_eq!(calls_to(&e, OBJECT_DOUBLE).len(), 0);
        assert!(e.call(0x00a8_6460, &args![skin, 0x77u32]).bool());
        let registered: Vec<u32> = calls_to(&e, OBJECT_DOUBLE).iter().map(|c| c[0]).collect();
        let bones = e.mem.u32(skin + BONES);
        assert_eq!(
            registered,
            vec![
                e.mem.u32(skin + SKIN_DATA),
                e.mem.u32(skin + ROOT_PARENT),
                e.mem.u32(bones),
                e.mem.u32(bones + 4)
            ]
        );
        let skin = instance(&mut e, 0, true);
        e.call(0x00a8_6460, &args![skin, 0x77u32]);
        assert_eq!(
            calls_to(&e, OBJECT_DOUBLE)[5][0],
            e.mem.u32(skin + SKIN_PARTITION)
        );
    }

    #[test]
    fn is_equal_checks_data_and_partition() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_IS_EQUAL, |_, _| word(1));
        // Equal when the other object's word at +0x10 is not 1.
        e.register(OBJECT_DOUBLE, |e, a| {
            word(u32::from(e.mem.u32(a[1] + 0x10) != 1))
        });
        let a = instance(&mut e, 0, false);
        let b = instance(&mut e, 0, false);
        assert!(e.call(0x00a8_64d0, &args![a, b]).bool());
        let their_data = e.mem.u32(b + SKIN_DATA);
        e.mem.set_u32(their_data + 0x10, 1);
        assert!(!e.call(0x00a8_64d0, &args![a, b]).bool());
        e.mem.set_u32(their_data + 0x10, 0);
        // A partition on the other side only.
        let p = object(&mut e, 0);
        e.mem.set_u32(b + SKIN_PARTITION, p);
        assert!(!e.call(0x00a8_64d0, &args![a, b]).bool());
        let mine = object(&mut e, 0);
        e.mem.set_u32(a + SKIN_PARTITION, mine);
        assert!(e.call(0x00a8_64d0, &args![a, b]).bool());
        e.mem.set_u32(p + 0x10, 1);
        assert!(!e.call(0x00a8_64d0, &args![a, b]).bool());
        e.register(NI_OBJECT_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a8_64d0, &args![a, b]).bool());
    }

    #[test]
    fn save_binary_saves_links_and_the_bone_count() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_DOUBLE, |_, _| Ret::default());
        let written: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(vec![]));
        let log = written.clone();
        e.register_double(WRITE_DOUBLE, move |e, a| {
            log.borrow_mut().push((e.mem.u32(a[1]), e.mem.u32(a[3])));
            word(a[2])
        });
        let stream = e.mem.alloc(0x260);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem
            .set_u32(vtable + STREAM_SAVE_LINK_ID_SLOT, STREAM_DOUBLE);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 0xc, WRITE_DOUBLE);
        e.mem.set_u32(stream + STREAM_OUTPUT, out);
        let skin = instance(&mut e, 2, true);
        e.call_log = Some(vec![]);
        e.call(0x00a8_67a0, &args![skin, stream]);
        let saved: Vec<u32> = calls_to(&e, STREAM_DOUBLE).iter().map(|c| c[1]).collect();
        let bones = e.mem.u32(skin + BONES);
        assert_eq!(
            saved,
            vec![
                e.mem.u32(skin + SKIN_DATA),
                e.mem.u32(skin + SKIN_PARTITION),
                e.mem.u32(skin + ROOT_PARENT),
                e.mem.u32(bones),
                e.mem.u32(bones + 4)
            ]
        );
        assert_eq!(*written.borrow(), vec![(2, 4)]);
        assert_eq!(e.mem.u32(out + 4), 4);
    }

    #[test]
    fn get_viewer_strings_lists_the_sorted_bones() {
        let mut e = Engine::new();
        e.map(NI_SKIN_INSTANCE_RTTI, 4);
        e.mem.set_u32(NI_SKIN_INSTANCE_RTTI, 0x0109_e700);
        e.register(NI_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| word(a[0]));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, a| word(a[1]));
        e.register(NI_GET_VIEWER_STRING_TEXT, |e, a| {
            // The index text's first byte, then the name: one word to check.
            word(if a[0] == LABEL_ROOT_PARENT {
                0x1111
            } else if a[1] == NO_NAME {
                0
            } else {
                u32::from(e.mem.u8(a[0])) << 24 | (a[1] & 0xffff)
            })
        });
        e.register(NI_ALLOC, |e, a| word(e.mem.alloc(a[0])));
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(OBJECT_DOUBLE, |_, _| Ret::default());
        // `sprintf` writes "b<low byte of the bone's name>".
        e.register(SPRINTF, |e, a| {
            e.mem.set_cstr(a[0], &[b'b', a[2] as u8]);
            Ret::default()
        });
        e.register(NI_SPRINTF, |e, a| {
            e.mem.set_u8(a[0], b'0' + a[3] as u8);
            Ret::default()
        });
        // The sort turns the order round and blanks the last name.
        e.register(QSORT, |e, a| {
            let (base, count) = (a[0], a[1]);
            let mut names: Vec<u32> = (0..count).map(|i| e.mem.u32(base + 4 * i)).collect();
            names.reverse();
            names[count as usize - 1] = 0;
            for (i, n) in names.iter().enumerate() {
                e.mem.set_u32(base + 4 * i as u32, *n);
            }
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let slot = e.mem.u32(a[0] + 4) + 4 * a[1];
            e.mem.set_u32(slot, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x80);
        e.mem.set_u32(strings + 4, array);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xe, 8);
        let skin = instance(&mut e, 2, true);
        e.call_log = Some(vec![]);
        e.call(0x00a8_6920, &args![skin, strings]);
        let root = e.mem.u32(skin + ROOT_PARENT);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_TEXT)[0],
            vec![LABEL_ROOT_PARENT, e.mem.u32(root + NAME)]
        );
        let values: Vec<u32> = (0..6).map(|i| e.mem.u32(array + 4 * i)).collect();
        // Class, root parent text, 3 partitions, 2 bones, then the bones in sorted order:
        // "b\x01" first, the blanked one last as "<noname>".
        let first_copy = e.mem.u32(calls_to(&e, NI_GET_VIEWER_STRING_TEXT)[1][1]);
        assert_eq!(values[0], 0x0109_e700);
        assert_eq!(values[2..4], [3, 2]);
        assert_eq!(
            values[4],
            (u32::from(b'0') << 24) | (calls_to(&e, NI_GET_VIEWER_STRING_TEXT)[1][1] & 0xffff)
        );
        assert_eq!(first_copy & 0xffff, u32::from(b'b') | 1 << 8);
        assert_eq!(values[5], 0);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING_TEXT)[2][1], NO_NAME);
        assert_eq!(e.mem.u16(strings + 0xa), 6);
        // The string at index 4 counted as set; the null one not.
        assert_eq!(e.mem.u16(strings + 0xc), 1);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 12]]);
        assert_eq!(calls_to(&e, NI_FREE).len(), 3);
        assert_eq!(
            calls_to(&e, OBJECT_DOUBLE),
            vec![vec![e.mem.u32(skin + SKIN_PARTITION), strings]]
        );
        // Without skin data only the class and the root parent.
        e.mem.set_u32(skin + SKIN_DATA, 0);
        e.call(0x00a8_6920, &args![skin, strings]);
        assert_eq!(e.mem.u16(strings + 0xa), 8);
    }
}
