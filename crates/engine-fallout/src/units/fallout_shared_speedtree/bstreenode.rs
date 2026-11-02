//! `fallout shared/speedtree/bstreenode.cpp` (Xbox PDB source unit), subsystem `fallout shared/speedtree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066d470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0066d410`, which forwards `this` to `00c6c290`
/// (a destructor with an exception frame), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `0066d410` ends right after that call and `00c6c290` ends with its frame unlink and
/// register restores, loading no result; Ghidra types both void.
pub fn fn_0066d470(e: &mut Engine, this: Ptr) {
    e.call(0x0066_d410, &args![this]);
}

// Translated from 0066b5b0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5e70 without reading the receiver. The map names this body `BSTreeNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066b5b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5e70
}

// Translated from 0066d400 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01268168 without reading the receiver. The map names this body `bhkPCollisionObject::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d400(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_8168
}

// Translated from 0066d430 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012683d0 without reading the receiver. The map names this body `bhkSPCollisionObject::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d430(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_83d0
}

// Translated from 0066d510 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012683e0 without reading the receiver. The map names this body `bhkMultiSphereShape::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d510(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_83e0
}

// Translated from 0066d610 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012681b4 without reading the receiver. The map names this body `bhkTransformShape::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d610(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_81b4
}

// Translated from 0066d720 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0126832c without reading the receiver. The map names this body `bhkCapsuleShape::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d720(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_832c
}

/// The callees of this unit's code (all `thiscall` unless said; named by
/// the engine map where it names them).
/// `00559450` (unnamed, no words): the object a `NiPointer` slot holds.
const POINTER_GET: u32 = 0x0055_9450;
/// `0066b0d0` (unnamed, one word): a `NiPointer` assignment.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `00633c90` (unnamed, one word): a `NiPointer` constructor.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0045cec0` (unnamed, no words): a `NiPointer` destructor.
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `0043b4a0` (unit `modelloader.cpp`, one word: 0 branch, 1 leaf, 2
/// billboard): the node of that kind.
const NODE_OF_KIND: u32 = 0x0043_b4a0;
/// `00664840` (`cdecl`, one word): the tree manager (here for 1).
const TREE_MANAGER: u32 = 0x0066_4840;
/// `BSTreeModel::GetNumBranchLODLevels` (`00669990`, no words; `u16`) and
/// its leaf counterpart (`006699d0`).
const BRANCH_LEVEL_COUNT: u32 = 0x0066_9990;
const LEAF_LEVEL_COUNT: u32 = 0x0066_99d0;
/// `00667120` (unit `bstreemodel.cpp`, one word: the flags): a vector
/// deleting destructor of the level arrays.
const LEVEL_ARRAY_DELETE: u32 = 0x0066_7120;
/// `00b4e1e0` (unit `bsfadenode.cpp`, no words): the `BSFadeNode` destructor body.
const FADE_NODE_DESTRUCTOR: u32 = 0x00b4_e1e0;
/// `BSFadeNode::BSFadeNode` (`00b4e150`, no words).
const FADE_NODE_CONSTRUCT: u32 = 0x00b4_e150;
/// `BSFadeNode::SetLODMultType` (`00b4dec0`, one word).
const FADE_NODE_SET_LOD_MULT_TYPE: u32 = 0x00b4_dec0;
/// `004902f0` (unit `animation.cpp`, one word).
const NODE_SET_FLAG_004902F0: u32 = 0x0049_02f0;
/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a5ecb0` (unit `ninode.cpp`, one word: the child count): an `NiNode`
/// constructor.
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `NiNode::SetAt_ov2` (`00a93ea0`, two words: the index, the child).
const NI_NODE_SET_AT: u32 = 0x00a9_3ea0;
/// `0051cf00` (unit `tesobjecttree.cpp`, no words): the billboard node
/// constructor; `0051d000` (one word) a setter on it; `006404b0` (four float
/// words) another.
const BILLBOARD_NODE_CONSTRUCT: u32 = 0x0051_cf00;
const BILLBOARD_NODE_SET_0051D000: u32 = 0x0051_d000;
const BILLBOARD_NODE_SET_006404B0: u32 = 0x0064_04b0;
/// `-pi/2` (`float`, `01016b78`), the first float the constructor passes.
const MINUS_HALF_PI: u32 = 0x0101_6b78;
/// `00401000` (`cdecl`, one word): `operator new[]`.
const OPERATOR_NEW_ARRAY: u32 = 0x0040_1000;
/// `00ec782f` (`stdcall`, five words: block, size, count, constructor,
/// destructor): the compiler's `eh vector constructor iterator`.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The element constructor and destructor the level arrays use.
const LEVEL_ELEMENT_CONSTRUCT: u32 = 0x0066_94e0;
const LEVEL_ELEMENT_DESTRUCT: u32 = 0x0045_cec0;
/// `NiObjectNET::AddExtraData_ov2` (`00a5bca0`, one word).
const ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// `0066a630` (unit `bstreemodel.cpp`, no words): the model's Havok object.
const MODEL_HAVOK_OBJECT: u32 = 0x0066_a630;
/// `0066b640` (this unit, not in this file's list, no words): the
/// collision object constructor the constructor uses.
const COLLISION_OBJECT_CONSTRUCT: u32 = 0x0066_b640;
/// `bhkNiCollisionObject::SetWorldObject` (`00c6b980`, one word).
const COLLISION_SET_WORLD_OBJECT: u32 = 0x00c6_b980;
/// The `BSTreeNode` vtable.
const TREE_NODE_VTABLE: u32 = 0x0106_68e4;
/// `00450f90` (unit `tes.cpp`, one word): sets an object's hidden state.
const SET_HIDDEN: u32 = 0x0045_0f90;
/// `NiAVObject::GetProperty` (engine map; `00a59d30`, one word).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `0094db80` (unit `playercharacter.cpp`, one word): a setter on that
/// property.
const PROPERTY_SET_0094DB80: u32 = 0x0094_db80;
/// `0054ee20` (unit `tesobjectcell.cpp`, `cdecl`, no words; `bool`).
const TREES_ENABLED: u32 = 0x0054_ee20;
/// `NiNode::OnVisible` (`00a5dbe0`, one word).
const NI_NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;
/// The profiling scope `00404eb0` (four words) / `00404ee0` the update
/// passes keep on their frame, and the source-file string it names.
const SCOPE_ENTER: u32 = 0x0040_4eb0;
const SCOPE_LEAVE: u32 = 0x0040_4ee0;
const SOURCE_FILE_NAME: u32 = 0x0106_6a30;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `00552380` (`cdecl`, no words) and `004b5fc0` (`cdecl`, two words): the
/// check the update passes make for a positive time.
const UPDATE_CHECK_ARGUMENT: u32 = 0x0055_2380;
const UPDATE_CHECK: u32 = 0x004b_5fc0;
/// `NiNode::UpdateDownwardPass` and `NiNode::UpdateSelectedDownwardPass`
/// (two words each).
const NI_NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
const NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS: u32 = 0x00a5_ded0;
/// `00620b80`, `00c89720` (`bhkCharacterProxy::UnMarkRead` in the engine
/// map), `hkpCharacterProxy::getLinearVelocity` (`00cac950`) and `00458620`
/// (`cdecl`, two words): the steps of `0066ca00`.
const CHARACTER_PROXY_OBJECT: u32 = 0x0062_0b80;
const CHARACTER_PROXY_UNMARK_READ: u32 = 0x00c8_9720;
const HK_CHARACTER_PROXY_GET_LINEAR_VELOCITY: u32 = 0x00ca_c950;
const VECTOR_FROM_HAVOK: u32 = 0x0045_8620;
/// `00403e20` (unit `effectitem.cpp`, no words): the address of the float
/// value of the object at `011d5e64`; `0043d4d0` the address of the integer
/// value of the one at `011d5e44`; `011d5e3c` a byte counter.
const FLOAT_VALUE_ADDRESS: u32 = 0x0040_3e20;
const INT_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const TIMER_INTERVAL: u32 = 0x011d_5e64;
const TIMER_SLOTS: u32 = 0x011d_5e44;
const TIMER_COUNTER: u32 = 0x011d_5e3c;
/// `00c6c290` (unit `bhknicollisionobject.obj`, no words).
const COLLISION_OBJECT_DESTRUCTOR: u32 = 0x00c6_c290;
/// `bhkCapsuleShape::~bhkCapsuleShape` (`00caacf0`, no words).
const CAPSULE_SHAPE_DESTRUCTOR: u32 = 0x00ca_acf0;
/// The steps of `0066d7c0`: `006838b0` (no words), `006fa820` (no words) and
/// `00653270` (`cdecl`, two words: `01268230`, the object).
const LEAF_OBJECT_006838B0: u32 = 0x0068_38b0;
const OBJECT_006FA820: u32 = 0x006f_a820;
const CAST_00653270: u32 = 0x0065_3270;
const RTTI_01268230: u32 = 0x0126_8230;

layout! {
    /// `BSTreeNode` (Xbox PDB name; PC offsets as the code uses them).
    pub struct BSTreeNode: 0xF8 {
        /// The tree model (`NiPointer<BSTreeModel>`).
        0xE4 spTreeModel: u32,
        /// The branch level array (`NiPointer` elements).
        0xE8 pBranchLevels: u32,
        /// The leaf level array.
        0xEC pLeafLevels: u32,
        /// The billboard node (`NiPointer`).
        0xF0 spBillboard: u32,
        /// A float timer (`0066ca50`), as bits.
        0xF4 fTimer: u32,
    }
}

// Translated from 0066b120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066b120 - the FS:[0] exception frame (states 0 to 8, unwind handler 00f003bf) is not translated
/// `BSTreeNode::BSTreeNode` (Xbox PDB; `thiscall`, two words: the tree model,
/// a Havok object): the `BSFadeNode` part, the vtable, the model and
/// billboard pointers, LOD multiplier type 6, the model, `004902f0(1)`; two
/// child nodes (0xAC bytes, `00a5ecb0(0)`, null on a failed allocation) at
/// children 0 and 1 and a billboard node (0xB4 bytes, `0051cf00`; set up with
/// `0051d000(1)` and `006404b0(-pi/2, 1, 0, 0)` without a null check) at 2;
/// the branch and leaf level arrays (`operator new[]` of `count * 4 + 4`
/// saturating, the count in front, every element constructed and set to
/// null; no array for a zero count), the billboard cleared, the tree
/// manager's extra data (`0066b660`) added; with a Havok object (the
/// argument, else the model's) a collision object (0x14 bytes, `0066b640`)
/// gets it as world object and is attached (vtable `+0x8C`); the timer 0.
pub fn bs_tree_node_bs_tree_node(
    e: &mut Engine,
    this: Ptr<BSTreeNode>,
    model: u32,
    havok_object: u32,
) -> Ptr<BSTreeNode> {
    let t = this.addr();
    e.call(FADE_NODE_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, TREE_NODE_VTABLE);
    e.call(POINTER_CONSTRUCT, &args![t + 0xe4, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![t + 0xf0, 0u32]);
    e.call(FADE_NODE_SET_LOD_MULT_TYPE, &args![this, 6u32]);
    e.call(POINTER_ASSIGN, &args![t + 0xe4, model]);
    e.call(NODE_SET_FLAG_004902F0, &args![this, 1u32]);
    for index in 0..2u32 {
        let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
        let child = if block == 0 {
            0
        } else {
            e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
        };
        e.call(NI_NODE_SET_AT, &args![this, index, child]);
    }
    let block = e.call(NI_ALLOCATE, &args![0xb4u32]).u32();
    let billboard = if block == 0 {
        0
    } else {
        e.call(BILLBOARD_NODE_CONSTRUCT, &args![block]).u32()
    };
    e.call(BILLBOARD_NODE_SET_0051D000, &args![billboard, 1u32]);
    let first = x87_quiet_bits(e.global::<u32>(MINUS_HALF_PI));
    e.call(
        BILLBOARD_NODE_SET_006404B0,
        &args![billboard, first, 1.0f32, 0.0f32, 0.0f32],
    );
    e.call(NI_NODE_SET_AT, &args![this, 2u32, billboard]);
    for (counter, field) in [(BRANCH_LEVEL_COUNT, 0xe8u32), (LEAF_LEVEL_COUNT, 0xec)] {
        let model = e.call(POINTER_GET, &args![t + 0xe4]).u32();
        let count = e.call(counter, &args![model]).u16();
        if count == 0 {
            e.mem.set_u32(t + field, 0);
            continue;
        }
        let count = u32::from(count);
        let bytes: u32 = (u64::from(count) * 4).try_into().unwrap_or(u32::MAX);
        let total = bytes.saturating_add(4);
        let block = e.call(OPERATOR_NEW_ARRAY, &args![total]).u32();
        let array = if block == 0 {
            0
        } else {
            e.mem.set_u32(block, count);
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    block + 4,
                    4u32,
                    count,
                    LEVEL_ELEMENT_CONSTRUCT,
                    LEVEL_ELEMENT_DESTRUCT
                ],
            );
            block + 4
        };
        e.mem.set_u32(t + field, array);
        let mut index: u16 = 0;
        while u32::from(index) < count {
            let slot = e.mem.u32(t + field).wrapping_add(u32::from(index) * 4);
            e.call(POINTER_ASSIGN, &args![slot, 0u32]);
            index = index.wrapping_add(1);
        }
    }
    e.call(POINTER_ASSIGN, &args![t + 0xf0, 0u32]);
    let extra = fn_0066b660(e);
    e.call(ADD_EXTRA_DATA, &args![this, extra]);
    let object = |e: &mut Engine| {
        if havok_object != 0 {
            havok_object
        } else {
            let model = e.call(POINTER_GET, &args![t + 0xe4]).u32();
            e.call(MODEL_HAVOK_OBJECT, &args![model]).u32()
        }
    };
    if object(e) != 0 {
        let block = e.call(NI_ALLOCATE, &args![0x14u32]).u32();
        let collision = if block == 0 {
            0
        } else {
            e.call(COLLISION_OBJECT_CONSTRUCT, &args![block]).u32()
        };
        let world = object(e);
        e.call(COLLISION_SET_WORLD_OBJECT, &args![collision, world]);
        e.vcall(collision, 0x8c, &args![this]);
    }
    e.set(this, BSTreeNode::fTimer, 0);
    this
}

// Translated from 0066b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::GetBranchNode` (Xbox PDB): `0043b4a0(0)`.
pub fn bs_tree_node_get_branch_node(e: &mut Engine, this: Ptr) -> u32 {
    e.call(NODE_OF_KIND, &args![this, 0u32]).u32()
}

// Translated from 0066b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::GetLeafNode` (Xbox PDB): `0043b4a0(1)`.
pub fn bs_tree_node_get_leaf_node(e: &mut Engine, this: Ptr) -> u32 {
    e.call(NODE_OF_KIND, &args![this, 1u32]).u32()
}

// Translated from 0066b600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::GetBillboardNode` (Xbox PDB): `0043b4a0(2)`.
pub fn bs_tree_node_get_billboard_node(e: &mut Engine, this: Ptr) -> u32 {
    e.call(NODE_OF_KIND, &args![this, 2u32]).u32()
}

// Translated from 0066b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): the object the tree
/// manager (`00664840(1)`) holds in its `NiPointer` at `+0x10`.
pub fn fn_0066b660(e: &mut Engine) -> u32 {
    let manager = e.call(TREE_MANAGER, &args![1u32]).u32();
    e.call(POINTER_GET, &args![manager.wrapping_add(0x10)])
        .u32()
}

// Translated from 0066b680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// body (`0066b6c0`), then the sized delete of 0xF8 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bs_tree_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSTreeNode>,
    flags: u32,
) -> Ptr<BSTreeNode> {
    fn_0066b6c0(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0xf8u32]);
    }
    this
}

// Translated from 0066b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066b6c0 - the FS:[0] exception frame (unwind handler 00f00404) is not translated
/// Unnamed in the engine map (`ECX`): the `BSTreeNode` destructor body: the
/// vtable, the branch and leaf level arrays deleted (`00667120(3)`) and
/// cleared, the billboard and model pointers destroyed (`0045cec0`), then
/// the `BSFadeNode` part (`00b4e1e0`).
pub fn fn_0066b6c0(e: &mut Engine, this: Ptr<BSTreeNode>) {
    let t = this.addr();
    e.mem.set_u32(t, TREE_NODE_VTABLE);
    for field in [0xe8u32, 0xec] {
        let array = e.mem.u32(t + field);
        if array != 0 {
            e.call(LEVEL_ARRAY_DELETE, &args![array, 3u32]);
            e.mem.set_u32(t + field, 0);
        }
    }
    e.call(POINTER_DESTRUCT, &args![t + 0xf0]);
    e.call(POINTER_DESTRUCT, &args![t + 0xe4]);
    e.call(FADE_NODE_DESTRUCTOR, &args![this]);
}

/// The shared body of `0066b7d0` and `0066b840`: the level node at `level`
/// of the array at `field`, 0 without a model, an array, or past the
/// model's level count (from `counter`).
fn level_node(e: &mut Engine, this: Ptr<BSTreeNode>, level: u16, field: u32, counter: u32) -> u32 {
    let t = this.addr();
    if e.call(POINTER_GET, &args![t + 0xe4]).u32() == 0 || e.mem.u32(t + field) == 0 {
        return 0;
    }
    let model = e.call(POINTER_GET, &args![t + 0xe4]).u32();
    let count = e.call(counter, &args![model]).u16();
    if level >= count {
        return 0;
    }
    let slot = e.mem.u32(t + field).wrapping_add(u32::from(level) * 4);
    e.call(POINTER_GET, &args![slot]).u32()
}

// Translated from 0066b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::GetBranchLevel` (Xbox PDB; one word, its low half the
/// level): the branch level node, or 0.
pub fn bs_tree_node_get_branch_level(e: &mut Engine, this: Ptr<BSTreeNode>, level: u16) -> u32 {
    level_node(e, this, level, 0xe8, BRANCH_LEVEL_COUNT)
}

