//! `NiMain/ninode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a5ece0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a5e930`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xac bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_e930;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xacu32]);
    }
    this
}

/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`, `thiscall`, no stack words).
const NI_AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// `NiNode`'s vtable (`0109b5ac`, RTTI `.?AVNiNode@@`).
const NI_NODE_VTABLE: u32 = 0x0109_b5ac;
/// The child array's constructor `004aff00` (`thiscall`, `RET 8`: initial size, grow-by).
const CHILD_ARRAY_CONSTRUCT: u32 = 0x004a_ff00;

// Translated from 00a5ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::NiNode` (unnamed in the engine map; the vtable it sets is `NiNode`'s by its RTTI).
/// `thiscall`, one word (`RET 4`): runs `NiAVObject`'s constructor, sets the vtable and
/// constructs the child array `m_kChildren` (+0x9c) with that initial size and a grow-by of 1
/// (`004aff00`). Answers `this`.
pub fn ni_node_ni_node(e: &mut Engine, this: Ptr, child_count: u32) -> Ptr {
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_NODE_VTABLE);
    e.call(
        CHILD_ARRAY_CONSTRUCT,
        &args![this.addr().wrapping_add(0x9c), child_count, 1u32],
    );
    this
}

/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiAVObject::AttachParent` (engine map; `thiscall` on the child, `RET 4`: the parent).
const NI_AV_OBJECT_ATTACH_PARENT: u32 = 0x00a5_9d00;
/// The child array's first-free-slot store `00a5eb20` (`thiscall` on the array, `RET 4`: the
/// address of the smart pointer to store).
const CHILD_ARRAY_ADD_FIRST_EMPTY: u32 = 0x00a5_eb20;
/// The child array's resize `0060bef0` (`thiscall` on the array, `RET 4`: the new size).
const CHILD_ARRAY_SET_SIZE: u32 = 0x0060_bef0;
/// The child array's store at an index `004b0310` (`thiscall` on the array, `RET 8`: the
/// index, the address of the smart pointer to store).
const CHILD_ARRAY_SET_AT: u32 = 0x004b_0310;

// Translated from 00a5ed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::AttachChild` (Xbox PDB). `thiscall`, two words (`RET 8`): the child and a flag
/// (its low byte is read; `bFirstAvail` in the Xbox PDB). Nothing for a null child. Otherwise
/// the child's reference count (+4) is raised, `NiAVObject::AttachParent(child, this)` runs,
/// and a temporary smart pointer to the child (raising the count again) is stored in the
/// child array `m_kChildren` (+0x9c): with the flag, in the first free slot (`00a5eb20`);
/// without it, at index `m_usSize` (+0xa6), after growing the array to `m_usSize +
/// m_usGrowBy` (+0xaa, `0060bef0`) when `m_usSize` is not below `m_usMaxSize` (+0xa4)
/// (`004b0310`). The two references are then dropped, each calling the child's slot +4
/// (its deleting destructor) when the count reaches zero.
pub fn ni_node_attach_child(e: &mut Engine, this: Ptr, child: u32, first_available: u32) {
    if child == 0 {
        return;
    }
    let node = this.addr();
    let count = child + 4;
    e.call(INTERLOCKED_INCREMENT, &args![count]);
    e.call(NI_AV_OBJECT_ATTACH_PARENT, &args![child, node]);
    let children = node + 0x9c;
    // The temporary smart pointer (the game reuses the flag's argument slot for it).
    e.with_stack(4, |e, temp| {
        e.mem.set_u32(temp.addr(), child);
        e.call(INTERLOCKED_INCREMENT, &args![count]);
        if first_available as u8 != 0 {
            e.call(CHILD_ARRAY_ADD_FIRST_EMPTY, &args![children, temp]);
        } else {
            let size = u32::from(e.mem.u16(node + 0xa6));
            let max_size = u32::from(e.mem.u16(node + 0xa4));
            if size >= max_size {
                let grow_by = u32::from(e.mem.u16(children + 0xe));
                e.call(CHILD_ARRAY_SET_SIZE, &args![children, grow_by + size]);
            }
            e.call(CHILD_ARRAY_SET_AT, &args![children, size, temp]);
        }
    });
    if e.call(INTERLOCKED_DECREMENT, &args![count]).u32() == 0 {
        e.vcall(child, 4, &[]);
    }
    if e.call(INTERLOCKED_DECREMENT, &args![count]).u32() == 0 {
        e.vcall(child, 4, &[]);
    }
}

/// `0060bef0`: the pointer array's set-capacity (`thiscall`, `RET 4`: the new
/// capacity).
const POINTER_ARRAY_SET_CAPACITY: u32 = 0x0060_bef0;
/// `004b0310`: stores an element at an index (`thiscall`, `RET 8`: the index,
/// the address of the `NiPointer` to store).
const POINTER_ARRAY_SET_AT: u32 = 0x004b_0310;

// Translated from 00a5eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. On an `NiTArray` of `NiPointer`s (layout
/// [`crate::types::NiTArray`]) it adds the pointer `*element` in the first
/// empty slot: `thiscall`, one word (`RET 4`), the address of the `NiPointer`.
/// A null pointer answers `0xffffffff`. Otherwise it scans
/// `0..m_usSize` for a null slot; there it releases the slot's old object
/// (always null here, the branch is translated as the exe has it: when not the
/// same pointer and non-null, `InterlockedDecrement` of its count and, at
/// zero, vtable slot `+4`), stores the pointer with `InterlockedIncrement`
/// of its count, adds 1 to `m_usESize` and answers the index. Without an
/// empty slot it grows the array to `m_usSize + m_usGrowBy` (`0060bef0`) when
/// `m_usSize` (read before the scan) has reached `m_usMaxSize`, stores the
/// pointer at index `m_usSize` (`004b0310`) and answers that index.
pub fn fn_00a5eb20(e: &mut Engine, this: Ptr<crate::types::NiTArray>, element: Ptr) -> u32 {
    use crate::types::NiTArray;
    let value = e.mem.u32(element.addr());
    if value == 0 {
        return 0xffff_ffff;
    }
    let size = e.get(this, NiTArray::m_usSize);
    let mut index: u16 = 0;
    if index < size {
        let base = e.get(this, NiTArray::m_pBase);
        loop {
            let slot = base.wrapping_add(u32::from(index).wrapping_mul(4));
            if e.mem.u32(slot) == 0 {
                let old = e.mem.u32(slot);
                if old != value {
                    if old != 0
                        && e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                            .u32()
                            == 0
                    {
                        e.vcall(old, 4, &args![]);
                    }
                    let new = e.mem.u32(element.addr());
                    e.mem.set_u32(slot, new);
                    if new != 0 {
                        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
                    }
                }
                let filled = e.get(this, NiTArray::m_usESize);
                e.set(this, NiTArray::m_usESize, filled.wrapping_add(1));
                return u32::from(index);
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiTArray::m_usSize) {
                break;
            }
        }
    }
    let size = u32::from(size);
    if size >= u32::from(e.get(this, NiTArray::m_usMaxSize)) {
        let grow_by = u32::from(e.get(this, NiTArray::m_usGrowBy));
        e.call(
            POINTER_ARRAY_SET_CAPACITY,
            &args![this, grow_by.wrapping_add(size)],
        );
    }
    e.call(POINTER_ARRAY_SET_AT, &args![this, size, element]);
    size
}

/// The child array `m_kChildren` (+0x9c): its element block (+0xa0), the
/// 16-bit capacity (+0xa4), the 16-bit used size (+0xa6), the 16-bit count
/// (+0xa8) and the 16-bit grow-by (+0xaa), as the PC code reads them.
const CHILDREN: u32 = 0x9c;
const CHILDREN_BASE: u32 = 0xa0;
const CHILDREN_CAPACITY: u32 = 0xa4;
const CHILDREN_SIZE: u32 = 0xa6;
/// `NiAVObject` words the node code reads: the parent (+0x18), the world bound
/// (+0x20) and the flags (+0x30).
const AV_PARENT: u32 = 0x18;
const AV_WORLD_BOUND: u32 = 0x20;
const AV_FLAGS: u32 = 0x30;
/// The bound the copies fall back to when a child has none (`011f4288`).
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
/// `005467e0` (`thiscall` on the node, no stack words): runs before the first
/// child's bound is copied into the node's.
const BEFORE_BOUND_COPY_005467E0: u32 = 0x0054_67e0;
/// `NiBound::Merge` (`00a7f3f0`, `thiscall` on the bound, `RET 4`: the other).
const BOUND_MERGE: u32 = 0x00a7_f3f0;

/// The used size of the child array.
fn child_count(e: &Engine, node: u32) -> u32 {
    u32::from(e.mem.u16(node + CHILDREN_SIZE))
}

