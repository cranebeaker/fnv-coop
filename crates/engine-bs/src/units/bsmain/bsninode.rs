//! `BSMain/bsninode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSNiNode` is an `NiNode` (0xAC bytes on PC) with no members of its own.
//! Besides the usual constructor, clone and destructor, the unit has the
//! constructor `00c5f680` that builds a `BSNiNode` from an existing node:
//! it takes over the node's name, controllers, collision object, children,
//! properties, local transform and extra data and replaces the node in its
//! parent; or, for a node with controllers or an `NiBillboardNode` when
//! asked to check, it keeps the node as its only child.
//!
//! C++ exception-unwinding frames (`00c5f680`, `CreateClone`) are not
//! translated; those functions carry the `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiAVObject, NiTArray, NiTListItem, NiTPointerList};
use crate::units::nimain::niobjectnet::NiObjectNET;

/// Address of the `BSNiNode` vtable.
pub(crate) const BS_NI_NODE_VTABLE: u32 = 0x010c_31a4;
/// Address of `BSNiNode`'s run-time type information.
pub(crate) const BS_NI_NODE_RTTI: u32 = 0x0120_4380;
/// Size of a `BSNiNode` on PC (the Xbox build is 0xD0).
pub(crate) const BS_NI_NODE_SIZE: u32 = 0xac;

/// `NiControllerManager`'s run-time type information (the block
/// `NiControllerManager::GetRTTI`, `00a2ec40`, returns).
const NI_CONTROLLER_MANAGER_RTTI: u32 = 0x011f_36ac;
/// `NiBillboardNode`'s run-time type information (the block
/// `NiBillboardNode::GetRTTI`, `0051cf90`, returns).
const NI_BILLBOARD_NODE_RTTI: u32 = 0x011f_4aa0;
/// `"Anim"`, the suffix `00c5f680` passes to `00c5fb80`.
const ANIM_SUFFIX: u32 = 0x010c_32a8;
/// The critical section `00c5f680` holds around the extra-data move.
const EXTRA_DATA_LOCK: u32 = 0x011f_4380;
/// The memory manager instance (`this` of [`MEMORY_ALLOCATE`]).
const MEMORY_MANAGER: u32 = 0x011f_6238;

/// The allocator `00aa13e0` (cdecl, size): returns the block.
const ALLOCATE: u32 = 0x00aa_13e0;
/// The sized free `00aa1460` (cdecl, block, size).
const FREE_SIZED: u32 = 0x00aa_1460;
/// The memory manager's allocation (`00aa3e40`, thiscall on
/// [`MEMORY_MANAGER`], size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The `NiNode` constructor (`00a5ecb0`, thiscall, one word: the initial
/// child count).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The `NiNode` destructor body (`00a5e930`, thiscall).
const NI_NODE_DESTRUCT: u32 = 0x00a5_e930;
/// The `NiNode` `CopyMembers(this, target, cloneProcess)` (`00a5dc80`).
const NI_NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;
/// `NiNode::AttachChild(this, child, firstAvailable)` (Xbox PDB).
const NI_NODE_ATTACH_CHILD: u32 = 0x00a5_ed10;
/// `NiNode::UpdateWorldBound(this)` (Xbox PDB).
const NI_NODE_UPDATE_WORLD_BOUND: u32 = 0x00a5_e2d0;
/// Copies the `NiFixedString` at the address passed into `this`'s name
/// (`00a5b950`).
const SET_NAME: u32 = 0x00a5_b950;
/// The kind-of test `006532c0` (`this` = the object, then the run-time type
/// information): walks the object's type chain; byte result.
const IS_KIND_OF: u32 = 0x0065_32c0;
/// `NiTimeController::SetTarget(this, target)` (Xbox PDB).
const NI_TIME_CONTROLLER_SET_TARGET: u32 = 0x00a6_d450;
/// `NiAVObject::SetCollisionObject(this, object)` (Xbox PDB).
const NI_AV_OBJECT_SET_COLLISION_OBJECT: u32 = 0x0062_bc90;
/// `NiAVObject::AttachProperty(this, property)` (Xbox PDB).
const NI_AV_OBJECT_ATTACH_PROPERTY: u32 = 0x0043_9410;
/// The method `00654640` (`this` = the source node, the property) that
/// `00c5f680` calls after attaching each property to the new node; the loop
/// relies on it emptying the source's property list.
const REMOVE_PROPERTY: u32 = 0x0065_4640;
/// `NiObjectNET::SetExtraDataSize(this, size)` (Xbox PDB).
const NI_OBJECT_NET_SET_EXTRA_DATA_SIZE: u32 = 0x00a5_bba0;
/// `NiObjectNET::AddExtraData_ov2(this, extraData)` (Xbox PDB).
const NI_OBJECT_NET_ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// Removes the extra data at an index (`00a5bf50`, `this`, index).
const REMOVE_EXTRA_DATA_AT: u32 = 0x00a5_bf50;
/// `fabs` (`00ec6cde`), double in, double out.
const FABS: u32 = 0x00ec_6cde;
/// `_strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `_strcpy_s(dst, size, src)` (`00ec65a6`, cdecl).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_strcat_s(dst, size, src)` (`00ec6bd2`, cdecl).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// Imported `EnterCriticalSection` / `LeaveCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// Imported `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Vtable slots used here (byte offsets).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_GET_RTTI: u32 = 0x08;
/// `IsNode` (Xbox PDB slot name): the object as a node, or null.
const SLOT_IS_NODE: u32 = 0x0c;
/// `IsGeometry` (Xbox PDB slot name): the object as a geometry, or null.
const SLOT_IS_GEOMETRY: u32 = 0x18;
/// The controller slot `00c5f680` calls with the new node when the
/// controller is not an `NiControllerManager` (`NiTimeController` slot
/// 0x98 is `SetTarget` in the Xbox PDB).
const SLOT_CONTROLLER_SET_TARGET: u32 = 0x98;
/// The slot `CreateClone` calls on the clone (no arguments).
const SLOT_AFTER_CLONE: u32 = 0xbc;
/// The parent's slot taking the child and the address of an
/// `NiPointer` out-parameter (`NiNode` slot 0xE4 is `DetachChild` in the
/// Xbox PDB).
const SLOT_DETACH_CHILD: u32 = 0xe4;
/// The parent's slot taking a child and a flag (`NiNode` slot 0xDC is
/// `AttachChild` in the Xbox PDB).
const SLOT_ATTACH_CHILD: u32 = 0xdc;
/// The slot `ReparentSkinInstances` recurses through (`this`'s own).
const SLOT_REPARENT_SKIN_INSTANCES: u32 = 0x100;

