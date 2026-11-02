//! `NiMain/nilodnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLODNode::ms_RTTI` (the class's `NiRTTI` record).
const NI_LOD_NODE_RTTI: u32 = 0x011f_5f1c;

// Translated from 00a994c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_lod_node_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LOD_NODE_RTTI)
}

// Translated from 00a99670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a99470`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xd4 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_lod_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_9470;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xd4u32]);
    }
    this
}

/// `NiLODNode` (Xbox PDB) fields at their PC offsets: `m_spLODData` (+0xcc) and
/// `m_bLODActive` (+0xd0); `NiSwitchNode`'s chosen child index (+0xb0), update time
/// (+0xb4), revision id (+0xb8), child revision array (+0xbc, its element block +0xc0) and
/// flags (+0xac); `NiNode`'s children (element block +0xa0, used size +0xa6).
const LOD_DATA: u32 = 0xcc;
const LOD_ACTIVE: u32 = 0xd0;
const SWITCH_INDEX: u32 = 0xb0;
const CHILDREN_BASE: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;
/// The vtables of `NiLODNode` (`010a0b64`) and of the child revision array's base
/// (`0101a3a4`).
const NI_LOD_NODE_VTABLE: u32 = 0x010a_0b64;
const REVISION_ARRAY_VTABLE: u32 = 0x0101_a3a4;
/// The forced level of detail (`011abdf4`, signed; below 0 means none).
const FORCED_LOD_LEVEL: u32 = 0x011a_bdf4;
/// `"NULL LOD Data"` (`010a0c64`).
const TEXT_NULL_LOD_DATA: u32 = 0x010a_0c64;
/// Import slots of `InterlockedIncrement` / `InterlockedDecrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The Ni allocator `operator new` (`00aa13e0`, `cdecl`) and free (`00aa10f0`, `cdecl`).
const NI_NEW: u32 = 0x00aa_13e0;
const NI_FREE: u32 = 0x00aa_10f0;
/// Base functions (`thiscall`): `NiAVObject::UpdateWorldData` (`00a68c60`); the switch
/// node's `OnVisible` (`00a93ef0`), member copy (`00a93c00`), `IsEqual` (`00a94000`),
/// constructor (`00a94360`), `LoadBinary` (`00a94570`), `SaveBinary` (`00a94610`) and
/// `GetViewerStrings` (`00a94680`); `RegisterStreamables` (`00a93c40`) and `LinkObject`
/// (`00a9a730`), which the engine map names after `NiBillboardNode` (folded code);
/// `NiNode`'s destructor body (`00a5e930`).
const AV_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
const SWITCH_ON_VISIBLE: u32 = 0x00a9_3ef0;
const SWITCH_COPY_MEMBERS: u32 = 0x00a9_3c00;
const SWITCH_IS_EQUAL: u32 = 0x00a9_4000;
const SWITCH_CONSTRUCT: u32 = 0x00a9_4360;
const SWITCH_LOAD_BINARY: u32 = 0x00a9_4570;
const SWITCH_SAVE_BINARY: u32 = 0x00a9_4610;
const SWITCH_GET_VIEWER_STRINGS: u32 = 0x00a9_4680;
const BASE_REGISTER_STREAMABLES: u32 = 0x00a9_3c40;
const BASE_LINK_OBJECT: u32 = 0x00a9_a730;
const NODE_DESTRUCT: u32 = 0x00a5_e930;
/// `NiRangeLODData`'s constructor (`00a97700`, `thiscall`) and `SetNumRanges` (`00a978a0`,
/// `thiscall`, the count); its center (+8, three words), range count (+0x20) and ranges
/// (+0x24, 0x10 bytes each: near +0, far +4).
const RANGE_DATA_CONSTRUCT: u32 = 0x00a9_7700;
const RANGE_DATA_SET_NUM_RANGES: u32 = 0x00a9_78a0;
/// `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the three floats, the stream).
const POINT3_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiStream`: `GetObjectFromLinkID` (`00a63430`) and `00a645b0` (both `thiscall`, no stack
/// words), its file version (+0xd8), input (+0x24c), `SaveLinkID` (vtable +0x30).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
/// The `NiTArray` `SetAt` (`0096ae90`: index, address of the value) and resize
/// (`0096ad30`), and `NiGetViewerString` (`00a7d330`).
const ARRAY_SET_AT: u32 = 0x0096_ae90;
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
/// The LOD data's vtable slots (`NiLODData`): `GetRTTI` (+8), `RegisterStreamables`
/// (+0x54), `IsEqual` (+0x5c), `GetViewerStrings` (+0x60), +0x8c (the child index for a
/// camera), +0x90 (run by the world update), +0x94 (the copy for a clone), +0x98 (the
/// child index for a forced level).
const DATA_GET_RTTI: u32 = 0x8;
const DATA_REGISTER_STREAMABLES: u32 = 0x54;
const DATA_IS_EQUAL: u32 = 0x5c;
const DATA_GET_VIEWER_STRINGS: u32 = 0x60;
const DATA_CHILD_FOR_CAMERA: u32 = 0x8c;
const DATA_UPDATE_WORLD_DATA: u32 = 0x90;
const DATA_DUPLICATE: u32 = 0x94;
const DATA_CHILD_FOR_LEVEL: u32 = 0x98;
/// A child's update function (vtable +0xa4: the update data and a flag).
const CHILD_UPDATE_SLOT: u32 = 0xa4;

/// `InterlockedDecrement` of the count at `object + 4`; `DeleteThis` (slot +4) at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The inlined smart-pointer assignment of the LOD data: nothing when it is the same;
/// otherwise the old one released, the new one stored and, when not null, counted.
fn assign_lod_data(e: &mut Engine, node: u32, data: u32) {
    let old = e.mem.u32(node + LOD_DATA);
    if old == data {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(node + LOD_DATA, data);
    if data != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data.wrapping_add(4)]);
    }
}

/// Adds the string in the word at `slot` to the viewer strings: the array grown by its
/// grow-by count when full (`0096ad30`), then `SetAt` (`0096ae90`) at the old size.
fn add_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let list = strings.addr();
    let index = u32::from(e.mem.u16(list + 0xa));
    if index >= u32::from(e.mem.u16(list + 8)) {
        let grow = u32::from(e.mem.u16(list + 0xe));
        e.call(ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a99110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::UpdateWorldData` (Xbox PDB): `NiAVObject::UpdateWorldData`, then the LOD