/// The child at `index` (read through the element block each time).
fn child_at(e: &Engine, node: u32, index: u32) -> u32 {
    let base = e.mem.u32(node + CHILDREN_BASE);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// `InterlockedIncrement` of the reference count at `object+4`.
fn take_reference(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// `InterlockedDecrement` of the reference count at `object+4`; at zero the
/// object's vtable slot `+4` (delete) runs.
fn drop_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The `FLDZ; FCOM(P) [bound+0xc]; TEST AH,0x44` test: whether the bound's
/// radius is not 0.0 (a NaN radius counts as not 0).
fn radius_not_zero(e: &Engine, bound: u32) -> bool {
    x87_m32(e.mem.u32(bound + 0xc)) != 0.0
}

/// The inline copy of a child's bound (or `011f4288` when it has none) into
/// the node's world bound: the centre words moved as they are, the radius
/// through `FLD`/`FSTP` (a signalling NaN is quieted), in the exe's order.
fn copy_bound(e: &mut Engine, node: u32, source: u32) {
    let source = if source != 0 {
        source
    } else {
        DEFAULT_BOUND_011F4288
    };
    let x = e.mem.u32(source);
    let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
    let target = e.mem.u32(node + AV_WORLD_BOUND);
    e.mem.set_u32(target, x);
    let y = e.mem.u32(source + 4);
    e.mem.set_u32(target + 4, y);
    let z = e.mem.u32(source + 8);
    e.mem.set_u32(target + 0xc, radius);
    e.mem.set_u32(target + 8, z);
}

/// Whether the node's flags let it merge child bounds: bit 13 clear and bit
/// 19 set.
fn merges_bounds(flags: u32) -> bool {
    (flags >> 0xd) & 1 == 0 && (flags >> 0x13) & 1 != 0
}

/// The shared child walk of the downward passes (`00a5dd70`, `00a5ded0`,
/// `00a5e0b0`). With bit 19 of the flags set: with bit 13 also set,
/// `005467e0` runs and every child goes through the merging walk; otherwise
/// the world bound's radius is set to 0.0 (only when `fresh`), and the
/// children go through `update` until the first one with a bound of a radius
/// not 0.0 (and `fresh`): `005467e0`, its bound copied into the node's, and
/// the rest through the merging walk. Without bit 19 every child goes through
/// the merging walk. The merging walk runs `update` on each child and merges
/// its bound (radius not 0.0) into the node's (`NiBound::Merge`) while the
/// node's flags (read again) allow it.
fn update_children(
    e: &mut Engine,
    node: u32,
    count: u32,
    fresh: bool,
    update: &mut dyn FnMut(&mut Engine, u32),
) {
    let flags = e.mem.u32(node + AV_FLAGS);
    let mut index = 0;
    if (flags >> 0x13) & 1 != 0 {
        if (flags >> 0xd) & 1 == 0 {
            let bound = e.mem.u32(node + AV_WORLD_BOUND);
            if fresh && bound != 0 {
                e.mem.set_u32(bound + 0xc, x87_store32(0.0));
            }
            if count == 0 {
                return;
            }
            loop {
                let child = child_at(e, node, index);
                if child != 0 {
                    update(e, child);
                    let bound = e.mem.u32(child + AV_WORLD_BOUND);
                    if bound != 0 && radius_not_zero(e, bound) && fresh {
                        e.call(BEFORE_BOUND_COPY_005467E0, &args![node]);
                        let source = e.mem.u32(child + AV_WORLD_BOUND);
                        copy_bound(e, node, source);
                        index += 1;
                        break;
                    }
                }
                index += 1;
                if index >= count {
                    return;
                }
            }
        } else {
            e.call(BEFORE_BOUND_COPY_005467E0, &args![node]);
        }
    }
    while index < count {
        let child = child_at(e, node, index);
        if child != 0 {
            update(e, child);
            if merges_bounds(e.mem.u32(node + AV_FLAGS)) {
                let bound = e.mem.u32(child + AV_WORLD_BOUND);
                if bound != 0 && radius_not_zero(e, bound) {
                    let own = e.mem.u32(node + AV_WORLD_BOUND);
                    e.call(BOUND_MERGE, &args![own, bound]);
                }
            }
        }
        index += 1;
    }
}

// Translated from 00a5db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdateUpwardPass` (engine map name), `thiscall`, no stack words:
/// reads the parent (+0x18) first, runs the node's vtable slot `+0xbc`
/// (`UpdateWorldBound`), then jumps to the parent's slot `+0xfc` (the same
/// pass upwards) when there is a parent.
pub fn ni_node_update_upward_pass(e: &mut Engine, this: Ptr) {
    let parent = e.mem.u32(this.addr() + AV_PARENT);
    e.vcall(this.addr(), 0xbc, &args![]);
    if parent != 0 {
        e.vcall(parent, 0xfc, &args![]);
    }
}

// Translated from 00a5e2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdateWorldBound` (engine map name), `thiscall`, no stack words.
/// Only when the flags (+0x30) have bit 13 clear and bit 19 set: finds the
/// first child with a world bound whose radius is not 0.0, runs `005467e0`,
/// copies that bound into the node's, and merges the later children's bounds
/// (radius not 0.0) into it (`NiBound::Merge`). The first search walks the
/// element block read once; the merging loop reads it again for each child.
pub fn ni_node_update_world_bound(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let flags = e.mem.u32(t + AV_FLAGS);
    if !merges_bounds(flags) {
        return;
    }
    let count = child_count(e, t);
    let mut index = 0;
    if count == 0 {
        return;
    }
    let mut slot = e.mem.u32(t + CHILDREN_BASE);
    let first = loop {
        let child = e.mem.u32(slot);
        if child != 0 {
            let bound = e.mem.u32(child + AV_WORLD_BOUND);
            if bound != 0 && radius_not_zero(e, bound) {
                break child;
            }
        }
        index += 1;
        slot = slot.wrapping_add(4);
        if index >= count {
            return;
        }
    };
    e.call(BEFORE_BOUND_COPY_005467E0, &args![this]);
    let source = e.mem.u32(first + AV_WORLD_BOUND);
    copy_bound(e, t, source);
    index += 1;
    while index < count {
        let child = child_at(e, t, index);
        if child != 0 {
            let bound = e.mem.u32(child + AV_WORLD_BOUND);
            if bound != 0 && radius_not_zero(e, bound) {
                let own = e.mem.u32(t + AV_WORLD_BOUND);
                e.call(BOUND_MERGE, &args![own, bound]);
            }
        }
        index += 1;
    }
}

/// `NiAVObject::UpdateObjectControllers` (`004eff50`, `thiscall`, `RET 8`: the
/// update data and a flag).
const AV_UPDATE_OBJECT_CONTROLLERS: u32 = 0x004e_ff50;

// Translated from 00a5dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdateDownwardPass` (engine map name), `thiscall`, `RET 8` (the
/// update data and the update flags). When the data's byte `+4` is set the
/// object controllers run (`004eff50` with 1); the node's vtable slot `+0xb8`
/// (`UpdateWorldData`) runs with the data; bit 0 of the node's flags is added
/// to the update flags. Then the children walk (each child's slot `+0xa4`
/// with the data and the flags) builds the world bound from theirs.
pub fn ni_node_update_downward_pass(e: &mut Engine, this: Ptr, data: u32, update_flags: u32) {
    let t = this.addr();
    if e.mem.u8(data + 4) != 0 {
        e.call(AV_UPDATE_OBJECT_CONTROLLERS, &args![this, data, 1u32]);
    }
    e.vcall(t, 0xb8, &args![data]);
    let mut update_flags = update_flags;
    if e.mem.u32(t + AV_FLAGS) & 1 != 0 {
        update_flags |= 1;
    }
    let count = child_count(e, t);
    update_children(e, t, count, true, &mut |e, child| {
        e.vcall(child, 0xa4, &args![data, update_flags]);
    });
}

// Translated from 00a5ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdateSelectedDownwardPass` (engine map name), `thiscall`, `RET 8`
/// (the update data and the update flags). The data's byte `+4` is set to bit
/// 3 of the node's flags for the object controllers (`004eff50` with 1) and
/// put back after; `fresh` is whether the data's byte `+5` is 0 (read after the
/// controllers). With bit 2 of the flags and `fresh`, the node's slot `+0xb8`
/// runs; bit 0 of the flags joins the update flags. The children walk: a child
/// with bit 1 of its flags runs its slot `+0xac` (bit 4 set) or `+0xa8` with
/// the data and the update flags; the bound is reset and taken from the first
/// child only when `fresh`.
pub fn ni_node_update_selected_downward_pass(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    update_flags: u32,
) {
    let t = this.addr();
    let saved = e.mem.u8(data + 4);
    let selected = ((e.mem.u32(t + AV_FLAGS) >> 3) & 1) as u8;
    e.mem.set_u8(data + 4, selected);
    e.call(AV_UPDATE_OBJECT_CONTROLLERS, &args![this, data, 1u32]);
    let fresh = e.mem.u8(data + 5) == 0;
    e.mem.set_u8(data + 4, saved);
    if (e.mem.u32(t + AV_FLAGS) >> 2) & 1 != 0 && fresh {
        e.vcall(t, 0xb8, &args![data]);
    }
    let mut update_flags = update_flags;
    if e.mem.u32(t + AV_FLAGS) & 1 != 0 {
        update_flags |= 1;
    }
    let count = child_count(e, t);
    update_children(e, t, count, fresh, &mut |e, child| {
        let child_flags = e.mem.u32(child + AV_FLAGS);
        if (child_flags >> 1) & 1 != 0 {
            let slot = if (child_flags >> 4) & 1 != 0 {
                0xac
            } else {
                0xa8
            };
            e.vcall(child, slot, &args![data, update_flags]);
        }
    });
}

// Translated from 00a5e0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdateTransformAndBounds` (engine map name), `thiscall`, `RET 4`
/// (the update data): with bit 2 of the flags the node's slot `+0xb8` runs
/// with the data; then the children walk with each child's slot `+0xc0`
/// (the data).
pub fn ni_node_update_transform_and_bounds(e: &mut Engine, this: Ptr, data: u32) {
    let t = this.addr();
    if (e.mem.u32(t + AV_FLAGS) >> 2) & 1 != 0 {
        e.vcall(t, 0xb8, &args![data]);
    }
    let count = child_count(e, t);
    update_children(e, t, count, true, &mut |e, child| {
        e.vcall(child, 0xc0, &args![data]);
    });
}

/// `NiAVObject::HasTransformController` (`00a5a0e0`) and
/// `NiAVObject::HasPropertyController` (`00a5a110`), `thiscall`, no stack
/// words, a byte answer each.
const AV_HAS_TRANSFORM_CONTROLLER: u32 = 0x00a5_a0e0;
const AV_HAS_PROPERTY_CONTROLLER: u32 = 0x00a5_a110;

// Translated from 00a5e440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::SetSelectiveUpdateFlags` (engine map name), `thiscall`, `RET 0xc`:
/// the address of the "selective update" answer, the "selective transforms"
/// word (its low byte) and the address of the "rigid" answer. The word's low
/// byte is set (when clear) from a transform controller (`00a5a0e0`) or bit 7
/// of the flags; it is the first answer, and the rigid answer starts at 1.
/// Each child's slot `+0xa0` runs with the address of a cleared byte, the
/// word and the address of a byte set to 1: a set first byte sets the first
/// answer, a clear second byte clears the rigid answer. Then the flags: bit 1
/// from a property controller (`00a5a110`) or a word at +0xc that is not 0, which also set
/// the first answer, else from the first answer; bit 2 from the word's byte;
/// bit 3 from the property controller; bit 4 from the rigid answer; with a
/// transform controller the rigid answer is cleared at the end.
pub fn ni_node_set_selective_update_flags(
    e: &mut Engine,
    this: Ptr,
    selective: u32,
    transforms: u32,
    rigid: u32,
) {
    let t = this.addr();
    let has_transform = e.call(AV_HAS_TRANSFORM_CONTROLLER, &args![this]).u32() as u8;
    let mut transforms = transforms;
    if transforms as u8 == 0 {
        if has_transform != 0 {
            transforms |= 1;
        } else {
            transforms &= !0xff;
            if (e.mem.u32(t + AV_FLAGS) >> 7) & 1 != 0 {
                transforms = (transforms & !0xff) | 1;
            }
        }
    }
    e.mem.set_u8(selective, transforms as u8);
    e.mem.set_u8(rigid, 1);
    let mut index = 0;
    e.with_stack(8, |e, frame| {
        // The frame: the first answer slot (the argument's own slot) and the
        // second byte.
        let child_selective = frame.addr();
        let child_rigid = child_selective + 4;
        e.mem.set_u32(child_selective, selective);
        while index < child_count(e, t) {
            let child = child_at(e, t, index);
            if child != 0 {
                e.mem.set_u8(child_selective, 0);
                e.mem.set_u8(child_rigid, 1);
                e.vcall(
                    child,
                    0xa0,
                    &args![child_selective, transforms, child_rigid],
                );
                if e.mem.u8(child_selective) != 0 {
                    e.mem.set_u8(selective, 1);
                }
                if e.mem.u8(child_rigid) == 0 {
                    e.mem.set_u8(rigid, 0);
                }
            }
            index += 1;
        }
    });
    let has_property = e.call(AV_HAS_PROPERTY_CONTROLLER, &args![this]).u32() as u8;
    let mut flags = e.mem.u32(t + AV_FLAGS);
    if has_property != 0 || e.mem.u32(t + 0xc) != 0 {
        e.mem.set_u8(selective, 1);
        flags |= 2;
    } else if e.mem.u8(selective) != 0 {
        flags |= 2;
    } else {
        flags &= !2;
    }
    e.mem.set_u32(t + AV_FLAGS, flags);
    let mut flags = e.mem.u32(t + AV_FLAGS);
    if transforms as u8 != 0 {
        flags |= 4;
    } else {
        flags &= !4;
    }
    if has_property != 0 {
        flags |= 8;
    } else {
        flags &= !8;
    }
    if e.mem.u8(rigid) != 0 {
        flags |= 0x10;
    } else {
        flags &= !0x10;
    }
    e.mem.set_u32(t + AV_FLAGS, flags);
    if has_transform != 0 {
        e.mem.set_u8(rigid, 0);
    }
}

/// `NiAVObject::UpdatePropertiesDownward` (`00a5a140`, `thiscall`, `RET 4`)
/// and `NiAVObject::PushLocalProperties` (`00a5a170`, `thiscall`, `RET 0xc`:
/// the parent state, a flag and the address of the new state).
const AV_UPDATE_PROPERTIES_DOWNWARD: u32 = 0x00a5_a140;
const AV_PUSH_LOCAL_PROPERTIES: u32 = 0x00a5_a170;
/// `NiPropertyState::~NiPropertyState` (`00a89890`) and
/// `NiPropertyState::NiPropertyState` (`00a89ab0`), `thiscall`, no stack words.
const PROPERTY_STATE_DESTRUCT: u32 = 0x00a8_9890;
const PROPERTY_STATE_CONSTRUCT: u32 = 0x00a8_9ab0;
/// The memory manager (`011f6238`), its `Allocate` (`00aa3e40`, `RET 4`: the
/// size) and `Deallocate` (`00aa4060`, `RET 4`: the block), `thiscall`.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

/// Frees a property state the node made for itself: when it is not
/// `parent_state` and not null, its destructor runs and the memory manager
/// takes it back.
fn free_own_state(e: &mut Engine, state: u32, parent_state: u32) {
    if parent_state != state && state != 0 {
        e.call(PROPERTY_STATE_DESTRUCT, &args![state]);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, state]);
    }
}

// Translated from 00a5db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdatePropertiesDownward` (engine map name), `thiscall`, `RET 4`
/// (the parent's property state): `NiAVObject::UpdatePropertiesDownward`,
/// then the node's own state (`PushLocalProperties` with 1, into a word that
/// starts at 0) is passed to each child's slot `+0xb0`; a state the node made
/// (not the parent's, not null) is destroyed and freed.
pub fn ni_node_update_properties_downward(e: &mut Engine, this: Ptr, parent_state: u32) {
    let t = this.addr();
    e.call(AV_UPDATE_PROPERTIES_DOWNWARD, &args![this, parent_state]);
    let state = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        e.call(
            AV_PUSH_LOCAL_PROPERTIES,
            &args![this, parent_state, 1u32, slot],
        );
        let mut index = 0;
        while index < child_count(e, t) {
            let child = child_at(e, t, index);
            if child != 0 {
                let state = e.mem.u32(slot.addr());
                e.vcall(child, 0xb0, &args![state]);
            }
            index += 1;
        }
        e.mem.u32(slot.addr())
    });
    free_own_state(e, state, parent_state);
}

// Translated from 00a5e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdatePropertiesUpward` (engine map name), `thiscall`, `RET 4`
/// (the address of the state to fill). Sets bit 18 of the flags from
/// `HasPropertyController`. The parent's state comes from the parent's own
/// upward pass (recursively, into a stack word), or without a parent from a
/// new `NiPropertyState` (0x1c bytes from the memory manager; 0 when that
/// fails). `PushLocalProperties` (with 0) fills the answer from it; the
/// parent's state is destroyed and freed when it is not the answer and not
/// null.
pub fn ni_node_update_properties_upward(e: &mut Engine, this: Ptr, state_out: u32) {
    let t = this.addr();
    let parent_state = e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, 0);
        let has_controller = e.call(AV_HAS_PROPERTY_CONTROLLER, &args![this]).u32() as u8;
        let flags = e.mem.u32(t + AV_FLAGS);
        let flags = if has_controller != 0 {
            flags | 0x40000
        } else {
            flags & 0xfffb_ffff
        };
        e.mem.set_u32(t + AV_FLAGS, flags);
        let parent = e.mem.u32(t + AV_PARENT);
        if parent != 0 {
            ni_node_update_properties_upward(e, Ptr::new(parent), slot);
        } else {
            let block = e
                .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x1cu32])
                .u32();
            let state = if block != 0 {
                e.call(PROPERTY_STATE_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(slot, state);
        }
        e.mem.u32(slot)
    });
    e.call(
        AV_PUSH_LOCAL_PROPERTIES,
        &args![this, parent_state, 0u32, state_out],
    );
    let current = e.mem.u32(state_out);
    free_own_state(e, parent_state, current);
}

/// `NiAVObject::LoadBinary` (`00a5a510`, `thiscall`, the stream) and
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`, `thiscall` on the stream).
const AV_LOAD_BINARY: u32 = 0x00a5_a510;
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;

// Translated from 00a5dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::LoadBinary` (engine map name), `thiscall`, `RET 4` (the stream):
/// `NiAVObject::LoadBinary`, two `ReadMultipleLinkIDs` (the children and the
/// effects), then bit 21 of the flags is cleared.
pub fn ni_node_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(AV_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    let flags = e.mem.u32(this.addr() + AV_FLAGS) & 0xffdf_ffff;
    e.mem.set_u32(this.addr() + AV_FLAGS, flags);
}

/// `NiAVObject::LinkObject` (`00a5a7f0`, `thiscall`, the stream),
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`) and
/// `NiStream::GetObjectFromLinkID` (`00a63430`), `thiscall` on the stream.
const AV_LINK_OBJECT: u32 = 0x00a5_a7f0;
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;

// Translated from 00a5e9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::LinkObject` (engine map name), `thiscall`, `RET 4` (the stream):
/// `NiAVObject::LinkObject`; with a link count the child array is sized
/// (`0060bef0`) and each linked object stored through the node's slot
/// `+0xf8` (`SetAt` with the index); then as many objects as the next link
/// count are read and dropped. The file version test (`stream+0xd8` below
/// 0x04010008) picks between two identical loops, kept as the exe has them.
#[allow(clippy::if_same_then_else)] // the exe's two branches are identical
pub fn ni_node_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let t = this.addr();
    e.call(AV_LINK_OBJECT, &args![this, stream]);
    let count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    if count != 0 {
        e.call(CHILD_ARRAY_SET_SIZE, &args![t + CHILDREN, count]);
        for index in 0..count {
            let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.vcall(t, 0xf8, &args![index, object]);
        }
    }
    let mut remaining = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    if e.mem.u32(stream + 0xd8) < 0x0401_0008 {
        while remaining != 0 {
            remaining -= 1;
            e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]);
        }
    } else {
        while remaining != 0 {
            remaining -= 1;
            e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]);
        }
    }
}

