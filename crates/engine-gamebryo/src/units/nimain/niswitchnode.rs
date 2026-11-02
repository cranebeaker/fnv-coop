//! `NiMain/niswitchnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiSwitchNode` is an `NiNode` (children array at +0x9c on PC) that shows
//! one child, the one at `m_iIndex`. It adds a flag word (+0xac), the index
//! (+0xb0), the time of the last update (+0xb4), a revision counter (+0xb8)
//! and an array of per-child revisions (+0xbc, an `NiTPrimitiveArray<uint>`)
//! that records for each child which revision of the switch it was last
//! updated at. Bit 0 of the flag word is `m_bUpdateOnlyActive`: with it set
//! only the selected child is updated, culled and used for the bound; bit 1
//! is set by the constructor, the update passes and the loader (the Xbox PDB
//! has no name for it; it is copied from the update data's controller flag by
//! `UpdateDownwardPass`).
//!
//! The compiler's exception-unwinding frames are not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The null bound (`NiAVObject::NullBoundS`, by use): four words, used when
/// the selected child has no world bound.
const NULL_BOUND: u32 = 0x011f_4288;
/// `NiSwitchNode`'s vtable.
const SWITCH_NODE_VTABLE: u32 = 0x010a_001c;
/// The child-revision array's vtable (RTTI: `NiTPrimitiveArray<unsigned
/// int>`) and the base class's one (`NiTArray<unsigned int,
/// NiTMallocInterface<unsigned int>>`) that the constructor and destructor
/// set while the array is being built or taken apart.
const REVISION_ARRAY_VTABLE: u32 = 0x0101_a3ac;
const REVISION_ARRAY_BASE_VTABLE: u32 = 0x0101_a3a4;
/// `NiSwitchNode`'s `NiRTTI` (`GetRTTI` returns its address; its first word
/// is the class name `NiGetViewerString` takes).
const SWITCH_NODE_RTTI: u32 = 0x011f_5eb4;
/// `"m_iIndex"` and `"m_bUpdateOnlyActive"`, the names of the viewer strings.
const NAME_INDEX: u32 = 0x0109_fff8;
const NAME_UPDATE_ONLY_ACTIVE: u32 = 0x010a_011c;

/// `NiNode::DetachChild` (the overload with the out pointer) and
/// `NiNode::DetachChildAt`'s.
const NI_NODE_DETACH_CHILD_OV2: u32 = 0x00a5_e780;
const NI_NODE_DETACH_CHILD_AT_OV2: u32 = 0x00a5_e690;
/// `NiNode::UpdateWorldBound`, `IsEqual`, `UpdateControllers`,
/// `UpdateDownwardPass`, `UpdateSelectedDownwardPass`,
/// `UpdateRigidDownwardPass`, `AttachChild`, `SetAt`, `LoadBinary`,
/// `SaveBinary`, `GetViewerStrings`.
const NI_NODE_UPDATE_WORLD_BOUND: u32 = 0x00a5_e2d0;
const NI_NODE_IS_EQUAL: u32 = 0x00a5_e600;
const NI_NODE_UPDATE_CONTROLLERS: u32 = 0x00a5_e280;
const NI_NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
const NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS: u32 = 0x00a5_ded0;
const NI_NODE_UPDATE_RIGID_DOWNWARD_PASS: u32 = 0x00a5_e1f0;
const NI_NODE_ATTACH_CHILD: u32 = 0x00a5_ed10;
const NI_NODE_SET_AT: u32 = 0x00a5_ee30;
const NI_NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
const NI_NODE_SAVE_BINARY: u32 = 0x00a5_e880;
const NI_NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
/// `NiNode` constructor (one word, 0 here), destructor body, and the member
/// copy `CreateClone` runs (unnamed in the engine map; `this` is the source,
/// the arguments the new node and the cloning process).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NI_NODE_DESTRUCT: u32 = 0x00a5_e930;
const NI_NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;
/// `NiAVObject::UpdateObjectControllers` (this, update data, 1).
const UPDATE_OBJECT_CONTROLLERS: u32 = 0x004e_ff50;
/// `NiAVObject::Cull` (this, culling process).
const NI_AV_OBJECT_CULL: u32 = 0x00a5_9e00;
/// Methods of the `NiTArray<unsigned int>` base: `SetAt(index, &value)`,
/// the growing step `(new capacity)` and `RemoveAt(index)`.
const REVISION_ARRAY_SET_AT: u32 = 0x0096_ae90;
const REVISION_ARRAY_GROW: u32 = 0x0096_ad30;
const REVISION_ARRAY_REMOVE_AT: u32 = 0x00a9_3c50;
/// `NiGetViewerString(class name)`, `(name, int)` and `(name, bool)`.
const VIEWER_STRING_RTTI: u32 = 0x00a7_d330;
const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// Allocation helpers (cdecl): `operator new(size)`, byte allocation,
/// byte free, `operator delete(block, size)`.
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const ALLOCATE_BYTES: u32 = 0x00aa_1070;
const FREE_BYTES: u32 = 0x00aa_10f0;
const OPERATOR_DELETE: u32 = 0x00aa_1460;
/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Vtable slots of the children (byte offsets).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_UPDATE_CONTROLLERS: u32 = 0x8c;
const SLOT_UPDATE_DOWNWARD_PASS: u32 = 0xa4;
const SLOT_UPDATE_SELECTED_DOWNWARD_PASS: u32 = 0xa8;
const SLOT_UPDATE_RIGID_DOWNWARD_PASS: u32 = 0xac;
const SLOT_UPDATE_WORLD_DATA: u32 = 0xb8;

layout! {
    /// `NiSwitchNode` (Xbox PDB), 0xcc bytes on PC (0xf0 on the Xbox, whose
    /// `NiNode` fields sit higher). The `NiNode` and `NiAVObject` fields it
    /// uses are declared here at their PC offsets.
    pub struct NiSwitchNode: 0xCC {
        /// `m_kWorldBound` (Xbox PDB, `NiAVObject`): `BSNiBound*`, four words.
        0x20 m_kWorldBound: u32,
        /// `m_uFlags` (Xbox PDB, `NiAVObject`): bit 1 is set on a child that
        /// takes part in selective updates and bit 3 selects the controller
        /// flag of the selected update passes (names not in the PDB).
        0x30 m_uFlagsAV: u32,
        /// `m_kChildren` (Xbox PDB, `NiNode`): an `NiTObjectArray` with the
        /// `NiTArray` layout.
        0x9C m_kChildren: Inline<NiTArray>,
        /// `m_uFlags` (Xbox PDB): bit 0 is `m_bUpdateOnlyActive`.
        0xAC m_uFlags: u16,
        /// `m_iIndex` (Xbox PDB): selected child, -1 for none.
        0xB0 m_iIndex: i32,
        /// `m_fSavedTime` (Xbox PDB).
        0xB4 m_fSavedTime: f32,
        /// `m_uiRevID` (Xbox PDB).
        0xB8 m_uiRevID: u32,
        /// `m_kChildRevID` (Xbox PDB): `NiTPrimitiveArray<unsigned int>`.
        0xBC m_kChildRevID: Inline<NiTArray>,
    }
}

fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).i32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &[]);
    }
}

/// The child at `index` of the children array (no bounds check).
fn child_at(e: &Engine, this: Ptr<NiSwitchNode>, index: i32) -> u32 {
    let children = this.at(NiSwitchNode::m_kChildren);
    let base = e.get(children, NiTArray::m_pBase);
    e.mem.u32(base.wrapping_add((index as u32).wrapping_mul(4)))
}

/// The selected child, if the index is not negative and the slot is filled.
fn selected_child(e: &Engine, this: Ptr<NiSwitchNode>) -> Option<u32> {
    let index = e.get(this, NiSwitchNode::m_iIndex);
    if index < 0 {
        return None;
    }
    match child_at(e, this, index) {
        0 => None,
        child => Some(child),
    }
}

/// Stores the current revision as the selected child's revision
/// (`m_kChildRevID.SetAt(index, &m_uiRevID)`).
fn record_revision(e: &mut Engine, this: Ptr<NiSwitchNode>, index: i32) {
    let array = this.at(NiSwitchNode::m_kChildRevID);
    e.call(
        REVISION_ARRAY_SET_AT,
        &args![array, index, this.addr() + NiSwitchNode::m_uiRevID.off],
    );
}

/// `m_kChildRevID.SetAt(index, &0)`, growing the array first when `index`
/// is past its capacity: the revision slot a new child gets.
fn add_empty_revision(e: &mut Engine, this: Ptr<NiSwitchNode>, index: u32, capacity: u16) {
    let array = this.at(NiSwitchNode::m_kChildRevID);
    if index >= capacity as u32 {
        let grow_by = e.get(array, NiTArray::m_usGrowBy) as u32;
        e.call(REVISION_ARRAY_GROW, &args![array, grow_by + index]);
    }
    let zero = e.mem.alloc(4);
    e.call(REVISION_ARRAY_SET_AT, &args![array, index, zero]);
    e.mem.free(zero);
}

/// Copies a `BSNiBound` (four words; the last one through a `float`).
fn copy_bound(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..3 {
        let value = e.mem.u32(source + word * 4);
        e.mem.set_u32(destination + word * 4, value);
    }
    let radius = e.mem.f32(source + 12);
    e.mem.set_f32(destination + 12, radius);
}

/// The selected child's world bound (the null bound if it has none) copied
/// into the switch node's own.
fn take_bound_from(e: &mut Engine, this: Ptr<NiSwitchNode>, child: u32) {
    let mut source = e.mem.u32(child + NiSwitchNode::m_kWorldBound.off);
    if source == 0 {
        source = NULL_BOUND;
    }
    let destination = e.get(this, NiSwitchNode::m_kWorldBound);
    copy_bound(e, destination, source);
}