// Translated from 0066b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::GetLeafLevel` (Xbox PDB): the leaf level node, or 0.
pub fn bs_tree_node_get_leaf_level(e: &mut Engine, this: Ptr<BSTreeNode>, level: u16) -> u32 {
    level_node(e, this, level, 0xec, LEAF_LEVEL_COUNT)
}

/// The shared body of `0066b8b0` and `0066ba30`: puts `node` at `level` of
/// the array at `field` under the node from vtable `slot`, detaching the
/// one there (vtable `+0xE8`) and attaching it (`+0xDC`, with 1). A local
/// `BSStringT` lives through it (`004037b0` / `004037d0`).
fn add_level(
    e: &mut Engine,
    this: Ptr<BSTreeNode>,
    level: u16,
    node: u32,
    field: u32,
    counter: u32,
    slot: u32,
) -> bool {
    let t = this.addr();
    e.with_stack(8, |e, text| {
        e.call(0x0040_37b0, &args![text]);
        let result = 'body: {
            if e.call(POINTER_GET, &args![t + 0xe4]).u32() == 0
                || e.mem.u32(t + field) == 0
                || node == 0
            {
                break 'body false;
            }
            let model = e.call(POINTER_GET, &args![t + 0xe4]).u32();
            let count = e.call(counter, &args![model]).u16();
            if level >= count {
                break 'body false;
            }
            let parent = e.vcall(t, slot, &[]).u32();
            if parent == 0 {
                break 'body false;
            }
            let element = e.mem.u32(t + field).wrapping_add(u32::from(level) * 4);
            if e.call(POINTER_GET, &args![element]).u32() != 0 {
                let old = e.call(POINTER_GET, &args![element]).u32();
                e.vcall(parent, 0xe8, &args![old]);
            }
            e.vcall(parent, 0xdc, &args![node, 1u32]);
            let element = e.mem.u32(t + field).wrapping_add(u32::from(level) * 4);
            e.call(POINTER_ASSIGN, &args![element, node]);
            true
        };
        e.call(0x0040_37d0, &args![text]);
        result
    })
}

// Translated from 0066b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066b8b0 - the FS:[0] exception frame that destroys the local string during unwinding (handler 00f00428) is not translated
/// `BSTreeNode::AddBranchLevel` (Xbox PDB; two words: the level, its low
/// half used, and the node): the node becomes that branch level under the
/// branch node (vtable `+0x104`); false when anything is missing.
pub fn bs_tree_node_add_branch_level(
    e: &mut Engine,
    this: Ptr<BSTreeNode>,
    level: u16,
    node: u32,
) -> bool {
    add_level(e, this, level, node, 0xe8, BRANCH_LEVEL_COUNT, 0x104)
}

// Translated from 0066ba30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066ba30 - the FS:[0] exception frame that destroys the local string during unwinding is not translated
/// `BSTreeNode::AddLeafLevel` (Xbox PDB): as `AddBranchLevel` with the leaf
/// array and the leaf node (vtable `+0x108`).
pub fn bs_tree_node_add_leaf_level(
    e: &mut Engine,
    this: Ptr<BSTreeNode>,
    level: u16,
    node: u32,
) -> bool {
    add_level(e, this, level, node, 0xec, LEAF_LEVEL_COUNT, 0x108)
}

/// The shared body of `0066bc40`, `0066bc90` and `0066bce0`: the node from
/// vtable `slot` is hidden when `visible` is 0 (`00450f90`); false without
/// the node.
fn set_visibility_for(e: &mut Engine, this: Ptr, visible: u8, slot: u32) -> bool {
    let node = e.vcall(this.addr(), slot, &[]).u32();
    if node == 0 {
        return false;
    }
    e.call(SET_HIDDEN, &args![node, u32::from(visible == 0)]);
    true
}

// Translated from 0066bc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetVisibilityForBranches` (Xbox PDB; one word, its low byte):
/// on the branch node (vtable `+0x104`).
pub fn bs_tree_node_set_visibility_for_branches(e: &mut Engine, this: Ptr, visible: u8) -> bool {
    set_visibility_for(e, this, visible, 0x104)
}

// Translated from 0066bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetVisibilityForLeaves` (Xbox PDB): on the leaf node
/// (vtable `+0x108`).
pub fn bs_tree_node_set_visibility_for_leaves(e: &mut Engine, this: Ptr, visible: u8) -> bool {
    set_visibility_for(e, this, visible, 0x108)
}

// Translated from 0066bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetVisibilityForBillboards` (Xbox PDB): on the billboard
/// node (vtable `+0x10C`).
pub fn bs_tree_node_set_visibility_for_billboards(e: &mut Engine, this: Ptr, visible: u8) -> bool {
    set_visibility_for(e, this, visible, 0x10c)
}

/// The tail `0066bd30`, `0066bdb0` and `0066be30` share on a found node
/// (each fetching it as the exe does, through `fetch`): 0xFF hides it;
/// any other value shows it and is passed to its property 0 (`00a59d30`,
/// then `0094db80`) when it has one.
fn set_visibility_value(e: &mut Engine, value: u8, fetch: &dyn Fn(&mut Engine) -> u32) {
    if value == 0xff {
        let node = fetch(e);
        e.call(SET_HIDDEN, &args![node, 1u32]);
        return;
    }
    let node = fetch(e);
    e.call(SET_HIDDEN, &args![node, 0u32]);
    let node = fetch(e);
    let property = e.call(GET_PROPERTY, &args![node, 0u32]).u32();
    if property != 0 {
        e.call(PROPERTY_SET_0094DB80, &args![property, u32::from(value)]);
    }
}

// Translated from 0066bd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetVisibilityOnBranch` (Xbox PDB; two words: the level, its
/// low half, and the value, its low byte): on the branch level node (vtable
/// `+0x110`); false without one.
pub fn bs_tree_node_set_visibility_on_branch(
    e: &mut Engine,
    this: Ptr,
    level: u16,
    value: u8,
) -> bool {
    let node = e.vcall(this.addr(), 0x110, &args![u32::from(level)]).u32();
    if node == 0 {
        return false;
    }
    set_visibility_value(e, value, &|_| node);
    true
}

// Translated from 0066bdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetVisibilityOnLeaf` (Xbox PDB): on the leaf level node
/// (vtable `+0x114`).
pub fn bs_tree_node_set_visibility_on_leaf(
    e: &mut Engine,
    this: Ptr,
    level: u16,
    value: u8,
) -> bool {
    let node = e.vcall(this.addr(), 0x114, &args![u32::from(level)]).u32();
    if node == 0 {
        return false;
    }
    set_visibility_value(e, value, &|_| node);
    true
}

// Translated from 0066be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetVisibilityOnBillboard` (Xbox PDB; one word, its low
/// byte): on the billboard (`+0xF0`, read through `00559450` each time).
pub fn bs_tree_node_set_visibility_on_billboard(
    e: &mut Engine,
    this: Ptr<BSTreeNode>,
    value: u8,
) -> bool {
    let slot = this.addr() + 0xf0;
    if e.call(POINTER_GET, &args![slot]).u32() == 0 {
        return false;
    }
    set_visibility_value(e, value, &|e| e.call(POINTER_GET, &args![slot]).u32());
    true
}

// Translated from 0066bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::OnVisible` (Xbox PDB; one word: the culler): `NiNode::
/// OnVisible` when `0054ee20` answers true.
pub fn bs_tree_node_on_visible(e: &mut Engine, this: Ptr, culler: u32) {
    if e.call(TREES_ENABLED, &[]).bool() {
        e.call(NI_NODE_ON_VISIBLE, &args![this, culler]);
    }
}

/// The shared body of `0066bf80` and `0066c070`: inside a profiling scope
/// (line `line`), when `0054ee20` answers true and, for a positive time
/// (the first float of `data`), `004b5fc0(this, 00552380())` does too: the
/// tree's update (vtable `+0x144` with the time) and then the `NiNode` pass.
fn update_pass(e: &mut Engine, this: Ptr, data: Ptr, flags: u32, line: u32, pass: u32) {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_ENTER,
            &args![scope, 0x1eu32, 1u32, SOURCE_FILE_NAME, line],
        );
        let run = 'check: {
            if !e.call(TREES_ENABLED, &[]).bool() {
                break 'check false;
            }
            let time = x87_load32(e.mem.u32(data.addr()));
            let zero: f64 = e.global(ZERO);
            if time > zero {
                let argument = e.call(UPDATE_CHECK_ARGUMENT, &[]).u32();
                if !e.call(UPDATE_CHECK, &args![this, argument]).bool() {
                    break 'check false;
                }
            }
            true
        };
        if run {
            let time = x87_quiet_bits(e.mem.u32(data.addr()));
            e.vcall(this.addr(), 0x144, &args![time]);
            e.call(pass, &args![this, data, flags]);
        }
        e.call(SCOPE_LEAVE, &args![scope]);
    });
}

// Translated from 0066bf80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066bf80 - the FS:[0] exception frame that closes the profiling scope during unwinding (handler 00eec338) is not translated
/// `BSTreeNode::UpdateDownwardPass` (Xbox PDB; two words: the update data,
/// the flags): the tree update, then `NiNode::UpdateDownwardPass`.
pub fn bs_tree_node_update_downward_pass(e: &mut Engine, this: Ptr, data: Ptr, flags: u32) {
    update_pass(e, this, data, flags, 0x20d, NI_NODE_UPDATE_DOWNWARD_PASS);
}

// Translated from 0066c070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066c070 - the FS:[0] exception frame that closes the profiling scope during unwinding (handler 00eec338) is not translated
/// `BSTreeNode::UpdateSelectedDownwardPass` (Xbox PDB): the tree update, then
/// `NiNode::UpdateSelectedDownwardPass`.
pub fn bs_tree_node_update_selected_downward_pass(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    flags: u32,
) {
    update_pass(
        e,
        this,
        data,
        flags,
        0x223,
        NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS,
    );
}

// Translated from 0066ca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::GetLinearVelocity` (engine map; one word: the output
/// vector): with the Havok proxy (`00620b80`), `00c89720`, the proxy's
/// linear velocity (`00cac950`) converted into `out` (`00458620(out, v)`),
/// then `00c89720` again.
pub fn bhk_character_proxy_get_linear_velocity(e: &mut Engine, this: Ptr, out: u32) {
    let proxy = e.call(CHARACTER_PROXY_OBJECT, &args![this]).u32();
    if proxy == 0 {
        return;
    }
    e.call(CHARACTER_PROXY_UNMARK_READ, &args![this]);
    let velocity = e
        .call(HK_CHARACTER_PROXY_GET_LINEAR_VELOCITY, &args![proxy])
        .u32();
    e.call(VECTOR_FROM_HAVOK, &args![out, velocity]);
    e.call(CHARACTER_PROXY_UNMARK_READ, &args![this]);
}

// Translated from 0066ca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (one float word: the time; `bool` in `AL`): a
/// staggered timer at `+0xF4`. A zero time: true. An unset timer (0) is set
/// to `time + interval + interval / slots * counter` (the float at
/// `011d5e64`'s value, the integer at `011d5e44`'s, the byte counter
/// `011d5e3c`, which then steps on modulo the slots; a zero slot count is
/// the exe's division fault, a panic here): true. Otherwise, a timer below
/// the time moves to `time + interval`: true; else false. x87 compares as
/// the exe tests them (NaN never equal, never below).
pub fn fn_0066ca50(e: &mut Engine, this: Ptr<BSTreeNode>, time: f32) -> bool {
    let zero: f64 = e.global(ZERO);
    let now = x87_load32(time.to_bits());
    if now == zero {
        return true;
    }
    let timer = x87_load32(e.get(this, BSTreeNode::fTimer));
    if timer == zero {
        let interval = e.call(FLOAT_VALUE_ADDRESS, &args![TIMER_INTERVAL]).u32();
        let base = x87_add(now, x87_m32(e.mem.u32(interval)));
        let interval = e.call(FLOAT_VALUE_ADDRESS, &args![TIMER_INTERVAL]).u32();
        let slots = e.call(INT_VALUE_ADDRESS, &args![TIMER_SLOTS]).u32();
        let share = x87_div(x87_m32(e.mem.u32(interval)), f64::from(e.mem.i32(slots)));
        let counter = e.global::<u8>(TIMER_COUNTER);
        let offset = x87_mul(share, f64::from(i32::from(counter)));
        let value = x87_store32(x87_add(offset, base));
        e.set(this, BSTreeNode::fTimer, value);
        let counter = counter.wrapping_add(1);
        e.set_global(TIMER_COUNTER, counter);
        let slots = e.call(INT_VALUE_ADDRESS, &args![TIMER_SLOTS]).u32();
        let remainder = i32::from(counter) % e.mem.i32(slots);
        e.set_global(TIMER_COUNTER, remainder as u8);
        return true;
    }
    if timer < now {
        let interval = e.call(FLOAT_VALUE_ADDRESS, &args![TIMER_INTERVAL]).u32();
        let value = x87_store32(x87_add(now, x87_m32(e.mem.u32(interval))));
        e.set(this, BSTreeNode::fTimer, value);
        return true;
    }
    false
}

