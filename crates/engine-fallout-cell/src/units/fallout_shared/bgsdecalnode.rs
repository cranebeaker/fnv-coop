//! `fallout shared/bgsdecalnode.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSDecalNode` is a `NiNode` (vtable `01023734`, 0xb4 bytes on PC) that
//! owns a list of the objects hanging on it (`pDecalList`, at 0xac) and a flag
//! byte (`bSkinnedNode`, at 0xb0). Every node registers itself in a global
//! list (`011c7d20`) when it is built and leaves it when it is destroyed; the
//! objects it owns are released through another global list (`011d6a30`)
//! with a counter (`011d6a20`).
//!
//! Functions that keep an exception-unwinding frame (`FS:[0]`) are marked
//! `Translation incomplete`: the unwind cleanup is not translated, the normal
//! path is.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSDecalNode`'s vtable.
const DECAL_NODE_VTABLE: u32 = 0x0102_3734;
/// The address `GetRTTI` returns.
const DECAL_NODE_RTTI: u32 = 0x011c_7d34;
/// The list every decal node is added to by its constructors and removed
/// from by its destructor (the head node sits in the data section).
const DECAL_NODE_LIST: u32 = 0x011c_7d20;
/// The global list `004ee2e0` removes an object from, and the counter it
/// lowers.
const RELEASED_LIST: u32 = 0x011d_6a30;
const RELEASED_COUNT: u32 = 0x011d_6a20;
/// The word `004ee440` returns.
const DEFAULT_NODE_VALUE: u32 = 0x011c_61d4;
/// Size of the node `004ee390` allocates for `pDecalList`.
const LIST_NODE_SIZE: u32 = 8;
/// Size of a `BGSDecalNode` on PC.
const DECAL_NODE_SIZE: u32 = 0xb4;

/// `NiNode` base constructor (`00a5ecb0`, one argument).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `NiNode` destructor body (`00a5e930`).
const NI_NODE_DESTRUCT: u32 = 0x00a5_e930;
/// Adds the pointer stored at the given address to a list (`005ae3d0`).
const LIST_ADD: u32 = 0x005a_e3d0;
/// Removes the pointer stored at the given address from a list (`00905330`).
const LIST_REMOVE_ENTRY: u32 = 0x0090_5330;
/// Test on a list node (`004a4460`); a non-zero result ends the walks here.
const LIST_NODE_IS_EMPTY: u32 = 0x004a_4460;
/// Returns its `this` (`006815c0`): the address of a list node's item.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// Returns the word at +4 (`00726070`): the next list node.
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// Returns the word at +0 (`00559450`): the object a pointer holder holds.
const POINTER_GET: u32 = 0x0055_9450;
/// Pointer holder constructor (`00633c90`): stores its argument, adds a
/// reference when it is not null.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// Releases the object a pointer holder holds (`0045cec0`).
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// Removes a pointer holder's object from a list (`00631620`, the shared
/// body the engine map names after a `BSSimpleList` instance).
const LIST_REMOVE: u32 = 0x0063_1620;
/// Pointer holder assignment from a raw pointer (`0066b0d0`).
const POINTER_ASSIGN_RAW: u32 = 0x0066_b0d0;
/// Pointer holder assignment from another holder (`006e5cc0`).
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Empties a list (`004ee920`).
const LIST_CLEAR: u32 = 0x004e_e920;
/// The game's allocator (`00401000`) and its free (`00401030`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `NiMemObject::operator new(size)` (`00aa13e0`) and `operator delete(ptr, size)` (`00aa1460`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const OPERATOR_DELETE: u32 = 0x00aa_1460;
/// `004b2960`: a test on the node; zero ends `OnVisible`.
const NODE_IS_ACTIVE: u32 = 0x004b_2960;
/// Constructor (`0043d410`) of a 12-byte struct from a float and two bytes.
const SMALL_STRUCT_CONSTRUCT: u32 = 0x0043_d410;
/// `00a59c60(object, struct)`, one pointer argument.
const SET_FROM_STRUCT: u32 = 0x00a5_9c60;
/// `NiAVObject::GetProperty(type)` (Xbox PDB name, `00a59d30`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `NiNode::AttachChild(child, flag)` (Xbox PDB name, `00a5ed10`).
const NODE_ATTACH_CHILD: u32 = 0x00a5_ed10;
/// `NiNode::DetachChild_ov2(child, out)` (Xbox PDB name, `00a5e780`).
const NODE_DETACH_CHILD: u32 = 0x00a5_e780;
/// `NiAVObject::Cull(argument)` (Xbox PDB name, `00a59e00`).
const OBJECT_CULL: u32 = 0x00a5_9e00;
/// `00a5b950(object, pointer)`: reads the word `pointer` points at.
const SET_FROM_POINTED_WORD: u32 = 0x00a5_b950;
/// `004a2c60`: returns the word at `011c61d0`.
const SKINNED_NODE_VALUE: u32 = 0x004a_2c60;
/// Number of children of a node (`0043b480`).
const NODE_CHILD_COUNT: u32 = 0x0043_b480;

layout! {
    /// `BGSDecalNode` (Xbox PDB) past its `NiNode` base, PC offsets (the
    /// Xbox PDB puts them at 0xd0 and 0xd4).
    pub struct BGSDecalNode: 0xb4 {
        /// `pDecalList` (Xbox PDB): `BSSimpleList<NiPointer<BSTempEffect>>*`.
        0xAC pDecalList: Ptr,
        /// `bSkinnedNode` (Xbox PDB).
        0xB0 bSkinnedNode: u8,
    }
}