/// The index fix-up and result hand-over `DetachChild` and `DetachChildAt`
/// end with: `m_iIndex` becomes -1 if the child it named is gone, and the
/// `NiPointer` at `out` takes the detached child (`detached`, which the
/// local smart pointer then releases).
fn finish_detach(e: &mut Engine, this: Ptr<NiSwitchNode>, out: u32, detached: u32) {
    let index = e.get(this, NiSwitchNode::m_iIndex);
    if index > -1 {
        let children = this.at(NiSwitchNode::m_kChildren);
        let size = e.get(children, NiTArray::m_usSize) as i32;
        if index >= size || child_at(e, this, index) == 0 {
            e.set(this, NiSwitchNode::m_iIndex, -1);
        }
    }
    let held = e.mem.u32(out);
    if held != detached {
        if held != 0 {
            release(e, held);
        }
        e.mem.set_u32(out, detached);
        if detached == 0 {
            return;
        }
        e.call(INTERLOCKED_INCREMENT, &args![detached + 4]);
    }
    if detached != 0 {
        release(e, detached);
    }
}

// Translated from 00a93ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::DetachChild` (Xbox PDB): bumps the revision, removes the
/// child's slot from the revision array (inlined `RemoveAt`) when the
/// child is found, detaches it through `NiNode::DetachChild`, drops a
/// selected index that no longer names a child, and stores the detached
/// child in the `NiPointer` at `out`.
pub fn ni_switch_node_detach_child(e: &mut Engine, this: Ptr<NiSwitchNode>, child: u32, out: u32) {
    e.set(this, NiSwitchNode::m_uiRevID, 1);
    let children = this.at(NiSwitchNode::m_kChildren);
    let size = e.get(children, NiTArray::m_usSize) as u32;
    let base = e.get(children, NiTArray::m_pBase);
    for index in 0..size {
        if e.mem.u32(base + index * 4) == child {
            let revisions = this.at(NiSwitchNode::m_kChildRevID);
            let revision_size = e.get(revisions, NiTArray::m_usSize) as u32;
            if index < revision_size {
                let slot = e.get(revisions, NiTArray::m_pBase) + index * 4;
                let old = e.mem.u32(slot);
                e.mem.set_u32(slot, 0);
                if old != 0 {
                    let count = e.get(revisions, NiTArray::m_usESize);
                    e.set(revisions, NiTArray::m_usESize, count.wrapping_sub(1));
                }
                let last = (revision_size as u16).wrapping_sub(1);
                if index == last as u32 {
                    e.set(revisions, NiTArray::m_usSize, last);
                }
            }
            break;
        }
    }
    // The exe's frame is its three pushes (0xc bytes) and the return address; it hands
    // `NiNode::DetachChild` its own `child` argument slot (+0x10, 00a93d1e) for the detached
    // pointer, so the frame reaches over the arguments as the caller pushed them.
    e.with_stack(0x18, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x10, 0); // the exe clears the slot before the call (00a93d26)
        e.mem.set_u32(frame + 0x14, out);
        let local = frame + 0x10;
        e.call(NI_NODE_DETACH_CHILD_OV2, &args![this, child, local]);
        let detached = e.mem.u32(local);
        finish_detach(e, this, out, detached);
    });
}

// Translated from 00a93dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::DetachChildAt` (Xbox PDB): as [`ni_switch_node_detach_child`]
/// for the child at `index` (the revision slot is removed through
/// `RemoveAt`, `00a93c50`, when `index` is below the children count).
pub fn ni_switch_node_detach_child_at(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    index: u32,
    out: u32,
) {
    e.set(this, NiSwitchNode::m_uiRevID, 1);
    let children = this.at(NiSwitchNode::m_kChildren);
    let size = e.get(children, NiTArray::m_usSize) as u32;
    if index < size {
        let revisions = this.at(NiSwitchNode::m_kChildRevID);
        e.call(REVISION_ARRAY_REMOVE_AT, &args![revisions, index]);
    }
    // As in `DetachChild`: the detached pointer goes to the `index` argument slot (+0x10 of the
    // frame of three pushes and the return address, 00a93dfa).
    e.with_stack(0x18, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x10, 0); // cleared before the call (00a93e02)
        e.mem.set_u32(frame + 0x14, out);
        let local = frame + 0x10;
        e.call(NI_NODE_DETACH_CHILD_AT_OV2, &args![this, index, local]);
        let detached = e.mem.u32(local);
        finish_detach(e, this, out, detached);
    });
}

// Translated from 00a93ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::OnVisible` (Xbox PDB): if there is a selected child whose
/// recorded revision is not the current one, records it and updates the
/// child (`UpdateDownwardPass` with a stack update-data holding the saved
/// time and bit 1 of the flags); then culls the child with
/// `culling_process`. Nothing happens without a selected child.
pub fn ni_switch_node_on_visible(e: &mut Engine, this: Ptr<NiSwitchNode>, culling_process: u32) {
    let Some(child) = selected_child(e, this) else {
        return;
    };
    let index = e.get(this, NiSwitchNode::m_iIndex);
    let revisions = this.at(NiSwitchNode::m_kChildRevID);
    let revision_base = e.get(revisions, NiTArray::m_pBase);
    let recorded = e.mem.u32(revision_base + index as u32 * 4);
    if recorded != e.get(this, NiSwitchNode::m_uiRevID) {
        record_revision(e, this, index);
        let flags = e.get(this, NiSwitchNode::m_uFlags);
        let time = e.get(this, NiSwitchNode::m_fSavedTime);
        e.with_stack(12, |e, data| {
            // Update data: time, the controller flag (bit 1 of the switch
            // flags), then four zero bytes.
            e.mem.set_f32(data.addr(), time);
            e.mem.set_u8(data.addr() + 4, (flags >> 1) as u8 & 1);
            e.vcall(child, SLOT_UPDATE_DOWNWARD_PASS, &args![data, 0u32]);
        });
    }
    e.call(NI_AV_OBJECT_CULL, &args![child, culling_process]);
}

// Translated from 00a93f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::UpdateWorldBound` (Xbox PDB): without
/// `m_bUpdateOnlyActive` it is `NiNode::UpdateWorldBound`. With it, the
/// node's world bound is the selected child's (or the null bound if that
/// child has none); with no selected child the bound's radius is set to 0.
pub fn ni_switch_node_update_world_bound(e: &mut Engine, this: Ptr<NiSwitchNode>) {
    if e.get(this, NiSwitchNode::m_uFlags) & 1 == 0 {
        e.call(NI_NODE_UPDATE_WORLD_BOUND, &args![this]);
        return;
    }
    match selected_child(e, this) {
        Some(child) => take_bound_from(e, this, child),
        None => {
            let bound = e.get(this, NiSwitchNode::m_kWorldBound);
            e.mem.set_f32(bound + 12, 0.0);
        }
    }
}