// Translated from 0066d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`): forwards `this` to `00c6c290`.
pub fn fn_0066d410(e: &mut Engine, this: Ptr) {
    e.call(COLLISION_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 0066d730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, not read): answers 0x30.
pub fn fn_0066d730(_e: &mut Engine, _this: Ptr) -> u32 {
    0x30
}

// Translated from 0066d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCapsuleShape::_scalar_deleting_destructor_` (engine map): the
/// destructor (`00caacf0`), then the sized delete of 0x14 bytes when bit 0
/// of `flags` is set. Answers `this`.
pub fn bhk_capsule_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CAPSULE_SHAPE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 0066d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066d7c0 - the FS:[0] exception frame that destroys the local NiPointer during unwinding (handler 00f005c8) is not translated
/// Unnamed in the engine map (`ECX`, no words): from the leaf node (vtable
/// `+0x108`), when there is one: a local `NiPointer` made from `006838b0`'s
/// object; when it holds one, `00653270(01268230, 006fa820(object))` is the
/// answer; the local is destroyed. 0 otherwise.
pub fn fn_0066d7c0(e: &mut Engine, this: Ptr) -> u32 {
    let leaf = e.vcall(this.addr(), 0x108, &[]).u32();
    if leaf == 0 {
        return 0;
    }
    let object = e.call(LEAF_OBJECT_006838B0, &args![leaf]).u32();
    e.with_stack(4, |e, local| {
        e.call(POINTER_CONSTRUCT, &args![local, object]);
        let mut result = 0;
        if e.call(POINTER_GET, &args![local]).u32() != 0 {
            let held = e.call(POINTER_GET, &args![local]).u32();
            let inner = e.call(OBJECT_006FA820, &args![held]).u32();
            result = e.call(CAST_00653270, &args![RTTI_01268230, inner]).u32();
        }
        e.call(POINTER_DESTRUCT, &args![local]);
        result
    })
}

/// The callees of `0066c160` (`thiscall` unless said; named by the engine
/// map where it names them).
mod update_callees {
    /// `0084e3a0` and `00b0d5d0` (no words): the model's file and its name.
    pub const MODEL_FILE: u32 = 0x0084_e3a0;
    pub const FILE_NAME: u32 = 0x00b0_d5d0;
    /// `strstr` (`00ec7750`, `cdecl`, two words) and the `"~CF"` it looks for.
    pub const STRSTR: u32 = 0x00ec_7750;
    pub const FALL_CHANCE_TAG: u32 = 0x0106_6a7c;
    /// The object whose integer value is the default fall chance (`011d5ea0`).
    pub const DEFAULT_CHANCE: u32 = 0x011d_5ea0;
    /// `100.0`, `0.25`, `90.0` and `1.0` (`double`).
    pub const HUNDRED: u32 = 0x0101_7a40;
    pub const QUARTER: u32 = 0x0102_90b0;
    pub const NINETY: u32 = 0x0106_6808;
    pub const ONE: u32 = 0x0101_2070;
    /// The phantom-contact collector: `0062d350` constructs it, `0062d4b0`
    /// (`hkpAllCdBodyPairCollector::~hkpAllCdBodyPairCollector`) destroys it,
    /// `00623150` (on the leaf object, one word) fills it, `00413f40` its list,
    /// `00726070` the list's count, `0062e150` (one word) its entry.
    pub const COLLECTOR_CONSTRUCT: u32 = 0x0062_d350;
    pub const COLLECTOR_DESTRUCT: u32 = 0x0062_d4b0;
    pub const COLLECT: u32 = 0x0062_3150;
    pub const COLLECTOR_LIST: u32 = 0x0041_3f40;
    pub const LIST_COUNT: u32 = 0x0072_6070;
    pub const LIST_ENTRY: u32 = 0x0062_e150;
    /// `004b5950` and `004b59f0` (`cdecl`, one word: the entry's body): the
    /// actor and the reference the body belongs to.
    pub const BODY_ACTOR: u32 = 0x004b_5950;
    pub const BODY_REFERENCE: u32 = 0x004b_59f0;
    /// `bhkCharacterController::FindBSReference` (`cdecl`, two words) and
    /// `MobileObject::GetCharController` (no words).
    pub const FIND_BS_REFERENCE: u32 = 0x00c6_dec0;
    pub const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
    /// `00621c40` (no words): constructs the contact data of the actor branch.
    pub const CONTACT_CONSTRUCT: u32 = 0x0062_1c40;
    /// `005c5420` and `004dfea0` (`cdecl`, no words; ST0): random floats.
    pub const RANDOM_SIGNED: u32 = 0x005c_5420;
    pub const RANDOM_UNIT: u32 = 0x004d_fea0;
    /// `005532a0` (four float words): sets a four-float vector.
    pub const VECTOR4_SET: u32 = 0x0055_32a0;
    /// `0066c930` (this unit, no words) on that vector.
    pub const VECTOR4_FIX: u32 = 0x0066_c930;
    /// `0044ddc0`, `00436aa0` (no words): the entry body's object, its position.
    pub const BODY_OBJECT: u32 = 0x0044_ddc0;
    pub const OBJECT_POSITION: u32 = 0x0043_6aa0;
    /// `004a3c90` (one word: a point): a vector from a point.
    pub const VECTOR_FROM_POINT: u32 = 0x004a_3c90;
    /// `0066c9c0` (this unit, three words) on the contact data.
    pub const CONTACT_SET: u32 = 0x0066_c9c0;
    /// `0043b560` (no words) on the leaf object.
    pub const LEAF_BODY: u32 = 0x0043_b560;
    /// `006240d0` and `006815c0` (no words): two local constructors.
    pub const SOUND_DATA_CONSTRUCT: u32 = 0x0062_40d0;
    pub const POINT_CONSTRUCT: u32 = 0x0068_15c0;
    /// `00457990` (no words; ST0): a vector's length.
    pub const POINT_LENGTH: u32 = 0x0045_7990;
    /// `00458620` (`cdecl`, two words): a point from two vectors; answers it.
    pub const POINT_COMBINE: u32 = 0x0045_8620;
    /// `ImpactMixer::PlayCollisionSound` (`cdecl`, one word).
    pub const PLAY_COLLISION_SOUND: u32 = 0x0083_7550;
    /// `00944460` (`cdecl`, two words): a random integer between them.
    pub const RANDOM_INT: u32 = 0x0094_4460;
    /// `00560dc0` and `00560e50` (no words) on the reference: its linear and
    /// angular velocity points.
    pub const REFERENCE_LINEAR: u32 = 0x0056_0dc0;
    pub const REFERENCE_ANGULAR: u32 = 0x0056_0e50;
    /// `00630100` and `00630b40` (one word: the result) on a vector, and
    /// `004586d0` (no words; ST0) on the result.
    pub const VECTOR_00630100: u32 = 0x0063_0100;
    pub const VECTOR_00630B40: u32 = 0x0063_0b40;
    pub const VECTOR_LENGTH: u32 = 0x0045_86d0;
    /// The objects whose integer values bound the impulse loss (`011d5e58`,
    /// `011d5e94`).
    pub const LOSS_LOW: u32 = 0x011d_5e58;
    pub const LOSS_HIGH: u32 = 0x011d_5e94;
    /// `005dbf20` (one float word) and `00627920` (one word).
    pub const SCALE_CONSTRUCT: u32 = 0x005d_bf20;
    pub const VECTOR_SCALE: u32 = 0x0062_7920;
    /// `hkpRigidBody::setLinearVelocity` / `setAngularVelocity` (engine map; one word).
    pub const SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
    pub const SET_ANGULAR_VELOCITY: u32 = 0x0056_1800;
    /// `004a3a40` (`cdecl`, one word) and `006205a0` (no words): the
    /// material of the reference's object.
    pub const OBJECT_MATERIAL_SOURCE: u32 = 0x004a_3a40;
    pub const MATERIAL: u32 = 0x0062_05a0;
    /// `004587d0` (`cdecl`, one float word; ST0): a scaled length.
    pub const SCALE_LENGTH: u32 = 0x0045_87d0;
    /// `004b4f00` (no words) on the reference, and `004b5a80` (`cdecl`, one word).
    pub const REFERENCE_OBJECT: u32 = 0x004b_4f00;
    pub const REFERENCE_SOUND: u32 = 0x004b_5a80;
}

// Translated from 0066c160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066c160 - the FS:[0] exception frame (handler 00f0045b, destroying the contact collector) and the /GS cookie check (00ec408c) are not translated
/// `BSTreeNode::UpdateBSTreeNode` (Xbox PDB; one float word: the time):
/// the model's update (vtable `+8` with the node); when the timer fires
/// (`0066ca50`) and the leaf object (`0066d7c0`) has a body (vtable `+0x94`):
/// the fall chance (the digit after `"~CF"` in the model's file name, else
/// the default setting) times the interval setting times 100 is the
/// threshold; the bodies touching the leaves are collected (`0062d350`,
/// `00623150`) and for each: an actor's character controller gets a random
/// contact and a collision sound plays; else a reference whose roll
/// (`00944460(0, 100)`) is within the threshold has its linear and angular
/// velocities (above 1) scaled down by random amounts and a collision sound
/// plays. The frame's locals sit at the exe's `EBP` offsets (one block).
/// Each x87 instruction is one helper call; calls whose argument the
/// compiler evaluated first (`005c5420`, `004dfea0`) take no words.
pub fn bs_tree_node_update_bs_tree_node(e: &mut Engine, this: Ptr<BSTreeNode>, time: f32) {
    use update_callees::*;
    let t = this.addr();
    if e.call(POINTER_GET, &args![t + 0xe4]).u32() != 0 {
        let model = e.call(POINTER_GET, &args![t + 0xe4]).u32();
        e.vcall(model, 8, &args![this]);
    }
    let time = f32::from_bits(x87_quiet_bits(time.to_bits()));
    if !fn_0066ca50(e, this, time) {
        return;
    }
    let leaf = fn_0066d7c0(e, this.cast());
    if leaf == 0 || e.vcall(leaf, 0x94, &[]).u32() == 0 {
        return;
    }
    let model = e.call(POINTER_GET, &args![t + 0xe4]).u32();
    let file = e.call(MODEL_FILE, &args![model]).u32();
    let name = e.call(FILE_NAME, &args![file]).u32();
    let tag = e.call(STRSTR, &args![name, FALL_CHANCE_TAG]).u32();
    let chance = if tag != 0 {
        i32::from(e.mem.i8(tag + 3)) - 0x30
    } else {
        let value = e.call(INT_VALUE_ADDRESS, &args![DEFAULT_CHANCE]).u32();
        e.mem.i32(value)
    };
    let chance = f64::from(chance);
    let interval = e.call(FLOAT_VALUE_ADDRESS, &args![TIMER_INTERVAL]).u32();
    let hundred: f64 = e.global(HUNDRED);
    let threshold = x87_store32(x87_mul(
        x87_mul(x87_load32(e.mem.u32(interval)), chance),
        hundred,
    ));
    e.with_stack(0x380, |e, frame| {
        let ebp = frame.addr() + 0x380;
        let local = |offset: u32| ebp - offset;
        let collector = local(0x150);
        e.call(COLLECTOR_CONSTRUCT, &args![collector]);
        e.call(COLLECT, &args![leaf, collector]);
        let list = e.call(COLLECTOR_LIST, &args![collector]).u32();
        let mut index: i32 = 0;
        while index < e.call(LIST_COUNT, &args![list]).i32() {
            let entry = e.call(LIST_ENTRY, &args![list, index]).u32();
            let body = e.mem.u32(entry + 8);
            let actor = e.call(BODY_ACTOR, &args![body]).u32();
            let entry = e.call(LIST_ENTRY, &args![list, index]).u32();
            let body = e.mem.u32(entry + 8);
            let reference = e.call(BODY_REFERENCE, &args![body]).u32();
            if actor != 0 {
                update_actor_contact(e, ebp, leaf, list, index, actor);
            } else if reference != 0 {
                let roll = e.call(RANDOM_INT, &args![0u32, 100u32]).i32();
                // FCOMPP of the threshold against the roll: on below
                // (or unordered) nothing happens.
                if x87_load32(threshold) >= f64::from(roll) {
                    update_falling_reference(e, ebp, leaf, list, index, reference);
                }
            }
            index += 1;
        }
        e.call(COLLECTOR_DESTRUCT, &args![collector]);
    });
}

/// The four floats of `005532a0` from three `stored` floats and 0 (`FLD` /
/// `FSTP` copies).
fn vector4_set(e: &mut Engine, vector: u32, x: u32, y: u32, z: u32) {
    use update_callees::*;
    let (x, y, z) = (x87_quiet_bits(x), x87_quiet_bits(y), x87_quiet_bits(z));
    e.call(VECTOR4_SET, &args![vector, x, y, z, 0.0f32]);
}

/// The actor branch of `0066c160` (its locals at the exe's frame offsets
/// below `ebp`).
fn update_actor_contact(e: &mut Engine, ebp: u32, leaf: u32, list: u32, index: i32, actor: u32) {
    use update_callees::*;
    let local = |offset: u32| ebp - offset;
    let reference = e.call(FIND_BS_REFERENCE, &args![1000u32, actor]).u32();
    if reference == 0 {
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![reference]).u32();
    if controller == 0 {
        return;
    }
    let contact = local(0x1e0);
    e.call(CONTACT_CONSTRUCT, &args![contact]);
    let first = x87_store32(e.call(RANDOM_SIGNED, &[]).f64());
    let second = x87_store32(e.call(RANDOM_SIGNED, &[]).f64());
    let third = x87_store32(e.call(RANDOM_SIGNED, &[]).f64());
    let direction = local(0x1f0);
    e.call(VECTOR4_SET, &args![direction, third, second, first, 0.0f32]);
    e.call(VECTOR4_FIX, &args![direction]);
    let entry = e.call(LIST_ENTRY, &args![list, index]).u32();
    let object = e.call(BODY_OBJECT, &args![e.mem.u32(entry + 8)]).u32();
    let position = e.call(OBJECT_POSITION, &args![object]).u32();
    let point = local(0x200);
    e.call(VECTOR_FROM_POINT, &args![point, position]);
    e.call(CONTACT_SET, &args![contact, point, direction, 0.0f32]);
    let leaf_body = e.call(LEAF_BODY, &args![leaf]).u32();
    e.mem.set_u32(local(0x1c0), leaf_body);
    let entry = e.call(LIST_ENTRY, &args![list, index]).u32();
    let body = e.mem.u32(entry + 8);
    e.mem.set_u32(local(0x198), body);
    e.mem.set_u32(local(0x194), 0);
    e.mem.set_u32(local(0x1bc), 0);
    e.vcall(controller + 0x410, 8, &args![0u32, contact]);
    let sound = local(0x224);
    e.call(SOUND_DATA_CONSTRUCT, &args![sound]);
    let velocity = local(0x230);
    e.call(POINT_CONSTRUCT, &args![velocity]);
    e.mem.set_u8(local(0x210), 4);
    e.mem.set_u8(local(0x20f), 7);
    e.mem.set_u32(local(0x214), 1.0f32.to_bits());
    bhk_character_proxy_get_linear_velocity(e, Ptr::new(controller), velocity);
    let length = x87_store32(e.call(POINT_LENGTH, &args![velocity]).f64());
    e.mem.set_u32(local(0x218), length);
    e.mem.set_u32(local(0x20c), actor.wrapping_add(leaf));
    let combined = e.call(POINT_COMBINE, &args![velocity, point]).u32();
    for i in 0..3 {
        let word = e.mem.u32(combined + 4 * i);
        e.mem.set_u32(sound + 4 * i, word);
    }
    e.mem.set_u32(local(0x208), 0);
    e.mem.set_u32(local(0x204), 0);
    e.call(PLAY_COLLISION_SOUND, &args![sound]);
}

/// One velocity of the reference branch of `0066c160`: when the length of
/// `00630100` of `vector` is above 1, a random direction (`004dfea0` three
/// times, the last minus 0.25) scaled by `scale(length)` goes to `apply`.
fn scale_reference_velocity(
    e: &mut Engine,
    ebp: u32,
    vector: u32,
    offsets: [u32; 4],
    scale: &dyn Fn(&mut Engine, f64) -> u32,
    reference: u32,
    apply: u32,
) {
    use update_callees::*;
    let local = |offset: u32| ebp - offset;
    let [first_out, second_out, direction, factor] = offsets;
    let result = e
        .call(VECTOR_00630100, &args![vector, local(first_out)])
        .u32();
    let one: f64 = e.global(ONE);
    let length = e.call(VECTOR_LENGTH, &args![result]).f64();
    // FCOMP with 1.0, `TEST AH,0x41`: only an ordered "above" goes on.
    if !(length > one) {
        return;
    }
    let result = e
        .call(VECTOR_00630B40, &args![vector, local(second_out)])
        .u32();
    let length = e.call(VECTOR_LENGTH, &args![result]).f64();
    let amount = scale(e, length);
    let x = x87_store32(e.call(RANDOM_UNIT, &[]).f64());
    let y = x87_store32(e.call(RANDOM_UNIT, &[]).f64());
    let quarter: f64 = e.global(QUARTER);
    let z = x87_store32(x87_sub(e.call(RANDOM_UNIT, &[]).f64(), quarter));
    vector4_set(e, local(direction), x, y, z);
    e.call(VECTOR4_FIX, &args![local(direction)]);
    let amount = x87_quiet_bits(amount);
    e.call(SCALE_CONSTRUCT, &args![local(factor), amount]);
    e.call(VECTOR_SCALE, &args![local(direction), local(factor)]);
    e.call(apply, &args![reference, local(direction)]);
}

/// The reference branch of `0066c160`.
fn update_falling_reference(
    e: &mut Engine,
    ebp: u32,
    leaf: u32,
    list: u32,
    index: i32,
    reference: u32,
) {
    use update_callees::*;
    let local = |offset: u32| ebp - offset;
    let linear = local(0x250);
    let angular = local(0x260);
    let source = e.call(REFERENCE_LINEAR, &args![reference]).u32();
    e.call(VECTOR_FROM_POINT, &args![linear, source]);
    let source = e.call(REFERENCE_ANGULAR, &args![reference]).u32();
    e.call(VECTOR_FROM_POINT, &args![angular, source]);
    let hundred: f64 = e.global(HUNDRED);
    let linear_scale = move |e: &mut Engine, length: f64| -> u32 {
        let high = e.call(INT_VALUE_ADDRESS, &args![LOSS_HIGH]).u32();
        let high = e.mem.u32(high);
        let low = e.call(INT_VALUE_ADDRESS, &args![LOSS_LOW]).u32();
        let low = e.mem.u32(low);
        let roll = e.call(RANDOM_INT, &args![low, high]).i32();
        let loss = x87_div(f64::from(roll), hundred);
        x87_store32(x87_mul(x87_sub(1.0, loss), length))
    };
    scale_reference_velocity(
        e,
        ebp,
        linear,
        [0x270, 0x280, 0x2b0, 0x2c0],
        &linear_scale,
        reference,
        SET_LINEAR_VELOCITY,
    );
    let ninety: f64 = e.global(NINETY);
    let angular_scale = move |e: &mut Engine, length: f64| -> u32 {
        let roll = e.call(RANDOM_INT, &args![0u32, 0x14u32]).i32();
        let gain = x87_add(x87_div(f64::from(roll), hundred), ninety);
        x87_store32(x87_mul(gain, length))
    };
    scale_reference_velocity(
        e,
        ebp,
        angular,
        [0x2d0, 0x2e0, 0x310, 0x320],
        &angular_scale,
        reference,
        SET_ANGULAR_VELOCITY,
    );
    let sound = local(0x344);
    e.call(SOUND_DATA_CONSTRUCT, &args![sound]);
    let velocity = local(0x350);
    e.call(POINT_CONSTRUCT, &args![velocity]);
    let entry = e.call(LIST_ENTRY, &args![list, index]).u32();
    let held = e.call(POINTER_GET, &args![e.mem.u32(entry + 8)]).u32();
    let source = e.call(OBJECT_MATERIAL_SOURCE, &args![held]).u32();
    e.mem.set_u8(local(0x330), 4);
    let material = if source != 0 {
        e.call(MATERIAL, &args![source]).u32()
    } else {
        0
    };
    e.mem.set_u32(local(0x358), material);
    e.mem.set_u8(local(0x32f), material as u8);
    e.mem.set_u32(local(0x334), 1.0f32.to_bits());
    let result = e.call(VECTOR_00630B40, &args![linear, local(0x370)]).u32();
    let length = x87_store32(e.call(VECTOR_LENGTH, &args![result]).f64());
    let scaled = e.call(SCALE_LENGTH, &args![length]).f64();
    e.mem.set_u32(local(0x338), x87_store32(scaled));
    e.mem.set_u32(local(0x32c), reference.wrapping_add(leaf));
    let object = e.call(REFERENCE_OBJECT, &args![reference]).u32();
    let position = e.call(OBJECT_POSITION, &args![object]).u32();
    let combined = e.call(POINT_COMBINE, &args![velocity, position]).u32();
    for i in 0..3 {
        let word = e.mem.u32(combined + 4 * i);
        e.mem.set_u32(sound + 4 * i, word);
    }
    let sound_kind = e.call(REFERENCE_SOUND, &args![reference]).u32();
    e.mem.set_u32(local(0x328), sound_kind);
    e.mem.set_u32(local(0x324), 0);
    e.call(PLAY_COLLISION_SOUND, &args![sound]);
}

/// The callees of `0066cb40` (`thiscall` unless said; named by the engine
/// map where it names them; the `0066d...` ones are this unit's, not yet
/// translated).
mod havok_leaf_callees {
    /// `0050e620` (one word) on the object at `011d5e84`: whether Havok
    /// leaves are made.
    pub const SETTING_ENABLED: u32 = 0x0050_e620;
    pub const HAVOK_LEAVES_SETTING: u32 = 0x011d_5e84;
    /// `00b0a260` (no words): the SpeedTree's collision object count;
    /// `CSpeedTreeRT::GetCollisionObject` (four words: the index, the type,
    /// the position, the dimensions).
    pub const COLLISION_OBJECT_COUNT: u32 = 0x00b0_a260;
    pub const GET_COLLISION_OBJECT: u32 = 0x00b0_a2a0;
    /// `0066b0b0` (`cdecl`, no words; ST0): the tree scale setting.
    pub const TREE_SCALE: u32 = 0x0066_b0b0;
    /// `004a3e90` (`cdecl`, one float word; ST0): a unit conversion.
    pub const TO_HAVOK_UNITS: u32 = 0x004a_3e90;
    /// `0056e090` / `0056ea90` (no words): the shape-phantom data on the
    /// frame; `00622290` (three words) another frame object.
    pub const PHANTOM_DATA_CONSTRUCT: u32 = 0x0056_e090;
    pub const PHANTOM_DATA_DESTRUCT: u32 = 0x0056_ea90;
    pub const FILTER_CONSTRUCT: u32 = 0x0062_2290;
    /// `hkTransformShapeCinfo::hkTransformShapeCinfo` (no words) and the
    /// capsule's counterpart `0066d650`.
    pub const TRANSFORM_CINFO_CONSTRUCT: u32 = 0x0066_d550;
    pub const CAPSULE_CINFO_CONSTRUCT: u32 = 0x0066_d650;
    /// `004b4db0` (one word) and `004b4ff0` (four float words).
    pub const CINFO_SET_TRANSLATION: u32 = 0x004b_4db0;
    pub const VECTOR4_SET_004B4FF0: u32 = 0x004b_4ff0;
    /// The shape constructors (`bhkSphereShape` two words, the others one).
    pub const SPHERE_SHAPE_CONSTRUCT: u32 = 0x0056_e950;
    pub const TRANSFORM_SHAPE_CONSTRUCT: u32 = 0x0066_d590;
    pub const CAPSULE_SHAPE_CONSTRUCT: u32 = 0x0066_d6a0;
    pub const BOX_SHAPE_CONSTRUCT: u32 = 0x0056_e610;
    pub const MULTI_SPHERE_SHAPE_CONSTRUCT: u32 = 0x0066_d490;
    /// `bhkCharacterProxy::operatorP` (engine map; no words): the Havok
    /// object of a wrapper.
    pub const HAVOK_OBJECT: u32 = 0x004a_e750;
    /// The sphere list: `0066d770` / `0066d7a0` (no words), `0066d9b0` (one word).
    pub const SPHERE_LIST_CONSTRUCT: u32 = 0x0066_d770;
    pub const SPHERE_LIST_DESTRUCT: u32 = 0x0066_d7a0;
    pub const SPHERE_LIST_ADD: u32 = 0x0066_d9b0;
    /// `005b5e40` (`cdecl`, one word) and its message (`01066a80`).
    pub const REPORT: u32 = 0x005b_5e40;
    pub const NOT_ALL_SPHERES: u32 = 0x0106_6a80;
    /// `bhkSimpleShapePhantom::bhkSimpleShapePhantom` (one word) and the
    /// collision object constructor `0066d3a0` (one word).
    pub const PHANTOM_CONSTRUCT: u32 = 0x0056_e2d0;
    pub const LEAF_COLLISION_CONSTRUCT: u32 = 0x0066_d3a0;
    /// `0.5` (`double`).
    pub const HALF: u32 = 0x0101_1588;
    /// `005532a0` (four float words): sets a four-float vector.
    pub use super::update_callees::VECTOR4_SET;
}

/// `size` bytes from the Ni allocator constructed by `construct` (with
/// `words` after the block), or 0 when the allocation fails.
fn allocate_and_construct(e: &mut Engine, size: u32, construct: u32, words: &[u32]) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut arguments = vec![block];
    arguments.extend_from_slice(words);
    e.call(construct, &arguments).u32()
}