/// Adds `node` to the global node list through a stack slot holding it.
fn add_to_node_list(e: &mut Engine, node: Ptr<BGSDecalNode>) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), node.addr());
        e.call(LIST_ADD, &args![DECAL_NODE_LIST, slot]);
    });
}

// Translated from 004ee050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee050 - the exception-unwinding frame (FS:[0]) is not translated.
/// Default constructor of `BGSDecalNode` (unnamed in the Xbox PDB): builds the
/// `NiNode` base with no children, stores the vtable, creates the decal list
/// (`004ee390`) and adds the node to the global node list.
pub fn fn_004ee050(e: &mut Engine, this: Ptr<BGSDecalNode>) -> Ptr<BGSDecalNode> {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0]);
    e.mem.set_u32(this.addr(), DECAL_NODE_VTABLE);
    fn_004ee390(e, this);
    add_to_node_list(e, this);
    this
}

// Translated from 004ee0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDecalNode::GetRTTI` (Xbox PDB): the address of the class's RTTI object.
pub fn bgs_decal_node_get_rtti(_e: &mut Engine, _this: Ptr<BGSDecalNode>) -> u32 {
    DECAL_NODE_RTTI
}

// Translated from 004ee0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDecalNode::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the 0xb4-byte block.
pub fn bgs_decal_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BGSDecalNode>,
    flags: u32,
) -> Ptr<BGSDecalNode> {
    fn_004ee1e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this, DECAL_NODE_SIZE]);
    }
    this
}

// Translated from 004ee120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee120 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BGSDecalNode::BGSDecalNode` (Xbox PDB): builds the node for `source`
/// (an object whose slot at 0xdc takes the new node and the flag 1), stores
/// `skinned` in `bSkinnedNode`, creates the decal list, passes `source` and a
/// zeroed struct (`0043d410`) to `00a59c60` and adds the node to the global
/// node list.
pub fn bgs_decal_node_bgs_decal_node(
    e: &mut Engine,
    this: Ptr<BGSDecalNode>,
    source: Ptr,
    skinned: u8,
) -> Ptr<BGSDecalNode> {
    e.call(NI_NODE_CONSTRUCT, &args![this, 0]);
    e.mem.set_u32(this.addr(), DECAL_NODE_VTABLE);
    e.set(this, BGSDecalNode::bSkinnedNode, skinned);
    fn_004ee390(e, this);
    e.vcall(source.addr(), 0xdc, &args![this, 1]);
    e.with_stack(12, |e, zero| {
        e.call(SMALL_STRUCT_CONSTRUCT, &args![zero, 0.0f32, 0, 0]);
        e.call(SET_FROM_STRUCT, &args![source, zero]);
    });
    add_to_node_list(e, this);
    this
}

// Translated from 004ee1e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee1e0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BGSDecalNode` destructor (unnamed in the Xbox PDB): restores the vtable,
/// removes the node from the global node list, releases every object of its
/// decal list (`004ee2e0`, then `RemoveHead`), deletes the list and runs the
/// `NiNode` destructor body.
pub fn fn_004ee1e0(e: &mut Engine, this: Ptr<BGSDecalNode>) {
    e.mem.set_u32(this.addr(), DECAL_NODE_VTABLE);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), this.addr());
        e.call(LIST_REMOVE_ENTRY, &args![DECAL_NODE_LIST, slot]);
    });
    let list = e.get(this, BGSDecalNode::pDecalList);
    if !list.is_null() {
        while !e.call(LIST_NODE_IS_EMPTY, &args![list]).bool() {
            let item = e.call(LIST_NODE_ITEM, &args![list]).u32();
            let object = e.call(POINTER_GET, &args![item]).u32();
            fn_004ee2e0(e, object);
            bs_simple_list_remove_head(e, list);
        }
        fn_004ee360(e, list, 1);
        e.set(this, BGSDecalNode::pDecalList, Ptr::NULL);
    }
    e.call(NI_NODE_DESTRUCT, &args![this]);
}

// Translated from 004ee2e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee2e0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Cdecl, one argument: removes `object` from the list at `011d6a30`
/// (through a temporary pointer holder), then lowers the counter at
/// `011d6a20`.
pub fn fn_004ee2e0(e: &mut Engine, object: u32) {
    e.with_stack(4, |e, holder| {
        e.call(POINTER_CONSTRUCT, &args![holder, object]);
        e.call(LIST_REMOVE, &args![RELEASED_LIST, holder]);
        e.call(POINTER_DESTRUCT, &args![holder]);
    });
    let count = e.mem.u32(RELEASED_COUNT);
    e.mem.set_u32(RELEASED_COUNT, count.wrapping_sub(1));
}