/// data's slot +0x90 with the node, when there is LOD data.
pub fn ni_lod_node_update_world_data(e: &mut Engine, this: Ptr, data: Ptr) {
    e.call(AV_UPDATE_WORLD_DATA, &args![this, data]);
    let lod = e.mem.u32(this.addr() + LOD_DATA);
    if lod != 0 {
        e.vcall(lod, DATA_UPDATE_WORLD_DATA, &args![this]);
    }
}

// Translated from 00a99140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::OnVisible` (Xbox PDB; the culling process): when the LOD is active and there
/// is LOD data, the chosen child (+0xb0) is the data's index for the culler's camera
/// (culler +0xc; slot +0x8c with (camera, node)), or for the forced level (`011abdf4`, slot
/// +0x98) when that is not negative; then, while the index is not negative, it steps down
/// until it names a present child. Last, `NiSwitchNode::OnVisible`.
pub fn ni_lod_node_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let node = this.addr();
    let lod = e.mem.u32(node + LOD_DATA);
    if e.mem.u8(node + LOD_ACTIVE) != 0 && lod != 0 {
        let camera = e.mem.u32(culler.addr() + 0xc);
        let index = e
            .vcall(lod, DATA_CHILD_FOR_CAMERA, &args![camera, this])
            .u32();
        e.mem.set_u32(node + SWITCH_INDEX, index);
        let forced = e.mem.u32(FORCED_LOD_LEVEL);
        if forced as i32 >= 0 {
            let lod = e.mem.u32(node + LOD_DATA);
            let index = e.vcall(lod, DATA_CHILD_FOR_LEVEL, &args![forced]).u32();
            e.mem.set_u32(node + SWITCH_INDEX, index);
        }
        if e.mem.u32(node + SWITCH_INDEX) as i32 >= 0 {
            loop {
                let index = e.mem.u32(node + SWITCH_INDEX) as i32;
                let size = i32::from(e.mem.u16(node + CHILDREN_SIZE));
                if index < size {
                    let base = e.mem.u32(node + CHILDREN_BASE);
                    if e.mem.u32(base.wrapping_add((index as u32).wrapping_mul(4))) != 0 {
                        break;
                    }
                }
                let lower = index.wrapping_sub(1);
                e.mem.set_u32(node + SWITCH_INDEX, lower as u32);
                if lower < 0 {
                    break;
                }
            }
        }
    }
    e.call(SWITCH_ON_VISIBLE, &args![this, culler]);
}