/// The float at `addr` converted (`004a3e90` on its `FLD`/`FSTP` copy) and
/// scaled by `factor` (`FMUL float`), stored back.
fn to_havok_scaled(e: &mut Engine, addr: u32, factor: u32) {
    use havok_leaf_callees::*;
    let value = x87_quiet_bits(e.mem.u32(addr));
    let converted = e.call(TO_HAVOK_UNITS, &args![value]).f64();
    let scaled = x87_store32(x87_mul(converted, x87_m32(factor)));
    e.mem.set_u32(addr, scaled);
}

/// The single collision object of `0066cb40` (type 0 sphere, 1 capsule, 2
/// box), its shape's Havok object stored at `shape_slot`.
fn single_leaf_shape(e: &mut Engine, ebp: u32, shape_slot: u32) {
    use havok_leaf_callees::*;
    let local = |offset: u32| ebp - offset;
    let word = |e: &Engine, offset: u32| e.mem.u32(local(offset));
    let quiet = |e: &Engine, offset: u32| x87_quiet_bits(e.mem.u32(local(offset)));
    match e.mem.u32(local(0xac)) {
        0 => {
            let cinfo = local(0x100);
            e.call(TRANSFORM_CINFO_CONSTRUCT, &args![cinfo]);
            let translation = local(0x110);
            let (x, y, z) = (quiet(e, 0xa8), quiet(e, 0xa4), quiet(e, 0xa0));
            e.call(VECTOR4_SET, &args![translation, x, y, z, 0.0f32]);
            e.call(CINFO_SET_TRANSLATION, &args![local(0xc0), translation]);
            let radius = quiet(e, 0x9c);
            let sphere = allocate_and_construct(e, 0x14, SPHERE_SHAPE_CONSTRUCT, &[radius, 0]);
            let held = e.call(HAVOK_OBJECT, &args![sphere]).u32();
            e.mem.set_u32(local(0xfc), held);
            let shape = allocate_and_construct(e, 0x14, TRANSFORM_SHAPE_CONSTRUCT, &[cinfo]);
            let held = e.call(HAVOK_OBJECT, &args![shape]).u32();
            e.mem.set_u32(shape_slot, held);
        }
        1 => {
            let cinfo = local(0x160);
            e.call(CAPSULE_CINFO_CONSTRUCT, &args![cinfo]);
            let top = x87_store32(x87_add(x87_load32(word(e, 0xa0)), x87_m32(word(e, 0x9c))));
            e.mem.set_u32(local(0x164), top);
            let (x, y) = (quiet(e, 0xa8), quiet(e, 0xa4));
            let top = quiet(e, 0x164);
            e.call(
                VECTOR4_SET_004B4FF0,
                &args![local(0x150), x, y, top, 0.0f32],
            );
            let bottom = x87_store32(x87_sub(
                x87_add(x87_load32(word(e, 0xa0)), x87_m32(word(e, 0x98))),
                x87_m32(word(e, 0x9c)),
            ));
            e.mem.set_u32(local(0x168), bottom);
            let (x, y) = (quiet(e, 0xa8), quiet(e, 0xa4));
            let bottom = quiet(e, 0x168);
            e.call(
                VECTOR4_SET_004B4FF0,
                &args![local(0x140), x, y, bottom, 0.0f32],
            );
            let radius = quiet(e, 0x9c);
            e.mem.set_u32(local(0x15c), radius);
            let shape = allocate_and_construct(e, 0x14, CAPSULE_SHAPE_CONSTRUCT, &[cinfo]);
            let held = e.call(HAVOK_OBJECT, &args![shape]).u32();
            e.mem.set_u32(shape_slot, held);
        }
        2 => {
            let cinfo = local(0x1d0);
            e.call(TRANSFORM_CINFO_CONSTRUCT, &args![cinfo]);
            let half: f64 = e.global(HALF);
            for k in 0..3 {
                let at = local(0x9c) + 4 * k;
                let value = x87_store32(x87_mul(x87_load32(e.mem.u32(at)), half));
                e.mem.set_u32(at, value);
            }
            let centre = x87_store32(x87_add(x87_load32(word(e, 0xa0)), x87_m32(word(e, 0x94))));
            e.mem.set_u32(local(0x1d8), centre);
            let translation = local(0x1f0);
            let (x, y, z) = (quiet(e, 0xa8), quiet(e, 0xa4), quiet(e, 0x1d8));
            e.call(VECTOR4_SET, &args![translation, x, y, z, 0.0f32]);
            let extents = local(0x200);
            let (a, b, c) = (quiet(e, 0x9c), quiet(e, 0x98), quiet(e, 0x94));
            e.call(VECTOR4_SET, &args![extents, a, b, c, 0.0f32]);
            e.call(CINFO_SET_TRANSLATION, &args![local(0x190), translation]);
            let boxed = allocate_and_construct(e, 0x14, BOX_SHAPE_CONSTRUCT, &[extents]);
            let held = e.call(HAVOK_OBJECT, &args![boxed]).u32();
            e.mem.set_u32(local(0x1cc), held);
            let shape = allocate_and_construct(e, 0x14, TRANSFORM_SHAPE_CONSTRUCT, &[cinfo]);
            let held = e.call(HAVOK_OBJECT, &args![shape]).u32();
            e.mem.set_u32(shape_slot, held);
        }
        _ => {}
    }
}

// Translated from 0066cb40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066cb40 - the FS:[0] exception frame (handler 00f00536, states 0 to 9) and the /GS cookie check (00ec408c) are not translated
/// `BSTreeNode::MakeHavokLeaves` (Xbox PDB; two words: the SpeedTree, the
/// scale as a float): when the setting at `011d5e84` allows it and the tree
/// has collision objects, their position and dimensions are converted
/// (`004a3e90`) and scaled by the tree scale (`0066b0b0`) times `scale`. One
/// object becomes a sphere (in a transform shape), a capsule or a box (in a
/// transform shape, its dimensions halved) by its type; several become a
/// multi-sphere shape (each must be a sphere, else `005b5e40` reports it).
/// The shape goes into a simple shape phantom (0x18 bytes) set as the world
/// object of a new collision object (0x14 bytes, `0066d3a0` on the leaf
/// node). The frame's locals sit at the exe's `EBP` offsets (one block).
pub fn bs_tree_node_make_havok_leaves(e: &mut Engine, this: Ptr, tree: u32, scale: f32) {
    use havok_leaf_callees::*;
    if !e
        .call(SETTING_ENABLED, &args![HAVOK_LEAVES_SETTING, 1u32])
        .bool()
    {
        return;
    }
    let count = e.call(COLLISION_OBJECT_COUNT, &args![tree]).u32();
    if count == 0 {
        return;
    }
    let tree_scale = e.call(TREE_SCALE, &[]).f64();
    let factor = x87_store32(x87_mul(tree_scale, x87_m32(scale.to_bits())));
    let leaf = e.vcall(this.addr(), 0x108, &[]).u32();
    e.with_stack(0x2a0, |e, frame| {
        let ebp = frame.addr() + 0x2a0;
        let local = |offset: u32| ebp - offset;
        let phantom_data = local(0x90);
        e.call(PHANTOM_DATA_CONSTRUCT, &args![phantom_data]);
        let shape_slot = local(0x8c);
        if count == 1 {
            let (kind, position, size) = (local(0xac), local(0xa8), local(0x9c));
            e.call(
                GET_COLLISION_OBJECT,
                &args![tree, 0u32, kind, position, size],
            );
            for offset in [0xa8u32, 0xa4, 0xa0, 0x9c, 0x98, 0x94] {
                to_havok_scaled(e, local(offset), factor);
            }
            single_leaf_shape(e, ebp, shape_slot);
        } else {
            let list = local(0x230);
            e.call(SPHERE_LIST_CONSTRUCT, &args![list]);
            let (kind, position, size) = (local(0x250), local(0x24c), local(0x240));
            let mut index = 0u32;
            while index < count {
                e.call(
                    GET_COLLISION_OBJECT,
                    &args![tree, index, kind, position, size],
                );
                if e.mem.u32(kind) == 0 {
                    let offsets = [0x24cu32, 0x248, 0x244, 0x240];
                    for offset in offsets {
                        to_havok_scaled(e, local(offset), factor);
                    }
                    let [a, b, c, d] =
                        offsets.map(|offset| x87_quiet_bits(e.mem.u32(local(offset))));
                    let sphere = local(0x260);
                    e.call(VECTOR4_SET, &args![sphere, a, b, c, d]);
                    e.call(SPHERE_LIST_ADD, &args![local(0x22c), sphere]);
                } else {
                    e.call(REPORT, &args![NOT_ALL_SPHERES]);
                }
                index += 1;
            }
            let shape = allocate_and_construct(e, 0x14, MULTI_SPHERE_SHAPE_CONSTRUCT, &[list]);
            let held = e.call(HAVOK_OBJECT, &args![shape]).u32();
            e.mem.set_u32(shape_slot, held);
            e.call(SPHERE_LIST_DESTRUCT, &args![list]);
        }
        let filter = local(0x274);
        e.call(FILTER_CONSTRUCT, &args![filter, 9u32, 4u32, 0u32]);
        let value = e.call(POINTER_GET, &args![filter]).u32();
        e.mem.set_u32(phantom_data, value);
        let phantom = allocate_and_construct(e, 0x18, PHANTOM_CONSTRUCT, &[phantom_data]);
        let collision = allocate_and_construct(e, 0x14, LEAF_COLLISION_CONSTRUCT, &[leaf]);
        e.call(COLLISION_SET_WORLD_OBJECT, &args![collision, phantom]);
        e.call(PHANTOM_DATA_DESTRUCT, &args![phantom_data]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0066_c160,
            bs_tree_node_update_bs_tree_node(Ptr<BSTreeNode>, f32)
        ),
        entry!(0x0066_cb40, bs_tree_node_make_havok_leaves(Ptr, u32, f32)),
        entry!(
            0x0066_b120,
            bs_tree_node_bs_tree_node(Ptr<BSTreeNode>, u32, u32) -> Ptr<BSTreeNode>
        ),
        entry!(0x0066_b5c0, bs_tree_node_get_branch_node(Ptr) -> u32),
        entry!(0x0066_b5e0, bs_tree_node_get_leaf_node(Ptr) -> u32),
        entry!(0x0066_b600, bs_tree_node_get_billboard_node(Ptr) -> u32),
        entry!(0x0066_b660, fn_0066b660() -> u32),
        entry!(
            0x0066_b680,
            bs_tree_node_scalar_deleting_destructor(Ptr<BSTreeNode>, u32) -> Ptr<BSTreeNode>
        ),
        entry!(0x0066_b6c0, fn_0066b6c0(Ptr<BSTreeNode>)),
        entry!(
            0x0066_b7d0,
            bs_tree_node_get_branch_level(Ptr<BSTreeNode>, u16) -> u32
        ),
        entry!(
            0x0066_b840,
            bs_tree_node_get_leaf_level(Ptr<BSTreeNode>, u16) -> u32
        ),
        entry!(
            0x0066_b8b0,
            bs_tree_node_add_branch_level(Ptr<BSTreeNode>, u16, u32) -> bool
        ),
        entry!(
            0x0066_ba30,
            bs_tree_node_add_leaf_level(Ptr<BSTreeNode>, u16, u32) -> bool
        ),
        entry!(0x0066_bc40, bs_tree_node_set_visibility_for_branches(Ptr, u8) -> bool),
        entry!(0x0066_bc90, bs_tree_node_set_visibility_for_leaves(Ptr, u8) -> bool),
        entry!(0x0066_bce0, bs_tree_node_set_visibility_for_billboards(Ptr, u8) -> bool),
        entry!(0x0066_bd30, bs_tree_node_set_visibility_on_branch(Ptr, u16, u8) -> bool),
        entry!(0x0066_bdb0, bs_tree_node_set_visibility_on_leaf(Ptr, u16, u8) -> bool),
        entry!(
            0x0066_be30,
            bs_tree_node_set_visibility_on_billboard(Ptr<BSTreeNode>, u8) -> bool
        ),
        entry!(0x0066_bf50, bs_tree_node_on_visible(Ptr, u32)),
        entry!(
            0x0066_bf80,
            bs_tree_node_update_downward_pass(Ptr, Ptr, u32)
        ),
        entry!(
            0x0066_c070,
            bs_tree_node_update_selected_downward_pass(Ptr, Ptr, u32)
        ),
        entry!(
            0x0066_ca00,
            bhk_character_proxy_get_linear_velocity(Ptr, u32)
        ),
        entry!(0x0066_ca50, fn_0066ca50(Ptr<BSTreeNode>, f32) -> bool),
        entry!(0x0066_d410, fn_0066d410(Ptr)),
        entry!(0x0066_d730, fn_0066d730(Ptr) -> u32),
        entry!(
            0x0066_d740,
            bhk_capsule_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0066_d7c0, fn_0066d7c0(Ptr) -> u32),
        entry!(0x0066_b5b0, fn_0066b5b0(Ptr) -> u32),
        entry!(0x0066_d400, fn_0066d400(Ptr) -> u32),
        entry!(0x0066_d430, fn_0066d430(Ptr) -> u32),
        entry!(0x0066_d510, fn_0066d510(Ptr) -> u32),
        entry!(0x0066_d610, fn_0066d610(Ptr) -> u32),
        entry!(0x0066_d720, fn_0066d720(Ptr) -> u32),
        entry!(0x0066_d470, fn_0066d470(Ptr)),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0066d470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0066_d410, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_d470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0066_d410), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0066b5b0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_b5b0, &args![0x1000u32]).u32(), 0x011d_5e70);
        assert_eq!(e.call(0x0066_b5b0, &args![0u32]).u32(), 0x011d_5e70);
    }

    #[test]
    fn fn_0066d400_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d400, &args![0x1000u32]).u32(), 0x0126_8168);
        assert_eq!(e.call(0x0066_d400, &args![0u32]).u32(), 0x0126_8168);
    }

    #[test]
    fn fn_0066d430_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d430, &args![0x1000u32]).u32(), 0x0126_83d0);
        assert_eq!(e.call(0x0066_d430, &args![0u32]).u32(), 0x0126_83d0);
    }

    #[test]
    fn fn_0066d510_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d510, &args![0x1000u32]).u32(), 0x0126_83e0);
        assert_eq!(e.call(0x0066_d510, &args![0u32]).u32(), 0x0126_83e0);
    }

    #[test]
    fn fn_0066d610_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d610, &args![0x1000u32]).u32(), 0x0126_81b4);
        assert_eq!(e.call(0x0066_d610, &args![0u32]).u32(), 0x0126_81b4);
    }

    #[test]
    fn fn_0066d720_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d720, &args![0x1000u32]).u32(), 0x0126_832c);
        assert_eq!(e.call(0x0066_d720, &args![0u32]).u32(), 0x0126_832c);
    }
}