// Translated from 004ee360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the list the node owns (unnamed in the Xbox PDB):
/// empties the list (`004ee840`) and, when bit 0 of `flags` is set, frees it.
pub fn fn_004ee360(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004ee840(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 004ee390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee390 - the exception-unwinding frame (FS:[0]) is not translated.
/// Allocates the node's decal list (8 bytes, built by `004ee810`; null when
/// the allocation fails), stores it in `pDecalList` and calls `00a5b950` on
/// the node with the value of `004a2c60` for a skinned node, else that of
/// `004ee440`.
pub fn fn_004ee390(e: &mut Engine, this: Ptr<BGSDecalNode>) {
    let block = e.call(ALLOCATE, &args![LIST_NODE_SIZE]).u32();
    let list = if block == 0 {
        Ptr::NULL
    } else {
        fn_004ee810(e, Ptr::new(block))
    };
    e.set(this, BGSDecalNode::pDecalList, list);
    let value = if e.get(this, BGSDecalNode::bSkinnedNode) != 0 {
        e.call(SKINNED_NODE_VALUE, &[]).u32()
    } else {
        fn_004ee440(e)
    };
    e.call(SET_FROM_POINTED_WORD, &args![this, value]);
}

// Translated from 004ee440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `011c61d4`.
pub fn fn_004ee440(e: &mut Engine) -> u32 {
    e.mem.u32(DEFAULT_NODE_VALUE)
}

// Translated from 004ee450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDecalNode::AttachChild` (Xbox PDB): takes the node that `child`'s
/// slot at 0x98 returns (no arguments); when that node has a property of
/// type 3, the float at 0x5c of the property is set to `1.0` (`004ee4b0`);
/// then the node the slot returns is attached with `flag`.
pub fn bgs_decal_node_attach_child(e: &mut Engine, this: Ptr<BGSDecalNode>, child: Ptr, flag: u8) {
    let node = e.vcall(child.addr(), 0x98, &[]).u32();
    let property = e.call(GET_PROPERTY, &args![node, 3]).u32();
    if property != 0 {
        fn_004ee4b0(e, Ptr::new(property), 1.0);
    }
    let node = e.vcall(child.addr(), 0x98, &[]).u32();
    e.call(NODE_ATTACH_CHILD, &args![this, node, flag]);
}

// Translated from 004ee4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at 0x5c (`FLD`/`FSTP`: a signalling NaN is quieted).
pub fn fn_004ee4b0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_f32(this.addr().wrapping_add(0x5c), x87_quiet(value));
}

// Translated from 004ee4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDecalNode::AttachChild_ov2` (Xbox PDB): `NiNode::AttachChild(child, flag)`.
pub fn bgs_decal_node_attach_child_ov2(
    e: &mut Engine,
    this: Ptr<BGSDecalNode>,
    child: u32,
    flag: u8,
) {
    e.call(NODE_ATTACH_CHILD, &args![this, child, flag]);
}

// Translated from 004ee4f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee4f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Removes `object` from the node's decal list through a temporary pointer
/// holder.
pub fn fn_004ee4f0(e: &mut Engine, this: Ptr<BGSDecalNode>, object: u32) {
    let list = e.get(this, BGSDecalNode::pDecalList);
    e.with_stack(4, |e, holder| {
        e.call(POINTER_CONSTRUCT, &args![holder, object]);
        e.call(LIST_REMOVE, &args![list, holder]);
        e.call(POINTER_DESTRUCT, &args![holder]);
    });
}

// Translated from 004ee560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee560 - the exception-unwinding frame (FS:[0]) is not translated.
/// Calls `004ee5d0(object, holder)` with a temporary pointer holder built
/// from null, released afterwards.
pub fn fn_004ee560(e: &mut Engine, this: Ptr<BGSDecalNode>, object: Ptr) {
    e.with_stack(4, |e, holder| {
        e.call(POINTER_CONSTRUCT, &args![holder, 0]);
        fn_004ee5d0(e, this, object, holder);
        e.call(POINTER_DESTRUCT, &args![holder]);
    });
}

// Translated from 004ee5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `object` from the decal list (`004ee4f0`), then detaches the node
/// that `object`'s slot at 0x98 returns (no arguments) from this node, with
/// `holder` as the second argument.
pub fn fn_004ee5d0(e: &mut Engine, this: Ptr<BGSDecalNode>, object: Ptr, holder: Ptr) {
    fn_004ee4f0(e, this, object.addr());
    let node = e.vcall(object.addr(), 0x98, &[]).u32();
    e.call(NODE_DETACH_CHILD, &args![this, node, holder]);
}

// Translated from 004ee610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee610 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BGSDecalNode::OnVisible` (Xbox PDB): when `004b2960` is non-zero, walks
/// the decal list from its first node; for each entry the slot at 0x98 of
/// the held object gives a node, which is `Cull`ed with `argument` when it
/// is not null. The walk ends at a null node or when `004a4460` reports the
/// node.
pub fn bgs_decal_node_on_visible(e: &mut Engine, this: Ptr<BGSDecalNode>, argument: u32) {
    if e.call(NODE_IS_ACTIVE, &args![this]).u32() == 0 {
        return;
    }
    let mut cursor = e.get(this, BGSDecalNode::pDecalList).addr();
    while cursor != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![cursor]).bool() {
        let item = e.call(LIST_NODE_ITEM, &args![cursor]).u32();
        let object = e.call(POINTER_GET, &args![item]).u32();
        let node = e.vcall(object, 0x98, &[]).u32();
        e.with_stack(4, |e, holder| {
            e.call(POINTER_CONSTRUCT, &args![holder, node]);
            if e.call(POINTER_GET, &args![holder]).u32() != 0 {
                let target = e.call(POINTER_GET, &args![holder]).u32();
                e.call(OBJECT_CULL, &args![target, argument]);
            }
            cursor = e.call(LIST_NODE_NEXT, &args![cursor]).u32();
            e.call(POINTER_DESTRUCT, &args![holder]);
        });
    }
}