/// Offset of an `NiNode`'s `m_kChildren` (`NiTObjectArray`, Xbox PDB
/// +0xC0).
const CHILDREN: u32 = 0x9c;
/// `NiAVObject::m_kLocal` (Xbox PDB +0x40): rotation (nine floats),
/// translation (three) and scale on PC.
const LOCAL_ROTATE: u32 = 0x34;
const LOCAL_TRANSLATE: u32 = 0x58;
const LOCAL_SCALE: u32 = 0x64;
/// `NiTimeController::m_spNext` offset (the next controller).
const CONTROLLER_NEXT: u32 = 0x30;
/// `NiGeometry::m_spSkinInstance` (Xbox PDB +0xE0).
const GEOMETRY_SKIN_INSTANCE: u32 = 0xbc;
/// `NiSkinInstance::m_pkRootParent` (Xbox PDB).
const SKIN_INSTANCE_ROOT_PARENT: u32 = 0x10;

layout! {
    /// `BSNiNode` (Xbox PDB), 0xAC bytes on PC: an `NiNode` with no members
    /// of its own.
    pub struct BSNiNode: 0xac {}
}

type Node = Ptr<BSNiNode>;

/// The child array of a node.
fn children(node: u32) -> Ptr<NiTArray> {
    Ptr::new(node + CHILDREN)
}

/// Child `index` of a node's child array.
fn child_at(e: &Engine, node: u32, index: u32) -> u32 {
    let base = e.get(children(node), NiTArray::m_pBase);
    e.mem.u32(base + 4 * index)
}

/// The number of child slots of a node (`m_usSize`).
fn child_count(e: &Engine, node: u32) -> u32 {
    e.get(children(node), NiTArray::m_usSize) as u32
}

/// Releases an `NiPointer` the code holds: decrements the reference count
/// and calls `DeleteThis` (slot 4) when it reaches zero.
fn release(e: &mut Engine, object: u32) {
    if object == 0 {
        return;
    }
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

// Translated from 00c5f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiNode::BSNiNode` (Xbox PDB): builds the `NiNode` (initial child count
/// 0) and sets the vtable. Returns `this`.
pub fn bs_ni_node_bs_ni_node(e: &mut Engine, this: Node) -> Node {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BS_NI_NODE_VTABLE);
    this
}