#[cfg(test)]
mod tree_node_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// `00559450` as the plain read of the slot it is given.
    fn pointer_get_reads(e: &mut Engine) {
        e.register(POINTER_GET, |e, a| word(e.mem.u32(a[0])));
    }

    /// An object whose vtable has `slots` (offset, target).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn constructor_builds_the_children_and_level_arrays() {
        let mut e = Engine::new();
        pointer_get_reads(&mut e);
        for callee in [
            FADE_NODE_CONSTRUCT,
            POINTER_CONSTRUCT,
            FADE_NODE_SET_LOD_MULT_TYPE,
            NODE_SET_FLAG_004902F0,
            NI_NODE_SET_AT,
            BILLBOARD_NODE_SET_0051D000,
            BILLBOARD_NODE_SET_006404B0,
            VECTOR_CONSTRUCTOR_ITERATOR,
            ADD_EXTRA_DATA,
            COLLISION_SET_WORLD_OBJECT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_NEW_ARRAY, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NODE_CONSTRUCT, |_, a| word(a[0]));
        e.register(BILLBOARD_NODE_CONSTRUCT, |_, a| word(a[0]));
        e.register(BRANCH_LEVEL_COUNT, |_, _| word(2));
        e.register(LEAF_LEVEL_COUNT, |_, _| word(0));
        e.register(TREE_MANAGER, |_, _| word(0x9000));
        e.map(0x9000, 0x20);
        e.mem.set_u32(0x9010, 0x4444);
        e.map(MINUS_HALF_PI, 4);
        e.mem.set_f32(MINUS_HALF_PI, -std::f32::consts::FRAC_PI_2);
        const ATTACH: u32 = 0x0100_008c;
        e.register(ATTACH, |_, _| Ret::default());
        let collision = object(&mut e, &[(0x8c, ATTACH)]);
        e.register_double(COLLISION_OBJECT_CONSTRUCT, move |_, _| word(collision));
        let this: Ptr<BSTreeNode> = e.new_object();
        e.mem.set_f32(this.addr() + 0xf4, 3.0);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x0066_b120, &args![this, 0x5150u32, 0x6160u32])
            .u32();
        assert_eq!(r, this.addr());
        assert_eq!(e.mem.u32(this.addr()), TREE_NODE_VTABLE);
        assert_eq!(e.get(this, BSTreeNode::spTreeModel), 0x5150);
        let set_at: Vec<u32> = calls_to(&e, NI_NODE_SET_AT).iter().map(|w| w[1]).collect();
        assert_eq!(set_at, vec![0, 1, 2]);
        let floats = &calls_to(&e, BILLBOARD_NODE_SET_006404B0)[0];
        assert_eq!(floats[1], (-std::f32::consts::FRAC_PI_2).to_bits());
        assert_eq!(&floats[2..], &[1.0f32.to_bits(), 0, 0]);
        let branches = e.get(this, BSTreeNode::pBranchLevels);
        assert_eq!(e.mem.u32(branches - 4), 2);
        assert_eq!(e.get(this, BSTreeNode::pLeafLevels), 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW_ARRAY), vec![vec![12]]);
        assert_eq!(
            calls_to(&e, ADD_EXTRA_DATA),
            vec![vec![this.addr(), 0x4444]]
        );
        assert_eq!(
            calls_to(&e, COLLISION_SET_WORLD_OBJECT),
            vec![vec![collision, 0x6160]]
        );
        assert_eq!(calls_to(&e, ATTACH), vec![vec![collision, this.addr()]]);
        assert_eq!(e.get(this, BSTreeNode::fTimer), 0);
    }

    #[test]
    fn node_getters_ask_for_their_kind() {
        let mut e = Engine::new();
        e.register(NODE_OF_KIND, |_, a| word(0x100 + a[1]));
        assert_eq!(e.call(0x0066_b5c0, &args![0x1000u32]).u32(), 0x100);
        assert_eq!(e.call(0x0066_b5e0, &args![0x1000u32]).u32(), 0x101);
        assert_eq!(e.call(0x0066_b600, &args![0x1000u32]).u32(), 0x102);
    }

    #[test]
    fn fn_0066b660_reads_the_managers_pointer() {
        let mut e = Engine::new();
        pointer_get_reads(&mut e);
        e.register(TREE_MANAGER, |_, a| {
            assert_eq!(a[0], 1);
            word(0x9000)
        });
        e.map(0x9000, 0x20);
        e.mem.set_u32(0x9010, 0x77);
        assert_eq!(e.call(0x0066_b660, &args![]).u32(), 0x77);
    }

    #[test]
    fn destructor_deletes_the_arrays_and_on_bit_0_the_node() {
        let mut e = Engine::new();
        for callee in [
            LEVEL_ARRAY_DELETE,
            POINTER_DESTRUCT,
            FADE_NODE_DESTRUCTOR,
            NI_SIZED_DELETE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this: Ptr<BSTreeNode> = e.new_object();
        e.set(this, BSTreeNode::pBranchLevels, 0x3330);
        e.call_log = Some(vec![]);
        e.call(0x0066_b680, &args![this, 1u32]);
        assert_eq!(calls_to(&e, LEVEL_ARRAY_DELETE), vec![vec![0x3330, 3]]);
        assert_eq!(e.get(this, BSTreeNode::pBranchLevels), 0);
        assert_eq!(
            calls_to(&e, POINTER_DESTRUCT),
            vec![vec![this.addr() + 0xf0], vec![this.addr() + 0xe4]]
        );
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![this.addr(), 0xf8]]);
        e.call(0x0066_b6c0, &args![this]);
        assert_eq!(calls_to(&e, LEVEL_ARRAY_DELETE).len(), 1);
        assert_eq!(calls_to(&e, FADE_NODE_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn level_getters_check_the_model_array_and_count() {
        let mut e = Engine::new();
        pointer_get_reads(&mut e);
        e.register(BRANCH_LEVEL_COUNT, |_, _| word(2));
        e.register(LEAF_LEVEL_COUNT, |_, _| word(1));
        let this: Ptr<BSTreeNode> = e.new_object();
        assert_eq!(e.call(0x0066_b7d0, &args![this, 0u32]).u32(), 0);
        e.set(this, BSTreeNode::spTreeModel, 0x5150);
        let levels = e.mem.alloc(8);
        e.mem.set_u32(levels + 4, 0x7171);
        e.set(this, BSTreeNode::pBranchLevels, levels);
        e.set(this, BSTreeNode::pLeafLevels, levels);
        assert_eq!(
            e.call(0x0066_b7d0, &args![this, 0x0001_0001u32]).u32(),
            0x7171
        );
        assert_eq!(e.call(0x0066_b7d0, &args![this, 2u32]).u32(), 0);
        assert_eq!(e.call(0x0066_b840, &args![this, 1u32]).u32(), 0);
        assert_eq!(e.call(0x0066_b840, &args![this, 0u32]).u32(), 0);
    }

    #[test]
    fn add_levels_replace_the_level_node_under_the_parent() {
        const BRANCH_NODE: u32 = 0x0100_0104;
        const LEAF_NODE: u32 = 0x0100_0108;
        const DETACH: u32 = 0x0100_00e8;
        const ATTACH: u32 = 0x0100_00dc;
        let mut e = Engine::new();
        pointer_get_reads(&mut e);
        e.register(0x0040_37b0, |_, _| Ret::default());
        e.register(0x0040_37d0, |_, _| Ret::default());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(BRANCH_LEVEL_COUNT, |_, _| word(2));
        e.register(LEAF_LEVEL_COUNT, |_, _| word(2));
        e.register(DETACH, |_, _| Ret::default());
        e.register(ATTACH, |_, _| Ret::default());
        let parent = object(&mut e, &[(0xe8, DETACH), (0xdc, ATTACH)]);
        e.register_double(BRANCH_NODE, move |_, _| word(parent));
        e.register(LEAF_NODE, |_, _| word(0));
        let this = object(&mut e, &[(0x104, BRANCH_NODE), (0x108, LEAF_NODE)]);
        let tree: Ptr<BSTreeNode> = Ptr::new(this);
        e.set(tree, BSTreeNode::spTreeModel, 0x5150);
        let levels = e.mem.alloc(8);
        e.mem.set_u32(levels + 4, 0x8181);
        e.set(tree, BSTreeNode::pBranchLevels, levels);
        e.set(tree, BSTreeNode::pLeafLevels, levels);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0066_b8b0, &args![this, 1u32, 0x9191u32]).bool());
        assert_eq!(calls_to(&e, DETACH), vec![vec![parent, 0x8181]]);
        assert_eq!(calls_to(&e, ATTACH), vec![vec![parent, 0x9191, 1]]);
        assert_eq!(e.mem.u32(levels + 4), 0x9191);
        // No node, a level past the count, no leaf node: false.
        assert!(!e.call(0x0066_b8b0, &args![this, 1u32, 0u32]).bool());
        assert!(!e.call(0x0066_b8b0, &args![this, 2u32, 0x9191u32]).bool());
        assert!(!e.call(0x0066_ba30, &args![this, 0u32, 0x9191u32]).bool());
        assert_eq!(
            calls_to(&e, 0x0040_37b0).len(),
            calls_to(&e, 0x0040_37d0).len()
        );
    }

    #[test]
    fn set_visibility_for_hides_on_false() {
        const NODE_SLOT: u32 = 0x0100_0104;
        let mut e = Engine::new();
        e.register(SET_HIDDEN, |_, _| Ret::default());
        e.register(NODE_SLOT, |_, _| word(0x2020));
        e.register(0x0100_0108, |_, _| word(0));
        e.register(0x0100_010c, |_, _| word(0x3030));
        let this = object(
            &mut e,
            &[
                (0x104, NODE_SLOT),
                (0x108, 0x0100_0108),
                (0x10c, 0x0100_010c),
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0066_bc40, &args![this, 0x100u32]).bool());
        assert!(!e.call(0x0066_bc90, &args![this, 1u32]).bool());
        assert!(e.call(0x0066_bce0, &args![this, 1u32]).bool());
        assert_eq!(
            calls_to(&e, SET_HIDDEN),
            vec![vec![0x2020, 1], vec![0x3030, 0]]
        );
    }

    #[test]
    fn set_visibility_on_levels_and_billboard() {
        const BRANCH_LEVEL: u32 = 0x0100_0110;
        const LEAF_LEVEL: u32 = 0x0100_0114;
        let mut e = Engine::new();
        pointer_get_reads(&mut e);
        e.register(SET_HIDDEN, |_, _| Ret::default());
        e.register(GET_PROPERTY, |_, _| word(0x4040));
        e.register(PROPERTY_SET_0094DB80, |_, _| Ret::default());
        e.register(BRANCH_LEVEL, |_, a| {
            word(if a[1] == 0 { 0x2020 } else { 0 })
        });
        e.register(LEAF_LEVEL, |_, _| word(0x2121));
        let this = object(&mut e, &[(0x110, BRANCH_LEVEL), (0x114, LEAF_LEVEL)]);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0066_bd30, &args![this, 0x0001_0000u32, 0xffu32])
            .bool());
        assert!(!e.call(0x0066_bd30, &args![this, 1u32, 0xffu32]).bool());
        assert!(e.call(0x0066_bdb0, &args![this, 0u32, 7u32]).bool());
        assert_eq!(
            calls_to(&e, SET_HIDDEN),
            vec![vec![0x2020, 1], vec![0x2121, 0]]
        );
        assert_eq!(calls_to(&e, GET_PROPERTY), vec![vec![0x2121, 0]]);
        assert_eq!(calls_to(&e, PROPERTY_SET_0094DB80), vec![vec![0x4040, 7]]);
        // The billboard: none, then one.
        assert!(!e.call(0x0066_be30, &args![this, 7u32]).bool());
        e.mem.set_u32(this + 0xf0, 0x2222);
        assert!(e.call(0x0066_be30, &args![this, 7u32]).bool());
        assert_eq!(calls_to(&e, SET_HIDDEN)[2], vec![0x2222, 0]);
        assert_eq!(calls_to(&e, PROPERTY_SET_0094DB80)[1], vec![0x4040, 7]);
    }

    #[test]
    fn on_visible_needs_trees_enabled() {
        let mut e = Engine::new();
        e.register(TREES_ENABLED, |_, _| word(0x100));
        e.register(NI_NODE_ON_VISIBLE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_bf50, &args![0x1000u32, 0x2000u32]);
        assert!(calls_to(&e, NI_NODE_ON_VISIBLE).is_empty());
        e.register(TREES_ENABLED, |_, _| word(1));
        e.call(0x0066_bf50, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, NI_NODE_ON_VISIBLE), vec![vec![0x1000, 0x2000]]);
    }

    fn update_doubles(e: &mut Engine) -> u32 {
        const TREE_UPDATE: u32 = 0x0100_0144;
        e.register(SCOPE_ENTER, |_, _| Ret::default());
        e.register(SCOPE_LEAVE, |_, _| Ret::default());
        e.register(TREES_ENABLED, |_, _| word(1));
        e.register(UPDATE_CHECK_ARGUMENT, |_, _| word(0x5555));
        e.register(UPDATE_CHECK, |_, _| word(0));
        e.register(TREE_UPDATE, |_, _| Ret::default());
        e.register(NI_NODE_UPDATE_DOWNWARD_PASS, |_, _| Ret::default());
        e.register(NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS, |_, _| Ret::default());
        e.map(ZERO, 8);
        object(e, &[(0x144, TREE_UPDATE)])
    }

    #[test]
    fn update_downward_pass_updates_unless_the_check_fails() {
        let mut e = Engine::new();
        let this = update_doubles(&mut e);
        let data = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        // A zero time skips the check.
        e.call(0x0066_bf80, &args![this, data, 3u32]);
        assert_eq!(
            calls_to(&e, NI_NODE_UPDATE_DOWNWARD_PASS),
            vec![vec![this, data, 3]]
        );
        assert_eq!(
            calls_to(&e, SCOPE_ENTER)[0][1..],
            [0x1e, 1, SOURCE_FILE_NAME, 0x20d]
        );
        // A positive time with a failing check: no update.
        e.mem.set_f32(data, 0.5);
        e.call(0x0066_bf80, &args![this, data, 3u32]);
        assert_eq!(calls_to(&e, UPDATE_CHECK), vec![vec![this, 0x5555]]);
        assert_eq!(calls_to(&e, NI_NODE_UPDATE_DOWNWARD_PASS).len(), 1);
        assert_eq!(calls_to(&e, SCOPE_LEAVE).len(), 2);
    }

    #[test]
    fn update_selected_downward_pass_uses_the_selected_pass() {
        let mut e = Engine::new();
        let this = update_doubles(&mut e);
        e.register(UPDATE_CHECK, |_, _| word(1));
        let data = e.mem.alloc(8);
        e.mem.set_f32(data, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0066_c070, &args![this, data, 3u32]);
        assert_eq!(
            calls_to(&e, 0x0100_0144),
            vec![vec![this, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS),
            vec![vec![this, data, 3]]
        );
        e.register(TREES_ENABLED, |_, _| word(0));
        e.call(0x0066_c070, &args![this, data, 3u32]);
        assert_eq!(calls_to(&e, NI_NODE_UPDATE_SELECTED_DOWNWARD_PASS).len(), 1);
    }

    #[test]
    fn character_proxy_velocity_converts_the_havok_vector() {
        let mut e = Engine::new();
        e.register(CHARACTER_PROXY_OBJECT, |_, _| word(0));
        e.register(CHARACTER_PROXY_UNMARK_READ, |_, _| Ret::default());
        e.register(HK_CHARACTER_PROXY_GET_LINEAR_VELOCITY, |_, _| word(0x6060));
        e.register(VECTOR_FROM_HAVOK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_ca00, &args![0x1000u32, 0x2000u32]);
        assert!(calls_to(&e, VECTOR_FROM_HAVOK).is_empty());
        e.register(CHARACTER_PROXY_OBJECT, |_, _| word(0x5050));
        e.call(0x0066_ca00, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, VECTOR_FROM_HAVOK), vec![vec![0x2000, 0x6060]]);
        assert_eq!(calls_to(&e, CHARACTER_PROXY_UNMARK_READ).len(), 2);
    }

    #[test]
    fn fn_0066ca50_staggers_and_advances_the_timer() {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.map(TIMER_COUNTER, 4);
        let interval = e.mem.alloc(4);
        e.mem.set_f32(interval, 2.0);
        let slots = e.mem.alloc(4);
        e.mem.set_i32(slots, 4);
        e.register_double(FLOAT_VALUE_ADDRESS, move |_, _| word(interval));
        e.register_double(INT_VALUE_ADDRESS, move |_, _| word(slots));
        let this: Ptr<BSTreeNode> = e.new_object();
        assert!(e.call(0x0066_ca50, &args![this, 0.0f32]).bool());
        e.set_global(TIMER_COUNTER, 3u8);
        // Unset: 1 + 2 + 2 / 4 * 3; the counter steps to 0 (4 % 4).
        assert!(e.call(0x0066_ca50, &args![this, 1.0f32]).bool());
        assert_eq!(e.mem.f32(this.addr() + 0xf4), 4.5);
        assert_eq!(e.global::<u8>(TIMER_COUNTER), 0);
        // Not yet due: false; due: advanced to time + interval.
        assert!(!e.call(0x0066_ca50, &args![this, 4.5f32]).bool());
        assert!(e.call(0x0066_ca50, &args![this, 5.0f32]).bool());
        assert_eq!(e.mem.f32(this.addr() + 0xf4), 7.0);
        assert!(!e.call(0x0066_ca50, &args![this, f32::NAN]).bool());
    }

    #[test]
    fn small_forwarders() {
        let mut e = Engine::new();
        e.register(COLLISION_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(CAPSULE_SHAPE_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_d410, &args![0x1000u32]);
        assert_eq!(
            calls_to(&e, COLLISION_OBJECT_DESTRUCTOR),
            vec![vec![0x1000]]
        );
        assert_eq!(e.call(0x0066_d730, &args![0x1000u32]).u32(), 0x30);
        assert_eq!(e.call(0x0066_d740, &args![0x1000u32, 0u32]).u32(), 0x1000);
        assert!(calls_to(&e, NI_SIZED_DELETE).is_empty());
        e.call(0x0066_d740, &args![0x1000u32, 1u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![0x1000, 0x14]]);
        assert_eq!(calls_to(&e, CAPSULE_SHAPE_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn fn_0066d7c0_casts_the_leaf_object() {
        const LEAF_NODE: u32 = 0x0100_0108;
        let mut e = Engine::new();
        pointer_get_reads(&mut e);
        e.register(LEAF_NODE, |_, _| word(0));
        e.register(LEAF_OBJECT_006838B0, |_, _| word(0x7070));
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(OBJECT_006FA820, |_, _| word(0x8080));
        e.register(CAST_00653270, |_, _| word(0x9090));
        let this = object(&mut e, &[(0x108, LEAF_NODE)]);
        assert_eq!(e.call(0x0066_d7c0, &args![this]).u32(), 0);
        e.register(LEAF_NODE, |_, _| word(0x6060));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_d7c0, &args![this]).u32(), 0x9090);
        assert_eq!(
            calls_to(&e, CAST_00653270),
            vec![vec![RTTI_01268230, 0x8080]]
        );
        assert_eq!(calls_to(&e, POINTER_DESTRUCT).len(), 1);
    }
}

#[cfg(test)]
mod update_tree_node_tests {
    use super::update_callees::*;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x500);
        e.mem.set_u32(object, vtable);
        object
    }

    type Sounds = Rc<RefCell<Vec<Vec<u8>>>>;

    /// A tree node whose timer fires, whose leaf object has a body, and
    /// whose collector holds an actor's body and a reference's body.
    fn scene(e: &mut Engine) -> (Ptr<BSTreeNode>, u32, Sounds) {
        const MODEL_UPDATE: u32 = 0x0100_0008;
        const LEAF_NODE: u32 = 0x0100_0108;
        const LEAF_HAS_BODY: u32 = 0x0100_0094;
        const CONTROLLER_CONTACT: u32 = 0x0100_0208;
        e.register(MODEL_UPDATE, |_, _| Ret::default());
        e.register(POINTER_GET, |e, a| word(e.mem.u32(a[0])));
        e.map(ZERO, 8);
        e.map(TIMER_COUNTER, 4);
        for (addr, value) in [
            (HUNDRED, 100.0f64),
            (QUARTER, 0.25),
            (NINETY, 90.0),
            (ONE, 1.0),
        ] {
            e.map(addr, 8);
            e.set_global(addr, value);
        }
        let interval = e.mem.alloc(4);
        e.mem.set_f32(interval, 1.0);
        e.register_double(FLOAT_VALUE_ADDRESS, move |_, _| word(interval));
        let slots = e.mem.alloc(4);
        e.mem.set_i32(slots, 1);
        e.register_double(INT_VALUE_ADDRESS, move |_, _| word(slots));
        // The leaf object through 0066d7c0.
        let leaf = object(e, &[(0x94, LEAF_HAS_BODY)]);
        e.register(LEAF_HAS_BODY, |_, _| word(1));
        e.register(LEAF_NODE, |_, _| word(0x6060));
        e.register(LEAF_OBJECT_006838B0, |_, _| word(0x7070));
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(OBJECT_006FA820, |_, _| word(0x8080));
        e.register_double(CAST_00653270, move |_, _| word(leaf));
        // The file name "tree~CF7".
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"tree~CF7");
        e.register(MODEL_FILE, |_, _| word(0x5050));
        e.register_double(FILE_NAME, move |_, _| word(name));
        e.register_double(STRSTR, move |_, _| word(name + 4));
        // Two collected bodies: an actor's, then a reference's.
        let (actor_body, reference_body) = (e.mem.alloc(8), e.mem.alloc(8));
        let entries = e.mem.alloc(0x20);
        e.mem.set_u32(entries + 8, actor_body);
        e.mem.set_u32(entries + 0x18, reference_body);
        for callee in [COLLECTOR_CONSTRUCT, COLLECTOR_DESTRUCT, COLLECT] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(COLLECTOR_LIST, |_, _| word(0x4444));
        e.register(LIST_COUNT, |_, _| word(2));
        e.register_double(LIST_ENTRY, move |_, a| word(entries + 0x10 * a[1]));
        e.register_double(BODY_ACTOR, move |_, a| {
            word(if a[0] == actor_body { 0x1110 } else { 0 })
        });
        e.register_double(BODY_REFERENCE, move |_, a| {
            word(if a[0] == reference_body { 0x2220 } else { 0 })
        });
        // The actor branch.
        let controller = e.mem.alloc(0x500);
        let contact_target = object(e, &[(8, CONTROLLER_CONTACT)]);
        let contact_vtable = e.mem.u32(contact_target);
        e.mem.set_u32(controller + 0x410, contact_vtable);
        e.register(CONTROLLER_CONTACT, |_, _| Ret::default());
        e.register(FIND_BS_REFERENCE, |_, _| word(0x3330));
        e.register_double(GET_CHAR_CONTROLLER, move |_, _| word(controller));
        e.register(CHARACTER_PROXY_OBJECT, |_, _| word(0));
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 9.0);
        e.register_double(POINT_COMBINE, move |_, _| word(point));
        e.register(POINT_LENGTH, |_, _| 3.0f64.into_ret());
        e.register(RANDOM_SIGNED, |_, _| 0.5f64.into_ret());
        e.register(RANDOM_UNIT, |_, _| 0.75f64.into_ret());
        e.register(RANDOM_INT, |_, a| word(if a[1] == 100 { 50 } else { 0 }));
        e.register(VECTOR_LENGTH, |_, _| 2.0f64.into_ret());
        e.register(VECTOR_00630100, |_, a| word(a[1]));
        e.register(VECTOR_00630B40, |_, a| word(a[1]));
        e.register(OBJECT_MATERIAL_SOURCE, |_, _| word(0));
        e.register(SCALE_LENGTH, |_, _| 4.0f64.into_ret());
        e.register(REFERENCE_SOUND, |_, _| word(0x55));
        for callee in [
            CONTACT_CONSTRUCT,
            VECTOR4_SET,
            VECTOR4_FIX,
            BODY_OBJECT,
            OBJECT_POSITION,
            VECTOR_FROM_POINT,
            CONTACT_SET,
            LEAF_BODY,
            SOUND_DATA_CONSTRUCT,
            POINT_CONSTRUCT,
            REFERENCE_LINEAR,
            REFERENCE_ANGULAR,
            SCALE_CONSTRUCT,
            VECTOR_SCALE,
            SET_LINEAR_VELOCITY,
            SET_ANGULAR_VELOCITY,
            REFERENCE_OBJECT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let sounds: Sounds = Rc::new(RefCell::new(vec![]));
        let seen = sounds.clone();
        e.register_double(PLAY_COLLISION_SOUND, move |e, a| {
            seen.borrow_mut().push(e.mem.bytes(a[0], 0x24));
            Ret::default()
        });
        let model = object(e, &[(8, MODEL_UPDATE)]);
        let tree = object(e, &[(0x108, LEAF_NODE)]);
        let this: Ptr<BSTreeNode> = Ptr::new(tree);
        e.set(this, BSTreeNode::spTreeModel, model);
        (this, leaf, sounds)
    }

    fn u32_at(bytes: &[u8], at: usize) -> u32 {
        u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
    }

    #[test]
    fn update_bs_tree_node_shakes_the_bodies_touching_the_leaves() {
        let mut e = Engine::new();
        let (this, leaf, sounds) = scene(&mut e);
        e.call_log = Some(vec![]);
        // A zero time: the timer answers true at once.
        e.call(0x0066_c160, &args![this, 0.0f32]);
        assert_eq!(
            calls_to(&e, 0x0100_0008),
            vec![vec![e.get(this, BSTreeNode::spTreeModel), this.addr()]]
        );
        let sounds = sounds.borrow();
        assert_eq!(sounds.len(), 2);
        // The actor's sound: position, length, 1.0, kinds 4 and 7, actor + leaf.
        assert_eq!(f32::from_bits(u32_at(&sounds[0], 0)), 9.0);
        assert_eq!(f32::from_bits(u32_at(&sounds[0], 0xc)), 3.0);
        assert_eq!(u32_at(&sounds[0], 0x10), 1.0f32.to_bits());
        assert_eq!(&sounds[0][0x14..0x16], &[4, 7]);
        assert_eq!(u32_at(&sounds[0], 0x18), 0x1110 + leaf);
        // The reference's sound: the scaled length, kind 4, no material.
        assert_eq!(f32::from_bits(u32_at(&sounds[1], 0xc)), 4.0);
        assert_eq!(&sounds[1][0x14..0x16], &[4, 0]);
        assert_eq!(u32_at(&sounds[1], 0x18), 0x2220 + leaf);
        assert_eq!(u32_at(&sounds[1], 0x1c), 0x55);
        // The direction of the actor contact: the three random floats, then 0.
        let set = &calls_to(&e, VECTOR4_SET)[0];
        assert_eq!(
            &set[1..],
            &[0.5f32.to_bits(), 0.5f32.to_bits(), 0.5f32.to_bits(), 0]
        );
        // The reference's velocities: (1 - 0/100) * 2 and (0/100 + 90) * 2.
        let scales: Vec<u32> = calls_to(&e, SCALE_CONSTRUCT).iter().map(|w| w[1]).collect();
        assert_eq!(scales, vec![2.0f32.to_bits(), 180.0f32.to_bits()]);
        let set = &calls_to(&e, VECTOR4_SET)[1];
        assert_eq!(set[3], 0.5f32.to_bits());
        assert_eq!(calls_to(&e, SET_LINEAR_VELOCITY)[0][0], 0x2220);
        assert_eq!(calls_to(&e, SET_ANGULAR_VELOCITY)[0][0], 0x2220);
        // The threshold is 1.0 * 7 * 100: the roll of 50 is within it.
        assert_eq!(calls_to(&e, RANDOM_INT)[0], vec![0, 100]);
    }

    #[test]
    fn update_bs_tree_node_stops_without_a_due_timer() {
        let mut e = Engine::new();
        let (this, _, sounds) = scene(&mut e);
        e.set(this, BSTreeNode::fTimer, 10.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x0066_c160, &args![this, 5.0f32]);
        assert!(sounds.borrow().is_empty());
        assert_eq!(calls_to(&e, 0x0100_0008).len(), 1);
        assert!(calls_to(&e, COLLECTOR_CONSTRUCT).is_empty());
    }
}