// Translated from 00a991d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::UpdateAllChildren` (Xbox PDB; `thiscall`, no stack words): an update data
/// with the node's update time (+0xb4) and, as its controller flag, bit 1 of the flags byte
/// at +0xac (the other four flags clear; its three padding bytes are not written, here 0);
/// every present child whose revision (child revision array) differs from the node's
/// (+0xb8) gets the node's revision stored for it (`SetAt` on the array at +0xbc) and its
/// update (vtable +0xa4) with the data and 0.
pub fn ni_lod_node_update_all_children(e: &mut Engine, this: Ptr) {
    let node = this.addr();
    e.with_stack(0xc, |e, update| {
        let flags = e.mem.u8(node + 0xac);
        let time = x87_quiet_bits(e.mem.u32(node + 0xb4));
        e.mem.set_u32(update.addr(), time);
        e.mem.set_u8(update.addr() + 4, (flags >> 1) & 1);
        for flag in 5..9 {
            e.mem.set_u8(update.addr() + flag, 0);
        }
        let mut index = 0u32;
        while (index as i32) < i32::from(e.mem.u16(node + CHILDREN_SIZE)) {
            let base = e.mem.u32(node + CHILDREN_BASE);
            let child = e.mem.u32(base.wrapping_add(4 * index));
            if child != 0 {
                let revisions = e.mem.u32(node + 0xc0);
                let revision = e.mem.u32(revisions.wrapping_add(4 * index));
                if revision != e.mem.u32(node + 0xb8) {
                    e.call(ARRAY_SET_AT, &args![node + 0xbc, index, node + 0xb8]);
                    e.vcall(child, CHILD_UPDATE_SLOT, &args![update, 0u32]);
                }
            }
            index += 1;
        }
    });
}

// Translated from 00a99270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::LinkObject` (Xbox PDB): the base's `LinkObject` (`00a9a730`); from file
/// version 0x0a00010c on the LOD data is the next linked object
/// (`NiStream::GetObjectFromLinkID`, stored as a counted pointer).
pub fn ni_lod_node_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + 0xd8) < 0x0a00_010c {
        return;
    }
    let data = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_lod_data(e, this.addr(), data);
}

// Translated from 00a992e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::RegisterStreamables` (Xbox PDB): false when the base's
/// `RegisterStreamables` (`00a93c40`) is; otherwise the LOD data registers itself (slot
/// +0x54) when there is one, and true.
pub fn ni_lod_node_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let lod = e.mem.u32(this.addr() + LOD_DATA);
    if lod != 0 {
        e.vcall(lod, DATA_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

// Translated from 00a99310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::SaveBinary` (Xbox PDB): `NiSwitchNode::SaveBinary`, then the LOD data's link
/// (`NiStream::SaveLinkID`, vtable +0x30).
pub fn ni_lod_node_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SWITCH_SAVE_BINARY, &args![this, stream]);
    let lod = e.mem.u32(this.addr() + LOD_DATA);
    e.vcall(stream.addr(), STREAM_SAVE_LINK_ID_SLOT, &args![lod]);
}

// Translated from 00a99340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the LOD data; by its body the LOD data setter):
/// stores the data as a counted pointer at +0xcc (the old one released).
pub fn fn_00a99340(e: &mut Engine, this: Ptr, data: Ptr) {
    assign_lod_data(e, this.addr(), data.addr());
}

// Translated from 00a99390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (clone, cloning process); by its body the member
/// copy): the switch node's member copy (`00a93c00`), then the clone's LOD data is this
/// one's copy (slot +0x94, stored with `00a99340`), or, without LOD data, the clone's is
/// released and cleared.
pub fn fn_00a99390(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(SWITCH_COPY_MEMBERS, &args![this, clone, cloning]);
    let lod = e.mem.u32(this.addr() + LOD_DATA);
    if lod != 0 {
        let copy = e.vcall(lod, DATA_DUPLICATE, &args![]).u32();
        fn_00a99340(e, clone, Ptr::new(copy));
        return;
    }
    let theirs = e.mem.u32(clone.addr() + LOD_DATA);
    if theirs != 0 {
        release(e, theirs);
        e.mem.set_u32(clone.addr() + LOD_DATA, 0);
    }
}

// Translated from 00a99400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::IsEqual` (Xbox PDB): `NiSwitchNode::IsEqual`, then the LOD data both absent
/// or both present; when both are present and their RTTI records (slot +8) are the same,
/// also this one's `IsEqual` (slot +0x5c) with the other's. Data of different classes count
/// as equal.
pub fn ni_lod_node_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(SWITCH_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.mem.u32(this.addr() + LOD_DATA);
    let theirs = e.mem.u32(other.addr() + LOD_DATA);
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine == 0 {
        return true;
    }
    let my_class = e.vcall(mine, DATA_GET_RTTI, &args![]).u32();
    let their_class = e.vcall(theirs, DATA_GET_RTTI, &args![]).u32();
    if my_class == their_class {
        let mine = e.mem.u32(this.addr() + LOD_DATA);
        if !e.vcall(mine, DATA_IS_EQUAL, &args![theirs]).bool() {
            return false;
        }
    }
    true
}