// Translated from 00a94000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::IsEqual` (Xbox PDB): `NiNode::IsEqual`, and the same
/// selected index and the same `m_bUpdateOnlyActive`.
pub fn ni_switch_node_is_equal(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    other: Ptr<NiSwitchNode>,
) -> bool {
    if !e.call(NI_NODE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(this, NiSwitchNode::m_iIndex) != e.get(other, NiSwitchNode::m_iIndex) {
        return false;
    }
    (e.get(other, NiSwitchNode::m_uFlags) ^ e.get(this, NiSwitchNode::m_uFlags)) & 1 == 0
}

/// What the three update passes do after the node's own bookkeeping, once
/// the selected child has been updated: records the revision and takes the
/// child's bound.
fn after_child_update(e: &mut Engine, this: Ptr<NiSwitchNode>, index: i32, child: u32) {
    record_revision(e, this, index);
    take_bound_from(e, this, child);
}

// Translated from 00a94040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::UpdateDownwardPass` (Xbox PDB): sets bit 1 of the flags
/// from the update data's controller flag (byte +4) and saves its time
/// (+0). Without `m_bUpdateOnlyActive` it continues as
/// `NiNode::UpdateDownwardPass`. Otherwise it bumps the revision, updates the
/// node's controllers if the data asks, updates its world data (slot
/// `0xb8`), and updates the selected child (slot `0xa4`) with the same data
/// and `flags`, records the revision and takes the child's bound.
pub fn ni_switch_node_update_downward_pass(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    update_data: u32,
    flags: u32,
) {
    let mut node_flags = e.get(this, NiSwitchNode::m_uFlags);
    if e.mem.u8(update_data + 4) != 0 {
        node_flags |= 2;
    } else {
        node_flags &= 0xfffd;
    }
    e.set(this, NiSwitchNode::m_uFlags, node_flags);
    let time = e.mem.f32(update_data);
    e.set(this, NiSwitchNode::m_fSavedTime, time);
    if node_flags & 1 == 0 {
        e.call(
            NI_NODE_UPDATE_DOWNWARD_PASS,
            &args![this, update_data, flags],
        );
        return;
    }
    let revision = e.get(this, NiSwitchNode::m_uiRevID);
    e.set(this, NiSwitchNode::m_uiRevID, revision.wrapping_add(1));
    if e.mem.u8(update_data + 4) != 0 {
        e.call(UPDATE_OBJECT_CONTROLLERS, &args![this, update_data, 1u32]);
    }
    e.vcall(this.addr(), SLOT_UPDATE_WORLD_DATA, &args![update_data]);
    if let Some(child) = selected_child(e, this) {
        e.vcall(child, SLOT_UPDATE_DOWNWARD_PASS, &args![update_data, flags]);
        let index = e.get(this, NiSwitchNode::m_iIndex);
        after_child_update(e, this, index, child);
    }
}

/// The part `UpdateSelectedDownwardPass` and `UpdateRigidDownwardPass`
/// share when `m_bUpdateOnlyActive` is set: bump the revision, update the
/// node's controllers with the update data's flag byte temporarily set from
/// bit 3 of the node's `m_uFlags`, update its world data, and then, for a
/// selected child, run `child_pass` on it (if the child has bit 1 of its
/// flags) before recording the revision and taking the bound.
fn selected_or_rigid_pass(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    update_data: u32,
    flags: u32,
    rigid_only: bool,
) {
    let revision = e.get(this, NiSwitchNode::m_uiRevID);
    e.set(this, NiSwitchNode::m_uiRevID, revision.wrapping_add(1));
    let controller_flag = e.mem.u8(update_data + 4);
    let node_flags = e.get(this, NiSwitchNode::m_uFlagsAV);
    e.mem.set_u8(update_data + 4, (node_flags >> 3) as u8 & 1);
    e.call(UPDATE_OBJECT_CONTROLLERS, &args![this, update_data, 1u32]);
    e.mem.set_u8(update_data + 4, controller_flag);
    e.vcall(this.addr(), SLOT_UPDATE_WORLD_DATA, &args![update_data]);
    if let Some(child) = selected_child(e, this) {
        let child_flags = e.mem.u32(child + NiSwitchNode::m_uFlagsAV.off);
        if (child_flags >> 1) & 1 != 0 {
            let slot = if rigid_only || (child_flags >> 4) & 1 != 0 {
                SLOT_UPDATE_RIGID_DOWNWARD_PASS
            } else {
                SLOT_UPDATE_SELECTED_DOWNWARD_PASS
            };
            e.vcall(child, slot, &args![update_data, flags]);
        }
        let index = e.get(this, NiSwitchNode::m_iIndex);
        after_child_update(e, this, index, child);
    }
}

// Translated from 00a94130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::UpdateSelectedDownwardPass` (Xbox PDB): saves the update
/// data's time; without `m_bUpdateOnlyActive` it is
/// `NiNode::UpdateSelectedDownwardPass`. Otherwise the pass of
/// [`selected_or_rigid_pass`], where the selected child (if it has bit 1 of
/// its `m_uFlags`) is updated through slot `0xac` if its bit 4 is set and
/// slot `0xa8` if not.
pub fn ni_switch_node_update_selected_downward_pass(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    update_data: u32,
    flags: u32,
) {
    let time = e.mem.f32(update_data);
    e.set(this, NiSwitchNode::m_fSavedTime, time);
    if e.get(this, NiSwitchNode::m_uFlags) & 1 == 0 {
        e.call(
            NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS,
            &args![this, update_data, flags],
        );
        return;
    }
    selected_or_rigid_pass(e, this, update_data, flags, false);
}

// Translated from 00a94220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::UpdateRigidDownwardPass` (Xbox PDB): as
/// [`ni_switch_node_update_selected_downward_pass`], but the child is always
/// updated through slot `0xac`.
pub fn ni_switch_node_update_rigid_downward_pass(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    update_data: u32,
    flags: u32,
) {
    let time = e.mem.f32(update_data);
    e.set(this, NiSwitchNode::m_fSavedTime, time);
    if e.get(this, NiSwitchNode::m_uFlags) & 1 == 0 {
        e.call(
            NI_NODE_UPDATE_RIGID_DOWNWARD_PASS,
            &args![this, update_data, flags],
        );
        return;
    }
    selected_or_rigid_pass(e, this, update_data, flags, true);
}

// Translated from 00a94300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::UpdateControllers` (Xbox PDB): without
/// `m_bUpdateOnlyActive` it is `NiNode::UpdateControllers`. With it, and a
/// non-negative index, it updates the node's own controllers and, if the
/// selected slot is filled, the selected child's (slot `0x8c`).
pub fn ni_switch_node_update_controllers(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    update_data: u32,
) {
    if e.get(this, NiSwitchNode::m_uFlags) & 1 == 0 {
        e.call(NI_NODE_UPDATE_CONTROLLERS, &args![this, update_data]);
        return;
    }
    let index = e.get(this, NiSwitchNode::m_iIndex);
    if index >= 0 {
        e.call(UPDATE_OBJECT_CONTROLLERS, &args![this, update_data, 1u32]);
        let child = child_at(e, this, index);
        if child != 0 {
            e.vcall(child, SLOT_UPDATE_CONTROLLERS, &args![update_data]);
        }
    }
}

// Translated from 00a94360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSwitchNode` constructor (unnamed in the engine map): the
/// `NiNode` constructor with 0, the vtable, no selected child (-1),
/// revision 1, flags 0, and an empty revision array of capacity 1 and
/// growth 1 with a 4-byte buffer; finally flags bits 0 and 1 set. Returns
/// `this`.
pub fn fn_00a94360(e: &mut Engine, this: Ptr<NiSwitchNode>) -> Ptr<NiSwitchNode> {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0u32]);
    e.set(this, NiSwitchNode::m_fSavedTime, 0.0);
    e.mem.set_u32(this.addr(), SWITCH_NODE_VTABLE);
    e.set(this, NiSwitchNode::m_iIndex, -1);
    e.set(this, NiSwitchNode::m_uiRevID, 1);
    e.set(this, NiSwitchNode::m_uFlags, 0);
    let revisions = this.at(NiSwitchNode::m_kChildRevID);
    e.set(revisions, NiTArray::m_usMaxSize, 1);
    e.mem.set_u32(revisions.addr(), REVISION_ARRAY_BASE_VTABLE);
    e.set(revisions, NiTArray::m_usGrowBy, 1);
    e.set(revisions, NiTArray::m_usSize, 0);
    e.set(revisions, NiTArray::m_usESize, 0);
    let buffer = e.call(ALLOCATE_BYTES, &args![4u32]).u32();
    e.set(revisions, NiTArray::m_pBase, buffer);
    e.mem.set_u32(revisions.addr(), REVISION_ARRAY_VTABLE);
    let flags = e.get(this, NiSwitchNode::m_uFlags);
    e.set(this, NiSwitchNode::m_uFlags, flags | 3);
    this
}

// Translated from 00a943f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::GetRTTI` (Xbox PDB): the class's `NiRTTI`.
pub fn ni_switch_node_get_rtti(_e: &mut Engine) -> u32 {
    SWITCH_NODE_RTTI
}

// Translated from 00a94400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::__scalar_deleting_destructor` (Xbox PDB): resets the
/// revision array's vtable to the base class's, frees its buffer, runs the
/// `NiNode` destructor body and, if bit 0 of `flags` is set, frees the 0xcc
/// bytes of the node. Returns `this`.
pub fn ni_switch_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    flags: u8,
) -> Ptr<NiSwitchNode> {
    let revisions = this.at(NiSwitchNode::m_kChildRevID);
    e.mem.set_u32(revisions.addr(), REVISION_ARRAY_BASE_VTABLE);
    let buffer = e.get(revisions, NiTArray::m_pBase);
    e.call(FREE_BYTES, &args![buffer]);
    e.call(NI_NODE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this, 0xccu32]);
    }
    this
}

// Translated from 00a94440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::AttachChild` (Xbox PDB): `NiNode::AttachChild` with the
/// same arguments, then bumps the revision to 1 and appends an empty
/// revision at the end of the revision array (growing it if it is full).
/// `first_available` is the byte flag `NiNode::AttachChild` tests to pick
/// its other store path (Gamebryo's `bFirstAvail`); it is passed on as is.
pub fn ni_switch_node_attach_child(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    child: u32,
    first_available: u32,
) {
    e.call(NI_NODE_ATTACH_CHILD, &args![this, child, first_available]);
    e.set(this, NiSwitchNode::m_uiRevID, 1);
    let revisions = this.at(NiSwitchNode::m_kChildRevID);
    let size = e.get(revisions, NiTArray::m_usSize) as u32;
    let capacity = e.get(revisions, NiTArray::m_usMaxSize);
    add_empty_revision(e, this, size, capacity);
}

// Translated from 00a944a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::SetAt` (Xbox PDB): bumps the revision to 1, writes an empty
/// revision at `index` (growing the revision array first if `index` is past
/// its capacity), then runs `NiNode::SetAt` with `index`, `child` (the new
/// child) and `previous` (the `NiPointer` that receives the child it
/// replaces); the last two are passed on unchanged.
pub fn ni_switch_node_set_at(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    index: u32,
    child: u32,
    previous: u32,
) {
    e.set(this, NiSwitchNode::m_uiRevID, 1);
    let revisions = this.at(NiSwitchNode::m_kChildRevID);
    let capacity = e.get(revisions, NiTArray::m_usMaxSize);
    add_empty_revision(e, this, index, capacity);
    e.call(NI_NODE_SET_AT, &args![this, index, child, previous]);
}