#[cfg(test)]
mod make_havok_leaves_tests {
    use super::havok_leaf_callees::*;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    type Objects = Rc<RefCell<Vec<(u32, [f32; 7])>>>;

    /// A tree whose collision objects are `objects` (type, position,
    /// dimensions); conversions double, the tree scale is 1.
    fn scene(e: &mut Engine, objects: Objects) -> u32 {
        const LEAF_NODE: u32 = 0x0100_0108;
        e.register(SETTING_ENABLED, |_, _| word(1));
        let count = objects.borrow().len() as u32;
        e.register_double(COLLISION_OBJECT_COUNT, move |_, _| word(count));
        e.register_double(GET_COLLISION_OBJECT, move |e, a| {
            let (kind, values) = objects.borrow()[a[1] as usize];
            e.mem.set_u32(a[2], kind);
            for (i, v) in values.iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(TREE_SCALE, |_, _| 1.0f64.into_ret());
        e.register(TO_HAVOK_UNITS, |_, a| {
            (f64::from(f32::from_bits(a[0])) * 2.0).into_ret()
        });
        e.register(LEAF_NODE, |_, _| word(0x6060));
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(HAVOK_OBJECT, |_, a| word(a[0] + 1));
        e.register(POINTER_GET, |_, _| word(0x7777));
        for callee in [
            PHANTOM_DATA_CONSTRUCT,
            PHANTOM_DATA_DESTRUCT,
            FILTER_CONSTRUCT,
            TRANSFORM_CINFO_CONSTRUCT,
            CAPSULE_CINFO_CONSTRUCT,
            CINFO_SET_TRANSLATION,
            VECTOR4_SET_004B4FF0,
            VECTOR4_SET,
            SPHERE_LIST_CONSTRUCT,
            SPHERE_LIST_DESTRUCT,
            SPHERE_LIST_ADD,
            REPORT,
            COLLISION_SET_WORLD_OBJECT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        for constructor in [
            SPHERE_SHAPE_CONSTRUCT,
            TRANSFORM_SHAPE_CONSTRUCT,
            CAPSULE_SHAPE_CONSTRUCT,
            BOX_SHAPE_CONSTRUCT,
            MULTI_SPHERE_SHAPE_CONSTRUCT,
            PHANTOM_CONSTRUCT,
            LEAF_COLLISION_CONSTRUCT,
        ] {
            e.register(constructor, |_, a| word(a[0]));
        }
        e.map(HALF, 8);
        e.set_global(HALF, 0.5f64);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x108, LEAF_NODE);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    fn floats(words: &[u32]) -> Vec<f32> {
        words.iter().map(|w| f32::from_bits(*w)).collect()
    }

    #[test]
    fn make_havok_leaves_needs_the_setting_and_objects() {
        let mut e = Engine::new();
        let this = scene(&mut e, Rc::new(RefCell::new(vec![])));
        e.call_log = Some(vec![]);
        e.call(0x0066_cb40, &args![this, 0x5000u32, 1.0f32]);
        assert!(calls_to(&e, TREE_SCALE).is_empty());
        e.register(SETTING_ENABLED, |_, _| word(0));
        e.call(0x0066_cb40, &args![this, 0x5000u32, 1.0f32]);
        assert_eq!(calls_to(&e, COLLISION_OBJECT_COUNT).len(), 1);
    }

    #[test]
    fn make_havok_leaves_builds_a_sphere_capsule_or_box() {
        let objects: Objects = Rc::new(RefCell::new(vec![(
            0,
            [1.0, 2.0, 3.0, 0.5, 0.25, 0.125, 0.0],
        )]));
        let mut e = Engine::new();
        let this = scene(&mut e, objects.clone());
        e.call_log = Some(vec![]);
        // The scale 1.5: each value * 2 (conversion) * 1.5.
        e.call(0x0066_cb40, &args![this, 0x5000u32, 1.5f32]);
        let sphere = &calls_to(&e, SPHERE_SHAPE_CONSTRUCT)[0];
        assert_eq!((f32::from_bits(sphere[1]), sphere[2]), (1.5, 0));
        let translation = &calls_to(&e, VECTOR4_SET)[0];
        assert_eq!(floats(&translation[1..]), vec![3.0, 6.0, 9.0, 0.0]);
        assert_eq!(calls_to(&e, TRANSFORM_SHAPE_CONSTRUCT).len(), 1);
        let world = &calls_to(&e, COLLISION_SET_WORLD_OBJECT)[0];
        let phantom = &calls_to(&e, PHANTOM_CONSTRUCT)[0];
        assert_eq!(world[1], phantom[0]);
        assert_eq!(calls_to(&e, LEAF_COLLISION_CONSTRUCT)[0][1], 0x6060);
        assert_eq!(e.mem.u32(phantom[1]), 0x7777);
        // A capsule: the two ends at z + radius and z + height - radius.
        objects.borrow_mut()[0] = (1, [1.0, 2.0, 3.0, 0.5, 4.0, 0.0, 0.0]);
        e.call(0x0066_cb40, &args![this, 0x5000u32, 1.0f32]);
        let ends = calls_to(&e, VECTOR4_SET_004B4FF0);
        assert_eq!(floats(&ends[0][1..]), vec![2.0, 4.0, 7.0, 0.0]);
        assert_eq!(floats(&ends[1][1..]), vec![2.0, 4.0, 13.0, 0.0]);
        assert_eq!(calls_to(&e, CAPSULE_SHAPE_CONSTRUCT).len(), 1);
        // A box: the dimensions halved, the centre raised by the half depth.
        objects.borrow_mut()[0] = (2, [1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 0.0]);
        e.call(0x0066_cb40, &args![this, 0x5000u32, 1.0f32]);
        let sets = calls_to(&e, VECTOR4_SET);
        assert_eq!(floats(&sets[1][1..]), vec![2.0, 4.0, 14.0, 0.0]);
        assert_eq!(floats(&sets[2][1..]), vec![4.0, 6.0, 8.0, 0.0]);
        assert_eq!(calls_to(&e, BOX_SHAPE_CONSTRUCT).len(), 1);
        // Another type: no shape.
        objects.borrow_mut()[0] = (3, [0.0; 7]);
        e.call(0x0066_cb40, &args![this, 0x5000u32, 1.0f32]);
        assert_eq!(calls_to(&e, PHANTOM_CONSTRUCT).len(), 4);
    }

    #[test]
    fn make_havok_leaves_joins_several_spheres() {
        let objects: Objects = Rc::new(RefCell::new(vec![
            (0, [1.0, 2.0, 3.0, 0.5, 0.0, 0.0, 0.0]),
            (1, [0.0; 7]),
        ]));
        let mut e = Engine::new();
        let this = scene(&mut e, objects);
        e.call_log = Some(vec![]);
        e.call(0x0066_cb40, &args![this, 0x5000u32, 1.0f32]);
        let sphere = &calls_to(&e, VECTOR4_SET)[0];
        assert_eq!(floats(&sphere[1..]), vec![2.0, 4.0, 6.0, 1.0]);
        assert_eq!(calls_to(&e, SPHERE_LIST_ADD).len(), 1);
        assert_eq!(calls_to(&e, REPORT), vec![vec![NOT_ALL_SPHERES]]);
        assert_eq!(calls_to(&e, MULTI_SPHERE_SHAPE_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, SPHERE_LIST_DESTRUCT).len(), 1);
    }
}

/// The vtables (RTTI names) the n086 constructors store:
/// `bhkCollisionObject` (`0103046c`), `bhkSPCollisionObject` (`01066acc`),
/// `bhkPCollisionObject` (`01066b94`), `bhkMultiSphereShape` (`01066c5c`),
/// `bhkTransformShape` (`01066d44`) and `bhkCapsuleShape` (`01066e2c`).
const BHK_COLLISION_OBJECT_VTABLE: u32 = 0x0103_046c;
const BHK_SP_COLLISION_OBJECT_VTABLE: u32 = 0x0106_6acc;
const BHK_P_COLLISION_OBJECT_VTABLE: u32 = 0x0106_6b94;
const BHK_MULTI_SPHERE_SHAPE_VTABLE: u32 = 0x0106_6c5c;
const BHK_TRANSFORM_SHAPE_VTABLE: u32 = 0x0106_6d44;
const BHK_CAPSULE_SHAPE_VTABLE: u32 = 0x0106_6e2c;
/// The base constructors (no words) and the `Init` calls (one word: the
/// construction info) of the three shapes, and the counters each
/// constructor raises.
const MULTI_SPHERE_BASE_0056E6C0: u32 = 0x0056_e6c0;
const MULTI_SPHERE_INIT_00CB7B80: u32 = 0x00cb_7b80;
const MULTI_SPHERE_COUNTER_012683DC: u32 = 0x0126_83dc;
const TRANSFORM_BASE_0056E6F0: u32 = 0x0056_e6f0;
const TRANSFORM_INIT_00C8C5B0: u32 = 0x00c8_c5b0;
const TRANSFORM_COUNTER_012681B0: u32 = 0x0126_81b0;
const CAPSULE_BASE_0056E690: u32 = 0x0056_e690;
const CAPSULE_INIT_00CAB820: u32 = 0x00ca_b820;
const CAPSULE_COUNTER_01268324: u32 = 0x0126_8324;
/// `bhkMultiSphereShape::~bhkMultiSphereShape` (`00cb78b0`) and
/// `bhkTransformShape::~bhkTransformShape` (`00c8bed0`), no words.
const MULTI_SPHERE_DESTRUCTOR: u32 = 0x00cb_78b0;
const TRANSFORM_DESTRUCTOR: u32 = 0x00c8_bed0;
/// The collision object base constructors: `00c6b910` (no words) and
/// `00c6b950` (one word).
const COLLISION_BASE_00C6B910: u32 = 0x00c6_b910;
const COLLISION_BASE_00C6B950: u32 = 0x00c6_b950;
/// The steps of `hkTransformShapeCinfo::hkTransformShapeCinfo`: `004b5020`
/// on `this`, `0056e270` and `0056e120` on `+0x10` (no words).
const CINFO_BASE_004B5020: u32 = 0x004b_5020;
const CINFO_MEMBER_0056E270: u32 = 0x0056_e270;
const CINFO_MEMBER_0056E120: u32 = 0x0056_e120;
/// The steps of `0066d650`: `0056e8d0` on `this`, `004b4fa0` on `+0x10` and
/// `+0x20` (no words), `004b4ff0` on `+0x20` (four floats).
const BASE_0056E8D0: u32 = 0x0056_e8d0;
const MEMBER_004B4FA0: u32 = 0x004b_4fa0;
const MEMBER_SET_004B4FF0: u32 = 0x004b_4ff0;
/// The steps of `0066d770`: `0056e900` on `this`, `006299a0` on `+4` (no
/// words).
const BASE_0056E900: u32 = 0x0056_e900;
const MEMBER_006299A0: u32 = 0x0062_99a0;
/// The steps of `0066da50`: `0066dab0` and `00483710` (no words).
const BODY_0066DAB0: u32 = 0x0066_dab0;
const AFTER_00483710: u32 = 0x0048_3710;
/// The steps of `0066c930`: `0062b0a0` (two words), `006815c0` (no words),
/// `004a3c90` and `00561240` (one word each).
const STEP_0062B0A0: u32 = 0x0062_b0a0;
const STEP_006815C0: u32 = 0x0068_15c0;
const STEP_004A3C90: u32 = 0x004a_3c90;
const STEP_00561240: u32 = 0x0056_1240;
/// The steps of `0066c9c0`: `004a3f10` (one word) on `this` and `+0x10`,
/// `00560d30` (one word: 3) on `+0x10`, answering a float's address.
const SET_004A3F10: u32 = 0x004a_3f10;
const SLOT_00560D30: u32 = 0x0056_0d30;
/// The steps of `0066d9b0`: `004a4840` (`thiscall` on a zeroed stack byte,
/// one word) and `0066d9e0` (two words).
const STEP_004A4840: u32 = 0x004a_4840;
const STEP_0066D9E0: u32 = 0x0066_d9e0;
/// `NiAVObject::AttachProperty` (`00439410`, engine map; one word) and
/// `00a5b230` (one word: the property type), which removes a property.
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const REMOVE_PROPERTY_00A5B230: u32 = 0x00a5_b230;
/// The calls of `BSTreeNode::GetViewerStrings`: `NiNode::GetViewerStrings`
/// (`00a5ea70`), `NiGetViewerString` (`00a7d330`, `cdecl`), the string list
/// add (`0096a610`), the model test `00666f00` and the seed `008d6f30` (no
/// words), and `00406d00` (`cdecl`: buffer, size, format, ...).
const NI_NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_ADD_0096A610: u32 = 0x0096_a610;
const MODEL_TEST_00666F00: u32 = 0x0066_6f00;
const MODEL_SEED_008D6F30: u32 = 0x008d_6f30;
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// The texts: `"BSTreeNode"` (`01066f40`), `"Type = Instance"`
/// (`01066f30`), `"Type = Base"` (`01066f24`), `"Seed = %d"` (`01066f18`)
/// and `"-No Model-"` (`01066f0c`).
const TEXT_BS_TREE_NODE: u32 = 0x0106_6f40;
const TEXT_TYPE_INSTANCE: u32 = 0x0106_6f30;
const TEXT_TYPE_BASE: u32 = 0x0106_6f24;
const TEXT_SEED: u32 = 0x0106_6f18;
const TEXT_NO_MODEL: u32 = 0x0106_6f0c;

/// Adds one to the word at `counter`.
fn count_instance(e: &mut Engine, counter: u32) {
    let count = e.mem.u32(counter).wrapping_add(1);
    e.mem.set_u32(counter, count);
}

// Translated from 00622430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::QCinfoSize` (engine map; `thiscall`, plain `RET`):
/// `0x50`.
pub fn bhk_transform_shape_q_cinfo_size(_e: &mut Engine, _this: Ptr) -> u32 {
    0x50
}

// Translated from 0066b620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::GetBillboard` (engine map; `thiscall`, plain `RET`): the
/// billboard pointer at `+0xf0` (`00559450`).
pub fn bs_tree_node_get_billboard(e: &mut Engine, this: Ptr) -> u32 {
    e.call(POINTER_GET, &args![this.addr() + 0xf0]).u32()
}

// Translated from 0066b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `bhkCollisionObject` constructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the base constructor `00c6b910`, then the
/// vtable `0103046c`. Returns `this`.
pub fn fn_0066b640(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(COLLISION_BASE_00C6B910, &args![this]);
    e.mem.set_u32(this.addr(), BHK_COLLISION_OBJECT_VTABLE);
    this
}

// Translated from 0066bbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetBillboard` (engine map; `thiscall`, `RET 4`): with a
/// billboard and a node from the virtual slot `+0x10c`, the old billboard
/// (when set) goes to the node's slot `+0xe8`, the new one to its slot
/// `+0xdc` with 1, and becomes the pointer at `+0xf0` (`0066b0d0`).
pub fn bs_tree_node_set_billboard(e: &mut Engine, this: Ptr, billboard: u32) {
    if billboard == 0 {
        return;
    }
    let node = e.vcall(this.addr(), 0x10c, &[]).u32();
    if node == 0 {
        return;
    }
    let slot = this.addr() + 0xf0;
    if e.call(POINTER_GET, &args![slot]).u32() != 0 {
        let old = e.call(POINTER_GET, &args![slot]).u32();
        e.vcall(node, 0xe8, &args![old]);
    }
    e.vcall(node, 0xdc, &args![billboard, 1u32]);
    e.call(POINTER_ASSIGN, &args![slot, billboard]);
}

// Translated from 0066bec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::SetTexturePropOnBillboard` (engine map; `thiscall`,
/// `RET 4`): false without a billboard (`+0xf0`) or a property; false when
/// the billboard's property of type 5 (`NiAVObject::GetProperty`) already
/// is `property`; otherwise that one (when set) is removed (`00a5b230` with
/// 5), `property` attached (`NiAVObject::AttachProperty`), and true.
pub fn bs_tree_node_set_texture_prop_on_billboard(
    e: &mut Engine,
    this: Ptr,
    property: u32,
) -> bool {
    let slot = this.addr() + 0xf0;
    if e.call(POINTER_GET, &args![slot]).u32() == 0 || property == 0 {
        return false;
    }
    let billboard = e.call(POINTER_GET, &args![slot]).u32();
    let current = e.call(GET_PROPERTY, &args![billboard, 5u32]).u32();
    if current != 0 {
        if current == property {
            return false;
        }
        let billboard = e.call(POINTER_GET, &args![slot]).u32();
        e.call(REMOVE_PROPERTY_00A5B230, &args![billboard, 5u32]);
    }
    let billboard = e.call(POINTER_GET, &args![slot]).u32();
    e.call(ATTACH_PROPERTY, &args![billboard, property]);
    true
}

// Translated from 0066c930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066c930 - the /GS stack-cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, plain `RET`; `this` is a 16-byte
/// vector): `0062b0a0` on `this` with a stack temporary and `this`,
/// `006815c0` on its answer; `RSQRTSS` of that vector (the first lane's
/// reciprocal root estimate, the others kept) made into a value by
/// `004a3c90` and `00561240`; `this` multiplied lane by lane (`MULPS`) by
/// the vector that answers.
pub fn fn_0066c930(e: &mut Engine, this: Ptr) {
    e.with_stack(0x70, |e, frame| {
        let frame = frame.addr();
        let first = e
            .call(STEP_0062B0A0, &args![this, frame + 0x50, this])
            .u32();
        let source = e.call(STEP_006815C0, &args![first]).u32();
        let mut vector = read_xmm(e, source);
        vector[0] = crate::x87::rsqrt_estimate(vector[0]);
        write_xmm(e, frame + 0x30, vector);
        e.call(STEP_004A3C90, &args![frame + 0x20, frame + 0x30]);
        let factor = e
            .call(STEP_00561240, &args![frame + 0x20, frame + 0x10])
            .u32();
        let factor = read_xmm(e, factor);
        let product = crate::sse::mulps(read_xmm(e, this.addr()), factor);
        write_xmm(e, frame, product);
        write_xmm(e, this.addr(), product);
    });
}

/// The four floats at `at` (`MOVAPS` load, bits kept).
fn read_xmm(e: &Engine, at: u32) -> [f32; 4] {
    std::array::from_fn(|lane| f32::from_bits(e.mem.u32(at + lane as u32 * 4)))
}

/// Stores four floats at `at` (`MOVAPS` store, bits kept).
fn write_xmm(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (lane, v) in (0u32..).zip(value) {
        e.mem.set_u32(at + lane * 4, v.to_bits());
    }
}

// Translated from 0066c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): `004a3f10` with
/// `first` on `this` and with `second` on `this + 0x10`, then `value`
/// (`FLD`/`FSTP`) stored where `00560d30` with 3 on `this + 0x10` points.
pub fn fn_0066c9c0(e: &mut Engine, this: Ptr, first: u32, second: u32, value: f32) {
    e.call(SET_004A3F10, &args![this, first]);
    e.call(SET_004A3F10, &args![this.addr() + 0x10, second]);
    let slot = e
        .call(SLOT_00560D30, &args![this.addr() + 0x10, 3u32])
        .u32();
    e.mem.set_u32(slot, x87_quiet_bits(value.to_bits()));
}

// Translated from 0066d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `bhkSPCollisionObject` constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the `bhkPCollisionObject` constructor (`0066d3d0`)
/// with the word, then the vtable `01066acc`. Returns `this`.
pub fn fn_0066d3a0(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    fn_0066d3d0(e, this, object);
    e.mem.set_u32(this.addr(), BHK_SP_COLLISION_OBJECT_VTABLE);
    this
}

// Translated from 0066d3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `bhkPCollisionObject` constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the base constructor `00c6b950` with the word,
/// then the vtable `01066b94`. Returns `this`.
pub fn fn_0066d3d0(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.call(COLLISION_BASE_00C6B950, &args![this, object]);
    e.mem.set_u32(this.addr(), BHK_P_COLLISION_OBJECT_VTABLE);
    this
}

// Translated from 0066d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): the destructor `0066d470`, then the sized delete `00aa1460`
/// of `this` (0x14 bytes) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0066d440(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0066d470(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 0066d490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066d490 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `bhkMultiSphereShape::bhkMultiSphereShape` (engine map; `thiscall`,
/// `RET 4`): the base constructor `0056e6c0`, the vtable `01066c5c`, `Init`
/// (`00cb7b80`) with the construction info, and the counter at `012683dc`
/// raised. Returns `this`.
pub fn bhk_multi_sphere_shape_bhk_multi_sphere_shape(e: &mut Engine, this: Ptr, info: u32) -> Ptr {
    e.call(MULTI_SPHERE_BASE_0056E6C0, &args![this]);
    e.mem.set_u32(this.addr(), BHK_MULTI_SPHERE_SHAPE_VTABLE);
    e.call(MULTI_SPHERE_INIT_00CB7B80, &args![this, info]);
    count_instance(e, MULTI_SPHERE_COUNTER_012683DC);
    this
}

// Translated from 0066d520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkMultiSphereShape` scalar deleting destructor (engine map;
/// `thiscall`, `RET 4`): the destructor (`00cb78b0`), then the sized delete
/// `00aa1460` of `this` (0x14 bytes) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn bhk_multi_sphere_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(MULTI_SPHERE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 0066d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTransformShapeCinfo::hkTransformShapeCinfo` (engine map; `thiscall`,
/// plain `RET`): `004b5020` on `this`, `+4` cleared, `0056e270` and
/// `0056e120` on `+0x10`. Returns `this`.
pub fn hk_transform_shape_cinfo_hk_transform_shape_cinfo(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CINFO_BASE_004B5020, &args![this]);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(CINFO_MEMBER_0056E270, &args![this.addr() + 0x10]);
    e.call(CINFO_MEMBER_0056E120, &args![this.addr() + 0x10]);
    this
}

// Translated from 0066d590 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066d590 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `bhkTransformShape::bhkTransformShape` (engine map; `thiscall`,
/// `RET 4`): the base constructor `0056e6f0`, the vtable `01066d44`, `Init`
/// (`00c8c5b0`) with the construction info, and the counter at `012681b0`
/// raised. Returns `this`.
pub fn bhk_transform_shape_bhk_transform_shape(e: &mut Engine, this: Ptr, info: u32) -> Ptr {
    e.call(TRANSFORM_BASE_0056E6F0, &args![this]);
    e.mem.set_u32(this.addr(), BHK_TRANSFORM_SHAPE_VTABLE);
    e.call(TRANSFORM_INIT_00C8C5B0, &args![this, info]);
    count_instance(e, TRANSFORM_COUNTER_012681B0);
    this
}

// Translated from 0066d620 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkTransformShape` scalar deleting destructor (engine map;
/// `thiscall`, `RET 4`): the destructor (`00c8bed0`), then the sized delete
/// `00aa1460` of `this` (0x14 bytes) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn bhk_transform_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(TRANSFORM_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 0066d650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a constructor):
/// `0056e8d0` on `this`, `004b4fa0` on `+0x10` and `+0x20`, then
/// `004b4ff0` on `+0x20` with the floats 1, 0, 0, 0. Returns `this`.
pub fn fn_0066d650(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_0056E8D0, &args![this]);
    e.call(MEMBER_004B4FA0, &args![this.addr() + 0x10]);
    e.call(MEMBER_004B4FA0, &args![this.addr() + 0x20]);
    let (one, zero) = (1.0f32.to_bits(), 0.0f32.to_bits());
    e.call(
        MEMBER_SET_004B4FF0,
        &args![this.addr() + 0x20, one, zero, zero, zero],
    );
    this
}

// Translated from 0066d6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066d6a0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `bhkCapsuleShape::bhkCapsuleShape` (engine map; `thiscall`, `RET 4`):
/// the base constructor `0056e690`, the vtable `01066e2c`, `Init_ov2`
/// (`00cab820`) with the construction info, and the counter at `01268324`
/// raised. Returns `this`.
pub fn bhk_capsule_shape_bhk_capsule_shape(e: &mut Engine, this: Ptr, info: u32) -> Ptr {
    e.call(CAPSULE_BASE_0056E690, &args![this]);
    e.mem.set_u32(this.addr(), BHK_CAPSULE_SHAPE_VTABLE);
    e.call(CAPSULE_INIT_00CAB820, &args![this, info]);
    count_instance(e, CAPSULE_COUNTER_01268324);
    this
}

// Translated from 0066d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; a constructor):
/// `0056e900` on `this`, `006299a0` on `+4`. Returns `this`.
pub fn fn_0066d770(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_0056E900, &args![this]);
    e.call(MEMBER_006299A0, &args![this.addr() + 4]);
    this
}