// Translated from 00a99470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the destructor body
/// `00a99670` runs): the vtable, the LOD data released, the child revision array's base
/// vtable and its block freed (`00aa10f0`), then `NiNode`'s destructor body (`00a5e930`).
pub fn fn_00a99470(e: &mut Engine, this: Ptr) {
    let node = this.addr();
    e.mem.set_u32(node, NI_LOD_NODE_VTABLE);
    let lod = e.mem.u32(node + LOD_DATA);
    if lod != 0 {
        release(e, lod);
    }
    let block = e.mem.u32(node + 0xc0);
    e.mem.set_u32(node + 0xbc, REVISION_ARRAY_VTABLE);
    e.call(NI_FREE, &args![block]);
    e.call(NODE_DESTRUCT, &args![this]);
}

// Translated from 00a994d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::LoadBinary` (Xbox PDB): `NiSwitchNode::LoadBinary`; from file version
/// 0x0a00010c on, the LOD data's link (`00a645b0`) and nothing else. Older files hold the
/// ranges inline: a new `NiRangeLODData` (0x28 bytes, `00a97700`; null when the allocation
/// fails), its center (three words read through `NiColor::LoadBinary` into a local), the
/// range count (4 bytes; `SetNumRanges` for a non-zero count) and per range its near and
/// far distances (4 bytes each, through the x87 stack, the data grown by `SetNumRanges`
/// when the index reaches its count); then the data becomes the node's LOD data (the old
/// one released; the new one counted without a null check, as the game does).
pub fn ni_lod_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let node = this.addr();
    let s = stream.addr();
    e.call(SWITCH_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(s + 0xd8) > 0x0a00_010b {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        return;
    }
    let block = e.call(NI_NEW, &args![0x28u32]).u32();
    let data = if block != 0 {
        e.call(RANGE_DATA_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        let (component, count, near, far, center) = (f, f + 4, f + 8, f + 0xc, f + 0x10);
        e.call(POINT3_LOAD_BINARY, &args![center, stream]);
        for word in 0..3 {
            let value = e.mem.u32(center + 4 * word);
            e.mem.set_u32(data.wrapping_add(8 + 4 * word), value);
        }
        let read = |e: &mut Engine, destination: u32| {
            let input = e.mem.u32(s + 0x24c);
            let function = e.mem.u32(input + 8);
            e.mem.set_u32(component, 4);
            let done = e
                .call(function, &args![input, destination, 4u32, component, 1u32])
                .u32();
            let position = e.mem.u32(input + 4).wrapping_add(done);
            e.mem.set_u32(input + 4, position);
        };
        read(e, count);
        let ranges = e.mem.u32(count);
        if ranges == 0 {
            return;
        }
        e.call(RANGE_DATA_SET_NUM_RANGES, &args![data, ranges]);
        let mut offset = 0u32;
        let mut index = 0u32;
        while index < e.mem.u32(count) {
            read(e, near);
            read(e, far);
            let near_value = x87_quiet_bits(e.mem.u32(near));
            let far_value = x87_quiet_bits(e.mem.u32(far));
            if index >= e.mem.u32(data + 0x20) {
                e.call(RANGE_DATA_SET_NUM_RANGES, &args![data, index + 1]);
            }
            let list = e.mem.u32(data + 0x24);
            e.mem
                .set_u32(offset.wrapping_add(list), x87_quiet_bits(near_value));
            let list = e.mem.u32(data + 0x24);
            e.mem.set_u32(
                offset.wrapping_add(list).wrapping_add(4),
                x87_quiet_bits(far_value),
            );
            index += 1;
            offset = offset.wrapping_add(0x10);
        }
    });
    let old = e.mem.u32(node + LOD_DATA);
    if old == data {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(node + LOD_DATA, data);
    e.call(INTERLOCKED_INCREMENT, &args![data.wrapping_add(4)]);
}

/// The fields every new LOD node gets after the switch node's constructor: the vtable, no
/// LOD data, child index 0, LOD active.
fn initialize(e: &mut Engine, node: u32) {
    e.call(SWITCH_CONSTRUCT, &args![node]);
    e.mem.set_u32(node, NI_LOD_NODE_VTABLE);
    e.mem.set_u32(node + LOD_DATA, 0);
    e.mem.set_u32(node + SWITCH_INDEX, 0);
    e.mem.set_u8(node + LOD_ACTIVE, 1);
}

// Translated from 00a996a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::CreateClone` (Xbox PDB): a new node as `00a99710` makes it (0xd4 bytes), then
/// the member copy (`00a99390`) into it; when the allocation failed the copy still runs
/// with a null clone. Answers the clone.
pub fn ni_lod_node_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(NI_NEW, &args![0xd4u32]).u32();
    if clone != 0 {
        initialize(e, clone);
    }
    fn_00a99390(e, this, Ptr::new(clone), cloning);
    Ptr::new(clone)
}

// Translated from 00a99710 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiLODNode`): allocates 0xd4 bytes (`00aa13e0`) and, when that succeeds, runs
/// the switch node's constructor (`00a94360`) and sets the vtable, no LOD data, child index
/// 0 and the LOD active. Answers the node, or null.
pub fn fn_00a99710(e: &mut Engine) -> Ptr {
    let node = e.call(NI_NEW, &args![0xd4u32]).u32();
    if node == 0 {
        return Ptr::NULL;
    }
    initialize(e, node);
    Ptr::new(node)
}