// Translated from 00c5f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiNode::ReparentSkinInstances` (Xbox PDB): with both `node` and
/// `root` set, goes through `node`'s child slots: a child that is a node
/// (`IsNode`, slot 0xC) is passed on to this object's slot 0x100 with
/// `root` (the recursion); otherwise, for a geometry (`IsGeometry`, slot
/// 0x18) with a skin instance, the skin instance's root parent becomes
/// `root`.
pub fn bs_ni_node_reparent_skin_instances(e: &mut Engine, this: Node, node: Ptr, root: Ptr) {
    if node.is_null() || root.is_null() {
        return;
    }
    let mut index = 0;
    while index < child_count(e, node.addr()) {
        let child = child_at(e, node.addr(), index);
        if child != 0 {
            let as_node = e.vcall(child, SLOT_IS_NODE, &args![]).u32();
            if as_node != 0 {
                e.vcall(
                    this.addr(),
                    SLOT_REPARENT_SKIN_INSTANCES,
                    &args![as_node, root],
                );
            } else {
                let geometry = e.vcall(child, SLOT_IS_GEOMETRY, &args![]).u32();
                if geometry != 0 {
                    let skin = e.mem.u32(geometry + GEOMETRY_SKIN_INSTANCE);
                    if skin != 0 {
                        e.mem.set_u32(skin + SKIN_INSTANCE_ROOT_PARENT, root.addr());
                    }
                }
            }
        }
        index += 1;
    }
}

/// The inline `NiIsKindOf(NiControllerManager, controller)` of `00c5f680`:
/// walks the type chain from the controller's `GetRTTI` (slot 8) through
/// each block's base pointer (+4).
fn is_controller_manager(e: &mut Engine, controller: u32) -> bool {
    if controller == 0 {
        return false;
    }
    let mut rtti = e.vcall(controller, SLOT_GET_RTTI, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_CONTROLLER_MANAGER_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00c5f680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a `BSNiNode` in `this` from the node `source`: constructs the
/// `NiNode`, sets the vtable and copies the name. With `check_animated`
/// set, a source that has controllers or is an `NiBillboardNode` is not
/// taken apart: `00c5fb80` is called with the source and `"Anim"` and the
/// source becomes this node's child.
///
/// Otherwise the controllers are moved (`NiTimeController::SetTarget` for an
/// `NiControllerManager`, the controller's slot 0x98 for the others), the
/// collision object is set on this node and cleared on the source, every
/// child is attached here, the properties are attached here one by one
/// (each removed from the source by `00654640`), the local translation and
/// rotation are copied and the scale is copied as its absolute value, the
/// extra data are moved under the lock at `011f4380`, the source is
/// detached from its parent (slot 0xE4, the `NiPointer` it hands back is
/// released) and the skin instances below this node are reparented to it.
///
/// Either way, a parent of the source then gets this node attached (slot
/// 0xDC with flag 1), and the world bound is updated. Returns `this`.
// Translation incomplete: 00c5f680 - C++ exception unwinding frame is not translated.
pub fn fn_00c5f680(e: &mut Engine, this: Node, source: Ptr, check_animated: bool) -> Node {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BS_NI_NODE_VTABLE);
    let mut take_apart = true;
    let source_av = source.cast::<NiAVObject>();
    let parent = e.get(source_av, NiAVObject::m_pkParent);
    e.call(SET_NAME, &args![this, source.addr() + 8]);
    if check_animated {
        let has_controllers = e.get(source_av, NiAVObject::m_spControllers) != 0;
        let animated = has_controllers
            || (!source.is_null()
                && e.call(IS_KIND_OF, &args![source, NI_BILLBOARD_NODE_RTTI])
                    .u8()
                    != 0);
        if animated {
            take_apart = false;
            fn_00c5fb80(e, source, ANIM_SUFFIX);
        }
    }

    if take_apart {
        // Controllers.
        let mut controller = e.get(source_av, NiAVObject::m_spControllers);
        while controller != 0 {
            let next = e.mem.u32(controller + CONTROLLER_NEXT);
            if is_controller_manager(e, controller) {
                e.call(NI_TIME_CONTROLLER_SET_TARGET, &args![controller, this]);
            } else {
                e.vcall(controller, SLOT_CONTROLLER_SET_TARGET, &args![this]);
            }
            controller = next;
        }

        // Collision object.
        let collision = e.get(source_av, NiAVObject::m_spCollisionObject);
        e.call(NI_AV_OBJECT_SET_COLLISION_OBJECT, &args![this, collision]);
        e.call(NI_AV_OBJECT_SET_COLLISION_OBJECT, &args![source, 0u32]);

        // Children.
        let mut index = 0;
        while index < child_count(e, source.addr()) {
            let child = child_at(e, source.addr(), index);
            if child != 0 {
                e.call(NI_NODE_ATTACH_CHILD, &args![this, child, 1u32]);
            }
            index += 1;
        }

        // Properties.
        let properties: Ptr<NiTPointerList> = source_av.at(NiAVObject::m_kPropertyList);
        while e.get(properties, NiTPointerList::m_uiCount) != 0 {
            let head = Ptr::<NiTListItem>::new(e.get(properties, NiTPointerList::m_pkHead));
            let property = e.get(head, NiTListItem::m_element);
            e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![this, property]);
            e.call(REMOVE_PROPERTY, &args![source, property]);
        }

        // Local transform.
        for word in 0..3 {
            let value = e.mem.u32(source.addr() + LOCAL_TRANSLATE + 4 * word);
            e.mem
                .set_u32(this.addr() + LOCAL_TRANSLATE + 4 * word, value);
        }
        for word in 0..9 {
            let value = e.mem.u32(source.addr() + LOCAL_ROTATE + 4 * word);
            e.mem.set_u32(this.addr() + LOCAL_ROTATE + 4 * word, value);
        }
        let scale = x87_quiet(e.mem.f32(source.addr() + LOCAL_SCALE));
        let magnitude = e.call(FABS, &args![scale as f64]).f64();
        e.mem
            .set_f32(this.addr() + LOCAL_SCALE, x87_quiet(magnitude as f32));

        // Extra data.
        e.call(ENTER_CRITICAL_SECTION, &args![EXTRA_DATA_LOCK]);
        let source_net = source.cast::<NiObjectNET>();
        let count = e.get(source_net, NiObjectNET::m_usExtraDataSize);
        e.call(NI_OBJECT_NET_SET_EXTRA_DATA_SIZE, &args![this, count]);
        while e.get(source_net, NiObjectNET::m_usExtraDataSize) != 0 {
            let extras = e.get(source_net, NiObjectNET::m_ppkExtra);
            let extra = e.mem.u32(extras.addr());
            e.call(NI_OBJECT_NET_ADD_EXTRA_DATA, &args![this, extra]);
            e.call(REMOVE_EXTRA_DATA_AT, &args![source, 0u32]);
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![EXTRA_DATA_LOCK]);

        // Out of the parent.
        if parent != 0 {
            e.with_stack(4, |e, detached| {
                e.mem.set_u32(detached.addr(), 0);
                e.vcall(parent, SLOT_DETACH_CHILD, &args![source, detached]);
                let object = e.mem.u32(detached.addr());
                release(e, object);
            });
        }
        bs_ni_node_reparent_skin_instances(e, this, this.cast(), this.cast());
    } else {
        e.call(NI_NODE_ATTACH_CHILD, &args![this, source, 1u32]);
    }

    if parent != 0 {
        e.vcall(parent, SLOT_ATTACH_CHILD, &args![this, 1u32]);
    }
    e.call(NI_NODE_UPDATE_WORLD_BOUND, &args![this]);
    this
}