// Translated from 0066d7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): `0066da50` on
/// `this + 4`, its answer passed on.
pub fn fn_0066d7a0(e: &mut Engine, this: Ptr) -> u32 {
    fn_0066da50(e, Ptr::new(this.addr() + 4))
}

// Translated from 0066d880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeNode::GetViewerStrings` (engine map; `thiscall`, `RET 4`: the
/// string list): `NiNode::GetViewerStrings`, then `"BSTreeNode"`; with a
/// model (`+0xe4`) a new 0x20-byte text (`00401000`, formatted by
/// `00406d00`) `"Type = Instance"` or `"Type = Base"` by the model's
/// `00666f00`, then another `"Seed = %d"` with its `008d6f30`; without one
/// `"-No Model-"`. Each string goes to the list through a stack word
/// holding it (`0096a610`).
pub fn bs_tree_node_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(NI_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(8, |e, locals| {
        let text = locals.addr();
        let title = locals.addr() + 4;
        let line = e
            .call(NI_GET_VIEWER_STRING, &args![TEXT_BS_TREE_NODE])
            .u32();
        e.mem.set_u32(title, line);
        e.call(STRINGS_ADD_0096A610, &args![strings, title]);
        let model_slot = this.addr() + 0xe4;
        if e.call(POINTER_GET, &args![model_slot]).u32() != 0 {
            let buffer = e.call(OPERATOR_NEW_ARRAY, &args![0x20u32]).u32();
            e.mem.set_u32(text, buffer);
            let model = e.call(POINTER_GET, &args![model_slot]).u32();
            let kind = if e.call(MODEL_TEST_00666F00, &args![model]).u32() & 0xff != 0 {
                TEXT_TYPE_INSTANCE
            } else {
                TEXT_TYPE_BASE
            };
            let buffer = e.mem.u32(text);
            e.call(FORMAT_00406D00, &args![buffer, 0x20u32, kind]);
            e.call(STRINGS_ADD_0096A610, &args![strings, text]);
            let buffer = e.call(OPERATOR_NEW_ARRAY, &args![0x20u32]).u32();
            e.mem.set_u32(text, buffer);
            let model = e.call(POINTER_GET, &args![model_slot]).u32();
            let seed = e.call(MODEL_SEED_008D6F30, &args![model]).u32();
            let buffer = e.mem.u32(text);
            e.call(FORMAT_00406D00, &args![buffer, 0x20u32, TEXT_SEED, seed]);
            e.call(STRINGS_ADD_0096A610, &args![strings, text]);
        } else {
            let buffer = e.call(OPERATOR_NEW_ARRAY, &args![0x20u32]).u32();
            e.mem.set_u32(text, buffer);
            let buffer = e.mem.u32(text);
            e.call(FORMAT_00406D00, &args![buffer, 0x20u32, TEXT_NO_MODEL]);
            e.call(STRINGS_ADD_0096A610, &args![strings, text]);
        }
    });
}