// Translated from 00a94500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::CreateClone` (Xbox PDB): allocates 0xcc bytes, constructs
/// a switch node in them ([`fn_00a94360`]), copies the `NiNode` members
/// into it (`00a5dc80`, source first) and copies the flags and the index.
/// Returns the clone. If the allocation fails the game goes on with a null
/// node and faults on the first member copy; that is kept (the copy is
/// called with 0).
pub fn ni_switch_node_create_clone(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    cloning_process: u32,
) -> Ptr<NiSwitchNode> {
    let memory = e.call(OPERATOR_NEW, &args![0xccu32]).ptr::<NiSwitchNode>();
    let clone = if memory.is_null() {
        memory
    } else {
        fn_00a94360(e, memory)
    };
    e.call(NI_NODE_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.get(this, NiSwitchNode::m_uFlags);
    e.set(clone, NiSwitchNode::m_uFlags, flags);
    let index = e.get(this, NiSwitchNode::m_iIndex);
    e.set(clone, NiSwitchNode::m_iIndex, index);
    clone
}

/// Reads or writes `size` bytes at `field` through the binary stream object's
/// function word at `slot` (+8 read, +0xC write; the call the loader and saver
/// make: the stream, the buffer, the byte count, the address of a one-element
/// component-size list holding the same count, and the element count 1) and
/// adds the returned byte count to the stream's running total (+4).
fn stream_transfer(e: &mut Engine, stream: u32, slot: u32, field: u32, size: u32) {
    let component_size = e.mem.alloc(4);
    e.mem.set_u32(component_size, size);
    // The function is a word of the stream object itself (00a945b0: `MOV EAX,[ESI+0x8]`, then
    // `CALL EAX` with the object first, cdecl), not a virtual slot.
    let function = e.mem.u32(stream + slot);
    let transferred = e
        .call(function, &args![stream, field, size, component_size, 1u32])
        .u32();
    let total = e.mem.u32(stream + 4);
    e.mem.set_u32(stream + 4, total.wrapping_add(transferred));
    e.mem.free(component_size);
}

// Translated from 00a94570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::LoadBinary` (Xbox PDB): `NiNode::LoadBinary`, then the
/// flag word: from the stream's file version (+0xd8) 0x0a000102 on it is read
/// from the stream (2 bytes through the reader at +0x24c, slot 8); before
/// that it is taken from bits 6 to 14 of a word at +0x284 of the stream.
/// Then the index (4 bytes), and bit 1 of the flags is set.
pub fn ni_switch_node_load_binary(e: &mut Engine, this: Ptr<NiSwitchNode>, stream: u32) {
    e.call(NI_NODE_LOAD_BINARY, &args![this, stream]);
    let flags_address = this.addr() + NiSwitchNode::m_uFlags.off;
    if e.mem.u32(stream + 0xd8) < 0x0a00_0102 {
        let old = e.mem.u16(stream + 0x284);
        e.mem.set_u16(flags_address, (old >> 6) & 0x1ff);
    } else {
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, flags_address, 2);
    }
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, this.addr() + NiSwitchNode::m_iIndex.off, 4);
    let flags = e.get(this, NiSwitchNode::m_uFlags);
    e.set(this, NiSwitchNode::m_uFlags, flags | 2);
}

// Translated from 00a94610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::SaveBinary` (Xbox PDB): `NiNode::SaveBinary`, then the
/// flag word (2 bytes) and the index (4 bytes) through the writer at
/// +0x250 of the stream (slot `0xc`).
pub fn ni_switch_node_save_binary(e: &mut Engine, this: Ptr<NiSwitchNode>, stream: u32) {
    e.call(NI_NODE_SAVE_BINARY, &args![this, stream]);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, this.addr() + NiSwitchNode::m_uFlags.off, 2);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, this.addr() + NiSwitchNode::m_iIndex.off, 4);
}

/// Appends `string` to the viewer-string array `strings` (`NiTArray` of
/// string pointers): grows it by its growth step when full, then
/// `SetAt(size, &string)`.
fn push_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, string: u32) {
    let size = e.get(strings, NiTArray::m_usSize) as u32;
    let capacity = e.get(strings, NiTArray::m_usMaxSize) as u32;
    let holder = e.mem.alloc(4);
    e.mem.set_u32(holder, string);
    if size >= capacity {
        let grow_by = e.get(strings, NiTArray::m_usGrowBy) as u32;
        e.call(REVISION_ARRAY_GROW, &args![strings, grow_by + size]);
    }
    e.call(REVISION_ARRAY_SET_AT, &args![strings, size, holder]);
    e.mem.free(holder);
}