// Translated from 004ee6f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee6f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BGSDecalNode::CreateClone` (Xbox PDB): allocates 0xb4 bytes and builds a
/// default node in them (null when the allocation fails). The argument is
/// never read.
pub fn bgs_decal_node_create_clone(
    e: &mut Engine,
    _this: Ptr<BGSDecalNode>,
    _unused_1: u32,
) -> Ptr<BGSDecalNode> {
    let block = e.call(OPERATOR_NEW, &args![DECAL_NODE_SIZE]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        fn_004ee050(e, Ptr::new(block))
    }
}

// Translated from 004ee770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the node's decal list (`004ee920`) when it has one, then calls the
/// slot at 0xf0 with each index below the child count (`0043b480`, read
/// again every time).
pub fn fn_004ee770(e: &mut Engine, this: Ptr<BGSDecalNode>) {
    let list = e.get(this, BGSDecalNode::pDecalList);
    if !list.is_null() {
        e.call(LIST_CLEAR, &args![list]);
    }
    let mut index = 0u32;
    while index < e.call(NODE_CHILD_COUNT, &args![this]).u32() {
        e.vcall(this.addr(), 0xf0, &args![index]);
        index = index.wrapping_add(1);
    }
}

// Translated from 004ee7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004ee770` on the item of every node of the global node list
/// (`011c7d20`) whose item is not null.
pub fn fn_004ee7d0(e: &mut Engine) {
    let mut node = DECAL_NODE_LIST;
    while node != 0 {
        let item_address = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        if item != 0 {
            fn_004ee770(e, Ptr::new(item));
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 004ee810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds an empty list node: its item holder constructed from null
/// (`00633c90`) and the next link cleared.
pub fn fn_004ee810(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POINTER_CONSTRUCT, &args![this, 0]);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    this
}

// Translated from 004ee840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ee840 - the exception-unwinding frame (FS:[0]) is not translated.
/// List destructor (unnamed in the Xbox PDB): empties the list (`004ee920`)
/// and releases its item holder (`0045cec0`).
pub fn fn_004ee840(e: &mut Engine, this: Ptr) {
    e.call(LIST_CLEAR, &args![this]);
    e.call(POINTER_DESTRUCT, &args![this]);
}

// Translated from 004ee8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<NiPointer<BSTempEffect>>::RemoveHead` (Xbox PDB): with a
/// second node, the head takes the second node's item (`006e5cc0`), links past
/// it and the second node is deleted (`004ee360`, flag 1); with only the
/// head, its item is set to null (`0066b0d0`).
pub fn bs_simple_list_remove_head(e: &mut Engine, this: Ptr) {
    let second = e.mem.u32(this.addr().wrapping_add(4));
    if second != 0 {
        let after = e.mem.u32(second.wrapping_add(4));
        e.mem.set_u32(this.addr().wrapping_add(4), after);
        e.call(POINTER_ASSIGN, &args![this, second]);
        e.mem.set_u32(second.wrapping_add(4), 0);
        fn_004ee360(e, Ptr::new(second), 1);
    } else {
        e.call(POINTER_ASSIGN_RAW, &args![this, 0]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004e_e050,
            fn_004ee050(Ptr<BGSDecalNode>) -> Ptr<BGSDecalNode>
        ),
        entry!(
            0x004e_e0d0,
            bgs_decal_node_get_rtti(Ptr<BGSDecalNode>) -> u32
        ),
        entry!(
            0x004e_e0e0,
            bgs_decal_node_scalar_deleting_destructor(Ptr<BGSDecalNode>, u32) -> Ptr<BGSDecalNode>
        ),
        entry!(
            0x004e_e120,
            bgs_decal_node_bgs_decal_node(Ptr<BGSDecalNode>, Ptr, u8) -> Ptr<BGSDecalNode>
        ),
        entry!(0x004e_e1e0, fn_004ee1e0(Ptr<BGSDecalNode>)),
        entry!(0x004e_e2e0, fn_004ee2e0(u32)),
        entry!(0x004e_e360, fn_004ee360(Ptr, u32) -> Ptr),
        entry!(0x004e_e390, fn_004ee390(Ptr<BGSDecalNode>)),
        entry!(0x004e_e440, fn_004ee440() -> u32),
        entry!(
            0x004e_e450,
            bgs_decal_node_attach_child(Ptr<BGSDecalNode>, Ptr, u8)
        ),
        entry!(0x004e_e4b0, fn_004ee4b0(Ptr, f32)),
        entry!(
            0x004e_e4d0,
            bgs_decal_node_attach_child_ov2(Ptr<BGSDecalNode>, u32, u8)
        ),
        entry!(0x004e_e4f0, fn_004ee4f0(Ptr<BGSDecalNode>, u32)),
        entry!(0x004e_e560, fn_004ee560(Ptr<BGSDecalNode>, Ptr)),
        entry!(0x004e_e5d0, fn_004ee5d0(Ptr<BGSDecalNode>, Ptr, Ptr)),
        entry!(
            0x004e_e610,
            bgs_decal_node_on_visible(Ptr<BGSDecalNode>, u32)
        ),
        entry!(
            0x004e_e6f0,
            bgs_decal_node_create_clone(Ptr<BGSDecalNode>, u32) -> Ptr<BGSDecalNode>
        ),
        entry!(0x004e_e770, fn_004ee770(Ptr<BGSDecalNode>)),
        entry!(0x004e_e7d0, fn_004ee7d0()),
        entry!(0x004e_e810, fn_004ee810(Ptr) -> Ptr),
        entry!(0x004e_e840, fn_004ee840(Ptr)),
        entry!(0x004e_e8a0, bs_simple_list_remove_head(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// Test knob words in a mapped page: what the list doubles saw.
    const LAST_ADDED: u32 = 0x011c_7ff0;
    const LAST_REMOVED_FROM_NODE_LIST: u32 = 0x011c_7ff4;
    const SLOT_ARGUMENTS: u32 = 0x011c_7ff8;
    /// Fake slot targets of the test vtables.
    const SLOT_TARGET: u32 = 0x7000_0001;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    type Calls = Vec<(u32, Vec<u32>)>;

    /// An engine with a double for every callee outside this file: list
    /// nodes are the real layout (item, next), `IsEmpty` means "item is null".
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011c_6000, 0x011c_7000, 0x011d_6000, 0x7000_0000] {
            e.map(page, 0x1000);
        }
        e.register(NI_NODE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(NI_NODE_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_ADD, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(LAST_ADDED, word);
            Ret::default()
        });
        e.register(LIST_REMOVE_ENTRY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(LAST_REMOVED_FROM_NODE_LIST, word);
            Ret::default()
        });
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0))
        });
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e.register(POINTER_ASSIGN_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_ASSIGN, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            ret(a[0])
        });
        e.register(LIST_CLEAR, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(FREE, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(NODE_IS_ACTIVE, |e, a| ret(e.mem.u32(a[0] + 0xa8)));
        e.register(SMALL_STRUCT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(SET_FROM_STRUCT, |_, _| Ret::default());
        e.register(GET_PROPERTY, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(NODE_ATTACH_CHILD, |_, _| Ret::default());
        e.register(NODE_DETACH_CHILD, |_, _| Ret::default());
        e.register(OBJECT_CULL, |_, _| Ret::default());
        e.register(SET_FROM_POINTED_WORD, |_, _| Ret::default());
        e.register(SKINNED_NODE_VALUE, |e, _| ret(e.mem.u32(0x011c_61d0)));
        e.register(NODE_CHILD_COUNT, |_, _| ret(0));
        e.set_global(0x011c_61d0, 0x1111u32);
        e.set_global(DEFAULT_NODE_VALUE, 0x2222u32);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine) -> Calls {
        e.call_log.clone().unwrap()
    }

    fn called(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        calls(e)
            .into_iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args)
            .collect()
    }

    fn node(e: &mut Engine) -> Ptr<BGSDecalNode> {
        e.new_object::<BGSDecalNode>()
    }

    /// An object whose vtable has `target` in each given slot.
    fn object_with_slots(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> Ptr {
        let size = slots.iter().map(|s| s.0).max().unwrap() / 4 + 1;
        let mut words = vec![0u32; size as usize];
        for (slot, target) in slots {
            words[(*slot / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &words);
        let object = Ptr::new(e.mem.alloc(0xc0));
        e.mem.set_u32(object.addr(), vtable);
        object
    }

    #[test]
    fn default_constructor_builds_the_list_and_registers_the_node() {
        let mut e = engine();
        let this = node(&mut e);
        e.call(0x004e_e050, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), DECAL_NODE_VTABLE);
        let list = e.get(this, BGSDecalNode::pDecalList);
        assert!(!list.is_null());
        assert_eq!(e.mem.u32(list.addr() + 4), 0);
        assert_eq!(e.mem.u32(LAST_ADDED), this.addr());
        assert_eq!(called(&e, NI_NODE_CONSTRUCT), vec![vec![this.addr(), 0]]);
        assert_eq!(
            called(&e, SET_FROM_POINTED_WORD),
            vec![vec![this.addr(), 0x2222]]
        );
    }

    #[test]
    fn get_rtti_returns_the_class_object() {
        let mut e = engine();
        let this = node(&mut e);
        assert_eq!(e.call(0x004e_e0d0, &args![this]).u32(), 0x011c_7d34);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = engine();
        let this = node(&mut e);
        let r = e.call(0x004e_e0e0, &args![this, 0]).u32();
        assert_eq!(r, this.addr());
        assert!(called(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(called(&e, NI_NODE_DESTRUCT).len(), 1);
        e.call(0x004e_e0e0, &args![this, 3]);
        assert_eq!(called(&e, OPERATOR_DELETE), vec![vec![this.addr(), 0xb4]]);
    }

    #[test]
    fn source_constructor_passes_the_node_to_the_source_and_zeroes_a_struct() {
        let mut e = engine();
        e.register(SLOT_TARGET, |e, a| {
            e.mem.set_u32(SLOT_ARGUMENTS, a[1]);
            e.mem.set_u32(SLOT_ARGUMENTS + 4, a[2]);
            Ret::default()
        });
        let source = object_with_slots(&mut e, 0x7000_0100, &[(0xdc, SLOT_TARGET)]);
        let this = node(&mut e);
        let r = e.call(0x004e_e120, &args![this, source, 1]).u32();
        assert_eq!(r, this.addr());
        assert_eq!(e.get(this, BGSDecalNode::bSkinnedNode), 1);
        assert_eq!(e.mem.u32(this.addr()), DECAL_NODE_VTABLE);
        assert_eq!(e.mem.u32(SLOT_ARGUMENTS), this.addr());
        assert_eq!(e.mem.u32(SLOT_ARGUMENTS + 4), 1);
        // A skinned node takes the other value.
        assert_eq!(
            called(&e, SET_FROM_POINTED_WORD),
            vec![vec![this.addr(), 0x1111]]
        );
        let built = called(&e, SMALL_STRUCT_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [0, 0, 0]);
        assert_eq!(
            called(&e, SET_FROM_STRUCT),
            vec![vec![source.addr(), built[0][0]]]
        );
        assert_eq!(e.mem.u32(LAST_ADDED), this.addr());
    }

    #[test]
    fn destructor_releases_every_listed_object_and_deletes_the_list() {
        let mut e = engine();
        let removed = Rc::new(RefCell::new(vec![]));
        let log = removed.clone();
        e.register_double(LIST_REMOVE, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let this = node(&mut e);
        let list = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(list, 0xA1);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(second, 0xA2);
        e.set(this, BGSDecalNode::pDecalList, Ptr::new(list));
        e.set_global(RELEASED_COUNT, 5u32);
        e.call(0x004e_e1e0, &args![this]);
        assert_eq!(
            *removed.borrow(),
            vec![(RELEASED_LIST, 0xA1), (RELEASED_LIST, 0xA2)]
        );
        assert_eq!(e.global::<u32>(RELEASED_COUNT), 3);
        assert_eq!(e.mem.u32(LAST_REMOVED_FROM_NODE_LIST), this.addr());
        assert!(e.get(this, BGSDecalNode::pDecalList).is_null());
        // The second node and the list itself are freed.
        assert_eq!(called(&e, FREE), vec![vec![second], vec![list]]);
        assert_eq!(calls(&e).last().unwrap().0, NI_NODE_DESTRUCT);
    }

    #[test]
    fn destructor_without_a_list_only_leaves_the_node_list() {
        let mut e = engine();
        let this = node(&mut e);
        e.call(0x004e_e1e0, &args![this]);
        assert_eq!(e.mem.u32(LAST_REMOVED_FROM_NODE_LIST), this.addr());
        assert!(called(&e, FREE).is_empty());
        assert_eq!(called(&e, NI_NODE_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn release_removes_the_object_and_wraps_the_counter() {
        let mut e = engine();
        let seen = Rc::new(Cell::new((0, 0)));
        let log = seen.clone();
        e.register_double(LIST_REMOVE, move |e, a| {
            log.set((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.set_global(RELEASED_COUNT, 0u32);
        e.call(0x004e_e2e0, &args![0xBEEF]);
        assert_eq!(seen.get(), (RELEASED_LIST, 0xBEEF));
        assert_eq!(e.global::<u32>(RELEASED_COUNT), u32::MAX);
        assert_eq!(called(&e, POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn list_deleting_destructor_frees_with_bit_zero() {
        let mut e = engine();
        let list: Ptr = Ptr::new(e.mem.alloc(8));
        let r = e.call(0x004e_e360, &args![list, 0]).u32();
        assert_eq!(r, list.addr());
        assert_eq!(called(&e, LIST_CLEAR), vec![vec![list.addr()]]);
        assert_eq!(called(&e, POINTER_DESTRUCT), vec![vec![list.addr()]]);
        assert!(called(&e, FREE).is_empty());
        e.call(0x004e_e360, &args![list, 1]);
        assert_eq!(called(&e, FREE), vec![vec![list.addr()]]);
    }

    #[test]
    fn list_creation_handles_allocation_failure_and_skinning() {
        let mut e = engine();
        let this = node(&mut e);
        e.call(0x004e_e390, &args![this]);
        let list = e.get(this, BGSDecalNode::pDecalList);
        assert!(!list.is_null());
        assert_eq!(called(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(called(&e, POINTER_CONSTRUCT), vec![vec![list.addr(), 0]]);
        assert_eq!(
            called(&e, SET_FROM_POINTED_WORD),
            vec![vec![this.addr(), 0x2222]]
        );
        // Skinned: the other value; allocation failure: no list.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.set(this, BGSDecalNode::bSkinnedNode, 1);
        e.call(0x004e_e390, &args![this]);
        assert!(e.get(this, BGSDecalNode::pDecalList).is_null());
        assert_eq!(
            called(&e, SET_FROM_POINTED_WORD)[1],
            vec![this.addr(), 0x1111]
        );
        assert_eq!(called(&e, POINTER_CONSTRUCT).len(), 1);
    }

    #[test]
    fn default_value_is_the_global_word() {
        let mut e = engine();
        assert_eq!(e.call(0x004e_e440, &[]).u32(), 0x2222);
    }

    #[test]
    fn attach_child_sets_a_found_property_to_one() {
        let mut e = engine();
        let child_node = e.mem.alloc(0x20);
        let property = e.mem.alloc(0x80);
        e.register(SLOT_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        let child = object_with_slots(&mut e, 0x7000_0200, &[(0x98, SLOT_TARGET)]);
        e.mem.set_u32(child.addr() + 0x14, child_node);
        e.mem.set_u32(child_node + 0x10, property);
        let this = node(&mut e);
        e.call(0x004e_e450, &args![this, child, 1]);
        assert_eq!(e.mem.f32(property + 0x5c), 1.0);
        assert_eq!(called(&e, GET_PROPERTY), vec![vec![child_node, 3]]);
        assert_eq!(
            called(&e, NODE_ATTACH_CHILD),
            vec![vec![this.addr(), child_node, 1]]
        );
        // No property: only the attach.
        e.mem.set_u32(child_node + 0x10, 0);
        e.mem.set_f32(property + 0x5c, 5.0);
        e.call(0x004e_e450, &args![this, child, 0]);
        assert_eq!(e.mem.f32(property + 0x5c), 5.0);
        assert_eq!(called(&e, NODE_ATTACH_CHILD).len(), 2);
    }

    #[test]
    fn float_store_quiets_signalling_nans() {
        let mut e = engine();
        let target: Ptr = Ptr::new(e.mem.alloc(0x80));
        let store = |e: &mut Engine, bits: u32| {
            e.call(0x004e_e4b0, &args![target, f32::from_bits(bits)]);
            e.mem.u32(target.addr() + 0x5c)
        };
        assert_eq!(store(&mut e, 1.5f32.to_bits()), 1.5f32.to_bits());
        assert_eq!(store(&mut e, 0x7f80_0001), 0x7fc0_0001);
        assert_eq!(store(&mut e, 0xff80_0001), 0xffc0_0001);
        assert_eq!(store(&mut e, 0x7fc0_0001), 0x7fc0_0001);
        assert_eq!(store(&mut e, 0x7f80_0000), 0x7f80_0000);
    }

    #[test]
    fn second_attach_child_forwards() {
        let mut e = engine();
        let this = node(&mut e);
        e.call(0x004e_e4d0, &args![this, 0x77, 1]);
        assert_eq!(
            called(&e, NODE_ATTACH_CHILD),
            vec![vec![this.addr(), 0x77, 1]]
        );
    }

    #[test]
    fn remove_by_holder_uses_the_nodes_list() {
        let mut e = engine();
        let this = node(&mut e);
        e.set(this, BGSDecalNode::pDecalList, Ptr::new(0x1234));
        e.call(0x004e_e4f0, &args![this, 0x55]);
        let removed = called(&e, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], 0x1234);
        assert_eq!(called(&e, POINTER_CONSTRUCT)[0][1], 0x55);
        assert_eq!(called(&e, POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn detach_removes_from_the_list_then_detaches_the_node() {
        let mut e = engine();
        e.register(SLOT_TARGET, |_, _| ret(0x9999));
        let object = object_with_slots(&mut e, 0x7000_0300, &[(0x98, SLOT_TARGET)]);
        let this = node(&mut e);
        e.set(this, BGSDecalNode::pDecalList, Ptr::new(0x1234));
        e.call(0x004e_e560, &args![this, object]);
        let order: Vec<u32> = calls(&e).iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x004e_e560,
                POINTER_CONSTRUCT,
                POINTER_CONSTRUCT,
                LIST_REMOVE,
                POINTER_DESTRUCT,
                SLOT_TARGET,
                NODE_DETACH_CHILD,
                POINTER_DESTRUCT,
            ]
        );
        assert_eq!(called(&e, POINTER_CONSTRUCT)[0][1], 0);
        let detach = &called(&e, NODE_DETACH_CHILD)[0];
        assert_eq!(detach[..2], [this.addr(), 0x9999]);
        assert_eq!(detach[2], called(&e, POINTER_CONSTRUCT)[0][0]);
    }

    /// A decal list of two nodes: the first holds an object whose slot
    /// returns a node, the second one whose slot returns null.
    fn visible_node(e: &mut Engine) -> (Ptr<BGSDecalNode>, u32) {
        e.register(SLOT_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        let first = object_with_slots(e, 0x7000_0400, &[(0x98, SLOT_TARGET)]);
        let second = object_with_slots(e, 0x7000_0500, &[(0x98, SLOT_TARGET)]);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(first.addr() + 0x14, target);
        let head = e.mem.alloc(8);
        let tail = e.mem.alloc(8);
        e.mem.set_u32(head, first.addr());
        e.mem.set_u32(head + 4, tail);
        e.mem.set_u32(tail, second.addr());
        let this = node(e);
        e.set(this, BGSDecalNode::pDecalList, Ptr::new(head));
        e.mem.set_u32(this.addr() + 0xa8, 1);
        (this, target)
    }

    #[test]
    fn on_visible_culls_each_listed_node() {
        let mut e = engine();
        let (this, target) = visible_node(&mut e);
        e.call(0x004e_e610, &args![this, 0xCAFE]);
        assert_eq!(called(&e, OBJECT_CULL), vec![vec![target, 0xCAFE]]);
        // Both entries were visited.
        assert_eq!(called(&e, POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn on_visible_does_nothing_when_inactive_or_empty() {
        let mut e = engine();
        let (this, _) = visible_node(&mut e);
        e.mem.set_u32(this.addr() + 0xa8, 0);
        e.call(0x004e_e610, &args![this, 1]);
        assert_eq!(calls(&e).len(), 2);
        // Active, but the first node reports itself.
        e.mem.set_u32(this.addr() + 0xa8, 1);
        let head = e.get(this, BGSDecalNode::pDecalList).addr();
        e.mem.set_u32(head, 0);
        e.call(0x004e_e610, &args![this, 1]);
        assert!(called(&e, OBJECT_CULL).is_empty());
        // No list.
        e.set(this, BGSDecalNode::pDecalList, Ptr::NULL);
        e.call(0x004e_e610, &args![this, 1]);
        assert!(called(&e, OBJECT_CULL).is_empty());
    }

    #[test]
    fn create_clone_builds_a_default_node_or_returns_null() {
        let mut e = engine();
        let this = node(&mut e);
        let clone = e
            .call(0x004e_e6f0, &args![this, 0xDEAD])
            .ptr::<BGSDecalNode>();
        assert!(!clone.is_null());
        assert_eq!(called(&e, OPERATOR_NEW), vec![vec![0xb4]]);
        assert_eq!(e.mem.u32(clone.addr()), DECAL_NODE_VTABLE);
        assert_eq!(e.mem.u32(LAST_ADDED), clone.addr());
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let none = e.call(0x004e_e6f0, &args![this, 0]).u32();
        assert_eq!(none, 0);
        assert_eq!(called(&e, NI_NODE_CONSTRUCT).len(), 1);
    }

    #[test]
    fn child_slot_runs_for_each_index_below_the_current_count() {
        let mut e = engine();
        let count = Rc::new(Cell::new(3u32));
        let counter = count.clone();
        e.register_double(NODE_CHILD_COUNT, move |_, _| ret(counter.get()));
        let indices = Rc::new(RefCell::new(vec![]));
        let seen = indices.clone();
        let counter = count.clone();
        e.register_double(SLOT_TARGET, move |_, a| {
            seen.borrow_mut().push(a[1]);
            counter.set(2);
            Ret::default()
        });
        let object = object_with_slots(&mut e, 0x7000_0600, &[(0xf0, SLOT_TARGET)]);
        let this = Ptr::<BGSDecalNode>::new(object.addr());
        let list = e.mem.alloc(8);
        e.set(this, BGSDecalNode::pDecalList, Ptr::new(list));
        e.call(0x004e_e770, &args![this]);
        assert_eq!(*indices.borrow(), vec![0, 1]);
        assert_eq!(called(&e, LIST_CLEAR), vec![vec![list]]);
        // No list and no children: nothing.
        e.set(this, BGSDecalNode::pDecalList, Ptr::NULL);
        count.set(0);
        e.call(0x004e_e770, &args![this]);
        assert_eq!(called(&e, LIST_CLEAR).len(), 1);
        assert_eq!(indices.borrow().len(), 2);
    }

    #[test]
    fn every_listed_node_with_an_item_is_emptied() {
        let mut e = engine();
        let make = |e: &mut Engine| {
            let n = node(e);
            let list = e.mem.alloc(8);
            e.set(n, BGSDecalNode::pDecalList, Ptr::new(list));
            (n, list)
        };
        let (first, first_list) = make(&mut e);
        let (third, third_list) = make(&mut e);
        let middle = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        e.mem.set_u32(DECAL_NODE_LIST, first.addr());
        e.mem.set_u32(DECAL_NODE_LIST + 4, middle);
        e.mem.set_u32(middle + 4, last);
        e.mem.set_u32(last, third.addr());
        e.call(0x004e_e7d0, &[]);
        assert_eq!(
            called(&e, LIST_CLEAR),
            vec![vec![first_list], vec![third_list]]
        );
    }

    #[test]
    fn list_node_construction_clears_the_next_link() {
        let mut e = engine();
        let list: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(list.addr() + 4, 7);
        let r = e.call(0x004e_e810, &args![list]).u32();
        assert_eq!(r, list.addr());
        assert_eq!(e.mem.u32(list.addr() + 4), 0);
        assert_eq!(called(&e, POINTER_CONSTRUCT), vec![vec![list.addr(), 0]]);
    }

    #[test]
    fn list_destruction_clears_then_releases() {
        let mut e = engine();
        e.call(0x004e_e840, &args![0x4000]);
        let order: Vec<u32> = calls(&e).iter().map(|c| c.0).collect();
        assert_eq!(order, vec![0x004e_e840, LIST_CLEAR, POINTER_DESTRUCT]);
    }

    #[test]
    fn remove_head_takes_the_second_nodes_item_or_clears() {
        let mut e = engine();
        let head = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(head, 1);
        e.mem.set_u32(head + 4, second);
        e.mem.set_u32(second, 2);
        e.mem.set_u32(second + 4, third);
        e.call(0x004e_e8a0, &args![head]);
        assert_eq!(e.mem.u32(head), 2);
        assert_eq!(e.mem.u32(head + 4), third);
        assert_eq!(e.mem.u32(second + 4), 0);
        let order: Vec<u32> = calls(&e).iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x004e_e8a0,
                POINTER_ASSIGN,
                LIST_CLEAR,
                POINTER_DESTRUCT,
                FREE
            ]
        );
        assert_eq!(called(&e, FREE), vec![vec![second]]);
        // Only the head left: its item is set to null.
        e.mem.set_u32(head + 4, 0);
        e.call(0x004e_e8a0, &args![head]);
        assert_eq!(e.mem.u32(head), 0);
        assert_eq!(called(&e, POINTER_ASSIGN_RAW), vec![vec![head, 0]]);
    }
}