/// `NiAVObject::GetObjectByName` (`00a59e20`, `thiscall`, `RET 4`: the name).
const AV_GET_OBJECT_BY_NAME: u32 = 0x00a5_9e20;

// Translated from 00a5e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::GetObjectByName` (engine map name), `thiscall`, `RET 4` (the
/// name): the node itself when `NiAVObject::GetObjectByName` finds it, else
/// the first child whose slot `+0x9c` (the same search) finds something; 0.
pub fn ni_node_get_object_by_name(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let t = this.addr();
    let found = e.call(AV_GET_OBJECT_BY_NAME, &args![this, name]).u32();
    if found != 0 {
        return found;
    }
    let mut index = 0;
    while index < child_count(e, t) {
        let child = child_at(e, t, index);
        if child != 0 {
            let found = e.vcall(child, 0x9c, &args![name]).u32();
            if found != 0 {
                return found;
            }
        }
        index += 1;
    }
    0
}

/// `NiAVObject::Cull` (`00a59e00`, `thiscall` on the child, `RET 4`: the
/// culling process).
const AV_CULL: u32 = 0x00a5_9e00;

// Translated from 00a5dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::OnVisible` (engine map name), `thiscall`, `RET 4` (the culling
/// process): only with a world bound whose radius is not 0.0 (a NaN radius
/// goes on), each child (count read once) is culled with the process
/// (`NiAVObject::Cull`).
pub fn ni_node_on_visible(e: &mut Engine, this: Ptr, culler: u32) {
    let t = this.addr();
    let bound = e.mem.u32(t + AV_WORLD_BOUND);
    if bound == 0 || !radius_not_zero(e, bound) {
        return;
    }
    let count = child_count(e, t);
    for index in 0..count {
        let child = child_at(e, t, index);
        if child != 0 {
            e.call(AV_CULL, &args![child, culler]);
        }
    }
}

/// Releases the `NiPointer` held in a stack word after a detach.
fn release_detached(e: &mut Engine, word: u32) {
    let object = e.mem.u32(word);
    if object != 0 {
        drop_reference(e, object);
    }
}

// Translated from 00a5d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::DetachChild` (engine map name), `thiscall`, `RET 4` (the child):
/// the node's slot `+0xe4` (the detach that hands the child back) with the
/// child and the address of a word starting at 0, then that word's object
/// gives up its reference.
pub fn ni_node_detach_child(e: &mut Engine, this: Ptr, child: u32) {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.vcall(this.addr(), 0xe4, &args![child, word]);
        release_detached(e, word.addr());
    });
}

// Translated from 00a5d8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::DetachChildAt` (engine map name), `thiscall`, `RET 4` (the index):
/// the node's slot `+0xec` with the index and the address of a word starting
/// at 0, then that word's object gives up its reference.
pub fn ni_node_detach_child_at(e: &mut Engine, this: Ptr, index: u32) {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.vcall(this.addr(), 0xec, &args![index, word]);
        release_detached(e, word.addr());
    });
}

// Translated from 00a93ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::SetAt_ov2` (engine map name), `thiscall`, `RET 8` (the index and
/// the child): the node's slot `+0xf4` with both and the address of a word
/// starting at 0 (the child that was there), then that word's object gives up
/// its reference.
pub fn ni_node_set_at_ov2(e: &mut Engine, this: Ptr, index: u32, child: u32) {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        e.vcall(this.addr(), 0xf4, &args![index, child, word]);
        release_detached(e, word.addr());
    });
}

/// `NiTArray<NiPointer<NiAVObject>>::RemoveAt` (`00869260`, `thiscall` on the
/// array, `RET 8`: the address of the word that gets the removed pointer, and
/// the index).
const CHILD_ARRAY_REMOVE_AT: u32 = 0x0086_9260;

/// The common end of the detaches: `out` gets `child` (the old object there
/// giving up its reference, the child taking one), and the child gives up the
/// reference the detach held.
fn hand_back(e: &mut Engine, out: u32, child: u32) {
    let old = e.mem.u32(out);
    if old != child {
        if old != 0 {
            drop_reference(e, old);
        }
        e.mem.set_u32(out, child);
        if child == 0 {
            return;
        }
        take_reference(e, child);
    }
    if child != 0 {
        drop_reference(e, child);
    }
}

/// Clears the hand-back word (its object giving up its reference) when there
/// is nothing to hand back.
fn clear_hand_back(e: &mut Engine, out: u32) {
    let old = e.mem.u32(out);
    if old != 0 {
        drop_reference(e, old);
        e.mem.set_u32(out, 0);
    }
}

// Translated from 00a5e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::DetachChildAt_ov2` (engine map name), `thiscall`, `RET 8` (the
/// index and the address of the word that gets the child). An index past the
/// used size clears that word. Otherwise the child there (when not null)
/// takes a reference, loses its parent (+0x18), and is removed from the array
/// (`RemoveAt`, into the argument's own slot, whose object then gives up its
/// reference); the word gets the child.
pub fn ni_node_detach_child_at_ov2(e: &mut Engine, this: Ptr, index: u32, out: u32) {
    let t = this.addr();
    if index >= child_count(e, t) {
        clear_hand_back(e, out);
        return;
    }
    let child = child_at(e, t, index);
    if child != 0 {
        take_reference(e, child);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), index);
            e.mem.set_u32(child + AV_PARENT, 0);
            e.call(CHILD_ARRAY_REMOVE_AT, &args![t + CHILDREN, slot, index]);
            release_detached(e, slot.addr());
        });
    }
    hand_back(e, out, child);
}

// Translated from 00a5e780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::DetachChild_ov2` (engine map name), `thiscall`, `RET 8` (the child
/// and the address of the word that gets it). Looks through the children
/// (each takes a reference while it is looked at, given up when it is not the
/// one); the one found loses its parent, is removed (`RemoveAt` into the
/// argument's own slot, whose object gives up its reference) and goes to the
/// word. Not found clears the word.
pub fn ni_node_detach_child_ov2(e: &mut Engine, this: Ptr, child: u32, out: u32) {
    let t = this.addr();
    let mut index = 0;
    while index < child_count(e, t) {
        let current = child_at(e, t, index);
        if current != 0 {
            take_reference(e, current);
            if current == child {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), child);
                    e.mem.set_u32(current + AV_PARENT, 0);
                    e.call(CHILD_ARRAY_REMOVE_AT, &args![t + CHILDREN, slot, index]);
                    release_detached(e, slot.addr());
                });
                let old = e.mem.u32(out);
                if old != current {
                    if old != 0 {
                        drop_reference(e, old);
                    }
                    e.mem.set_u32(out, current);
                    take_reference(e, current);
                }
                drop_reference(e, current);
                return;
            }
            drop_reference(e, current);
        }
        index += 1;
    }
    clear_hand_back(e, out);
}

// Translated from 00a5ee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::SetAt` (engine map name), `thiscall`, `RET 0xc` (the index, the
/// child and the address of the word that gets the child that was there).
/// The new child (when not null) gets the node as parent (`AttachParent`) and
/// a reference for the store. An index within the used size: the old child
/// there (when not null) takes a reference and loses its parent, the new one
/// is stored (`004b0310`, from the argument's own slot) and gives up its
/// store reference, and the word gets the old child. Past the used size: the
/// array grows (`0060bef0`, to the index plus the grow-by) when the index is
/// not below the capacity, the child is stored the same way and the word is
/// cleared.
pub fn ni_node_set_at(e: &mut Engine, this: Ptr, index: u32, child: u32, out: u32) {
    let t = this.addr();
    let array = t + CHILDREN;
    if child_count(e, t) > index {
        let old = child_at(e, t, index);
        if old != 0 {
            take_reference(e, old);
            e.mem.set_u32(old + AV_PARENT, 0);
        }
        store_child(e, this, index, child);
        hand_back(e, out, old);
        return;
    }
    if child != 0 {
        e.call(NI_AV_OBJECT_ATTACH_PARENT, &args![child, this]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), child);
        if child != 0 {
            take_reference(e, child);
        }
        let capacity = u32::from(e.mem.u16(t + CHILDREN_CAPACITY));
        if index >= capacity {
            let grow = u32::from(e.mem.u16(array + 0xe));
            e.call(
                CHILD_ARRAY_SET_SIZE,
                &args![array, grow.wrapping_add(index)],
            );
        }
        e.call(CHILD_ARRAY_SET_AT, &args![array, index, slot]);
    });
    if child != 0 {
        drop_reference(e, child);
    }
    clear_hand_back(e, out);
}

/// The in-range store of `SetAt`: parent, store reference, `004b0310` from a
/// stack word holding the child, the store reference given up.
fn store_child(e: &mut Engine, this: Ptr, index: u32, child: u32) {
    if child != 0 {
        e.call(NI_AV_OBJECT_ATTACH_PARENT, &args![child, this]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), child);
        if child != 0 {
            take_reference(e, child);
        }
        e.call(
            CHILD_ARRAY_SET_AT,
            &args![this.addr() + CHILDREN, index, slot],
        );
    });
    if child != 0 {
        drop_reference(e, child);
    }
}

/// `NiGeometry::CopyMembers` (engine map name; `00a5ac90`, `thiscall`, `RET 8`:
/// the clone and the cloning process).
const AV_COPY_MEMBERS: u32 = 0x00a5_ac90;

// Translated from 00a5dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the node's copy-members step of cloning;
/// `thiscall`, `RET 8`: the clone and the cloning process): the base copy
/// (`00a5ac90`), then each child is cloned (its slot `+0x48` with the process)
/// and stored in the clone at the same index (the clone's slot `+0xf8`).
pub fn fn_00a5dc80(e: &mut Engine, this: Ptr, clone: u32, cloning: u32) {
    let t = this.addr();
    e.call(AV_COPY_MEMBERS, &args![this, clone, cloning]);
    let mut index = 0;
    while index < child_count(e, t) {
        let child = child_at(e, t, index);
        if child != 0 {
            let copy = e.vcall(child, 0x48, &args![cloning]).u32();
            e.vcall(clone, 0xf8, &args![index, copy]);
        }
        index += 1;
    }
}

/// `NiAVObject::ProcessClone` (`00a59ec0`, `thiscall`, `RET 4`) and the clone
/// map lookup `00a53c30` (`thiscall` on the map at `*cloning`, `RET 8`: the
/// key and the address of the word that gets the value).
const AV_PROCESS_CLONE: u32 = 0x00a5_9ec0;
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;

// Translated from 00a5dce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::ProcessClone` (engine map name), `thiscall`, `RET 4` (the cloning
/// process): `NiAVObject::ProcessClone`, the node's clone looked up in the
/// process's map (`00a53c30`, into the argument's own slot; not used after),
/// then each child's slot `+0x68` with the process.
pub fn ni_node_process_clone(e: &mut Engine, this: Ptr, cloning: u32) {
    let t = this.addr();
    e.call(AV_PROCESS_CLONE, &args![this, cloning]);
    let map = e.mem.u32(cloning);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), cloning);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
    });
    let mut index = 0;
    while index < child_count(e, t) {
        let child = child_at(e, t, index);
        if child != 0 {
            e.vcall(child, 0x68, &args![cloning]);
        }
        index += 1;
    }
}

/// The Gamebryo allocation (`00aa13e0`, `cdecl`: the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The vtable the child array gets in a new clone (`0101fbd4`).
const CHILD_ARRAY_VTABLE_0101FBD4: u32 = 0x0101_fbd4;

// Translated from 00a5efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::CreateClone` (engine map name), `thiscall`, `RET 4` (the cloning
/// process): 0xac bytes (`00aa13e0`); when that succeeds `NiAVObject`'s
/// constructor, `NiNode`'s vtable, an empty child array (grow-by 1, vtable
/// `0101fbd4`), then the copy (`00a5dc80`) into it; a failed allocation still
/// runs the copy with a null clone. Answers the clone.
pub fn ni_node_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    let clone = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    if clone == 0 {
        fn_00a5dc80(e, this, 0, cloning);
        return 0;
    }
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![clone]);
    e.mem.set_u32(clone, NI_NODE_VTABLE);
    e.mem.set_u16(clone + 0xaa, 1);
    e.mem.set_u16(clone + CHILDREN_CAPACITY, 0);
    e.mem.set_u16(clone + CHILDREN_SIZE, 0);
    e.mem.set_u16(clone + 0xa8, 0);
    e.mem.set_u32(clone + CHILDREN_BASE, 0);
    e.mem.set_u32(clone + CHILDREN, CHILD_ARRAY_VTABLE_0101FBD4);
    fn_00a5dc80(e, this, clone, cloning);
    clone
}

/// The vtable the child array gets while the node is taken down (`0101fbcc`,
/// RTTI `NiTArray<NiPointer<NiAVObject>, ..>`), the array's block delete
/// (`004b0220`, `cdecl`: the block) and the `NiAVObject` destructor body
/// (`00a5a390`, `thiscall`).
const CHILD_ARRAY_VTABLE_0101FBCC: u32 = 0x0101_fbcc;
const CHILD_ARRAY_DELETE_BLOCK: u32 = 0x004b_0220;
const NI_AV_OBJECT_DESTRUCT: u32 = 0x00a5_a390;

// Translated from 00a5e930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiNode`'s destructor body (`thiscall`, no stack
/// words; `NiNode::_scalar_deleting_destructor_` runs it). Stores `NiNode`'s
/// vtable, detaches every child through the node's slot `+0xec` (index, the
/// address of a word starting at 0; that word's object gives up its
/// reference), stores the array vtable `0101fbcc`, deletes the element block
/// (`004b0220`) and jumps to the `NiAVObject` destructor body.
pub fn fn_00a5e930(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, NI_NODE_VTABLE);
    let mut index = 0;
    while index < child_count(e, t) {
        e.with_stack(4, |e, word| {
            e.mem.set_u32(word.addr(), 0);
            e.vcall(t, 0xec, &args![index, word]);
            release_detached(e, word.addr());
        });
        index += 1;
    }
    let block = e.mem.u32(t + CHILDREN_BASE);
    e.mem.set_u32(t + CHILDREN, CHILD_ARRAY_VTABLE_0101FBCC);
    e.call(CHILD_ARRAY_DELETE_BLOCK, &args![block]);
    e.call(NI_AV_OBJECT_DESTRUCT, &args![this]);
}