// Translated from 00c5faa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSNiNode` destructor body: restores the vtable and runs the
/// `NiNode` one.
pub fn fn_00c5faa0(e: &mut Engine, this: Node) {
    e.mem.set_u32(this.addr(), BS_NI_NODE_VTABLE);
    e.call(NI_NODE_DESTRUCT, &args![this]);
}

// Translated from 00c5fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiNode::CreateClone` (Xbox PDB): allocates 0xAC bytes and constructs a
/// `BSNiNode` (null if the allocation failed), copies this node's members
/// into it (`00c5fb60`) and calls the clone's slot 0xBC (no arguments).
/// Both still run on a null clone, as in the game.
// Translation incomplete: 00c5fac0 - C++ exception unwinding frame is not translated.
pub fn bs_ni_node_create_clone(e: &mut Engine, this: Node, clone_process: u32) -> Node {
    let block = e.call(ALLOCATE, &args![BS_NI_NODE_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        bs_ni_node_bs_ni_node(e, Ptr::new(block))
    };
    fn_00c5fb60(e, this, clone, clone_process);
    e.vcall(clone.addr(), SLOT_AFTER_CLONE, &args![]);
    clone
}

// Translated from 00c5fb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CopyMembers` `CreateClone` calls: forwards to the `NiNode` one
/// (`00a5dc80`).
pub fn fn_00c5fb60(e: &mut Engine, this: Node, target: Node, clone_process: u32) {
    e.call(NI_NODE_COPY_MEMBERS, &args![this, target, clone_process]);
}

// Translated from 00c5fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the object's name followed by `suffix` in a new block from the
/// memory manager (`strlen` of both plus one; `strcpy_s` of the name when
/// there is one, then `strcat_s` of the suffix). The block is neither
/// stored nor returned. Cdecl.
pub fn fn_00c5fb80(e: &mut Engine, object: Ptr, suffix: u32) {
    let name = e.mem.u32(object.addr() + 8);
    let name_length = if name == 0 {
        0
    } else {
        e.call(STRLEN, &args![name]).u32()
    };
    let suffix_length = e.call(STRLEN, &args![suffix]).u32();
    let size = name_length.wrapping_add(suffix_length).wrapping_add(1);
    let text = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    if name != 0 {
        e.call(STRCPY_S, &args![text, size, name]);
    }
    e.call(STRCAT_S, &args![text, size, suffix]);
}

// Translated from 00c5fc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiNode::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// body and, when bit 0 of `flags` is set, frees the 0xAC bytes. Returns
/// `this`.
pub fn bs_ni_node_scalar_deleting_destructor(e: &mut Engine, this: Node, flags: u32) -> Node {
    fn_00c5faa0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, BS_NI_NODE_SIZE]);
    }
    this
}