// Translated from 00a94680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchNode::GetViewerStrings` (Xbox PDB): `NiNode::GetViewerStrings`,
/// then appends three strings to `strings`: the class-name string for the
/// RTTI name, `"m_iIndex"` with the index, and `"m_bUpdateOnlyActive"` with
/// bit 0 of the flags. (The array methods used are the shared
/// `NiTArray` grow and `SetAt` bodies, `0096ad30` and `0096ae90`.)
pub fn ni_switch_node_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiSwitchNode>,
    strings: Ptr<NiTArray>,
) {
    e.call(NI_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(SWITCH_NODE_RTTI);
    let text = e.call(VIEWER_STRING_RTTI, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let index = e.get(this, NiSwitchNode::m_iIndex);
    let text = e.call(VIEWER_STRING_INT, &args![NAME_INDEX, index]).u32();
    push_viewer_string(e, strings, text);
    let only_active = e.get(this, NiSwitchNode::m_uFlags) & 1 != 0;
    let text = e
        .call(
            VIEWER_STRING_BOOL,
            &args![NAME_UPDATE_ONLY_ACTIVE, only_active],
        )
        .u32();
    push_viewer_string(e, strings, text);
}

// Translated from 00a94550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class stream loader (Gamebryo `CreateObject`; Ghidra made no function of it):
/// registered with `NiStream::RegisterLoader` under `"NiSwitchNode"` at `00a8a0c4`. A new
/// 0xCC-byte object (`00aa13e0`) handed to the constructor ([`fn_00a94360`], a tail jump),
/// whose result it returns; null when the allocation fails.
pub fn ni_switch_node_create_object(e: &mut Engine) -> Ptr<NiSwitchNode> {
    let memory = e.call(OPERATOR_NEW, &args![0xccu32]).ptr::<NiSwitchNode>();
    if memory.is_null() {
        return Ptr::NULL;
    }
    fn_00a94360(e, memory)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a93ca0,
            ni_switch_node_detach_child(Ptr<NiSwitchNode>, u32, u32)
        ),
        entry!(
            0x00a93dd0,
            ni_switch_node_detach_child_at(Ptr<NiSwitchNode>, u32, u32)
        ),
        entry!(
            0x00a93ef0,
            ni_switch_node_on_visible(Ptr<NiSwitchNode>, u32)
        ),
        entry!(
            0x00a93f90,
            ni_switch_node_update_world_bound(Ptr<NiSwitchNode>)
        ),
        entry!(
            0x00a94000,
            ni_switch_node_is_equal(Ptr<NiSwitchNode>, Ptr<NiSwitchNode>) -> bool
        ),
        entry!(
            0x00a94040,
            ni_switch_node_update_downward_pass(Ptr<NiSwitchNode>, u32, u32)
        ),
        entry!(
            0x00a94130,
            ni_switch_node_update_selected_downward_pass(Ptr<NiSwitchNode>, u32, u32)
        ),
        entry!(
            0x00a94220,
            ni_switch_node_update_rigid_downward_pass(Ptr<NiSwitchNode>, u32, u32)
        ),
        entry!(
            0x00a94300,
            ni_switch_node_update_controllers(Ptr<NiSwitchNode>, u32)
        ),
        entry!(
            0x00a94360,
            fn_00a94360(Ptr<NiSwitchNode>) -> Ptr<NiSwitchNode>
        ),
        entry!(0x00a943f0, ni_switch_node_get_rtti() -> u32),
        entry!(
            0x00a94400,
            ni_switch_node_scalar_deleting_destructor(Ptr<NiSwitchNode>, u8) -> Ptr<NiSwitchNode>
        ),
        entry!(
            0x00a94440,
            ni_switch_node_attach_child(Ptr<NiSwitchNode>, u32, u32)
        ),
        entry!(
            0x00a944a0,
            ni_switch_node_set_at(Ptr<NiSwitchNode>, u32, u32, u32)
        ),
        entry!(
            0x00a94500,
            ni_switch_node_create_clone(Ptr<NiSwitchNode>, u32) -> Ptr<NiSwitchNode>
        ),
        entry!(
            0x00a94570,
            ni_switch_node_load_binary(Ptr<NiSwitchNode>, u32)
        ),
        entry!(
            0x00a94610,
            ni_switch_node_save_binary(Ptr<NiSwitchNode>, u32)
        ),
        entry!(
            0x00a94680,
            ni_switch_node_get_viewer_strings(Ptr<NiSwitchNode>, Ptr<NiTArray>)
        ),
        entry!(0x00a9_4550, ni_switch_node_create_object() -> Ptr<NiSwitchNode>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const NODE_VTABLE: u32 = 0x0310_0000;
    const DOUBLE_STREAM: u32 = 0x0700_0102;

    type Log = Vec<(u32, Vec<u32>)>;

    struct Fixture {
        e: Engine,
        stream_requests: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    /// Every callee outside this file is a double that does nothing unless a
    /// test replaces it; the array methods and `NiNode`'s detach routines
    /// behave as small models of the real ones.
    impl Fixture {
        fn new() -> Fixture {
            let mut e = Engine::new();
            let imports = [
                (INTERLOCKED_INCREMENT, "InterlockedIncrement"),
                (INTERLOCKED_DECREMENT, "InterlockedDecrement"),
            ]
            .map(|(slot, name)| crate::exe::Import {
                slot,
                dll: "KERNEL32.dll".into(),
                name: name.into(),
            });
            assert_eq!(e.register_imports(&imports), 2);
            e.map(0x011f_4000, 0x2000);
            for (i, word) in [11u32, 22, 33].into_iter().enumerate() {
                e.mem.set_u32(NULL_BOUND + 4 * i as u32, word);
            }
            e.mem.set_f32(NULL_BOUND + 12, 44.0);
            e.mem.set_u32(SWITCH_NODE_RTTI, 0x0108_0000);
            // One vtable for every node and child; each slot we use is a
            // distinct do-nothing double so the call log tells them apart.
            let mut slots = vec![0u32; 0x40];
            for slot in [
                SLOT_DELETE_THIS,
                SLOT_UPDATE_CONTROLLERS,
                SLOT_UPDATE_DOWNWARD_PASS,
                SLOT_UPDATE_SELECTED_DOWNWARD_PASS,
                SLOT_UPDATE_RIGID_DOWNWARD_PASS,
                SLOT_UPDATE_WORLD_DATA,
            ] {
                slots[slot as usize / 4] = 0x0700_0000 + slot;
                e.register(0x0700_0000 + slot, |_, _| Ret::default());
            }
            e.put_vtable(NODE_VTABLE, &slots);
            for addr in [
                NI_NODE_UPDATE_WORLD_BOUND,
                NI_NODE_UPDATE_CONTROLLERS,
                NI_NODE_UPDATE_DOWNWARD_PASS,
                NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS,
                NI_NODE_UPDATE_RIGID_DOWNWARD_PASS,
                NI_NODE_ATTACH_CHILD,
                NI_NODE_SET_AT,
                NI_NODE_LOAD_BINARY,
                NI_NODE_SAVE_BINARY,
                NI_NODE_GET_VIEWER_STRINGS,
                NI_NODE_CONSTRUCT,
                NI_NODE_DESTRUCT,
                NI_NODE_COPY_MEMBERS,
                UPDATE_OBJECT_CONTROLLERS,
                NI_AV_OBJECT_CULL,
                REVISION_ARRAY_REMOVE_AT,
                FREE_BYTES,
                OPERATOR_DELETE,
            ] {
                e.register(addr, |_, _| Ret::default());
            }
            e.register(NI_NODE_IS_EQUAL, |_, _| 1u32.into_ret());
            e.register(ALLOCATE_BYTES, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
            // `NiTArray<T>::SetAt(index, &value)` and the growing step.
            e.register(REVISION_ARRAY_SET_AT, |e, a| {
                let base = e.mem.u32(a[0] + 4);
                let value = e.mem.u32(a[2]);
                e.mem.set_u32(base + a[1] * 4, value);
                let size = e.mem.u16(a[0] + 0xa);
                if a[1] >= size as u32 {
                    e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
                }
                if value != 0 {
                    let count = e.mem.u16(a[0] + 0xc);
                    e.mem.set_u16(a[0] + 0xc, count + 1);
                }
                Ret::default()
            });
            e.register(REVISION_ARRAY_GROW, |e, a| {
                e.mem.set_u16(a[0] + 8, a[1] as u16);
                Ret::default()
            });
            // `NiNode::DetachChild` and `DetachChildAt`: take the child out
            // of the children array and hand it to the out pointer, which
            // holds one reference of its own.
            e.register(NI_NODE_DETACH_CHILD_OV2, |e, a| {
                let base = e.mem.u32(a[0] + 0xa0);
                let size = e.mem.u16(a[0] + 0xa6) as u32;
                for i in 0..size {
                    if e.mem.u32(base + i * 4) == a[1] {
                        e.mem.set_u32(base + i * 4, 0);
                        e.mem.set_u32(a[2], a[1]);
                        let refs = e.mem.u32(a[1] + 4);
                        e.mem.set_u32(a[1] + 4, refs + 1);
                    }
                }
                Ret::default()
            });
            e.register(NI_NODE_DETACH_CHILD_AT_OV2, |e, a| {
                let base = e.mem.u32(a[0] + 0xa0);
                let child = e.mem.u32(base + a[1] * 4);
                e.mem.set_u32(base + a[1] * 4, 0);
                e.mem.set_u32(a[2], child);
                if child != 0 {
                    let refs = e.mem.u32(child + 4);
                    e.mem.set_u32(child + 4, refs + 1);
                }
                Ret::default()
            });
            e.register(VIEWER_STRING_RTTI, |_, _| 0x1111u32.into_ret());
            e.register(VIEWER_STRING_INT, |_, _| 0x2222u32.into_ret());
            e.register(VIEWER_STRING_BOOL, |_, _| 0x3333u32.into_ret());
            // Streams: the reader/writer object's read (+8) and write (+0xc) words copy a
            // fixed value for the requested size.
            let stream_requests = Rc::new(RefCell::new(Vec::new()));
            let requests = stream_requests.clone();
            e.register_double(DOUBLE_STREAM, move |e, a| {
                // a = [stream, buffer, bytes, component size list, count]
                assert_eq!(a[4], 1);
                assert_eq!(
                    e.mem.u32(a[3]),
                    a[2],
                    "component size equals the byte count"
                );
                requests.borrow_mut().push((a[1], a[2]));
                match a[2] {
                    2 => e.mem.set_u16(a[1], 0x0040),
                    4 => e.mem.set_u32(a[1], 7),
                    _ => unreachable!(),
                }
                a[2].into_ret()
            });
            Fixture { e, stream_requests }
        }

        fn log(&mut self) {
            self.e.call_log = Some(Vec::new());
        }

        fn take_log(&mut self) -> Log {
            self.e.call_log.take().expect("log was started")
        }

        /// A node-like object: vtable, reference count 1, world bound, the
        /// `m_uFlags` word of `NiAVObject` and room for 64 children.
        fn object(&mut self) -> u32 {
            let object = self.e.mem.alloc(0xcc);
            self.e.mem.set_u32(object, NODE_VTABLE);
            self.e.mem.set_u32(object + 4, 1);
            let bound = self.e.mem.alloc(16);
            self.e.mem.set_u32(object + 0x20, bound);
            let children = self.e.mem.alloc(64 * 4);
            self.e.mem.set_u32(object + 0xa0, children);
            self.e.mem.set_u16(object + 0xa4, 64);
            let revisions = self.e.mem.alloc(64 * 4);
            self.e.mem.set_u32(object + 0xc0, revisions);
            self.e.mem.set_u16(object + 0xc4, 64);
            self.e.mem.set_u16(object + 0xca, 2);
            object
        }

        fn switch_node(&mut self, children: usize) -> (Ptr<NiSwitchNode>, Vec<u32>) {
            let node = self.object();
            let mut kids = Vec::new();
            for i in 0..children {
                let child = self.object();
                let base = self.e.mem.u32(node + 0xa0);
                self.e.mem.set_u32(base + i as u32 * 4, child);
                let revisions = self.e.mem.u32(node + 0xc0);
                self.e.mem.set_u32(revisions + i as u32 * 4, 100 + i as u32);
                kids.push(child);
            }
            self.e.mem.set_u16(node + 0xa6, children as u16);
            self.e.mem.set_u16(node + 0xc6, children as u16);
            self.e.mem.set_u16(node + 0xc8, children as u16);
            (Ptr::new(node), kids)
        }

        fn revision(&self, node: Ptr<NiSwitchNode>, index: u32) -> u32 {
            self.e
                .mem
                .u32(self.e.mem.u32(node.addr() + 0xc0) + index * 4)
        }

        fn bound_words(&self, object: u32) -> [u32; 4] {
            let bound = self.e.mem.u32(object + 0x20);
            [0, 4, 8, 12].map(|off| self.e.mem.u32(bound + off))
        }

        fn set_bound(&mut self, object: u32, words: [u32; 4]) {
            let bound = self.e.mem.u32(object + 0x20);
            for (i, w) in words.into_iter().enumerate() {
                self.e.mem.set_u32(bound + 4 * i as u32, w);
            }
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn vslot(slot: u32) -> u32 {
        0x0700_0000 + slot
    }

    #[test]
    fn detach_child_removes_the_revision_slot_and_hands_the_child_back() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(3);
        f.e.set(node, NiSwitchNode::m_iIndex, 2);
        let out = f.e.mem.alloc(4);
        f.log();
        ni_switch_node_detach_child(&mut f.e, node, kids[2], out);
        let log = f.take_log();
        assert_eq!(f.e.get(node, NiSwitchNode::m_uiRevID), 1);
        // The last slot of the revision array went away completely.
        assert_eq!(f.revision(node, 2), 0);
        assert_eq!(f.e.mem.u16(node.addr() + 0xc8), 2);
        assert_eq!(f.e.mem.u16(node.addr() + 0xc6), 2);
        assert_eq!(calls(&log, NI_NODE_DETACH_CHILD_OV2).len(), 1);
        assert_eq!(f.e.mem.u32(out), kids[2]);
        // Reference held by the out pointer and the parent's own are all
        // that remain (the local one was released).
        assert_eq!(f.e.mem.u32(kids[2] + 4), 2);
        assert_eq!(
            f.e.get(node, NiSwitchNode::m_iIndex),
            -1,
            "its child is gone"
        );
    }

    #[test]
    fn detaching_hands_the_callee_the_argument_slot_of_the_exe_frame() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(3);
        let out = f.e.mem.alloc(4);
        // The frame is the next local the game thread's stack gives out.
        let frame = f.e.next_stack_local(0x18);
        f.log();
        ni_switch_node_detach_child(&mut f.e, node, kids[2], out);
        let log = f.take_log();
        assert_eq!(calls(&log, NI_NODE_DETACH_CHILD_OV2)[0][2], frame + 0x10);
        let out = f.e.mem.alloc(4);
        let frame = f.e.next_stack_local(0x18);
        f.log();
        ni_switch_node_detach_child_at(&mut f.e, node, 1, out);
        let log = f.take_log();
        assert_eq!(calls(&log, NI_NODE_DETACH_CHILD_AT_OV2)[0][2], frame + 0x10);
    }

    #[test]
    fn detach_child_in_the_middle_keeps_the_size_and_a_valid_index() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(3);
        f.e.set(node, NiSwitchNode::m_iIndex, 1);
        let out = f.e.mem.alloc(4);
        ni_switch_node_detach_child(&mut f.e, node, kids[0], out);
        assert_eq!(f.revision(node, 0), 0);
        assert_eq!(f.e.mem.u16(node.addr() + 0xc6), 3, "not the last slot");
        assert_eq!(f.e.mem.u16(node.addr() + 0xc8), 2);
        assert_eq!(f.e.get(node, NiSwitchNode::m_iIndex), 1);
        // An unknown child leaves the revision array alone.
        let stranger = f.object();
        let other_out = f.e.mem.alloc(4);
        ni_switch_node_detach_child(&mut f.e, node, stranger, other_out);
        assert_eq!(f.revision(node, 1), 101);
        assert_eq!(f.e.mem.u32(other_out), 0);
        assert_eq!(f.e.mem.u32(stranger + 4), 1);
    }

    #[test]
    fn detach_child_releases_the_pointer_it_replaces() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(2);
        let old = f.object();
        let out = f.e.mem.alloc(4);
        f.e.mem.set_u32(out, old);
        ni_switch_node_detach_child(&mut f.e, node, kids[0], out);
        assert_eq!(f.e.mem.u32(out), kids[0]);
        assert_eq!(f.e.mem.u32(old + 4), 0, "the old reference was dropped");
        assert_eq!(f.e.mem.u32(kids[0] + 4), 2);
        // A pointer already holding the detached child gains nothing.
        let (node, kids) = f.switch_node(1);
        let out = f.e.mem.alloc(4);
        f.e.mem.set_u32(out, kids[0]);
        ni_switch_node_detach_child(&mut f.e, node, kids[0], out);
        assert_eq!(f.e.mem.u32(out), kids[0]);
        assert_eq!(
            f.e.mem.u32(kids[0] + 4),
            1,
            "the local reference was released"
        );
    }

    #[test]
    fn detach_child_at_removes_the_revision_slot_only_below_the_count() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(3);
        f.e.set(node, NiSwitchNode::m_iIndex, 0);
        let out = f.e.mem.alloc(4);
        f.log();
        ni_switch_node_detach_child_at(&mut f.e, node, 1, out);
        let log = f.take_log();
        assert_eq!(f.e.get(node, NiSwitchNode::m_uiRevID), 1);
        let revisions = node.addr() + 0xbc;
        assert_eq!(
            calls(&log, REVISION_ARRAY_REMOVE_AT),
            vec![vec![revisions, 1]]
        );
        assert_eq!(f.e.mem.u32(out), kids[1]);
        assert_eq!(f.e.get(node, NiSwitchNode::m_iIndex), 0);
        // Past the children count nothing is removed from the revisions.
        f.log();
        ni_switch_node_detach_child_at(&mut f.e, node, 9, out);
        let log = f.take_log();
        assert!(calls(&log, REVISION_ARRAY_REMOVE_AT).is_empty());
        assert_eq!(f.e.mem.u32(out), 0);
        assert_eq!(f.e.mem.u32(kids[1] + 4), 1, "the old pointer was released");
        // A selected index that is now empty is dropped.
        f.e.set(node, NiSwitchNode::m_iIndex, 1);
        ni_switch_node_detach_child_at(&mut f.e, node, 2, out);
        assert_eq!(f.e.get(node, NiSwitchNode::m_iIndex), -1);
    }

    #[test]
    fn on_visible_updates_a_stale_child_then_culls_it() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(2);
        f.e.set(node, NiSwitchNode::m_iIndex, 1);
        f.e.set(node, NiSwitchNode::m_uiRevID, 7);
        f.e.set(node, NiSwitchNode::m_fSavedTime, 1.5);
        f.e.set(node, NiSwitchNode::m_uFlags, 0b10);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        f.e.register_double(vslot(SLOT_UPDATE_DOWNWARD_PASS), move |e, a| {
            record
                .borrow_mut()
                .push((a.to_vec(), e.mem.bytes(a[1], 12)));
            Ret::default()
        });
        f.log();
        ni_switch_node_on_visible(&mut f.e, node, 0x5000);
        let log = f.take_log();
        // The revision slot of the child was set from the node's.
        assert_eq!(f.revision(node, 1), 7);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[0], kids[1]);
        assert_eq!(seen[0].0[2], 0);
        let mut data = vec![0u8; 12];
        data[..4].copy_from_slice(&1.5f32.to_le_bytes());
        data[4] = 1;
        assert_eq!(seen[0].1, data);
        assert_eq!(calls(&log, NI_AV_OBJECT_CULL), vec![vec![kids[1], 0x5000]]);
        // Up to date: no update, still culled.
        f.log();
        ni_switch_node_on_visible(&mut f.e, node, 0x5001);
        let log = f.take_log();
        assert!(calls(&log, vslot(SLOT_UPDATE_DOWNWARD_PASS)).is_empty());
        assert_eq!(calls(&log, NI_AV_OBJECT_CULL), vec![vec![kids[1], 0x5001]]);
    }

    #[test]
    fn on_visible_does_nothing_without_a_selected_child() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(1);
        f.e.set(node, NiSwitchNode::m_iIndex, -1);
        f.log();
        ni_switch_node_on_visible(&mut f.e, node, 1);
        assert!(f.take_log().is_empty());
        f.e.set(node, NiSwitchNode::m_iIndex, 1);
        f.log();
        ni_switch_node_on_visible(&mut f.e, node, 1);
        assert!(f.take_log().is_empty(), "empty slot");
    }

    #[test]
    fn update_world_bound_follows_the_selected_child_when_only_active_updates() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(2);
        f.log();
        ni_switch_node_update_world_bound(&mut f.e, node);
        let log = f.take_log();
        assert_eq!(
            calls(&log, NI_NODE_UPDATE_WORLD_BOUND),
            vec![vec![node.addr()]]
        );
        f.e.set(node, NiSwitchNode::m_uFlags, 1);
        f.e.set(node, NiSwitchNode::m_iIndex, 1);
        f.set_bound(kids[1], [1, 2, 3, 4.5f32.to_bits()]);
        ni_switch_node_update_world_bound(&mut f.e, node);
        assert_eq!(f.bound_words(node.addr()), [1, 2, 3, 4.5f32.to_bits()]);
        // A child without a bound gives the null bound.
        f.e.mem.set_u32(kids[1] + 0x20, 0);
        ni_switch_node_update_world_bound(&mut f.e, node);
        assert_eq!(f.bound_words(node.addr()), [11, 22, 33, 44.0f32.to_bits()]);
        // No selected child: only the radius is cleared.
        f.e.set(node, NiSwitchNode::m_iIndex, -1);
        ni_switch_node_update_world_bound(&mut f.e, node);
        assert_eq!(f.bound_words(node.addr()), [11, 22, 33, 0]);
    }

    #[test]
    fn is_equal_compares_the_index_and_only_the_active_flag() {
        let mut f = Fixture::new();
        let (a, _) = f.switch_node(0);
        let (b, _) = f.switch_node(0);
        f.e.set(a, NiSwitchNode::m_iIndex, 2);
        f.e.set(b, NiSwitchNode::m_iIndex, 2);
        f.e.set(a, NiSwitchNode::m_uFlags, 0b11);
        f.e.set(b, NiSwitchNode::m_uFlags, 0b01);
        f.log();
        assert!(ni_switch_node_is_equal(&mut f.e, a, b));
        assert_eq!(
            calls(&f.take_log(), NI_NODE_IS_EQUAL),
            vec![vec![a.addr(), b.addr()]]
        );
        f.e.set(b, NiSwitchNode::m_uFlags, 0b10);
        assert!(!ni_switch_node_is_equal(&mut f.e, a, b));
        f.e.set(b, NiSwitchNode::m_uFlags, 0b01);
        f.e.set(b, NiSwitchNode::m_iIndex, 3);
        assert!(!ni_switch_node_is_equal(&mut f.e, a, b));
        f.e.register(NI_NODE_IS_EQUAL, |_, _| 0u32.into_ret());
        f.e.set(b, NiSwitchNode::m_iIndex, 2);
        assert!(
            !ni_switch_node_is_equal(&mut f.e, a, b),
            "base class differs"
        );
    }

    fn update_data(f: &mut Fixture, time: f32, controllers: u8) -> u32 {
        let data = f.e.mem.alloc(12);
        f.e.mem.set_f32(data, time);
        f.e.mem.set_u8(data + 4, controllers);
        data
    }

    #[test]
    fn update_downward_pass_without_only_active_defers_to_the_base_class() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(1);
        f.e.set(node, NiSwitchNode::m_uFlags, 0b10);
        let data = update_data(&mut f, 2.5, 0);
        f.log();
        ni_switch_node_update_downward_pass(&mut f.e, node, data, 9);
        let log = f.take_log();
        assert_eq!(
            f.e.get(node, NiSwitchNode::m_uFlags),
            0,
            "bit 1 follows the data"
        );
        assert_eq!(f.e.get(node, NiSwitchNode::m_fSavedTime), 2.5);
        assert_eq!(
            calls(&log, NI_NODE_UPDATE_DOWNWARD_PASS),
            vec![vec![node.addr(), data, 9]]
        );
        assert_eq!(log.len(), 1, "only the base class is called");
        f.e.mem.set_u8(data + 4, 1);
        ni_switch_node_update_downward_pass(&mut f.e, node, data, 9);
        assert_eq!(f.e.get(node, NiSwitchNode::m_uFlags), 0b10);
    }

    #[test]
    fn update_downward_pass_updates_only_the_selected_child() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(2);
        f.e.set(node, NiSwitchNode::m_uFlags, 1);
        f.e.set(node, NiSwitchNode::m_iIndex, 0);
        f.e.set(node, NiSwitchNode::m_uiRevID, 4);
        f.set_bound(kids[0], [5, 6, 7, 8.0f32.to_bits()]);
        let data = update_data(&mut f, 3.0, 1);
        f.log();
        ni_switch_node_update_downward_pass(&mut f.e, node, data, 6);
        let log = f.take_log();
        assert_eq!(f.e.get(node, NiSwitchNode::m_uFlags), 0b11);
        assert_eq!(f.e.get(node, NiSwitchNode::m_uiRevID), 5);
        assert_eq!(
            calls(&log, UPDATE_OBJECT_CONTROLLERS),
            vec![vec![node.addr(), data, 1]]
        );
        assert_eq!(
            calls(&log, vslot(SLOT_UPDATE_WORLD_DATA)),
            vec![vec![node.addr(), data]]
        );
        assert_eq!(
            calls(&log, vslot(SLOT_UPDATE_DOWNWARD_PASS)),
            vec![vec![kids[0], data, 6]]
        );
        assert_eq!(f.revision(node, 0), 5);
        assert_eq!(f.bound_words(node.addr()), [5, 6, 7, 8.0f32.to_bits()]);
        // Without the controller flag the controllers are skipped, and with
        // no selected child nothing else happens.
        f.e.mem.set_u8(data + 4, 0);
        f.e.set(node, NiSwitchNode::m_iIndex, -1);
        f.log();
        ni_switch_node_update_downward_pass(&mut f.e, node, data, 6);
        let log = f.take_log();
        assert!(calls(&log, UPDATE_OBJECT_CONTROLLERS).is_empty());
        assert_eq!(calls(&log, vslot(SLOT_UPDATE_WORLD_DATA)).len(), 1);
        assert!(calls(&log, vslot(SLOT_UPDATE_DOWNWARD_PASS)).is_empty());
        assert_eq!(f.e.get(node, NiSwitchNode::m_uFlags), 0b01);
    }

    fn controller_flag_probe(f: &mut Fixture) -> Rc<Cell<Option<u8>>> {
        let seen = Rc::new(Cell::new(None));
        let record = seen.clone();
        f.e.register_double(UPDATE_OBJECT_CONTROLLERS, move |e, a| {
            record.set(Some(e.mem.u8(a[1] + 4)));
            Ret::default()
        });
        seen
    }

    #[test]
    fn update_selected_downward_pass_picks_the_child_slot_from_its_flags() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(1);
        let data = update_data(&mut f, 1.25, 0);
        f.log();
        ni_switch_node_update_selected_downward_pass(&mut f.e, node, data, 3);
        assert_eq!(f.e.get(node, NiSwitchNode::m_fSavedTime), 1.25);
        assert_eq!(
            calls(&f.take_log(), NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS),
            vec![vec![node.addr(), data, 3]]
        );
        f.e.set(node, NiSwitchNode::m_uFlags, 1);
        f.e.set(node, NiSwitchNode::m_iIndex, 0);
        f.e.set(node, NiSwitchNode::m_uiRevID, 9);
        // Bit 3 of the node's own flags becomes the controller flag for the
        // duration of the controllers update and is restored afterwards.
        f.e.mem.set_u32(node.addr() + 0x30, 0b1000);
        let seen = controller_flag_probe(&mut f);
        f.e.mem.set_u8(data + 4, 0);
        f.e.mem.set_u32(kids[0] + 0x30, 0b10);
        f.set_bound(kids[0], [1, 1, 1, 2.0f32.to_bits()]);
        f.log();
        ni_switch_node_update_selected_downward_pass(&mut f.e, node, data, 3);
        let log = f.take_log();
        assert_eq!(seen.get(), Some(1));
        assert_eq!(f.e.mem.u8(data + 4), 0);
        assert_eq!(f.e.get(node, NiSwitchNode::m_uiRevID), 10);
        assert_eq!(
            calls(&log, vslot(SLOT_UPDATE_SELECTED_DOWNWARD_PASS)),
            vec![vec![kids[0], data, 3]]
        );
        assert!(calls(&log, vslot(SLOT_UPDATE_RIGID_DOWNWARD_PASS)).is_empty());
        assert_eq!(f.revision(node, 0), 10);
        assert_eq!(f.bound_words(node.addr()), [1, 1, 1, 2.0f32.to_bits()]);
        // Bit 4 of the child's flags selects the rigid slot.
        f.e.mem.set_u32(kids[0] + 0x30, 0b10010);
        f.log();
        ni_switch_node_update_selected_downward_pass(&mut f.e, node, data, 3);
        let log = f.take_log();
        assert_eq!(calls(&log, vslot(SLOT_UPDATE_RIGID_DOWNWARD_PASS)).len(), 1);
        // A child without bit 1 is not updated but is still recorded.
        f.e.mem.set_u32(kids[0] + 0x30, 0);
        f.log();
        ni_switch_node_update_selected_downward_pass(&mut f.e, node, data, 3);
        let log = f.take_log();
        assert!(calls(&log, vslot(SLOT_UPDATE_SELECTED_DOWNWARD_PASS)).is_empty());
        assert_eq!(f.revision(node, 0), 12);
    }

    #[test]
    fn update_rigid_downward_pass_always_uses_the_rigid_slot() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(1);
        let data = update_data(&mut f, 0.5, 1);
        f.log();
        ni_switch_node_update_rigid_downward_pass(&mut f.e, node, data, 4);
        assert_eq!(
            calls(&f.take_log(), NI_NODE_UPDATE_RIGID_DOWNWARD_PASS),
            vec![vec![node.addr(), data, 4]]
        );
        f.e.set(node, NiSwitchNode::m_uFlags, 1);
        f.e.set(node, NiSwitchNode::m_iIndex, 0);
        f.e.mem.set_u32(kids[0] + 0x30, 0b10);
        let seen = controller_flag_probe(&mut f);
        f.log();
        ni_switch_node_update_rigid_downward_pass(&mut f.e, node, data, 4);
        let log = f.take_log();
        assert_eq!(seen.get(), Some(0), "bit 3 of the node flags is clear");
        assert_eq!(f.e.mem.u8(data + 4), 1, "restored");
        assert_eq!(
            calls(&log, vslot(SLOT_UPDATE_RIGID_DOWNWARD_PASS)),
            vec![vec![kids[0], data, 4]]
        );
        f.e.mem.set_u32(kids[0] + 0x30, 0b10000);
        f.log();
        ni_switch_node_update_rigid_downward_pass(&mut f.e, node, data, 4);
        assert!(calls(&f.take_log(), vslot(SLOT_UPDATE_RIGID_DOWNWARD_PASS)).is_empty());
    }

    #[test]
    fn update_controllers_updates_the_node_and_the_selected_child() {
        let mut f = Fixture::new();
        let (node, kids) = f.switch_node(2);
        f.log();
        ni_switch_node_update_controllers(&mut f.e, node, 0x600);
        assert_eq!(
            calls(&f.take_log(), NI_NODE_UPDATE_CONTROLLERS),
            vec![vec![node.addr(), 0x600]]
        );
        f.e.set(node, NiSwitchNode::m_uFlags, 1);
        f.e.set(node, NiSwitchNode::m_iIndex, -1);
        f.log();
        ni_switch_node_update_controllers(&mut f.e, node, 0x600);
        assert!(f.take_log().is_empty(), "negative index: nothing");
        f.e.set(node, NiSwitchNode::m_iIndex, 1);
        f.log();
        ni_switch_node_update_controllers(&mut f.e, node, 0x600);
        let log = f.take_log();
        assert_eq!(
            calls(&log, UPDATE_OBJECT_CONTROLLERS),
            vec![vec![node.addr(), 0x600, 1]]
        );
        assert_eq!(
            calls(&log, vslot(SLOT_UPDATE_CONTROLLERS)),
            vec![vec![kids[1], 0x600]]
        );
        // An empty slot still updates the node's own controllers.
        f.e.set(node, NiSwitchNode::m_iIndex, 5);
        f.log();
        ni_switch_node_update_controllers(&mut f.e, node, 0x600);
        let log = f.take_log();
        assert_eq!(calls(&log, UPDATE_OBJECT_CONTROLLERS).len(), 1);
        assert!(calls(&log, vslot(SLOT_UPDATE_CONTROLLERS)).is_empty());
    }

    #[test]
    fn constructor_sets_up_the_switch_state_and_the_revision_array() {
        let mut f = Fixture::new();
        let memory = Ptr::<NiSwitchNode>::new(f.e.mem.alloc(0xcc));
        f.log();
        let node = fn_00a94360(&mut f.e, memory);
        let log = f.take_log();
        assert_eq!(node, memory);
        assert_eq!(calls(&log, NI_NODE_CONSTRUCT), vec![vec![memory.addr(), 0]]);
        assert_eq!(f.e.mem.u32(node.addr()), 0x010a_001c);
        assert_eq!(f.e.get(node, NiSwitchNode::m_iIndex), -1);
        assert_eq!(f.e.get(node, NiSwitchNode::m_uiRevID), 1);
        assert_eq!(f.e.get(node, NiSwitchNode::m_uFlags), 3);
        assert_eq!(f.e.get(node, NiSwitchNode::m_fSavedTime), 0.0);
        let revisions = node.at(NiSwitchNode::m_kChildRevID);
        assert_eq!(f.e.mem.u32(revisions.addr()), 0x0101_a3ac);
        assert_eq!(f.e.get(revisions, NiTArray::m_usMaxSize), 1);
        assert_eq!(f.e.get(revisions, NiTArray::m_usGrowBy), 1);
        assert_eq!(f.e.get(revisions, NiTArray::m_usSize), 0);
        assert_eq!(f.e.get(revisions, NiTArray::m_usESize), 0);
        assert_eq!(calls(&log, ALLOCATE_BYTES), vec![vec![4]]);
        assert_ne!(f.e.get(revisions, NiTArray::m_pBase), 0);
    }

    #[test]
    fn get_rtti_returns_the_class_rtti() {
        let mut f = Fixture::new();
        assert_eq!(ni_switch_node_get_rtti(&mut f.e), 0x011f_5eb4);
    }

    #[test]
    fn destructor_frees_the_revisions_and_the_node_only_when_asked() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(0);
        let buffer = f.e.mem.u32(node.addr() + 0xc0);
        f.log();
        let returned = ni_switch_node_scalar_deleting_destructor(&mut f.e, node, 0);
        let log = f.take_log();
        assert_eq!(returned, node);
        assert_eq!(f.e.mem.u32(node.addr() + 0xbc), 0x0101_a3a4);
        assert_eq!(calls(&log, FREE_BYTES), vec![vec![buffer]]);
        assert_eq!(calls(&log, NI_NODE_DESTRUCT), vec![vec![node.addr()]]);
        assert!(calls(&log, OPERATOR_DELETE).is_empty());
        f.log();
        ni_switch_node_scalar_deleting_destructor(&mut f.e, node, 0b11);
        let log = f.take_log();
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![node.addr(), 0xcc]]);
    }

    #[test]
    fn attach_child_appends_an_empty_revision_and_grows_when_full() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(2);
        f.e.set(node, NiSwitchNode::m_uiRevID, 9);
        f.e.mem.set_u16(node.addr() + 0xc4, 4);
        f.log();
        ni_switch_node_attach_child(&mut f.e, node, 0x700, 1);
        let log = f.take_log();
        assert_eq!(
            calls(&log, NI_NODE_ATTACH_CHILD),
            vec![vec![node.addr(), 0x700, 1]]
        );
        assert_eq!(f.e.get(node, NiSwitchNode::m_uiRevID), 1);
        assert!(calls(&log, REVISION_ARRAY_GROW).is_empty());
        assert_eq!(f.revision(node, 2), 0);
        assert_eq!(f.e.mem.u16(node.addr() + 0xc6), 3);
        // At capacity the array is grown by its growth step first.
        f.e.mem.set_u16(node.addr() + 0xc4, 3);
        f.log();
        ni_switch_node_attach_child(&mut f.e, node, 0x701, 0);
        let log = f.take_log();
        assert_eq!(
            calls(&log, REVISION_ARRAY_GROW),
            vec![vec![node.addr() + 0xbc, 2 + 3]]
        );
        assert_eq!(f.e.mem.u16(node.addr() + 0xc6), 4);
    }

    #[test]
    fn set_at_clears_the_revision_then_sets_the_child() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(2);
        f.e.set(node, NiSwitchNode::m_uiRevID, 9);
        f.e.mem.set_u16(node.addr() + 0xc4, 8);
        f.log();
        ni_switch_node_set_at(&mut f.e, node, 1, 0x700, 0x800);
        let log = f.take_log();
        assert_eq!(f.e.get(node, NiSwitchNode::m_uiRevID), 1);
        assert_eq!(f.revision(node, 1), 0);
        assert!(calls(&log, REVISION_ARRAY_GROW).is_empty());
        assert_eq!(
            calls(&log, NI_NODE_SET_AT),
            vec![vec![node.addr(), 1, 0x700, 0x800]]
        );
        // Past the capacity the array grows by the growth step plus index.
        f.log();
        ni_switch_node_set_at(&mut f.e, node, 10, 0x701, 0x801);
        let log = f.take_log();
        assert_eq!(
            calls(&log, REVISION_ARRAY_GROW),
            vec![vec![node.addr() + 0xbc, 2 + 10]]
        );
        assert_eq!(f.e.mem.u16(node.addr() + 0xc6), 11);
    }

    #[test]
    fn create_clone_builds_a_switch_node_and_copies_flags_and_index() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(0);
        f.e.set(node, NiSwitchNode::m_uFlags, 0x1235);
        f.e.set(node, NiSwitchNode::m_iIndex, 3);
        f.log();
        let clone = ni_switch_node_create_clone(&mut f.e, node, 0x900);
        let log = f.take_log();
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0xcc]]);
        assert_eq!(calls(&log, NI_NODE_CONSTRUCT), vec![vec![clone.addr(), 0]]);
        assert_eq!(
            calls(&log, NI_NODE_COPY_MEMBERS),
            vec![vec![node.addr(), clone.addr(), 0x900]]
        );
        assert_eq!(f.e.mem.u32(clone.addr()), 0x010a_001c);
        assert_eq!(f.e.get(clone, NiSwitchNode::m_uFlags), 0x1235);
        assert_eq!(f.e.get(clone, NiSwitchNode::m_iIndex), 3);
    }

    fn stream(f: &mut Fixture, version: u32) -> (u32, u32) {
        let stream = f.e.mem.alloc(0x300);
        let reader = f.e.mem.alloc(0x10);
        // The binary stream object's own words: read at +8, write at +0xc (00a945b0).
        f.e.mem.set_u32(reader + 8, DOUBLE_STREAM);
        f.e.mem.set_u32(reader + 0xc, DOUBLE_STREAM);
        f.e.mem.set_u32(stream + 0xd8, version);
        f.e.mem.set_u32(stream + 0x24c, reader);
        f.e.mem.set_u32(stream + 0x250, reader);
        (stream, reader)
    }

    #[test]
    fn load_binary_reads_flags_and_index_from_new_streams() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(0);
        let (stream, reader) = stream(&mut f, 0x0a00_0102);
        f.log();
        ni_switch_node_load_binary(&mut f.e, node, stream);
        let log = f.take_log();
        assert_eq!(
            calls(&log, NI_NODE_LOAD_BINARY),
            vec![vec![node.addr(), stream]]
        );
        let requests = f.stream_requests.borrow().clone();
        assert_eq!(
            requests,
            vec![(node.addr() + 0xac, 2), (node.addr() + 0xb0, 4)]
        );
        assert_eq!(
            f.e.get(node, NiSwitchNode::m_uFlags),
            0x0042,
            "read 0x40, bit 1 set"
        );
        assert_eq!(f.e.get(node, NiSwitchNode::m_iIndex), 7);
        assert_eq!(
            f.e.mem.u32(reader + 4),
            6,
            "byte counts added to the reader's total"
        );
    }

    #[test]
    fn load_binary_takes_the_flags_from_the_old_stream_word() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(0);
        let (stream, _) = stream(&mut f, 0x0a00_0101);
        f.e.mem.set_u16(stream + 0x284, 0b0101_0100_0000_0000);
        ni_switch_node_load_binary(&mut f.e, node, stream);
        // (word >> 6) & 0x1ff = 0b1_0101_0000 | bit 1.
        assert_eq!(f.e.get(node, NiSwitchNode::m_uFlags), 0x150 | 2);
        assert_eq!(
            f.stream_requests.borrow().len(),
            1,
            "only the index is read"
        );
        assert_eq!(f.e.get(node, NiSwitchNode::m_iIndex), 7);
    }

    #[test]
    fn save_binary_writes_flags_and_index() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(0);
        let (stream, writer) = stream(&mut f, 0x0a00_0102);
        f.log();
        ni_switch_node_save_binary(&mut f.e, node, stream);
        let log = f.take_log();
        assert_eq!(
            calls(&log, NI_NODE_SAVE_BINARY),
            vec![vec![node.addr(), stream]]
        );
        let requests = f.stream_requests.borrow().clone();
        assert_eq!(
            requests,
            vec![(node.addr() + 0xac, 2), (node.addr() + 0xb0, 4)]
        );
        assert_eq!(f.e.mem.u32(writer + 4), 6);
    }

    #[test]
    fn viewer_strings_append_the_class_index_and_flag_strings() {
        let mut f = Fixture::new();
        let (node, _) = f.switch_node(0);
        f.e.set(node, NiSwitchNode::m_iIndex, -2);
        f.e.set(node, NiSwitchNode::m_uFlags, 0b11);
        let strings = Ptr::<NiTArray>::new(f.e.mem.alloc(0x10));
        let base = f.e.mem.alloc(64 * 4);
        f.e.set(strings, NiTArray::m_pBase, base);
        f.e.set(strings, NiTArray::m_usMaxSize, 2);
        f.e.set(strings, NiTArray::m_usSize, 1);
        f.e.set(strings, NiTArray::m_usGrowBy, 4);
        f.log();
        ni_switch_node_get_viewer_strings(&mut f.e, node, strings);
        let log = f.take_log();
        assert_eq!(
            calls(&log, NI_NODE_GET_VIEWER_STRINGS),
            vec![vec![node.addr(), strings.addr()]]
        );
        assert_eq!(calls(&log, VIEWER_STRING_RTTI), vec![vec![0x0108_0000]]);
        assert_eq!(
            calls(&log, VIEWER_STRING_INT),
            vec![vec![0x0109_fff8, (-2i32) as u32]]
        );
        assert_eq!(calls(&log, VIEWER_STRING_BOOL), vec![vec![0x010a_011c, 1]]);
        // Three strings were appended after the existing one, growing the
        // array once, when it was full (size 2, capacity 2, grow by 4).
        assert_eq!(
            calls(&log, REVISION_ARRAY_GROW),
            vec![vec![strings.addr(), 4 + 2]]
        );
        let stored: Vec<u32> = (1..4).map(|i| f.e.mem.u32(base + i * 4)).collect();
        assert_eq!(stored, vec![0x1111, 0x2222, 0x3333]);
        assert_eq!(f.e.get(strings, NiTArray::m_usSize), 4);
        // Without the flag the boolean string gets false.
        f.e.set(node, NiSwitchNode::m_uFlags, 0b10);
        f.log();
        ni_switch_node_get_viewer_strings(&mut f.e, node, strings);
        assert_eq!(
            calls(&f.take_log(), VIEWER_STRING_BOOL),
            vec![vec![0x010a_011c, 0]]
        );
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    #[test]
    fn create_object_constructs_a_new_switch_node_or_gives_null() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_4550, &args![]).u32(), 0);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NODE_CONSTRUCT, |_, _| Ret::default());
        e.register(ALLOCATE_BYTES, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let node = e.call(0x00a9_4550, &args![]).u32();
        assert_eq!(e.mem.u32(node), SWITCH_NODE_VTABLE);
        let news: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == OPERATOR_NEW)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(news, vec![vec![0xcc]]);
        assert_eq!(
            e.get(Ptr::<NiSwitchNode>::new(node), NiSwitchNode::m_iIndex),
            -1
        );
    }
}