/// `NiNode::ms_RTTI` (`011f4428`).
const NI_NODE_RTTI: u32 = 0x011f_4428;

// Translated from 00a5e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::GetRTTI` (engine map name): the address of the class's `NiRTTI`
/// record (`011f4428`).
pub fn ni_node_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_NODE_RTTI)
}

/// `NiMatrix3::operator*` (`0043f8d0`, `thiscall` on the left matrix: result, right) and
/// `NiMatrix3::TransposeTimes` (`00a588a0`, `thiscall`: result, other).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const MATRIX_TRANSPOSE_TIMES: u32 = 0x00a5_88a0;
/// `NiAVObject`'s local transform as the PC code reads it: the rotation (+0x34, nine
/// floats), the translation (+0x58, three floats) and the scale (+0x64).
const AV_LOCAL_ROTATE: u32 = 0x34;
const AV_LOCAL_TRANSLATE: u32 = 0x58;
const AV_LOCAL_SCALE: u32 = 0x64;
/// `NiAVObject` vtable slots the children are called through: `PurgeRendererData` (+0xd8),
/// `ApplyTransform` (+0x90), `UpdateControllers` (+0x8c), `UpdateRigidDownwardPass` (+0xac),
/// `RegisterStreamables` (+0x54), `IsEqual` (+0x5c); and the node's own slot +0xb8 (run by
/// the rigid pass with the update data).
const SLOT_PURGE_RENDERER_DATA: u32 = 0xd8;
const SLOT_APPLY_TRANSFORM: u32 = 0x90;
const SLOT_UPDATE_CONTROLLERS: u32 = 0x8c;
const SLOT_UPDATE_RIGID_DOWNWARD_PASS: u32 = 0xac;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_RIGID_PASS_OWN: u32 = 0xb8;
/// `NiStream::SaveLinkID` (vtable +0x30, the object).
const SLOT_SAVE_LINK_ID: u32 = 0x30;
/// The `NiAVObject` versions the node functions start with (`thiscall` on the node):
/// `PurgeRendererData` (`00a59e50`), `RegisterStreamables` (`00a59f00`), `IsEqual`
/// (`00a5a260`), `SaveBinary` (`00a5a900`) and `GetViewerStrings` (`00a5ad00`).
const AV_PURGE_RENDERER_DATA: u32 = 0x00a5_9e50;
const AV_REGISTER_STREAMABLES: u32 = 0x00a5_9f00;
const AV_IS_EQUAL: u32 = 0x00a5_a260;
const AV_SAVE_BINARY: u32 = 0x00a5_a900;
const AV_GET_VIEWER_STRINGS: u32 = 0x00a5_ad00;
/// `NiStream::m_pkOstr` (+0x250) and the binary stream's write function (+0xc) and
/// position (+4).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
const BINARY_STREAM_POSITION: u32 = 0x4;
/// Viewer string builders (`cdecl`): `NiGetViewerString` (`00a7d330`, the class name) and
/// `NiGetViewerString_ov2` (`00a7d370`, a label and a flag word).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// `"m_bVisual"` (`0109b6ac`).
const TEXT_VISUAL: u32 = 0x0109_b6ac;
/// `NiTArray<char *>` resize (`0096ad30`) and `SetAt` (`0096ae90`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;

/// Calls `call` on each non-null child, the used size read again after each one.
fn for_each_child(e: &mut Engine, node: u32, call: &mut dyn FnMut(&mut Engine, u32)) {
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        if child != 0 {
            call(e, child);
        }
        index += 1;
    }
}

// Translated from 00a5d940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::ApplyTransform` (Xbox PDB; (matrix, translation, flag)): without the flag every
/// child gets `ApplyTransform(matrix, translation, 1)` (vtable +0x90). With it the
/// transform is moved into the node's frame: the matrix `R^T * (M * R)` (`R` the local
/// rotation, `NiMatrix3::operator*` and `TransposeTimes`) and the translation
/// `R^T (M T + t - T) / s` (`T` the local translation, `s` the local scale, each step
/// stored as a float in the exe's order), and every child gets those with 1.
pub fn ni_node_apply_transform(
    e: &mut Engine,
    this: Ptr,
    matrix: Ptr,
    translation: Ptr,
    flag: u32,
) {
    let node = this.addr();
    if flag as u8 == 0 {
        for_each_child(e, node, &mut |e, child| {
            e.vcall(
                child,
                SLOT_APPLY_TRANSFORM,
                &args![matrix, translation, 1u32],
            );
        });
        return;
    }
    e.with_stack(0x6c, |e, frame| {
        let moved_translation = frame.addr() + 0x18;
        let moved_matrix = frame.addr() + 0x24;
        let product = frame.addr() + 0x48;
        let rotate = node + AV_LOCAL_ROTATE;
        let first = e
            .call(MATRIX_MULTIPLY, &args![matrix, product, rotate])
            .u32();
        e.call(MATRIX_TRANSPOSE_TIMES, &args![rotate, moved_matrix, first]);
        let (m, t) = (matrix.addr(), node + AV_LOCAL_TRANSLATE);
        let ld = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
        let op = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
        let through_float = |value: f64| x87_load32(x87_store32(value));
        // `M T`, row by row.
        let mut rotated = [0u32; 3];
        let products: [[(u32, u32); 3]; 3] = [
            [(m + 4, t + 4), (t, m), (m + 8, t + 8)],
            [(m + 0xc, t), (m + 0x10, t + 4), (m + 0x14, t + 8)],
            [(m + 0x18, t), (m + 0x1c, t + 4), (m + 0x20, t + 8)],
        ];
        for (row, terms) in products.iter().enumerate() {
            let term = |(loaded, operand): (u32, u32)| x87_mul(ld(e, loaded), op(e, operand));
            rotated[row] = x87_store32(x87_add(
                x87_add(term(terms[0]), term(terms[1])),
                term(terms[2]),
            ));
        }
        // `+ t`, then `- T`.
        let mut offset = [0f64; 3];
        for axis in 0..3u32 {
            let shifted = x87_store32(x87_add(
                ld(e, translation.addr() + 4 * axis),
                x87_m32(rotated[axis as usize]),
            ));
            offset[axis as usize] =
                through_float(x87_sub(x87_load32(shifted), op(e, t + 4 * axis)));
        }
        let [c0, c1, c2] = offset;
        // `R^T` times the offset.
        let r = rotate;
        let d0 = through_float(x87_add(
            x87_mul(ld(e, r + 0x18), c2),
            x87_add(x87_mul(ld(e, r + 0xc), c1), x87_mul(c0, op(e, r))),
        ));
        let d1 = through_float(x87_add(
            x87_add(x87_mul(ld(e, r + 0x10), c1), x87_mul(ld(e, r + 4), c0)),
            x87_mul(ld(e, r + 0x1c), c2),
        ));
        let d2 = through_float(x87_add(
            x87_mul(c2, ld(e, r + 0x20)),
            x87_add(x87_mul(c1, ld(e, r + 0x14)), x87_mul(c0, op(e, r + 8))),
        ));
        let inverse = through_float(x87_div(1.0, ld(e, node + AV_LOCAL_SCALE)));
        let out = moved_translation;
        e.mem.set_u32(out, x87_store32(x87_mul(inverse, d0)));
        e.mem.set_u32(out + 4, x87_store32(x87_mul(d1, inverse)));
        e.mem.set_u32(out + 8, x87_store32(x87_mul(inverse, d2)));
        for_each_child(e, node, &mut |e, child| {
            e.vcall(
                child,
                SLOT_APPLY_TRANSFORM,
                &args![moved_matrix, moved_translation, 1u32],
            );
        });
    });
}

// Translated from 00a5dc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::PurgeRendererData` (Xbox PDB): `NiAVObject::PurgeRendererData` (`00a59e50`),
/// then `PurgeRendererData` (vtable +0xd8) of every child, with the renderer.
pub fn ni_node_purge_renderer_data(e: &mut Engine, this: Ptr, renderer: u32) {
    e.call(AV_PURGE_RENDERER_DATA, &args![this, renderer]);
    for_each_child(e, this.addr(), &mut |e, child| {
        e.vcall(child, SLOT_PURGE_RENDERER_DATA, &args![renderer]);
    });
}

// Translated from 00a5e1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdateRigidDownwardPass` (Xbox PDB; (update data, flags)): the data's byte +4
/// set to bit 3 of the node's flags (+0x30) around `NiAVObject::UpdateObjectControllers`
/// (`004eff50`, with 1) and put back; with flag bit 2 the node's slot +0xb8 with the data;
/// flag bit 0 adds bit 0 to the flags passed on. Then every child with its flag bit 1 set
/// gets `UpdateRigidDownwardPass` (vtable +0xac) with the data and those flags (the child
/// count is read once).
pub fn ni_node_update_rigid_downward_pass(e: &mut Engine, this: Ptr, data: u32, flags: u32) {
    let node = this.addr();
    let saved = e.mem.u8(data + 4);
    let bit3 = ((e.mem.u32(node + AV_FLAGS) >> 3) & 1) as u8;
    e.mem.set_u8(data + 4, bit3);
    e.call(AV_UPDATE_OBJECT_CONTROLLERS, &args![this, data, 1u32]);
    e.mem.set_u8(data + 4, saved);
    if (e.mem.u32(node + AV_FLAGS) >> 2) & 1 != 0 {
        e.vcall(node, SLOT_RIGID_PASS_OWN, &args![data]);
    }
    let mut flags = flags;
    if e.mem.u8(node + AV_FLAGS) & 1 != 0 {
        flags |= 1;
    }
    let count = child_count(e, node);
    for index in 0..count {
        let child = child_at(e, node, index);
        if child != 0 && (e.mem.u32(child + AV_FLAGS) >> 1) & 1 != 0 {
            e.vcall(child, SLOT_UPDATE_RIGID_DOWNWARD_PASS, &args![data, flags]);
        }
    }
}

// Translated from 00a5e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::UpdateControllers` (Xbox PDB): `NiAVObject::UpdateObjectControllers`
/// (`004eff50`, with 1), then `UpdateControllers` (vtable +0x8c) of every child (the child
/// count is read once).
pub fn ni_node_update_controllers(e: &mut Engine, this: Ptr, data: u32) {
    let node = this.addr();
    e.call(AV_UPDATE_OBJECT_CONTROLLERS, &args![this, data, 1u32]);
    let count = child_count(e, node);
    for index in 0..count {
        let child = child_at(e, node, index);
        if child != 0 {
            e.vcall(child, SLOT_UPDATE_CONTROLLERS, &args![data]);
        }
    }
}

// Translated from 00a5e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::RegisterStreamables` (Xbox PDB): false when `NiAVObject::RegisterStreamables`
/// (`00a59f00`) is; otherwise every child's `RegisterStreamables` (vtable +0x54) with the
/// stream, and true.
pub fn ni_node_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    if !e.call(AV_REGISTER_STREAMABLES, &args![this, stream]).bool() {
        return false;
    }
    for_each_child(e, this.addr(), &mut |e, child| {
        e.vcall(child, SLOT_REGISTER_STREAMABLES, &args![stream]);
    });
    true
}

// Translated from 00a5e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::IsEqual` (Xbox PDB): `NiAVObject::IsEqual` (`00a5a260`), the same child count,
/// and child by child both null or both present with the first's `IsEqual` (vtable +0x5c)
/// true for the second.
pub fn ni_node_is_equal(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if !e.call(AV_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let node = this.addr();
    let count = child_count(e, node);
    if count != child_count(e, other) {
        return false;
    }
    for index in 0..count {
        let mine = child_at(e, node, index);
        let theirs = child_at(e, other, index);
        if mine == 0 {
            if theirs != 0 {
                return false;
            }
        } else if theirs == 0 || !e.vcall(mine, SLOT_IS_EQUAL, &args![theirs]).bool() {
            return false;
        }
    }
    true
}

/// One 4-byte write to the stream's output (`NiStream` +0x250): its write function
/// (+0xc) with (output, the word's address, 4, the address of a count local set to 4, 1),
/// the result added to its position (+4).
fn write_word(e: &mut Engine, stream: u32, word: u32, count: u32) {
    let output = e.mem.u32(stream + STREAM_OUTPUT);
    let function = e.mem.u32(output + BINARY_STREAM_WRITE);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![output, word, 4u32, count, 1u32])
        .u32();
    let position = e
        .mem
        .u32(output + BINARY_STREAM_POSITION)
        .wrapping_add(done);
    e.mem.set_u32(output + BINARY_STREAM_POSITION, position);
}

// Translated from 00a5e880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::SaveBinary` (Xbox PDB): `NiAVObject::SaveBinary` (`00a5a900`), the child count
/// as a 4-byte word, every child's link id (`NiStream::SaveLinkID`, vtable +0x30, nulls
/// included), then a 4-byte 0.
pub fn ni_node_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let node = this.addr();
    e.call(AV_SAVE_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        let (count, zero, size) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        let children = child_count(e, node);
        e.mem.set_u32(size, children);
        write_word(e, stream, size, count);
        for index in 0..children {
            let child = child_at(e, node, index);
            e.vcall(stream, SLOT_SAVE_LINK_ID, &args![child]);
        }
        e.mem.set_u32(zero, 0);
        write_word(e, stream, zero, count);
    });
}