// Translated from 00c5fc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiNode::GetRTTI` (Xbox PDB): the address of the class's run-time type
/// information.
pub fn bs_ni_node_get_rtti(_e: &mut Engine, _this: Node) -> u32 {
    BS_NI_NODE_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5f570,
            bs_ni_node_bs_ni_node(Ptr<BSNiNode>) -> Ptr<BSNiNode>
        ),
        entry!(
            0x00c5f5a0,
            bs_ni_node_reparent_skin_instances(Ptr<BSNiNode>, Ptr, Ptr)
        ),
        entry!(
            0x00c5f680,
            fn_00c5f680(Ptr<BSNiNode>, Ptr, bool) -> Ptr<BSNiNode>
        ),
        entry!(0x00c5faa0, fn_00c5faa0(Ptr<BSNiNode>)),
        entry!(
            0x00c5fac0,
            bs_ni_node_create_clone(Ptr<BSNiNode>, u32) -> Ptr<BSNiNode>
        ),
        entry!(0x00c5fb60, fn_00c5fb60(Ptr<BSNiNode>, Ptr<BSNiNode>, u32)),
        entry!(0x00c5fb80, fn_00c5fb80(Ptr, u32)),
        entry!(
            0x00c5fc20,
            bs_ni_node_scalar_deleting_destructor(Ptr<BSNiNode>, u32) -> Ptr<BSNiNode>
        ),
        entry!(0x00c5fc60, bs_ni_node_get_rtti(Ptr<BSNiNode>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    /// Fake vtables and slot functions.
    const OBJECT_VTABLE: u32 = 0x7200_0000;
    const PARENT_VTABLE: u32 = 0x7200_1000;
    const CONTROLLER_VTABLE: u32 = 0x7200_2000;
    const MANAGER_VTABLE: u32 = 0x7200_3000;
    const RETURN_THIS: u32 = 0x7201_0000;
    const RETURN_NULL: u32 = 0x7201_1000;
    const NOTHING: u32 = 0x7201_2000;
    const PARENT_DETACH: u32 = 0x7201_3000;
    const CONTROLLER_RTTI: u32 = 0x7201_4000;
    const MANAGER_RTTI: u32 = 0x7201_5000;
    const DELETE_THIS: u32 = 0x7201_6000;

    fn test_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            NI_NODE_CONSTRUCT,
            NI_NODE_DESTRUCT,
            NI_NODE_COPY_MEMBERS,
            NI_NODE_ATTACH_CHILD,
            NI_NODE_UPDATE_WORLD_BOUND,
            SET_NAME,
            NI_TIME_CONTROLLER_SET_TARGET,
            NI_AV_OBJECT_SET_COLLISION_OBJECT,
            NI_AV_OBJECT_ATTACH_PROPERTY,
            NI_OBJECT_NET_SET_EXTRA_DATA_SIZE,
            NI_OBJECT_NET_ADD_EXTRA_DATA,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            FREE_SIZED,
            STRCPY_S,
            STRCAT_S,
            NOTHING,
            DELETE_THIS,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(STRLEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(RETURN_THIS, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(RETURN_NULL, |_, _| Ret::default());
        e.register(FABS, |_, a| Ret {
            st0: f64::take(a, &mut 0).abs(),
            ..Ret::default()
        });
        e.register(CONTROLLER_RTTI, |_, _| Ret {
            eax: 0x7300_0000,
            ..Ret::default()
        });
        e.register(MANAGER_RTTI, |_, _| Ret {
            eax: 0x7300_0100,
            ..Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        // `00654640`: drops the head of the source's property list.
        e.register(REMOVE_PROPERTY, |e, a| {
            let list = a[0] + 0x24;
            let head = e.mem.u32(list);
            let next = e.mem.u32(head);
            e.mem.set_u32(list, next);
            let count = e.mem.u32(list + 8);
            e.mem.set_u32(list + 8, count - 1);
            Ret::default()
        });
        // `00a5bf50`: drops the first extra data of the source.
        e.register(REMOVE_EXTRA_DATA_AT, |e, a| {
            let array = e.mem.u32(a[0] + 0x10);
            let count = e.mem.u16(a[0] + 0x14);
            for i in 1..count as u32 {
                let value = e.mem.u32(array + 4 * i);
                e.mem.set_u32(array + 4 * (i - 1), value);
            }
            e.mem.set_u16(a[0] + 0x14, count - 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        // The parent's detach: hands back an object with one reference.
        e.register(PARENT_DETACH, |e, a| {
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, OBJECT_VTABLE);
            e.mem.set_u32(object + 4, 1);
            e.mem.set_u32(a[2], object);
            Ret::default()
        });
        e.map(ANIM_SUFFIX & !0xfff, 0x1000);
        e.mem.set_cstr(ANIM_SUFFIX, b"Anim");
        // RTTI blocks: the controller's has no base, the manager's base is
        // `NiControllerManager`'s.
        e.map(0x7300_0000, 0x1000);
        e.mem.set_u32(0x7300_0104, NI_CONTROLLER_MANAGER_RTTI);
        let mut table = vec![NOTHING; 0x48];
        table[1] = DELETE_THIS;
        table[(SLOT_IS_NODE / 4) as usize] = RETURN_NULL;
        table[(SLOT_IS_GEOMETRY / 4) as usize] = RETURN_THIS;
        e.put_vtable(OBJECT_VTABLE, &table);
        let mut table = vec![NOTHING; 0x48];
        table[(SLOT_DETACH_CHILD / 4) as usize] = PARENT_DETACH;
        e.put_vtable(PARENT_VTABLE, &table);
        let mut table = vec![NOTHING; 0x48];
        table[2] = CONTROLLER_RTTI;
        e.put_vtable(CONTROLLER_VTABLE, &table);
        let mut table = vec![NOTHING; 0x48];
        table[2] = MANAGER_RTTI;
        e.put_vtable(MANAGER_VTABLE, &table);
        e
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A node of 0xAC bytes whose child array holds `kids`.
    fn node_with_children(e: &mut Engine, kids: &[u32]) -> u32 {
        let node = e.mem.alloc(BS_NI_NODE_SIZE);
        let array = e.mem.alloc(4 * kids.len().max(1) as u32);
        for (i, kid) in kids.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *kid);
        }
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, kids.len() as u16);
        node
    }

    #[test]
    fn constructor_builds_the_node() {
        let mut e = test_engine();
        let node: Node = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x00c5_f570, &args![node]).ptr::<BSNiNode>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, node);
        assert_eq!(e.mem.u32(node.addr()), BS_NI_NODE_VTABLE);
        assert_eq!(
            calls_to(&log, NI_NODE_CONSTRUCT),
            vec![vec![node.addr(), 0]]
        );
    }

    #[test]
    fn reparent_skin_instances_recurses_into_nodes_and_sets_skin_roots() {
        let mut e = test_engine();
        // A geometry child with a skin instance, one without, a node child
        // and an empty slot.
        let skinned = e.mem.alloc(0xc0);
        e.mem.set_u32(skinned, OBJECT_VTABLE);
        let skin = e.mem.alloc(0x14);
        e.mem.set_u32(skinned + GEOMETRY_SKIN_INSTANCE, skin);
        let bare = e.mem.alloc(0xc0);
        e.mem.set_u32(bare, OBJECT_VTABLE);
        let child_node = e.mem.alloc(0xc0);
        let mut table = vec![NOTHING; 0x48];
        table[(SLOT_IS_NODE / 4) as usize] = RETURN_THIS;
        e.put_vtable(0x7200_4000, &table);
        e.mem.set_u32(child_node, 0x7200_4000);
        let node = node_with_children(&mut e, &[skinned, 0, bare, child_node]);
        let this = e.mem.alloc(BS_NI_NODE_SIZE);
        let mut table = vec![NOTHING; 0x48];
        table[(SLOT_REPARENT_SKIN_INSTANCES / 4) as usize] = 0x7201_7000;
        e.put_vtable(0x7200_5000, &table);
        e.mem.set_u32(this, 0x7200_5000);
        e.register(0x7201_7000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_f5a0, &args![this, node, 0x5555u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(skin + SKIN_INSTANCE_ROOT_PARENT), 0x5555);
        assert_eq!(
            calls_to(&log, 0x7201_7000),
            vec![vec![this, child_node, 0x5555]]
        );

        // Nothing happens without a root.
        e.mem.set_u32(skin + SKIN_INSTANCE_ROOT_PARENT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f5a0, &args![this, node, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.mem.u32(skin + SKIN_INSTANCE_ROOT_PARENT), 0);
    }

    /// A source node for `00c5f680` with a parent, two controllers, a
    /// collision object, two children, two properties, a transform and two
    /// extra data.
    fn full_source(e: &mut Engine) -> (u32, u32) {
        let kid_a = e.mem.alloc(8);
        let kid_b = e.mem.alloc(8);
        let source = node_with_children(e, &[kid_a, 0, kid_b]);
        let parent = e.mem.alloc(0xac);
        e.mem.set_u32(parent, PARENT_VTABLE);
        e.mem.set_u32(source + 0x18, parent);
        let manager = e.mem.alloc(0x34);
        e.mem.set_u32(manager, MANAGER_VTABLE);
        let controller = e.mem.alloc(0x34);
        e.mem.set_u32(controller, CONTROLLER_VTABLE);
        e.mem.set_u32(manager + CONTROLLER_NEXT, controller);
        e.mem.set_u32(source + 0x0c, manager);
        e.mem.set_u32(source + 0x1c, 0xc011);
        let second = e.mem.alloc(0x0c);
        e.mem.set_u32(second + 8, 0x9902);
        let first = e.mem.alloc(0x0c);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x9901);
        e.mem.set_u32(source + 0x24, first);
        e.mem.set_u32(source + 0x2c, 2);
        for i in 0..13 {
            e.mem.set_f32(source + 0x34 + 4 * i, i as f32 + 0.5);
        }
        e.mem.set_f32(source + LOCAL_SCALE, -2.5);
        let extras = e.mem.alloc(8);
        e.mem.set_u32(extras, 0xe001);
        e.mem.set_u32(extras + 4, 0xe002);
        e.mem.set_u32(source + 0x10, extras);
        e.mem.set_u16(source + 0x14, 2);
        (source, parent)
    }

    #[test]
    fn convert_takes_the_source_apart() {
        let mut e = test_engine();
        let (source, parent) = full_source(&mut e);
        let manager = e.mem.u32(source + 0x0c);
        let controller = e.mem.u32(manager + CONTROLLER_NEXT);
        let kid_a = child_at(&e, source, 0);
        let kid_b = child_at(&e, source, 2);
        let this = e.mem.alloc(BS_NI_NODE_SIZE);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00c5_f680, &args![this, source, false])
            .ptr::<BSNiNode>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back.addr(), this);
        assert_eq!(e.mem.u32(this), BS_NI_NODE_VTABLE);
        assert_eq!(calls_to(&log, SET_NAME), vec![vec![this, source + 8]]);
        assert_eq!(calls_to(&log, IS_KIND_OF).len(), 0);
        assert!(calls_to(&log, STRCAT_S).is_empty());
        assert_eq!(
            calls_to(&log, NI_TIME_CONTROLLER_SET_TARGET),
            vec![vec![manager, this]]
        );
        assert!(log.contains(&(NOTHING, vec![controller, this])));
        assert_eq!(
            calls_to(&log, NI_AV_OBJECT_SET_COLLISION_OBJECT),
            vec![vec![this, 0xc011], vec![source, 0]]
        );
        assert_eq!(
            calls_to(&log, NI_NODE_ATTACH_CHILD),
            vec![vec![this, kid_a, 1], vec![this, kid_b, 1]]
        );
        assert_eq!(
            calls_to(&log, NI_AV_OBJECT_ATTACH_PROPERTY),
            vec![vec![this, 0x9901], vec![this, 0x9902]]
        );
        for i in 0..12 {
            assert_eq!(e.mem.f32(this + 0x34 + 4 * i), i as f32 + 0.5);
        }
        assert_eq!(e.mem.f32(this + LOCAL_SCALE), 2.5);
        assert_eq!(
            calls_to(&log, NI_OBJECT_NET_SET_EXTRA_DATA_SIZE),
            vec![vec![this, 2]]
        );
        assert_eq!(
            calls_to(&log, NI_OBJECT_NET_ADD_EXTRA_DATA),
            vec![vec![this, 0xe001], vec![this, 0xe002]]
        );
        assert_eq!(calls_to(&log, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls_to(&log, LEAVE_CRITICAL_SECTION).len(), 1);
        let detach = calls_to(&log, PARENT_DETACH);
        assert_eq!(detach.len(), 1);
        assert_eq!(&detach[0][..2], &[parent, source]);
        // The detached pointer had one reference: released and deleted.
        assert_eq!(calls_to(&log, DELETE_THIS).len(), 1);
        assert!(log.contains(&(NOTHING, vec![parent, this, 1])));
        assert_eq!(calls_to(&log, NI_NODE_UPDATE_WORLD_BOUND), vec![vec![this]]);
    }

    #[test]
    fn convert_keeps_an_animated_source_as_child() {
        let mut e = test_engine();
        let (source, parent) = full_source(&mut e);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Gun");
        e.mem.set_u32(source + 8, name);
        let this = e.mem.alloc(BS_NI_NODE_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f680, &args![this, source, true]);
        let log = e.call_log.take().unwrap();
        // Controllers present: no kind-of test, the name is extended.
        assert!(calls_to(&log, IS_KIND_OF).is_empty());
        let text = calls_to(&log, MEMORY_ALLOCATE);
        assert_eq!(text.len(), 1);
        assert_eq!(text[0][1], 3 + 4 + 1);
        assert_eq!(calls_to(&log, STRCAT_S)[0][2], ANIM_SUFFIX);
        assert_eq!(
            calls_to(&log, NI_NODE_ATTACH_CHILD),
            vec![vec![this, source, 1]]
        );
        assert!(calls_to(&log, NI_TIME_CONTROLLER_SET_TARGET).is_empty());
        assert!(calls_to(&log, PARENT_DETACH).is_empty());
        assert!(log.contains(&(NOTHING, vec![parent, this, 1])));
        assert_eq!(calls_to(&log, NI_NODE_UPDATE_WORLD_BOUND).len(), 1);
    }

    #[test]
    fn convert_tests_for_a_billboard_without_controllers() {
        let mut e = test_engine();
        let source = node_with_children(&mut e, &[]);
        e.register(IS_KIND_OF, |_, _| Ret {
            eax: 0xff01,
            ..Ret::default()
        });
        let this = e.mem.alloc(BS_NI_NODE_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f680, &args![this, source, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, IS_KIND_OF),
            vec![vec![source, NI_BILLBOARD_NODE_RTTI]]
        );
        assert_eq!(
            calls_to(&log, NI_NODE_ATTACH_CHILD),
            vec![vec![this, source, 1]]
        );

        // Not a billboard: taken apart (no parent, nothing to move).
        e.register(IS_KIND_OF, |_, _| Ret {
            eax: 0xff00,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_f680, &args![this, source, true]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, NI_NODE_ATTACH_CHILD).is_empty());
        assert_eq!(calls_to(&log, NI_AV_OBJECT_SET_COLLISION_OBJECT).len(), 2);
    }

    #[test]
    fn convert_without_the_check_never_tests() {
        let mut e = test_engine();
        let (source, _) = full_source(&mut e);
        let this = e.mem.alloc(BS_NI_NODE_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00c5_f680, &args![this, source, false]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, IS_KIND_OF).is_empty());
        assert!(calls_to(&log, STRLEN).is_empty());
        assert_eq!(calls_to(&log, NI_TIME_CONTROLLER_SET_TARGET).len(), 1);
    }

    #[test]
    fn destructor_body_restores_the_vtable() {
        let mut e = test_engine();
        let node: Node = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c5_faa0, &args![node]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(node.addr()), BS_NI_NODE_VTABLE);
        assert_eq!(calls_to(&log, NI_NODE_DESTRUCT), vec![vec![node.addr()]]);
    }

    #[test]
    fn create_clone_constructs_copies_and_calls_slot_0xbc() {
        let mut e = test_engine();
        // The clone's vtable is the game's; give the test one at the same
        // address with slot 0xBC.
        let mut table = vec![NOTHING; 0x48];
        table[(SLOT_AFTER_CLONE / 4) as usize] = 0x7201_8000;
        e.put_vtable(BS_NI_NODE_VTABLE, &table);
        e.register(0x7201_8000, |_, _| Ret::default());
        let this: Node = e.new_object();
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c5_fac0, &args![this, 0x66u32]).ptr::<BSNiNode>();
        let log = e.call_log.take().unwrap();
        assert!(!clone.is_null());
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0xac]]);
        assert_eq!(
            calls_to(&log, NI_NODE_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x66]]
        );
        assert_eq!(calls_to(&log, 0x7201_8000), vec![vec![clone.addr()]]);
    }

    #[test]
    fn copy_members_forwards() {
        let mut e = test_engine();
        e.call_log = Some(vec![]);
        e.call(0x00c5_fb60, &args![1u32, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NI_NODE_COPY_MEMBERS), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn name_with_suffix_handles_a_missing_name() {
        let mut e = test_engine();
        let object = e.mem.alloc(0x10);
        let suffix = e.mem.alloc(8);
        e.mem.set_cstr(suffix, b"Anim");
        e.call_log = Some(vec![]);
        e.call(0x00c5_fb80, &args![object, suffix]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRLEN), vec![vec![suffix]]);
        assert_eq!(calls_to(&log, MEMORY_ALLOCATE)[0], vec![MEMORY_MANAGER, 5]);
        assert!(calls_to(&log, STRCPY_S).is_empty());
        assert_eq!(&calls_to(&log, STRCAT_S)[0][1..], &[5, suffix]);

        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Node");
        e.mem.set_u32(object + 8, name);
        e.call_log = Some(vec![]);
        e.call(0x00c5_fb80, &args![object, suffix]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMORY_ALLOCATE)[0], vec![MEMORY_MANAGER, 9]);
        assert_eq!(&calls_to(&log, STRCPY_S)[0][1..], &[9, name]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = test_engine();
        let node: Node = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c5_fc20, &args![node, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FREE_SIZED).is_empty());
        assert_eq!(calls_to(&log, NI_NODE_DESTRUCT).len(), 1);
        e.call_log = Some(vec![]);
        let back = e.call(0x00c5_fc20, &args![node, 1u32]).ptr::<BSNiNode>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, node);
        assert_eq!(calls_to(&log, FREE_SIZED), vec![vec![node.addr(), 0xac]]);
    }

    #[test]
    fn get_rtti_is_the_class_block() {
        let mut e = test_engine();
        assert_eq!(e.call(0x00c5_fc60, &args![0u32]).u32(), 0x0120_4380);
    }
}