// Translated from 00a99760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::GetViewerStrings` (Xbox PDB): after `NiSwitchNode::GetViewerStrings`, adds the
/// class name, then the LOD data's own strings (slot +0x60) or, without LOD data,
/// `"NULL LOD Data"`.
pub fn ni_lod_node_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(SWITCH_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_LOD_NODE_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let lod = e.mem.u32(this.addr() + LOD_DATA);
        if lod != 0 {
            e.vcall(lod, DATA_GET_VIEWER_STRINGS, &args![strings]);
            return;
        }
        e.mem.set_u32(slot.addr(), TEXT_NULL_LOD_DATA);
        add_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a994c0, ni_lod_node_get_rtti() -> Ptr),
        entry!(0x00a99670, ni_lod_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_9110, ni_lod_node_update_world_data(Ptr, Ptr)),
        entry!(0x00a9_9140, ni_lod_node_on_visible(Ptr, Ptr)),
        entry!(0x00a9_91d0, ni_lod_node_update_all_children(Ptr)),
        entry!(0x00a9_9270, ni_lod_node_link_object(Ptr, Ptr)),
        entry!(0x00a9_92e0, ni_lod_node_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_9310, ni_lod_node_save_binary(Ptr, Ptr)),
        entry!(0x00a9_9340, fn_00a99340(Ptr, Ptr)),
        entry!(0x00a9_9390, fn_00a99390(Ptr, Ptr, Ptr)),
        entry!(0x00a9_9400, ni_lod_node_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_9470, fn_00a99470(Ptr)),
        entry!(0x00a9_94d0, ni_lod_node_load_binary(Ptr, Ptr)),
        entry!(0x00a9_96a0, ni_lod_node_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_9710, fn_00a99710() -> Ptr),
        entry!(0x00a9_9760, ni_lod_node_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_lod_node_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a994c0, &args![]).u32(), NI_LOD_NODE_RTTI);
    }

    #[test]
    fn ni_lod_node_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_9470, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a99670, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a99670, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xd4]]);
        assert_eq!(calls_to(&e, 0x00a9_9470), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VTABLE: u32 = 0x0320_0000;
    const TARGETS: u32 = 0x0330_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x40).map(|i| TARGETS + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        quiet(&mut e, &[TARGETS + 4]);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, VTABLE);
        p
    }

    /// A LOD node with the given children and, optionally, LOD data.
    fn node(e: &mut Engine, children: &[u32], lod: u32) -> u32 {
        let n = object(e, 0xd4);
        let block = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *c);
        }
        e.mem.set_u32(n + CHILDREN_BASE, block);
        e.mem.set_u16(n + CHILDREN_SIZE, children.len() as u16);
        e.mem.set_u32(n + LOD_DATA, lod);
        n
    }

    #[test]
    fn update_world_data_runs_the_lod_data_slot() {
        let mut e = engine();
        quiet(
            &mut e,
            &[AV_UPDATE_WORLD_DATA, TARGETS + DATA_UPDATE_WORLD_DATA],
        );
        let lod = object(&mut e, 8);
        let n = node(&mut e, &[], lod);
        e.call(0x00a9_9110, &args![n, 0x11u32]);
        assert_eq!(calls(&e, AV_UPDATE_WORLD_DATA), vec![vec![n, 0x11]]);
        assert_eq!(
            calls(&e, TARGETS + DATA_UPDATE_WORLD_DATA),
            vec![vec![lod, n]]
        );
        let bare = node(&mut e, &[], 0);
        e.call(0x00a9_9110, &args![bare, 0x11u32]);
        assert_eq!(calls(&e, TARGETS + DATA_UPDATE_WORLD_DATA).len(), 1);
    }

    #[test]
    fn on_visible_picks_the_child_and_steps_down_to_a_present_one() {
        let mut e = engine();
        e.map(FORCED_LOD_LEVEL & !0xfff, 0x1000);
        e.mem.set_u32(FORCED_LOD_LEVEL, u32::MAX);
        quiet(&mut e, &[SWITCH_ON_VISIBLE]);
        let child = object(&mut e, 8);
        let lod = object(&mut e, 8);
        let n = node(&mut e, &[child, 0, 0], lod);
        e.mem.set_u8(n + LOD_ACTIVE, 1);
        let culler = e.mem.alloc(0x10);
        e.mem.set_u32(culler + 0xc, 0x0777_0000);
        // The camera index 5 is past the children: down to 0, the only present one.
        e.register(TARGETS + DATA_CHILD_FOR_CAMERA, |_, _| 5u32.into_ret());
        e.call(0x00a9_9140, &args![n, culler]);
        assert_eq!(
            calls(&e, TARGETS + DATA_CHILD_FOR_CAMERA),
            vec![vec![lod, 0x0777_0000, n]]
        );
        assert_eq!(e.mem.u32(n + SWITCH_INDEX), 0);
        assert_eq!(calls(&e, SWITCH_ON_VISIBLE), vec![vec![n, culler]]);
        // A forced level overrides the camera's index.
        e.mem.set_u32(FORCED_LOD_LEVEL, 2);
        e.register(TARGETS + DATA_CHILD_FOR_LEVEL, |_, _| 1u32.into_ret());
        e.mem.set_u32(e.mem.u32(n + CHILDREN_BASE) + 4, child);
        e.call(0x00a9_9140, &args![n, culler]);
        assert_eq!(
            calls(&e, TARGETS + DATA_CHILD_FOR_LEVEL),
            vec![vec![lod, 2]]
        );
        assert_eq!(e.mem.u32(n + SWITCH_INDEX), 1);
        // No present child at all: the index ends at -1.
        let empty = node(&mut e, &[0, 0], lod);
        e.mem.set_u8(empty + LOD_ACTIVE, 1);
        e.call(0x00a9_9140, &args![empty, culler]);
        assert_eq!(e.mem.u32(empty + SWITCH_INDEX), u32::MAX);
        // Inactive: the index is left alone.
        e.mem.set_u8(empty + LOD_ACTIVE, 0);
        e.mem.set_u32(empty + SWITCH_INDEX, 7);
        e.call(0x00a9_9140, &args![empty, culler]);
        assert_eq!(e.mem.u32(empty + SWITCH_INDEX), 7);
        assert_eq!(calls(&e, SWITCH_ON_VISIBLE).len(), 4);
    }

    #[test]
    fn update_all_children_updates_the_out_of_date_ones() {
        let mut e = engine();
        let (a, b) = (object(&mut e, 8), object(&mut e, 8));
        let n = node(&mut e, &[a, 0, b], 0);
        e.mem.set_f32(n + 0xb4, 1.5);
        e.mem.set_u8(n + 0xac, 0b10);
        e.mem.set_u32(n + 0xb8, 9);
        let revisions = e.mem.alloc(0xc);
        e.mem.set_u32(revisions, 9);
        e.mem.set_u32(revisions + 8, 3);
        e.mem.set_u32(n + 0xc0, revisions);
        quiet(&mut e, &[ARRAY_SET_AT]);
        let seen = Rc::new(RefCell::new(vec![]));
        let s = seen.clone();
        e.register_double(TARGETS + CHILD_UPDATE_SLOT, move |e, a| {
            s.borrow_mut().push((a[0], e.mem.bytes(a[1], 9), a[2]));
            Ret::default()
        });
        e.call(0x00a9_91d0, &args![n]);
        let mut want = 1.5f32.to_le_bytes().to_vec();
        want.extend([1, 0, 0, 0, 0]);
        assert_eq!(*seen.borrow(), vec![(b, want, 0)]);
        assert_eq!(calls(&e, ARRAY_SET_AT), vec![vec![n + 0xbc, 2, n + 0xb8]]);
    }

    #[test]
    fn link_object_takes_the_lod_data_from_new_files() {
        let mut e = engine();
        quiet(&mut e, &[BASE_LINK_OBJECT]);
        let old = object(&mut e, 8);
        e.mem.set_u32(old + 4, 1);
        let new = object(&mut e, 8);
        let n = node(&mut e, &[], old);
        let stream = e.mem.alloc(0x100);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| new.into_ret());
        e.mem.set_u32(stream + 0xd8, 0x0a00_010b);
        e.call(0x00a9_9270, &args![n, stream]);
        assert_eq!(e.mem.u32(n + LOD_DATA), old);
        e.mem.set_u32(stream + 0xd8, 0x0a00_010c);
        e.call(0x00a9_9270, &args![n, stream]);
        assert_eq!(e.mem.u32(n + LOD_DATA), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls(&e, TARGETS + 4), vec![vec![old]]);
        assert_eq!(calls(&e, BASE_LINK_OBJECT).len(), 2);
    }

    #[test]
    fn register_streamables_includes_the_lod_data() {
        let mut e = engine();
        quiet(&mut e, &[TARGETS + DATA_REGISTER_STREAMABLES]);
        let lod = object(&mut e, 8);
        let n = node(&mut e, &[], lod);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_92e0, &args![n, 0x22u32]).bool());
        e.register(BASE_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a9_92e0, &args![n, 0x22u32]).bool());
        assert_eq!(
            calls(&e, TARGETS + DATA_REGISTER_STREAMABLES),
            vec![vec![lod, 0x22]]
        );
        let bare = node(&mut e, &[], 0);
        assert!(e.call(0x00a9_92e0, &args![bare, 0x22u32]).bool());
    }

    #[test]
    fn save_binary_links_the_lod_data() {
        let mut e = engine();
        quiet(
            &mut e,
            &[SWITCH_SAVE_BINARY, TARGETS + STREAM_SAVE_LINK_ID_SLOT],
        );
        let lod = object(&mut e, 8);
        let n = node(&mut e, &[], lod);
        let stream = object(&mut e, 0x100);
        e.call(0x00a9_9310, &args![n, stream]);
        assert_eq!(calls(&e, SWITCH_SAVE_BINARY), vec![vec![n, stream]]);
        assert_eq!(
            calls(&e, TARGETS + STREAM_SAVE_LINK_ID_SLOT),
            vec![vec![stream, lod]]
        );
    }

    #[test]
    fn lod_data_setter_counts_and_releases() {
        let mut e = engine();
        let old = object(&mut e, 8);
        e.mem.set_u32(old + 4, 2);
        let n = node(&mut e, &[], old);
        e.call(0x00a9_9340, &args![n, old]);
        assert_eq!(e.mem.u32(old + 4), 2);
        let new = object(&mut e, 8);
        e.call(0x00a9_9340, &args![n, new]);
        assert_eq!((e.mem.u32(old + 4), e.mem.u32(new + 4)), (1, 1));
        e.call(0x00a9_9340, &args![n, 0u32]);
        assert_eq!(e.mem.u32(n + LOD_DATA), 0);
        assert_eq!(calls(&e, TARGETS + 4), vec![vec![new]]);
    }

    #[test]
    fn member_copy_duplicates_or_clears_the_lod_data() {
        let mut e = engine();
        quiet(&mut e, &[SWITCH_COPY_MEMBERS]);
        let copy = object(&mut e, 8);
        e.register_double(TARGETS + DATA_DUPLICATE, move |_, _| copy.into_ret());
        let lod = object(&mut e, 8);
        let this = node(&mut e, &[], lod);
        let clone = node(&mut e, &[], 0);
        e.call(0x00a9_9390, &args![this, clone, 0x44u32]);
        assert_eq!(
            calls(&e, SWITCH_COPY_MEMBERS),
            vec![vec![this, clone, 0x44]]
        );
        assert_eq!(e.mem.u32(clone + LOD_DATA), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
        let bare = node(&mut e, &[], 0);
        e.call(0x00a9_9390, &args![bare, clone, 0x44u32]);
        assert_eq!(e.mem.u32(clone + LOD_DATA), 0);
        assert_eq!(calls(&e, TARGETS + 4), vec![vec![copy]]);
    }

    #[test]
    fn is_equal_compares_lod_data_of_the_same_class_only() {
        let mut e = engine();
        e.register(SWITCH_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(TARGETS + DATA_GET_RTTI, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TARGETS + DATA_IS_EQUAL, |_, _| 0u32.into_ret());
        let (a, b, c) = (object(&mut e, 8), object(&mut e, 8), object(&mut e, 8));
        // The test reads each data's class from its second word.
        e.mem.set_u32(a + 4, 1);
        e.mem.set_u32(b + 4, 1);
        e.mem.set_u32(c + 4, 2);
        let eq = |e: &mut Engine, x: u32, y: u32| e.call(0x00a9_9400, &args![x, y]).bool();
        let (na, nb, nc) = (
            node(&mut e, &[], a),
            node(&mut e, &[], b),
            node(&mut e, &[], c),
        );
        let (none, none2) = (node(&mut e, &[], 0), node(&mut e, &[], 0));
        assert!(!eq(&mut e, na, nb));
        assert_eq!(calls(&e, TARGETS + DATA_IS_EQUAL), vec![vec![a, b]]);
        assert!(eq(&mut e, na, nc));
        assert!(!eq(&mut e, na, none));
        assert!(!eq(&mut e, none, na));
        assert!(eq(&mut e, none, none2));
        e.register(SWITCH_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!eq(&mut e, none, none2));
    }

    #[test]
    fn destructor_body_releases_frees_and_runs_the_node_destructor() {
        let mut e = engine();
        quiet(&mut e, &[NI_FREE, NODE_DESTRUCT]);
        let lod = object(&mut e, 8);
        e.mem.set_u32(lod + 4, 1);
        let n = node(&mut e, &[], lod);
        e.mem.set_u32(n + 0xc0, 0x0123_0000);
        e.call(0x00a9_9470, &args![n]);
        assert_eq!(e.mem.u32(n), NI_LOD_NODE_VTABLE);
        assert_eq!(e.mem.u32(n + 0xbc), REVISION_ARRAY_VTABLE);
        assert_eq!(calls(&e, TARGETS + 4), vec![vec![lod]]);
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x0123_0000]]);
        assert_eq!(calls(&e, NODE_DESTRUCT), vec![vec![n]]);
    }

    #[test]
    fn load_binary_reads_ranges_from_old_files_and_links_new_ones() {
        let mut e = engine();
        quiet(&mut e, &[SWITCH_LOAD_BINARY, STREAM_READ_LINK_ID]);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0xd8, 0x0a00_010c);
        let n = node(&mut e, &[], 0);
        e.call(0x00a9_94d0, &args![n, stream]);
        assert_eq!(calls(&e, STREAM_READ_LINK_ID), vec![vec![stream]]);
        assert_eq!(e.mem.u32(n + LOD_DATA), 0);
        // Old file: center, two ranges (near, far).
        e.mem.set_u32(stream + 0xd8, 0x0a00_010b);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0340_0000);
        let words = Rc::new(RefCell::new(vec![
            2u32,
            1.0f32.to_bits(),
            2.0f32.to_bits(),
            3.0f32.to_bits(),
            0x7f80_0001,
        ]));
        let w = words.clone();
        e.register_double(0x0340_0000, move |e, a| {
            let word = w.borrow_mut().remove(0);
            e.mem.set_u32(a[1], word);
            4u32.into_ret()
        });
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(RANGE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(RANGE_DATA_SET_NUM_RANGES, |e, a| {
            let block = e.mem.alloc(0x10 * a[1]);
            e.mem.set_u32(a[0] + 0x24, block);
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.register(POINT3_LOAD_BINARY, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            Ret::default()
        });
        e.call(0x00a9_94d0, &args![n, stream]);
        let data = e.mem.u32(n + LOD_DATA);
        assert_eq!(e.mem.u32(data + 4), 1);
        let center: Vec<u32> = (0..3).map(|i| e.mem.u32(data + 8 + 4 * i)).collect();
        assert_eq!(center, vec![0x100, 0x101, 0x102]);
        assert_eq!(calls(&e, RANGE_DATA_SET_NUM_RANGES), vec![vec![data, 2]]);
        let ranges = e.mem.u32(data + 0x24);
        assert_eq!(
            (
                e.mem.f32(ranges),
                e.mem.f32(ranges + 4),
                e.mem.f32(ranges + 0x10)
            ),
            (1.0, 2.0, 3.0)
        );
        // The far distance went through the x87 stack: a signalling NaN is quieted.
        assert_eq!(e.mem.u32(ranges + 0x14), 0x7fc0_0001);
        assert_eq!(e.mem.u32(input + 4), 20);
    }

    #[test]
    fn create_and_clone_build_an_active_lod_node() {
        let mut e = engine();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        quiet(&mut e, &[SWITCH_CONSTRUCT, SWITCH_COPY_MEMBERS]);
        let created = e.call(0x00a9_9710, &args![]).u32();
        assert_eq!(e.mem.u32(created), NI_LOD_NODE_VTABLE);
        assert_eq!(e.mem.u8(created + LOD_ACTIVE), 1);
        assert_eq!(calls(&e, NI_NEW), vec![vec![0xd4]]);
        let this = node(&mut e, &[], 0);
        let clone = e.call(0x00a9_96a0, &args![this, 0x55u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_LOD_NODE_VTABLE);
        assert_eq!(
            calls(&e, SWITCH_COPY_MEMBERS),
            vec![vec![this, clone, 0x55]]
        );
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_9710, &args![]).u32(), 0);
        // (A failed allocation in `CreateClone` makes the member copy read through a null
        // clone, as the game does; that path is not run here.)
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_data_strings_or_a_note() {
        let mut e = engine();
        e.map(NI_LOD_NODE_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(NI_LOD_NODE_RTTI, 0x0abc_0000);
        quiet(
            &mut e,
            &[
                SWITCH_GET_VIEWER_STRINGS,
                ARRAY_SET_SIZE,
                TARGETS + DATA_GET_VIEWER_STRINGS,
            ],
        );
        e.register(VIEWER_STRING_NAME, |_, _| 0x61u32.into_ret());
        let added = Rc::new(RefCell::new(vec![]));
        let a = added.clone();
        e.register_double(ARRAY_SET_AT, move |e, w| {
            a.borrow_mut().push(e.mem.u32(w[2]));
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        let lod = object(&mut e, 8);
        let n = node(&mut e, &[], lod);
        e.call(0x00a9_9760, &args![n, strings]);
        assert_eq!(
            calls(&e, TARGETS + DATA_GET_VIEWER_STRINGS),
            vec![vec![lod, strings]]
        );
        let bare = node(&mut e, &[], 0);
        e.call(0x00a9_9760, &args![bare, strings]);
        assert_eq!(*added.borrow(), vec![0x61, 0x61, TEXT_NULL_LOD_DATA]);
        assert_eq!(calls(&e, VIEWER_STRING_NAME)[0], vec![0x0abc_0000]);
    }
}