/// Adds the string in the word at `slot` to the viewer strings: the array grown by its
/// grow-by count when full (`0096ad30`), then `SetAt` (`0096ae90`) at the old size.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let index = u32::from(e.mem.u16(strings + 0xa));
    if index >= u32::from(e.mem.u16(strings + 8)) {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a5ea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::GetViewerStrings` (Xbox PDB): after `NiAVObject::GetViewerStrings`
/// (`00a5ad00`), adds the class name (`NiNode`'s RTTI name) and `"m_bVisual"`: whether
/// the world bound (+0x20) exists with a radius not 0 (a NaN radius counts as visual).
pub fn ni_node_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let node = this.addr();
    e.call(AV_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_NODE_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let bound = e.mem.u32(node + AV_WORLD_BOUND);
        let visual = bound != 0 && radius_not_zero(e, bound);
        let text = e
            .call(VIEWER_STRING_BOOL, &args![TEXT_VISUAL, u32::from(visual)])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
}

// Translated from 00a5ebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on an `NiTArray` of `NiPointer`s, (index, address
/// of the pointer)): inserts the pointer at `index`. A null pointer answers `0xffffffff`.
/// Otherwise every element from the used size down to `index + 1` takes the one before it
/// (through a smart-pointer temporary: its count raised, `SetAt` `004b0310`, the count
/// dropped with `DeleteThis`, slot +4, at zero), the array grown first by its grow-by
/// count (`0060bef0`) when the used size has reached the capacity; then the pointer goes in
/// at `index` (the array grown the same way when `index` has reached the capacity). Answers
/// `index`.
pub fn fn_00a5ebe0(e: &mut Engine, this: Ptr, index: u32, element: Ptr) -> u32 {
    let array = this.addr();
    if e.mem.u32(element.addr()) == 0 {
        return 0xffff_ffff;
    }
    let size = u32::from(e.mem.u16(array + 0xa));
    let mut position = size;
    if position > index {
        e.with_stack(4, |e, temp| loop {
            let base = e.mem.u32(array + 4);
            let moved = e
                .mem
                .u32(base.wrapping_add(position.wrapping_mul(4)).wrapping_sub(4));
            e.mem.set_u32(temp.addr(), moved);
            if moved != 0 {
                take_reference(e, moved);
            }
            if position == size && position >= u32::from(e.mem.u16(array + 8)) {
                let grow = u32::from(e.mem.u16(array + 0xe));
                e.call(CHILD_ARRAY_SET_SIZE, &args![array, grow + position]);
            }
            e.call(CHILD_ARRAY_SET_AT, &args![array, position, temp]);
            if moved != 0 {
                drop_reference(e, moved);
            }
            position -= 1;
            if position <= index {
                break;
            }
        });
    }
    if index >= u32::from(e.mem.u16(array + 8)) {
        let grow = u32::from(e.mem.u16(array + 0xe));
        e.call(
            CHILD_ARRAY_SET_SIZE,
            &args![array, grow.wrapping_add(index)],
        );
    }
    e.call(CHILD_ARRAY_SET_AT, &args![array, index, element]);
    index
}

// Translated from 00a5edc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::InsertChildAt` (Xbox PDB; (index, child)): nothing for a null child. Otherwise
/// the child's count is raised, `NiAVObject::AttachParent(child, this)` (`00a59d00`) runs,
/// a smart-pointer temporary to the child (raising the count again) is inserted into the
/// child array at `index` (`00a5ebe0`), and both references are dropped (`DeleteThis`,
/// slot +4, at zero).
pub fn ni_node_insert_child_at(e: &mut Engine, this: Ptr, index: u32, child: u32) {
    if child == 0 {
        return;
    }
    e.call(INTERLOCKED_INCREMENT, &args![child.wrapping_add(4)]);
    e.call(NI_AV_OBJECT_ATTACH_PARENT, &args![child, this]);
    e.with_stack(4, |e, temp| {
        e.mem.set_u32(temp.addr(), child);
        e.call(INTERLOCKED_INCREMENT, &args![child.wrapping_add(4)]);
        let children = Ptr::new(this.addr() + CHILDREN);
        fn_00a5ebe0(e, children, index, temp);
    });
    drop_reference(e, child);
    drop_reference(e, child);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_d940, ni_node_apply_transform(Ptr, Ptr, Ptr, u32)),
        entry!(0x00a5_dc30, ni_node_purge_renderer_data(Ptr, u32)),
        entry!(
            0x00a5_e1f0,
            ni_node_update_rigid_downward_pass(Ptr, u32, u32)
        ),
        entry!(0x00a5_e280, ni_node_update_controllers(Ptr, u32)),
        entry!(0x00a5_e5b0, ni_node_register_streamables(Ptr, u32) -> bool),
        entry!(0x00a5_e600, ni_node_is_equal(Ptr, u32) -> bool),
        entry!(0x00a5_e880, ni_node_save_binary(Ptr, u32)),
        entry!(0x00a5_ea70, ni_node_get_viewer_strings(Ptr, u32)),
        entry!(0x00a5_ebe0, fn_00a5ebe0(Ptr, u32, Ptr) -> u32),
        entry!(0x00a5_edc0, ni_node_insert_child_at(Ptr, u32, u32)),
        entry!(0x00a5ece0, ni_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a5_ecb0, ni_node_ni_node(Ptr, u32) -> Ptr),
        entry!(0x00a5_ed10, ni_node_attach_child(Ptr, u32, u32)),
        entry!(
            0x00a5_eb20,
            fn_00a5eb20(Ptr<crate::types::NiTArray>, Ptr) -> u32
        ),
        entry!(0x00a5_db30, ni_node_update_upward_pass(Ptr)),
        entry!(0x00a5_e2d0, ni_node_update_world_bound(Ptr)),
        entry!(0x00a5_dd70, ni_node_update_downward_pass(Ptr, u32, u32)),
        entry!(
            0x00a5_ded0,
            ni_node_update_selected_downward_pass(Ptr, u32, u32)
        ),
        entry!(0x00a5_e0b0, ni_node_update_transform_and_bounds(Ptr, u32)),
        entry!(
            0x00a5_e440,
            ni_node_set_selective_update_flags(Ptr, u32, u32, u32)
        ),
        entry!(0x00a5_db60, ni_node_update_properties_downward(Ptr, u32)),
        entry!(0x00a5_e3a0, ni_node_update_properties_upward(Ptr, u32)),
        entry!(0x00a5_dd40, ni_node_load_binary(Ptr, u32)),
        entry!(0x00a5_e9d0, ni_node_link_object(Ptr, u32)),
        entry!(0x00a5_e560, ni_node_get_object_by_name(Ptr, u32) -> u32),
        entry!(0x00a5_dbe0, ni_node_on_visible(Ptr, u32)),
        entry!(0x00a5_d8f0, ni_node_detach_child(Ptr, u32)),
        entry!(0x00a5_d8a0, ni_node_detach_child_at(Ptr, u32)),
        entry!(0x00a9_3ea0, ni_node_set_at_ov2(Ptr, u32, u32)),
        entry!(0x00a5_e690, ni_node_detach_child_at_ov2(Ptr, u32, u32)),
        entry!(0x00a5_e780, ni_node_detach_child_ov2(Ptr, u32, u32)),
        entry!(0x00a5_ee30, ni_node_set_at(Ptr, u32, u32, u32)),
        entry!(0x00a5_dc80, fn_00a5dc80(Ptr, u32, u32)),
        entry!(0x00a5_dce0, ni_node_process_clone(Ptr, u32)),
        entry!(0x00a5_efa0, ni_node_create_clone(Ptr, u32) -> u32),
        entry!(0x00a5_e930, fn_00a5e930(Ptr)),
        entry!(0x00a5_e9c0, ni_node_get_rtti() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_node_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_e930, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a5ece0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a5ece0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xac]]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    #[test]
    fn ni_node_ni_node_builds_the_object_and_its_child_array() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CHILD_ARRAY_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xac);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_ecb0, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [0x00a5_ecb0, NI_AV_OBJECT_CONSTRUCT, CHILD_ARRAY_CONSTRUCT]
        );
        let args_of = |addr: u32| log.iter().find(|c| c.0 == addr).unwrap().1.clone();
        assert_eq!(args_of(NI_AV_OBJECT_CONSTRUCT), vec![this]);
        assert_eq!(args_of(CHILD_ARRAY_CONSTRUCT), vec![this + 0x9c, 3, 1]);
        assert_eq!(e.mem.u32(this), NI_NODE_VTABLE);
    }
}

#[cfg(test)]
mod b000o_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const CHILD_VTABLE: u32 = 0x0300_0000;
    const CHILD_DELETE: u32 = 0x0310_0004;

    /// A node with the given child-array sizes and a child whose count starts at `count`
    /// (kept at child +8 for the doubles' checks).
    fn attach_engine(size: u16, max_size: u16, count: i32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.i32(a[0]).wrapping_add(1);
            e.mem.set_i32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.i32(a[0]).wrapping_sub(1);
            e.mem.set_i32(a[0], v);
            v.into_ret()
        });
        e.register(NI_AV_OBJECT_ATTACH_PARENT, |e, a| {
            // The count is raised once before the parent is attached.
            assert_eq!(e.mem.i32(a[0] + 4), e.mem.i32(a[0] + 8) + 1);
            Ret::default()
        });
        e.register(CHILD_ARRAY_ADD_FIRST_EMPTY, |e, a| {
            let child = e.mem.u32(a[1]);
            assert_eq!(e.mem.i32(child + 4), e.mem.i32(child + 8) + 2);
            Ret::default()
        });
        e.register(CHILD_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(CHILD_ARRAY_SET_AT, |e, a| {
            let child = e.mem.u32(a[2]);
            assert_eq!(e.mem.i32(child + 4), e.mem.i32(child + 8) + 2);
            Ret::default()
        });
        e.register(CHILD_DELETE, |_, _| Ret::default());
        e.put_vtable(CHILD_VTABLE, &[0, CHILD_DELETE]);
        let node = e.mem.alloc(0xac);
        e.mem.set_u16(node + 0xa4, max_size);
        e.mem.set_u16(node + 0xa6, size);
        e.mem.set_u16(node + 0xaa, 4);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, CHILD_VTABLE);
        e.mem.set_i32(child + 4, count);
        e.mem.set_i32(child + 8, count);
        (e, node, child)
    }

    #[test]
    fn ni_node_attach_child_appends_and_grows_the_array() {
        // Full array: grows to size + grow-by, then stores at the old size. The count ends
        // where it started; the second drop reaching zero deletes the child.
        let (mut e, node, child) = attach_engine(3, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 0x100u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, NI_AV_OBJECT_ATTACH_PARENT),
            vec![vec![child, node]]
        );
        assert_eq!(
            calls_to(&log, CHILD_ARRAY_SET_SIZE),
            vec![vec![node + 0x9c, 7]]
        );
        let set_at = calls_to(&log, CHILD_ARRAY_SET_AT);
        assert_eq!(set_at.len(), 1);
        assert_eq!(set_at[0][..2], [node + 0x9c, 3]);
        assert!(calls_to(&log, CHILD_ARRAY_ADD_FIRST_EMPTY).is_empty());
        assert_eq!(e.mem.i32(child + 4), 0);
        assert_eq!(calls_to(&log, CHILD_DELETE), vec![vec![child]]);
        // Room left: no growth; a held child is not deleted.
        let (mut e, node, child) = attach_engine(2, 3, 5);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CHILD_ARRAY_SET_SIZE).is_empty());
        assert_eq!(calls_to(&log, CHILD_ARRAY_SET_AT)[0][1], 2);
        assert_eq!(e.mem.i32(child + 4), 5);
        assert!(calls_to(&log, CHILD_DELETE).is_empty());
        // A count that reaches zero on the first drop deletes there.
        let (mut e, node, child) = attach_engine(0, 3, -1);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CHILD_DELETE), vec![vec![child]]);
        assert_eq!(e.mem.i32(child + 4), -1);
    }

    #[test]
    fn ni_node_attach_child_first_available_and_null() {
        let (mut e, node, child) = attach_engine(3, 3, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, child, 1u32]);
        let log = e.call_log.take().unwrap();
        let add = calls_to(&log, CHILD_ARRAY_ADD_FIRST_EMPTY);
        assert_eq!(add.len(), 1);
        assert_eq!(add[0][0], node + 0x9c);
        assert!(calls_to(&log, CHILD_ARRAY_SET_AT).is_empty());
        assert!(calls_to(&log, CHILD_ARRAY_SET_SIZE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00a5_ed10, &args![node, 0u32, 1u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;
    use crate::types::NiTArray;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |_, _| 1u32.into_ret());
        e.register(POINTER_ARRAY_SET_CAPACITY, |_, _| Ret::default());
        e.register(POINTER_ARRAY_SET_AT, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00a5eb20_fills_the_first_empty_slot_or_appends() {
        let mut e = engine();
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(16);
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, 4);
        e.set(array, NiTArray::m_usSize, 3);
        e.set(array, NiTArray::m_usESize, 2);
        e.set(array, NiTArray::m_usGrowBy, 5);
        e.mem.set_u32(base, 0x111);
        e.mem.set_u32(base + 8, 0x333);
        let object = e.mem.alloc(8);
        let element = e.mem.alloc(4);
        // A null pointer is refused.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a5_eb20, &args![array, element]).u32(),
            0xffff_ffff
        );
        // The empty slot 1 takes it and counts it.
        e.mem.set_u32(element, object);
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 1);
        assert_eq!(e.mem.u32(base + 4), object);
        assert_eq!(e.get(array, NiTArray::m_usESize), 3);
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT), vec![vec![object + 4]]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        // Full but below capacity: appended at index 3 without growing.
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 3);
        assert!(calls_to(&e, POINTER_ARRAY_SET_CAPACITY).is_empty());
        assert_eq!(
            calls_to(&e, POINTER_ARRAY_SET_AT),
            vec![vec![array.addr(), 3, element]]
        );
        // At capacity: grows by m_usGrowBy first.
        e.set(array, NiTArray::m_usSize, 4);
        e.mem.set_u32(base + 12, 0x444);
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 4);
        assert_eq!(
            calls_to(&e, POINTER_ARRAY_SET_CAPACITY),
            vec![vec![array.addr(), 9]]
        );
        // An empty array appends at 0 (the scan does not run).
        e.set(array, NiTArray::m_usSize, 0);
        assert_eq!(e.call(0x00a5_eb20, &args![array, element]).u32(), 0);
    }
}