// Translated from 0066d9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `004a4840` on a zeroed
/// stack byte with `this`, then `0066d9e0` on `this` with that answer and
/// `value` (the word the first call leaves pushed); its answer passed on.
pub fn fn_0066d9b0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        let first = e.call(STEP_004A4840, &args![byte, this]).u32();
        e.call(STEP_0066D9E0, &args![this, first, value]).u32()
    })
}

// Translated from 0066da50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0066da50 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, plain `RET`; a destructor body):
/// `0066dab0`, then `00483710` on `this`, whose answer is passed on.
pub fn fn_0066da50(e: &mut Engine, this: Ptr) -> u32 {
    e.call(BODY_0066DAB0, &args![this]);
    e.call(AFTER_00483710, &args![this]).u32()
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_2430, bhk_transform_shape_q_cinfo_size(Ptr) -> u32),
        entry!(0x0066_b620, bs_tree_node_get_billboard(Ptr) -> u32),
        entry!(0x0066_b640, fn_0066b640(Ptr) -> Ptr),
        entry!(0x0066_bbb0, bs_tree_node_set_billboard(Ptr, u32)),
        entry!(
            0x0066_bec0,
            bs_tree_node_set_texture_prop_on_billboard(Ptr, u32) -> bool
        ),
        entry!(0x0066_c930, fn_0066c930(Ptr)),
        entry!(0x0066_c9c0, fn_0066c9c0(Ptr, u32, u32, f32)),
        entry!(0x0066_d3a0, fn_0066d3a0(Ptr, u32) -> Ptr),
        entry!(0x0066_d3d0, fn_0066d3d0(Ptr, u32) -> Ptr),
        entry!(0x0066_d440, fn_0066d440(Ptr, u32) -> Ptr),
        entry!(
            0x0066_d490,
            bhk_multi_sphere_shape_bhk_multi_sphere_shape(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0066_d520,
            bhk_multi_sphere_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0066_d550,
            hk_transform_shape_cinfo_hk_transform_shape_cinfo(Ptr) -> Ptr
        ),
        entry!(
            0x0066_d590,
            bhk_transform_shape_bhk_transform_shape(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0066_d620,
            bhk_transform_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0066_d650, fn_0066d650(Ptr) -> Ptr),
        entry!(
            0x0066_d6a0,
            bhk_capsule_shape_bhk_capsule_shape(Ptr, u32) -> Ptr
        ),
        entry!(0x0066_d770, fn_0066d770(Ptr) -> Ptr),
        entry!(0x0066_d7a0, fn_0066d7a0(Ptr) -> u32),
        entry!(0x0066_d880, bs_tree_node_get_viewer_strings(Ptr, u32)),
        entry!(0x0066_d9b0, fn_0066d9b0(Ptr, u32) -> u32),
        entry!(0x0066_da50, fn_0066da50(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

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
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn plain_pointers(e: &mut Engine) {
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    /// An object whose vtable slots at `offsets` go to `0x7200_0000 + offset`.
    fn object_with_slots(e: &mut Engine, offsets: &[u32]) -> u32 {
        let object = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        for &offset in offsets {
            e.mem.set_u32(vtable + offset, 0x7200_0000 + offset);
        }
        object
    }

    #[test]
    fn small_getters() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        assert_eq!(e.call(0x0062_2430, &args![0x1234u32]).u32(), 0x50);
        let this = e.mem.alloc(0xf8);
        e.mem.set_u32(this + 0xf0, 0xbbbb);
        assert_eq!(e.call(0x0066_b620, &args![this]).u32(), 0xbbbb);
    }

    #[test]
    fn collision_object_constructors_set_their_vtables() {
        let mut e = Engine::new();
        quiet(&mut e, &[COLLISION_BASE_00C6B910, COLLISION_BASE_00C6B950]);
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_b640, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BHK_COLLISION_OBJECT_VTABLE);
        assert_eq!(calls(&e, COLLISION_BASE_00C6B910), vec![vec![this]]);
        assert_eq!(e.call(0x0066_d3d0, &args![this, 7u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BHK_P_COLLISION_OBJECT_VTABLE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_d3a0, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BHK_SP_COLLISION_OBJECT_VTABLE);
        assert_eq!(calls(&e, COLLISION_BASE_00C6B950), vec![vec![this, 9]]);
    }

    #[test]
    fn set_billboard_swaps_the_child() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[0x7200_00e8, 0x7200_00dc]);
        let node = object_with_slots(&mut e, &[0xe8, 0xdc]);
        let this = object_with_slots(&mut e, &[0x10c]);
        e.register_double(0x7200_010c, move |_, _| node.into_ret());
        e.mem.set_u32(this + 0xf0, 0x0aaa);
        e.call_log = Some(vec![]);
        e.call(0x0066_bbb0, &args![this, 0x0bbbu32]);
        assert_eq!(calls(&e, 0x7200_00e8), vec![vec![node, 0x0aaa]]);
        assert_eq!(calls(&e, 0x7200_00dc), vec![vec![node, 0x0bbb, 1]]);
        assert_eq!(e.mem.u32(this + 0xf0), 0x0bbb);
        // No old billboard: no detach.
        e.mem.set_u32(this + 0xf0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_bbb0, &args![this, 0x0cccu32]);
        assert!(calls(&e, 0x7200_00e8).is_empty());
        assert_eq!(e.mem.u32(this + 0xf0), 0x0ccc);
        // No billboard, or no node: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0066_bbb0, &args![this, 0u32]);
        assert!(calls(&e, 0x7200_010c).is_empty());
        e.register(0x7200_010c, |_, _| 0u32.into_ret());
        e.call(0x0066_bbb0, &args![this, 0x0dddu32]);
        assert_eq!(e.mem.u32(this + 0xf0), 0x0ccc);
    }

    #[test]
    fn set_texture_prop_on_billboard_replaces_the_property() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[REMOVE_PROPERTY_00A5B230, ATTACH_PROPERTY]);
        e.register(GET_PROPERTY, |e, a| e.mem.u32(a[0]).into_ret());
        let billboard = e.mem.alloc(4);
        let this = e.mem.alloc(0xf8);
        e.mem.set_u32(this + 0xf0, billboard);
        e.mem.set_u32(billboard, 0x51);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0066_bec0, &args![this, 0x52u32]).bool());
        assert_eq!(calls(&e, GET_PROPERTY), vec![vec![billboard, 5]]);
        assert_eq!(
            calls(&e, REMOVE_PROPERTY_00A5B230),
            vec![vec![billboard, 5]]
        );
        assert_eq!(calls(&e, ATTACH_PROPERTY), vec![vec![billboard, 0x52]]);
        // The same property: false, nothing changed.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0066_bec0, &args![this, 0x51u32]).bool());
        assert!(calls(&e, ATTACH_PROPERTY).is_empty());
        // No current property: attached without a removal.
        e.mem.set_u32(billboard, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0066_bec0, &args![this, 0x52u32]).bool());
        assert!(calls(&e, REMOVE_PROPERTY_00A5B230).is_empty());
        // No property or no billboard: false.
        assert!(!e.call(0x0066_bec0, &args![this, 0u32]).bool());
        e.mem.set_u32(this + 0xf0, 0);
        assert!(!e.call(0x0066_bec0, &args![this, 0x52u32]).bool());
    }

    #[test]
    fn fn_0066c930_scales_by_the_reciprocal_root() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for (i, v) in (0u32..).zip([2.0f32, 3.0, 4.0, 5.0]) {
            e.mem.set_f32(this + i * 4, v);
        }
        let squares = e.mem.alloc(0x10);
        for (i, v) in (0u32..).zip([16.0f32, 7.0, 8.0, 9.0]) {
            e.mem.set_f32(squares + i * 4, v);
        }
        e.register(STEP_0062B0A0, |_, a| a[1].into_ret());
        e.register_double(STEP_006815C0, move |_, _| squares.into_ret());
        // 004a3c90 copies the estimate's first lane into every lane of
        // `this`; 00561240 answers `this`.
        e.register(STEP_004A3C90, |e, a| {
            let r = e.mem.u32(a[1]);
            for i in 0..4 {
                e.mem.set_u32(a[0] + i * 4, r);
            }
            Ret::default()
        });
        e.register(STEP_00561240, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0066_c930, &args![this]);
        let r = crate::x87::rsqrt_estimate(16.0);
        for (i, v) in (0u32..).zip([2.0f32, 3.0, 4.0, 5.0]) {
            assert_eq!(e.mem.f32(this + i * 4), v * r);
        }
        assert_eq!(calls(&e, STEP_0062B0A0)[0][2], this);
    }

    #[test]
    fn fn_0066c9c0_sets_both_parts_and_the_float() {
        let mut e = Engine::new();
        quiet(&mut e, &[SET_004A3F10]);
        let slot = e.mem.alloc(4);
        e.register_double(SLOT_00560D30, move |_, _| slot.into_ret());
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0066_c9c0, &args![this, 1u32, 2u32, 0.5f32.to_bits()]);
        assert_eq!(
            calls(&e, SET_004A3F10),
            vec![vec![this, 1], vec![this + 0x10, 2]]
        );
        assert_eq!(calls(&e, SLOT_00560D30), vec![vec![this + 0x10, 3]]);
        assert_eq!(e.mem.f32(slot), 0.5);
    }

    #[test]
    fn fn_0066d440_destroys_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0066_d410, NI_SIZED_DELETE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_d440, &args![0x4000u32, 0u32]).u32(), 0x4000);
        assert_eq!(calls(&e, 0x0066_d410), vec![vec![0x4000]]);
        assert!(calls(&e, NI_SIZED_DELETE).is_empty());
        e.call(0x0066_d440, &args![0x4000u32, 1u32]);
        assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![0x4000, 0x14]]);
    }

    #[test]
    fn shape_constructors_init_and_count() {
        let mut e = Engine::new();
        let cases: [(u32, u32, u32, u32, u32); 3] = [
            (
                0x0066_d490,
                MULTI_SPHERE_BASE_0056E6C0,
                MULTI_SPHERE_INIT_00CB7B80,
                MULTI_SPHERE_COUNTER_012683DC,
                BHK_MULTI_SPHERE_SHAPE_VTABLE,
            ),
            (
                0x0066_d590,
                TRANSFORM_BASE_0056E6F0,
                TRANSFORM_INIT_00C8C5B0,
                TRANSFORM_COUNTER_012681B0,
                BHK_TRANSFORM_SHAPE_VTABLE,
            ),
            (
                0x0066_d6a0,
                CAPSULE_BASE_0056E690,
                CAPSULE_INIT_00CAB820,
                CAPSULE_COUNTER_01268324,
                BHK_CAPSULE_SHAPE_VTABLE,
            ),
        ];
        for (addr, base, init, counter, vtable) in cases {
            quiet(&mut e, &[base, init]);
            e.map(counter, 4);
            e.mem.set_u32(counter, 6);
            let this = e.mem.alloc(0x14);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this, 0x99u32]).u32(), this);
            assert_eq!(calls(&e, base), vec![vec![this]]);
            assert_eq!(calls(&e, init), vec![vec![this, 0x99]]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(counter), 7);
        }
    }

    #[test]
    fn shape_deleting_destructors_free_on_bit_0() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                MULTI_SPHERE_DESTRUCTOR,
                TRANSFORM_DESTRUCTOR,
                NI_SIZED_DELETE,
            ],
        );
        for (addr, destructor) in [
            (0x0066_d520, MULTI_SPHERE_DESTRUCTOR),
            (0x0066_d620, TRANSFORM_DESTRUCTOR),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![0x5000u32, 2u32]).u32(), 0x5000);
            assert_eq!(calls(&e, destructor), vec![vec![0x5000]]);
            assert!(calls(&e, NI_SIZED_DELETE).is_empty());
            e.call(addr, &args![0x5000u32, 1u32]);
            assert_eq!(calls(&e, NI_SIZED_DELETE), vec![vec![0x5000, 0x14]]);
        }
    }

    #[test]
    fn small_constructors_run_their_parts() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                CINFO_BASE_004B5020,
                CINFO_MEMBER_0056E270,
                CINFO_MEMBER_0056E120,
                BASE_0056E8D0,
                MEMBER_004B4FA0,
                MEMBER_SET_004B4FF0,
                BASE_0056E900,
                MEMBER_006299A0,
            ],
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, 0xffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_d550, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(calls(&e, CINFO_BASE_004B5020), vec![vec![this]]);
        assert_eq!(calls(&e, CINFO_MEMBER_0056E270), vec![vec![this + 0x10]]);
        assert_eq!(calls(&e, CINFO_MEMBER_0056E120), vec![vec![this + 0x10]]);
        assert_eq!(e.call(0x0066_d650, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_0056E8D0), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMBER_004B4FA0),
            vec![vec![this + 0x10], vec![this + 0x20]]
        );
        assert_eq!(
            calls(&e, MEMBER_SET_004B4FF0),
            vec![vec![this + 0x20, 0x3f80_0000, 0, 0, 0]]
        );
        assert_eq!(e.call(0x0066_d770, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_0056E900), vec![vec![this]]);
        assert_eq!(calls(&e, MEMBER_006299A0), vec![vec![this + 4]]);
    }

    #[test]
    fn destructor_bodies_pass_on_the_last_answer() {
        let mut e = Engine::new();
        quiet(&mut e, &[BODY_0066DAB0]);
        e.register(AFTER_00483710, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_da50, &args![0x6000u32]).u32(), 0x6001);
        assert_eq!(calls(&e, BODY_0066DAB0), vec![vec![0x6000]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_d7a0, &args![0x6000u32]).u32(), 0x6005);
        assert_eq!(calls(&e, BODY_0066DAB0), vec![vec![0x6004]]);
    }

    #[test]
    fn get_viewer_strings_describes_the_model() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[NI_NODE_GET_VIEWER_STRINGS, FORMAT_00406D00]);
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(OPERATOR_NEW_ARRAY, |_, _| 0x2222u32.into_ret());
        e.register(MODEL_TEST_00666F00, |_, _| 0xff01u32.into_ret());
        e.register(MODEL_SEED_008D6F30, |_, _| 42u32.into_ret());
        e.register_double(STRINGS_ADD_0096A610, |e, a| {
            let string = e.mem.u32(a[1]);
            let n = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4 + n * 4, string);
            e.mem.set_u32(a[0], n + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0xf8);
        e.mem.set_u32(this + 0xe4, 0x3333);
        let strings = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0066_d880, &args![this, strings]);
        assert_eq!(
            calls(&e, NI_NODE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING),
            vec![vec![TEXT_BS_TREE_NODE]]
        );
        assert_eq!(
            calls(&e, FORMAT_00406D00),
            vec![
                vec![0x2222, 0x20, TEXT_TYPE_INSTANCE],
                vec![0x2222, 0x20, TEXT_SEED, 42]
            ]
        );
        assert_eq!(e.mem.u32(strings), 3);
        assert_eq!(e.mem.u32(strings + 4), 0x1111);
        assert_eq!(e.mem.u32(strings + 8), 0x2222);
        assert_eq!(calls(&e, MODEL_TEST_00666F00), vec![vec![0x3333]]);

        // A base model; then no model.
        e.register(MODEL_TEST_00666F00, |_, _| 0xff00u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0066_d880, &args![this, strings]);
        assert_eq!(calls(&e, FORMAT_00406D00)[0][2], TEXT_TYPE_BASE);
        e.mem.set_u32(this + 0xe4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_d880, &args![this, strings]);
        assert_eq!(
            calls(&e, FORMAT_00406D00),
            vec![vec![0x2222, 0x20, TEXT_NO_MODEL]]
        );
        assert!(calls(&e, MODEL_TEST_00666F00).is_empty());
    }

    #[test]
    fn fn_0066d9b0_passes_the_left_word_on() {
        let mut e = Engine::new();
        e.register(STEP_004A4840, |e, a| {
            (u32::from(e.mem.u8(a[0])) + a[1]).into_ret()
        });
        e.register(STEP_0066D9E0, |_, a| (a[1] ^ a[2]).into_ret());
        e.call_log = Some(vec![]);
        let answer = e.call(0x0066_d9b0, &args![0x100u32, 0x0f0fu32]).u32();
        assert_eq!(answer, 0x100 ^ 0x0f0f);
        assert_eq!(calls(&e, STEP_0066D9E0), vec![vec![0x100, 0x100, 0x0f0f]]);
    }
}