#[cfg(test)]
mod b008_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// A fake vtable address range: slot `s` of object kind `k` is
    /// `0x0130_0000 + k * 0x1000 + s`; every slot is registered as a recorder
    /// answering `answer(slot)`.
    const SLOTS: u32 = 0x0130_0000;

    fn vtable(e: &mut Engine, kind: u32) -> u32 {
        let table = e.mem.alloc(0x200);
        for slot in (0..0x200).step_by(4) {
            let target = SLOTS + kind * 0x1000 + slot;
            e.mem.set_u32(table + slot, target);
            e.register(target, |_, _| Ret::default());
        }
        table
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for f in [
            INTERLOCKED_INCREMENT,
            INTERLOCKED_DECREMENT,
            BEFORE_BOUND_COPY_005467E0,
            BOUND_MERGE,
            AV_UPDATE_OBJECT_CONTROLLERS,
            AV_CULL,
            AV_COPY_MEMBERS,
            AV_PROCESS_CLONE,
            CLONE_MAP_LOOKUP,
            CHILD_ARRAY_REMOVE_AT,
            CHILD_ARRAY_SET_AT,
            CHILD_ARRAY_SET_SIZE,
            NI_AV_OBJECT_ATTACH_PARENT,
            AV_LOAD_BINARY,
            STREAM_READ_MULTIPLE_LINK_IDS,
            AV_LINK_OBJECT,
            PROPERTY_STATE_DESTRUCT,
            MEMORY_DEALLOCATE,
            AV_UPDATE_PROPERTIES_DOWNWARD,
            AV_PUSH_LOCAL_PROPERTIES,
            CHILD_ARRAY_DELETE_BLOCK,
            NI_AV_OBJECT_DESTRUCT,
            NI_AV_OBJECT_CONSTRUCT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A node of vtable kind 0 with the children `children` (0 for a hole).
    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let table = vtable(e, 0);
        let n = e.mem.alloc(0xac);
        e.mem.set_u32(n, table);
        let block = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *c);
        }
        e.mem.set_u32(n + CHILDREN_BASE, block);
        e.mem.set_u16(n + CHILDREN_SIZE, children.len() as u16);
        e.mem.set_u16(n + CHILDREN_CAPACITY, children.len() as u16);
        n
    }

    /// A child of vtable kind `kind` with a bound of radius `radius` (no bound
    /// for `None`).
    fn child(e: &mut Engine, kind: u32, radius: Option<f32>) -> u32 {
        let table = vtable(e, kind);
        let c = e.mem.alloc(0xac);
        e.mem.set_u32(c, table);
        if let Some(r) = radius {
            let b = e.mem.alloc(0x10);
            e.mem.set_u32(b, 0x100 + kind);
            e.mem.set_u32(b + 4, 0x200 + kind);
            e.mem.set_u32(b + 8, 0x300 + kind);
            e.mem.set_f32(b + 0xc, r);
            e.mem.set_u32(c + AV_WORLD_BOUND, b);
        }
        c
    }

    fn slot(kind: u32, offset: u32) -> u32 {
        SLOTS + kind * 0x1000 + offset
    }

    fn log(e: &mut Engine) -> Log {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    fn with_bound(e: &mut Engine, n: u32) -> u32 {
        let b = e.mem.alloc(0x10);
        e.mem.set_f32(b + 0xc, 9.0);
        e.mem.set_u32(n + AV_WORLD_BOUND, b);
        b
    }

    #[test]
    fn ni_node_update_upward_pass_goes_to_the_parent() {
        let mut e = engine();
        let parent = node(&mut e, &[]);
        let n = node(&mut e, &[]);
        e.mem.set_u32(n + AV_PARENT, parent);
        e.call(0x00a5_db30, &args![n]);
        assert_eq!(
            log(&mut e),
            vec![(slot(0, 0xbc), vec![n]), (slot(0, 0xfc), vec![parent])]
        );
        e.call_log = Some(vec![]);
        e.mem.set_u32(n + AV_PARENT, 0);
        e.call(0x00a5_db30, &args![n]);
        assert_eq!(log(&mut e), vec![(slot(0, 0xbc), vec![n])]);
    }

    #[test]
    fn ni_node_update_world_bound_copies_the_first_and_merges_the_rest() {
        let mut e = engine();
        let a = child(&mut e, 1, Some(0.0));
        let b = child(&mut e, 2, Some(f32::NAN));
        let c = child(&mut e, 3, None);
        let d = child(&mut e, 4, Some(2.0));
        let n = node(&mut e, &[a, 0, b, c, d]);
        let own = with_bound(&mut e, n);
        e.mem.set_u32(n + AV_FLAGS, 1 << 19);
        e.call(0x00a5_e2d0, &args![n]);
        let db = e.mem.u32(d + AV_WORLD_BOUND);
        assert_eq!(
            log(&mut e),
            vec![
                (BEFORE_BOUND_COPY_005467E0, vec![n]),
                (BOUND_MERGE, vec![own, db])
            ]
        );
        assert_eq!(e.mem.u32(own), 0x102);
        assert_eq!(e.mem.u32(own + 8), 0x302);
        assert!(e.mem.f32(own + 0xc).is_nan());
        for flags in [0, (1 << 19) | (1 << 13)] {
            e.mem.set_u32(n + AV_FLAGS, flags);
            e.call_log = Some(vec![]);
            e.call(0x00a5_e2d0, &args![n]);
            assert!(log(&mut e).is_empty());
        }
    }

    #[test]
    fn ni_node_update_downward_pass_walks_and_bounds() {
        let mut e = engine();
        let a = child(&mut e, 1, Some(0.0));
        let b = child(&mut e, 2, Some(1.0));
        let c = child(&mut e, 3, Some(3.0));
        let n = node(&mut e, &[a, b, c]);
        let own = with_bound(&mut e, n);
        e.mem.set_u32(n + AV_FLAGS, (1 << 19) | 1);
        let data = e.mem.alloc(8);
        e.mem.set_u8(data + 4, 1);
        e.call(0x00a5_dd70, &args![n, data, 4u32]);
        let cb = e.mem.u32(c + AV_WORLD_BOUND);
        assert_eq!(
            log(&mut e),
            vec![
                (AV_UPDATE_OBJECT_CONTROLLERS, vec![n, data, 1]),
                (slot(0, 0xb8), vec![n, data]),
                (slot(1, 0xa4), vec![a, data, 5]),
                (slot(2, 0xa4), vec![b, data, 5]),
                (BEFORE_BOUND_COPY_005467E0, vec![n]),
                (slot(3, 0xa4), vec![c, data, 5]),
                (BOUND_MERGE, vec![own, cb]),
            ]
        );
        assert_eq!(e.mem.u32(own), 0x102);
        assert_eq!(e.mem.f32(own + 0xc), 1.0);
    }

    #[test]
    fn ni_node_update_downward_pass_without_bounds() {
        let mut e = engine();
        let a = child(&mut e, 1, Some(1.0));
        let n = node(&mut e, &[a]);
        let data = e.mem.alloc(8);
        e.call(0x00a5_dd70, &args![n, data, 0u32]);
        assert_eq!(
            log(&mut e),
            vec![
                (slot(0, 0xb8), vec![n, data]),
                (slot(1, 0xa4), vec![a, data, 0])
            ]
        );
        e.mem.set_u32(n + AV_FLAGS, (1 << 19) | (1 << 13));
        e.call_log = Some(vec![]);
        e.call(0x00a5_dd70, &args![n, data, 0u32]);
        assert_eq!(
            log(&mut e),
            vec![
                (slot(0, 0xb8), vec![n, data]),
                (BEFORE_BOUND_COPY_005467E0, vec![n]),
                (slot(1, 0xa4), vec![a, data, 0]),
            ]
        );
    }

    #[test]
    fn ni_node_update_selected_downward_pass_picks_the_child_slot() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        let b = child(&mut e, 2, None);
        let c = child(&mut e, 3, Some(1.0));
        e.mem.set_u32(a + AV_FLAGS, 2);
        e.mem.set_u32(b + AV_FLAGS, 2 | 0x10);
        let n = node(&mut e, &[a, b, c]);
        let own = with_bound(&mut e, n);
        e.mem.set_u32(n + AV_FLAGS, (1 << 19) | 8 | 4);
        let data = e.mem.alloc(8);
        e.mem.set_u8(data + 4, 7);
        e.register_double(AV_UPDATE_OBJECT_CONTROLLERS, |e, a| {
            assert_eq!(e.mem.u8(a[1] + 4), 1);
            Ret::default()
        });
        e.call(0x00a5_ded0, &args![n, data, 2u32]);
        assert_eq!(e.mem.u8(data + 4), 7);
        assert_eq!(
            log(&mut e),
            vec![
                (AV_UPDATE_OBJECT_CONTROLLERS, vec![n, data, 1]),
                (slot(0, 0xb8), vec![n, data]),
                (slot(1, 0xa8), vec![a, data, 2]),
                (slot(2, 0xac), vec![b, data, 2]),
                (BEFORE_BOUND_COPY_005467E0, vec![n]),
            ]
        );
        assert_eq!(e.mem.f32(own + 0xc), 1.0);
        // Not fresh (byte +5 set): no world data, no bound reset, no copy.
        e.mem.set_u8(data + 5, 1);
        e.mem.set_f32(own + 0xc, 9.0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ded0, &args![n, data, 2u32]);
        let calls: Vec<u32> = log(&mut e).into_iter().map(|c| c.0).collect();
        assert_eq!(
            calls,
            vec![AV_UPDATE_OBJECT_CONTROLLERS, slot(1, 0xa8), slot(2, 0xac)]
        );
        assert_eq!(e.mem.f32(own + 0xc), 9.0);
    }

    #[test]
    fn ni_node_update_transform_and_bounds_walks_slot_c0() {
        let mut e = engine();
        let a = child(&mut e, 1, Some(1.0));
        let b = child(&mut e, 2, Some(2.0));
        let n = node(&mut e, &[a, b]);
        let own = with_bound(&mut e, n);
        e.mem.set_u32(n + AV_FLAGS, (1 << 19) | 4);
        e.call(0x00a5_e0b0, &args![n, 0x77u32]);
        let bb = e.mem.u32(b + AV_WORLD_BOUND);
        assert_eq!(
            log(&mut e),
            vec![
                (slot(0, 0xb8), vec![n, 0x77]),
                (slot(1, 0xc0), vec![a, 0x77]),
                (BEFORE_BOUND_COPY_005467E0, vec![n]),
                (slot(2, 0xc0), vec![b, 0x77]),
                (BOUND_MERGE, vec![own, bb]),
            ]
        );
    }

    #[test]
    fn ni_node_set_selective_update_flags_collects_the_children() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        let b = child(&mut e, 2, None);
        e.register(slot(1, 0xa0), |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        e.register(slot(2, 0xa0), |e, a| {
            e.mem.set_u8(a[3], 0);
            Ret::default()
        });
        e.register(AV_HAS_TRANSFORM_CONTROLLER, |_, _| Ret::default());
        e.register(AV_HAS_PROPERTY_CONTROLLER, |_, _| Ret::default());
        let n = node(&mut e, &[a, b]);
        e.mem.set_u32(n + AV_FLAGS, 0x80 | 0x1e);
        let answers = e.mem.alloc(8);
        e.call(0x00a5_e440, &args![n, answers, 0x1200u32, answers + 4]);
        assert_eq!(e.mem.u8(answers), 1);
        assert_eq!(e.mem.u8(answers + 4), 0);
        // Bit 1 from the first answer, bit 2 from the selective byte (bit 7),
        // bits 3 and 4 cleared.
        assert_eq!(e.mem.u32(n + AV_FLAGS), 0x80 | 2 | 4);
        let calls: Vec<_> = log(&mut e)
            .into_iter()
            .filter(|c| c.0 == slot(1, 0xa0))
            .collect();
        assert_eq!(calls[0].1[2], 0x1201);
    }

    #[test]
    fn ni_node_set_selective_update_flags_with_controllers() {
        let mut e = engine();
        e.register(AV_HAS_TRANSFORM_CONTROLLER, |_, _| 1u32.into_ret());
        e.register(AV_HAS_PROPERTY_CONTROLLER, |_, _| 1u32.into_ret());
        let n = node(&mut e, &[]);
        let answers = e.mem.alloc(8);
        e.call(0x00a5_e440, &args![n, answers, 0u32, answers + 4]);
        assert_eq!(e.mem.u8(answers), 1);
        assert_eq!(e.mem.u8(answers + 4), 0);
        assert_eq!(e.mem.u32(n + AV_FLAGS), 2 | 4 | 8 | 0x10);
    }

    #[test]
    fn ni_node_update_properties_downward_frees_its_own_state() {
        let mut e = engine();
        e.register(AV_PUSH_LOCAL_PROPERTIES, |e, a| {
            e.mem.set_u32(a[3], 0x5550);
            Ret::default()
        });
        let a = child(&mut e, 1, None);
        let n = node(&mut e, &[a, 0]);
        e.call(0x00a5_db60, &args![n, 0x4440u32]);
        let calls = log(&mut e);
        assert_eq!(calls[0], (AV_UPDATE_PROPERTIES_DOWNWARD, vec![n, 0x4440]));
        assert_eq!(calls[1].0, AV_PUSH_LOCAL_PROPERTIES);
        assert_eq!(calls[1].1[1..3].to_vec(), vec![0x4440, 1]);
        assert_eq!(
            calls[2..].to_vec(),
            vec![
                (slot(1, 0xb0), vec![a, 0x5550]),
                (PROPERTY_STATE_DESTRUCT, vec![0x5550]),
                (MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x5550]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00a5_db60, &args![n, 0x5550u32]);
        assert!(log(&mut e).iter().all(|c| c.0 != PROPERTY_STATE_DESTRUCT));
    }

    #[test]
    fn ni_node_update_properties_upward_climbs_to_the_root() {
        let mut e = engine();
        let block = e.mem.alloc(0x1c);
        e.register_double(MEMORY_ALLOCATE, move |_, _| block.into_ret());
        e.register(PROPERTY_STATE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(AV_PUSH_LOCAL_PROPERTIES, |e, a| {
            e.mem.set_u32(a[3], a[0]);
            Ret::default()
        });
        let root = node(&mut e, &[]);
        let n = node(&mut e, &[]);
        e.mem.set_u32(n + AV_PARENT, root);
        e.mem.set_u32(n + AV_FLAGS, 0x40000);
        e.register_double(AV_HAS_PROPERTY_CONTROLLER, move |_, a| {
            u32::from(a[0] == root).into_ret()
        });
        let out = e.mem.alloc(4);
        e.call(0x00a5_e3a0, &args![n, out]);
        assert_eq!(e.mem.u32(out), n);
        let calls = log(&mut e);
        let pushes: Vec<Vec<u32>> = calls
            .iter()
            .filter(|c| c.0 == AV_PUSH_LOCAL_PROPERTIES)
            .map(|c| vec![c.1[0], c.1[1], c.1[2]])
            .collect();
        assert_eq!(pushes, vec![vec![root, block, 0], vec![n, root, 0]]);
        // The root's own state (the block) and the root's answer (the root,
        // which is not this answer) are freed.
        let frees: Vec<u32> = calls
            .iter()
            .filter(|c| c.0 == MEMORY_DEALLOCATE)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(frees, vec![block, root]);
        assert_eq!(e.mem.u32(root + AV_FLAGS), 0x40000);
        assert_eq!(e.mem.u32(n + AV_FLAGS), 0);
    }

    #[test]
    fn ni_node_load_binary_reads_two_link_lists() {
        let mut e = engine();
        let n = node(&mut e, &[]);
        e.mem.set_u32(n + AV_FLAGS, 0xffff_ffff);
        e.call(0x00a5_dd40, &args![n, 0x900u32]);
        assert_eq!(
            log(&mut e),
            vec![
                (AV_LOAD_BINARY, vec![n, 0x900]),
                (STREAM_READ_MULTIPLE_LINK_IDS, vec![0x900]),
                (STREAM_READ_MULTIPLE_LINK_IDS, vec![0x900]),
            ]
        );
        assert_eq!(e.mem.u32(n + AV_FLAGS), 0xffdf_ffff);
    }

    #[test]
    fn ni_node_link_object_stores_the_children_and_skips_the_rest() {
        let mut e = engine();
        let mut counts = vec![2u32, 1].into_iter();
        e.register_double(STREAM_GET_NUMBER_OF_LINK_IDS, move |_, _| {
            counts.next().unwrap().into_ret()
        });
        let mut objects = 0x700u32..;
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            objects.next().unwrap().into_ret()
        });
        let n = node(&mut e, &[]);
        let stream = e.mem.alloc(0x100);
        e.call(0x00a5_e9d0, &args![n, stream]);
        let calls = log(&mut e);
        assert!(calls.contains(&(CHILD_ARRAY_SET_SIZE, vec![n + CHILDREN, 2])));
        let stores: Vec<_> = calls
            .iter()
            .filter(|c| c.0 == slot(0, 0xf8))
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(stores, vec![vec![n, 0, 0x700], vec![n, 1, 0x701]]);
        assert_eq!(
            calls
                .iter()
                .filter(|c| c.0 == STREAM_GET_OBJECT_FROM_LINK_ID)
                .count(),
            3
        );
    }

    #[test]
    fn ni_node_get_object_by_name_searches_the_children() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        let b = child(&mut e, 2, None);
        e.register(slot(2, 0x9c), |_, _| 0x4242u32.into_ret());
        e.register(AV_GET_OBJECT_BY_NAME, |_, _| Ret::default());
        let n = node(&mut e, &[0, a, b]);
        assert_eq!(e.call(0x00a5_e560, &args![n, 0x10u32]).u32(), 0x4242);
        e.register(slot(2, 0x9c), |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_e560, &args![n, 0x10u32]).u32(), 0);
        e.register(AV_GET_OBJECT_BY_NAME, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x00a5_e560, &args![n, 0x10u32]).u32(), n);
    }

    #[test]
    fn ni_node_on_visible_culls_the_children_of_a_real_bound() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        let n = node(&mut e, &[a, 0]);
        e.call(0x00a5_dbe0, &args![n, 0x55u32]);
        assert!(log(&mut e).is_empty());
        let own = with_bound(&mut e, n);
        e.call_log = Some(vec![]);
        e.call(0x00a5_dbe0, &args![n, 0x55u32]);
        assert_eq!(log(&mut e), vec![(AV_CULL, vec![a, 0x55])]);
        e.mem.set_f32(own + 0xc, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_dbe0, &args![n, 0x55u32]);
        assert!(log(&mut e).is_empty());
    }

    #[test]
    fn ni_node_detach_wrappers_release_what_comes_back() {
        for (addr, offset, words) in [
            (0x00a5_d8f0u32, 0xe4u32, vec![0x31u32]),
            (0x00a5_d8a0, 0xec, vec![2]),
            (0x00a9_3ea0, 0xf4, vec![2, 0x31]),
        ] {
            let mut e = engine();
            let back = child(&mut e, 1, None);
            let n = node(&mut e, &[]);
            e.register_double(slot(0, offset), move |e, a| {
                let word = *a.last().unwrap();
                assert_eq!(e.mem.u32(word), 0);
                e.mem.set_u32(word, back);
                Ret::default()
            });
            e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
            let mut call = vec![n];
            call.extend(&words);
            e.call(addr, &call);
            let calls = log(&mut e);
            assert_eq!(calls[0].1[1..1 + words.len()].to_vec(), words);
            assert_eq!(
                calls[1..].to_vec(),
                vec![
                    (INTERLOCKED_DECREMENT, vec![back + 4]),
                    (slot(1, 4), vec![back])
                ]
            );
        }
    }

    #[test]
    fn ni_node_detach_child_at_ov2_hands_the_child_back() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        let n = node(&mut e, &[a]);
        e.mem.set_u32(a + AV_PARENT, n);
        e.register(CHILD_ARRAY_REMOVE_AT, |e, a| {
            assert_eq!(e.mem.u32(a[1]), a[2]);
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let out = e.mem.alloc(4);
        e.call(0x00a5_e690, &args![n, 0u32, out]);
        assert_eq!(e.mem.u32(out), a);
        assert_eq!(e.mem.u32(a + AV_PARENT), 0);
        let counts: Vec<_> = log(&mut e)
            .into_iter()
            .filter(|c| c.0 == INTERLOCKED_INCREMENT || c.0 == INTERLOCKED_DECREMENT)
            .collect();
        assert_eq!(
            counts,
            vec![
                (INTERLOCKED_INCREMENT, vec![a + 4]),
                (INTERLOCKED_INCREMENT, vec![a + 4]),
                (INTERLOCKED_DECREMENT, vec![a + 4]),
            ]
        );
        // Past the end: the word is cleared.
        let old = child(&mut e, 2, None);
        e.mem.set_u32(out, old);
        e.call_log = Some(vec![]);
        e.call(0x00a5_e690, &args![n, 1u32, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(log(&mut e), vec![(INTERLOCKED_DECREMENT, vec![old + 4])]);
    }

    #[test]
    fn ni_node_detach_child_ov2_finds_the_child() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        let b = child(&mut e, 2, None);
        let n = node(&mut e, &[a, 0, b]);
        let out = e.mem.alloc(4);
        e.call(0x00a5_e780, &args![n, b, out]);
        assert_eq!(e.mem.u32(out), b);
        let calls = log(&mut e);
        assert!(calls
            .iter()
            .any(|c| c.0 == CHILD_ARRAY_REMOVE_AT && c.1[0] == n + CHILDREN && c.1[2] == 2));
        let a_counts = calls.iter().filter(|c| c.1 == vec![a + 4]).count();
        assert_eq!(a_counts, 2);
        // Not there: the word is cleared.
        let other = child(&mut e, 3, None);
        e.call_log = Some(vec![]);
        e.call(0x00a5_e780, &args![n, other, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert!(log(&mut e).iter().all(|c| c.0 != CHILD_ARRAY_REMOVE_AT));
    }

    #[test]
    fn ni_node_set_at_replaces_or_grows() {
        let mut e = engine();
        let old = child(&mut e, 1, None);
        let new = child(&mut e, 2, None);
        let n = node(&mut e, &[old]);
        let out = e.mem.alloc(4);
        e.call(0x00a5_ee30, &args![n, 0u32, new, out]);
        assert_eq!(e.mem.u32(out), old);
        assert_eq!(e.mem.u32(old + AV_PARENT), 0);
        let calls = log(&mut e);
        assert!(calls.contains(&(NI_AV_OBJECT_ATTACH_PARENT, vec![new, n])));
        let set: Vec<_> = calls.iter().filter(|c| c.0 == CHILD_ARRAY_SET_AT).collect();
        assert_eq!(set[0].1[..2].to_vec(), vec![n + CHILDREN, 0]);
        assert!(calls.iter().all(|c| c.0 != CHILD_ARRAY_SET_SIZE));
        // Past the capacity: the array grows by the grow-by.
        e.mem.set_u16(n + 0xaa, 3);
        e.mem.set_u32(out, old);
        e.call_log = Some(vec![]);
        e.call(0x00a5_ee30, &args![n, 1u32, new, out]);
        let calls = log(&mut e);
        assert!(calls.contains(&(CHILD_ARRAY_SET_SIZE, vec![n + CHILDREN, 4])));
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn fn_00a5dc80_clones_each_child_into_the_copy() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        e.register(slot(1, 0x48), |_, _| 0x6060u32.into_ret());
        let n = node(&mut e, &[0, a]);
        let clone = node(&mut e, &[]);
        e.call(0x00a5_dc80, &args![n, clone, 0x33u32]);
        assert_eq!(
            log(&mut e),
            vec![
                (AV_COPY_MEMBERS, vec![n, clone, 0x33]),
                (slot(1, 0x48), vec![a, 0x33]),
                (slot(0, 0xf8), vec![clone, 1, 0x6060]),
            ]
        );
    }

    #[test]
    fn ni_node_process_clone_looks_up_and_walks() {
        let mut e = engine();
        let a = child(&mut e, 1, None);
        let n = node(&mut e, &[a]);
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x8800);
        e.call(0x00a5_dce0, &args![n, cloning]);
        let calls = log(&mut e);
        assert_eq!(calls[0], (AV_PROCESS_CLONE, vec![n, cloning]));
        assert_eq!(calls[1].1[..2].to_vec(), vec![0x8800, n]);
        assert_eq!(calls[2], (slot(1, 0x68), vec![a, cloning]));
    }

    #[test]
    fn ni_node_create_clone_builds_an_empty_node() {
        let mut e = engine();
        let block = e.mem.alloc(0xac);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        let n = node(&mut e, &[]);
        assert_eq!(e.call(0x00a5_efa0, &args![n, 0x33u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_NODE_VTABLE);
        assert_eq!(e.mem.u16(block + 0xaa), 1);
        assert_eq!(e.mem.u32(block + CHILDREN), CHILD_ARRAY_VTABLE_0101FBD4);
        assert!(log(&mut e).contains(&(AV_COPY_MEMBERS, vec![n, block, 0x33])));
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_efa0, &args![n, 0x33u32]).u32(), 0);
        assert!(log(&mut e).contains(&(AV_COPY_MEMBERS, vec![n, 0, 0x33])));
    }

    #[test]
    fn fn_00a5e930_detaches_every_child_and_frees_the_block() {
        let mut e = engine();
        let n = node(&mut e, &[1, 2]);
        let block = e.mem.u32(n + CHILDREN_BASE);
        // The destructor stores NiNode's own vtable: give it slot 0xec.
        e.map(NI_NODE_VTABLE, 0x100);
        e.mem.set_u32(NI_NODE_VTABLE + 0xec, slot(0, 0xec));
        e.call(0x00a5_e930, &args![n]);
        let calls = log(&mut e);
        let detaches: Vec<u32> = calls
            .iter()
            .filter(|c| c.0 == slot(0, 0xec))
            .map(|c| c.1[1])
            .collect();
        assert_eq!(detaches, vec![0, 1]);
        assert!(calls.contains(&(CHILD_ARRAY_DELETE_BLOCK, vec![block])));
        assert_eq!(calls.last().unwrap(), &(NI_AV_OBJECT_DESTRUCT, vec![n]));
        assert_eq!(e.mem.u32(n + CHILDREN), CHILD_ARRAY_VTABLE_0101FBCC);
    }

    #[test]
    fn ni_node_get_rtti_answers_the_record() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a5_e9c0, &args![]).u32(), NI_NODE_RTTI);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Every slot of the test vtable points at `TARGETS + slot`.
    const VTABLE: u32 = 0x0320_0000;
    const TARGETS: u32 = 0x0330_0000;

    /// Reference counting that changes the count word, logged, and a logged `DeleteThis`.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x50).map(|i| TARGETS + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        let log = Log::default();
        for (addr, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let l = log.clone();
            e.register_double(addr, move |e, a| {
                l.borrow_mut().push((addr, a.to_vec()));
                let value = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], value);
                value.into_ret()
            });
        }
        for addr in [
            NI_AV_OBJECT_ATTACH_PARENT,
            CHILD_ARRAY_SET_SIZE,
            TARGETS + 4,
        ] {
            logged(&mut e, &log, addr, 0);
        }
        (e, log)
    }

    /// Logs calls to `addr` and answers `eax`.
    fn logged(e: &mut Engine, log: &Log, addr: u32, eax: u32) {
        let l = log.clone();
        e.register_double(addr, move |_, a| {
            l.borrow_mut().push((addr, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn object(e: &mut Engine, flags: u32) -> u32 {
        let p = e.mem.alloc(0xac);
        e.mem.set_u32(p, VTABLE);
        e.mem.set_u32(p + AV_FLAGS, flags);
        p
    }

    /// A node with the given children (0 for an empty slot) and spare capacity `extra`.
    fn node(e: &mut Engine, children: &[u32], extra: u16) -> u32 {
        let n = object(e, 0);
        let capacity = children.len() as u16 + extra;
        let block = e.mem.alloc(4 * u32::from(capacity) + 8);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *c);
        }
        e.mem.set_u32(n + CHILDREN_BASE, block);
        e.mem.set_u16(n + CHILDREN_CAPACITY, capacity);
        e.mem.set_u16(n + CHILDREN_SIZE, children.len() as u16);
        e.mem.set_u16(n + 0xaa, 2);
        n
    }

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    #[test]
    fn apply_transform_passes_on_or_moves_into_the_node_frame() {
        let (mut e, log) = engine();
        let (a, b) = (object(&mut e, 0), object(&mut e, 0));
        let n = node(&mut e, &[a, 0, b], 0);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let s = seen.clone();
        e.register_double(TARGETS + SLOT_APPLY_TRANSFORM, move |e, a| {
            let t: Vec<f32> = (0..3).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
            s.borrow_mut().push((a.to_vec(), t));
            Ret::default()
        });
        let t = e.mem.alloc(0xc);
        put(&mut e, t, &[4.0, 6.0, 8.0]);
        e.call(0x00a5_d940, &args![n, 0x1111u32, t, 0x100u32]);
        let got: Vec<Vec<u32>> = seen.borrow().iter().map(|c| c.0.clone()).collect();
        assert_eq!(got, vec![vec![a, 0x1111, t, 1], vec![b, 0x1111, t, 1]]);
        seen.borrow_mut().clear();

        // A quarter turn about z as the local rotation, T = (1, 2, 3), scale 2, M = 2 I,
        // t = (4, 6, 8): M T + t - T = (5, 8, 11), R^T of it (-8, 5, 11), halved.
        put(
            &mut e,
            n + AV_LOCAL_ROTATE,
            &[0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        );
        put(&mut e, n + AV_LOCAL_TRANSLATE, &[1.0, 2.0, 3.0]);
        put(&mut e, n + AV_LOCAL_SCALE, &[2.0]);
        let m = e.mem.alloc(0x24);
        put(&mut e, m, &[2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0]);
        logged(&mut e, &log, MATRIX_MULTIPLY, 0x5555);
        logged(&mut e, &log, MATRIX_TRANSPOSE_TIMES, 0);
        e.call(0x00a5_d940, &args![n, m, t, 1u32]);
        let product = calls(&log, MATRIX_MULTIPLY);
        assert_eq!(product.len(), 1);
        assert_eq!((product[0][0], product[0][2]), (m, n + AV_LOCAL_ROTATE));
        let transposed = calls(&log, MATRIX_TRANSPOSE_TIMES);
        assert_eq!(transposed[0][0], n + AV_LOCAL_ROTATE);
        assert_eq!(transposed[0][2], 0x5555);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        for (call, child) in seen.iter().zip([a, b]) {
            assert_eq!(call.0[0], child);
            assert_eq!(call.0[1], transposed[0][1]);
            assert_eq!(call.0[3], 1);
            assert_eq!(call.1, vec![-4.0, 2.5, 5.5]);
        }
    }

    #[test]
    fn purge_renderer_data_runs_the_base_then_the_children() {
        let (mut e, log) = engine();
        let a = object(&mut e, 0);
        let n = node(&mut e, &[0, a], 0);
        logged(&mut e, &log, AV_PURGE_RENDERER_DATA, 0);
        logged(&mut e, &log, TARGETS + SLOT_PURGE_RENDERER_DATA, 0);
        e.call(0x00a5_dc30, &args![n, 0x77u32]);
        assert_eq!(calls(&log, AV_PURGE_RENDERER_DATA), vec![vec![n, 0x77]]);
        assert_eq!(
            calls(&log, TARGETS + SLOT_PURGE_RENDERER_DATA),
            vec![vec![a, 0x77]]
        );
    }

    #[test]
    fn rigid_downward_pass_sets_the_data_byte_and_picks_the_children() {
        let (mut e, log) = engine();
        let (a, b) = (object(&mut e, 2), object(&mut e, 0));
        let n = node(&mut e, &[a, b, 0], 0);
        let data = e.mem.alloc(8);
        e.mem.set_u8(data + 4, 0x55);
        let l = log.clone();
        e.register_double(AV_UPDATE_OBJECT_CONTROLLERS, move |e, a| {
            let byte = u32::from(e.mem.u8(a[1] + 4));
            l.borrow_mut()
                .push((AV_UPDATE_OBJECT_CONTROLLERS, vec![a[0], a[1], a[2], byte]));
            Ret::default()
        });
        logged(&mut e, &log, TARGETS + SLOT_RIGID_PASS_OWN, 0);
        logged(&mut e, &log, TARGETS + SLOT_UPDATE_RIGID_DOWNWARD_PASS, 0);
        // Flag bits 3, 2 and 0.
        e.mem.set_u32(n + AV_FLAGS, 0b1101);
        e.call(0x00a5_e1f0, &args![n, data, 0x10u32]);
        assert_eq!(
            calls(&log, AV_UPDATE_OBJECT_CONTROLLERS),
            vec![vec![n, data, 1, 1]]
        );
        assert_eq!(e.mem.u8(data + 4), 0x55);
        assert_eq!(
            calls(&log, TARGETS + SLOT_RIGID_PASS_OWN),
            vec![vec![n, data]]
        );
        assert_eq!(
            calls(&log, TARGETS + SLOT_UPDATE_RIGID_DOWNWARD_PASS),
            vec![vec![a, data, 0x11]]
        );
        // No flag bits: byte 0 during the call, no own slot, the flags as given.
        log.borrow_mut().clear();
        e.mem.set_u32(n + AV_FLAGS, 0);
        e.call(0x00a5_e1f0, &args![n, data, 0x10u32]);
        assert_eq!(
            calls(&log, AV_UPDATE_OBJECT_CONTROLLERS),
            vec![vec![n, data, 1, 0]]
        );
        assert!(calls(&log, TARGETS + SLOT_RIGID_PASS_OWN).is_empty());
        assert_eq!(
            calls(&log, TARGETS + SLOT_UPDATE_RIGID_DOWNWARD_PASS),
            vec![vec![a, data, 0x10]]
        );
    }

    #[test]
    fn update_controllers_runs_the_base_then_the_children() {
        let (mut e, log) = engine();
        let a = object(&mut e, 0);
        let n = node(&mut e, &[a, 0], 0);
        logged(&mut e, &log, AV_UPDATE_OBJECT_CONTROLLERS, 0);
        logged(&mut e, &log, TARGETS + SLOT_UPDATE_CONTROLLERS, 0);
        e.call(0x00a5_e280, &args![n, 0x99u32]);
        assert_eq!(
            calls(&log, AV_UPDATE_OBJECT_CONTROLLERS),
            vec![vec![n, 0x99, 1]]
        );
        assert_eq!(
            calls(&log, TARGETS + SLOT_UPDATE_CONTROLLERS),
            vec![vec![a, 0x99]]
        );
    }

    #[test]
    fn register_streamables_stops_when_the_base_does() {
        let (mut e, log) = engine();
        let a = object(&mut e, 0);
        let n = node(&mut e, &[a], 0);
        logged(&mut e, &log, AV_REGISTER_STREAMABLES, 0);
        logged(&mut e, &log, TARGETS + SLOT_REGISTER_STREAMABLES, 0);
        assert!(!e.call(0x00a5_e5b0, &args![n, 0x44u32]).bool());
        assert!(calls(&log, TARGETS + SLOT_REGISTER_STREAMABLES).is_empty());
        logged(&mut e, &log, AV_REGISTER_STREAMABLES, 1);
        assert!(e.call(0x00a5_e5b0, &args![n, 0x44u32]).bool());
        assert_eq!(
            calls(&log, TARGETS + SLOT_REGISTER_STREAMABLES),
            vec![vec![a, 0x44]]
        );
    }

    #[test]
    fn is_equal_compares_the_base_the_count_and_each_child() {
        let (mut e, log) = engine();
        let (a, b) = (object(&mut e, 0), object(&mut e, 0));
        let mine = node(&mut e, &[a, 0], 0);
        let same = node(&mut e, &[b, 0], 0);
        let shorter = node(&mut e, &[b], 0);
        let swapped = node(&mut e, &[0, b], 0);
        logged(&mut e, &log, AV_IS_EQUAL, 1);
        logged(&mut e, &log, TARGETS + SLOT_IS_EQUAL, 1);
        assert!(e.call(0x00a5_e600, &args![mine, same]).bool());
        assert_eq!(calls(&log, TARGETS + SLOT_IS_EQUAL), vec![vec![a, b]]);
        assert!(!e.call(0x00a5_e600, &args![mine, shorter]).bool());
        assert!(!e.call(0x00a5_e600, &args![mine, swapped]).bool());
        assert!(!e.call(0x00a5_e600, &args![swapped, mine]).bool());
        logged(&mut e, &log, TARGETS + SLOT_IS_EQUAL, 0);
        assert!(!e.call(0x00a5_e600, &args![mine, same]).bool());
        logged(&mut e, &log, AV_IS_EQUAL, 0);
        logged(&mut e, &log, TARGETS + SLOT_IS_EQUAL, 1);
        assert!(!e.call(0x00a5_e600, &args![mine, same]).bool());
    }

    #[test]
    fn save_binary_writes_the_count_the_links_and_a_zero() {
        let (mut e, log) = engine();
        let a = object(&mut e, 0);
        let n = node(&mut e, &[a, 0], 0);
        // The stream reaches past `m_pkOstr` (+0x250), beyond the 0xac bytes `object` gives:
        // a block of its own, so the field stays in it.
        let stream = e.mem.alloc(STREAM_OUTPUT + 4);
        e.mem.set_u32(stream, VTABLE);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, 0x0340_0000);
        e.mem.set_u32(output + BINARY_STREAM_POSITION, 0x20);
        let l = log.clone();
        e.register_double(0x0340_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            l.borrow_mut()
                .push((0x0340_0000, vec![a[0], e.mem.u32(a[1])]));
            4u32.into_ret()
        });
        logged(&mut e, &log, AV_SAVE_BINARY, 0);
        logged(&mut e, &log, TARGETS + SLOT_SAVE_LINK_ID, 0);
        e.call(0x00a5_e880, &args![n, stream]);
        let order: Vec<(u32, Vec<u32>)> = log.borrow().clone();
        assert_eq!(
            order,
            vec![
                (AV_SAVE_BINARY, vec![n, stream]),
                (0x0340_0000, vec![output, 2]),
                (TARGETS + SLOT_SAVE_LINK_ID, vec![stream, a]),
                (TARGETS + SLOT_SAVE_LINK_ID, vec![stream, 0]),
                (0x0340_0000, vec![output, 0]),
            ]
        );
        assert_eq!(e.mem.u32(output + BINARY_STREAM_POSITION), 0x28);
    }

    #[test]
    fn viewer_strings_add_the_name_and_whether_the_bound_is_visual() {
        let (mut e, log) = engine();
        e.map(NI_NODE_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(NI_NODE_RTTI, 0x0abc_0000);
        let n = node(&mut e, &[], 0);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 10);
        logged(&mut e, &log, AV_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, VIEWER_STRING_NAME, 0x61);
        logged(&mut e, &log, VIEWER_STRING_BOOL, 0x62);
        let l = log.clone();
        e.register_double(STRING_ARRAY_SET_AT, move |e, a| {
            l.borrow_mut()
                .push((STRING_ARRAY_SET_AT, vec![a[1], e.mem.u32(a[2])]));
            Ret::default()
        });
        let bound = e.mem.alloc(0x10);
        for (world_bound, radius) in [(0, 0.0f32), (bound, 0.0), (bound, 3.0), (bound, f32::NAN)] {
            e.mem.set_u32(n + AV_WORLD_BOUND, world_bound);
            e.mem.set_f32(bound + 0xc, radius);
            e.call(0x00a5_ea70, &args![n, strings]);
        }
        let flags: Vec<u32> = calls(&log, VIEWER_STRING_BOOL)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(flags, vec![0, 0, 1, 1]);
        assert_eq!(calls(&log, VIEWER_STRING_BOOL)[0][0], TEXT_VISUAL);
        assert_eq!(calls(&log, VIEWER_STRING_NAME)[0], vec![0x0abc_0000]);
        assert_eq!(calls(&log, AV_GET_VIEWER_STRINGS)[0], vec![n, strings]);
        assert_eq!(
            calls(&log, STRING_ARRAY_SET_AT)[..2],
            [vec![0, 0x61], vec![0, 0x62]]
        );
    }

    #[test]
    fn insert_at_shifts_the_tail_up_and_grows_when_full() {
        let (mut e, log) = engine();
        let (a, b, c) = (object(&mut e, 0), object(&mut e, 0), object(&mut e, 0));
        for o in [a, b, c] {
            e.mem.set_u32(o + 4, 1);
        }
        let n = node(&mut e, &[a, 0, b], 0);
        let array = n + CHILDREN;
        let l = log.clone();
        e.register_double(CHILD_ARRAY_SET_AT, move |e, w| {
            l.borrow_mut()
                .push((CHILD_ARRAY_SET_AT, vec![w[1], e.mem.u32(w[2])]));
            let base = e.mem.u32(w[0] + 4);
            let value = e.mem.u32(w[2]);
            e.mem.set_u32(base + 4 * w[1], value);
            Ret::default()
        });
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, c);
        assert_eq!(e.call(0x00a5_ebe0, &args![array, 1u32, element]).u32(), 1);
        // Positions 3 and 2 take b and the empty slot, then c goes in at 1; the array grows
        // once, at the first move (its size 3 has reached the capacity 3).
        assert_eq!(
            calls(&log, CHILD_ARRAY_SET_AT),
            vec![vec![3, b], vec![2, 0], vec![1, c]]
        );
        assert_eq!(calls(&log, CHILD_ARRAY_SET_SIZE), vec![vec![array, 5]]);
        // The moved object's count went up and back down, without a delete.
        assert_eq!(e.mem.u32(b + 4), 1);
        assert!(calls(&log, TARGETS + 4).is_empty());
        // Inserting at the capacity grows the array by the grow-by count first.
        log.borrow_mut().clear();
        let n = node(&mut e, &[a], 0);
        e.mem.set_u16(n + CHILDREN_SIZE, 0);
        e.mem.set_u32(element, c);
        assert_eq!(
            e.call(0x00a5_ebe0, &args![n + CHILDREN, 1u32, element])
                .u32(),
            1
        );
        assert_eq!(
            calls(&log, CHILD_ARRAY_SET_SIZE),
            vec![vec![n + CHILDREN, 3]]
        );
        // A null pointer is refused.
        e.mem.set_u32(element, 0);
        assert_eq!(
            e.call(0x00a5_ebe0, &args![array, 0u32, element]).u32(),
            0xffff_ffff
        );
    }

    #[test]
    fn insert_child_at_attaches_and_inserts_with_balanced_references() {
        let (mut e, log) = engine();
        let child = object(&mut e, 0);
        let n = node(&mut e, &[], 4);
        let l = log.clone();
        e.register_double(CHILD_ARRAY_SET_AT, move |e, a| {
            l.borrow_mut()
                .push((CHILD_ARRAY_SET_AT, vec![a[0], a[1], e.mem.u32(a[2])]));
            Ret::default()
        });
        e.call(0x00a5_edc0, &args![n, 0u32, 0u32]);
        assert!(log.borrow().is_empty());
        e.call(0x00a5_edc0, &args![n, 2u32, child]);
        assert_eq!(
            calls(&log, NI_AV_OBJECT_ATTACH_PARENT),
            vec![vec![child, n]]
        );
        assert_eq!(
            calls(&log, CHILD_ARRAY_SET_AT),
            vec![vec![n + CHILDREN, 2, child]]
        );
        assert_eq!(calls(&log, INTERLOCKED_INCREMENT).len(), 2);
        assert_eq!(calls(&log, INTERLOCKED_DECREMENT).len(), 2);
        // Nobody else held the child: the second drop reaches zero and deletes it.
        assert_eq!(e.mem.u32(child + 4), 0);
        assert_eq!(calls(&log, TARGETS + 4), vec![vec![child]]);
    }
}
